mod lib;

use std::{error::Error, io::{BufRead, BufReader, Lines, Write}, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}};
use lib::time::Time;

fn main() -> Result<(), Box<dyn Error>> {
	let listener: TcpListener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 80))?;

	for stream in listener.incoming() {
		let mut stream: TcpStream = stream?;
		let mut lines: Lines<BufReader<&mut TcpStream>> = BufReader::new(&mut stream).lines();

		match lines.next() {
			Some(request_uri) => {
				stream.write_all(format!("HTTP/1.1 {}\r\nServer: DHMO\r\nDate: {}\r\nContent-Length: 0\r\n\r\n", if request_uri? == "GET / HTTP/1.1" {
					"200 OK"
				} else {
					"404 Not Found"
				}, Time::now()?.as_imf_fixdate()).as_bytes())?;
			},
			None => {
				eprint!("Failed to read stream")
			}
		}
	}

	Ok(())
}
