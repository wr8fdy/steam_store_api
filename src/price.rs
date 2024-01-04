use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Featured {
    pub featured_win: Vec<FeaturedItem>,
    pub featured_mac: Vec<FeaturedItem>,
    pub featured_linux: Vec<FeaturedItem>,
    pub(crate) status: i32,
}

#[derive(Deserialize)]
pub(crate) struct FeaturedCategories {
    pub status: i8,
    #[serde(flatten)]
    pub featured_categories: HashMap<String, FeaturedCategorie>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct FeaturedCategorie {
    pub id: String,
    pub name: String,
    pub items: Option<Vec<FeaturedItem>>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct FeaturedItem {
    #[serde(rename(deserialize = "id"))]
    pub app_id: usize,
    pub r#type: u8,
    pub name: String,
    pub discounted: bool,
    pub discount_percent: u8,
    /// Pre-discount application price.
    pub original_price: Option<u32>,
    /// Post-discount application price.
    pub final_price: u32,
    /// What currency prices are denoted in.
    pub currency: String,
    pub large_capsule_image: String,
    pub small_capsule_image: String,
    pub windows_available: bool,
    pub mac_available: bool,
    pub linux_available: bool,
    pub streamingvideo_available: bool,
    /// Unix timestamp of when the discount noted above expires.
    /// Is not provided if the app is not discounted.
    pub discount_expiration: Option<usize>,
    pub header_image: String,
    pub controller_support: Option<String>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Price {
    /// What currency prices are denoted in.
    pub currency: String,
    /// Pre-discount application price.
    pub initial: u32,
    /// Post-discount application price.
    pub r#final: u32,
    pub discount_percent: u8,
}

#[derive(Deserialize)]
pub(crate) struct PriceData {
    pub data: Option<PriceOverview>,
    pub success: bool,
}

#[derive(Deserialize)]
pub(crate) struct PriceOverview {
    pub price_overview: AppPrice,
}

/// Parsed application price.
#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct AppPrice {
    /// Steam application ID
    #[serde(skip_deserializing)]
    pub app_id: u64,
    pub final_formatted: String,
    pub initial_formatted: String,
    #[serde(flatten)]
    pub price: Price,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct PackagePrice {
    pub individual: u32,
    #[serde(flatten)]
    pub price: Price,
}
