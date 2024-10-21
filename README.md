# Typed Enum

### String Enum

```rs
use map_enum::*;

#[derive(Default)]
#[StringEnum]
pub enum HTTPVersion {
    Http0_9 = "HTTP/0.9",
    Http1_0 = "HTTP/1.0",
    #[default]
    Http1_1 = "HTTP/1.1",
    Http2 = "HTTP/2",
    Http3 = "HTTP/3",
}
```

Introduces new enum grammar under the `#[StringEnum]` macro, which implements string serialization and deserialization traits. 
When compiling with the `nom` feature, additional parser methods are provided.
