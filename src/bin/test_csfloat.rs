use log::{error, info, debug};
use backend::csfloat::client::CSFloatClient;
use backend::csfloat::endpoints::listings::ListingsQuery;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();
    info!("Starting CSFloat API client test");

    match CSFloatClient::new() {
        Ok(client) => {
            // Test getting listings
            info!("Fetching CSFloat listings...");
            let query = ListingsQuery {
                limit: Some(5),
                page: Some(0),
                sort_by: Some("most_recent".to_string()),
                category: None,
                def_index: None,
                min_float: None,
                max_float: None,
                rarity: None,
                paint_seed: None,
                paint_index: None,
                user_id: None,
                collection: None,
                min_price: None,
                max_price: None,
                market_hash_name: None,
                type_: None,
                stickers: None,
            };

            match client.listings().get_listings(Some(query)).await {
                Ok(listings_response) => {
                    info!("Successfully retrieved {} listings", listings_response.data.len());
                    for listing in listings_response.data {
                        let float_display = match listing.item.float_value {
                            Some(float) => format!("{}", float),
                            None => "N/A".to_string(),
                        };
                        
                        info!("Listing: {} - ${:.2} - Float: {}", 
                             listing.item.market_hash_name, 
                             listing.price as f64 / 100.0, 
                             float_display);
                    }
                }
                Err(e) => {
                    error!("Failed to get listings: {}", e);
                }
            }

            // Test getting a specific listing if we have an ID
            // Note: This requires knowing a valid listing ID
            /*
            info!("Fetching a specific listing...");
            match client.listings().get_listing("some-listing-id").await {
                Ok(listing) => {
                    info!("Successfully retrieved listing: {}", listing.item.market_hash_name);
                }
                Err(e) => {
                    error!("Failed to get specific listing: {}", e);
                }
            }
            */
        }
        Err(e) => {
            error!("Failed to initialize CSFloat client: {}", e);
        }
    }
} 