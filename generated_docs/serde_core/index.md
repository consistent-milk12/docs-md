# Crate `serde_core`

Serde is a framework for ***ser***ializing and ***de***serializing Rust data
structures efficiently and generically.

The `serde_core` crate contains Serde's trait definitions with **no support
for #\[derive()\]**.

In crates that derive an implementation of `Serialize` or `Deserialize`, you
must depend on the `serde` crate, not `serde_core`.

In crates that handwrite implementations of Serde traits, or only use them
as trait bounds, depending on `serde_core` is permitted. But `serde`
re-exports all of these traits and can be used for this use case too. If in
doubt, disregard `serde_core` and always use `serde`.

Crates that depend on `serde_core` instead of `serde` are able to compile in
parallel with `serde_derive` even when `serde`'s "derive" feature is turned on,
as shown in the following build timings.

<br>

<table>
<tr><td align="center">When <code>serde_json</code> depends on <code>serde</code></td></tr>
<tr><td><img src="https://github.com/user-attachments/assets/78dc179c-6ab1-4059-928c-1474b0d9d0bb"></td></tr>
</table>

<br>

<table>
<tr><td align="center">When <code>serde_json</code> depends on <code>serde_core</code></td></tr>
<tr><td><img src="https://github.com/user-attachments/assets/6b6cff5e-3e45-4ac7-9db1-d99ee8b9f5f7"></td></tr>
</table>

## Contents

