mod linux;
mod windows;

mod lib;

use serde::Deserialize;
use reqwest::Error;
use tokio::*;
use std::env::consts;
use crate::linux::handle_linux;
use linux::*;
use windows::*;
use lib::*;

#[tokio::main]
async fn main() -> Result<(), Error> {

    //check os type
    match consts::OS {
        "linux" => handle_linux(),
        "windows" => handle_windows(),
        &_ => return Ok(())
    }

    Ok(())
}