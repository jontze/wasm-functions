use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

impl From<crate::auth::model::TokenResponse> for Tokens {
    fn from(token_response: crate::auth::model::TokenResponse) -> Self {
        Self {
            access_token: token_response.access_token.expect("No access token found"),
            refresh_token: token_response
                .refresh_token
                .expect("No refresh token found"),
        }
    }
}

pub(crate) trait CredentialStoreTrait {
    fn save(&mut self, tokens: &Tokens) -> miette::Result<()>;
    fn load(&mut self) -> miette::Result<()>;
    fn get_tokens(&self) -> miette::Result<&Tokens>;
    fn delete(&mut self) -> miette::Result<()>;
}

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");
const CREDENTIALS_STORE_FILE: &str = ".auth.json";

pub(crate) struct CredentialStore {
    file: std::path::PathBuf,
    tokens: Option<Tokens>,
}

impl Default for CredentialStore {
    fn default() -> Self {
        Self {
            file: dirs::config_local_dir()
                .expect("Could not determine local config directory")
                .join(APPLICATION_NAME)
                .join(CREDENTIALS_STORE_FILE),
            tokens: None,
        }
    }
}

impl CredentialStoreTrait for CredentialStore {
    fn save(&mut self, tokens: &Tokens) -> miette::Result<()> {
        let json_bytes = serde_json::to_vec(tokens).into_diagnostic()?;

        // Ensurethe directory exists
        if let Some(parent) = self.file.parent() {
            std::fs::create_dir_all(parent).into_diagnostic()?;
        }

        std::fs::write(&self.file, &json_bytes).into_diagnostic()?;
        self.tokens = Some(tokens.to_owned());
        Ok(())
    }

    fn load(&mut self) -> miette::Result<()> {
        let json = std::fs::read_to_string(&self.file).into_diagnostic()?;
        let tokens = serde_json::from_str::<Tokens>(&json).into_diagnostic()?;
        self.tokens = Some(tokens);
        Ok(())
    }

    fn get_tokens(&self) -> miette::Result<&Tokens> {
        self.tokens.as_ref().ok_or_else(|| {
            miette::Report::from_err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No credentials found",
            ))
        })
    }

    fn delete(&mut self) -> miette::Result<()> {
        match std::fs::remove_file(&self.file) {
            Ok(_) => {
                self.tokens = None;
                Ok(())
            }
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(err).into_diagnostic()
                }
            }
        }
    }
}
