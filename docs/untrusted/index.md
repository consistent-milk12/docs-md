# Crate `untrusted`

untrusted.rs: Safe, fast, zero-panic, zero-crashing, zero-allocation
parsing of untrusted inputs in Rust.

<code>git clone https://github.com/briansmith/untrusted</code>

untrusted.rs goes beyond Rust's normal safety guarantees by  also
guaranteeing that parsing will be panic-free, as long as
`untrusted::Input::as_slice_less_safe()` is not used. It avoids copying
data and heap allocation and strives to prevent common pitfalls such as
accidentally parsing input bytes multiple times. In order to meet these
goals, untrusted.rs is limited in functionality such that it works best for
input languages with a small fixed amount of lookahead such as ASN.1, TLS,
TCP/IP, and many other networking, IPC, and related protocols. Languages
that require more lookahead and/or backtracking require some significant
contortions to parse using this framework. It would not be realistic to use
it for parsing programming language code, for example.

The overall pattern for using untrusted.rs is:

1. Write a recursive-descent-style parser for the input language, where the
   input data is given as a `&mut untrusted::Reader` parameter to each
   function. Each function should have a return type of `Result<V, E>` for
   some value type `V` and some error type `E`, either or both of which may
   be `()`. Functions for parsing the lowest-level language constructs
   should be defined. Those lowest-level functions will parse their inputs
   using `::read_byte()`, `Reader::peek()`, and similar functions.
   Higher-level language constructs are then parsed by calling the
   lower-level functions in sequence.

2. Wrap the top-most functions of your recursive-descent parser in
   functions that take their input data as an `untrusted::Input`. The
   wrapper functions should call the `Input`'s `read_all` (or a variant
   thereof) method. The wrapper functions are the only ones that should be
   exposed outside the parser's module.

3. After receiving the input data to parse, wrap it in an `untrusted::Input`
   using `untrusted::Input::from()` as early as possible. Pass the
   `untrusted::Input` to the wrapper functions when they need to be parsed.

In general parsers built using `untrusted::Reader` do not need to explicitly
check for end-of-input unless they are parsing optional constructs, because
`Reader::read_byte()` will return `Err(EndOfInput)` on end-of-input.
Similarly, parsers using `untrusted::Reader` generally don't need to check
for extra junk at the end of the input as long as the parser's API uses the
pattern described above, as `read_all` and its variants automatically check
for trailing junk. `Reader::skip_to_end()` must be used when any remaining
unread input should be ignored without triggering an error.

untrusted.rs works best when all processing of the input data is done
through the `untrusted::Input` and `untrusted::Reader` types. In
particular, avoid trying to parse input data using functions that take
byte slices. However, when you need to access a part of the input data as
a slice to use a function that isn't written using untrusted.rs,
`Input::as_slice_less_safe()` can be used.

It is recommend to use `use untrusted;` and then `untrusted::Input`,
`untrusted::Reader`, etc., instead of using `use untrusted::*`. Qualifying
the names with `untrusted` helps remind the reader of the code that it is
dealing with *untrusted* input.

# Examples

[*ring*](https://github.com/briansmith/ring)'s parser for the subset of
ASN.1 DER it needs to understand,
[`ring::der`](https://github.com/briansmith/ring/blob/main/src/io/der.rs),
is built on top of untrusted.rs. *ring* also uses untrusted.rs to parse ECC
public keys, RSA PKCS#1 1.5 padding, and for all other parsing it does.

All of [webpki](https://github.com/briansmith/webpki)'s parsing of X.509
certificates (also ASN.1 DER) is done using untrusted.rs.

## Structs

### `Input<'a>`

```rust
struct Input<'a> {
    // [REDACTED: Private Fields]
}
```

A wrapper around `&'a [u8](#u8)
` that helps in writing panic-free code.

No methods of `Input` will ever panic.

Intentionally avoids implementing `PartialEq` and `Eq` to avoid implicit
non-constant-time comparisons.

#### Implementations

- `const fn from(bytes: &'a [u8]) -> Self`
  Construct a new `Input` for the given input `bytes`.

- `fn is_empty(self: &Self) -> bool`
  Returns `true` if the input is empty and false otherwise.

- `fn len(self: &Self) -> usize`
  Returns the length of the `Input`.

- `fn read_all<F, R, E>(self: &Self, incomplete_read: E, read: F) -> Result<R, E>`
  Calls `read` with the given input as a `Reader`, ensuring that `read`

- `fn as_slice_less_safe(self: &Self) -> &'a [u8]`
  Access the input as a slice so it can be processed by functions that

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'a>`

- `fn from(value: &'a [u8]) -> Self`

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

- `fn clone(self: &Self) -> Input<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'a>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `EndOfInput`

```rust
struct EndOfInput;
```

The error type used to indicate the end of the input was reached before the
operation could be completed.

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

- `fn clone(self: &Self) -> EndOfInput`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &EndOfInput) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Reader<'a>`

```rust
struct Reader<'a> {
    // [REDACTED: Private Fields]
}
```

A read-only, forward-only cursor into the data in an `Input`.

Using `Reader` to parse input helps to ensure that no byte of the input
will be accidentally processed more than once. Using `Reader` in
conjunction with `read_all` and `read_all_optional` helps ensure that no
byte of the input is accidentally left unprocessed. The methods of `Reader`
never panic, so `Reader` also assists the writing of panic-free code.

Intentionally avoids implementing `PartialEq` and `Eq` to avoid implicit
non-constant-time comparisons.

#### Implementations

- `fn new(input: Input<'a>) -> Self`
  Construct a new Reader for the given input. Use `read_all` or

- `fn at_end(self: &Self) -> bool`
  Returns `true` if the reader is at the end of the input, and `false`

- `fn peek(self: &Self, b: u8) -> bool`
  Returns `true` if there is at least one more byte in the input and that

- `fn read_byte(self: &mut Self) -> Result<u8, EndOfInput>`
  Reads the next input byte.

- `fn read_bytes(self: &mut Self, num_bytes: usize) -> Result<Input<'a>, EndOfInput>`
  Skips `num_bytes` of the input, returning the skipped input as an

- `fn read_bytes_to_end(self: &mut Self) -> Input<'a>`
  Skips the reader to the end of the input, returning the skipped input

- `fn read_partial<F, R, E>(self: &mut Self, read: F) -> Result<(Input<'a>, R), E>`
  Calls `read()` with the given input as a `Reader`. On success, returns a

- `fn skip(self: &mut Self, num_bytes: usize) -> Result<(), EndOfInput>`
  Skips `num_bytes` of the input.

- `fn skip_to_end(self: &mut Self)`
  Skips the reader to the end of the input.

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

## Functions

### `read_all_optional`

```rust
fn read_all_optional<'a, F, R, E>(input: Option<Input<'a>>, incomplete_read: E, read: F) -> Result<R, E>
where
    F: FnOnce(Option<&mut Reader<'a>>) -> Result<R, E>
```

Calls `read` with the given input as a `Reader`, ensuring that `read`
consumed the entire input. When `input` is `None`, `read` will be
called with `None`.

