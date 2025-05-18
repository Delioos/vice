//! This module provides the DMarket API client and related structures.

pub mod client;
pub mod error;
pub mod models;
pub mod endpoints;

// For convenience, re-export main client and error types if desired,
// or users can access them via client::DMarketClient and error::DMarketError.
// pub use client::DMarketClient;
// pub use error::DMarketError;

// Re-export all models for easier access.
pub use models::*;

pub use models::{
    UserProfile, Balance, ApiResponse, ApiError,
    MarketItemsResponse, MarketItem, Price, MarketItemExtra, MarketItemAttribute,
    Game,
    InventoryResponse, InventoryItem, InventoryItemExtra,
    SellOfferRequest, SellOfferItem, SellOfferPrice, SellOfferResponse, SellOfferResponseItem,
    BuyOfferRequest, BuyOfferResponse, BuyOfferResponseItem,
    ImportRequest, ImportItem, ImportResponse, ImportedItem,
    ExportRequest, ExportResponse, ExportedItem,
    Target, CreateTargetRequest, CreateTargetResponse, CreateTargetResponseItem, TargetListResponse, DeleteTargetRequest,
}; 