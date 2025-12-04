*[ureq_proto](../index.md) / [parser](index.md)*

---

# Module `parser`

Low level HTTP parser

This is to bridge `httparse` crate to `http` crate.

## Functions

### `try_parse_response`

```rust
fn try_parse_response<const N: usize>(input: &[u8]) -> Result<Option<(usize, http::Response<()>)>, crate::Error>
```

Parse bytes into a complete response.

Complete means that the last HTTP header is followed by an `\r\n`.

If the result is `None`, the bytes did not contain a full response. That
typically means you need to read more bytes and append to the in input buffer
before trying again.

The first `usize` in the resulting pair, is the number of bytes required from
the input buffer to form the response.

The const `N` is the number of headers to max expect. If the input has more
headers than `N` you get an error [`Error::HttpParseTooManyHeaders`](#httpparsetoomanyheaders).

### `try_parse_partial_response`

```rust
fn try_parse_partial_response<const N: usize>(input: &[u8]) -> Result<Option<http::Response<()>>, crate::Error>
```

Try parsing as much as possible of a response.

To get a result we need at least the complete initial status row,
but we don't need complete headers.

The const `N` is the number of headers to max expect. If the input has more
headers than `N` you get an error [`Error::HttpParseTooManyHeaders`](#httpparsetoomanyheaders).

### `try_parse_request`

```rust
fn try_parse_request<const N: usize>(input: &[u8]) -> Result<Option<(usize, http::Request<()>)>, crate::Error>
```

Parse bytes into a complete request.

Complete means that the last HTTP header is followed by an `\r\n`.

If the result is `None`, the bytes did not contain a full request. That
typically means you need to read more bytes and append to the in input buffer
before trying again.

The first `usize` in the resulting pair, is the number of bytes required from
the input buffer to form the request.

The const `N` is the number of headers to max expect. If the input has more
headers than `N` you get an error [`Error::HttpParseTooManyHeaders`](#httpparsetoomanyheaders).

