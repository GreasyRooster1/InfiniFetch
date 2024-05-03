use std::fs;
use crate::RenderData;
use os_version;
use os_version::{detect, OsVersion};

pub fn handle_windows()->RenderData{
    println!("detected windows!");

    let version:OsVersion = match detect() {
        Ok(ver) => {
            ver
        }
        Err(_) => {
            panic!("error occurred!");
        }
    };

    let windows = match version {
        OsVersion::Windows(win) => {win}
        _=>{
            panic!("this system is somehow not windows!");
        }
    };

    RenderData{
        icon_text: "".to_string(),
        distro_name: format!("Windows {0}",windows.version),
    }
}