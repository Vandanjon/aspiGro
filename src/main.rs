mod downloader;
mod http_client;
mod load_env;
mod models;
mod repo_fetcher;
mod running_mode;

#[tokio::main]
async fn main() {
    println!("\n                🚀 AspiGro                 ");
    println!("==========================================================");
    println!("l'outil pour aspirer des repos comme un pro !");
    println!("==========================================================\n");

    // Chargement des variables d'environnement
    let (github_token, dl_folder_path, organization) = load_env::load_vars();

    // Configuration de la recherche (mode + mot-clé si nécessaire)
    let keyword = running_mode::configure_search();

    // Création du client HTTP
    let client = http_client::create_http_client();

    // Récupération des repositories
    let repos =
        repo_fetcher::fetch_repositories(&client, &github_token, &organization, keyword.as_deref())
            .await;

    // Utilisation du dossier configuré dans .env
    let target_dir = std::path::PathBuf::from(&dl_folder_path);

    // Vérification que le dossier existe ou création
    if let Err(e) = std::fs::create_dir_all(&target_dir) {
        eprintln!("❌ Impossible de créer le dossier {:?}: {}", target_dir, e);
        std::process::exit(1);
    }

    // Téléchargement des repositories
    downloader::download_repositories(
        &client,
        &github_token,
        &organization,
        repos,
        target_dir,
        keyword.as_deref(),
    )
    .await;
}
