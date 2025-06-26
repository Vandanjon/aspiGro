use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub default_branch: String,
}
