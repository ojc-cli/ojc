use std::fs;
use std::io::Error;

use json::JsonValue;

pub fn get_config(filepath: &'static str) -> Result<JsonValue, Error> {
    let file = fs::read_to_string(filepath);
    match file {
        Ok(file) => {
            Ok(json::parse(file.as_str()).unwrap())
        },
        Err(e) => Err(e),
    }
}

pub fn set_config(filepath: &'static str, config: JsonValue) {
    let file = fs::write(filepath, config.dump());
    match file {
        Ok(_) => {},
        Err(e) => {
            println!("Error with writing config file: {}", e);
        },
    }
}