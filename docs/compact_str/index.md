# Crate `compact_str`

<div align="center">
  <h1><code>compact_str</code></h1>
  <p><strong>A memory efficient string type that can store up to 24* bytes on the stack.</strong></p>

  <a href="https://crates.io/crates/compact_str">
    <img alt="version on crates.io" src="https://img.shields.io/crates/v/compact_str"/>
  </a>
  <img alt="Minimum supported Rust Version: 1.60" src="https://img.shields.io/badge/MSRV-1.60-blueviolet">
  <a href="LICENSE">
    <img alt="mit license" src="https://img.shields.io/crates/l/compact_str"/>
  </a>

   <br />

  <a href="https://github.com/ParkMyCar/compact_str/actions/workflows/ci.yml">
    <img alt="Continuous Integration Status" src="https://github.com/ParkMyCar/compact_str/actions/workflows/ci.yml/badge.svg?branch=main&event=push"/>
  </a>
  <a href="https://github.com/ParkMyCar/compact_str/actions/workflows/cross_platform.yml">
    <img alt="Cross Platform Status" src="https://github.com/ParkMyCar/compact_str/actions/workflows/cross_platform.yml/badge.svg?branch=main&event=push"/>
  </a>
  <a href="https://github.com/ParkMyCar/compact_str/actions/workflows/msrv.yml">
    <img alt="Minimum Supported Rust Version Status" src="https://github.com/ParkMyCar/compact_str/actions/workflows/msrv.yml/badge.svg?branch=main&event=push"/>
  </a>
  <a href="https://github.com/ParkMyCar/compact_str/actions/workflows/clippy.yml">
    <img alt="Clippy Status" src="https://github.com/ParkMyCar/compact_str/actions/workflows/clippy.yml/badge.svg?branch=main&event=push"/>
  </a>

  <p  align=right><sub>* 12 bytes for 32-bit architectures</sub></p>
</div>

<br />

### About
A `CompactString` is a more memory efficient string type, that can store smaller strings on the stack, and transparently stores longer strings on the heap (aka a small string optimization).
It can mostly be used as a drop in replacement for `String` and are particularly useful in parsing, deserializing, or any other application where you may
have smaller strings.

