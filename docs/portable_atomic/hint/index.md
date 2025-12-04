*[portable_atomic](../index.md) / [hint](index.md)*

---

# Module `hint`

Re-export of the `core::hint` module.

The only difference from the `core::hint` module is that [`spin_loop`](hint/index.md)
is available in all rust versions that this crate supports.

```
use portable_atomic::hint;

hint::spin_loop();
```

## Functions

### `spin_loop`

```rust
fn spin_loop()
```

Emits a machine instruction to signal the processor that it is running in
a busy-wait spin-loop ("spin lock").

Upon receiving the spin-loop signal the processor can optimize its behavior by,
for example, saving power or switching hyper-threads.

This function is different from `thread::yield_now` which directly
yields to the system's scheduler, whereas `spin_loop` does not interact
with the operating system.

A common use case for `spin_loop` is implementing bounded optimistic
spinning in a CAS loop in synchronization primitives. To avoid problems
like priority inversion, it is strongly recommended that the spin loop is
terminated after a finite amount of iterations and an appropriate blocking
syscall is made.

**Note:** On platforms that do not support receiving spin-loop hints this
function does not do anything at all.


