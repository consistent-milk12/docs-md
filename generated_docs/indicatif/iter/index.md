*[indicatif](../index.md) / [iter](index.md)*

---

# Module `iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ProgressBarIter`](#progressbariter) | struct | Wraps an iterator to display its progress. |
| [`ProgressIterator`](#progressiterator) | trait | Wraps an iterator to display its progress. |

## Structs

### `ProgressBarIter<T>`

```rust
struct ProgressBarIter<T> {
    it: T,
    pub progress: crate::progress_bar::ProgressBar,
}
```

*Defined in [`indicatif-0.18.3/src/iter.rs:62-65`](../../../.source_1765521767/indicatif-0.18.3/src/iter.rs#L62-L65)*

Wraps an iterator to display its progress.

#### Implementations

- <span id="progressbariter-with-style"></span>`fn with_style(self, style: ProgressStyle) -> Self` — [`ProgressStyle`](../style/index.md#progressstyle)

- <span id="progressbariter-with-prefix"></span>`fn with_prefix(self, prefix: impl Into<Cow<'static, str>>) -> Self`

- <span id="progressbariter-with-message"></span>`fn with_message(self, message: impl Into<Cow<'static, str>>) -> Self`

- <span id="progressbariter-with-position"></span>`fn with_position(self, position: u64) -> Self`

- <span id="progressbariter-with-elapsed"></span>`fn with_elapsed(self, elapsed: Duration) -> Self`

- <span id="progressbariter-with-finish"></span>`fn with_finish(self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](../state/index.md#progressfinish)

#### Trait Implementations

##### `impl<R: io::BufRead> BufRead for ProgressBarIter<R>`

- <span id="progressbariter-fill-buf"></span>`fn fill_buf(&mut self) -> io::Result<&[u8]>`

- <span id="progressbariter-consume"></span>`fn consume(&mut self, amt: usize)`

##### `impl<T: fmt::Debug> Debug for ProgressBarIter<T>`

- <span id="progressbariter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: DoubleEndedIterator> DoubleEndedIterator for ProgressBarIter<T>`

- <span id="progressbariter-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T: ExactSizeIterator> ExactSizeIterator for ProgressBarIter<T>`

- <span id="progressbariter-len"></span>`fn len(&self) -> usize`

##### `impl<T: FusedIterator> FusedIterator for ProgressBarIter<T>`

##### `impl IntoIterator for ProgressBarIter<T>`

- <span id="progressbariter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="progressbariter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="progressbariter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: Iterator<Item = S>> Iterator for ProgressBarIter<T>`

- <span id="progressbariter-iterator-type-item"></span>`type Item = S`

- <span id="progressbariter-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ProgressIterator for ProgressBarIter<T>`

- <span id="progressbariter-progress-with"></span>`fn progress_with(self, progress: ProgressBar) -> ProgressBarIter<T>` — [`ProgressBar`](../progress_bar/index.md#progressbar), [`ProgressBarIter`](#progressbariter)

##### `impl<R: io::Read> Read for ProgressBarIter<R>`

- <span id="progressbariter-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

- <span id="progressbariter-read-vectored"></span>`fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>`

- <span id="progressbariter-read-to-string"></span>`fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize>`

- <span id="progressbariter-read-exact"></span>`fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()>`

##### `impl<S: io::Seek> Seek for ProgressBarIter<S>`

- <span id="progressbariter-seek"></span>`fn seek(&mut self, f: io::SeekFrom) -> io::Result<u64>`

- <span id="progressbariter-stream-position"></span>`fn stream_position(&mut self) -> io::Result<u64>`

##### `impl<W: io::Write> Write for ProgressBarIter<W>`

- <span id="progressbariter-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="progressbariter-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>`

- <span id="progressbariter-flush"></span>`fn flush(&mut self) -> io::Result<()>`

## Traits

### `ProgressIterator`

```rust
trait ProgressIterator
where
    Self: Sized + Iterator { ... }
```

*Defined in [`indicatif-0.18.3/src/iter.rs:18-58`](../../../.source_1765521767/indicatif-0.18.3/src/iter.rs#L18-L58)*

Wraps an iterator to display its progress.

#### Required Methods

- `fn progress_with(self, progress: ProgressBar) -> ProgressBarIter<Self>`

  Wrap an iterator with a custom progress bar.

#### Provided Methods

- `fn try_progress(self) -> Option<ProgressBarIter<Self>>`

  Wrap an iterator with default styling. Uses `Iterator::size_hint()` to get length.

- `fn progress(self) -> ProgressBarIter<Self>`

  Wrap an iterator with default styling.

- `fn progress_count(self, len: u64) -> ProgressBarIter<Self>`

  Wrap an iterator with an explicit element count.

- `fn progress_with_style(self, style: crate::ProgressStyle) -> ProgressBarIter<Self>`

  Wrap an iterator with a progress bar and style it.

#### Implementors

- `T`

