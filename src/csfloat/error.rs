use thiserror::Error;

#[derive(Error, Debug)]
pub enum CSFloatError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
} 