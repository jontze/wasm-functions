use axum::{extract::Request, http, http::StatusCode, middleware::Next, response::Response};

#[derive(Debug, Clone)]
pub(crate) struct AuthenticatedUser {}

pub(crate) async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
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

    let authenticated_user = authorize_user_by_token(auth_header).await?;

    req.extensions_mut().insert(authenticated_user);

    Ok(next.run(req).await)
}

async fn authorize_user_by_token(token: &str) -> Result<AuthenticatedUser, StatusCode> {
    todo!("Validate token and decode user information")
}
