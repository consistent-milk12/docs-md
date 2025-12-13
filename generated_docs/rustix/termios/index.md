*[rustix](../index.md) / [termios](index.md)*

---

# Module `termios`

Terminal I/O stream operations.

This API automatically supports setting arbitrary I/O speeds, on any
platform that supports them, including Linux and the BSDs.

The [`speed`](#speed) module contains various predefined speed constants which are
more likely to be portable, however any `u32` value can be passed to
`Termios::set_speed`, `Termios::set_input_speed`, and
`Termios::set_output_speed`, and they will simply fail if the speed is
not supported by the platform or the device.

## Contents

- [Modules](#modules)
  - [`ioctl`](#ioctl)
  - [`tc`](#tc)
  - [`tty`](#tty)
  - [`types`](#types)
  - [`speed`](#speed)
- [Structs](#structs)
  - [`Termios`](#termios)
  - [`InputModes`](#inputmodes)
  - [`OutputModes`](#outputmodes)
  - [`ControlModes`](#controlmodes)
  - [`LocalModes`](#localmodes)
  - [`SpecialCodes`](#specialcodes)
  - [`SpecialCode`](#specialcode)
  - [`SpecialCodeIndex`](#specialcodeindex)
  - [`Winsize`](#winsize)
- [Enums](#enums)
  - [`OptionalActions`](#optionalactions)
  - [`QueueSelector`](#queueselector)
  - [`Action`](#action)
- [Functions](#functions)
  - [`ioctl_tiocexcl`](#ioctl-tiocexcl)
  - [`ioctl_tiocnxcl`](#ioctl-tiocnxcl)
  - [`tcgetattr`](#tcgetattr)
  - [`tcgetwinsize`](#tcgetwinsize)
  - [`tcgetpgrp`](#tcgetpgrp)
  - [`tcsetpgrp`](#tcsetpgrp)
  - [`tcsetattr`](#tcsetattr)
  - [`tcsendbreak`](#tcsendbreak)
  - [`tcdrain`](#tcdrain)
  - [`tcflush`](#tcflush)
  - [`tcflow`](#tcflow)
  - [`tcgetsid`](#tcgetsid)
  - [`tcsetwinsize`](#tcsetwinsize)
  - [`isatty`](#isatty)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ioctl`](#ioctl) | mod | Terminal-related `ioctl` functions. |
| [`tc`](#tc) | mod |  |
| [`tty`](#tty) | mod | Functions which operate on file descriptors which might be terminals. |
| [`types`](#types) | mod |  |
| [`speed`](#speed) | mod | Speeds for use with [`Termios::set_speed`], [`Termios::set_input_speed`], and [`Termios::set_output_speed`]. |
| [`Termios`](#termios) | struct | `struct termios` for use with [`tcgetattr`] and [`tcsetattr`]. |
| [`InputModes`](#inputmodes) | struct | Flags controlling terminal input. |
| [`OutputModes`](#outputmodes) | struct | Flags controlling terminal output. |
| [`ControlModes`](#controlmodes) | struct | Flags controlling special terminal modes. |
| [`LocalModes`](#localmodes) | struct | Flags controlling “local” terminal modes. |
| [`SpecialCodes`](#specialcodes) | struct | An array indexed by [`SpecialCodeIndex`] indicating the current values of various special control codes. |
| [`SpecialCode`](#specialcode) | struct | A newtype for pretty printing. |
| [`SpecialCodeIndex`](#specialcodeindex) | struct | Indices for use with [`Termios::special_codes`]. |
| [`Winsize`](#winsize) | struct | `struct winsize` for use with [`tcgetwinsize`]. |
| [`OptionalActions`](#optionalactions) | enum | `TCSA*` values for use with [`tcsetattr`]. |
| [`QueueSelector`](#queueselector) | enum | `TC*` values for use with [`tcflush`]. |
| [`Action`](#action) | enum | `TC*` values for use with [`tcflow`]. |
| [`ioctl_tiocexcl`](#ioctl-tiocexcl) | fn | `ioctl(fd, TIOCEXCL)`—Enables exclusive mode on a terminal. |
| [`ioctl_tiocnxcl`](#ioctl-tiocnxcl) | fn | `ioctl(fd, TIOCNXCL)`—Disables exclusive mode on a terminal. |
| [`tcgetattr`](#tcgetattr) | fn | `tcgetattr(fd)`—Get terminal attributes. |
| [`tcgetwinsize`](#tcgetwinsize) | fn | `tcgetwinsize(fd)`—Get the current terminal window size. |
| [`tcgetpgrp`](#tcgetpgrp) | fn | `tcgetpgrp(fd)`—Get the terminal foreground process group. |
| [`tcsetpgrp`](#tcsetpgrp) | fn | `tcsetpgrp(fd, pid)`—Set the terminal foreground process group. |
| [`tcsetattr`](#tcsetattr) | fn | `tcsetattr(fd)`—Set terminal attributes. |
| [`tcsendbreak`](#tcsendbreak) | fn | `tcsendbreak(fd, 0)`—Transmit zero-valued bits. |
| [`tcdrain`](#tcdrain) | fn | `tcdrain(fd, duration)`—Wait until all pending output has been written. |
| [`tcflush`](#tcflush) | fn | `tcflush(fd, queue_selector)`—Wait until all pending output has been written. |
| [`tcflow`](#tcflow) | fn | `tcflow(fd, action)`—Suspend or resume transmission or reception. |
| [`tcgetsid`](#tcgetsid) | fn | `tcgetsid(fd)`—Return the session ID of the current session with `fd` as its controlling terminal. |
| [`tcsetwinsize`](#tcsetwinsize) | fn | `tcsetwinsize(fd)`—Set the current terminal window size. |
| [`isatty`](#isatty) | fn | `isatty(fd)`—Tests whether a file descriptor refers to a terminal. |

## Modules

- [`ioctl`](ioctl/index.md) — Terminal-related `ioctl` functions.
- [`tc`](tc/index.md)
- [`tty`](tty/index.md) — Functions which operate on file descriptors which might be terminals.
- [`types`](types/index.md)
- [`speed`](speed/index.md) — Speeds for use with [`Termios::set_speed`], [`Termios::set_input_speed`],

## Structs

### `Termios`

```rust
struct Termios {
    pub input_modes: InputModes,
    pub output_modes: OutputModes,
    pub control_modes: ControlModes,
    pub local_modes: LocalModes,
    pub line_discipline: u8,
    pub special_codes: SpecialCodes,
    input_speed: linux_raw_sys::general::speed_t,
    output_speed: linux_raw_sys::general::speed_t,
}
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:14-75`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L14-L75)*

`struct termios` for use with [`tcgetattr`](#tcgetattr) and [`tcsetattr`](../backend/termios/syscalls/index.md).



#### Fields

- **`input_modes`**: `InputModes`

  How is input interpreted?

- **`output_modes`**: `OutputModes`

  How is output translated?

- **`control_modes`**: `ControlModes`

  Low-level configuration flags.

- **`local_modes`**: `LocalModes`

  High-level configuration flags.

- **`line_discipline`**: `u8`

  Line discipline.

- **`special_codes`**: `SpecialCodes`

  How are various special control codes handled?

- **`input_speed`**: `linux_raw_sys::general::speed_t`

  See the `input_speed` and `set_input_seed` functions.
  
  On Linux and BSDs, this is the arbitrary integer speed value. On all
  other platforms, this is the encoded speed value.

- **`output_speed`**: `linux_raw_sys::general::speed_t`

  See the `output_speed` and `set_output_seed` functions.
  
  On Linux and BSDs, this is the integer speed value. On all other
  platforms, this is the encoded speed value.

#### Implementations

- <span id="termios-make-raw"></span>`fn make_raw(&mut self)`

  `cfmakeraw(self)`—Set a `Termios` value to the settings for “raw” mode.

  

  In raw mode, input is available a byte at a time, echoing is disabled,

  and special terminal input and output codes are disabled.

- <span id="termios-input-speed"></span>`fn input_speed(&self) -> u32`

  Return the input communication speed.

  

  Unlike the `c_ispeed` field in glibc and others, this returns the

  integer value of the speed, rather than the `B*` encoded constant

  value.

- <span id="termios-output-speed"></span>`fn output_speed(&self) -> u32`

  Return the output communication speed.

  

  Unlike the `c_ospeed` field in glibc and others, this returns the

  arbitrary integer value of the speed, rather than the `B*` encoded

  constant value.

- <span id="termios-set-speed"></span>`fn set_speed(&mut self, new_speed: u32) -> io::Result<()>` — [`Result`](../io/errno/index.md#result)

  Set the input and output communication speeds.

  

  Unlike the `c_ispeed` and `c_ospeed` fields in glibc and others, this

  takes the arbitrary integer value of the speed, rather than the `B*`

  encoded constant value. Not all implementations support all integer

  values; use the constants in the [`speed`](#speed) module for likely-supported

  speeds.

- <span id="termios-set-input-speed"></span>`fn set_input_speed(&mut self, new_speed: u32) -> io::Result<()>` — [`Result`](../io/errno/index.md#result)

  Set the input communication speed.

  

  Unlike the `c_ispeed` field in glibc and others, this takes the

  arbitrary integer value of the speed, rather than the `B*` encoded

  constant value. Not all implementations support all integer values; use

  the constants in the [`speed`](#speed) module for known-supported speeds.

  

  On some platforms, changing the input speed changes the output speed to

  the same speed.

- <span id="termios-set-output-speed"></span>`fn set_output_speed(&mut self, new_speed: u32) -> io::Result<()>` — [`Result`](../io/errno/index.md#result)

  Set the output communication speed.

  

  Unlike the `c_ospeed` field in glibc and others, this takes the

  arbitrary integer value of the speed, rather than the `B*` encoded

  constant value. Not all implementations support all integer values; use

  the constants in the [`speed`](#speed) module for known-supported speeds.

  

  On some platforms, changing the output speed changes the input speed to

  the same speed.

#### Trait Implementations

##### `impl Any for Termios`

- <span id="termios-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Termios`

- <span id="termios-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Termios`

- <span id="termios-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Termios`

- <span id="termios-clone"></span>`fn clone(&self) -> Termios` — [`Termios`](#termios)

##### `impl CloneToUninit for Termios`

- <span id="termios-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Termios`

- <span id="termios-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for Termios`

- <span id="termios-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Termios`

- <span id="termios-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Termios`

- <span id="termios-toowned-type-owned"></span>`type Owned = T`

- <span id="termios-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="termios-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Termios`

- <span id="termios-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="termios-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Termios`

- <span id="termios-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="termios-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `InputModes`

```rust
struct InputModes(<InputModes as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:245-311`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L245-L311)*

Flags controlling terminal input.

#### Implementations

- <span id="inputmodes-const-ignbrk"></span>`const IGNBRK: Self`

- <span id="inputmodes-const-brkint"></span>`const BRKINT: Self`

- <span id="inputmodes-const-ignpar"></span>`const IGNPAR: Self`

- <span id="inputmodes-const-parmrk"></span>`const PARMRK: Self`

- <span id="inputmodes-const-inpck"></span>`const INPCK: Self`

- <span id="inputmodes-const-istrip"></span>`const ISTRIP: Self`

- <span id="inputmodes-const-inlcr"></span>`const INLCR: Self`

- <span id="inputmodes-const-igncr"></span>`const IGNCR: Self`

- <span id="inputmodes-const-icrnl"></span>`const ICRNL: Self`

- <span id="inputmodes-const-iuclc"></span>`const IUCLC: Self`

- <span id="inputmodes-const-ixon"></span>`const IXON: Self`

- <span id="inputmodes-const-ixany"></span>`const IXANY: Self`

- <span id="inputmodes-const-ixoff"></span>`const IXOFF: Self`

- <span id="inputmodes-const-imaxbel"></span>`const IMAXBEL: Self`

- <span id="inputmodes-const-iutf8"></span>`const IUTF8: Self`

#### Trait Implementations

##### `impl Any for InputModes`

- <span id="inputmodes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Binary for InputModes`

- <span id="inputmodes-binary-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for InputModes`

- <span id="inputmodes-bitand-type-output"></span>`type Output = InputModes`

- <span id="inputmodes-bitand"></span>`fn bitand(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitAndAssign for InputModes`

- <span id="inputmodes-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitOr for InputModes`

- <span id="inputmodes-bitor-type-output"></span>`type Output = InputModes`

- <span id="inputmodes-bitor"></span>`fn bitor(self, other: InputModes) -> Self` — [`InputModes`](#inputmodes)

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitOrAssign for InputModes`

- <span id="inputmodes-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitXor for InputModes`

- <span id="inputmodes-bitxor-type-output"></span>`type Output = InputModes`

- <span id="inputmodes-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl BitXorAssign for InputModes`

- <span id="inputmodes-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl<T> Borrow for InputModes`

- <span id="inputmodes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for InputModes`

- <span id="inputmodes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for InputModes`

- <span id="inputmodes-clone"></span>`fn clone(&self) -> InputModes` — [`InputModes`](#inputmodes)

##### `impl CloneToUninit for InputModes`

- <span id="inputmodes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for InputModes`

##### `impl Debug for InputModes`

- <span id="inputmodes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for InputModes`

##### `impl Extend for InputModes`

- <span id="inputmodes-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Flags for InputModes`

- <span id="inputmodes-flags-const-flags"></span>`const FLAGS: &'static [Flag<InputModes>]`

- <span id="inputmodes-flags-type-bits"></span>`type Bits = u32`

- <span id="inputmodes-flags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../ffi/index.md#c-uint)

- <span id="inputmodes-flags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> InputModes` — [`c_uint`](../ffi/index.md#c-uint), [`InputModes`](#inputmodes)

##### `impl<T> From for InputModes`

- <span id="inputmodes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for InputModes`

- <span id="inputmodes-fromiterator-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Hash for InputModes`

- <span id="inputmodes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for InputModes`

- <span id="inputmodes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for InputModes`

- <span id="inputmodes-intoiterator-type-item"></span>`type Item = InputModes`

- <span id="inputmodes-intoiterator-type-intoiter"></span>`type IntoIter = Iter<InputModes>`

- <span id="inputmodes-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for InputModes`

- <span id="inputmodes-lowerhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for InputModes`

- <span id="inputmodes-not-type-output"></span>`type Output = InputModes`

- <span id="inputmodes-not"></span>`fn not(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### `impl Octal for InputModes`

- <span id="inputmodes-octal-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for InputModes`

- <span id="inputmodes-partialeq-eq"></span>`fn eq(&self, other: &InputModes) -> bool` — [`InputModes`](#inputmodes)

##### `impl PublicFlags for InputModes`

- <span id="inputmodes-publicflags-type-primitive"></span>`type Primitive = u32`

- <span id="inputmodes-publicflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for InputModes`

##### `impl Sub for InputModes`

- <span id="inputmodes-sub-type-output"></span>`type Output = InputModes`

- <span id="inputmodes-sub"></span>`fn sub(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl SubAssign for InputModes`

- <span id="inputmodes-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl ToOwned for InputModes`

- <span id="inputmodes-toowned-type-owned"></span>`type Owned = T`

- <span id="inputmodes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="inputmodes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for InputModes`

- <span id="inputmodes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="inputmodes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for InputModes`

- <span id="inputmodes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="inputmodes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperHex for InputModes`

- <span id="inputmodes-upperhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `OutputModes`

```rust
struct OutputModes(<OutputModes as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:313-526`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L313-L526)*

Flags controlling terminal output.

#### Implementations

- <span id="outputmodes-const-opost"></span>`const OPOST: Self`

- <span id="outputmodes-const-olcuc"></span>`const OLCUC: Self`

- <span id="outputmodes-const-onlcr"></span>`const ONLCR: Self`

- <span id="outputmodes-const-ocrnl"></span>`const OCRNL: Self`

- <span id="outputmodes-const-onocr"></span>`const ONOCR: Self`

- <span id="outputmodes-const-onlret"></span>`const ONLRET: Self`

- <span id="outputmodes-const-ofill"></span>`const OFILL: Self`

- <span id="outputmodes-const-ofdel"></span>`const OFDEL: Self`

- <span id="outputmodes-const-nldly"></span>`const NLDLY: Self`

- <span id="outputmodes-const-nl0"></span>`const NL0: Self`

- <span id="outputmodes-const-nl1"></span>`const NL1: Self`

- <span id="outputmodes-const-crdly"></span>`const CRDLY: Self`

- <span id="outputmodes-const-cr0"></span>`const CR0: Self`

- <span id="outputmodes-const-cr1"></span>`const CR1: Self`

- <span id="outputmodes-const-cr2"></span>`const CR2: Self`

- <span id="outputmodes-const-cr3"></span>`const CR3: Self`

- <span id="outputmodes-const-tabdly"></span>`const TABDLY: Self`

- <span id="outputmodes-const-tab0"></span>`const TAB0: Self`

- <span id="outputmodes-const-tab1"></span>`const TAB1: Self`

- <span id="outputmodes-const-tab2"></span>`const TAB2: Self`

- <span id="outputmodes-const-tab3"></span>`const TAB3: Self`

- <span id="outputmodes-const-xtabs"></span>`const XTABS: Self`

- <span id="outputmodes-const-bsdly"></span>`const BSDLY: Self`

- <span id="outputmodes-const-bs0"></span>`const BS0: Self`

- <span id="outputmodes-const-bs1"></span>`const BS1: Self`

- <span id="outputmodes-const-ffdly"></span>`const FFDLY: Self`

- <span id="outputmodes-const-ff0"></span>`const FF0: Self`

- <span id="outputmodes-const-ff1"></span>`const FF1: Self`

- <span id="outputmodes-const-vtdly"></span>`const VTDLY: Self`

- <span id="outputmodes-const-vt0"></span>`const VT0: Self`

- <span id="outputmodes-const-vt1"></span>`const VT1: Self`

#### Trait Implementations

##### `impl Any for OutputModes`

- <span id="outputmodes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Binary for OutputModes`

- <span id="outputmodes-binary-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for OutputModes`

- <span id="outputmodes-bitand-type-output"></span>`type Output = OutputModes`

- <span id="outputmodes-bitand"></span>`fn bitand(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitAndAssign for OutputModes`

- <span id="outputmodes-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitOr for OutputModes`

- <span id="outputmodes-bitor-type-output"></span>`type Output = OutputModes`

- <span id="outputmodes-bitor"></span>`fn bitor(self, other: OutputModes) -> Self` — [`OutputModes`](#outputmodes)

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitOrAssign for OutputModes`

- <span id="outputmodes-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitXor for OutputModes`

- <span id="outputmodes-bitxor-type-output"></span>`type Output = OutputModes`

- <span id="outputmodes-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl BitXorAssign for OutputModes`

- <span id="outputmodes-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl<T> Borrow for OutputModes`

- <span id="outputmodes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OutputModes`

- <span id="outputmodes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OutputModes`

- <span id="outputmodes-clone"></span>`fn clone(&self) -> OutputModes` — [`OutputModes`](#outputmodes)

##### `impl CloneToUninit for OutputModes`

- <span id="outputmodes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for OutputModes`

##### `impl Debug for OutputModes`

- <span id="outputmodes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for OutputModes`

##### `impl Extend for OutputModes`

- <span id="outputmodes-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Flags for OutputModes`

- <span id="outputmodes-flags-const-flags"></span>`const FLAGS: &'static [Flag<OutputModes>]`

- <span id="outputmodes-flags-type-bits"></span>`type Bits = u32`

- <span id="outputmodes-flags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../ffi/index.md#c-uint)

- <span id="outputmodes-flags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> OutputModes` — [`c_uint`](../ffi/index.md#c-uint), [`OutputModes`](#outputmodes)

##### `impl<T> From for OutputModes`

- <span id="outputmodes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for OutputModes`

- <span id="outputmodes-fromiterator-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Hash for OutputModes`

- <span id="outputmodes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for OutputModes`

- <span id="outputmodes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for OutputModes`

- <span id="outputmodes-intoiterator-type-item"></span>`type Item = OutputModes`

- <span id="outputmodes-intoiterator-type-intoiter"></span>`type IntoIter = Iter<OutputModes>`

- <span id="outputmodes-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for OutputModes`

- <span id="outputmodes-lowerhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for OutputModes`

- <span id="outputmodes-not-type-output"></span>`type Output = OutputModes`

- <span id="outputmodes-not"></span>`fn not(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### `impl Octal for OutputModes`

- <span id="outputmodes-octal-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for OutputModes`

- <span id="outputmodes-partialeq-eq"></span>`fn eq(&self, other: &OutputModes) -> bool` — [`OutputModes`](#outputmodes)

##### `impl PublicFlags for OutputModes`

- <span id="outputmodes-publicflags-type-primitive"></span>`type Primitive = u32`

- <span id="outputmodes-publicflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for OutputModes`

##### `impl Sub for OutputModes`

- <span id="outputmodes-sub-type-output"></span>`type Output = OutputModes`

- <span id="outputmodes-sub"></span>`fn sub(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl SubAssign for OutputModes`

- <span id="outputmodes-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl ToOwned for OutputModes`

- <span id="outputmodes-toowned-type-owned"></span>`type Owned = T`

- <span id="outputmodes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="outputmodes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OutputModes`

- <span id="outputmodes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="outputmodes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OutputModes`

- <span id="outputmodes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="outputmodes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperHex for OutputModes`

- <span id="outputmodes-upperhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `ControlModes`

```rust
struct ControlModes(<ControlModes as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:528-592`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L528-L592)*

Flags controlling special terminal modes.

`CBAUD`, `CBAUDEX`, `CIBAUD`, `CIBAUDEX`, and various `B*` speed
constants are often included in the control modes, however rustix
handles them separately, in `Termios::set_speed` and related
functions. If you see extra bits in the `Debug` output, they're
probably these flags.

#### Implementations

- <span id="controlmodes-const-csize"></span>`const CSIZE: Self`

- <span id="controlmodes-const-cs5"></span>`const CS5: Self`

- <span id="controlmodes-const-cs6"></span>`const CS6: Self`

- <span id="controlmodes-const-cs7"></span>`const CS7: Self`

- <span id="controlmodes-const-cs8"></span>`const CS8: Self`

- <span id="controlmodes-const-cstopb"></span>`const CSTOPB: Self`

- <span id="controlmodes-const-cread"></span>`const CREAD: Self`

- <span id="controlmodes-const-parenb"></span>`const PARENB: Self`

- <span id="controlmodes-const-parodd"></span>`const PARODD: Self`

- <span id="controlmodes-const-hupcl"></span>`const HUPCL: Self`

- <span id="controlmodes-const-clocal"></span>`const CLOCAL: Self`

- <span id="controlmodes-const-crtscts"></span>`const CRTSCTS: Self`

- <span id="controlmodes-const-cmspar"></span>`const CMSPAR: Self`

#### Trait Implementations

##### `impl Any for ControlModes`

- <span id="controlmodes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Binary for ControlModes`

- <span id="controlmodes-binary-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for ControlModes`

- <span id="controlmodes-bitand-type-output"></span>`type Output = ControlModes`

- <span id="controlmodes-bitand"></span>`fn bitand(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitAndAssign for ControlModes`

- <span id="controlmodes-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitOr for ControlModes`

- <span id="controlmodes-bitor-type-output"></span>`type Output = ControlModes`

- <span id="controlmodes-bitor"></span>`fn bitor(self, other: ControlModes) -> Self` — [`ControlModes`](#controlmodes)

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitOrAssign for ControlModes`

- <span id="controlmodes-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitXor for ControlModes`

- <span id="controlmodes-bitxor-type-output"></span>`type Output = ControlModes`

- <span id="controlmodes-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl BitXorAssign for ControlModes`

- <span id="controlmodes-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl<T> Borrow for ControlModes`

- <span id="controlmodes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ControlModes`

- <span id="controlmodes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ControlModes`

- <span id="controlmodes-clone"></span>`fn clone(&self) -> ControlModes` — [`ControlModes`](#controlmodes)

##### `impl CloneToUninit for ControlModes`

- <span id="controlmodes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ControlModes`

##### `impl Debug for ControlModes`

- <span id="controlmodes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ControlModes`

##### `impl Extend for ControlModes`

- <span id="controlmodes-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Flags for ControlModes`

- <span id="controlmodes-flags-const-flags"></span>`const FLAGS: &'static [Flag<ControlModes>]`

- <span id="controlmodes-flags-type-bits"></span>`type Bits = u32`

- <span id="controlmodes-flags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../ffi/index.md#c-uint)

- <span id="controlmodes-flags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> ControlModes` — [`c_uint`](../ffi/index.md#c-uint), [`ControlModes`](#controlmodes)

##### `impl<T> From for ControlModes`

- <span id="controlmodes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for ControlModes`

- <span id="controlmodes-fromiterator-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Hash for ControlModes`

- <span id="controlmodes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ControlModes`

- <span id="controlmodes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ControlModes`

- <span id="controlmodes-intoiterator-type-item"></span>`type Item = ControlModes`

- <span id="controlmodes-intoiterator-type-intoiter"></span>`type IntoIter = Iter<ControlModes>`

- <span id="controlmodes-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for ControlModes`

- <span id="controlmodes-lowerhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for ControlModes`

- <span id="controlmodes-not-type-output"></span>`type Output = ControlModes`

- <span id="controlmodes-not"></span>`fn not(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### `impl Octal for ControlModes`

- <span id="controlmodes-octal-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for ControlModes`

- <span id="controlmodes-partialeq-eq"></span>`fn eq(&self, other: &ControlModes) -> bool` — [`ControlModes`](#controlmodes)

##### `impl PublicFlags for ControlModes`

- <span id="controlmodes-publicflags-type-primitive"></span>`type Primitive = u32`

- <span id="controlmodes-publicflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for ControlModes`

##### `impl Sub for ControlModes`

- <span id="controlmodes-sub-type-output"></span>`type Output = ControlModes`

- <span id="controlmodes-sub"></span>`fn sub(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl SubAssign for ControlModes`

- <span id="controlmodes-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl ToOwned for ControlModes`

- <span id="controlmodes-toowned-type-owned"></span>`type Owned = T`

- <span id="controlmodes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="controlmodes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ControlModes`

- <span id="controlmodes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="controlmodes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ControlModes`

- <span id="controlmodes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="controlmodes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperHex for ControlModes`

- <span id="controlmodes-upperhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `LocalModes`

```rust
struct LocalModes(<LocalModes as __private::PublicFlags>::Internal);
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:594-664`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L594-L664)*

Flags controlling “local” terminal modes.

#### Implementations

- <span id="localmodes-const-xcase"></span>`const XCASE: Self`

- <span id="localmodes-const-echoctl"></span>`const ECHOCTL: Self`

- <span id="localmodes-const-echoprt"></span>`const ECHOPRT: Self`

- <span id="localmodes-const-echoke"></span>`const ECHOKE: Self`

- <span id="localmodes-const-flusho"></span>`const FLUSHO: Self`

- <span id="localmodes-const-pendin"></span>`const PENDIN: Self`

- <span id="localmodes-const-extproc"></span>`const EXTPROC: Self`

- <span id="localmodes-const-isig"></span>`const ISIG: Self`

- <span id="localmodes-const-icanon"></span>`const ICANON: Self`

- <span id="localmodes-const-echo"></span>`const ECHO: Self`

- <span id="localmodes-const-echoe"></span>`const ECHOE: Self`

- <span id="localmodes-const-echok"></span>`const ECHOK: Self`

- <span id="localmodes-const-echonl"></span>`const ECHONL: Self`

- <span id="localmodes-const-noflsh"></span>`const NOFLSH: Self`

- <span id="localmodes-const-tostop"></span>`const TOSTOP: Self`

- <span id="localmodes-const-iexten"></span>`const IEXTEN: Self`

#### Trait Implementations

##### `impl Any for LocalModes`

- <span id="localmodes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Binary for LocalModes`

- <span id="localmodes-binary-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl BitAnd for LocalModes`

- <span id="localmodes-bitand-type-output"></span>`type Output = LocalModes`

- <span id="localmodes-bitand"></span>`fn bitand(self, other: Self) -> Self`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitAndAssign for LocalModes`

- <span id="localmodes-bitandassign-bitand-assign"></span>`fn bitand_assign(&mut self, other: Self)`

  The bitwise and (`&`) of the bits in two flags values.

##### `impl BitOr for LocalModes`

- <span id="localmodes-bitor-type-output"></span>`type Output = LocalModes`

- <span id="localmodes-bitor"></span>`fn bitor(self, other: LocalModes) -> Self` — [`LocalModes`](#localmodes)

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitOrAssign for LocalModes`

- <span id="localmodes-bitorassign-bitor-assign"></span>`fn bitor_assign(&mut self, other: Self)`

  The bitwise or (`|`) of the bits in two flags values.

##### `impl BitXor for LocalModes`

- <span id="localmodes-bitxor-type-output"></span>`type Output = LocalModes`

- <span id="localmodes-bitxor"></span>`fn bitxor(self, other: Self) -> Self`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl BitXorAssign for LocalModes`

- <span id="localmodes-bitxorassign-bitxor-assign"></span>`fn bitxor_assign(&mut self, other: Self)`

  The bitwise exclusive-or (`^`) of the bits in two flags values.

##### `impl<T> Borrow for LocalModes`

- <span id="localmodes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LocalModes`

- <span id="localmodes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for LocalModes`

- <span id="localmodes-clone"></span>`fn clone(&self) -> LocalModes` — [`LocalModes`](#localmodes)

##### `impl CloneToUninit for LocalModes`

- <span id="localmodes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for LocalModes`

##### `impl Debug for LocalModes`

- <span id="localmodes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LocalModes`

##### `impl Extend for LocalModes`

- <span id="localmodes-extend"></span>`fn extend<T: __private::core::iter::IntoIterator<Item = Self>>(&mut self, iterator: T)`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Flags for LocalModes`

- <span id="localmodes-flags-const-flags"></span>`const FLAGS: &'static [Flag<LocalModes>]`

- <span id="localmodes-flags-type-bits"></span>`type Bits = u32`

- <span id="localmodes-flags-bits"></span>`fn bits(&self) -> ffi::c_uint` — [`c_uint`](../ffi/index.md#c-uint)

- <span id="localmodes-flags-from-bits-retain"></span>`fn from_bits_retain(bits: ffi::c_uint) -> LocalModes` — [`c_uint`](../ffi/index.md#c-uint), [`LocalModes`](#localmodes)

##### `impl<T> From for LocalModes`

- <span id="localmodes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromIterator for LocalModes`

- <span id="localmodes-fromiterator-from-iter"></span>`fn from_iter<T: __private::core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self`

  The bitwise or (`|`) of the bits in each flags value.

##### `impl Hash for LocalModes`

- <span id="localmodes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for LocalModes`

- <span id="localmodes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for LocalModes`

- <span id="localmodes-intoiterator-type-item"></span>`type Item = LocalModes`

- <span id="localmodes-intoiterator-type-intoiter"></span>`type IntoIter = Iter<LocalModes>`

- <span id="localmodes-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl LowerHex for LocalModes`

- <span id="localmodes-lowerhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl Not for LocalModes`

- <span id="localmodes-not-type-output"></span>`type Output = LocalModes`

- <span id="localmodes-not"></span>`fn not(self) -> Self`

  The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### `impl Octal for LocalModes`

- <span id="localmodes-octal-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

##### `impl PartialEq for LocalModes`

- <span id="localmodes-partialeq-eq"></span>`fn eq(&self, other: &LocalModes) -> bool` — [`LocalModes`](#localmodes)

##### `impl PublicFlags for LocalModes`

- <span id="localmodes-publicflags-type-primitive"></span>`type Primitive = u32`

- <span id="localmodes-publicflags-type-internal"></span>`type Internal = InternalBitFlags`

##### `impl StructuralPartialEq for LocalModes`

##### `impl Sub for LocalModes`

- <span id="localmodes-sub-type-output"></span>`type Output = LocalModes`

- <span id="localmodes-sub"></span>`fn sub(self, other: Self) -> Self`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl SubAssign for LocalModes`

- <span id="localmodes-subassign-sub-assign"></span>`fn sub_assign(&mut self, other: Self)`

  The intersection of a source flags value with the complement of a target flags value (`&!`).

  

  This method is not equivalent to `self & !other` when `other` has unknown bits set.

  `difference` won't truncate `other`, but the `!` operator will.

##### `impl ToOwned for LocalModes`

- <span id="localmodes-toowned-type-owned"></span>`type Owned = T`

- <span id="localmodes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="localmodes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for LocalModes`

- <span id="localmodes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="localmodes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LocalModes`

- <span id="localmodes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="localmodes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl UpperHex for LocalModes`

- <span id="localmodes-upperhex-fmt"></span>`fn fmt(&self, f: &mut __private::core::fmt::Formatter<'_>) -> __private::core::fmt::Result`

### `SpecialCodes`

```rust
struct SpecialCodes([linux_raw_sys::general::cc_t; 19]);
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:1124`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L1124)*

An array indexed by [`SpecialCodeIndex`](#specialcodeindex) indicating the current values of
various special control codes.

#### Trait Implementations

##### `impl Any for SpecialCodes`

- <span id="specialcodes-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SpecialCodes`

- <span id="specialcodes-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SpecialCodes`

- <span id="specialcodes-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SpecialCodes`

- <span id="specialcodes-clone"></span>`fn clone(&self) -> SpecialCodes` — [`SpecialCodes`](#specialcodes)

##### `impl CloneToUninit for SpecialCodes`

- <span id="specialcodes-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SpecialCodes`

- <span id="specialcodes-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for SpecialCodes`

- <span id="specialcodes-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Index for SpecialCodes`

- <span id="specialcodes-index-type-output"></span>`type Output = u8`

- <span id="specialcodes-index"></span>`fn index(&self, index: SpecialCodeIndex) -> &<Self as >::Output` — [`SpecialCodeIndex`](#specialcodeindex)

##### `impl IndexMut for SpecialCodes`

- <span id="specialcodes-indexmut-index-mut"></span>`fn index_mut(&mut self, index: SpecialCodeIndex) -> &mut <Self as >::Output` — [`SpecialCodeIndex`](#specialcodeindex)

##### `impl<U> Into for SpecialCodes`

- <span id="specialcodes-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SpecialCodes`

- <span id="specialcodes-toowned-type-owned"></span>`type Owned = T`

- <span id="specialcodes-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="specialcodes-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SpecialCodes`

- <span id="specialcodes-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="specialcodes-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SpecialCodes`

- <span id="specialcodes-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="specialcodes-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SpecialCode`

```rust
struct SpecialCode(u8);
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:1162`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L1162)*

A newtype for pretty printing.

#### Trait Implementations

##### `impl Any for SpecialCode`

- <span id="specialcode-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SpecialCode`

- <span id="specialcode-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SpecialCode`

- <span id="specialcode-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SpecialCode`

- <span id="specialcode-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for SpecialCode`

- <span id="specialcode-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SpecialCode`

- <span id="specialcode-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for SpecialCode`

- <span id="specialcode-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="specialcode-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SpecialCode`

- <span id="specialcode-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="specialcode-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SpecialCodeIndex`

```rust
struct SpecialCodeIndex(usize);
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:1183`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L1183)*

Indices for use with `Termios::special_codes`.

#### Implementations

- <span id="specialcodeindex-const-vintr"></span>`const VINTR: Self`

- <span id="specialcodeindex-const-vquit"></span>`const VQUIT: Self`

- <span id="specialcodeindex-const-verase"></span>`const VERASE: Self`

- <span id="specialcodeindex-const-vkill"></span>`const VKILL: Self`

- <span id="specialcodeindex-const-veof"></span>`const VEOF: Self`

- <span id="specialcodeindex-const-vtime"></span>`const VTIME: Self`

- <span id="specialcodeindex-const-vmin"></span>`const VMIN: Self`

- <span id="specialcodeindex-const-vswtc"></span>`const VSWTC: Self`

- <span id="specialcodeindex-const-vstart"></span>`const VSTART: Self`

- <span id="specialcodeindex-const-vstop"></span>`const VSTOP: Self`

- <span id="specialcodeindex-const-vsusp"></span>`const VSUSP: Self`

- <span id="specialcodeindex-const-veol"></span>`const VEOL: Self`

- <span id="specialcodeindex-const-vreprint"></span>`const VREPRINT: Self`

- <span id="specialcodeindex-const-vdiscard"></span>`const VDISCARD: Self`

- <span id="specialcodeindex-const-vwerase"></span>`const VWERASE: Self`

- <span id="specialcodeindex-const-vlnext"></span>`const VLNEXT: Self`

- <span id="specialcodeindex-const-veol2"></span>`const VEOL2: Self`

#### Trait Implementations

##### `impl Any for SpecialCodeIndex`

- <span id="specialcodeindex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SpecialCodeIndex`

- <span id="specialcodeindex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SpecialCodeIndex`

- <span id="specialcodeindex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SpecialCodeIndex`

- <span id="specialcodeindex-clone"></span>`fn clone(&self) -> SpecialCodeIndex` — [`SpecialCodeIndex`](#specialcodeindex)

##### `impl CloneToUninit for SpecialCodeIndex`

- <span id="specialcodeindex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for SpecialCodeIndex`

##### `impl Debug for SpecialCodeIndex`

- <span id="specialcodeindex-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for SpecialCodeIndex`

##### `impl<T> From for SpecialCodeIndex`

- <span id="specialcodeindex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for SpecialCodeIndex`

- <span id="specialcodeindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Index for SpecialCodes`

- <span id="specialcodes-index-type-output"></span>`type Output = u8`

- <span id="specialcodes-index"></span>`fn index(&self, index: SpecialCodeIndex) -> &<Self as >::Output` — [`SpecialCodeIndex`](#specialcodeindex)

##### `impl IndexMut for SpecialCodes`

- <span id="specialcodes-indexmut-index-mut"></span>`fn index_mut(&mut self, index: SpecialCodeIndex) -> &mut <Self as >::Output` — [`SpecialCodeIndex`](#specialcodeindex)

##### `impl<U> Into for SpecialCodeIndex`

- <span id="specialcodeindex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SpecialCodeIndex`

- <span id="specialcodeindex-partialeq-eq"></span>`fn eq(&self, other: &SpecialCodeIndex) -> bool` — [`SpecialCodeIndex`](#specialcodeindex)

##### `impl StructuralPartialEq for SpecialCodeIndex`

##### `impl ToOwned for SpecialCodeIndex`

- <span id="specialcodeindex-toowned-type-owned"></span>`type Owned = T`

- <span id="specialcodeindex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="specialcodeindex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SpecialCodeIndex`

- <span id="specialcodeindex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="specialcodeindex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SpecialCodeIndex`

- <span id="specialcodeindex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="specialcodeindex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Winsize`

```rust
struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:1436-1444`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L1436-L1444)*

`struct winsize` for use with [`tcgetwinsize`](../backend/termios/syscalls/index.md).


#### Fields

- **`ws_row`**: `u16`

  The number of rows the terminal has.

- **`ws_col`**: `u16`

  The number of columns the terminal has.

#### Trait Implementations

##### `impl Any for Winsize`

- <span id="winsize-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Winsize`

- <span id="winsize-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Winsize`

- <span id="winsize-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Winsize`

- <span id="winsize-clone"></span>`fn clone(&self) -> Winsize` — [`Winsize`](#winsize)

##### `impl CloneToUninit for Winsize`

- <span id="winsize-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Winsize`

##### `impl Debug for Winsize`

- <span id="winsize-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Winsize`

##### `impl<T> From for Winsize`

- <span id="winsize-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Winsize`

- <span id="winsize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Winsize`

- <span id="winsize-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Winsize`

- <span id="winsize-partialeq-eq"></span>`fn eq(&self, other: &Winsize) -> bool` — [`Winsize`](#winsize)

##### `impl StructuralPartialEq for Winsize`

##### `impl ToOwned for Winsize`

- <span id="winsize-toowned-type-owned"></span>`type Owned = T`

- <span id="winsize-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="winsize-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Winsize`

- <span id="winsize-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="winsize-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Winsize`

- <span id="winsize-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="winsize-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `OptionalActions`

```rust
enum OptionalActions {
    Now,
    Drain,
    Flush,
}
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:1372-1385`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L1372-L1385)*

`TCSA*` values for use with [`tcsetattr`](../backend/termios/syscalls/index.md).


#### Variants

- **`Now`**

  `TCSANOW`—Make the change immediately.

- **`Drain`**

  `TCSADRAIN`—Make the change after all output has been transmitted.

- **`Flush`**

  `TCSAFLUSH`—Discard any pending input and then make the change
  after all output has been transmitted.

#### Trait Implementations

##### `impl Any for OptionalActions`

- <span id="optionalactions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OptionalActions`

- <span id="optionalactions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OptionalActions`

- <span id="optionalactions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OptionalActions`

- <span id="optionalactions-clone"></span>`fn clone(&self) -> OptionalActions` — [`OptionalActions`](#optionalactions)

##### `impl CloneToUninit for OptionalActions`

- <span id="optionalactions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for OptionalActions`

##### `impl Debug for OptionalActions`

- <span id="optionalactions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for OptionalActions`

##### `impl<T> From for OptionalActions`

- <span id="optionalactions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for OptionalActions`

- <span id="optionalactions-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for OptionalActions`

- <span id="optionalactions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for OptionalActions`

- <span id="optionalactions-partialeq-eq"></span>`fn eq(&self, other: &OptionalActions) -> bool` — [`OptionalActions`](#optionalactions)

##### `impl StructuralPartialEq for OptionalActions`

##### `impl ToOwned for OptionalActions`

- <span id="optionalactions-toowned-type-owned"></span>`type Owned = T`

- <span id="optionalactions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="optionalactions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OptionalActions`

- <span id="optionalactions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="optionalactions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OptionalActions`

- <span id="optionalactions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="optionalactions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `QueueSelector`

```rust
enum QueueSelector {
    IFlush,
    OFlush,
    IOFlush,
}
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:1392-1404`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L1392-L1404)*

`TC*` values for use with [`tcflush`](#tcflush).


#### Variants

- **`IFlush`**

  `TCIFLUSH`—Flush data received but not read.

- **`OFlush`**

  `TCOFLUSH`—Flush data written but not transmitted.

- **`IOFlush`**

  `TCIOFLUSH`—`IFlush` and `OFlush` combined.

#### Trait Implementations

##### `impl Any for QueueSelector`

- <span id="queueselector-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for QueueSelector`

- <span id="queueselector-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for QueueSelector`

- <span id="queueselector-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for QueueSelector`

- <span id="queueselector-clone"></span>`fn clone(&self) -> QueueSelector` — [`QueueSelector`](#queueselector)

##### `impl CloneToUninit for QueueSelector`

- <span id="queueselector-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for QueueSelector`

##### `impl Debug for QueueSelector`

- <span id="queueselector-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for QueueSelector`

##### `impl<T> From for QueueSelector`

- <span id="queueselector-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for QueueSelector`

- <span id="queueselector-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for QueueSelector`

- <span id="queueselector-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for QueueSelector`

- <span id="queueselector-partialeq-eq"></span>`fn eq(&self, other: &QueueSelector) -> bool` — [`QueueSelector`](#queueselector)

##### `impl StructuralPartialEq for QueueSelector`

##### `impl ToOwned for QueueSelector`

- <span id="queueselector-toowned-type-owned"></span>`type Owned = T`

- <span id="queueselector-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="queueselector-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for QueueSelector`

- <span id="queueselector-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="queueselector-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for QueueSelector`

- <span id="queueselector-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="queueselector-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Action`

```rust
enum Action {
    OOff,
    OOn,
    IOff,
    IOn,
}
```

*Defined in [`rustix-1.1.2/src/termios/types.rs:1411-1427`](../../../.source_1765633015/rustix-1.1.2/src/termios/types.rs#L1411-L1427)*

`TC*` values for use with [`tcflow`](#tcflow).


#### Variants

- **`OOff`**

  `TCOOFF`—Suspend output.

- **`OOn`**

  `TCOON`—Restart suspended output.

- **`IOff`**

  `TCIOFF`—Transmits a STOP byte.

- **`IOn`**

  `TCION`—Transmits a START byte.

#### Trait Implementations

##### `impl Any for Action`

- <span id="action-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Action`

- <span id="action-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Action`

- <span id="action-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Action`

- <span id="action-clone"></span>`fn clone(&self) -> Action` — [`Action`](#action)

##### `impl CloneToUninit for Action`

- <span id="action-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Action`

##### `impl Debug for Action`

- <span id="action-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Action`

##### `impl<T> From for Action`

- <span id="action-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Action`

- <span id="action-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Action`

- <span id="action-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Action`

- <span id="action-partialeq-eq"></span>`fn eq(&self, other: &Action) -> bool` — [`Action`](#action)

##### `impl StructuralPartialEq for Action`

##### `impl ToOwned for Action`

- <span id="action-toowned-type-owned"></span>`type Owned = T`

- <span id="action-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="action-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Action`

- <span id="action-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="action-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Action`

- <span id="action-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="action-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `ioctl_tiocexcl`

```rust
fn ioctl_tiocexcl<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/ioctl.rs:32-38`](../../../.source_1765633015/rustix-1.1.2/src/termios/ioctl.rs#L32-L38)*

`ioctl(fd, TIOCEXCL)`—Enables exclusive mode on a terminal.

In exclusive mode, subsequent unprivileged `open` calls on the terminal
device fail with `io::Errno::BUSY`.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]





### `ioctl_tiocnxcl`

```rust
fn ioctl_tiocnxcl<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/ioctl.rs:60-66`](../../../.source_1765633015/rustix-1.1.2/src/termios/ioctl.rs#L60-L66)*

`ioctl(fd, TIOCNXCL)`—Disables exclusive mode on a terminal.

# References
 - [Linux]
 - [FreeBSD]
 - [NetBSD]
 - [OpenBSD]





### `tcgetattr`

```rust
fn tcgetattr<Fd: AsFd>(fd: Fd) -> io::Result<crate::termios::Termios>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:30-32`](../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L30-L32)*

`tcgetattr(fd)`—Get terminal attributes.

Also known as the `TCGETS` (or `TCGETS2` on Linux) operation with `ioctl`.

On Linux, this uses `TCGETS2`. If that fails in a way that indicates that
the host doesn't support it, this falls back to the old `TCGETS`, manually
initializes the fields that `TCGETS` doesn't initialize, and fails with
`io::Errno::RANGE` if the input or output speeds cannot be supported.

# References
 - [POSIX `tcgetattr`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcgetwinsize`

```rust
fn tcgetwinsize<Fd: AsFd>(fd: Fd) -> io::Result<crate::termios::Winsize>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:50-52`](../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L50-L52)*

`tcgetwinsize(fd)`—Get the current terminal window size.

Also known as the `TIOCGWINSZ` operation with `ioctl`.

# References
 - [Linux]


### `tcgetpgrp`

```rust
fn tcgetpgrp<Fd: AsFd>(fd: Fd) -> io::Result<Pid>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:72-74`](../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L72-L74)*

`tcgetpgrp(fd)`—Get the terminal foreground process group.

Also known as the `TIOCGPGRP` operation with `ioctl`.

On Linux, if `fd` is a pseudo-terminal, the underlying system call here can
return a pid of 0, which rustix's `Pid` type doesn't support. So rustix
instead handles this case by failing with `io::Errno::OPNOTSUPP` if the
pid is 0.

# References
 - [POSIX]
 - [Linux]



### `tcsetpgrp`

```rust
fn tcsetpgrp<Fd: AsFd>(fd: Fd, pid: Pid) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:89-91`](../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L89-L91)*

`tcsetpgrp(fd, pid)`—Set the terminal foreground process group.

Also known as the `TIOCSPGRP` operation with `ioctl`.

# References
 - [POSIX]
 - [Linux]



### `tcsetattr`

```rust
fn tcsetattr<Fd: AsFd>(fd: Fd, optional_actions: crate::termios::OptionalActions, termios: &crate::termios::Termios) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:114-120`](../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L114-L120)*

`tcsetattr(fd)`—Set terminal attributes.

Also known as the `TCSETS` (or `TCSETS2` on Linux) operation with `ioctl`.

On Linux, this uses `TCSETS2`. If that fails in a way that indicates that
the host doesn't support it, this falls back to the old `TCSETS`, and fails
with `io::Errno::RANGE` if the input or output speeds cannot be supported.

# References
 - [POSIX `tcsetattr`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcsendbreak`

```rust
fn tcsendbreak<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:142-144`](../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L142-L144)*

`tcsendbreak(fd, 0)`—Transmit zero-valued bits.

This transmits zero-valued bits for at least 0.25 seconds.

This function does not have a `duration` parameter, and always uses the
implementation-defined value, which transmits for at least 0.25 seconds.

Also known as the `TCSBRK` operation with `ioctl`, with a duration
parameter of 0.

# References
 - [POSIX `tcsendbreak`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcdrain`

```rust
fn tcdrain<Fd: AsFd>(fd: Fd) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:158-160`](../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L158-L160)*

`tcdrain(fd, duration)`—Wait until all pending output has been written.

# References
 - [POSIX `tcdrain`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcflush`

```rust
fn tcflush<Fd: AsFd>(fd: Fd, queue_selector: crate::termios::QueueSelector) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:176-178`](../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L176-L178)*

`tcflush(fd, queue_selector)`—Wait until all pending output has been
written.

# References
 - [POSIX `tcflush`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcflow`

```rust
fn tcflow<Fd: AsFd>(fd: Fd, action: crate::termios::Action) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:193-195`](../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L193-L195)*

`tcflow(fd, action)`—Suspend or resume transmission or reception.

# References
 - [POSIX `tcflow`]
 - [Linux `ioctl_tty`]
 - [Linux `termios`]




### `tcgetsid`

```rust
fn tcgetsid<Fd: AsFd>(fd: Fd) -> io::Result<Pid>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:208-210`](../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L208-L210)*

`tcgetsid(fd)`—Return the session ID of the current session with `fd` as
its controlling terminal.

# References
 - [POSIX]
 - [Linux]



### `tcsetwinsize`

```rust
fn tcsetwinsize<Fd: AsFd>(fd: Fd, winsize: crate::termios::Winsize) -> io::Result<()>
```

*Defined in [`rustix-1.1.2/src/termios/tc.rs:223-225`](../../../.source_1765633015/rustix-1.1.2/src/termios/tc.rs#L223-L225)*

`tcsetwinsize(fd)`—Set the current terminal window size.

Also known as the `TIOCSWINSZ` operation with `ioctl`.

# References
 - [Linux]


### `isatty`

```rust
fn isatty<Fd: AsFd>(fd: Fd) -> bool
```

*Defined in [`rustix-1.1.2/src/termios/tty.rs:22-24`](../../../.source_1765633015/rustix-1.1.2/src/termios/tty.rs#L22-L24)*

`isatty(fd)`—Tests whether a file descriptor refers to a terminal.

# References
 - [POSIX]
 - [Linux]



