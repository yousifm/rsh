use lib::{*};

fn main() {
    loop {
        print_prompt();
        let command = read_command(); 
        eval_command(&command);
    }
}
