use finance_query::finance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all = finance::earnings_transcripts("AAPL", None).await?;
    for t in &all {
        println!("{}: {} {}", t.title, t.transcript.quarter(), t.transcript.year());
    }
    Ok(())
}