use reqwest::Client;
use std::time::Duration;

pub fn create_http_client() -> Client {
    Client::builder()
        .user_agent("aspigro/1.0")
        .timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(30))
        .http2_prior_knowledge()
        .build()
        .expect("❌ Échec création client HTTP")
}
