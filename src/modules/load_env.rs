use crate::utils::errors::{AppError, Result};
use dotenvy::dotenv;
use std::{env, fs};

/// Récupère une variable d'environnement ou retourne une erreur
fn get_env_var(var_name: &str) -> Result<String> {
    env::var(var_name)
        .map_err(|_| AppError::EnvError(format!("{} non défini dans .env", var_name)))
        .and_then(|val| {
            if val.trim().is_empty() {
                Err(AppError::EnvError(format!("{} vide dans .env", var_name)))
            } else {
                Ok(val)
            }
        })
}

/// Expansion du tilde (~) vers le répertoire HOME
fn expand_tilde(path: &str) -> Result<String> {
    if path.starts_with("~/") {
        env::var("HOME")
            .map(|home| path.replacen("~", &home, 1))
            .map_err(|_| AppError::EnvError("Variable HOME introuvable".to_string()))
    } else if path == "~" {
        env::var("HOME").map_err(|_| AppError::EnvError("Variable HOME introuvable".to_string()))
    } else {
        Ok(path.to_string())
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

pub fn load_vars() -> Result<(String, String, String, usize)> {
    if fs::metadata(".env").is_err() {
        return Err(AppError::EnvError(
            "Fichier .env manquant. Créez un .env avec : GITHUB_TOKEN, DL_FOLDER_PATH, ORGANIZATION_TO_FETCH".to_string()
        ));
    }

    dotenv().map_err(|e| AppError::EnvError(format!("Erreur chargement .env : {}", e)))?;

    let token = get_env_var("GITHUB_TOKEN")?;
    let path = expand_tilde(&get_env_var("DL_FOLDER_PATH")?)?;
    let org = get_env_var("ORGANIZATION_TO_FETCH")?;
    let var_count = count_env_variables();

    Ok((token, path, org, var_count))
}
