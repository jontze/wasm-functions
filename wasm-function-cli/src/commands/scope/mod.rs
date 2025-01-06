use clap::{Parser, Subcommand};

use super::{command_context, command_executor, CredentialStoreTrait};

mod delete;
mod list;

#[derive(Subcommand)]
pub(super) enum ScopeCommand {
    /// List all scopes
    List,
    /// Delete a scope by name
    Delete(DeleteScopeCommand),
}

#[derive(Parser)]
pub(super) struct DeleteScopeCommand {
    /// Name of the scope to delete
    #[clap(short, long)]
    name: String,
}

impl<TCredStore: CredentialStoreTrait> command_executor::CommandExecutorTrait<TCredStore>
    for ScopeCommand
{
    fn execute(&self, ctx: &mut command_context::CommandContext<TCredStore>) -> miette::Result<()> {
        let active_token = crate::auth::token_refresh::get_active_token(ctx)?;
        let function_runtime_url = &ctx.config.function_runtime_url;

        match self {
            ScopeCommand::List => list::execute(&active_token, function_runtime_url),
            ScopeCommand::Delete(delete_command) => {
                delete::execute(&active_token, function_runtime_url, &delete_command.name)
            }
        }
    }
}
