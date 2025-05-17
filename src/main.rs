use std::env;
use log::{error, info};
use crate::dmarket::client::DMarketClient;

mod dmarket;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();
    info!("Starting DMarket API client");

    match DMarketClient::new() {
        Ok(client) => {
            // First, try to get the raw profile response
            info!("Trying user profile endpoint (raw response)...");
            match client.get_user_profile_raw().await {
                Ok(text) => {
                    info!("Received raw response successfully");
                }
                Err(e) => {
                    error!("Failed to get user profile (raw): {}", e);
                }
            }
            
            // Now try to get the properly deserialized profile
            info!("Trying user profile endpoint (deserialized)...");
            match client.get_user_profile().await {
                Ok(profile) => {
                    info!("Successfully retrieved user profile:");
                    info!("User ID: {}", profile.id);
                    info!("Username: {}", profile.username);
                    info!("Email: {}", profile.email);
                }
                Err(e) => {
                    error!("Failed to get user profile (deserialized): {}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to initialize DMarket client: {}", e);
        }
    }
}
