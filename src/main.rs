use log::{error, info, debug};
use backend::BuffMarketClient;
use backend::CSFloatClient;
use backend::DMarketClient;

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
            match client.account().get_user_profile().await {
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
            match client.account().get_account_balance().await {
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
            match client.exchange().get_market_items(
                "a8db", // game_id (CS2)
                "USD",  // currency
                5,      // limit
                0,      // offset
                Some("title"), // order_by
                Some("desc"),  // order_dir
                None,   // title (for filtering by specific title)
                None,   // tree_filters
                None,   // price_from
                None,   // price_to
                None,   // types
                None    // cursor
            ).await {
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
            match client.exchange().search_market_items("AWP", "USD", 5, 0, Some("a8db")).await {
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
            match client.exchange().get_games().await {
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
            let cs2_id_dmarket =  "a8db";
            match client.inventory().get_inventory(10, 0, Some(cs2_id_dmarket)).await {
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

    // Example for Csfloat (assuming it exists and works)
    // let csfloat_client = CsfloatClient::new();
    // match csfloat_client.get_listings("AK-47 | Redline (Field-Tested)", None, None).await {
    //     Ok(listings) => println!("Csfloat listings: {:?}", listings.len()),
    //     Err(e) => eprintln!("Error fetching Csfloat listings: {:?}", e),
    // }
    
    // Example for BuffMarket (New endpoint: /api/market/goods)
    let buff_session_cookie = "your_buff_session_cookie_here".to_string(); 
    let buff_csrf_token = "your_buff_csrf_token_here".to_string(); // Get this from your browser dev tools
    let buff_game_name = "csgo";
    let items_per_page = 20; // How many items to fetch per API call in get_all_market_listings

    if buff_session_cookie == "your_buff_session_cookie_here" || buff_csrf_token == "your_buff_csrf_token_here" {
        eprintln!("Please update main.rs with your buff.market session cookie AND CSRF token to test BuffMarketClient new endpoint.");
    } else {
        let buff_client = BuffMarketClient::new(buff_session_cookie.clone(), buff_csrf_token.clone());
        println!("Testing BuffMarket API (all market listings) for game: {}", buff_game_name);

        match buff_client.get_all_market_listings(buff_game_name, items_per_page).await {
            Ok(all_items) => {
                println!("Successfully fetched all market listings from BuffMarket.");
                println!("Total items found: {}", all_items.len());
                for item in all_items.iter().take(10) { // Print first 10 items as a sample
                    println!(
                        "  Item: {} (ID: {}), Price: {}, Sell Count: {:?}",
                        item.market_hash_name,
                        item.goods_internal_id,
                        item.sell_min_price,
                        item.sell_num
                    );
                    if let Some(info) = &item.info {
                        if let Some(tags) = &info.tags {
                            if let Some(ext) = &tags.exterior {
                                println!("    Exterior: {}", ext.localized_name);
                            }
                            if let Some(qual) = &tags.quality {
                                println!("    Quality: {}", qual.localized_name);
                            }
                        }
                    }
                }
                if all_items.len() > 10 {
                    println!("... and {} more items.", all_items.len() - 10);
                }
            }
            Err(e) => {
                eprintln!("Error fetching all BuffMarket market listings: {:?}", e);
            }
        }

        // Example of fetching a single page (optional, can be commented out)
        // println!("\nTesting BuffMarket API (single page of market listings) for game: {}", buff_game_name);
        // match buff_client.get_market_listings(buff_game_name, 1, 5).await { // page 1, 5 items
        //     Ok(response) => {
        //         println!("Successfully fetched single page from BuffMarket.");
        //         if let Some(data) = response.data {
        //             println!("Page: {}/{}, Total items on page: {}, Total Count: {}", 
        //                 data.page_num, data.total_page, data.items.len(), data.total_count);
        //             for item in data.items.iter().take(5) {
        //                 println!("  Item: {} Price: {}", item.market_hash_name, item.sell_min_price);
        //             }
        //         } else {
        //             println!("Response OK, but no data field in BuffMarket response (single page).");
        //         }
        //     }
        //     Err(e) => {
        //         eprintln!("Error fetching single page of BuffMarket listings: {:?}", e);
        //     }
        // }

    }
}
