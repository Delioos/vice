use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    #[serde(rename = "publicName")]
    pub public_name: String,
    pub balance: Vec<Balance>,
    #[serde(rename = "lastOnline")]
    pub last_online: String,
    #[serde(rename = "registrationDate")]
    pub registration_date: String,
    pub status: String,
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