# Crate `clap_lex`

Minimal, flexible command-line parser

As opposed to a declarative parser, this processes arguments as a stream of tokens.  As lexing
a command-line is not context-free, we rely on the caller to decide how to interpret the
arguments.

# Examples

```rust
use std::path::PathBuf;
use std::ffi::OsStr;

type BoxedError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
struct Args {
    paths: Vec<PathBuf>,
    color: Color,
    verbosity: usize,
}

#[derive(Debug)]
enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    fn parse(s: Option<&OsStr>) -> Result<Self, BoxedError> {
        let s = s.map(|s| s.to_str().ok_or(s));
        match s {
            Some(Ok("always")) | Some(Ok("")) | None => {
                Ok(Color::Always)
            }
            Some(Ok("auto")) => {
                Ok(Color::Auto)
            }
            Some(Ok("never")) => {
                Ok(Color::Never)
            }
            Some(invalid) => {
                Err(format!("Invalid value for `--color`, {invalid:?}").into())
            }
        }
    }
}

fn parse_args(
    raw: impl IntoIterator<Item=impl Into<std::ffi::OsString>>
) -> Result<Args, BoxedError> {
    let mut args = Args {
        paths: Vec::new(),
        color: Color::Auto,
        verbosity: 0,
    };

    let raw = clap_lex::RawArgs::new(raw);
    let mut cursor = raw.cursor();
    raw.next(&mut cursor);  // Skip the bin
    while let Some(arg) = raw.next(&mut cursor) {
        if arg.is_escape() {
            args.paths.extend(raw.remaining(&mut cursor).map(PathBuf::from));
        } else if arg.is_stdio() {
            args.paths.push(PathBuf::from("-"));
        } else if let Some((long, value)) = arg.to_long() {
            match long {
                Ok("verbose") => {
                    if let Some(value) = value {
                        return Err(format!("`--verbose` does not take a value, got `{value:?}`").into());
                    }
                    args.verbosity += 1;
                }
                Ok("color") => {
                    args.color = Color::parse(value)?;
                }
                _ => {
                    return Err(
                        format!("Unexpected flag: --{}", arg.display()).into()
                    );
                }
            }
        } else if let Some(mut shorts) = arg.to_short() {
            while let Some(short) = shorts.next_flag() {
                match short {
                    Ok('v') => {
                        args.verbosity += 1;
                    }
                    Ok('c') => {
                        let value = shorts.next_value_os();
                        args.color = Color::parse(value)?;
                    }
                    Ok(c) => {
                        return Err(format!("Unexpected flag: -{c}").into());
                    }
                    Err(e) => {
                        return Err(format!("Unexpected flag: -{}", e.to_string_lossy()).into());
                    }
                }
            }
        } else {
            args.paths.push(PathBuf::from(arg.to_value_os().to_owned()));
        }
    }

    Ok(args)
}

let args = parse_args(["bin", "--hello", "world"]);
println!("{args:?}");
```

## Contents

