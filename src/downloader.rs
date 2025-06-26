use crate::models::Repo;
use reqwest::Client;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

pub async fn download_repositories(
    client: &Client,
    token: &str,
    organization: &str,
    repos: Vec<Repo>,
    target_dir: PathBuf,
    keyword: Option<&str>,
) {
    let total_repos = if keyword.is_some() {
        repos.len()
    } else {
        repos.len().min(500)
    };

    println!("\n📊 Résumé :");
    if let Some(kw) = keyword {
        println!("   🔍 Mot-clé : '{}'", kw);
        println!("   📦 {} repositories trouvés correspondants", total_repos);
    } else {
        println!("   📦 {} repositories à télécharger (500 max)", total_repos);
    }

    println!("   📁 Téléchargement dans : {:?}", target_dir);

    if total_repos > 10 {
        let confirm = get_user_input(&format!(
            "\n⚠️  Vous allez télécharger {} repositories. Continuer ? (o/N) : ",
            total_repos
        ));
        if !matches!(confirm.to_lowercase().as_str(), "o" | "oui" | "y" | "yes") {
            println!("❌ Opération annulée par l'utilisateur");
            std::process::exit(0);
        }
    }

    println!("\n🚀 Début du téléchargement...");

    let repos_to_download: Vec<_> = if keyword.is_some() {
        repos.into_iter().collect()
    } else {
        repos.into_iter().take(500).collect()
    };

    for (index, repo) in repos_to_download.iter().enumerate() {
        let zip_url = format!(
            "https://api.github.com/repos/{}/{}/zipball/{}",
            organization, repo.name, repo.default_branch
        );

        println!(
            "⬇️  [{}/{}] Téléchargement de '{}'...",
            index + 1,
            total_repos,
            repo.name
        );

        let resp = client.get(&zip_url).bearer_auth(token).send().await;

        let resp = match resp {
            Ok(r) => match r.error_for_status() {
                Ok(resp) => resp,
                Err(e) => {
                    eprintln!("   ❌ Erreur téléchargement '{}': {}", repo.name, e);
                    continue;
                }
            },
            Err(e) => {
                eprintln!("   ❌ Erreur téléchargement '{}': {}", repo.name, e);
                continue;
            }
        };

        let out_path = target_dir.join(format!("{}.zip", repo.name));
        let out_file = File::create(&out_path);

        let mut out_file = match out_file {
            Ok(f) => f,
            Err(e) => {
                eprintln!("   ❌ Impossible de créer '{:?}': {}", out_path, e);
                continue;
            }
        };

        let mut response_bytes = match resp.bytes().await {
            Ok(bytes) => std::io::Cursor::new(bytes),
            Err(e) => {
                eprintln!("   ❌ Erreur lecture '{:?}': {}", out_path, e);
                continue;
            }
        };

        if let Err(e) = copy(&mut response_bytes, &mut out_file) {
            eprintln!("   ❌ Erreur écriture '{:?}': {}", out_path, e);
            let _ = std::fs::remove_file(&out_path);
        } else {
            println!("   ✅ '{}' téléchargé avec succès", repo.name);
        }
    }

    println!(
        "\n🎉 Téléchargement terminé ! {} repositories traités",
        total_repos
    );

    if let Some(kw) = keyword {
        println!("🔍 Filtrage appliqué : '{}'", kw);
    }
    println!("📁 Fichiers sauvegardés dans : {:?}", target_dir);
}

fn get_user_input(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture de l'entrée");
    input.trim().to_string()
}
