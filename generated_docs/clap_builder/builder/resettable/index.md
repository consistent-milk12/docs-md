*[clap_builder](../../index.md) / [builder](../index.md) / [resettable](index.md)*

---

# Module `resettable`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Resettable`](#resettable) | enum | Clearable builder value |
| [`IntoResettable`](#intoresettable) | trait | Convert to the intended resettable type |

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

- <span id="resettable-into-option"></span>`fn into_option(self) -> Option<T>`

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Resettable<T>`

- <span id="resettable-clone"></span>`fn clone(&self) -> Resettable<T>` — [`Resettable`](../index.md)

##### `impl<T: marker::Copy> Copy for Resettable<T>`

##### `impl<T: fmt::Debug> Debug for Resettable<T>`

- <span id="resettable-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for Resettable<T>`

##### `impl<T: hash::Hash> Hash for Resettable<T>`

- <span id="resettable-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> IntoResettable for Resettable<T>`

- <span id="resettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<T>` — [`Resettable`](../index.md)

##### `impl<T: cmp::Ord> Ord for Resettable<T>`

- <span id="resettable-cmp"></span>`fn cmp(&self, other: &Resettable<T>) -> cmp::Ordering` — [`Resettable`](../index.md)

##### `impl<T: cmp::PartialEq> PartialEq for Resettable<T>`

- <span id="resettable-eq"></span>`fn eq(&self, other: &Resettable<T>) -> bool` — [`Resettable`](../index.md)

##### `impl<T: cmp::PartialOrd> PartialOrd for Resettable<T>`

- <span id="resettable-partial-cmp"></span>`fn partial_cmp(&self, other: &Resettable<T>) -> option::Option<cmp::Ordering>` — [`Resettable`](../index.md)

##### `impl<T> StructuralPartialEq for Resettable<T>`

## Traits

### `IntoResettable<T>`

```rust
trait IntoResettable<T> { ... }
```

Convert to the intended resettable type

#### Required Methods

- `fn into_resettable(self) -> Resettable<T>`

  Convert to the intended resettable type

