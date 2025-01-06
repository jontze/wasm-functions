use miette::IntoDiagnostic;

pub(super) fn execute(
    active_token: &str,
    function_runtime_url: &str,
    scope_name: &str,
    id: &str,
) -> miette::Result<()> {
    let client = reqwest::blocking::Client::new();

    client
        .delete(format!(
            "{function_runtime_url}/api/scope/{scope_name}/variable/{id}"
        ))
        .bearer_auth(active_token.to_owned())
        .send()
        .into_diagnostic()?
        .error_for_status()
        .into_diagnostic()?;

    println!("Variable deleted successfully");

    Ok(())
}
