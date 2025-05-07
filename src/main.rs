use backend::dmarket::DMarketClient;
use log::{error, info};

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();
    info!("Starting DMarket API client");

    match DMarketClient::new() {
        Ok(client) => {
            info!("Trying user profile endpoint...");
            match client.get_user_profile().await {
                Ok(profile) => {
                    info!("Successfully retrieved user profile:");
                    println!("User Profile: {:#?}", profile);
                }
                Err(e) => {
                    error!("Failed to get user profile: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to initialize DMarket client: {}", e);
        }
    }
}
