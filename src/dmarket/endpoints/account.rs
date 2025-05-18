use crate::dmarket::client::{DMarketClient, API_BASE_URL};
use crate::dmarket::error::DMarketError;
use crate::dmarket::models::{Balance, UserProfile};
use chrono::Utc;
use log::{debug, error};

/// Handles account-related API endpoints.
pub struct AccountHandler<'a> {
    client: &'a DMarketClient,
}

impl<'a> AccountHandler<'a> {
    /// Creates a new AccountHandler.
    pub fn new(client: &'a DMarketClient) -> Self {
        Self { client }
    }

    /// Retrieves the user profile information.
    pub async fn get_user_profile(&self) -> Result<UserProfile, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = "/account/v1/user";
        let body = "";

        debug!("Generating signature for user profile request");
        let signature = self.client.generate_signature(&timestamp, method, path, body)?;
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

        let profile = response.json::<UserProfile>().await?;
        Ok(profile)
    }

    /// Retrieves the raw user profile information as a JSON string.
    pub async fn get_user_profile_raw(&self) -> Result<String, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = "/account/v1/user";
        let body = "";

        debug!("Generating signature for user profile request");
        let signature = self.client.generate_signature(&timestamp, method, path, body)?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);

        let response = self.client.http_client.get(&url).headers(headers).send().await?;

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

    /// Retrieves the account balance for various currencies.
    pub async fn get_account_balance(&self) -> Result<Vec<Balance>, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = "/account/v1/balance";
        let body = "";

        debug!("Generating signature for balance request");
        let signature = self.client.generate_signature(&timestamp, method, path, body)?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);

        let response = self.client.http_client.get(&url).headers(headers).send().await?;

        let status = response.status();
        let response_text = response.text().await?;
        debug!("Response: {}", response_text);

        if !status.is_success() {
            error!("API returned error status {}: {}", status, response_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, response_text
            )));
        }

        // The balance endpoint returns a JSON object, not an array
        // Example: {"dmc":"0.00","dmcAvailableToWithdraw":"0.00","usd":"0.00","usdAvailableToWithdraw":"0.00"}
        let balance_response: serde_json::Value = serde_json::from_str(&response_text)?;

        let mut balances = Vec::new();
        
        if let Some(usd) = balance_response.get("usd") {
            if let Some(usd_str) = usd.as_str() {
                balances.push(Balance {
                    amount: usd_str.to_string(),
                    currency: "USD".to_string(),
                });
            }
        }
        
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
} 