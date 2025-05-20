use dotenv::dotenv;
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

use crate::csfloat::error::CSFloatError;
use crate::csfloat::endpoints::listings::ListingsHandler;

pub const API_BASE_URL: &str = "https://csfloat.com/api/v1";

/// The main CSFloat API client.
pub struct CSFloatClient {
    pub(crate) api_key: String,
    pub(crate) http_client: reqwest::Client,
}

impl CSFloatClient {
    /// Creates a new CSFloatClient instance.
    /// 
    /// Reads `CSFLOAT_API_KEY` from environment variables.
    pub fn new() -> Result<Self, CSFloatError> {
        dotenv().ok();
        let api_key = env::var("CSFLOAT_API_KEY")?;

        if api_key.is_empty() {
            return Err(CSFloatError::ApiError(
                "API key cannot be empty".to_string(),
            ));
        }

        debug!(
            "Initializing CSFloat client with API key (first 8 chars): {}...",
            api_key.chars().take(8).collect::<String>()
        );

        Ok(CSFloatClient {
            api_key,
            http_client: reqwest::Client::builder()
                .user_agent("CSFloat-API-Client/1.0")
                .build()?,
        })
    }

    /// Creates the required HTTP headers for an API request.
    pub(crate) fn create_headers(&self) -> Result<HeaderMap, CSFloatError> {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&self.api_key)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        Ok(headers)
    }

    /// Makes a GET request to the CSFloat API.
    pub(crate) async fn get<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, CSFloatError> {
        let url = format!("{}{}", API_BASE_URL, endpoint);
        debug!("GET request to URL: {}", url);
        
        let response = self.http_client
            .get(&url)
            .headers(self.create_headers()?)
            .send()
            .await?;

        let status = response.status();
        debug!("Response status: {}", status);
        
        if !status.is_success() {
            let error_text = response.text().await?;
            debug!("Error response: {}", error_text);
            return Err(CSFloatError::ApiError(format!(
                "API request failed with status {}: {}",
                status, error_text
            )));
        }

        // Debug the response body before trying to deserialize it
        let text = response.text().await?;
        debug!("Response body: {}", text);
        
        match serde_json::from_str::<T>(&text) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                debug!("Failed to deserialize response: {}", e);
                Err(CSFloatError::SerializationError(e))
            }
        }
    }

    /// Makes a POST request to the CSFloat API.
    pub(crate) async fn post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T, CSFloatError> {
        let url = format!("{}{}", API_BASE_URL, endpoint);
        
        let response = self.http_client
            .post(&url)
            .headers(self.create_headers()?)
            .json(body)
            .send()
            .await?;

        let status = response.status();
        
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(CSFloatError::ApiError(format!(
                "API request failed with status {}: {}",
                status,
                error_text
            )));
        }

        // Debug the response body before trying to deserialize it
        let text = response.text().await?;
        debug!("Response body: {}", text);
        
        match serde_json::from_str::<T>(&text) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                debug!("Failed to deserialize response: {}", e);
                Err(CSFloatError::SerializationError(e))
            }
        }
    }

    /// Provides access to listings-related API endpoints.
    pub fn listings(&self) -> ListingsHandler<'_> {
        ListingsHandler::new(self)
    }
} 