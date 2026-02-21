use finance_query::{Ticker, StatementType, Frequency};
use super::Period;

pub struct BalanceSheetData {
    pub dates: Vec<String>,
    pub values: Vec<f64>,
}

pub async fn get_balance_sheet(
    symbol: &str,
    field: &str,
    period: Period,
) -> Result<BalanceSheetData, Box<dyn std::error::Error + Send + Sync>> {
    use tokio::time::{timeout, Duration};
    
    // Add timeout to prevent Discord interaction timeout
    let result = timeout(Duration::from_secs(25), async {
        let ticker = Ticker::new(symbol).await?;
        
        let frequency = match period {
            Period::Yearly => Frequency::Annual,
            Period::Quarterly => Frequency::Quarterly,
        };

        let balance_sheet = ticker.financials(StatementType::Balance, frequency).await?;

        let field_data = balance_sheet.statement.get(field)
            .ok_or_else(|| format!("Field '{}' not found in balance sheet", field))?;

        let mut dates: Vec<String> = field_data.keys().cloned().collect();
        dates.sort_by(|a, b| b.cmp(a)); // Sort descending (newest first)

        let values: Vec<f64> = dates.iter()
            .filter_map(|date| field_data.get(date).copied())
            .collect();

        Ok(BalanceSheetData { dates, values })
    }).await;

    match result {
        Ok(data) => data,
        Err(_) => Err("Request timed out after 25 seconds".into()),
    }
}

pub fn format_field_name(field: &str) -> String {
    field
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && c.is_uppercase() {
                vec![' ', c]
            } else {
                vec![c]
            }
        })
        .collect()
}
