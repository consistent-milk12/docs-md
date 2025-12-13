*[foldhash](../index.md) / [fast](index.md)*

---

# Module `fast`

The foldhash implementation optimized for speed.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FoldHasher`](#foldhasher) | struct | A [`Hasher`] instance implementing foldhash, optimized for speed. |
| [`RandomState`](#randomstate) | struct | A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly initialized. |
| [`SeedableRandomState`](#seedablerandomstate) | struct | A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that is randomly initialized by default, but can also be initialized with a specific seed. |
| [`FixedState`](#fixedstate) | struct | A [`BuildHasher`] for [`fast::FoldHasher`](FoldHasher) that always has the same fixed seed. |

## Structs

### `FoldHasher<'a>`

```rust
struct FoldHasher<'a> {
    accumulator: u64,
    sponge: u128,
    sponge_len: u8,
    seeds: &'a [u64; 6],
}
```

*Defined in [`foldhash-0.2.0/src/fast.rs:14-19`](../../../.source_1765521767/foldhash-0.2.0/src/fast.rs#L14-L19)*

A `Hasher` instance implementing foldhash, optimized for speed.

While you can create one directly with `FoldHasher::with_seed`, you
most likely want to use [`RandomState`](#randomstate), [`SeedableRandomState`](#seedablerandomstate) or
[`FixedState`](#fixedstate) to create [`FoldHasher`](#foldhasher)s.

#### Implementations

- <span id="foldhasher-with-seed"></span>`const fn with_seed(per_hasher_seed: u64, shared_seed: &'a SharedSeed) -> FoldHasher<'a>` — [`SharedSeed`](../seed/index.md#sharedseed), [`FoldHasher`](#foldhasher)

  Initializes this [`FoldHasher`](#foldhasher) with the given per-hasher seed and

  [`SharedSeed`](../seed/index.md).

- <span id="foldhasher-write-num"></span>`fn write_num<T: Into<u128>>(&mut self, x: T)`

#### Trait Implementations

##### `impl Any for FoldHasher<'a>`

- <span id="foldhasher-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FoldHasher<'a>`

- <span id="foldhasher-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FoldHasher<'a>`

- <span id="foldhasher-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for FoldHasher<'a>`

- <span id="foldhasher-clone"></span>`fn clone(&self) -> FoldHasher<'a>` — [`FoldHasher`](#foldhasher)

##### `impl CloneToUninit for FoldHasher<'a>`

- <span id="foldhasher-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for FoldHasher<'a>`

- <span id="foldhasher-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hasher for FoldHasher<'a>`

- <span id="foldhasher-hasher-write"></span>`fn write(&mut self, bytes: &[u8])`

- <span id="foldhasher-hasher-write-u8"></span>`fn write_u8(&mut self, i: u8)`

- <span id="foldhasher-hasher-write-u16"></span>`fn write_u16(&mut self, i: u16)`

- <span id="foldhasher-hasher-write-u32"></span>`fn write_u32(&mut self, i: u32)`

- <span id="foldhasher-hasher-write-u64"></span>`fn write_u64(&mut self, i: u64)`

- <span id="foldhasher-hasher-write-u128"></span>`fn write_u128(&mut self, i: u128)`

- <span id="foldhasher-hasher-write-usize"></span>`fn write_usize(&mut self, i: usize)`

- <span id="foldhasher-hasher-finish"></span>`fn finish(&self) -> u64`

##### `impl<U> Into for FoldHasher<'a>`

- <span id="foldhasher-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for FoldHasher<'a>`

- <span id="foldhasher-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="foldhasher-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FoldHasher<'a>`

- <span id="foldhasher-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="foldhasher-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RandomState`

```rust
struct RandomState {
    per_hasher_seed: u64,
    global_seed: global::GlobalSeed,
}
```

*Defined in [`foldhash-0.2.0/src/fast.rs:127-130`](../../../.source_1765521767/foldhash-0.2.0/src/fast.rs#L127-L130)*

A [`BuildHasher`](../../serde_core/lib/index.md) for [`fast::FoldHasher`](FoldHasher) that is randomly initialized.

#### Trait Implementations

##### `impl Any for RandomState`

- <span id="randomstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RandomState`

- <span id="randomstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RandomState`

- <span id="randomstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl BuildHasher for RandomState`

- <span id="randomstate-buildhasher-type-hasher"></span>`type Hasher = FoldHasher<'static>`

- <span id="randomstate-buildhasher-build-hasher"></span>`fn build_hasher(&self) -> FoldHasher<'static>` — [`FoldHasher`](#foldhasher)

##### `impl Clone for RandomState`

- <span id="randomstate-clone"></span>`fn clone(&self) -> RandomState` — [`RandomState`](#randomstate)

##### `impl CloneToUninit for RandomState`

- <span id="randomstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RandomState`

- <span id="randomstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RandomState`

- <span id="randomstate-default"></span>`fn default() -> Self`

##### `impl<T> From for RandomState`

- <span id="randomstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RandomState`

- <span id="randomstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for RandomState`

- <span id="randomstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="randomstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RandomState`

- <span id="randomstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="randomstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SeedableRandomState`

```rust
struct SeedableRandomState {
    per_hasher_seed: u64,
    shared_seed: &'static crate::seed::SharedSeed,
}
```

*Defined in [`foldhash-0.2.0/src/fast.rs:157-160`](../../../.source_1765521767/foldhash-0.2.0/src/fast.rs#L157-L160)*

A [`BuildHasher`](../../serde_core/lib/index.md) for [`fast::FoldHasher`](FoldHasher) that is randomly
initialized by default, but can also be initialized with a specific seed.

This can be useful for e.g. testing, but the downside is that this type
has a size of 16 bytes rather than the 8 bytes [`RandomState`](#randomstate) is.

#### Implementations

- <span id="seedablerandomstate-random"></span>`fn random() -> Self`

  Generates a random [`SeedableRandomState`](#seedablerandomstate), similar to [`RandomState`](#randomstate).

- <span id="seedablerandomstate-fixed"></span>`fn fixed() -> Self`

  Generates a fixed [`SeedableRandomState`](#seedablerandomstate), similar to [`FixedState`](#fixedstate).

- <span id="seedablerandomstate-with-seed"></span>`fn with_seed(per_hasher_seed: u64, shared_seed: &'static SharedSeed) -> Self` — [`SharedSeed`](../seed/index.md#sharedseed)

  Generates a [`SeedableRandomState`](#seedablerandomstate) with the given per-hasher seed

  and [`SharedSeed`](../seed/index.md).

#### Trait Implementations

##### `impl Any for SeedableRandomState`

- <span id="seedablerandomstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SeedableRandomState`

- <span id="seedablerandomstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SeedableRandomState`

- <span id="seedablerandomstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl BuildHasher for SeedableRandomState`

- <span id="seedablerandomstate-buildhasher-type-hasher"></span>`type Hasher = FoldHasher<'static>`

- <span id="seedablerandomstate-buildhasher-build-hasher"></span>`fn build_hasher(&self) -> FoldHasher<'static>` — [`FoldHasher`](#foldhasher)

##### `impl Clone for SeedableRandomState`

- <span id="seedablerandomstate-clone"></span>`fn clone(&self) -> SeedableRandomState` — [`SeedableRandomState`](#seedablerandomstate)

##### `impl CloneToUninit for SeedableRandomState`

- <span id="seedablerandomstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SeedableRandomState`

- <span id="seedablerandomstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for SeedableRandomState`

- <span id="seedablerandomstate-default"></span>`fn default() -> Self`

##### `impl<T> From for SeedableRandomState`

- <span id="seedablerandomstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SeedableRandomState`

- <span id="seedablerandomstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SeedableRandomState`

- <span id="seedablerandomstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="seedablerandomstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SeedableRandomState`

- <span id="seedablerandomstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="seedablerandomstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FixedState`

```rust
struct FixedState {
    per_hasher_seed: u64,
}
```

*Defined in [`foldhash-0.2.0/src/fast.rs:213-215`](../../../.source_1765521767/foldhash-0.2.0/src/fast.rs#L213-L215)*

A [`BuildHasher`](../../serde_core/lib/index.md) for [`fast::FoldHasher`](FoldHasher) that always has the same fixed seed.

Not recommended unless you absolutely need determinism.

#### Implementations

- <span id="fixedstate-with-seed"></span>`const fn with_seed(per_hasher_seed: u64) -> Self`

  Creates a [`FixedState`](#fixedstate) with the given per-hasher-seed.

#### Trait Implementations

##### `impl Any for FixedState`

- <span id="fixedstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FixedState`

- <span id="fixedstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FixedState`

- <span id="fixedstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl BuildHasher for FixedState`

- <span id="fixedstate-buildhasher-type-hasher"></span>`type Hasher = FoldHasher<'static>`

- <span id="fixedstate-buildhasher-build-hasher"></span>`fn build_hasher(&self) -> FoldHasher<'static>` — [`FoldHasher`](#foldhasher)

##### `impl Clone for FixedState`

- <span id="fixedstate-clone"></span>`fn clone(&self) -> FixedState` — [`FixedState`](#fixedstate)

##### `impl CloneToUninit for FixedState`

- <span id="fixedstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for FixedState`

- <span id="fixedstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FixedState`

- <span id="fixedstate-default"></span>`fn default() -> Self`

##### `impl<T> From for FixedState`

- <span id="fixedstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FixedState`

- <span id="fixedstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for FixedState`

- <span id="fixedstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="fixedstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FixedState`

- <span id="fixedstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="fixedstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

