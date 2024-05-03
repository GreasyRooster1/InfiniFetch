use std::fs;

pub fn handle_linux(){
    println!("detected linux!");
    let contents = fs::read_to_string("/etc/lsb-release")
        .expect("Should have been able to read the file");
    let name_line = contents.lines().next().unwrap();
    let distro_name = name_line.replace("DISTRIB_ID=","");
    println!("{}", distro_name);
}