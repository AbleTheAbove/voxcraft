//use std::convert::From;
use std::fs;
use toml::Value;

use crate::ROOT_PATH;
#[derive(Debug)]
pub struct Config {
    pub height: i64,
    pub width: i64,
    pub ui_scale: i64,
    pub vsync: bool,
}
// This will return the config
// On the mean time just fake it
pub fn load_config() -> Config {
    let filename = format!("{}/config.toml", ROOT_PATH);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let value = contents.parse::<Value>().unwrap();
    let mut config = Config {
        height: 0,
        width: 0,
        ui_scale: 1,
        vsync: false,
    };
    /* //This insures the values exist
    match value["height"].as_integer() {
        Some(e) => {
            config.height = e;
        }
        _ => {}
    }*/
    config.width = value["width"].as_integer().unwrap();
    config.height = value["height"].as_integer().unwrap();
    config.ui_scale = value["ui_scale"].as_integer().unwrap();
    config.vsync = value["vsync"].as_bool().unwrap();
    config
}
