#[macro_export]
macro_rules! send {
	($stream:expr, $status:expr, $header:expr, $body:expr) => {
		$stream.write_all(&get_response($status, $header, $body)?)?
	};
}