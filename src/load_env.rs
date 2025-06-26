use dotenvy::dotenv;
use std::{env, fs};

fn get_env_var(var_name: &str) -> String {
    match env::var(var_name) {
        Ok(value) => {
            if value.trim().is_empty() {
                eprintln!("❌ {} est vide. Vérifiez votre fichier .env.", var_name);
                std::process::exit(1);
            } else {
                value
            }
        }
        Err(_) => {
            eprintln!("❌ {} n'est pas défini dans le fichier .env.", var_name);
            std::process::exit(1);
        }
    }
}

pub fn load_vars() -> (String, String, String) {
    if !fs::metadata(".env").is_ok() {
        eprintln!("❌ Le fichier .env est manquant.");
        eprintln!(
            "Créez un fichier .env avec GITHUB_TOKEN, DL_FOLDER_PATH et ORGANIZATION_TO_FETCH"
        );
        std::process::exit(1);
    }

    if let Err(e) = dotenv() {
        eprintln!("❌ Erreur lors du chargement du fichier .env : {}", e);
        std::process::exit(1);
    }

    let github_token = get_env_var("GITHUB_TOKEN");
    let dl_folder_path = get_env_var("DL_FOLDER_PATH");
    let organization = get_env_var("ORGANIZATION_TO_FETCH");

    (github_token, dl_folder_path, organization)
}
