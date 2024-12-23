use axum::{
    extract::{Request, State},
    http::{self, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation};
use serde::Deserialize;
use tracing::error;

const JWKS_ENTRY_CACHE_KEY: &str = "jwks";

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AuthenticatedUser {
    /// Object ID of the user
    pub oid: String,
}

pub(crate) async fn auth(
    State(state): State<crate::server_state::RuntimeStateRef>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_header_value| {
            if auth_header_value.starts_with("Bearer ") {
                Some(auth_header_value.trim_start_matches("Bearer "))
            } else {
                None
            }
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let authenticated_user = authorize_user_by_token(
        auth_header,
        &state.jwk_cache,
        &state.app_config.openid_connect,
    )
    .await?;

    req.extensions_mut().insert(authenticated_user);

    Ok(next.run(req).await)
}

async fn authorize_user_by_token(
    token: &str,
    jwks_cache: &crate::server_state::JwkSetCache,
    oidc_config: &crate::config::OpenIdConnectConfig,
) -> Result<AuthenticatedUser, StatusCode> {
    // Only decode the header and get the key ID
    let key_id = jsonwebtoken::decode_header(token)
        .map_err(|err| {
            error!("Failed to decode token header: {:?}", err);
            StatusCode::UNAUTHORIZED
        })
        .and_then(|header| header.kid.ok_or(StatusCode::UNAUTHORIZED))?;

    // Extract the public key from the JWKS endpoint or cache
    let jwks = match jwks_cache.get(JWKS_ENTRY_CACHE_KEY).await {
        Some(jwks) => jwks,
        None => {
            let fetched_jwk_set = fetch_jwks(&oidc_config.jwks_url).await?;
            jwks_cache
                .insert(JWKS_ENTRY_CACHE_KEY.to_string(), fetched_jwk_set.clone())
                .await;
            fetched_jwk_set
        }
    };

    // Find the key with the matching key ID
    let jwk = jwks
        .find(&key_id)
        .ok_or(StatusCode::UNAUTHORIZED)
        .map_err(|err| {
            error!("Failed to find key in key set with key ID: {:?}", err);
            err
        })?;

    // Create a decoding key from the JWK
    let decoding_key = DecodingKey::from_jwk(jwk).map_err(|err| {
        error!(
            "Failed to create decoding key from key set: {:?} - {:?}",
            err, jwk
        );
        StatusCode::UNAUTHORIZED
    })?;

    let validation = {
        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_issuer(&[&oidc_config.issuer]);
        validation.set_audience(&[&oidc_config.audience]);
        validation.set_required_spec_claims(&["oid"]);
        validation.validate_exp = true;
        validation.validate_nbf = true;
        validation
    };

    let token_data = jsonwebtoken::decode::<AuthenticatedUser>(token, &decoding_key, &validation)
        .map_err(|err| {
        match err.kind() {
            _ => {}
        };
        error!("Failed to decode token: {:?}", err);
        StatusCode::UNAUTHORIZED
    })?;

    Ok(token_data.claims)
}

async fn fetch_jwks(jwks_uri: &str) -> Result<jsonwebtoken::jwk::JwkSet, StatusCode> {
    let jwks = reqwest::get(jwks_uri)
        .await
        .map_err(|err| {
            error!("Failed to fetch JWKS: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .json::<jsonwebtoken::jwk::JwkSet>()
        .await
        .map_err(|err| {
            error!("Failed to parse JWKS: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(jwks)
}
