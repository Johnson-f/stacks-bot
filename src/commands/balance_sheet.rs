use crate::{Context, Error};
use crate::service::finance::{self, Period};

/// Get balance sheet data for a stock
#[poise::command(slash_command)]
pub async fn balancesheet(
    ctx: Context<'_>,
    #[description = "Stock ticker symbol (e.g., AAPL)"] symbol: String,
    #[description = "Balance sheet field to display"]
    #[rename = "field"]
    field: BalanceSheetField,
    #[description = "Period type (default: Yearly)"]
    #[rename = "period"]
    period: Option<PeriodChoice>,
) -> Result<(), Error> {
    // Defer immediately to prevent timeout
    if let Err(e) = ctx.defer().await {
        tracing::error!("Failed to defer: {}", e);
        return Err(e.into());
    }

    let period_type = match period.unwrap_or(PeriodChoice::Yearly) {
        PeriodChoice::Yearly => Period::Yearly,
        PeriodChoice::Quarterly => Period::Quarterly,
    };

    let field_str = field.as_str();
    let symbol_upper = symbol.to_uppercase();

    tracing::info!("Fetching balance sheet for {} - field: {}", symbol_upper, field_str);

    match finance::get_balance_sheet(&symbol_upper, field_str, period_type).await {
        Ok(data) => {
            if data.dates.is_empty() {
                let msg = format!("No balance sheet data available for {}", symbol_upper);
                tracing::warn!("{}", msg);
                ctx.say(msg).await?;
                return Ok(());
            }

            let field_name = finance::format_balance_sheet_field_name(field_str);
            let period_label = match period.unwrap_or(PeriodChoice::Yearly) {
                PeriodChoice::Yearly => "Annual",
                PeriodChoice::Quarterly => "Quarterly",
            };

            let mut response = format!("**{} - {} ({})**\n\n", symbol_upper, field_name, period_label);

            for (date, value) in data.dates.iter().zip(data.values.iter()).take(5) {
                let formatted = if value.abs() >= 1_000_000_000.0 {
                    format!("${:.2}B", value / 1_000_000_000.0)
                } else if value.abs() >= 1_000_000.0 {
                    format!("${:.2}M", value / 1_000_000.0)
                } else {
                    format!("${:.2}", value)
                };
                response.push_str(&format!("**{}**: {}\n", date, formatted));
            }

            tracing::info!("Successfully fetched balance sheet for {}", symbol_upper);
            ctx.say(response).await?;
        }
        Err(e) => {
            let error_msg = format!("Could not fetch balance sheet for {}: {}", symbol_upper, e);
            tracing::error!("{}", error_msg);
            ctx.say(error_msg).await?;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum PeriodChoice {
    #[name = "Yearly"]
    Yearly,
    #[name = "Quarterly"]
    Quarterly,
}

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum BalanceSheetField {
    // Assets
    #[name = "Total Assets"]
    TotalAssets,
    #[name = "Current Assets"]
    CurrentAssets,
    #[name = "Cash And Cash Equivalents"]
    CashAndCashEquivalents,
    #[name = "Accounts Receivable"]
    AccountsReceivable,
    #[name = "Inventory"]
    Inventory,
    #[name = "Net PPE"]
    NetPPE,
    #[name = "Goodwill"]
    Goodwill,
    
    // Liabilities
    #[name = "Total Liabilities"]
    TotalLiabilitiesNetMinorityInterest,
    #[name = "Current Liabilities"]
    CurrentLiabilities,
    #[name = "Accounts Payable"]
    AccountsPayable,
    #[name = "Current Debt"]
    CurrentDebt,
    #[name = "Long Term Debt"]
    LongTermDebt,
    #[name = "Total Debt"]
    TotalDebt,
    
    // Equity
    #[name = "Stockholders Equity"]
    StockholdersEquity,
    #[name = "Common Stock Equity"]
    CommonStockEquity,
    #[name = "Retained Earnings"]
    RetainedEarnings,
    #[name = "Treasury Stock"]
    TreasuryStock,
    
    // Other Metrics
    #[name = "Working Capital"]
    WorkingCapital,
    #[name = "Tangible Book Value"]
    TangibleBookValue,
    #[name = "Net Debt"]
    NetDebt,
    #[name = "Ordinary Shares Number"]
    OrdinarySharesNumber,
}

impl BalanceSheetField {
    fn as_str(&self) -> &'static str {
        match self {
            // Assets
            Self::TotalAssets => "TotalAssets",
            Self::CurrentAssets => "CurrentAssets",
            Self::CashAndCashEquivalents => "CashAndCashEquivalents",
            Self::AccountsReceivable => "AccountsReceivable",
            Self::Inventory => "Inventory",
            Self::NetPPE => "NetPPE",
            Self::Goodwill => "Goodwill",
            
            // Liabilities
            Self::TotalLiabilitiesNetMinorityInterest => "TotalLiabilitiesNetMinorityInterest",
            Self::CurrentLiabilities => "CurrentLiabilities",
            Self::AccountsPayable => "AccountsPayable",
            Self::CurrentDebt => "CurrentDebt",
            Self::LongTermDebt => "LongTermDebt",
            Self::TotalDebt => "TotalDebt",
            
            // Equity
            Self::StockholdersEquity => "StockholdersEquity",
            Self::CommonStockEquity => "CommonStockEquity",
            Self::RetainedEarnings => "RetainedEarnings",
            Self::TreasuryStock => "TreasuryStock",
            
            // Other Metrics
            Self::WorkingCapital => "WorkingCapital",
            Self::TangibleBookValue => "TangibleBookValue",
            Self::NetDebt => "NetDebt",
            Self::OrdinarySharesNumber => "OrdinarySharesNumber",
        }
    }
}
