use thiserror::Error;

#[derive(Error, Debug)]
pub enum BuffMarketError {
    #[error("HTTP request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("Failed to parse JSON response: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("API error: {message}")]
    ApiError {
        message: String,
        // code: Option<String>, // Depending on how Buff API reports errors
    },

    #[error("Missing data in API response: {0}")]
    MissingData(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Unknown error")]
    Unknown,
} 