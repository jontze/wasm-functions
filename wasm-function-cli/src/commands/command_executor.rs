use crate::cred_store::CredentialStoreTrait;

pub(crate) trait CommandExecutorTrait<TCredStore: CredentialStoreTrait> {
    fn execute(
        &self,
        ctx: &mut super::command_context::CommandContext<TCredStore>,
    ) -> miette::Result<()>;
}
