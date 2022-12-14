
use std::fs::File;
use std::io::{Read, BufRead};
use std::sync::mpsc::{Receiver, Sender, channel, TryRecvError};
use std::thread;

use serde::Deserialize;

use crate::ccnet_dev::CcnetDevConfig;
use crate::cctalk_dev::CctalkDevConfig;
use crate::extbus::ExtbusConfig;
use crate::intio::IntioConfig;
use crate::iobus::IobusConfig;
use crate::ledmatrix::LedmatrixConfig;
use crate::ledpanel::LedpanelConfig;
use crate::wiegand_dev::WiegandConfig;
use crate::terminal::TerminalConfig;

#[derive(Deserialize)]
pub struct Config {
	pub extbus: ExtbusConfig,
	pub intio: IntioConfig,
	pub iobus: IobusConfig,
	pub ledmatrix: LedmatrixConfig,
	pub ledpanel: LedpanelConfig,
	pub rfid: WiegandConfig,
	pub terminal: TerminalConfig,
	pub ccnet: CcnetDevConfig,
	pub cctalk: CctalkDevConfig
}

pub fn parse_config(path: &str) -> Config {
	let mut file = File::open(&path).unwrap();
	let mut buf = Vec::new();
	file.read_to_end(&mut buf).unwrap();
	toml::from_slice(&buf).unwrap()
}


pub struct Exiter {
	rx: Receiver<()>
}

fn exiter_handler(tx: Sender<()>) {
	let mut stdin = std::io::stdin().lock();
	let mut buf = String::new();
	stdin.read_line(&mut buf).unwrap();
	drop(tx);
}

impl Exiter {
	pub fn new() -> Self {
		println!("\ttype 'Enter' for exit");
		let (tx, rx) = channel();
		thread::spawn(|| exiter_handler(tx));
		Self {
			rx: rx
		}
	}

	pub fn check(&self) -> bool {
		match self.rx.try_recv() {
			Ok(()) => false,
			Err(TryRecvError::Empty) => false,
			_ => true
		}
	}
}

pub struct InController {
	rx: Receiver<char>
}

fn incontroller_handler(tx: Sender<char>) {
	let mut stdin = std::io::stdin().lock();
	let mut buf = String::new();
	loop {
		stdin.read_line(&mut buf).unwrap();
		let c = buf.chars().next().unwrap();
		tx.send(c).unwrap();
		if c == 'q' {
			break;
		}
	}
}

impl InController {
	pub fn new(map: &[&str]) -> Self {
		println!("Control map:");
		println!("\tq - exiting the session");
		for row in map {
			println!("\t{}", row);
		}
		let (tx, rx) = channel();
		thread::spawn(|| incontroller_handler(tx));
		Self {
			rx: rx
		}
	}

	pub fn try_get(&self) -> Option<char> {
		match self.rx.try_recv() {
			Ok(c) => Some(c),
			Err(TryRecvError::Empty) => None,
			_ => Some('q')
		}
	}

	pub fn get(&self) -> char {
		match self.rx.recv() {
			Ok(c) => c,
			_ => 'q'
		}
	}
}