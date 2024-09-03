use std::error::Error;

use crate::time::Time;

pub enum HttpStatus {
	Continue = 100,
	SwitchingProtocols = 101,
	Processing = 102,
	Checkpoint = 103,
	Ok = 200,
	Created = 201,
	Accepted = 202,
	NonAuthoritativeInformation = 203,
	NoContent = 204,
	ResetContent = 205,
	PartialContent = 206,
	MultiStatus = 207,
	AlreadyReported = 208,
	MultipleChoices = 300,
	MovedPermanently = 301,
	Found = 302,
	SeeOther = 303,
	NotModified = 304,
	UseProxy = 305,
	SwitchProxy = 306,
	TemporaryRedirect = 307,
	PermanentRedirect = 308,
	BadRequest = 400,
	Unauthorized = 401,
	PaymentRequired = 402,
	Forbidden = 403,
	NotFound = 404,
	MethodNotAllowed = 405,
	NotAcceptable = 406,
	ProxyAuthenticationRequired = 407,
	RequestTimeout = 408,
	Conflict = 409,
	Gone = 410,
	LengthRequired = 411,
	PreconditionFailed = 412,
	RequestEntityTooLarge = 413,
	RequestURITooLong = 414,
	UnsupportedMediaType = 415,
	RequestedRangeNotSatisfiable = 416,
	ExpectationFailed = 417,
	Imateapot = 418,
	UnprocessableEntity = 421,
	MisdirectedRequest = 422,
	Locked = 423,
	FailedDependency = 424,
	UpgradeRequired = 426,
	PreconditionRequired = 428,
	TooManyRequests = 429,
	RequestHeaderFiledsTooLarge = 431,
	UnavailableForLegalReasons = 451,
	InternalServerError = 500,
	NotImplemented = 501,
	BadGateway = 502,
	ServiceUnavailable = 503,
	GatewayTimeout = 504,
	HTTPVersionNotSupported = 505,
	VariantAlsoNegotiates = 506,
	InsufficientStorage = 507,
	LoopDetected = 508,
	BandwidthLimitExceeded = 509,
	NotExtended = 510,
	NetworkAuthenticationRequired = 511
}

impl HttpStatus {
	pub fn as_str(self: &Self) -> &'static str {
		match self {
			HttpStatus::Continue => "100 Continue",
			HttpStatus::SwitchingProtocols => "101 Switching Protocols",
			HttpStatus::Processing => "102 Processing",
			HttpStatus::Checkpoint => "103 Checkpoint",
			HttpStatus::Ok => "200 OK",
			HttpStatus::Created => "201 Created",
			HttpStatus::Accepted => "202 Accepted",
			HttpStatus::NonAuthoritativeInformation => "203 Non-Authoritative Information",
			HttpStatus::NoContent => "204 No Content",
			HttpStatus::ResetContent => "205 Reset Content",
			HttpStatus::PartialContent => "206 Partial Content",
			HttpStatus::MultiStatus => "207 Multi-Status",
			HttpStatus::AlreadyReported => "208 Already Reported",
			HttpStatus::MultipleChoices => "300 Multiple Choices",
			HttpStatus::MovedPermanently => "301 Moved Permanently",
			HttpStatus::Found => "302 Found",
			HttpStatus::SeeOther => "303 See Other",
			HttpStatus::NotModified => "304 Not Modified",
			HttpStatus::UseProxy => "305 Use Proxy",
			HttpStatus::SwitchProxy => "306 Switch Proxy",
			HttpStatus::TemporaryRedirect => "307 Temporary Redirect",
			HttpStatus::PermanentRedirect => "308 Permanent Redirect",
			HttpStatus::BadRequest => "400 Bad Request",
			HttpStatus::Unauthorized => "401 Unauthorized",
			HttpStatus::PaymentRequired => "402 Payment Required",
			HttpStatus::Forbidden => "403 Forbidden",
			HttpStatus::NotFound => "404 Not Found",
			HttpStatus::MethodNotAllowed => "405 Method Not Allowed",
			HttpStatus::NotAcceptable => "406 Not Acceptable",
			HttpStatus::ProxyAuthenticationRequired => "407 Proxy Authentication Required",
			HttpStatus::RequestTimeout => "408 Request Time-out",
			HttpStatus::Conflict => "409 Conflict",
			HttpStatus::Gone => "410 Gone",
			HttpStatus::LengthRequired => "411 Length Required",
			HttpStatus::PreconditionFailed => "412 Precondition Failed",
			HttpStatus::RequestEntityTooLarge => "413 Request Entity Too Large",
			HttpStatus::RequestURITooLong => "414 Request-URI Too Long",
			HttpStatus::UnsupportedMediaType => "415 Unsupported Media Type",
			HttpStatus::RequestedRangeNotSatisfiable => "416 Requested Range Not Satisfiable",
			HttpStatus::ExpectationFailed => "417 Expectation Failed",
			HttpStatus::Imateapot => "418 I'm a teapot",
			HttpStatus::UnprocessableEntity => "421 Unprocessable Entity",
			HttpStatus::MisdirectedRequest => "422 Misdirected Request",
			HttpStatus::Locked => "423 Locked",
			HttpStatus::FailedDependency => "424 Failed Dependency",
			HttpStatus::UpgradeRequired => "426 Upgrade Required",
			HttpStatus::PreconditionRequired => "428 Precondition Required",
			HttpStatus::TooManyRequests => "429 Too Many Requests",
			HttpStatus::RequestHeaderFiledsTooLarge => "431 Request Header Fileds Too Large",
			HttpStatus::UnavailableForLegalReasons => "451 Unavailable For Legal Reasons",
			HttpStatus::InternalServerError => "500 Internal Server Error",
			HttpStatus::NotImplemented => "501 Not Implemented",
			HttpStatus::BadGateway => "502 Bad Gateway",
			HttpStatus::ServiceUnavailable => "503 Service Unavailable",
			HttpStatus::GatewayTimeout => "504 Gateway Timeout",
			HttpStatus::HTTPVersionNotSupported => "505 HTTP Version Not Supported",
			HttpStatus::VariantAlsoNegotiates => "506 Variant Also Negotiates",
			HttpStatus::InsufficientStorage => "507 Insufficient Storage",
			HttpStatus::LoopDetected => "508 Loop Detected",
			HttpStatus::BandwidthLimitExceeded => "509 Bandwidth Limit Exceeded",
			HttpStatus::NotExtended => "510 Not Extended",
			HttpStatus::NetworkAuthenticationRequired => "511 Network Authentication Required"
		}
	}
}

pub fn get_response(status: HttpStatus, headers: Option<Vec<&str>>, body: Option<&str>) -> Result<Vec<u8>, Box<dyn Error>> {
	let mut response: String = format!("HTTP/1.1 {}\r\nDate: {}", status.as_str(), Time::now()?.as_imf_fixdate());

	if let Some(headers) = headers {
			for header in headers {
					response.push_str(&format!("\r\n{}", header));
			}
	}

	if let Some(body) = body {
			response.push_str(&format!("Content-Length: {}\r\n\r\n{}", body.len(), body));
	}

	Ok(response.into_bytes())
}
