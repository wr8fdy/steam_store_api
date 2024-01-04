extern crate steam_store_api;

use anyhow::Result;
use steam_store_api::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SteamBuilder::new()
        .with_country_code("US")
        .with_language(&Language::English)
        .build()?;

    let app = client.app(&219990_u64).await?;
    println!("App details:");
    println!("{:#?} - {:#?}", &app.app_id, &app.name);

    let dlc_list = client.dlc(&219990_u64).await?;

    if let Some(dlcs) = dlc_list.dlc {
        println!("\nDLC list:");
        for dlc in dlcs {
            println!("{:#?} - {:#?}", &dlc.dlc_id, &dlc.name);
        }
    }

    if let Some(pkgs) = app.packages {
        println!("\nPackage list:");
        for pkg_id in pkgs {
            let pkg = client.package(&pkg_id).await?;
            println!(
                "{:#?} - {:#?} - discount {}%",
                &pkg.pkg_id, &pkg.name, pkg.price.price.discount_percent
            );
        }
    }

    let prices = client.price(vec![app.app_id]).await?;
    println!("\nPrice owerview:");
    for price in prices {
        println!(
            "{:#?} with discount {:#?}% = {:#?}",
            &price.initial_formatted, &price.price.discount_percent, &price.final_formatted
        );
    }

    Ok(())
}
