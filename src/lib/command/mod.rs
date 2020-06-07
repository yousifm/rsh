use std::error::Error;
use std::fmt;
use std::process;
use std::io::{stderr, stdout, Write};

type CommandFunction = fn(&Vec<String>) -> Result<(), EvalError>;

pub struct Command {
    command: String,
    args: Vec<String>,
}

impl Command {
    pub fn new(command: &str, args: Vec<String>) -> Command {
        Command {
            command: String::from(command),
            args,
        }
    }

    pub fn exec (&self) {
        if self.try_builtin() {
            match exit(&self.args) {
                Err(e) => println!("exit: {}", e.message),
                _ => (),
            }
        } else {
            let mut cmd = process::Command::new(&self.command);
            cmd.args(&self.args);
    
            match cmd.output() {
                Ok(val) => {
                    stdout().write_all(&val.stdout).unwrap();
                    stderr().write_all(&val.stderr).unwrap();
                }
                Err(_) => println!("Unrecognized command: {}", self.command),
            }
        }
    }

    fn try_builtin (&self) -> bool {
        static BUILT_INS : &'static [(&'static str, CommandFunction)]= &[
            ("exit", exit),
        ];

        for built_in in BUILT_INS {
            if built_in.0 == self.command.to_string() {
                match (built_in.1)(&self.args) {
                    Err(err) => println!("{}: {}", self.command, err.message),
                    _ => (),
                }

                return true;
            }
        }

        false
    }
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