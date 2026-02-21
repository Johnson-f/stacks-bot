use finance_query::{Ticker, StatementType, Frequency};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create ticker
    let ticker = Ticker::new("AAPL").await?;

    // Income Statement (Annual)
    let income = ticker.financials(StatementType::Income, Frequency::Annual).await?;

    if let Some(revenue_map) = income.statement.get("TotalRevenue") {
        for (date, value) in revenue_map {
            println!("{}: Revenue ${:.2}B", date, value / 1e9);
        }
    }

    if let Some(net_income_map) = income.statement.get("NetIncome") {
        for (date, value) in net_income_map {
            println!("{}: Net Income ${:.2}B", date, value / 1e9);
        }
    }

    // Balance Sheet (Quarterly)
    let balance = ticker.financials(StatementType::Balance, Frequency::Quarterly).await?;

    if let Some(assets) = balance.statement.get("TotalAssets") {
        for (date, value) in assets {
            println!("{}: Assets ${:.2}B", date, value / 1e9);
        }
    }

    // Cash Flow Statement (Annual)
    let cashflow = ticker.financials(StatementType::CashFlow, Frequency::Annual).await?;

    if let Some(operating_cf) = cashflow.statement.get("OperatingCashFlow") {
        for (date, value) in operating_cf {
            println!("{}: Operating CF ${:.2}B", date, value / 1e9);
        }
    }

    Ok(())
}