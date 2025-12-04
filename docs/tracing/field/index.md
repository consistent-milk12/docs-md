*[tracing](../index.md) / [field](index.md)*

---

# Module `field`

`Span` and `Event` key-value data.

Spans and events may be annotated with key-value data, referred to as _fields_.
These fields consist of a mapping from a key (corresponding to
a `&str` but represented internally as an array index) to a [`Value`](../../serde_json/serde_json/value/index.md).

# `Value`s and `Subscriber`s

`Subscriber`s consume `Value`s as fields attached to [span](#span)s or [`Event`](../../tracing_core/tracing_core/event/index.md)s.
The set of field keys on a given span or event is defined on its [`Metadata`](../../log/log/index.md).
When a span is created, it provides [`Attributes`](../../tracing_core/tracing_core/span/index.md) to the `Subscriber`'s
[`new_span`](#new-span) method, containing any fields whose values were provided when
the span was created; and may call the `Subscriber`'s [`record`](#record) method
with additional [`Record`](../../log/log/index.md)s if values are added for more of its fields.
Similarly, the [`Event`](../../tracing_core/tracing_core/event/index.md) type passed to the subscriber's [`event`](tracing/index.md) method
will contain any fields attached to each event.

`tracing` represents values as either one of a set of Rust primitives
(`i64`, `u64`, `f64`, `bool`, and `&str`) or using a `fmt::Display` or
`fmt::Debug` implementation. `Subscriber`s are provided these primitive
value types as `dyn Value` trait objects.

These trait objects can be formatted using `fmt::Debug`, but may also be
recorded as typed data by calling the [`Value::record`](#record) method on these
trait objects with a _visitor_ implementing the [`Visit`](../../tracing_core/tracing_core/field/index.md) trait. This trait
represents the behavior used to record values of various types. For example,
an implementation of `Visit` might record integers by incrementing counters
for their field names rather than printing them.


# Using `valuable`

`tracing`'s [`Value`](../../serde_json/serde_json/value/index.md) trait is intentionally minimalist: it supports only a small
number of Rust primitives as typed values, and only permits recording
user-defined types with their [`fmt::Debug`](#debug) or [`fmt::Display`](#display)
implementations. However, there are some cases where it may be useful to record
nested values (such as arrays, `Vec`s, or `HashMap`s containing values), or
user-defined `struct` and `enum` types without having to format them as
unstructured text.

To address `Value`'s limitations, `tracing` offers experimental support for
the [`valuable`](#valuable) crate, which provides object-safe inspection of structured
values. User-defined types can implement the [`valuable::Valuable`](#valuable) trait,
and be recorded as a `tracing` field by calling their [`as_value`](#as-value) method.
If the [`Subscriber`](../../tracing_core/tracing_core/subscriber/index.md) also supports the `valuable` crate, it can
then visit those types fields as structured values using `valuable`.

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: <code>valuable</code> support is an
    <a href = "../index.html#unstable-features">unstable feature</a>. See
    the documentation on unstable features for details on how to enable it.
</pre>

For example:
```ignore
// Derive `Valuable` for our types:
use valuable::Valuable;

#[derive(Clone, Debug, Valuable)]
struct User {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Clone, Debug, Valuable)]
struct Address {
    country: String,
    city: String,
    street: String,
}

let user = User {
    name: "Arwen Undomiel".to_string(),
    age: 3000,
    address: Address {
        country: "Middle Earth".to_string(),
        city: "Rivendell".to_string(),
        street: "leafy lane".to_string(),
    },
};

// Recording `user` as a `valuable::Value` will allow the `tracing` subscriber
// to traverse its fields as a nested, typed structure:
tracing::info!(current_user = user.as_value());
```

Alternatively, the [`valuable()`](#valuable) function may be used to convert a type
implementing [`Valuable`](#valuable) into a `tracing` field value.

When the `valuable` feature is enabled, the [`Visit`](../../tracing_core/tracing_core/field/index.md) trait will include an
optional [`record_value`](#record-value) method. `Visit` implementations that wish to
record `valuable` values can implement this method with custom behavior.
If a visitor does not implement `record_value`, the [`valuable::Value`](#value) will
be forwarded to the visitor's [`record_debug`](#record-debug) method.









[span](#span): mod@crate::span








## Traits

### `AsField`

```rust
trait AsField: crate::sealed::Sealed { ... }
```

Trait implemented to allow a type to be used as a field key.

<pre class="ignore" style="white-space:normal;font:inherit;">
<strong>Note</strong>: Although this is implemented for both the
<a href="./struct.Field.html"><code>Field</code></a> type <em>and</em> any
type that can be borrowed as an <code>&str</code>, only <code>Field</code>
allows <em>O</em>(1) access.
Indexing a field with a string results in an iterative search that performs
string comparisons. Thus, if possible, once the key for a field is known, it
should be used whenever possible.
</pre>

#### Required Methods

- `fn as_field(self: &Self, metadata: &Metadata<'_>) -> Option<Field>`

  Attempts to convert `&self` into a `Field` with the specified `metadata`.

