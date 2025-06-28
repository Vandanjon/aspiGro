use crate::models::Repo;
use crate::{http_client, repo_fetcher};

pub async fn scan_repositories(
    github_token: &str,
    organization: &str,
    keyword: Option<String>,
) -> Vec<Repo> {
    // Création du client HTTP
    let client = http_client::create_http_client();

    // Récupération des repositories avec le keyword
    let repos =
        repo_fetcher::fetch_repositories(&client, github_token, organization, keyword.as_deref())
            .await;

    // Affichage du résultat selon le mode
    match keyword {
        Some(kw) => {
            println!(
                "   🔍 {} repositories trouvés avec le mot-clé '{}'",
                repos.len(),
                kw
            );

            // Affichage de la liste des repositories trouvés avec liens
            if !repos.is_empty() {
                println!("   📋 Repositories trouvés :");
                for (i, repo) in repos.iter().enumerate() {
                    println!("      {}. {} - {}", i + 1, repo.name, repo.html_url);
                }
            }
        }
        None => {
            let count = repos.len().min(500);
            println!(
                "   📦 {} repositories récupérés (triés par dernière activité)",
                count
            );

            // Affichage de la liste des noms de repositories
            if !repos.is_empty() {
                println!("   📋 Liste des repositories :");
                for (i, repo) in repos.iter().take(10).enumerate() {
                    println!("      {}. {} - {}", i + 1, repo.name, repo.html_url);
                }

                if repos.len() > 10 {
                    println!("      ... et {} autres", repos.len() - 10);
                    println!("      💡 Utilisez un mot-clé pour filtrer les résultats");
                }
            }
        }
    }

    repos
}
