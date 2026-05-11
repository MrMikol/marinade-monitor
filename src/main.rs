#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let url = "https://validator-bonds-api.marinade.finance/docs.json";
    let url = "https://validator-bonds-api.marinade.finance/bonds/bidding";
    // let body = reqwest::get(url).await?.text().await?;
    
    let json: serde_json::Value = reqwest::get(url).await?.json().await?;
    
    println!("{:#}", json);
    
    Ok(())
}
