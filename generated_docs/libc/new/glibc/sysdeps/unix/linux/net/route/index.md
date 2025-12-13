*[libc](../../../../../../../index.md) / [new](../../../../../../index.md) / [glibc](../../../../../index.md) / [sysdeps](../../../../index.md) / [unix](../../../index.md) / [linux](../../index.md) / [net](../index.md) / [route](index.md)*

---

# Module `route`

Header: `net/route.h`

Source header: `sysdeps/unix/sysv/linux/net/route.h`
<https://github.com/bminor/glibc/blob/master/sysdeps/unix/sysv/linux/net/route.h>

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`rtentry`](#rtentry) | struct |  |

## Structs

### `rtentry`

```rust
struct rtentry {
    pub rt_pad1: crate::c_ulong,
    pub rt_dst: crate::sockaddr,
    pub rt_gateway: crate::sockaddr,
    pub rt_genmask: crate::sockaddr,
    pub rt_flags: crate::c_ushort,
    pub rt_pad2: crate::c_short,
    pub rt_pad3: crate::c_ulong,
    pub rt_tos: crate::c_uchar,
    pub rt_class: crate::c_uchar,
    pub rt_pad4: [crate::c_short; 3],
    pub rt_metric: crate::c_short,
    pub rt_dev: *mut crate::c_char,
    pub rt_mtu: crate::c_ulong,
    pub rt_window: crate::c_ulong,
    pub rt_irtt: crate::c_ushort,
}
```

*Defined in [`libc-0.2.178/src/new/glibc/sysdeps/unix/linux/net/route.rs:8-30`](../../../../../../../../../.source_1765633015/libc-0.2.178/src/new/glibc/sysdeps/unix/linux/net/route.rs#L8-L30)*

#### Trait Implementations

##### `impl Any for rtentry`

- <span id="rtentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for rtentry`

- <span id="rtentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for rtentry`

- <span id="rtentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for rtentry`

- <span id="rtentry-clone"></span>`fn clone(&self) -> rtentry` — [`rtentry`](../../../../../../index.md#rtentry)

##### `impl CloneToUninit for rtentry`

- <span id="rtentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for rtentry`

##### `impl Debug for rtentry`

- <span id="rtentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for rtentry`

- <span id="rtentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for rtentry`

- <span id="rtentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for rtentry`

- <span id="rtentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rtentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for rtentry`

- <span id="rtentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rtentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