### Properties
A `CompactString` specifically has the following properties:
  * `size_of::<CompactString>() == size_of::<String>()`
  * Stores up to 24 bytes on the stack
    * 12 bytes if running on a 32 bit architecture
  * Strings longer than 24 bytes are stored on the heap
  * `Clone` is `O(n)`
  * `From<String>` or `From<Box<str>>` re-uses underlying buffer
    * Eagerly inlines small strings
  * `O(1)` creation from `&'static str` with `CompactString::const_new`
  * Heap based string grows at a rate of 1.5x
    * The std library `String` grows at a rate of 2x
  * Space optimized for `Option<_>`
    * `size_of::<CompactString>() == size_of::<Option<CompactString>>()`
  * Uses [branchless instructions](https://en.algorithmica.org/hpc/pipelining/branchless/) for string accesses
  * Supports `no_std` environments

### Traits
This crate exposes two traits, `ToCompactString` and `CompactStringExt`.

#### `ToCompactString`
Provides the `to_compact_string(&self)` method for converting types into a `CompactString`. This trait is automatically implemented for all types that are `std::fmt::Display`, with specialized higher performance impls for:
* `u8`, `u16`, `u32`, `u64`, `usize`, `u128`
* `i8`, `i16`, `i32`, `i64`, `isize`, `i128`
* `f32`, `f64`
* `bool`, `char`
* `NonZeroU*`, `NonZeroI*`
* `String`, `CompactString`

#### `CompactStringExt`
Provides two methods `join_compact(seperator: impl AsRef<str>)` and `concat_compact()`. This trait is automatically implemented for all types that can be converted into an iterator and yield types that `impl AsRef<str>`. This allows you to join Vec's, slices, and any other collection to form `CompactString`s.

### Macros
This crate exposes one macro `format_compact!` that can be used to create `CompactString`s from arguments, like you can `String`s with the `std::format!` macro.

### Features
`compact_str` has the following optional features:
* `serde`, which implements [`Deserialize`](https://docs.rs/serde/1/serde/trait.Deserialize.html) and [`Serialize`](https://docs.rs/serde/1/serde/trait.Serialize.html) from the popular [`serde`](https://docs.rs/serde/1/serde/) crate, for `CompactString`
* `bytes`, which provides two methods `from_utf8_buf<B: Buf>(buf: &mut B)` and `from_utf8_buf_unchecked<B: Buf>(buf: &mut B)`, which allows for the creation of a `CompactString` from a [`bytes::Buf`](https://docs.rs/bytes/1/bytes/trait.Buf.html)
* `markup`, which implements [`Render`](https://docs.rs/markup/0.13/markup/trait.Render.html) trait, so `CompactString`s can be used in templates as HTML escaped strings
* `diesel`, which allows using CompactStrings in [`diesel`](https://diesel.rs/) text columns
* `sqlx-mysql` / `sqlx-postgres` / `sqlx-sqlite`, which allows using CompactStrings in [`sqlx`](https://github.com/launchbadge/sqlx) text columns
* `arbitrary`, which implements the [`arbitrary::Arbitrary`](https://docs.rs/arbitrary/1/arbitrary/trait.Arbitrary.html) trait for fuzzing
* `proptest`, which implements the [`proptest::arbitrary::Arbitrary`](https://docs.rs/proptest/1/proptest/arbitrary/trait.Arbitrary.html) trait for fuzzing
* `quickcheck`, which implements the [`quickcheck::Arbitrary`](https://docs.rs/quickcheck/1/quickcheck/trait.Arbitrary.html) trait for fuzzing
* `rkyv`, which implements [`rkyv::Archive`](https://docs.rs/rkyv/0.7/rkyv/trait.Archive.html), [`rkyv::Serialize`](https://docs.rs/rkyv/0.7/rkyv/trait.Serialize.html) and [`rkyv::Deserialize`](https://docs.rs/rkyv/0.7/rkyv/trait.Deserialize.html) for fast zero-copy serialization, interchangable with serialized Strings
* `smallvec`, provides the `into_bytes()` method which enables you to convert a `CompactString` into a byte vector, using [`smallvec::SmallVec`](https://docs.rs/smallvec/latest/smallvec/struct.SmallVec.html)

### How it works
Note: this explanation assumes a 64-bit architecture, for 32-bit architectures generally divide any number by 2.

Normally strings are stored on the heap since they're dynamically sized. In Rust a `String` consists of three fields, each of which are the size of a `usize`.
e.g. its layout is something like the following:

`String: [ ptr<8> | len<8> | cap<8> ]`
1. `ptr` is a pointer to a location on the heap that stores the string
2. `len` is the length of the string
3. `cap` is the total capacity of the buffer being pointed to

This results in 24 bytes being stored on the stack, 8 bytes for each field. Then the actual string is stored on the heap, usually with additional memory allocated to prevent re-allocating if the string is mutated.

The idea of `CompactString` is instead of storing metadata on the stack, just store the string itself. This way for smaller strings we save a bit of memory, and we
don't have to heap allocate so it's more performant. A `CompactString` is limited to 24 bytes (aka `size_of::<String>()`) so it won't ever use more memory than a
`String` would.

The memory layout of a `CompactString` looks something like:

`CompactString: [ buffer<23> | len<1> ]`

#### Memory Layout
Internally a `CompactString` has two variants:

1. **Inline**, a string <= 24 bytes long
2. **Heap** allocated, a string > 24 bytes long

We define a discriminant (aka track which variant we are) *within* the last byte, specifically:

1. `0b11111110` - All 1s with a trailing 0, indicates **heap** allocated
2. `0b11XXXXXX` - Two leading 1s, indicates **inline**, with the trailing 6 bits used to store the length

and the overall memory layout of a `CompactString` is:

1. `heap:   { ptr: NonNull<u8>, len: usize, cap: Capacity }`
2. `inline: { buffer: [u8; 24] }`

<sub>Both variants are 24 bytes long</sub>

For **heap** allocated strings we use a custom `HeapBuffer` which normally stores the capacity of the string on the stack, but also optionally allows us to store it on the heap. Since we use the last byte to track our discriminant, we only have 7 bytes to store the capacity, or 3 bytes on a 32-bit architecture. 7 bytes allows us to store a value up to `2^56`, aka 64 petabytes, while 3 bytes only allows us to store a value up to `2^24`, aka 16 megabytes.

For 64-bit architectures we always inline the capacity, because we can safely assume our strings will never be larger than 64 petabytes, but on 32-bit architectures, when creating or growing a `CompactString`, if the text is larger than 16MB then we move the capacity onto the heap.

We handle the capacity in this way for two reasons:
1. Users shouldn't have to pay for what they don't use. Meaning, in the _majority_ of cases the capacity of the buffer could easily fit into 7 or 3 bytes, so the user shouldn't have to pay the memory cost of storing the capacity on the heap, if they don't need to.
2. Allows us to convert `From<String>` in `O(1)` time, by taking the parts of a `String` (e.g. `ptr`, `len`, and `cap`) and using those to create a `CompactString`, without having to do any heap allocations. This is important when using `CompactString` in large codebases where you might have `CompactString` working alongside of `String`.

For **inline** strings we only have a 24 byte buffer on the stack. This might make you wonder how can we store a 24 byte long string, inline? Don't we also need to store the length somewhere?

To do this, we utilize the fact that the last byte of our string could only ever have a value in the range `[0, 192)`. We know this because all strings in Rust are valid [UTF-8](https://en.wikipedia.org/wiki/UTF-8), and the only valid byte pattern for the last byte of a UTF-8 character (and thus the possible last byte of a string) is `0b0XXXXXXX` aka `[0, 128)` or `0b10XXXXXX` aka `[128, 192)`. This leaves all values in `[192, 255]` as unused in our last byte. Therefore, we can use values in the range of `[192, 215]` to represent a length in the range of `[0, 23]`, and if our last byte has a value `< 192`, we know that's a UTF-8 character, and can interpret the length of our string as `24`.

Specifically, the last byte on the stack for a `CompactString` has the following uses:
* `[0, 191]` - Is the last byte of a UTF-8 char, the `CompactString` is stored on the stack and implicitly has a length of `24`
* `[192, 215]` - Denotes a length in the range of `[0, 23]`, this `CompactString` is stored on the stack.
* `216` - Denotes this `CompactString` is stored on the heap
* `217` - Denotes this `CompactString` stores a `&'static str`.
* `[218, 255]` - Unused, denotes e.g. the `None` variant for `Option<CompactString>`

### Testing
Strings and unicode can be quite messy, even further, we're working with things at the bit level. `compact_str` has an _extensive_ test suite comprised of unit testing, property testing, and fuzz testing, to ensure our invariants are upheld. We test across all major OSes (Windows, macOS, and Linux), architectures (64-bit and 32-bit), and endian-ness (big endian and little endian).

Fuzz testing is run with `libFuzzer`, `AFL++`, *and* `honggfuzz`, with `AFL++` running on both `x86_64` and `ARMv7` architectures. We test with [`miri`](https://github.com/rust-lang/miri) to catch cases of undefined behavior, and run all tests on every Rust compiler since `v1.60` to ensure support for our minimum supported Rust version (MSRV).

### `unsafe` code
`CompactString` uses a bit of unsafe code because we manually define what variant we are, so unlike an enum, the compiler can't guarantee what value is actually stored.
We also have some manually implemented heap data structures, i.e. `HeapBuffer`, and mess with bytes at a bit level, to make the most out of our resources.
That being said, uses of unsafe code in this library are constrained to only where *absolutely* necessary, and always documented with
`// SAFETY: <reason>`.

### Similar Crates
Storing strings on the stack is not a new idea, in fact there are a few other crates in the Rust ecosystem that do similar things, an incomplete list:

*  [`arcstr`](https://crates.io/crates/arcstr)
*  [`byteyarn`](https://crates.io/crates/byteyarn)
*  [`ecow`](https://crates.io/crates/ecow)
*  [`flexstr`](https://crates.io/crates/flexstr)
*  [`hipstr`](https://crates.io/crates/hipstr)
*  [`imstr`](https://crates.io/crates/imstr)
*  [`kstring`](https://crates.io/crates/kstring)
*  [`smartstring`](https://crates.io/crates/smartstring)

For a comparison of all these crates (and possibly more!) please see the [Rust String Benchmarks](https://github.com/rosetta-rs/string-rosetta-rs).

<br />
Thanks for readingme!

## Structs

### `CompactString`

```rust
struct CompactString(repr::Repr);
```

A [`CompactString`](#compactstring) is a compact string type that can be used almost anywhere a
[`String`](#string) or [`str`](#str) can be used.

## Using `CompactString`
```rust
use compact_str::CompactString;
use std::collections::HashMap;

// CompactString auto derefs into a str so you can use all methods from `str`
// that take a `&self`
if CompactString::new("hello world!").is_ascii() {
    println!("we're all ASCII")
}

// You can use a CompactString in collections like you would a String or &str
let mut map: HashMap<CompactString, CompactString> = HashMap::new();

// directly construct a new `CompactString`
map.insert(CompactString::new("nyc"), CompactString::new("empire state building"));
// create a `CompactString` from a `&str`
map.insert("sf".into(), "transamerica pyramid".into());
// create a `CompactString` from a `String`
map.insert(String::from("sea").into(), String::from("space needle").into());

fn wrapped_print<T: AsRef<str>>(text: T) {
    println!("{}", text.as_ref());
}

// CompactString impls AsRef<str> and Borrow<str>, so it can be used anywhere
// that expects a generic string
if let Some(building) = map.get("nyc") {
    wrapped_print(building);
}

// CompactString can also be directly compared to a String or &str
assert_eq!(CompactString::new("chicago"), "chicago");
assert_eq!(CompactString::new("houston"), String::from("houston"));
```

# Converting from a `String`
It's important that a `CompactString` interops well with `String`, so you can easily use both in
your code base.

`CompactString` implements `From<String>` and operates in the following manner:
- Eagerly inlines the string, possibly dropping excess capacity
- Otherwise re-uses the same underlying buffer from `String`

```rust
use compact_str::CompactString;

// eagerly inlining
let short = String::from("hello world");
let short_c = CompactString::from(short);
assert!(!short_c.is_heap_allocated());

// dropping excess capacity
let mut excess = String::with_capacity(256);
excess.push_str("abc");

let excess_c = CompactString::from(excess);
assert!(!excess_c.is_heap_allocated());
assert!(excess_c.capacity() < 256);

// re-using the same buffer
let long = String::from("this is a longer string that will be heap allocated");

let long_ptr = long.as_ptr();
let long_len = long.len();
let long_cap = long.capacity();

let mut long_c = CompactString::from(long);
assert!(long_c.is_heap_allocated());

let cpt_ptr = long_c.as_ptr();
let cpt_len = long_c.len();
let cpt_cap = long_c.capacity();

// the original String and the CompactString point to the same place in memory, buffer re-use!
assert_eq!(cpt_ptr, long_ptr);
assert_eq!(cpt_len, long_len);
assert_eq!(cpt_cap, long_cap);
```

### Prevent Eagerly Inlining
A consequence of eagerly inlining is you then need to de-allocate the existing buffer, which
might not always be desirable if you're converting a very large amount of `String`s. If your
code is very sensitive to allocations, consider the `CompactString::from_string_buffer` API.

#### Implementations

- `fn new<T: AsRef<str>>(text: T) -> Self`

- `fn try_new<T: AsRef<str>>(text: T) -> Result<Self, ReserveError>` — [`ReserveError`](../index.md)

- `const fn const_new(text: &'static str) -> Self`

- `const fn as_static_str(self: &Self) -> Option<&'static str>`

- `fn with_capacity(capacity: usize) -> Self`

- `fn try_with_capacity(capacity: usize) -> Result<Self, ReserveError>` — [`ReserveError`](../index.md)

- `fn from_utf8<B: AsRef<[u8]>>(buf: B) -> Result<Self, Utf8Error>`

- `unsafe fn from_utf8_unchecked<B: AsRef<[u8]>>(buf: B) -> Self`

- `fn from_utf16<B: AsRef<[u16]>>(buf: B) -> Result<Self, Utf16Error>` — [`Utf16Error`](../index.md)

- `fn from_utf16_lossy<B: AsRef<[u16]>>(buf: B) -> Self`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn capacity(self: &Self) -> usize`

- `fn reserve(self: &mut Self, additional: usize)`

- `fn try_reserve(self: &mut Self, additional: usize) -> Result<(), ReserveError>` — [`ReserveError`](../index.md)

- `fn as_str(self: &Self) -> &str`

- `fn as_mut_str(self: &mut Self) -> &mut str`

- `unsafe fn spare_capacity_mut(self: &mut Self) -> &mut [mem::MaybeUninit<u8>]`

- `fn as_bytes(self: &Self) -> &[u8]`

- `unsafe fn as_mut_bytes(self: &mut Self) -> &mut [u8]`

- `fn push(self: &mut Self, ch: char)`

- `fn pop(self: &mut Self) -> Option<char>`

- `fn push_str(self: &mut Self, s: &str)`

- `fn remove(self: &mut Self, idx: usize) -> char`

- `unsafe fn set_len(self: &mut Self, new_len: usize)`

- `fn is_heap_allocated(self: &Self) -> bool`

- `fn ensure_range(self: &Self, range: impl RangeBounds<usize>) -> (usize, usize)`

- `fn replace_range(self: &mut Self, range: impl RangeBounds<usize>, replace_with: &str)`

- `unsafe fn replace_range_same_size(self: &mut Self, start: usize, end: usize, replace_with: &str)`

- `unsafe fn replace_range_shrink(self: &mut Self, start: usize, end: usize, replace_with: &str)`

- `unsafe fn replace_range_grow(self: &mut Self, start: usize, end: usize, replace_with: &str)`

- `fn repeat(self: &Self, n: usize) -> Self`

- `fn truncate(self: &mut Self, new_len: usize)`

- `fn as_ptr(self: &Self) -> *const u8`

- `fn as_mut_ptr(self: &mut Self) -> *mut u8`

- `fn insert_str(self: &mut Self, idx: usize, string: &str)`

- `fn insert(self: &mut Self, idx: usize, ch: char)`

- `fn clear(self: &mut Self)`

- `fn split_off(self: &mut Self, at: usize) -> Self`

- `fn drain(self: &mut Self, range: impl RangeBounds<usize>) -> Drain<'_>` — [`Drain`](../index.md)

- `fn shrink_to(self: &mut Self, min_capacity: usize)`

- `fn shrink_to_fit(self: &mut Self)`

- `fn retain(self: &mut Self, predicate: impl FnMut(char) -> bool)`

- `fn from_utf8_lossy(v: &[u8]) -> Self`

- `fn from_utf16x(v: &[u8], from_int: impl Fn(u16) -> u16, from_bytes: impl Fn([u8; 2]) -> u16) -> Result<Self, Utf16Error>` — [`Utf16Error`](../index.md)

- `fn from_utf16x_lossy(v: &[u8], from_int: impl Fn(u16) -> u16, from_bytes: impl Fn([u8; 2]) -> u16) -> Self`

- `fn from_utf16le(v: impl AsRef<[u8]>) -> Result<Self, Utf16Error>` — [`Utf16Error`](../index.md)

- `fn from_utf16be(v: impl AsRef<[u8]>) -> Result<Self, Utf16Error>` — [`Utf16Error`](../index.md)

- `fn from_utf16le_lossy(v: impl AsRef<[u8]>) -> Self`

- `fn from_utf16be_lossy(v: impl AsRef<[u8]>) -> Self`

- `fn into_string(self: Self) -> String`

- `fn from_string_buffer(s: String) -> Self`

- `fn to_ascii_lowercase(self: &Self) -> Self`

- `fn to_ascii_uppercase(self: &Self) -> Self`

- `fn to_lowercase(self: &Self) -> Self`

- `fn from_str_to_lowercase(input: &str) -> Self`

- `fn to_uppercase(self: &Self) -> Self`

- `fn from_str_to_uppercase(input: &str) -> Self`

#### Trait Implementations

##### `impl Add`

- `type Output = CompactString`

- `fn add(self: Self, rhs: &str) -> <Self as >::Output`

##### `impl AddAssign`

- `fn add_assign(self: &mut Self, rhs: &str)`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &OsStr`

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default`

- `fn default() -> Self`

##### `impl Deref`

- `type Target = str`

- `fn deref(self: &Self) -> &str`

##### `impl DerefMut`

- `fn deref_mut(self: &mut Self) -> &mut str`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Extend`

- `fn extend<T: IntoIterator<Item = String>>(self: &mut Self, iter: T)`

##### `impl FromIterator`

- `fn from_iter<T: IntoIterator<Item = CompactString>>(iter: T) -> Self`

##### `impl FromStr`

- `type Err = Infallible`

- `fn from_str(s: &str) -> Result<CompactString, <Self as >::Err>` — [`CompactString`](../index.md)

##### `impl Hash`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl LifetimeFree`

##### `impl Ord`

- `fn cmp(self: &Self, other: &Self) -> Ordering`

##### `impl PartialEq<T: AsRef<str> + ?Sized>`

- `fn eq(self: &Self, other: &T) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering>`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl ToCompactString<T>`

- `fn try_to_compact_string(self: &Self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](../index.md), [`ToCompactStringError`](../index.md)

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl Write`

- `fn write_str(self: &mut Self, s: &str) -> fmt::Result`

- `fn write_fmt(self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result`

### `Utf16Error`

```rust
struct Utf16Error(());
```

A possible error value when converting a [`CompactString`](#compactstring) from a UTF-16 byte slice.

This type is the error type for the [`from_utf16`](#from-utf16) method on [`CompactString`](#compactstring).

# Examples

Basic usage:

```rust
use compact_str::CompactString;
// 𝄞mu<invalid>ic
let v = &[0xD834, 0xDD1E, 0x006d, 0x0075,
          0xD800, 0x0069, 0x0063];

assert!(CompactString::from_utf16(v).is_err());
```

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Utf16Error` — [`Utf16Error`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToCompactString<T>`

- `fn try_to_compact_string(self: &Self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](../index.md), [`ToCompactStringError`](../index.md)

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `Drain<'a>`

```rust
struct Drain<'a> {
    compact_string: *mut CompactString,
    start: usize,
    end: usize,
    chars: core::str::Chars<'a>,
}
```

An iterator over the exacted data by `CompactString::drain()`.

#### Implementations

- `fn as_str(self: &Self) -> &str`

#### Trait Implementations

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref`

- `type Target = str`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator`

- `fn next_back(self: &mut Self) -> Option<char>`

##### `impl Drop`

- `fn drop(self: &mut Self)`

##### `impl FusedIterator`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator`

- `type Item = char`

- `fn next(self: &mut Self) -> Option<char>`

- `fn count(self: Self) -> usize`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn last(self: Self) -> Option<char>`

##### `impl Receiver<P, T>`

- `type Target = T`

##### `impl Send`

##### `impl Sync`

##### `impl ToCompactString<T>`

- `fn try_to_compact_string(self: &Self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](../index.md), [`ToCompactStringError`](../index.md)

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

### `ReserveError`

```rust
struct ReserveError(());
```

A possible error value if allocating or resizing a [`CompactString`](#compactstring) failed.

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ReserveError` — [`ReserveError`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ReserveError) -> bool` — [`ReserveError`](../index.md)

##### `impl StructuralPartialEq`

##### `impl ToCompactString<T>`

- `fn try_to_compact_string(self: &Self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](../index.md), [`ToCompactStringError`](../index.md)

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

## Enums

### `ToCompactStringError`

```rust
enum ToCompactStringError {
    Reserve(ReserveError),
    Fmt(fmt::Error),
}
```

A possible error value if `ToCompactString::try_to_compact_string()` failed.

#### Variants

- **`Reserve`**

  Cannot allocate memory to hold CompactString

- **`Fmt`**

  `Display::fmt()` returned an error

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ToCompactStringError` — [`ToCompactStringError`](../index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error`

- `fn source(self: &Self) -> Option<&dyn std::error::Error>`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ToCompactStringError) -> bool` — [`ToCompactStringError`](../index.md)

##### `impl StructuralPartialEq`

##### `impl ToCompactString<T>`

- `fn try_to_compact_string(self: &Self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](../index.md), [`ToCompactStringError`](../index.md)

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

## Traits

## Macros

### `format_compact!`

Creates a `CompactString` using interpolation of runtime expressions.

The first argument `format_compact!` receives is a format string.
This must be a string literal.
The power of the formatting string is in the `{}`s contained.

Additional parameters passed to `format_compact!` replace the `{}`s within
the formatting string in the order given unless named or
positional parameters are used; see `std::fmt` for more information.

A common use for `format_compact!` is concatenation and interpolation
of strings.
The same convention is used with [`print!`](#print) and [`write!`](#write) macros,
depending on the intended destination of the string.

To convert a single value to a string, use the
`ToCompactString::to_compact_string` method, which uses
the `std::fmt::Display` formatting trait.

# Panics

`format_compact!` panics if a formatting trait implementation returns
an error.

This indicates an incorrect implementation since
`ToCompactString::to_compact_string` never returns an error itself.

