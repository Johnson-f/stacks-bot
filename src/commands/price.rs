use crate::{Context, Error};
use crate::service::finance;

/// Get the current price of a stock
#[poise::command(slash_command)]
pub async fn price(
    ctx: Context<'_>,
    #[description = "Stock ticker symbol (e.g., AAPL)"] symbol: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    match finance::get_stock_price(&symbol.to_uppercase()).await {
        Ok(price) => {
            ctx.say(format!("**{}**: ${:.2}", symbol.to_uppercase(), price)).await?;
        }
        Err(e) => {
            ctx.say(format!("Could not fetch price for {}: {}", symbol, e)).await?;
        }
    }

    Ok(())
}
