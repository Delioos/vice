use serde::{Deserialize, Serialize};
use crate::csfloat::client::CSFloatClient;
use crate::csfloat::error::CSFloatError;
use crate::csfloat::models::{Listing, ListingResponse};

/// Query parameters for listing search
#[derive(Debug, Serialize, Deserialize)]
pub struct ListingsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sort_by")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "def_index")]
    pub def_index: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "min_float")]
    pub min_float: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "max_float")]
    pub max_float: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "paint_seed")]
    pub paint_seed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "paint_index")]
    pub paint_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "user_id")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "min_price")]
    pub min_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "max_price")]
    pub max_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "market_hash_name")]
    pub market_hash_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickers: Option<String>,
}

/// Request body for creating a new listing
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListingRequest {
    #[serde(rename = "asset_id")]
    pub asset_id: i64,
    #[serde(rename = "type")]
    pub type_: String,
    pub price: i64,
    #[serde(skip_serializing_if = "Option::is_none", rename = "max_offer_discount")]
    pub max_offer_discount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reserve_price")]
    pub reserve_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "duration_days")]
    pub duration_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
}

/// Handler for CSFloat listings endpoints
pub struct ListingsHandler<'a> {
    client: &'a CSFloatClient,
}

impl<'a> ListingsHandler<'a> {
    pub(crate) fn new(client: &'a CSFloatClient) -> Self {
        Self { client }
    }

    /// Get all listings with optional query parameters
    pub async fn get_listings(&self, query: Option<ListingsQuery>) -> Result<ListingResponse, CSFloatError> {
        let endpoint = if let Some(query) = query {
            let query_string = serde_urlencoded::to_string(query)?;
            format!("/listings?{}", query_string)
        } else {
            "/listings".to_string()
        };

        self.client.get(&endpoint).await
    }

    /// Get a specific listing by ID
    pub async fn get_listing(&self, id: &str) -> Result<Listing, CSFloatError> {
        let endpoint = format!("/listings/{}", id);
        self.client.get(&endpoint).await
    }

    /// Create a new listing
    pub async fn create_listing(&self, request: CreateListingRequest) -> Result<Listing, CSFloatError> {
        self.client.post("/listings", &request).await
    }
} 