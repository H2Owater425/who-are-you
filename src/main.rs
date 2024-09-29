mod constant;
mod time;
mod logger;
mod http;
mod utility;
mod whois;

use std::{env::args, error::Error, io::{BufRead, BufReader, Lines, Write}, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}, thread::spawn};
use http::{HttpStatus, get_response};
use logger::Logger;
use utility::is_domain;
use whois::lookup;

fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = args().collect();
	let port: u16 = if args.len() == 2 {
		args[1].parse::<u16>()?
	} else {
		3000
	};
	let listener: TcpListener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port))?;
	let logger: Logger = Logger::new();

	drop(args);

	logger.info(&format!("Running at http://localhost:{}", port))?;

	for stream in listener.incoming() {
		let logger: Logger = logger.clone();
		
		spawn(move || {
			let mut stream: TcpStream = stream.unwrap();
			let mut lines: Lines<BufReader<&mut TcpStream>> = BufReader::new(&mut stream).lines();

			if let Some(Ok(request_uri)) = lines.next() {
				logger.info(&request_uri).unwrap();

				if let Some(end_index) = request_uri.rfind(' ') {
					if &request_uri[end_index..] == " HTTP/1.1" {
						if let Some(start_index) = request_uri.find(' ') {
							if &request_uri[..start_index + 1] == "GET " {
								let domain_length: usize = end_index - start_index;

								if domain_length <= 253 && domain_length >= 4 {
									let domain: &str = &request_uri[start_index+2..end_index];

									if domain != "favicon.ico" {
										if is_domain(domain) {
											let result: String = lookup("whois.iana.org", &format!("{}\r\n", domain)).unwrap();

											if result.len() > 124 {
												let server: &str = &result[124..];
												let server: &str = &server[..server.find('\n').unwrap()];

												if is_domain(server) {
													send!(stream, HttpStatus::Ok, Some(vec!["Content-Type: text/plain; charset=utf8"]), Some(&lookup(server, &format!("{}\r\n", domain)).unwrap()));
												} else {
													send!(stream, HttpStatus::InternalServerError, None, None);
												}
											} else {
												send!(stream, HttpStatus::InternalServerError, None, None);
											}
										} else {
											send!(stream, HttpStatus::BadRequest, None, None);
										}
									} else {
										send!(stream, HttpStatus::NoContent, None, None);
									}
								} else {
									send!(stream, HttpStatus::BadRequest, None, None);
								}
							} else {
								send!(stream, HttpStatus::MethodNotAllowed, None, None);
							}
						} else {
							send!(stream, HttpStatus::BadRequest, None, None);
						}
					} else {
						send!(stream, HttpStatus::HTTPVersionNotSupported, None, None);
					}
				} else {
					send!(stream, HttpStatus::BadRequest, None, None);
				}
			} else {
				logger.error("Failed to read stream").unwrap();
			}
		});
	}

	Ok(())
}
