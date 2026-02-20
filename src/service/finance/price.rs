use finance_query::Ticker;

pub async fn get_stock_price(symbol: &str) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    let ticker = Ticker::builder(symbol).build().await?;
    let quote = ticker.quote().await?;

    let price = quote
        .regular_market_price
        .as_ref()
        .and_then(|v| v.raw)
        .ok_or("Price not available")?;

    Ok(price)
}
