use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    GitError(String),
    EnvError(String),
    HttpError(String),
    Timeout,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "Erreur I/O: {}", e),
            AppError::GitError(msg) => write!(f, "Erreur Git: {}", msg),
            AppError::EnvError(msg) => write!(f, "Erreur environnement: {}", msg),
            AppError::HttpError(msg) => write!(f, "Erreur HTTP: {}", msg),
            AppError::Timeout => write!(f, "Timeout"),
        }
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HttpError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
