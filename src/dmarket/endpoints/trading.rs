use crate::dmarket::client::{DMarketClient, API_BASE_URL};
use crate::dmarket::error::DMarketError;
use crate::dmarket::models::*;
use chrono::Utc;
use log::{debug, error};
use serde::Serialize;

/// Handles trading-related API endpoints (sell, buy, import, export, offers).
pub struct TradingHandler<'a> {
    client: &'a DMarketClient,
}

impl<'a> TradingHandler<'a> {
    /// Creates a new TradingHandler.
    pub fn new(client: &'a DMarketClient) -> Self {
        Self { client }
    }

    /// Creates a new sell offer for one or more items.
    pub async fn create_sell_offer(&self, request: &SellOfferRequest) -> Result<SellOfferResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/trading/v1/sell-offer";
        let body = serde_json::to_string(request)?;

        debug!("Generating signature for create sell offer request");
        let signature = self.client.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.http_client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let sell_response = response.json::<SellOfferResponse>().await?;
        Ok(sell_response)
    }

    /// Buys one or more existing sell offers.
    pub async fn buy_offer(&self, request: &BuyOfferRequest) -> Result<BuyOfferResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/trading/v1/buy/offers";
        let body = serde_json::to_string(request)?;

        debug!("Generating signature for buy offer request");
        let signature = self.client.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.http_client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let buy_response = response.json::<BuyOfferResponse>().await?;
        Ok(buy_response)
    }

    /// Imports items from an external inventory (e.g., Steam) to DMarket.
    pub async fn import_items(&self, request: &ImportRequest) -> Result<ImportResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/trading/v1/import";
        let body = serde_json::to_string(request)?;

        debug!("Generating signature for import items request");
        let signature = self.client.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.http_client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let import_response = response.json::<ImportResponse>().await?;
        Ok(import_response)
    }

    /// Exports items from DMarket to an external inventory.
    pub async fn export_items(&self, request: &ExportRequest) -> Result<ExportResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/trading/v1/export";
        let body = serde_json::to_string(request)?;

        debug!("Generating signature for export items request");
        let signature = self.client.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.http_client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let export_response = response.json::<ExportResponse>().await?;
        Ok(export_response)
    }

    /// Cancels one or more active sell offers.
    pub async fn cancel_sell_offer(&self, offer_ids: Vec<String>) -> Result<SellOfferResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST"; // DMarket API docs say DELETE, but examples use POST for /offers/cancel
        let path = "/trading/v1/offers/cancel";
        
        #[derive(Debug, Serialize)]
        struct CancelRequest {
            offers: Vec<String>,
        }
        
        let request_body = CancelRequest { offers: offer_ids };
        let body = serde_json::to_string(&request_body)?;

        debug!("Generating signature for cancel offer request");
        let signature = self.client.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        // Using POST as per common practice for this endpoint despite some docs ambiguity
        let response = self.client.http_client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let cancel_response = response.json::<SellOfferResponse>().await?;
        Ok(cancel_response)
    }

    /// Updates the price of an active sell offer.
    pub async fn update_offer_price(&self, offer_id: &str, new_price: &SellOfferPrice) -> Result<SellOfferResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/trading/v1/offers/update-price";
        
        #[derive(Debug, Serialize)]
        struct UpdatePriceItem {
            offerId: String, // Matches DMarket API field name
            price: SellOfferPrice,
        }

        #[derive(Debug, Serialize)]
        struct UpdatePriceRequest {
            offers: Vec<UpdatePriceItem>,
        }
        
        let request_body = UpdatePriceRequest { 
            offers: vec![UpdatePriceItem { 
                offerId: offer_id.to_string(), 
                price: new_price.clone()
            }] 
        };
        let body = serde_json::to_string(&request_body)?;

        debug!("Generating signature for update price request");
        let signature = self.client.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.http_client.post(&url)
            .headers(headers)
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API returned error status {}: {}", status, error_text);
            return Err(DMarketError::ApiError(format!(
                "API returned error status {}: {}",
                status, error_text
            )));
        }

        let update_response = response.json::<SellOfferResponse>().await?;
        Ok(update_response)
    }

    /// Retrieves a list of the user's active sell offers.
    pub async fn get_user_sell_offers(&self, limit: u32, offset: u32, game_id: Option<&str>, status: Option<&str>) -> Result<MarketItemsResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        
        let mut path = format!("/offers-search/v1/user/sell-offers?limit={}&offset={}", limit, offset);
        
        if let Some(game_id) = game_id {
            path = format!("{}&gameId={}", path, game_id);
        }
        
        if let Some(status_filter) = status {
            path = format!("{}&status={}", path, status_filter);
        }

        debug!("Generating signature for user sell offers request");
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
        // Assuming this endpoint returns data compatible with MarketItemsResponse.
        // If parsing fails, dedicated DMarket* temporary structs might be needed here too.
        let sell_offers = response.json::<MarketItemsResponse>().await?;
        Ok(sell_offers)
    }
} 