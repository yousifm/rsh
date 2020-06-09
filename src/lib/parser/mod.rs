use super::command::Command;
use std::env;
use std::io::{stdin, stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn read_command(prompt_length: u16) -> Option<Command> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut line = String::new();
    let mut line_pointer = 0;

    for c in stdin.keys() {
        let pos = stdout.cursor_pos().unwrap();

        match c.unwrap() {
            Key::Char('\n') => {
                write!(stdout, "{}", termion::cursor::Goto(1, pos.1 + 1)).unwrap();
                stdout.flush().unwrap();

                return parse_command(line);
            }
            Key::Backspace => {
                if line.len() > 0 && line_pointer > 0{
                    line.remove(line_pointer - 1);
                    line_pointer -= 1;
                    update_current_line_on_terminal(prompt_length, &line, line_pointer);
                }
            }
            Key::Left => {
                if line_pointer > 0 {
                    line_pointer -= 1;
                    update_cursor_pos_on_current_line(pos.0 - 1);
                }
            }
            Key::Right => {
                if line_pointer < line.len() {
                    line_pointer += 1;
                    update_cursor_pos_on_current_line(pos.0 + 1);
                }
            }
            Key::Ctrl('l') => {
                write!(
                    stdout,
                    "{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1)
                )
                .unwrap();
                return None;
            }
            Key::Char(c) => {
                line.insert(line_pointer, c);
                line_pointer += 1;
                update_current_line_on_terminal(prompt_length, &line, line_pointer);
            }
            _ => (),
        }

        stdout.flush().unwrap();
    }
    return None;
}

fn update_current_line_on_terminal(prompt_length: u16, line: &String, line_pointer: usize) {
    update_cursor_pos_on_current_line(prompt_length + 1);
    clear_after_cursor();
    write_line(&line);
    update_cursor_pos_on_current_line(prompt_length + 1 + line_pointer as u16);
}

fn update_cursor_pos_on_current_line(dist: u16) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let pos = stdout.cursor_pos().unwrap();

    write!(stdout, "{}", termion::cursor::Goto(dist, pos.1)).unwrap();
}

fn clear_after_cursor() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::clear::AfterCursor).unwrap();
}

fn write_line(line: &String) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", line).unwrap();
}

fn parse_command(line: String) -> Option<Command> {
    let command: String;
    let mut args: Vec<String>;

    if line.is_empty() {
        return None;
    }

    let separated = line.split_whitespace();

    // Convert from &str iterator to Vec<String>
    args = separated
        .map(|val| {
            let mut val = String::from(val);

            // Replace value by environment value
            if is_env_var(&val) {
                val = to_env_var_value(&val);
            }

            val
        })
        .collect();

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
