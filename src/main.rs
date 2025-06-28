mod app;
mod modules {
    pub mod downloader;
    pub mod http_client;
    pub mod load_env;
    pub mod repo_fetcher;
    pub mod running_mode;
}
mod utils {
    pub mod models;
    pub mod ui;
}

#[tokio::main]
async fn main() {
    let mut app = app::App::new();
    if let Err(e) = app.run().await {
        eprintln!("Erreur: {}", e);
        std::process::exit(1);
    }
}
