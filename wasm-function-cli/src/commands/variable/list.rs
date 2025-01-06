use miette::IntoDiagnostic;
use serde::Deserialize;
use tabled::{Table, Tabled};

#[derive(Deserialize)]
struct Variable {
    name: String,
    value: String,
}

#[derive(Deserialize)]
struct VariableListResponse {
    variables: Vec<Variable>,
}

#[derive(Tabled)]
struct OutputTableRow {
    name: String,
    value: String,
}

impl From<VariableListResponse> for Vec<OutputTableRow> {
    fn from(response: VariableListResponse) -> Self {
        response
            .variables
            .into_iter()
            .map(|variable| OutputTableRow {
                name: variable.name,
                value: variable.value,
            })
            .collect()
    }
}

pub(super) fn execute(
    active_token: &str,
    function_runtime_url: &str,
    scope_name: &str,
) -> miette::Result<()> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(format!(
            "{function_runtime_url}/api/scope/{scope_name}/variable"
        ))
        .bearer_auth(active_token.to_owned())
        .send()
        .into_diagnostic()?
        .error_for_status()
        .into_diagnostic()?
        .json::<VariableListResponse>()
        .expect("Failed to parse response");

    let rows: Vec<OutputTableRow> = response.into();

    let table = Table::new(rows);
    println!("{table}");

    Ok(())
}
