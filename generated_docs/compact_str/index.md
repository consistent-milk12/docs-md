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

## Contents

- [Modules](#modules)
  - [`features`](#features)
  - [`macros`](#macros)
  - [`unicode_data`](#unicode-data)
  - [`repr`](#repr)
  - [`traits`](#traits)
- [Structs](#structs)
  - [`CompactString`](#compactstring)
  - [`Utf16Error`](#utf16error)
  - [`Drain`](#drain)
  - [`ReserveError`](#reserveerror)
- [Enums](#enums)
  - [`ToCompactStringError`](#tocompactstringerror)
- [Traits](#traits)
  - [`CompactStringExt`](#compactstringext)
  - [`ToCompactString`](#tocompactstring)
  - [`UnwrapWithMsg`](#unwrapwithmsg)
- [Functions](#functions)
  - [`convert_while_ascii`](#convert-while-ascii)
  - [`unwrap_with_msg_fail`](#unwrap-with-msg-fail)
- [Macros](#macros)
  - [`format_compact!`](#format-compact)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`features`](#features) | mod | A module that contains the implementations for optional features. |
| [`macros`](#macros) | mod |  |
| [`unicode_data`](#unicode-data) | mod | Adapted from <https://doc.rust-lang.org/nightly/src/core/unicode/unicode_data.rs.html> |
| [`repr`](#repr) | mod |  |
| [`traits`](#traits) | mod |  |
| [`CompactString`](#compactstring) | struct | A [`CompactString`] is a compact string type that can be used almost anywhere a [`String`] or [`str`] can be used. |
| [`Utf16Error`](#utf16error) | struct | A possible error value when converting a [`CompactString`] from a UTF-16 byte slice. |
| [`Drain`](#drain) | struct | An iterator over the exacted data by [`CompactString::drain()`]. |
| [`ReserveError`](#reserveerror) | struct | A possible error value if allocating or resizing a [`CompactString`] failed. |
| [`ToCompactStringError`](#tocompactstringerror) | enum | A possible error value if [`ToCompactString::try_to_compact_string()`] failed. |
| [`CompactStringExt`](#compactstringext) | trait |  |
| [`ToCompactString`](#tocompactstring) | trait |  |
| [`UnwrapWithMsg`](#unwrapwithmsg) | trait |  |
| [`convert_while_ascii`](#convert-while-ascii) | fn | Converts the bytes while the bytes are still ascii. |
| [`unwrap_with_msg_fail`](#unwrap-with-msg-fail) | fn |  |
| [`format_compact!`](#format-compact) | macro | Creates a `CompactString` using interpolation of runtime expressions. |

## Modules

- [`features`](features/index.md) — A module that contains the implementations for optional features. For example `serde` support
- [`macros`](macros/index.md)
- [`unicode_data`](unicode_data/index.md) — Adapted from
- [`repr`](repr/index.md)
- [`traits`](traits/index.md)

## Structs

### `CompactString`

```rust
struct CompactString(repr::Repr);
```

*Defined in [`compact_str-0.9.0/src/lib.rs:128`](../../.source_1765521767/compact_str-0.9.0/src/lib.rs#L128)*

A [`CompactString`](#compactstring) is a compact string type that can be used almost anywhere a
[`String`](../cargo_platform/index.md) or [`str`](../clap_builder/builder/str/index.md) can be used.

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

- <span id="compactstring-new"></span>`fn new<T: AsRef<str>>(text: T) -> Self`

- <span id="compactstring-try-new"></span>`fn try_new<T: AsRef<str>>(text: T) -> Result<Self, ReserveError>` — [`ReserveError`](#reserveerror)

- <span id="compactstring-const-new"></span>`const fn const_new(text: &'static str) -> Self`

- <span id="compactstring-as-static-str"></span>`const fn as_static_str(&self) -> Option<&'static str>`

- <span id="compactstring-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

- <span id="compactstring-try-with-capacity"></span>`fn try_with_capacity(capacity: usize) -> Result<Self, ReserveError>` — [`ReserveError`](#reserveerror)

- <span id="compactstring-from-utf8"></span>`fn from_utf8<B: AsRef<[u8]>>(buf: B) -> Result<Self, Utf8Error>`

- <span id="compactstring-from-utf8-unchecked"></span>`unsafe fn from_utf8_unchecked<B: AsRef<[u8]>>(buf: B) -> Self`

- <span id="compactstring-from-utf16"></span>`fn from_utf16<B: AsRef<[u16]>>(buf: B) -> Result<Self, Utf16Error>` — [`Utf16Error`](#utf16error)

- <span id="compactstring-from-utf16-lossy"></span>`fn from_utf16_lossy<B: AsRef<[u16]>>(buf: B) -> Self`

- <span id="compactstring-len"></span>`fn len(&self) -> usize`

- <span id="compactstring-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="compactstring-capacity"></span>`fn capacity(&self) -> usize`

- <span id="compactstring-reserve"></span>`fn reserve(&mut self, additional: usize)`

- <span id="compactstring-try-reserve"></span>`fn try_reserve(&mut self, additional: usize) -> Result<(), ReserveError>` — [`ReserveError`](#reserveerror)

- <span id="compactstring-as-str"></span>`fn as_str(&self) -> &str`

- <span id="compactstring-as-mut-str"></span>`fn as_mut_str(&mut self) -> &mut str`

- <span id="compactstring-spare-capacity-mut"></span>`unsafe fn spare_capacity_mut(&mut self) -> &mut [mem::MaybeUninit<u8>]`

- <span id="compactstring-as-bytes"></span>`fn as_bytes(&self) -> &[u8]`

- <span id="compactstring-as-mut-bytes"></span>`unsafe fn as_mut_bytes(&mut self) -> &mut [u8]`

- <span id="compactstring-push"></span>`fn push(&mut self, ch: char)`

- <span id="compactstring-pop"></span>`fn pop(&mut self) -> Option<char>`

- <span id="compactstring-push-str"></span>`fn push_str(&mut self, s: &str)`

- <span id="compactstring-remove"></span>`fn remove(&mut self, idx: usize) -> char`

- <span id="compactstring-set-len"></span>`unsafe fn set_len(&mut self, new_len: usize)`

- <span id="compactstring-is-heap-allocated"></span>`fn is_heap_allocated(&self) -> bool`

- <span id="compactstring-ensure-range"></span>`fn ensure_range(&self, range: impl RangeBounds<usize>) -> (usize, usize)`

- <span id="compactstring-replace-range"></span>`fn replace_range(&mut self, range: impl RangeBounds<usize>, replace_with: &str)`

- <span id="compactstring-replace-range-same-size"></span>`unsafe fn replace_range_same_size(&mut self, start: usize, end: usize, replace_with: &str)`

- <span id="compactstring-replace-range-shrink"></span>`unsafe fn replace_range_shrink(&mut self, start: usize, end: usize, replace_with: &str)`

- <span id="compactstring-replace-range-grow"></span>`unsafe fn replace_range_grow(&mut self, start: usize, end: usize, replace_with: &str)`

- <span id="compactstring-repeat"></span>`fn repeat(&self, n: usize) -> Self`

- <span id="compactstring-truncate"></span>`fn truncate(&mut self, new_len: usize)`

- <span id="compactstring-as-ptr"></span>`fn as_ptr(&self) -> *const u8`

- <span id="compactstring-as-mut-ptr"></span>`fn as_mut_ptr(&mut self) -> *mut u8`

- <span id="compactstring-insert-str"></span>`fn insert_str(&mut self, idx: usize, string: &str)`

- <span id="compactstring-insert"></span>`fn insert(&mut self, idx: usize, ch: char)`

- <span id="compactstring-clear"></span>`fn clear(&mut self)`

- <span id="compactstring-split-off"></span>`fn split_off(&mut self, at: usize) -> Self`

- <span id="compactstring-drain"></span>`fn drain(&mut self, range: impl RangeBounds<usize>) -> Drain<'_>` — [`Drain`](#drain)

- <span id="compactstring-shrink-to"></span>`fn shrink_to(&mut self, min_capacity: usize)`

- <span id="compactstring-shrink-to-fit"></span>`fn shrink_to_fit(&mut self)`

- <span id="compactstring-retain"></span>`fn retain(&mut self, predicate: impl FnMut(char) -> bool)`

- <span id="compactstring-from-utf8-lossy"></span>`fn from_utf8_lossy(v: &[u8]) -> Self`

- <span id="compactstring-from-utf16x"></span>`fn from_utf16x(v: &[u8], from_int: impl Fn(u16) -> u16, from_bytes: impl Fn([u8; 2]) -> u16) -> Result<Self, Utf16Error>` — [`Utf16Error`](#utf16error)

- <span id="compactstring-from-utf16x-lossy"></span>`fn from_utf16x_lossy(v: &[u8], from_int: impl Fn(u16) -> u16, from_bytes: impl Fn([u8; 2]) -> u16) -> Self`

- <span id="compactstring-from-utf16le"></span>`fn from_utf16le(v: impl AsRef<[u8]>) -> Result<Self, Utf16Error>` — [`Utf16Error`](#utf16error)

- <span id="compactstring-from-utf16be"></span>`fn from_utf16be(v: impl AsRef<[u8]>) -> Result<Self, Utf16Error>` — [`Utf16Error`](#utf16error)

- <span id="compactstring-from-utf16le-lossy"></span>`fn from_utf16le_lossy(v: impl AsRef<[u8]>) -> Self`

- <span id="compactstring-from-utf16be-lossy"></span>`fn from_utf16be_lossy(v: impl AsRef<[u8]>) -> Self`

- <span id="compactstring-into-string"></span>`fn into_string(self) -> String`

- <span id="compactstring-from-string-buffer"></span>`fn from_string_buffer(s: String) -> Self`

- <span id="compactstring-to-ascii-lowercase"></span>`fn to_ascii_lowercase(&self) -> Self`

- <span id="compactstring-to-ascii-uppercase"></span>`fn to_ascii_uppercase(&self) -> Self`

- <span id="compactstring-to-lowercase"></span>`fn to_lowercase(&self) -> Self`

- <span id="compactstring-from-str-to-lowercase"></span>`fn from_str_to_lowercase(input: &str) -> Self`

- <span id="compactstring-to-uppercase"></span>`fn to_uppercase(&self) -> Self`

- <span id="compactstring-from-str-to-uppercase"></span>`fn from_str_to_uppercase(input: &str) -> Self`

#### Trait Implementations

##### `impl Add for CompactString`

- <span id="compactstring-add-type-output"></span>`type Output = CompactString`

- <span id="compactstring-add"></span>`fn add(self, rhs: &str) -> <Self as >::Output`

##### `impl AddAssign for CompactString`

- <span id="compactstring-add-assign"></span>`fn add_assign(&mut self, rhs: &str)`

##### `impl AsRef for CompactString`

- <span id="compactstring-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl Clone for CompactString`

- <span id="compactstring-clone"></span>`fn clone(&self) -> Self`

- <span id="compactstring-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl Debug for CompactString`

- <span id="compactstring-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for CompactString`

- <span id="compactstring-default"></span>`fn default() -> Self`

##### `impl Deref for CompactString`

- <span id="compactstring-deref-type-target"></span>`type Target = str`

- <span id="compactstring-deref"></span>`fn deref(&self) -> &str`

##### `impl DerefMut for CompactString`

- <span id="compactstring-deref-mut"></span>`fn deref_mut(&mut self) -> &mut str`

##### `impl Display for CompactString`

- <span id="compactstring-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompactString`

##### `impl Extend for CompactString`

- <span id="compactstring-extend"></span>`fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T)`

##### `impl FromIterator for super::Repr`

- <span id="superrepr-from-iter"></span>`fn from_iter<T: IntoIterator<Item = CompactString>>(iter: T) -> Self`

##### `impl FromStr for CompactString`

- <span id="compactstring-fromstr-type-err"></span>`type Err = Infallible`

- <span id="compactstring-from-str"></span>`fn from_str(s: &str) -> Result<CompactString, <Self as >::Err>` — [`CompactString`](#compactstring)

##### `impl Hash for CompactString`

- <span id="compactstring-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl LifetimeFree for crate::CompactString`

##### `impl Ord for CompactString`

- <span id="compactstring-cmp"></span>`fn cmp(&self, other: &Self) -> Ordering`

##### `impl<T: AsRef<str> + ?Sized> PartialEq for CompactString`

- <span id="compactstring-eq"></span>`fn eq(&self, other: &T) -> bool`

##### `impl PartialOrd for CompactString`

- <span id="compactstring-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<Ordering>`

##### `impl Receiver for CompactString`

- <span id="compactstring-receiver-type-target"></span>`type Target = T`

##### `impl ToCompactString for CompactString`

- <span id="compactstring-try-to-compact-string"></span>`fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](#compactstring), [`ToCompactStringError`](#tocompactstringerror)

##### `impl ToString for CompactString`

- <span id="compactstring-to-string"></span>`fn to_string(&self) -> String`

##### `impl Write for CompactString`

- <span id="compactstring-write-str"></span>`fn write_str(&mut self, s: &str) -> fmt::Result`

- <span id="compactstring-write-fmt"></span>`fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result`

### `Utf16Error`

```rust
struct Utf16Error(());
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2483`](../../.source_1765521767/compact_str-0.9.0/src/lib.rs#L2483)*

A possible error value when converting a [`CompactString`](#compactstring) from a UTF-16 byte slice.

This type is the error type for the `from_utf16` method on [`CompactString`](#compactstring).

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

##### `impl Clone for Utf16Error`

- <span id="utf16error-clone"></span>`fn clone(&self) -> Utf16Error` — [`Utf16Error`](#utf16error)

##### `impl Copy for Utf16Error`

##### `impl Debug for Utf16Error`

- <span id="utf16error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Utf16Error`

- <span id="utf16error-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToCompactString for Utf16Error`

- <span id="utf16error-try-to-compact-string"></span>`fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](#compactstring), [`ToCompactStringError`](#tocompactstringerror)

##### `impl ToString for Utf16Error`

- <span id="utf16error-to-string"></span>`fn to_string(&self) -> String`

### `Drain<'a>`

```rust
struct Drain<'a> {
    compact_string: *mut CompactString,
    start: usize,
    end: usize,
    chars: core::str::Chars<'a>,
}
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2493-2498`](../../.source_1765521767/compact_str-0.9.0/src/lib.rs#L2493-L2498)*

An iterator over the exacted data by `CompactString::drain()`.

#### Implementations

- <span id="drain-as-str"></span>`fn as_str(&self) -> &str`

#### Trait Implementations

##### `impl Debug for Drain<'_>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for Drain<'_>`

- <span id="drain-deref-type-target"></span>`type Target = str`

- <span id="drain-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl Display for Drain<'_>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Drain<'_>`

- <span id="drain-next-back"></span>`fn next_back(&mut self) -> Option<char>`

##### `impl Drop for Drain<'_>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl FusedIterator for Drain<'_>`

##### `impl IntoIterator for Drain<'a>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Drain<'_>`

- <span id="drain-iterator-type-item"></span>`type Item = char`

- <span id="drain-next"></span>`fn next(&mut self) -> Option<char>`

- <span id="drain-count"></span>`fn count(self) -> usize`

- <span id="drain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="drain-last"></span>`fn last(self) -> Option<char>`

##### `impl Receiver for Drain<'a>`

- <span id="drain-receiver-type-target"></span>`type Target = T`

##### `impl Send for Drain<'_>`

##### `impl Sync for Drain<'_>`

##### `impl ToCompactString for Drain<'a>`

- <span id="drain-try-to-compact-string"></span>`fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](#compactstring), [`ToCompactStringError`](#tocompactstringerror)

##### `impl ToString for Drain<'a>`

- <span id="drain-to-string"></span>`fn to_string(&self) -> String`

### `ReserveError`

```rust
struct ReserveError(());
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2579`](../../.source_1765521767/compact_str-0.9.0/src/lib.rs#L2579)*

A possible error value if allocating or resizing a [`CompactString`](#compactstring) failed.

#### Trait Implementations

##### `impl Clone for ReserveError`

- <span id="reserveerror-clone"></span>`fn clone(&self) -> ReserveError` — [`ReserveError`](#reserveerror)

##### `impl Copy for ReserveError`

##### `impl Debug for ReserveError`

- <span id="reserveerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ReserveError`

- <span id="reserveerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ReserveError`

##### `impl PartialEq for ReserveError`

- <span id="reserveerror-eq"></span>`fn eq(&self, other: &ReserveError) -> bool` — [`ReserveError`](#reserveerror)

##### `impl StructuralPartialEq for ReserveError`

##### `impl ToCompactString for ReserveError`

- <span id="reserveerror-try-to-compact-string"></span>`fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](#compactstring), [`ToCompactStringError`](#tocompactstringerror)

##### `impl ToString for ReserveError`

- <span id="reserveerror-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `ToCompactStringError`

```rust
enum ToCompactStringError {
    Reserve(ReserveError),
    Fmt(fmt::Error),
}
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2594-2599`](../../.source_1765521767/compact_str-0.9.0/src/lib.rs#L2594-L2599)*

A possible error value if `ToCompactString::try_to_compact_string()` failed.

#### Variants

- **`Reserve`**

  Cannot allocate memory to hold CompactString

- **`Fmt`**

  `Display::fmt()` returned an error

#### Trait Implementations

##### `impl Clone for ToCompactStringError`

- <span id="tocompactstringerror-clone"></span>`fn clone(&self) -> ToCompactStringError` — [`ToCompactStringError`](#tocompactstringerror)

##### `impl Copy for ToCompactStringError`

##### `impl Debug for ToCompactStringError`

- <span id="tocompactstringerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ToCompactStringError`

- <span id="tocompactstringerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ToCompactStringError`

- <span id="tocompactstringerror-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl PartialEq for ToCompactStringError`

- <span id="tocompactstringerror-eq"></span>`fn eq(&self, other: &ToCompactStringError) -> bool` — [`ToCompactStringError`](#tocompactstringerror)

##### `impl StructuralPartialEq for ToCompactStringError`

##### `impl ToCompactString for ToCompactStringError`

- <span id="tocompactstringerror-try-to-compact-string"></span>`fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>` — [`CompactString`](#compactstring), [`ToCompactStringError`](#tocompactstringerror)

##### `impl ToString for ToCompactStringError`

- <span id="tocompactstringerror-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `CompactStringExt`

```rust
trait CompactStringExt { ... }
```

*Defined in [`compact_str-0.9.0/src/traits.rs:142-169`](../../.source_1765521767/compact_str-0.9.0/src/traits.rs#L142-L169)*

A trait that provides convenience methods for creating a [`CompactString`](#compactstring) from a collection of
items. It is implemented for all types that can be converted into an iterator, and that iterator
yields types that can be converted into a `str`.

i.e. `C: IntoIterator<Item = AsRef<str>>`.

# Concatenate and Join
Two methods that this trait provides are `concat_compact(...)` and `join_compact(...)`
```rust
use compact_str::CompactStringExt;

let words = vec!["☀️", "🌕", "🌑", "☀️"];

// directly concatenate all the words together
let concat = words.iter().concat_compact();
assert_eq!(concat, "☀️🌕🌑☀️");

// join the words, with a separator
let join = words.iter().join_compact(" ➡️ ");
assert_eq!(join, "☀️ ➡️ 🌕 ➡️ 🌑 ➡️ ☀️");
```

#### Required Methods

- `fn concat_compact(self) -> CompactString`

  Concatenates all the items of a collection into a [`CompactString`](#compactstring)

- `fn join_compact<S: AsRef<str>>(self, separator: S) -> CompactString`

  Joins all the items of a collection, placing a separator between them, forming a

#### Implementors

- `C`

### `ToCompactString`

```rust
trait ToCompactString { ... }
```

*Defined in [`compact_str-0.9.0/src/traits.rs:16-49`](../../.source_1765521767/compact_str-0.9.0/src/traits.rs#L16-L49)*

A trait for converting a value to a `CompactString`.

This trait is automatically implemented for any type which implements the
[`fmt::Display`](../miette_derive/fmt/index.md) trait. As such, [`ToCompactString`](traits/index.md) shouldn't be implemented directly:
[`fmt::Display`](../miette_derive/fmt/index.md) should be implemented instead, and you get the [`ToCompactString`](traits/index.md)
implementation for free.

#### Required Methods

- `fn try_to_compact_string(&self) -> Result<CompactString, ToCompactStringError>`

  Fallible version of `ToCompactString::to_compact_string()`

#### Provided Methods

- `fn to_compact_string(&self) -> CompactString`

  Converts the given value to a [`CompactString`](#compactstring).

#### Implementors

- `T`

### `UnwrapWithMsg`

```rust
trait UnwrapWithMsg { ... }
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2635-2639`](../../.source_1765521767/compact_str-0.9.0/src/lib.rs#L2635-L2639)*

#### Associated Types

- `type T`

#### Required Methods

- `fn unwrap_with_msg(self) -> <Self as >::T`

#### Implementors

- `Result<T, E>`

## Functions

### `convert_while_ascii`

```rust
fn convert_while_ascii(b: &[u8], convert: fn(&u8) -> u8) -> CompactString
```

*Defined in [`compact_str-0.9.0/src/lib.rs:1907-1947`](../../.source_1765521767/compact_str-0.9.0/src/lib.rs#L1907-L1947)*

Converts the bytes while the bytes are still ascii.
For better average performance, this is happens in chunks of `2*size_of::<usize>()`.
Returns a vec with the converted bytes.

Copied from https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#623-666

### `unwrap_with_msg_fail`

```rust
fn unwrap_with_msg_fail<E: fmt::Display>(error: E) -> never
```

*Defined in [`compact_str-0.9.0/src/lib.rs:2657-2659`](../../.source_1765521767/compact_str-0.9.0/src/lib.rs#L2657-L2659)*

## Macros

### `format_compact!`

*Defined in [`compact_str-0.9.0/src/macros.rs:28-32`](../../.source_1765521767/compact_str-0.9.0/src/macros.rs#L28-L32)*

Creates a `CompactString` using interpolation of runtime expressions.

The first argument `format_compact!` receives is a format string.
This must be a string literal.
The power of the formatting string is in the `{}`s contained.

Additional parameters passed to `format_compact!` replace the `{}`s within
the formatting string in the order given unless named or
positional parameters are used; see [`std::fmt`](../anstream/fmt/index.md) for more information.

A common use for `format_compact!` is concatenation and interpolation
of strings.
The same convention is used with [`print!`](../backtrace/print/index.md) and [`write!`](../anstream/strip/index.md) macros,
depending on the intended destination of the string.

To convert a single value to a string, use the
`ToCompactString::to_compact_string` method, which uses
the [`std::fmt::Display`](../miette_derive/fmt/index.md) formatting trait.

# Panics

`format_compact!` panics if a formatting trait implementation returns
an error.

This indicates an incorrect implementation since
`ToCompactString::to_compact_string` never returns an error itself.

