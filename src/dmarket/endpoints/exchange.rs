use crate::dmarket::client::{DMarketClient, API_BASE_URL};
use crate::dmarket::error::DMarketError;
use crate::dmarket::models::{Game, MarketItem, MarketItemExtra, MarketItemsResponse, Price};
use chrono::Utc;
use log::{debug, error};
use serde::Deserialize;
use std::collections::HashMap;

/// Handles exchange and market-related API endpoints.
pub struct ExchangeHandler<'a> {
    client: &'a DMarketClient,
}

impl<'a> ExchangeHandler<'a> {
    /// Creates a new ExchangeHandler.
    pub fn new(client: &'a DMarketClient) -> Self {
        Self { client }
    }

    /// Retrieves a list of items available on the market.
    pub async fn get_market_items(
        &self,
        game_id: &str, // Mandatory
        currency: &str, // Mandatory
        limit: u32,
        offset: u32,
        order_by: Option<&str>,
        order_dir: Option<&str>,
        title: Option<&str>,
        tree_filters: Option<&str>,
        price_from: Option<u32>,
        price_to: Option<u32>,
        types: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<MarketItemsResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        
        let mut params = vec![
            ("gameId".to_string(), game_id.to_string()),
            ("currency".to_string(), currency.to_string()),
            ("limit".to_string(), limit.to_string()),
            ("offset".to_string(), offset.to_string()),
        ];

        if let Some(val) = order_by {
            params.push(("orderBy".to_string(), val.to_string()));
        }
        if let Some(val) = order_dir {
            params.push(("orderDir".to_string(), val.to_string()));
        }
        if let Some(val) = title {
            params.push(("title".to_string(), urlencoding::encode(val).into_owned()));
        }
        if let Some(val) = tree_filters {
            params.push(("treeFilters".to_string(), val.to_string()));
        }
        if let Some(val) = price_from {
            params.push(("priceFrom".to_string(), val.to_string()));
        }
        if let Some(val) = price_to {
            params.push(("priceTo".to_string(), val.to_string()));
        }
        if let Some(val) = types {
            params.push(("types".to_string(), val.to_string()));
        }
        if let Some(val) = cursor {
            params.push(("cursor".to_string(), val.to_string()));
        }

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");
        
        let path = format!("/exchange/v1/market/items?{}", query_string);

        debug!("Generating signature for market items request, path: {}", path);
        let signature = self.client.generate_signature(&timestamp, method, &path, "")?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);

