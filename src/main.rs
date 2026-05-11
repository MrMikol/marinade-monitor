use serde::Deserialize;

const TARGET_VOTE_ACCOUNT: &str = "FdGcvmbpebUwYA3vSywnagsaC3Tq3pAVmcR6VoxVcdV9";

#[derive(Debug, Deserialize)]
struct BondResponse {
    bonds: Vec<ValidatorBond>,
}

#[derive(Debug, serde::Deserialize)]
struct ValidatorsResponse {
    validators: Vec<Validator>,
}

#[derive(Debug, serde::Deserialize)]
struct Validator {
    vote_account: String,
    info_name: Option<String>,
    commission_advertised: Option<u64>,
    institutional_stake: String,
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
    let url_1 = "https://validator-bonds-api.marinade.finance/bonds/bidding";
    let url_2 = "https://validators-api.marinade.finance/validators?limit=9999&epochs=0";
    let response: ValidatorsResponse = reqwest::get(url_2).await?.json().await?;
    
    // let validator = response.bonds.iter().find(|bond| bond.vote_account == TARGET_VOTE_ACCOUNT);
    let validator = response.validators.iter().find(|v| v.vote_account == TARGET_VOTE_ACCOUNT);
    
    match validator {
        Some(v) => {
            let select_tvl_sol: f64 = v.institutional_stake.parse::<f64>().unwrap_or(0.0) / 1_000_000_000.0;

            println!("Name: {}", v.info_name.as_deref().unwrap_or("Unknown"));
            println!("Vote account: {}", v.vote_account);
            println!("Select TVL: {:.2} SOL", select_tvl_sol);
            println!("Inflation comission: {}%", v.commission_advertised.unwrap_or(0));
        }
         None => println!("Validator not found"),       
    }
    
    Ok(())
}

fn lamports_to_sol(lamports: f64) -> f64 {
    lamports / 1_000_000_000.0
}