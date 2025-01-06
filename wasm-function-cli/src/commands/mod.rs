use clap::{Parser, Subcommand};
use command_executor::CommandExecutorTrait;

pub(crate) mod command_context;
mod command_executor;
mod deploy;
mod function;
mod login;
mod logout;
mod scope;
mod variable;

pub(crate) use command_context::CommandContext;
use function::FunctionCommand;
use scope::ScopeCommand;
use variable::VariableCommand;

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
    /// Commands to manage functions of a scope
    #[clap(subcommand)]
    Function(FunctionCommand),
    /// Commands to manage scopes
    #[clap(subcommand)]
    Scope(ScopeCommand),
    /// Commands to manage variables of a scope
    #[clap(subcommand)]
    Variable(VariableCommand),
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
            Command::Function(function_command) => function_command.execute(ctx),
            Command::Scope(scope_command) => scope_command.execute(ctx),
            Command::Variable(variable_command) => variable_command.execute(ctx),
        }
    }
}

pub(crate) fn invoke<TCredStore: CredentialStoreTrait>(
    ctx: &mut command_context::CommandContext<TCredStore>,
) -> miette::Result<()> {
    let cli = Cli::parse();

    cli.command.execute(ctx)
}
