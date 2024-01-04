use serde::{Deserialize, Serialize};

use crate::{
    price::{PackagePrice, Price},
    types::{Platforms, ReleaseDate},
};

#[derive(Deserialize)]
pub(crate) struct PackageData {
    pub success: bool,
    pub data: Option<PackageDetails>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct PackageDetails {
    #[serde(skip_deserializing)]
    pub pkg_id: u64,
    pub name: String,
    pub page_image: String,
    pub small_logo: String,
    /// Array of apps the package contains.
    pub apps: Vec<PackageApp>,
    pub price: PackagePrice,
    pub platforms: Platforms,
    pub controller: Controller,
    pub release_date: ReleaseDate,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct PackageApp {
    #[serde(rename(deserialize = "id"))]
    pub app_id: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Controller {
    pub full_gamepad: bool,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct DlcData {
    pub(crate) status: i8,
    #[serde(rename(deserialize = "appid"))]
    pub app_id: String,
    pub name: String,
    pub dlc: Option<Vec<DlcDetails>>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct DlcDetails {
    #[serde(rename(deserialize = "id"))]
    pub dlc_id: u64,
    pub name: String,
    pub header_image: String,
    pub price_overview: Price,
    pub platforms: Platforms,
    pub release_date: ReleaseDate,
    pub controller_support: Option<String>,
}
