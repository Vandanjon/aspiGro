use crate::models::Repo;
use reqwest::Client;
use std::io::{self, Write};

pub async fn fetch_repositories(client: &Client, token: &str, keyword: Option<&str>) -> Vec<Repo> {
    println!("\n📡 Récupération de la liste des repositories...");

    let mut all_repos = Vec::new();
    let max_pages = if keyword.is_some() { 50 } else { 5 };

    for page in 1..=max_pages {
        let url = format!(
            "https://api.github.com/orgs/WildCodeSchool/repos?per_page=100&sort=pushed&direction=desc&page={}",
            page
        );

        let resp = match client.get(&url).bearer_auth(token).send().await {
            Ok(response) => match response.error_for_status() {
                Ok(resp) => resp,
                Err(e) => {
                    eprintln!("\n❌ Erreur API page {}: {}", page, e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("\n❌ Erreur connexion page {}: {}", page, e);
                std::process::exit(1);
            }
        };

        let repos: Vec<Repo> = match resp.json().await {
            Ok(repos) => repos,
            Err(e) => {
                eprintln!("\n❌ Impossible de parser la page {}: {}", page, e);
                std::process::exit(1);
            }
        };

        if repos.is_empty() {
            if keyword.is_some() {
                update_progress(max_pages, all_repos.len(), true);
                println!("\n   ✅ Recherche terminée");
            } else {
                println!("   📄 Page {} vide - fin de la récupération", page);
            }
            break;
        }

        let filtered_repos: Vec<Repo> = if let Some(kw) = keyword {
            repos
                .into_iter()
                .filter(|repo| repo.name.to_lowercase().contains(kw))
                .collect()
        } else {
            repos
        };

        all_repos.extend(filtered_repos);

        if keyword.is_some() {
            update_progress(page, all_repos.len(), true);
        } else {
            println!(
                "   📄 Page {} : {} repositories récupérés",
                page,
                all_repos.len()
            );
        }

        // Limite pour éviter une boucle infinie en mode normal
        if keyword.is_none() && all_repos.len() >= 500 {
            break;
        }
    }

    if keyword.is_some() {
        println!();
    }

    if all_repos.is_empty() {
        if let Some(kw) = keyword {
            println!("❌ Aucun repository trouvé avec le mot-clé '{}'", kw);
        } else {
            println!("❌ Aucun repository trouvé");
        }
        std::process::exit(1);
    }

    all_repos
}

fn update_progress(page: usize, found: usize, is_filtering: bool) {
    if is_filtering {
        let max_pages = 50;
        let progress = (page as f32 / max_pages as f32 * 100.0) as usize;
        print!(
            "\r🔍 Progression: {}% - {} repositories trouvés",
            progress, found
        );
        io::stdout().flush().unwrap();
    }
}
