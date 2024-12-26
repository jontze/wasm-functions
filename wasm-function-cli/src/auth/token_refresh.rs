use base64::Engine;
use miette::{Context, IntoDiagnostic};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TokenRefreshResponse {
    /// Expiry timestamp in seconds since epoch
    exp: u64,
}

fn is_expired(token: &str) -> miette::Result<bool> {
    let token_slices = token.split('.').collect::<Vec<&str>>();

    if token_slices.len() != 3 {
        return Err(miette::Report::msg("Invalid jwt format"));
    }

    let payload = token_slices[1];
    let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload)
        .into_diagnostic()?;

    let payload = serde_json::from_slice::<TokenRefreshResponse>(&payload).into_diagnostic()?;

    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    Ok(payload.exp < current_time)
}

pub(crate) fn get_active_token<TCredStore: crate::cred_store::CredentialStoreTrait>(
    ctx: &mut crate::commands::CommandContext<TCredStore>,
) -> miette::Result<String> {
    // Extract the credentials
    ctx.cred_store
        .load()
        .wrap_err("No credentials found. Please authenticate first.")?;
    let current_token = ctx
        .cred_store
        .get_tokens()
        .wrap_err("No credentials found. Please authenticate first.")?;

    if is_expired(&current_token.access_token)? {
        let tenant_url = &ctx.config.tenant_url;
        let tenant_id = &ctx.config.tenant_id;
        let client_id = &ctx.config.client_id;

        // Refresh the token
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{tenant_url}/{tenant_id}/oauth2/v2.0/token"))
            .form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", &current_token.refresh_token),
                ("client_id", client_id),
            ])
            .send()
            .into_diagnostic()
            .wrap_err("Failed to refresh token")?
            .json::<super::model::TokenResponse>()
            .into_diagnostic()
            .wrap_err("Failed to parse token refresh response")?;

        // Update and save the token
        ctx.cred_store
            .save(&response.clone().into())
            .wrap_err("Failed to update token")?;

        let renewed_token = response.access_token.unwrap();
        Ok(renewed_token)
    } else {
        Ok(current_token.access_token.to_owned())
    }
}
