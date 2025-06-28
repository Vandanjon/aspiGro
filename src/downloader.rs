use crate::models::Repo;
use futures::future::join_all;
use std::path::Path;
use std::process::Stdio;
use tokio::fs;
use tokio::time::Instant;

pub async fn download_repositories(repos: &[Repo], target_dir: &str) {
    let start = Instant::now();
    println!("\n🚀 Clonage de {} repos dans {}", repos.len(), target_dir);

    fs::create_dir_all(target_dir).await.unwrap_or_else(|e| {
        eprintln!("❌ Création dossier impossible : {}", e);
        std::process::exit(1);
    });

    const BATCH_SIZE: usize = 8;
    let mut stats = (0, 0); // (succès, échecs)

    // Traitement par chunks pour optimiser les ressources
    for (batch_num, chunk) in repos.chunks(BATCH_SIZE).enumerate() {
        println!("   📦 Batch {} - {} repos", batch_num + 1, chunk.len());

        let futures = chunk.iter().enumerate().map(|(i, repo)| {
            let path = format!("{}/{}", target_dir, repo.name);
            let url = repo.html_url.clone();
            async move { clone_repo(&url, &path, i + 1, chunk.len()).await }
        });

        let results = join_all(futures).await;
        let (success, fail) = results.iter().fold(
            (0, 0),
            |(s, f), &ok| {
                if ok {
                    (s + 1, f)
                } else {
                    (s, f + 1)
                }
            },
        );

        stats.0 += success;
        stats.1 += fail;
    }

    let duration = start.elapsed();
    println!(
        "\n✅ Terminé en {:.1}s - {} succès, {} échecs",
        duration.as_secs_f32(),
        stats.0,
        stats.1
    );
}

async fn clone_repo(url: &str, path: &str, num: usize, total: usize) -> bool {
    if Path::new(path).exists() {
        println!(
            "      ⏭️  {}/{} {} (existe)",
            num,
            total,
            Path::new(path).file_name().unwrap().to_string_lossy()
        );
        return true;
    }

    let success = std::process::Command::new("git")
        .args(["clone", "--no-single-branch", url, path])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    let name = Path::new(path).file_name().unwrap().to_string_lossy();
    if success {
        println!("      ✅ {}/{} {} (toutes branches)", num, total, name);
    } else {
        println!("      ❌ {}/{} {} (échec)", num, total, name);
    }

    success
}
