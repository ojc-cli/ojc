use clap::{Command, arg, ArgMatches, Arg, ArgAction};
use ansi_term::Colour::{Red, Blue};
use crate::basics::basic;
use crate::basics;


pub fn command() -> Command {
    basic::command("settings")
    .about("Set the settings of cli")
    .alias("s")
    .subcommand(
        Command::new("path").alias("p")
        .about("Set the storage path of ojc cli (alias: p)")
        .args([
            arg!(<NAME> "change the storage path"),
            Arg::new("force").short('f').long("force").help("force to perform the operation").action(ArgAction::SetTrue),
        ])
    )
}

pub fn change_stoarge_path(path: &String) {
    println!("Change path to { }", path);
    let mut config = basics::config::get_config("config.json").unwrap();
    config["storage"] = json::JsonValue::String(path.clone());
    basics::config::set_config("config.json", config);
}

pub fn matchd(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("path") {
        let allow_perform = if matches.get_flag("force") {
            println!("{}", Red.paint("Force Mode Opend."));
            true
        } else {
            println!("{}", Red.paint("Please notice that change the storage path may cause can't handle oj module."));
            basic::risk_question()
        };
        if allow_perform == false {
            println!("{}", Blue.paint("Nothing is changed."));
            return;
        }
        let value = matches.get_one::<String>("NAME").unwrap();
        change_stoarge_path(value);
    }
}