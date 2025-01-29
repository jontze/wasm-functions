pub(crate) struct AppConfig {
    pub openid_connect: OpenIdConnectConfig,
    pub minio_storage: Option<MinioStorageConfig>,
    pub azure_storage: Option<AzureStorageConfig>,
    pub hetzner_storage: Option<HetznerStorageConfig>,
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

struct AzureStorageConfigBuilder {
    account: Option<String>,
    access_key: Option<String>,
    container: Option<String>,
}

impl AzureStorageConfigBuilder {
    fn new() -> Self {
        Self {
            account: None,
            access_key: None,
            container: None,
        }
    }

    fn with_account(&mut self, account: String) {
        self.account = Some(account);
    }

    fn with_access_key(&mut self, access_key: String) {
        self.access_key = Some(access_key);
    }

    fn with_container(&mut self, container: String) {
        self.container = Some(container);
    }

    fn build(self) -> Option<AzureStorageConfig> {
        let account = self.account?;
        let access_key = self.access_key?;
        let container = self.container?;

        Some(AzureStorageConfig {
            account,
            access_key,
            container,
        })
    }
}

pub(crate) struct AzureStorageConfig {
    pub account: String,
    pub access_key: String,
    pub container: String,
}

struct HetznerStorageConfigBuilder {
    access_key: Option<String>,
    secret_key: Option<String>,
    bucket_url: Option<String>,
    bucket_name: Option<String>,
    region: Option<String>,
}

impl HetznerStorageConfigBuilder {
    fn new() -> Self {
        Self {
            access_key: None,
            secret_key: None,
            bucket_url: None,
            bucket_name: None,
            region: None,
        }
    }

    fn with_access_key(&mut self, access_key: String) {
        self.access_key = Some(access_key);
    }

    fn with_secret_key(&mut self, secret_key: String) {
        self.secret_key = Some(secret_key);
    }

    fn with_bucket_url(&mut self, bucket_url: String) {
        self.bucket_url = Some(bucket_url);
    }

    fn with_bucket_name(&mut self, bucket_name: String) {
        self.bucket_name = Some(bucket_name);
    }

    fn with_region(&mut self, region: String) {
        self.region = Some(region);
    }

    fn build(self) -> Option<HetznerStorageConfig> {
        let access_key = self.access_key?;
        let secret_key = self.secret_key?;
        let bucket_url = self.bucket_url?;
        let bucket_name = self.bucket_name?;
        let region = self.region?;

        Some(HetznerStorageConfig {
            access_key,
            secret_key,
            bucket_url,
            bucket_name,
            region,
        })
    }
}

pub(crate) struct HetznerStorageConfig {
    pub access_key: String,
    pub secret_key: String,
    pub bucket_url: String,
    pub bucket_name: String,
    pub region: String,
}

impl Loader for AppConfig {
    fn load() -> Self {
        // Tryto Build MinioStorageConfig
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

        // Try to build AzureStorageConfig
        let mut azure_storage_config_builder = AzureStorageConfigBuilder::new();
        if let Ok(account) = std::env::var("AZURE_STORAGE_ACCOUNT_NAME") {
            azure_storage_config_builder.with_account(account);
        }
        if let Ok(access_key) = std::env::var("AZURE_STORAGE_ACCOUNT_ACCESS_KEY") {
            azure_storage_config_builder.with_access_key(access_key);
        }
        if let Ok(container) = std::env::var("AZURE_STORAGE_ACCOUNT_BUCKET_NAME") {
            azure_storage_config_builder.with_container(container);
        }

        // Try to build HetznerStorageConfig
        let mut hetzner_storage_config_builder = HetznerStorageConfigBuilder::new();
        if let Ok(access_key) = std::env::var("HETZNER_BUCKET_ACCESS_KEY") {
            hetzner_storage_config_builder.with_access_key(access_key);
        }
        if let Ok(secret_key) = std::env::var("HETZNER_BUCKET_ACCESS_SECRET_KEY") {
            hetzner_storage_config_builder.with_secret_key(secret_key);
        }
        if let Ok(bucket) = std::env::var("HETZNER_BUCKET_URL") {
            hetzner_storage_config_builder.with_bucket_url(bucket);
        }
        if let Ok(bucket) = std::env::var("HETZNER_BUCKET_NAME") {
            hetzner_storage_config_builder.with_bucket_name(bucket);
        }
        if let Ok(region) = std::env::var("HETZNER_BUCKET_REGION") {
            hetzner_storage_config_builder.with_region(region);
        }

        let minio_storage = minio_storage_config_builder.build();
        let azure_storage = azure_storage_config_builder.build();
        let hetzner_storage = hetzner_storage_config_builder.build();

        // Panic if multiple storage backends are configured
        if minio_storage.is_some() && azure_storage.is_some() && hetzner_storage.is_some() {
            panic!("Multiple storage backends are configured");
        }

        Self {
            openid_connect: OpenIdConnectConfig::load(),
            minio_storage,
            azure_storage,
            hetzner_storage,
        }
    }
}
