*[clap_builder](../../index.md) / [builder](../index.md) / [resettable](index.md)*

---

# Module `resettable`

## Enums

### `Resettable<T>`

```rust
enum Resettable<T> {
    Value(T),
    Reset,
}
```

Clearable builder value

This allows a builder function to both accept any value that can `Into::into` `T` (like
`&str` into `OsStr`) as well as `None` to reset it to the default.  This is needed to
workaround a limitation where you can't have a function argument that is `impl Into<Option<T>>`
where `T` is `impl Into<S>` accept `None` as its type is ambiguous.

# Example

```rust
use clap_builder as clap;
use clap::Command;
use clap::Arg;
fn common() -> Command {
    Command::new("cli")
        .arg(Arg::new("input").short('i').long("input"))
}
let mut command = common();
command.mut_arg("input", |arg| arg.short(None));
```

#### Variants

- **`Value`**

  Overwrite builder value

- **`Reset`**

  Reset builder value

#### Implementations

- `fn into_option(self: Self) -> Option<T>`

#### Trait Implementations

##### `impl<T: $crate::clone::Clone> Clone for Resettable<T>`

- `fn clone(self: &Self) -> Resettable<T>` — [`Resettable`](../index.md)

##### `impl<T: $crate::marker::Copy> Copy for Resettable<T>`

##### `impl<T: $crate::fmt::Debug> Debug for Resettable<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: $crate::cmp::Eq> Eq for Resettable<T>`

##### `impl<T: $crate::hash::Hash> Hash for Resettable<T>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl<T> IntoResettable for Resettable<T>`

- `fn into_resettable(self: Self) -> Resettable<T>` — [`Resettable`](../index.md)

##### `impl<T: $crate::cmp::Ord> Ord for Resettable<T>`

- `fn cmp(self: &Self, other: &Resettable<T>) -> $crate::cmp::Ordering` — [`Resettable`](../index.md)

##### `impl<T: $crate::cmp::PartialEq> PartialEq for Resettable<T>`

- `fn eq(self: &Self, other: &Resettable<T>) -> bool` — [`Resettable`](../index.md)

##### `impl<T: $crate::cmp::PartialOrd> PartialOrd for Resettable<T>`

- `fn partial_cmp(self: &Self, other: &Resettable<T>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`Resettable`](../index.md)

##### `impl<T> StructuralPartialEq for Resettable<T>`

## Traits

### `IntoResettable<T>`

```rust
trait IntoResettable<T> { ... }
```

Convert to the intended resettable type

#### Required Methods

- `fn into_resettable(self: Self) -> Resettable<T>`

  Convert to the intended resettable type

