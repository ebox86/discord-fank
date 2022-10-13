use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Stock {
    symbol: String,
    latestPrice: f64,
    previousClose: f64,
}

pub async fn get_quote(iex_api_key: String, symbol: String) -> (f64, f64) {
    let client = Client::new();
    let url = format!("https://cloud.iexapis.com/stable/stock/{}/quote?token={}", symbol, iex_api_key);

    let request = client.get(url).build().unwrap();

    let response = client.execute(request).await.unwrap();
    if response.status().is_success() {
        let json = response.json::<Stock>().await.unwrap();
        return (json.latestPrice, json.previousClose);
    } else { 
        return (0.0, 0.0)
    }
}

pub fn price_diff_formatter(price: f64, previous_price: f64) -> String {
    let diff = price - previous_price;
    let diff_percent = diff / previous_price * 100.0;
    let diff_percent = f64::trunc(diff_percent  * 100.0) / 100.0;
    let diff = f64::trunc(diff  * 100.0) / 100.0;
    let diff = if diff > 0.0 { format!("+{}", diff) } else { format!("{}", diff) };
    let diff_percent = if diff_percent > 0.0 { format!("(+{}%)\t🟢", diff_percent) } else { format!("({}%)\t🔴", diff_percent) };
    return format!("{} {}", diff, diff_percent);
}

pub fn price_diff (price: f64, previous_price: f64) -> (f64, f64) {
    let diff = price - previous_price;
    let diff_percent = diff / previous_price * 100.0;
    let diff_percent = f64::trunc(diff_percent  * 100.0) / 100.0;
    let diff = f64::trunc(diff  * 100.0) / 100.0;
    return (diff, diff_percent);
}