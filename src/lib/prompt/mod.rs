use std::env;
use std::io::Write;
use termion::{color, style};

pub fn print_prompt() {
    let user = env::var("USER").unwrap();
    let mut pwd = String::from(env::current_dir().unwrap().to_str().unwrap());
    let home = env::var("HOME").unwrap();

    pwd = pwd.replace(home.as_str(), "~");

    print!(
        "{}{}[{} {}{}{}> {}{}",
        style::Bold,
        color::Fg(color::Green),
        user,
        color::Fg(color::Blue),
        pwd,
        color::Fg(color::Green),
        color::Fg(color::Reset),
        style::Reset,
    );
    std::io::stdout().flush().unwrap();
}
