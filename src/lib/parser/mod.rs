use std::io::{stdin, };
use super::command::Command;

pub fn read_command() -> Command {
    let command: String;
    let mut args: Vec<String>;

    let mut line = String::new();

    stdin().read_line(&mut line).unwrap();

    // Remove trailing new line character
    line.truncate(line.len() - 1);

    let separated = line.split_whitespace();

    // Convert from &str iterator to Vec<String>
    args = separated.map(|val| String::from(val)).collect();

    // Command is first 'word'
    command = args.remove(0);

    Command::new(&command, args)
}
