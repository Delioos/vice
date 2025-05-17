use chrono::Utc;
use dotenv::dotenv;
use log::{debug, error, info};
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;
use ed25519_dalek::{Signer, SigningKey};
use hex;
use serde::{Deserialize, Serialize};

use crate::dmarket::error::DMarketError;
use crate::dmarket::models::*;

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

    pub async fn get_market_items(&self, currency: &str, limit: u32, offset: u32, game_id: Option<&str>) -> Result<MarketItemsResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        
        let mut path = format!("/exchange/v1/market/items?currency={}&limit={}&offset={}", currency, limit, offset);
        
        // Add game_id if provided - this is a required parameter according to docs
        if let Some(game_id) = game_id {
            path = format!("{}&gameId={}", path, game_id);
        } else {
            // If no game_id is provided, we'll use CS2 as default (a411)
            path = format!("{}&gameId=a411", path);
        }

        debug!("Generating signature for market items request");
        let signature = self.generate_signature(&timestamp, method, &path, "")?;
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

        let response_text = response.text().await?;
        debug!("Market items response: {}", response_text);
        
        // Parse the response as a JSON Value first
        let response_value: serde_json::Value = serde_json::from_str(&response_text)?;
        
        // Based on the specific response structure we get from DMarket
        // Create a more flexible structure to parse their data
        #[derive(Debug, Deserialize)]
        struct DMarketMarketItem {
            #[serde(rename = "itemId")]
            item_id: String,
            title: String,
            #[serde(rename = "gameId")]
            game_id: String,
            #[serde(rename = "classId")]
            class_id: String,
            price: ApiPrice,
            #[serde(flatten)]
            other: std::collections::HashMap<String, serde_json::Value>,
        }
        
        #[derive(Debug, Deserialize)]
        struct ApiPrice {
            #[serde(rename = "USD")]
            usd: String,
            #[serde(rename = "DMC")]
            dmc: Option<String>,
        }
        
        #[derive(Debug, Deserialize)]
        struct TotalCounts {
            #[serde(default)]
            offers: i32,
            #[serde(default)]
            targets: i32,
            #[serde(default)]
            items: i32,
            #[serde(default)]
            completedOffers: i32,
            #[serde(default)]
            closedTargets: i32,
        }
        
        #[derive(Debug, Deserialize)]
        struct DMarketResponse {
            #[serde(default)]
            objects: Vec<DMarketMarketItem>,
            #[serde(default)]
            items: Vec<DMarketMarketItem>,
            total: TotalCounts,
        }
        
        let dmarket_response = match serde_json::from_str::<DMarketResponse>(&response_text) {
            Ok(response) => response,
            Err(e) => {
                debug!("Failed to parse DMarket response: {}", e);
                return Err(DMarketError::JsonError(e));
            }
        };
        
        // Use either objects or items field, depending on what is populated
        let items = if !dmarket_response.objects.is_empty() {
            dmarket_response.objects
        } else {
            dmarket_response.items
        };
        
        // Calculate total items (use offers as the primary count)
        let total_items = dmarket_response.total.offers;
        
        // Convert to our model
        let market_items = items
            .into_iter()
            .map(|item| {
                MarketItem {
                    item_id: item.item_id,
                    item_type: "offer".to_string(),
                    title: item.title,
                    description: None,
                    slug: "".to_string(),
                    status: "active".to_string(),
                    ownersCount: None,
                    image: "".to_string(),
                    class_id: item.class_id,
                    game: "".to_string(),
                    price: Price {
                        amount: item.price.usd,
                        currency: "USD".to_string(),
                    },
                    suggested_price: None,
                    discount: None,
                    extra: MarketItemExtra {
                        name_color: None,
                        background_color: None,
                        category: None,
                        exterior: None,
                        category_path: None,
                        tradable: None,
                        daysBeforeTrade: None,
                        floatValue: None,
                        gameId: Some(item.game_id.clone()),
                    },
                    attributes: Vec::new(),
                    locked: false,
                    createdAt: 0,
                    updatedAt: 0,
                    inMarket: true,
                    gameId: item.game_id,
                    withdrawable: true,
                    tradeLock: None,
                    offer_type: None,
                    asset_id: None,
                }
            })
            .collect();
        
        Ok(MarketItemsResponse {
            objects: market_items,
            total: total_items,
        })
    }

    pub async fn search_market_items(&self, query: &str, currency: &str, limit: u32, offset: u32, game_id: Option<&str>) -> Result<MarketItemsResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        
        let mut path = format!("/exchange/v1/market/items?currency={}&limit={}&offset={}&title={}", currency, limit, offset, query);
        
        // Add game_id if provided
        if let Some(game_id) = game_id {
            path = format!("{}&gameId={}", path, game_id);
        } else {
            // If no game_id is provided, we'll use CS2 as default (a411)
            path = format!("{}&gameId=a411", path);
        }

        debug!("Generating signature for market search request");
        let signature = self.generate_signature(&timestamp, method, &path, "")?;
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

        let response_text = response.text().await?;
        debug!("Search market items response: {}", response_text);
        
        // Use the same parsing logic as get_market_items
        #[derive(Debug, Deserialize)]
        struct DMarketMarketItem {
            #[serde(rename = "itemId")]
            item_id: String,
            title: String,
            #[serde(rename = "gameId")]
            game_id: String,
            #[serde(rename = "classId")]
            class_id: String,
            price: ApiPrice,
            #[serde(flatten)]
            other: std::collections::HashMap<String, serde_json::Value>,
        }
        
        #[derive(Debug, Deserialize)]
        struct ApiPrice {
            #[serde(rename = "USD")]
            usd: String,
            #[serde(rename = "DMC")]
            dmc: Option<String>,
        }
        
        #[derive(Debug, Deserialize)]
        struct TotalCounts {
            #[serde(default)]
            offers: i32,
            #[serde(default)]
            targets: i32,
            #[serde(default)]
            items: i32,
            #[serde(default)]
            completedOffers: i32,
            #[serde(default)]
            closedTargets: i32,
        }
        
        #[derive(Debug, Deserialize)]
        struct DMarketResponse {
            #[serde(default)]
            objects: Vec<DMarketMarketItem>,
            #[serde(default)]
            items: Vec<DMarketMarketItem>,
            total: TotalCounts,
        }
        
        let dmarket_response = match serde_json::from_str::<DMarketResponse>(&response_text) {
            Ok(response) => response,
            Err(e) => {
                debug!("Failed to parse DMarket response: {}", e);
                return Err(DMarketError::JsonError(e));
            }
        };
        
        // Use either objects or items field, depending on what is populated
        let items = if !dmarket_response.objects.is_empty() {
            dmarket_response.objects
        } else {
            dmarket_response.items
        };
        
        // Calculate total items (use offers as the primary count)
        let total_items = dmarket_response.total.offers;
        
        // Convert to our model
        let market_items = items
            .into_iter()
            .map(|item| {
                MarketItem {
                    item_id: item.item_id,
                    item_type: "offer".to_string(),
                    title: item.title,
                    description: None,
                    slug: "".to_string(),
                    status: "active".to_string(),
                    ownersCount: None,
                    image: "".to_string(),
                    class_id: item.class_id,
                    game: "".to_string(),
                    price: Price {
                        amount: item.price.usd,
                        currency: "USD".to_string(),
                    },
                    suggested_price: None,
                    discount: None,
                    extra: MarketItemExtra {
                        name_color: None,
                        background_color: None,
                        category: None,
                        exterior: None,
                        category_path: None,
                        tradable: None,
                        daysBeforeTrade: None,
                        floatValue: None,
                        gameId: Some(item.game_id.clone()),
                    },
                    attributes: Vec::new(),
                    locked: false,
                    createdAt: 0,
                    updatedAt: 0,
                    inMarket: true,
                    gameId: item.game_id,
                    withdrawable: true,
                    tradeLock: None,
                    offer_type: None,
                    asset_id: None,
                }
            })
            .collect();
        
        Ok(MarketItemsResponse {
            objects: market_items,
            total: total_items,
        })
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
        debug!("Response: {}", response_text);

        // The balance endpoint returns a JSON object, not an array
        // Example: {"dmc":"0.00","dmcAvailableToWithdraw":"0.00","usd":"0.00","usdAvailableToWithdraw":"0.00"}
        let balance_response: serde_json::Value = serde_json::from_str(&response_text)?;

        if !status.is_success() {
            error!("API returned error status {}: {}", status, response_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, response_text
            )));
        }

        // Convert the response into our Balance model
        let mut balances = Vec::new();
        
        // Extract USD balance
        if let Some(usd) = balance_response.get("usd") {
            if let Some(usd_str) = usd.as_str() {
                balances.push(Balance {
                    amount: usd_str.to_string(),
                    currency: "USD".to_string(),
                });
            }
        }
        
        // Extract DMC balance
        if let Some(dmc) = balance_response.get("dmc") {
            if let Some(dmc_str) = dmc.as_str() {
                balances.push(Balance {
                    amount: dmc_str.to_string(),
                    currency: "DMC".to_string(),
                });
            }
        }

        Ok(balances)
    }

    pub async fn get_inventory(&self, limit: u32, offset: u32, game_id: Option<&str>) -> Result<InventoryResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        
        let mut path = format!("/exchange/v1/user-inventory?limit={}&offset={}", limit, offset);
        if let Some(game_id) = game_id {
            path = format!("{}&gameId={}", path, game_id);
        }
        
        debug!("Generating signature for inventory request");
        let signature = self.generate_signature(&timestamp, method, &path, "")?;
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

        let response_text = response.text().await?;
        debug!("Inventory response: {}", response_text);
        
        // Parse the DMarket inventory response format
        #[derive(Debug, Deserialize)]
        struct DMarketInventoryItem {
            #[serde(rename = "itemId")]
            item_id: String,
            name: String,
            #[serde(rename = "classId")]
            class_id: String,
            #[serde(rename = "gameId")]
            game_id: String,
            #[serde(flatten)]
            other: std::collections::HashMap<String, serde_json::Value>,
        }
        
        #[derive(Debug, Deserialize)]
        struct DMarketInventoryResponse {
            #[serde(default)]
            items: Vec<DMarketInventoryItem>,
            #[serde(default = "default_total")]
            total: i32,
        }
        
        fn default_total() -> i32 { 0 }
        
        match serde_json::from_str::<DMarketInventoryResponse>(&response_text) {
            Ok(dmarket_inventory) => {
                // Convert to our model
                let inventory_items = dmarket_inventory.items
                    .into_iter()
                    .map(|item| {
                        InventoryItem {
                            item_id: item.item_id,
                            title: item.name,
                            status: "active".to_string(),
                            image: "".to_string(),
                            game: "".to_string(),
                            class_id: item.class_id,
                            extra: InventoryItemExtra {
                                exterior: None,
                                category: None,
                                category_path: None,
                                name_color: None,
                                background_color: None,
                                tradable: Some(true),
                                daysBeforeTrade: None,
                                floatValue: None,
                            },
                            inMarket: false,
                            locked: false,
                        }
                    })
                    .collect();
                
                Ok(InventoryResponse {
                    objects: inventory_items,
                    total: dmarket_inventory.total,
                })
            },
            Err(e) => {
                debug!("Failed to parse inventory response: {}", e);
                Err(DMarketError::JsonError(e))
            }
        }
    }

    pub async fn create_sell_offer(&self, request: &SellOfferRequest) -> Result<SellOfferResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/trading/v1/sell-offer";
        let body = serde_json::to_string(request)?;

        debug!("Generating signature for create sell offer request");
        let signature = self.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let sell_response = response.json::<SellOfferResponse>().await?;
        Ok(sell_response)
    }

    pub async fn buy_offer(&self, request: &BuyOfferRequest) -> Result<BuyOfferResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/trading/v1/buy/offers";
        let body = serde_json::to_string(request)?;

        debug!("Generating signature for buy offer request");
        let signature = self.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let buy_response = response.json::<BuyOfferResponse>().await?;
        Ok(buy_response)
    }

    pub async fn import_items(&self, request: &ImportRequest) -> Result<ImportResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/trading/v1/import";
        let body = serde_json::to_string(request)?;

        debug!("Generating signature for import items request");
        let signature = self.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let import_response = response.json::<ImportResponse>().await?;
        Ok(import_response)
    }

    pub async fn export_items(&self, request: &ExportRequest) -> Result<ExportResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/trading/v1/export";
        let body = serde_json::to_string(request)?;

        debug!("Generating signature for export items request");
        let signature = self.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let export_response = response.json::<ExportResponse>().await?;
        Ok(export_response)
    }

    pub async fn get_games(&self) -> Result<Vec<Game>, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = "/exchange/v1/games";
        
        debug!("Generating signature for games request");
        let signature = self.generate_signature(&timestamp, method, path, "")?;
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

        let response_text = response.text().await?;
        debug!("Games response: {}", response_text);
        
        #[derive(Debug, Deserialize)]
        struct DMarketGame {
            id: String,
            name: String,
            #[serde(flatten)]
            other: std::collections::HashMap<String, serde_json::Value>,
        }
        
        #[derive(Debug, Deserialize)]
        struct DMarketGamesResponse {
            games: Vec<DMarketGame>,
        }
        
        match serde_json::from_str::<DMarketGamesResponse>(&response_text) {
            Ok(response) => {
                // Convert to our model
                let games = response.games
                    .into_iter()
                    .map(|game| {
                        Game {
                            id: game.id,
                            title: game.name,
                            logo: "".to_string(),
                            slug: "".to_string(),
                            status: "active".to_string(),
                        }
                    })
                    .collect();
                
                Ok(games)
            },
            Err(e) => {
                debug!("Failed to parse games response: {}", e);
                Err(DMarketError::JsonError(e))
            }
        }
    }

    pub async fn cancel_sell_offer(&self, offer_ids: Vec<String>) -> Result<SellOfferResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/trading/v1/offers/cancel";
        
        #[derive(Debug, Serialize)]
        struct CancelRequest {
            offers: Vec<String>,
        }
        
        let request = CancelRequest { offers: offer_ids };
        let body = serde_json::to_string(&request)?;

        debug!("Generating signature for cancel offer request");
        let signature = self.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let cancel_response = response.json::<SellOfferResponse>().await?;
        Ok(cancel_response)
    }

    pub async fn update_offer_price(&self, offer_id: &str, new_price: &SellOfferPrice) -> Result<SellOfferResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/trading/v1/offers/update-price";
        
        #[derive(Debug, Serialize)]
        struct UpdatePriceRequest {
            offers: Vec<UpdatePriceItem>,
        }
        
        #[derive(Debug, Serialize)]
        struct UpdatePriceItem {
            offerId: String,
            price: SellOfferPrice,
        }
        
        let request = UpdatePriceRequest { 
            offers: vec![UpdatePriceItem { 
                offerId: offer_id.to_string(), 
                price: new_price.clone()
            }] 
        };
        let body = serde_json::to_string(&request)?;

        debug!("Generating signature for update price request");
        let signature = self.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let update_response = response.json::<SellOfferResponse>().await?;
        Ok(update_response)
    }

    pub async fn get_user_sell_offers(&self, limit: u32, offset: u32, game_id: Option<&str>, status: Option<&str>) -> Result<MarketItemsResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        
        let mut path = format!("/offers-search/v1/user/sell-offers?limit={}&offset={}", limit, offset);
        
        if let Some(game_id) = game_id {
            path = format!("{}&gameId={}", path, game_id);
        }
        
        if let Some(status) = status {
            path = format!("{}&status={}", path, status);
        }

        debug!("Generating signature for user sell offers request");
        let signature = self.generate_signature(&timestamp, method, &path, "")?;
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

        let sell_offers = response.json::<MarketItemsResponse>().await?;
        Ok(sell_offers)
    }

    pub async fn get_market_item_details(&self, class_id: &str, limit: u32, offset: u32, currency: &str) -> Result<MarketItemsResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = format!("/offers-search/v1/aggregated-class/{}/sell-offers?limit={}&offset={}&currency={}", class_id, limit, offset, currency);

        debug!("Generating signature for market item details request");
        let signature = self.generate_signature(&timestamp, method, &path, "")?;
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

        let item_details = response.json::<MarketItemsResponse>().await?;
        Ok(item_details)
    }

    pub async fn create_target(&self, request: &CreateTargetRequest) -> Result<CreateTargetResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/target-predictor/v1/target";
        let body = serde_json::to_string(request)?;

        debug!("Generating signature for create target request");
        let signature = self.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let create_response = response.json::<CreateTargetResponse>().await?;
        Ok(create_response)
    }

    pub async fn get_targets(&self, limit: u32, offset: u32) -> Result<TargetListResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = format!("/target-predictor/v1/user/targets?limit={}&offset={}", limit, offset);

        debug!("Generating signature for get targets request");
        let signature = self.generate_signature(&timestamp, method, &path, "")?;
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

        let targets = response.json::<TargetListResponse>().await?;
        Ok(targets)
    }

    pub async fn delete_targets(&self, target_ids: Vec<String>) -> Result<CreateTargetResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "DELETE";
        let path = "/target-predictor/v1/target";
        
        let request = DeleteTargetRequest { targets: target_ids };
        let body = serde_json::to_string(&request)?;

        debug!("Generating signature for delete targets request");
        let signature = self.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.delete(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let delete_response = response.json::<CreateTargetResponse>().await?;
        Ok(delete_response)
    }
}
