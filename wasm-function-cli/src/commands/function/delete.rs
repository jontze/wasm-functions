use miette::IntoDiagnostic;

pub(super) fn execute(
    token: &str,
    runtime_url: &str,
    scope_name: &str,
    function_id: &str,
    kind: &super::FunctionKind,
) -> miette::Result<()> {
    let client = reqwest::blocking::Client::new();

    let func_kind = match kind {
        super::FunctionKind::Http => "http",
        super::FunctionKind::Scheduled => "scheduled",
    };

    client
        .delete(format!(
            "{runtime_url}/api/scope/{scope_name}/function/{func_kind}/{function_id}"
        ))
        .bearer_auth(token.to_owned())
        .send()
        .into_diagnostic()?
        .error_for_status()
        .into_diagnostic()?;

    Ok(())
}
