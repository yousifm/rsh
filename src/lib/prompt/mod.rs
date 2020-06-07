use std::io::{stdout, Write};
use std::env;

pub fn print_prompt() {
    let user = env::var("USER").unwrap();
    let mut pwd = env::var("PWD").unwrap();
    let home = env::var("HOME").unwrap(); 

    pwd = pwd.replace(home.as_str(), "~");

    print!("[{} {}> ", user, pwd);
    stdout().flush().unwrap();
}