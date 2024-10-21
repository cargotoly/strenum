//! This is an example of an HTTP version enum generated with
//! the `StringEnum` macro.

use map_enum::StringEnum;

/// This enum represents the versions of the HTTP protocol.
#[derive(Clone, Default)]
#[StringEnum]
pub enum Version {
    Http0_9 = "HTTP/0.9",
    Http1_0 = "HTTP/1.0",
    #[default]
    Http1_1 = "HTTP/1.1",
    Http2 = "HTTP/2",
    Http3 = "HTTP/3",
}

#[test]
fn version() {
    assert_eq!(Version::Http1_1.str_len(), 8);
}