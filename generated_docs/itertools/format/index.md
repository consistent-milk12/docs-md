*[itertools](../index.md) / [format](index.md)*

---

# Module `format`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FormatWith`](#formatwith) | struct | Format all iterator elements lazily, separated by `sep`. |
| [`Format`](#format) | struct | Format all iterator elements lazily, separated by `sep`. |
| [`new_format`](#new-format) | fn |  |
| [`new_format_default`](#new-format-default) | fn |  |
| [`impl_format!`](#impl-format) | macro |  |

## Structs

### `FormatWith<'a, I, F>`

```rust
struct FormatWith<'a, I, F> {
    sep: &'a str,
    inner: std::cell::Cell<Option<(I, F)>>,
}
```

*Defined in [`itertools-0.14.0/src/format.rs:10-14`](../../../.source_1765894658/itertools-0.14.0/src/format.rs#L10-L14)*

Format all iterator elements lazily, separated by `sep`.

The format value can only be formatted once, after that the iterator is
exhausted.

See [`.format_with()`](crate::Itertools::format_with) for more information.

#### Fields

- **`inner`**: `std::cell::Cell<Option<(I, F)>>`

  `FormatWith` uses interior mutability because `Display::fmt` takes `&self`.

#### Trait Implementations

##### `impl Any for FormatWith<'a, I, F>`

- <span id="formatwith-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FormatWith<'a, I, F>`

- <span id="formatwith-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FormatWith<'a, I, F>`

- <span id="formatwith-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I, F> Clone for FormatWith<'_, I, F>`

- <span id="formatwith-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for FormatWith<'a, I, F>`

- <span id="formatwith-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I, F> Debug for FormatWith<'_, I, F>`

- <span id="formatwith-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I, F> Display for FormatWith<'_, I, F>`

- <span id="formatwith-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FormatWith<'a, I, F>`

- <span id="formatwith-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FormatWith<'a, I, F>`

- <span id="formatwith-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FormatWith<'a, I, F>`

##### `impl ToOwned for FormatWith<'a, I, F>`

- <span id="formatwith-toowned-type-owned"></span>`type Owned = T`

- <span id="formatwith-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="formatwith-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for FormatWith<'a, I, F>`

- <span id="formatwith-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for FormatWith<'a, I, F>`

- <span id="formatwith-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="formatwith-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FormatWith<'a, I, F>`

- <span id="formatwith-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="formatwith-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Format<'a, I>`

```rust
struct Format<'a, I> {
    sep: &'a str,
    inner: std::cell::Cell<Option<I>>,
}
```

*Defined in [`itertools-0.14.0/src/format.rs:23-27`](../../../.source_1765894658/itertools-0.14.0/src/format.rs#L23-L27)*

Format all iterator elements lazily, separated by `sep`.

The format value can only be formatted once, after that the iterator is
exhausted.

See [`.format()`](crate::Itertools::format)
for more information.

#### Fields

- **`inner`**: `std::cell::Cell<Option<I>>`

  `Format` uses interior mutability because `Display::fmt` takes `&self`.

#### Implementations

- <span id="format-format"></span>`fn format(&self, f: &mut fmt::Formatter<'_>, cb: fn(&<I as >::Item, &mut fmt::Formatter<'_>) -> fmt::Result) -> fmt::Result`

#### Trait Implementations

##### `impl Any for Format<'a, I>`

- <span id="format-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<I> Binary for Format<'a, I>`

- <span id="format-binary-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Borrow for Format<'a, I>`

- <span id="format-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Format<'a, I>`

- <span id="format-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<I> Clone for Format<'_, I>`

- <span id="format-clone"></span>`fn clone(&self) -> Self`

##### `impl CloneToUninit for Format<'a, I>`

- <span id="format-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<I> Debug for Format<'a, I>`

- <span id="format-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> Display for Format<'a, I>`

- <span id="format-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Format<'a, I>`

- <span id="format-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Format<'a, I>`

- <span id="format-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for Format<'a, I>`

##### `impl<I> LowerExp for Format<'a, I>`

- <span id="format-lowerexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> LowerHex for Format<'a, I>`

- <span id="format-lowerhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> Octal for Format<'a, I>`

- <span id="format-octal-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> Pointer for Format<'a, I>`

- <span id="format-pointer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToOwned for Format<'a, I>`

- <span id="format-toowned-type-owned"></span>`type Owned = T`

- <span id="format-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="format-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Format<'a, I>`

- <span id="format-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Format<'a, I>`

- <span id="format-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="format-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Format<'a, I>`

- <span id="format-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="format-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<I> UpperExp for Format<'a, I>`

- <span id="format-upperexp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> UpperHex for Format<'a, I>`

- <span id="format-upperhex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `new_format`

```rust
fn new_format<I, F>(iter: I, separator: &str, f: F) -> FormatWith<'_, I, F>
where
    I: Iterator,
    F: FnMut(<I as >::Item, &mut dyn FnMut(&dyn fmt::Display) -> fmt::Result) -> fmt::Result
```

*Defined in [`itertools-0.14.0/src/format.rs:29-38`](../../../.source_1765894658/itertools-0.14.0/src/format.rs#L29-L38)*

### `new_format_default`

```rust
fn new_format_default<I>(iter: I, separator: &str) -> Format<'_, I>
where
    I: Iterator
```

*Defined in [`itertools-0.14.0/src/format.rs:40-48`](../../../.source_1765894658/itertools-0.14.0/src/format.rs#L40-L48)*

## Macros

### `impl_format!`

*Defined in [`itertools-0.14.0/src/format.rs:111-124`](../../../.source_1765894658/itertools-0.14.0/src/format.rs#L111-L124)*

