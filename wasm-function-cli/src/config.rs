#[derive(Debug)]
pub(crate) struct Config {
    pub function_runtime_url: String,
    pub tenant_url: String,
    pub tenant_id: String,
    pub client_id: String,
    pub scopes: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        dotenv::dotenv().ok();

        Ok(Self {
            function_runtime_url: std::env::var("FUNCTION_RUNTIME_URL")?,
            tenant_url: std::env::var("TENANT_URL")?,
            tenant_id: std::env::var("TENANT_ID")?,
            client_id: std::env::var("CLIENT_ID")?,
            scopes: std::env::var("SCOPES")?,
        })
    }
}
