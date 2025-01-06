use clap::{Parser, Subcommand};

use super::{command_context, command_executor, CredentialStoreTrait};

mod add;
mod delete;
mod edit;
mod list;

#[derive(Subcommand)]
pub(super) enum VariableCommand {
    /// Add a variable to a scope
    Add(AddVariableCommand),
    /// Delete a variable from a scope
    Delete(DeleteVariableCommand),
    /// Edit a variable of a scope
    Edit(EditVariableCommand),
    /// List all variables of a scope
    List(ListVariableCommand),
}

#[derive(Parser)]
pub(super) struct AddVariableCommand {
    /// Name of the scope to add the variable to
    #[clap(short, long)]
    scope_name: String,
    /// Name of the variable to add
    #[clap(short, long)]
    name: String,
    /// Value of the variable to add
    #[clap(short, long)]
    value: String,
}

#[derive(Parser)]
pub(super) struct EditVariableCommand {
    /// Name of the scope the variable belongs to
    #[clap(short, long)]
    scope_name: String,
    /// Unique identifier of the variable to edit
    #[clap(short, long)]
    id: String,
    /// New value of the variable
    #[clap(short, long)]
    value: String,
}

#[derive(Parser)]
pub(super) struct DeleteVariableCommand {
    /// Name of the scope the variable belongs to
    #[clap(short, long)]
    scope_name: String,
    /// Unique identifier of the variable to delete
    #[clap(short, long)]
    id: String,
}

#[derive(Parser)]
pub(super) struct ListVariableCommand {
    /// Name of the scope the variables belong to
    #[clap(short, long)]
    scope_name: String,
}

impl<TCredStore: CredentialStoreTrait> command_executor::CommandExecutorTrait<TCredStore>
    for VariableCommand
{
    fn execute(&self, ctx: &mut command_context::CommandContext<TCredStore>) -> miette::Result<()> {
        let active_token = crate::auth::token_refresh::get_active_token(ctx)?;
        let function_runtime_url = &ctx.config.function_runtime_url;

        match self {
            VariableCommand::Add(add_command) => add::execute(
                &active_token,
                function_runtime_url,
                &add_command.scope_name,
                &add_command.name,
                &add_command.value,
            ),
            VariableCommand::Delete(delete_command) => delete::execute(
                &active_token,
                function_runtime_url,
                &delete_command.scope_name,
                &delete_command.id,
            ),
            VariableCommand::Edit(edit_command) => edit::execute(
                &active_token,
                function_runtime_url,
                &edit_command.scope_name,
                &edit_command.id,
                &edit_command.value,
            ),
            VariableCommand::List(list_command) => list::execute(
                &active_token,
                function_runtime_url,
                &list_command.scope_name,
            ),
        }
    }
}
