// use chrono::Utc; // No longer needed here
use dotenv::dotenv;
use log::debug;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;
use ed25519_dalek::{Signer, SigningKey};
use hex;
// use serde::{Deserialize, Serialize}; // No longer needed here

use crate::dmarket::error::DMarketError;
// use crate::dmarket::models::*; // No longer needed here

// Import handlers
use crate::dmarket::endpoints::{
    account::AccountHandler,
    exchange::ExchangeHandler,
    inventory::InventoryHandler,
    target::TargetHandler,
    trading::TradingHandler,
};

pub const API_BASE_URL: &str = "https://api.dmarket.com";

/// The main DMarket API client.
/// It provides access to various API endpoint categories through dedicated handlers.
pub struct DMarketClient {
    pub(crate) signing_key: SigningKey,
    pub(crate) public_key: String,
    pub(crate) http_client: reqwest::Client, // Renamed from client
}

impl DMarketClient {
    /// Creates a new DMarketClient instance.
    /// 
    /// Reads `DMARKET_PRIVATE_KEY` and `DMARKET_PUBLIC_KEY` from environment variables.
    pub fn new() -> Result<Self, DMarketError> {
        dotenv().ok();
        let private_key = env::var("DMARKET_PRIVATE_KEY")?;
        let public_key = env::var("DMARKET_PUBLIC_KEY")?;

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

        let private_bytes = hex::decode(&private_key)?;
        
        let key_bytes: [u8; 32] = match private_bytes.len() {
            32 => private_bytes.as_slice().try_into().map_err(|_| {
                DMarketError::ApiError("Failed to convert private key to fixed-size array".to_string())
            })?,
            64 => {
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
            http_client: reqwest::Client::builder()
                .user_agent("DMarket-API-Client/1.0")
                .build()?,
        })
    }

    /// Generates the Ed25519 signature for an API request.
    /// This method is `pub(crate)` and intended for use by endpoint handlers.
    pub(crate) fn generate_signature(
        &self,
        timestamp: &str,
        method: &str,
        path: &str,
        body: &str,
    ) -> Result<String, DMarketError> {
        let method = method.to_uppercase();
        let path = if !path.starts_with('/') {
            format!("/{}", path)
        } else {
            path.to_string()
        };
        let message = format!("{}{}{}{}", method, path, body, timestamp);
        debug!("Generating signature for message: {}", message);
        let signature = self.signing_key.sign(message.as_bytes());
        let signature = format!("dmar ed25519 {}", hex::encode(signature.to_bytes()));
        debug!("Generated signature: {}", signature);
        Ok(signature)
    }

    /// Creates the required HTTP headers for an API request.
    /// This method is `pub(crate)` and intended for use by endpoint handlers.
    pub(crate) fn create_headers(&self, timestamp: &str, signature: &str) -> Result<HeaderMap, DMarketError> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Api-Key", HeaderValue::from_str(&self.public_key)?);
        headers.insert("X-Request-Sign", HeaderValue::from_str(signature)?);
        headers.insert("X-Sign-Date", HeaderValue::from_str(timestamp)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("Accept", HeaderValue::from_static("application/json"));
        Ok(headers)
    }

    // Accessor methods for handlers

    /// Provides access to account-related API endpoints.
    pub fn account(&self) -> AccountHandler<'_> {
        AccountHandler::new(self)
    }

    /// Provides access to exchange and market-related API endpoints.
    pub fn exchange(&self) -> ExchangeHandler<'_> {
        ExchangeHandler::new(self)
    }

    /// Provides access to user inventory-related API endpoints.
    pub fn inventory(&self) -> InventoryHandler<'_> {
        InventoryHandler::new(self)
    }

    /// Provides access to trading-related API endpoints.
    pub fn trading(&self) -> TradingHandler<'_> {
        TradingHandler::new(self)
    }

    /// Provides access to target-related API endpoints.
    pub fn target(&self) -> TargetHandler<'_> {
        TargetHandler::new(self)
    }
}
