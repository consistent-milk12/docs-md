*[regex_automata](../../index.md) / [util](../index.md) / [interpolate](index.md)*

---

# Module `interpolate`

Provides routines for interpolating capture group references.

That is, if a replacement string contains references like `$foo` or `${foo1}`,
then they are replaced with the corresponding capture values for the groups
named `foo` and `foo1`, respectively. Similarly, syntax like `$1` and `${1}`
is supported as well, with `1` corresponding to a capture group index and not
a name.

This module provides the free functions [`string`](#string) and [`bytes`](#bytes), which
interpolate Rust Unicode strings and byte strings, respectively.

# Format

These routines support two different kinds of capture references: unbraced and
braced.

For the unbraced format, the format supported is `$ref` where `name` can be
any character in the class `[0-9A-Za-z_]`. `ref` is always the longest
possible parse. So for example, `$1a` corresponds to the capture group named
`1a` and not the capture group at index `1`. If `ref` matches `^[0-9]+$`, then
it is treated as a capture group index itself and not a name.

For the braced format, the format supported is `${ref}` where `ref` can be any
sequence of bytes except for `}`. If no closing brace occurs, then it is not
considered a capture reference. As with the unbraced format, if `ref` matches
`^[0-9]+$`, then it is treated as a capture group index and not a name.

The braced format is useful for exerting precise control over the name of the
capture reference. For example, `${1}a` corresponds to the capture group
reference `1` followed by the letter `a`, where as `$1a` (as mentioned above)
corresponds to the capture group reference `1a`. The braced format is also
useful for expressing capture group names that use characters not supported by
the unbraced format. For example, `${foo[bar].baz}` refers to the capture group
named `foo[bar].baz`.

If a capture group reference is found and it does not refer to a valid capture
group, then it will be replaced with the empty string.

To write a literal `$`, use `$$`.

To be clear, and as exhibited via the type signatures in the routines in this
module, it is impossible for a replacement string to be invalid. A replacement
string may not have the intended semantics, but the interpolation procedure
itself can never fail.

## Contents

- [Structs](#structs)
  - [`CaptureRef`](#captureref)
- [Enums](#enums)
  - [`Ref`](#ref)
- [Functions](#functions)
  - [`string`](#string)
  - [`bytes`](#bytes)
  - [`find_cap_ref`](#find-cap-ref)
  - [`find_cap_ref_braced`](#find-cap-ref-braced)
  - [`is_valid_cap_letter`](#is-valid-cap-letter)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`CaptureRef`](#captureref) | struct | `CaptureRef` represents a reference to a capture group inside some text. |
| [`Ref`](#ref) | enum | A reference to a capture group in some text. |
| [`string`](#string) | fn | Accepts a replacement string and interpolates capture references with their corresponding values. |
| [`bytes`](#bytes) | fn | Accepts a replacement byte string and interpolates capture references with their corresponding values. |
| [`find_cap_ref`](#find-cap-ref) | fn | Parses a possible reference to a capture group name in the given text, starting at the beginning of `replacement`. |
| [`find_cap_ref_braced`](#find-cap-ref-braced) | fn | Looks for a braced reference, e.g., `${foo1}`. |
| [`is_valid_cap_letter`](#is-valid-cap-letter) | fn | Returns true if and only if the given byte is allowed in a capture name written in non-brace form. |

## Structs

### `CaptureRef<'a>`

```rust
struct CaptureRef<'a> {
    cap: Ref<'a>,
    end: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/interpolate.rs:226-229`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/interpolate.rs#L226-L229)*

`CaptureRef` represents a reference to a capture group inside some text.
The reference is either a capture group name or a number.

It is also tagged with the position in the text following the
capture reference.

#### Trait Implementations

##### `impl Any for CaptureRef<'a>`

- <span id="captureref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CaptureRef<'a>`

- <span id="captureref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CaptureRef<'a>`

- <span id="captureref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CaptureRef<'a>`

- <span id="captureref-clone"></span>`fn clone(&self) -> CaptureRef<'a>` — [`CaptureRef`](#captureref)

##### `impl CloneToUninit for CaptureRef<'a>`

- <span id="captureref-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for CaptureRef<'a>`

##### `impl Debug for CaptureRef<'a>`

- <span id="captureref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CaptureRef<'a>`

##### `impl<T> From for CaptureRef<'a>`

- <span id="captureref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CaptureRef<'a>`

- <span id="captureref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for CaptureRef<'a>`

- <span id="captureref-partialeq-eq"></span>`fn eq(&self, other: &CaptureRef<'a>) -> bool` — [`CaptureRef`](#captureref)

##### `impl StructuralPartialEq for CaptureRef<'a>`

##### `impl ToOwned for CaptureRef<'a>`

- <span id="captureref-toowned-type-owned"></span>`type Owned = T`

- <span id="captureref-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="captureref-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CaptureRef<'a>`

- <span id="captureref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="captureref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CaptureRef<'a>`

- <span id="captureref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="captureref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Ref<'a>`

```rust
enum Ref<'a> {
    Named(&'a str),
    Number(usize),
}
```

*Defined in [`regex-automata-0.4.13/src/util/interpolate.rs:235-238`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/interpolate.rs#L235-L238)*

A reference to a capture group in some text.

e.g., `$2`, `$foo`, `${foo}`.

#### Trait Implementations

##### `impl Any for Ref<'a>`

- <span id="ref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Ref<'a>`

- <span id="ref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Ref<'a>`

- <span id="ref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Ref<'a>`

- <span id="ref-clone"></span>`fn clone(&self) -> Ref<'a>` — [`Ref`](#ref)

##### `impl CloneToUninit for Ref<'a>`

- <span id="ref-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Ref<'a>`

##### `impl Debug for Ref<'a>`

- <span id="ref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Ref<'a>`

##### `impl<T> From for Ref<'a>`

- <span id="ref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Ref<'a>`

- <span id="ref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Ref<'a>`

- <span id="ref-partialeq-eq"></span>`fn eq(&self, other: &Ref<'a>) -> bool` — [`Ref`](#ref)

##### `impl StructuralPartialEq for Ref<'a>`

##### `impl ToOwned for Ref<'a>`

- <span id="ref-toowned-type-owned"></span>`type Owned = T`

- <span id="ref-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ref-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Ref<'a>`

- <span id="ref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Ref<'a>`

- <span id="ref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `string`

```rust
fn string(replacement: &str, append: impl FnMut(usize, &mut alloc::string::String), name_to_index: impl FnMut(&str) -> Option<usize>, dst: &mut alloc::string::String)
```

*Defined in [`regex-automata-0.4.13/src/util/interpolate.rs:94-134`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/interpolate.rs#L94-L134)*

Accepts a replacement string and interpolates capture references with their
corresponding values.

`append` should be a function that appends the string value of a capture
group at a particular index to the string given. If the capture group
index is invalid, then nothing should be appended.

`name_to_index` should be a function that maps a capture group name to a
capture group index. If the given name doesn't exist, then `None` should
be returned.

Finally, `dst` is where the final interpolated contents should be written.
If `replacement` contains no capture group references, then `dst` will be
equivalent to `replacement`.

See the [module documentation](self) for details about the format
supported.

# Example

```rust
use regex_automata::util::interpolate;

let mut dst = String::new();
interpolate::string(
    "foo $bar baz",
    |index, dst| {
        if index == 0 {
            dst.push_str("BAR");
        }
    },
    |name| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    },
    &mut dst,
);
assert_eq!("foo BAR baz", dst);
```

### `bytes`

```rust
fn bytes(replacement: &[u8], append: impl FnMut(usize, &mut alloc::vec::Vec<u8>), name_to_index: impl FnMut(&str) -> Option<usize>, dst: &mut alloc::vec::Vec<u8>)
```

*Defined in [`regex-automata-0.4.13/src/util/interpolate.rs:178-218`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/interpolate.rs#L178-L218)*

Accepts a replacement byte string and interpolates capture references with
their corresponding values.

`append` should be a function that appends the byte string value of a
capture group at a particular index to the byte string given. If the
capture group index is invalid, then nothing should be appended.

`name_to_index` should be a function that maps a capture group name to a
capture group index. If the given name doesn't exist, then `None` should
be returned.

Finally, `dst` is where the final interpolated contents should be written.
If `replacement` contains no capture group references, then `dst` will be
equivalent to `replacement`.

See the [module documentation](self) for details about the format
supported.

# Example

```rust
use regex_automata::util::interpolate;

let mut dst = vec![];
interpolate::bytes(
    b"foo $bar baz",
    |index, dst| {
        if index == 0 {
            dst.extend_from_slice(b"BAR");
        }
    },
    |name| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    },
    &mut dst,
);
assert_eq!(&b"foo BAR baz"[..], dst);
```

### `find_cap_ref`

```rust
fn find_cap_ref(replacement: &[u8]) -> Option<CaptureRef<'_>>
```

*Defined in [`regex-automata-0.4.13/src/util/interpolate.rs:260-290`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/interpolate.rs#L260-L290)*

Parses a possible reference to a capture group name in the given text,
starting at the beginning of `replacement`.

If no such valid reference could be found, None is returned.

Note that this returns a "possible" reference because this routine doesn't
know whether the reference is to a valid group or not. If it winds up not
being a valid reference, then it should be replaced with the empty string.

### `find_cap_ref_braced`

```rust
fn find_cap_ref_braced(rep: &[u8], i: usize) -> Option<CaptureRef<'_>>
```

*Defined in [`regex-automata-0.4.13/src/util/interpolate.rs:295-319`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/interpolate.rs#L295-L319)*

Looks for a braced reference, e.g., `${foo1}`. This assumes that an opening
brace has been found at `i-1` in `rep`. This then looks for a closing
brace and returns the capture reference within the brace.

### `is_valid_cap_letter`

```rust
fn is_valid_cap_letter(b: u8) -> bool
```

*Defined in [`regex-automata-0.4.13/src/util/interpolate.rs:323-325`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/interpolate.rs#L323-L325)*

Returns true if and only if the given byte is allowed in a capture name
written in non-brace form.

