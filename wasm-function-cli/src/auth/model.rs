use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
pub(crate) struct DeviceCodeResponse {
    pub(crate) device_code: String,
    pub(crate) user_code: String,
    pub(crate) verification_uri: String,
    pub(crate) expires_in: u32,
    pub(crate) interval: u32,
    pub(crate) message: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: Option<usize>,
    pub scope: Option<String>,
}
