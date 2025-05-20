pub mod dmarket;
pub mod csfloat;
pub mod buff_market;

pub use crate::dmarket::client::DMarketClient;
pub use crate::dmarket::error::DMarketError;
pub use crate::dmarket::models::{
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

pub use crate::csfloat::client::CSFloatClient;
pub use crate::csfloat::error::CSFloatError;
pub use crate::csfloat::models::{Listing, ListingResponse};
pub use crate::csfloat::endpoints::listings::{ListingsQuery, CreateListingRequest};

pub use crate::buff_market::BuffMarketClient;
pub use crate::buff_market::BuffMarketError;
// Potentially re-export models from buff_market too, if needed directly by consumers of the library
// pub use crate::buff_market::models::*; 