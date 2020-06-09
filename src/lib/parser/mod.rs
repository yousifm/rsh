use std::io::{stdin, stdout, Write};
use std::env;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor::DetectCursorPos;
use super::command::Command;

pub fn read_command(prompt_length: u16) -> Option<Command> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut line = String::new();
    
    for c in stdin.keys() {
        let pos = stdout.cursor_pos().unwrap();

        match c.unwrap() {
            Key::Char('\n') => {
                write!(stdout, "\n{}", termion::cursor::Goto(1, pos.1 + 1)).unwrap();
                return parse_command(line);
            },
            Key::Backspace => {
                line.pop();
                update_cursor_pos(prompt_length, &line);
            },
            Key::Ctrl('l') => {
                write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
                return None;
            },
            Key::Char(c) => {
                line.push(c);
                update_cursor_pos(prompt_length, &line);
            }
            _ => (),
        }

        stdout.flush().unwrap();
    }
    return None;
}

fn update_cursor_pos(prompt_length: u16, line: &String) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let pos = stdout.cursor_pos().unwrap();

    write!(stdout, "{}{}", termion::cursor::Goto(prompt_length + 1, pos.1), termion::clear::AfterCursor).unwrap();
    write!(stdout, "{}", line).unwrap();
}

fn parse_command(line : String) -> Option<Command> {
    let command: String;
    let mut args: Vec<String>;

    if line.is_empty() {
        return None;
    }

    let separated = line.split_whitespace();

    // Convert from &str iterator to Vec<String>
    args = separated.map(|val| {
        let mut val = String::from(val);

        // Replace value by environment value
        if is_env_var(&val) {
            val = to_env_var_value(&val);
        }

        val
    }).collect();

    // Command is first 'word'
    command = args.remove(0);

    Some(Command::new(&command, args))
}

fn is_env_var(value: &String) -> bool {
    value.starts_with("$")
}

fn to_env_var_value(value: &String) -> String {
    match env::var(&value[1..]) {
        Ok(var_val) => var_val,
        Err(_) => String::new(),
    }
}
