use std::env;
use std::io::Write;
use termion::{color, style};

const EXTRACHARACTERS : usize = 4;

pub fn print_prompt() -> u16 {
    let user = env::var("USER").unwrap();
    let mut pwd = String::from(env::current_dir().unwrap().to_str().unwrap());
    let home = env::var("HOME").unwrap();

    pwd = pwd.replace(home.as_str(), "~");

    let prompt = format!("{}{}[{} {}{}{}> {}{}",
        style::Bold,
        color::Fg(color::Green),
        user,
        color::Fg(color::Blue),
        pwd,
        color::Fg(color::Green),
        color::Fg(color::Reset),
        style::Reset
    );

    print!("{}", prompt);

    std::io::stdout().flush().unwrap();

    return (pwd.len() + user.len() + EXTRACHARACTERS) as u16;
}
