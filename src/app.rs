use std::io::{self, Stdin, Stdout, Write};

use pcan_basic::socket::usb::UsbCanSocket;
use termion::{
    cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use crate::command::CommandHandler;

#[derive(Copy, Clone)]
pub enum Mode {
    Normal,
    Command,
}

pub struct AppState {
    pub mode: Mode,
    pub probe: Option<UsbCanSocket>,
}

pub struct App {
    cmd_handler: CommandHandler,
    stdin: Stdin,
    stdout: RawTerminal<Stdout>,
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            cmd_handler: CommandHandler::new(),
            stdin: io::stdin(),
            stdout: io::stdout().into_raw_mode().unwrap(),
            state: AppState::default(),
        }
    }

    pub fn run(mut self) {
        for c in self.stdin.keys() {
            match (self.state.mode, c.unwrap()) {
                (Mode::Normal, Key::Char('q')) => break,
                (Mode::Normal, Key::Char(';') | Key::Char(':')) => {
                    self.cmd_handler.start_command(&mut self.stdout);
                    self.state.mode = Mode::Command;
                }
                (Mode::Command, Key::Char('\n')) => {
                    self.cmd_handler
                        .execute_command(&mut self.state, &mut self.stdout);
                    self.state.mode = Mode::Normal;
                }
                (Mode::Command, Key::Esc) => {
                    self.cmd_handler.cancel_command(&mut self.stdout);
                    self.state.mode = Mode::Normal
                }
                (Mode::Command, key) => self.cmd_handler.handle_key(key, &mut self.stdout),
                _ => {}
            }

            self.stdout.flush().unwrap();
        }

        write!(self.stdout, "{}", cursor::Show).unwrap();
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: Mode::Normal,
            probe: None,
        }
    }
}
