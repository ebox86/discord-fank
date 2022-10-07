

use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Stock {
    symbol: String,
    lastSalePrice: f64
}

pub async fn get_quote(iex_api_key: String, symbol: String) -> String {
    let client = Client::new();
    let url = format!("https://cloud.iexapis.com/stable/tops?token={}&symbols={}", iex_api_key, symbol);
    let response = client.get(url).send().await.unwrap().json::<Vec<Stock>>().await.unwrap();
    return if response.get(0).is_some() { response.get(0).unwrap().lastSalePrice.to_string() } else { "N/A".to_string() };
}