- [Modules](#modules)
  - [`crate_root`](#crate_root)
  - [`macros`](#macros)
  - [`lib`](#lib)
  - [`de`](#de)
  - [`ser`](#ser)
  - [`format`](#format)
- [Traits](#traits)
  - [`Deserialize`](#deserialize)
  - [`Deserializer`](#deserializer)
  - [`Serialize`](#serialize)
  - [`Serializer`](#serializer)
- [Macros](#macros)
  - [`tri!`](#tri)
  - [`forward_to_deserialize_any!`](#forward_to_deserialize_any)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`crate_root`](#crate_root) | mod |  |
| [`macros`](#macros) | mod |  |
| [`lib`](#lib) | mod | A facade around all the types we need from the `std`, `core`, and `alloc` crates. |
| [`de`](#de) | mod | Generic data structure deserialization framework. |
| [`ser`](#ser) | mod | Generic data structure serialization framework. |
| [`format`](#format) | mod |  |
| [`Deserialize`](#deserialize) | trait |  |
| [`Deserializer`](#deserializer) | trait |  |
| [`Serialize`](#serialize) | trait |  |
| [`Serializer`](#serializer) | trait |  |
| [`tri!`](#tri) | macro |  |
| [`forward_to_deserialize_any!`](#forward_to_deserialize_any) | macro | Helper macro when implementing the `Deserializer` part of a new data format for Serde. |

## Modules

- [`crate_root`](crate_root/index.md)
- [`macros`](macros/index.md)
- [`lib`](lib/index.md) — A facade around all the types we need from the `std`, `core`, and `alloc`
- [`de`](de/index.md) — Generic data structure deserialization framework.
- [`ser`](ser/index.md) — Generic data structure serialization framework.
- [`format`](format/index.md)

## Traits

### `Deserialize<'de>`

```rust
trait Deserialize<'de>: Sized { ... }
```

*Defined in [`serde_core-1.0.228/src/de/mod.rs:554-593`](../../.source_1765210505/serde_core-1.0.228/src/de/mod.rs#L554-L593)*

A **data structure** that can be deserialized from any data format supported
by Serde.

Serde provides `Deserialize` implementations for many Rust primitive and
standard library types. The complete list is `here`. All of these
can be deserialized using Serde out of the box.

Additionally, Serde provides a procedural macro called `serde_derive` to
automatically generate `Deserialize` implementations for structs and enums
in your program. See the [derive section of the manual][`derive`](../clap_builder/derive/index.md) for how to
use this.

In rare cases it may be necessary to implement `Deserialize` manually for
some type in your program. See the [Implementing
`Deserialize`][impl-deserialize] section of the manual for more about this.

Third-party crates may provide `Deserialize` implementations for types that
they expose. For example the `linked-hash-map` crate provides a
`LinkedHashMap<K, V>` type that is deserializable by Serde because the crate
provides an implementation of `Deserialize` for it.


# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by `Self` when deserialized. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.


#### Required Methods

- `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

  Deserialize this value from the given Serde deserializer.

#### Implementors

- [`AtomicBool`](lib/index.md#atomicbool)
- [`AtomicI16`](lib/index.md#atomici16)
- [`AtomicI32`](lib/index.md#atomici32)
- [`AtomicI64`](lib/index.md#atomici64)
- [`AtomicI8`](lib/index.md#atomici8)
- [`AtomicIsize`](lib/index.md#atomicisize)
- [`AtomicU16`](lib/index.md#atomicu16)
- [`AtomicU32`](lib/index.md#atomicu32)
- [`AtomicU64`](lib/index.md#atomicu64)
- [`AtomicU8`](lib/index.md#atomicu8)
- [`AtomicUsize`](lib/index.md#atomicusize)
- [`BTreeMap`](lib/index.md#btreemap)
- [`BTreeSet`](lib/index.md#btreeset)
- [`BinaryHeap`](lib/index.md#binaryheap)
- [`Bound`](lib/index.md#bound)
- [`Box`](lib/index.md#box)
- [`CString`](lib/index.md#cstring)
- [`Cell`](lib/index.md#cell)
- [`Cow`](lib/index.md#cow)
- [`Duration`](lib/index.md#duration)
- [`Field`](de/impls/range/index.md#field)
- [`Field`](de/impls/range_from/index.md#field)
- [`Field`](de/impls/range_to/index.md#field)
- [`HashMap`](lib/index.md#hashmap)
- [`HashSet`](lib/index.md#hashset)
- [`IgnoredAny`](de/ignored_any/index.md#ignoredany)
- [`LinkedList`](lib/index.md#linkedlist)
- [`Mutex`](lib/index.md#mutex)
- [`OsStringKind`](de/impls/index.md#osstringkind)
- [`OsString`](lib/index.md#osstring)
- [`PathBuf`](lib/index.md#pathbuf)
- [`PhantomData`](lib/index.md#phantomdata)
- [`RangeFrom`](lib/index.md#rangefrom)
- [`RangeInclusive`](lib/index.md#rangeinclusive)
- [`RangeTo`](lib/index.md#rangeto)
- [`Range`](lib/index.md#range)
- [`RefCell`](lib/index.md#refcell)
- [`Reverse`](lib/index.md#reverse)
- [`RwLock`](lib/index.md#rwlock)
- [`Saturating`](lib/index.md#saturating)
- [`String`](lib/index.md#string)
- [`SystemTime`](lib/index.md#systemtime)
- [`VecDeque`](lib/index.md#vecdeque)
- [`Vec`](lib/index.md#vec)
- [`Wrapping`](lib/index.md#wrapping)
- `&'a Path`
- `&'a [u8]`
- `&'a str`
- `()`
- `(T)`
- `(T0, T1)`
- `(T0, T1, T2)`
- `(T0, T1, T2, T3)`
- `(T0, T1, T2, T3, T4)`
- `(T0, T1, T2, T3, T4, T5)`
- `(T0, T1, T2, T3, T4, T5, T6)`
- `(T0, T1, T2, T3, T4, T5, T6, T7)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)`
- `Option<T>`
- `Result<T, E>`
- `[T; 0]`
- `[T; 10]`
- `[T; 11]`
- `[T; 12]`
- `[T; 13]`
- `[T; 14]`
- `[T; 15]`
- `[T; 16]`
- `[T; 17]`
- `[T; 18]`
- `[T; 19]`
- `[T; 1]`
- `[T; 20]`
- `[T; 21]`
- `[T; 22]`
- `[T; 23]`
- `[T; 24]`
- `[T; 25]`
- `[T; 26]`
- `[T; 27]`
- `[T; 28]`
- `[T; 29]`
- `[T; 2]`
- `[T; 30]`
- `[T; 31]`
- `[T; 32]`
- `[T; 3]`
- `[T; 4]`
- `[T; 5]`
- `[T; 6]`
- `[T; 7]`
- `[T; 8]`
- `[T; 9]`
- `bool`
- `char`
- `f32`
- `f64`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `net::IpAddr`
- `net::Ipv4Addr`
- `net::Ipv6Addr`
- `net::SocketAddrV4`
- `net::SocketAddrV6`
- `net::SocketAddr`
- `num::NonZeroI128`
- `num::NonZeroI16`
- `num::NonZeroI32`
- `num::NonZeroI64`
- `num::NonZeroI8`
- `num::NonZeroIsize`
- `num::NonZeroU128`
- `num::NonZeroU16`
- `num::NonZeroU32`
- `num::NonZeroU64`
- `num::NonZeroU8`
- `num::NonZeroUsize`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `Deserializer<'de>`

```rust
trait Deserializer<'de>: Sized { ... }
```

*Defined in [`serde_core-1.0.228/src/de/mod.rs:945-1266`](../../.source_1765210505/serde_core-1.0.228/src/de/mod.rs#L945-L1266)*

A **data format** that can deserialize any data structure supported by
Serde.

The role of this trait is to define the deserialization half of the [Serde
data model], which is a way to categorize every Rust data type into one of
29 possible types. Each method of the `Deserializer` trait corresponds to one
of the types of the data model.

Implementations of `Deserialize` map themselves into this data model by
passing to the `Deserializer` a `Visitor` implementation that can receive
these various types.

The types that make up the Serde data model are:

 - **14 primitive types**
   - bool
   - i8, i16, i32, i64, i128
   - u8, u16, u32, u64, u128
   - f32, f64
   - char
 - **string**
   - UTF-8 bytes with a length and no null terminator.
   - When serializing, all strings are handled equally. When deserializing,
     there are three flavors of strings: transient, owned, and borrowed.
 - **byte array** - \[u8\]
   - Similar to strings, during deserialization byte arrays can be
     transient, owned, or borrowed.
 - **option**
   - Either none or some value.
 - **unit**
   - The type of `()` in Rust. It represents an anonymous value containing
     no data.
 - **unit_struct**
   - For example `struct Unit` or `PhantomData<T>`. It represents a named
     value containing no data.
 - **unit_variant**
   - For example the `E::A` and `E::B` in `enum E { A, B }`.
 - **newtype_struct**
   - For example `struct Millimeters(u8)`.
 - **newtype_variant**
   - For example the `E::N` in `enum E { N(u8) }`.
 - **seq**
   - A variably sized heterogeneous sequence of values, for example `Vec<T>`
     or `HashSet<T>`. When serializing, the length may or may not be known
     before iterating through all the data. When deserializing, the length
     is determined by looking at the serialized data.
 - **tuple**
   - A statically sized heterogeneous sequence of values for which the
     length will be known at deserialization time without looking at the
     serialized data, for example `(u8,)` or `(String, u64, Vec<T>)` or
     `[u64; 10]`.
 - **tuple_struct**
   - A named tuple, for example `struct Rgb(u8, u8, u8)`.
 - **tuple_variant**
   - For example the `E::T` in `enum E { T(u8, u8) }`.
 - **map**
   - A heterogeneous key-value pairing, for example `BTreeMap<K, V>`.
 - **struct**
   - A heterogeneous key-value pairing in which the keys are strings and
     will be known at deserialization time without looking at the serialized
     data, for example `struct S { r: u8, g: u8, b: u8 }`.
 - **struct_variant**
   - For example the `E::S` in `enum E { S { r: u8, g: u8, b: u8 } }`.

The `Deserializer` trait supports two entry point styles which enables
different kinds of deserialization.

1. The `deserialize_any` method. Self-describing data formats like JSON are
   able to look at the serialized data and tell what it represents. For
   example the JSON deserializer may see an opening curly brace (`{`) and
   know that it is seeing a map. If the data format supports
   `Deserializer::deserialize_any`, it will drive the Visitor using whatever
   type it sees in the input. JSON uses this approach when deserializing
   `serde_json::Value` which is an enum that can represent any JSON
   document. Without knowing what is in a JSON document, we can deserialize
   it to `serde_json::Value` by going through
   `Deserializer::deserialize_any`.

2. The various `deserialize_*` methods. Non-self-describing formats like
   Postcard need to be told what is in the input in order to deserialize it.
   The `deserialize_*` methods are hints to the deserializer for how to
   interpret the next piece of input. Non-self-describing formats are not
   able to deserialize something like `serde_json::Value` which relies on
   `Deserializer::deserialize_any`.

When implementing `Deserialize`, you should avoid relying on
`Deserializer::deserialize_any` unless you need to be told by the
Deserializer what type is in the input. Know that relying on
`Deserializer::deserialize_any` means your data type will be able to
deserialize from self-describing formats only, ruling out Postcard and many
others.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed from the input when deserializing. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

# Example implementation

The [example data format] presented on the website contains example code for
a basic JSON `Deserializer`.


#### Associated Types

- `type Error: 1`

#### Required Methods

- `fn deserialize_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Require the `Deserializer` to figure out how to drive the visitor based

- `fn deserialize_bool<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `bool` value.

- `fn deserialize_i8<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `i8` value.

- `fn deserialize_i16<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `i16` value.

- `fn deserialize_i32<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `i32` value.

- `fn deserialize_i64<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `i64` value.

- `fn deserialize_u8<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `u8` value.

- `fn deserialize_u16<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `u16` value.

- `fn deserialize_u32<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `u32` value.

- `fn deserialize_u64<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `u64` value.

- `fn deserialize_f32<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `f32` value.

- `fn deserialize_f64<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `f64` value.

- `fn deserialize_char<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `char` value.

- `fn deserialize_str<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a string value and does

- `fn deserialize_string<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a string value and would

- `fn deserialize_bytes<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a byte array and does not

- `fn deserialize_byte_buf<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a byte array and would

- `fn deserialize_option<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an optional value.

- `fn deserialize_unit<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a unit value.

- `fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a unit struct with a

- `fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a newtype struct with a

- `fn deserialize_seq<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a sequence of values.

- `fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a sequence of values and

- `fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a tuple struct with a

- `fn deserialize_map<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a map of key-value pairs.

- `fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a struct with a particular

- `fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an enum value with a

- `fn deserialize_identifier<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting the name of a struct

- `fn deserialize_ignored_any<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type needs to deserialize a value whose type

#### Provided Methods

- `fn deserialize_i128<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `i128` value.

- `fn deserialize_u128<V>(self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `u128` value.

- `fn is_human_readable(&self) -> bool`

  Determine whether `Deserialize` implementations should expect to

#### Implementors

- [`BoolDeserializer`](de/value/index.md#booldeserializer)
- [`BorrowedBytesDeserializer`](de/value/index.md#borrowedbytesdeserializer)
- [`BorrowedStrDeserializer`](de/value/index.md#borrowedstrdeserializer)
- [`BytesDeserializer`](de/value/index.md#bytesdeserializer)
- [`CharDeserializer`](de/value/index.md#chardeserializer)
- [`CowStrDeserializer`](de/value/index.md#cowstrdeserializer)
- [`EnumAccessDeserializer`](de/value/index.md#enumaccessdeserializer)
- [`F32Deserializer`](de/value/index.md#f32deserializer)
- [`F64Deserializer`](de/value/index.md#f64deserializer)
- [`I128Deserializer`](de/value/index.md#i128deserializer)
- [`I16Deserializer`](de/value/index.md#i16deserializer)
- [`I32Deserializer`](de/value/index.md#i32deserializer)
- [`I64Deserializer`](de/value/index.md#i64deserializer)
- [`I8Deserializer`](de/value/index.md#i8deserializer)
- [`IsizeDeserializer`](de/value/index.md#isizedeserializer)
- [`MapAccessDeserializer`](de/value/index.md#mapaccessdeserializer)
- [`MapDeserializer`](de/value/index.md#mapdeserializer)
- [`PairDeserializer`](de/value/index.md#pairdeserializer)
- [`SeqAccessDeserializer`](de/value/index.md#seqaccessdeserializer)
- [`SeqDeserializer`](de/value/index.md#seqdeserializer)
- [`StrDeserializer`](de/value/index.md#strdeserializer)
- [`StringDeserializer`](de/value/index.md#stringdeserializer)
- [`U128Deserializer`](de/value/index.md#u128deserializer)
- [`U16Deserializer`](de/value/index.md#u16deserializer)
- [`U32Deserializer`](de/value/index.md#u32deserializer)
- [`U64Deserializer`](de/value/index.md#u64deserializer)
- [`U8Deserializer`](de/value/index.md#u8deserializer)
- [`UnitDeserializer`](de/value/index.md#unitdeserializer)
- [`UsizeDeserializer`](de/value/index.md#usizedeserializer)

### `Serialize`

```rust
trait Serialize { ... }
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:234-268`](../../.source_1765210505/serde_core-1.0.228/src/ser/mod.rs#L234-L268)*

A **data structure** that can be serialized into any data format supported
by Serde.

Serde provides `Serialize` implementations for many Rust primitive and
standard library types. The complete list is `here`. All of
these can be serialized using Serde out of the box.

Additionally, Serde provides a procedural macro called `serde_derive` to
automatically generate `Serialize` implementations for structs and enums in
your program. See the [derive section of the manual] for how to use this.

In rare cases it may be necessary to implement `Serialize` manually for some
type in your program. See the [Implementing `Serialize`] section of the
manual for more about this.

Third-party crates may provide `Serialize` implementations for types that
they expose. For example the `linked-hash-map` crate provides a
`LinkedHashMap<K, V>` type that is serializable by Serde because the crate
provides an implementation of `Serialize` for it.






#### Required Methods

- `fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

  Serialize this value into the given Serde serializer.

#### Implementors

- [`AtomicBool`](lib/index.md#atomicbool)
- [`AtomicI16`](lib/index.md#atomici16)
- [`AtomicI32`](lib/index.md#atomici32)
- [`AtomicI64`](lib/index.md#atomici64)
- [`AtomicI8`](lib/index.md#atomici8)
- [`AtomicIsize`](lib/index.md#atomicisize)
- [`AtomicU16`](lib/index.md#atomicu16)
- [`AtomicU32`](lib/index.md#atomicu32)
- [`AtomicU64`](lib/index.md#atomicu64)
- [`AtomicU8`](lib/index.md#atomicu8)
- [`AtomicUsize`](lib/index.md#atomicusize)
- [`BTreeMap`](lib/index.md#btreemap)
- [`BTreeSet`](lib/index.md#btreeset)
- [`BinaryHeap`](lib/index.md#binaryheap)
- [`Bound`](lib/index.md#bound)
- [`Box`](lib/index.md#box)
- [`CStr`](lib/index.md#cstr)
- [`CString`](lib/index.md#cstring)
- [`Cell`](lib/index.md#cell)
- [`Cow`](lib/index.md#cow)
- [`Duration`](lib/index.md#duration)
- [`HashMap`](lib/index.md#hashmap)
- [`HashSet`](lib/index.md#hashset)
- [`LinkedList`](lib/index.md#linkedlist)
- [`Mutex`](lib/index.md#mutex)
- [`OsStr`](lib/index.md#osstr)
- [`OsString`](lib/index.md#osstring)
- [`PathBuf`](lib/index.md#pathbuf)
- [`Path`](lib/index.md#path)
- [`PhantomData`](lib/index.md#phantomdata)
- [`RangeFrom`](lib/index.md#rangefrom)
- [`RangeInclusive`](lib/index.md#rangeinclusive)
- [`RangeTo`](lib/index.md#rangeto)
- [`Range`](lib/index.md#range)
- [`RefCell`](lib/index.md#refcell)
- [`Reverse`](lib/index.md#reverse)
- [`RwLock`](lib/index.md#rwlock)
- [`Saturating`](lib/index.md#saturating)
- [`String`](lib/index.md#string)
- [`SystemTime`](lib/index.md#systemtime)
- [`VecDeque`](lib/index.md#vecdeque)
- [`Vec`](lib/index.md#vec)
- [`Wrapping`](lib/index.md#wrapping)
- `&'a T`
- `&'a mut T`
- `()`
- `(T)`
- `(T0, T1)`
- `(T0, T1, T2)`
- `(T0, T1, T2, T3)`
- `(T0, T1, T2, T3, T4)`
- `(T0, T1, T2, T3, T4, T5)`
- `(T0, T1, T2, T3, T4, T5, T6)`
- `(T0, T1, T2, T3, T4, T5, T6, T7)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)`
- `(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)`
- `Option<T>`
- `Result<T, E>`
- `[T; 0]`
- `[T; 10]`
- `[T; 11]`
- `[T; 12]`
- `[T; 13]`
- `[T; 14]`
- `[T; 15]`
- `[T; 16]`
- `[T; 17]`
- `[T; 18]`
- `[T; 19]`
- `[T; 1]`
- `[T; 20]`
- `[T; 21]`
- `[T; 22]`
- `[T; 23]`
- `[T; 24]`
- `[T; 25]`
- `[T; 26]`
- `[T; 27]`
- `[T; 28]`
- `[T; 29]`
- `[T; 2]`
- `[T; 30]`
- `[T; 31]`
- `[T; 32]`
- `[T; 3]`
- `[T; 4]`
- `[T; 5]`
- `[T; 6]`
- `[T; 7]`
- `[T; 8]`
- `[T; 9]`
- `[T]`
- `bool`
- `char`
- `f32`
- `f64`
- `fmt::Arguments<'a>`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `net::IpAddr`
- `net::Ipv4Addr`
- `net::Ipv6Addr`
- `net::SocketAddrV4`
- `net::SocketAddrV6`
- `net::SocketAddr`
- `num::NonZeroI128`
- `num::NonZeroI16`
- `num::NonZeroI32`
- `num::NonZeroI64`
- `num::NonZeroI8`
- `num::NonZeroIsize`
- `num::NonZeroU128`
- `num::NonZeroU16`
- `num::NonZeroU32`
- `num::NonZeroU64`
- `num::NonZeroU8`
- `num::NonZeroUsize`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

### `Serializer`

```rust
trait Serializer: Sized { ... }
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:355-1462`](../../.source_1765210505/serde_core-1.0.228/src/ser/mod.rs#L355-L1462)*

A **data format** that can serialize any data structure supported by Serde.

The role of this trait is to define the serialization half of the [Serde
data model], which is a way to categorize every Rust data structure into one
of 29 possible types. Each method of the `Serializer` trait corresponds to
one of the types of the data model.

Implementations of `Serialize` map themselves into this data model by
invoking exactly one of the `Serializer` methods.

The types that make up the Serde data model are:

 - **14 primitive types**
   - bool
   - i8, i16, i32, i64, i128
   - u8, u16, u32, u64, u128
   - f32, f64
   - char
 - **string**
   - UTF-8 bytes with a length and no null terminator.
   - When serializing, all strings are handled equally. When deserializing,
     there are three flavors of strings: transient, owned, and borrowed.
 - **byte array** - \[u8\]
   - Similar to strings, during deserialization byte arrays can be
     transient, owned, or borrowed.
 - **option**
   - Either none or some value.
 - **unit**
   - The type of `()` in Rust. It represents an anonymous value containing
     no data.
 - **unit_struct**
   - For example `struct Unit` or `PhantomData<T>`. It represents a named
     value containing no data.
 - **unit_variant**
   - For example the `E::A` and `E::B` in `enum E { A, B }`.
 - **newtype_struct**
   - For example `struct Millimeters(u8)`.
 - **newtype_variant**
   - For example the `E::N` in `enum E { N(u8) }`.
 - **seq**
   - A variably sized heterogeneous sequence of values, for example
     `Vec<T>` or `HashSet<T>`. When serializing, the length may or may not
     be known before iterating through all the data. When deserializing,
     the length is determined by looking at the serialized data.
 - **tuple**
   - A statically sized heterogeneous sequence of values for which the
     length will be known at deserialization time without looking at the
     serialized data, for example `(u8,)` or `(String, u64, Vec<T>)` or
     `[u64; 10]`.
 - **tuple_struct**
   - A named tuple, for example `struct Rgb(u8, u8, u8)`.
 - **tuple_variant**
   - For example the `E::T` in `enum E { T(u8, u8) }`.
 - **map**
   - A heterogeneous key-value pairing, for example `BTreeMap<K, V>`.
 - **struct**
   - A heterogeneous key-value pairing in which the keys are strings and
     will be known at deserialization time without looking at the
     serialized data, for example `struct S { r: u8, g: u8, b: u8 }`.
 - **struct_variant**
   - For example the `E::S` in `enum E { S { r: u8, g: u8, b: u8 } }`.

Many Serde serializers produce text or binary data as output, for example
JSON or Postcard. This is not a requirement of the `Serializer` trait, and
there are serializers that do not produce text or binary output. One example
is the `serde_json::value::Serializer` (distinct from the main `serde_json`
serializer) that produces a `serde_json::Value` data structure in memory as
output.

# Example implementation

The [example data format] presented on the website contains example code for
a basic JSON `Serializer`.


#### Associated Types

- `type Ok`

- `type Error: 1`

- `type SerializeSeq: 1`

- `type SerializeTuple: 1`

- `type SerializeTupleStruct: 1`

- `type SerializeTupleVariant: 1`

- `type SerializeMap: 1`

- `type SerializeStruct: 1`

- `type SerializeStructVariant: 1`

#### Required Methods

- `fn serialize_bool(self, v: bool) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `bool` value.

- `fn serialize_i8(self, v: i8) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i8` value.

- `fn serialize_i16(self, v: i16) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i16` value.

- `fn serialize_i32(self, v: i32) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i32` value.

- `fn serialize_i64(self, v: i64) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i64` value.

- `fn serialize_u8(self, v: u8) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u8` value.

- `fn serialize_u16(self, v: u16) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u16` value.

- `fn serialize_u32(self, v: u32) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u32` value.

- `fn serialize_u64(self, v: u64) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u64` value.

- `fn serialize_f32(self, v: f32) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `f32` value.

- `fn serialize_f64(self, v: f64) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `f64` value.

- `fn serialize_char(self, v: char) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a character.

- `fn serialize_str(self, v: &str) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `&str`.

- `fn serialize_bytes(self, v: &[u8]) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a chunk of raw byte data.

- `fn serialize_none(self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `None` value.

- `fn serialize_some<T>(self, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `Some(T)` value.

- `fn serialize_unit(self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `()` value.

- `fn serialize_unit_struct(self, name: &'static str) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a unit struct like `struct Unit` or `PhantomData<T>`.

- `fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a unit variant like `E::A` in `enum E { A, B }`.

- `fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a newtype struct like `struct Millimeters(u8)`.

- `fn serialize_newtype_variant<T>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a newtype variant like `E::N` in `enum E { N(u8) }`.

- `fn serialize_seq(self, len: Option<usize>) -> Result<<Self as >::SerializeSeq, <Self as >::Error>`

  Begin to serialize a variably sized sequence. This call must be

- `fn serialize_tuple(self, len: usize) -> Result<<Self as >::SerializeTuple, <Self as >::Error>`

  Begin to serialize a statically sized sequence whose length will be

- `fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct, <Self as >::Error>`

  Begin to serialize a tuple struct like `struct Rgb(u8, u8, u8)`. This

- `fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant, <Self as >::Error>`

  Begin to serialize a tuple variant like `E::T` in `enum E { T(u8, u8)

- `fn serialize_map(self, len: Option<usize>) -> Result<<Self as >::SerializeMap, <Self as >::Error>`

  Begin to serialize a map. This call must be followed by zero or more

- `fn serialize_struct(self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct, <Self as >::Error>`

  Begin to serialize a struct like `struct Rgb { r: u8, g: u8, b: u8 }`.

- `fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeStructVariant, <Self as >::Error>`

  Begin to serialize a struct variant like `E::S` in `enum E { S { r: u8,

#### Provided Methods

- `fn serialize_i128(self, v: i128) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i128` value.

- `fn serialize_u128(self, v: u128) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u128` value.

- `fn collect_seq<I>(self, iter: I) -> Result<<Self as >::Ok, <Self as >::Error>`

  Collect an iterator as a sequence.

- `fn collect_map<K, V, I>(self, iter: I) -> Result<<Self as >::Ok, <Self as >::Error>`

  Collect an iterator as a map.

- `fn collect_str<T>(self, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a string produced by an implementation of `Display`.

- `fn is_human_readable(&self) -> bool`

  Determine whether `Serialize` implementations should serialize in

#### Implementors

- `&mut fmt::Formatter<'a>`

## Macros

### `tri!`

*Defined in [`serde_core-1.0.228/src/lib.rs:111`](../../.source_1765210505/serde_core-1.0.228/src/lib.rs#L111)*

### `forward_to_deserialize_any!`

*Defined in [`serde_core-1.0.228/src/macros.rs:110-118`](../../.source_1765210505/serde_core-1.0.228/src/macros.rs#L110-L118)*

Helper macro when implementing the `Deserializer` part of a new data format
for Serde.

Some [`Deserializer`](de/index.md) implementations for self-describing formats do not
care what hint the [`Visitor`](de/index.md) gives them, they just want to blindly call
the [`Visitor`](de/index.md) method corresponding to the data they can tell is in the
input. This requires repetitive implementations of all the [`Deserializer`](de/index.md)
trait methods.

```edition2021
use serde::forward_to_deserialize_any;
use serde::de::{value, Deserializer, Visitor};

struct MyDeserializer;

impl<'de> Deserializer<'de> for MyDeserializer {
    type Error = value::Error;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

#[inline]
fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
where
    V: Visitor<'de>,
{
    self.deserialize_any(visitor)
}

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
```

The `forward_to_deserialize_any!` macro implements these simple forwarding
methods so that they forward directly to `Deserializer::deserialize_any`.
You can choose which methods to forward.

```edition2021
use serde::forward_to_deserialize_any;
use serde::de::{value, Deserializer, Visitor};

struct MyDeserializer;

impl<'de> Deserializer<'de> for MyDeserializer {
  type Error = value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        /* ... */
      let _ = visitor;
      unimplemented!()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
```

The macro assumes the convention that your `Deserializer` lifetime parameter
is called `'de` and that the `Visitor` type parameters on each method are
called `V`. A different type parameter and a different lifetime can be
specified explicitly if necessary.

```edition2021
use serde::forward_to_deserialize_any;
use serde::de::{value, Deserializer, Visitor};
use std::marker::PhantomData;

struct MyDeserializer<V>(PhantomData<V>);

impl<'q, V> Deserializer<'q> for MyDeserializer<V> {
    type Error = value::Error;

    fn deserialize_any<W>(self, visitor: W) -> Result<W::Value, Self::Error>
    where
        W: Visitor<'q>,
    {
        unimplemented!()
    }

forward_to_deserialize_any! {
    <W: Visitor<'q>>
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
    bytes byte_buf option unit unit_struct newtype_struct seq tuple
    tuple_struct map struct enum identifier ignored_any
}
}
```




