use std::io::{Write};
use std::env;
extern crate term;

pub fn print_prompt() {
    let user = env::var("USER").unwrap();
    let mut pwd = env::var("PWD").unwrap();
    let home = env::var("HOME").unwrap(); 

    let mut t = term::stdout().unwrap();

    pwd = pwd.replace(home.as_str(), "~");


    color_write(t.as_mut(), term::color::GREEN, "[");
    color_write(t.as_mut(), term::color::GREEN, &user);
    write!(t, " ").unwrap();
    color_write(t.as_mut(), term::color::BLUE, &pwd);
    color_write(t.as_mut(), term::color::GREEN, "> ");
    t.fg(term::color::WHITE).unwrap();
    t.flush().unwrap();
}

pub fn color_write(t : &mut term::StdoutTerminal, color: term::color::Color, text: &str) {
    t.fg(color).unwrap();
    write!(t, "{}", text).unwrap();
}