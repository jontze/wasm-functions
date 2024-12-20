use clap::{Parser, Subcommand};
use command_executor::CommandExecutorTrait;

pub(crate) mod command_context;
mod command_executor;
mod deploy;
mod login;
mod logout;

pub(crate) use command_context::CommandContext;

use crate::cred_store::CredentialStoreTrait;

#[derive(Parser)]
#[clap(author, version, about = "CLI Tool for managing wasm functions")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Authenticate the cli against the function runtime
    Login,
    /// Logout the cli from the function runtime
    Logout,
    /// Deploy a local function to the runtime
    Deploy(DeployCommand),
}

#[derive(Parser)]
struct DeployCommand {
    /// Path to the manifest file
    #[arg(short, long)]
    manifest_path: Option<std::path::PathBuf>,
    /// Path to the wasm file
    #[arg(short, long)]
    wasm_path: std::path::PathBuf,
}

impl<TCredStore: CredentialStoreTrait> command_executor::CommandExecutorTrait<TCredStore>
    for DeployCommand
{
    fn execute(&self, ctx: &mut command_context::CommandContext<TCredStore>) -> miette::Result<()> {
        deploy::execute(ctx, &self.wasm_path, self.manifest_path.as_ref())
    }
}

impl<TCredStore: CredentialStoreTrait> command_executor::CommandExecutorTrait<TCredStore>
    for Command
{
    fn execute(&self, ctx: &mut command_context::CommandContext<TCredStore>) -> miette::Result<()> {
        match self {
            Command::Login => login::execute(ctx),
            Command::Logout => logout::execute(ctx),
            Command::Deploy(deploy_command) => deploy_command.execute(ctx),
        }
    }
}

pub(crate) fn invoke<TCredStore: CredentialStoreTrait>(
    ctx: &mut command_context::CommandContext<TCredStore>,
) -> miette::Result<()> {
    let cli = Cli::parse();

    cli.command.execute(ctx)
}
