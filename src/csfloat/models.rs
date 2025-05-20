use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub id: String,
    pub created_at: String,
    pub type_: ListingType,
    pub price: Price,
    pub state: ListingState,
    pub seller: Seller,
    pub item: Item,
    pub is_seller: bool,
    pub min_offer_price: Option<Price>,
    pub max_offer_discount: Option<Price>,
    pub is_watchlisted: bool,
    pub watchers: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingResponse {
    pub listings: Vec<Listing>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    pub amount: i64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub asset_id: String,
    pub def_index: i32,
    pub paint_index: i32,
    pub paint_seed: i32,
    pub float_value: f64,
    pub icon_url: String,
    pub d_param: String,
    pub is_stattrak: bool,
    pub is_souvenir: bool,
    pub rarity: i32,
    pub quality: i32,
    pub market_hash_name: String,
    pub stickers: Vec<Sticker>,
    pub tradable: i32,
    pub inspect_link: String,
    pub has_screenshot: bool,
    pub scm: Option<Price>,
    pub item_name: String,
    pub wear_name: String,
    pub description: String,
    pub collection: String,
    pub badges: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Seller {
    pub avatar: Option<String>,
    pub flags: i32,
    pub online: bool,
    pub stall_public: bool,
    pub statistics: SellerStatistics,
    pub steam_id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellerStatistics {
    pub median_trade_time: i32,
    pub total_failed_trades: i32,
    pub total_trades: i32,
    pub total_verified_trades: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sticker {
    pub sticker_id: i32,
    pub slot: i32,
    pub icon_url: String,
    pub name: String,
    pub scm: Option<Price>,
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