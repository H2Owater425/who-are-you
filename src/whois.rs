use std::{io::{Error, Read, Write}, net::TcpStream};

pub fn lookup(server: &str, query: &str) -> Result<String, Error> {
	let mut socket: TcpStream = TcpStream::connect(format!("{}:43", server))?;
	let mut result: String = String::new();

	socket.write(query.as_bytes())?;

	socket.read_to_string(&mut result)?;

	Ok(result)
}