use thiserror::Error;

#[derive(Error, Debug)]
pub enum DMarketError {
    #[error("Environment error: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Header error: {0}")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error("HMAC error: {0}")]
    HmacError(String),

    #[error("API error: {0}")]
    ApiError(String),
} 