use crate::dmarket::client::{DMarketClient, API_BASE_URL};
use crate::dmarket::error::DMarketError;
use crate::dmarket::models::{CreateTargetRequest, CreateTargetResponse, DeleteTargetRequest, TargetListResponse};
use chrono::Utc;
use log::{debug, error};

/// Handles target-related API endpoints (create, list, delete targets).
pub struct TargetHandler<'a> {
    client: &'a DMarketClient,
}

impl<'a> TargetHandler<'a> {
    /// Creates a new TargetHandler.
    pub fn new(client: &'a DMarketClient) -> Self {
        Self { client }
    }

    /// Creates one or more new targets.
    pub async fn create_target(&self, request: &CreateTargetRequest) -> Result<CreateTargetResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "POST";
        let path = "/target-predictor/v1/target";
        let body = serde_json::to_string(request)?;

        debug!("Generating signature for create target request");
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

        let create_response = response.json::<CreateTargetResponse>().await?;
        Ok(create_response)
    }

    /// Retrieves a list of the user's targets.
    pub async fn get_targets(&self, limit: u32, offset: u32) -> Result<TargetListResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "GET";
        let path = format!("/target-predictor/v1/user/targets?limit={}&offset={}", limit, offset);

        debug!("Generating signature for get targets request");
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

        let targets = response.json::<TargetListResponse>().await?;
        Ok(targets)
    }

    /// Deletes one or more targets.
    /// Note: DMarket API for deleting targets uses the HTTP DELETE method.
    pub async fn delete_targets(&self, target_ids: Vec<String>) -> Result<CreateTargetResponse, DMarketError> {
        let timestamp = Utc::now().timestamp().to_string();
        let method = "DELETE"; // Correct HTTP method for delete
        let path = "/target-predictor/v1/target";
        
        // DeleteTargetRequest is defined in models.rs
        let request_payload = DeleteTargetRequest { targets: target_ids };
        let body = serde_json::to_string(&request_payload)?;

        debug!("Generating signature for delete targets request");
        let signature = self.client.generate_signature(&timestamp, method, path, &body)?;
        let headers = self.client.create_headers(&timestamp, &signature)?;

        debug!("Making request to DMarket API with headers: {:?}", headers);
        let url = format!("{}{}", API_BASE_URL, path);
        debug!("Request URL: {}", url);
        debug!("Request body: {}", body);

        let response = self.client.http_client.delete(&url) // Using .delete() method
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
        // DMarket might return a specific response for delete; CreateTargetResponse is a placeholder if not.
        // Often, successful DELETE operations might return 204 No Content or a summary response.
        // Assuming CreateTargetResponse is what DMarket returns based on original client structure.
        let delete_response = response.json::<CreateTargetResponse>().await?;
        Ok(delete_response)
    }
} 