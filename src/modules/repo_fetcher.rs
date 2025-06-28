use crate::modules::http_client;
use crate::utils::models::Repo;
use futures::future::join_all;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchResponse {
    total_count: u32,
    items: Vec<Repo>,
}

pub async fn fetch_repositories(
    client: &Client,
    token: &str,
    organization: &str,
    keyword: Option<&str>,
) -> Vec<Repo> {
    match keyword {
        Some(kw) => fetch_repositories_by_search(client, token, organization, kw).await,
        None => fetch_repositories_by_pagination(client, token, organization).await,
    }
}

async fn fetch_repositories_by_search(
    client: &Client,
    token: &str,
    organization: &str,
    keyword: &str,
) -> Vec<Repo> {
    let max_pages = 10;
    let concurrent_requests = 5;
    let mut all_repos = Vec::new();

    // Première requête pour obtenir le total
    let first_result = search_repositories_page(client, token, organization, keyword, 1).await;
    match first_result {
        Ok((repos, total_count)) => {
            all_repos.extend(repos);
            let total_pages = ((total_count as f64 / 100.0).ceil() as usize).min(max_pages);

            // Récupération des pages restantes en parallèle
            if total_pages > 1 {
                let mut current_page = 2;

                while current_page <= total_pages {
                    let pages_to_fetch =
                        std::cmp::min(concurrent_requests, total_pages - current_page + 1);
                    let page_range: Vec<usize> =
                        (current_page..current_page + pages_to_fetch).collect();

                    let futures: Vec<_> = page_range
                        .iter()
                        .map(|&page| {
                            search_repositories_page(client, token, organization, keyword, page)
                        })
                        .collect();

                    let results = join_all(futures).await;

                    for result in results.into_iter() {
                        match result {
                            Ok((repos, _)) => {
                                all_repos.extend(repos);
                            }
                            Err(_) => {}
                        }
                    }

                    current_page += pages_to_fetch;
                }
            }
        }
        Err(_) => {
            return Vec::new();
        }
    }

    all_repos
}

async fn fetch_repositories_by_pagination(
    client: &Client,
    token: &str,
    organization: &str,
) -> Vec<Repo> {
    let max_pages = 20;
    let concurrent_requests = 6;
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

        for result in results.into_iter() {
            match result {
                Ok(repos) => {
                    if repos.is_empty() {
                        page_empty = true;
                        break;
                    }
                    batch_repos.extend(repos);
                }
                Err(_) => continue,
            }
        }

        all_repos.extend(batch_repos);

        if page_empty {
            break;
        }

        current_page += pages_to_fetch;
    }

    if all_repos.len() > 500 {
        all_repos.truncate(500);
    }

    all_repos
}

async fn search_repositories_page(
    client: &Client,
    token: &str,
    organization: &str,
    keyword: &str,
    page: usize,
) -> Result<(Vec<Repo>, u32), String> {
    let url = format!(
        "https://api.github.com/search/repositories?q={}+org:{}&per_page=100&page={}",
        keyword, organization, page
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

    let search_result: SearchResponse = response
        .json()
        .await
        .map_err(|e| format!("Parse JSON échoué: {}", e))?;

    Ok((search_result.items, search_result.total_count))
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

/// Point d'entrée principal : scan et affichage des résultats
pub async fn scan_and_fetch(token: &str, organization: &str, keyword: Option<String>) -> Vec<Repo> {
    let client = http_client::create_http_client();
    let repos = fetch_repositories(&client, token, organization, keyword.as_deref()).await;

    // Pas d'affichage détaillé ici - juste le retour des résultats
    repos
}
