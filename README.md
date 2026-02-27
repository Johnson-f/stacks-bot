# Stacks Bot

A powerful Discord bot written in Rust for retrieving real-time stock market data and detailed financial statements. Stacks Bot helps investors and traders access fundamental data directly within Discord using simple slash commands.

## Features

- **Real-time Stock Prices**: Fetch current market prices for any ticker.
- **Financial Statements**: Access historical data for:
  - **Balance Sheets**: Assets, liabilities, equity, and more.
  - **Cash Flow**: Operating, investing, and financing cash flows.
  - **Income Statements**: Revenue, profit, EPS, and other key metrics.
- **Flexible Reporting**: View data on a **Yearly** or **Quarterly** basis.
- **Formatted Output**: Large numbers are automatically formatted (e.g., $1.25B, $500M) for easy reading.

## Commands

All interactions are performed via Discord Slash Commands.

### `/price`
Get the current trading price of a stock.
- **Usage**: `/price symbol: AAPL`

### `/balancesheet`
Retrieve balance sheet data for a specific field.
- **Usage**: `/balancesheet symbol: TSLA field: "Total Assets" period: Quarterly`
- **Supported Fields**: Total Assets, Current Assets, Total Liabilities, Long Term Debt, Stockholders Equity, Working Capital, and more.

### `/cashflow`
Retrieve cash flow statement data.
- **Usage**: `/cashflow symbol: MSFT field: "Free Cash Flow" period: Yearly`
- **Supported Fields**: Operating Cash Flow, Free Cash Flow, Capital Expenditure, Dividends Paid, Stock Based Compensation, and more.

### `/incomestatement`
Retrieve income statement data.
- **Usage**: `/incomestatement symbol: NVDA field: "Total Revenue" period: Yearly`
- **Supported Fields**: Revenue, Gross Profit, Operating Income, Net Income, EPS (Basic & Diluted), EBITDA, and more.

## Setup & Configuration

1.  **Prerequisites**:
    - Rust (latest stable)
    - A Discord Bot Token

2.  **Environment Variables**:
    Create a `.env` file in the root directory (see `.env.example`):
    ```env
    DISCORD_TOKEN=your_discord_bot_token_here
    GUILD_IDS=123456789,987654321 # Optional: Comma-separated guild IDs for instant command registration during development
    ```

3.  **Running the Bot**:
    ```bash
    cargo run
    ```

## Tech Stack

- **Language**: Rust
- **Framework**: [poise](https://github.com/serenity-rs/poise) (built on [serenity](https://github.com/serenity-rs/serenity))
- **Data Source**: `finance-query` (Internal crate wrapping financial data providers)

## License

This project is licensed under the terms of the MIT license.
