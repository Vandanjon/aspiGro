use dotenvy::dotenv;
use std::{env, fs};

pub fn load_github_token() -> Option<String> {
    if !fs::metadata(".env").is_ok() {
        eprintln!("Erreur : Le fichier .env est manquant.");
        eprintln!("Assurez-vous d'avoir un fichier .env à la racine du projet.");
        return None;
    }

    if let Err(e) = dotenv() {
        eprintln!("Erreur lors du chargement du fichier .env : {}", e);
        return None;
    }

    match env::var("GITHUB_TOKEN") {
        Ok(token) => {
            if token.trim().is_empty() {
                eprintln!("Erreur : GITHUB_TOKEN est vide. Vérifiez votre fichier .env.");
                None
            } else {
                Some(token)
            }
        }
        Err(_) => {
            eprintln!("Erreur : GITHUB_TOKEN n'est pas défini.");
            eprintln!("Assurez-vous que la variable d'environnement GITHUB_TOKEN est définie dans le fichier .env.");
            None
        }
    }
}
