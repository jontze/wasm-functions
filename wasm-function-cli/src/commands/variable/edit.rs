use miette::IntoDiagnostic;

pub(super) fn execute(
    active_token: &str,
    function_runtime_url: &str,
    scope_name: &str,
    id: &str,
    value: &str,
) -> miette::Result<()> {
    let client = reqwest::blocking::Client::new();

    client
        .put(format!(
            "{function_runtime_url}/api/scope/{scope_name}/variable/{id}",
            function_runtime_url = function_runtime_url,
            scope_name = scope_name,
            id = id
        ))
        .bearer_auth(active_token.to_owned())
        .json(&serde_json::json!({ "value": value }))
        .send()
        .into_diagnostic()?
        .error_for_status()
        .into_diagnostic()?;

    println!("Variable updated successfully");

    Ok(())
}
