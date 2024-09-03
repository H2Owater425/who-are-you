mod constant;
mod time;
mod logger;
mod http;

use std::{error::Error, io::{BufRead, BufReader, Lines, Write}, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}};
use http::{HttpStatus, get_response};
use logger::Logger;

fn main() -> Result<(), Box<dyn Error>> {
	let listener: TcpListener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 80))?;
	let mut logger: Logger = Logger::new();

	for stream in listener.incoming() {
		let mut stream: TcpStream = stream?;
		let mut lines: Lines<BufReader<&mut TcpStream>> = BufReader::new(&mut stream).lines();

		if let Some(Ok(request_uri)) = lines.next() {
			logger.info(&request_uri)?;
			
			if let Some(end_index) = request_uri.rfind(' ') {
				if &request_uri[end_index..] == " HTTP/1.1" {
					if let Some(start_index) = request_uri.find(' ') {
						if &request_uri[..start_index + 1] == "GET " {
							stream.write_all(&get_response(HttpStatus::Ok, Some(vec!["Content-Type: text/plain"]), Some(&request_uri[start_index+1..end_index]))?)?;
						} else {
							stream.write_all(&get_response(HttpStatus::MethodNotAllowed, None, None)?)?;
						}
					} else {
						stream.write_all(&get_response(HttpStatus::BadRequest, None, None)?)?;
					}
				} else {
					stream.write_all(&get_response(HttpStatus::HTTPVersionNotSupported, None, None)?)?;
				}
			} else {
				stream.write_all(&get_response(HttpStatus::BadRequest, None, None)?)?;
			}
		} else {
			logger.error("Failed to read stream")?;
		}
	}

	Ok(())
}
