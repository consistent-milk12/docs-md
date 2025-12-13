*[portable_atomic](../../../index.md) / [imp](../../index.md) / [atomic128](../index.md) / [x86_64](index.md)*

---

# Module `x86_64`

## Contents

- [Modules](#modules)
  - [`fallback`](#fallback)
  - [`detect`](#detect)
- [Structs](#structs)
  - [`AtomicI128`](#atomici128)
  - [`AtomicU128`](#atomicu128)
- [Functions](#functions)
  - [`cmpxchg16b`](#cmpxchg16b)
  - [`atomic_load_vmovdqa`](#atomic-load-vmovdqa)
  - [`atomic_store_vmovdqa`](#atomic-store-vmovdqa)
  - [`atomic_load`](#atomic-load)
  - [`atomic_load_cmpxchg16b`](#atomic-load-cmpxchg16b)
  - [`atomic_store`](#atomic-store)
  - [`atomic_store_cmpxchg16b`](#atomic-store-cmpxchg16b)
  - [`atomic_compare_exchange`](#atomic-compare-exchange)
  - [`atomic_swap_cmpxchg16b`](#atomic-swap-cmpxchg16b)
  - [`atomic_add_cmpxchg16b`](#atomic-add-cmpxchg16b)
  - [`atomic_sub_cmpxchg16b`](#atomic-sub-cmpxchg16b)
  - [`atomic_and_cmpxchg16b`](#atomic-and-cmpxchg16b)
  - [`atomic_nand_cmpxchg16b`](#atomic-nand-cmpxchg16b)
  - [`atomic_or_cmpxchg16b`](#atomic-or-cmpxchg16b)
  - [`atomic_xor_cmpxchg16b`](#atomic-xor-cmpxchg16b)
  - [`atomic_not_cmpxchg16b`](#atomic-not-cmpxchg16b)
  - [`atomic_neg_cmpxchg16b`](#atomic-neg-cmpxchg16b)
  - [`atomic_max_cmpxchg16b`](#atomic-max-cmpxchg16b)
  - [`atomic_umax_cmpxchg16b`](#atomic-umax-cmpxchg16b)
  - [`atomic_min_cmpxchg16b`](#atomic-min-cmpxchg16b)
  - [`atomic_umin_cmpxchg16b`](#atomic-umin-cmpxchg16b)
  - [`atomic_swap`](#atomic-swap)
  - [`atomic_add`](#atomic-add)
  - [`atomic_sub`](#atomic-sub)
  - [`atomic_and`](#atomic-and)
  - [`atomic_nand`](#atomic-nand)
  - [`atomic_or`](#atomic-or)
  - [`atomic_xor`](#atomic-xor)
  - [`atomic_max`](#atomic-max)
  - [`atomic_umax`](#atomic-umax)
  - [`atomic_min`](#atomic-min)
  - [`atomic_umin`](#atomic-umin)
  - [`atomic_not`](#atomic-not)
  - [`atomic_neg`](#atomic-neg)
  - [`is_lock_free`](#is-lock-free)
- [Constants](#constants)
  - [`IS_ALWAYS_LOCK_FREE`](#is-always-lock-free)
- [Macros](#macros)
  - [`atomic128!`](#atomic128)
  - [`atomic_rmw_by_atomic_update!`](#atomic-rmw-by-atomic-update)
  - [`debug_assert_cmpxchg16b!`](#debug-assert-cmpxchg16b)
  - [`debug_assert_vmovdqa_atomic!`](#debug-assert-vmovdqa-atomic)
  - [`ptr_modifier!`](#ptr-modifier)
  - [`load_store_detect!`](#load-store-detect)
  - [`atomic_rmw_cas_3!`](#atomic-rmw-cas-3)
  - [`atomic_rmw_cas_2!`](#atomic-rmw-cas-2)
  - [`select_atomic_rmw!`](#select-atomic-rmw)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`fallback`](#fallback) | mod |  |
| [`detect`](#detect) | mod |  |
| [`AtomicI128`](#atomici128) | struct |  |
| [`AtomicU128`](#atomicu128) | struct |  |
| [`cmpxchg16b`](#cmpxchg16b) | fn |  |
| [`atomic_load_vmovdqa`](#atomic-load-vmovdqa) | fn |  |
| [`atomic_store_vmovdqa`](#atomic-store-vmovdqa) | fn |  |
| [`atomic_load`](#atomic-load) | fn |  |
| [`atomic_load_cmpxchg16b`](#atomic-load-cmpxchg16b) | fn |  |
| [`atomic_store`](#atomic-store) | fn |  |
| [`atomic_store_cmpxchg16b`](#atomic-store-cmpxchg16b) | fn |  |
| [`atomic_compare_exchange`](#atomic-compare-exchange) | fn |  |
| [`atomic_swap_cmpxchg16b`](#atomic-swap-cmpxchg16b) | fn |  |
| [`atomic_add_cmpxchg16b`](#atomic-add-cmpxchg16b) | fn |  |
| [`atomic_sub_cmpxchg16b`](#atomic-sub-cmpxchg16b) | fn |  |
| [`atomic_and_cmpxchg16b`](#atomic-and-cmpxchg16b) | fn |  |
| [`atomic_nand_cmpxchg16b`](#atomic-nand-cmpxchg16b) | fn |  |
| [`atomic_or_cmpxchg16b`](#atomic-or-cmpxchg16b) | fn |  |
| [`atomic_xor_cmpxchg16b`](#atomic-xor-cmpxchg16b) | fn |  |
| [`atomic_not_cmpxchg16b`](#atomic-not-cmpxchg16b) | fn |  |
| [`atomic_neg_cmpxchg16b`](#atomic-neg-cmpxchg16b) | fn |  |
| [`atomic_max_cmpxchg16b`](#atomic-max-cmpxchg16b) | fn |  |
| [`atomic_umax_cmpxchg16b`](#atomic-umax-cmpxchg16b) | fn |  |
| [`atomic_min_cmpxchg16b`](#atomic-min-cmpxchg16b) | fn |  |
| [`atomic_umin_cmpxchg16b`](#atomic-umin-cmpxchg16b) | fn |  |
| [`atomic_swap`](#atomic-swap) | fn |  |
| [`atomic_add`](#atomic-add) | fn |  |
| [`atomic_sub`](#atomic-sub) | fn |  |
| [`atomic_and`](#atomic-and) | fn |  |
| [`atomic_nand`](#atomic-nand) | fn |  |
| [`atomic_or`](#atomic-or) | fn |  |
| [`atomic_xor`](#atomic-xor) | fn |  |
| [`atomic_max`](#atomic-max) | fn |  |
| [`atomic_umax`](#atomic-umax) | fn |  |
| [`atomic_min`](#atomic-min) | fn |  |
| [`atomic_umin`](#atomic-umin) | fn |  |
| [`atomic_not`](#atomic-not) | fn |  |
| [`atomic_neg`](#atomic-neg) | fn |  |
| [`is_lock_free`](#is-lock-free) | fn |  |
| [`IS_ALWAYS_LOCK_FREE`](#is-always-lock-free) | const |  |
| [`atomic128!`](#atomic128) | macro |  |
| [`atomic_rmw_by_atomic_update!`](#atomic-rmw-by-atomic-update) | macro |  |
| [`debug_assert_cmpxchg16b!`](#debug-assert-cmpxchg16b) | macro |  |
| [`debug_assert_vmovdqa_atomic!`](#debug-assert-vmovdqa-atomic) | macro |  |
| [`ptr_modifier!`](#ptr-modifier) | macro |  |
| [`load_store_detect!`](#load-store-detect) | macro |  |
| [`atomic_rmw_cas_3!`](#atomic-rmw-cas-3) | macro | Atomic RMW by CAS loop (3 arguments) `unsafe fn(dst: *mut u128, val: u128, order: Ordering) -> u128;` |
| [`atomic_rmw_cas_2!`](#atomic-rmw-cas-2) | macro | Atomic RMW by CAS loop (2 arguments) `unsafe fn(dst: *mut u128, order: Ordering) -> u128;` |
| [`select_atomic_rmw!`](#select-atomic-rmw) | macro |  |

## Modules

- [`fallback`](fallback/index.md)
- [`detect`](detect/index.md)

## Structs

### `AtomicI128`

```rust
struct AtomicI128 {
    v: core::cell::UnsafeCell<i128>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:858`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L858)*

#### Implementations

- <span id="atomici128-add"></span>`fn add(&self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md#ordering)

- <span id="atomici128-sub"></span>`fn sub(&self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md#ordering)

- <span id="atomici128-and"></span>`fn and(&self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md#ordering)

- <span id="atomici128-or"></span>`fn or(&self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md#ordering)

- <span id="atomici128-xor"></span>`fn xor(&self, val: i128, order: Ordering)` — [`Ordering`](../../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicI128`

- <span id="atomici128-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicI128`

- <span id="atomici128-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicI128`

- <span id="atomici128-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AtomicI128`

- <span id="atomici128-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicI128`

- <span id="atomici128-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Sync for AtomicI128`

##### `impl<U> TryFrom for AtomicI128`

- <span id="atomici128-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomici128-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicI128`

- <span id="atomici128-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomici128-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AtomicU128`

```rust
struct AtomicU128 {
    v: core::cell::UnsafeCell<u128>,
}
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:859`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L859)*

#### Implementations

- <span id="atomicu128-add"></span>`fn add(&self, val: u128, order: Ordering)` — [`Ordering`](../../../index.md#ordering)

- <span id="atomicu128-sub"></span>`fn sub(&self, val: u128, order: Ordering)` — [`Ordering`](../../../index.md#ordering)

- <span id="atomicu128-and"></span>`fn and(&self, val: u128, order: Ordering)` — [`Ordering`](../../../index.md#ordering)

- <span id="atomicu128-or"></span>`fn or(&self, val: u128, order: Ordering)` — [`Ordering`](../../../index.md#ordering)

- <span id="atomicu128-xor"></span>`fn xor(&self, val: u128, order: Ordering)` — [`Ordering`](../../../index.md#ordering)

#### Trait Implementations

##### `impl Any for AtomicU128`

- <span id="atomicu128-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AtomicU128`

- <span id="atomicu128-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AtomicU128`

- <span id="atomicu128-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for AtomicU128`

- <span id="atomicu128-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AtomicU128`

- <span id="atomicu128-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Sync for AtomicU128`

##### `impl<U> TryFrom for AtomicU128`

- <span id="atomicu128-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="atomicu128-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AtomicU128`

- <span id="atomicu128-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="atomicu128-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `cmpxchg16b`

```rust
unsafe fn cmpxchg16b(dst: *mut u128, old: u128, new: u128) -> (u128, bool)
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:95-142`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L95-L142)*

### `atomic_load_vmovdqa`

```rust
unsafe fn atomic_load_vmovdqa(src: *mut u128) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:158-175`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L158-L175)*

### `atomic_store_vmovdqa`

```rust
unsafe fn atomic_store_vmovdqa(dst: *mut u128, val: u128, order: core::sync::atomic::Ordering)
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:180-218`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L180-L218)*

### `atomic_load`

```rust
unsafe fn atomic_load(src: *mut u128, _order: core::sync::atomic::Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:259-287`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L259-L287)*

### `atomic_load_cmpxchg16b`

```rust
unsafe fn atomic_load_cmpxchg16b(src: *mut u128) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:294-333`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L294-L333)*

### `atomic_store`

```rust
unsafe fn atomic_store(dst: *mut u128, val: u128, order: core::sync::atomic::Ordering)
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:336-389`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L336-L389)*

### `atomic_store_cmpxchg16b`

```rust
unsafe fn atomic_store_cmpxchg16b(dst: *mut u128, val: u128)
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:396-402`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L396-L402)*

### `atomic_compare_exchange`

```rust
unsafe fn atomic_compare_exchange(dst: *mut u128, old: u128, new: u128, _success: core::sync::atomic::Ordering, _failure: core::sync::atomic::Ordering) -> Result<u128, u128>
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:405-431`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L405-L431)*

### `atomic_swap_cmpxchg16b`

```rust
unsafe fn atomic_swap_cmpxchg16b(dst: *mut u128, val: u128, _order: core::sync::atomic::Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:442-495`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L442-L495)*

### `atomic_add_cmpxchg16b`

```rust
unsafe fn atomic_add_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:634-640`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L634-L640)*

### `atomic_sub_cmpxchg16b`

```rust
unsafe fn atomic_sub_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:641-647`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L641-L647)*

### `atomic_and_cmpxchg16b`

```rust
unsafe fn atomic_and_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:648-654`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L648-L654)*

### `atomic_nand_cmpxchg16b`

```rust
unsafe fn atomic_nand_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:655-663`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L655-L663)*

### `atomic_or_cmpxchg16b`

```rust
unsafe fn atomic_or_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:664-670`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L664-L670)*

### `atomic_xor_cmpxchg16b`

```rust
unsafe fn atomic_xor_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:671-677`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L671-L677)*

### `atomic_not_cmpxchg16b`

```rust
unsafe fn atomic_not_cmpxchg16b(dst: *mut u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:679-685`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L679-L685)*

### `atomic_neg_cmpxchg16b`

```rust
unsafe fn atomic_neg_cmpxchg16b(dst: *mut u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:686-692`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L686-L692)*

### `atomic_max_cmpxchg16b`

```rust
unsafe fn atomic_max_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:694-703`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L694-L703)*

### `atomic_umax_cmpxchg16b`

```rust
unsafe fn atomic_umax_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:704-713`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L704-L713)*

### `atomic_min_cmpxchg16b`

```rust
unsafe fn atomic_min_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:714-723`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L714-L723)*

### `atomic_umin_cmpxchg16b`

```rust
unsafe fn atomic_umin_cmpxchg16b(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:724-733`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L724-L733)*

### `atomic_swap`

```rust
unsafe fn atomic_swap(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:777-781`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L777-L781)*

### `atomic_add`

```rust
unsafe fn atomic_add(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:782-786`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L782-L786)*

### `atomic_sub`

```rust
unsafe fn atomic_sub(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:787-791`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L787-L791)*

### `atomic_and`

```rust
unsafe fn atomic_and(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:792-796`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L792-L796)*

### `atomic_nand`

```rust
unsafe fn atomic_nand(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:797-801`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L797-L801)*

### `atomic_or`

```rust
unsafe fn atomic_or(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:802-806`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L802-L806)*

### `atomic_xor`

```rust
unsafe fn atomic_xor(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:807-811`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L807-L811)*

### `atomic_max`

```rust
unsafe fn atomic_max(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:812-816`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L812-L816)*

### `atomic_umax`

```rust
unsafe fn atomic_umax(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:817-821`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L817-L821)*

### `atomic_min`

```rust
unsafe fn atomic_min(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:822-826`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L822-L826)*

### `atomic_umin`

```rust
unsafe fn atomic_umin(dst: *mut u128, val: u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:827-831`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L827-L831)*

### `atomic_not`

```rust
unsafe fn atomic_not(dst: *mut u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:832-836`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L832-L836)*

### `atomic_neg`

```rust
unsafe fn atomic_neg(dst: *mut u128, _order: Ordering) -> u128
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:837-841`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L837-L841)*

### `is_lock_free`

```rust
fn is_lock_free() -> bool
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:844-854`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L844-L854)*

## Constants

### `IS_ALWAYS_LOCK_FREE`
```rust
const IS_ALWAYS_LOCK_FREE: bool = false;
```

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:855-856`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L855-L856)*

## Macros

### `atomic128!`

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/macros.rs:3-255`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/macros.rs#L3-L255)*

### `atomic_rmw_by_atomic_update!`

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/macros.rs:259-349`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/macros.rs#L259-L349)*

### `debug_assert_cmpxchg16b!`

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:46-56`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L46-L56)*

### `debug_assert_vmovdqa_atomic!`

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:59-64`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L59-L64)*

### `ptr_modifier!`

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:77-81`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L77-L81)*

### `load_store_detect!`

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:224-256`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L224-L256)*

### `atomic_rmw_cas_3!`

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:506-566`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L506-L566)*

Atomic RMW by CAS loop (3 arguments)
`unsafe fn(dst: *mut u128, val: u128, order: Ordering) -> u128;`

`$op` can use the following registers:
- rsi/r8 pair: val argument (read-only for `$op`)
- rax/rdx pair: previous value loaded (read-only for `$op`)
- rbx/rcx pair: new value that will be stored

### `atomic_rmw_cas_2!`

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:575-632`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L575-L632)*

Atomic RMW by CAS loop (2 arguments)
`unsafe fn(dst: *mut u128, order: Ordering) -> u128;`

`$op` can use the following registers:
- rax/rdx pair: previous value loaded (read-only for `$op`)
- rbx/rcx pair: new value that will be stored

### `select_atomic_rmw!`

*Defined in [`portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs:735-775`](../../../../../.source_1765521767/portable-atomic-1.11.1/src/imp/atomic128/x86_64.rs#L735-L775)*

