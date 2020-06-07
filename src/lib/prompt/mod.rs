use std::env;
use std::io::Write;
use ansi_term::Color::{Green, Blue, };

pub fn print_prompt() {
    let user = env::var("USER").unwrap();
    let mut pwd = env::var("PWD").unwrap();
    let home = env::var("HOME").unwrap(); 

    pwd = pwd.replace(home.as_str(), "~");

    print!("{}", Green.bold().paint("["));
    print!("{} ", Green.bold().underline().paint(user));
    print!("{}", Blue.bold().paint(pwd));
    print!("{}", Green.bold().paint("> "));

    std::io::stdout().flush().unwrap();
}
