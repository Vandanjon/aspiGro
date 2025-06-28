use crate::utils::errors::Result;
use crate::utils::models::Repo;
use async_trait::async_trait;

/// Domain service pour la récupération de repositories
#[async_trait]
pub trait RepoFetcher {
    async fn fetch_repos(
        &self,
        token: &str,
        org: &str,
        keyword: Option<String>,
    ) -> Result<Vec<Repo>>;
}

/// Domain service pour le téléchargement
#[async_trait]
pub trait Downloader {
    async fn download_repos(&self, repos: &[Repo], target_dir: &str) -> Result<()>;
}

/// Domain service pour l'interface utilisateur
pub trait UserInterface {
    fn show_header(&self);
    fn start_step(&mut self, step: usize, message: &str);
    fn complete_step(&mut self, step: usize, message: &str, details: &str);
    fn show_progress(&self, current: usize, total: usize);
    fn configure_search(&self) -> Option<String>;
    fn finalize(&self);
}
