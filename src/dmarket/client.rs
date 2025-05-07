use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::Utc;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use log::{debug, error, info};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use sha2::Sha256;
use std::env;

use crate::dmarket::error::DMarketError;
use crate::dmarket::models::{ApiResponse, UserProfile, Balance};

const API_BASE_URL: &str = "https://api.dmarket.com";

pub struct DMarketClient {
    private_key: String,
    public_key: String,
    client: reqwest::Client,
}

impl DMarketClient {
    pub fn new() -> Result<Self, DMarketError> {
        dotenv().ok();
        let private_key = env::var("DMARKET_PRIVATE_KEY")?;
        let public_key = env::var("DMARKET_PUBLIC_KEY")?;
        
        // Validate API keys
        if private_key.len() < 32 {
            return Err(DMarketError::ApiError("Private key is too short".to_string()));
        }
        if public_key.len() < 32 {
            return Err(DMarketError::ApiError("Public key is too short".to_string()));
        }

        debug!("Initializing DMarket client with public key (first 8 chars): {}...", 
               public_key.chars().take(8).collect::<String>());
        
        Ok(DMarketClient {
            private_key,
            public_key,
            client: reqwest::Client::builder()
                .user_agent("DMarket-API-Client/1.0")
                .build()?,
        })
    }

    fn generate_signature(&self, timestamp: &str, method: &str, path: &str, body: &str) -> Result<String, DMarketError> {
        // Ensure method is uppercase
        let method = method.to_uppercase();
        
        // Ensure path starts with /
        let path = if !path.starts_with('/') {
            format!("/{}", path)
        } else {
            path.to_string()
        };
        
        // Concatenate the string without any separators
        let message = format!("{}{}{}{}", timestamp, method, path, body);
        debug!("Generating signature for message: {}", message);
        
        // Create HMAC-SHA256
        let mut mac = Hmac::<Sha256>::new_from_slice(self.private_key.as_bytes())
            .map_err(|e| DMarketError::HmacError(e.to_string()))?;
        
        // Update with message bytes
        mac.update(message.as_bytes());
        
        // Get the HMAC result and encode as base64
        let result = mac.finalize();
        let signature = BASE64.encode(result.into_bytes());
        debug!("Generated signature: {}", signature);
        
        Ok(signature)
    }

    fn create_headers(&self, timestamp: &str, signature: &str) -> Result<HeaderMap, DMarketError> {
        let mut headers = HeaderMap::new();
        
        // Add required headers with exact casing
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&self.public_key)?,
        );
        headers.insert(
            "x-sign",
            HeaderValue::from_str(signature)?,
        );
        headers.insert(
            "x-time",
            HeaderValue::from_str(timestamp)?,
        );
        headers.insert(
            "content-type",
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "accept",
            HeaderValue::from_static("application/json"),
        );
        
        Ok(headers)
    }

    pub async fn get_user_profile(&self) -> Result<UserProfile, DMarketError> {
        let timestamp = Utc::now().timestamp_millis().to_string();
        let method = "GET";
        let path = "/account/v1/user";
        let body = "";

        debug!("Generating signature for user profile request");
        let signature = self.generate_signature(&timestamp, method, path, body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);

        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}", 
                status,
                error_text
            )));
        }

        let profile = response.json::<UserProfile>().await?;
        Ok(profile)
    }

    pub async fn get_market_items(&self) -> Result<serde_json::Value, DMarketError> {
        let timestamp = Utc::now().timestamp_millis().to_string();
        let method = "GET";
        let path = "/exchange/v1/market/items";
        let query = "currency=USD&limit=10";
        let full_path = format!("{}?{}", path, query);

        debug!("Generating signature for market items request");
        let signature = self.generate_signature(&timestamp, method, &full_path, "")?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, full_path);
        debug!("Request URL: {}", url);
        
        let response = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;
        debug!("Received response with status {}: {}", status, response_text);

        if !status.is_success() {
            error!("API returned error status {}: {}", status, response_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status,
                response_text
            )));
        }

        Ok(serde_json::from_str(&response_text)?)
    }

    pub async fn get_account_balance(&self) -> Result<Vec<Balance>, DMarketError> {
        let timestamp = Utc::now().timestamp_millis().to_string();
        let method = "GET";
        let path = "/account/v1/balance";
        let body = "";

        debug!("Generating signature for balance request");
        let signature = self.generate_signature(&timestamp, method, path, body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        
        let response = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;
        debug!("Received response with status {}: {}", status, response_text);

        if !status.is_success() {
            error!("API returned error status {}: {}", status, response_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status,
                response_text
            )));
        }

        match serde_json::from_str::<ApiResponse<Vec<Balance>>>(&response_text) {
            Ok(api_response) => match api_response {
                ApiResponse { status: Some(s), data: Some(balances), error: None, .. } if s == "ok" => {
                    info!("Successfully retrieved account balance");
                    Ok(balances)
                },
                ApiResponse { error: Some(err), .. } => {
                    error!("API error: {} - {}", err.code, err.message);
                    Err(DMarketError::ApiError(format!("{}: {}", err.code, err.message)))
                },
                ApiResponse { code: Some(c), message: Some(m), .. } => {
                    error!("API error: {} - {}", c, m);
                    Err(DMarketError::ApiError(format!("{}: {}", c, m)))
                },
                _ => {
                    error!("Unexpected API response format");
                    Err(DMarketError::ApiError("Unexpected API response format".to_string()))
                }
            },
            Err(e) => {
                error!("Failed to parse API response: {}", e);
                Err(DMarketError::JsonError(e))
            }
        }
    }
} 