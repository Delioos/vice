use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoodsBuyOrderResponse {
    pub code: String,
    pub data: Option<GoodsBuyOrderData>,
    pub msg: Option<String>,
    // pub request_id: String, // This seems to be missing in some responses
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoodsBuyOrderData {
    pub items: Vec<Item>,
    pub page_num: i32,
    pub page_size: i32,
    pub total_count: i32,
    pub total_page: i32,
    // pub user_infos: serde_json::Value, // Assuming this can be dynamic or we might need specific structs later
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub app_id: i32,
    pub asset_info: Option<AssetInfo>,
    pub bookmarked: bool,
    pub buy_max_price: Option<String>,
    pub buy_num: i32,
    pub can_bargain: bool,
    pub can_search_intent: bool,
    pub created_at: i64,
    pub goods_id: i64,
    pub id: String,
    pub updated_at: i64,
    pub user_id: i64,
    pub price: String, // This was 'url' in your python example, but 'price' seems more standard from API responses
    pub state: i32,    // 0: pending, 1: active etc. (need to confirm exact states)
    pub supported_pay_method: i32,
    pub trade_max_price: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetInfo {
    pub appid: i32,
    pub assetid: String,
    pub classid: String,
    pub goods_id: i64, 
    pub instanceid: String,
    pub market_hash_name: String,
    // pub paintwear: Option<String>, // exterior wear, e.g. "0.06..."
    // pub inspect_details: Option<InspectDetails>, // More detailed info, if needed
    // pub sticker_info: Option<Vec<Sticker>> // Sticker details
}

// Structs for https://api.buff.market/api/market/goods endpoint

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketGoodsResponse {
    pub code: String,
    pub data: Option<MarketGoodsData>,
    pub msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketGoodsData {
    pub items: Vec<MarketGoodsItem>,
    pub page_num: i32,
    pub page_size: i32,
    pub total_count: i32,
    pub total_page: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketGoodsItem {
    pub appid: i32,
    #[serde(rename = "id")] // Assuming 'id' is the goods_id for the item itself in this context
    pub goods_internal_id: i64, // Renamed to avoid conflict if used alongside other IDs. The example shows "id": 1092
    pub name: String,
    pub market_hash_name: String,
    pub sell_min_price: String, // Price in a string format, e.g., "605"
    pub steam_price: Option<String>,
    pub steam_price_cny: Option<String>,
    pub icon_url: String,
    pub original_icon_url: Option<String>,
    pub goods_info: Option<GoodsInfo>, // Detailed info, might need further refinement
    pub info: Option<ItemInfoContainer>, // Contains tags
    pub bookmarked: Option<bool>,
    pub buy_max_price: Option<String>,
    pub buy_num: Option<i32>,
    pub can_bargain: Option<bool>,
    pub sell_num: Option<i32>,
    pub steam_market_url: Option<String>,
    pub transacted_num: Option<i32>, // Or i64 if it can be large
    pub short_name: Option<String>,
    pub has_buff_price_history: Option<bool>,
    // Add other fields from the JSON as needed, marking them optional if they might be missing
    // e.g., auction_num, can_search_by_tournament, description, game, item_id, market_min_price etc.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoodsInfo {
    // Based on user example: goods_info: {,…}
    // This suggests it might be a complex object. For now, let's assume it might contain various details.
    // If its structure is consistent, define specific fields. Otherwise, serde_json::Value might be an option.
    // Example fields that *might* be here based on typical market data:
    pub icon_url: Option<String>,
    pub original_icon_url: Option<String>,
    pub steam_price: Option<String>,
    pub steam_price_cny: Option<String>,
    // Add other relevant fields if known
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemInfoContainer {
    pub tags: Option<ItemTags>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemTags {
    pub category: Option<TagDetails>,
    pub exterior: Option<TagDetails>,
    pub quality: Option<TagDetails>,
    pub rarity: Option<TagDetails>,
    #[serde(rename = "type")]
    pub type_tag: Option<TagDetails>, // Renamed 'type' to 'type_tag' to avoid Rust keyword conflict
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagDetails {
    pub category: String,
    pub id: i64, // Or String if it can be non-numeric
    pub internal_name: String,
    pub localized_name: String,
} 