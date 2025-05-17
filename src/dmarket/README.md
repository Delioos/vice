# DMarket API Client for Rust

A Rust client library for interacting with the [DMarket API](https://docs.dmarket.com/v1/swagger.html). This library provides a simple, intuitive way to access DMarket's API for trading skins and virtual items.

## Features

- Robust Ed25519 authentication with the DMarket API
- Support for both 32-byte and 64-byte private keys
- Comprehensive error handling
- Typed data structures for all API responses
- Asynchronous API using Tokio and Reqwest

## Supported Endpoints

- Account
  - User Profile
  - Account Balance
- Marketplace
  - List Market Items
  - Search Market Items
- Inventory
  - Get Inventory Items
- Trading
  - Create Sell Offers
  - Buy Offers
  - Cancel Sell Offers
  - Update Offer Prices
- Import/Export
  - Import Items (from Steam)
  - Export Items (to Steam)
- Games
  - List Available Games

## Setup

1. Add this library to your Cargo.toml:

```toml
[dependencies]
dmarket = { path = "path/to/this/library" }
tokio = { version = "1", features = ["full"] }
```

2. Create a `.env` file in your project root with your DMarket API credentials:

```
DMARKET_PUBLIC_KEY=your_public_key
DMARKET_PRIVATE_KEY=your_private_key
```

## Usage Examples

### Initialize the client

```rust
use dmarket::DMarketClient;

#[tokio::main]
async fn main() {
    match DMarketClient::new() {
        Ok(client) => {
            // Use the client here
        }
        Err(e) => {
            eprintln!("Failed to initialize DMarket client: {}", e);
        }
    }
}
```

### Get user profile

```rust
match client.get_user_profile().await {
    Ok(profile) => {
        println!("User ID: {}", profile.id);
        println!("Username: {}", profile.username);
        println!("Email: {}", profile.email);
    }
    Err(e) => {
        eprintln!("Failed to get user profile: {}", e);
    }
}
```

### Get account balance

```rust
match client.get_account_balance().await {
    Ok(balances) => {
        for balance in balances {
            println!("{}: {}", balance.currency, balance.amount);
        }
    }
    Err(e) => {
        eprintln!("Failed to get account balance: {}", e);
    }
}
```

### Get market items

```rust
// Parameters: currency, limit, offset
match client.get_market_items("USD", 10, 0).await {
    Ok(market_items) => {
        println!("Total items: {}", market_items.total);
        for item in market_items.objects {
            println!("Item: {} - {} {}", item.title, item.price.amount, item.price.currency);
        }
    }
    Err(e) => {
        eprintln!("Failed to get market items: {}", e);
    }
}
```

### Search market items

```rust
// Parameters: query, currency, limit
match client.search_market_items("AWP", "USD", 5).await {
    Ok(market_items) => {
        for item in market_items.objects {
            println!("Found: {} - {} {}", item.title, item.price.amount, item.price.currency);
        }
    }
    Err(e) => {
        eprintln!("Failed to search market items: {}", e);
    }
}
```

### Get inventory

```rust
// Parameters: limit, offset, game_id (optional)
match client.get_inventory(10, 0, Some("csgo")).await {
    Ok(inventory) => {
        for item in inventory.objects {
            println!("Inventory item: {} (ID: {})", item.title, item.item_id);
        }
    }
    Err(e) => {
        eprintln!("Failed to get inventory: {}", e);
    }
}
```

### Create sell offer

```rust
use dmarket::{SellOfferRequest, SellOfferItem, SellOfferPrice};

let request = SellOfferRequest {
    items: vec![
        SellOfferItem {
            asset_id: "your_asset_id".to_string(),
            price: SellOfferPrice {
                amount: "10.00".to_string(),
                currency: "USD".to_string(),
            },
        },
    ],
};

match client.create_sell_offer(&request).await {
    Ok(response) => {
        println!("Successfully created {} offer(s)", response.TotalSucceed);
        for item in response.Items {
            println!("Offer ID: {:?}", item.OfferID);
        }
    }
    Err(e) => {
        eprintln!("Failed to create sell offer: {}", e);
    }
}
```

### Buy offer

```rust
use dmarket::BuyOfferRequest;

let request = BuyOfferRequest {
    offers: vec!["offer_id_1".to_string(), "offer_id_2".to_string()],
    currency: "USD".to_string(),
};

match client.buy_offer(&request).await {
    Ok(response) => {
        println!("Successfully bought {} offer(s)", response.TotalSucceed);
    }
    Err(e) => {
        eprintln!("Failed to buy offers: {}", e);
    }
}
```

### Cancel sell offer

```rust
let offer_ids = vec!["offer_id_1".to_string(), "offer_id_2".to_string()];

match client.cancel_sell_offer(offer_ids).await {
    Ok(response) => {
        println!("Successfully cancelled {} offer(s)", response.TotalSucceed);
    }
    Err(e) => {
        eprintln!("Failed to cancel sell offers: {}", e);
    }
}
```

### Update offer price

```rust
use dmarket::SellOfferPrice;

let new_price = SellOfferPrice {
    amount: "15.00".to_string(),
    currency: "USD".to_string(),
};

match client.update_offer_price("offer_id", &new_price).await {
    Ok(response) => {
        println!("Successfully updated price for {} offer(s)", response.TotalSucceed);
    }
    Err(e) => {
        eprintln!("Failed to update offer price: {}", e);
    }
}
```

## Error Handling

The library uses a custom `DMarketError` type that handles various error cases:

- `ApiError`: Errors returned by the DMarket API
- `ReqwestError`: HTTP request errors
- `EnvError`: Environment variable errors
- `JsonError`: JSON parsing errors
- `HexError`: Errors in hexadecimal encoding/decoding
- `InvalidHeaderValue`: Errors in HTTP header values

## License

This project is licensed under the MIT License.
