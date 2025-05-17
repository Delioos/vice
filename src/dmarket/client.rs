use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::Utc;
use dotenv::dotenv;
use log::{debug, error, info};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::to_string;
use std::env;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use hex;

use crate::dmarket::error::DMarketError;
use crate::dmarket::models::{ApiResponse, Balance, UserProfile};

const API_BASE_URL: &str = "https://api.dmarket.com";

pub struct DMarketClient {
    signing_key: SigningKey,
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
            return Err(DMarketError::ApiError(
                "Private key is too short".to_string(),
            ));
        }
        if public_key.len() < 32 {
            return Err(DMarketError::ApiError(
                "Public key is too short".to_string(),
            ));
        }

        debug!(
            "Initializing DMarket client with public key (first 8 chars): {}...",
            public_key.chars().take(8).collect::<String>()
        );

        // Convert hex private key to bytes
        let private_bytes = hex::decode(&private_key)?;
        
        // The key can be either 32 bytes (seed only) or 64 bytes (seed + public key)
        let key_bytes: [u8; 32] = match private_bytes.len() {
            32 => private_bytes.as_slice().try_into().map_err(|_| {
                DMarketError::ApiError("Failed to convert private key to fixed-size array".to_string())
            })?,
            64 => {
                // For 64-byte keys, take only the first 32 bytes (the seed)
                let mut seed = [0u8; 32];
                seed.copy_from_slice(&private_bytes[0..32]);
                seed
            },
            len => {
                return Err(DMarketError::ApiError(format!(
                    "Invalid private key length: {}. Expected 32 or 64 bytes", len
                )));
            }
        };
        
        let signing_key = SigningKey::from_bytes(&key_bytes);

        Ok(DMarketClient {
            signing_key,
            public_key,
            client: reqwest::Client::builder()
                .user_agent("DMarket-API-Client/1.0")
                .build()?,
        })
    }

    fn generate_signature(
        &self,
        timestamp: &str,
        method: &str,
        path: &str,
        body: &str,
    ) -> Result<String, DMarketError> {
        // Ensure method is uppercase
        let method = method.to_uppercase();

        // Ensure path starts with /
        let path = if !path.starts_with('/') {
            format!("/{}", path)
        } else {
            path.to_string()
        };

        // Concatenate the string without any separators (method + path + body + timestamp)
        let message = format!("{}{}{}{}", method, path, body, timestamp);
        debug!("Generating signature for message: {}", message);

        // Sign using Ed25519
        let signature = self.signing_key.sign(message.as_bytes());
        
        // Convert to hex and add prefix
        let signature = format!("dmar ed25519 {}", hex::encode(signature.to_bytes()));
        debug!("Generated signature: {}", signature);

        Ok(signature)
    }

    fn create_headers(&self, timestamp: &str, signature: &str) -> Result<HeaderMap, DMarketError> {
        let mut headers = HeaderMap::new();

        // Add headers with the naming convention from the examples
        headers.insert("X-Api-Key", HeaderValue::from_str(&self.public_key)?);
        headers.insert("X-Request-Sign", HeaderValue::from_str(signature)?);
        headers.insert("X-Sign-Date", HeaderValue::from_str(timestamp)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));

        Ok(headers)
    }

    pub async fn get_user_profile(&self) -> Result<UserProfile, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = "/account/v1/user";
        let body = "";

        debug!("Generating signature for user profile request");
        let signature = self.generate_signature(&timestamp, method, path, body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);

        let response = self.client.get(&url).headers(headers).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let profile = response.json::<UserProfile>().await?;
        Ok(profile)
    }
    
    pub async fn get_user_profile_raw(&self) -> Result<String, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = "/account/v1/user";
        let body = "";

        debug!("Generating signature for user profile request");
        let signature = self.generate_signature(&timestamp, method, path, body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);

        let response = self.client.get(&url).headers(headers).send().await?;

        let status = response.status();
        let text = response.text().await?;
        
        if !status.is_success() {
            error!("API returned error status {}: {}", status, text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, text
            )));
        }

        debug!("Raw response: {}", text);
        Ok(text)
    }

    pub async fn get_market_items(&self) -> Result<serde_json::Value, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
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

        let response = self.client.get(&url).headers(headers).send().await?;

        let status = response.status();
        let response_text = response.text().await?;
        debug!(
            "Received response with status {}: {}",
            status, response_text
        );

        if !status.is_success() {
            error!("API returned error status {}: {}", status, response_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, response_text
            )));
        }

        Ok(serde_json::from_str(&response_text)?)
    }

    pub async fn get_account_balance(&self) -> Result<Vec<Balance>, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = "/account/v1/balance";
        let body = "";

        debug!("Generating signature for balance request");
        let signature = self.generate_signature(&timestamp, method, path, body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);

        let response = self.client.get(&url).headers(headers).send().await?;

        let status = response.status();
        let response_text = response.text().await?;
        debug!(
            "Received response with status {}: {}",
            status, response_text
        );

        if !status.is_success() {
            error!("API returned error status {}: {}", status, response_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, response_text
            )));
        }

        match serde_json::from_str::<ApiResponse<Vec<Balance>>>(&response_text) {
            Ok(api_response) => match api_response {
                ApiResponse {
                    status: Some(s),
                    data: Some(balances),
                    error: None,
                    ..
                } if s == "ok" => {
                    info!("Successfully retrieved account balance");
                    Ok(balances)
                }
                ApiResponse {
                    error: Some(err), ..
                } => {
                    error!("API error: {} - {}", err.code, err.message);
                    Err(DMarketError::ApiError(format!(
                        "{}: {}",
                        err.code, err.message
                    )))
                }
                ApiResponse {
                    code: Some(c),
                    message: Some(m),
                    ..
                } => {
                    error!("API error: {} - {}", c, m);
                    Err(DMarketError::ApiError(format!("{}: {}", c, m)))
                }
                _ => {
                    error!("Unexpected API response format");
                    Err(DMarketError::ApiError(
                        "Unexpected API response format".to_string(),
                    ))
                }
            },
            Err(e) => {
                error!("Failed to parse API response: {}", e);
                Err(DMarketError::JsonError(e))
            }
        }
    }
}
