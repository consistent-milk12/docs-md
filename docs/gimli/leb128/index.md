*[gimli](../index.md) / [leb128](index.md)*

---

# Module `leb128`

Read and write DWARF's "Little Endian Base 128" (LEB128) variable length
integer encoding.

The implementation is a direct translation of the psuedocode in the DWARF 4
standard's appendix C.

Read and write signed integers:

```
# #[cfg(all(feature = "read", feature = "write"))] {
use gimli::{EndianSlice, NativeEndian, leb128};

let mut buf = [0; 1024];

// Write to anything that implements `std::io::Write`.
{
    let mut writable = &mut buf[..];
    leb128::write::signed(&mut writable, -12345).expect("Should write number");
}

// Read from anything that implements `gimli::Reader`.
let mut readable = EndianSlice::new(&buf[..], NativeEndian);
let val = leb128::read::signed(&mut readable).expect("Should read number");
assert_eq!(val, -12345);
# }
```

Or read and write unsigned integers:

```
# #[cfg(all(feature = "read", feature = "write"))] {
use gimli::{EndianSlice, NativeEndian, leb128};

let mut buf = [0; 1024];

{
    let mut writable = &mut buf[..];
    leb128::write::unsigned(&mut writable, 98765).expect("Should write number");
}

let mut readable = EndianSlice::new(&buf[..], NativeEndian);
let val = leb128::read::unsigned(&mut readable).expect("Should read number");
assert_eq!(val, 98765);
# }
```

## Modules

- [`read`](read/index.md) - A module for reading signed and unsigned integers that have been LEB128

