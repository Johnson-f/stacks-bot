use chrono::NaiveDate;
use finance_query::edgar;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    edgar::init("johnsonnifemi53@gmail.com")?;

    // Resolve ticker to CIK
    let cik = edgar::resolve_cik("AAPL").await?;

    // Get filing history
    let submissions = edgar::submissions(cik).await?;
    if let Some(name) = &submissions.name {
        println!("Company: {}", name);
    }

    // Get XBRL financial data
    let facts = edgar::company_facts(cik).await?;
    if let Some(us_gaap) = facts.facts.get("us-gaap") {
        if let Some(revenue) = us_gaap.0.get("Revenues") {
            if let Some(usd) = revenue.units.get("USD") {
                let mut seen = HashSet::new();

                let mut annual: Vec<_> = usd
                    .iter()
                    .filter_map(|point| {
                        let val = point.val?;
                        let start = NaiveDate::parse_from_str(point.start.as_deref()?, "%Y-%m-%d").ok()?;
                        let end = NaiveDate::parse_from_str(point.end.as_deref()?, "%Y-%m-%d").ok()?;
                        let days = (end - start).num_days();

                        // Only full-year periods and 10-K filings
                        if days >= 340
                            && days <= 390
                            && point.form.as_deref() == Some("10-K")
                            && seen.insert(end)
                        {
                            Some((end, val))
                        } else {
                            None
                        }
                    })
                    .collect();

                // Sort by date ascending
                annual.sort_by_key(|(end, _)| *end);

                for (end, val) in annual {
                    println!("FY ending {}: ${}", end, val);
                }
            }
        }
    }

    Ok(())
}