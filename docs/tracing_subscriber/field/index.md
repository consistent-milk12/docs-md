*[tracing_subscriber](../index.md) / [field](index.md)*

---

# Module `field`

Utilities for working with [`fields`](../../tracing_attributes/attr/kw/index.md) and [field visitors].



## Modules

- [`debug`](debug/index.md) - `MakeVisitor` wrappers for working with `fmt::Debug` fields.
- [`delimited`](delimited/index.md) - A `MakeVisitor` wrapper that separates formatted fields with a delimiter.
- [`display`](display/index.md) - `MakeVisitor` wrappers for working with `fmt::Display` fields.

## Traits

### `MakeVisitor<T>`

```rust
trait MakeVisitor<T> { ... }
```

Creates new [visitors].

A type implementing `MakeVisitor` represents a composable factory for types
implementing the [`Visit` trait][visitors]. The `MakeVisitor` trait defines
a single function, `make_visitor`, which takes in a `T`-typed `target` and
returns a type implementing `Visit` configured for that target. A target may
be a string, output stream, or data structure that the visitor will record
data to, configuration variables that determine the visitor's behavior, or
`()` when no input is required to produce a visitor.


#### Required Methods

- `type Visitor: 1`

- `fn make_visitor(self: &Self, target: T) -> <Self as >::Visitor`

  Make a new visitor for the provided `target`.

### `VisitOutput<Out>`

```rust
trait VisitOutput<Out>: Visit { ... }
```

A [`visitor`](../../regex_syntax/ast/visitor/index.md) that produces output once it has visited a set of fields.


#### Required Methods

- `fn finish(self: Self) -> Out`

  Completes the visitor, returning any output.

- `fn visit<R>(self: Self, fields: &R) -> Out`

  Visit a set of fields, and return the output of finishing the visitor

### `RecordFields`

```rust
trait RecordFields: crate::sealed::Sealed<RecordFieldsMarker> { ... }
```

Extension trait implemented by types which can be recorded by a [`visitor`](../../regex_syntax/ast/visitor/index.md).

This allows writing code that is generic over `tracing_core`'s
[`span::Attributes`][`attr`](../../clap_derive/attr/index.md), [`span::Record`][rec], and [`Event`](../../tracing_core/event/index.md)
types. These types all provide inherent `record` methods that allow a
visitor to record their fields, but there is no common trait representing this.

With `RecordFields`, we can write code like this:
```rust
use tracing_core::field::Visit;
use tracing_core::field::Field;
use tracing_subscriber::field::RecordFields;

struct MyVisitor {
    // ...
}
impl MyVisitor { fn new() -> Self { Self{} } }
impl Visit for MyVisitor {
    // ...
fn record_debug(&mut self, _: &Field, _: &dyn std::fmt::Debug) {}
}

fn record_with_my_visitor<R>(r: R)
where
    R: RecordFields,
{
    let mut visitor = MyVisitor::new();
    r.record(&mut visitor);
}
```




#### Required Methods

- `fn record(self: &Self, visitor: &mut dyn Visit)`

  Record all the fields in `self` with the provided `visitor`.

### `MakeOutput<T, Out>`

```rust
trait MakeOutput<T, Out>
where
    Self: MakeVisitor<T> + crate::sealed::Sealed<(T, Out)>,
    <Self as >::Visitor: VisitOutput<Out> { ... }
```

Extension trait implemented for all `MakeVisitor` implementations that
produce a visitor implementing `VisitOutput`.

#### Required Methods

- `fn visit_with<F>(self: &Self, target: T, fields: &F) -> Out`

  Visits all fields in `fields` with a new visitor constructed from

### `VisitWrite`

```rust
trait VisitWrite: VisitOutput<Result<(), io::Error>> { ... }
```

Extension trait implemented by visitors to indicate that they write to an
`io::Write` instance, and allow access to that writer.

#### Required Methods

- `fn writer(self: &mut Self) -> &mut dyn io::Write`

  Returns the writer that this visitor writes to.

### `VisitFmt`

```rust
trait VisitFmt: VisitOutput<fmt::Result> { ... }
```

Extension trait implemented by visitors to indicate that they write to a
`fmt::Write` instance, and allow access to that writer.

#### Required Methods

- `fn writer(self: &mut Self) -> &mut dyn fmt::Write`

  Returns the formatter that this visitor writes to.

### `MakeExt<T>`

```rust
trait MakeExt<T>
where
    Self: MakeVisitor<T> + Sized + crate::sealed::Sealed<MakeExtMarker<T>> { ... }
```

Extension trait providing `MakeVisitor` combinators.

#### Required Methods

- `fn debug_alt(self: Self) -> debug::Alt<Self>`

  Wraps `self` so that any `fmt::Debug` fields are recorded using the

- `fn display_messages(self: Self) -> display::Messages<Self>`

  Wraps `self` so that any string fields named "message" are recorded

- `fn delimited<D>(self: Self, delimiter: D) -> delimited::Delimited<D, Self>`

  Wraps `self` so that when fields are formatted to a writer, they are

