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

*Defined in [`indicatif-0.18.3/src/iter.rs:62-65`](../../../.source_1765633015/indicatif-0.18.3/src/iter.rs#L62-L65)*

Wraps an iterator to display its progress.

#### Implementations

- <span id="progressbariter-with-style"></span>`fn with_style(self, style: ProgressStyle) -> Self` — [`ProgressStyle`](../style/index.md#progressstyle)

  Builder-like function for setting underlying progress bar's style.

  

  See `ProgressBar::with_style()`.

- <span id="progressbariter-with-prefix"></span>`fn with_prefix(self, prefix: impl Into<Cow<'static, str>>) -> Self`

  Builder-like function for setting underlying progress bar's prefix.

  

  See `ProgressBar::with_prefix()`.

- <span id="progressbariter-with-message"></span>`fn with_message(self, message: impl Into<Cow<'static, str>>) -> Self`

  Builder-like function for setting underlying progress bar's message.

  

  See `ProgressBar::with_message()`.

- <span id="progressbariter-with-position"></span>`fn with_position(self, position: u64) -> Self`

  Builder-like function for setting underlying progress bar's position.

  

  See `ProgressBar::with_position()`.

- <span id="progressbariter-with-elapsed"></span>`fn with_elapsed(self, elapsed: Duration) -> Self`

  Builder-like function for setting underlying progress bar's elapsed time.

  

  See `ProgressBar::with_elapsed()`.

- <span id="progressbariter-with-finish"></span>`fn with_finish(self, finish: ProgressFinish) -> Self` — [`ProgressFinish`](../state/index.md#progressfinish)

  Builder-like function for setting underlying progress bar's finish behavior.

  

  See `ProgressBar::with_finish()`.

#### Trait Implementations

##### `impl<T> Any for ProgressBarIter<T>`

- <span id="progressbariter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ProgressBarIter<T>`

- <span id="progressbariter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ProgressBarIter<T>`

- <span id="progressbariter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: io::BufRead> BufRead for ProgressBarIter<R>`

- <span id="progressbariter-bufread-fill-buf"></span>`fn fill_buf(&mut self) -> io::Result<&[u8]>`

- <span id="progressbariter-bufread-consume"></span>`fn consume(&mut self, amt: usize)`

##### `impl<T: fmt::Debug> Debug for ProgressBarIter<T>`

- <span id="progressbariter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: DoubleEndedIterator> DoubleEndedIterator for ProgressBarIter<T>`

- <span id="progressbariter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T: ExactSizeIterator> ExactSizeIterator for ProgressBarIter<T>`

- <span id="progressbariter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for ProgressBarIter<T>`

- <span id="progressbariter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: FusedIterator> FusedIterator for ProgressBarIter<T>`

##### `impl<T, U> Into for ProgressBarIter<T>`

- <span id="progressbariter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ProgressBarIter<T>`

- <span id="progressbariter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="progressbariter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="progressbariter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: Iterator<Item = S>> Iterator for ProgressBarIter<T>`

- <span id="progressbariter-iterator-type-item"></span>`type Item = S`

- <span id="progressbariter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ProgressIterator for ProgressBarIter<T>`

- <span id="progressbariter-progressiterator-progress-with"></span>`fn progress_with(self, progress: ProgressBar) -> ProgressBarIter<T>` — [`ProgressBar`](../progress_bar/index.md#progressbar), [`ProgressBarIter`](#progressbariter)

##### `impl<R: io::Read> Read for ProgressBarIter<R>`

- <span id="progressbariter-read"></span>`fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>`

- <span id="progressbariter-read-read-vectored"></span>`fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>`

- <span id="progressbariter-read-read-to-string"></span>`fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize>`

- <span id="progressbariter-read-read-exact"></span>`fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()>`

##### `impl<S: io::Seek> Seek for ProgressBarIter<S>`

- <span id="progressbariter-seek"></span>`fn seek(&mut self, f: io::SeekFrom) -> io::Result<u64>`

- <span id="progressbariter-seek-stream-position"></span>`fn stream_position(&mut self) -> io::Result<u64>`

##### `impl<T, U> TryFrom for ProgressBarIter<T>`

- <span id="progressbariter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="progressbariter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ProgressBarIter<T>`

- <span id="progressbariter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="progressbariter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<W: io::Write> Write for ProgressBarIter<W>`

- <span id="progressbariter-write"></span>`fn write(&mut self, buf: &[u8]) -> io::Result<usize>`

- <span id="progressbariter-write-write-vectored"></span>`fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize>`

- <span id="progressbariter-write-flush"></span>`fn flush(&mut self) -> io::Result<()>`

## Traits

### `ProgressIterator`

```rust
trait ProgressIterator
where
    Self: Sized + Iterator { ... }
```

*Defined in [`indicatif-0.18.3/src/iter.rs:18-58`](../../../.source_1765633015/indicatif-0.18.3/src/iter.rs#L18-L58)*

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

