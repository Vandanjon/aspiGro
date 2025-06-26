mod load_env;
mod running_mode;

fn main() {
    println!("\n                🚀 AspiGro                 ");
    println!("==========================================================");
    println!("l'outil pour aspirer des repos comme un pro !");
    println!("==========================================================\n");

    // Chargement des variables d'environnement
    let (github_token, dl_folder_path) = load_env::load_vars();

    // Configuration de la recherche (mode + mot-clé si nécessaire)
    let keyword = running_mode::configure_search();

    // TODO: préparation de l'URL de l'API GitHub
    // TODO: exécution de la requête HTTP
    // TODO: résumé des résultats
}
