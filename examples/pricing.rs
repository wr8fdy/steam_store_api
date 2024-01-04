extern crate steam_store_api;

use anyhow::Result;
use steam_store_api::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SteamBuilder::new()
        .with_country_code("US")
        .with_language(&Language::English)
        .build()?;

    let featured = client.featured().await?;
    println!("Featured:");
    for item in featured.featured_linux {
        if item.discounted {
            println!(
                "{:#?} - {:#?} - {:#?}%",
                item.app_id, item.name, item.discount_percent
            );
        }
    }

    let featured_categories = client.featured_categories().await?;

    if let Some(specials) = featured_categories.get("specials") {
        if let Some(items) = &specials.items {
            println!("\nFeatured specials:");
            for item in items {
                if item.discounted {
                    println!(
                        "{:#?} - {:#?} - {:#?}%",
                        item.app_id, item.name, item.discount_percent
                    );
                }
            }
        }
    }

    let prices = client
        .price(vec![483840, 565610, 642280, 897670, 1088290, 1250890])
        .await?;

    println!("\nPrice by app id:");
    for app in prices {
        println!(
            "{:#?} - {:#?}% - {:#?}",
            app.app_id, app.price.discount_percent, app.final_formatted
        );
    }

    Ok(())
}
