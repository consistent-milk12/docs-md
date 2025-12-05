# Crate `httparse`

A push library for parsing HTTP/1.x requests and responses.

The focus is on speed and safety. Unsafe code is used to keep parsing fast,
but unsafety is contained in a submodule, with invariants enforced. The
parsing internals use an `Iterator` instead of direct indexing, while
skipping bounds checks.

With Rust 1.27.0 or later, support for SIMD is enabled automatically.
If building an executable to be run on multiple platforms, and thus
not passing `target_feature` or `target_cpu` flags to the compiler,
runtime detection can still detect SSE4.2 or AVX2 support to provide
massive wins.

If compiling for a specific target, remembering to include
`-C target_cpu=native` allows the detection to become compile time checks,
making it *even* faster.

## Structs

### `InvalidChunkSize`

```rust
struct InvalidChunkSize;
```

An error in parsing a chunk size.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &InvalidChunkSize) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ParserConfig`

```rust
struct ParserConfig {
    // [REDACTED: Private Fields]
}
```

Parser configuration.

#### Implementations

- `fn allow_spaces_after_header_name_in_responses(self: &mut Self, value: bool) -> &mut Self`
  Sets whether spaces and tabs should be allowed after header names in responses.

- `fn allow_multiple_spaces_in_request_line_delimiters(self: &mut Self, value: bool) -> &mut Self`
  Sets whether multiple spaces are allowed as delimiters in request lines.

- `fn multiple_spaces_in_request_line_delimiters_are_allowed(self: &Self) -> bool`
  Whether multiple spaces are allowed as delimiters in request lines.

- `fn allow_multiple_spaces_in_response_status_delimiters(self: &mut Self, value: bool) -> &mut Self`
  Sets whether multiple spaces are allowed as delimiters in response status lines.

- `fn multiple_spaces_in_response_status_delimiters_are_allowed(self: &Self) -> bool`
  Whether multiple spaces are allowed as delimiters in response status lines.

- `fn allow_obsolete_multiline_headers_in_responses(self: &mut Self, value: bool) -> &mut Self`
  Sets whether obsolete multiline headers should be allowed.

- `fn obsolete_multiline_headers_in_responses_are_allowed(self: &Self) -> bool`
  Whether obsolete multiline headers should be allowed.

- `fn allow_space_before_first_header_name(self: &mut Self, value: bool) -> &mut Self`
  Sets whether white space before the first header is allowed

- `fn space_before_first_header_name_are_allowed(self: &Self) -> bool`
  Whether white space before first header is allowed or not

- `fn parse_request<'buf>(self: &Self, request: &mut Request<'_, 'buf>, buf: &'buf [u8]) -> Result<usize>`
  Parses a request with the given config.

- `fn parse_request_with_uninit_headers<'headers, 'buf>(self: &Self, request: &mut Request<'headers, 'buf>, buf: &'buf [u8], headers: &'headers mut [MaybeUninit<Header<'buf>>]) -> Result<usize>`
  Parses a request with the given config and buffer for headers

- `fn ignore_invalid_headers_in_responses(self: &mut Self, value: bool) -> &mut Self`
  Sets whether invalid header lines should be silently ignored in responses.

- `fn ignore_invalid_headers_in_requests(self: &mut Self, value: bool) -> &mut Self`
  Sets whether invalid header lines should be silently ignored in requests.

- `fn parse_response<'buf>(self: &Self, response: &mut Response<'_, 'buf>, buf: &'buf [u8]) -> Result<usize>`
  Parses a response with the given config.

- `fn parse_response_with_uninit_headers<'headers, 'buf>(self: &Self, response: &mut Response<'headers, 'buf>, buf: &'buf [u8], headers: &'headers mut [MaybeUninit<Header<'buf>>]) -> Result<usize>`
  Parses a response with the given config and buffer for headers

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ParserConfig`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> ParserConfig`

### `Request<'headers, 'buf>`

```rust
struct Request<'headers, 'buf> {
    pub method: Option<&'buf str>,
    pub path: Option<&'buf str>,
    pub version: Option<u8>,
    pub headers: &'headers mut [Header<'buf>],
}
```

A parsed Request.

The optional values will be `None` if a parse was not complete, and did not
parse the associated property. This allows you to inspect the parts that
could be parsed, before reading more, in case you wish to exit early.

# Example

```no_run
let buf = b"GET /404 HTTP/1.1\r\nHost:";
let mut headers = [httparse::EMPTY_HEADER; 16];
let mut req = httparse::Request::new(&mut headers);
let res = req.parse(buf).unwrap();
if res.is_partial() {
    match req.path {
        Some(ref path) => {
            // check router for path.
            // /404 doesn't exist? we could stop parsing
        },
        None => {
            // must read more and parse again
        }
    }
}
```

#### Fields

- **`method`**: `Option<&'buf str>`

  The request method, such as `GET`.

- **`path`**: `Option<&'buf str>`

  The request path, such as `/about-us`.

- **`version`**: `Option<u8>`

  The request minor version, such as `1` for `HTTP/1.1`.

- **`headers`**: `&'headers mut [Header<'buf>]`

  The request headers.

#### Implementations

- `fn new(headers: &'h mut [Header<'b>]) -> Request<'h, 'b>`
  Creates a new Request, using a slice of headers you allocate.

- `fn parse_with_uninit_headers(self: &mut Self, buf: &'b [u8], headers: &'h mut [MaybeUninit<Header<'b>>]) -> Result<usize>`
  Try to parse a buffer of bytes into the Request,

- `fn parse(self: &mut Self, buf: &'b [u8]) -> Result<usize>`
  Try to parse a buffer of bytes into the Request.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Eq<'headers, 'buf>`

##### `impl PartialEq<'headers, 'buf>`

- `fn eq(self: &Self, other: &Request<'headers, 'buf>) -> bool`

##### `impl StructuralPartialEq<'headers, 'buf>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'headers, 'buf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Response<'headers, 'buf>`

```rust
struct Response<'headers, 'buf> {
    pub version: Option<u8>,
    pub code: Option<u16>,
    pub reason: Option<&'buf str>,
    pub headers: &'headers mut [Header<'buf>],
}
```

A parsed Response.

See `Request` docs for explanation of optional values.

#### Fields

- **`version`**: `Option<u8>`

  The response minor version, such as `1` for `HTTP/1.1`.

- **`code`**: `Option<u16>`

  The response code, such as `200`.

- **`reason`**: `Option<&'buf str>`

  The response reason-phrase, such as `OK`.
  
  Contains an empty string if the reason-phrase was missing or contained invalid characters.

- **`headers`**: `&'headers mut [Header<'buf>]`

  The response headers.

#### Implementations

- `fn new(headers: &'h mut [Header<'b>]) -> Response<'h, 'b>`
  Creates a new `Response` using a slice of `Header`s you have allocated.

- `fn parse(self: &mut Self, buf: &'b [u8]) -> Result<usize>`
  Try to parse a buffer of bytes into this `Response`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Eq<'headers, 'buf>`

##### `impl PartialEq<'headers, 'buf>`

- `fn eq(self: &Self, other: &Response<'headers, 'buf>) -> bool`

##### `impl StructuralPartialEq<'headers, 'buf>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'headers, 'buf>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Header<'a>`

```rust
struct Header<'a> {
    pub name: &'a str,
    pub value: &'a [u8],
}
```

Represents a parsed header.

#### Fields

- **`name`**: `&'a str`

  The name portion of a header.
  
  A header name must be valid ASCII-US, so it's safe to store as a `&str`.

- **`value`**: `&'a [u8]`

  The value portion of a header.
  
  While headers **should** be ASCII-US, the specification allows for
  values that may not be, and so the value is stored as bytes.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'a>`

- `fn clone(self: &Self) -> Header<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'a>`

##### `impl Eq<'a>`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &Header<'a>) -> bool`

##### `impl StructuralPartialEq<'a>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `Error`

```rust
enum Error {
    HeaderName,
    HeaderValue,
    NewLine,
    Status,
    Token,
    TooManyHeaders,
    Version,
}
```

An error in parsing.

#### Variants

- **`HeaderName`**

  Invalid byte in header name.

- **`HeaderValue`**

  Invalid byte in header value.

- **`NewLine`**

  Invalid byte in new line.

- **`Status`**

  Invalid byte in Response status.

- **`Token`**

  Invalid byte where token is required.

- **`TooManyHeaders`**

  Parsed more headers than provided buffer can contain.

- **`Version`**

  Invalid byte in HTTP version.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Error`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Error) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Status<T>`

```rust
enum Status<T> {
    Complete(T),
    Partial,
}
```

The result of a successful parse pass.

`Complete` is used when the buffer contained the complete value.
`Partial` is used when parsing did not reach the end of the expected value,
but no invalid data was found.

#### Variants

- **`Complete`**

  The completed result.

- **`Partial`**

  A partial result.

#### Implementations

- `fn is_complete(self: &Self) -> bool`
  Convenience method to check if status is complete.

- `fn is_partial(self: &Self) -> bool`
  Convenience method to check if status is partial.

- `fn unwrap(self: Self) -> T`
  Convenience method to unwrap a Complete value. Panics if the status is

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<T: $crate::clone::Clone>`

- `fn clone(self: &Self) -> Status<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<T: $crate::marker::Copy>`

##### `impl Eq<T: $crate::cmp::Eq>`

##### `impl PartialEq<T: $crate::cmp::PartialEq>`

- `fn eq(self: &Self, other: &Status<T>) -> bool`

##### `impl StructuralPartialEq<T>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `parse_headers`

```rust
fn parse_headers<'b: 'h, 'h>(src: &'b [u8], dst: &'h mut [Header<'b>]) -> Result<(usize, &'h [Header<'b>])>
```

Parse a buffer of bytes as headers.

The return value, if complete and successful, includes the index of the
buffer that parsing stopped at, and a sliced reference to the parsed
headers. The length of the slice will be equal to the number of properly
parsed headers.

# Example

```rust
let buf = b"Host: foo.bar\nAccept: */*\n\nblah blah";
let mut headers = [httparse::EMPTY_HEADER; 4];
assert_eq!(httparse::parse_headers(buf, &mut headers),
           Ok(httparse::Status::Complete((27, &[
               httparse::Header { name: "Host", value: b"foo.bar" },
               httparse::Header { name: "Accept", value: b"*/*" }
           ][..]))));
```

### `parse_chunk_size`

```rust
fn parse_chunk_size(buf: &[u8]) -> result::Result<Status<(usize, u64)>, InvalidChunkSize>
```

Parse a buffer of bytes as a chunk size.

The return value, if complete and successful, includes the index of the
buffer that parsing stopped at, and the size of the following chunk.

# Example

```rust
let buf = b"4\r\nRust\r\n0\r\n\r\n";
assert_eq!(httparse::parse_chunk_size(buf),
           Ok(httparse::Status::Complete((3, 4))));
```

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<Status<T>, Error>;
```

A Result of any parsing action.

If the input is invalid, an `Error` will be returned. Note that incomplete
data is not considered invalid, and so will not return an error, but rather
a `Ok(Status::Partial)`.

## Constants

### `EMPTY_HEADER`

```rust
const EMPTY_HEADER: Header<'static>;
```

An empty header, useful for constructing a `Header` array to pass in for
parsing.

# Example

```rust
let headers = [httparse::EMPTY_HEADER; 64];
```

