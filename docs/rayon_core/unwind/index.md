*[rayon_core](../index.md) / [unwind](index.md)*

---

# Module `unwind`

Package up unwind recovery. Note that if you are in some sensitive
place, you can use the `AbortIfPanic` helper to protect against
accidental panics in the rayon code itself.

## Structs

### `AbortIfPanic`

```rust
struct AbortIfPanic;
```

#### Trait Implementations

##### `impl Drop for AbortIfPanic`

- `fn drop(self: &mut Self)`

##### `impl<T> Pointable for AbortIfPanic`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

## Functions

### `halt_unwinding`

```rust
fn halt_unwinding<F, R>(func: F) -> thread::Result<R>
where
    F: FnOnce() -> R
```

Executes `f` and captures any panic, translating that panic into a
`Err` result. The assumption is that any panic will be propagated
later with `resume_unwinding`, and hence `f` can be treated as
exception safe.

### `resume_unwinding`

```rust
fn resume_unwinding(payload: Box<dyn Any + Send>) -> never
```

