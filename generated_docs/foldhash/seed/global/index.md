*[foldhash](../../index.md) / [seed](../index.md) / [global](index.md)*

---

# Module `global`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`GlobalSeedStorage`](#globalseedstorage) | struct |  |
| [`GlobalSeed`](#globalseed) | struct | An object representing an initialized global seed. |
| [`generate_global_seed`](#generate-global-seed) | fn |  |
| [`UNINIT`](#uninit) | const |  |
| [`LOCKED`](#locked) | const |  |
| [`INIT`](#init) | const |  |

## Structs

### `GlobalSeedStorage`

```rust
struct GlobalSeedStorage {
    state: core::sync::atomic::AtomicU8,
    seed: core::cell::UnsafeCell<SharedSeed>,
}
```

*Defined in [`foldhash-0.2.0/src/seed.rs:181-184`](../../../../.source_1765633015/foldhash-0.2.0/src/seed.rs#L181-L184)*

#### Trait Implementations

##### `impl Any for GlobalSeedStorage`

- <span id="globalseedstorage-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GlobalSeedStorage`

- <span id="globalseedstorage-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GlobalSeedStorage`

- <span id="globalseedstorage-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for GlobalSeedStorage`

- <span id="globalseedstorage-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GlobalSeedStorage`

- <span id="globalseedstorage-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Sync for GlobalSeedStorage`

##### `impl<U> TryFrom for GlobalSeedStorage`

- <span id="globalseedstorage-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="globalseedstorage-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GlobalSeedStorage`

- <span id="globalseedstorage-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="globalseedstorage-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `GlobalSeed`

```rust
struct GlobalSeed {
    _no_accidental_unsafe_init: (),
}
```

*Defined in [`foldhash-0.2.0/src/seed.rs:205-208`](../../../../.source_1765633015/foldhash-0.2.0/src/seed.rs#L205-L208)*

An object representing an initialized global seed.

Does not actually store the seed inside itself, it is a zero-sized type.
This prevents inflating the RandomState size and in turn HashMap's size.

#### Implementations

- <span id="globalseed-new"></span>`fn new() -> Self`

- <span id="globalseed-init-slow"></span>`fn init_slow()`

- <span id="globalseed-get"></span>`fn get(self) -> &'static SharedSeed` — [`SharedSeed`](../index.md#sharedseed)

#### Trait Implementations

##### `impl Any for GlobalSeed`

- <span id="globalseed-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GlobalSeed`

- <span id="globalseed-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GlobalSeed`

- <span id="globalseed-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for GlobalSeed`

- <span id="globalseed-clone"></span>`fn clone(&self) -> GlobalSeed` — [`GlobalSeed`](#globalseed)

##### `impl CloneToUninit for GlobalSeed`

- <span id="globalseed-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for GlobalSeed`

##### `impl Debug for GlobalSeed`

- <span id="globalseed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for GlobalSeed`

- <span id="globalseed-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GlobalSeed`

- <span id="globalseed-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for GlobalSeed`

- <span id="globalseed-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="globalseed-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GlobalSeed`

- <span id="globalseed-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="globalseed-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `generate_global_seed`

```rust
fn generate_global_seed() -> SharedSeed
```

*Defined in [`foldhash-0.2.0/src/seed.rs:140-174`](../../../../.source_1765633015/foldhash-0.2.0/src/seed.rs#L140-L174)*

## Constants

### `UNINIT`
```rust
const UNINIT: u8 = 0u8;
```

*Defined in [`foldhash-0.2.0/src/seed.rs:186`](../../../../.source_1765633015/foldhash-0.2.0/src/seed.rs#L186)*

### `LOCKED`
```rust
const LOCKED: u8 = 1u8;
```

*Defined in [`foldhash-0.2.0/src/seed.rs:187`](../../../../.source_1765633015/foldhash-0.2.0/src/seed.rs#L187)*

### `INIT`
```rust
const INIT: u8 = 2u8;
```

*Defined in [`foldhash-0.2.0/src/seed.rs:188`](../../../../.source_1765633015/foldhash-0.2.0/src/seed.rs#L188)*

