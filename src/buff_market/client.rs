use reqwest::header::{HeaderMap, COOKIE, USER_AGENT, HeaderValue};
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};

use super::endpoints;
use super::error::BuffMarketError;
use super::models::{GoodsBuyOrderResponse, MarketGoodsResponse, MarketGoodsItem};

const DEFAULT_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

const X_CSRFTOKEN_HEADER: &str = "x-csrftoken";

#[derive(Debug, Clone)]
pub struct BuffMarketClient {
    client: Client,
    session_cookie: String,
    csrf_token: String,
}

impl BuffMarketClient {
    pub fn new(session_cookie: String, csrf_token: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, DEFAULT_USER_AGENT.parse().unwrap());
        // It's important to set the cookie for requests to buff.market
        // The specific cookie needed is typically obtained by logging into buff.163.com
        // and copying it from the browser's developer tools (Network tab -> Request Headers -> cookie).
        // Example format: "Device-Id=...; Locale-Supported=...; game=...; NTES_YD_SESS=...; S_INFO=...; P_INFO=...; remember_me=...; session=...; csrf_token=..."

        let client = Client::builder()
            .default_headers(headers)
            // .cookie_store(true) // Consider if reqwest's cookie store is beneficial here
            .build()
            .unwrap(); // Consider error handling for client creation

