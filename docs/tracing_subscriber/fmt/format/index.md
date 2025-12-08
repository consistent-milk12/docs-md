*[tracing_subscriber](../../index.md) / [fmt](../index.md) / [format](index.md)*

---

# Module `format`

Formatters for logging [`tracing`](../../../tracing/index.md) events.

This module provides several formatter implementations, as well as utilities
for implementing custom formatters.

# Formatters
This module provides a number of formatter implementations:

* [`Full`](#full): The default formatter. This emits human-readable,
  single-line logs for each event that occurs, with the current span context
  displayed before the formatted representation of the event. See
  [here](Full#example-output) for sample output.

* [`Compact`](#compact): A variant of the default formatter, optimized for
  short line lengths. Fields from the current span context are appended to
  the fields of the formatted event, and span names are not shown; the
  verbosity level is abbreviated to a single character. See
  [here](Compact#example-output) for sample output.

* [`Pretty`](#pretty): Emits excessively pretty, multi-line logs, optimized
  for human readability. This is primarily intended to be used in local
  development and debugging, or for command-line applications, where
  automated analysis and compact storage of logs is less of a priority than
  readability and visual appeal. See [here](Pretty#example-output)
  for sample output.

* `Json`: Outputs newline-delimited JSON logs. This is intended
  for production use with systems where structured logs are consumed as JSON
  by analysis and viewing tools. The JSON output is not optimized for human
  readability. See [here](Json#example-output) for sample output.

## Structs

### `Writer<'writer>`

```rust
struct Writer<'writer> {
    writer: &'writer mut dyn fmt::Write,
    is_ansi: bool,
}
```

A writer to which formatted representations of spans and events are written.

This type is provided as input to the `FormatEvent::format_event` and
`FormatFields::format_fields` methods, which will write formatted
representations of [`Event`](../../../tracing_core/event/index.md)s and [`fields`](../../../tracing_attributes/attr/kw/index.md) to the [`Writer`](#writer).

This type implements the [`std::fmt::Write`](../../../fs_err/index.md) trait, allowing it to be used
with any function that takes an instance of [`std::fmt::Write`](../../../fs_err/index.md).
Additionally, it can be used with the standard library's [`std::write!`](../../../nu_ansi_term/write/index.md) and
`std::writeln!` macros.

Additionally, a [`Writer`](#writer) may expose additional [`tracing`](../../../tracing/index.md)-specific
information to the formatter implementation.


#### Implementations

- `fn new(writer: &'writer mut impl fmt::Write) -> Self`

- `fn with_ansi(self: Self, is_ansi: bool) -> Self`

- `fn by_ref(self: &mut Self) -> Writer<'_>` — [`Writer`](#writer)

- `fn write_str(self: &mut Self, s: &str) -> fmt::Result`

- `fn write_char(self: &mut Self, c: char) -> fmt::Result`

- `fn write_fmt(self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result`

- `fn has_ansi_escapes(self: &Self) -> bool`

- `fn bold(self: &Self) -> Style`

- `fn dimmed(self: &Self) -> Style`

- `fn italic(self: &Self) -> Style`

#### Trait Implementations

##### `impl Debug for Writer<'_>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for Writer<'writer>`

##### `impl<T> WithSubscriber for Writer<'writer>`

##### `impl Write for Writer<'_>`

- `fn write_str(self: &mut Self, s: &str) -> fmt::Result`

- `fn write_char(self: &mut Self, c: char) -> fmt::Result`

- `fn write_fmt(self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result`

### `FieldFn<F>`

```rust
struct FieldFn<F>(F);
```

A [`FormatFields`](#formatfields) implementation that formats fields by calling a function
or closure.


#### Trait Implementations

##### `impl<F: $crate::clone::Clone> Clone for FieldFn<F>`

- `fn clone(self: &Self) -> FieldFn<F>` — [`FieldFn`](#fieldfn)

##### `impl<F: $crate::fmt::Debug> Debug for FieldFn<F>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for FieldFn<F>`

##### `impl<T, M> MakeExt for FieldFn<F>`

##### `impl<'a, F> MakeVisitor for FieldFn<F>`

- `type Visitor = FieldFnVisitor<'a, F>`

- `fn make_visitor(self: &Self, writer: Writer<'a>) -> <Self as >::Visitor` — [`Writer`](#writer), [`MakeVisitor`](../../field/index.md)

##### `impl<T, M> Sealed for FieldFn<F>`

##### `impl<T> WithSubscriber for FieldFn<F>`

### `FieldFnVisitor<'a, F>`

```rust
struct FieldFnVisitor<'a, F> {
    f: F,
    writer: Writer<'a>,
    result: fmt::Result,
}
```

The [`visitor`](../../../regex_syntax/ast/visitor/index.md) produced by [`FieldFn`](#fieldfn)'s [`MakeVisitor`](../../field/index.md) implementation.



#### Trait Implementations

##### `impl<F> Debug for FieldFnVisitor<'_, F>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for FieldFnVisitor<'a, F>`

##### `impl<'a, F> Visit for FieldFnVisitor<'a, F>`

- `fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug)`

##### `impl<'a, F> VisitFmt for FieldFnVisitor<'a, F>`

- `fn writer(self: &mut Self) -> &mut dyn fmt::Write`

##### `impl<'a, F> VisitOutput for FieldFnVisitor<'a, F>`

- `fn finish(self: Self) -> fmt::Result`

##### `impl<T> WithSubscriber for FieldFnVisitor<'a, F>`

### `Compact`

```rust
struct Compact;
```

Marker for [`Format`](#format) that indicates that the compact log format should be used.

The compact format includes fields from all currently entered spans, after
the event's fields. Span fields are ordered (but not grouped) by
span, and span names are not shown. A more compact representation of the
event's [`Level`](../../../tracing_core/metadata/index.md) is used, and additional information—such as the event's
target—is disabled by default.

# Example Output

<pre><font color="#4E9A06"><b>:;</b></font> <font color="#4E9A06">cargo</font> run --example fmt-compact
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/fmt-compact`
<font color="#AAAAAA">2022-02-17T19:51:05.809287Z </font><font color="#4E9A06"> INFO</font> <b>fmt_compact</b><font color="#AAAAAA">: preparing to shave yaks </font><i>number_of_yaks</i><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809367Z </font><font color="#4E9A06"> INFO</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: shaving yaks </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809414Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809443Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: yak shaved successfully </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809477Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=1 </font><i>shaved</i><font color="#AAAAAA">=true </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809500Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=1 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809531Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809554Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: yak shaved successfully </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809581Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=2 </font><i>shaved</i><font color="#AAAAAA">=true </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809606Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=2 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809635Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot; </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809664Z </font><font color="#C4A000"> WARN</font> <b>shaving_yaks</b>:<b>shave</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: could not locate yak </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3 </font><font color="#AAAAAA"><i>yak</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809693Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks</b>: <b>yak_events</b><font color="#AAAAAA">: </font><i>yak</i><font color="#AAAAAA">=3 </font><i>shaved</i><font color="#AAAAAA">=false </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809717Z </font><font color="#CC0000">ERROR</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: failed to shave yak </font><i>yak</i><font color="#AAAAAA">=3 </font><i>error</i><font color="#AAAAAA">=missing yak </font><i>error.sources</i><font color="#AAAAAA">=[out of space, out of cash] </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809743Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks</b>: <b>fmt_compact::yak_shave</b><font color="#AAAAAA">: </font><i>yaks_shaved</i><font color="#AAAAAA">=2 </font><font color="#AAAAAA"><i>yaks</i></font><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-17T19:51:05.809768Z </font><font color="#4E9A06"> INFO</font> <b>fmt_compact</b><font color="#AAAAAA">: yak shaving completed </font><i>all_yaks_shaved</i><font color="#AAAAAA">=false</font>

</pre>

#### Trait Implementations

##### `impl Clone for Compact`

- `fn clone(self: &Self) -> Compact` — [`Compact`](#compact)

##### `impl Copy for Compact`

##### `impl Debug for Compact`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Compact`

- `fn default() -> Compact` — [`Compact`](#compact)

##### `impl Eq for Compact`

##### `impl<T> Instrument for Compact`

##### `impl PartialEq for Compact`

- `fn eq(self: &Self, other: &Compact) -> bool` — [`Compact`](#compact)

##### `impl StructuralPartialEq for Compact`

##### `impl<T> WithSubscriber for Compact`

### `Full`

```rust
struct Full;
```

Marker for [`Format`](#format) that indicates that the default log format should be used.

This formatter shows the span context before printing event data. Spans are
displayed including their names and fields.

# Example Output

<pre><font color="#4E9A06"><b>:;</b></font> <font color="#4E9A06">cargo</font> run --example fmt
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/fmt`
<font color="#AAAAAA">2022-02-15T18:40:14.289898Z </font><font color="#4E9A06"> INFO</font> fmt: preparing to shave yaks <i>number_of_yaks</i><font color="#AAAAAA">=3</font>
<font color="#AAAAAA">2022-02-15T18:40:14.289974Z </font><font color="#4E9A06"> INFO</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: shaving yaks</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290011Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=1</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290038Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=1</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: yak shaved successfully</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290070Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=1 </font><i>shaved</i><font color="#AAAAAA">=true</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290089Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=1</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290114Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=2</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290134Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=2</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: yak shaved successfully</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290157Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=2 </font><i>shaved</i><font color="#AAAAAA">=true</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290174Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290198Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: hello! I&apos;m gonna shave a yak </font><i>excitement</i><font color="#AAAAAA">=&quot;yay!&quot;</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290222Z </font><font color="#C4A000"> WARN</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">:</font><b>shave{</b><i>yak</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: could not locate yak</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290247Z </font><font color="#3465A4">DEBUG</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: yak_events: </font><i>yak</i><font color="#AAAAAA">=3 </font><i>shaved</i><font color="#AAAAAA">=false</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290268Z </font><font color="#CC0000">ERROR</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: failed to shave yak </font><i>yak</i><font color="#AAAAAA">=3 </font><i>error</i><font color="#AAAAAA">=missing yak </font><i>error.sources</i><font color="#AAAAAA">=[out of space, out of cash]</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290287Z </font><font color="#75507B">TRACE</font> <b>shaving_yaks{</b><i>yaks</i><font color="#AAAAAA">=3</font><b>}</b><font color="#AAAAAA">: fmt::yak_shave: </font><i>yaks_shaved</i><font color="#AAAAAA">=2</font>
<font color="#AAAAAA">2022-02-15T18:40:14.290309Z </font><font color="#4E9A06"> INFO</font> fmt: yak shaving completed. <i>all_yaks_shaved</i><font color="#AAAAAA">=false</font>
</pre>

#### Trait Implementations

##### `impl Clone for Full`

- `fn clone(self: &Self) -> Full` — [`Full`](#full)

##### `impl Copy for Full`

##### `impl Debug for Full`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Full`

- `fn default() -> Full` — [`Full`](#full)

##### `impl Eq for Full`

##### `impl<T> Instrument for Full`

##### `impl PartialEq for Full`

- `fn eq(self: &Self, other: &Full) -> bool` — [`Full`](#full)

##### `impl StructuralPartialEq for Full`

##### `impl<T> WithSubscriber for Full`

### `Format<F, T>`

```rust
struct Format<F, T> {
    format: F,
    timer: T,
    ansi: Option<bool>,
    display_timestamp: bool,
    display_target: bool,
    display_level: bool,
    display_thread_id: bool,
    display_thread_name: bool,
    display_filename: bool,
    display_line_number: bool,
}
```

A pre-configured event formatter.

You will usually want to use this as the [`FormatEvent`](#formatevent) for a [`FmtSubscriber`](../../index.md).

The default logging format, [`Full`](#full) includes all fields in each event and its containing
spans. The [`Compact`](#compact) logging format is intended to produce shorter log
lines; it displays each event's fields, along with fields from the current
span context, but other information is abbreviated. The [`Pretty`](#pretty) logging
format is an extra-verbose, multi-line human-readable logging format
intended for use in development.


#### Implementations

- `fn compact(self: Self) -> Format<Compact, T>` — [`Format`](#format), [`Compact`](#compact)

- `fn pretty(self: Self) -> Format<Pretty, T>` — [`Format`](#format), [`Pretty`](#pretty)

- `fn with_timer<T2>(self: Self, timer: T2) -> Format<F, T2>` — [`Format`](#format)

- `fn without_time(self: Self) -> Format<F, ()>` — [`Format`](#format)

- `fn with_ansi(self: Self, ansi: bool) -> Format<F, T>` — [`Format`](#format)

- `fn with_target(self: Self, display_target: bool) -> Format<F, T>` — [`Format`](#format)

- `fn with_level(self: Self, display_level: bool) -> Format<F, T>` — [`Format`](#format)

- `fn with_thread_ids(self: Self, display_thread_id: bool) -> Format<F, T>` — [`Format`](#format)

- `fn with_thread_names(self: Self, display_thread_name: bool) -> Format<F, T>` — [`Format`](#format)

- `fn with_file(self: Self, display_filename: bool) -> Format<F, T>` — [`Format`](#format)

- `fn with_line_number(self: Self, display_line_number: bool) -> Format<F, T>` — [`Format`](#format)

- `fn with_source_location(self: Self, display_location: bool) -> Self`

- `fn format_timestamp(self: &Self, writer: &mut Writer<'_>) -> fmt::Result` — [`Writer`](#writer)

#### Trait Implementations

##### `impl<F: $crate::clone::Clone, T: $crate::clone::Clone> Clone for Format<F, T>`

- `fn clone(self: &Self) -> Format<F, T>` — [`Format`](#format)

##### `impl<F: $crate::fmt::Debug, T: $crate::fmt::Debug> Debug for Format<F, T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Format<Full, super::time::SystemTime>`

- `fn default() -> Self`

##### `impl<C, N, T> FormatEvent for Format<Pretty, T>`

- `fn format_event(self: &Self, ctx: &FmtContext<'_, C, N>, writer: Writer<'_>, event: &Event<'_>) -> fmt::Result` — [`FmtContext`](../fmt_layer/index.md), [`Writer`](#writer)

##### `impl<T> Instrument for Format<F, T>`

##### `impl<T> WithSubscriber for Format<F, T>`

### `DefaultFields`

```rust
struct DefaultFields {
    _private: (),
}
```

The default [`FormatFields`](#formatfields) implementation.


#### Implementations

- `fn new() -> Self`

#### Trait Implementations

##### `impl Debug for DefaultFields`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for DefaultFields`

- `fn default() -> Self`

##### `impl<T> Instrument for DefaultFields`

##### `impl<T, M> MakeExt for DefaultFields`

##### `impl<'a> MakeVisitor for DefaultFields`

- `type Visitor = DefaultVisitor<'a>`

- `fn make_visitor(self: &Self, target: Writer<'a>) -> <Self as >::Visitor` — [`Writer`](#writer), [`MakeVisitor`](../../field/index.md)

##### `impl<T, M> Sealed for DefaultFields`

##### `impl<T> WithSubscriber for DefaultFields`

### `DefaultVisitor<'a>`

```rust
struct DefaultVisitor<'a> {
    writer: Writer<'a>,
    is_empty: bool,
    result: fmt::Result,
}
```

The [`visitor`](../../../regex_syntax/ast/visitor/index.md) produced by [`DefaultFields`](#defaultfields)'s [`MakeVisitor`](../../field/index.md) implementation.



#### Implementations

- `fn new(writer: Writer<'a>, is_empty: bool) -> Self` — [`Writer`](#writer)

- `fn maybe_pad(self: &mut Self)`

#### Trait Implementations

##### `impl<'a> Debug for DefaultVisitor<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for DefaultVisitor<'a>`

##### `impl Visit for DefaultVisitor<'_>`

- `fn record_str(self: &mut Self, field: &Field, value: &str)`

- `fn record_error(self: &mut Self, field: &Field, value: &dyn std::error::Error)`

- `fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug)`

##### `impl VisitFmt for DefaultVisitor<'_>`

- `fn writer(self: &mut Self) -> &mut dyn fmt::Write`

##### `impl VisitOutput for DefaultVisitor<'_>`

- `fn finish(self: Self) -> fmt::Result`

##### `impl<T> WithSubscriber for DefaultVisitor<'a>`

### `FmtSpan`

```rust
struct FmtSpan(u8);
```

Configures what points in the span lifecycle are logged as events.

See also `with_span_events`.


#### Implementations

- `const NEW: FmtSpan`

- `const ENTER: FmtSpan`

- `const EXIT: FmtSpan`

- `const CLOSE: FmtSpan`

- `const NONE: FmtSpan`

- `const ACTIVE: FmtSpan`

- `const FULL: FmtSpan`

- `fn contains(self: &Self, other: FmtSpan) -> bool` — [`FmtSpan`](#fmtspan)

#### Trait Implementations

##### `impl BitAnd for FmtSpan`

- `type Output = FmtSpan`

- `fn bitand(self: Self, rhs: Self) -> <Self as >::Output`

##### `impl BitAndAssign for FmtSpan`

- `fn bitand_assign(self: &mut Self, rhs: Self)`

##### `impl BitOr for FmtSpan`

- `type Output = FmtSpan`

- `fn bitor(self: Self, rhs: Self) -> <Self as >::Output`

##### `impl BitOrAssign for FmtSpan`

- `fn bitor_assign(self: &mut Self, rhs: Self)`

##### `impl BitXor for FmtSpan`

- `type Output = FmtSpan`

- `fn bitxor(self: Self, rhs: Self) -> <Self as >::Output`

##### `impl BitXorAssign for FmtSpan`

- `fn bitxor_assign(self: &mut Self, rhs: Self)`

##### `impl Clone for FmtSpan`

- `fn clone(self: &Self) -> FmtSpan` — [`FmtSpan`](#fmtspan)

##### `impl Debug for FmtSpan`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FmtSpan`

##### `impl<T> Instrument for FmtSpan`

##### `impl Ord for FmtSpan`

- `fn cmp(self: &Self, other: &FmtSpan) -> $crate::cmp::Ordering` — [`FmtSpan`](#fmtspan)

##### `impl PartialEq for FmtSpan`

- `fn eq(self: &Self, other: &FmtSpan) -> bool` — [`FmtSpan`](#fmtspan)

##### `impl PartialOrd for FmtSpan`

- `fn partial_cmp(self: &Self, other: &FmtSpan) -> $crate::option::Option<$crate::cmp::Ordering>` — [`FmtSpan`](#fmtspan)

##### `impl StructuralPartialEq for FmtSpan`

##### `impl<T> WithSubscriber for FmtSpan`

### `Pretty`

```rust
struct Pretty {
    display_location: bool,
}
```

An excessively pretty, human-readable event formatter.

Unlike the [`Full`](#full), [`Compact`](#compact), and `Json` formatters, this is a
multi-line output format. Each individual event may output multiple lines of
text.

# Example Output

<pre><font color="#4E9A06"><b>:;</b></font> <font color="#4E9A06">cargo</font> run --example fmt-pretty
<font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
<font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/fmt-pretty`
  2022-02-15T18:44:24.535324Z <font color="#4E9A06"> INFO</font> <font color="#4E9A06"><b>fmt_pretty</b></font><font color="#4E9A06">: preparing to shave yaks, </font><font color="#4E9A06"><b>number_of_yaks</b></font><font color="#4E9A06">: 3</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt-pretty.rs:16 <font color="#AAAAAA"><i>on</i></font> main

  2022-02-15T18:44:24.535403Z <font color="#4E9A06"> INFO</font> <font color="#4E9A06"><b>fmt_pretty::yak_shave</b></font><font color="#4E9A06">: shaving yaks</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:41 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535442Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: hello! I&apos;m gonna shave a yak, </font><font color="#75507B"><b>excitement</b></font><font color="#75507B">: &quot;yay!&quot;</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:16 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 1
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535469Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: yak shaved successfully</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:25 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 1
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535502Z <font color="#3465A4">DEBUG</font> <font color="#3465A4"><b>yak_events</b></font><font color="#3465A4">: </font><font color="#3465A4"><b>yak</b></font><font color="#3465A4">: 1, </font><font color="#3465A4"><b>shaved</b></font><font color="#3465A4">: true</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:46 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535524Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: </font><font color="#75507B"><b>yaks_shaved</b></font><font color="#75507B">: 1</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:55 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535551Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: hello! I&apos;m gonna shave a yak, </font><font color="#75507B"><b>excitement</b></font><font color="#75507B">: &quot;yay!&quot;</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:16 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 2
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535573Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: yak shaved successfully</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:25 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 2
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535600Z <font color="#3465A4">DEBUG</font> <font color="#3465A4"><b>yak_events</b></font><font color="#3465A4">: </font><font color="#3465A4"><b>yak</b></font><font color="#3465A4">: 2, </font><font color="#3465A4"><b>shaved</b></font><font color="#3465A4">: true</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:46 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535618Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: </font><font color="#75507B"><b>yaks_shaved</b></font><font color="#75507B">: 2</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:55 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535644Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: hello! I&apos;m gonna shave a yak, </font><font color="#75507B"><b>excitement</b></font><font color="#75507B">: &quot;yay!&quot;</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:16 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 3
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535670Z <font color="#C4A000"> WARN</font> <font color="#C4A000"><b>fmt_pretty::yak_shave</b></font><font color="#C4A000">: could not locate yak</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:18 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shave</b> <font color="#AAAAAA"><i>with</i></font> <b>yak</b>: 3
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535698Z <font color="#3465A4">DEBUG</font> <font color="#3465A4"><b>yak_events</b></font><font color="#3465A4">: </font><font color="#3465A4"><b>yak</b></font><font color="#3465A4">: 3, </font><font color="#3465A4"><b>shaved</b></font><font color="#3465A4">: false</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:46 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535720Z <font color="#CC0000">ERROR</font> <font color="#CC0000"><b>fmt_pretty::yak_shave</b></font><font color="#CC0000">: failed to shave yak, </font><font color="#CC0000"><b>yak</b></font><font color="#CC0000">: 3, </font><font color="#CC0000"><b>error</b></font><font color="#CC0000">: missing yak, </font><font color="#CC0000"><b>error.sources</b></font><font color="#CC0000">: [out of space, out of cash]</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:51 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535742Z <font color="#75507B">TRACE</font> <font color="#75507B"><b>fmt_pretty::yak_shave</b></font><font color="#75507B">: </font><font color="#75507B"><b>yaks_shaved</b></font><font color="#75507B">: 2</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt/yak_shave.rs:55 <font color="#AAAAAA"><i>on</i></font> main
    <font color="#AAAAAA"><i>in</i></font> fmt_pretty::yak_shave::<b>shaving_yaks</b> <font color="#AAAAAA"><i>with</i></font> <b>yaks</b>: 3

  2022-02-15T18:44:24.535765Z <font color="#4E9A06"> INFO</font> <font color="#4E9A06"><b>fmt_pretty</b></font><font color="#4E9A06">: yak shaving completed, </font><font color="#4E9A06"><b>all_yaks_shaved</b></font><font color="#4E9A06">: false</font>
    <font color="#AAAAAA"><i>at</i></font> examples/examples/fmt-pretty.rs:19 <font color="#AAAAAA"><i>on</i></font> main
</pre>

#### Implementations

- `fn style_for(level: &Level) -> Style`

- `fn with_source_location(self: Self, display_location: bool) -> Self`

#### Trait Implementations

##### `impl Clone for Pretty`

- `fn clone(self: &Self) -> Pretty` — [`Pretty`](#pretty)

##### `impl Debug for Pretty`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Pretty`

- `fn default() -> Self`

##### `impl Eq for Pretty`

##### `impl<'writer> FormatFields for Pretty`

- `fn format_fields<R: RecordFields>(self: &Self, writer: Writer<'writer>, fields: R) -> fmt::Result` — [`Writer`](#writer)

- `fn add_fields(self: &Self, current: &'writer mut FormattedFields<Self>, fields: &span::Record<'_>) -> fmt::Result` — [`FormattedFields`](../fmt_layer/index.md)

##### `impl<T> Instrument for Pretty`

##### `impl PartialEq for Pretty`

- `fn eq(self: &Self, other: &Pretty) -> bool` — [`Pretty`](#pretty)

##### `impl StructuralPartialEq for Pretty`

##### `impl<T> WithSubscriber for Pretty`

### `PrettyVisitor<'a>`

```rust
struct PrettyVisitor<'a> {
    writer: Writer<'a>,
    is_empty: bool,
    style: nu_ansi_term::Style,
    result: fmt::Result,
}
```

The [`visitor`](../../../regex_syntax/ast/visitor/index.md) produced by [`Pretty`](#pretty)'s [`MakeVisitor`](../../field/index.md) implementation.



#### Implementations

- `fn new(writer: Writer<'a>, is_empty: bool) -> Self` — [`Writer`](#writer)

- `fn with_style(self: Self, style: Style) -> Self`

- `fn write_padded(self: &mut Self, value: &impl fmt::Debug)`

- `fn bold(self: &Self) -> Style`

#### Trait Implementations

##### `impl<'a> Debug for PrettyVisitor<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T> Instrument for PrettyVisitor<'a>`

##### `impl Visit for PrettyVisitor<'_>`

- `fn record_str(self: &mut Self, field: &Field, value: &str)`

- `fn record_error(self: &mut Self, field: &Field, value: &dyn std::error::Error)`

- `fn record_debug(self: &mut Self, field: &Field, value: &dyn fmt::Debug)`

##### `impl VisitFmt for PrettyVisitor<'_>`

- `fn writer(self: &mut Self) -> &mut dyn fmt::Write`

##### `impl VisitOutput for PrettyVisitor<'_>`

- `fn finish(self: Self) -> fmt::Result`

##### `impl<T> WithSubscriber for PrettyVisitor<'a>`

### `PrettyFields`

```rust
struct PrettyFields {
    ansi: Option<bool>,
}
```

An excessively pretty, human-readable [`MakeVisitor`](../../field/index.md) implementation.


#### Fields

- **`ansi`**: `Option<bool>`

  A value to override the provided `Writer`'s ANSI formatting
  configuration.
  
  If this is `Some`, we override the `Writer`'s ANSI setting. This is
  necessary in order to continue supporting the deprecated
  `PrettyFields::with_ansi` method. If it is `None`, we don't override the
  ANSI formatting configuration (because the deprecated method was not
  called).

#### Implementations

- `fn new() -> Self`

- `fn with_ansi(self: Self, ansi: bool) -> Self`

#### Trait Implementations

##### `impl Debug for PrettyFields`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for PrettyFields`

- `fn default() -> Self`

##### `impl<T> Instrument for PrettyFields`

##### `impl<T, M> MakeExt for PrettyFields`

##### `impl<'a> MakeVisitor for PrettyFields`

- `type Visitor = PrettyVisitor<'a>`

- `fn make_visitor(self: &Self, target: Writer<'a>) -> <Self as >::Visitor` — [`Writer`](#writer), [`MakeVisitor`](../../field/index.md)

##### `impl<T, M> Sealed for PrettyFields`

##### `impl<T> WithSubscriber for PrettyFields`

## Traits

### `FormatEvent<S, N>`

```rust
trait FormatEvent<S, N>
where
    S: Subscriber + LookupSpan<'a>,
    N: FormatFields<'a> + 'static { ... }
```

A type that can format a tracing `Event` to a [`Writer`](#writer).

[`FormatEvent`](#formatevent) is primarily used in the context of `fmt::Subscriber` or
`fmt::Layer`. Each time an event is dispatched to `fmt::Subscriber` or
`fmt::Layer`, the subscriber or layer
forwards it to its associated [`FormatEvent`](#formatevent) to emit a log message.

This trait is already implemented for function pointers with the same
signature as `format_event`.

# Arguments

The following arguments are passed to `FormatEvent::format_event`:

* A [`FmtContext`](../fmt_layer/index.md). This is an extension of the `layer::Context` type,
  which can be used for accessing stored information such as the current
  span context an event occurred in.

  In addition, [`FmtContext`](../fmt_layer/index.md) exposes access to the [`FormatFields`](#formatfields)
  implementation that the subscriber was configured to use via the
  `FmtContext::field_format` method. This can be used when the
  [`FormatEvent`](#formatevent) implementation needs to format the event's fields.

  For convenience, [`FmtContext`](../fmt_layer/index.md) also implements [`FormatFields`](#formatfields),
  forwarding to the configured [`FormatFields`](#formatfields) type.

* A [`Writer`](#writer) to which the formatted representation of the event is
  written. This type implements the [`std::fmt::Write`](../../../fs_err/index.md) trait, and therefore
  can be used with the [`std::write!`](../../../nu_ansi_term/write/index.md) and `std::writeln!` macros, as well
  as calling [`std::fmt::Write`](../../../fs_err/index.md) methods directly.

  The [`Writer`](#writer) type also implements additional methods that provide
  information about how the event should be formatted. The
  `Writer::has_ansi_escapes` method indicates whether [ANSI terminal
  escape codes] are supported by the underlying I/O writer that the event
  will be written to. If this returns `true`, the formatter is permitted to
  use ANSI escape codes to add colors and other text formatting to its
  output. If it returns `false`, the event will be written to an output that
  does not support ANSI escape codes (such as a log file), and they should
  not be emitted.

  Crates like `nu_ansi_term` and `owo-colors` can be used to add ANSI
  escape codes to formatted output.

* The actual `Event` to be formatted.

# Examples

This example re-implements a simplified version of this crate's [default
formatter]:

```rust
use std::fmt;
use tracing_core::{Subscriber, Event};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext,
    FormattedFields,
};
use tracing_subscriber::registry::LookupSpan;

struct MyFormatter;

impl<S, N> FormatEvent<S, N> for MyFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();
        write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        // Format all the spans in the event's span context.
        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;

                // `FormattedFields` is a formatted representation of the span's
                // fields, which is stored in its extensions by the `fmt` layer's
                // `new_span` method. The fields will have been formatted
                // by the same field formatter that's provided to the event
                // formatter in the `FmtContext`.
                let ext = span.extensions();
                let fields = &ext
                    .get::<FormattedFields<N>>()
                    .expect("will never be `None`");

                // Skip formatting the fields if the span had no fields.
                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, ": ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

let _subscriber = tracing_subscriber::fmt()
    .event_format(MyFormatter)
    .init();

let _span = tracing::info_span!("my_span", answer = 42).entered();
tracing::info!(question = "life, the universe, and everything", "hello world");
```

This formatter will print events like this:

```text
DEBUG yak_shaving::shaver: some-span{field-on-span=foo}: started shaving yak
```











#### Required Methods

- `fn format_event(self: &Self, ctx: &FmtContext<'_, S, N>, writer: Writer<'_>, event: &Event<'_>) -> fmt::Result`

  Write a log message for [`Event`](../../../tracing_core/event/index.md) in [`FmtContext`](../fmt_layer/index.md) to the given [`Writer`](#writer).

### `FormatFields<'writer>`

```rust
trait FormatFields<'writer> { ... }
```

A type that can format a [set of fields] to a [`Writer`](#writer).

[`FormatFields`](#formatfields) is primarily used in the context of [`FmtSubscriber`](../../index.md). Each
time a span or event with fields is recorded, the subscriber will format
those fields with its associated [`FormatFields`](#formatfields) implementation.



#### Required Methods

- `fn format_fields<R: RecordFields>(self: &Self, writer: Writer<'writer>, fields: R) -> fmt::Result`

  Format the provided `fields` to the provided [`Writer`](#writer), returning a result.

- `fn add_fields(self: &Self, current: &'writer mut FormattedFields<Self>, fields: &span::Record<'_>) -> fmt::Result`

  Record additional field(s) on an existing span.

## Functions

### `format`

```rust
fn format() -> Format
```

Returns the default configuration for an event formatter.

Methods on the returned event formatter can be used for further
configuration. For example:

```rust
let format = tracing_subscriber::fmt::format()
    .without_time()         // Don't include timestamps
    .with_target(false)     // Don't include event targets.
    .with_level(false)      // Don't include event levels.
    .compact();             // Use a more compact, abbreviated format.

// Use the configured formatter when building a new subscriber.
tracing_subscriber::fmt()
    .event_format(format)
    .init();
```

### `debug_fn`

```rust
fn debug_fn<F>(f: F) -> FieldFn<F>
where
    F: Fn(&mut Writer<'_>, &tracing_core::field::Field, &dyn fmt::Debug) -> fmt::Result + Clone
```

Returns a [`FormatFields`](#formatfields) implementation that formats fields using the
provided function or closure.


