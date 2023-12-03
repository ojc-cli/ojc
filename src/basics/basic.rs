use clap::Command;
use inquire::Confirm;

pub fn command(name: &'static str) -> Command {
    Command::new(name)
    .arg_required_else_help(true)
}

pub fn risk_question() -> bool {
    let risk_question = Confirm::new("Do you know the risk of this action?").prompt();
    match risk_question {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => {
            println!("Error with question, try again later.");
            false
        },
    }
}

pub fn ask_storage_path() -> String {
    let path = inquire::Text::new("Please input the storage path: ").with_default("~/.ojc/").prompt();
    match path {
        Ok(path) => path,
        Err(_) => {
            println!("Error with input, try again later.");
            String::from("exit")
        },
    }
}