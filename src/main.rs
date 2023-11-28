use clap::Command;

const VERSION:&str = "0.0.0 alpha";
const AUTHOR:&str = "Rotriw";
const ABOUT:&str = "A CLI tool for OJ";


mod scripts;

fn main() {
    let setmod = scripts::settings::command();
    Command::new("ojc")
    .author(AUTHOR)
    .version(VERSION)
    .subcommand(setmod)
    .about(ABOUT)
    .after_help("If you have any problem in using this project, please go to https://github.com/rotriw/OJConnect/issues and create an issue.\n ")
    .get_matches();
}