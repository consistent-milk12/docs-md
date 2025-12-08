*[libc](../../index.md) / [new](../index.md) / [glibc](index.md)*

---

# Module `glibc`

GNU libc.

* Headers: <https://sourceware.org/git/?p=glibc.git> (official)
* Headers: <https://github.com/bminor/glibc> (mirror)

This module structure is modeled after glibc's source tree. Its build system selects headers
from different locations based on the platform, which we mimic here with reexports.

## Modules

- [`posix`](posix/index.md) - Source directory: `posix/`
- [`sysdeps`](sysdeps/index.md) - Source directory: `sysdeps/`

