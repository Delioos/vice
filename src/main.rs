use log::{error, info, debug};
use crate::dmarket::client::DMarketClient;

mod dmarket;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();
    info!("Starting DMarket API client");

    match DMarketClient::new() {
        Ok(client) => {
            // Get user profile
            info!("Trying user profile endpoint...");
            match client.get_user_profile().await {
                Ok(profile) => {
                    info!("Successfully retrieved user profile:");
                    info!("User ID: {}", profile.id);
                    info!("Username: {}", profile.username);
                    info!("Email: {}", profile.email);
                }
                Err(e) => {
                    error!("Failed to get user profile: {}", e);
                }
            }
            
            // Get account balance
            info!("Retrieving account balance...");
            match client.get_account_balance().await {
                Ok(balances) => {
                    info!("Successfully retrieved account balance:");
                    for balance in balances {
                        info!("{}: {}", balance.currency, balance.amount);
                    }
                }
                Err(e) => {
                    error!("Failed to get account balance: {}", e);
                }
            }
            
            // Get market items (CS2 items)
            info!("Retrieving market items for CS2...");
            match client.get_market_items("USD", 5, 0, Some("a411")).await {
                Ok(market_items) => {
                    info!("Successfully retrieved {} market items out of {}", market_items.objects.len(), market_items.total);
                    for item in market_items.objects {
                        info!("Item: {} - {} {}", item.title, item.price.amount, item.price.currency);
                    }
                }
                Err(e) => {
                    error!("Failed to get market items: {}", e);
                    debug!("Error details: {:?}", e);
                }
            }
            
            // Search market items
            info!("Searching market items for 'AWP'...");
            match client.search_market_items("AWP", "USD", 5, 0, Some("a411")).await {
                Ok(search_results) => {
                    info!("Successfully searched for market items: found {} items", search_results.objects.len());
                    for item in search_results.objects {
                        info!("Found: {} - {} {}", item.title, item.price.amount, item.price.currency);
                    }
                }
                Err(e) => {
                    error!("Failed to search market items: {}", e);
                    debug!("Error details: {:?}", e);
                }
            }
            
            // Get available games
            info!("Retrieving available games...");
            match client.get_games().await {
                Ok(games) => {
                    info!("Successfully retrieved {} games", games.len());
                    for game in games {
                        info!("Game: {} (ID: {})", game.title, game.id);
                    }
                }
                Err(e) => {
                    error!("Failed to get games: {}", e);
                    debug!("Error details: {:?}", e);
                }
            }
            
            // Get inventory
            info!("Retrieving inventory...");
            match client.get_inventory(10, 0, Some("a411")).await {
                Ok(inventory) => {
                    info!("Successfully retrieved {} inventory items out of {}", inventory.objects.len(), inventory.total);
                    for item in inventory.objects {
                        info!("Inventory item: {} (ID: {})", item.title, item.item_id);
                    }
                }
                Err(e) => {
                    error!("Failed to get inventory: {}", e);
                    debug!("Error details: {:?}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to initialize DMarket client: {}", e);
        }
    }
}
