use crate::{Context, Error};
use crate::service::finance::{self, Period};

/// Get income statement data for a stock
#[poise::command(slash_command)]
pub async fn incomestatement(
    ctx: Context<'_>,
    #[description = "Stock ticker symbol (e.g., AAPL)"] symbol: String,
    #[description = "Income statement field to display"]
    #[rename = "field"]
    field: IncomeStatementField,
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

    tracing::info!("Fetching income statement for {} - field: {}", symbol_upper, field_str);

    match finance::get_income_statement(&symbol_upper, field_str, period_type).await {
        Ok(data) => {
            if data.dates.is_empty() {
                let msg = format!("No income statement data available for {}", symbol_upper);
                tracing::warn!("{}", msg);
                ctx.say(msg).await?;
                return Ok(());
            }

            let field_name = finance::format_income_statement_field_name(field_str);
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

            tracing::info!("Successfully fetched income statement for {}", symbol_upper);
            ctx.say(response).await?;
        }
        Err(e) => {
            let error_msg = format!("Could not fetch income statement for {}: {}", symbol_upper, e);
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
pub enum IncomeStatementField {
    // Revenue & Profit
    #[name = "Total Revenue"]
    TotalRevenue,
    #[name = "Operating Revenue"]
    OperatingRevenue,
    #[name = "Cost Of Revenue"]
    CostOfRevenue,
    #[name = "Gross Profit"]
    GrossProfit,
    
    // Operating Metrics
    #[name = "Operating Expense"]
    OperatingExpense,
    #[name = "Selling General And Administration"]
    SellingGeneralAndAdministration,
    #[name = "Research And Development"]
    ResearchAndDevelopment,
    #[name = "Operating Income"]
    OperatingIncome,
    
    // Interest & Other
    #[name = "Interest Expense"]
    InterestExpense,
    #[name = "Interest Income"]
    InterestIncome,
    #[name = "Net Interest Income"]
    NetInterestIncome,
    #[name = "Other Income Expense"]
    OtherIncomeExpense,
    
    // Income & Tax
    #[name = "Pretax Income"]
    PretaxIncome,
    #[name = "Tax Provision"]
    TaxProvision,
    #[name = "Net Income"]
    NetIncome,
    #[name = "Net Income Common Stockholders"]
    NetIncomeCommonStockholders,
    
    // Per Share Metrics
    #[name = "Diluted EPS"]
    DilutedEPS,
    #[name = "Basic EPS"]
    BasicEPS,
    #[name = "Diluted Average Shares"]
    DilutedAverageShares,
    #[name = "Basic Average Shares"]
    BasicAverageShares,
    
    // Advanced Metrics
    #[name = "EBIT"]
    EBIT,
    #[name = "EBITDA"]
    EBITDA,
    #[name = "Normalized EBITDA"]
    NormalizedEBITDA,
    #[name = "Total Expenses"]
    TotalExpenses,
}

impl IncomeStatementField {
    fn as_str(&self) -> &'static str {
        match self {
            // Revenue & Profit
            Self::TotalRevenue => "TotalRevenue",
            Self::OperatingRevenue => "OperatingRevenue",
            Self::CostOfRevenue => "CostOfRevenue",
            Self::GrossProfit => "GrossProfit",
            
            // Operating Metrics
            Self::OperatingExpense => "OperatingExpense",
            Self::SellingGeneralAndAdministration => "SellingGeneralAndAdministration",
            Self::ResearchAndDevelopment => "ResearchAndDevelopment",
            Self::OperatingIncome => "OperatingIncome",
            
            // Interest & Other
            Self::InterestExpense => "InterestExpense",
            Self::InterestIncome => "InterestIncome",
            Self::NetInterestIncome => "NetInterestIncome",
            Self::OtherIncomeExpense => "OtherIncomeExpense",
            
            // Income & Tax
            Self::PretaxIncome => "PretaxIncome",
            Self::TaxProvision => "TaxProvision",
            Self::NetIncome => "NetIncome",
            Self::NetIncomeCommonStockholders => "NetIncomeCommonStockholders",
            
            // Per Share Metrics
            Self::DilutedEPS => "DilutedEPS",
            Self::BasicEPS => "BasicEPS",
            Self::DilutedAverageShares => "DilutedAverageShares",
            Self::BasicAverageShares => "BasicAverageShares",
            
            // Advanced Metrics
            Self::EBIT => "EBIT",
            Self::EBITDA => "EBITDA",
            Self::NormalizedEBITDA => "NormalizedEBITDA",
            Self::TotalExpenses => "TotalExpenses",
        }
    }
}
