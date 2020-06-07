use std::io::{stdin, stdout, Write};

use lib::{*};

fn main() {
    loop {
        print_prompt();
        let command = read_command(); 
        eval_command(&command);
    }
}

pub fn print_prompt() {
    print!("> ");
    stdout().flush().unwrap();
}

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

pub fn eval_command(command: &Command) {
    command.exec();
}
