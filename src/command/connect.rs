use std::io::Stdout;

use pcan_basic::{
    bus::UsbBus,
    socket::{usb::UsbCanSocket, Baudrate},
};
use termion::raw::RawTerminal;

use crate::{app::AppState, baud_ext::BaudExt};

pub fn connect(
    args: &[&str],
    state: &mut AppState,
    _terminal: &mut RawTerminal<Stdout>,
) -> Result<String, String> {
    if args.len() > 1 {
        return Err("Too many arguments".to_owned());
    }

    let baud = match args.get(0) {
        Some(baud) => Baudrate::try_from_str(baud)
            .map_err(|_| format!("Not a known baud rate: `{}`", baud))?,
        None => Baudrate::Baud250K,
    };

    // Remove the probe, if something was already connected.
    state.probe.take();

    let baud_string = baud.to_string();
    match UsbCanSocket::open(UsbBus::USB1, baud) {
        Ok(probe) => {
            state.probe = Some(probe);
            Ok("Connected to probe with Baudrate ".to_owned() + &baud_string)
        }
        Err(e) => Err(format!("Unable to connect to probe: {:?}", e)),
    }
}
