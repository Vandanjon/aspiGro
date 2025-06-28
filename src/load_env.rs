use dotenvy::dotenv;
use std::{env, fs, process};

/// Récupère une variable d'environnement ou termine le programme
fn get_env_var(var_name: &str) -> String {
    env::var(var_name)
        .map_err(|_| format!("❌ {} non défini dans .env", var_name))
        .and_then(|val| {
            if val.trim().is_empty() {
                Err(format!("❌ {} vide dans .env", var_name))
            } else {
                Ok(val)
            }
        })
        .unwrap_or_else(|msg| {
            eprintln!("{}", msg);
            process::exit(1);
        })
}

/// Expansion du tilde (~) vers le répertoire HOME
fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") {
        env::var("HOME")
            .map(|home| path.replacen("~", &home, 1))
            .unwrap_or_else(|_| {
                eprintln!("❌ Variable HOME introuvable");
                process::exit(1);
            })
    } else if path == "~" {
        env::var("HOME").unwrap_or_else(|_| {
            eprintln!("❌ Variable HOME introuvable");
            process::exit(1);
        })
    } else {
        path.to_string()
    }
}

pub fn load_vars() -> (String, String, String) {
    // Vérification existence .env
    if fs::metadata(".env").is_err() {
        eprintln!("❌ Fichier .env manquant");
        eprintln!("Créez un .env avec : GITHUB_TOKEN, DL_FOLDER_PATH, ORGANIZATION_TO_FETCH");
        process::exit(1);
    }

    dotenv().unwrap_or_else(|e| {
        eprintln!("❌ Erreur chargement .env : {}", e);
        process::exit(1);
    });

    let token = get_env_var("GITHUB_TOKEN");
    let path = expand_tilde(&get_env_var("DL_FOLDER_PATH"));
    let org = get_env_var("ORGANIZATION_TO_FETCH");

    (token, path, org)
}
