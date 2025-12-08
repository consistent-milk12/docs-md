*[serde_core](../../index.md) / [ser](../index.md) / [impossible](index.md)*

---

# Module `impossible`

This module contains `Impossible` serializer and its implementations.

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
corresponding to the [`Serializer`](../../index.md) compound types: [`SerializeSeq`](../index.md),
[`SerializeTuple`](../index.md), [`SerializeTupleStruct`](../index.md), [`SerializeTupleVariant`](../index.md),
[`SerializeMap`](../index.md), [`SerializeStruct`](../index.md), and [`SerializeStructVariant`](../index.md).

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

## Enums

### `Void`

```rust
enum Void {
}
```

