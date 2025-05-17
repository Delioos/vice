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