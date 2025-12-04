*[miniz_oxide](../../index.md) / [deflate](../index.md) / [stream](index.md)*

---

# Module `stream`

Extra streaming compression functionality.

As of now this is mainly intended for use to build a higher-level wrapper.

There is no DeflateState as the needed state is contained in the compressor struct itself.

## Functions

### `deflate`

```rust
fn deflate(compressor: &mut crate::deflate::core::CompressorOxide, input: &[u8], output: &mut [u8], flush: crate::MZFlush) -> crate::StreamResult
```

Try to compress from input to output with the given [`CompressorOxide`](../core/index.md).

# Errors

Returns `MZError::Buf` If the size of the `output` slice is empty or no progress was made due
to lack of expected input data, or if called without `MZFlush::Finish` after the compression
was already finished.

Returns `MZError::Param` if the compressor parameters are set wrong.

Returns `MZError::Stream` when lower-level decompressor returns a
`TDEFLStatus::PutBufFailed`; may not actually be possible.

