use clap::Command;
use crate::basics::basic;
use crate::basics;

pub fn command() -> Command {
    basic::command("init")
    .about("Init the cli")
    .arg_required_else_help(false)
}

pub fn matchd(_cli :&clap::ArgMatches) {
    println!("Init the cli");
    let mut config = json::JsonValue::Object(json::object::Object::new());
    config["storage"] = json::JsonValue::String(basic::ask_storage_path());
    basics::config::set_config("config.json", config);
}