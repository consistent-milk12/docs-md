*[clap_builder](../../index.md) / [builder](../index.md) / [styled_str](index.md)*

---

# Module `styled_str`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StyledStr`](#styledstr) | struct | Terminal-styling container |

## Structs

### `StyledStr`

```rust
struct StyledStr(String);
```

*Defined in [`clap_builder-4.5.53/src/builder/styled_str.rs:25`](../../../../.source_1765633015/clap_builder-4.5.53/src/builder/styled_str.rs#L25)*

Terminal-styling container

Styling may be encoded as [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code)

# Examples

```rust
use clap_builder as clap;
// `cstr!` converts tags to ANSI codes
let after_help: &'static str = color_print::cstr!(
r#"<bold><underline>Examples</underline></bold>

  <dim>$</dim> <bold>mybin --input file.toml</bold>
"#);

let cmd = clap::Command::new("mybin")
    .after_help(after_help)  // The `&str` gets converted into a `StyledStr`
    // ...
  ;
```

#### Implementations

- <span id="styledstr-new"></span>`const fn new() -> Self`

  Create an empty buffer

- <span id="styledstr-ansi"></span>`fn ansi(&self) -> impl std::fmt::Display + '_`

  Display using [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code) styling

- <span id="styledstr-push-string"></span>`fn push_string(&mut self, msg: String)`

  May allow the compiler to consolidate the `Drop`s for `msg`, reducing code size compared to

  `styled.push_str(&msg)`

- <span id="styledstr-push-str"></span>`fn push_str(&mut self, msg: &str)`

  Appends a given string slice onto the end of this `StyledStr`.

- <span id="styledstr-trim-start-lines"></span>`fn trim_start_lines(&mut self)`

- <span id="styledstr-trim-end"></span>`fn trim_end(&mut self)`

- <span id="styledstr-replace-newline-var"></span>`fn replace_newline_var(&mut self)`

- <span id="styledstr-indent"></span>`fn indent(&mut self, initial: &str, trailing: &str)`

- <span id="styledstr-wrap"></span>`fn wrap(&mut self, _hard_width: usize)`

- <span id="styledstr-display-width"></span>`fn display_width(&self) -> usize`

- <span id="styledstr-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="styledstr-as-styled-str"></span>`fn as_styled_str(&self) -> &str`

- <span id="styledstr-iter-text"></span>`fn iter_text(&self) -> impl Iterator<Item = &str>`

- <span id="styledstr-push-styled"></span>`fn push_styled(&mut self, other: &Self)`

- <span id="styledstr-write-to"></span>`fn write_to(&self, buffer: &mut dyn std::io::Write) -> std::io::Result<()>`

#### Trait Implementations

##### `impl Any for StyledStr`

- <span id="styledstr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StyledStr`

- <span id="styledstr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StyledStr`

- <span id="styledstr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StyledStr`

- <span id="styledstr-clone"></span>`fn clone(&self) -> StyledStr` — [`StyledStr`](#styledstr)

##### `impl CloneToUninit for StyledStr`

- <span id="styledstr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StyledStr`

- <span id="styledstr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StyledStr`

- <span id="styledstr-default"></span>`fn default() -> StyledStr` — [`StyledStr`](#styledstr)

##### `impl Display for StyledStr`

- <span id="styledstr-display-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for StyledStr`

##### `impl<T> From for StyledStr`

- <span id="styledstr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StyledStr`

- <span id="styledstr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for StyledStr`

- <span id="styledstr-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<StyledStr>` — [`Resettable`](../resettable/index.md#resettable), [`StyledStr`](#styledstr)

##### `impl Ord for StyledStr`

- <span id="styledstr-ord-cmp"></span>`fn cmp(&self, other: &StyledStr) -> cmp::Ordering` — [`StyledStr`](#styledstr)

##### `impl PartialEq for StyledStr`

- <span id="styledstr-partialeq-eq"></span>`fn eq(&self, other: &StyledStr) -> bool` — [`StyledStr`](#styledstr)

##### `impl PartialOrd for StyledStr`

- <span id="styledstr-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &StyledStr) -> option::Option<cmp::Ordering>` — [`StyledStr`](#styledstr)

##### `impl StructuralPartialEq for StyledStr`

##### `impl ToOwned for StyledStr`

- <span id="styledstr-toowned-type-owned"></span>`type Owned = T`

- <span id="styledstr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="styledstr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for StyledStr`

- <span id="styledstr-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for StyledStr`

- <span id="styledstr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="styledstr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StyledStr`

- <span id="styledstr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="styledstr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Write for StyledStr`

- <span id="styledstr-write-write-str"></span>`fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error>`

- <span id="styledstr-write-write-char"></span>`fn write_char(&mut self, c: char) -> Result<(), std::fmt::Error>`

