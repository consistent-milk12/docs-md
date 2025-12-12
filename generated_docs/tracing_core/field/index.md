*[tracing_core](../index.md) / [field](index.md)*

---

# Module `field`

`Span` and `Event` key-value data.

Spans and events may be annotated with key-value data, known as _fields_.
These fields consist of a mapping from a key (corresponding to a `&str` but
represented internally as an array index) to a [`Value`](#value).

# `Value`s and `Subscriber`s

`Subscriber`s consume `Value`s as fields attached to [`span`](../span/index.md)s or [`Event`](../event/index.md)s.
The set of field keys on a given span or event is defined on its [`Metadata`](../metadata/index.md).
When a span is created, it provides [`Attributes`](../span/index.md) to the `Subscriber`'s
`new_span` method, containing any fields whose values were provided when
the span was created; and may call the `Subscriber`'s `record` method
with additional [`Record`](../span/index.md)s if values are added for more of its fields.
Similarly, the [`Event`](../event/index.md) type passed to the subscriber's [`event`](../event/index.md) method
will contain any fields attached to each event.

`tracing` represents values as either one of a set of Rust primitives
(`i64`, `u64`, `f64`, `i128`, `u128`, `bool`, and `&str`) or using a
`fmt::Display` or `fmt::Debug` implementation. `Subscriber`s are provided
these primitive value types as `dyn Value` trait objects.

These trait objects can be formatted using `fmt::Debug`, but may also be
recorded as typed data by calling the `Value::record` method on these
trait objects with a _visitor_ implementing the [`Visit`](#visit) trait. This trait
represents the behavior used to record values of various types. For example,
an implementation of `Visit` might record integers by incrementing counters
for their field names rather than printing them.


# Using `valuable`

`tracing`'s [`Value`](#value) trait is intentionally minimalist: it supports only a small
number of Rust primitives as typed values, and only permits recording
user-defined types with their [`fmt::Debug`](../../object/index.md) or [`fmt::Display`](../../miette_derive/index.md)
implementations. However, there are some cases where it may be useful to record
nested values (such as arrays, `Vec`s, or `HashMap`s containing values), or
user-defined `struct` and `enum` types without having to format them as
unstructured text.

To address `Value`'s limitations, `tracing` offers experimental support for
the `valuable` crate, which provides object-safe inspection of structured
values. User-defined types can implement the `valuable::Valuable` trait,
and be recorded as a `tracing` field by calling their `as_value` method.
If the [`Subscriber`](../subscriber/index.md) also supports the `valuable` crate, it can
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

Alternatively, the `valuable()` function may be used to convert a type
implementing `Valuable` into a `tracing` field value.

When the `valuable` feature is enabled, the [`Visit`](#visit) trait will include an
optional `record_value` method. `Visit` implementations that wish to
record `valuable` values can implement this method with custom behavior.
If a visitor does not implement `record_value`, the `valuable::Value` will
be forwarded to the visitor's `record_debug` method.














## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`Field`](#field)
  - [`Empty`](#empty)
  - [`FieldSet`](#fieldset)
  - [`ValueSet`](#valueset)
  - [`Iter`](#iter)
  - [`DisplayValue`](#displayvalue)
  - [`DebugValue`](#debugvalue)
  - [`HexBytes`](#hexbytes)
- [Enums](#enums)
  - [`Values`](#values)
- [Traits](#traits)
  - [`Visit`](#visit)
  - [`Value`](#value)
- [Functions](#functions)
  - [`display`](#display)
  - [`debug`](#debug)
- [Macros](#macros)
  - [`impl_values!`](#impl-values)
  - [`ty_to_nonzero!`](#ty-to-nonzero)
  - [`impl_one_value!`](#impl-one-value)
  - [`impl_value!`](#impl-value)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod |  |
| [`Field`](#field) | struct | An opaque key allowing _O_(1) access to a field in a `Span`'s key-value data. |
| [`Empty`](#empty) | struct | An empty field. |
| [`FieldSet`](#fieldset) | struct | Describes the fields present on a span. |
| [`ValueSet`](#valueset) | struct | A set of fields and values for a span. |
| [`Iter`](#iter) | struct | An iterator over a set of fields. |
| [`DisplayValue`](#displayvalue) | struct | A `Value` which serializes using `fmt::Display`. |
| [`DebugValue`](#debugvalue) | struct | A `Value` which serializes as a string using `fmt::Debug`. |
| [`HexBytes`](#hexbytes) | struct |  |
| [`Values`](#values) | enum |  |
| [`Visit`](#visit) | trait | Visits typed values. |
| [`Value`](#value) | trait | A field value of an erased type. |
| [`display`](#display) | fn | Wraps a type implementing `fmt::Display` as a `Value` that can be recorded using its `Display` implementation. |
| [`debug`](#debug) | fn | Wraps a type implementing `fmt::Debug` as a `Value` that can be recorded using its `Debug` implementation. |
| [`impl_values!`](#impl-values) | macro |  |
| [`ty_to_nonzero!`](#ty-to-nonzero) | macro |  |
| [`impl_one_value!`](#impl-one-value) | macro |  |
| [`impl_value!`](#impl-value) | macro |  |

## Modules

- [`private`](private/index.md)

## Structs

### `Field`

```rust
struct Field {
    i: usize,
    fields: FieldSet,
}
```

*Defined in [`tracing-core-0.1.35/src/field.rs:134-137`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L134-L137)*

An opaque key allowing _O_(1) access to a field in a `Span`'s key-value
data.

As keys are defined by the _metadata_ of a span, rather than by an
individual instance of a span, a key may be used to access the same field
across all instances of a given span with the same metadata. Thus, when a
subscriber observes a new span, it need only access a field by name _once_,
and use the key for that name for all other accesses.

#### Implementations

- <span id="field-callsite"></span>`fn callsite(&self) -> callsite::Identifier` — [`Identifier`](../callsite/index.md#identifier)

- <span id="field-name"></span>`fn name(&self) -> &'static str`

- <span id="field-index"></span>`fn index(&self) -> usize`

#### Trait Implementations

##### `impl AsRef for Field`

- <span id="field-as-ref"></span>`fn as_ref(&self) -> &str`

##### `impl Clone for Field`

- <span id="field-clone"></span>`fn clone(&self) -> Self`

##### `impl Debug for Field`

- <span id="field-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Field`

- <span id="field-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Field`

##### `impl Hash for Field`

- <span id="field-hash"></span>`fn hash<H>(&self, state: &mut H)`

##### `impl PartialEq for Field`

- <span id="field-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToString for Field`

- <span id="field-to-string"></span>`fn to_string(&self) -> String`

### `Empty`

```rust
struct Empty;
```

*Defined in [`tracing-core-0.1.35/src/field.rs:146`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L146)*

An empty field.

This can be used to indicate that the value of a field is not currently
present but will be recorded later.

When a field's value is `Empty`. it will not be recorded.

#### Trait Implementations

##### `impl Debug for Empty`

- <span id="empty-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Empty`

##### `impl PartialEq for Empty`

- <span id="empty-eq"></span>`fn eq(&self, other: &Empty) -> bool` — [`Empty`](#empty)

##### `impl Sealed for Empty`

##### `impl StructuralPartialEq for Empty`

##### `impl Value for Empty`

- <span id="empty-record"></span>`fn record(&self, _: &Field, _: &mut dyn Visit)` — [`Field`](#field), [`Visit`](#visit)

### `FieldSet`

```rust
struct FieldSet {
    names: &'static [&'static str],
    callsite: callsite::Identifier,
}
```

*Defined in [`tracing-core-0.1.35/src/field.rs:159-164`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L159-L164)*

Describes the fields present on a span.

## Equality

In well-behaved applications, two `FieldSet`s [initialized] with equal
[callsite identifiers] will have identical fields. Consequently, in release
builds, `FieldSet::eq` *only* checks that its arguments have equal
callsites. However, the equality of field names is checked in debug builds.



#### Fields

- **`names`**: `&'static [&'static str]`

  The names of each field on the described span.

- **`callsite`**: `callsite::Identifier`

  The callsite where the described span originates.

#### Implementations

- <span id="fieldset-new"></span>`const fn new(names: &'static [&'static str], callsite: callsite::Identifier) -> Self` — [`Identifier`](../callsite/index.md#identifier)

- <span id="fieldset-callsite"></span>`fn callsite(&self) -> callsite::Identifier` — [`Identifier`](../callsite/index.md#identifier)

- <span id="fieldset-field"></span>`fn field<Q: Borrow<str> + ?Sized>(&self, name: &Q) -> Option<Field>` — [`Field`](#field)

- <span id="fieldset-contains"></span>`fn contains(&self, field: &Field) -> bool` — [`Field`](#field)

- <span id="fieldset-iter"></span>`fn iter(&self) -> Iter` — [`Iter`](#iter)

- <span id="fieldset-len"></span>`fn len(&self) -> usize`

- <span id="fieldset-is-empty"></span>`fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl Debug for FieldSet`

- <span id="fieldset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FieldSet`

- <span id="fieldset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FieldSet`

##### `impl IntoIterator for &FieldSet`

- <span id="fieldset-intoiterator-type-intoiter"></span>`type IntoIter = Iter`

- <span id="fieldset-intoiterator-type-item"></span>`type Item = Field`

- <span id="fieldset-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl PartialEq for FieldSet`

- <span id="fieldset-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl ToString for FieldSet`

- <span id="fieldset-to-string"></span>`fn to_string(&self) -> String`

### `ValueSet<'a>`

```rust
struct ValueSet<'a> {
    values: Values<'a>,
    fields: &'a FieldSet,
}
```

*Defined in [`tracing-core-0.1.35/src/field.rs:167-170`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L167-L170)*

A set of fields and values for a span.

#### Implementations

- <span id="valueset-callsite"></span>`fn callsite(&self) -> callsite::Identifier` — [`Identifier`](../callsite/index.md#identifier)

- <span id="valueset-record"></span>`fn record(&self, visitor: &mut dyn Visit)` — [`Visit`](#visit)

- <span id="valueset-len"></span>`fn len(&self) -> usize`

- <span id="valueset-contains"></span>`fn contains(&self, field: &Field) -> bool` — [`Field`](#field)

- <span id="valueset-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="valueset-field-set"></span>`fn field_set(&self) -> &FieldSet` — [`FieldSet`](#fieldset)

#### Trait Implementations

##### `impl Debug for ValueSet<'_>`

- <span id="valueset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ValueSet<'_>`

- <span id="valueset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for ValueSet<'a>`

- <span id="valueset-to-string"></span>`fn to_string(&self) -> String`

### `Iter`

```rust
struct Iter {
    idxs: core::ops::Range<usize>,
    fields: FieldSet,
}
```

*Defined in [`tracing-core-0.1.35/src/field.rs:182-185`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L182-L185)*

An iterator over a set of fields.

#### Trait Implementations

##### `impl Debug for Iter`

- <span id="iter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for Iter`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Iter`

- <span id="iter-iterator-type-item"></span>`type Item = Field`

- <span id="iter-next"></span>`fn next(&mut self) -> Option<Field>` — [`Field`](#field)

### `DisplayValue<T: fmt::Display>`

```rust
struct DisplayValue<T: fmt::Display>(T);
```

*Defined in [`tracing-core-0.1.35/src/field.rs:360`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L360)*

A `Value` which serializes using `fmt::Display`.

Uses `record_debug` in the `Value` implementation to
avoid an unnecessary evaluation.

#### Trait Implementations

##### `impl<T: clone::Clone + fmt::Display> Clone for DisplayValue<T>`

- <span id="displayvalue-clone"></span>`fn clone(&self) -> DisplayValue<T>` — [`DisplayValue`](#displayvalue)

##### `impl<T: fmt::Display> Debug for DisplayValue<T>`

- <span id="displayvalue-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Display> Display for DisplayValue<T>`

- <span id="displayvalue-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Display> Sealed for DisplayValue<T>`

##### `impl<T> ToString for DisplayValue<T>`

- <span id="displayvalue-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T> Value for DisplayValue<T>`

- <span id="displayvalue-record"></span>`fn record(&self, key: &Field, visitor: &mut dyn Visit)` — [`Field`](#field), [`Visit`](#visit)

### `DebugValue<T: fmt::Debug>`

```rust
struct DebugValue<T: fmt::Debug>(T);
```

*Defined in [`tracing-core-0.1.35/src/field.rs:364`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L364)*

A `Value` which serializes as a string using `fmt::Debug`.

#### Trait Implementations

##### `impl<T: clone::Clone + fmt::Debug> Clone for DebugValue<T>`

- <span id="debugvalue-clone"></span>`fn clone(&self) -> DebugValue<T>` — [`DebugValue`](#debugvalue)

##### `impl<T: fmt::Debug> Debug for DebugValue<T>`

- <span id="debugvalue-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: fmt::Debug> Sealed for DebugValue<T>`

##### `impl<T> Value for DebugValue<T>`

- <span id="debugvalue-record"></span>`fn record(&self, key: &Field, visitor: &mut dyn Visit)` — [`Field`](#field), [`Visit`](#visit)

### `HexBytes<'a>`

```rust
struct HexBytes<'a>(&'a [u8]);
```

*Defined in [`tracing-core-0.1.35/src/field.rs:397`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L397)*

#### Trait Implementations

##### `impl Debug for HexBytes<'_>`

- <span id="hexbytes-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `Values<'a>`

```rust
enum Values<'a> {
    Explicit(&'a [(&'a Field, Option<&'a dyn Value>)]),
    All(&'a [Option<&'a dyn Value>]),
}
```

*Defined in [`tracing-core-0.1.35/src/field.rs:172-178`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L172-L178)*

#### Variants

- **`Explicit`**

  A set of field-value pairs. Fields may be for the wrong field set, some
  fields may be missing, and fields may be in any order.

- **`All`**

  A list of values corresponding exactly to the fields in a `FieldSet`.

## Traits

### `Visit`

```rust
trait Visit { ... }
```

*Defined in [`tracing-core-0.1.35/src/field.rs:275-341`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L275-L341)*

Visits typed values.

An instance of `Visit` ("a visitor") represents the logic necessary to
record field values of various types. When an implementor of [`Value`](#value) is
[recorded], it calls the appropriate method on the provided visitor to
indicate the type that value should be recorded as.

When a [`Subscriber`](../subscriber/index.md) implementation [records an `Event`] or a
[set of `Value`s added to a `Span`], it can pass an `&mut Visit` to the
`record` method on the provided [`ValueSet`](#valueset) or [`Event`](../event/index.md). This visitor
will then be used to record all the field-value pairs present on that
`Event` or `ValueSet`.

# Examples

A simple visitor that writes to a string might be implemented like so:
```rust
extern crate tracing_core as tracing;
use std::fmt::{self, Write};
use tracing::field::{Value, Visit, Field};
pub struct StringVisitor<'a> {
    string: &'a mut String,
}

impl<'a> Visit for StringVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        write!(self.string, "{} = {:?}; ", field.name(), value).unwrap();
    }
}
```
This visitor will format each recorded value using `fmt::Debug`, and
append the field name and formatted value to the provided string,
regardless of the type of the recorded value. When all the values have
been recorded, the `StringVisitor` may be dropped, allowing the string
to be printed or stored in some other data structure.

The `Visit` trait provides default implementations for `record_i64`,
`record_u64`, `record_bool`, `record_str`, and `record_error`, which simply
forward the recorded value to `record_debug`. Thus, `record_debug` is the
only method which a `Visit` implementation *must* implement. However,
visitors may override the default implementations of these functions in
order to implement type-specific behavior.

Additionally, when a visitor receives a value of a type it does not care
about, it is free to ignore those values completely. For example, a
visitor which only records numeric data might look like this:

```rust
extern crate tracing_core as tracing;
use std::fmt::{self, Write};
use tracing::field::{Value, Visit, Field};
pub struct SumVisitor {
    sum: i64,
}

impl Visit for SumVisitor {
    fn record_i64(&mut self, _field: &Field, value: i64) {
       self.sum += value;
    }

    fn record_u64(&mut self, _field: &Field, value: u64) {
        self.sum += value as i64;
    }

    fn record_debug(&mut self, _field: &Field, _value: &dyn fmt::Debug) {
        // Do nothing
    }
}
```

This visitor (which is probably not particularly useful) keeps a running
sum of all the numeric values it records, and ignores all other values. A
more practical example of recording typed values is presented in
`examples/counters.rs`, which demonstrates a very simple metrics system
implemented using `tracing`.

<div class="example-wrap" style="display:inline-block">
<pre class="ignore" style="white-space:normal;font:inherit;">
<strong>Note</strong>: The <code>record_error</code> trait method is only
available when the Rust standard library is present, as it requires the
<code>std::error::Error</code> trait.
</pre></div>






#### Required Methods

- `fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug)`

  Visit a value implementing `fmt::Debug`.

#### Provided Methods

- `fn record_f64(&mut self, field: &Field, value: f64)`

  Visit a double-precision floating point value.

- `fn record_i64(&mut self, field: &Field, value: i64)`

  Visit a signed 64-bit integer value.

- `fn record_u64(&mut self, field: &Field, value: u64)`

  Visit an unsigned 64-bit integer value.

- `fn record_i128(&mut self, field: &Field, value: i128)`

  Visit a signed 128-bit integer value.

- `fn record_u128(&mut self, field: &Field, value: u128)`

  Visit an unsigned 128-bit integer value.

- `fn record_bool(&mut self, field: &Field, value: bool)`

  Visit a boolean value.

- `fn record_str(&mut self, field: &Field, value: &str)`

  Visit a string value.

- `fn record_bytes(&mut self, field: &Field, value: &[u8])`

  Visit a byte slice.

- `fn record_error(&mut self, field: &Field, value: &dyn std::error::Error)`

  Records a type implementing `Error`.

#### Implementors

- `F`
- `fmt::DebugMap<'_, '_>`
- `fmt::DebugStruct<'_, '_>`

### `Value`

```rust
trait Value: crate::sealed::Sealed { ... }
```

*Defined in [`tracing-core-0.1.35/src/field.rs:350-353`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L350-L353)*

A field value of an erased type.

Implementors of `Value` may call the appropriate typed recording methods on
the [`visitor`](../../regex_syntax/ast/visitor/index.md) passed to their `record` method in order to indicate how
their data should be recorded.


#### Required Methods

- `fn record(&self, key: &Field, visitor: &mut dyn Visit)`

  Visits this value with the given `Visitor`.

#### Implementors

- [`DebugValue`](#debugvalue)
- [`DisplayValue`](#displayvalue)
- [`Empty`](#empty)
- `&'a T`
- `&'a mut T`
- `NonZeroI128`
- `NonZeroI16`
- `NonZeroI32`
- `NonZeroI64`
- `NonZeroI8`
- `NonZeroIsize`
- `NonZeroU128`
- `NonZeroU16`
- `NonZeroU32`
- `NonZeroU64`
- `NonZeroU8`
- `NonZeroUsize`
- `Option<T>`
- `Wrapping<T>`
- `[u8]`
- `alloc::boxed::Box<T>`
- `alloc::string::String`
- `bool`
- `dyn std::error::Error + Send + Sync`
- `dyn std::error::Error + Send`
- `dyn std::error::Error + Sync`
- `dyn std::error::Error`
- `f32`
- `f64`
- `fmt::Arguments<'_>`
- `i128`
- `i16`
- `i32`
- `i64`
- `i8`
- `isize`
- `str`
- `u128`
- `u16`
- `u32`
- `u64`
- `u8`
- `usize`

## Functions

### `display`

```rust
fn display<T>(t: T) -> DisplayValue<T>
where
    T: fmt::Display
```

*Defined in [`tracing-core-0.1.35/src/field.rs:368-373`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L368-L373)*

Wraps a type implementing `fmt::Display` as a `Value` that can be
recorded using its `Display` implementation.

### `debug`

```rust
fn debug<T>(t: T) -> DebugValue<T>
where
    T: fmt::Debug
```

*Defined in [`tracing-core-0.1.35/src/field.rs:377-382`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L377-L382)*

Wraps a type implementing `fmt::Debug` as a `Value` that can be
recorded using its `Debug` implementation.

## Macros

### `impl_values!`

*Defined in [`tracing-core-0.1.35/src/field.rs:442-448`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L442-L448)*

### `ty_to_nonzero!`

*Defined in [`tracing-core-0.1.35/src/field.rs:450-487`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L450-L487)*

### `impl_one_value!`

*Defined in [`tracing-core-0.1.35/src/field.rs:489-534`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L489-L534)*

### `impl_value!`

*Defined in [`tracing-core-0.1.35/src/field.rs:536-547`](../../../.source_1765210505/tracing-core-0.1.35/src/field.rs#L536-L547)*

