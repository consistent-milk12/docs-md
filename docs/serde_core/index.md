# Crate `serde_core`

Serde is a framework for ***ser***ializing and ***de***serializing Rust data
structures efficiently and generically.

The `serde_core` crate contains Serde's trait definitions with **no support
for #\[derive()\]**.

In crates that derive an implementation of `Serialize` or `Deserialize`, you
must depend on the [`serde`](#serde) crate, not `serde_core`.

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

## Modules

- [`de`](de/index.md) - Generic data structure deserialization framework.
- [`ser`](ser/index.md) - Generic data structure serialization framework.

## Traits

## Macros

### `forward_to_deserialize_any!`

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




