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

## Structs

### `SeekFrom<R: gimli::Reader>`

```rust
struct SeekFrom<R: gimli::Reader> {
}
```

### `RawArgs`

```rust
struct RawArgs {
    items: Vec<std::ffi::OsString>,
}
```

Command-line arguments

#### Implementations

- `fn from_args() -> Self`

- `fn new(iter: impl IntoIterator<Item = impl Into<OsString>>) -> Self`

- `fn cursor(self: &Self) -> ArgCursor` — [`ArgCursor`](../index.md)

- `fn next(self: &Self, cursor: &mut ArgCursor) -> Option<ParsedArg<'_>>` — [`ArgCursor`](../index.md), [`ParsedArg`](../index.md)

- `fn next_os(self: &Self, cursor: &mut ArgCursor) -> Option<&OsStr>` — [`ArgCursor`](../index.md)

- `fn peek(self: &Self, cursor: &ArgCursor) -> Option<ParsedArg<'_>>` — [`ArgCursor`](../index.md), [`ParsedArg`](../index.md)

- `fn peek_os(self: &Self, cursor: &ArgCursor) -> Option<&OsStr>` — [`ArgCursor`](../index.md)

- `fn remaining(self: &Self, cursor: &mut ArgCursor) -> impl Iterator<Item = &OsStr>` — [`ArgCursor`](../index.md)

- `fn seek(self: &Self, cursor: &mut ArgCursor, pos: SeekFrom)` — [`ArgCursor`](../index.md)

- `fn insert(self: &mut Self, cursor: &ArgCursor, insert_items: impl IntoIterator<Item = impl Into<OsString>>)` — [`ArgCursor`](../index.md)

- `fn is_end(self: &Self, cursor: &ArgCursor) -> bool` — [`ArgCursor`](../index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> RawArgs` — [`RawArgs`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> RawArgs` — [`RawArgs`](../index.md)

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &RawArgs) -> bool` — [`RawArgs`](../index.md)

##### `impl StructuralPartialEq`

### `ArgCursor`

```rust
struct ArgCursor {
    cursor: usize,
}
```

Position within [`RawArgs`](#rawargs)

#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ArgCursor` — [`ArgCursor`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl Ord`

- `fn cmp(self: &Self, other: &ArgCursor) -> $crate::cmp::Ordering` — [`ArgCursor`](../index.md)

##### `impl PartialEq`

- `fn eq(self: &Self, other: &ArgCursor) -> bool` — [`ArgCursor`](../index.md)

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &ArgCursor) -> $crate::option::Option<$crate::cmp::Ordering>` — [`ArgCursor`](../index.md)

##### `impl StructuralPartialEq`

### `ParsedArg<'s>`

```rust
struct ParsedArg<'s> {
    inner: &'s std::ffi::OsStr,
}
```

Command-line Argument

#### Implementations

- `fn new(inner: &'s OsStr) -> Self`

- `fn is_empty(self: &Self) -> bool`

- `fn is_stdio(self: &Self) -> bool`

- `fn is_escape(self: &Self) -> bool`

- `fn is_negative_number(self: &Self) -> bool`

- `fn to_long(self: &Self) -> Option<(Result<&str, &OsStr>, Option<&OsStr>)>`

- `fn is_long(self: &Self) -> bool`

- `fn to_short(self: &Self) -> Option<ShortFlags<'_>>` — [`ShortFlags`](../index.md)

- `fn is_short(self: &Self) -> bool`

- `fn to_value_os(self: &Self) -> &OsStr`

- `fn to_value(self: &Self) -> Result<&str, &OsStr>`

- `fn display(self: &Self) -> impl std::fmt::Display + '_`

#### Trait Implementations

##### `impl Clone<'s>`

- `fn clone(self: &Self) -> ParsedArg<'s>` — [`ParsedArg`](../index.md)

##### `impl Debug<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq<'s>`

##### `impl Hash<'s>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<'s>`

- `fn cmp(self: &Self, other: &ParsedArg<'s>) -> $crate::cmp::Ordering` — [`ParsedArg`](../index.md)

##### `impl PartialEq<'s>`

- `fn eq(self: &Self, other: &ParsedArg<'s>) -> bool` — [`ParsedArg`](../index.md)

##### `impl PartialOrd<'s>`

- `fn partial_cmp(self: &Self, other: &ParsedArg<'s>) -> $crate::option::Option<$crate::cmp::Ordering>` — [`ParsedArg`](../index.md)

##### `impl StructuralPartialEq<'s>`

### `ShortFlags<'s>`

```rust
struct ShortFlags<'s> {
    inner: &'s std::ffi::OsStr,
    utf8_prefix: std::str::CharIndices<'s>,
    invalid_suffix: Option<&'s std::ffi::OsStr>,
}
```

Walk through short flags within a [`ParsedArg`](#parsedarg)

#### Implementations

- `fn new(inner: &'s OsStr) -> Self`

- `fn advance_by(self: &mut Self, n: usize) -> Result<(), usize>`

- `fn is_empty(self: &Self) -> bool`

- `fn is_negative_number(self: &Self) -> bool`

- `fn next_flag(self: &mut Self) -> Option<Result<char, &'s OsStr>>`

- `fn next_value_os(self: &mut Self) -> Option<&'s OsStr>`

#### Trait Implementations

##### `impl Clone<'s>`

- `fn clone(self: &Self) -> ShortFlags<'s>` — [`ShortFlags`](../index.md)

##### `impl Debug<'s>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'s>`

- `type Item = Result<char, &'s OsStr>`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

## Traits

