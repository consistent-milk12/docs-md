*[portable_atomic](../../../index.md) / [imp](../../index.md) / [atomic128](../index.md) / [x86_64](index.md)*

---

# Module `x86_64`

## Modules

- [`fallback`](fallback/index.md) - 
- [`detect`](detect/index.md) - 

## Structs

### `AtomicI128`

```rust
struct AtomicI128 {
    v: core::cell::UnsafeCell<i128>,
}
```

#### Implementations

- `fn add(self: &Self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md)

- `fn sub(self: &Self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md)

- `fn and(self: &Self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md)

- `fn or(self: &Self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md)

- `fn xor(self: &Self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md)

#### Trait Implementations

##### `impl Sync for AtomicI128`

### `AtomicU128`

```rust
struct AtomicU128 {
    v: core::cell::UnsafeCell<u128>,
}
```

#### Implementations

- `const fn new(v: u128) -> Self`

- `fn is_lock_free() -> bool`

- `const IS_ALWAYS_LOCK_FREE: bool`

- `fn load(self: &Self, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn store(self: &Self, val: u128, order: Ordering)` — [`Ordering`](../../../index.md)

- `fn swap(self: &Self, val: u128, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn compare_exchange(self: &Self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128>` — [`Ordering`](../../../index.md)

- `fn compare_exchange_weak(self: &Self, current: u128, new: u128, success: Ordering, failure: Ordering) -> Result<u128, u128>` — [`Ordering`](../../../index.md)

- `fn fetch_add(self: &Self, val: u128, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn fetch_sub(self: &Self, val: u128, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn fetch_and(self: &Self, val: u128, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn fetch_nand(self: &Self, val: u128, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn fetch_or(self: &Self, val: u128, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn fetch_xor(self: &Self, val: u128, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn fetch_max(self: &Self, val: u128, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn fetch_min(self: &Self, val: u128, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn fetch_not(self: &Self, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn not(self: &Self, order: Ordering)` — [`Ordering`](../../../index.md)

- `fn fetch_neg(self: &Self, order: Ordering) -> u128` — [`Ordering`](../../../index.md)

- `fn neg(self: &Self, order: Ordering)` — [`Ordering`](../../../index.md)

- `const fn as_ptr(self: &Self) -> *mut u128`

#### Trait Implementations

##### `impl Sync for AtomicU128`

## Functions

### `cmpxchg16b`

```rust
unsafe fn cmpxchg16b(dst: *mut u128, old: u128, new: u128) -> (u128, bool)
```

### `atomic_load_vmovdqa`

```rust
unsafe fn atomic_load_vmovdqa(src: *mut u128) -> u128
```

### `atomic_store_vmovdqa`

```rust
unsafe fn atomic_store_vmovdqa(dst: *mut u128, val: u128, order: core::sync::atomic::Ordering)
```

### `atomic_load`

```rust
unsafe fn atomic_load(src: *mut u128, _order: core::sync::atomic::Ordering) -> u128
```

### `atomic_load_cmpxchg16b`

```rust
unsafe fn atomic_load_cmpxchg16b(src: *mut u128) -> u128
```

### `atomic_store`

```rust
unsafe fn atomic_store(dst: *mut u128, val: u128, order: core::sync::atomic::Ordering)
```

### `atomic_store_cmpxchg16b`

```rust
unsafe fn atomic_store_cmpxchg16b(dst: *mut u128, val: u128)
```

### `atomic_compare_exchange`

```rust
unsafe fn atomic_compare_exchange(dst: *mut u128, old: u128, new: u128, _success: core::sync::atomic::Ordering, _failure: core::sync::atomic::Ordering) -> Result<u128, u128>
```

### `atomic_swap_cmpxchg16b`

```rust
unsafe fn atomic_swap_cmpxchg16b(dst: *mut u128, val: u128, _order: core::sync::atomic::Ordering) -> u128
```

### `atomic_add_cmpxchg16b`

```rust
unsafe fn atomic_add_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_sub_cmpxchg16b`

```rust
unsafe fn atomic_sub_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_and_cmpxchg16b`

```rust
unsafe fn atomic_and_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_nand_cmpxchg16b`

```rust
unsafe fn atomic_nand_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_or_cmpxchg16b`

```rust
unsafe fn atomic_or_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_xor_cmpxchg16b`

```rust
unsafe fn atomic_xor_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_not_cmpxchg16b`

```rust
unsafe fn atomic_not_cmpxchg16b(dst: *mut u128, _order: Ordering) -> u128
```

### `atomic_neg_cmpxchg16b`

```rust
unsafe fn atomic_neg_cmpxchg16b(dst: *mut u128, _order: Ordering) -> u128
```

### `atomic_max_cmpxchg16b`

```rust
unsafe fn atomic_max_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_umax_cmpxchg16b`

```rust
unsafe fn atomic_umax_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_min_cmpxchg16b`

```rust
unsafe fn atomic_min_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_umin_cmpxchg16b`

```rust
unsafe fn atomic_umin_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_swap`

```rust
unsafe fn atomic_swap(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_add`

```rust
unsafe fn atomic_add(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_sub`

```rust
unsafe fn atomic_sub(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_and`

```rust
unsafe fn atomic_and(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_nand`

```rust
unsafe fn atomic_nand(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_or`

```rust
unsafe fn atomic_or(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_xor`

```rust
unsafe fn atomic_xor(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_max`

```rust
unsafe fn atomic_max(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_umax`

```rust
unsafe fn atomic_umax(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_min`

```rust
unsafe fn atomic_min(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_umin`

```rust
unsafe fn atomic_umin(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

### `atomic_not`

```rust
unsafe fn atomic_not(dst: *mut u128, _order: Ordering) -> u128
```

### `atomic_neg`

```rust
unsafe fn atomic_neg(dst: *mut u128, _order: Ordering) -> u128
```

### `is_lock_free`

```rust
fn is_lock_free() -> bool
```

## Constants

### `IS_ALWAYS_LOCK_FREE`

```rust
const IS_ALWAYS_LOCK_FREE: bool = false;
```

## Macros

### `atomic128!`

### `atomic_rmw_by_atomic_update!`

### `debug_assert_cmpxchg16b!`

### `debug_assert_vmovdqa_atomic!`

### `ptr_modifier!`

### `load_store_detect!`

### `atomic_rmw_cas_3!`

Atomic RMW by CAS loop (3 arguments)
`unsafe fn(dst: *mut u128, val: u128, order: Ordering) -> u128;`

`$op` can use the following registers:
- rsi/r8 pair: val argument (read-only for `$op`)
- rax/rdx pair: previous value loaded (read-only for `$op`)
- rbx/rcx pair: new value that will be stored

### `atomic_rmw_cas_2!`

Atomic RMW by CAS loop (2 arguments)
`unsafe fn(dst: *mut u128, order: Ordering) -> u128;`

`$op` can use the following registers:
- rax/rdx pair: previous value loaded (read-only for `$op`)
- rbx/rcx pair: new value that will be stored

### `select_atomic_rmw!`

