use std::fs;
use crate::RenderData;

pub fn handle_windows()->RenderData{
    println!("detected windows!");
    RenderData{
        icon_text: "".to_string(),
        distro_name: "windows 10".to_string(),
    }
}