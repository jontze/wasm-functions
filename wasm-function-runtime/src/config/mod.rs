pub(crate) struct AppConfig {
    pub openid_connect: OpenIdConnectConfig,
    pub minio_storage: Option<MinioStorageConfig>,
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

struct MinioStorageConfigBuilder {
    endpoint: Option<String>,
    access_key: Option<String>,
    secret_key: Option<String>,
    bucket: Option<String>,
}

impl MinioStorageConfigBuilder {
    fn new() -> Self {
        Self {
            endpoint: None,
            access_key: None,
            secret_key: None,
            bucket: None,
        }
    }

    fn with_endpoint(&mut self, endpoint: String) {
        self.endpoint = Some(endpoint);
    }

    fn with_access_key(&mut self, access_key: String) {
        self.access_key = Some(access_key);
    }

    fn with_secret_key(&mut self, secret_key: String) {
        self.secret_key = Some(secret_key);
    }

    fn with_bucket(&mut self, bucket: String) {
        self.bucket = Some(bucket);
    }

    fn build(self) -> Option<MinioStorageConfig> {
        let endpoint = self.endpoint?;
        let access_key = self.access_key?;
        let secret_key = self.secret_key?;
        let bucket = self.bucket?;

        Some(MinioStorageConfig {
            endpoint,
            access_key,
            secret_key,
            bucket,
        })
    }
}

pub(crate) struct MinioStorageConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
}

impl Loader for AppConfig {
    fn load() -> Self {
        let mut minio_storage_config_builder = MinioStorageConfigBuilder::new();
        if let Ok(endpoint) = std::env::var("MINIO_ENDPOINT") {
            minio_storage_config_builder.with_endpoint(endpoint);
        }
        if let Ok(access_key) = std::env::var("MINIO_ACCESS_KEY") {
            minio_storage_config_builder.with_access_key(access_key);
        }
        if let Ok(secret_key) = std::env::var("MINIO_SECRET_KEY") {
            minio_storage_config_builder.with_secret_key(secret_key);
        }
        if let Ok(bucket) = std::env::var("MINIO_BUCKET") {
            minio_storage_config_builder.with_bucket(bucket);
        }

        Self {
            openid_connect: OpenIdConnectConfig::load(),
            minio_storage: minio_storage_config_builder.build(),
        }
    }
}
