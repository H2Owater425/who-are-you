mod constant;
mod time;
mod logger;

use std::{error::Error, io::{BufRead, BufReader, Lines, Write}, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}, sync::Arc};
use logger::Logger;
use time::Time;

fn main() -> Result<(), Box<dyn Error>> {
	let listener: TcpListener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 80))?;
	let mut logger: Logger = Logger::new();

	for stream in listener.incoming() {
		let mut stream: TcpStream = stream?;
		let mut lines: Lines<BufReader<&mut TcpStream>> = BufReader::new(&mut stream).lines();

		if let Some(Ok(request_uri)) = lines.next() {
			logger.info(&request_uri)?;
			
			stream.write_all(format!("HTTP/1.1 200 OK\r\nServer: DHMO\r\nDate: {}\r\nContent-Length: 0\r\n\r\n", Time::now()?.as_imf_fixdate()).as_bytes())?;
		} else {
			logger.error("Failed to read stream")?;
		}
	}

	Ok(())
}
