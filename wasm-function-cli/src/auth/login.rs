use miette::{Context, IntoDiagnostic};
use reqwest::blocking::Client;

use crate::auth::model;

pub(crate) fn login(config: &crate::config::Config) -> miette::Result<model::TokenResponse> {
    let client = Client::new();

    let tenant_url = &config.tenant_url;
    let tenant_id = &config.tenant_id;
    let client_id = &config.client_id;
    let scopes = &config.scopes;

    let response = client
        .post(format!(
            "{tenant_url}/{tenant_id}/oauth2/v2.0/devicecode
    "
        ))
        .form(&[("client_id", client_id), ("scope", scopes)])
        .send().into_diagnostic().wrap_err("Request to receive device codes failed. Is you're tenant url, id, client id or scopes correct?")?.json::<model::DeviceCodeResponse>().into_diagnostic().wrap_err("Failed to parse device code response")?;

    // Show user message with code and device login url
    println!("{}\n", response.message);

    let spinner =
        indicatif::ProgressBar::new_spinner().with_message("Waiting for code verification...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(50));

    // Ignore failures to open the browser
    let _ = open::that(response.verification_uri).into_diagnostic();

    let start_instant = std::time::Instant::now();
    loop {
        // Abort condition when timeout is reached
        if std::time::Instant::now()
            >= start_instant + std::time::Duration::from_secs(response.expires_in.into())
        {
            spinner.finish_with_message("Device code expired");
            return Err(miette::Report::msg("Device code expired"));
        }

        // Poll for token
        let token_poll_response = client
            .post(format!("{tenant_url}/{tenant_id}/oauth2/v2.0/token"))
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ("client_id", client_id),
                ("device_code", &response.device_code),
            ])
            .send()
            .into_diagnostic()
            .wrap_err("Request to poll for token failed")?
            .json::<model::TokenResponse>()
            .into_diagnostic()
            .wrap_err("Failed to parse token response")?;

        // Abort when token is available
        if token_poll_response.access_token.is_some() {
            spinner.finish_with_message("Login successful");
            return Ok(token_poll_response);
        }

        // Respect the interval between requests
        std::thread::sleep(std::time::Duration::from_secs(response.interval.into()));
    }
}
