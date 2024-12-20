use super::CredentialStoreTrait;

pub(super) fn execute<TCredStore: CredentialStoreTrait>(
    ctx: &mut super::command_context::CommandContext<TCredStore>,
) -> miette::Result<()> {
    let tokens = crate::auth::login::login(ctx.config)?;
    ctx.cred_store.save(&tokens.into())?;
    Ok(())
}
