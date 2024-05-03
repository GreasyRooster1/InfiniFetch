use reqwest::Error;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct RawDistro {
    pub id: String,
    pub identifier: String,
    pub name: String,
    pub description: String,
    pub distrowatch_page: String,
    pub image_path: String,
    pub homepage: String,
    pub light_badge_path: String,
    pub dark_badge_path: String,
}

pub struct RenderData{
    //todo: gpu, ram, other specs
    pub(crate) icon_text:String,

    pub(crate) distro_name:String,

    pub(crate) cpu_name:String,
    pub(crate) cpu_speed:String,
    pub(crate) cpu_cores:String,
}

pub async fn get_distro(distro_name:&str)->Result<(),Error>{
    let request_url = format!("https://distro-api.up.railway.app/distribution?identifier={0}",distro_name);
    println!("requesting from {}", request_url);

    let response = reqwest::get(&request_url).await?;

    let raw_distro: RawDistro = response.json().await?;

    println!("{:?}", raw_distro);

    Ok(())
}