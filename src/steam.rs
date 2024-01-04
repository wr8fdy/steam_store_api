use std::collections::HashMap;
use std::num::ParseIntError;

use thiserror::Error;
use url::Url;

use crate::app::{AppData, AppDetails, AppsIn, Genre, GenreData};
use crate::package::{DlcData, PackageData, PackageDetails};
use crate::price::{AppPrice, Featured, FeaturedCategorie, FeaturedCategories, PriceData};
use crate::review::{Reviews, ReviewsFilter};
use crate::types::Language;

/// Builder for Steam.
pub struct SteamBuilder {
    language: Option<Language>,
    country_code: Option<String>,
    store_url: Url,
}

impl Default for SteamBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SteamBuilder {
    pub fn new() -> Self {
        SteamBuilder {
            language: None,
            country_code: None,
            store_url: Url::parse("https://store.steampowered.com").unwrap(),
        }
    }

    pub fn build(self) -> Result<Steam, SteamErr> {
        if let Some(cc) = &self.country_code {
            if rust_iso3166::from_alpha2(cc).is_none() {
                return Err(SteamErr::IncorrectCountryCode);
            };
        };

        Ok(Steam {
            language: self.language,
            country_code: self.country_code,
            store_url: self.store_url,
            client: reqwest::Client::new(),
        })
    }

    /// ISO 3166-1 alpha-2 – two-letter country code:
    /// <https://en.wikipedia.org/wiki/ISO_3166-2>
    pub fn with_country_code(mut self, country_code: &str) -> Self {
        self.country_code = Some(country_code.to_uppercase());
        self
    }

    /// Language for localized strings.
    /// Takes English name of language (none of those fancy ISO distractions)
    pub fn with_language(mut self, language: &Language) -> Self {
        self.language = Some(language.clone());
        self
    }
}

/// API client for the Steam store
pub struct Steam {
    language: Option<Language>,
    country_code: Option<String>,
    store_url: Url,
    client: reqwest::Client,
}

/// Steam store API error types and error messages.
#[derive(Error, Debug)]
pub enum SteamErr {
    #[error("response with no data; this could be due to a rate limit")]
    ResponseWithNoData,
    #[error("response with no success")]
    ResponseWithNoSuccess,
    #[error("id {0} was not found in response")]
    IdNotFound(String),
    #[error("failed to parse country from country code")]
    IncorrectCountryCode,
    #[error(transparent)]
    RequestError(reqwest::Error),
    #[error(transparent)]
    UrlError(url::ParseError),
    #[error("failed to parse id: {0}")]
    ParseIdError(ParseIntError),
}

impl Steam {
    pub fn builder() -> SteamBuilder {
        SteamBuilder::new()
    }

    async fn send<T: for<'de> serde::Deserialize<'de>>(&self, mut url: Url) -> Result<T, SteamErr> {
        if let Some(l) = &self.language {
            url.query_pairs_mut().append_pair("l", l.as_ref());
        }

        if let Some(cc) = &self.country_code {
            url.query_pairs_mut().append_pair("cc", cc);
        }

        let res = self
            .client
            .get(url)
            .send()
            .await
            .map_err(SteamErr::RequestError)?;

        //res.error_for_status_ref().map_err(SteamErr::RequestError)?;

        let data: T = res.json().await.map_err(SteamErr::RequestError)?;

        Ok(data)
    }

    /// Get featured page from the Steam store
    pub async fn featured(&self) -> Result<Featured, SteamErr> {
        let url = self
            .store_url
            .join("api/featured/")
            .map_err(SteamErr::UrlError)?;

        let data: Featured = self.send(url).await?;

        if data.status != 1 {
            return Err(SteamErr::ResponseWithNoSuccess);
        }

        Ok(data)
    }

    /// Get list of genres
    pub async fn genres(&self) -> Result<Vec<Genre>, SteamErr> {
        let url = self
            .store_url
            .join("api/getgenrelist/")
            .map_err(SteamErr::UrlError)?;

        let data: GenreData = self.send(url).await?;

        if data.status != 1 {
            return Err(SteamErr::ResponseWithNoSuccess);
        }

        Ok(data.genres)
    }

    /// Get apps for genre, e.g. `action`, `rpg`
    /// Apps are divided by tabs, e.g. `topsellers` or `specials`
    pub async fn apps_in_genre(&self, genre: &str) -> Result<AppsIn, SteamErr> {
        let mut url = self
            .store_url
            .join("api/getappsingenre/")
            .map_err(SteamErr::UrlError)?;

        url.query_pairs_mut().extend_pairs(vec![("genre", &genre)]);

        let data: AppsIn = self.send(url).await?;

        if data.status != 1 {
            return Err(SteamErr::ResponseWithNoSuccess);
        }

        Ok(data)
    }

