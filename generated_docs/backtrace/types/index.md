*[backtrace](../index.md) / [types](index.md)*

---

# Module `types`

Platform dependent types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BytesOrWideString`](#bytesorwidestring) | enum | A platform independent representation of a string. |

## Enums

### `BytesOrWideString<'a>`

```rust
enum BytesOrWideString<'a> {
    Bytes(&'a [u8]),
    Wide(&'a [u16]),
}
```

*Defined in [`backtrace-0.3.76/src/types.rs:17-22`](../../../.source_1765210505/backtrace-0.3.76/src/types.rs#L17-L22)*

A platform independent representation of a string. When working with `std`
enabled it is recommended to the convenience methods for providing
conversions to `std` types.

#### Variants

- **`Bytes`**

  A slice, typically provided on Unix platforms.

- **`Wide`**

  Wide strings typically from Windows.

#### Implementations

- <span id="bytesorwidestring-to-str-lossy"></span>`fn to_str_lossy(&self) -> Cow<'a, str>`

- <span id="bytesorwidestring-into-path-buf"></span>`fn into_path_buf(self) -> PathBuf`

#### Trait Implementations

##### `impl Debug for BytesOrWideString<'a>`

- <span id="bytesorwidestring-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BytesOrWideString<'a>`

- <span id="bytesorwidestring-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for BytesOrWideString<'a>`

- <span id="bytesorwidestring-to-string"></span>`fn to_string(&self) -> String`

