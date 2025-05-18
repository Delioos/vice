use crate::dmarket::client::{DMarketClient, API_BASE_URL};
use crate::dmarket::error::DMarketError;
use crate::dmarket::models::{InventoryResponse, InventoryItem, InventoryItemExtra};
use chrono::Utc;
use log::{debug, error};
use serde::Deserialize;
use std::collections::HashMap; // For DMarketInventoryItem's `other` field

/// Handles user inventory-related API endpoints.
pub struct InventoryHandler<'a> {
    client: &'a DMarketClient,
}

impl<'a> InventoryHandler<'a> {
    /// Creates a new InventoryHandler.
    pub fn new(client: &'a DMarketClient) -> Self {
        Self { client }
    }

    /// Retrieves the user's inventory.
    pub async fn get_inventory(&self, limit: u32, offset: u32, game_id: Option<&str>) -> Result<InventoryResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        
        // Corrected path based on documentation/previous fixes
        let mut path = format!("/exchange/v1/user/inventory?limit={}&offset={}", limit, offset);
        if let Some(game_id) = game_id {
            path = format!("{}&gameId={}", path, game_id);
        }
        
        debug!("Generating signature for inventory request");
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
        debug!("Inventory response: {}", response_text);
        
        #[derive(Debug, Deserialize)]
        struct DMarketInventoryItem {
            #[serde(rename = "itemId")]
            item_id: String,
            name: String,
            #[serde(rename = "classId")]
            class_id: String,
            #[serde(rename = "gameId")]
            game_id: String, // This field is present in DMarket's response
            #[serde(flatten)]
            other: HashMap<String, serde_json::Value>,
        }
        
        #[derive(Debug, Deserialize)]
        struct DMarketInventoryApiResponse {
            #[serde(default)]
            items: Vec<DMarketInventoryItem>,
            #[serde(default = "default_total")]
            total: i32,
        }
        
        fn default_total() -> i32 { 0 }
        
        match serde_json::from_str::<DMarketInventoryApiResponse>(&response_text) {
            Ok(dmarket_inventory) => {
                let inventory_items = dmarket_inventory.items
                    .into_iter()
                    .map(|item| {
                        InventoryItem {
                            item_id: item.item_id,
                            title: item.name,
                            status: item.other.get("status").and_then(|s| s.as_str()).unwrap_or("active").to_string(),
                            image: item.other.get("image").and_then(|s| s.as_str()).unwrap_or("").to_string(),
                            // game field in our InventoryItem can be populated from item.game_id or related info
                            game: item.game_id.clone(), // Using game_id directly, or map it to a game title if needed
                            class_id: item.class_id,
                            extra: InventoryItemExtra {
                                exterior: item.other.get("exterior").and_then(|s| s.as_str()).map(String::from),
                                category: item.other.get("category").and_then(|s| s.as_str()).map(String::from),
                                category_path: item.other.get("categoryPath").and_then(|s| s.as_str()).map(String::from),
                                name_color: item.other.get("nameColor").and_then(|s| s.as_str()).map(String::from),
                                background_color: item.other.get("backgroundColor").and_then(|s| s.as_str()).map(String::from),
                                tradable: item.other.get("tradable").and_then(|b| b.as_bool()),
                                daysBeforeTrade: item.other.get("daysBeforeTrade").and_then(|n| n.as_i64()).map(|n| n as i32),
                                floatValue: item.other.get("floatValue").and_then(|n| n.as_f64()),
                            },
                            inMarket: item.other.get("inMarket").and_then(|b| b.as_bool()).unwrap_or(false),
                            locked: item.other.get("locked").and_then(|b| b.as_bool()).unwrap_or(false),
                        }
                    })
                    .collect();
                
                Ok(InventoryResponse {
                    objects: inventory_items,
                    total: dmarket_inventory.total,
                })
            },
            Err(e) => {
                debug!("Failed to parse inventory response: {}. Raw: {}", e, response_text);
                Err(DMarketError::JsonError(e))
            }
        }
    }
} 