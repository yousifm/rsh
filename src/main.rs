use lib::{*};

fn main() {
    loop {
        prompt::print_prompt();
        let command = match parser::read_command() {
            Some(command) => command,
            None => continue,
        }; 
        command.exec();
    }
}