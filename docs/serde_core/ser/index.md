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

Additionally, Serde provides a procedural macro called [`serde_derive`](../../serde_derive/index.md) to
automatically generate [`Serialize`](#serialize) implementations for structs and enums
in your program. See the [derive section of the manual] for how to use this.

In rare cases it may be necessary to implement [`Serialize`](#serialize) manually for
some type in your program. See the [Implementing `Serialize`] section of the
manual for more about this.

Third-party crates may provide [`Serialize`](#serialize) implementations for types that
they expose. For example the [`linked-hash-map`](#linked-hash-map) crate provides a
[`LinkedHashMap<K, V>`](#linkedhashmap) type that is serializable by Serde because the crate
provides an implementation of [`Serialize`](#serialize) for it.

# The Serializer trait

[`Serializer`](#serializer) implementations are provided by third-party crates, for
example [`serde_json`](#serde-json), [`serde_yaml`](#serde-yaml) and [`postcard`](#postcard).

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












## Structs

### `Impossible<Ok, Error>`

```rust
struct Impossible<Ok, Error> {
    void: Void,
    ok: PhantomData<Ok>,
    error: PhantomData<Error>,
}
```

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

##### `impl<Ok, Error> SerializeMap for Impossible<Ok, Error>`

- `type Ok = Ok`

- `type Error = Error`

- `fn serialize_key<T>(self: &mut Self, key: &T) -> Result<(), Error>`

- `fn serialize_value<T>(self: &mut Self, value: &T) -> Result<(), Error>`

- `fn end(self: Self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeSeq for Impossible<Ok, Error>`

- `type Ok = Ok`

- `type Error = Error`

- `fn serialize_element<T>(self: &mut Self, value: &T) -> Result<(), Error>`

- `fn end(self: Self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeStruct for Impossible<Ok, Error>`

- `type Ok = Ok`

- `type Error = Error`

- `fn serialize_field<T>(self: &mut Self, key: &'static str, value: &T) -> Result<(), Error>`

- `fn end(self: Self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeStructVariant for Impossible<Ok, Error>`

- `type Ok = Ok`

- `type Error = Error`

- `fn serialize_field<T>(self: &mut Self, key: &'static str, value: &T) -> Result<(), Error>`

- `fn end(self: Self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeTuple for Impossible<Ok, Error>`

- `type Ok = Ok`

- `type Error = Error`

- `fn serialize_element<T>(self: &mut Self, value: &T) -> Result<(), Error>`

- `fn end(self: Self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeTupleStruct for Impossible<Ok, Error>`

- `type Ok = Ok`

- `type Error = Error`

- `fn serialize_field<T>(self: &mut Self, value: &T) -> Result<(), Error>`

- `fn end(self: Self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeTupleVariant for Impossible<Ok, Error>`

- `type Ok = Ok`

- `type Error = Error`

- `fn serialize_field<T>(self: &mut Self, value: &T) -> Result<(), Error>`

- `fn end(self: Self) -> Result<Ok, Error>`

## Traits

### `Error`

```rust
trait Error: Sized + StdError { ... }
```

Trait used by `Serialize` implementations to generically construct
errors belonging to the `Serializer` against which they are
currently running.

# Example implementation

The [example data format] presented on the website shows an error
type appropriate for a basic JSON data format.


#### Required Methods

- `fn custom<T>(msg: T) -> Self`

  Used when a [`Serialize`](#serialize) implementation encounters any error

### `Serialize`

```rust
trait Serialize { ... }
```

A **data structure** that can be serialized into any data format supported
by Serde.

Serde provides `Serialize` implementations for many Rust primitive and
standard library types. The complete list is `here`. All of
these can be serialized using Serde out of the box.

Additionally, Serde provides a procedural macro called [`serde_derive`](../../serde_derive/index.md) to
automatically generate `Serialize` implementations for structs and enums in
your program. See the [derive section of the manual] for how to use this.

In rare cases it may be necessary to implement `Serialize` manually for some
type in your program. See the [Implementing `Serialize`] section of the
manual for more about this.

Third-party crates may provide `Serialize` implementations for types that
they expose. For example the [`linked-hash-map`](#linked-hash-map) crate provides a
[`LinkedHashMap<K, V>`](#linkedhashmap) type that is serializable by Serde because the crate
provides an implementation of `Serialize` for it.






#### Required Methods

- `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

  Serialize this value into the given Serde serializer.

### `Serializer`

```rust
trait Serializer: Sized { ... }
```

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


#### Required Methods

- `type Ok`

- `type Error: 1`

- `type SerializeSeq: 1`

- `type SerializeTuple: 1`

- `type SerializeTupleStruct: 1`

- `type SerializeTupleVariant: 1`

- `type SerializeMap: 1`

- `type SerializeStruct: 1`

- `type SerializeStructVariant: 1`

- `fn serialize_bool(self: Self, v: bool) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `bool` value.

- `fn serialize_i8(self: Self, v: i8) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i8` value.

- `fn serialize_i16(self: Self, v: i16) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i16` value.

- `fn serialize_i32(self: Self, v: i32) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i32` value.

- `fn serialize_i64(self: Self, v: i64) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i64` value.

- `fn serialize_i128(self: Self, v: i128) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `i128` value.

- `fn serialize_u8(self: Self, v: u8) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u8` value.

- `fn serialize_u16(self: Self, v: u16) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u16` value.

- `fn serialize_u32(self: Self, v: u32) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u32` value.

- `fn serialize_u64(self: Self, v: u64) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u64` value.

- `fn serialize_u128(self: Self, v: u128) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `u128` value.

- `fn serialize_f32(self: Self, v: f32) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `f32` value.

- `fn serialize_f64(self: Self, v: f64) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize an `f64` value.

- `fn serialize_char(self: Self, v: char) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a character.

- `fn serialize_str(self: Self, v: &str) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `&str`.

- `fn serialize_bytes(self: Self, v: &[u8]) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a chunk of raw byte data.

- `fn serialize_none(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a [`None`](#none) value.

- `fn serialize_some<T>(self: Self, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a [`Some(T)`](#somet) value.

- `fn serialize_unit(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a `()` value.

- `fn serialize_unit_struct(self: Self, name: &'static str) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a unit struct like `struct Unit` or `PhantomData<T>`.

- `fn serialize_unit_variant(self: Self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a unit variant like `E::A` in `enum E { A, B }`.

- `fn serialize_newtype_struct<T>(self: Self, name: &'static str, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a newtype struct like `struct Millimeters(u8)`.

- `fn serialize_newtype_variant<T>(self: Self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a newtype variant like `E::N` in `enum E { N(u8) }`.

- `fn serialize_seq(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeSeq, <Self as >::Error>`

  Begin to serialize a variably sized sequence. This call must be

- `fn serialize_tuple(self: Self, len: usize) -> Result<<Self as >::SerializeTuple, <Self as >::Error>`

  Begin to serialize a statically sized sequence whose length will be

- `fn serialize_tuple_struct(self: Self, name: &'static str, len: usize) -> Result<<Self as >::SerializeTupleStruct, <Self as >::Error>`

  Begin to serialize a tuple struct like `struct Rgb(u8, u8, u8)`. This

- `fn serialize_tuple_variant(self: Self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeTupleVariant, <Self as >::Error>`

  Begin to serialize a tuple variant like `E::T` in `enum E { T(u8, u8)

- `fn serialize_map(self: Self, len: Option<usize>) -> Result<<Self as >::SerializeMap, <Self as >::Error>`

  Begin to serialize a map. This call must be followed by zero or more

- `fn serialize_struct(self: Self, name: &'static str, len: usize) -> Result<<Self as >::SerializeStruct, <Self as >::Error>`

  Begin to serialize a struct like `struct Rgb { r: u8, g: u8, b: u8 }`.

- `fn serialize_struct_variant(self: Self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<<Self as >::SerializeStructVariant, <Self as >::Error>`

  Begin to serialize a struct variant like `E::S` in `enum E { S { r: u8,

- `fn collect_seq<I>(self: Self, iter: I) -> Result<<Self as >::Ok, <Self as >::Error>`

  Collect an iterator as a sequence.

- `fn collect_map<K, V, I>(self: Self, iter: I) -> Result<<Self as >::Ok, <Self as >::Error>`

  Collect an iterator as a map.

- `fn collect_str<T>(self: Self, value: &T) -> Result<<Self as >::Ok, <Self as >::Error>`

  Serialize a string produced by an implementation of `Display`.

- `fn is_human_readable(self: &Self) -> bool`

  Determine whether `Serialize` implementations should serialize in

### `SerializeSeq`

```rust
trait SerializeSeq { ... }
```

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


#### Required Methods

- `type Ok`

- `type Error: 1`

- `fn serialize_element<T>(self: &mut Self, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a sequence element.

- `fn end(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a sequence.

### `SerializeTuple`

```rust
trait SerializeTuple { ... }
```

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


#### Required Methods

- `type Ok`

- `type Error: 1`

- `fn serialize_element<T>(self: &mut Self, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a tuple element.

- `fn end(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a tuple.

### `SerializeTupleStruct`

```rust
trait SerializeTupleStruct { ... }
```

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


#### Required Methods

- `type Ok`

- `type Error: 1`

- `fn serialize_field<T>(self: &mut Self, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a tuple struct field.

- `fn end(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a tuple struct.

### `SerializeTupleVariant`

```rust
trait SerializeTupleVariant { ... }
```

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


#### Required Methods

- `type Ok`

- `type Error: 1`

- `fn serialize_field<T>(self: &mut Self, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a tuple variant field.

- `fn end(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a tuple variant.

### `SerializeMap`

```rust
trait SerializeMap { ... }
```

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


#### Required Methods

- `type Ok`

- `type Error: 1`

- `fn serialize_key<T>(self: &mut Self, key: &T) -> Result<(), <Self as >::Error>`

  Serialize a map key.

- `fn serialize_value<T>(self: &mut Self, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a map value.

- `fn serialize_entry<K, V>(self: &mut Self, key: &K, value: &V) -> Result<(), <Self as >::Error>`

  Serialize a map entry consisting of a key and a value.

- `fn end(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a map.

### `SerializeStruct`

```rust
trait SerializeStruct { ... }
```

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


#### Required Methods

- `type Ok`

- `type Error: 1`

- `fn serialize_field<T>(self: &mut Self, key: &'static str, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a struct field.

- `fn skip_field(self: &mut Self, key: &'static str) -> Result<(), <Self as >::Error>`

  Indicate that a struct field has been skipped.

- `fn end(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a struct.

### `SerializeStructVariant`

```rust
trait SerializeStructVariant { ... }
```

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


#### Required Methods

- `type Ok`

- `type Error: 1`

- `fn serialize_field<T>(self: &mut Self, key: &'static str, value: &T) -> Result<(), <Self as >::Error>`

  Serialize a struct variant field.

- `fn skip_field(self: &mut Self, key: &'static str) -> Result<(), <Self as >::Error>`

  Indicate that a struct variant field has been skipped.

- `fn end(self: Self) -> Result<<Self as >::Ok, <Self as >::Error>`

  Finish serializing a struct variant.

