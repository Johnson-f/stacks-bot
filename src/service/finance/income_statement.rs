use finance_query::{Ticker, StatementType, Frequency};
use super::Period;

pub struct IncomeStatementData {
    pub dates: Vec<String>,
    pub values: Vec<f64>,
}

pub async fn get_income_statement(
    symbol: &str,
    field: &str,
    period: Period,
) -> Result<IncomeStatementData, Box<dyn std::error::Error + Send + Sync>> {
    use tokio::time::{timeout, Duration};
    
    // Add timeout to prevent Discord interaction timeout
    let result = timeout(Duration::from_secs(25), async {
        let ticker = Ticker::new(symbol).await?;
        
        let frequency = match period {
            Period::Yearly => Frequency::Annual,
            Period::Quarterly => Frequency::Quarterly,
        };

        let income_statement = ticker.financials(StatementType::Income, frequency).await?;

        let field_data = income_statement.statement.get(field)
            .ok_or_else(|| format!("Field '{}' not found in income statement", field))?;

        let mut dates: Vec<String> = field_data.keys().cloned().collect();
        dates.sort_by(|a, b| b.cmp(a)); // Sort descending (newest first)

        let values: Vec<f64> = dates.iter()
            .filter_map(|date| field_data.get(date).copied())
            .collect();

        Ok(IncomeStatementData { dates, values })
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
