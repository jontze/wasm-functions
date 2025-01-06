use miette::IntoDiagnostic;

pub(super) fn execute(
    active_token: &str,
    function_runtime_url: &str,
    scope_name: &str,
    name: &str,
    value: &str,
) -> miette::Result<()> {
    let client = reqwest::blocking::Client::new();

    client
        .post(format!(
            "{function_runtime_url}/api/scope/{scope_name}/variable",
            scope_name = scope_name
        ))
        .bearer_auth(active_token.to_owned())
        .json(&serde_json::json!({ "name": name, "value": value }))
        .send()
        .into_diagnostic()?
        .error_for_status()
        .into_diagnostic()?;

    println!("Variable added successfully");

    Ok(())
}
