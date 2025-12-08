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

#### Trait Implementations

##### `impl Clone for rtentry`

- <span id="rtentry-clone"></span>`fn clone(&self) -> rtentry` — [`rtentry`](../../../../../../index.md)

##### `impl Copy for rtentry`

##### `impl Debug for rtentry`

- <span id="rtentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

