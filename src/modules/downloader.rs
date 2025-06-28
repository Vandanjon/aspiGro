use crate::utils::models::Repo;
use crate::utils::ui::ProgressBar;
use futures::future::join_all;
use std::path::Path;
use std::process::Stdio;
use std::sync::atomic::Ordering;
use tokio::fs;

pub async fn download_repositories(repos: &[Repo], target_dir: &str) {
    fs::create_dir_all(target_dir).await.unwrap_or_else(|e| {
        eprintln!("❌ Création dossier impossible : {}", e);
        std::process::exit(1);
    });

    let progress_bar = ProgressBar::new(repos.len());
    let counter = progress_bar.get_counter();

    progress_bar.update();

    const BATCH_SIZE: usize = 8;

    // Traitement par chunks pour optimiser les ressources
    for chunk in repos.chunks(BATCH_SIZE) {
        let futures = chunk.iter().map(|repo| {
            let path = format!("{}/{}", target_dir, repo.name);
            let url = repo.html_url.clone();
            let counter_clone = counter.clone();
            async move {
                let result = clone_repo_with_fallback(&url, &path).await;
                counter_clone.fetch_add(1, Ordering::Relaxed);
                result
            }
        });

        join_all(futures).await;

        progress_bar.update();
    }

    progress_bar.finish();
}

async fn clone_repo(url: &str, path: &str) -> bool {
    if Path::new(path).exists() {
        return true;
    }

    // Configuration Git pour éviter les prompts de mot de passe
    let mut cmd = std::process::Command::new("git");
    cmd.args([
        "clone",
        "--no-single-branch",
        "--quiet",
        "--no-checkout",
        url,
        path,
    ])
    .env("GIT_TERMINAL_PROMPT", "0")
    .env("GIT_ASKPASS", "true")
    .env("SSH_ASKPASS", "true")
    .stdout(Stdio::null())
    .stderr(Stdio::null());

    // Timeout de 30 secondes pour éviter les blocages
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        tokio::task::spawn_blocking(move || cmd.status().map(|s| s.success()).unwrap_or(false)),
    )
    .await;

    match result {
        Ok(Ok(success)) => {
            if success {
                std::process::Command::new("git")
                    .args(["-C", path, "checkout", "HEAD"])
                    .env("GIT_TERMINAL_PROMPT", "0")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            } else {
                false
            }
        }
        _ => false, // Timeout ou erreur
    }
}

/// Tente de cloner un repo avec différentes méthodes si nécessaire
async fn clone_repo_with_fallback(url: &str, path: &str) -> bool {
    if clone_repo(url, path).await {
        return true;
    }

    if url.starts_with("git@github.com:") {
        let https_url = url
            .replace("git@github.com:", "https://github.com/")
            .replace(".git", "");
        let https_url = if !https_url.ends_with(".git") {
            format!("{}.git", https_url)
        } else {
            https_url
        };

        if clone_repo(&https_url, path).await {
            return true;
        }
    }

    false
}
