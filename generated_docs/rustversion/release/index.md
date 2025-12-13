*[rustversion](../index.md) / [release](index.md)*

---

# Module `release`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Release`](#release) | struct |  |
| [`parse`](#parse) | fn |  |
| [`try_parse`](#try-parse) | fn |  |

## Structs

### `Release`

```rust
struct Release {
    pub minor: u16,
    pub patch: Option<u16>,
}
```

*Defined in [`rustversion-1.0.22/src/release.rs:7-10`](../../../.source_1765633015/rustversion-1.0.22/src/release.rs#L7-L10)*

#### Trait Implementations

##### `impl Any for Release`

- <span id="release-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Release`

- <span id="release-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Release`

- <span id="release-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Release`

- <span id="release-clone"></span>`fn clone(&self) -> Release` — [`Release`](#release)

##### `impl CloneToUninit for Release`

- <span id="release-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Release`

##### `impl Debug for Release`

- <span id="release-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Release`

##### `impl<T> From for Release`

- <span id="release-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Release`

- <span id="release-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for Release`

- <span id="release-ord-cmp"></span>`fn cmp(&self, other: &Release) -> cmp::Ordering` — [`Release`](#release)

##### `impl PartialEq for Release`

- <span id="release-partialeq-eq"></span>`fn eq(&self, other: &Release) -> bool` — [`Release`](#release)

##### `impl PartialOrd for Release`

- <span id="release-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Release) -> option::Option<cmp::Ordering>` — [`Release`](#release)

##### `impl StructuralPartialEq for Release`

##### `impl ToOwned for Release`

- <span id="release-toowned-type-owned"></span>`type Owned = T`

- <span id="release-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="release-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Release`

- <span id="release-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="release-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Release`

- <span id="release-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="release-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse`

```rust
fn parse(paren: proc_macro::Group, iter: &'_ mut IterImpl) -> std::result::Result<Release, Error>
```

*Defined in [`rustversion-1.0.22/src/release.rs:12-14`](../../../.source_1765633015/rustversion-1.0.22/src/release.rs#L12-L14)*

### `try_parse`

```rust
fn try_parse(iter: &'_ mut IterImpl) -> std::result::Result<Release, ()>
```

*Defined in [`rustversion-1.0.22/src/release.rs:16-34`](../../../.source_1765633015/rustversion-1.0.22/src/release.rs#L16-L34)*

