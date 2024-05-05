use std::fs;
use crate::render::render;
use crate::RenderData;

pub fn handle_linux()->RenderData{
    println!("detected linux!");
    let contents = fs::read_to_string("/etc/lsb-release")
        .expect("Should have been able to read the file");
    let name_line = contents.lines().next().unwrap();
    let distro_name = name_line.replace("DISTRIB_ID=","");

    let mut render_data:RenderData = Default::default();
    render_data.distro_name = distro_name;
    render_data = render_data.setup();
    render_data = render_data.get_info();
    render_data = render_data.get_image_linux();

    render_data
}