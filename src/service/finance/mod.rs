pub mod balance_sheet;
pub mod cash_flow;
pub mod income_statement;
pub mod price;

// Shared Period enum for all financial statements
pub enum Period {
    Yearly,
    Quarterly,
}

pub use balance_sheet::{format_field_name as format_balance_sheet_field_name, get_balance_sheet};
pub use cash_flow::{format_field_name, get_cash_flow};
pub use income_statement::{
    format_field_name as format_income_statement_field_name, get_income_statement,
};
pub use price::get_stock_price;