    /// Get apps for category, e.g. `cat_comingsoon`, `cat_newreleases`
    /// Apps are divided by tabs, e.g. `topsellers` or `specials`
    pub async fn apps_in_category(&self, category: &str) -> Result<AppsIn, SteamErr> {
        let mut url = self
            .store_url
            .join("api/getappsincategory/")
            .map_err(SteamErr::UrlError)?;

        url.query_pairs_mut()
            .extend_pairs(vec![("category", &category)]);

        let data: AppsIn = self.send(url).await?;

        if data.status != 1 {
            return Err(SteamErr::ResponseWithNoSuccess);
        }

        Ok(data)
    }

    /// Get featured categories with prices, e.g Specials, Top Sellers
    pub async fn featured_categories(
        &self,
    ) -> Result<HashMap<String, FeaturedCategorie>, SteamErr> {
        let url = self
            .store_url
            .join("api/featuredcategories/")
            .map_err(SteamErr::UrlError)?;

        let data: FeaturedCategories = self.send(url).await?;

        if data.status != 1 {
            return Err(SteamErr::ResponseWithNoSuccess);
        }

        Ok(data.featured_categories)
    }

    /// Information about application packages (not bundles)
    pub async fn package(&self, pkg_id: &u64) -> Result<PackageDetails, SteamErr> {
        let id = pkg_id.to_string();

        let mut url = self
            .store_url
            .join("api/packagedetails/")
            .map_err(SteamErr::UrlError)?;

        url.query_pairs_mut()
            .extend_pairs(vec![("packageids", &id)]);

        let mut data: HashMap<String, PackageData> = self.send(url).await?;

        let data = match data.remove(&id) {
            Some(data) => data,
            None => return Err(SteamErr::IdNotFound(id)),
        };

        if !data.success {
            return Err(SteamErr::ResponseWithNoSuccess);
        }

        match data.data {
            Some(mut d) => {
                d.pkg_id = *pkg_id;
                Ok(d)
            }
            None => Err(SteamErr::ResponseWithNoData),
        }
    }

    /// Get reviews for application with filters
    pub async fn reviews(&self, app_id: &u64, filter: &ReviewsFilter) -> Result<Reviews, SteamErr> {
        let mut url = self
            .store_url
            .join(&format!("appreviews/{}", app_id))
            .map_err(SteamErr::UrlError)?;

        url.query_pairs_mut().extend_pairs(filter.to_url_params());

        let data: Reviews = self.send(url).await?;

        if data.success != 1 {
            return Err(SteamErr::ResponseWithNoSuccess);
        }

        Ok(data)
    }

    /// Information about application DLCs
    pub async fn dlc(&self, app_id: &u64) -> Result<DlcData, SteamErr> {
        let id = app_id.to_string();

        let mut url = self
            .store_url
            .join("api/dlcforapp/")
            .map_err(SteamErr::UrlError)?;

        url.query_pairs_mut().extend_pairs(vec![("appid", &id)]);

        let data: DlcData = self.send(url).await?;

        if data.status != 1 {
            return Err(SteamErr::ResponseWithNoSuccess);
        }

        match data.dlc {
            Some(_) => Ok(data),
            None => Err(SteamErr::ResponseWithNoData),
        }
    }

    /// Detailed information about application
    pub async fn app(&self, app_id: &u64) -> Result<AppDetails, SteamErr> {
        let id = app_id.to_string();

        let mut url = self
            .store_url
            .join("api/appdetails/")
            .map_err(SteamErr::UrlError)?;

        url.query_pairs_mut().extend_pairs(vec![("appids", &id)]);

        let mut data: HashMap<String, AppData> = self.send(url).await?;

        let data = match data.remove(&id) {
            Some(data) => data,
            None => return Err(SteamErr::IdNotFound(id)),
        };

        if !data.success {
            return Err(SteamErr::ResponseWithNoSuccess);
        }

        match data.data {
            Some(d) => Ok(d),
            None => Err(SteamErr::ResponseWithNoData),
        }
    }

    /// Get price overview for a multiple applications
    pub async fn price<I: IntoIterator<Item = u64>>(
        &self,
        app_ids: I,
    ) -> Result<Vec<AppPrice>, SteamErr> {
        let ids: Vec<String> = app_ids.into_iter().map(|v| v.to_string()).collect();

        let mut url = self
            .store_url
            .join("api/appdetails/")
            .map_err(SteamErr::UrlError)?;

        url.query_pairs_mut().extend_pairs(vec![
            ("appids", &ids.join(",")),
            ("filters", &"price_overview".to_owned()),
        ]);

        let data: HashMap<String, PriceData> = self.send(url).await?;

        let mut out = Vec::with_capacity(data.len());

        for (k, v) in data {
            if v.success {
                if let Some(price) = v.data {
                    let mut p = price.price_overview;
                    p.app_id = k.parse().map_err(SteamErr::ParseIdError)?;
                    out.push(p);
                }
            }
        }

        out.shrink_to_fit();
        Ok(out)
    }
}
