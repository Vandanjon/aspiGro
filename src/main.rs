mod downloader;
mod header;
mod http_client;
mod load_env;
mod models;
mod repo_fetcher;
mod running_mode;

#[tokio::main]
async fn main() {
    header::print_header();

    println!("1/5 Chargement des variables d'environnement...");
    let (github_token, dl_folder_path, organization) = load_env::load_vars();

    println!("2/5 Configuration de la recherche...");
    let keyword = running_mode::configure_search();

    println!("3/5 Scan des repos de l'organisation...");
    let repos = repo_fetcher::scan_and_fetch(&github_token, &organization, keyword).await;

    println!("4/5 Téléchargement des repos...");
    if !repos.is_empty() {
        println!("   📁 Dossier de téléchargement : {}", dl_folder_path);
        downloader::download_repositories(&repos, &dl_folder_path).await;
    } else {
        println!("   ⚠️  Aucun repository à télécharger");
    }

    println!("5/5 Terminé !");
    println!("   ✅ {} repositories scannés", repos.len());
}
