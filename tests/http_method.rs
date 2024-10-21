//! This is an example of an HTTP method enum generated with
//! the `StringEnum` macro.

use map_enum::StringEnum;

/// This enum represents the methods of the HTTP protocol. The
/// methods are used to indicate the desired action to be performed.
/// [See Mozilla Docs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods)
#[derive(Debug, Clone, Default)]
#[StringEnum]
pub enum Method {
    /// The GET method requests a representation of the specified
    /// resource. Requests using GET should only retrieve data and
    /// should not contain a request content.
    #[default]
    GET = "GET",

    /// The HEAD method asks for a response identical to a GET
    /// request, but without a response body.
    HEAD = "HEAD",

    /// The POST method submits an entity to the specified resource,
    /// often causing a change in state or side effects on the server.
    POST = "POST",

    /// The PUT method replaces all current representations of the
    /// target resource with the request content.
    PUT = "PUT",

    /// The DELETE method deletes the specified resource.
    DELETE = "DELETE",

    /// The CONNECT method establishes a tunnel to the server
    /// identified by the target resource.
    CONNECT = "CONNECT",

    /// The OPTIONS method describes the communication options for
    /// the target resource.
    OPTIONS = "OPTIONS",

    /// The TRACE method performs a message loop-back test along the
    /// path to the target resource.
    TRACE = "TRACE",

    /// The PATCH method applies partial modifications to a resource.
    PATCH = "PATCH",
}

#[test]
fn methods() {
    use std::str::FromStr;

    assert_eq!(Method::GET.to_string(), "GET".to_string());
    assert_eq!(Method::from_str("HEAD").unwrap(), Method::HEAD);
}
