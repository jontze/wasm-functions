pub(crate) struct AppConfig {
    pub openid_connect: OpenIdConnectConfig,
}

pub(crate) struct OpenIdConnectConfig {
    pub jwks_url: String,
    pub issuer: String,
    pub audience: String,
}

pub(crate) trait Loader {
    fn load() -> Self;
}

impl Loader for OpenIdConnectConfig {
    fn load() -> Self {
        let issuer = std::env::var("OIDC_ISSUER").expect("OIDC_ISSUER is not set");
        let jwks_url = std::env::var("OIDC_JWKS_URI").expect("OIDC_JWKS_URI is not set");
        let client_id = std::env::var("OIDC_CLIENT_ID").expect("OIDC_CLIENT_ID is not set");

        Self {
            jwks_url,
            issuer,
            audience: client_id,
        }
    }
}

impl Loader for AppConfig {
    fn load() -> Self {
        Self {
            openid_connect: OpenIdConnectConfig::load(),
        }
    }
}
