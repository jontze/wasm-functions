use super::CredentialStoreTrait;

pub(super) fn execute<TCredStore: CredentialStoreTrait>(
    ctx: &mut super::command_context::CommandContext<TCredStore>,
) -> miette::Result<()> {
    ctx.cred_store.delete()?;
    Ok(())
}
