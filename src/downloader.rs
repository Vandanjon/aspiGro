use crate::models::Repo;
use futures::future::join_all;
use std::path::Path;
use std::process::Stdio;
use tokio::fs;
use tokio::time::Instant;

pub async fn download_repositories(repos: &[Repo], target_dir: &str) {
    let start_time = Instant::now();

    println!("\n🚀 Téléchargement ultra-rapide avec git clone...");
    println!("   📦 {} repositories à cloner", repos.len());
    println!("   📁 Dossier cible : {}", target_dir);

    // Créer le dossier cible s'il n'existe pas
    if let Err(e) = fs::create_dir_all(target_dir).await {
        eprintln!("❌ Impossible de créer le dossier {}: {}", target_dir, e);
        return;
    }

    // Pool de workers pour éviter de surcharger le système
    let max_concurrent = 8; // Ajustable selon votre machine
    let mut current_batch = 0;
    let total_repos = repos.len();
    let mut successful_clones = 0;
    let mut failed_clones = 0;

    println!(
        "   ⚡ Clonage par batch de {} repos en parallèle...",
        max_concurrent
    );

    // Traitement par batches pour éviter trop de processus simultanés
    for chunk in repos.chunks(max_concurrent) {
        current_batch += 1;
        let batch_size = chunk.len();

        println!(
            "\n   📦 Batch {}/{} - {} repos...",
            current_batch,
            (total_repos + max_concurrent - 1) / max_concurrent,
            batch_size
        );

        // Lancer tous les clones du batch en parallèle
        let clone_futures: Vec<_> = chunk
            .iter()
            .enumerate()
            .map(|(i, repo)| {
                let target_dir_path = target_dir.to_string();
                let repo_name = repo.name.clone();
                let repo_url = repo.html_url.clone();
                async move {
                    let repo_path = Path::new(&target_dir_path).join(&repo_name);
                    clone_repository(&repo_url, &repo_path, i + 1, batch_size).await
                }
            })
            .collect();

        // Attendre que tous les clones du batch se terminent
        let results = join_all(clone_futures).await;

        // Compter les succès/échecs
        for result in results {
            if result {
                successful_clones += 1;
            } else {
                failed_clones += 1;
            }
        }
    }

    let duration = start_time.elapsed();

    println!(
        "\n✅ Téléchargement terminé en {:.2}s !",
        duration.as_secs_f64()
    );
    println!(
        "   ✅ {} repositories clonés avec succès",
        successful_clones
    );

    if failed_clones > 0 {
        println!("   ⚠️  {} échecs de clonage", failed_clones);
    }

    let repos_per_second = total_repos as f64 / duration.as_secs_f64();
    println!(
        "   ⚡ Vitesse moyenne : {:.1} repos/seconde",
        repos_per_second
    );
}

async fn clone_repository(
    clone_url: &str,
    target_path: &Path,
    repo_num: usize,
    batch_size: usize,
) -> bool {
    // Vérifier si le repo existe déjà
    if target_path.exists() {
        println!(
            "      ⏭️  {}/{} {} (déjà présent)",
            repo_num,
            batch_size,
            target_path.file_name().unwrap().to_string_lossy()
        );
        return true;
    }

    // Exécuter git clone avec toutes les branches
    let mut cmd = std::process::Command::new("git");
    cmd.arg("clone")
        .arg("--no-single-branch") // Important : récupère toutes les branches
        .arg(clone_url)
        .arg(target_path)
        .stdout(Stdio::null()) // Réduire le bruit
        .stderr(Stdio::null());

    match cmd.status() {
        Ok(status) if status.success() => {
            println!(
                "      ✅ {}/{} {} (toutes branches)",
                repo_num,
                batch_size,
                target_path.file_name().unwrap().to_string_lossy()
            );
            true
        }
        Ok(_) => {
            println!(
                "      ❌ {}/{} {} (échec git clone)",
                repo_num,
                batch_size,
                target_path.file_name().unwrap().to_string_lossy()
            );
            false
        }
        Err(e) => {
            println!(
                "      ❌ {}/{} {} (erreur: {})",
                repo_num,
                batch_size,
                target_path.file_name().unwrap().to_string_lossy(),
                e
            );
            false
        }
    }
}
