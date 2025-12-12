*[rayon_core](../index.md) / [unwind](index.md)*

---

# Module `unwind`

Package up unwind recovery. Note that if you are in some sensitive
place, you can use the `AbortIfPanic` helper to protect against
accidental panics in the rayon code itself.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AbortIfPanic`](#abortifpanic) | struct |  |
| [`halt_unwinding`](#halt-unwinding) | fn | Executes `f` and captures any panic, translating that panic into a `Err` result. |
| [`resume_unwinding`](#resume-unwinding) | fn |  |

## Structs

### `AbortIfPanic`

```rust
struct AbortIfPanic;
```

*Defined in [`rayon-core-1.13.0/src/unwind.rs:24`](../../../.source_1765521767/rayon-core-1.13.0/src/unwind.rs#L24)*

#### Trait Implementations

##### `impl Drop for AbortIfPanic`

- <span id="abortifpanic-drop"></span>`fn drop(&mut self)`

##### `impl Pointable for AbortIfPanic`

- <span id="abortifpanic-pointable-const-align"></span>`const ALIGN: usize`

- <span id="abortifpanic-pointable-type-init"></span>`type Init = T`

- <span id="abortifpanic-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="abortifpanic-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="abortifpanic-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="abortifpanic-drop"></span>`unsafe fn drop(ptr: usize)`

## Functions

### `halt_unwinding`

```rust
fn halt_unwinding<F, R>(func: F) -> thread::Result<R>
where
    F: FnOnce() -> R
```

*Defined in [`rayon-core-1.13.0/src/unwind.rs:13-18`](../../../.source_1765521767/rayon-core-1.13.0/src/unwind.rs#L13-L18)*

Executes `f` and captures any panic, translating that panic into a
`Err` result. The assumption is that any panic will be propagated
later with `resume_unwinding`, and hence `f` can be treated as
exception safe.

### `resume_unwinding`

```rust
fn resume_unwinding(payload: Box<dyn Any + Send>) -> never
```

*Defined in [`rayon-core-1.13.0/src/unwind.rs:20-22`](../../../.source_1765521767/rayon-core-1.13.0/src/unwind.rs#L20-L22)*

