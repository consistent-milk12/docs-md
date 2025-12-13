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

*Defined in [`clap_builder-4.5.53/src/builder/resettable.rs:33-38`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/resettable.rs#L33-L38)*

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

##### `impl<T> Any for Resettable<T>`

- <span id="resettable-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Resettable<T>`

- <span id="resettable-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Resettable<T>`

- <span id="resettable-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for Resettable<T>`

- <span id="resettable-clone"></span>`fn clone(&self) -> Resettable<T>` — [`Resettable`](#resettable)

##### `impl<T> CloneToUninit for Resettable<T>`

- <span id="resettable-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for Resettable<T>`

##### `impl<T: fmt::Debug> Debug for Resettable<T>`

- <span id="resettable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for Resettable<T>`

##### `impl<T> From for Resettable<T>`

- <span id="resettable-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for Resettable<T>`

- <span id="resettable-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for Resettable<T>`

- <span id="resettable-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoResettable for Resettable<T>`

- <span id="resettable-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<T>` — [`Resettable`](#resettable)

##### `impl<T: cmp::Ord> Ord for Resettable<T>`

- <span id="resettable-ord-cmp"></span>`fn cmp(&self, other: &Resettable<T>) -> cmp::Ordering` — [`Resettable`](#resettable)

##### `impl<T: cmp::PartialEq> PartialEq for Resettable<T>`

- <span id="resettable-partialeq-eq"></span>`fn eq(&self, other: &Resettable<T>) -> bool` — [`Resettable`](#resettable)

##### `impl<T: cmp::PartialOrd> PartialOrd for Resettable<T>`

- <span id="resettable-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Resettable<T>) -> option::Option<cmp::Ordering>` — [`Resettable`](#resettable)

##### `impl<T> StructuralPartialEq for Resettable<T>`

##### `impl<T> ToOwned for Resettable<T>`

- <span id="resettable-toowned-type-owned"></span>`type Owned = T`

- <span id="resettable-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="resettable-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T, U> TryFrom for Resettable<T>`

- <span id="resettable-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="resettable-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for Resettable<T>`

- <span id="resettable-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="resettable-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `IntoResettable<T>`

```rust
trait IntoResettable<T> { ... }
```

*Defined in [`clap_builder-4.5.53/src/builder/resettable.rs:65-68`](../../../../.source_1765521767/clap_builder-4.5.53/src/builder/resettable.rs#L65-L68)*

Convert to the intended resettable type

#### Required Methods

- `fn into_resettable(self) -> Resettable<T>`

  Convert to the intended resettable type

#### Implementors

- [`ArgAction`](../action/index.md#argaction)
- [`Resettable`](#resettable)
- [`ValueHint`](../value_hint/index.md#valuehint)
- `I`
- `Option<&'static str>`
- `Option<char>`
- `Option<crate::builder::ArgAction>`
- `Option<crate::builder::ValueHint>`
- `Option<crate::builder::ValueParser>`
- `Option<usize>`
- `char`
- `usize`

