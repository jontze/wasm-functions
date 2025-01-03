use miette::IntoDiagnostic;

pub(crate) fn execute(token: &str, runtime_url: &str, name: &str) -> miette::Result<()> {
    let client = reqwest::blocking::Client::new();

    client
        .delete(format!(
            "{runtime_url}/api/scope/{name}",
            runtime_url = runtime_url,
            name = name
        ))
        .bearer_auth(token.to_owned())
        .send()
        .into_diagnostic()?
        .error_for_status()
        .into_diagnostic()?;

    Ok(())
}
