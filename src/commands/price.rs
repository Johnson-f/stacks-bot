use crate::service::finance;
use crate::{Context, Error};

/// Get the current price of a stock
#[poise::command(slash_command)]
pub async fn price(
    ctx: Context<'_>,
    #[description = "Stock ticker symbol (e.g., AAPL)"] symbol: String,
) -> Result<(), Error> {
    // Defer immediately to prevent timeout
    if let Err(e) = ctx.defer().await {
        tracing::error!("Failed to defer: {}", e);
        return Err(e.into());
    }

    let symbol_upper = symbol.to_uppercase();
    tracing::info!("Fetching price for {}", symbol_upper);

    match finance::get_stock_price(&symbol_upper).await {
        Ok(price) => {
            tracing::info!(
                "Successfully fetched price for {}: ${:.2}",
                symbol_upper,
                price
            );
            ctx.say(format!("**{}**: ${:.2}", symbol_upper, price))
                .await?;
        }
        Err(e) => {
            let error_msg = format!("Could not fetch price for {}: {}", symbol_upper, e);
            tracing::error!("{}", error_msg);
            ctx.say(error_msg).await?;
        }
    }

    Ok(())
}
