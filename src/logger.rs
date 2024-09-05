use std::{error::Error, io::{stderr, stdout, Write}, sync::{Arc, Mutex, MutexGuard}};
use crate::time::Time;

enum Level {
	FATAL,
	ERROR,
	WARN,
	INFO,
	DEBUG,
	TRACE
}

impl Level {
	pub fn as_parts(self: &Self) -> (u8, &str, &str) {
		match self {
			Level::FATAL => (31, " ", "FATAL"),
			Level::ERROR => (31, " ", "ERROR"),
			Level::WARN => (33, "  ", "WARN"),
			Level::INFO => (32, "  ", "INFO"),
			Level::DEBUG => (32, " ", "DEBUG"),
			Level::TRACE => (32, " ", "TRACE")
		}
	}
}

#[derive(Clone)]
pub struct Logger {
	output: Arc<Mutex<dyn Write + Send>>,
	error: Arc<Mutex<dyn Write + Send>>,
}

impl Logger {
	pub fn new() -> Self {
		Self {
			output: Arc::new(Mutex::new(stdout())),
			error: Arc::new(Mutex::new(stderr()))
		}
	}

	fn write(&self, level: Level, message: &str) -> Result<(), Box<dyn Error>> {
		let parts: (u8, &str, &str) = level.as_parts();
		let time: Time = Time::now()?;
		let message: String = format!("[\x1b[36m{:02}:{:02}:{:02}\x1b[37m][\x1b[{}m{}\x1b[37m]{}{}\n", time.hour, time.minute, time.second, parts.0, parts.2, parts.1, message);

		let mut stream: MutexGuard<'_, dyn Write + Send> = if parts.0 >= 32 {
			self.output.lock().unwrap()
		} else {
			self.error.lock().unwrap()
		};

		stream.write_all(message.as_bytes())?;
		
		Ok(())
	}

	pub fn fatal(&self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::FATAL, message)
	}

	pub fn error(&self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::ERROR, message)
	}

	pub fn warn(&self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::WARN, message)
	}

	pub fn info(&self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::INFO, message)
	}

	pub fn debug(&self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::DEBUG, message)
	}

	pub fn trace(&self, message: &str) -> Result<(), Box<dyn Error>> {
		self.write(Level::TRACE, message)
	}
}