        let response = self.client.http_client.get(&url).headers(headers).send().await?;

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
            other: HashMap<String, serde_json::Value>,
        }
        
        #[derive(Debug, Deserialize)]
        struct ApiPrice {
            #[serde(rename = "USD")]
            usd: String,
            #[serde(rename = "DMC")]
            dmc: Option<String>,
        }
        
        #[derive(Debug, Deserialize)]
        struct DMarketResponse {
            cursor: Option<String>, // Added cursor as per user example
            #[serde(default)]
            objects: Vec<DMarketMarketItem>,
            #[serde(default)]
            items: Vec<DMarketMarketItem>, 
            total: String, // Changed from TotalCounts to String
        }
        
        let dmarket_response = match serde_json::from_str::<DMarketResponse>(&response_text) {
            Ok(response) => response,
            Err(e) => {
                debug!("Failed to parse DMarket market items response: {}. Raw: {}", e, response_text);
                return Err(DMarketError::JsonError(e));
            }
        };
        
        let items_to_process = if !dmarket_response.objects.is_empty() {
            dmarket_response.objects
        } else {
            dmarket_response.items // Fallback if objects is empty but items is not
        };
        
        let total_items_str = dmarket_response.total;
        
        let market_items = items_to_process
            .into_iter()
            .map(|item| MarketItem {
                item_id: item.item_id,
                item_type: item.other.get("type").and_then(|v| v.as_str()).unwrap_or("offer").to_string(),
                title: item.title,
                description: item.other.get("description").and_then(|v| v.as_str()).map(String::from),
                slug: item.other.get("slug").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                status: item.other.get("status").and_then(|v| v.as_str()).unwrap_or("active").to_string(),
                ownersCount: item.other.get("ownersCount").and_then(|v| v.as_i64()).map(|v| v as i32),
                image: item.other.get("image").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                class_id: item.class_id,
                game: item.other.get("game").and_then(|v| v.as_str()).unwrap_or("").to_string(), 
                price: Price {
                    amount: item.price.usd.clone(),
                    currency: "USD".to_string(), // Assuming currency is USD from ApiPrice, adjust if needed
                },
                suggested_price: item.price.dmc.as_ref().map(|dmc_price| Price { amount: dmc_price.clone(), currency: "DMC".to_string() }),
                discount: item.other.get("discount").and_then(|v| v.as_f64()),
                extra: MarketItemExtra { 
                    gameId: Some(item.game_id.clone()), 
                    name_color: item.other.get("nameColor").and_then(|v| v.as_str()).map(String::from),
                    background_color: item.other.get("backgroundColor").and_then(|v| v.as_str()).map(String::from),
                    category: item.other.get("category").and_then(|v| v.as_str()).map(String::from),
                    exterior: item.other.get("exterior").and_then(|v| v.as_str()).map(String::from),
                    category_path: item.other.get("categoryPath").and_then(|v| v.as_str()).map(String::from),
                    tradable: item.other.get("tradable").and_then(|v| v.as_bool()),
                    daysBeforeTrade: item.other.get("daysBeforeTrade").and_then(|v| v.as_i64()).map(|v| v as i32),
                    floatValue: item.other.get("floatValue").and_then(|v| v.as_f64()),
                },
                attributes: Vec::new(), // Assuming attributes are not directly in this response or need separate handling
                locked: item.other.get("locked").and_then(|v| v.as_bool()).unwrap_or(false),
                createdAt: item.other.get("createdAt").and_then(|v| v.as_i64()).unwrap_or(0),
                updatedAt: item.other.get("updatedAt").and_then(|v| v.as_i64()).unwrap_or(0),
                inMarket: item.other.get("inMarket").and_then(|v| v.as_bool()).unwrap_or(true),
                gameId: item.game_id, // This is the gameId from the item itself
                withdrawable: item.other.get("withdrawable").and_then(|v| v.as_bool()).unwrap_or(true),
                tradeLock: item.other.get("tradeLock").and_then(|v| v.as_i64()).map(|v| v as i32),
                offer_type: item.other.get("offerType").and_then(|v| v.as_str()).map(String::from),
                asset_id: item.other.get("assetId").and_then(|v| v.as_str()).map(String::from),
            })
            .collect();
        
        Ok(MarketItemsResponse {
            objects: market_items,
            total: total_items_str, // Pass the string total
        })
    }

    /// Searches for market items based on a query.
    pub async fn search_market_items(&self, query: &str, currency: &str, limit: u32, offset: u32, game_id: Option<&str>) -> Result<MarketItemsResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        
        let encoded_query = urlencoding::encode(query);
        let mut path = format!("/exchange/v1/market/items?currency={}&limit={}&offset={}&title={}", currency, limit, offset, encoded_query);
        
        if let Some(game_id) = game_id {
            path = format!("{}&gameId={}", path, game_id);
        } else {
            path = format!("{}&gameId=a411", path); 
        }

        debug!("Generating signature for market search request: path='{}'", path);
        let signature = self.client.generate_signature(&timestamp, method, &path, "")?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);

        let response = self.client.http_client.get(&url).headers(headers).send().await?;

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
            other: HashMap<String, serde_json::Value>,
        }
        
        #[derive(Debug, Deserialize)]
        struct ApiPrice {
            #[serde(rename = "USD")]
            usd: String,
            #[serde(rename = "DMC")]
            dmc: Option<String>,
        }
        
        #[derive(Debug, Deserialize)]
        #[allow(non_snake_case)]
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
                debug!("Failed to parse DMarket search response: {}. Raw: {}", e, response_text);
                return Err(DMarketError::JsonError(e));
            }
        };
        
        let items_to_process = if !dmarket_response.objects.is_empty() {
            dmarket_response.objects
        } else {
            dmarket_response.items
        };
        
        let total_items = dmarket_response.total.offers;
        
        let market_items = items_to_process
            .into_iter()
            .map(|item| MarketItem {
                item_id: item.item_id,
                item_type: item.other.get("type").and_then(|v| v.as_str()).unwrap_or("offer").to_string(),
                title: item.title,
                description: item.other.get("description").and_then(|v| v.as_str()).map(String::from),
                slug: item.other.get("slug").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                status: item.other.get("status").and_then(|v| v.as_str()).unwrap_or("active").to_string(),
                ownersCount: item.other.get("ownersCount").and_then(|v| v.as_i64()).map(|v| v as i32),
                image: item.other.get("image").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                class_id: item.class_id,
                game: item.other.get("game").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                price: Price {
                    amount: item.price.usd.clone(),
                    currency: "USD".to_string(),
                },
                suggested_price: item.price.dmc.as_ref().map(|dmc_price| Price { amount: dmc_price.clone(), currency: "DMC".to_string() }),
                discount: item.other.get("discount").and_then(|v| v.as_f64()),
                extra: MarketItemExtra { 
                    gameId: Some(item.game_id.clone()),
                    name_color: item.other.get("nameColor").and_then(|v| v.as_str()).map(String::from),
                    background_color: item.other.get("backgroundColor").and_then(|v| v.as_str()).map(String::from),
                    category: item.other.get("category").and_then(|v| v.as_str()).map(String::from),
                    exterior: item.other.get("exterior").and_then(|v| v.as_str()).map(String::from),
                    category_path: item.other.get("categoryPath").and_then(|v| v.as_str()).map(String::from),
                    tradable: item.other.get("tradable").and_then(|v| v.as_bool()),
                    daysBeforeTrade: item.other.get("daysBeforeTrade").and_then(|v| v.as_i64()).map(|v| v as i32),
                    floatValue: item.other.get("floatValue").and_then(|v| v.as_f64()),
                },
                attributes: Vec::new(), 
                locked: item.other.get("locked").and_then(|v| v.as_bool()).unwrap_or(false),
                createdAt: item.other.get("createdAt").and_then(|v| v.as_i64()).unwrap_or(0),
                updatedAt: item.other.get("updatedAt").and_then(|v| v.as_i64()).unwrap_or(0),
                inMarket: item.other.get("inMarket").and_then(|v| v.as_bool()).unwrap_or(true),
                gameId: item.game_id,
                withdrawable: item.other.get("withdrawable").and_then(|v| v.as_bool()).unwrap_or(true),
                tradeLock: item.other.get("tradeLock").and_then(|v| v.as_i64()).map(|v| v as i32),
                offer_type: item.other.get("offerType").and_then(|v| v.as_str()).map(String::from),
                asset_id: item.other.get("assetId").and_then(|v| v.as_str()).map(String::from),
            })
            .collect();
        
        Ok(MarketItemsResponse {
            objects: market_items,
            total: total_items.to_string(), // Convert i32 to String to match updated MarketItemsResponse
        })
    }

    /// Retrieves a list of available games.
    pub async fn get_games(&self) -> Result<Vec<Game>, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = "/exchange/v1/games";
        
        debug!("Generating signature for games request");
        let signature = self.client.generate_signature(&timestamp, method, path, "")?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);

        let response = self.client.http_client.get(&url).headers(headers).send().await?;

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
        struct DMarketApiGameItem {
            id: String,
            title: String, 
            slug: Option<String>,
            #[serde(rename = "logoImageUrl")]
            logo_image_url: Option<String>,
            // Capturing other fields that might be useful or for debugging
            #[serde(rename = "type")]
            game_type: Option<String>,
            #[serde(rename = "homeImageURL")]
            home_image_url: Option<String>,
            #[serde(rename = "offersCount")]
            offers_count: Option<i32>,
            #[serde(rename = "isReleased")]
            is_released: Option<bool>,
            #[serde(rename = "authMethod")]
            auth_method: Option<String>,
            maintenance: Option<bool>,
            #[serde(flatten)]
            other: HashMap<String, serde_json::Value>,
        }
        
        #[derive(Debug, Deserialize)]
        struct DMarketGamesApiResponse {
            objects: Vec<DMarketApiGameItem>,
            total: i32,
        }
        
        match serde_json::from_str::<DMarketGamesApiResponse>(&response_text) {
            Ok(api_response) => {
                let games = api_response.objects
                    .into_iter()
                    .map(|api_game| {
                        Game { 
                            id: api_game.id,
                            title: api_game.title,
                            logo: api_game.logo_image_url.unwrap_or_default(),
                            slug: api_game.slug.unwrap_or_default(),
                            status: api_game.other.get("status").and_then(|s| s.as_str()).unwrap_or("active").to_string(),
                        }
                    })
                    .collect();
                Ok(games)
            },
            Err(e) => {
                error!("Failed to parse games response: {}. Raw text: {}", e, response_text);
                Err(DMarketError::JsonError(e))
            }
        }
    }

    /// Retrieves details for a specific market item class.
    pub async fn get_market_item_details(&self, class_id: &str, limit: u32, offset: u32, currency: &str) -> Result<MarketItemsResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = format!("/offers-search/v1/aggregated-class/{}/sell-offers?limit={}&offset={}&currency={}", class_id, limit, offset, currency);

        debug!("Generating signature for market item details request");
        let signature = self.client.generate_signature(&timestamp, method, &path, "")?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);

        let response = self.client.http_client.get(&url).headers(headers).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }
        
        // If status is success, response is not consumed yet.
        let item_details = response.json::<MarketItemsResponse>().await?;
        Ok(item_details)
    }
} 