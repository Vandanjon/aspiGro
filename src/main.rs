mod downloader;
mod http_client;
mod load_env;
mod models;
mod repo_fetcher;
mod running_mode;

use dirs;

#[tokio::main]
async fn main() {
    println!("\n                🚀 AspiGro                 ");
    println!("==========================================================");
    println!("l'outil pour aspirer des repos comme un pro !");
    println!("==========================================================\n");

    // Chargement des variables d'environnement
    let (github_token, _dl_folder_path) = load_env::load_vars();

    // Configuration de la recherche (mode + mot-clé si nécessaire)
    let keyword = running_mode::configure_search();

    // Création du client HTTP
    let client = http_client::create_http_client();

    // Récupération des repositories
    let repos = repo_fetcher::fetch_repositories(&client, &github_token, keyword.as_deref()).await;

    // Détermination du dossier de téléchargement
    let target_dir = dirs::download_dir().unwrap_or_else(|| {
        println!("⚠️  Dossier de téléchargement introuvable, utilisation du répertoire courant");
        std::path::PathBuf::from(".")
    });

    // Téléchargement des repositories
    downloader::download_repositories(
        &client,
        &github_token,
        repos,
        target_dir,
        keyword.as_deref(),
    )
    .await;
}
