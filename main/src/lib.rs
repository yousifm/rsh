use std::error::Error;
use std::fmt;
use std::io::{stderr, stdin, stdout, Write};
use std::process;

pub struct Command {
    command: String,
    args: Vec<String>,
}

#[derive(Debug)]
pub struct EvalError {
    message: String,
}

impl Error for EvalError {}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl EvalError {
    fn new(message: &str) -> EvalError {
        EvalError {
            message: String::from(message),
        }
    }

    fn from_string(message: String) -> EvalError {
        EvalError { message }
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

    Command { command, args }
}

pub fn eval_command(command: &Command) {
    if command.command == "exit" {
        match exit(&command.args) {
            Err(e) => println!("exit: {}", e.message),
            _ => (),
        }
    } else {
        let mut cmd = process::Command::new(&command.command);
        cmd.args(&command.args);

        match cmd.output() {
            Ok(val) => {
                stdout().write_all(&val.stdout).unwrap();
                stderr().write_all(&val.stderr).unwrap();
            }
            Err(_) => println!("Unrecognized command: {}", command.command),
        }
    }
}

fn exit(args: &Vec<String>) -> Result<(), EvalError> {
    if args.len() > 1 {
        Err(EvalError::new("Too many arguments"))
    } else {
        let exit_code: i32;

        if args.len() == 1 {
            let arg_val = &args[0];

            exit_code = match arg_val.parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    return Err(EvalError::from_string(format!(
                        "Argument '{}' is not a valid integer",
                        arg_val
                    )))
                }
            };
        } else {
            exit_code = 0;
        }
        process::exit(exit_code);
    }
}