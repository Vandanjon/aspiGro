use reqwest::Client;
use std::time::Duration;

pub fn create_http_client() -> Client {
    Client::builder()
        .user_agent("wcs-backup-script/2.0")
        .timeout(Duration::from_secs(30))
        .build()
        .expect("❌ Échec création client HTTP")
}
