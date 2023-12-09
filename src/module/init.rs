use std::fs;
use std::path::Path;

use clap::Command;
use crate::basics::basic;
use crate::basics;

pub fn command() -> Command {
    basic::command("init")
    .about("Init the cli")
    .arg_required_else_help(false)
}

pub fn init_folder(path: &String) {
    println!("Init the folder");
    // println!("{:?}", );
    let parser = String::from(shellexpand::tilde(path));
    fs::create_dir_all(Path::new(&parser)).unwrap();
    fs::create_dir_all(Path::new(&parser).join("oj")).unwrap();
}

pub fn matchd(_cli :&clap::ArgMatches) {
    println!("Init the cli");
    let mut config = json::JsonValue::Object(json::object::Object::new());
    let config_path = basic::ask_storage_path();
    config["storage"] = json::JsonValue::String(config_path.clone());
    basics::config::set_config("config.json", config);
    init_folder(&config_path);
}