#[macro_export]
macro_rules! http_error {
    ($httpcode:expr) => {
        $crate::HttpError::<i16>::new($httpcode, None, None, None)
    };
    ($httpcode:expr, $message:expr) => {
        $crate::HttpError::<i16>::new($httpcode,  Some($message), None, None)
    };
    ($httpcode:expr, $message:expr, $extra:expr) => {
        $crate::HttpError::new($httpcode, Some($message), None, Some($extra))
    };
    ($httpcode:expr, $message:expr, $extra:expr, $msgid:expr) => {
        $crate::HttpError::new($httpcode,  Some($message), $msgid, Some($extra))
    };
}
