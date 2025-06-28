use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub default_branch: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub updated_at: String,
}
