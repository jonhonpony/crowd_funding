use serde::{Deserialize, Serialize};
use std::error::Error;

const HORIZON_URL: &str = "https://horizon-testnet.stellar.org";

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResponse {
    pub sequence: String,
}

pub fn get_account_info(public_key: &str) -> Result<AccountResponse, Box<dyn Error>> {
    let account_url = format!("{}/accounts/{}", HORIZON_URL, public_key);
    let account_resp: AccountResponse = reqwest::blocking::get(&account_url)?.json()?;
    Ok(account_resp)
}

pub fn create_transaction(source_secret: &str, destination_public: &str, amount: &str) -> Result<String, Box<dyn Error>> {
    let account_info = get_account_info(&stellar_public_key_from_secret(source_secret)?)?;

    let tx_json = serde_json::json!({
        "source_account": stellar_public_key_from_secret(source_secret)?,
        "destination": destination_public,
        "amount": amount,
        "sequence": account_info.sequence,
    });

    let client = reqwest::blocking::Client::new();
    let submit_tx_url = format!("{}/transactions", HORIZON_URL);
    let response = client.post(&submit_tx_url)
        .json(&tx_json)
        .send()?;

    Ok(response.text()?)
}

fn stellar_public_key_from_secret(_secret: &str) -> Result<String, Box<dyn Error>> {
    // Dummy public key
    Ok("GDTCHOOGATSGQT7CJ72UIEJT7SNOUUES4XTUWCSDXHVQSKUSZ4XQFPVJ".to_string())
}

