use miette::IntoDiagnostic;
use serde::Deserialize;
use tabled::{Table, Tabled};

#[derive(Deserialize)]
struct Scope {
    name: String,
}

#[derive(Deserialize)]
struct ScopeListResponse {
    scopes: Vec<Scope>,
}

#[derive(Tabled)]
struct OutputTableRow {
    name: String,
}

impl From<ScopeListResponse> for Vec<OutputTableRow> {
    fn from(response: ScopeListResponse) -> Self {
        response
            .scopes
            .into_iter()
            .map(|scope| OutputTableRow { name: scope.name })
            .collect()
    }
}

pub(crate) fn execute(token: &str, runtime_url: &str) -> miette::Result<()> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(format!("{runtime_url}/api/scope"))
        .bearer_auth(token.to_owned())
        .send()
        .into_diagnostic()?
        .error_for_status()
        .into_diagnostic()?
        .json::<ScopeListResponse>()
        .expect("Failed to parse response");

    let rows: Vec<OutputTableRow> = response.into();

    let table = Table::new(rows);

    println!("{table}");
    Ok(())
}
