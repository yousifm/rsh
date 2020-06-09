use lib::{*};

fn main() {
    loop {
        let command = match parser::read_command() {
            Some(command) => command,
            None => continue,
        }; 
        command.exec();
    }
}