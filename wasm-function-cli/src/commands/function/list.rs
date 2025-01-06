use miette::IntoDiagnostic;
use serde::Deserialize;
use tabled::{Table, Tabled};

#[derive(Deserialize)]
struct Function {
    uuid: String,
    name: String,
    kind: String,
}

#[derive(Deserialize)]
struct FunctionListResponse {
    functions: Vec<Function>,
}

#[derive(Tabled)]
struct OutputTableRow {
    name: String,
    kind: String,
    uuid: String,
}

impl From<FunctionListResponse> for Vec<OutputTableRow> {
    fn from(response: FunctionListResponse) -> Self {
        response
            .functions
            .into_iter()
            .map(|function| OutputTableRow {
                uuid: function.uuid,
                name: function.name,
                kind: function.kind,
            })
            .collect()
    }
}

pub(super) fn execute(token: &str, runtime_url: &str, scope_name: &str) -> miette::Result<()> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(format!("{runtime_url}/api/scope/{scope_name}/function"))
        .bearer_auth(token.to_owned())
        .send()
        .into_diagnostic()?
        .error_for_status()
        .into_diagnostic()?
        .json::<FunctionListResponse>()
        .expect("Failed to parse response");

    let rows: Vec<OutputTableRow> = response.into();

    let table = Table::new(rows);

    println!("{table}");
    Ok(())
}
