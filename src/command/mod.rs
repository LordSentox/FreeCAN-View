mod connect;

use std::io::{Stdout, Write};

use termion::{clear, color, cursor, event::Key, raw::RawTerminal, terminal_size};

use crate::app::AppState;

/// Handle command input including displaying
pub struct CommandHandler {
    command: String,
    last_result: Option<Result<String, String>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_command(&mut self, terminal: &mut RawTerminal<Stdout>) {
        self.command.clear();
	self.last_result = None;
        self.present(terminal);
    }

    pub fn cancel_command(&mut self, terminal: &mut RawTerminal<Stdout>) {
        self.command.clear();
        self.clear_command_line(terminal);
    }

    pub fn handle_key(&mut self, key: Key, stdout: &mut RawTerminal<Stdout>) {
        match key {
            Key::Char(c) => self.command.push(c),
            Key::Backspace => {
                self.command.pop();
            }
            _ => {}
        }
        self.present(stdout);
    }

    pub fn execute_command(&mut self, state: &mut AppState, terminal: &mut RawTerminal<Stdout>) {
        let split: Vec<&str> = self.command.split_whitespace().collect();
        if split.is_empty() {
            return;
        }

        match split[0] {
            "connect" => self.last_result = Some(connect::connect(&split[1..], state, terminal)),
            other => self.last_result = Some(Err(format!("Unknown command `{}`", other))),
        }

	self.present(terminal);
    }

    fn present(&self, terminal: &mut RawTerminal<Stdout>) {
        let show = match &self.last_result {
            Some(res) => match res {
                Ok(s) => format!("{}> {}", color::Fg(color::Green), s),
                Err(e) => format!("{}! {}", color::Fg(color::Red), e),
            },
            None => format!(":{}", self.command),
        };

        write!(
            terminal,
            "{}{}{}",
            cursor::Goto(1, terminal_size().unwrap().1 - 1),
            clear::CurrentLine,
            show
        )
        .unwrap();
    }

    fn clear_command_line(&self, terminal: &mut RawTerminal<Stdout>) {
        write!(
            terminal,
            "{}{}",
            cursor::Goto(1, terminal_size().unwrap().1 - 1),
            clear::CurrentLine
        )
        .unwrap();
    }
}

impl Default for CommandHandler {
    fn default() -> Self {
        Self {
            command: String::new(),
            last_result: None,
        }
    }
}
