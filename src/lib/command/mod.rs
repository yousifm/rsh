use std::process;

mod builtins;
use builtins::{cd::cd, exit::exit};

pub mod evalerror;
pub use evalerror::EvalError;

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

    pub fn exec(&self) {
        if !self.try_builtin() {
            process::Command::new(&self.command)
                .args(&self.args)
                .spawn()
                .expect(format!("Unrecognized command: {}", self.command).as_ref())
                .wait()
                .unwrap();
        }
    }

    fn try_builtin(&self) -> bool {
        static BUILT_INS: &'static [(&'static str, CommandFunction)] =
            &[("exit", exit), ("cd", cd)];

        for built_in in BUILT_INS {
            if built_in.0 == self.command {
                let result = (built_in.1)(&self.args);
                match result {
                    Err(err) => println!("{}: {}", self.command, err.message()),
                    _ => (),
                }

                return true;
            }
        }

        false
    }
}
