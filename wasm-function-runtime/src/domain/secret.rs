use uuid::Uuid;

#[allow(unused)]
pub(crate) struct Secret {
    pub uuid: Uuid,
    pub name: String,
    pub encrypted_value: String,
}

impl From<entity::secret::Model> for Secret {
    fn from(model: entity::secret::Model) -> Self {
        Self {
            uuid: model.id,
            name: model.name,
            encrypted_value: model.value,
        }
    }
}

#[allow(unused)]
pub(crate) trait SecretDecrpytTrait {
    /// Returns the decrypted value of the secret
    /// The key is used to decrypt the value.
    fn decrypt(&self, key: &str) -> String;
}

impl SecretDecrpytTrait for Secret {
    fn decrypt(&self, _key: &str) -> String {
        todo!("Decrypt the value using the key with some algorithm")
    }
}
