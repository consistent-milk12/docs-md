# Crate `static_assertions`

[![Banner](https://raw.githubusercontent.com/nvzqz/static-assertions-rs/assets/Banner.png)](https://github.com/nvzqz/static-assertions-rs)

<div align="center">
    <a href="https://crates.io/crates/static_assertions">
        <img src="https://img.shields.io/crates/d/static_assertions.svg" alt="Downloads">
    </a>
    <a href="https://travis-ci.org/nvzqz/static-assertions-rs">
        <img src="https://travis-ci.org/nvzqz/static-assertions-rs.svg?branch=master" alt="Build Status">
    </a>
    <img src="https://img.shields.io/badge/rustc-^1.37.0-blue.svg" alt="rustc ^1.37.0">
    <br><br>
</div>

Assertions to ensure correct assumptions about constants, types, and more.

_All_ checks provided by this crate are performed at [compile-time]. This
allows for finding errors quickly and early when it comes to ensuring
certain features or aspects of a codebase. These macros are especially
important when exposing a public API that requires types to be the same size
or implement certain traits.

# Usage

This crate is available [on crates.io][crate] and can be used by adding the
following to your project's `Cargo.toml`:

```toml
[dependencies]
static_assertions = "1.1.0"
```

and this to your crate root (`main.rs` or `lib.rs`):

```rust
#[macro_use]
extern crate static_assertions;
fn main() {}
```

When using [Rust 2018 edition][2018], the following shorthand can help if
having `#[macro_use]` is undesirable.

```edition2018
extern crate static_assertions as sa;

sa::const_assert!(true);
```

# Examples

Very thorough examples are provided in the docs for
[each individual macro](#macros). Failure case examples are also documented.

# Changes

See [`CHANGELOG.md`](https://github.com/nvzqz/static-assertions-rs/blob/master/CHANGELOG.md)
for an exhaustive list of what has changed from one version to another.

# Donate

This project is made freely available (as in free beer), but unfortunately
not all beer is free! So, if you would like to buy me a beer (or coffee or
*more*), then consider supporting my work that's benefited your project
and thousands of others.

<a href="https://www.patreon.com/nvzqz">
    <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" alt="Become a Patron!" height="35">
</a>
<a href="https://www.paypal.me/nvzqz">
    <img src="https://buymecoffee.intm.org/img/button-paypal-white.png" alt="Buy me a coffee" height="35">
</a>






## Contents

- [Modules](#modules)
  - [`assert_cfg`](#assert-cfg)
  - [`assert_eq_align`](#assert-eq-align)
  - [`assert_eq_size`](#assert-eq-size)
  - [`assert_fields`](#assert-fields)
  - [`assert_impl`](#assert-impl)
  - [`assert_obj_safe`](#assert-obj-safe)
  - [`assert_trait`](#assert-trait)
  - [`assert_type`](#assert-type)
  - [`const_assert`](#const-assert)
- [Macros](#macros)
  - [`assert_cfg!`](#assert-cfg)
  - [`assert_eq_align!`](#assert-eq-align)
  - [`assert_eq_size!`](#assert-eq-size)
  - [`assert_eq_size_ptr!`](#assert-eq-size-ptr)
  - [`assert_eq_size_val!`](#assert-eq-size-val)
  - [`assert_fields!`](#assert-fields)
  - [`assert_impl_one!`](#assert-impl-one)
  - [`assert_impl_all!`](#assert-impl-all)
  - [`assert_impl_any!`](#assert-impl-any)
  - [`assert_not_impl_all!`](#assert-not-impl-all)
  - [`assert_not_impl_any!`](#assert-not-impl-any)
  - [`assert_obj_safe!`](#assert-obj-safe)
  - [`assert_trait_sub_all!`](#assert-trait-sub-all)
  - [`assert_trait_super_all!`](#assert-trait-super-all)
  - [`assert_type_eq_all!`](#assert-type-eq-all)
  - [`assert_type_ne_all!`](#assert-type-ne-all)
  - [`const_assert!`](#const-assert)
  - [`const_assert_eq!`](#const-assert-eq)
  - [`const_assert_ne!`](#const-assert-ne)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`assert_cfg`](#assert-cfg) | mod |  |
| [`assert_eq_align`](#assert-eq-align) | mod |  |
| [`assert_eq_size`](#assert-eq-size) | mod |  |
| [`assert_fields`](#assert-fields) | mod |  |
| [`assert_impl`](#assert-impl) | mod |  |
| [`assert_obj_safe`](#assert-obj-safe) | mod |  |
| [`assert_trait`](#assert-trait) | mod |  |
| [`assert_type`](#assert-type) | mod |  |
| [`const_assert`](#const-assert) | mod |  |
| [`assert_cfg!`](#assert-cfg) | macro | Asserts that a given configuration is set. |
| [`assert_eq_align!`](#assert-eq-align) | macro | Asserts that types are equal in alignment. |
| [`assert_eq_size!`](#assert-eq-size) | macro | Asserts that types are equal in size. |
| [`assert_eq_size_ptr!`](#assert-eq-size-ptr) | macro | Asserts that values pointed to are equal in size. |
| [`assert_eq_size_val!`](#assert-eq-size-val) | macro | Asserts that values are equal in size. |
| [`assert_fields!`](#assert-fields) | macro | Asserts that the type has the given fields. |
| [`assert_impl_one!`](#assert-impl-one) | macro | Asserts that the type implements exactly one in a set of traits. |
| [`assert_impl_all!`](#assert-impl-all) | macro | Asserts that the type implements _all_ of the given traits. |
| [`assert_impl_any!`](#assert-impl-any) | macro | Asserts that the type implements _any_ of the given traits. |
| [`assert_not_impl_all!`](#assert-not-impl-all) | macro | Asserts that the type does **not** implement _all_ of the given traits. |
| [`assert_not_impl_any!`](#assert-not-impl-any) | macro | Asserts that the type does **not** implement _any_ of the given traits. |
| [`assert_obj_safe!`](#assert-obj-safe) | macro | Asserts that the traits support dynamic dispatch ([object-safety](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects)). |
| [`assert_trait_sub_all!`](#assert-trait-sub-all) | macro | Asserts that the trait is a child of all of the other traits. |
| [`assert_trait_super_all!`](#assert-trait-super-all) | macro | Asserts that the trait is a parent of all of the other traits. |
| [`assert_type_eq_all!`](#assert-type-eq-all) | macro | Asserts that _all_ types in a list are equal to each other. |
| [`assert_type_ne_all!`](#assert-type-ne-all) | macro | Asserts that _all_ types are **not** equal to each other. |
| [`const_assert!`](#const-assert) | macro | Asserts that constant expressions evaluate to `true`. |
| [`const_assert_eq!`](#const-assert-eq) | macro | Asserts that constants are equal in value. |
| [`const_assert_ne!`](#const-assert-ne) | macro | Asserts that constants are **not** equal in value. |

## Modules

- [`assert_cfg`](assert_cfg/index.md)
- [`assert_eq_align`](assert_eq_align/index.md)
- [`assert_eq_size`](assert_eq_size/index.md)
- [`assert_fields`](assert_fields/index.md)
- [`assert_impl`](assert_impl/index.md)
- [`assert_obj_safe`](assert_obj_safe/index.md)
- [`assert_trait`](assert_trait/index.md)
- [`assert_type`](assert_type/index.md)
- [`const_assert`](const_assert/index.md)

## Macros

### `assert_cfg!`

*Defined in [`static_assertions-1.1.0/src/assert_cfg.rs:39-49`](../../.source_1765521767/static_assertions-1.1.0/src/assert_cfg.rs#L39-L49)*

Asserts that a given configuration is set.

# Examples

A project will simply fail to compile if the given configuration is not set.

```rust
#[macro_use] extern crate static_assertions; fn main() {}
// We're not masochists
#[cfg(not(target_pointer_width = "16"))] // Just in case
assert_cfg!(not(target_pointer_width = "16"));
```

If a project does not support a set of configurations, you may want to
report why. There is the option of providing a compile error message string:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
#[cfg(any(unix, windows))]
assert_cfg!(any(unix, windows), "There is only support for Unix or Windows");

// User needs to specify a database back-end
#[cfg(target_pointer_width = "0")] // Impossible
assert_cfg!(all(not(all(feature = "mysql", feature = "mongodb")),
                any(    feature = "mysql", feature = "mongodb")),
            "Must exclusively use MySQL or MongoDB as database back-end");
```

Some configurations are impossible. For example, we can't be compiling for
both macOS _and_ Windows simultaneously:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
assert_cfg!(all(target_os = "macos",
                target_os = "windows"),
            "No, that's not how it works! ಠ_ಠ");
```

### `assert_eq_align!`

*Defined in [`static_assertions-1.1.0/src/assert_eq_align.rs:36-45`](../../.source_1765521767/static_assertions-1.1.0/src/assert_eq_align.rs#L36-L45)*

Asserts that types are equal in alignment.

This is useful when ensuring that pointer arithmetic is done correctly, or
when [FFI] requires a type to have the same alignment as some foreign type.

# Examples

A `usize` has the same alignment as any pointer type:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_eq_align!(usize, *const u8, *mut u8);
```

The following passes because `[i32; 4]` has the same alignment as `i32`:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_eq_align!([i32; 4], i32);
```

The following example fails to compile because `i32x4` explicitly has 4
times the alignment as `[i32; 4]`:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
#[allow(non_camel_case_types)]
#[repr(align(16))]
struct i32x4([i32; 4]);

assert_eq_align!(i32x4, [i32; 4]);
```


### `assert_eq_size!`

*Defined in [`static_assertions-1.1.0/src/assert_eq_size.rs:34-40`](../../.source_1765521767/static_assertions-1.1.0/src/assert_eq_size.rs#L34-L40)*

Asserts that types are equal in size.

When performing operations such as pointer casts or dealing with `usize`
versus `u64` versus `u32`, the size of your types matter. That is where
this macro comes into play.

# Alternatives

There also exists [`assert_eq_size_val`](#assert_eq_size_val) and
[`assert_eq_size_ptr`](#assert_eq_size_ptr). Instead of specifying
types to compare, values' sizes can be directly compared against each other.

# Examples

These three types, despite being very different, all have the same size:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_eq_size!([u8; 4], (u16, u16), u32);
```

The following example fails to compile because `u32` has 4 times the size of
`u8`:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
assert_eq_size!(u32, u8);
```




### `assert_eq_size_ptr!`

*Defined in [`static_assertions-1.1.0/src/assert_eq_size.rs:76-86`](../../.source_1765521767/static_assertions-1.1.0/src/assert_eq_size.rs#L76-L86)*

Asserts that values pointed to are equal in size.

# Examples

This especially is useful for when coercing pointers between different types
and ensuring the underlying values are the same size.

```rust
#[macro_use] extern crate static_assertions; fn main() {}
fn operation(x: &(u32, u32), y: &[u16; 4]) {
    assert_eq_size_ptr!(x, y);
    // ...
}
```

The following example fails to compile because byte arrays of different
lengths have different sizes:

```compile_fail
#[macro_use] extern crate static_assertions;
fn main() {
static BYTES: &[u8; 4] = &[
    /* ... */
    0; 4
];

static TABLE: &[u8; 16] = &[
    /* ... */
    0; 16
];

assert_eq_size_ptr!(BYTES, TABLE);
```

### `assert_eq_size_val!`

*Defined in [`static_assertions-1.1.0/src/assert_eq_size.rs:119-123`](../../.source_1765521767/static_assertions-1.1.0/src/assert_eq_size.rs#L119-L123)*

Asserts that values are equal in size.

This macro doesn't consume its arguments and thus works for
non-[`Clone`](#clone)able values.

# Examples

```rust
#[macro_use] extern crate static_assertions;
fn main() {
struct Byte(u8);

let x = 10u8;
let y = Byte(42); // Works for non-cloneable types

assert_eq_size_val!(x, y);
assert_eq_size_val!(x, y, 0u8);
}
```

Even though both values are 0, they are of types with different sizes:

```compile_fail
#[macro_use] extern crate static_assertions;
fn main() {
assert_eq_size_val!(0u8, 0u32);
}
```


### `assert_fields!`

*Defined in [`static_assertions-1.1.0/src/assert_fields.rs:53-72`](../../.source_1765521767/static_assertions-1.1.0/src/assert_fields.rs#L53-L72)*

Asserts that the type has the given fields.

# Examples

One common use case is when types have fields defined multiple times as a
result of `#[cfg]`. This can be an issue when exposing a public API.

```rust
#[macro_use] extern crate static_assertions;
pub struct Ty {
    #[cfg(windows)]
    pub val1: u8,
    #[cfg(not(windows))]
    pub val1: usize,

    #[cfg(unix)]
    pub val2: u32,
    #[cfg(not(unix))]
    pub val2: usize,
}

// Always have `val2` regardless of OS
assert_fields!(Ty: val2);
```

This macro even works with `enum` variants:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
enum Data {
    Val {
        id: i32,
        name: String,
        bytes: [u8; 128],
    },
    Ptr(*const u8),
}

assert_fields!(Data::Val: id, bytes);
```

The following example fails to compile because `Range` does not have a field named `middle`:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
use std::ops::Range;

assert_fields!(Range<u32>: middle);
```


### `assert_impl_one!`

*Defined in [`static_assertions-1.1.0/src/assert_impl.rs:57-83`](../../.source_1765521767/static_assertions-1.1.0/src/assert_impl.rs#L57-L83)*

Asserts that the type implements exactly one in a set of traits.

Related:
- `assert_impl_any!`
- `assert_impl_all!`
- `assert_not_impl_all!`
- `assert_not_impl_any!`

# Examples

Given some type `Foo`, it is expected to implement either `Snap`, `Crackle`,
or `Pop`:

```compile_fail
use static_assertions::assert_impl_one; fn main() {}
struct Foo;

trait Snap {}
trait Crackle {}
trait Pop {}

assert_impl_one!(Foo: Snap, Crackle, Pop);
```

If _only_ `Crackle` is implemented, the assertion passes:

```rust
use static_assertions::assert_impl_one; fn main() {}
struct Foo;
trait Snap {}
trait Crackle {}
trait Pop {}
impl Crackle for Foo {}

assert_impl_one!(Foo: Snap, Crackle, Pop);
```

If `Snap` or `Pop` is _also_ implemented, the assertion fails:

```compile_fail
use static_assertions::assert_impl_one; fn main() {}
struct Foo;
trait Snap {}
trait Crackle {}
trait Pop {}
impl Crackle for Foo {}
impl Pop for Foo {}

assert_impl_one!(Foo: Snap, Crackle, Pop);
```





### `assert_impl_all!`

*Defined in [`static_assertions-1.1.0/src/assert_impl.rs:113-121`](../../.source_1765521767/static_assertions-1.1.0/src/assert_impl.rs#L113-L121)*

Asserts that the type implements _all_ of the given traits.

See `assert_not_impl_all!` for achieving the opposite effect.

# Examples

This can be used to ensure types implement auto traits such as `Send` and
[`Sync`](#sync), as well as traits with [blanket `impl`s][blanket].

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_impl_all!(u32: Copy, Send);
assert_impl_all!(&str: Into<String>);
```

The following example fails to compile because raw pointers do not implement
`Send` since they cannot be moved between threads safely:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
assert_impl_all!(*const u8: Send);
```





### `assert_impl_any!`

*Defined in [`static_assertions-1.1.0/src/assert_impl.rs:157-212`](../../.source_1765521767/static_assertions-1.1.0/src/assert_impl.rs#L157-L212)*

Asserts that the type implements _any_ of the given traits.

See `assert_not_impl_any!` for achieving the opposite effect.

# Examples

`u8` cannot be converted from `u16`, but it can be converted into `u16`:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_impl_any!(u8: From<u16>, Into<u16>);
```

The unit type cannot be converted from `u8` or `u16`, but it does implement

# #[macro_use] extern crate static_assertions; fn main() {}
assert_impl_any!((): From<u8>, From<u16>, Send);
```rust

The following example fails to compile because raw pointers do not implement
`Send` or [`Sync`](#sync) since they cannot be moved or shared between threads
safely:

```compile_fail
# #[macro_use] extern crate static_assertions; fn main() {}
assert_impl_any!(*const u8: Send, Sync);
```rust




### `assert_not_impl_all!`

*Defined in [`static_assertions-1.1.0/src/assert_impl.rs:263-287`](../../.source_1765521767/static_assertions-1.1.0/src/assert_impl.rs#L263-L287)*

Asserts that the type does **not** implement _all_ of the given traits.

This can be used to ensure types do not implement auto traits such as
`Send` and [`Sync`](#sync), as well as traits with [blanket `impl`s][blanket].

Note that the combination of all provided traits is required to not be
implemented. If you want to check that none of multiple traits are
implemented you should invoke `assert_not_impl_any!` instead.

# Examples

Although `u32` implements `From<u16>`, it does not implement `Into<usize>`:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_not_impl_all!(u32: From<u16>, Into<usize>);
```

The following example fails to compile since `u32` can be converted into
`u64`.

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
assert_not_impl_all!(u32: Into<u64>);
```

The following compiles because [`Cell`](#cell) is not both [`Sync`](#sync) _and_ `Send`:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
use std::cell::Cell;

assert_not_impl_all!(Cell<u32>: Sync, Send);
```

But it is `Send`, so this fails to compile:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
std::cell::Cell;
assert_not_impl_all!(Cell<u32>: Send);
```






### `assert_not_impl_any!`

*Defined in [`static_assertions-1.1.0/src/assert_impl.rs:329-356`](../../.source_1765521767/static_assertions-1.1.0/src/assert_impl.rs#L329-L356)*

Asserts that the type does **not** implement _any_ of the given traits.

This can be used to ensure types do not implement auto traits such as
`Send` and [`Sync`](#sync), as well as traits with [blanket `impl`s][blanket].

This macro causes a compilation failure if any of the provided individual
traits are implemented for the type. If you want to check that a combination
of traits is not implemented you should invoke `assert_not_impl_all!`
instead. For single traits both macros behave the same.

# Examples

If `u32` were to implement `Into` conversions for `usize` _and_ for `u8`,
the following would fail to compile:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_not_impl_any!(u32: Into<usize>, Into<u8>);
```

This is also good for simple one-off cases:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_not_impl_any!(&'static mut u8: Copy);
```

The following example fails to compile since `u32` can be converted into
`u64` even though it can not be converted into a `u16`:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
assert_not_impl_any!(u32: Into<u64>, Into<u16>);
```





### `assert_obj_safe!`

*Defined in [`static_assertions-1.1.0/src/assert_obj_safe.rs:72-76`](../../.source_1765521767/static_assertions-1.1.0/src/assert_obj_safe.rs#L72-L76)*

Asserts that the traits support dynamic dispatch
([object-safety](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects)).

This is useful for when changes are made to a trait that accidentally
prevent it from being used as an [`object`](../object/index.md). Such a case would be adding a
generic method and forgetting to add `where Self: Sized` after it. If left
unnoticed, that mistake will affect crate users and break both forward and
backward compatibility.

# Examples

When exposing a public API, it's important that traits that could previously
use dynamic dispatch can still do so in future compatible crate versions.

```rust
#[macro_use] extern crate static_assertions; fn main() {}
trait MySafeTrait {
    fn foo(&self) -> u32;
}

assert_obj_safe!(std::fmt::Write, MySafeTrait);
```

Works with traits that are not in the calling module:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
mod inner {
    pub trait BasicTrait {
        fn bar(&self);
    }
    assert_obj_safe!(BasicTrait);
}

assert_obj_safe!(inner::BasicTrait);
```

The following example fails to compile because raw pointers cannot be sent
 between threads safely:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
assert_impl!(*const u8, Send);
```

The following example fails to compile because generics without
`where Self: Sized` are not allowed in [object-safe][`object`](../object/index.md) trait methods:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
trait MyUnsafeTrait {
    fn baz<T>(&self) -> T;
}

assert_obj_safe!(MyUnsafeTrait);
```

When we fix that, the previous code will compile:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
trait MyUnsafeTrait {
    fn baz<T>(&self) -> T where Self: Sized;
}

assert_obj_safe!(MyUnsafeTrait);
```


### `assert_trait_sub_all!`

*Defined in [`static_assertions-1.1.0/src/assert_trait.rs:40-54`](../../.source_1765521767/static_assertions-1.1.0/src/assert_trait.rs#L40-L54)*

Asserts that the trait is a child of all of the other traits.

Related:
- `assert_trait_super_all!`

# Examples

All types that implement [`Copy`](#copy) must implement [`Clone`](#clone):

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_trait_sub_all!(Copy: Clone);
```

All types that implement `Ord` must implement `PartialEq`, `Eq`, and

# #[macro_use] extern crate static_assertions; fn main() {}
assert_trait_sub_all!(Ord: PartialEq, Eq, PartialOrd);
```rust

The following example fails to compile because `Eq` is not required for

#[macro_use] extern crate static_assertions; fn main() {}
assert_trait_sub_all!(PartialOrd: Eq);
```







### `assert_trait_super_all!`

*Defined in [`static_assertions-1.1.0/src/assert_trait.rs:101-105`](../../.source_1765521767/static_assertions-1.1.0/src/assert_trait.rs#L101-L105)*

Asserts that the trait is a parent of all of the other traits.

Related:
- `assert_trait_sub_all!`

# Examples

With this, traits `A` and `B` can both be tested to require [`Copy`](#copy) on a
single line:

```rust
use static_assertions::assert_trait_super_all;
trait A: Copy {}
trait B: Copy {}

assert_trait_super_all!(Copy: A, B);
```

Otherwise, each sub-trait would require its own call to

# #[macro_use] extern crate static_assertions; fn main() {}
# trait A: Copy {}
# trait B: Copy {}
assert_trait_sub_all!(A: Copy);
assert_trait_sub_all!(B: Copy);
```rust

The following example fails to compile because trait `C` does not require

use static_assertions::assert_trait_super_all;
trait A: Copy {}
trait B: Copy {}
trait C {}

assert_trait_super_all!(Copy: A, B, C);
```


### `assert_type_eq_all!`

*Defined in [`static_assertions-1.1.0/src/assert_type.rs:47-67`](../../.source_1765521767/static_assertions-1.1.0/src/assert_type.rs#L47-L67)*

Asserts that _all_ types in a list are equal to each other.

# Examples

Often times, type aliases are used to express usage semantics via naming. In
some cases, the underlying type may differ based on platform. However, other
types like [`c_float`](#c-float) will always alias the same type.

```rust
#[macro_use] extern crate static_assertions; fn main() {}
use std::os::raw::c_float;

assert_type_eq_all!(c_float, f32);
```

This macro can also be used to compare types that involve lifetimes! Just
use `'static` in that case:

```rust
#[macro_use] extern crate static_assertions;
fn main() {
type Buf<'a> = &'a [u8];

assert_type_eq_all!(Buf<'static>, &'static [u8]);
}
```

The following example fails to compile because `String` and `str` do not
refer to the same type:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
assert_type_eq_all!(String, str);
```

This should also work the other way around, regardless of `Deref`
implementations.

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
assert_type_eq_all!(str, String);
```



### `assert_type_ne_all!`

*Defined in [`static_assertions-1.1.0/src/assert_type.rs:93-101`](../../.source_1765521767/static_assertions-1.1.0/src/assert_type.rs#L93-L101)*

Asserts that _all_ types are **not** equal to each other.

# Examples

Rust has all sorts of slices, but they represent different types of data:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_type_ne_all!([u8], [u16], str);
```

The following example fails to compile because [`c_uchar`](#c-uchar) is a type alias
for `u8`:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
use std::os::raw::c_uchar;

assert_type_ne_all!(c_uchar, u8, u32);
```



### `const_assert!`

*Defined in [`static_assertions-1.1.0/src/const_assert.rs:52-57`](../../.source_1765521767/static_assertions-1.1.0/src/const_assert.rs#L52-L57)*

Asserts that constant expressions evaluate to `true`.

Constant expressions can be ensured to have certain properties via this
macro If the expression evaluates to `false`, the file will fail to compile.
This is synonymous to [`static_assert` in C++][`static_assert`](../portable_atomic/utils/index.md).

# Alternatives

There also exists [`const_assert_eq`](#const_assert_eq) for
validating whether a sequence of expressions are equal to one another.

# Examples

A common use case is to guarantee properties about a constant value that's
generated via meta-programming.

```rust
#[macro_use] extern crate static_assertions; fn main() {}
const VALUE: i32 = // ...
3;

const_assert!(VALUE >= 2);
```

Inputs are type-checked as booleans:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
const_assert!(!0);
```

Despite this being a macro, we see this produces a type error:

```txt
  | const_assert!(!0);
  |               ^^ expected bool, found integral variable
  |
  = note: expected type `bool`
             found type `{integer}`
```

The following fails to compile because multiplying by 5 does not have an
identity property:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
const_assert!(5 * 5 == 5);
```


### `const_assert_eq!`

*Defined in [`static_assertions-1.1.0/src/const_assert.rs:79-83`](../../.source_1765521767/static_assertions-1.1.0/src/const_assert.rs#L79-L83)*

Asserts that constants are equal in value.

# Examples

This works as a shorthand for `const_assert!(a == b)`:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
const TWO: usize = 2;

const_assert_eq!(TWO * TWO, TWO + TWO);
```

Just because 2 × 2 = 2 + 2 doesn't mean it holds true for other numbers:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
const_assert_eq!(4 + 4, 4 * 4);
```

### `const_assert_ne!`

*Defined in [`static_assertions-1.1.0/src/const_assert.rs:105-109`](../../.source_1765521767/static_assertions-1.1.0/src/const_assert.rs#L105-L109)*

Asserts that constants are **not** equal in value.

# Examples

This works as a shorthand for `const_assert!(a != b)`:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
const NUM: usize = 32;

const_assert_ne!(NUM * NUM, 64);
```

The following example fails to compile because 2 is magic and 2 × 2 = 2 + 2:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
const_assert_ne!(2 + 2, 2 * 2);
```

