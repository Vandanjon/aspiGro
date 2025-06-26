use crate::models::Repo;
use futures::future::join_all;
use reqwest::Client;
use std::io::{self, Write};
use tokio::time::Instant;

pub async fn fetch_repositories(
    client: &Client,
    token: &str,
    organization: &str,
    keyword: Option<&str>,
) -> Vec<Repo> {
    let start_time = Instant::now();
    println!("\n� Récupération rapide des repositories...");

    let max_pages = if keyword.is_some() { 10 } else { 5 };
    let concurrent_requests = 3; // Limité pour respecter les rate limits GitHub

    let mut all_repos = Vec::new();
    let mut current_page = 1;

    while current_page <= max_pages {
        let pages_to_fetch = std::cmp::min(concurrent_requests, max_pages - current_page + 1);
        let page_range: Vec<usize> = (current_page..current_page + pages_to_fetch).collect();

        let futures: Vec<_> = page_range
            .iter()
            .map(|&page| fetch_page(client, token, organization, page))
            .collect();

        let results = join_all(futures).await;

        let mut page_empty = false;
        let mut batch_repos = Vec::new();

        for (i, result) in results.into_iter().enumerate() {
            let page_num = page_range[i];
            match result {
                Ok(repos) => {
                    if repos.is_empty() {
                        println!("   📄 Page {} vide - arrêt", page_num);
                        page_empty = true;
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

                    batch_repos.extend(filtered_repos);
                    print!("📄{} ", page_num);
                    io::stdout().flush().unwrap();
                }
                Err(e) => {
                    eprintln!("\n❌ Erreur page {}: {}", page_num, e);
                    continue;
                }
            }
        }

        all_repos.extend(batch_repos);

        if page_empty || (keyword.is_none() && all_repos.len() >= 500) {
            break;
        }

        current_page += pages_to_fetch;
    }

    let duration = start_time.elapsed();
    println!(
        "\n✅ {} repos récupérés en {:.2}s",
        all_repos.len(),
        duration.as_secs_f64()
    );

    if all_repos.is_empty() {
        if let Some(kw) = keyword {
            println!("❌ Aucun repository trouvé avec le mot-clé '{}'", kw);
        } else {
            println!("❌ Aucun repository trouvé");
        }
        std::process::exit(1);
    }

    if keyword.is_none() && all_repos.len() > 500 {
        all_repos.truncate(500);
    }

    all_repos
}

async fn fetch_page(
    client: &Client,
    token: &str,
    organization: &str,
    page: usize,
) -> Result<Vec<Repo>, String> {
    let url = format!(
        "https://api.github.com/orgs/{}/repos?per_page=100&sort=pushed&direction=desc&page={}",
        organization, page
    );

    let response = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Connexion échouée: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }

    let repos: Vec<Repo> = response
        .json()
        .await
        .map_err(|e| format!("Parse JSON échoué: {}", e))?;

    Ok(repos)
}
