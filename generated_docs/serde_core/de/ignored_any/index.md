*[serde_core](../../index.md) / [de](../index.md) / [ignored_any](index.md)*

---

# Module `ignored_any`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IgnoredAny`](#ignoredany) | struct | An efficient way of discarding data from a deserializer. |

## Structs

### `IgnoredAny`

```rust
struct IgnoredAny;
```

*Defined in [`serde_core-1.0.228/src/de/ignored_any.rs:111`](../../../../.source_1765521767/serde_core-1.0.228/src/de/ignored_any.rs#L111)*

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

fn example<'de, D>(deserializer: D) -> Result<(), D::Error>
where
    D: Deserializer<'de>,
{
// Deserialize only the sequence element at index 3 from this deserializer.
// The element at index 3 is required to be a string. Elements before and
// after index 3 are allowed to be of any type.
let s: String = NthElement::new(3).deserialize(deserializer)?;
    Ok(())
}
```

#### Trait Implementations

##### `impl Clone for IgnoredAny`

- <span id="ignoredany-clone"></span>`fn clone(&self) -> IgnoredAny` — [`IgnoredAny`](#ignoredany)

##### `impl Copy for IgnoredAny`

##### `impl Debug for IgnoredAny`

- <span id="ignoredany-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for IgnoredAny`

- <span id="ignoredany-default"></span>`fn default() -> IgnoredAny` — [`IgnoredAny`](#ignoredany)

##### `impl Deserialize for IgnoredAny`

- <span id="ignoredany-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<IgnoredAny, <D as >::Error>` — [`IgnoredAny`](#ignoredany), [`Deserializer`](../index.md#deserializer)

##### `impl DeserializeOwned for IgnoredAny`

##### `impl Expected for IgnoredAny`

- <span id="ignoredany-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PartialEq for IgnoredAny`

- <span id="ignoredany-eq"></span>`fn eq(&self, other: &IgnoredAny) -> bool` — [`IgnoredAny`](#ignoredany)

##### `impl StructuralPartialEq for IgnoredAny`

##### `impl Visitor for IgnoredAny`

- <span id="ignoredany-visitor-type-value"></span>`type Value = IgnoredAny`

- <span id="ignoredany-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="ignoredany-visit-bool"></span>`fn visit_bool<E>(self, x: bool) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-i64"></span>`fn visit_i64<E>(self, x: i64) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-i128"></span>`fn visit_i128<E>(self, x: i128) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-u64"></span>`fn visit_u64<E>(self, x: u64) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-u128"></span>`fn visit_u128<E>(self, x: u128) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-f64"></span>`fn visit_f64<E>(self, x: f64) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-str"></span>`fn visit_str<E>(self, s: &str) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-none"></span>`fn visit_none<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-some"></span>`fn visit_some<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-newtype-struct"></span>`fn visit_newtype_struct<D>(self, deserializer: D) -> Result<<Self as >::Value, <D as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-seq"></span>`fn visit_seq<A>(self, seq: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-map"></span>`fn visit_map<A>(self, map: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-bytes"></span>`fn visit_bytes<E>(self, bytes: &[u8]) -> Result<<Self as >::Value, E>` — [`Visitor`](../index.md#visitor)

- <span id="ignoredany-visit-enum"></span>`fn visit_enum<A>(self, data: A) -> Result<<Self as >::Value, <A as >::Error>` — [`Visitor`](../index.md#visitor)

