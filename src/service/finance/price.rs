use finance_query::Ticker;

pub async fn get_stock_price(symbol: &str) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    use tokio::time::{timeout, Duration};
    
    // Add timeout to prevent Discord interaction timeout
    let result = timeout(Duration::from_secs(25), async {
        let ticker = Ticker::builder(symbol).build().await?;
        let quote = ticker.quote().await?;

        let price = quote
            .regular_market_price
            .as_ref()
            .and_then(|v| v.raw)
            .ok_or("Price not available")?;

        Ok(price)
    }).await;

    match result {
        Ok(data) => data,
        Err(_) => Err("Request timed out after 25 seconds".into()),
    }
}
