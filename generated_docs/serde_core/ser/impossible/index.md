*[serde_core](../../index.md) / [ser](../index.md) / [impossible](index.md)*

---

# Module `impossible`

This module contains `Impossible` serializer and its implementations.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Impossible`](#impossible) | struct | Helper type for implementing a `Serializer` that does not support serializing one of the compound types. |
| [`Void`](#void) | enum |  |

## Structs

### `Impossible<Ok, Error>`

```rust
struct Impossible<Ok, Error> {
    void: Void,
    ok: PhantomData<Ok>,
    error: PhantomData<Error>,
}
```

*Defined in [`serde_core-1.0.228/src/ser/impossible.rs:60-64`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impossible.rs#L60-L64)*

Helper type for implementing a `Serializer` that does not support
serializing one of the compound types.

This type cannot be instantiated, but implements every one of the traits
corresponding to the [`Serializer`](../index.md) compound types: [`SerializeSeq`](../index.md),
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

- <span id="impossible-type-ok"></span>`type Ok = Ok`

- <span id="impossible-type-error"></span>`type Error = Error`

- <span id="impossible-serialize-key"></span>`fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>`

- <span id="impossible-serialize-value"></span>`fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>`

- <span id="impossible-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeSeq for Impossible<Ok, Error>`

- <span id="impossible-type-ok"></span>`type Ok = Ok`

- <span id="impossible-type-error"></span>`type Error = Error`

- <span id="impossible-serialize-element"></span>`fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>`

- <span id="impossible-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeStruct for Impossible<Ok, Error>`

- <span id="impossible-type-ok"></span>`type Ok = Ok`

- <span id="impossible-type-error"></span>`type Error = Error`

- <span id="impossible-serialize-field"></span>`fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>`

- <span id="impossible-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeStructVariant for Impossible<Ok, Error>`

- <span id="impossible-type-ok"></span>`type Ok = Ok`

- <span id="impossible-type-error"></span>`type Error = Error`

- <span id="impossible-serialize-field"></span>`fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>`

- <span id="impossible-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeTuple for Impossible<Ok, Error>`

- <span id="impossible-type-ok"></span>`type Ok = Ok`

- <span id="impossible-type-error"></span>`type Error = Error`

- <span id="impossible-serialize-element"></span>`fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>`

- <span id="impossible-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeTupleStruct for Impossible<Ok, Error>`

- <span id="impossible-type-ok"></span>`type Ok = Ok`

- <span id="impossible-type-error"></span>`type Error = Error`

- <span id="impossible-serialize-field"></span>`fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>`

- <span id="impossible-end"></span>`fn end(self) -> Result<Ok, Error>`

##### `impl<Ok, Error> SerializeTupleVariant for Impossible<Ok, Error>`

- <span id="impossible-type-ok"></span>`type Ok = Ok`

- <span id="impossible-type-error"></span>`type Error = Error`

- <span id="impossible-serialize-field"></span>`fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>`

- <span id="impossible-end"></span>`fn end(self) -> Result<Ok, Error>`

## Enums

### `Void`

```rust
enum Void {
}
```

*Defined in [`serde_core-1.0.228/src/ser/impossible.rs:66`](../../../../.source_1765210505/serde_core-1.0.228/src/ser/impossible.rs#L66)*

