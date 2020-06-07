use lib::{*};

fn main() {
    loop {
        prompt::print_prompt();
        let command = parser::read_command(); 
        command.exec();
    }
}