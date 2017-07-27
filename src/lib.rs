extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "hyper")]
extern crate hyper;

#[macro_use]
pub mod macros;

#[derive(Debug, Clone, Copy)]
pub enum HttpCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ContentDifferent = 210,
    IMUsed = 226,
    MultipleChoices = 300,
    MovedPermanently = 301,
    MovedTemporarily = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    SwitchProxy = 306,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    TooManyRedirects = 310,
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
    RequestedRangeUnsatisfiable = 416,
    ExpectationFailed = 417,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
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
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
    Unregistered = 600
}

impl HttpCode {
    pub fn description(&self) -> &'static str {
        match *self {
            HttpCode::Continue => "The server has received the request headers and the client should proceed to send the request body",
            HttpCode::SwitchingProtocols => "The requester has asked the server to switch protocols and the server has agreed to do so.",
            HttpCode::Processing => "A WebDAV request may contain many sub-requests involving file operations, requiring a long time to complete the request.",
            HttpCode::OK => "Standard response for successful HTTP requests.",
            HttpCode::Created => "The request has been fulfilled, resulting in the creation of a new resource.",
            HttpCode::Accepted => "he request has been accepted for processing, but the processing has not been completed.",
            HttpCode::NonAuthoritativeInformation => "The server is a transforming proxy (e.g. a Web accelerator) that received a 200 OK from its origin, but is returning a modified version of the origin's response.",
            HttpCode::NoContent => "The server successfully processed the request and is not returning any content.",
            HttpCode::ResetContent => "The server successfully processed the request, but is not returning any content.",
            HttpCode::PartialContent => "The server is delivering only part of the resource (byte serving) due to a range header sent by the client.",
            HttpCode::MultiStatus => "The message body that follows is an XML message and can contain a number of separate response codes, depending on how many sub-requests were made.",
            HttpCode::AlreadyReported => "The members of a DAV binding have already been enumerated in a preceding part of the (multistatus) response, and are not being included again.",
            HttpCode::ContentDifferent => "The copy of the client-side resource differs from that of the server (content or properties).",
            HttpCode::IMUsed => "The server has fulfilled a request for the resource, and the response is a representation of the result of one or more instance-manipulations applied to the current instance.",
            HttpCode::MultipleChoices => "Indicates multiple options for the resource from which the client may choose (via agent-driven content negotiation).",
            HttpCode::MovedPermanently => "This and all future requests should be directed to the given URI.",
            HttpCode::MovedTemporarily => "This is an example of industry practice contradicting the standard.",
            HttpCode::SeeOther => "The response to the request can be found under another URI using a GET method.",
            HttpCode::NotModified => "Indicates that the resource has not been modified since the version specified by the request headers If-Modified-Since or If-None-Match.",
            HttpCode::UseProxy => "The requested resource is available only through a proxy, the address for which is provided in the response.",
            HttpCode::SwitchProxy => "No longer used. Originally meant 'Subsequent requests should use the specified proxy'.",
            HttpCode::TemporaryRedirect => "In this case, the request should be repeated with another URI; however, future requests should still use the original URI.",
            HttpCode::PermanentRedirect => "The request and all future requests should be repeated using another URI.",
            HttpCode::TooManyRedirects => "The request must have been redirected too many times, or suffers from a redirect loop.",
            HttpCode::BadRequest => "The server cannot or will not process the request due to an apparent client error.",
            HttpCode::Unauthorized => "Similar to 403 Forbidden, but specifically for use when authentication is required and has failed or has not yet been provided.",
            HttpCode::PaymentRequired => "Reserved for future use. The original intention was that this code might be used as part of some form of digital cash or micropayment scheme, as proposed for example by GNU Taler, but that has not yet happened, and this code is not usually used.",
            HttpCode::Forbidden => "The request was valid, but the server is refusing action.",
            HttpCode::NotFound => "The requested resource could not be found but may be available in the future.",
            HttpCode::MethodNotAllowed => "A request method is not supported for the requested resource.",
            HttpCode::NotAcceptable => "The requested resource is capable of generating only content not acceptable according to the Accept headers sent in the request.",
            HttpCode::ProxyAuthenticationRequired => "The client must first authenticate itself with the proxy.",
            HttpCode::RequestTimeout => "The server timed out waiting for the request.",
            HttpCode::Conflict => "Indicates that the request could not be processed because of conflict in the request, such as an edit conflict between multiple simultaneous updates.",
            HttpCode::Gone => "Indicates that the resource requested is no longer available and will not be available again.",
            HttpCode::LengthRequired => "The request did not specify the length of its content, which is required by the requested resource.",
            HttpCode::PreconditionFailed => "The server does not meet one of the preconditions that the requester put on the request.",
            HttpCode::RequestEntityTooLarge => "The request is larger than the server is willing or able to process.",
            HttpCode::RequestURITooLong => "The URI provided was too long for the server to process.",
            HttpCode::UnsupportedMediaType => "The request entity has a media type which the server or resource does not support.",
            HttpCode::RequestedRangeUnsatisfiable => "The client has asked for a portion of the file (byte serving), but the server cannot supply that portion.",
            HttpCode::ExpectationFailed => "The server cannot meet the requirements of the Expect request-header field.",
            HttpCode::MisdirectedRequest => "The request was directed at a server that is not able to produce a response.",
            HttpCode::UnprocessableEntity => "The request was well-formed but was unable to be followed due to semantic errors.",
            HttpCode::Locked => "The resource that is being accessed is locked.",
            HttpCode::FailedDependency => "The request failed due to failure of a previous request (e.g., a PROPPATCH).",
            HttpCode::UpgradeRequired => "The client should switch to a different protocol such as TLS/1.0, given in the Upgrade header field.",
            HttpCode::PreconditionRequired => "The origin server requires the request to be conditional.",
            HttpCode::TooManyRequests => "The user has sent too many requests in a given amount of time.",
            HttpCode::RequestHeaderFieldsTooLarge => "The server is unwilling to process the request because either an individual header field, or all the header fields collectively, are too large.",
            HttpCode::UnavailableForLegalReasons => "A server operator has received a legal demand to deny access to a resource or to a set of resources that includes the requested resource.",
            HttpCode::InternalServerError => "A generic error message, given when an unexpected condition was encountered and no more specific message is suitable.",
            HttpCode::NotImplemented => "The server either does not recognize the request method, or it lacks the ability to fulfil the request.",
            HttpCode::BadGateway => "The server was acting as a gateway or proxy and received an invalid response from the upstream server.",
            HttpCode::ServiceUnavailable => "The server is currently unavailable (because it is overloaded or down for maintenance).",
            HttpCode::GatewayTimeout => "The server was acting as a gateway or proxy and did not receive a timely response from the upstream server.",
            HttpCode::HTTPVersionNotSupported => "The server does not support the HTTP protocol version used in the request.",
            HttpCode::VariantAlsoNegotiates => "Transparent content negotiation for the request results in a circular reference.",
            HttpCode::InsufficientStorage => "The server is unable to store the representation needed to complete the request.",
            HttpCode::LoopDetected => "The server detected an infinite loop while processing the request (sent in lieu of 208 Already Reported).",
            HttpCode::NotExtended => "Further extensions to the request are required for the server to fulfil it.",
            HttpCode::NetworkAuthenticationRequired => "The client needs to authenticate to gain network access.",
            HttpCode::Unregistered => "A status code not in the IANA HTTP status code registry or very well known",
        }
    }
}

