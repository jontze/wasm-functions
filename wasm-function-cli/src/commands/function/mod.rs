use clap::{Parser, Subcommand, ValueEnum};

use super::{command_context, command_executor, CredentialStoreTrait};

mod delete;
mod list;

#[derive(Subcommand)]
pub(super) enum FunctionCommand {
    /// Delete a function by name
    Delete(DeleteFunctionCommand),
    /// List all functions of a scope
    List(ListFunctionCommand),
}

#[derive(Parser)]
pub(super) struct DeleteFunctionCommand {
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
pub(super) enum FunctionKind {
    Http,
    Scheduled,
}

#[derive(Parser)]
pub(super) struct ListFunctionCommand {
    /// Name of the scope the functions belong to
    #[clap(short, long)]
    scope_name: String,
}

impl<TCredStore: CredentialStoreTrait> command_executor::CommandExecutorTrait<TCredStore>
    for FunctionCommand
{
    fn execute(&self, ctx: &mut command_context::CommandContext<TCredStore>) -> miette::Result<()> {
        let active_token = crate::auth::token_refresh::get_active_token(ctx)?;
        let function_runtime_url = &ctx.config.function_runtime_url;

        match self {
            FunctionCommand::Delete(delete_command) => delete::execute(
                &active_token,
                function_runtime_url,
                &delete_command.scope_name,
                &delete_command.id,
                &delete_command.kind,
            ),
            FunctionCommand::List(list_command) => list::execute(
                &active_token,
                function_runtime_url,
                &list_command.scope_name,
            ),
        }
    }
}
