*[tracing_core](../index.md) / [field](index.md)*

---

# Module `field`

`Span` and `Event` key-value data.

Spans and events may be annotated with key-value data, known as _fields_.
These fields consist of a mapping from a key (corresponding to a `&str` but
represented internally as an array index) to a [`Value`](tracing_core/field/index.md).

# `Value`s and `Subscriber`s

`Subscriber`s consume `Value`s as fields attached to [span](#span)
s or [`Event`](tracing_core/event/index.md)s.
The set of field keys on a given span or event is defined on its [`Metadata`](tracing_core/metadata/index.md).
When a span is created, it provides [`Attributes`](tracing_core/span/index.md) to the `Subscriber`'s
[`new_span`](#new-span) method, containing any fields whose values were provided when
the span was created; and may call the `Subscriber`'s [`record`](#record) method
with additional [`Record`](tracing_core/span/index.md)s if values are added for more of its fields.
Similarly, the [`Event`](tracing_core/event/index.md) type passed to the subscriber's [`event`](tracing_core/event/index.md) method
will contain any fields attached to each event.

`tracing` represents values as either one of a set of Rust primitives
(`i64`, `u64`, `f64`, `i128`, `u128`, `bool`, and `&str`) or using a
`fmt::Display` or `fmt::Debug` implementation. `Subscriber`s are provided
these primitive value types as `dyn Value` trait objects.

These trait objects can be formatted using `fmt::Debug`, but may also be
recorded as typed data by calling the [`Value::record`](#record) method on these
trait objects with a _visitor_ implementing the [`Visit`](tracing_core/field/index.md) trait. This trait
represents the behavior used to record values of various types. For example,
an implementation of `Visit` might record integers by incrementing counters
for their field names rather than printing them.


# Using `valuable`

`tracing`'s [`Value`](tracing_core/field/index.md) trait is intentionally minimalist: it supports only a small
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
If the [`Subscriber`](tracing_core/subscriber/index.md) also supports the `valuable` crate, it can
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

When the `valuable` feature is enabled, the [`Visit`](tracing_core/field/index.md) trait will include an
optional [`record_value`](#record-value) method. `Visit` implementations that wish to
record `valuable` values can implement this method with custom behavior.
If a visitor does not implement `record_value`, the [`valuable::Value`](#value) will
be forwarded to the visitor's [`record_debug`](#record-debug) method.





[span](#span)
: super::span









## Structs

### `Field`

```rust
struct Field {
}
```

An opaque key allowing _O_(1) access to a field in a `Span`'s key-value
data.

As keys are defined by the _metadata_ of a span, rather than by an
individual instance of a span, a key may be used to access the same field
across all instances of a given span with the same metadata. Thus, when a
subscriber observes a new span, it need only access a field by name _once_,
and use the key for that name for all other accesses.

#### Implementations

- `fn callsite(self: &Self) -> callsite::Identifier`
  Returns an [`Identifier`] that uniquely identifies the [`Callsite`]

- `fn name(self: &Self) -> &'static str`
  Returns a string representing the name of the field.

- `fn index(self: &Self) -> usize`
  Returns the index of this field in its [`FieldSet`].

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl AsRef`

- `fn as_ref(self: &Self) -> &str`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Self`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl Hash`

- `fn hash<H>(self: &Self, state: &mut H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

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

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Empty`

```rust
struct Empty;
```

An empty field.

This can be used to indicate that the value of a field is not currently
present but will be recorded later.

When a field's value is `Empty`. it will not be recorded.

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

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Empty) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Value`

- `fn record(self: &Self, _: &Field, _: &mut dyn Visit)`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FieldSet`

```rust
struct FieldSet {
}
```

Describes the fields present on a span.

## Equality

In well-behaved applications, two `FieldSet`s [initialized](#initialized)
 with equal
[callsite identifiers] will have identical fields. Consequently, in release
builds, [`FieldSet::eq`](#eq) *only* checks that its arguments have equal
callsites. However, the equality of field names is checked in debug builds.

[initialized](#initialized)
: Self::new
[callsite identifiers]: callsite::Identifier

#### Implementations

- `const fn new(names: &'static [&'static str], callsite: callsite::Identifier) -> Self`
  Constructs a new `FieldSet` with the given array of field names and callsite.

- `fn field<Q: Borrow<str> + ?Sized>(self: &Self, name: &Q) -> Option<Field>`
  Returns the [`Field`] named `name`, or `None` if no such field exists.

- `fn contains(self: &Self, field: &Field) -> bool`
  Returns `true` if `self` contains the given `field`.

- `fn iter(self: &Self) -> Iter`
  Returns an iterator over the `Field`s in this `FieldSet`.

- `fn len(self: &Self) -> usize`
  Returns the number of fields in this `FieldSet`.

- `fn is_empty(self: &Self) -> bool`
  Returns whether or not this `FieldSet` has fields.

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

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ValueSet<'a>`

```rust
struct ValueSet<'a> {
}
```

A set of fields and values for a span.

#### Implementations

- `fn callsite(self: &Self) -> callsite::Identifier`
  Returns an [`Identifier`] that uniquely identifies the [`Callsite`]

- `fn record(self: &Self, visitor: &mut dyn Visit)`
  Visits all the fields in this `ValueSet` with the provided [visitor].

- `fn len(self: &Self) -> usize`
  Returns the number of fields in this `ValueSet` that would be visited

- `fn is_empty(self: &Self) -> bool`
  Returns true if this `ValueSet` contains _no_ values.

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

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Iter`

```rust
struct Iter {
}
```

An iterator over a set of fields.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator`

- `type Item = Field`

- `fn next(self: &mut Self) -> Option<Field>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `DisplayValue<T: fmt::Display>`

```rust
struct DisplayValue<T: fmt::Display>();
```

A `Value` which serializes using `fmt::Display`.

Uses `record_debug` in the `Value` implementation to
avoid an unnecessary evaluation.

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

##### `impl Clone<T: $crate::clone::Clone + fmt::Display>`

- `fn clone(self: &Self) -> DisplayValue<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display<T: fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

##### `impl Value<T>`

- `fn record(self: &Self, key: &Field, visitor: &mut dyn Visit)`

##### `impl Debug<T: fmt::Display>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DebugValue<T: fmt::Debug>`

```rust
struct DebugValue<T: fmt::Debug>();
```

A `Value` which serializes as a string using `fmt::Debug`.

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

##### `impl Clone<T: $crate::clone::Clone + fmt::Debug>`

- `fn clone(self: &Self) -> DebugValue<T>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

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

##### `impl Value<T>`

- `fn record(self: &Self, key: &Field, visitor: &mut dyn Visit)`

##### `impl Debug<T: fmt::Debug>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `Visit`

```rust
trait Visit { ... }
```

Visits typed values.

An instance of `Visit` ("a visitor") represents the logic necessary to
record field values of various types. When an implementor of [`Value`](tracing_core/field/index.md) is
[recorded](#recorded)
, it calls the appropriate method on the provided visitor to
indicate the type that value should be recorded as.

When a [`Subscriber`](tracing_core/subscriber/index.md) implementation [records an `Event`] or a
[set of `Value`s added to a `Span`], it can pass an `&mut Visit` to the
`record` method on the provided [`ValueSet`](tracing_core/field/index.md) or [`Event`](tracing_core/event/index.md). This visitor
will then be used to record all the field-value pairs present on that
`Event` or `ValueSet`.

# Examples

A simple visitor that writes to a string might be implemented like so:
```
# extern crate tracing_core as tracing;
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

```
# extern crate tracing_core as tracing;
# use std::fmt::{self, Write};
# use tracing::field::{Value, Visit, Field};
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

[recorded](#recorded)
: Value::record

[records an `Event`]: super::subscriber::Subscriber::event
[set of `Value`s added to a `Span`]: super::subscriber::Subscriber::record


#### Required Methods

- `fn record_f64(self: &mut Self, field: &Field, value: f64)`

  Visit a double-precision floating point value.

- `fn record_i64(self: &mut Self, field: &Field, value: i64)`

  Visit a signed 64-bit integer value.

- `fn record_u64(self: &mut Self, field: &Field, value: u64)`

  Visit an unsigned 64-bit integer value.

- `fn record_i128(self: &mut Self, field: &Field, value: i128)`

  Visit a signed 128-bit integer value.

- `fn record_u128(self: &mut Self, field: &Field, value: u128)`

  Visit an unsigned 128-bit integer value.

- `fn record_bool(self: &mut Self, field: &Field, value: bool)`

  Visit a boolean value.

- `fn record_str(self: &mut Self, field: &Field, value: &str)`

  Visit a string value.

- `fn record_bytes(self: &mut Self, field: &Field, value: &[u8])`

  Visit a byte slice.

- `fn record_error(self: &mut Self, field: &Field, value: &dyn std::error::Error)`

  Records a type implementing `Error`.

- `fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug)`

  Visit a value implementing `fmt::Debug`.

### `Value`

```rust
trait Value: crate::sealed::Sealed { ... }
```

A field value of an erased type.

Implementors of `Value` may call the appropriate typed recording methods on
the [visitor](#visitor)
 passed to their `record` method in order to indicate how
their data should be recorded.

[visitor](#visitor)
: Visit

#### Required Methods

- `fn record(self: &Self, key: &Field, visitor: &mut dyn Visit)`

  Visits this value with the given `Visitor`.

## Functions

### `display`

```rust
fn display<T>(t: T) -> DisplayValue<T>
where
    T: fmt::Display
```

Wraps a type implementing `fmt::Display` as a `Value` that can be
recorded using its `Display` implementation.

### `debug`

```rust
fn debug<T>(t: T) -> DebugValue<T>
where
    T: fmt::Debug
```

Wraps a type implementing `fmt::Debug` as a `Value` that can be
recorded using its `Debug` implementation.

