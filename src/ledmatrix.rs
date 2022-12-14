use std::{time::Duration, thread};

use serde::Deserialize;
use serialport::SerialPort;
use crate::utils;

const DISPLAY_WIDTH: u32 = 64;
const DISPLAY_HEIGHT: u32 = 32;
const DRAW_DATA_SIZE: usize = (DISPLAY_HEIGHT*DISPLAY_WIDTH/2 + 2) as usize; 

const TTY_DATA_SPEED: u32 = termios::os::linux::B500000;
const TTY_BREAK_SPEED: u32 = termios::os::linux::B1200;
const BREAK_DELAY: Duration = Duration::from_millis(10);

#[derive(Deserialize)]
pub struct LedmatrixConfig {
	driver: String,
	fill_delay: u32 
}

/// Send encoded draw data to display
fn send_draw_data(tty: &mut Box<dyn SerialPort>, buf: &[u8]) {
    let break_byte: [u8;1] = [0;1];
	tty.set_baud_rate(TTY_BREAK_SPEED).unwrap();
    tty.write(&break_byte).unwrap();
    thread::sleep(BREAK_DELAY);
    tty.set_baud_rate(TTY_DATA_SPEED).unwrap();
	tty.write(&buf).unwrap();
}

pub fn test(config: &LedmatrixConfig) -> Result<(), String> {
	println!("\n[LEDMATRIX] Test begin..");
	println!("Display should be filling of white color..");
	let mut dd_buf: [u8;DRAW_DATA_SIZE] = [0;DRAW_DATA_SIZE];
	let mut serial = match serialport::new(&config.driver, TTY_DATA_SPEED).open() {
		Ok(port) => port,
		Err(e) => return Err(format!("Fail to open serial port: {}", e.to_string()))
	};

	let exiter = utils::Exiter::new();
	let mut i = 0;
	loop {
		dd_buf[i] = 0xFF;
		i += 1;
		if i >= DRAW_DATA_SIZE {
			i = 0;
		}
		send_draw_data(&mut serial, &dd_buf);
		if exiter.check() {
			break;
		}
		thread::sleep(Duration::from_millis(config.fill_delay as u64));
	}
	Ok(())
}