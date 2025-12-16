*[serde_core](../index.md) / [ser](index.md)*

---

# Module `ser`

Generic data structure serialization framework.

The two most important traits in this module are [`Serialize`](#serialize) and
[`Serializer`](#serializer).

 - **A type that implements `Serialize` is a data structure** that can be
   serialized to any data format supported by Serde, and conversely
 - **A type that implements `Serializer` is a data format** that can
   serialize any data structure supported by Serde.

# The Serialize trait

Serde provides [`Serialize`](#serialize) implementations for many Rust primitive and
standard library types. The complete list is below. All of these can be
serialized using Serde out of the box.

Additionally, Serde provides a procedural macro called `serde_derive` to
automatically generate [`Serialize`](#serialize) implementations for structs and enums
in your program. See the [derive section of the manual] for how to use this.

In rare cases it may be necessary to implement [`Serialize`](#serialize) manually for
some type in your program. See the [Implementing `Serialize`] section of the
manual for more about this.

Third-party crates may provide [`Serialize`](#serialize) implementations for types that
they expose. For example the `linked-hash-map` crate provides a
`LinkedHashMap<K, V>` type that is serializable by Serde because the crate
provides an implementation of [`Serialize`](#serialize) for it.

# The Serializer trait

[`Serializer`](#serializer) implementations are provided by third-party crates, for
example `serde_json`, `serde_yaml` and `postcard`.

A partial list of well-maintained formats is given on the [Serde
website][data formats].

# Implementations of Serialize provided by Serde

 - **Primitive types**:
   - bool
   - i8, i16, i32, i64, i128, isize
   - u8, u16, u32, u64, u128, usize
   - f32, f64
   - char
   - str
   - &T and &mut T
 - **Compound types**:
   - \[T\]
   - \[T; 0\] through \[T; 32\]
   - tuples up to size 16
 - **Common standard library types**:
   - String
   - Option\<T\>
   - Result\<T, E\>
   - PhantomData\<T\>
 - **Wrapper types**:
   - Box\<T\>
   - Cow\<'a, T\>
   - Cell\<T\>
   - RefCell\<T\>
   - Mutex\<T\>
   - RwLock\<T\>
   - Rc\<T\>&emsp;*(if* features = \["rc"\] *is enabled)*
   - Arc\<T\>&emsp;*(if* features = \["rc"\] *is enabled)*
 - **Collection types**:
   - BTreeMap\<K, V\>
   - BTreeSet\<T\>
   - BinaryHeap\<T\>
   - HashMap\<K, V, H\>
   - HashSet\<T, H\>
   - LinkedList\<T\>
   - VecDeque\<T\>
   - Vec\<T\>
 - **FFI types**:
   - CStr
   - CString
   - OsStr
   - OsString
 - **Miscellaneous standard library types**:
   - Duration
   - SystemTime
   - Path
   - PathBuf
   - Range\<T\>
   - RangeInclusive\<T\>
   - Bound\<T\>
   - num::NonZero*
   - `!` *(unstable)*
 - **Net types**:
   - IpAddr
   - Ipv4Addr
   - Ipv6Addr
   - SocketAddr
   - SocketAddrV4
   - SocketAddrV6












## Contents

- [Modules](#modules)
  - [`fmt`](#fmt)
  - [`impls`](#impls)
  - [`impossible`](#impossible)
- [Structs](#structs)
  - [`Impossible`](#impossible)
- [Traits](#traits)
  - [`Error`](#error)
  - [`Serialize`](#serialize)
  - [`Serializer`](#serializer)
  - [`SerializeSeq`](#serializeseq)
  - [`SerializeTuple`](#serializetuple)
  - [`SerializeTupleStruct`](#serializetuplestruct)
  - [`SerializeTupleVariant`](#serializetuplevariant)
  - [`SerializeMap`](#serializemap)
  - [`SerializeStruct`](#serializestruct)
  - [`SerializeStructVariant`](#serializestructvariant)
- [Functions](#functions)
  - [`iterator_len_hint`](#iterator-len-hint)
- [Macros](#macros)
  - [`declare_error_trait!`](#declare-error-trait)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fmt`](#fmt) | mod |  |
| [`impls`](#impls) | mod |  |
| [`impossible`](#impossible) | mod | This module contains `Impossible` serializer and its implementations. |
| [`Impossible`](#impossible) | struct |  |
| [`Error`](#error) | trait | Trait used by `Serialize` implementations to generically construct errors belonging to the `Serializer` against which they are currently running. |
| [`Serialize`](#serialize) | trait | A **data structure** that can be serialized into any data format supported by Serde. |
| [`Serializer`](#serializer) | trait | A **data format** that can serialize any data structure supported by Serde. |
| [`SerializeSeq`](#serializeseq) | trait | Returned from `Serializer::serialize_seq`. |
| [`SerializeTuple`](#serializetuple) | trait | Returned from `Serializer::serialize_tuple`. |
| [`SerializeTupleStruct`](#serializetuplestruct) | trait | Returned from `Serializer::serialize_tuple_struct`. |
| [`SerializeTupleVariant`](#serializetuplevariant) | trait | Returned from `Serializer::serialize_tuple_variant`. |
| [`SerializeMap`](#serializemap) | trait | Returned from `Serializer::serialize_map`. |
| [`SerializeStruct`](#serializestruct) | trait | Returned from `Serializer::serialize_struct`. |
| [`SerializeStructVariant`](#serializestructvariant) | trait | Returned from `Serializer::serialize_struct_variant`. |
| [`iterator_len_hint`](#iterator-len-hint) | fn |  |
| [`declare_error_trait!`](#declare-error-trait) | macro |  |

## Modules

- [`fmt`](fmt/index.md)
- [`impls`](impls/index.md)
- [`impossible`](impossible/index.md) — This module contains `Impossible` serializer and its implementations.

## Structs

### `Impossible<Ok, Error>`

```rust
struct Impossible<Ok, Error> {
    void: Void,
    ok: PhantomData<Ok>,
    error: PhantomData<Error>,
}
```

*Defined in [`serde_core-1.0.228/src/ser/impossible.rs:60-64`](../../../.source_1765894658/serde_core-1.0.228/src/ser/impossible.rs#L60-L64)*

Helper type for implementing a `Serializer` that does not support
serializing one of the compound types.

This type cannot be instantiated, but implements every one of the traits
corresponding to the [`Serializer`](#serializer) compound types: [`SerializeSeq`](#serializeseq),
[`SerializeTuple`](#serializetuple), [`SerializeTupleStruct`](#serializetuplestruct), [`SerializeTupleVariant`](#serializetuplevariant),
[`SerializeMap`](#serializemap), [`SerializeStruct`](#serializestruct), and [`SerializeStructVariant`](#serializestructvariant).

```edition2021
use serde::ser::{Serializer, Impossible};
use serde_core::__private::doc::Error;

struct MySerializer;

impl Serializer for MySerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<(), Error>;
    /* other associated types */

    /// This data format does not support serializing sequences.
    fn serialize_seq(self,
                     len: Option<usize>)
                     -> Result<Self::SerializeSeq, Error> {
        // Given Impossible cannot be instantiated, the only
        // thing we can do here is to return an error.
        stringify! {
        Err(...)
        };
        unimplemented!()
    }

    /* other Serializer methods */
    serde_core::__serialize_unimplemented! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str bytes none some
        unit unit_struct unit_variant newtype_struct newtype_variant
        tuple tuple_struct tuple_variant map struct struct_variant
    }
}
```









#### Trait Implementations

##### `impl Any for Impossible<Ok, Error>`

- <span id="impossible-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Impossible<Ok, Error>`

- <span id="impossible-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Impossible<Ok, Error>`

- <span id="impossible-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Impossible<Ok, Error>`

- <span id="impossible-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Impossible<Ok, Error>`

- <span id="impossible-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<Ok, Error> SerializeMap for Impossible<Ok, Error>`

- <span id="impossible-serializemap-type-ok"></span>`type Ok = Ok`

- <span id="impossible-serializemap-type-error"></span>`type Error = Error`

- <span id="impossible-serializemap-serialize-key"></span>`fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>`

- <span id="impossible-serializemap-serialize-value"></span>`fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>`

- <span id="impossible-serializemap-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeSeq for Impossible<Ok, Error>`

- <span id="impossible-serializeseq-type-ok"></span>`type Ok = Ok`

- <span id="impossible-serializeseq-type-error"></span>`type Error = Error`

- <span id="impossible-serializeseq-serialize-element"></span>`fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>`

- <span id="impossible-serializeseq-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeStruct for Impossible<Ok, Error>`

- <span id="impossible-serializestruct-type-ok"></span>`type Ok = Ok`

- <span id="impossible-serializestruct-type-error"></span>`type Error = Error`

- <span id="impossible-serializestruct-serialize-field"></span>`fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>`

- <span id="impossible-serializestruct-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeStructVariant for Impossible<Ok, Error>`

- <span id="impossible-serializestructvariant-type-ok"></span>`type Ok = Ok`

- <span id="impossible-serializestructvariant-type-error"></span>`type Error = Error`

- <span id="impossible-serializestructvariant-serialize-field"></span>`fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>`

- <span id="impossible-serializestructvariant-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeTuple for Impossible<Ok, Error>`

- <span id="impossible-serializetuple-type-ok"></span>`type Ok = Ok`

- <span id="impossible-serializetuple-type-error"></span>`type Error = Error`

- <span id="impossible-serializetuple-serialize-element"></span>`fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>`

- <span id="impossible-serializetuple-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeTupleStruct for Impossible<Ok, Error>`

- <span id="impossible-serializetuplestruct-type-ok"></span>`type Ok = Ok`

- <span id="impossible-serializetuplestruct-type-error"></span>`type Error = Error`

- <span id="impossible-serializetuplestruct-serialize-field"></span>`fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>`

- <span id="impossible-serializetuplestruct-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeTupleVariant for Impossible<Ok, Error>`

- <span id="impossible-serializetuplevariant-type-ok"></span>`type Ok = Ok`

- <span id="impossible-serializetuplevariant-type-error"></span>`type Error = Error`

- <span id="impossible-serializetuplevariant-serialize-field"></span>`fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>`

- <span id="impossible-serializetuplevariant-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<U> TryFrom for Impossible<Ok, Error>`

- <span id="impossible-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="impossible-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Impossible<Ok, Error>`

- <span id="impossible-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="impossible-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Error`

```rust
trait Error: Sized + StdError { ... }
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:192`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L192)*

Trait used by `Serialize` implementations to generically construct
errors belonging to the `Serializer` against which they are
currently running.

# Example implementation

The [example data format] presented on the website shows an error
type appropriate for a basic JSON data format.


#### Required Methods

- `fn custom<T>(msg: T) -> Self`

  Used when a [`Serialize`](#serialize) implementation encounters any error
  while serializing a type.
  
  The message should not be capitalized and should not end with a
  period.
  
  For example, a filesystem [`Path`](../../rustdoc_types/index.md) may refuse to serialize
  itself if it contains invalid UTF-8 data.
  
  ```edition2021
  struct Path;
  
  impl Path {
      fn to_str(&self) -> Option<&str> {
          unimplemented!()
      }
  }
  
  use serde::ser::{self, Serialize, Serializer};
  
  impl Serialize for Path {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          match self.to_str() {
              Some(s) => serializer.serialize_str(s),
              None => Err(ser::Error::custom("path contains invalid UTF-8 characters")),
          }
      }
  }
  ```
  

#### Implementors

- [`Error`](../de/value/index.md#error)
- `fmt::Error`

### `Serialize`

```rust
trait Serialize { ... }
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:234-268`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L234-L268)*

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
  
  See the [Implementing `Serialize`] section of the manual for more
  information about how to implement this method.
  
  ```edition2021
  use serde::ser::{Serialize, SerializeStruct, Serializer};
  
  struct Person {
      name: String,
      age: u8,
      phones: Vec<String>,
  }
  
  // This is what #[derive(Serialize)] would generate.
  impl Serialize for Person {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          let mut s = serializer.serialize_struct("Person", 3)?;
          s.serialize_field("name", &self.name)?;
          s.serialize_field("age", &self.age)?;
          s.serialize_field("phones", &self.phones)?;
          s.end()
      }
  }
  ```

#### Implementors

- [`AtomicBool`](../lib/index.md#atomicbool)
- [`AtomicI16`](../lib/index.md#atomici16)
- [`AtomicI32`](../lib/index.md#atomici32)
- [`AtomicI64`](../lib/index.md#atomici64)
- [`AtomicI8`](../lib/index.md#atomici8)
- [`AtomicIsize`](../lib/index.md#atomicisize)
- [`AtomicU16`](../lib/index.md#atomicu16)
- [`AtomicU32`](../lib/index.md#atomicu32)
- [`AtomicU64`](../lib/index.md#atomicu64)
- [`AtomicU8`](../lib/index.md#atomicu8)
- [`AtomicUsize`](../lib/index.md#atomicusize)
- [`BTreeMap`](../lib/index.md#btreemap)
- [`BTreeSet`](../lib/index.md#btreeset)
- [`BinaryHeap`](../lib/index.md#binaryheap)
- [`Bound`](../lib/index.md#bound)
- [`Box`](../lib/index.md#box)
- [`CStr`](../lib/index.md#cstr)
- [`CString`](../lib/index.md#cstring)
- [`Cell`](../lib/index.md#cell)
- [`Cow`](../lib/index.md#cow)
- [`Duration`](../lib/index.md#duration)
- [`HashMap`](../lib/index.md#hashmap)
- [`HashSet`](../lib/index.md#hashset)
- [`LinkedList`](../lib/index.md#linkedlist)
- [`Mutex`](../lib/index.md#mutex)
- [`OsStr`](../lib/index.md#osstr)
- [`OsString`](../lib/index.md#osstring)
- [`PathBuf`](../lib/index.md#pathbuf)
- [`Path`](../lib/index.md#path)
- [`PhantomData`](../lib/index.md#phantomdata)
- [`RangeFrom`](../lib/index.md#rangefrom)
- [`RangeInclusive`](../lib/index.md#rangeinclusive)
- [`RangeTo`](../lib/index.md#rangeto)
- [`Range`](../lib/index.md#range)
- [`RefCell`](../lib/index.md#refcell)
- [`Reverse`](../lib/index.md#reverse)
- [`RwLock`](../lib/index.md#rwlock)
- [`Saturating`](../lib/index.md#saturating)
- [`String`](../lib/index.md#string)
- [`SystemTime`](../lib/index.md#systemtime)
- [`VecDeque`](../lib/index.md#vecdeque)
- [`Vec`](../lib/index.md#vec)
- [`Wrapping`](../lib/index.md#wrapping)
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

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:355-1462`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L355-L1462)*

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
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for bool {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_bool(*self)
      }
  }
  ```

- `fn serialize_i8(self, v: i8) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i8` value.
  
  If the format does not differentiate between `i8` and `i64`, a
  reasonable implementation would be to cast the value to `i64` and
  forward to `serialize_i64`.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for i8 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_i8(*self)
      }
  }
  ```

- `fn serialize_i16(self, v: i16) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i16` value.
  
  If the format does not differentiate between `i16` and `i64`, a
  reasonable implementation would be to cast the value to `i64` and
  forward to `serialize_i64`.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for i16 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_i16(*self)
      }
  }
  ```

- `fn serialize_i32(self, v: i32) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i32` value.
  
  If the format does not differentiate between `i32` and `i64`, a
  reasonable implementation would be to cast the value to `i64` and
  forward to `serialize_i64`.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for i32 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_i32(*self)
      }
  }
  ```

- `fn serialize_i64(self, v: i64) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i64` value.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for i64 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_i64(*self)
      }
  }
  ```

- `fn serialize_u8(self, v: u8) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u8` value.
  
  If the format does not differentiate between `u8` and `u64`, a
  reasonable implementation would be to cast the value to `u64` and
  forward to `serialize_u64`.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for u8 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_u8(*self)
      }
  }
  ```

- `fn serialize_u16(self, v: u16) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u16` value.
  
  If the format does not differentiate between `u16` and `u64`, a
  reasonable implementation would be to cast the value to `u64` and
  forward to `serialize_u64`.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for u16 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_u16(*self)
      }
  }
  ```

- `fn serialize_u32(self, v: u32) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u32` value.
  
  If the format does not differentiate between `u32` and `u64`, a
  reasonable implementation would be to cast the value to `u64` and
  forward to `serialize_u64`.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for u32 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_u32(*self)
      }
  }
  ```

- `fn serialize_u64(self, v: u64) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u64` value.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for u64 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_u64(*self)
      }
  }
  ```

- `fn serialize_f32(self, v: f32) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `f32` value.
  
  If the format does not differentiate between `f32` and `f64`, a
  reasonable implementation would be to cast the value to `f64` and
  forward to `serialize_f64`.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for f32 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_f32(*self)
      }
  }
  ```

- `fn serialize_f64(self, v: f64) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `f64` value.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for f64 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_f64(*self)
      }
  }
  ```

- `fn serialize_char(self, v: char) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a character.
  
  If the format does not support characters, it is reasonable to serialize
  it as a single element `str` or a `u32`.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for char {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_char(*self)
      }
  }
  ```

- `fn serialize_str(self, v: &str) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `&str`.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for str {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_str(self)
      }
  }
  ```

- `fn serialize_bytes(self, v: &[u8]) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a chunk of raw byte data.
  
  Enables serializers to serialize byte slices more compactly or more
  efficiently than other types of slices. If no efficient implementation
  is available, a reasonable implementation would be to forward to
  `serialize_seq`. If forwarded, the implementation looks usually just
  like this:
  
  ```edition2021
  use serde::ser::{Serializer, SerializeSeq};
  use serde_core::__private::doc::Error;
  
  struct MySerializer;
  
  impl Serializer for MySerializer {
      type Ok = ();
      type Error = Error;
  
  fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
      let mut seq = self.serialize_seq(Some(v.len()))?;
      for b in v {
          seq.serialize_element(b)?;
      }
      seq.end()
  }
  
      serde_core::__serialize_unimplemented! {
          bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str none some
          unit unit_struct unit_variant newtype_struct newtype_variant
          seq tuple tuple_struct tuple_variant map struct struct_variant
      }
  }
  ```

- `fn serialize_none(self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `None` value.
  
  ```edition2021
  use serde::{Serialize, Serializer};
  
  enum Option<T> {
      Some(T),
      None,
  }
  
  use self::Option::{Some, None};
  
  impl<T> Serialize for Option<T>
  where
      T: Serialize,
  {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          match *self {
              Some(ref value) => serializer.serialize_some(value),
              None => serializer.serialize_none(),
          }
      }
  }
  
  fn main() {}
  ```

- `fn serialize_some<T>(self, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `Some(T)` value.
  
  ```edition2021
  use serde::{Serialize, Serializer};
  
  enum Option<T> {
      Some(T),
      None,
  }
  
  use self::Option::{Some, None};
  
  impl<T> Serialize for Option<T>
  where
      T: Serialize,
  {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          match *self {
              Some(ref value) => serializer.serialize_some(value),
              None => serializer.serialize_none(),
          }
      }
  }
  
  fn main() {}
  ```

- `fn serialize_unit(self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `()` value.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for () {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_unit()
      }
  }
  ```

- `fn serialize_unit_struct(self, name: &'static str) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a unit struct like `struct Unit` or `PhantomData<T>`.
  
  A reasonable implementation would be to forward to `serialize_unit`.
  
  ```edition2021
  use serde::{Serialize, Serializer};
  
  struct Nothing;
  
  impl Serialize for Nothing {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_unit_struct("Nothing")
      }
  }
  ```

- `fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a unit variant like `E::A` in `enum E { A, B }`.
  
  The `name` is the name of the enum, the `variant_index` is the index of
  this variant within the enum, and the `variant` is the name of the
  variant.
  
  ```edition2021
  use serde::{Serialize, Serializer};
  
  enum E {
      A,
      B,
  }
  
  impl Serialize for E {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          match *self {
              E::A => serializer.serialize_unit_variant("E", 0, "A"),
              E::B => serializer.serialize_unit_variant("E", 1, "B"),
          }
      }
  }
  ```

- `fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a newtype struct like `struct Millimeters(u8)`.
  
  Serializers are encouraged to treat newtype structs as insignificant
  wrappers around the data they contain. A reasonable implementation would
  be to forward to `value.serialize(self)`.
  
  ```edition2021
  use serde::{Serialize, Serializer};
  
  struct Millimeters(u8);
  
  impl Serialize for Millimeters {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_newtype_struct("Millimeters", &self.0)
      }
  }
  ```

- `fn serialize_newtype_variant<T>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a newtype variant like `E::N` in `enum E { N(u8) }`.
  
  The `name` is the name of the enum, the `variant_index` is the index of
  this variant within the enum, and the `variant` is the name of the
  variant. The `value` is the data contained within this newtype variant.
  
  ```edition2021
  use serde::{Serialize, Serializer};
  
  enum E {
      M(String),
      N(u8),
  }
  
  impl Serialize for E {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          match *self {
              E::M(ref s) => serializer.serialize_newtype_variant("E", 0, "M", s),
              E::N(n) => serializer.serialize_newtype_variant("E", 1, "N", &n),
          }
      }
  }
  ```

- `fn serialize_seq(self, len: Option<usize>) -> Result<<Self as >::SerializeSeq, <Self as >::Error>`

  Begin to serialize a variably sized sequence. This call must be
  followed by zero or more calls to `serialize_element`, then a call to
  `end`.
  
  The argument is the number of elements in the sequence, which may or may
  not be computable before the sequence is iterated. Some serializers only
  support sequences whose length is known up front.
  
  ```edition2021
  use std::marker::PhantomData;
  
  struct Vec<T>(PhantomData<T>);
  
  impl<T> Vec<T> {
      fn len(&self) -> usize {
          unimplemented!()
      }
  }
  
  impl<'a, T> IntoIterator for &'a Vec<T> {
      type Item = &'a T;
      type IntoIter = Box<dyn Iterator<Item = &'a T>>;
  
      fn into_iter(self) -> Self::IntoIter {
          unimplemented!()
      }
  }
  
  use serde::ser::{Serialize, SerializeSeq, Serializer};
  
  impl<T> Serialize for Vec<T>
  where
      T: Serialize,
  {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          let mut seq = serializer.serialize_seq(Some(self.len()))?;
          for element in self {
              seq.serialize_element(element)?;
          }
          seq.end()
      }
  }
  ```

- `fn serialize_tuple(self, len: usize) -> Result<<Self as >::SerializeTuple, <Self as >::Error>`

  Begin to serialize a statically sized sequence whose length will be
  known at deserialization time without looking at the serialized data.
  This call must be followed by zero or more calls to `serialize_element`,
  then a call to `end`.
  
  ```edition2021
  use serde::ser::{Serialize, SerializeTuple, Serializer};
  
  mod fool {
      trait Serialize {}
  impl<A, B, C> Serialize for (A, B, C)
      {}
  }
  
  struct Tuple3<A, B, C>(A, B, C);
  
  impl<A, B, C> Serialize for Tuple3<A, B, C>
  where
      A: Serialize,
      B: Serialize,
      C: Serialize,
  {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          let mut tup = serializer.serialize_tuple(3)?;
          tup.serialize_element(&self.0)?;
          tup.serialize_element(&self.1)?;
          tup.serialize_element(&self.2)?;
          tup.end()
      }
  }
  ```
  
  ```edition2021
  use serde::ser::{Serialize, SerializeTuple, Serializer};
  
  const VRAM_SIZE: usize = 386;
  struct Vram([u16; VRAM_SIZE]);
  
  impl Serialize for Vram {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          let mut seq = serializer.serialize_tuple(VRAM_SIZE)?;
          for element in &self.0[..] {
              seq.serialize_element(element)?;
          }
          seq.end()
      }
  }
  ```

- `fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct, <Self as >::Error>`

  Begin to serialize a tuple struct like `struct Rgb(u8, u8, u8)`. This
  call must be followed by zero or more calls to `serialize_field`, then a
  call to `end`.
  
  The `name` is the name of the tuple struct and the `len` is the number
  of data fields that will be serialized.
  
  ```edition2021
  use serde::ser::{Serialize, SerializeTupleStruct, Serializer};
  
  struct Rgb(u8, u8, u8);
  
  impl Serialize for Rgb {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          let mut ts = serializer.serialize_tuple_struct("Rgb", 3)?;
          ts.serialize_field(&self.0)?;
          ts.serialize_field(&self.1)?;
          ts.serialize_field(&self.2)?;
          ts.end()
      }
  }
  ```

- `fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant, <Self as >::Error>`

  Begin to serialize a tuple variant like `E::T` in `enum E { T(u8, u8)
  }`. This call must be followed by zero or more calls to
  `serialize_field`, then a call to `end`.
  
  The `name` is the name of the enum, the `variant_index` is the index of
  this variant within the enum, the `variant` is the name of the variant,
  and the `len` is the number of data fields that will be serialized.
  
  ```edition2021
  use serde::ser::{Serialize, SerializeTupleVariant, Serializer};
  
  enum E {
      T(u8, u8),
      U(String, u32, u32),
  }
  
  impl Serialize for E {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          match *self {
              E::T(ref a, ref b) => {
                  let mut tv = serializer.serialize_tuple_variant("E", 0, "T", 2)?;
                  tv.serialize_field(a)?;
                  tv.serialize_field(b)?;
                  tv.end()
              }
              E::U(ref a, ref b, ref c) => {
                  let mut tv = serializer.serialize_tuple_variant("E", 1, "U", 3)?;
                  tv.serialize_field(a)?;
                  tv.serialize_field(b)?;
                  tv.serialize_field(c)?;
                  tv.end()
              }
          }
      }
  }
  ```

- `fn serialize_map(self, len: Option<usize>) -> Result<<Self as >::SerializeMap, <Self as >::Error>`

  Begin to serialize a map. This call must be followed by zero or more
  calls to `serialize_key` and `serialize_value`, then a call to `end`.
  
  The argument is the number of elements in the map, which may or may not
  be computable before the map is iterated. Some serializers only support
  maps whose length is known up front.
  
  ```edition2021
  use std::marker::PhantomData;
  
  struct HashMap<K, V>(PhantomData<K>, PhantomData<V>);
  
  impl<K, V> HashMap<K, V> {
      fn len(&self) -> usize {
          unimplemented!()
      }
  }
  
  impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
      type Item = (&'a K, &'a V);
      type IntoIter = Box<dyn Iterator<Item = (&'a K, &'a V)>>;
  
      fn into_iter(self) -> Self::IntoIter {
          unimplemented!()
      }
  }
  
  use serde::ser::{Serialize, SerializeMap, Serializer};
  
  impl<K, V> Serialize for HashMap<K, V>
  where
      K: Serialize,
      V: Serialize,
  {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          let mut map = serializer.serialize_map(Some(self.len()))?;
          for (k, v) in self {
              map.serialize_entry(k, v)?;
          }
          map.end()
      }
  }
  ```

- `fn serialize_struct(self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct, <Self as >::Error>`

  Begin to serialize a struct like `struct Rgb { r: u8, g: u8, b: u8 }`.
  This call must be followed by zero or more calls to `serialize_field`,
  then a call to `end`.
  
  The `name` is the name of the struct and the `len` is the number of
  data fields that will be serialized. `len` does not include fields
  which are skipped with `SerializeStruct::skip_field`.
  
  ```edition2021
  use serde::ser::{Serialize, SerializeStruct, Serializer};
  
  struct Rgb {
      r: u8,
      g: u8,
      b: u8,
  }
  
  impl Serialize for Rgb {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          let mut rgb = serializer.serialize_struct("Rgb", 3)?;
          rgb.serialize_field("r", &self.r)?;
          rgb.serialize_field("g", &self.g)?;
          rgb.serialize_field("b", &self.b)?;
          rgb.end()
      }
  }
  ```

- `fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeStructVariant, <Self as >::Error>`

  Begin to serialize a struct variant like `E::S` in `enum E { S { r: u8,
  g: u8, b: u8 } }`. This call must be followed by zero or more calls to
  `serialize_field`, then a call to `end`.
  
  The `name` is the name of the enum, the `variant_index` is the index of
  this variant within the enum, the `variant` is the name of the variant,
  and the `len` is the number of data fields that will be serialized.
  `len` does not include fields which are skipped with
  `SerializeStructVariant::skip_field`.
  
  ```edition2021
  use serde::ser::{Serialize, SerializeStructVariant, Serializer};
  
  enum E {
      S { r: u8, g: u8, b: u8 },
  }
  
  impl Serialize for E {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          match *self {
              E::S {
                  ref r,
                  ref g,
                  ref b,
              } => {
                  let mut sv = serializer.serialize_struct_variant("E", 0, "S", 3)?;
                  sv.serialize_field("r", r)?;
                  sv.serialize_field("g", g)?;
                  sv.serialize_field("b", b)?;
                  sv.end()
              }
          }
      }
  }
  ```

#### Provided Methods

- `fn serialize_i128(self, v: i128) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i128` value.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for i128 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_i128(*self)
      }
  }
  ```
  
  The default behavior unconditionally returns an error.

- `fn serialize_u128(self, v: u128) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u128` value.
  
  ```edition2021
  use serde::Serializer;
  
  serde_core::__private_serialize!();
  
  impl Serialize for u128 {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.serialize_u128(*self)
      }
  }
  ```
  
  The default behavior unconditionally returns an error.

- `fn collect_seq<I>(self, iter: I) -> Result<<Self as >::Ok, <Self as >::Error>`

  Collect an iterator as a sequence.
  
  The default implementation serializes each item yielded by the iterator
  using `serialize_seq`. Implementors should not need to override this
  method.
  
  ```edition2021
  use serde::{Serialize, Serializer};
  
  struct SecretlyOneHigher {
      data: Vec<i32>,
  }
  
  impl Serialize for SecretlyOneHigher {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.collect_seq(self.data.iter().map(|x| x + 1))
      }
  }
  ```

- `fn collect_map<K, V, I>(self, iter: I) -> Result<<Self as >::Ok, <Self as >::Error>`

  Collect an iterator as a map.
  
  The default implementation serializes each pair yielded by the iterator
  using `serialize_map`. Implementors should not need to override this
  method.
  
  ```edition2021
  use serde::{Serialize, Serializer};
  use std::collections::BTreeSet;
  
  struct MapToUnit {
      keys: BTreeSet<i32>,
  }
  
  // Serializes as a map in which the values are all unit.
  impl Serialize for MapToUnit {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.collect_map(self.keys.iter().map(|k| (k, ())))
      }
  }
  ```

- `fn collect_str<T>(self, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a string produced by an implementation of `Display`.
  
  The default implementation builds a heap-allocated [`String`](../lib/index.md) and
  delegates to `serialize_str`. Serializers are encouraged to provide a
  more efficient implementation if possible.
  
  ```edition2021
  struct DateTime;
  
  impl DateTime {
      fn naive_local(&self) -> () { () }
      fn offset(&self) -> () { () }
  }
  
  use serde::{Serialize, Serializer};
  
  impl Serialize for DateTime {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          serializer.collect_str(&format_args!("{:?}{:?}", self.naive_local(), self.offset()))
      }
  }
  ```

- `fn is_human_readable(&self) -> bool`

  Determine whether `Serialize` implementations should serialize in
  human-readable form.
  
  Some types have a human-readable form that may be somewhat expensive to
  construct, as well as a binary form that is compact and efficient.
  Generally text-based formats like JSON and YAML will prefer to use the
  human-readable one and binary formats like Postcard will prefer the
  compact one.
  
  ```edition2021
  use std::fmt::{self, Display};
  
  struct Timestamp;
  
  impl Timestamp {
      fn seconds_since_epoch(&self) -> u64 { unimplemented!() }
  }
  
  impl Display for Timestamp {
      fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
          unimplemented!()
      }
  }
  
  use serde::{Serialize, Serializer};
  
  impl Serialize for Timestamp {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: Serializer,
      {
          if serializer.is_human_readable() {
              // Serialize to a human-readable string "2015-05-15T17:01:00Z".
              self.to_string().serialize(serializer)
          } else {
              // Serialize to a compact binary representation.
              self.seconds_since_epoch().serialize(serializer)
          }
      }
  }
  ```
  
  The default implementation of this method returns `true`. Data formats
  may override this to `false` to request a compact form for types that
  support one. Note that modifying this method to change a format from
  human-readable to compact or vice versa should be regarded as a breaking
  change, as a value serialized in human-readable mode is not required to
  deserialize from the same data in compact mode.

#### Implementors

- `&mut fmt::Formatter<'a>`

### `SerializeSeq`

```rust
trait SerializeSeq { ... }
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:1518-1532`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L1518-L1532)*

Returned from `Serializer::serialize_seq`.

# Example use

```edition2021
use std::marker::PhantomData;

struct Vec<T>(PhantomData<T>);

impl<T> Vec<T> {
    fn len(&self) -> usize {
        unimplemented!()
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = Box<dyn Iterator<Item = &'a T>>;
    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

use serde::ser::{Serialize, SerializeSeq, Serializer};

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeSeq` for a basic JSON data format.


#### Associated Types

- `type Ok`

- `type Error: 1`

#### Required Methods

- `fn serialize_element<T>(&mut self, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a sequence element.

- `fn end(self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a sequence.

#### Implementors

- [`Impossible`](impossible/index.md#impossible)

### `SerializeTuple`

```rust
trait SerializeTuple { ... }
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:1624-1638`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L1624-L1638)*

Returned from `Serializer::serialize_tuple`.

# Example use

```edition2021
use serde::ser::{Serialize, SerializeTuple, Serializer};

mod fool {
    trait Serialize {}
impl<A, B, C> Serialize for (A, B, C)
    {}
}

struct Tuple3<A, B, C>(A, B, C);

impl<A, B, C> Serialize for Tuple3<A, B, C>
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(3)?;
        tup.serialize_element(&self.0)?;
        tup.serialize_element(&self.1)?;
        tup.serialize_element(&self.2)?;
        tup.end()
    }
}
```

```edition2021
use std::marker::PhantomData;

struct Array<T>(PhantomData<T>);

impl<T> Array<T> {
    fn len(&self) -> usize {
        unimplemented!()
    }
}

impl<'a, T> IntoIterator for &'a Array<T> {
    type Item = &'a T;
    type IntoIter = Box<dyn Iterator<Item = &'a T>>;
    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

use serde::ser::{Serialize, SerializeTuple, Serializer};

mod fool {
    trait Serialize {}
impl<T> Serialize for [T; 16]
    {}
}

impl<T> Serialize for Array<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(16)?;
        for element in self {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeTuple` for a basic JSON data format.


#### Associated Types

- `type Ok`

- `type Error: 1`

#### Required Methods

- `fn serialize_element<T>(&mut self, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a tuple element.

- `fn end(self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a tuple.

#### Implementors

- [`Impossible`](impossible/index.md#impossible)

### `SerializeTupleStruct`

```rust
trait SerializeTupleStruct { ... }
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:1675-1689`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L1675-L1689)*

Returned from `Serializer::serialize_tuple_struct`.

# Example use

```edition2021
use serde::ser::{Serialize, SerializeTupleStruct, Serializer};

struct Rgb(u8, u8, u8);

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ts = serializer.serialize_tuple_struct("Rgb", 3)?;
        ts.serialize_field(&self.0)?;
        ts.serialize_field(&self.1)?;
        ts.serialize_field(&self.2)?;
        ts.end()
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeTupleStruct` for a basic JSON data format.


#### Associated Types

- `type Ok`

- `type Error: 1`

#### Required Methods

- `fn serialize_field<T>(&mut self, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a tuple struct field.

- `fn end(self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a tuple struct.

#### Implementors

- [`Impossible`](impossible/index.md#impossible)

### `SerializeTupleVariant`

```rust
trait SerializeTupleVariant { ... }
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:1739-1753`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L1739-L1753)*

Returned from `Serializer::serialize_tuple_variant`.

# Example use

```edition2021
use serde::ser::{Serialize, SerializeTupleVariant, Serializer};

enum E {
    T(u8, u8),
    U(String, u32, u32),
}

impl Serialize for E {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            E::T(ref a, ref b) => {
                let mut tv = serializer.serialize_tuple_variant("E", 0, "T", 2)?;
                tv.serialize_field(a)?;
                tv.serialize_field(b)?;
                tv.end()
            }
            E::U(ref a, ref b, ref c) => {
                let mut tv = serializer.serialize_tuple_variant("E", 1, "U", 3)?;
                tv.serialize_field(a)?;
                tv.serialize_field(b)?;
                tv.serialize_field(c)?;
                tv.end()
            }
        }
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeTupleVariant` for a basic JSON data format.


#### Associated Types

- `type Ok`

- `type Error: 1`

#### Required Methods

- `fn serialize_field<T>(&mut self, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a tuple variant field.

- `fn end(self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a tuple variant.

#### Implementors

- [`Impossible`](impossible/index.md#impossible)

### `SerializeMap`

```rust
trait SerializeMap { ... }
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:1811-1866`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L1811-L1866)*

Returned from `Serializer::serialize_map`.

# Example use

```edition2021
use std::marker::PhantomData;

struct HashMap<K, V>(PhantomData<K>, PhantomData<V>);

impl<K, V> HashMap<K, V> {
    fn len(&self) -> usize {
        unimplemented!()
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Box<dyn Iterator<Item = (&'a K, &'a V)>>;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

use serde::ser::{Serialize, SerializeMap, Serializer};

impl<K, V> Serialize for HashMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeMap` for a basic JSON data format.


#### Associated Types

- `type Ok`

- `type Error: 1`

#### Required Methods

- `fn serialize_key<T>(&mut self, key: &T) -> Result<(), <Self as >::Error>`

  Serialize a map key.
  
  If possible, `Serialize` implementations are encouraged to use
  `serialize_entry` instead as it may be implemented more efficiently in
  some formats compared to a pair of calls to `serialize_key` and
  `serialize_value`.

- `fn serialize_value<T>(&mut self, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a map value.
  
  # Panics
  
  Calling `serialize_value` before `serialize_key` is incorrect and is
  allowed to panic or produce bogus results.

- `fn end(self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a map.

#### Provided Methods

- `fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), <Self as >::Error>`

  Serialize a map entry consisting of a key and a value.
  
  Some [`Serialize`](#serialize) types are not able to hold a key and value in memory
  at the same time so `SerializeMap` implementations are required to
  support `serialize_key` and `serialize_value` individually. The
  `serialize_entry` method allows serializers to optimize for the case
  where key and value are both available. [`Serialize`](#serialize) implementations
  are encouraged to use `serialize_entry` if possible.
  
  The default implementation delegates to `serialize_key` and
  `serialize_value`. This is appropriate for serializers that do not
  care about performance or are not able to optimize `serialize_entry` any
  better than this.
  
  

#### Implementors

- [`Impossible`](impossible/index.md#impossible)

### `SerializeStruct`

```rust
trait SerializeStruct { ... }
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:1907-1930`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L1907-L1930)*

Returned from `Serializer::serialize_struct`.

# Example use

```edition2021
use serde::ser::{Serialize, SerializeStruct, Serializer};

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut rgb = serializer.serialize_struct("Rgb", 3)?;
        rgb.serialize_field("r", &self.r)?;
        rgb.serialize_field("g", &self.g)?;
        rgb.serialize_field("b", &self.b)?;
        rgb.end()
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeStruct` for a basic JSON data format.


#### Associated Types

- `type Ok`

- `type Error: 1`

#### Required Methods

- `fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a struct field.

- `fn end(self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a struct.

#### Provided Methods

- `fn skip_field(&mut self, key: &'static str) -> Result<(), <Self as >::Error>`

  Indicate that a struct field has been skipped.
  
  The default implementation does nothing.

#### Implementors

- [`Impossible`](impossible/index.md#impossible)

### `SerializeStructVariant`

```rust
trait SerializeStructVariant { ... }
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:1977-2000`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L1977-L2000)*

Returned from `Serializer::serialize_struct_variant`.

# Example use

```edition2021
use serde::ser::{Serialize, SerializeStructVariant, Serializer};

enum E {
    S { r: u8, g: u8, b: u8 },
}

impl Serialize for E {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            E::S {
                ref r,
                ref g,
                ref b,
            } => {
                let mut sv = serializer.serialize_struct_variant("E", 0, "S", 3)?;
                sv.serialize_field("r", r)?;
                sv.serialize_field("g", g)?;
                sv.serialize_field("b", b)?;
                sv.end()
            }
        }
    }
}
```

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SerializeStructVariant` for a basic JSON data format.


#### Associated Types

- `type Ok`

- `type Error: 1`

#### Required Methods

- `fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a struct variant field.

- `fn end(self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a struct variant.

#### Provided Methods

- `fn skip_field(&mut self, key: &'static str) -> Result<(), <Self as >::Error>`

  Indicate that a struct variant field has been skipped.
  
  The default implementation does nothing.

#### Implementors

- [`Impossible`](impossible/index.md#impossible)

## Functions

### `iterator_len_hint`

```rust
fn iterator_len_hint<I>(iter: &I) -> Option<usize>
where
    I: Iterator
```

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:2002-2010`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L2002-L2010)*

## Macros

### `declare_error_trait!`

*Defined in [`serde_core-1.0.228/src/ser/mod.rs:130-189`](../../../.source_1765894658/serde_core-1.0.228/src/ser/mod.rs#L130-L189)*

