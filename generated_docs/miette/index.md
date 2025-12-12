# Crate `miette`

You run miette? You run her code like the software? Oh. Oh! Error code for
coder! Error code for One Thousand Lines!

## About

`miette` is a diagnostic library for Rust. It includes a series of
traits/protocols that allow you to hook into its error reporting facilities,
and even write your own error reports! It lets you define error types that
can print out like this (or in any format you like!):

<img src="https://raw.githubusercontent.com/zkat/miette/main/images/serde_json.png" alt="Hi! miette also includes a screen-reader-oriented diagnostic printer that's enabled in various situations, such as when you use NO_COLOR or CLICOLOR settings, or on CI. This behavior is also fully configurable and customizable. For example, this is what this particular diagnostic will look like when the narrated printer is enabled:
\
Error: Received some bad JSON from the source. Unable to parse.
    Caused by: missing field `foo` at line 1 column 1700
\
Begin snippet for https://api.nuget.org/v3/registration5-gz-semver2/json.net/index.json starting
at line 1, column 1659
\
snippet line 1: gs&quot;:[&quot;json&quot;],&quot;title&quot;:&quot;&quot;,&quot;version&quot;:&quot;1.0.0&quot;},&quot;packageContent&quot;:&quot;https://api.nuget.o
    highlight starting at line 1, column 1699: last parsing location
\
diagnostic help: This is a bug. It might be in ruget, or it might be in the
source you're using, but it's definitely a bug and should be reported.
diagnostic error code: ruget::api::bad_json
" />

> **NOTE: You must enable the `"fancy"` crate feature to get fancy report
> output like in the screenshots above.** You should only do this in your
> toplevel crate, as the fancy feature pulls in a number of dependencies that
> libraries and such might not want.

## Table of Contents <!-- omit in toc -->

