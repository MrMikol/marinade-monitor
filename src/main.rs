use dotenvy::from_path;
use serde::Deserialize;

const TARGET_VOTE_ACCOUNT: &str =
    "FdGcvmbpebUwYA3vSywnagsaC3Tq3pAVmcR6VoxVcdV9";

#[derive(Debug, Deserialize)]
struct ValidatorsResponse {
    validators: Vec<Validator>,
}

#[derive(Debug, Deserialize)]
struct Validator {
    vote_account: String,
    info_name: Option<String>,
    commission_advertised: Option<u64>,
    institutional_stake: String,
}

#[derive(Debug, Deserialize)]
struct BondsResponse {
    bonds: Vec<Bond>,
}

#[derive(Debug, Deserialize)]
struct Bond {
    vote_account: String,
    effective_amount: f64,
}

fn lamports_to_sol(lamports: f64) -> f64 {
    lamports / 1_000_000_000.0
}

async fn send_to_slack(
    webhook_url: &str,
    message: &str,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    client
        .post(webhook_url)
        .json(&serde_json::json!({
            "text": message
        }))
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Load environment variables from server path
    from_path("/etc/opt/app/slack/.env").ok();

    let validator_url =
        "https://validators-api.marinade.finance/validators?limit=9999&epochs=0";

    let bond_url =
        "https://validator-bonds-api.marinade.finance/bonds/institutional";

    // Fetch both APIs concurrently
    let (validator_res, bond_res) = tokio::join!(
        reqwest::get(validator_url),
        reqwest::get(bond_url)
    );

    let validators: ValidatorsResponse =
        validator_res?.json().await?;

    let bonds: BondsResponse =
        bond_res?.json().await?;

    let validator = validators
        .validators
        .iter()
        .find(|v| v.vote_account == TARGET_VOTE_ACCOUNT);

    let bond = bonds
        .bonds
        .iter()
        .find(|b| b.vote_account == TARGET_VOTE_ACCOUNT);

    match (validator, bond) {
        (Some(v), Some(b)) => {
            let select_tvl_sol =
                v.institutional_stake
                    .parse::<f64>()
                    .unwrap_or(0.0)
                    / 1_000_000_000.0;

            let bond_sol =
                lamports_to_sol(b.effective_amount);

            // Select TVL / 1000
            let select_tvl_bond_required =
                select_tvl_sol / 1000.0;

            // Minimum required bond = above / 2
            let minimum_required_bond =
                select_tvl_bond_required / 2.0;

            let status = if bond_sol >= minimum_required_bond {
                "🟢"
            } else {
                "🔴"
            };

            let comparison_symbol =
                if bond_sol >= minimum_required_bond {
                    ">"
                } else {
                    "<"
                };

            println!("==============================");

            println!(
                "Validator: {}",
                v.info_name.as_deref().unwrap_or("Unknown")
            );

            println!("Vote Account: {}", v.vote_account);

            println!("Select TVL: {:.2} SOL", select_tvl_sol);

            println!(
                "Inflation Commission: {}%",
                v.commission_advertised.unwrap_or(0)
            );

            println!("Bond: {:.3} SOL", bond_sol);

            println!(
                "Select TVL Bond Required: {:.3}",
                select_tvl_bond_required
            );

            println!(
                "Bond amount {:.3} {} Select TVL Required ({:.3}/2) = {:.3}",
                bond_sol,
                comparison_symbol,
                select_tvl_bond_required,
                minimum_required_bond
            );

            println!("Status: {}", status);

            println!("==============================");

            // Slack message
            let slack_message = format!(
                "*Marinade Validator Bond Check*\n\
                Name: {}\n\
                Validator: `{}`\n\
                Select TVL: {:.2} SOL\n\
                Bond: {:.3} SOL\n\
                Select TVL Bond Required: {:.3}\n\
                Bond amount {:.3} {} Select TVL Required ({:.3}/2) = {:.3}\n\
                Status: {}",
                v.info_name.as_deref().unwrap_or("Unknown"),
                v.vote_account,
                select_tvl_sol,
                // v.commission_advertised.unwrap_or(0),
                bond_sol,
                select_tvl_bond_required,
                bond_sol,
                comparison_symbol,
                select_tvl_bond_required,
                minimum_required_bond,
                status
            );

            let webhook_url = std::env::var("SLACK_WEBHOOK_URL")
                .expect("SLACK_WEBHOOK_URL is not set");

            send_to_slack(&webhook_url, &slack_message).await?;
        }

        _ => {
            println!("Validator or bond not found");
        }
    }

    Ok(())
}