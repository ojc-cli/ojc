use clap::Command;


pub fn command() -> Command {
    return Command::new("set")
        .about("Set the settings of Cli");
}

