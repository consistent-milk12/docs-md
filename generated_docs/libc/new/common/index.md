*[libc](../../index.md) / [new](../index.md) / [common](index.md)*

---

# Module `common`

Interfaces that are common across multiple platforms

We make these available everywhere but each platform must opt in to reexporting.

There shouldn't be any repeated definitions or complex configuration in this module. On
platforms that don't use common APIs it is fine to use `#[cfg(not(...))]`, but if a platform
needs a custom definition then it should be defined in the platform-specific module.

The goal is that platforms need to opt in to the definitions here, so that worst case we have
an unused warning on untested platforms (rather than exposing incorrect API).

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`linux_like`](#linux_like) | mod | API that primarily comes from Linux but is also used other platforms (e.g. Android). |
| [`posix`](#posix) | mod | POSIX APIs that are used by a number of platforms |

## Modules

- [`linux_like`](linux_like/index.md) - API that primarily comes from Linux but is also used other platforms (e.g. Android).
- [`posix`](posix/index.md) - POSIX APIs that are used by a number of platforms

