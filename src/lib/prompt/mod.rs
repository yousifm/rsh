use std::io::{stdout, Write};

pub fn print_prompt() {
    print!("> ");
    stdout().flush().unwrap();
}