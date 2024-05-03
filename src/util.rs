use std::ptr::null;
use reqwest::Error;
use serde::Deserialize;
use sysinfo::{
    Components, Disks, Networks, System,
};

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

#[derive(Default)]
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
impl RenderData{
    pub(crate) fn get_cpu_info(mut self) -> RenderData  where Self: Sized {
        let mut sys = System::new_all();
        sys.refresh_all();
        self.cpu_speed = sys.cpus().first().unwrap().frequency().to_string();
        self.cpu_speed = sys.cpus().first().unwrap().frequency().to_string();
        self
    }
}