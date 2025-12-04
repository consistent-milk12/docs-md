# Crate `object`

# `object`

The `object` crate provides a unified interface to working with object files
across platforms. It supports reading relocatable object files and executable files,
and writing relocatable object files and some executable files.

## Raw struct definitions

Raw structs are defined for: [ELF](elf), [Mach-O](macho), [PE/COFF](pe),
[XCOFF](xcoff), [archive](#archive).
Types and traits for zerocopy support are defined in the [`pod`](object/pod/index.md) and [`endian`](object/endian/index.md) modules.

## Unified read API

The [`read`](object/read/index.md) module provides a unified read API using the [`read::Object`](#object) trait.
There is an implementation of this trait for [`read::File`](#file), which allows reading any
file format, as well as implementations for each file format.

## Low level read API

The [`read#modules`](#readmodules) submodules define helpers that operate on the raw structs.
These can be used instead of the unified API, or in conjunction with it to access
details that are not available via the unified API.

## Unified write API

The [`mod@write`](#modwrite) module provides a unified write API for relocatable object files
using [`write::Object`](#object). This does not support writing executable files.

## Low level write API

The [`mod@write#modules`](#modwritemodules) submodules define helpers for writing the raw structs.

## Build API

The [`mod@build`](#modbuild) submodules define helpers for building object files, either from
scratch or by modifying existing files.

## Shared definitions

The crate provides a number of definitions that are used by both the read and write
APIs. These are defined at the top level module, but none of these are the main entry
points of the crate.

## Modules

- [`endian`](endian/index.md) - Types for compile-time and run-time endianness.
- [`pod`](pod/index.md) - Tools for converting file format structures to and from bytes.
- [`read`](read/index.md) - Interface for reading object files.
- [`archive`](archive/index.md) - Archive definitions.
- [`elf`](elf/index.md) - ELF definitions.
- [`macho`](macho/index.md) - Mach-O definitions.
- [`pe`](pe/index.md) - PE/COFF definitions.
- [`xcoff`](xcoff/index.md) - XCOFF definitions

