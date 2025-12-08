# Crate `pin_project_lite`

<!-- tidy:crate-doc:start -->
A lightweight version of [pin-project] written with declarative macros.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pin-project-lite = "0.2"
```

## Examples

`pin_project!` macro creates a projection type covering all the fields of
struct.

```rust
use std::pin::Pin;

use pin_project_lite::pin_project;

pin_project! {
    struct Struct<T, U> {
        #[pin]
        pinned: T,
        unpinned: U,
    }
}

impl<T, U> Struct<T, U> {
    fn method(self: Pin<&mut Self>) {
        let this = self.project();
        let _: Pin<&mut T> = this.pinned; // Pinned reference to the field
        let _: &mut U = this.unpinned; // Normal reference to the field
    }
}
```

To use `pin_project!` on enums, you need to name the projection type
returned from the method.

```rust
use std::pin::Pin;

use pin_project_lite::pin_project;

pin_project! {
    #[project = EnumProj]
    enum Enum<T, U> {
        Variant { #[pin] pinned: T, unpinned: U },
    }
}

impl<T, U> Enum<T, U> {
    fn method(self: Pin<&mut Self>) {
        match self.project() {
            EnumProj::Variant { pinned, unpinned } => {
                let _: Pin<&mut T> = pinned;
                let _: &mut U = unpinned;
            }
        }
    }
}
```

## [pin-project] vs pin-project-lite

Here are some similarities and differences compared to [pin-project].

### Similar: Safety

pin-project-lite guarantees safety in much the same way as [pin-project].
Both are completely safe unless you write other unsafe code.

### Different: Minimal design

This library does not tackle as expansive of a range of use cases as
[pin-project] does. If your use case is not already covered, please use
[pin-project].

### Different: No proc-macro related dependencies

This is the **only** reason to use this crate. However, **if you already
have proc-macro related dependencies in your crate's dependency graph, there
is no benefit from using this crate.** (Note: There is almost no difference
in the amount of code generated between [pin-project] and pin-project-lite.)

### Different: No useful error messages

This macro does not handle any invalid input. So error messages are not to
be useful in most cases. If you do need useful error messages, then upon
error you can pass the same input to [pin-project] to receive a helpful
description of the compile error.

### Different: No support for custom Unpin implementation

pin-project supports this by [`UnsafeUnpin`][unsafe-unpin]. (`!Unpin` is supported by both [pin-project][not-unpin] and [pin-project-lite][not-unpin-lite].)

### Different: No support for tuple structs and tuple variants

pin-project supports this.



<!-- tidy:crate-doc:end -->


## Macros

### `pin_project!`

A macro that creates a projection type covering all the fields of struct.

This macro creates a projection type according to the following rules:

- For the field that uses `#[pin]` attribute, makes the pinned reference to the field.
- For the other fields, makes the unpinned reference to the field.

And the following methods are implemented on the original type:

```rust
use std::pin::Pin;
type Projection<'a> = &'a ();
type ProjectionRef<'a> = &'a ();
trait Dox {
fn project(self: Pin<&mut Self>) -> Projection<'_>;
fn project_ref(self: Pin<&Self>) -> ProjectionRef<'_>;
}
```

By passing an attribute with the same name as the method to the macro,
you can name the projection type returned from the method. This allows you
to use pattern matching on the projected types.

```rust
use pin_project_lite::pin_project;
use std::pin::Pin;
pin_project! {
    #[project = EnumProj]
    enum Enum<T> {
        Variant { #[pin] field: T },
    }
}

impl<T> Enum<T> {
    fn method(self: Pin<&mut Self>) {
        let this: EnumProj<'_, T> = self.project();
        match this {
            EnumProj::Variant { field } => {
                let _: Pin<&mut T> = field;
            }
        }
    }
}
```

By passing the `#[project_replace = MyProjReplace]` attribute you may create an additional
method which allows the contents of `Pin<&mut Self>` to be replaced while simultaneously moving
out all unpinned fields in `Self`.

```rust
use std::pin::Pin;
type MyProjReplace = ();
trait Dox {
fn project_replace(self: Pin<&mut Self>, replacement: Self) -> MyProjReplace;
}
```

Also, note that the projection types returned by `project` and `project_ref` have
an additional lifetime at the beginning of generics.

```text
let this: EnumProj<'_, T> = self.project();
                   ^^
```

The visibility of the projected types and projection methods is based on the
original type. However, if the visibility of the original type is `pub`, the
visibility of the projected types and the projection methods is downgraded
to `pub(crate)`.

# Safety

`pin_project!` macro guarantees safety in much the same way as [pin-project] crate.
Both are completely safe unless you write other unsafe code.

See [pin-project] crate for more details.

# Examples

```rust
use std::pin::Pin;

use pin_project_lite::pin_project;

pin_project! {
    struct Struct<T, U> {
        #[pin]
        pinned: T,
        unpinned: U,
    }
}

impl<T, U> Struct<T, U> {
    fn method(self: Pin<&mut Self>) {
        let this = self.project();
        let _: Pin<&mut T> = this.pinned; // Pinned reference to the field
        let _: &mut U = this.unpinned; // Normal reference to the field
    }
}
```

To use `pin_project!` on enums, you need to name the projection type
returned from the method.

```rust
use std::pin::Pin;

use pin_project_lite::pin_project;

pin_project! {
    #[project = EnumProj]
    enum Enum<T> {
        Struct {
            #[pin]
            field: T,
        },
        Unit,
    }
}

impl<T> Enum<T> {
    fn method(self: Pin<&mut Self>) {
        match self.project() {
            EnumProj::Struct { field } => {
                let _: Pin<&mut T> = field;
            }
            EnumProj::Unit => {}
        }
    }
}
```

If you want to call the `project()` method multiple times or later use the
original `Pin` type, it needs to use [`.as_mut()`]`Pin::as_mut` to avoid
consuming the `Pin`.

```rust
use std::pin::Pin;

use pin_project_lite::pin_project;

pin_project! {
    struct Struct<T> {
        #[pin]
        field: T,
    }
}

impl<T> Struct<T> {
    fn call_project_twice(mut self: Pin<&mut Self>) {
        // `project` consumes `self`, so reborrow the `Pin<&mut Self>` via `as_mut`.
        self.as_mut().project();
        self.as_mut().project();
    }
}
```

# `!Unpin`

If you want to make sure `Unpin` is not implemented, use the `#[project(!Unpin)]`
attribute.

```rust
use pin_project_lite::pin_project;

pin_project! {
     #[project(!Unpin)]
     struct Struct<T> {
         #[pin]
         field: T,
     }
}
```

This is equivalent to using `#[pin]` attribute for a `PhantomPinned` field.

```rust
use std::marker::PhantomPinned;

use pin_project_lite::pin_project;

pin_project! {
    struct Struct<T> {
        field: T,
        #[pin]
        _pin: PhantomPinned,
    }
}
```

Note that using `PhantomPinned` without `#[pin]` or `#[project(!Unpin)]`
attribute has no effect.

# Pinned Drop

In order to correctly implement pin projections, a type’s `Drop` impl must not move out of any
structurally pinned fields. Unfortunately, `Drop::drop` takes `&mut Self`, not `Pin<&mut
Self>`.

To implement `Drop` for type that has pin, add an `impl PinnedDrop` block at the end of the
[`pin_project`](#pin-project) macro block. PinnedDrop has the following interface:

```rust
use std::pin::Pin;
trait PinnedDrop {
    fn drop(this: Pin<&mut Self>);
}
```

Note that the argument to `PinnedDrop::drop` cannot be named `self`.

`pin_project!` implements the actual `Drop` trait via PinnedDrop you implemented. To
explicitly drop a type that implements PinnedDrop, use the [drop] function just like dropping a
type that directly implements `Drop`.

`PinnedDrop::drop` will never be called more than once, just like `Drop::drop`.

```rust
use pin_project_lite::pin_project;

pin_project! {
    pub struct Struct<'a> {
        was_dropped: &'a mut bool,
        #[pin]
        field: u8,
    }

    impl PinnedDrop for Struct<'_> {
        fn drop(this: Pin<&mut Self>) { // <----- NOTE: this is not `self`
            **this.project().was_dropped = true;
        }
    }
}

let mut was_dropped = false;
drop(Struct { was_dropped: &mut was_dropped, field: 42 });
assert!(was_dropped);
```





