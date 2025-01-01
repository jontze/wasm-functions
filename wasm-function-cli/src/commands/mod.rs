use clap::{Parser, Subcommand, ValueEnum};
use command_executor::CommandExecutorTrait;

pub(crate) mod command_context;
mod command_executor;
mod deploy;
mod function;
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
    /// Commands to manage functions
    #[clap(subcommand)]
    Functions(FunctionCommand),
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

#[derive(Subcommand)]
enum FunctionCommand {
    /// Delete a function by name
    Delete(DeleteFunctionCommand),
}

#[derive(Parser)]
struct DeleteFunctionCommand {
    /// Id of the function to delete
    #[clap(short, long)]
    id: String,
    /// Name of the scope the function belongs to
    #[clap(short, long)]
    scope_name: String,
    /// Kind of the function to delete
    #[clap(short, long)]
    kind: FunctionKind,
}

#[derive(Clone, ValueEnum)]
enum FunctionKind {
    Http,
    Scheduled,
}

impl<TCredStore: CredentialStoreTrait> command_executor::CommandExecutorTrait<TCredStore>
    for DeployCommand
{
    fn execute(&self, ctx: &mut command_context::CommandContext<TCredStore>) -> miette::Result<()> {
        let active_token = crate::auth::token_refresh::get_active_token(ctx)?;
        let function_runtime_url = &ctx.config.function_runtime_url;

        deploy::execute(
            &active_token,
            function_runtime_url,
            &self.wasm_path,
            self.manifest_path.as_ref(),
        )
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
            Command::Functions(function_command) => function_command.execute(ctx),
        }
    }
}

impl<TCredStore: CredentialStoreTrait> command_executor::CommandExecutorTrait<TCredStore>
    for FunctionCommand
{
    fn execute(&self, ctx: &mut command_context::CommandContext<TCredStore>) -> miette::Result<()> {
        let active_token = crate::auth::token_refresh::get_active_token(ctx)?;
        let function_runtime_url = &ctx.config.function_runtime_url;

        match self {
            FunctionCommand::Delete(delete_command) => function::delete::execute(
                &active_token,
                function_runtime_url,
                &delete_command.scope_name,
                &delete_command.id,
                &delete_command.kind,
            ),
        }
    }
}

pub(crate) fn invoke<TCredStore: CredentialStoreTrait>(
    ctx: &mut command_context::CommandContext<TCredStore>,
) -> miette::Result<()> {
    let cli = Cli::parse();

    cli.command.execute(ctx)
}