impl From<u16> for HttpCode {
    fn from(src: u16) -> HttpCode {
        match src {
            100 => HttpCode::Continue,
            101 => HttpCode::SwitchingProtocols,
            102 => HttpCode::Processing,
            200 => HttpCode::OK,
            201 => HttpCode::Created,
            202 => HttpCode::Accepted,
            203 => HttpCode::NonAuthoritativeInformation,
            204 => HttpCode::NoContent,
            205 => HttpCode::ResetContent,
            206 => HttpCode::PartialContent,
            207 => HttpCode::MultiStatus,
            208 => HttpCode::AlreadyReported,
            210 => HttpCode::ContentDifferent,
            226 => HttpCode::IMUsed,
            300 => HttpCode::MultipleChoices,
            301 => HttpCode::MovedPermanently,
            302 => HttpCode::MovedTemporarily,
            303 => HttpCode::SeeOther,
            304 => HttpCode::NotModified,
            305 => HttpCode::UseProxy,
            306 => HttpCode::SwitchProxy,
            307 => HttpCode::TemporaryRedirect,
            308 => HttpCode::PermanentRedirect,
            310 => HttpCode::TooManyRedirects,
            400 => HttpCode::BadRequest,
            401 => HttpCode::Unauthorized,
            402 => HttpCode::PaymentRequired,
            403 => HttpCode::Forbidden,
            404 => HttpCode::NotFound,
            405 => HttpCode::MethodNotAllowed,
            406 => HttpCode::NotAcceptable,
            407 => HttpCode::ProxyAuthenticationRequired,
            408 => HttpCode::RequestTimeout,
            409 => HttpCode::Conflict,
            410 => HttpCode::Gone,
            411 => HttpCode::LengthRequired,
            412 => HttpCode::PreconditionFailed,
            413 => HttpCode::RequestEntityTooLarge,
            414 => HttpCode::RequestURITooLong,
            415 => HttpCode::UnsupportedMediaType,
            416 => HttpCode::RequestedRangeUnsatisfiable,
            417 => HttpCode::ExpectationFailed,
            421 => HttpCode::MisdirectedRequest,
            422 => HttpCode::UnprocessableEntity,
            423 => HttpCode::Locked,
            424 => HttpCode::FailedDependency,
            426 => HttpCode::UpgradeRequired,
            428 => HttpCode::PreconditionRequired,
            429 => HttpCode::TooManyRequests,
            431 => HttpCode::RequestHeaderFieldsTooLarge,
            451 => HttpCode::UnavailableForLegalReasons,
            500 => HttpCode::InternalServerError,
            501 => HttpCode::NotImplemented,
            502 => HttpCode::BadGateway,
            503 => HttpCode::ServiceUnavailable,
            504 => HttpCode::GatewayTimeout,
            505 => HttpCode::HTTPVersionNotSupported,
            506 => HttpCode::VariantAlsoNegotiates,
            507 => HttpCode::InsufficientStorage,
            508 => HttpCode::LoopDetected,
            510 => HttpCode::NotExtended,
            511 => HttpCode::NetworkAuthenticationRequired,
            _ => HttpCode::Unregistered,
        }
    }
}

