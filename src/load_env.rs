use dotenvy::dotenv;
use std::{env, fs};

fn get_env_var(var_name: &str) -> String {
    match env::var(var_name) {
        Ok(value) => {
            if value.trim().is_empty() {
                eprintln!(
                    "Erreur : {} est vide. Vérifiez votre fichier .env.",
                    var_name
                );
                std::process::exit(1);
            } else {
                value
            }
        }
        Err(_) => {
            eprintln!("Erreur : {} n'est pas défini.", var_name);
            eprintln!(
                "Assurez-vous que {} est défini dans le fichier .env.",
                var_name
            );
            std::process::exit(1);
        }
    }
}

pub fn load_vars() -> (String, String) {
    if !fs::metadata(".env").is_ok() {
        eprintln!("Erreur : Le fichier .env est manquant.");
        eprintln!("Assurez-vous d'avoir un fichier .env à la racine du projet.");
        std::process::exit(1);
    }

    if let Err(e) = dotenv() {
        eprintln!("Erreur lors du chargement du fichier .env : {}", e);
        std::process::exit(1);
    }

    let github_token = get_env_var("GITHUB_TOKEN");
    let dl_folder_path = get_env_var("DL_FOLDER_PATH");

    (github_token, dl_folder_path)
}
