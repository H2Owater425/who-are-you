#[macro_export]
macro_rules! send {
	($stream:expr, $status:expr, $header:expr, $body:expr) => {
		$stream.write_all(&get_response($status, $header, $body).unwrap()).unwrap();
		$stream.flush().unwrap();
	};
}

pub fn is_domain(domain: &str) -> bool {
	domain.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-')
}

// TODO: Add optional macro