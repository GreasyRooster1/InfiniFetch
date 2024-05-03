use std::fs;
use crate::RenderData;

pub fn handle_linux()->RenderData{
    println!("detected linux!");
    let contents = fs::read_to_string("/etc/lsb-release")
        .expect("Should have been able to read the file");
    let name_line = contents.lines().next().unwrap();
    let distro_name = name_line.replace("DISTRIB_ID=","");
    println!("{}", distro_name);
    RenderData{
        icon_text: " ".to_string(),
        distro_name,
    }
}