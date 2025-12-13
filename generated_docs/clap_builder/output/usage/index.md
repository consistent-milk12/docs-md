*[clap_builder](../../index.md) / [output](../index.md) / [usage](index.md)*

---

# Module `usage`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Usage`](#usage) | struct |  |
| [`USAGE_SEP`](#usage-sep) | const |  |

## Structs

### `Usage<'cmd>`

```rust
struct Usage<'cmd> {
    cmd: &'cmd crate::builder::Command,
    styles: &'cmd crate::builder::Styles,
    required: Option<&'cmd self::graph::ChildGraph<crate::util::Id>>,
}
```

*Defined in [`clap_builder-4.5.53/src/output/usage.rs:19-23`](../../../../.source_1765633015/clap_builder-4.5.53/src/output/usage.rs#L19-L23)*

#### Implementations

- <span id="usage-new"></span>`fn new(cmd: &'cmd Command) -> Self` — [`Command`](../../builder/command/index.md#command)

- <span id="usage-required"></span>`fn required(self, required: &'cmd ChildGraph<Id>) -> Self` — [`ChildGraph`](../../util/graph/index.md#childgraph), [`Id`](../../util/id/index.md#id)

- <span id="usage-create-usage-with-title"></span>`fn create_usage_with_title(&self, used: &[Id]) -> Option<StyledStr>` — [`Id`](../../util/id/index.md#id), [`StyledStr`](../../builder/styled_str/index.md#styledstr)

- <span id="usage-create-usage-no-title"></span>`fn create_usage_no_title(&self, used: &[Id]) -> Option<StyledStr>` — [`Id`](../../util/id/index.md#id), [`StyledStr`](../../builder/styled_str/index.md#styledstr)

- <span id="usage-write-usage-no-title"></span>`fn write_usage_no_title(&self, styled: &mut StyledStr, used: &[Id]) -> bool` — [`StyledStr`](../../builder/styled_str/index.md#styledstr), [`Id`](../../util/id/index.md#id)

#### Trait Implementations

##### `impl Any for Usage<'cmd>`

- <span id="usage-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Usage<'cmd>`

- <span id="usage-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Usage<'cmd>`

- <span id="usage-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Usage<'cmd>`

- <span id="usage-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Usage<'cmd>`

- <span id="usage-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Usage<'cmd>`

- <span id="usage-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="usage-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Usage<'cmd>`

- <span id="usage-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="usage-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `USAGE_SEP`
```rust
const USAGE_SEP: &str;
```

*Defined in [`clap_builder-4.5.53/src/output/usage.rs:17`](../../../../.source_1765633015/clap_builder-4.5.53/src/output/usage.rs#L17)*

