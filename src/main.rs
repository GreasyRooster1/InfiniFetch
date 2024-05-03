use serde::Deserialize;
use reqwest::Error;
use tokio::*;
use std::env::consts;

#[derive(Deserialize, Debug)]
struct RawDistro {
    id: String,
    identifier: String,
    name: String,
    description: String,
    distrowatch_page: String,
    image_path: String,
    homepage: String,
    light_badge_path: String,
    dark_badge_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://distro-api.up.railway.app/distribution?identifier=mx_linux");
    println!("{}", request_url);

    let response = reqwest::get(&request_url).await?;

    let distros: RawDistro = response.json().await?;

    println!("{:?}", distros);
    println!("{}", std::env::consts::OS);
    Ok(())
}