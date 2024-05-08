mod linux;
mod windows;
mod util;
mod render;

use serde::Deserialize;
use reqwest::Error;
use tokio::*;
use std::env::consts;
use crate::linux::handle_linux;
use linux::*;
use windows::*;
use util::*;
use crate::render::render;
use crate::RenderData;

#[tokio::main]
async fn main() -> Result<(), Error> {

    //check os type
    let render_data :RenderData = match consts::OS {
        "linux" => handle_linux(),
        "windows" => handle_windows(),
        "macos" => println!("get your ass of OSX"),
        &_ => return Ok(())
    };

    render(render_data);

    Ok(())
}