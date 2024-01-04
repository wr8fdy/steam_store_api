use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::{
    price::AppPrice,
    types::{Platforms, ReleaseDate},
};

#[derive(Deserialize)]
pub(crate) struct AppData {
    pub data: Option<AppDetails>,
    pub success: bool,
}

/// Contains information about steam application
#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct AppDetails {
    /// Observed values: "game", "dlc", "demo", "advertising", "mod", "video".
    pub r#type: String,
    pub name: String,
    #[serde(rename(deserialize = "steam_appid"))]
    pub app_id: u64,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub required_age: Option<u8>,
    pub is_free: Option<bool>,
    pub controller_support: Option<String>,
    /// Array of app ids.
    pub dlc: Option<Vec<i64>>,
    pub detailed_description: Option<String>,
    pub about_the_game: Option<String>,
    pub short_description: Option<String>,
    pub supported_languages: Option<String>,
    pub header_image: Option<String>,
    pub capsule_image: Option<String>,
    pub capsule_imagev5: Option<String>,
    pub website: Option<String>,
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub pc_requirements: Option<Requirements>,
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub mac_requirements: Option<Requirements>,
    #[serde(deserialize_with = "deserialize_default_from_empty_object")]
    pub linux_requirements: Option<Requirements>,
    pub legal_notice: Option<String>,
    pub developers: Option<Vec<String>>,
    pub publishers: Option<Vec<String>>,
    pub price_overview: Option<AppPrice>,
    pub packages: Option<Vec<u64>>,
    pub platforms: Option<Platforms>,
    pub metacritic: Option<Metacritic>,
    pub categories: Option<Vec<Categorie>>,
    pub genres: Option<Vec<Genre>>,
    pub screenshots: Option<Vec<Screenshot>>,
    pub movies: Option<Vec<Movie>>,
    pub recommendations: Option<Recommendations>,
    pub achievements: Option<Achievements>,
    pub release_date: Option<ReleaseDate>,
    pub support_info: Option<SupportInfo>,
    pub background: Option<String>,
    pub background_raw: Option<String>,
    pub content_descriptors: Option<ContentDescriptors>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct ContentDescriptors {
    pub ids: Vec<u64>,
    pub notes: Option<String>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct SupportInfo {
    pub url: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Achievement {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Achievements {
    pub total: u32,
    pub highlighted: Option<Vec<Achievement>>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct MovieFormat {
    #[serde(rename(deserialize = "480"))]
    pub x480: String,
    pub max: String,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Movie {
    pub id: u64,
    pub name: String,
    pub thumbnail: String,
    pub highlight: bool,
    pub webm: MovieFormat,
    pub mp4: MovieFormat,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Screenshot {
    pub id: u64,
    pub path_thumbnail: String,
    pub path_full: String,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Metacritic {
    pub score: u8,
    pub url: String,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Requirements {
    pub minimum: Option<String>,
    pub recommended: Option<String>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Recommendations {
    pub total: u64,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Categorie {
    /// Steam categorie id
    pub id: u8,
    pub description: String,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Genre {
    /// Steam genre id
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct GenreData {
    pub status: i8,
    pub genres: Vec<Genre>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct App {
    #[serde(rename(deserialize = "appid"))]
    /// Steam application id
    pub app_id: u64,
    /// Application name
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppsIn {
    pub(crate) status: i8,
    /// ID of genre or category
    pub id: String,
    /// Name of genre or category
    pub name: String,
    /// List of tabs with items, e.g. Top Sellers, Specials
    pub tabs: Option<HashMap<String, Tab>>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Tab {
    pub name: String,
    pub total_item_count: u64,
    pub items: Vec<TabItem>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct TabItem {
    /// Game, dlc, movie etc
    pub r#type: u8,
    /// Application id
    #[serde(rename(deserialize = "id"))]
    pub app_id: u64,
}
