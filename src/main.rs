pub mod app;
pub mod baud_ext;
pub mod command;

use app::App;
use termion::{clear, cursor, terminal_size};

fn print_start_screen() {
    const WELCOME_TEXT: &str = "Welcome to FreeCAN-View!";
    let (width, height) = match terminal_size() {
        Ok(size) => size,
        Err(_) => (80, 60),
    };

    let x = (width / 2 + 1).saturating_sub((WELCOME_TEXT.len() / 2) as u16);

    println!(
        "{}{}{}{}☭",
        clear::All,
        cursor::Goto(x, height / 2 + 1),
        WELCOME_TEXT,
        cursor::Goto(width - 1, height - 1),
    );
}

fn main() {
    print_start_screen();

    let app = App::new();
    app.run();
}
