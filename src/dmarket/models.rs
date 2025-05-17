use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "sagaPublicKey")]
    pub saga_public_key: String,
    pub username: String,
    pub email: String,
    #[serde(rename = "isEmailVerified")]
    pub is_email_verified: bool,
    #[serde(rename = "isPasswordSet")]
    pub is_password_set: bool,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub level: i32,
    #[serde(rename = "countryCodeFromIP")]
    pub country_code_from_ip: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    pub settings: UserSettings,
    #[serde(rename = "steamAccount")]
    pub steam_account: SteamAccount,
    #[serde(rename = "agreementsInfo")]
    pub agreements_info: AgreementsInfo,
    #[serde(rename = "regType")]
    pub reg_type: String,
    #[serde(rename = "hasHistoryEvents")]
    pub has_history_events: bool,
    #[serde(rename = "ga_client_id")]
    pub ga_client_id: String,
    pub migrated: bool,
    #[serde(rename = "hasActiveSubscriptions")]
    pub has_active_subscriptions: bool,
    #[serde(rename = "linkedGames")]
    pub linked_games: Vec<String>,
    pub features: Vec<Feature>,
    pub restrictions: Vec<String>,
    #[serde(rename = "twitchAccount")]
    pub twitch_account: TwitchAccount,
    #[serde(rename = "instagramAccount")]
    pub instagram_account: InstagramAccount,
    #[serde(rename = "twitterAccount")]
    pub twitter_account: TwitterAccount,
    #[serde(rename = "ethereumAccount")]
    pub ethereum_account: EthereumAccount,
    pub labels: Option<serde_json::Value>,
    pub storefront: Storefront,
    #[serde(rename = "tinNotRequired")]
    pub tin_not_required: bool,
    #[serde(rename = "promoToken")]
    pub promo_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    #[serde(rename = "enabledDeviceConfirmation")]
    pub enabled_device_confirmation: bool,
    #[serde(rename = "tradingApiToken")]
    pub trading_api_token: String,
    #[serde(rename = "isSubscribedToNewsletters")]
    pub is_subscribed_to_newsletters: bool,
    #[serde(rename = "targetsLimit")]
    pub targets_limit: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamAccount {
    #[serde(rename = "steamId")]
    pub steam_id: String,
    pub icon: String,
    #[serde(rename = "tradeUrl")]
    pub trade_url: String,
    #[serde(rename = "isValidTradeURL")]
    pub is_valid_trade_url: bool,
    pub username: String,
    #[serde(rename = "isProfilePrivate")]
    pub is_profile_private: bool,
    #[serde(rename = "tradingStatus")]
    pub trading_status: String,
    pub level: i32,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "apiKeyStatus")]
    pub api_key_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgreementsInfo {
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    pub updated: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchAccount {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub icon: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstagramAccount {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterAccount {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EthereumAccount {
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storefront {
    pub disabled: bool,
    pub alias: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Price {
    pub amount: String,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    pub currency: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: Option<String>,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

// New models for market items

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketItemsResponse {
    pub objects: Vec<MarketItem>,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketItem {
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub title: String,
    pub description: Option<String>,
    pub slug: String,
    pub status: String,
    pub ownersCount: Option<i32>,
    pub image: String,
    #[serde(rename = "classId")]
    pub class_id: String,
    pub game: String,
    pub price: Price,
    #[serde(rename = "suggestedPrice")]
    pub suggested_price: Option<Price>,
    pub discount: Option<f64>,
    pub extra: MarketItemExtra,
    pub attributes: Vec<MarketItemAttribute>,
    pub locked: bool,
    pub createdAt: i64,
    pub updatedAt: i64,
    pub inMarket: bool,
    pub gameId: String,
    pub withdrawable: bool,
    pub tradeLock: Option<i32>,
    #[serde(rename = "offerType")]
    pub offer_type: Option<String>,
    #[serde(rename = "assetId")]
    pub asset_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketItemExtra {
    #[serde(rename = "nameColor")]
    pub name_color: Option<String>,
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    pub category: Option<String>,
    pub exterior: Option<String>,
    #[serde(rename = "categoryPath")]
    pub category_path: Option<String>,
    pub tradable: Option<bool>,
    pub daysBeforeTrade: Option<i32>,
    pub floatValue: Option<f64>,
    pub gameId: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketItemAttribute {
    pub name: String,
    pub category: String,
    pub value: String,
    #[serde(rename = "displayValue")]
    pub display_value: Option<String>,
}

// Game models
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub logo: String,
    pub slug: String,
    pub status: String,
}

// Inventory models
#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryResponse {
    pub objects: Vec<InventoryItem>,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItem {
    #[serde(rename = "itemId")]
    pub item_id: String,
    pub title: String,
    pub status: String,
    pub image: String,
    pub game: String,
    #[serde(rename = "classId")]
    pub class_id: String,
    pub extra: InventoryItemExtra,
    pub inMarket: bool,
    pub locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItemExtra {
    pub exterior: Option<String>,
    pub category: Option<String>,
    #[serde(rename = "categoryPath")]
    pub category_path: Option<String>,
    #[serde(rename = "nameColor")]
    pub name_color: Option<String>,
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    pub tradable: Option<bool>,
    pub daysBeforeTrade: Option<i32>,
    pub floatValue: Option<f64>,
}

// Trading models
#[derive(Debug, Serialize, Deserialize)]
pub struct SellOfferRequest {
    pub items: Vec<SellOfferItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellOfferItem {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub price: SellOfferPrice,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SellOfferPrice {
    pub amount: String,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellOfferResponse {
    pub Items: Vec<SellOfferResponseItem>,
    pub HasErrors: bool,
    pub TotalSucceed: i32,
    pub TotalFailed: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SellOfferResponseItem {
    pub Status: String,
    pub AssetID: String,
    pub OfferID: Option<String>,
    pub ErrorCode: Option<String>,
    pub ErrorMessage: Option<String>,
}

// Buy offer models
#[derive(Debug, Serialize, Deserialize)]
pub struct BuyOfferRequest {
    pub offers: Vec<String>, // List of offer IDs
    pub currency: String,    // USD, DMC, etc.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyOfferResponse {
    pub Items: Vec<BuyOfferResponseItem>,
    pub HasErrors: bool,
    pub TotalSucceed: i32,
    pub TotalFailed: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyOfferResponseItem {
    pub Status: String,
    pub OfferID: String,
    pub ErrorCode: Option<String>,
    pub ErrorMessage: Option<String>,
}

// Import/Export models
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportRequest {
    pub gameId: String,
    pub items: Vec<ImportItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportItem {
    pub appId: String,
    pub contextId: String,
    pub assetId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResponse {
    pub OperationID: String,
    pub SteamTradeID: String,
    pub SteamTradeState: String,
    pub Items: Vec<ImportedItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportedItem {
    pub DMarketAssetID: String,
    pub SteamAppID: String,
    pub SteamContextID: String,
    pub SteamAssetID: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportRequest {
    pub gameId: String,
    pub assetIds: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportResponse {
    pub OperationID: String,
    pub SteamTradeID: String,
    pub State: String,
    pub Assets: Vec<ExportedItem>,
    pub ErrorMessage: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportedItem {
    pub DMarketAssetID: String,
    pub Title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Target {
    pub id: Option<String>,
    #[serde(rename = "targetType")]
    pub target_type: String,
    #[serde(rename = "gameId")]
    pub game_id: String,
    #[serde(rename = "classId")]
    pub class_id: String,
    pub title: Option<String>,
    pub price: Option<Price>,
    pub status: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i64>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    // Advanced targeting parameters
    pub phase: Option<String>,
    #[serde(rename = "floatPartValue")]
    pub float_part_value: Option<String>,
    #[serde(rename = "paintSeed")]
    pub paint_seed: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTargetRequest {
    pub targets: Vec<Target>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTargetResponse {
    pub Items: Vec<CreateTargetResponseItem>,
    pub HasErrors: bool,
    pub TotalSucceed: i32,
    pub TotalFailed: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTargetResponseItem {
    pub Status: String,
    pub TargetID: Option<String>,
    pub ErrorCode: Option<String>,
    pub ErrorMessage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetListResponse {
    pub objects: Vec<Target>,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTargetRequest {
    pub targets: Vec<String>,
} 