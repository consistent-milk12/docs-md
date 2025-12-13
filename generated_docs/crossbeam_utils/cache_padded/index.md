*[crossbeam_utils](../index.md) / [cache_padded](index.md)*

---

# Module `cache_padded`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CachePadded`](#cachepadded) | struct | Pads and aligns a value to the length of a cache line. |

## Structs

### `CachePadded<T>`

```rust
struct CachePadded<T> {
    value: T,
}
```

*Defined in [`crossbeam-utils-0.8.21/src/cache_padded.rs:148-150`](../../../.source_1765633015/crossbeam-utils-0.8.21/src/cache_padded.rs#L148-L150)*

Pads and aligns a value to the length of a cache line.

In concurrent programming, sometimes it is desirable to make sure commonly accessed pieces of
data are not placed into the same cache line. Updating an atomic value invalidates the whole
cache line it belongs to, which makes the next access to the same cache line slower for other
CPU cores. Use `CachePadded` to ensure updating one piece of data doesn't invalidate other
cached data.

# Size and alignment

Cache lines are assumed to be N bytes long, depending on the architecture:

* On x86-64, aarch64, and powerpc64, N = 128.
* On arm, mips, mips64, sparc, and hexagon, N = 32.
* On m68k, N = 16.
* On s390x, N = 256.
* On all others, N = 64.

Note that N is just a reasonable guess and is not guaranteed to match the actual cache line
length of the machine the program is running on. On modern Intel architectures, spatial
prefetcher is pulling pairs of 64-byte cache lines at a time, so we pessimistically assume that
cache lines are 128 bytes long.

The size of `CachePadded<T>` is the smallest multiple of N bytes large enough to accommodate
a value of type `T`.

The alignment of `CachePadded<T>` is the maximum of N bytes and the alignment of `T`.

# Examples

Alignment and padding:

```rust
use crossbeam_utils::CachePadded;

let array = [CachePadded::new(1i8), CachePadded::new(2i8)];
let addr1 = &*array[0] as *const i8 as usize;
let addr2 = &*array[1] as *const i8 as usize;

assert!(addr2 - addr1 >= 32);
assert_eq!(addr1 % 32, 0);
assert_eq!(addr2 % 32, 0);
```

When building a concurrent queue with a head and a tail index, it is wise to place them in
different cache lines so that concurrent threads pushing and popping elements don't invalidate
each other's cache lines:

```rust
use crossbeam_utils::CachePadded;
use std::sync::atomic::AtomicUsize;

struct Queue<T> {
    head: CachePadded<AtomicUsize>,
    tail: CachePadded<AtomicUsize>,
    buffer: *mut T,
}
```

#### Implementations

- <span id="cachepadded-new"></span>`const fn new(t: T) -> CachePadded<T>` — [`CachePadded`](#cachepadded)

  Pads and aligns a value to the length of a cache line.

  

  # Examples

  

  ```rust

  use crossbeam_utils::CachePadded;

  

  let padded_value = CachePadded::new(1);

  ```

- <span id="cachepadded-into-inner"></span>`fn into_inner(self) -> T`

  Returns the inner value.

  

  # Examples

  

  ```rust

  use crossbeam_utils::CachePadded;

  

  let padded_value = CachePadded::new(7);

  let value = padded_value.into_inner();

  assert_eq!(value, 7);

  ```

#### Trait Implementations

##### `impl<T> Any for CachePadded<T>`

- <span id="cachepadded-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CachePadded<T>`

- <span id="cachepadded-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CachePadded<T>`

- <span id="cachepadded-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: clone::Clone> Clone for CachePadded<T>`

- <span id="cachepadded-clone"></span>`fn clone(&self) -> CachePadded<T>` — [`CachePadded`](#cachepadded)

##### `impl<T> CloneToUninit for CachePadded<T>`

- <span id="cachepadded-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T: marker::Copy> Copy for CachePadded<T>`

##### `impl<T: fmt::Debug> Debug for CachePadded<T>`

- <span id="cachepadded-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default> Default for CachePadded<T>`

- <span id="cachepadded-default"></span>`fn default() -> CachePadded<T>` — [`CachePadded`](#cachepadded)

##### `impl<T> Deref for CachePadded<T>`

- <span id="cachepadded-deref-type-target"></span>`type Target = T`

- <span id="cachepadded-deref"></span>`fn deref(&self) -> &T`

##### `impl<T> DerefMut for CachePadded<T>`

- <span id="cachepadded-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut T`

##### `impl<T: fmt::Display> Display for CachePadded<T>`

- <span id="cachepadded-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for CachePadded<T>`

##### `impl<T> From for CachePadded<T>`

- <span id="cachepadded-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: hash::Hash> Hash for CachePadded<T>`

- <span id="cachepadded-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T, U> Into for CachePadded<T>`

- <span id="cachepadded-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T: cmp::PartialEq> PartialEq for CachePadded<T>`

- <span id="cachepadded-partialeq-eq"></span>`fn eq(&self, other: &CachePadded<T>) -> bool` — [`CachePadded`](#cachepadded)

##### `impl<T> Receiver for CachePadded<T>`

- <span id="cachepadded-receiver-type-target"></span>`type Target = T`

##### `impl<T: Send> Send for CachePadded<T>`

##### `impl<T> StructuralPartialEq for CachePadded<T>`

##### `impl<T: Sync> Sync for CachePadded<T>`

##### `impl<T> ToOwned for CachePadded<T>`

- <span id="cachepadded-toowned-type-owned"></span>`type Owned = T`

- <span id="cachepadded-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cachepadded-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<T> ToString for CachePadded<T>`

- <span id="cachepadded-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for CachePadded<T>`

- <span id="cachepadded-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cachepadded-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for CachePadded<T>`

- <span id="cachepadded-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cachepadded-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

