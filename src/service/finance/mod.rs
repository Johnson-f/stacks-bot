pub mod price;
pub mod cash_flow;
pub mod balance_sheet;
pub mod income_statement;

// Shared Period enum for all financial statements
pub enum Period {
    Yearly,
    Quarterly,
}

pub use price::get_stock_price;
pub use cash_flow::{get_cash_flow, format_field_name};
pub use balance_sheet::{get_balance_sheet, format_field_name as format_balance_sheet_field_name};
pub use income_statement::{get_income_statement, format_field_name as format_income_statement_field_name};
