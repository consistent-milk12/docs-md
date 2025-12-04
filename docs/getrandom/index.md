# Crate `getrandom`

Interface to the operating system's random number generator.

# Supported targets

| Target            | Target Triple      | Implementation
| ----------------- | ------------------ | --------------
| Linux, Android    | `*‑linux‑*`        | [`getrandom`][1] system call if available, otherwise [`/dev/urandom`][2] after successfully polling `/dev/random`
| Windows           | `*‑windows‑*`      | [`BCryptGenRandom`](#bcryptgenrandom)
| macOS             | `*‑apple‑darwin`   | [`getentropy`][3]
| iOS, tvOS, watchOS | `*‑apple‑ios`, `*-apple-tvos`, `*-apple-watchos` | [`CCRandomGenerateBytes`](#ccrandomgeneratebytes)
| FreeBSD           | `*‑freebsd`        | [`getrandom`][5]
| OpenBSD           | `*‑openbsd`        | [`getentropy`][7]
| NetBSD            | `*‑netbsd`         | [`getrandom`][16] if available, otherwise [`kern.arandom`][8]
| Dragonfly BSD     | `*‑dragonfly`      | [`getrandom`][9]
| Solaris           | `*‑solaris`        | [`getrandom`][11] (with `GRND_RANDOM`)
| illumos           | `*‑illumos`        | [`getrandom`][12]
| Fuchsia OS        | `*‑fuchsia`        | [`cprng_draw`](#cprng-draw)
| Redox             | `*‑redox`          | `/dev/urandom`
| Haiku             | `*‑haiku`          | `/dev/urandom` (identical to `/dev/random`)
| Hermit            | `*-hermit`         | [`sys_read_entropy`](#sys-read-entropy)
| Hurd              | `*-hurd-*`         | [`getrandom`][17]
| SGX               | `x86_64‑*‑sgx`     | [`RDRAND`](#rdrand)
| VxWorks           | `*‑wrs‑vxworks‑*`  | `randABytes` after checking entropy pool initialization with `randSecure`
| ESP-IDF           | `*‑espidf`         | [`esp_fill_random`](#esp-fill-random)
| Emscripten        | `*‑emscripten`     | [`getentropy`][13]
| WASI              | `wasm32‑wasi`      | [`random_get`](#random-get)
| Web Browser and Node.js | `wasm*‑*‑unknown` | [`Crypto.getRandomValues`](#cryptogetrandomvalues) if available, then [`crypto.randomFillSync`](#cryptorandomfillsync) if on Node.js, see [WebAssembly support]
| SOLID             | `*-kmc-solid_*`    | `SOLID_RNG_SampleRandomBytes`
| Nintendo 3DS      | `*-nintendo-3ds`   | [`getrandom`][18]
| PS Vita           | `*-vita-*`         | [`getentropy`][13]
| QNX Neutrino      | `*‑nto-qnx*`       | [`/dev/urandom`][14] (identical to `/dev/random`)
| AIX               | `*-ibm-aix`        | [`/dev/urandom`][15]
| Cygwin            | `*-cygwin`         | [`getrandom`][19] (based on [`RtlGenRandom`](#rtlgenrandom))

Pull Requests that add support for new targets to `getrandom` are always welcome.

## Unsupported targets

By default, `getrandom` will not compile on unsupported targets, but certain
features allow a user to select a "fallback" implementation if no supported
implementation exists.

All of the below mechanisms only affect unsupported
targets. Supported targets will _always_ use their supported implementations.
This prevents a crate from overriding a secure source of randomness
(either accidentally or intentionally).

## `/dev/urandom` fallback on Linux and Android

On Linux targets the fallback is present only if either `target_env` is `musl`,
or `target_arch` is one of the following: `aarch64`, `arm`, `powerpc`, `powerpc64`,
`s390x`, `x86`, `x86_64`. Other supported targets [require][platform-support]
kernel versions which support `getrandom` system call, so fallback is not needed.

On Android targets the fallback is present only for the following `target_arch`es:
`aarch64`, `arm`, `x86`, `x86_64`. Other `target_arch`es (e.g. RISC-V) require
sufficiently high API levels.

The fallback can be disabled by enabling the `linux_disable_fallback` crate feature.
Note that doing so will bump minimum supported Linux kernel version to 3.17 and
Android API level to 23 (Marshmallow).

### RDRAND on x86

*If the `rdrand` Cargo feature is enabled*, `getrandom` will fallback to using
the [`RDRAND`](#rdrand) instruction to get randomness on `no_std` `x86`/`x86_64`
targets. This feature has no effect on other CPU architectures.

### WebAssembly support

This crate fully supports the
[`wasm32-wasi`](https://github.com/CraneStation/wasi) and
[`wasm32-unknown-emscripten`](https://www.hellorust.com/setup/emscripten/)
targets. However, the `wasm32-unknown-unknown` target (i.e. the target used
by `wasm-pack`) is not automatically
supported since, from the target name alone, we cannot deduce which
JavaScript interface is in use (or if JavaScript is available at all).

Instead, *if the `js` Cargo feature is enabled*, this crate will assume
that you are building for an environment containing JavaScript, and will
call the appropriate methods. Both web browser (main window and Web Workers)
and Node.js environments are supported, invoking the methods
[described above](#supported-targets) using the [`wasm-bindgen`](#wasm-bindgen) toolchain.

To enable the `js` Cargo feature, add the following to the `dependencies`
section in your `Cargo.toml` file:
```toml
[dependencies](#dependencies)

getrandom = { version = "0.2", features = ["js"] }
```

This can be done even if `getrandom` is not a direct dependency. Cargo
allows crates to enable features for indirect dependencies.

This feature should only be enabled for binary, test, or benchmark crates.
Library crates should generally not enable this feature, leaving such a
decision to *users* of their library. Also, libraries should not introduce
their own `js` features *just* to enable `getrandom`'s `js` feature.

This feature has no effect on targets other than `wasm32-unknown-unknown`.

#### Node.js ES module support

Node.js supports both [CommonJS modules] and [ES modules]. Due to
limitations in wasm-bindgen's [`module`](../docs_md/docs_md/generator/module/index.md) support, we cannot directly
support ES Modules running on Node.js. However, on Node v15 and later, the
module author can add a simple shim to support the Web Cryptography API:
```js
import { webcrypto } from 'node:crypto'
globalThis.crypto = webcrypto
```
This crate will then use the provided `webcrypto` implementation.

### Platform Support
This crate generally supports the same operating system and platform versions
that the Rust standard library does. Additional targets may be supported using
pluggable custom implementations.

This means that as Rust drops support for old versions of operating systems
(such as old Linux kernel versions, Android API levels, etc) in stable releases,
`getrandom` may create new patch releases (`0.N.x`) that remove support for
outdated platform versions.

### Custom implementations

The [`register_custom_getrandom!`](#register-custom-getrandom) macro allows a user to mark their own
function as the backing implementation for [`getrandom`](index.md). See the macro's
documentation for more information about writing and registering your own
custom implementations.

Note that registering a custom implementation only has an effect on targets
that would otherwise not compile. Any supported targets (including those
using `rdrand` and `js` Cargo features) continue using their normal
implementations even if a function is registered.

## Early boot

Sometimes, early in the boot process, the OS has not collected enough
entropy to securely seed its RNG. This is especially common on virtual
machines, where standard "random" events are hard to come by.

Some operating system interfaces always block until the RNG is securely
seeded. This can take anywhere from a few seconds to more than a minute.
A few (Linux, NetBSD and Solaris) offer a choice between blocking and
getting an error; in these cases, we always choose to block.

On Linux (when the `getrandom` system call is not available), reading from
`/dev/urandom` never blocks, even when the OS hasn't collected enough
entropy yet. To avoid returning low-entropy bytes, we first poll
`/dev/random` and only switch to `/dev/urandom` once this has succeeded.

On OpenBSD, this kind of entropy accounting isn't available, and on
NetBSD, blocking on it is discouraged. On these platforms, nonblocking
interfaces are used, even when reliable entropy may not be available.
On the platforms where it is used, the reliability of entropy accounting
itself isn't free from controversy. This library provides randomness
sourced according to the platform's best practices, but each platform has
its own limits on the grade of randomness it can promise in environments
with few sources of entropy.

## Error handling

We always choose failure over returning known insecure "random" bytes. In
general, on supported platforms, failure is highly unlikely, though not
impossible. If an error does occur, then it is likely that it will occur
on every call to `getrandom`, hence after the first successful call one
can be reasonably confident that no errors will occur.

[1]: https://manned.org/getrandom.2
[2]: https://manned.org/urandom.4
[3]: https://www.unix.com/man-page/mojave/2/getentropy/
[4]: https://www.unix.com/man-page/mojave/4/urandom/
[5]: https://www.freebsd.org/cgi/man.cgi?query=getrandom&manpath=FreeBSD+12.0-stable
[7]: https://man.openbsd.org/getentropy.2
[8]: https://man.netbsd.org/sysctl.7
[9]: https://leaf.dragonflybsd.org/cgi/web-man?command=getrandom
[11]: https://docs.oracle.com/cd/E88353_01/html/E37841/getrandom-2.html
[12]: https://illumos.org/man/2/getrandom
[13]: https://github.com/emscripten-core/emscripten/pull/12240
[14]: https://www.qnx.com/developers/docs/7.1/index.html#com.qnx.doc.neutrino.utilities/topic/r/random.html
[15]: https://www.ibm.com/docs/en/aix/7.3?topic=files-random-urandom-devices
[16]: https://man.netbsd.org/getrandom.2
[17]: https://www.gnu.org/software/libc/manual/html_mono/libc.html#index-getrandom
[18]: https://github.com/rust3ds/shim-3ds/commit/b01d2568836dea2a65d05d662f8e5f805c64389d
[19]: https://github.com/cygwin/cygwin/blob/main/winsup/cygwin/libc/getentropy.cc









[WebAssembly support]: #webassembly-support


[CommonJS modules]: https://nodejs.org/api/modules.html
[ES modules]: https://nodejs.org/api/esm.html

[platform-support]: https://doc.rust-lang.org/stable/rustc/platform-support.html

## Structs

### `Error`

```rust
struct Error();
```

A small and `no_std` compatible error type

The [`Error::raw_os_error()`](#raw-os-error) will indicate if the error is from the OS, and
if so, which error code the OS gave the application. If such an error is
encountered, please consult with your system documentation.

Internally this type is a NonZeroU32, with certain values reserved for
certain purposes, see [`Error::INTERNAL_START`](#internal-start) and [`Error::CUSTOM_START`](#custom-start).

*If this crate's `"std"` Cargo feature is enabled*, then:
- [`getrandom::Error`][Error] implements
  [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
- [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) implements
  [`From<getrandom::Error>`](https://doc.rust-lang.org/std/convert/trait.From.html).

#### Implementations

- `const UNSUPPORTED: Error`

- `const ERRNO_NOT_POSITIVE: Error`

- `const UNEXPECTED: Error`

- `const IOS_SEC_RANDOM: Error`

- `const WINDOWS_RTL_GEN_RANDOM: Error`

- `const FAILED_RDRAND: Error`

- `const NO_RDRAND: Error`

- `const WEB_CRYPTO: Error`

- `const WEB_GET_RANDOM_VALUES: Error`

- `const VXWORKS_RAND_SECURE: Error`

- `const NODE_CRYPTO: Error`

- `const NODE_RANDOM_FILL_SYNC: Error`

- `const NODE_ES_MODULE: Error`

- `const INTERNAL_START: u32`

- `const CUSTOM_START: u32`

- `fn raw_os_error(self: Self) -> Option<i32>`
  Extract the raw OS error code (if this error came from the OS)

- `const fn code(self: Self) -> NonZeroU32`
  Extract the bare error code.

#### Trait Implementations

##### `impl From`

- `fn from(code: NonZeroU32) -> Self`

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Error`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Error) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `getrandom`

```rust
fn getrandom(dest: &mut [u8]) -> Result<(), Error>
```

Fill `dest` with random bytes from the system's preferred random number
source.

This function returns an error on any failure, including partial reads. We
make no guarantees regarding the contents of `dest` on error. If `dest` is
empty, `getrandom` immediately returns success, making no calls to the
underlying operating system.

Blocking is possible, at least during early boot; see module documentation.

In general, `getrandom` will be fast enough for interactive usage, though
significantly slower than a user-space CSPRNG; for the latter consider
[`rand::thread_rng`](https://docs.rs/rand/*/rand/fn.thread_rng.html).

### `getrandom_uninit`

```rust
fn getrandom_uninit(dest: &mut [core::mem::MaybeUninit<u8>]) -> Result<&mut [u8], Error>
```

Version of the `getrandom` function which fills `dest` with random bytes
returns a mutable reference to those bytes.

On successful completion this function is guaranteed to return a slice
which points to the same memory as `dest` and has the same length.
In other words, it's safe to assume that `dest` is initialized after
this function has returned `Ok`.

No part of `dest` will ever be de-initialized at any point, regardless
of what is returned.

# Examples

```ignore
# // We ignore this test since `uninit_array` is unstable.
#![feature(maybe_uninit_uninit_array)]
# fn main() -> Result<(), getrandom::Error> {
let mut buf = core::mem::MaybeUninit::uninit_array::<1024>();
let buf: &mut [u8](#u8)
 = getrandom::getrandom_uninit(&mut buf)?;
# Ok(()) }
```

