use std::io::Write;
use std::ops::Add;
use std::ptr::null;
use reqwest::Error;
use serde::Deserialize;
use sysinfo::{Components, Disk, Disks, Networks, System};
use machine_info::{GraphicCard, Machine};
use tokio::runtime::Handle;
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use image::imageops::FilterType;
use rascii_art::{render_image, render_image_to, render_to, RenderOptions};


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
    pub(crate) sys:System,

    pub(crate) icon_text:Vec<String>,

    pub(crate) distro_name:String,

    pub(crate) cpu_name:String,
    pub(crate) cpu_frequency:String,
    pub(crate) cpu_brand:String,
    pub(crate) cpu_cores:String,

    pub(crate) max_ram:String,
    pub(crate) free_ram:String,

    pub(crate) max_disc:String,
    pub(crate) free_disc:String,
    pub(crate) disc_amount:String,

    pub(crate) gpu_name:String,
    pub(crate) gpu_memory:String,
}

pub async fn get_distro(distro_name: String) ->RawDistro{
    let request_url = format!("https://distro-api.up.railway.app/distribution?identifier={0}",distro_name.to_lowercase().replace(" ","_"));
    println!("requesting from {}", request_url);

    let response = reqwest::get(&request_url).await.unwrap();

    let raw_distro = response.json().await.unwrap();

    raw_distro
}

pub async fn download_image(url:String){
    let mut file = std::fs::File::create("image.png").unwrap();
    println!("requesting from {}", url);

    let response = reqwest::get(&url).await.unwrap();

    let bytes = response.bytes().await.unwrap();
    file.write_all(&*bytes).unwrap();
}

pub fn get_luma(r:u8,g:u8,b:u8)->f32{
    return 0.299*(r as f32) + 0.587*(g as f32) + 0.114*(b as f32);
}

pub fn convert_bytes(bytes:u64)->String{
    match bytes {
        b if b<=1_000 =>round_conversion(b,1," bytes"),
        b if b<=1_000_000 =>round_conversion(b,1_000,"kB"),
        b if b<=1_000_000_000 =>round_conversion(b,1_000_000,"MB"),
        b if b<=1_000_000_000_000 =>round_conversion(b,1_000_000_000,"GB"),
        b if b<=1_000_000_000_000_000 =>round_conversion(b,1_000_000_000_000,"TB"),
        b if b<=1_000_000_000_000_000_000 =>round_conversion(b,1_000_000_000_000_000,"PB"),
        //honestly if you have more than this you need help lmao
        _ => {bytes.to_string()}
    }
}
pub fn round_conversion(number:u64,factor:u64,factor_name:&str) -> String{
    let f_num = number as f64;
    let fac_times_10 = (factor as f64)/10f64;
    let mut str = (f64::round(f_num/fac_times_10)/10.).to_string();
    str.push_str(factor_name);
    str
}

impl RenderData{
    pub(crate) fn setup(mut self) -> RenderData  where Self: Sized {
        self.sys = System::new_all();
        self.sys.refresh_all();
        self
    }
    pub(crate) fn create_render_vecs(&self)->(Vec<&str>,Vec<&str>,Vec<bool>){
        let names = vec![
            "Operating System",

            "CPU Name",
            "CPU Brand",
            "CPU Frequency",
            "CPU Cores",

            "RAM Usage",
            "RAM Total",

            "Disc Amount",
            "Disc Total",
            "Disc Free",

            "GPU Name",
            "GPU Memory",
        ];
        let data = vec![
            self.distro_name.as_str(),

            self.cpu_name.as_str(),
            self.cpu_brand.as_str(),
            self.cpu_frequency.as_str(),
            self.cpu_cores.as_str(),

            self.free_ram.as_str(),
            self.max_ram.as_str(),

            self.disc_amount.as_str(),
            self.max_disc.as_str(),
            self.free_disc.as_str(),

            self.gpu_name.as_str(),
            self.gpu_memory.as_str(),
        ];
        let mut options:Vec<bool> = vec![];
        for d in data.clone(){
            options.append(&mut vec![!d.eq("")]);
        }
        (names,data,options)
    }
    pub(crate) fn get_info(mut self) -> RenderData  where Self: Sized {
        let mut tmp = self.get_cpu_info();
        tmp = tmp.get_memory_info();
        tmp = tmp.get_gpu_info();

        tmp
    }
    pub(crate) fn get_cpu_info(mut self) -> RenderData  where Self: Sized {
        self.sys.refresh_all();
        self.cpu_name = self.sys.cpus().first().unwrap().vendor_id().to_string();
        self.cpu_brand = self.sys.cpus().first().unwrap().brand().to_string();

        let freq = self.sys.cpus().first().unwrap().frequency() as f32;
        if freq>1000f32{
            self.cpu_frequency = (freq/1000f32).to_string();
            self.cpu_frequency.push_str("GHz");
        }else{
            self.cpu_frequency.push_str("MHz");
        }

        self.cpu_cores = self.sys.physical_core_count().unwrap().to_string();
        self
    }
    pub(crate) fn get_memory_info(mut self)->RenderData where Self:Sized{
        self.sys.refresh_memory();

        self.free_ram = convert_bytes(self.sys.free_memory());
        self.max_ram = convert_bytes(self.sys.total_memory());

        let disks = Disks::new_with_refreshed_list();

        let mut total_space = 0;
        self.disc_amount = disks.len().to_string();

        for disc in disks.iter() {
            total_space+=disc.total_space();
        }
        self.max_disc = convert_bytes(total_space);

        let mut free_space = 0;
        for disc in disks.iter() {
            free_space+=disc.available_space();
        }
        self.free_disc = convert_bytes(free_space);

        self
    }

    pub fn get_gpu_info(mut self)->RenderData where Self:Sized{
        let mut m = Machine::new();
        let mut si = m.system_info();
        match si.graphics.first() {
            None => {
                self.gpu_name = "".to_string();
                self.gpu_memory = "".to_string();
            }
            Some(gpu) => {
                self.gpu_name = gpu.name.to_string();
                self.gpu_memory = convert_bytes(gpu.memory);
            }
        }


        self
    }
    pub fn get_image_linux(mut self) ->RenderData where Self:Sized{
        let handle = Handle::current();
        let _ = handle.enter();
        let raw_distro = futures::executor::block_on(get_distro(self.distro_name.clone()));
        println!("{}", raw_distro.image_path);
        futures::executor::block_on( download_image(raw_distro.image_path));

        let mut buffer = String::new();

        let mut image = ImageReader::open("image.png")
            .unwrap()
            .decode()
            .unwrap();
        let top_pixel = image.get_pixel(1,1).0;
        let invert = get_luma(top_pixel[0],top_pixel[0],top_pixel[0]) > 190f32;

        render_image_to(
            &image,
            &mut buffer,
            &RenderOptions::new()
                .width(64)
                .colored(true)
                .charset(&[".", ",", "*", "/", "?", "£", "#"])
                .invert(invert),
        ).unwrap();

        for line in buffer.split("\n"){
            let res = line.to_string().replace("."," ");
            self.icon_text.append(&mut vec![res]);
        }

        self
    }
}