- [Modules](#modules)
  - [`ext`](#ext)
- [Structs](#structs)
  - [`SeekFrom`](#seekfrom)
  - [`RawArgs`](#rawargs)
  - [`ArgCursor`](#argcursor)
  - [`ParsedArg`](#parsedarg)
  - [`ShortFlags`](#shortflags)
- [Traits](#traits)
  - [`OsStrExt`](#osstrext)
- [Functions](#functions)
  - [`split_nonutf8_once`](#split-nonutf8-once)
  - [`is_number`](#is-number)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ext`](#ext) | mod |  |
| [`SeekFrom`](#seekfrom) | struct |  |
| [`RawArgs`](#rawargs) | struct | Command-line arguments |
| [`ArgCursor`](#argcursor) | struct | Position within [`RawArgs`] |
| [`ParsedArg`](#parsedarg) | struct | Command-line Argument |
| [`ShortFlags`](#shortflags) | struct | Walk through short flags within a [`ParsedArg`] |
| [`OsStrExt`](#osstrext) | trait |  |
| [`split_nonutf8_once`](#split-nonutf8-once) | fn |  |
| [`is_number`](#is-number) | fn |  |

## Modules

- [`ext`](ext/index.md)

## Structs

### `SeekFrom<R: gimli::Reader>`

```rust
struct SeekFrom<R: gimli::Reader> {
    dw_die_offset: gimli::UnitOffset<<R as >::Offset>,
    name: Option<R>,
    inlined_functions: alloc::boxed::Box<[InlinedFunction<R>]>,
    inlined_addresses: alloc::boxed::Box<[InlinedFunctionAddress]>,
}
```

*Defined in [`addr2line-0.25.1/src/function.rs:67-74`](../../.source_1765900590/addr2line-0.25.1/src/function.rs#L67-L74)*

*Re-exported from `addr2line`*

#### Fields

- **`inlined_functions`**: `alloc::boxed::Box<[InlinedFunction<R>]>`

  List of all `DW_TAG_inlined_subroutine` details in this function.

- **`inlined_addresses`**: `alloc::boxed::Box<[InlinedFunctionAddress]>`

  List of `DW_TAG_inlined_subroutine` address ranges in this function.

#### Implementations

- <span id="function-parse"></span>`fn parse(dw_die_offset: gimli::UnitOffset<<R as >::Offset>, file: DebugFile, unit: gimli::UnitRef<'_, R>, ctx: &Context<R>) -> Result<Self, gimli::Error>`

- <span id="function-parse-children"></span>`fn parse_children(state: &mut InlinedState<'_, R>, depth: isize, inlined_depth: usize) -> Result<(), gimli::Error>`

- <span id="function-skip"></span>`fn skip(entries: &mut gimli::EntriesRaw<'_, '_, R>, abbrev: &gimli::Abbreviation, depth: isize) -> Result<(), gimli::Error>`

- <span id="function-find-inlined-functions"></span>`fn find_inlined_functions(&self, probe: u64) -> alloc::vec::Vec<&InlinedFunction<R>>` — [`OsStrExt`](#osstrext)

  Build the list of inlined functions that contain `probe`.

#### Trait Implementations

##### `impl Any for Function<R>`

- <span id="function-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Function<R>`

- <span id="function-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Function<R>`

- <span id="function-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for Function<R>`

- <span id="function-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Function<R>`

- <span id="function-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Function<R>`

- <span id="function-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="function-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Function<R>`

- <span id="function-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="function-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RawArgs`

```rust
struct RawArgs {
    items: Vec<std::ffi::OsString>,
}
```

*Defined in [`clap_lex-0.7.6/src/lib.rs:129-131`](../../.source_1765900590/clap_lex-0.7.6/src/lib.rs#L129-L131)*

Command-line arguments

#### Implementations

- <span id="rawargs-from-args"></span>`fn from_args() -> Self`

  
  <div class="warning">
  
  **NOTE:** The argument returned will be the current binary.
  
  </div>
  
  ##### Example
  
  ```rust,no_run
  use std::path::PathBuf;
  let raw = clap_lex::RawArgs::from_args();
  let mut cursor = raw.cursor();
  let _bin = raw.next_os(&mut cursor);
  
  let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();
  println!("{paths:?}");
  ```

- <span id="rawargs-new"></span>`fn new(iter: impl IntoIterator<Item = impl Into<OsString>>) -> Self`

  
  ##### Example
  
  ```rust,no_run
  use std::path::PathBuf;
  let raw = clap_lex::RawArgs::new(["bin", "foo.txt"]);
  let mut cursor = raw.cursor();
  let _bin = raw.next_os(&mut cursor);
  
  let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();
  println!("{paths:?}");
  ```

- <span id="rawargs-cursor"></span>`fn cursor(&self) -> ArgCursor` — [`ArgCursor`](#argcursor)

  Create a cursor for walking the arguments
  
  ##### Example
  
  ```rust,no_run
  use std::path::PathBuf;
  let raw = clap_lex::RawArgs::new(["bin", "foo.txt"]);
  let mut cursor = raw.cursor();
  let _bin = raw.next_os(&mut cursor);
  
  let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();
  println!("{paths:?}");
  ```

- <span id="rawargs-next"></span>`fn next(&self, cursor: &mut ArgCursor) -> Option<ParsedArg<'_>>` — [`ArgCursor`](#argcursor), [`ParsedArg`](#parsedarg)

  Advance the cursor, returning the next [`ParsedArg`](#parsedarg)

- <span id="rawargs-next-os"></span>`fn next_os(&self, cursor: &mut ArgCursor) -> Option<&OsStr>` — [`ArgCursor`](#argcursor)

  Advance the cursor, returning a raw argument value.

- <span id="rawargs-peek"></span>`fn peek(&self, cursor: &ArgCursor) -> Option<ParsedArg<'_>>` — [`ArgCursor`](#argcursor), [`ParsedArg`](#parsedarg)

  Return the next [`ParsedArg`](#parsedarg)

- <span id="rawargs-peek-os"></span>`fn peek_os(&self, cursor: &ArgCursor) -> Option<&OsStr>` — [`ArgCursor`](#argcursor)

  Return a raw argument value.

- <span id="rawargs-remaining"></span>`fn remaining(&self, cursor: &mut ArgCursor) -> impl Iterator<Item = &OsStr>` — [`ArgCursor`](#argcursor)

  Return all remaining raw arguments, advancing the cursor to the end
  
  ##### Example
  
  ```rust,no_run
  use std::path::PathBuf;
  let raw = clap_lex::RawArgs::new(["bin", "foo.txt"]);
  let mut cursor = raw.cursor();
  let _bin = raw.next_os(&mut cursor);
  
  let mut paths = raw.remaining(&mut cursor).map(PathBuf::from).collect::<Vec<_>>();
  println!("{paths:?}");
  ```

- <span id="rawargs-seek"></span>`fn seek(&self, cursor: &mut ArgCursor, pos: SeekFrom)` — [`ArgCursor`](#argcursor), [`SeekFrom`](#seekfrom)

  Adjust the cursor's position

- <span id="rawargs-insert"></span>`fn insert(&mut self, cursor: &ArgCursor, insert_items: impl IntoIterator<Item = impl Into<OsString>>)` — [`ArgCursor`](#argcursor)

  Inject arguments before the `RawArgs::next`

- <span id="rawargs-is-end"></span>`fn is_end(&self, cursor: &ArgCursor) -> bool` — [`ArgCursor`](#argcursor)

  Any remaining args?

#### Trait Implementations

##### `impl Any for RawArgs`

- <span id="rawargs-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RawArgs`

- <span id="rawargs-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RawArgs`

- <span id="rawargs-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RawArgs`

- <span id="rawargs-clone"></span>`fn clone(&self) -> RawArgs` — [`RawArgs`](#rawargs)

##### `impl CloneToUninit for RawArgs`

- <span id="rawargs-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RawArgs`

- <span id="rawargs-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RawArgs`

- <span id="rawargs-default"></span>`fn default() -> RawArgs` — [`RawArgs`](#rawargs)

##### `impl Eq for RawArgs`

##### `impl<T> From for RawArgs`

- <span id="rawargs-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RawArgs`

- <span id="rawargs-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for RawArgs`

- <span id="rawargs-partialeq-eq"></span>`fn eq(&self, other: &RawArgs) -> bool` — [`RawArgs`](#rawargs)

##### `impl StructuralPartialEq for RawArgs`

##### `impl ToOwned for RawArgs`

- <span id="rawargs-toowned-type-owned"></span>`type Owned = T`

- <span id="rawargs-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rawargs-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RawArgs`

- <span id="rawargs-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rawargs-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RawArgs`

- <span id="rawargs-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rawargs-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArgCursor`

```rust
struct ArgCursor {
    cursor: usize,
}
```

*Defined in [`clap_lex-0.7.6/src/lib.rs:276-278`](../../.source_1765900590/clap_lex-0.7.6/src/lib.rs#L276-L278)*

Position within [`RawArgs`](#rawargs)

#### Implementations

- <span id="argcursor-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Any for ArgCursor`

- <span id="argcursor-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArgCursor`

- <span id="argcursor-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArgCursor`

- <span id="argcursor-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ArgCursor`

- <span id="argcursor-clone"></span>`fn clone(&self) -> ArgCursor` — [`ArgCursor`](#argcursor)

##### `impl CloneToUninit for ArgCursor`

- <span id="argcursor-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ArgCursor`

- <span id="argcursor-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ArgCursor`

##### `impl<T> From for ArgCursor`

- <span id="argcursor-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ArgCursor`

- <span id="argcursor-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for ArgCursor`

- <span id="argcursor-ord-cmp"></span>`fn cmp(&self, other: &ArgCursor) -> cmp::Ordering` — [`ArgCursor`](#argcursor)

##### `impl PartialEq for ArgCursor`

- <span id="argcursor-partialeq-eq"></span>`fn eq(&self, other: &ArgCursor) -> bool` — [`ArgCursor`](#argcursor)

##### `impl PartialOrd for ArgCursor`

- <span id="argcursor-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ArgCursor) -> option::Option<cmp::Ordering>` — [`ArgCursor`](#argcursor)

##### `impl StructuralPartialEq for ArgCursor`

##### `impl ToOwned for ArgCursor`

- <span id="argcursor-toowned-type-owned"></span>`type Owned = T`

- <span id="argcursor-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="argcursor-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ArgCursor`

- <span id="argcursor-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="argcursor-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArgCursor`

- <span id="argcursor-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="argcursor-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ParsedArg<'s>`

```rust
struct ParsedArg<'s> {
    inner: &'s std::ffi::OsStr,
}
```

*Defined in [`clap_lex-0.7.6/src/lib.rs:288-290`](../../.source_1765900590/clap_lex-0.7.6/src/lib.rs#L288-L290)*

Command-line Argument

#### Implementations

- <span id="parsedarg-new"></span>`fn new(inner: &'s OsStr) -> Self`

- <span id="parsedarg-is-empty"></span>`fn is_empty(&self) -> bool`

  Argument is length of 0

- <span id="parsedarg-is-stdio"></span>`fn is_stdio(&self) -> bool`

  Does the argument look like a stdio argument (`-`)

- <span id="parsedarg-is-escape"></span>`fn is_escape(&self) -> bool`

  Does the argument look like an argument escape (`--`)

- <span id="parsedarg-is-negative-number"></span>`fn is_negative_number(&self) -> bool`

  Does the argument look like a negative number?
  
  This won't parse the number in full but attempts to see if this looks
  like something along the lines of `-3`, `-0.3`, or `-33.03`

- <span id="parsedarg-to-long"></span>`fn to_long(&self) -> Option<(Result<&str, &OsStr>, Option<&OsStr>)>`

  Treat as a long-flag

- <span id="parsedarg-is-long"></span>`fn is_long(&self) -> bool`

  Can treat as a long-flag

- <span id="parsedarg-to-short"></span>`fn to_short(&self) -> Option<ShortFlags<'_>>` — [`ShortFlags`](#shortflags)

  Treat as a short-flag

- <span id="parsedarg-is-short"></span>`fn is_short(&self) -> bool`

  Can treat as a short-flag

- <span id="parsedarg-to-value-os"></span>`fn to_value_os(&self) -> &OsStr`

  Treat as a value
  
  <div class="warning">
  
  **NOTE:** May return a flag or an escape.
  
  </div>

- <span id="parsedarg-to-value"></span>`fn to_value(&self) -> Result<&str, &OsStr>`

  Treat as a value
  
  <div class="warning">
  
  **NOTE:** May return a flag or an escape.
  
  </div>

- <span id="parsedarg-display"></span>`fn display(&self) -> impl std::fmt::Display + '_`

  Safely print an argument that may contain non-UTF8 content
  
  This may perform lossy conversion, depending on the platform. If you would like an implementation which escapes the path please use Debug instead.

#### Trait Implementations

##### `impl Any for ParsedArg<'s>`

- <span id="parsedarg-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParsedArg<'s>`

- <span id="parsedarg-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParsedArg<'s>`

- <span id="parsedarg-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ParsedArg<'s>`

- <span id="parsedarg-clone"></span>`fn clone(&self) -> ParsedArg<'s>` — [`ParsedArg`](#parsedarg)

##### `impl CloneToUninit for ParsedArg<'s>`

- <span id="parsedarg-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ParsedArg<'s>`

- <span id="parsedarg-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParsedArg<'s>`

##### `impl<T> From for ParsedArg<'s>`

- <span id="parsedarg-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for ParsedArg<'s>`

- <span id="parsedarg-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for ParsedArg<'s>`

- <span id="parsedarg-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for ParsedArg<'s>`

- <span id="parsedarg-ord-cmp"></span>`fn cmp(&self, other: &ParsedArg<'s>) -> cmp::Ordering` — [`ParsedArg`](#parsedarg)

##### `impl PartialEq for ParsedArg<'s>`

- <span id="parsedarg-partialeq-eq"></span>`fn eq(&self, other: &ParsedArg<'s>) -> bool` — [`ParsedArg`](#parsedarg)

##### `impl PartialOrd for ParsedArg<'s>`

- <span id="parsedarg-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ParsedArg<'s>) -> option::Option<cmp::Ordering>` — [`ParsedArg`](#parsedarg)

##### `impl StructuralPartialEq for ParsedArg<'s>`

##### `impl ToOwned for ParsedArg<'s>`

- <span id="parsedarg-toowned-type-owned"></span>`type Owned = T`

- <span id="parsedarg-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="parsedarg-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ParsedArg<'s>`

- <span id="parsedarg-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parsedarg-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParsedArg<'s>`

- <span id="parsedarg-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parsedarg-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ShortFlags<'s>`

```rust
struct ShortFlags<'s> {
    inner: &'s std::ffi::OsStr,
    utf8_prefix: std::str::CharIndices<'s>,
    invalid_suffix: Option<&'s std::ffi::OsStr>,
}
```

*Defined in [`clap_lex-0.7.6/src/lib.rs:399-403`](../../.source_1765900590/clap_lex-0.7.6/src/lib.rs#L399-L403)*

Walk through short flags within a [`ParsedArg`](#parsedarg)

#### Implementations

- <span id="shortflags-new"></span>`fn new(inner: &'s OsStr) -> Self`

- <span id="shortflags-advance-by"></span>`fn advance_by(&mut self, n: usize) -> Result<(), usize>`

  Move the iterator forward by `n` short flags

- <span id="shortflags-is-empty"></span>`fn is_empty(&self) -> bool`

  No short flags left

- <span id="shortflags-is-negative-number"></span>`fn is_negative_number(&self) -> bool`

  Does the short flag look like a number
  
  Ideally call this before doing any iterator

- <span id="shortflags-next-flag"></span>`fn next_flag(&mut self) -> Option<Result<char, &'s OsStr>>`

  Advance the iterator, returning the next short flag on success
  
  On error, returns the invalid-UTF8 value

- <span id="shortflags-next-value-os"></span>`fn next_value_os(&mut self) -> Option<&'s OsStr>`

  Advance the iterator, returning everything left as a value

#### Trait Implementations

##### `impl Any for ShortFlags<'s>`

- <span id="shortflags-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ShortFlags<'s>`

- <span id="shortflags-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ShortFlags<'s>`

- <span id="shortflags-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ShortFlags<'s>`

- <span id="shortflags-clone"></span>`fn clone(&self) -> ShortFlags<'s>` — [`ShortFlags`](#shortflags)

##### `impl CloneToUninit for ShortFlags<'s>`

- <span id="shortflags-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ShortFlags<'s>`

- <span id="shortflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ShortFlags<'s>`

- <span id="shortflags-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ShortFlags<'s>`

- <span id="shortflags-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.
  
  That is, this conversion is whatever the implementation of
  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ShortFlags<'s>`

- <span id="shortflags-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="shortflags-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="shortflags-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ShortFlags<'s>`

- <span id="shortflags-iterator-type-item"></span>`type Item = Result<char, &'s OsStr>`

- <span id="shortflags-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl ToOwned for ShortFlags<'s>`

- <span id="shortflags-toowned-type-owned"></span>`type Owned = T`

- <span id="shortflags-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="shortflags-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ShortFlags<'s>`

- <span id="shortflags-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="shortflags-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ShortFlags<'s>`

- <span id="shortflags-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="shortflags-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `OsStrExt`

```rust
trait OsStrExt: private::Sealed { ... }
```

*Defined in [`clap_lex-0.7.6/src/ext.rs:4-183`](../../.source_1765900590/clap_lex-0.7.6/src/ext.rs#L4-L183)*

String-like methods for [`OsStr`](#osstr)

#### Required Methods

- `fn OsStrExt::try_str(&self) -> Result<&str, std::str::Utf8Error>`

  Converts to a string slice.
  
  The `Utf8Error` is guaranteed to have a valid UTF8 boundary
  in its `valid_up_to()`

- `fn OsStrExt::contains(&self, needle: &str) -> bool`

  Returns `true` if the given pattern matches a sub-slice of
  this string slice.
  
  Returns `false` if it does not.
  
  ##### Examples
  
  ```rust
  use clap_lex::OsStrExt as _;
  let bananas = std::ffi::OsStr::new("bananas");
  
  assert!(bananas.contains("nana"));
  assert!(!bananas.contains("apples"));
  ```

- `fn OsStrExt::find(&self, needle: &str) -> Option<usize>`

  Returns the byte index of the first character of this string slice that
  matches the pattern.
  
  Returns [`None`](#none) if the pattern doesn't match.
  
  ##### Examples
  
  ```rust
  use clap_lex::OsStrExt as _;
  let s = std::ffi::OsStr::new("Löwe 老虎 Léopard Gepardi");
  
  assert_eq!(s.find("L"), Some(0));
  assert_eq!(s.find("é"), Some(14));
  assert_eq!(s.find("par"), Some(17));
  ```
  
  Not finding the pattern:
  
  ```rust
  use clap_lex::OsStrExt as _;
  let s = std::ffi::OsStr::new("Löwe 老虎 Léopard");
  
  assert_eq!(s.find("1"), None);
  ```

- `fn OsStrExt::strip_prefix(&self, prefix: &str) -> Option<&OsStr>`

  Returns a string slice with the prefix removed.
  
  If the string starts with the pattern `prefix`, returns substring after the prefix, wrapped
  in `Some`.
  
  If the string does not start with `prefix`, returns `None`.
  
  ##### Examples
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  assert_eq!(OsStr::new("foo:bar").strip_prefix("foo:"), Some(OsStr::new("bar")));
  assert_eq!(OsStr::new("foo:bar").strip_prefix("bar"), None);
  assert_eq!(OsStr::new("foofoo").strip_prefix("foo"), Some(OsStr::new("foo")));
  ```

- `fn OsStrExt::starts_with(&self, prefix: &str) -> bool`

  Returns `true` if the given pattern matches a prefix of this
  string slice.
  
  Returns `false` if it does not.
  
  ##### Examples
  
  ```rust
  use clap_lex::OsStrExt as _;
  let bananas = std::ffi::OsStr::new("bananas");
  
  assert!(bananas.starts_with("bana"));
  assert!(!bananas.starts_with("nana"));
  ```

- `fn OsStrExt::split<'s, 'n>(self: &'s Self, needle: &'n str) -> Split<'s, 'n>`

  An iterator over substrings of this string slice, separated by
  characters matched by a pattern.
  
  ##### Examples
  
  Simple patterns:
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let v: Vec<_> = OsStr::new("Mary had a little lamb").split(" ").collect();
  assert_eq!(v, [OsStr::new("Mary"), OsStr::new("had"), OsStr::new("a"), OsStr::new("little"), OsStr::new("lamb")]);
  
  let v: Vec<_> = OsStr::new("").split("X").collect();
  assert_eq!(v, [OsStr::new("")]);
  
  let v: Vec<_> = OsStr::new("lionXXtigerXleopard").split("X").collect();
  assert_eq!(v, [OsStr::new("lion"), OsStr::new(""), OsStr::new("tiger"), OsStr::new("leopard")]);
  
  let v: Vec<_> = OsStr::new("lion::tiger::leopard").split("::").collect();
  assert_eq!(v, [OsStr::new("lion"), OsStr::new("tiger"), OsStr::new("leopard")]);
  ```
  
  If a string contains multiple contiguous separators, you will end up
  with empty strings in the output:
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let x = OsStr::new("||||a||b|c");
  let d: Vec<_> = x.split("|").collect();
  
  assert_eq!(d, &[OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new("a"), OsStr::new(""), OsStr::new("b"), OsStr::new("c")]);
  ```
  
  Contiguous separators are separated by the empty string.
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let x = OsStr::new("(///)");
  let d: Vec<_> = x.split("/").collect();
  
  assert_eq!(d, &[OsStr::new("("), OsStr::new(""), OsStr::new(""), OsStr::new(")")]);
  ```
  
  Separators at the start or end of a string are neighbored
  by empty strings.
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let d: Vec<_> = OsStr::new("010").split("0").collect();
  assert_eq!(d, &[OsStr::new(""), OsStr::new("1"), OsStr::new("")]);
  ```
  
  When the empty string is used as a separator, it panics
  
  ```should_panic
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let f: Vec<_> = OsStr::new("rust").split("").collect();
  assert_eq!(f, &[OsStr::new(""), OsStr::new("r"), OsStr::new("u"), OsStr::new("s"), OsStr::new("t"), OsStr::new("")]);
  ```
  
  Contiguous separators can lead to possibly surprising behavior
  when whitespace is used as the separator. This code is correct:
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  let x = OsStr::new("    a  b c");
  let d: Vec<_> = x.split(" ").collect();
  
  assert_eq!(d, &[OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new(""), OsStr::new("a"), OsStr::new(""), OsStr::new("b"), OsStr::new("c")]);
  ```
  
  It does _not_ give you:
  
  ```,ignore
  assert_eq!(d, &[OsStr::new("a"), OsStr::new("b"), OsStr::new("c")]);
  ```
  
  Use `split_whitespace` for this behavior.

- `fn OsStrExt::split_once(&self, needle: &str) -> Option<(&OsStr, &OsStr)>`

  Splits the string on the first occurrence of the specified delimiter and
  returns prefix before delimiter and suffix after delimiter.
  
  ##### Examples
  
  ```rust
  use std::ffi::OsStr;
  use clap_lex::OsStrExt as _;
  assert_eq!(OsStr::new("cfg").split_once("="), None);
  assert_eq!(OsStr::new("cfg=").split_once("="), Some((OsStr::new("cfg"), OsStr::new(""))));
  assert_eq!(OsStr::new("cfg=foo").split_once("="), Some((OsStr::new("cfg"), OsStr::new("foo"))));
  assert_eq!(OsStr::new("cfg=foo=bar").split_once("="), Some((OsStr::new("cfg"), OsStr::new("foo=bar"))));
  ```

#### Implementors

- `std::ffi::OsStr`

## Functions

### `split_nonutf8_once`

```rust
fn split_nonutf8_once(b: &std::ffi::OsStr) -> (&str, Option<&std::ffi::OsStr>)
```

*Defined in [`clap_lex-0.7.6/src/lib.rs:479-490`](../../.source_1765900590/clap_lex-0.7.6/src/lib.rs#L479-L490)*

### `is_number`

```rust
fn is_number(arg: &str) -> bool
```

*Defined in [`clap_lex-0.7.6/src/lib.rs:492-522`](../../.source_1765900590/clap_lex-0.7.6/src/lib.rs#L492-L522)*

