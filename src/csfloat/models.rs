use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub id: String,
    pub created_at: String,
    #[serde(rename = "type")]
    pub type_: ListingType,
    pub price: i64,
    pub state: ListingState,
    pub seller: Seller,
    pub item: Item,
    pub is_seller: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_offer_price: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_offer_discount: Option<i64>,
    pub is_watchlisted: bool,
    pub watchers: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<Reference>,
}

// The API returns data wrapped in a data field
#[derive(Debug, Serialize, Deserialize)]
pub struct ListingResponse {
    pub data: Vec<Listing>,
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    pub amount: i64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    pub base_price: Option<i64>,
    pub predicted_price: Option<i64>,
    pub quantity: Option<i64>,
    pub last_updated: Option<String>,
    pub float_factor: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub asset_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint_seed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_value: Option<f64>,
    pub icon_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_stattrak: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_souvenir: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i32>,
    pub market_hash_name: String,
    #[serde(default)]
    pub stickers: Vec<Sticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradable: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspect_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_screenshot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scm: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wear_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<String>,
    #[serde(default)]
    pub badges: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_commodity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Seller {
    pub avatar: Option<String>,
    pub flags: i32,
    pub online: bool,
    pub stall_public: bool,
    pub statistics: SellerStatistics,
    pub steam_id: Option<String>,
    pub username: Option<String>,
    pub away: Option<bool>,
    pub has_valid_steam_api_key: Option<bool>,
    pub obfuscated_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerStatistics {
    pub median_trade_time: i32,
    pub total_failed_trades: i32,
    pub total_trades: i32,
    pub total_verified_trades: i32,
    pub total_avoided_trades: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sticker {
    #[serde(rename = "stickerId")]
    pub sticker_id: i32,
    pub slot: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wear: Option<f64>,
    pub icon_url: String,
    pub name: String,
    pub scm: Option<Price>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<StickerReference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StickerReference {
    pub price: Option<i64>,
    pub quantity: Option<i64>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListingType {
    BuyNow,
    Auction,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListingState {
    Listed,
    Sold,
    Cancelled,
    Expired,
} 