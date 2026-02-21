use crate::{Context, Error};
use crate::service::finance::{self, Period};

/// Get cash flow statement data for a stock
#[poise::command(slash_command)]
pub async fn cashflow(
    ctx: Context<'_>,
    #[description = "Stock ticker symbol (e.g., AAPL)"] symbol: String,
    #[description = "Cash flow field to display"]
    #[rename = "field"]
    field: CashFlowField,
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

    tracing::info!("Fetching cash flow for {} - field: {}", symbol_upper, field_str);

    match finance::get_cash_flow(&symbol_upper, field_str, period_type).await {
        Ok(data) => {
            if data.dates.is_empty() {
                let msg = format!("No cash flow data available for {}", symbol_upper);
                tracing::warn!("{}", msg);
                ctx.say(msg).await?;
                return Ok(());
            }

            let field_name = finance::format_field_name(field_str);
            let period_label = match period.unwrap_or(PeriodChoice::Yearly) {
                PeriodChoice::Yearly => "Annual",
                PeriodChoice::Quarterly => "Quarterly",
            };

            let mut response = format!("**{} - {} Cash Flow ({})**\n\n", symbol_upper, field_name, period_label);

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

            tracing::info!("Successfully fetched cash flow for {}", symbol_upper);
            ctx.say(response).await?;
        }
        Err(e) => {
            let error_msg = format!("Could not fetch cash flow for {}: {}", symbol_upper, e);
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
pub enum CashFlowField {
    #[name = "Operating Cash Flow"]
    OperatingCashFlow,
    #[name = "Free Cash Flow"]
    FreeCashFlow,
    #[name = "Capital Expenditure"]
    CapitalExpenditure,
    #[name = "Net Income From Continuing Operations"]
    NetIncomeFromContinuingOperations,
    #[name = "Depreciation And Amortization"]
    DepreciationAndAmortization,
    #[name = "Change In Working Capital"]
    ChangeInWorkingCapital,
    #[name = "Stock Based Compensation"]
    StockBasedCompensation,
    #[name = "Investing Cash Flow"]
    InvestingCashFlow,
    #[name = "Purchase Of PPE"]
    PurchaseOfPPE,
    #[name = "Purchase Of Investment"]
    PurchaseOfInvestment,
    #[name = "Sale Of Investment"]
    SaleOfInvestment,
    #[name = "Financing Cash Flow"]
    FinancingCashFlow,
    #[name = "Net Long Term Debt Issuance"]
    NetLongTermDebtIssuance,
    #[name = "Long Term Debt Payments"]
    LongTermDebtPayments,
    #[name = "Net Common Stock Issuance"]
    NetCommonStockIssuance,
    #[name = "Repurchase Of Capital Stock"]
    RepurchaseOfCapitalStock,
    #[name = "Cash Dividends Paid"]
    CashDividendsPaid,
    #[name = "Common Stock Dividend Paid"]
    CommonStockDividendPaid,
    #[name = "End Cash Position"]
    EndCashPosition,
    #[name = "Beginning Cash Position"]
    BeginningCashPosition,
    #[name = "Changes In Cash"]
    ChangesinCash,
    #[name = "Change In Receivables"]
    ChangeInReceivables,
    #[name = "Change In Inventory"]
    ChangeInInventory,
    #[name = "Change In Account Payable"]
    ChangeInAccountPayable,
    #[name = "Deferred Income Tax"]
    DeferredIncomeTax,
}

impl CashFlowField {
    fn as_str(&self) -> &'static str {
        match self {
            Self::OperatingCashFlow => "OperatingCashFlow",
            Self::FreeCashFlow => "FreeCashFlow",
            Self::CapitalExpenditure => "CapitalExpenditure",
            Self::NetIncomeFromContinuingOperations => "NetIncomeFromContinuingOperations",
            Self::DepreciationAndAmortization => "DepreciationAndAmortization",
            Self::ChangeInWorkingCapital => "ChangeInWorkingCapital",
            Self::StockBasedCompensation => "StockBasedCompensation",
            Self::InvestingCashFlow => "InvestingCashFlow",
            Self::PurchaseOfPPE => "PurchaseOfPPE",
            Self::PurchaseOfInvestment => "PurchaseOfInvestment",
            Self::SaleOfInvestment => "SaleOfInvestment",
            Self::FinancingCashFlow => "FinancingCashFlow",
            Self::NetLongTermDebtIssuance => "NetLongTermDebtIssuance",
            Self::LongTermDebtPayments => "LongTermDebtPayments",
            Self::NetCommonStockIssuance => "NetCommonStockIssuance",
            Self::RepurchaseOfCapitalStock => "RepurchaseOfCapitalStock",
            Self::CashDividendsPaid => "CashDividendsPaid",
            Self::CommonStockDividendPaid => "CommonStockDividendPaid",
            Self::EndCashPosition => "EndCashPosition",
            Self::BeginningCashPosition => "BeginningCashPosition",
            Self::ChangesinCash => "ChangesinCash",
            Self::ChangeInReceivables => "ChangeInReceivables",
            Self::ChangeInInventory => "ChangeInInventory",
            Self::ChangeInAccountPayable => "ChangeInAccountPayable",
            Self::DeferredIncomeTax => "DeferredIncomeTax",
        }
    }
}
