*[backtrace](../index.md) / [types](index.md)*

---

# Module `types`

Platform dependent types.

## Enums

### `BytesOrWideString<'a>`

```rust
enum BytesOrWideString<'a> {
    Bytes(&'a [u8]),
    Wide(&'a [u16]),
}
```

A platform independent representation of a string. When working with `std`
enabled it is recommended to the convenience methods for providing
conversions to `std` types.

#### Variants

- **`Bytes`**

  A slice, typically provided on Unix platforms.

- **`Wide`**

  Wide strings typically from Windows.

#### Implementations

- `fn to_str_lossy(self: &Self) -> Cow<'a, str>`

- `fn into_path_buf(self: Self) -> PathBuf`

#### Trait Implementations

##### `impl<'a> Debug for BytesOrWideString<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> Display for BytesOrWideString<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for BytesOrWideString<'a>`

- `fn to_string(self: &Self) -> String`

