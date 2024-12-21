use miette::IntoDiagnostic;
use reqwest::blocking::{multipart, Client};

const MANIFEST_FILE_NAME: &str = "manifest.toml";

pub(super) fn execute(
    token: &str,
    runtime_url: &str,
    wasm_path: &std::path::PathBuf,
    manifest_path: Option<&std::path::PathBuf>,
) -> miette::Result<()> {
    // Fallback to default manifest path if not provided
    let default_manifest_path: std::path::PathBuf = MANIFEST_FILE_NAME.into();
    let manifest_path = manifest_path.unwrap_or(&default_manifest_path);

    // Prepare the form
    let form = multipart::Form::new()
        .part(
            "manifest",
            multipart::Part::file(manifest_path).into_diagnostic()?,
        )
        .part("wasm", multipart::Part::file(wasm_path).into_diagnostic()?);

    // Send the request
    let client = Client::new();
    client
        .post(format!("{}/api/deploy", runtime_url))
        .bearer_auth(token.to_owned())
        .multipart(form)
        .send()
        .into_diagnostic()?
        .error_for_status()
        .into_diagnostic()?;

    Ok(())
}
