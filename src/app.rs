use crate::modules::{downloader, load_env, repo_fetcher, running_mode};
use crate::utils::ui;

pub struct App {
    progress: ui::ProgressTracker,
}

impl App {
    pub fn new() -> Self {
        Self {
            progress: ui::ProgressTracker::new(5),
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        ui::print_header();

        // Étape 1: Chargement environnement
        self.progress
            .start_step("Chargement des variables d'environnement");
        let (github_token, dl_folder_path, organization, var_count) = load_env::load_vars();
        self.progress.complete_step(
            "Chargement des variables d'environnement",
            &format!("{} variables chargées", var_count),
        );

        // Étape 2: Configuration recherche
        self.progress.start_step("Configuration de la recherche");
        let keyword = running_mode::configure_search();
        let search_details = match &keyword {
            Some(kw) => format!("Mot-clé : '{}'", kw),
            None => "Mode 500 derniers repos".to_string(),
        };
        self.progress
            .complete_step("Configuration de la recherche", &search_details);

        // Étape 3: Scan des repos
        self.progress.start_step("Scan des repos de l'organisation");
        let repos = repo_fetcher::scan_and_fetch(&github_token, &organization, keyword).await;
        self.progress.complete_step(
            "Scan des repos de l'organisation",
            &format!("{} repos trouvés", repos.len()),
        );

        // Étape 4: Téléchargement
        self.progress.start_step("Téléchargement des repos");
        if !repos.is_empty() {
            self.progress
                .complete_step_with_status("Téléchargement des repos", "🚀");
            self.progress.show_info(&format!(
                "📁 Dossier de téléchargement : {}",
                dl_folder_path
            ));

            downloader::download_repositories(&repos, &dl_folder_path).await;

            self.progress.complete_step(
                "Téléchargement des repos",
                &format!("{} repos téléchargés", repos.len()),
            );
        } else {
            self.progress
                .complete_step_with_status("Téléchargement des repos", "⚠️");
            self.progress
                .show_info("⚠️  Aucun repository à télécharger");
        }

        // Étape 5: Finalisation
        self.progress.finalize();
        self.progress
            .show_info(&format!("✅ {} repositories scannés", repos.len()));

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
