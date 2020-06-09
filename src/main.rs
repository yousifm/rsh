use lib::{*};

fn main() {
    loop {
        let prompt_length = prompt::print_prompt();
        let command = match parser::read_command(prompt_length) {
            Some(command) => command,
            None => continue,
        }; 
        command.exec();
    }
}