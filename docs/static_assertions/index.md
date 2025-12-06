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

This crate is available [on crates.io][crate](#crate) and can be used by adding the
following to your project's [`Cargo.toml`](#cargotoml):

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






## Macros

### `assert_cfg!`

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

Asserts that types are equal in size.

When performing operations such as pointer casts or dealing with [`usize`](#usize)
versus [`u64`](#u64) versus [`u32`](#u32), the size of your types matter. That is where
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

The following example fails to compile because [`Range`](#range) does not have a field named `middle`:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
use std::ops::Range;

assert_fields!(Range<u32>: middle);
```


### `assert_impl_one!`

Asserts that the type implements exactly one in a set of traits.

Related:
- [`assert_impl_any!`](#assert-impl-any)
- [`assert_impl_all!`](#assert-impl-all)
- [`assert_not_impl_all!`](#assert-not-impl-all)
- [`assert_not_impl_any!`](#assert-not-impl-any)

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

Asserts that the type implements _all_ of the given traits.

See [`assert_not_impl_all!`](#assert-not-impl-all) for achieving the opposite effect.

# Examples

This can be used to ensure types implement auto traits such as [`Send`](#send) and
[`Sync`](#sync), as well as traits with [blanket `impl`s][blanket](#blanket).

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_impl_all!(u32: Copy, Send);
assert_impl_all!(&str: Into<String>);
```

The following example fails to compile because raw pointers do not implement
[`Send`](#send) since they cannot be moved between threads safely:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
assert_impl_all!(*const u8: Send);
```





### `assert_impl_any!`

Asserts that the type implements _any_ of the given traits.

See [`assert_not_impl_any!`](#assert-not-impl-any) for achieving the opposite effect.

# Examples

`u8` cannot be converted from `u16`, but it can be converted into `u16`:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_impl_any!(u8: From<u16>, Into<u16>);
```

The unit type cannot be converted from `u8` or `u16`, but it does implement

# #[macro_use](#macro-use) extern crate static_assertions; fn main() {}
assert_impl_any!((): From<u8>, From<u16>, Send);
```rust

The following example fails to compile because raw pointers do not implement
[`Send`](#send) or [`Sync`](#sync) since they cannot be moved or shared between threads
safely:

```compile_fail
# #[macro_use](#macro-use) extern crate static_assertions; fn main() {}
assert_impl_any!(*const u8: Send, Sync);
```rust




### `assert_not_impl_all!`

Asserts that the type does **not** implement _all_ of the given traits.

This can be used to ensure types do not implement auto traits such as
[`Send`](#send) and [`Sync`](#sync), as well as traits with [blanket `impl`s][blanket](#blanket).

Note that the combination of all provided traits is required to not be
implemented. If you want to check that none of multiple traits are
implemented you should invoke [`assert_not_impl_any!`](#assert-not-impl-any) instead.

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

The following compiles because [`Cell`](#cell) is not both [`Sync`](#sync) _and_ [`Send`](#send):

```rust
#[macro_use] extern crate static_assertions; fn main() {}
use std::cell::Cell;

assert_not_impl_all!(Cell<u32>: Sync, Send);
```

But it is [`Send`](#send), so this fails to compile:

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
std::cell::Cell;
assert_not_impl_all!(Cell<u32>: Send);
```






### `assert_not_impl_any!`

Asserts that the type does **not** implement _any_ of the given traits.

This can be used to ensure types do not implement auto traits such as
[`Send`](#send) and [`Sync`](#sync), as well as traits with [blanket `impl`s][blanket](#blanket).

This macro causes a compilation failure if any of the provided individual
traits are implemented for the type. If you want to check that a combination
of traits is not implemented you should invoke [`assert_not_impl_all!`](#assert-not-impl-all)
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

Asserts that the traits support dynamic dispatch
([object-safety](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects)).

This is useful for when changes are made to a trait that accidentally
prevent it from being used as an [object](#object). Such a case would be adding a
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
`where Self: Sized` are not allowed in [object-safe][object](#object) trait methods:

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

Asserts that the trait is a child of all of the other traits.

Related:
- [`assert_trait_super_all!`](#assert-trait-super-all)

# Examples

All types that implement [`Copy`](#copy) must implement [`Clone`](#clone):

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_trait_sub_all!(Copy: Clone);
```

All types that implement [`Ord`](#ord) must implement [`PartialEq`](#partialeq), [`Eq`](#eq), and

# #[macro_use](#macro-use) extern crate static_assertions; fn main() {}
assert_trait_sub_all!(Ord: PartialEq, Eq, PartialOrd);
```rust

The following example fails to compile because [`Eq`](#eq) is not required for

#[macro_use] extern crate static_assertions; fn main() {}
assert_trait_sub_all!(PartialOrd: Eq);
```







### `assert_trait_super_all!`

Asserts that the trait is a parent of all of the other traits.

Related:
- [`assert_trait_sub_all!`](#assert-trait-sub-all)

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

# #[macro_use](#macro-use) extern crate static_assertions; fn main() {}
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

This should also work the other way around, regardless of [`Deref`](#deref)
implementations.

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
assert_type_eq_all!(str, String);
```



### `assert_type_ne_all!`

Asserts that _all_ types are **not** equal to each other.

# Examples

Rust has all sorts of slices, but they represent different types of data:

```rust
#[macro_use] extern crate static_assertions; fn main() {}
assert_type_ne_all!([u8], [u16], str);
```

The following example fails to compile because [`c_uchar`](#c-uchar) is a type alias
for [`u8`](#u8):

```compile_fail
#[macro_use] extern crate static_assertions; fn main() {}
use std::os::raw::c_uchar;

assert_type_ne_all!(c_uchar, u8, u32);
```



### `const_assert!`

Asserts that constant expressions evaluate to `true`.

Constant expressions can be ensured to have certain properties via this
macro If the expression evaluates to `false`, the file will fail to compile.
This is synonymous to [`static_assert` in C++][static_assert](#static-assert).

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

