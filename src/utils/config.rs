use crate::utils::errors::Result;

#[derive(Debug, Clone)]
pub struct Config {
    pub github_token: String,
    pub download_path: String,
    pub organization: String,
    pub batch_size: usize,
    pub timeout_seconds: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        use crate::modules::load_env;
        let (token, path, org, _) = load_env::load_vars()?;

        Ok(Self {
            github_token: token,
            download_path: path,
            organization: org,
            batch_size: 8,
            timeout_seconds: 30,
        })
    }
}
