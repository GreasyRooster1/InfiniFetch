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
    pub(crate) cpu_frequency:String,
    pub(crate) cpu_brand:String,
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
    pub(crate) fn create_render_vecs(&self)->(Vec<&str>,Vec<&str>){
        let names = vec![
            "Operating System",
            "CPU Name",
            "CPU Brand",
            "CPU Frequency",
            "CPU Cores",
        ];
        let data = vec![
            self.distro_name.as_str(),
            self.cpu_name.as_str(),
            self.cpu_brand.as_str(),
            self.cpu_frequency.as_str(),
            self.cpu_cores.as_str(),
        ];
        (names,data)
    }
    pub(crate) fn get_cpu_info(mut self) -> RenderData  where Self: Sized {
        let mut sys = System::new_all();
        sys.refresh_all();
        self.cpu_name = sys.cpus().first().unwrap().vendor_id().to_string();
        self.cpu_brand = sys.cpus().first().unwrap().brand().to_string();

        let freq = sys.cpus().first().unwrap().frequency() as f32;
        if freq>1000f32{
            self.cpu_frequency = (freq/1000f32).to_string();
            self.cpu_frequency.push_str("GHz");
        }else{
            self.cpu_frequency.push_str("MHz");
        }

        self.cpu_cores = sys.physical_core_count().unwrap().to_string();
        self
    }
    // pub(crate) fn get_gpu_info(mut self)->RenderData where Self:Sized{
    //     let mut sys = System::new_all();
    //     sys.refresh_all();
    //     r
    // }
}