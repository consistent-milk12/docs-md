*[clap_builder](../../index.md) / [builder](../index.md) / [value_hint](index.md)*

---

# Module `value_hint`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ValueHint`](#valuehint) | enum | Provide shell with hint on how to complete an argument. |

## Enums

### `ValueHint`

```rust
enum ValueHint {
    Unknown,
    Other,
    AnyPath,
    FilePath,
    DirPath,
    ExecutablePath,
    CommandName,
    CommandString,
    CommandWithArguments,
    Username,
    Hostname,
    Url,
    EmailAddress,
}
```

*Defined in [`clap_builder-4.5.53/src/builder/value_hint.rs:29-68`](../../../../.source_1765633015/clap_builder-4.5.53/src/builder/value_hint.rs#L29-L68)*

Provide shell with hint on how to complete an argument.

See `Arg::value_hint` to set this on an argument.

See the `clap_complete` crate for completion script generation.

Overview of which hints are supported by which shell:

| Hint                   | zsh | fish[^1] | dynamic |
| ---------------------- | --- | ---------|---------|
| `AnyPath`              | Yes | Yes      | Yes     |
| `FilePath`             | Yes | Yes      | Yes     |
| `DirPath`              | Yes | Yes      | Yes     |
| `ExecutablePath`       | Yes | Partial  | Yes     |
| `CommandName`          | Yes | Yes      | No      |
| `CommandString`        | Yes | Partial  | No      |
| `CommandWithArguments` | Yes |          | No      |
| `Username`             | Yes | Yes      | No      |
| `Hostname`             | Yes | Yes      | No      |
| `Url`                  | Yes |          | No      |
| `EmailAddress`         | Yes |          | No      |

[^1]: fish completions currently only support named arguments (e.g. -o or --opt), not
      positional arguments.

#### Variants

- **`Unknown`**

  Default value if hint is not specified. Follows shell default behavior, which is usually
  auto-completing filenames.

- **`Other`**

  None of the hints below apply. Disables shell completion for this argument.

- **`AnyPath`**

  Any existing path.

- **`FilePath`**

  Path to a file.

- **`DirPath`**

  Path to a directory.

- **`ExecutablePath`**

  Path to an executable file.

- **`CommandName`**

  Name of a command, without arguments. May be relative to PATH, or full path to executable.

- **`CommandString`**

  A single string containing a command and its arguments.

- **`CommandWithArguments`**

  Capture the remaining arguments as a command name and arguments for that command. This is
  common when writing shell wrappers that execute anther command, for example `sudo` or `env`.
  
  This hint is special, the argument must be a positional argument and have
  `.num_args(1..)` and Command must use `Command::trailing_var_arg(true)`. The result is that the
  command line `my_app ls -la /` will be parsed as `["ls", "-la", "/"]` and clap won't try to
  parse the `-la` argument itself.
  
  

- **`Username`**

  Name of a local operating system user.

- **`Hostname`**

  Host name of a computer.
  Shells usually parse `/etc/hosts` and `.ssh/known_hosts` to complete hostnames.

- **`Url`**

  Complete web address.

- **`EmailAddress`**

  Email address.

#### Trait Implementations

##### `impl Any for ValueHint`

- <span id="valuehint-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ValueHint`

- <span id="valuehint-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ValueHint`

- <span id="valuehint-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ValueHint`

- <span id="valuehint-clone"></span>`fn clone(&self) -> ValueHint` — [`ValueHint`](#valuehint)

##### `impl CloneToUninit for ValueHint`

- <span id="valuehint-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for ValueHint`

##### `impl Debug for ValueHint`

- <span id="valuehint-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ValueHint`

- <span id="valuehint-default"></span>`fn default() -> ValueHint` — [`ValueHint`](#valuehint)

##### `impl Eq for ValueHint`

##### `impl<T> From for ValueHint`

- <span id="valuehint-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for ValueHint`

- <span id="valuehint-fromstr-type-err"></span>`type Err = String`

- <span id="valuehint-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err>`

##### `impl Hash for ValueHint`

- <span id="valuehint-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ValueHint`

- <span id="valuehint-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoResettable for Option<crate::builder::ValueHint>`

- <span id="option-intoresettable-into-resettable"></span>`fn into_resettable(self) -> Resettable<ValueHint>` — [`Resettable`](../resettable/index.md#resettable), [`ValueHint`](#valuehint)

##### `impl PartialEq for ValueHint`

- <span id="valuehint-partialeq-eq"></span>`fn eq(&self, other: &ValueHint) -> bool` — [`ValueHint`](#valuehint)

##### `impl StructuralPartialEq for ValueHint`

##### `impl ToOwned for ValueHint`

- <span id="valuehint-toowned-type-owned"></span>`type Owned = T`

- <span id="valuehint-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="valuehint-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ValueHint`

- <span id="valuehint-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="valuehint-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ValueHint`

- <span id="valuehint-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="valuehint-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

