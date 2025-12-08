*[indicatif](../index.md) / [iter](index.md)*

---

# Module `iter`

## Structs

### `ProgressBarIter<T>`

```rust
struct ProgressBarIter<T> {
    it: T,
    pub progress: crate::progress_bar::ProgressBar,
}
```

Wraps an iterator to display its progress.

#### Implementations

- `fn with_style(self: Self, style: ProgressStyle) -> Self` — [`ProgressStyle`](../style/index.md)

- `fn with_prefix(self: Self, prefix: impl Into<Cow<'static, str>>) -> Self`

- `fn with_message(self: Self, message: impl Into<Cow<'static, str>>) -> Self`

- `fn with_position(self: Self, position: u64) -> Self`

- `fn with_elapsed(self: Self, elapsed: Duration) -> Self`

- `fn with_finish(self: Self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](../state/index.md)

#### Trait Implementations

##### `impl<R: io::BufRead> BufRead for ProgressBarIter<R>`

- `fn fill_buf(self: &mut Self) -> io::Result<&[u8]>`

- `fn consume(self: &mut Self, amt: usize)`

##### `impl<T: $crate::fmt::Debug> Debug for ProgressBarIter<T>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<T: DoubleEndedIterator> DoubleEndedIterator for ProgressBarIter<T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<T: ExactSizeIterator> ExactSizeIterator for ProgressBarIter<T>`

- `fn len(self: &Self) -> usize`

##### `impl<T: FusedIterator> FusedIterator for ProgressBarIter<T>`

##### `impl<I> IntoIterator for ProgressBarIter<T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<S, T: Iterator<Item = S>> Iterator for ProgressBarIter<T>`

- `type Item = S`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

##### `impl<S, T> ProgressIterator for ProgressBarIter<T>`

- `fn progress_with(self: Self, progress: ProgressBar) -> ProgressBarIter<T>` — [`ProgressBar`](../progress_bar/index.md), [`ProgressBarIter`](#progressbariter)

##### `impl<R: io::Read> Read for ProgressBarIter<R>`

- `fn read(self: &mut Self, buf: &mut [u8]) -> io::Result<usize>`

- `fn read_vectored(self: &mut Self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>`

- `fn read_to_string(self: &mut Self, buf: &mut String) -> io::Result<usize>`

- `fn read_exact(self: &mut Self, buf: &mut [u8]) -> io::Result<()>`

##### `impl<S: io::Seek> Seek for ProgressBarIter<S>`

- `fn seek(self: &mut Self, f: io::SeekFrom) -> io::Result<u64>`

- `fn stream_position(self: &mut Self) -> io::Result<u64>`

##### `impl<W: io::Write> Write for ProgressBarIter<W>`

- `fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize>`

- `fn write_vectored(self: &mut Self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>`

- `fn flush(self: &mut Self) -> io::Result<()>`

## Traits

### `ProgressIterator`

```rust
trait ProgressIterator
where
    Self: Sized + Iterator { ... }
```

Wraps an iterator to display its progress.

#### Required Methods

- `fn try_progress(self: Self) -> Option<ProgressBarIter<Self>>`

  Wrap an iterator with default styling. Uses `Iterator::size_hint()` to get length.

- `fn progress(self: Self) -> ProgressBarIter<Self>`

  Wrap an iterator with default styling.

- `fn progress_count(self: Self, len: u64) -> ProgressBarIter<Self>`

  Wrap an iterator with an explicit element count.

- `fn progress_with(self: Self, progress: ProgressBar) -> ProgressBarIter<Self>`

  Wrap an iterator with a custom progress bar.

- `fn progress_with_style(self: Self, style: crate::ProgressStyle) -> ProgressBarIter<Self>`

  Wrap an iterator with a progress bar and style it.

