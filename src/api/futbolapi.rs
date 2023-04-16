use serde_json::{Result, Value};
use reqwest;
pub async fn get_data_futbol_api(id: &str) -> Result<String> {

    const API_KEY: &str = "1dedb546ed1c4bd4ada5e66f19b814a8";
    const API_URL: &str = "http://api.football-data.org/v4";     
    let PATH = format!("/competitions/{}/matches", id) ;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("X-Auth-Token", API_KEY.parse().unwrap());

    let response = reqwest::Client::new()
        .get(&format!("{}{}", API_URL, PATH))
        .headers(headers)
        .send()
        .await.unwrap();

    match response.status() {
    reqwest::StatusCode::OK => {
        println!("Success!");
    },
    reqwest::StatusCode::UNAUTHORIZED => {
        println!("Need to grab a new token");
    },
    _ => {
        panic!("Uh oh! Something unexpected happened.");
    },
    };

    let body = response.text().await.unwrap();
    Ok(body)
    
}