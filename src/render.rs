use crate::RenderData;
use inline_colorization::*;

const ICON_SIZE:i32 = 32;
pub fn render(data:RenderData){
    let data_lines = vec![data.distro_name.as_str()];
    let prefix_lines = vec!["Distro"];
    for i in 0..ICON_SIZE as usize{
        if i >= data_lines.len() {
            continue;
        }
        render_line(i as i32,prefix_lines[i],data_lines[i]);
    }
}

pub fn render_line(line:i32,prefix:&str,data: &str){
    println!("{0}{prefix}: {data}",get_icon_line(line));
}

pub fn get_icon_line(line:i32)->String{
    " ".repeat(ICON_SIZE as usize)
}