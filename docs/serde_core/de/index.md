*[serde_core](../index.md) / [de](index.md)*

---

# Module `de`

Generic data structure deserialization framework.

The two most important traits in this module are [`Deserialize`](de/index.md) and
[`Deserializer`](de/index.md).

 - **A type that implements `Deserialize` is a data structure** that can be
   deserialized from any data format supported by Serde, and conversely
 - **A type that implements `Deserializer` is a data format** that can
   deserialize any data structure supported by Serde.

# The Deserialize trait

Serde provides [`Deserialize`](de/index.md) implementations for many Rust primitive and
standard library types. The complete list is below. All of these can be
deserialized using Serde out of the box.

Additionally, Serde provides a procedural macro called [`serde_derive`](../../serde_derive/index.md) to
automatically generate [`Deserialize`](de/index.md) implementations for structs and enums
in your program. See the [derive section of the manual] for how to use this.

In rare cases it may be necessary to implement [`Deserialize`](de/index.md) manually for
some type in your program. See the [Implementing `Deserialize`] section of
the manual for more about this.

Third-party crates may provide [`Deserialize`](de/index.md) implementations for types
that they expose. For example the [`linked-hash-map`](#linked-hash-map) crate provides a
[`LinkedHashMap<K, V>`](#linkedhashmap) type that is deserializable by Serde because the
crate provides an implementation of [`Deserialize`](de/index.md) for it.

# The Deserializer trait

[`Deserializer`](de/index.md) implementations are provided by third-party crates, for
example [`serde_json`](../../serde_json/index.md), [`serde_yaml`](#serde-yaml) and [`postcard`](#postcard).

A partial list of well-maintained formats is given on the [Serde
website][data formats].

# Implementations of Deserialize provided by Serde

This is a slightly different set of types than what is supported for
serialization. Some types can be serialized by Serde but not deserialized.
One example is `OsStr`.

 - **Primitive types**:
   - bool
   - i8, i16, i32, i64, i128, isize
   - u8, u16, u32, u64, u128, usize
   - f32, f64
   - char
 - **Compound types**:
   - \[T; 0\] through \[T; 32\]
   - tuples up to size 16
 - **Common standard library types**:
   - String
   - Option\<T\>
   - Result\<T, E\>
   - PhantomData\<T\>
 - **Wrapper types**:
   - Box\<T\>
   - Box\<\[T\]\>
   - Box\<str\>
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
 - **Zero-copy types**:
   - &str
   - &\[u8\]
 - **FFI types**:
   - CString
   - Box\<CStr\>
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

[Implementing `Deserialize`]: https://serde.rs/impl-deserialize.html








[derive section of the manual]: https://serde.rs/derive.html
[data formats]: https://serde.rs/#data-formats

## Modules

- [`value`](value/index.md) - Building blocks for deserializing basic values using the `IntoDeserializer`

## Structs

### `IgnoredAny`

```rust
struct IgnoredAny;
```

An efficient way of discarding data from a deserializer.

Think of this like `serde_json::Value` in that it can be deserialized from
any type, except that it does not store any information about the data that
gets deserialized.

```edition2021
use serde::de::{
    self, Deserialize, DeserializeSeed, Deserializer, IgnoredAny, SeqAccess, Visitor,
};
use std::fmt;
use std::marker::PhantomData;

/// A seed that can be used to deserialize only the `n`th element of a sequence
/// while efficiently discarding elements of any type before or after index `n`.
///
/// For example to deserialize only the element at index 3:
///
/// ```
/// NthElement::new(3).deserialize(deserializer)
/// ```
pub struct NthElement<T> {
    n: usize,
    marker: PhantomData<T>,
}

impl<T> NthElement<T> {
    pub fn new(n: usize) -> Self {
        NthElement {
            n: n,
            marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for NthElement<T>
where
    T: Deserialize<'de>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a sequence in which we care about element {}",
            self.n
        )
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        // Skip over the first `n` elements.
        for i in 0..self.n {
            // It is an error if the sequence ends before we get to element `n`.
            if seq.next_element::<IgnoredAny>()?.is_none() {
                return Err(de::Error::invalid_length(i, &self));
            }
        }

        // Deserialize the one we care about.
        let nth = match seq.next_element()? {
            Some(nth) => nth,
            None => {
                return Err(de::Error::invalid_length(self.n, &self));
            }
        };

        // Skip over any remaining elements in the sequence after `n`.
        while let Some(IgnoredAny) = seq.next_element()? {
            // ignore
        }

        Ok(nth)
    }
}

impl<'de, T> DeserializeSeed<'de> for NthElement<T>
where
    T: Deserialize<'de>,
{
    type Value = T;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(self)
    }
}

# fn example<'de, D>(deserializer: D) -> Result<(), D::Error>
# where
#     D: Deserializer<'de>,
# {
// Deserialize only the sequence element at index 3 from this deserializer.
// The element at index 3 is required to be a string. Elements before and
// after index 3 are allowed to be of any type.
let s: String = NthElement::new(3).deserialize(deserializer)?;
#     Ok(())
# }
```

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

- `fn clone(self: &Self) -> IgnoredAny`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Expected<'de, T>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &IgnoredAny) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Visitor<'de>`

- `type Value = IgnoredAny`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- `fn visit_bool<E>(self: Self, x: bool) -> Result<<Self as >::Value, E>`

- `fn visit_i64<E>(self: Self, x: i64) -> Result<<Self as >::Value, E>`

- `fn visit_i128<E>(self: Self, x: i128) -> Result<<Self as >::Value, E>`

- `fn visit_u64<E>(self: Self, x: u64) -> Result<<Self as >::Value, E>`

- `fn visit_u128<E>(self: Self, x: u128) -> Result<<Self as >::Value, E>`

- `fn visit_f64<E>(self: Self, x: f64) -> Result<<Self as >::Value, E>`

- `fn visit_str<E>(self: Self, s: &str) -> Result<<Self as >::Value, E>`

- `fn visit_none<E>(self: Self) -> Result<<Self as >::Value, E>`

- `fn visit_some<D>(self: Self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>`

- `fn visit_newtype_struct<D>(self: Self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>`

- `fn visit_unit<E>(self: Self) -> Result<<Self as >::Value, E>`

- `fn visit_seq<A>(self: Self, seq: A) -> Result<<Self as >::Value, <A as >::Error>`

- `fn visit_map<A>(self: Self, map: A) -> Result<<Self as >::Value, <A as >::Error>`

- `fn visit_bytes<E>(self: Self, bytes: &[u8]) -> Result<<Self as >::Value, E>`

- `fn visit_enum<A>(self: Self, data: A) -> Result<<Self as >::Value, <A as >::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> IgnoredAny`

##### `impl Deserialize<'de>`

- `fn deserialize<D>(deserializer: D) -> Result<IgnoredAny, <D as >::Error>`

##### `impl DeserializeOwned<T>`

## Enums

### `Unexpected<'a>`

```rust
enum Unexpected<'a> {
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Char(char),
    Str(&'a str),
    Bytes(&'a [u8]),
    Unit,
    Option,
    NewtypeStruct,
    Seq,
    Map,
    Enum,
    UnitVariant,
    NewtypeVariant,
    TupleVariant,
    StructVariant,
    Other(&'a str),
}
```

`Unexpected` represents an unexpected invocation of any one of the `Visitor`
trait methods.

This is used as an argument to the `invalid_type`, `invalid_value`, and
`invalid_length` methods of the `Error` trait to build error messages.

```edition2021
# use std::fmt;
#
# use serde::de::{self, Unexpected, Visitor};
#
# struct Example;
#
# impl<'de> Visitor<'de> for Example {
#     type Value = ();
#
#     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
#         write!(formatter, "definitely not a boolean")
#     }
#
fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
where
    E: de::Error,
{
    Err(de::Error::invalid_type(Unexpected::Bool(v), &self))
}
# }
```

#### Variants

- **`Bool`**

  The input contained a boolean value that was not expected.

- **`Unsigned`**

  The input contained an unsigned integer `u8`, `u16`, `u32` or `u64` that
  was not expected.

- **`Signed`**

  The input contained a signed integer `i8`, `i16`, `i32` or `i64` that
  was not expected.

- **`Float`**

  The input contained a floating point `f32` or `f64` that was not
  expected.

- **`Char`**

  The input contained a `char` that was not expected.

- **`Str`**

  The input contained a `&str` or `String` that was not expected.

- **`Bytes`**

  The input contained a `&[u8](#u8)
  ` or `Vec<u8>` that was not expected.

- **`Unit`**

  The input contained a unit `()` that was not expected.

- **`Option`**

  The input contained an `Option<T>` that was not expected.

- **`NewtypeStruct`**

  The input contained a newtype struct that was not expected.

- **`Seq`**

  The input contained a sequence that was not expected.

- **`Map`**

  The input contained a map that was not expected.

- **`Enum`**

  The input contained an enum that was not expected.

- **`UnitVariant`**

  The input contained a unit variant that was not expected.

- **`NewtypeVariant`**

  The input contained a newtype variant that was not expected.

- **`TupleVariant`**

  The input contained a tuple variant that was not expected.

- **`StructVariant`**

  The input contained a struct variant that was not expected.

- **`Other`**

  A message stating what uncategorized thing the input contained that was
  not expected.
  
  The message should be a noun or noun phrase, not capitalized and without
  a period. An example message is "unoriginal superhero".

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

- `fn clone(self: &Self) -> Unexpected<'a>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<'a>`

##### `impl Display<'a>`

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq<'a>`

- `fn eq(self: &Self, other: &Unexpected<'a>) -> bool`

##### `impl StructuralPartialEq<'a>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `Error`

```rust
trait Error: Sized + StdError { ... }
```

The `Error` trait allows `Deserialize` implementations to create descriptive
error messages belonging to the `Deserializer` against which they are
currently running.

Every `Deserializer` declares an `Error` type that encompasses both
general-purpose deserialization errors as well as errors specific to the
particular deserialization format. For example the `Error` type of
`serde_json` can represent errors like an invalid JSON escape sequence or an
unterminated string literal, in addition to the error cases that are part of
this trait.

Most deserializers should only need to provide the `Error::custom` method
and inherit the default behavior for the other methods.

# Example implementation

The [example data format] presented on the website shows an error
type appropriate for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

#### Required Methods

- `fn custom<T>(msg: T) -> Self`

  Raised when there is general error when deserializing a type.

- `fn invalid_type(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self`

  Raised when a `Deserialize` receives a type different from what it was

- `fn invalid_value(unexp: Unexpected<'_>, exp: &dyn Expected) -> Self`

  Raised when a `Deserialize` receives a value of the right type but that

- `fn invalid_length(len: usize, exp: &dyn Expected) -> Self`

  Raised when deserializing a sequence or map and the input data contains

- `fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self`

  Raised when a `Deserialize` enum type received a variant with an

- `fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self`

  Raised when a `Deserialize` struct type received a field with an

- `fn missing_field(field: &'static str) -> Self`

  Raised when a `Deserialize` struct type expected to receive a required

- `fn duplicate_field(field: &'static str) -> Self`

  Raised when a `Deserialize` struct type received more than one of the

### `Expected`

```rust
trait Expected { ... }
```

`Expected` represents an explanation of what data a `Visitor` was expecting
to receive.

This is used as an argument to the `invalid_type`, `invalid_value`, and
`invalid_length` methods of the `Error` trait to build error messages. The
message should be a noun or noun phrase that completes the sentence "This
Visitor expects to receive ...", for example the message could be "an
integer between 0 and 64". The message should not be capitalized and should
not end with a period.

Within the context of a `Visitor` implementation, the `Visitor` itself
(`&self`) is an implementation of this trait.

```edition2021
# use serde::de::{self, Unexpected, Visitor};
# use std::fmt;
#
# struct Example;
#
# impl<'de> Visitor<'de> for Example {
#     type Value = ();
#
#     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
#         write!(formatter, "definitely not a boolean")
#     }
#
fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
where
    E: de::Error,
{
    Err(de::Error::invalid_type(Unexpected::Bool(v), &self))
}
# }
```

Outside of a `Visitor`, `&"..."` can be used.

```edition2021
# use serde::de::{self, Unexpected};
#
# fn example<E>() -> Result<(), E>
# where
#     E: de::Error,
# {
#     let v = true;
return Err(de::Error::invalid_type(
    Unexpected::Bool(v),
    &"a negative integer",
));
# }
```

#### Required Methods

- `fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

  Format an explanation of what data was being expected. Same signature as

### `Deserialize<'de>`

```rust
trait Deserialize<'de>: Sized { ... }
```

A **data structure** that can be deserialized from any data format supported
by Serde.

Serde provides `Deserialize` implementations for many Rust primitive and
standard library types. The complete list is `here`. All of these
can be deserialized using Serde out of the box.

Additionally, Serde provides a procedural macro called `serde_derive` to
automatically generate `Deserialize` implementations for structs and enums
in your program. See the [derive section of the manual][derive](#derive)
 for how to
use this.

In rare cases it may be necessary to implement `Deserialize` manually for
some type in your program. See the [Implementing
`Deserialize`][impl-deserialize] section of the manual for more about this.

Third-party crates may provide `Deserialize` implementations for types that
they expose. For example the `linked-hash-map` crate provides a
`LinkedHashMap<K, V>` type that is deserializable by Serde because the crate
provides an implementation of `Deserialize` for it.

[derive](#derive)
: https://serde.rs/derive.html
[impl-deserialize]: https://serde.rs/impl-deserialize.html

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by `Self` when deserialized. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

#### Required Methods

- `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

  Deserialize this value from the given Serde deserializer.

### `DeserializeOwned`

```rust
trait DeserializeOwned: Deserialize<'de> { ... }
```

A data structure that can be deserialized without borrowing any data from
the deserializer.

This is primarily useful for trait bounds on functions. For example a
`from_str` function may be able to deserialize a data structure that borrows
from the input string, but a `from_reader` function may only deserialize
owned data.

```edition2021
# use serde::de::{Deserialize, DeserializeOwned};
# use std::io::{Read, Result};
#
# trait Ignore {
fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>;

fn from_reader<R, T>(rdr: R) -> Result<T>
where
    R: Read,
    T: DeserializeOwned;
# }
```

# Lifetime

The relationship between `Deserialize` and `DeserializeOwned` in trait
bounds is explained in more detail on the page [Understanding deserializer
lifetimes].

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

### `DeserializeSeed<'de>`

```rust
trait DeserializeSeed<'de>: Sized { ... }
```

`DeserializeSeed` is the stateful form of the `Deserialize` trait. If you
ever find yourself looking for a way to pass data into a `Deserialize` impl,
this trait is the way to do it.

As one example of stateful deserialization consider deserializing a JSON
array into an existing buffer. Using the `Deserialize` trait we could
deserialize a JSON array into a `Vec<T>` but it would be a freshly allocated
`Vec<T>`; there is no way for `Deserialize` to reuse a previously allocated
buffer. Using `DeserializeSeed` instead makes this possible as in the
example code below.

The canonical API for stateless deserialization looks like this:

```edition2021
# use serde::Deserialize;
#
# enum Error {}
#
fn func<'de, T: Deserialize<'de>>() -> Result<T, Error>
# {
#     unimplemented!()
# }
```

Adjusting an API like this to support stateful deserialization is a matter
of accepting a seed as input:

```edition2021
# use serde::de::DeserializeSeed;
#
# enum Error {}
#
fn func_seed<'de, T: DeserializeSeed<'de>>(seed: T) -> Result<T::Value, Error>
# {
#     let _ = seed;
#     unimplemented!()
# }
```

In practice the majority of deserialization is stateless. An API expecting a
seed can be appeased by passing `std::marker::PhantomData` as a seed in the
case of stateless deserialization.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by `Self::Value` when deserialized. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example

Suppose we have JSON that looks like `[[1, 2], [3, 4, 5], [6]]` and we need
to deserialize it into a flat representation like `vec![1, 2, 3, 4, 5, 6]`.
Allocating a brand new `Vec<T>` for each subarray would be slow. Instead we
would like to allocate a single `Vec<T>` and then deserialize each subarray
into it. This requires stateful deserialization using the `DeserializeSeed`
trait.

```edition2021
use serde::de::{Deserialize, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;
use std::marker::PhantomData;

// A DeserializeSeed implementation that uses stateful deserialization to
// append array elements onto the end of an existing vector. The preexisting
// state ("seed") in this case is the Vec<T>. The `deserialize` method of
// `ExtendVec` will be traversing the inner arrays of the JSON input and
// appending each integer into the existing Vec.
struct ExtendVec<'a, T: 'a>(&'a mut Vec<T>);

impl<'de, 'a, T> DeserializeSeed<'de> for ExtendVec<'a, T>
where
    T: Deserialize<'de>,
{
    // The return type of the `deserialize` method. This implementation
    // appends onto an existing vector but does not create any new data
    // structure, so the return type is ().
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Visitor implementation that will walk an inner array of the JSON
        // input.
        struct ExtendVecVisitor<'a, T: 'a>(&'a mut Vec<T>);

        impl<'de, 'a, T> Visitor<'de> for ExtendVecVisitor<'a, T>
        where
            T: Deserialize<'de>,
        {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an array of integers")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
            where
                A: SeqAccess<'de>,
            {
                // Decrease the number of reallocations if there are many elements
                if let Some(size_hint) = seq.size_hint() {
                    self.0.reserve(size_hint);
                }

                // Visit each element in the inner array and push it onto
                // the existing vector.
                while let Some(elem) = seq.next_element()? {
                    self.0.push(elem);
                }
                Ok(())
            }
        }

        deserializer.deserialize_seq(ExtendVecVisitor(self.0))
    }
}

// Visitor implementation that will walk the outer array of the JSON input.
struct FlattenedVecVisitor<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for FlattenedVecVisitor<T>
where
    T: Deserialize<'de>,
{
    // This Visitor constructs a single Vec<T> to hold the flattened
    // contents of the inner arrays.
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an array of arrays")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Vec<T>, A::Error>
    where
        A: SeqAccess<'de>,
    {
        // Create a single Vec to hold the flattened contents.
        let mut vec = Vec::new();

        // Each iteration through this loop is one inner array.
        while let Some(()) = seq.next_element_seed(ExtendVec(&mut vec))? {
            // Nothing to do; inner array has been appended into `vec`.
        }

        // Return the finished vec.
        Ok(vec)
    }
}

# fn example<'de, D>(deserializer: D) -> Result<(), D::Error>
# where
#     D: Deserializer<'de>,
# {
let visitor = FlattenedVecVisitor(PhantomData);
let flattened: Vec<u64> = deserializer.deserialize_seq(visitor)?;
#     Ok(())
# }
```

#### Required Methods

- `type Value`

- `fn deserialize<D>(self: Self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>`

  Equivalent to the more common `Deserialize::deserialize` method, except

### `Deserializer<'de>`

```rust
trait Deserializer<'de>: Sized { ... }
```

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

[Serde data model]: https://serde.rs/data-model.html

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed from the input when deserializing. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example implementation

The [example data format] presented on the website contains example code for
a basic JSON `Deserializer`.

[example data format]: https://serde.rs/data-format.html

#### Required Methods

- `type Error: 1`

- `fn deserialize_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Require the `Deserializer` to figure out how to drive the visitor based

- `fn deserialize_bool<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `bool` value.

- `fn deserialize_i8<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `i8` value.

- `fn deserialize_i16<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `i16` value.

- `fn deserialize_i32<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `i32` value.

- `fn deserialize_i64<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `i64` value.

- `fn deserialize_i128<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `i128` value.

- `fn deserialize_u8<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `u8` value.

- `fn deserialize_u16<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `u16` value.

- `fn deserialize_u32<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `u32` value.

- `fn deserialize_u64<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `u64` value.

- `fn deserialize_u128<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an `u128` value.

- `fn deserialize_f32<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `f32` value.

- `fn deserialize_f64<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `f64` value.

- `fn deserialize_char<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a `char` value.

- `fn deserialize_str<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a string value and does

- `fn deserialize_string<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a string value and would

- `fn deserialize_bytes<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a byte array and does not

- `fn deserialize_byte_buf<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a byte array and would

- `fn deserialize_option<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an optional value.

- `fn deserialize_unit<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a unit value.

- `fn deserialize_unit_struct<V>(self: Self, name: &'static str, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a unit struct with a

- `fn deserialize_newtype_struct<V>(self: Self, name: &'static str, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a newtype struct with a

- `fn deserialize_seq<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a sequence of values.

- `fn deserialize_tuple<V>(self: Self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a sequence of values and

- `fn deserialize_tuple_struct<V>(self: Self, name: &'static str, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a tuple struct with a

- `fn deserialize_map<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a map of key-value pairs.

- `fn deserialize_struct<V>(self: Self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting a struct with a particular

- `fn deserialize_enum<V>(self: Self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting an enum value with a

- `fn deserialize_identifier<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type is expecting the name of a struct

- `fn deserialize_ignored_any<V>(self: Self, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Hint that the `Deserialize` type needs to deserialize a value whose type

- `fn is_human_readable(self: &Self) -> bool`

  Determine whether `Deserialize` implementations should expect to

### `Visitor<'de>`

```rust
trait Visitor<'de>: Sized { ... }
```

This trait represents a visitor that walks through a deserializer.

# Lifetime

The `'de` lifetime of this trait is the requirement for lifetime of data
that may be borrowed by `Self::Value`. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example

```edition2021
# use serde::de::{self, Unexpected, Visitor};
# use std::fmt;
#
/// A visitor that deserializes a long string - a string containing at least
/// some minimum number of bytes.
struct LongString {
    min: usize,
}

impl<'de> Visitor<'de> for LongString {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string containing at least {} bytes", self.min)
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if s.len() >= self.min {
            Ok(s.to_owned())
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}
```

#### Required Methods

- `type Value`

- `fn expecting(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

  Format a message stating what data this Visitor expects to receive.

- `fn visit_bool<E>(self: Self, v: bool) -> Result<<Self as >::Value, E>`

  The input contains a boolean.

- `fn visit_i8<E>(self: Self, v: i8) -> Result<<Self as >::Value, E>`

  The input contains an `i8`.

- `fn visit_i16<E>(self: Self, v: i16) -> Result<<Self as >::Value, E>`

  The input contains an `i16`.

- `fn visit_i32<E>(self: Self, v: i32) -> Result<<Self as >::Value, E>`

  The input contains an `i32`.

- `fn visit_i64<E>(self: Self, v: i64) -> Result<<Self as >::Value, E>`

  The input contains an `i64`.

- `fn visit_i128<E>(self: Self, v: i128) -> Result<<Self as >::Value, E>`

  The input contains a `i128`.

- `fn visit_u8<E>(self: Self, v: u8) -> Result<<Self as >::Value, E>`

  The input contains a `u8`.

- `fn visit_u16<E>(self: Self, v: u16) -> Result<<Self as >::Value, E>`

  The input contains a `u16`.

- `fn visit_u32<E>(self: Self, v: u32) -> Result<<Self as >::Value, E>`

  The input contains a `u32`.

- `fn visit_u64<E>(self: Self, v: u64) -> Result<<Self as >::Value, E>`

  The input contains a `u64`.

- `fn visit_u128<E>(self: Self, v: u128) -> Result<<Self as >::Value, E>`

  The input contains a `u128`.

- `fn visit_f32<E>(self: Self, v: f32) -> Result<<Self as >::Value, E>`

  The input contains an `f32`.

- `fn visit_f64<E>(self: Self, v: f64) -> Result<<Self as >::Value, E>`

  The input contains an `f64`.

- `fn visit_char<E>(self: Self, v: char) -> Result<<Self as >::Value, E>`

  The input contains a `char`.

- `fn visit_str<E>(self: Self, v: &str) -> Result<<Self as >::Value, E>`

  The input contains a string. The lifetime of the string is ephemeral and

- `fn visit_borrowed_str<E>(self: Self, v: &'de str) -> Result<<Self as >::Value, E>`

  The input contains a string that lives at least as long as the

- `fn visit_string<E>(self: Self, v: String) -> Result<<Self as >::Value, E>`

  The input contains a string and ownership of the string is being given

- `fn visit_bytes<E>(self: Self, v: &[u8]) -> Result<<Self as >::Value, E>`

  The input contains a byte array. The lifetime of the byte array is

- `fn visit_borrowed_bytes<E>(self: Self, v: &'de [u8]) -> Result<<Self as >::Value, E>`

  The input contains a byte array that lives at least as long as the

- `fn visit_byte_buf<E>(self: Self, v: Vec<u8>) -> Result<<Self as >::Value, E>`

  The input contains a byte array and ownership of the byte array is being

- `fn visit_none<E>(self: Self) -> Result<<Self as >::Value, E>`

  The input contains an optional that is absent.

- `fn visit_some<D>(self: Self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>`

  The input contains an optional that is present.

- `fn visit_unit<E>(self: Self) -> Result<<Self as >::Value, E>`

  The input contains a unit `()`.

- `fn visit_newtype_struct<D>(self: Self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>`

  The input contains a newtype struct.

- `fn visit_seq<A>(self: Self, seq: A) -> Result<<Self as >::Value, <A as >::Error>`

  The input contains a sequence of elements.

- `fn visit_map<A>(self: Self, map: A) -> Result<<Self as >::Value, <A as >::Error>`

  The input contains a key-value map.

- `fn visit_enum<A>(self: Self, data: A) -> Result<<Self as >::Value, <A as >::Error>`

  The input contains an enum.

### `SeqAccess<'de>`

```rust
trait SeqAccess<'de> { ... }
```

Provides a `Visitor` access to each element of a sequence in the input.

This is a trait that a `Deserializer` passes to a `Visitor` implementation,
which deserializes each item in a sequence.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by deserialized sequence elements. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `SeqAccess` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

#### Required Methods

- `type Error: 1`

- `fn next_element_seed<T>(self: &mut Self, seed: T) -> Result<Option<<T as >::Value>, <Self as >::Error>`

  This returns `Ok(Some(value))` for the next value in the sequence, or

- `fn next_element<T>(self: &mut Self) -> Result<Option<T>, <Self as >::Error>`

  This returns `Ok(Some(value))` for the next value in the sequence, or

- `fn size_hint(self: &Self) -> Option<usize>`

  Returns the number of elements remaining in the sequence, if known.

### `MapAccess<'de>`

```rust
trait MapAccess<'de> { ... }
```

Provides a `Visitor` access to each entry of a map in the input.

This is a trait that a `Deserializer` passes to a `Visitor` implementation.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by deserialized map entries. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `MapAccess` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

#### Required Methods

- `type Error: 1`

- `fn next_key_seed<K>(self: &mut Self, seed: K) -> Result<Option<<K as >::Value>, <Self as >::Error>`

  This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`

- `fn next_value_seed<V>(self: &mut Self, seed: V) -> Result<<V as >::Value, <Self as >::Error>`

  This returns a `Ok(value)` for the next value in the map.

- `fn next_entry_seed<K, V>(self: &mut Self, kseed: K, vseed: V) -> Result<Option<(<K as >::Value, <V as >::Value)>, <Self as >::Error>`

  This returns `Ok(Some((key, value)))` for the next (key-value) pair in

- `fn next_key<K>(self: &mut Self) -> Result<Option<K>, <Self as >::Error>`

  This returns `Ok(Some(key))` for the next key in the map, or `Ok(None)`

- `fn next_value<V>(self: &mut Self) -> Result<V, <Self as >::Error>`

  This returns a `Ok(value)` for the next value in the map.

- `fn next_entry<K, V>(self: &mut Self) -> Result<Option<(K, V)>, <Self as >::Error>`

  This returns `Ok(Some((key, value)))` for the next (key-value) pair in

- `fn size_hint(self: &Self) -> Option<usize>`

  Returns the number of entries remaining in the map, if known.

### `EnumAccess<'de>`

```rust
trait EnumAccess<'de>: Sized { ... }
```

Provides a `Visitor` access to the data of an enum in the input.

`EnumAccess` is created by the `Deserializer` and passed to the
`Visitor` in order to identify which variant of an enum to deserialize.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by the deserialized enum variant. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `EnumAccess` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

#### Required Methods

- `type Error: 1`

- `type Variant: 1`

- `fn variant_seed<V>(self: Self, seed: V) -> Result<(<V as >::Value, <Self as >::Variant), <Self as >::Error>`

  `variant` is called to identify which variant to deserialize.

- `fn variant<V>(self: Self) -> Result<(V, <Self as >::Variant), <Self as >::Error>`

  `variant` is called to identify which variant to deserialize.

### `VariantAccess<'de>`

```rust
trait VariantAccess<'de>: Sized { ... }
```

`VariantAccess` is a visitor that is created by the `Deserializer` and
passed to the `Deserialize` to deserialize the content of a particular enum
variant.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed by the deserialized enum variant. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example implementation

The [example data format] presented on the website demonstrates an
implementation of `VariantAccess` for a basic JSON data format.

[example data format]: https://serde.rs/data-format.html

#### Required Methods

- `type Error: 1`

- `fn unit_variant(self: Self) -> Result<(), <Self as >::Error>`

  Called when deserializing a variant with no values.

- `fn newtype_variant_seed<T>(self: Self, seed: T) -> Result<<T as >::Value, <Self as >::Error>`

  Called when deserializing a variant with a single value.

- `fn newtype_variant<T>(self: Self) -> Result<T, <Self as >::Error>`

  Called when deserializing a variant with a single value.

- `fn tuple_variant<V>(self: Self, len: usize, visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Called when deserializing a tuple-like variant.

- `fn struct_variant<V>(self: Self, fields: &'static [&'static str], visitor: V) -> Result<<V as >::Value, <Self as >::Error>`

  Called when deserializing a struct-like variant.

### `IntoDeserializer<'de, E: Error>`

```rust
trait IntoDeserializer<'de, E: Error> { ... }
```

Converts an existing value into a `Deserializer` from which other values can
be deserialized.

# Lifetime

The `'de` lifetime of this trait is the lifetime of data that may be
borrowed from the resulting `Deserializer`. See the page [Understanding
deserializer lifetimes] for a more detailed explanation of these lifetimes.

[Understanding deserializer lifetimes]: https://serde.rs/lifetimes.html

# Example

```edition2021
use serde::de::{value, Deserialize, IntoDeserializer};
use serde_derive::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
enum Setting {
    On,
    Off,
}

impl FromStr for Setting {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
```

#### Required Methods

- `type Deserializer: 1`

- `fn into_deserializer(self: Self) -> <Self as >::Deserializer`

  Convert this value into a deserializer.

## Functions