- [About](#about)
- [Features](#features)
- [Installing](#installing)
- [Example](#example)
- [Using](#using)
  - [... in libraries](#-in-libraries)
  - [... in application code](#-in-application-code)
  - [... in `main()`](#-in-main)
  - [... diagnostic code URLs](#-diagnostic-code-urls)
  - [... snippets](#-snippets)
  - [... help text](#-help-text)
  - [... severity level](#-severity-level)
  - [... multiple related errors](#-multiple-related-errors)
  - [... delayed source code](#-delayed-source-code)
  - [... handler options](#-handler-options)
  - [... dynamic diagnostics](#-dynamic-diagnostics)
  - [... syntax highlighting](#-syntax-highlighting)
  - [... primary label](#-primary-label)
  - [... collection of labels](#-collection-of-labels)
- [Acknowledgements](#acknowledgements)
- [License](#license)

## Features

- Generic [`Diagnostic`](#diagnostic) protocol, compatible (and dependent on)
  [`std::error::Error`](../cargo_docs_md/error/index.md).
- Unique error codes on every [`Diagnostic`](#diagnostic).
- Custom links to get more details on error codes.
- Super handy derive macro for defining diagnostic metadata.
- Replacements for [`anyhow`](https://docs.rs/anyhow)/[`eyre`](https://docs.rs/eyre)
  types [`Result`](#result), [`Report`](#report) and the [`miette!`](#miette) macro for the
  `anyhow!`/`eyre!` macros.
- Generic support for arbitrary [`SourceCode`](#sourcecode)s for snippet data, with
  default support for `String`s included.

The `miette` crate also comes bundled with a default [`ReportHandler`](#reporthandler) with
the following features:

- Fancy graphical [diagnostic output](#about), using ANSI/Unicode text
- single- and multi-line highlighting support
- Screen reader/braille support, gated on [`NO_COLOR`](http://no-color.org/),
  and other heuristics.
- Fully customizable graphical theming (or overriding the printers
  entirely).
- Cause chain printing
- Turns diagnostic codes into links in [supported terminals](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda).

## Installing

```sh
$ cargo add miette
```

If you want to use the fancy printer in all these screenshots:

```sh
$ cargo add miette --features fancy
```

## Example

```rust
/*
You can derive a `Diagnostic` from any `std::error::Error` type.

`thiserror` is a great way to define them, and plays nicely with `miette`!
*/
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("oops!")]
#[diagnostic(
    code(oops::my::bad),
    url(docsrs),
    help("try doing it better next time?")
)]
struct MyBad {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: NamedSource<String>,
    // Snippets and highlights can be included in the diagnostic!
    #[label("This bit here")]
    bad_bit: SourceSpan,
}

/*
Now let's define a function!

Use this `Result` type (or its expanded version) as the return type
throughout your app (but NOT your libraries! Those should always return
concrete types!).
*/
use miette::Result;
fn this_fails() -> Result<()> {
    // You can use plain strings as a `Source`, or anything that implements
    // the one-method `Source` trait.
    let src = "source\n  text\n    here".to_string();

    Err(MyBad {
        src: NamedSource::new("bad_file.rs", src),
        bad_bit: (9, 4).into(),
    })?;

    Ok(())
}

/*
Now to get everything printed nicely, just return a `Result<()>`
and you're all set!

Note: You can swap out the default reporter for a custom one using
`miette::set_hook()`
*/
fn pretend_this_is_main() -> Result<()> {
    // kaboom~
    this_fails()?;

    Ok(())
}
```

And this is the output you'll get if you run this program:

<img src="https://raw.githubusercontent.com/zkat/miette/main/images/single-line-example.png" alt="
Narratable printout:
\
diagnostic error code: oops::my::bad (link)
Error: oops!
\
Begin snippet for bad_file.rs starting
at line 2, column 3
\
snippet line 1: source
\
snippet line 2:  text
    highlight starting at line 1, column 3: This bit here
\
snippet line 3: here
\
diagnostic help: try doing it better next time?">

## Using

### ... in libraries

`miette` is _fully compatible_ with library usage. Consumers who don't know
about, or don't want, `miette` features can safely use its error types as
regular [`std::error::Error`](../cargo_docs_md/error/index.md).

We highly recommend using something like [`thiserror`](https://docs.rs/thiserror)
to define unique error types and error wrappers for your library.

While `miette` integrates smoothly with `thiserror`, it is _not required_.
If you don't want to use the [`Diagnostic`](#diagnostic) derive macro, you can implement
the trait directly, just like with `std::error::Error`.

```rust
// lib/error.rs
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum MyLibError {
    #[error(transparent)]
    #[diagnostic(code(my_lib::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Oops it blew up")]
    #[diagnostic(code(my_lib::bad_code))]
    BadThingHappened,

    #[error(transparent)]
    // Use `#[diagnostic(transparent)]` to wrap another [`Diagnostic`](#diagnostic). You won't see labels otherwise
    #[diagnostic(transparent)]
    AnotherError(#[from] AnotherError),
}

#[derive(Error, Diagnostic, Debug)]
#[error("another error")]
pub struct AnotherError {
   #[label("here")]
   pub at: SourceSpan
}
```

Then, return this error type from all your fallible public APIs. It's a best
practice to wrap any "external" error types in your error `enum` instead of
using something like [`Report`](#report) in a library.

### ... in application code

Application code tends to work a little differently than libraries. You
don't always need or care to define dedicated error wrappers for errors
coming from external libraries and tools.

For this situation, `miette` includes two tools: [`Report`](#report) and
[`IntoDiagnostic`](eyreish/index.md). They work in tandem to make it easy to convert regular
`std::error::Error`s into [`Diagnostic`](#diagnostic)s. Additionally, there's a
[`Result`](#result) type alias that you can use to be more terse.

When dealing with non-`Diagnostic` types, you'll want to
`.into_diagnostic()` them:

```rust
// my_app/lib/my_internal_file.rs
use miette::{IntoDiagnostic, Result};
use semver::Version;

pub fn some_tool() -> Result<Version> {
    "1.2.x".parse().into_diagnostic()
}
```

`miette` also includes an `anyhow`/`eyre`-style `Context`/`WrapErr` traits
that you can import to add ad-hoc context messages to your `Diagnostic`s, as
well, though you'll still need to use `.into_diagnostic()` to make use of
it:

```rust
// my_app/lib/my_internal_file.rs
use miette::{IntoDiagnostic, Result, WrapErr};
use semver::Version;

pub fn some_tool() -> Result<Version> {
    "1.2.x"
        .parse()
        .into_diagnostic()
        .wrap_err("Parsing this tool's semver version failed.")
}
```

To construct your own simple adhoc error use the [`miette!`](#miette) macro:
```rust
// my_app/lib/my_internal_file.rs
use miette::{miette, Result};
use semver::Version;

pub fn some_tool() -> Result<Version> {
    let version = "1.2.x";
    version
        .parse()
        .map_err(|_| miette!("Invalid version {}", version))
}
```
There are also similar [bail!] and [ensure!] macros.

### ... in `main()`

`main()` is just like any other part of your application-internal code. Use
`Result` as your return value, and it will pretty-print your diagnostics
automatically.

> **NOTE:** You must enable the `"fancy"` crate feature to get fancy report
> output like in the screenshots here.** You should only do this in your
> toplevel crate, as the fancy feature pulls in a number of dependencies that
> libraries and such might not want.

```rust
use miette::{IntoDiagnostic, Result};
use semver::Version;

fn pretend_this_is_main() -> Result<()> {
    let version: Version = "1.2.x".parse().into_diagnostic()?;
    println!("{}", version);
    Ok(())
}
```

Please note: in order to get fancy diagnostic rendering with all the pretty
colors and arrows, you should install `miette` with the `fancy` feature
enabled:

```toml
miette = { version = "X.Y.Z", features = ["fancy"] }
```

Another way to display a diagnostic is by printing them using the debug formatter.
This is, in fact, what returning diagnostics from main ends up doing.
To do it yourself, you can write the following:

```rust
use miette::{IntoDiagnostic, Result};
use semver::Version;

fn just_a_random_function() {
    let version_result: Result<Version> = "1.2.x".parse().into_diagnostic();
    match version_result {
        Err(e) => println!("{:?}", e),
        Ok(version) => println!("{}", version),
    }
}
```

### ... diagnostic code URLs

`miette` supports providing a URL for individual diagnostics. This URL will
be displayed as an actual link in supported terminals, like so:

<img
src="https://raw.githubusercontent.com/zkat/miette/main/images/code_linking.png"
alt=" Example showing the graphical report printer for miette
pretty-printing an error code. The code is underlined and followed by text
saying to 'click here'. A hover tooltip shows a full-fledged URL that can be
Ctrl+Clicked to open in a browser.
\
This feature is also available in the narratable printer. It will add a line
after printing the error code showing a plain URL that you can visit.
">

To use this, you can add a `url()` sub-param to your `#[diagnostic]`
attribute:

```rust
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("kaboom")]
#[diagnostic(
    code(my_app::my_error),
    // You can do formatting!
    url("https://my_website.com/error_codes#{}", self.code().unwrap())
)]
struct MyErr;
```

Additionally, if you're developing a library and your error type is exported
from your crate's top level, you can use a special `url(docsrs)` option
instead of manually constructing the URL. This will automatically create a
link to this diagnostic on `docs.rs`, so folks can just go straight to your
(very high quality and detailed!) documentation on this diagnostic:

```rust
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[diagnostic(
    code(my_app::my_error),
    // Will link users to https://docs.rs/my_crate/0.0.0/my_crate/struct.MyErr.html
    url(docsrs)
)]
#[error("kaboom")]
struct MyErr;
```

### ... snippets

Along with its general error handling and reporting features, `miette` also
includes facilities for adding error spans/annotations/labels to your
output. This can be very useful when an error is syntax-related, but you can
even use it to print out sections of your own source code!

To achieve this, `miette` defines its own lightweight [`SourceSpan`](#sourcespan) type.
This is a basic byte-offset and length into an associated [`SourceCode`](#sourcecode)
and, along with the latter, gives `miette` all the information it needs to
pretty-print some snippets! You can also use your own `Into<SourceSpan>`
types as label spans.

The easiest way to define errors like this is to use the
`derive(Diagnostic)` macro:

```rust
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("oops")]
#[diagnostic(code(my_lib::random_error))]
pub struct MyErrorType {
    // The `Source` that miette will use.
    #[source_code]
    src: String,

    // This will underline/mark the specific code inside the larger
    // snippet context.
    #[label = "This is the highlight"]
    err_span: SourceSpan,

    // You can add as many labels as you want.
    // They'll be rendered sequentially.
    #[label("This is bad")]
    snip2: (usize, usize), // `(usize, usize)` is `Into<SourceSpan>`!

    // Snippets can be optional, by using Option:
    #[label("some text")]
    snip3: Option<SourceSpan>,

    // with or without label text
    #[label]
    snip4: Option<SourceSpan>,
}
```

### ... help text
`miette` provides two facilities for supplying help text for your errors:

The first is the `#[help()]` format attribute that applies to structs or
enum variants:

```rust
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("welp")]
#[diagnostic(help("try doing this instead"))]
struct Foo;
```

The other is by programmatically supplying the help text as a field to
your diagnostic:

```rust
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("welp")]
#[diagnostic()]
struct Foo {
    #[help]
    advice: Option<String>, // Can also just be `String`
}

let err = Foo {
    advice: Some("try doing this instead".to_string()),
};
```

### ... severity level
`miette` provides a way to set the severity level of a diagnostic.

```rust
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("welp")]
#[diagnostic(severity(Warning))]
struct Foo;
```

### ... multiple related errors

`miette` supports collecting multiple errors into a single diagnostic, and
printing them all together nicely.

To do so, use the `#[related]` tag on any `IntoIter` field in your
`Diagnostic` type:

```rust
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("oops")]
struct MyError {
    #[related]
    others: Vec<MyError>,
}
```

### ... delayed source code

Sometimes it makes sense to add source code to the error message later.
One option is to use [`with_source_code()`](Report::with_source_code)
method for that:

```rust,no_run
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("oops")]
#[diagnostic()]
pub struct MyErrorType {
    // Note: label but no source code
    #[label]
    err_span: SourceSpan,
}

fn do_something() -> miette::Result<()> {
    // This function emits actual error with label
    return Err(MyErrorType {
        err_span: (7..11).into(),
    })?;
}

fn main() -> miette::Result<()> {
    do_something().map_err(|error| {
        // And this code provides the source code for inner error
        error.with_source_code(String::from("source code"))
    })
}
```

Also source code can be provided by a wrapper type. This is especially
useful in combination with `related`, when multiple errors should be
emitted at the same time:

```rust,no_run
use miette::{Diagnostic, Report, SourceSpan};
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("oops")]
#[diagnostic()]
pub struct InnerError {
    // Note: label but no source code
    #[label]
    err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("oops: multiple errors")]
#[diagnostic()]
pub struct MultiError {
    // Note source code by no labels
    #[source_code]
    source_code: String,
    // The source code above is used for these errors
    #[related]
    related: Vec<InnerError>,
}

fn do_something() -> Result<(), Vec<InnerError>> {
    Err(vec![
        InnerError {
            err_span: (0..6).into(),
        },
        InnerError {
            err_span: (7..11).into(),
        },
    ])
}

fn main() -> miette::Result<()> {
    do_something().map_err(|err_list| MultiError {
        source_code: "source code".into(),
        related: err_list,
    })?;
    Ok(())
}
```

### ... Diagnostic-based error sources.

When one uses the `#[source]` attribute on a field, that usually comes
from `thiserror`, and implements a method for
`std::error::Error::source`. This works in many cases, but it's lossy:
if the source of the diagnostic is a diagnostic itself, the source will
simply be treated as an `std::error::Error`.

While this has no effect on the existing _reporters_, since they don't use
that information right now, APIs who might want this information will have
no access to it.

If it's important for you for this information to be available to users,
you can use `#[diagnostic_source]` alongside `#[source]`. Not that you
will likely want to use _both_:

```rust
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
#[error("MyError")]
struct MyError {
    #[source]
    #[diagnostic_source]
    the_cause: OtherError,
}

#[derive(Debug, Diagnostic, Error)]
#[error("OtherError")]
struct OtherError;
```

### ... handler options

[`MietteHandler`](#miettehandler) is the default handler, and is very customizable. In
most cases, you can simply use [`MietteHandlerOpts`](#miettehandleropts) to tweak its behavior
instead of falling back to your own custom handler.

Usage is like so:

```rust,ignore
miette::set_hook(Box::new(|_| {
    Box::new(
        miette::MietteHandlerOpts::new()
            .terminal_links(true)
            .unicode(false)
            .context_lines(3)
            .tab_width(4)
            .break_words(true)
            .build(),
    )
}))

```

See the docs for [`MietteHandlerOpts`](#miettehandleropts) for more details on what you can
customize!

### ... dynamic diagnostics

If you...
- ...don't know all the possible errors upfront
- ...need to serialize/deserialize errors
  then you may want to use [`miette!`](#miette), [`diagnostic!`](#diagnostic) macros or
  [`MietteDiagnostic`](#miettediagnostic) directly to create diagnostic on the fly.

```rust,ignore
use miette::{miette, LabeledSpan, Report};

let source = "2 + 2 * 2 = 8".to_string();
let report = miette!(
  labels = vec![
      LabeledSpan::at(12..13, "this should be 6"),
  ],
  help = "'*' has greater precedence than '+'",
  "Wrong answer"
).with_source_code(source);
println!("{:?}", report)
```

### ... syntax highlighting

`miette` can be configured to highlight syntax in source code snippets.

<!-- TODO: screenshot goes here once default Theme is decided -->

To use the built-in highlighting functionality, you must enable the
`syntect-highlighter` crate feature. When this feature is enabled, `miette` will
automatically use the `syntect` crate to highlight the `#[source_code]`
field of your [`Diagnostic`](#diagnostic).

Syntax detection with `syntect` is handled by checking 2 methods on the [`SpanContents`](#spancontents) trait, in order:
* [`language()`](SpanContents::language) - Provides the name of the language
  as a string. For example `"Rust"` will indicate Rust syntax highlighting.
  You can set the language of the [`SpanContents`](#spancontents) produced by a
  [`NamedSource`](#namedsource) via the [`with_language`](NamedSource::with_language)
  method.
* [`name()`](SpanContents::name) - In the absence of an explicitly set
  language, the name is assumed to contain a file name or file path.
  The highlighter will check for a file extension at the end of the name and
  try to guess the syntax from that.

If you want to use a custom highlighter, you can provide a custom
implementation of the [`Highlighter`](highlighters::Highlighter)
trait to [`MietteHandlerOpts`](#miettehandleropts) by calling the
[`with_syntax_highlighting`](MietteHandlerOpts::with_syntax_highlighting)
method. See the [`highlighters`](highlighters/index.md) module docs for more details.

### ... primary label

You can use the `primary` parameter to `label` to indicate that the label
is the primary label.

```rust,ignore
#[derive(Debug, Diagnostic, Error)]
#[error("oops!")]
struct MyError {
    #[label(primary, "main issue")]
    primary_span: SourceSpan,

    #[label("other label")]
    other_span: SourceSpan,
}
```

The `primary` parameter can be used at most once:

```rust,ignore
#[derive(Debug, Diagnostic, Error)]
#[error("oops!")]
struct MyError {
    #[label(primary, "main issue")]
    primary_span: SourceSpan,

    #[label(primary, "other label")] // Error: Cannot have more than one primary label.
    other_span: SourceSpan,
}
```

### ... collection of labels

When the number of labels is unknown, you can use a collection of `SourceSpan`
(or any type convertible into `SourceSpan`). For this, add the `collection`
parameter to `label` and use any type than can be iterated over for the field.

```rust,ignore
#[derive(Debug, Diagnostic, Error)]
#[error("oops!")]
struct MyError {
    #[label("main issue")]
    primary_span: SourceSpan,

    #[label(collection, "related to this")]
    other_spans: Vec<Range<usize>>,
}

let report: miette::Report = MyError {
    primary_span: (6, 9).into(),
    other_spans: vec![19..26, 30..41],
}.into();

println!("{:?}", report.with_source_code("About something or another or yet another ...".to_string()));
```

A collection can also be of `LabeledSpan` if you want to have different text
for different labels. Labels with no text will use the one from the `label`
attribute

```rust,ignore
#[derive(Debug, Diagnostic, Error)]
#[error("oops!")]
struct MyError {
    #[label("main issue")]
    primary_span: SourceSpan,

    #[label(collection, "related to this")]
    other_spans: Vec<LabeledSpan>, // LabeledSpan
}

let report: miette::Report = MyError {
    primary_span: (6, 9).into(),
    other_spans: vec![
        LabeledSpan::new(None, 19, 7), // Use default text `related to this`
        LabeledSpan::new(Some("and also this".to_string()), 30, 11), // Use specific text
    ],
}.into();

println!("{:?}", report.with_source_code("About something or another or yet another ...".to_string()));
```

## MSRV

This crate requires rustc 1.70.0 or later.

## Acknowledgements

`miette` was not developed in a void. It owes enormous credit to various
other projects and their authors:

- [`anyhow`](http://crates.io/crates/anyhow) and [`color-eyre`](https://crates.io/crates/color-eyre):
  these two enormously influential error handling libraries have pushed
  forward the experience of application-level error handling and error
  reporting. `miette`'s `Report` type is an attempt at a very very rough
  version of their `Report` types.
- [`thiserror`](https://crates.io/crates/thiserror) for setting the standard
  for library-level error definitions, and for being the inspiration behind
  `miette`'s derive macro.
- `rustc` and [@estebank](https://github.com/estebank) for their
  state-of-the-art work in compiler diagnostics.
- [`ariadne`](https://crates.io/crates/ariadne) for pushing forward how
  _pretty_ these diagnostics can really look!

## License

`miette` is released to the Rust community under the [Apache license
2.0](./LICENSE).

It also includes code taken from [`eyre`](https://github.com/yaahc/eyre),
and some from [`thiserror`](https://github.com/dtolnay/thiserror), also
under the Apache License. Some code is taken from
[`ariadne`](https://github.com/zesterer/ariadne), which is MIT licensed.

## Contents

- [Modules](#modules)
  - [`chain`](#chain)
  - [`diagnostic_chain`](#diagnostic-chain)
  - [`diagnostic_impls`](#diagnostic-impls)
  - [`error`](#error)
  - [`eyreish`](#eyreish)
  - [`handler`](#handler)
  - [`handlers`](#handlers)
  - [`highlighters`](#highlighters)
  - [`miette_diagnostic`](#miette-diagnostic)
  - [`named_source`](#named-source)
  - [`panic`](#panic)
  - [`protocol`](#protocol)
  - [`source_impls`](#source-impls)
  - [`context`](#context)
  - [`error`](#error)
  - [`fmt`](#fmt)
  - [`into_diagnostic`](#into-diagnostic)
  - [`kind`](#kind)
  - [`macros`](#macros)
  - [`ptr`](#ptr)
  - [`wrapper`](#wrapper)
  - [`syscall`](#syscall)
  - [`debug`](#debug)
  - [`graphical`](#graphical)
  - [`json`](#json)
  - [`narratable`](#narratable)
  - [`theme`](#theme)
- [Structs](#structs)
  - [`Report`](#report)
  - [`InstallError`](#installerror)
  - [`MietteHandlerOpts`](#miettehandleropts)
  - [`MietteHandler`](#miettehandler)
  - [`MietteDiagnostic`](#miettediagnostic)
  - [`NamedSource`](#namedsource)
  - [`Panic`](#panic)
  - [`LabeledSpan`](#labeledspan)
  - [`MietteSpanContents`](#miettespancontents)
  - [`SourceSpan`](#sourcespan)
  - [`SourceOffset`](#sourceoffset)
- [Enums](#enums)
  - [`MietteError`](#mietteerror)
  - [`RgbColors`](#rgbcolors)
  - [`HighlighterOption`](#highlighteroption)
  - [`Severity`](#severity)
- [Traits](#traits)
  - [`ReportHandler`](#reporthandler)
  - [`WrapErr`](#wraperr)
  - [`Diagnostic`](#diagnostic)
  - [`SourceCode`](#sourcecode)
  - [`SpanContents`](#spancontents)
- [Functions](#functions)
  - [`set_hook`](#set-hook)
  - [`capture_handler`](#capture-handler)
  - [`get_default_printer`](#get-default-printer)
  - [`set_panic_hook`](#set-panic-hook)
- [Type Aliases](#type-aliases)
  - [`ErrorHook`](#errorhook)
  - [`Result`](#result)
  - [`ByteOffset`](#byteoffset)
- [Macros](#macros)
  - [`bail!`](#bail)
  - [`ensure!`](#ensure)
  - [`miette!`](#miette)
  - [`diagnostic!`](#diagnostic)
  - [`box_error_impls!`](#box-error-impls)
  - [`box_borrow_impls!`](#box-borrow-impls)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`chain`](#chain) | mod | Iterate over error `.source()` chains. |
| [`diagnostic_chain`](#diagnostic-chain) | mod | Iterate over error `.diagnostic_source()` chains. |
| [`diagnostic_impls`](#diagnostic-impls) | mod | Default trait implementations for [`Diagnostic`]. |
| [`error`](#error) | mod |  |
| [`eyreish`](#eyreish) | mod |  |
| [`handler`](#handler) | mod |  |
| [`handlers`](#handlers) | mod | Reporters included with `miette`. |
| [`highlighters`](#highlighters) | mod | This module provides a trait for creating custom syntax highlighters that highlight [`Diagnostic`](crate::Diagnostic) source code with ANSI escape sequences when rendering with the [`GraphicalReportHighlighter`](crate::GraphicalReportHandler). |
| [`miette_diagnostic`](#miette-diagnostic) | mod |  |
| [`named_source`](#named-source) | mod |  |
| [`panic`](#panic) | mod |  |
| [`protocol`](#protocol) | mod | This module defines the core of the miette protocol: a series of types and traits that you can implement to get access to miette's (and related library's) full reporting and such features. |
| [`source_impls`](#source-impls) | mod | Default trait implementations for [`SourceCode`]. |
| [`context`](#context) | mod |  |
| [`error`](#error) | mod |  |
| [`fmt`](#fmt) | mod |  |
| [`into_diagnostic`](#into-diagnostic) | mod |  |
| [`kind`](#kind) | mod |  |
| [`macros`](#macros) | mod |  |
| [`ptr`](#ptr) | mod |  |
| [`wrapper`](#wrapper) | mod |  |
| [`syscall`](#syscall) | mod |  |
| [`debug`](#debug) | mod |  |
| [`graphical`](#graphical) | mod |  |
| [`json`](#json) | mod |  |
| [`narratable`](#narratable) | mod |  |
| [`theme`](#theme) | mod |  |
| [`Report`](#report) | struct | Core Diagnostic wrapper type. |
| [`InstallError`](#installerror) | struct | Error indicating that [`set_hook()`] was unable to install the provided [`ErrorHook`]. |
| [`MietteHandlerOpts`](#miettehandleropts) | struct | Create a custom [`MietteHandler`] from options. |
| [`MietteHandler`](#miettehandler) | struct | A [`ReportHandler`] that displays a given [`Report`](crate::Report) in a quasi-graphical way, using terminal colors, unicode drawing characters, and other such things. |
| [`MietteDiagnostic`](#miettediagnostic) | struct | Diagnostic that can be created at runtime. |
| [`NamedSource`](#namedsource) | struct | Utility struct for when you have a regular [`SourceCode`] type that doesn't implement `name`. |
| [`Panic`](#panic) | struct |  |
| [`LabeledSpan`](#labeledspan) | struct | A labeled [`SourceSpan`]. |
| [`MietteSpanContents`](#miettespancontents) | struct | Basic implementation of the [`SpanContents`] trait, for convenience. |
| [`SourceSpan`](#sourcespan) | struct | Span within a [`SourceCode`] |
| [`SourceOffset`](#sourceoffset) | struct | Newtype that represents the [`ByteOffset`] from the beginning of a [`SourceCode`] |
| [`MietteError`](#mietteerror) | enum | Error enum for miette. |
| [`RgbColors`](#rgbcolors) | enum | Settings to control the color format used for graphical rendering. |
| [`HighlighterOption`](#highlighteroption) | enum |  |
| [`Severity`](#severity) | enum | [`Diagnostic`] severity. |
| [`ReportHandler`](#reporthandler) | trait | Error Report Handler trait for customizing `miette::Report` |
| [`WrapErr`](#wraperr) | trait | Provides the [`wrap_err()`](WrapErr::wrap_err) method for [`Result`]. |
| [`Diagnostic`](#diagnostic) | trait | Adds rich metadata to your Error that can be used by [`Report`](crate::Report) to print really nice and human-friendly error messages. |
| [`SourceCode`](#sourcecode) | trait | Represents readable source code of some sort. |
| [`SpanContents`](#spancontents) | trait | Contents of a [`SourceCode`] covered by [`SourceSpan`]. |
| [`set_hook`](#set-hook) | fn | Set the error hook. |
| [`capture_handler`](#capture-handler) | fn |  |
| [`get_default_printer`](#get-default-printer) | fn |  |
| [`set_panic_hook`](#set-panic-hook) | fn | Tells miette to render panics using its rendering engine. |
| [`ErrorHook`](#errorhook) | type |  |
| [`Result`](#result) | type | type alias for `Result<T, Report>` |
| [`ByteOffset`](#byteoffset) | type | "Raw" type for the byte offset from the beginning of a [`SourceCode`]. |
| [`bail!`](#bail) | macro | Return early with an error. |
| [`ensure!`](#ensure) | macro | Return early with an error if a condition is not satisfied. |
| [`miette!`](#miette) | macro | Construct an ad-hoc [`Report`]. |
| [`diagnostic!`](#diagnostic) | macro | Construct a [`MietteDiagnostic`] in more user-friendly way. |
| [`box_error_impls!`](#box-error-impls) | macro |  |
| [`box_borrow_impls!`](#box-borrow-impls) | macro |  |

## Modules

- [`chain`](chain/index.md) — Iterate over error `.source()` chains.
- [`diagnostic_chain`](diagnostic_chain/index.md) — Iterate over error `.diagnostic_source()` chains.
- [`diagnostic_impls`](diagnostic_impls/index.md) — Default trait implementations for [`Diagnostic`].
- [`error`](error/index.md)
- [`eyreish`](eyreish/index.md)
- [`handler`](handler/index.md)
- [`handlers`](handlers/index.md) — Reporters included with `miette`.
- [`highlighters`](highlighters/index.md) — This module provides a trait for creating custom syntax highlighters that
- [`miette_diagnostic`](miette_diagnostic/index.md)
- [`named_source`](named_source/index.md)
- [`panic`](panic/index.md)
- [`protocol`](protocol/index.md) — This module defines the core of the miette protocol: a series of types and
- [`source_impls`](source_impls/index.md) — Default trait implementations for [`SourceCode`].
- [`context`](context/index.md)
- [`error`](error/index.md)
- [`fmt`](fmt/index.md)
- [`into_diagnostic`](into_diagnostic/index.md)
- [`kind`](kind/index.md)
- [`macros`](macros/index.md)
- [`ptr`](ptr/index.md)
- [`wrapper`](wrapper/index.md)
- [`syscall`](syscall/index.md)
- [`debug`](debug/index.md)
- [`graphical`](graphical/index.md)
- [`json`](json/index.md)
- [`narratable`](narratable/index.md)
- [`theme`](theme/index.md)

## Structs

### `Report`

```rust
struct Report {
    inner: self::ptr::Own<error::ErrorImpl<()>>,
}
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:53-55`](../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L53-L55)*

Core Diagnostic wrapper type.

## `eyre` Users

You can just replace `use`s of `eyre::Report` with `miette::Report`.

#### Implementations

- <span id="superreport-new"></span>`fn new<E>(error: E) -> Self`

- <span id="superreport-msg"></span>`fn msg<M>(message: M) -> Self`

- <span id="superreport-new-boxed"></span>`fn new_boxed(error: Box<dyn Diagnostic + Send + Sync>) -> Self` — [`Diagnostic`](#diagnostic)

- <span id="superreport-from-std"></span>`fn from_std<E>(error: E) -> Self`

- <span id="superreport-from-adhoc"></span>`fn from_adhoc<M>(message: M) -> Self`

- <span id="superreport-from-msg"></span>`fn from_msg<D, E>(msg: D, error: E) -> Self`

- <span id="superreport-from-boxed"></span>`fn from_boxed(error: Box<dyn Diagnostic + Send + Sync>) -> Self` — [`Diagnostic`](#diagnostic)

- <span id="superreport-construct"></span>`unsafe fn construct<E>(error: E, vtable: &'static ErrorVTable, handler: Option<Box<dyn ReportHandler>>) -> Self` — [`ErrorVTable`](eyreish/error/index.md#errorvtable), [`ReportHandler`](#reporthandler)

- <span id="superreport-wrap-err"></span>`fn wrap_err<D>(self, msg: D) -> Self`

- <span id="superreport-context"></span>`fn context<D>(self, msg: D) -> Self`

- <span id="superreport-chain"></span>`fn chain(&self) -> Chain<'_>` — [`Chain`](chain/index.md#chain)

- <span id="superreport-root-cause"></span>`fn root_cause(&self) -> &dyn StdError`

- <span id="superreport-is"></span>`fn is<E>(&self) -> bool`

- <span id="superreport-downcast"></span>`fn downcast<E>(self) -> Result<E, Self>`

- <span id="superreport-downcast-ref"></span>`fn downcast_ref<E>(&self) -> Option<&E>`

- <span id="superreport-downcast-mut"></span>`fn downcast_mut<E>(&mut self) -> Option<&mut E>`

- <span id="superreport-handler"></span>`fn handler(&self) -> &dyn ReportHandler` — [`ReportHandler`](#reporthandler)

- <span id="superreport-handler-mut"></span>`fn handler_mut(&mut self) -> &mut dyn ReportHandler` — [`ReportHandler`](#reporthandler)

- <span id="superreport-with-source-code"></span>`fn with_source_code(self, source_code: impl SourceCode + 'static) -> Report` — [`SourceCode`](#sourcecode), [`Report`](#report)

- <span id="superreport-from-err"></span>`fn from_err<E>(err: E) -> Self`

#### Trait Implementations

##### `impl AsRef for super::Report`

- <span id="superreport-as-ref"></span>`fn as_ref(&self) -> &dyn Diagnostic + Send + Sync` — [`Diagnostic`](#diagnostic)

##### `impl Debug for super::Report`

- <span id="superreport-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deref for super::Report`

- <span id="superreport-deref-type-target"></span>`type Target = dyn Diagnostic + Sync + Send`

- <span id="superreport-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl DerefMut for super::Report`

- <span id="superreport-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Diag for super::Report`

- <span id="superreport-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](#report)

##### `impl Display for super::Report`

- <span id="superreport-fmt"></span>`fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Drop for super::Report`

- <span id="superreport-drop"></span>`fn drop(&mut self)`

##### `impl OwoColorize for Report`

##### `impl Receiver for Report`

- <span id="report-receiver-type-target"></span>`type Target = T`

##### `impl Send for Report`

##### `impl Sync for Report`

##### `impl ToString for Report`

- <span id="report-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for Report`

### `InstallError`

```rust
struct InstallError;
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:69`](../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L69)*

Error indicating that [`set_hook()`](#set-hook) was unable to install the provided
[`ErrorHook`](#errorhook).

#### Trait Implementations

##### `impl Debug for InstallError`

- <span id="installerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for InstallError`

- <span id="installerror-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](#report)

##### `impl Diagnostic for InstallError`

##### `impl Display for InstallError`

- <span id="installerror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for InstallError`

##### `impl OwoColorize for InstallError`

##### `impl ToString for InstallError`

- <span id="installerror-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for InstallError`

### `MietteHandlerOpts`

```rust
struct MietteHandlerOpts {
    linkify: Option<bool>,
    width: Option<usize>,
    theme: Option<crate::GraphicalTheme>,
    force_graphical: Option<bool>,
    force_narrated: Option<bool>,
    rgb_colors: RgbColors,
    color: Option<bool>,
    unicode: Option<bool>,
    footer: Option<String>,
    context_lines: Option<usize>,
    tab_width: Option<usize>,
    with_cause_chain: Option<bool>,
    break_words: Option<bool>,
    wrap_lines: Option<bool>,
    word_separator: Option<textwrap::WordSeparator>,
    word_splitter: Option<textwrap::WordSplitter>,
    highlighter: Option<crate::highlighters::MietteHighlighter>,
    show_related_as_nested: Option<bool>,
}
```

*Defined in [`miette-7.6.0/src/handler.rs:42-61`](../../.source_1765210505/miette-7.6.0/src/handler.rs#L42-L61)*

Create a custom [`MietteHandler`](#miettehandler) from options.

## Example

```no_run
miette::set_hook(Box::new(|_| {
    Box::new(miette::MietteHandlerOpts::new()
        .terminal_links(true)
        .unicode(false)
        .context_lines(3)
        .build())
}))
.unwrap();
```

#### Implementations

- <span id="miettehandleropts-new"></span>`fn new() -> Self`

- <span id="miettehandleropts-terminal-links"></span>`fn terminal_links(self, linkify: bool) -> Self`

- <span id="miettehandleropts-graphical-theme"></span>`fn graphical_theme(self, theme: GraphicalTheme) -> Self` — [`GraphicalTheme`](handlers/index.md#graphicaltheme)

- <span id="miettehandleropts-with-syntax-highlighting"></span>`fn with_syntax_highlighting(self, highlighter: impl Highlighter + Send + Sync + 'static) -> Self` — [`Highlighter`](highlighters/index.md#highlighter)

- <span id="miettehandleropts-without-syntax-highlighting"></span>`fn without_syntax_highlighting(self) -> Self`

- <span id="miettehandleropts-width"></span>`fn width(self, width: usize) -> Self`

- <span id="miettehandleropts-wrap-lines"></span>`fn wrap_lines(self, wrap_lines: bool) -> Self`

- <span id="miettehandleropts-break-words"></span>`fn break_words(self, break_words: bool) -> Self`

- <span id="miettehandleropts-word-separator"></span>`fn word_separator(self, word_separator: textwrap::WordSeparator) -> Self`

- <span id="miettehandleropts-word-splitter"></span>`fn word_splitter(self, word_splitter: textwrap::WordSplitter) -> Self`

- <span id="miettehandleropts-with-cause-chain"></span>`fn with_cause_chain(self) -> Self`

- <span id="miettehandleropts-without-cause-chain"></span>`fn without_cause_chain(self) -> Self`

- <span id="miettehandleropts-show-related-errors-as-siblings"></span>`fn show_related_errors_as_siblings(self) -> Self`

- <span id="miettehandleropts-show-related-errors-as-nested"></span>`fn show_related_errors_as_nested(self) -> Self`

- <span id="miettehandleropts-color"></span>`fn color(self, color: bool) -> Self`

- <span id="miettehandleropts-rgb-colors"></span>`fn rgb_colors(self, color: RgbColors) -> Self` — [`RgbColors`](#rgbcolors)

- <span id="miettehandleropts-unicode"></span>`fn unicode(self, unicode: bool) -> Self`

- <span id="miettehandleropts-force-graphical"></span>`fn force_graphical(self, force: bool) -> Self`

- <span id="miettehandleropts-force-narrated"></span>`fn force_narrated(self, force: bool) -> Self`

- <span id="miettehandleropts-footer"></span>`fn footer(self, footer: String) -> Self`

- <span id="miettehandleropts-context-lines"></span>`fn context_lines(self, context_lines: usize) -> Self`

- <span id="miettehandleropts-tab-width"></span>`fn tab_width(self, width: usize) -> Self`

- <span id="miettehandleropts-build"></span>`fn build(self) -> MietteHandler` — [`MietteHandler`](#miettehandler)

- <span id="miettehandleropts-is-graphical"></span>`fn is_graphical(&self) -> bool`

- <span id="miettehandleropts-use-links"></span>`fn use_links(&self) -> bool`

- <span id="miettehandleropts-get-width"></span>`fn get_width(&self) -> usize`

#### Trait Implementations

##### `impl Clone for MietteHandlerOpts`

- <span id="miettehandleropts-clone"></span>`fn clone(&self) -> MietteHandlerOpts` — [`MietteHandlerOpts`](#miettehandleropts)

##### `impl Debug for MietteHandlerOpts`

- <span id="miettehandleropts-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MietteHandlerOpts`

- <span id="miettehandleropts-default"></span>`fn default() -> MietteHandlerOpts` — [`MietteHandlerOpts`](#miettehandleropts)

##### `impl OwoColorize for MietteHandlerOpts`

### `MietteHandler`

```rust
struct MietteHandler {
    inner: Box<dyn ReportHandler + Send + Sync>,
}
```

*Defined in [`miette-7.6.0/src/handler.rs:387-389`](../../.source_1765210505/miette-7.6.0/src/handler.rs#L387-L389)*

A [`ReportHandler`](#reporthandler) that displays a given [`Report`](crate::Report) in a
quasi-graphical way, using terminal colors, unicode drawing characters, and
other such things.

This is the default reporter bundled with `miette`.

This printer can be customized by using
`GraphicalReportHandler::new_themed()` and handing it a [`GraphicalTheme`](handlers/index.md) of
your own creation (or using one of its own defaults).

See [`set_hook`](crate::set_hook) for more details on customizing your global
printer.

#### Implementations

- <span id="miettehandler-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl Default for MietteHandler`

- <span id="miettehandler-default"></span>`fn default() -> Self`

##### `impl OwoColorize for MietteHandler`

##### `impl ReportHandler for MietteHandler`

- <span id="miettehandler-debug"></span>`fn debug(&self, diagnostic: &dyn Diagnostic, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`Diagnostic`](#diagnostic)

### `MietteDiagnostic`

```rust
struct MietteDiagnostic {
    pub message: String,
    pub code: Option<String>,
    pub severity: Option<crate::Severity>,
    pub help: Option<String>,
    pub url: Option<String>,
    pub labels: Option<Vec<crate::LabeledSpan>>,
}
```

*Defined in [`miette-7.6.0/src/miette_diagnostic.rs:14-39`](../../.source_1765210505/miette-7.6.0/src/miette_diagnostic.rs#L14-L39)*

Diagnostic that can be created at runtime.

#### Fields

- **`message`**: `String`

  Displayed diagnostic message

- **`code`**: `Option<String>`

  Unique diagnostic code to look up more information
  about this Diagnostic. Ideally also globally unique, and documented
  in the toplevel crate's documentation for easy searching.
  Rust path format (`foo::bar::baz`) is recommended, but more classic
  codes like `E0123` will work just fine

- **`severity`**: `Option<crate::Severity>`

  [`Diagnostic`](#diagnostic) severity. Intended to be used by
  [`ReportHandler`](crate::ReportHandler)s to change the way different
  [`Diagnostic`](#diagnostic)s are displayed. Defaults to [`Severity::Error`](#severityerror)

- **`help`**: `Option<String>`

  Additional help text related to this Diagnostic

- **`url`**: `Option<String>`

  URL to visit for a more detailed explanation/help about this
  [`Diagnostic`](#diagnostic).

- **`labels`**: `Option<Vec<crate::LabeledSpan>>`

  Labels to apply to this `Diagnostic`'s `Diagnostic::source_code`

#### Implementations

- <span id="miettediagnostic-new"></span>`fn new(message: impl Into<String>) -> Self`

- <span id="miettediagnostic-with-code"></span>`fn with_code(self, code: impl Into<String>) -> Self`

- <span id="miettediagnostic-with-severity"></span>`fn with_severity(self, severity: Severity) -> Self` — [`Severity`](#severity)

- <span id="miettediagnostic-with-help"></span>`fn with_help(self, help: impl Into<String>) -> Self`

- <span id="miettediagnostic-with-url"></span>`fn with_url(self, url: impl Into<String>) -> Self`

- <span id="miettediagnostic-with-label"></span>`fn with_label(self, label: impl Into<LabeledSpan>) -> Self` — [`LabeledSpan`](#labeledspan)

- <span id="miettediagnostic-with-labels"></span>`fn with_labels(self, labels: impl IntoIterator<Item = LabeledSpan>) -> Self` — [`LabeledSpan`](#labeledspan)

- <span id="miettediagnostic-and-label"></span>`fn and_label(self, label: impl Into<LabeledSpan>) -> Self` — [`LabeledSpan`](#labeledspan)

- <span id="miettediagnostic-and-labels"></span>`fn and_labels(self, labels: impl IntoIterator<Item = LabeledSpan>) -> Self` — [`LabeledSpan`](#labeledspan)

#### Trait Implementations

##### `impl Clone for MietteDiagnostic`

- <span id="miettediagnostic-clone"></span>`fn clone(&self) -> MietteDiagnostic` — [`MietteDiagnostic`](#miettediagnostic)

##### `impl Debug for MietteDiagnostic`

- <span id="miettediagnostic-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for MietteDiagnostic`

- <span id="miettediagnostic-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](#report)

##### `impl Diagnostic for MietteDiagnostic`

- <span id="miettediagnostic-code"></span>`fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="miettediagnostic-severity"></span>`fn severity(&self) -> Option<Severity>` — [`Severity`](#severity)

- <span id="miettediagnostic-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="miettediagnostic-url"></span>`fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

- <span id="miettediagnostic-labels"></span>`fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>` — [`LabeledSpan`](#labeledspan)

##### `impl Display for MietteDiagnostic`

- <span id="miettediagnostic-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Eq for MietteDiagnostic`

##### `impl Error for MietteDiagnostic`

##### `impl OwoColorize for MietteDiagnostic`

##### `impl PartialEq for MietteDiagnostic`

- <span id="miettediagnostic-eq"></span>`fn eq(&self, other: &MietteDiagnostic) -> bool` — [`MietteDiagnostic`](#miettediagnostic)

##### `impl StructuralPartialEq for MietteDiagnostic`

##### `impl ToString for MietteDiagnostic`

- <span id="miettediagnostic-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for MietteDiagnostic`

### `NamedSource<S: SourceCode + 'static>`

```rust
struct NamedSource<S: SourceCode + 'static> {
    source: S,
    name: String,
    language: Option<String>,
}
```

*Defined in [`miette-7.6.0/src/named_source.rs:7-11`](../../.source_1765210505/miette-7.6.0/src/named_source.rs#L7-L11)*

Utility struct for when you have a regular [`SourceCode`](#sourcecode) type that doesn't
implement `name`. For example [`String`](../clap_builder/index.md). Or if you want to override the
`name` returned by the `SourceCode`.

#### Implementations

- <span id="namedsource-new"></span>`fn new(name: impl AsRef<str>, source: S) -> Self`

- <span id="namedsource-name"></span>`fn name(&self) -> &str`

- <span id="namedsource-inner"></span>`fn inner(&self) -> &S`

- <span id="namedsource-with-language"></span>`fn with_language(self, language: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl<S: clone::Clone + SourceCode + 'static> Clone for NamedSource<S>`

- <span id="namedsource-clone"></span>`fn clone(&self) -> NamedSource<S>` — [`NamedSource`](#namedsource)

##### `impl<S: SourceCode> Debug for NamedSource<S>`

- <span id="namedsource-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl<S: cmp::Eq + SourceCode + 'static> Eq for NamedSource<S>`

##### `impl<S: hash::Hash + SourceCode + 'static> Hash for NamedSource<S>`

- <span id="namedsource-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<S: cmp::Ord + SourceCode + 'static> Ord for NamedSource<S>`

- <span id="namedsource-cmp"></span>`fn cmp(&self, other: &NamedSource<S>) -> cmp::Ordering` — [`NamedSource`](#namedsource)

##### `impl OwoColorize for NamedSource<S>`

##### `impl<S: cmp::PartialEq + SourceCode + 'static> PartialEq for NamedSource<S>`

- <span id="namedsource-eq"></span>`fn eq(&self, other: &NamedSource<S>) -> bool` — [`NamedSource`](#namedsource)

##### `impl<S: cmp::PartialOrd + SourceCode + 'static> PartialOrd for NamedSource<S>`

- <span id="namedsource-partial-cmp"></span>`fn partial_cmp(&self, other: &NamedSource<S>) -> option::Option<cmp::Ordering>` — [`NamedSource`](#namedsource)

##### `impl<S: SourceCode + 'static> SourceCode for NamedSource<S>`

- <span id="namedsource-read-span"></span>`fn read_span<'a>(self: &'a Self, span: &crate::SourceSpan, context_lines_before: usize, context_lines_after: usize) -> Result<Box<dyn SpanContents<'a>>, MietteError>` — [`SourceSpan`](#sourcespan), [`SpanContents`](#spancontents), [`MietteError`](#mietteerror)

##### `impl<S: SourceCode + 'static> StructuralPartialEq for NamedSource<S>`

### `Panic`

```rust
struct Panic(String);
```

*Defined in [`miette-7.6.0/src/panic.rs:30`](../../.source_1765210505/miette-7.6.0/src/panic.rs#L30)*

#### Implementations

- <span id="panic-backtrace"></span>`fn backtrace() -> String`

#### Trait Implementations

##### `impl Debug for Panic`

- <span id="panic-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for Panic`

- <span id="panic-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](#report)

##### `impl Diagnostic for Panic`

- <span id="panic-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

##### `impl Display for Panic`

- <span id="panic-fmt"></span>`fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result`

##### `impl Error for Panic`

##### `impl OwoColorize for Panic`

##### `impl ToString for Panic`

- <span id="panic-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for Panic`

### `LabeledSpan`

```rust
struct LabeledSpan {
    label: Option<String>,
    span: SourceSpan,
    primary: bool,
}
```

*Defined in [`miette-7.6.0/src/protocol.rs:250-255`](../../.source_1765210505/miette-7.6.0/src/protocol.rs#L250-L255)*

A labeled [`SourceSpan`](#sourcespan).

#### Implementations

- <span id="labeledspan-new"></span>`const fn new(label: Option<String>, offset: ByteOffset, len: usize) -> Self` — [`ByteOffset`](#byteoffset)

- <span id="labeledspan-new-with-span"></span>`fn new_with_span(label: Option<String>, span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](#sourcespan)

- <span id="labeledspan-new-primary-with-span"></span>`fn new_primary_with_span(label: Option<String>, span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](#sourcespan)

- <span id="labeledspan-set-label"></span>`fn set_label(&mut self, label: Option<String>)`

- <span id="labeledspan-at"></span>`fn at(span: impl Into<SourceSpan>, label: impl Into<String>) -> Self` — [`SourceSpan`](#sourcespan)

- <span id="labeledspan-at-offset"></span>`fn at_offset(offset: ByteOffset, label: impl Into<String>) -> Self` — [`ByteOffset`](#byteoffset)

- <span id="labeledspan-underline"></span>`fn underline(span: impl Into<SourceSpan>) -> Self` — [`SourceSpan`](#sourcespan)

- <span id="labeledspan-label"></span>`fn label(&self) -> Option<&str>`

- <span id="labeledspan-inner"></span>`const fn inner(&self) -> &SourceSpan` — [`SourceSpan`](#sourcespan)

- <span id="labeledspan-offset"></span>`const fn offset(&self) -> usize`

- <span id="labeledspan-len"></span>`const fn len(&self) -> usize`

- <span id="labeledspan-is-empty"></span>`const fn is_empty(&self) -> bool`

- <span id="labeledspan-primary"></span>`const fn primary(&self) -> bool`

#### Trait Implementations

##### `impl Clone for LabeledSpan`

- <span id="labeledspan-clone"></span>`fn clone(&self) -> LabeledSpan` — [`LabeledSpan`](#labeledspan)

##### `impl Debug for LabeledSpan`

- <span id="labeledspan-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for LabeledSpan`

##### `impl OwoColorize for LabeledSpan`

##### `impl PartialEq for LabeledSpan`

- <span id="labeledspan-eq"></span>`fn eq(&self, other: &LabeledSpan) -> bool` — [`LabeledSpan`](#labeledspan)

##### `impl StructuralPartialEq for LabeledSpan`

### `MietteSpanContents<'a>`

```rust
struct MietteSpanContents<'a> {
    data: &'a [u8],
    span: SourceSpan,
    line: usize,
    column: usize,
    line_count: usize,
    name: Option<String>,
    language: Option<String>,
}
```

*Defined in [`miette-7.6.0/src/protocol.rs:458-473`](../../.source_1765210505/miette-7.6.0/src/protocol.rs#L458-L473)*

Basic implementation of the [`SpanContents`](#spancontents) trait, for convenience.

#### Implementations

- <span id="miettespancontents-new"></span>`const fn new(data: &'a [u8], span: SourceSpan, line: usize, column: usize, line_count: usize) -> MietteSpanContents<'a>` — [`SourceSpan`](#sourcespan), [`MietteSpanContents`](#miettespancontents)

- <span id="miettespancontents-new-named"></span>`const fn new_named(name: String, data: &'a [u8], span: SourceSpan, line: usize, column: usize, line_count: usize) -> MietteSpanContents<'a>` — [`SourceSpan`](#sourcespan), [`MietteSpanContents`](#miettespancontents)

- <span id="miettespancontents-with-language"></span>`fn with_language(self, language: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl Clone for MietteSpanContents<'a>`

- <span id="miettespancontents-clone"></span>`fn clone(&self) -> MietteSpanContents<'a>` — [`MietteSpanContents`](#miettespancontents)

##### `impl Debug for MietteSpanContents<'a>`

- <span id="miettespancontents-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl OwoColorize for MietteSpanContents<'a>`

##### `impl SpanContents for MietteSpanContents<'a>`

- <span id="miettespancontents-data"></span>`fn data(&self) -> &'a [u8]`

- <span id="miettespancontents-span"></span>`fn span(&self) -> &SourceSpan` — [`SourceSpan`](#sourcespan)

- <span id="miettespancontents-line"></span>`fn line(&self) -> usize`

- <span id="miettespancontents-column"></span>`fn column(&self) -> usize`

- <span id="miettespancontents-line-count"></span>`fn line_count(&self) -> usize`

- <span id="miettespancontents-name"></span>`fn name(&self) -> Option<&str>`

- <span id="miettespancontents-language"></span>`fn language(&self) -> Option<&str>`

### `SourceSpan`

```rust
struct SourceSpan {
    offset: SourceOffset,
    length: usize,
}
```

*Defined in [`miette-7.6.0/src/protocol.rs:549-554`](../../.source_1765210505/miette-7.6.0/src/protocol.rs#L549-L554)*

Span within a [`SourceCode`](#sourcecode)

#### Fields

- **`offset`**: `SourceOffset`

  The start of the span.

- **`length`**: `usize`

  The total length of the span

#### Implementations

- <span id="sourcespan-new"></span>`const fn new(start: SourceOffset, length: usize) -> Self` — [`SourceOffset`](#sourceoffset)

- <span id="sourcespan-offset"></span>`const fn offset(&self) -> usize`

- <span id="sourcespan-len"></span>`const fn len(&self) -> usize`

- <span id="sourcespan-is-empty"></span>`const fn is_empty(&self) -> bool`

#### Trait Implementations

##### `impl Clone for SourceSpan`

- <span id="sourcespan-clone"></span>`fn clone(&self) -> SourceSpan` — [`SourceSpan`](#sourcespan)

##### `impl Copy for SourceSpan`

##### `impl Debug for SourceSpan`

- <span id="sourcespan-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SourceSpan`

##### `impl Hash for SourceSpan`

- <span id="sourcespan-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SourceSpan`

- <span id="sourcespan-cmp"></span>`fn cmp(&self, other: &SourceSpan) -> cmp::Ordering` — [`SourceSpan`](#sourcespan)

##### `impl OwoColorize for SourceSpan`

##### `impl PartialEq for SourceSpan`

- <span id="sourcespan-eq"></span>`fn eq(&self, other: &SourceSpan) -> bool` — [`SourceSpan`](#sourcespan)

##### `impl PartialOrd for SourceSpan`

- <span id="sourcespan-partial-cmp"></span>`fn partial_cmp(&self, other: &SourceSpan) -> option::Option<cmp::Ordering>` — [`SourceSpan`](#sourcespan)

##### `impl StructuralPartialEq for SourceSpan`

### `SourceOffset`

```rust
struct SourceOffset(ByteOffset);
```

*Defined in [`miette-7.6.0/src/protocol.rs:673`](../../.source_1765210505/miette-7.6.0/src/protocol.rs#L673)*

Newtype that represents the [`ByteOffset`](#byteoffset) from the beginning of a [`SourceCode`](#sourcecode)

#### Implementations

- <span id="sourceoffset-offset"></span>`const fn offset(&self) -> ByteOffset` — [`ByteOffset`](#byteoffset)

- <span id="sourceoffset-from-location"></span>`fn from_location(source: impl AsRef<str>, loc_line: usize, loc_col: usize) -> Self`

- <span id="sourceoffset-from-current-location"></span>`fn from_current_location() -> Result<(String, Self), MietteError>` — [`MietteError`](#mietteerror)

#### Trait Implementations

##### `impl Clone for SourceOffset`

- <span id="sourceoffset-clone"></span>`fn clone(&self) -> SourceOffset` — [`SourceOffset`](#sourceoffset)

##### `impl Copy for SourceOffset`

##### `impl Debug for SourceOffset`

- <span id="sourceoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SourceOffset`

##### `impl Hash for SourceOffset`

- <span id="sourceoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SourceOffset`

- <span id="sourceoffset-cmp"></span>`fn cmp(&self, other: &SourceOffset) -> cmp::Ordering` — [`SourceOffset`](#sourceoffset)

##### `impl OwoColorize for SourceOffset`

##### `impl PartialEq for SourceOffset`

- <span id="sourceoffset-eq"></span>`fn eq(&self, other: &SourceOffset) -> bool` — [`SourceOffset`](#sourceoffset)

##### `impl PartialOrd for SourceOffset`

- <span id="sourceoffset-partial-cmp"></span>`fn partial_cmp(&self, other: &SourceOffset) -> option::Option<cmp::Ordering>` — [`SourceOffset`](#sourceoffset)

##### `impl StructuralPartialEq for SourceOffset`

## Enums

### `MietteError`

```rust
enum MietteError {
    IoError(io::Error),
    OutOfBounds,
}
```

*Defined in [`miette-7.6.0/src/error.rs:13-21`](../../.source_1765210505/miette-7.6.0/src/error.rs#L13-L21)*

Error enum for miette. Used by certain operations in the protocol.

#### Variants

- **`IoError`**

  Wrapper around [`std::io::Error`](../cargo_docs_md/error/index.md). This is returned when something went
  wrong while reading a [`SourceCode`](crate::SourceCode).

- **`OutOfBounds`**

  Returned when a [`SourceSpan`](crate::SourceSpan) extends beyond the
  bounds of a given [`SourceCode`](crate::SourceCode).

#### Trait Implementations

##### `impl Debug for MietteError`

- <span id="mietteerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for MietteError`

- <span id="mietteerror-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](#report)

##### `impl Diagnostic for MietteError`

- <span id="mietteerror-code"></span>`fn code<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

- <span id="mietteerror-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

- <span id="mietteerror-url"></span>`fn url<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

##### `impl Display for MietteError`

- <span id="mietteerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for MietteError`

- <span id="mietteerror-source"></span>`fn source(&self) -> Option<&dyn Error>`

##### `impl OwoColorize for MietteError`

##### `impl ToString for MietteError`

- <span id="mietteerror-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for MietteError`

### `RgbColors`

```rust
enum RgbColors {
    Always,
    Preferred,
    Never,
}
```

*Defined in [`miette-7.6.0/src/handler.rs:15-23`](../../.source_1765210505/miette-7.6.0/src/handler.rs#L15-L23)*

Settings to control the color format used for graphical rendering.

#### Variants

- **`Always`**

  Use RGB colors even if the terminal does not support them

- **`Preferred`**

  Use RGB colors instead of ANSI if the terminal supports RGB

- **`Never`**

  Always use ANSI, regardless of terminal support for RGB

#### Trait Implementations

##### `impl Clone for RgbColors`

- <span id="rgbcolors-clone"></span>`fn clone(&self) -> RgbColors` — [`RgbColors`](#rgbcolors)

##### `impl Copy for RgbColors`

##### `impl Debug for RgbColors`

- <span id="rgbcolors-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RgbColors`

- <span id="rgbcolors-default"></span>`fn default() -> RgbColors` — [`RgbColors`](#rgbcolors)

##### `impl Eq for RgbColors`

##### `impl OwoColorize for RgbColors`

##### `impl PartialEq for RgbColors`

- <span id="rgbcolors-eq"></span>`fn eq(&self, other: &RgbColors) -> bool` — [`RgbColors`](#rgbcolors)

##### `impl StructuralPartialEq for RgbColors`

### `HighlighterOption`

```rust
enum HighlighterOption {
    Disable,
    EnableCustom(crate::highlighters::MietteHighlighter),
}
```

*Defined in [`miette-7.6.0/src/handler.rs:414-419`](../../.source_1765210505/miette-7.6.0/src/handler.rs#L414-L419)*

#### Implementations

- <span id="highlighteroption-select"></span>`fn select(color: Option<bool>, highlighter: Option<MietteHighlighter>, supports_color: bool) -> HighlighterOption` — [`MietteHighlighter`](highlighters/index.md#miettehighlighter), [`HighlighterOption`](handler/index.md#highlighteroption)

#### Trait Implementations

##### `impl Default for HighlighterOption`

- <span id="highlighteroption-default"></span>`fn default() -> Self`

##### `impl OwoColorize for HighlighterOption`

### `Severity`

```rust
enum Severity {
    Advice,
    Warning,
    Error,
}
```

*Defined in [`miette-7.6.0/src/protocol.rs:189-198`](../../.source_1765210505/miette-7.6.0/src/protocol.rs#L189-L198)*

[`Diagnostic`](#diagnostic) severity. Intended to be used by
[`ReportHandler`](crate::ReportHandler)s to change the way different
[`Diagnostic`](#diagnostic)s are displayed. Defaults to [`Severity::Error`](#severityerror).

#### Variants

- **`Advice`**

  Just some help. Here's how you could be doing it better.

- **`Warning`**

  Warning. Please take note.

- **`Error`**

  Critical failure. The program cannot continue.
  This is the default severity, if you don't specify another one.

#### Trait Implementations

##### `impl Clone for Severity`

- <span id="severity-clone"></span>`fn clone(&self) -> Severity` — [`Severity`](#severity)

##### `impl Copy for Severity`

##### `impl Debug for Severity`

- <span id="severity-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Severity`

- <span id="severity-default"></span>`fn default() -> Severity` — [`Severity`](#severity)

##### `impl Eq for Severity`

##### `impl Ord for Severity`

- <span id="severity-cmp"></span>`fn cmp(&self, other: &Severity) -> cmp::Ordering` — [`Severity`](#severity)

##### `impl OwoColorize for Severity`

##### `impl PartialEq for Severity`

- <span id="severity-eq"></span>`fn eq(&self, other: &Severity) -> bool` — [`Severity`](#severity)

##### `impl PartialOrd for Severity`

- <span id="severity-partial-cmp"></span>`fn partial_cmp(&self, other: &Severity) -> option::Option<cmp::Ordering>` — [`Severity`](#severity)

##### `impl StructuralPartialEq for Severity`

## Traits

### `ReportHandler`

```rust
trait ReportHandler: core::any::Any + Send + Sync { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:144-201`](../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L144-L201)*

Error Report Handler trait for customizing `miette::Report`

#### Required Methods

- `fn debug(&self, error: &dyn Diagnostic, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

  Define the report format

#### Provided Methods

- `fn display(&self, error: &dyn StdError, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

  Override for the `Display` format

- `fn track_caller(&mut self, location: &'static std::panic::Location<'static>)`

  Store the location of the caller who constructed this error report

#### Implementors

- [`DebugReportHandler`](handlers/index.md#debugreporthandler)
- [`GraphicalReportHandler`](handlers/index.md#graphicalreporthandler)
- [`JSONReportHandler`](handlers/index.md#jsonreporthandler)
- [`MietteHandler`](#miettehandler)
- [`NarratableReportHandler`](handlers/index.md#narratablereporthandler)

### `WrapErr<T, E>`

```rust
trait WrapErr<T, E>: context::private::Sealed { ... }
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:433-460`](../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L433-L460)*

Provides the [`wrap_err()`](WrapErr::wrap_err) method for [`Result`](#result).

This trait is sealed and cannot be implemented for types outside of
`miette`.

# Example

```rust
use miette::{WrapErr, IntoDiagnostic, Result};
use std::{fs, path::PathBuf};

pub struct ImportantThing {
    path: PathBuf,
}

impl ImportantThing {
    const IGNORE: &'static str = stringify! {
    pub fn detach(&mut self) -> Result<()> {...}
    };
    fn detach(&mut self) -> Result<()> {
        unimplemented!()
    }
}

pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
    it.detach().wrap_err("Failed to detach the important thing")?;

    let path = &it.path;
    let content = fs::read(path)
        .into_diagnostic()
        .wrap_err_with(|| format!(
            "Failed to read instrs from {}",
            path.display())
        )?;

    Ok(content)
}
```

When printed, the outermost error would be printed first and the lower
level underlying causes would be enumerated below.

```console
Error: Failed to read instrs from ./path/to/instrs.json

Caused by:
    No such file or directory (os error 2)
```

# Wrapping Types That Do Not Implement `Error`

For example `&str` and `Box<dyn Error>`.

Due to restrictions for coherence `Report` cannot implement `From` for types
that don't implement `Error`. Attempts to do so will give `"this type might
implement Error in the future"` as an error. As such, `wrap_err()`, which
uses `From` under the hood, cannot be used to wrap these types. Instead we
encourage you to use the combinators provided for `Result` in `std`/`core`.

For example, instead of this:

```rust,compile_fail
use std::error::Error;
use miette::{WrapErr, Report};

fn wrap_example(err: Result<(), Box<dyn Error + Send + Sync + 'static>>)
    -> Result<(), Report>
{
    err.wrap_err("saw a downstream error")
}
```

We encourage you to write this:

```rust
use miette::{miette, Report, WrapErr};
use std::error::Error;

fn wrap_example(err: Result<(), Box<dyn Error + Send + Sync + 'static>>) -> Result<(), Report> {
    err.map_err(|e| miette!(e))
        .wrap_err("saw a downstream error")
}
```

# Effect on Downcasting

After attaching a message of type `D` onto an error of type `E`, the
resulting `miette::Error` may be downcast to `D` **or** to `E`.

That is, in codebases that rely on downcasting, `miette`'s `wrap_err()`
supports both of the following use cases:

  - **Attaching messages whose type is insignificant onto errors whose type
    is used in downcasts.**

    In other error libraries whose `wrap_err()` is not designed this way, it
    can be risky to introduce messages to existing code because new message
    might break existing working downcasts. In miette, any downcast that
    worked before adding the message will continue to work after you add a
    message, so you should freely wrap errors wherever it would be helpful.

    ```rust
    use miette::bail;
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("???")]
    struct SuspiciousError;

    fn helper() -> Result<()> {
        bail!(SuspiciousError);
    }

    use miette::{WrapErr, Result};

    fn do_it() -> Result<()> {
        helper().wrap_err("Failed to complete the work")?;
        const IGNORE: &str = stringify! {
        ...
        };
        unreachable!()
    }

    fn main() {
        let err = do_it().unwrap_err();
        if let Some(e) = err.downcast_ref::<SuspiciousError>() {
            // If helper() returned SuspiciousError, this downcast will
            // correctly succeed even with the message in between.
            return;
        }
        panic!("expected downcast to succeed");
    }
    ```

  - **Attaching message whose type is used in downcasts onto errors whose
    type is insignificant.**

    Some codebases prefer to use machine-readable messages to categorize
    lower level errors in a way that will be actionable to higher levels of
    the application.

    ```rust
    use miette::bail;
    use thiserror::Error;

    #[derive(Error, Debug)]
    #[error("???")]
    struct HelperFailed;

    fn helper() -> Result<()> {
        bail!("no such file or directory");
    }

    use miette::{WrapErr, Result};

    fn do_it() -> Result<()> {
        helper().wrap_err(HelperFailed)?;
        const IGNORE: &str = stringify! {
        ...
        };
        unreachable!()
    }

    fn main() {
        let err = do_it().unwrap_err();
        if let Some(e) = err.downcast_ref::<HelperFailed>() {
            // If helper failed, this downcast will succeed because
            // HelperFailed is the message that has been attached to
            // that error.
            return;
        }
        panic!("expected downcast to succeed");
    }
    ```

#### Required Methods

- `fn wrap_err<D>(self, msg: D) -> Result<T, Report>`

  Wrap the error value with a new adhoc error

- `fn wrap_err_with<D, F>(self, f: F) -> Result<T, Report>`

  Wrap the error value with a new adhoc error that is evaluated lazily

- `fn context<D>(self, msg: D) -> Result<T, Report>`

  Compatibility re-export of `wrap_err()` for interop with `anyhow`

- `fn with_context<D, F>(self, f: F) -> Result<T, Report>`

  Compatibility re-export of `wrap_err_with()` for interop with `anyhow`

#### Implementors

- `Option<T>`
- `Result<T, E>`

### `Diagnostic`

```rust
trait Diagnostic: std::error::Error { ... }
```

*Defined in [`miette-7.6.0/src/protocol.rs:20-70`](../../.source_1765210505/miette-7.6.0/src/protocol.rs#L20-L70)*

Adds rich metadata to your Error that can be used by
[`Report`](crate::Report) to print really nice and human-friendly error
messages.

#### Provided Methods

- `fn code<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

  Unique diagnostic code that can be used to look up more information

- `fn severity(&self) -> Option<Severity>`

  Diagnostic severity. This may be used by

- `fn help<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

  Additional help text related to this `Diagnostic`. Do you have any

- `fn url<'a>(self: &'a Self) -> Option<Box<dyn Display>>`

  URL to visit for a more detailed explanation/help about this

- `fn source_code(&self) -> Option<&dyn SourceCode>`

  Source code to apply this `Diagnostic`'s `Diagnostic::labels` to.

- `fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>>`

  Labels to apply to this `Diagnostic`'s `Diagnostic::source_code`

- `fn related<'a>(self: &'a Self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic>>>`

  Additional related `Diagnostic`s.

- `fn diagnostic_source(&self) -> Option<&dyn Diagnostic>`

  The cause of the error.

#### Implementors

- [`BoxedError`](eyreish/wrapper/index.md#boxederror)
- [`ContextError`](eyreish/error/index.md#contexterror)
- [`DiagnosticError`](eyreish/into_diagnostic/index.md#diagnosticerror)
- [`DisplayError`](eyreish/wrapper/index.md#displayerror)
- [`InstallError`](#installerror)
- [`MessageError`](eyreish/wrapper/index.md#messageerror)
- [`MietteDiagnostic`](#miettediagnostic)
- [`MietteError`](#mietteerror)
- [`Panic`](panic/index.md#panic)
- [`WithSourceCode`](eyreish/wrapper/index.md#withsourcecode)
- `std::convert::Infallible`

### `SourceCode`

```rust
trait SourceCode: Send + Sync { ... }
```

*Defined in [`miette-7.6.0/src/protocol.rs:236-245`](../../.source_1765210505/miette-7.6.0/src/protocol.rs#L236-L245)*

Represents readable source code of some sort.

This trait is able to support simple `SourceCode` types like [`String`](../clap_builder/index.md)s, as
well as more involved types like indexes into centralized `SourceMap`-like
types, file handles, and even network streams.

If you can read it, you can source it, and it's not necessary to read the
whole thing--meaning you should be able to support `SourceCode`s which are
gigabytes or larger in size.

#### Required Methods

- `fn read_span<'a>(self: &'a Self, span: &SourceSpan, context_lines_before: usize, context_lines_after: usize) -> Result<Box<dyn SpanContents<'a>>, MietteError>`

  Read the bytes for a specific span from this `SourceCode`, keeping a

#### Implementors

- [`NamedSource`](#namedsource)
- `&[u8]`
- `&str`
- `String`
- `Vec<u8>`
- `[u8]`
- `std::borrow::Cow<'_, T>`
- `std::sync::Arc<T>`
- `str`

### `SpanContents<'a>`

```rust
trait SpanContents<'a> { ... }
```

*Defined in [`miette-7.6.0/src/protocol.rs:426-452`](../../.source_1765210505/miette-7.6.0/src/protocol.rs#L426-L452)*

Contents of a [`SourceCode`](#sourcecode) covered by [`SourceSpan`](#sourcespan).

Includes line and column information to optimize highlight calculations.

#### Required Methods

- `fn data(&self) -> &'a [u8]`

  Reference to the data inside the associated span, in bytes.

- `fn span(&self) -> &SourceSpan`

  [`SourceSpan`](#sourcespan) representing the span covered by this `SpanContents`.

- `fn line(&self) -> usize`

  The 0-indexed line in the associated [`SourceCode`](#sourcecode) where the data

- `fn column(&self) -> usize`

  The 0-indexed column in the associated [`SourceCode`](#sourcecode) where the data

- `fn line_count(&self) -> usize`

  Total number of lines covered by this `SpanContents`.

#### Provided Methods

- `fn name(&self) -> Option<&str>`

  An optional (file?) name for the container of this `SpanContents`.

- `fn language(&self) -> Option<&str>`

  Optional method. The language name for this source code, if any.

#### Implementors

- [`MietteSpanContents`](#miettespancontents)

## Functions

### `set_hook`

```rust
fn set_hook(hook: ErrorHook) -> Result<(), InstallError>
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:83-85`](../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L83-L85)*

Set the error hook.

### `capture_handler`

```rust
fn capture_handler(error: &dyn Diagnostic) -> Box<dyn ReportHandler>
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:89-102`](../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L89-L102)*

### `get_default_printer`

```rust
fn get_default_printer(_err: &dyn Diagnostic) -> Box<dyn ReportHandler>
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:104-109`](../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L104-L109)*

### `set_panic_hook`

```rust
fn set_panic_hook()
```

*Defined in [`miette-7.6.0/src/panic.rs:8-27`](../../.source_1765210505/miette-7.6.0/src/panic.rs#L8-L27)*

Tells miette to render panics using its rendering engine.

## Type Aliases

### `ErrorHook`

```rust
type ErrorHook = Box<dyn Fn(&dyn Diagnostic) -> Box<dyn ReportHandler> + Sync + Send>;
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:61-62`](../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L61-L62)*

### `Result<T, E>`

```rust
type Result<T, E> = core::result::Result<T, E>;
```

*Defined in [`miette-7.6.0/src/eyreish/mod.rs:257`](../../.source_1765210505/miette-7.6.0/src/eyreish/mod.rs#L257)*

type alias for `Result<T, Report>`

This is a reasonable return type to use throughout your application but also
for `main()`. If you do, failures will be printed along with a backtrace if
one was captured.

`miette::Result` may be used with one *or* two type parameters.

```rust
use miette::Result;

const IGNORE: &str = stringify! {
fn demo1() -> Result<T> {...}
           // ^ equivalent to std::result::Result<T, miette::Error>

fn demo2() -> Result<T, OtherError> {...}
           // ^ equivalent to std::result::Result<T, OtherError>
};
```

# Example

```rust
pub trait Deserialize {}

mod serde_json {
    use super::Deserialize;
    use std::io;

    pub fn from_str<T: Deserialize>(json: &str) -> io::Result<T> {
        unimplemented!()
    }
}

#[derive(Debug)]
struct ClusterMap;

impl Deserialize for ClusterMap {}

use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    return Ok(());
    let config = std::fs::read_to_string("cluster.json").into_diagnostic()?;
    let map: ClusterMap = serde_json::from_str(&config).into_diagnostic()?;
    println!("cluster info: {:#?}", map);
    Ok(())
}
```

## `anyhow`/`eyre` Users

You can just replace `use`s of `anyhow::Result`/`eyre::Result` with
`miette::Result`.

### `ByteOffset`

```rust
type ByteOffset = usize;
```

*Defined in [`miette-7.6.0/src/protocol.rs:666`](../../.source_1765210505/miette-7.6.0/src/protocol.rs#L666)*

"Raw" type for the byte offset from the beginning of a [`SourceCode`](#sourcecode).

## Macros

### `bail!`

*Defined in [`miette-7.6.0/src/eyreish/macros.rs:80-89`](../../.source_1765210505/miette-7.6.0/src/eyreish/macros.rs#L80-L89)*

Return early with an error.

This macro is equivalent to `return Err(From::from($err))`.

# Example

```rust
use miette::{bail, Result};

fn has_permission(user: usize, resource: usize) -> bool {
    true
}

fn main() -> Result<()> {
    let user = 0;
    let resource = 0;

if !has_permission(user, resource) {
     bail!("permission denied for accessing {resource}");
}
    Ok(())
}
```

```rust
use miette::{bail, Result};
use thiserror::Error;

const MAX_DEPTH: usize = 1;

#[derive(Error, Debug)]
enum ScienceError {
    #[error("recursion limit exceeded")]
    RecursionLimitExceeded,
    #[error("...")]
    More = (stringify! {
    ...
    }, 1).1,
}

fn main() -> Result<()> {
    let depth = 0;
    let err: &'static dyn std::error::Error = &ScienceError::RecursionLimitExceeded;

if depth > MAX_DEPTH {
    bail!(ScienceError::RecursionLimitExceeded);
}
    Ok(())
}
```

```rust
use miette::{bail, Result, Severity};

fn divide(x: f64, y: f64) -> Result<f64> {
    if y.abs() < 1e-3 {
        bail!(
            severity = Severity::Warning,
             "dividing by value ({y}) close to 0"
        );
    }
    Ok(x / y)
}
```

### `ensure!`

*Defined in [`miette-7.6.0/src/eyreish/macros.rs:156-169`](../../.source_1765210505/miette-7.6.0/src/eyreish/macros.rs#L156-L169)*

Return early with an error if a condition is not satisfied.

This macro is equivalent to `if !$cond { return Err(From::from($err)); }`.

Analogously to `assert!`, `ensure!` takes a condition and exits the function
if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`
rather than panicking.

# Example

```rust
use miette::{ensure, Result};

fn main() -> Result<()> {
    let user = 0;

ensure!(user == 0, "only user 0 is allowed");
    Ok(())
}
```

```rust
use miette::{ensure, Result};
use thiserror::Error;

const MAX_DEPTH: usize = 1;

#[derive(Error, Debug)]
enum ScienceError {
    #[error("recursion limit exceeded")]
    RecursionLimitExceeded,
    #[error("...")]
    More = (stringify! {
    ...
    }, 1).1,
}

fn main() -> Result<()> {
    let depth = 0;

ensure!(depth <= MAX_DEPTH, ScienceError::RecursionLimitExceeded);
    Ok(())
}
```

```rust
use miette::{ensure, Result, Severity};

fn divide(x: f64, y: f64) -> Result<f64> {
    ensure!(
        y.abs() >= 1e-3,
        severity = Severity::Warning,
             "dividing by value ({y}) close to 0"
    );
    Ok(x / y)
}
```

### `miette!`

*Defined in [`miette-7.6.0/src/eyreish/macros.rs:229-240`](../../.source_1765210505/miette-7.6.0/src/eyreish/macros.rs#L229-L240)*

Construct an ad-hoc [`Report`](#report).

# Examples

With string literal and interpolation:
```rust
use miette::miette;
let x = 1;
let y = 2;
let report = miette!("{x} + {} = {z}", y, z = x + y);

assert_eq!(report.to_string().as_str(), "1 + 2 = 3");

let z = x + y;
let report = miette!("{x} + {y} = {z}");
assert_eq!(report.to_string().as_str(), "1 + 2 = 3");
```

With `diagnostic!`-like arguments:
```rust
use miette::{miette, LabeledSpan, Severity};

let source = "(2 + 2".to_string();
let report = miette!(
    // Those fields are optional
    severity = Severity::Error,
    code = "expected::rparen",
    help = "always close your parens",
    labels = vec![LabeledSpan::at_offset(6, "here")],
    url = "https://example.com",
    // Rest of the arguments are passed to `format!`
    // to form diagnostic message
    "expected closing ')'"
)
.with_source_code(source);
```

## `anyhow`/`eyre` Users

You can just replace `use`s of the `anyhow!`/`eyre!` macros with `miette!`.



### `diagnostic!`

*Defined in [`miette-7.6.0/src/eyreish/macros.rs:291-300`](../../.source_1765210505/miette-7.6.0/src/eyreish/macros.rs#L291-L300)*

Construct a [`MietteDiagnostic`](#miettediagnostic) in more user-friendly way.

# Examples
```rust
use miette::{diagnostic, LabeledSpan, Severity};

let source = "(2 + 2".to_string();
let diag = diagnostic!(
    // Those fields are optional
    severity = Severity::Error,
    code = "expected::rparen",
    help = "always close your parens",
    labels = vec![LabeledSpan::at_offset(6, "here")],
    url = "https://example.com",
    // Rest of the arguments are passed to `format!`
    // to form diagnostic message
    "expected closing ')'",
);
```
Diagnostic without any fields:
```rust
use miette::diagnostic;
let x = 1;
let y = 2;

 let diag = diagnostic!("{x} + {} = {z}", y, z = x + y);
assert_eq!(diag.message, "1 + 2 = 3");

let z = x + y;
let diag = diagnostic!("{x} + {y} = {z}");
assert_eq!(diag.message, "1 + 2 = 3");
```


### `box_error_impls!`

*Defined in [`miette-7.6.0/src/protocol.rs:72-86`](../../.source_1765210505/miette-7.6.0/src/protocol.rs#L72-L86)*

### `box_borrow_impls!`

*Defined in [`miette-7.6.0/src/protocol.rs:94-104`](../../.source_1765210505/miette-7.6.0/src/protocol.rs#L94-L104)*

