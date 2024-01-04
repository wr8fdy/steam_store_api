//! Steam Store
//! ===========
//!
//! An API client for the [unofficial Steam Storefront](https://wiki.teamfortress.com/wiki/User:RJackson/StorefrontAPI)
//! resource, which provides methods to retrieve product information from the platform.
//! # Example
//!
//! <br>
//!
//! ```rust
//! #
//! use anyhow::Result;
//! use steam_store_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = SteamBuilder::new()
//!         .with_country_code("US")
//!         .with_language(&Language::English)
//!         .build()?;
//!     let app = client.app(&219990_u64).await?;
//!     println!("{:#?} - {:#?}", &app.app_id, &app.name);
//!     anyhow::Ok(())
//! }
//! ```
//!
//! <br>
//!

#![deny(warnings)]

/// Apps info.
pub mod app;
/// Apps package and DLC types.
pub mod package;
/// Price and featured info.
pub mod price;
/// User's reviews for apps.
pub mod review;
/// API client
pub mod steam;
/// Contains helpers for types.
pub mod types;
/// Prelude module, contains the most needed helpers from this library.
pub mod prelude {
    pub use crate::app::*;
    pub use crate::package::*;
    pub use crate::price::*;
    pub use crate::review::*;
    pub use crate::steam::*;
    pub use crate::types::*;
}
