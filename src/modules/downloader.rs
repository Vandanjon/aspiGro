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
                let result = clone_repo(&url, &path).await;
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

    std::process::Command::new("git")
        .args(["clone", "--no-single-branch", url, path])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
