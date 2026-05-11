use serde::Deserialize;

const TARGET_VOTE_ACCOUNT: &str = "FdGcvmbpebUwYA3vSywnagsaC3Tq3pAVmcR6VoxVcdV9";

#[derive(Debug, Deserialize)]
struct BondResponse {
    bonds: Vec<ValidatorBond>,
}

#[derive(Debug, Deserialize)]
struct ValidatorBond {
    authority: String,
    vote_account: String,
    pubkey: String,
    bond_type: String,
    funded_amount: f64,
    effective_amount: f64,
    max_stake_wanted: f64,
    updated_at: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://validator-bonds-api.marinade.finance/bonds/bidding";
    
    let response: BondResponse = reqwest::get(url).await?.json().await?;
    
    let validator = response.bonds.iter().find(|bond| bond.vote_account == TARGET_VOTE_ACCOUNT);
    
    match validator {
        Some(bond) => {
            println!("Found validator:");
            println!("Vote account: {}", bond.vote_account);
            println!("Authority: {}", bond.authority);
            println!("Bond pubkey: {}", bond.pubkey);
            println!("Bond type: {}", bond.bond_type);
            println!("Funded amount: {}", bond.funded_amount);
            println!("Effective amount: {}", bond.effective_amount);
            println!("Max stake wanted: {}", bond.max_stake_wanted);
            println!("Updated at: {}", bond.updated_at);
        }
        None => {
            println!("No validator found");
        }
    }
    
    Ok(())
}
