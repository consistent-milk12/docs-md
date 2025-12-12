*[camino](../index.md) / [serde_impls](index.md)*

---

# Module `serde_impls`

Serde implementations for the types in this crate.

* `Utf8Path` is an unsized type which the derive impls can't handle.
* `Utf8PathBuf` could be derived, but we don't depend on serde_derive to
  improve compile times. It's also very straightforward to implement.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Utf8PathBufVisitor`](#utf8pathbufvisitor) | struct |  |
| [`Utf8PathVisitor`](#utf8pathvisitor) | struct |  |

## Structs

### `Utf8PathBufVisitor`

```rust
struct Utf8PathBufVisitor;
```

*Defined in [`camino-1.2.1/src/serde_impls.rs:14`](../../../.source_1765521767/camino-1.2.1/src/serde_impls.rs#L14)*

#### Trait Implementations

##### `impl Expected for Utf8PathBufVisitor`

- <span id="utf8pathbufvisitor-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for Utf8PathBufVisitor`

- <span id="utf8pathbufvisitor-visitor-type-value"></span>`type Value = Utf8PathBuf`

- <span id="utf8pathbufvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="utf8pathbufvisitor-visit-str"></span>`fn visit_str<E>(self, v: &str) -> Result<<Self as >::Value, E>`

- <span id="utf8pathbufvisitor-visit-string"></span>`fn visit_string<E>(self, v: String) -> Result<<Self as >::Value, E>`

- <span id="utf8pathbufvisitor-visit-bytes"></span>`fn visit_bytes<E>(self, v: &[u8]) -> Result<<Self as >::Value, E>`

- <span id="utf8pathbufvisitor-visit-byte-buf"></span>`fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<<Self as >::Value, E>`

### `Utf8PathVisitor`

```rust
struct Utf8PathVisitor;
```

*Defined in [`camino-1.2.1/src/serde_impls.rs:74`](../../../.source_1765521767/camino-1.2.1/src/serde_impls.rs#L74)*

#### Trait Implementations

##### `impl Expected for Utf8PathVisitor`

- <span id="utf8pathvisitor-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for Utf8PathVisitor`

- <span id="utf8pathvisitor-visitor-type-value"></span>`type Value = &'a Utf8Path`

- <span id="utf8pathvisitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="utf8pathvisitor-visit-borrowed-str"></span>`fn visit_borrowed_str<E>(self, v: &'a str) -> Result<<Self as >::Value, E>`

- <span id="utf8pathvisitor-visit-borrowed-bytes"></span>`fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<<Self as >::Value, E>`

