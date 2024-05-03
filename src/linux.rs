use std::fs;
use crate::RenderData;
use crate::util::get_cpu_info;

pub fn handle_linux()->RenderData{
    println!("detected linux!");
    let contents = fs::read_to_string("/etc/lsb-release")
        .expect("Should have been able to read the file");
    let name_line = contents.lines().next().unwrap();
    let distro_name = name_line.replace("DISTRIB_ID=","");

    let mut render_data:RenderData = Default::default();
    render_data.get_cpu_info()
}