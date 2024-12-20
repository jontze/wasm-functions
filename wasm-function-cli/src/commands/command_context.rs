use crate::cred_store::CredentialStoreTrait;

#[derive(Debug)]
pub(crate) struct CommandContext<'a, TCredStore: CredentialStoreTrait> {
    pub config: &'a crate::config::Config,
    pub cred_store: &'a mut TCredStore,
}
