use std::process;
use std::io::{stderr, stdout, Write};

mod builtins;
use builtins::{exit::exit};

mod evalerror;
use evalerror::EvalError;

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
                Err(e) => println!("exit: {}", e.message()),
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
                    Err(err) => println!("{}: {}", self.command, err.message()),
                    _ => (),
                }

                return true;
            }
        }

        false
    }
}
