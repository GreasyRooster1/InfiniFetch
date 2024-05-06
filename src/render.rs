use std::fmt::format;
use std::string::ToString;
use crate::RenderData;
use inline_colorization::*;
use sysinfo::System;

const ICON_SIZE:i32 = 32;
const FULL_RESET:&str = "\x1B[49m\x1B[39m\x1B[0m";
const PREFIX_FORMAT: &str = "\x1B[35m\x1B[1m"; //format!("{color_magenta}{style_bold}").as_str();
const DATA_FORMAT: &str = "\x1B[36m"; //format!("{color_cyan}").as_str();
const SEPARATOR_FORMAT: &str = "\x1B[37m\x1B[1m"; //format!("{color_white}{style_bold}").as_str();

pub fn render(data:RenderData){
    let (prefix_lines,data_lines,options) = data.create_render_vecs();
    for i in 0..ICON_SIZE as usize{
        if i >= data_lines.len() {
            println!("{}", get_icon_line(&data, i));
            continue;
        }
        if options[i] {
            render_line(i, prefix_lines[i], data_lines[i],&data);
        }
    }
}

pub fn render_line(line:usize,prefix:&str,data: &str,render_data: &RenderData){
    println!("{0}{FULL_RESET}{PREFIX_FORMAT}{prefix}{FULL_RESET}{SEPARATOR_FORMAT}: {FULL_RESET}{DATA_FORMAT}{data}{FULL_RESET}",get_icon_line(render_data,line));
}

pub fn get_icon_line(data:&RenderData,line:usize)->String{
    if line>data.icon_text.len()-1{
        return " ".repeat(ICON_SIZE as usize);
    }
    return data.icon_text[line].clone();
}