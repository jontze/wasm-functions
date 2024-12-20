use miette::IntoDiagnostic;

mod auth;
mod commands;
mod config;
mod cred_store;

fn main() -> miette::Result<()> {
    let config = config::Config::from_env().into_diagnostic()?;
    let mut cred_store = cred_store::CredentialStore::default();

    let mut context = commands::CommandContext {
        config: &config,
        cred_store: &mut cred_store,
    };

    commands::invoke(&mut context)
}
