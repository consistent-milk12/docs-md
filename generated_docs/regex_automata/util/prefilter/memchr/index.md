*[regex_automata](../../../index.md) / [util](../../index.md) / [prefilter](../index.md) / [memchr](index.md)*

---

# Module `memchr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Memchr`](#memchr) | struct |  |
| [`Memchr2`](#memchr2) | struct |  |
| [`Memchr3`](#memchr3) | struct |  |

## Structs

### `Memchr`

```rust
struct Memchr(u8);
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/memchr.rs:7`](../../../../../.source_1765521767/regex-automata-0.4.13/src/util/prefilter/memchr.rs#L7)*

#### Implementations

- <span id="memchr-new"></span>`fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memchr>` — [`MatchKind`](../../../index.md#matchkind), [`Memchr`](#memchr)

#### Trait Implementations

##### `impl Any for Memchr`

- <span id="memchr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Memchr`

- <span id="memchr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Memchr`

- <span id="memchr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Memchr`

- <span id="memchr-clone"></span>`fn clone(&self) -> Memchr` — [`Memchr`](#memchr)

##### `impl CloneToUninit for Memchr`

- <span id="memchr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Memchr`

- <span id="memchr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Memchr`

- <span id="memchr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Memchr`

- <span id="memchr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for Memchr`

- <span id="memchr-prefilteri-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="memchr-prefilteri-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="memchr-prefilteri-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="memchr-prefilteri-is-fast"></span>`fn is_fast(&self) -> bool`

##### `impl ToOwned for Memchr`

- <span id="memchr-toowned-type-owned"></span>`type Owned = T`

- <span id="memchr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="memchr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Memchr`

- <span id="memchr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memchr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Memchr`

- <span id="memchr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memchr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Memchr2`

```rust
struct Memchr2(u8, u8);
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/memchr.rs:66`](../../../../../.source_1765521767/regex-automata-0.4.13/src/util/prefilter/memchr.rs#L66)*

#### Implementations

- <span id="memchr2-new"></span>`fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memchr2>` — [`MatchKind`](../../../index.md#matchkind), [`Memchr2`](#memchr2)

#### Trait Implementations

##### `impl Any for Memchr2`

- <span id="memchr2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Memchr2`

- <span id="memchr2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Memchr2`

- <span id="memchr2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Memchr2`

- <span id="memchr2-clone"></span>`fn clone(&self) -> Memchr2` — [`Memchr2`](#memchr2)

##### `impl CloneToUninit for Memchr2`

- <span id="memchr2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Memchr2`

- <span id="memchr2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Memchr2`

- <span id="memchr2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Memchr2`

- <span id="memchr2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for Memchr2`

- <span id="memchr2-prefilteri-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="memchr2-prefilteri-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="memchr2-prefilteri-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="memchr2-prefilteri-is-fast"></span>`fn is_fast(&self) -> bool`

##### `impl ToOwned for Memchr2`

- <span id="memchr2-toowned-type-owned"></span>`type Owned = T`

- <span id="memchr2-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="memchr2-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Memchr2`

- <span id="memchr2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memchr2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Memchr2`

- <span id="memchr2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memchr2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Memchr3`

```rust
struct Memchr3(u8, u8, u8);
```

*Defined in [`regex-automata-0.4.13/src/util/prefilter/memchr.rs:127`](../../../../../.source_1765521767/regex-automata-0.4.13/src/util/prefilter/memchr.rs#L127)*

#### Implementations

- <span id="memchr3-new"></span>`fn new<B: AsRef<[u8]>>(_kind: MatchKind, needles: &[B]) -> Option<Memchr3>` — [`MatchKind`](../../../index.md#matchkind), [`Memchr3`](#memchr3)

#### Trait Implementations

##### `impl Any for Memchr3`

- <span id="memchr3-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Memchr3`

- <span id="memchr3-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Memchr3`

- <span id="memchr3-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Memchr3`

- <span id="memchr3-clone"></span>`fn clone(&self) -> Memchr3` — [`Memchr3`](#memchr3)

##### `impl CloneToUninit for Memchr3`

- <span id="memchr3-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Memchr3`

- <span id="memchr3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Memchr3`

- <span id="memchr3-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Memchr3`

- <span id="memchr3-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for Memchr3`

- <span id="memchr3-prefilteri-find"></span>`fn find(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="memchr3-prefilteri-prefix"></span>`fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>` — [`Span`](../../../index.md#span)

- <span id="memchr3-prefilteri-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="memchr3-prefilteri-is-fast"></span>`fn is_fast(&self) -> bool`

##### `impl ToOwned for Memchr3`

- <span id="memchr3-toowned-type-owned"></span>`type Owned = T`

- <span id="memchr3-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="memchr3-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Memchr3`

- <span id="memchr3-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memchr3-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Memchr3`

- <span id="memchr3-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memchr3-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

