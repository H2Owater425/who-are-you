use std::{error::Error, io::{stderr, stdout, Stderr, Stdout, Write}};
use crate::time::Time;

enum Level {
	FATAL,
	ERROR,
	WARN,
	INFO,
	DEBUG,
	TRACE,
}

pub struct Logger {
	output: Stdout,
	error: Stderr,
}

impl Logger {
	pub fn new() -> Self {
		Self {
			output: stdout(),
			error: stderr()
		}
	}
	fn write(&mut self, level: Level, message: &str) -> Result<(), Box<dyn Error>> {
		let (color, space, level): (u8, &str, &str) = match level {
			Level::FATAL => (31, " ", "FATAL"),
			Level::ERROR => (31, " ", "ERROR"),
			Level::WARN => (33, "  ", "WARN"),
			Level::INFO => (32, "  ", "INFO"),
			Level::DEBUG => (32, " ", "DEBUG"),
			Level::TRACE => (32, " ", "TRACE")
		};

		let time: Time = Time::now()?;

		let message: String = format!("[\x1b[36m{:02}:{:02}:{:02}\x1b[37m][\x1b[{}m{}\x1b[37m]{}{}\n", time.hour, time.minute, time.second, color, level, space, message);

		let mut stream: Box<dyn Write> = if color >= 32 {
			Box::new(&self.output)
		} else {
			Box::new(&self.error)
		};

		stream.write_all(message.as_bytes())?;

		Ok(())
	}

	pub fn fatal(&mut self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::FATAL, message)
	}

	pub fn error(&mut self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::ERROR, message)
	}

	pub fn warn(&mut self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::WARN, message)
	}

	pub fn info(&mut self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::INFO, message)
	}

	pub fn debug(&mut self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::DEBUG, message)
	}

	pub fn trace(&mut self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::TRACE, message)
	}
}