#[cfg(feature = "hyper")]
impl From<hyper::StatusCode> for HttpCode {
    fn from(h: hyper::StatusCode) -> HttpCode {
        HttpCode::from(u16::from(h))
    }
}


#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct HttpError<T: serde::Serialize> {
    code: u16,
    message: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    msgid: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    extra: Option<T>
}


impl<T: serde::Serialize> HttpError<T> {
    pub fn new(code: HttpCode, message: Option<&'static str>, msgid: Option<&'static str>, extra: Option<T>) -> HttpError<T> {
        HttpError {
            code: code as u16,
            message: message.unwrap_or(code.description()),
            msgid: msgid,
            extra: extra
        }
    }
    pub fn extra(&mut self, extra: T) {
        self.extra = Some(extra);
    }
}

impl<T: serde::Serialize> From<HttpCode> for HttpError<T> {
    fn from(httpcode: HttpCode) -> HttpError<T> {
        HttpError::new(httpcode, None, None, None)
    }
}

impl<T: serde::Serialize> std::fmt::Display for HttpError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

#[cfg(feature = "hyper")]
impl<T: serde::Serialize> From<hyper::StatusCode> for HttpError<T> {
    fn from(h: hyper::StatusCode) -> HttpError<T> {
        let code = HttpCode::from(h);
        HttpError::new(code, h.canonical_reason(), None, None)
    }
}
