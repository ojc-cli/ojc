use clap::Command;

const VERSION:&str = "0.0.0 alpha";
const AUTHOR:&str = "Rotriw";
const ABOUT:&str = "A CLI tool for OJ";

mod basics;
mod scripts;
mod macros;

fn main() {
    let cli = Command::new("ojc")
    .author(AUTHOR)
    .version(VERSION)
    .about(ABOUT)
    .after_help("If you have any problem in using this project, please go to https://github.com/ojc-cli/ojc/issues and create an issue.\n ")
    .arg_required_else_help(true);// If run ./ojc, show help page.
    subcommands![cli, settings, init];
}