        BuffMarketClient {
            client,
            session_cookie,
            csrf_token,
        }
    }

    fn get_timestamp_ms() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }

    pub async fn get_buy_orders(
        &self,
        game: &str,
        goods_id: i64,
        page_num: i32,
    ) -> Result<GoodsBuyOrderResponse, BuffMarketError> {
        // The timestamp parameter `_` seems important.
        let timestamp = Self::get_timestamp_ms();
        let url = format!(
            "{}?game={}&goods_id={}&page_num={}&_={}",
            endpoints::BUFF_BUY_ORDERS_API_URL, // We'll define this in endpoints/mod.rs
            game,
            goods_id,
            page_num,
            timestamp
        );

        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, self.session_cookie.parse().map_err(|_| BuffMarketError::InvalidInput("Invalid session cookie format".to_string()))?);


        let response = self
            .client
            .get(&url)
            .headers(headers) // Send the cookie with this specific request
            .send()
            .await?;

        if response.status().is_success() {
            let body = response.text().await?;
            // For debugging:
            // println!("Raw JSON response: {}", body);
            let parsed_response: GoodsBuyOrderResponse = serde_json::from_str(&body)
                .map_err(BuffMarketError::JsonParse)?;

            if parsed_response.code != "OK" { // "OK" is a common success code, but verify for Buff
                 // Log the raw body if parsing succeeds but API indicates error
                // eprintln!("API Error. Raw response: {}", body);
                return Err(BuffMarketError::ApiError {
                    message: parsed_response.msg.unwrap_or_else(|| "Unknown API error".to_string()),
                });
            }
            Ok(parsed_response)
        } else {
            // If the status is not a success, `error_for_status()` will consume the response
            // and return an `Err(reqwest::Error)`.
            // We then explicitly convert this `reqwest::Error` into our `BuffMarketError::HttpRequest`.
            match response.error_for_status() {
                Ok(_) => {
                    // This case should ideally not be reached if status().is_success() was false.
                    // If it somehow is, it means error_for_status found a success status code (e.g. after a redirect)
                    // that we didn't initially catch. This is unlikely.
                    Err(BuffMarketError::Unknown)
                }
                Err(err) => Err(BuffMarketError::HttpRequest(err)),
            }
        }
    }

    pub async fn get_market_listings(
        &self,
        game: &str,
        page_num: i32,
        page_size: i32,
    ) -> Result<MarketGoodsResponse, BuffMarketError> {
        let url = format!(
            "{}?game={}&page_num={}&page_size={}",
            endpoints::BUFF_MARKET_GOODS_API_URL,
            game,
            page_num,
            page_size
        );

        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, self.session_cookie.parse().map_err(|_| BuffMarketError::InvalidInput("Invalid session cookie format".to_string()))?);
        headers.insert(X_CSRFTOKEN_HEADER, self.csrf_token.parse().map_err(|_| BuffMarketError::InvalidInput("Invalid CSRF token format".to_string()))?);
        headers.insert(reqwest::header::ORIGIN, HeaderValue::from_static("https://buff.market"));
        headers.insert(reqwest::header::REFERER, HeaderValue::from_static("https://buff.market/"));

        let response = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        if response.status().is_success() {
            let body = response.text().await?;
            let parsed_response: MarketGoodsResponse = serde_json::from_str(&body)
                .map_err(|e| {
                    BuffMarketError::JsonParse(e)
                })?;

            if parsed_response.code != "OK" {
                return Err(BuffMarketError::ApiError {
                    message: parsed_response.msg.unwrap_or_else(|| "Unknown API error".to_string()),
                });
            }
            Ok(parsed_response)
        } else {
            match response.error_for_status() {
                Ok(_) => Err(BuffMarketError::Unknown),
                Err(err) => Err(BuffMarketError::HttpRequest(err)),
            }
        }
    }

    pub async fn get_all_market_listings(
        &self,
        game: &str,
        page_size: i32,
    ) -> Result<Vec<MarketGoodsItem>, BuffMarketError> {
        let mut all_items = Vec::new();
        let mut current_page = 1;
        let mut total_pages = 1;

        loop {
            if current_page > total_pages && total_pages != 0 {
                break;
            }

            let response = self.get_market_listings(game, current_page, page_size).await?;
            
            if let Some(data) = response.data {
                if data.items.is_empty() {
                    // If items list is empty, check conditions to break
                    if data.total_count == 0 { // No items at all for this query
                        break;
                    }
                    // If not the first page, or if total_pages suggests we shouldn't expect more
                    if current_page >= total_pages && total_pages != 0 { 
                        break;
                    }
                    // If it's the first page and it's empty, but total_count > 0, something is off or we wait.
                    // For now, if items are empty and it's not total_count 0, we might continue if total_pages is high,
                    // but it's safer to break if we are at/past total_pages.
                }
                all_items.extend(data.items); // Now extend after potential checks
                total_pages = data.total_page;
                
                if data.total_count == 0 { // Double check, if total_count is 0, definitely no items.
                    break; 
                }
            } else {
                if response.code == "OK" {
                    if current_page == 1 {
                        return Ok(all_items);
                    } else {
                        break;
                    }
                }
                return Err(BuffMarketError::MissingData(format!(
                    "No data found for game {} on page {} using get_market_listings. API msg: {:?}",
                    game, current_page, response.msg
                )));
            }

            current_page += 1;
            if current_page > total_pages && total_pages !=0 {
                 break;
            }

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
        Ok(all_items)
    }
}

// Example of how one might try to get *all* listings for a particular goods_id.
// This would require knowing the total_pages from a first call, then iterating.
// pub async fn get_all_buy_orders_for_item(
//     &self,
//     game: &str,
//     goods_id: i64,
// ) -> Result<Vec<super::models::Item>, BuffMarketError> {
//     let mut all_items = Vec::new();
//     let mut current_page = 1;
//     let mut total_pages = 1; // Initialize to 1 to make at least one call

//     loop {
//         if current_page > total_pages {
//             break;
//         }

//         let response = self.get_buy_orders(game, goods_id, current_page).await?;
//         if let Some(data) = response.data {
//             all_items.extend(data.items);
//             total_pages = data.total_page; // Update total_pages from the response
//             current_page += 1;
//         } else {
//             // No data, or an API error code might have been returned.
//             // If response.code was not "OK", get_buy_orders would have returned Err already.
//             // This handles cases where 'data' is None even if code is "OK".
//             return Err(BuffMarketError::MissingData(format!(
//                 "No data found for goods_id {} on page {}",
//                 goods_id, current_page
//             )));
//         }
//         // Basic rate limiting to be polite to the API
//         // tokio::time::sleep(std::time::Duration::from_millis(200)).await;
//     }
//     Ok(all_items)
// } 