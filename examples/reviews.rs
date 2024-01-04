extern crate steam_store_api;

use anyhow::Result;
use steam_store_api::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SteamBuilder::new()
        .with_country_code("US")
        .with_language(&Language::English)
        .build()?;

    let mut filter = ReviewsFilter {
        language: Some(Language::English),
        day_range: Some(365),
        num_per_page: Some(100),
        offtopic_activity: OfftopicActivity::Include,
        ..Default::default()
    };

    let mut reviews_data = client.reviews(&489830_u64, &filter).await?;
    loop {
        for review in reviews_data.reviews {
            if review.votes_funny > 100 {
                println!(
                    "Votes funny: {:#?}\nReview: {:#?}\n",
                    &review.votes_funny, &review.review
                );
            }
        }
        if reviews_data.query_summary.num_reviews < 100 {
            break;
        }

        // send cursor inside filter for the next page
        filter.cursor = Some(reviews_data.cursor);
        reviews_data = client.reviews(&489830_u64, &filter).await?;
    }

    Ok(())
}
