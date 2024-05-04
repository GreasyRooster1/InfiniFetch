use crate::RenderData;
use inline_colorization::*;

const ICON_SIZE:i32 = 32;
pub fn render(data:RenderData){
    let (prefix_lines,data_lines) = data.create_render_vecs();
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