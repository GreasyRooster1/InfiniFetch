use reqwest::Error;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct RawDistro {
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

pub struct RenderData{
    //todo: cpu, gpu, ram, other specs
    pub(crate) icon_text:String,
    pub(crate) distro_name:String,
}

pub async fn get_distro(distro_name:&str)->Result<(),Error>{
    let request_url = format!("https://distro-api.up.railway.app/distribution?identifier={0}",distro_name);
    println!("requesting from {}", request_url);

    let response = reqwest::get(&request_url).await?;

    let raw_distro: RawDistro = response.json().await?;

    println!("{:?}", raw_distro);

    Ok(())
}