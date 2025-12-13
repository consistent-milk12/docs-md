*[anstream](../index.md) / [adapter](index.md)*

---

# Module `adapter`

Gracefully degrade styled output

## Contents

- [Modules](#modules)
  - [`strip`](#strip)
  - [`wincon`](#wincon)
- [Structs](#structs)
  - [`StripBytes`](#stripbytes)
  - [`StripBytesIter`](#stripbytesiter)
  - [`StripStr`](#stripstr)
  - [`StripStrIter`](#stripstriter)
  - [`StrippedBytes`](#strippedbytes)
  - [`StrippedStr`](#strippedstr)
  - [`WinconBytes`](#winconbytes)
  - [`WinconBytesIter`](#winconbytesiter)
- [Functions](#functions)
  - [`strip_bytes`](#strip-bytes)
  - [`strip_str`](#strip-str)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`strip`](#strip) | mod |  |
| [`wincon`](#wincon) | mod |  |
| [`StripBytes`](#stripbytes) | struct |  |
| [`StripBytesIter`](#stripbytesiter) | struct |  |
| [`StripStr`](#stripstr) | struct |  |
| [`StripStrIter`](#stripstriter) | struct |  |
| [`StrippedBytes`](#strippedbytes) | struct |  |
| [`StrippedStr`](#strippedstr) | struct |  |
| [`WinconBytes`](#winconbytes) | struct |  |
| [`WinconBytesIter`](#winconbytesiter) | struct |  |
| [`strip_bytes`](#strip-bytes) | fn |  |
| [`strip_str`](#strip-str) | fn |  |

## Modules

- [`strip`](strip/index.md)
- [`wincon`](wincon/index.md)

## Structs

### `StripBytes`

```rust
struct StripBytes {
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:245-248`](../../../.source_1765633015/anstream-0.6.21/src/adapter/strip.rs#L245-L248)*

Incrementally strip non-contiguous data

#### Implementations

- <span id="stripbytes-new"></span>`fn new() -> Self`

  Initial state

- <span id="stripbytes-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> StripBytesIter<'s>` — [`StripBytesIter`](strip/index.md#stripbytesiter)

  Strip the next segment of data

#### Trait Implementations

##### `impl Any for StripBytes`

- <span id="stripbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StripBytes`

- <span id="stripbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StripBytes`

- <span id="stripbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StripBytes`

- <span id="stripbytes-clone"></span>`fn clone(&self) -> StripBytes` — [`StripBytes`](strip/index.md#stripbytes)

##### `impl CloneToUninit for StripBytes`

- <span id="stripbytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StripBytes`

- <span id="stripbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripBytes`

- <span id="stripbytes-default"></span>`fn default() -> StripBytes` — [`StripBytes`](strip/index.md#stripbytes)

##### `impl Eq for StripBytes`

##### `impl<T> From for StripBytes`

- <span id="stripbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StripBytes`

- <span id="stripbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for StripBytes`

- <span id="stripbytes-partialeq-eq"></span>`fn eq(&self, other: &StripBytes) -> bool` — [`StripBytes`](strip/index.md#stripbytes)

##### `impl StructuralPartialEq for StripBytes`

##### `impl ToOwned for StripBytes`

- <span id="stripbytes-toowned-type-owned"></span>`type Owned = T`

- <span id="stripbytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stripbytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StripBytes`

- <span id="stripbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stripbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StripBytes`

- <span id="stripbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stripbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StripBytesIter<'s>`

```rust
struct StripBytesIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
    utf8parser: &'s mut Utf8Parser,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:268-272`](../../../.source_1765633015/anstream-0.6.21/src/adapter/strip.rs#L268-L272)*

See [`StripBytes`](strip/index.md)

#### Trait Implementations

##### `impl Any for StripBytesIter<'s>`

- <span id="stripbytesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StripBytesIter<'s>`

- <span id="stripbytesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StripBytesIter<'s>`

- <span id="stripbytesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for StripBytesIter<'s>`

- <span id="stripbytesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StripBytesIter<'s>`

##### `impl<T> From for StripBytesIter<'s>`

- <span id="stripbytesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StripBytesIter<'s>`

- <span id="stripbytesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for StripBytesIter<'s>`

- <span id="stripbytesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stripbytesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="stripbytesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StripBytesIter<'s>`

- <span id="stripbytesiter-iterator-type-item"></span>`type Item = &'s [u8]`

- <span id="stripbytesiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StripBytesIter<'s>`

- <span id="stripbytesiter-partialeq-eq"></span>`fn eq(&self, other: &StripBytesIter<'s>) -> bool` — [`StripBytesIter`](strip/index.md#stripbytesiter)

##### `impl StructuralPartialEq for StripBytesIter<'s>`

##### `impl<U> TryFrom for StripBytesIter<'s>`

- <span id="stripbytesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stripbytesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StripBytesIter<'s>`

- <span id="stripbytesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stripbytesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StripStr`

```rust
struct StripStr {
    state: anstyle_parse::state::State,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:79-81`](../../../.source_1765633015/anstream-0.6.21/src/adapter/strip.rs#L79-L81)*

Incrementally strip non-contiguous data

#### Implementations

- <span id="stripstr-new"></span>`fn new() -> Self`

  Initial state

- <span id="stripstr-strip-next"></span>`fn strip_next<'s>(self: &'s mut Self, data: &'s str) -> StripStrIter<'s>` — [`StripStrIter`](strip/index.md#stripstriter)

  Strip the next segment of data

#### Trait Implementations

##### `impl Any for StripStr`

- <span id="stripstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StripStr`

- <span id="stripstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StripStr`

- <span id="stripstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StripStr`

- <span id="stripstr-clone"></span>`fn clone(&self) -> StripStr` — [`StripStr`](strip/index.md#stripstr)

##### `impl CloneToUninit for StripStr`

- <span id="stripstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StripStr`

- <span id="stripstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StripStr`

- <span id="stripstr-default"></span>`fn default() -> StripStr` — [`StripStr`](strip/index.md#stripstr)

##### `impl Eq for StripStr`

##### `impl<T> From for StripStr`

- <span id="stripstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StripStr`

- <span id="stripstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for StripStr`

- <span id="stripstr-partialeq-eq"></span>`fn eq(&self, other: &StripStr) -> bool` — [`StripStr`](strip/index.md#stripstr)

##### `impl StructuralPartialEq for StripStr`

##### `impl ToOwned for StripStr`

- <span id="stripstr-toowned-type-owned"></span>`type Owned = T`

- <span id="stripstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="stripstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StripStr`

- <span id="stripstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stripstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StripStr`

- <span id="stripstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stripstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StripStrIter<'s>`

```rust
struct StripStrIter<'s> {
    bytes: &'s [u8],
    state: &'s mut anstyle_parse::state::State,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:100-103`](../../../.source_1765633015/anstream-0.6.21/src/adapter/strip.rs#L100-L103)*

See [`StripStr`](strip/index.md)

#### Trait Implementations

##### `impl Any for StripStrIter<'s>`

- <span id="stripstriter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StripStrIter<'s>`

- <span id="stripstriter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StripStrIter<'s>`

- <span id="stripstriter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for StripStrIter<'s>`

- <span id="stripstriter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StripStrIter<'s>`

##### `impl<T> From for StripStrIter<'s>`

- <span id="stripstriter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StripStrIter<'s>`

- <span id="stripstriter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for StripStrIter<'s>`

- <span id="stripstriter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="stripstriter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="stripstriter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StripStrIter<'s>`

- <span id="stripstriter-iterator-type-item"></span>`type Item = &'s str`

- <span id="stripstriter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StripStrIter<'s>`

- <span id="stripstriter-partialeq-eq"></span>`fn eq(&self, other: &StripStrIter<'s>) -> bool` — [`StripStrIter`](strip/index.md#stripstriter)

##### `impl StructuralPartialEq for StripStrIter<'s>`

##### `impl<U> TryFrom for StripStrIter<'s>`

- <span id="stripstriter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="stripstriter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StripStrIter<'s>`

- <span id="stripstriter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="stripstriter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StrippedBytes<'s>`

```rust
struct StrippedBytes<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
    utf8parser: Utf8Parser,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:184-188`](../../../.source_1765633015/anstream-0.6.21/src/adapter/strip.rs#L184-L188)*

See [`strip_bytes`](strip/index.md)

#### Implementations

- <span id="strippedbytes-new"></span>`fn new(bytes: &'s [u8]) -> Self`

  See [`strip_bytes`](strip/index.md)

- <span id="strippedbytes-extend"></span>`fn extend(&mut self, bytes: &'s [u8])`

  Strip the next slice of bytes

  

  Used when the content is in several non-contiguous slices

  

  # Panic

  

  May panic if it is not exhausted / empty

- <span id="strippedbytes-is-empty"></span>`fn is_empty(&self) -> bool`

  Report the bytes has been exhausted

- <span id="strippedbytes-into-vec"></span>`fn into_vec(self) -> Vec<u8>`

  Create a [`Vec`](../../addr2line/maybe_small/index.md) of the printable content

#### Trait Implementations

##### `impl Any for StrippedBytes<'s>`

- <span id="strippedbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StrippedBytes<'s>`

- <span id="strippedbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StrippedBytes<'s>`

- <span id="strippedbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StrippedBytes<'s>`

- <span id="strippedbytes-clone"></span>`fn clone(&self) -> StrippedBytes<'s>` — [`StrippedBytes`](strip/index.md#strippedbytes)

##### `impl CloneToUninit for StrippedBytes<'s>`

- <span id="strippedbytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StrippedBytes<'s>`

- <span id="strippedbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StrippedBytes<'s>`

- <span id="strippedbytes-default"></span>`fn default() -> StrippedBytes<'s>` — [`StrippedBytes`](strip/index.md#strippedbytes)

##### `impl Eq for StrippedBytes<'s>`

##### `impl<T> From for StrippedBytes<'s>`

- <span id="strippedbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StrippedBytes<'s>`

- <span id="strippedbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for StrippedBytes<'s>`

- <span id="strippedbytes-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedbytes-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="strippedbytes-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StrippedBytes<'s>`

- <span id="strippedbytes-iterator-type-item"></span>`type Item = &'s [u8]`

- <span id="strippedbytes-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StrippedBytes<'s>`

- <span id="strippedbytes-partialeq-eq"></span>`fn eq(&self, other: &StrippedBytes<'s>) -> bool` — [`StrippedBytes`](strip/index.md#strippedbytes)

##### `impl StructuralPartialEq for StrippedBytes<'s>`

##### `impl ToOwned for StrippedBytes<'s>`

- <span id="strippedbytes-toowned-type-owned"></span>`type Owned = T`

- <span id="strippedbytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="strippedbytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StrippedBytes<'s>`

- <span id="strippedbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="strippedbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StrippedBytes<'s>`

- <span id="strippedbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="strippedbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StrippedStr<'s>`

```rust
struct StrippedStr<'s> {
    bytes: &'s [u8],
    state: anstyle_parse::state::State,
}
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:28-31`](../../../.source_1765633015/anstream-0.6.21/src/adapter/strip.rs#L28-L31)*

See [`strip_str`](strip/index.md)

#### Implementations

- <span id="strippedstr-new"></span>`fn new(data: &'s str) -> Self`

- <span id="strippedstr-to-string"></span>`fn to_string(&self) -> String`

  Create a [`String`](../../cargo_platform/index.md) of the printable content

#### Trait Implementations

##### `impl Any for StrippedStr<'s>`

- <span id="strippedstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StrippedStr<'s>`

- <span id="strippedstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StrippedStr<'s>`

- <span id="strippedstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StrippedStr<'s>`

- <span id="strippedstr-clone"></span>`fn clone(&self) -> StrippedStr<'s>` — [`StrippedStr`](strip/index.md#strippedstr)

##### `impl CloneToUninit for StrippedStr<'s>`

- <span id="strippedstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StrippedStr<'s>`

- <span id="strippedstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StrippedStr<'s>`

- <span id="strippedstr-default"></span>`fn default() -> StrippedStr<'s>` — [`StrippedStr`](strip/index.md#strippedstr)

##### `impl Display for StrippedStr<'_>`

- <span id="strippedstr-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

  **Note:** this does *not* exhaust the [`Iterator`](../../cargo_docs_md/index.md)

##### `impl Eq for StrippedStr<'s>`

##### `impl<T> From for StrippedStr<'s>`

- <span id="strippedstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StrippedStr<'s>`

- <span id="strippedstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for StrippedStr<'s>`

- <span id="strippedstr-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="strippedstr-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="strippedstr-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for StrippedStr<'s>`

- <span id="strippedstr-iterator-type-item"></span>`type Item = &'s str`

- <span id="strippedstr-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for StrippedStr<'s>`

- <span id="strippedstr-partialeq-eq"></span>`fn eq(&self, other: &StrippedStr<'s>) -> bool` — [`StrippedStr`](strip/index.md#strippedstr)

##### `impl StructuralPartialEq for StrippedStr<'s>`

##### `impl ToOwned for StrippedStr<'s>`

- <span id="strippedstr-toowned-type-owned"></span>`type Owned = T`

- <span id="strippedstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="strippedstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for StrippedStr<'s>`

- <span id="strippedstr-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for StrippedStr<'s>`

- <span id="strippedstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="strippedstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StrippedStr<'s>`

- <span id="strippedstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="strippedstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WinconBytes`

```rust
struct WinconBytes {
    parser: anstyle_parse::Parser,
    capture: WinconCapture,
}
```

*Defined in [`anstream-0.6.21/src/adapter/wincon.rs:3-6`](../../../.source_1765633015/anstream-0.6.21/src/adapter/wincon.rs#L3-L6)*

Incrementally convert to wincon calls for non-contiguous data

#### Implementations

- <span id="winconbytes-new"></span>`fn new() -> Self`

  Initial state

- <span id="winconbytes-extract-next"></span>`fn extract_next<'s>(self: &'s mut Self, bytes: &'s [u8]) -> WinconBytesIter<'s>` — [`WinconBytesIter`](wincon/index.md#winconbytesiter)

  Strip the next segment of data

#### Trait Implementations

##### `impl Any for WinconBytes`

- <span id="winconbytes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WinconBytes`

- <span id="winconbytes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WinconBytes`

- <span id="winconbytes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WinconBytes`

- <span id="winconbytes-clone"></span>`fn clone(&self) -> WinconBytes` — [`WinconBytes`](wincon/index.md#winconbytes)

##### `impl CloneToUninit for WinconBytes`

- <span id="winconbytes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for WinconBytes`

- <span id="winconbytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WinconBytes`

- <span id="winconbytes-default"></span>`fn default() -> WinconBytes` — [`WinconBytes`](wincon/index.md#winconbytes)

##### `impl Eq for WinconBytes`

##### `impl<T> From for WinconBytes`

- <span id="winconbytes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WinconBytes`

- <span id="winconbytes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for WinconBytes`

- <span id="winconbytes-partialeq-eq"></span>`fn eq(&self, other: &WinconBytes) -> bool` — [`WinconBytes`](wincon/index.md#winconbytes)

##### `impl StructuralPartialEq for WinconBytes`

##### `impl ToOwned for WinconBytes`

- <span id="winconbytes-toowned-type-owned"></span>`type Owned = T`

- <span id="winconbytes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="winconbytes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WinconBytes`

- <span id="winconbytes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="winconbytes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WinconBytes`

- <span id="winconbytes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="winconbytes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `WinconBytesIter<'s>`

```rust
struct WinconBytesIter<'s> {
    bytes: &'s [u8],
    parser: &'s mut anstyle_parse::Parser,
    capture: &'s mut WinconCapture,
}
```

*Defined in [`anstream-0.6.21/src/adapter/wincon.rs:28-32`](../../../.source_1765633015/anstream-0.6.21/src/adapter/wincon.rs#L28-L32)*

See [`WinconBytes`](wincon/index.md)

#### Trait Implementations

##### `impl Any for WinconBytesIter<'s>`

- <span id="winconbytesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WinconBytesIter<'s>`

- <span id="winconbytesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WinconBytesIter<'s>`

- <span id="winconbytesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for WinconBytesIter<'s>`

- <span id="winconbytesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for WinconBytesIter<'s>`

##### `impl<T> From for WinconBytesIter<'s>`

- <span id="winconbytesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WinconBytesIter<'s>`

- <span id="winconbytesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for WinconBytesIter<'s>`

- <span id="winconbytesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="winconbytesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="winconbytesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for WinconBytesIter<'_>`

- <span id="winconbytesiter-iterator-type-item"></span>`type Item = (Style, String)`

- <span id="winconbytesiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl PartialEq for WinconBytesIter<'s>`

- <span id="winconbytesiter-partialeq-eq"></span>`fn eq(&self, other: &WinconBytesIter<'s>) -> bool` — [`WinconBytesIter`](wincon/index.md#winconbytesiter)

##### `impl StructuralPartialEq for WinconBytesIter<'s>`

##### `impl<U> TryFrom for WinconBytesIter<'s>`

- <span id="winconbytesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="winconbytesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WinconBytesIter<'s>`

- <span id="winconbytesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="winconbytesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `strip_bytes`

```rust
fn strip_bytes(data: &[u8]) -> StrippedBytes<'_>
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:178-180`](../../../.source_1765633015/anstream-0.6.21/src/adapter/strip.rs#L178-L180)*

Strip ANSI escapes from bytes, returning the printable content

This can be used to take output from a program that includes escape sequences and write it
somewhere that does not easily support them, such as a log file.

# Example

```rust
use std::io::Write as _;

let styled_text = "\x1b[32mfoo\x1b[m bar";
let plain_str = anstream::adapter::strip_bytes(styled_text.as_bytes()).into_vec();
assert_eq!(plain_str.as_slice(), &b"foo bar"[..]);
```

### `strip_str`

```rust
fn strip_str(data: &str) -> StrippedStr<'_>
```

*Defined in [`anstream-0.6.21/src/adapter/strip.rs:22-24`](../../../.source_1765633015/anstream-0.6.21/src/adapter/strip.rs#L22-L24)*

Strip ANSI escapes from a `&str`, returning the printable content

This can be used to take output from a program that includes escape sequences and write it
somewhere that does not easily support them, such as a log file.

For non-contiguous data, see [`StripStr`](strip/index.md).

# Example

```rust
use std::io::Write as _;

let styled_text = "\x1b[32mfoo\x1b[m bar";
let plain_str = anstream::adapter::strip_str(&styled_text).to_string();
assert_eq!(plain_str, "foo bar");
```

