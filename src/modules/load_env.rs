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

/// Compte le nombre de variables dans .env.sample
fn count_env_variables() -> usize {
    match fs::read_to_string(".env.sample") {
        Ok(content) => content
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with('#') && trimmed.contains('=')
            })
            .count(),
        Err(_) => 3, // Valeur par défaut si .env.sample n'existe pas
    }
}

pub fn load_vars() -> (String, String, String, usize) {
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
    let var_count = count_env_variables();

    (token, path, org, var_count)
}
