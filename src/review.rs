use serde::{Deserialize, Serialize};

use crate::types::Language;

const MAX_REVIEW_NUM_PER_PAGE: u8 = 100;
const MAX_DAY_RANGE: u16 = 365;

#[derive(Default, Clone, Debug, Hash)]
pub struct ReviewsFilter {
    /// helpfulness, creation time or last updated time
    pub filter: Option<Filter>,
    /// all, positive or negative
    pub review_type: Option<ReviewType>,
    /// all, steam or non-steam
    pub purchase_type: Option<PurchaseType>,
    /// pass “all” for all reviews
    pub language: Option<Language>,
    /// range from now to n days ago to look for helpful reviews.
    /// Only applicable for the “all” filter. Maximum value is 365.
    pub day_range: Option<u16>,
    /// reviews are returned in batches of 20, so pass "*" for the first set,
    /// then the value of "cursor" that was returned in the response for the next set, etc.
    /// should be URL encoded
    pub cursor: Option<String>,
    /// by default, up to 20 reviews will be returned.
    /// More reviews can be returned based on this parameter
    /// (with a maximum of 100 reviews)
    pub num_per_page: Option<u8>,
    /// by default, off-topic reviews (aka "Review Bombs") are filtered out
    /// and are not returned in this API.
    pub offtopic_activity: OfftopicActivity,
}

impl ReviewsFilter {
    pub(crate) fn to_url_params(&self) -> Vec<(&str, String)> {
        let mut params: Vec<(&str, String)> = vec![("json", "1".to_owned())];

        if let Some(cursor) = &self.cursor {
            params.push(("cursor", cursor.to_owned()));
        };
        if let Some(filter) = &self.filter {
            params.push(("filter", filter.as_ref().to_owned()));
        };
        if let Some(language) = &self.language {
            params.push(("language", language.as_ref().to_owned()));
        };
        if let Some(review_type) = &self.review_type {
            params.push(("review_type", review_type.as_ref().to_owned()));
        };
        if let Some(purchase_type) = &self.purchase_type {
            params.push(("purchase_type", purchase_type.as_ref().to_owned()));
        };
        if let Some(day_range) = self.day_range {
            params.push(("day_range", day_range.max(MAX_DAY_RANGE).to_string()));
        };
        if let Some(num_per_page) = self.num_per_page {
            params.push((
                "num_per_page",
                num_per_page.max(MAX_REVIEW_NUM_PER_PAGE).to_string(),
            ));
        }

        if self.offtopic_activity == OfftopicActivity::Include {
            params.push(("filter_offtopic_activity", 0.to_string()));
        }

        params
    }
}

#[derive(Default, Clone, Debug, Hash)]
pub enum Filter {
    /// (default) sorted by helpfulness, with sliding windows based on day_range parameter,
    /// will always find results to return.
    #[default]
    All,
    /// sorted by creation time
    Recent,
    /// sorted by last updated time
    Updated,
}

impl AsRef<str> for Filter {
    fn as_ref(&self) -> &str {
        use Filter::*;

        match &self {
            Recent => "recent",
            Updated => "updated",
            All => "all",
        }
    }
}

#[derive(Default, Clone, Debug, Hash)]
pub enum ReviewType {
    /// all reviews (default)
    #[default]
    All,
    /// only positive reviews
    Positive,
    /// only negative reviews
    Negative,
}

impl AsRef<str> for ReviewType {
    fn as_ref(&self) -> &str {
        use ReviewType::*;

        match &self {
            Positive => "positive",
            Negative => "negative",
            All => "all",
        }
    }
}

#[derive(Default, Clone, Debug, Hash)]
pub enum PurchaseType {
    /// all reviews
    #[default]
    All,
    /// reviews written by users who did not pay for the product on Steam
    NonSteamPurchase,
    /// reviews written by users who paid for the product on Steam (default)
    Steam,
}

impl AsRef<str> for PurchaseType {
    fn as_ref(&self) -> &str {
        use PurchaseType::*;

        match &self {
            NonSteamPurchase => "nonsteampurchase",
            Steam => "steam",
            All => "all",
        }
    }
}

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
pub enum OfftopicActivity {
    /// exclude off-topic reviews (aka "Review Bombs")
    #[default]
    Exclude,
    /// or include them
    Include,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Reviews {
    pub(crate) success: u8,
    #[serde(skip_deserializing)]
    pub app_id: u64,
    pub reviews: Vec<Review>,
    pub query_summary: QuerySummary,
    /// The value to pass into the next request as the cursor to retrieve the next batch of reviews
    pub cursor: String,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct QuerySummary {
    /// The number of reviews returned in this response
    pub num_reviews: u32,
    /// The review score
    pub review_score: Option<u32>,
    /// The description of the review score
    pub review_score_desc: Option<String>,
    /// Total number of positive reviews
    pub total_positive: Option<u32>,
    /// Total number of negative reviews
    pub total_negative: Option<u32>,
    /// Total number of reviews matching the query parameters
    pub total_reviews: Option<u32>,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Author {
    #[serde(rename(deserialize = "steamid"))]
    pub user_id: String,
    pub num_games_owned: u64,
    pub num_reviews: u64,
    pub playtime_forever: u64,
    pub playtime_last_two_weeks: u64,
    pub playtime_at_review: u64,
    pub last_played: u64,
}

#[derive(Deserialize, Serialize, Hash, Debug)]
pub struct Review {
    #[serde(rename(deserialize = "recommendationid"))]
    pub review_id: String,
    pub author: Author,
    pub language: String,
    pub review: String,
    pub timestamp_created: u64,
    pub timestamp_updated: u64,
    pub received_for_free: bool,
    pub steam_purchase: bool,
    pub voted_up: bool,
    pub votes_up: u64,
    pub votes_funny: u64,
    pub weighted_vote_score: String,
    pub written_during_early_access: bool,
    pub comment_count: Option<usize>,
}
