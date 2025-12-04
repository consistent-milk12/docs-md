*[linux_raw_sys](../index.md) / [general](index.md)*

---

# Module `general`

## Structs

### `__BindgenBitfieldUnit<Storage>`

```rust
struct __BindgenBitfieldUnit<Storage> {
}
```

#### Implementations

- `fn get_bit(self: &Self, index: usize) -> bool`

- `unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool`

- `fn set_bit(self: &mut Self, index: usize, val: bool)`

- `unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool)`

- `fn get(self: &Self, bit_offset: usize, bit_width: u8) -> u64`

- `unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64`

- `fn set(self: &mut Self, bit_offset: usize, bit_width: u8, val: u64)`

- `unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64)`

- `const fn new(storage: Storage) -> Self`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<Storage: $crate::clone::Clone>`

- `fn clone(self: &Self) -> __BindgenBitfieldUnit<Storage>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy<Storage: $crate::marker::Copy>`

##### `impl Eq<Storage: $crate::cmp::Eq>`

##### `impl Hash<Storage: $crate::hash::Hash>`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord<Storage: $crate::cmp::Ord>`

- `fn cmp(self: &Self, other: &__BindgenBitfieldUnit<Storage>) -> $crate::cmp::Ordering`

##### `impl PartialEq<Storage: $crate::cmp::PartialEq>`

- `fn eq(self: &Self, other: &__BindgenBitfieldUnit<Storage>) -> bool`

##### `impl PartialOrd<Storage: $crate::cmp::PartialOrd>`

- `fn partial_cmp(self: &Self, other: &__BindgenBitfieldUnit<Storage>) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq<Storage>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<Storage: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default<Storage: $crate::default::Default>`

- `fn default() -> __BindgenBitfieldUnit<Storage>`

### `__IncompleteArrayField<T>`

```rust
struct __IncompleteArrayField<T>();
```

#### Implementations

- `const fn new() -> Self`

- `fn as_ptr(self: &Self) -> *const T`

- `fn as_mut_ptr(self: &mut Self) -> *mut T`

- `unsafe fn as_slice(self: &Self, len: usize) -> &[T]`

- `unsafe fn as_mut_slice(self: &mut Self, len: usize) -> &mut [T]`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<T>`

- `fn fmt(self: &Self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result`

##### `impl Default<T: $crate::default::Default>`

- `fn default() -> __IncompleteArrayField<T>`

### `__kernel_fd_set`

```rust
struct __kernel_fd_set {
    pub fds_bits: [crate::ctypes::c_ulong; 16],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __kernel_fd_set`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__kernel_fsid_t`

```rust
struct __kernel_fsid_t {
    pub val: [crate::ctypes::c_int; 2],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __kernel_fsid_t`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__user_cap_header_struct`

```rust
struct __user_cap_header_struct {
    pub version: __u32,
    pub pid: crate::ctypes::c_int,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __user_cap_header_struct`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__user_cap_data_struct`

```rust
struct __user_cap_data_struct {
    pub effective: __u32,
    pub permitted: __u32,
    pub inheritable: __u32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __user_cap_data_struct`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `vfs_cap_data`

```rust
struct vfs_cap_data {
    pub magic_etc: __le32,
    pub data: [vfs_cap_data__bindgen_ty_1; 2],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> vfs_cap_data`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `vfs_cap_data__bindgen_ty_1`

```rust
struct vfs_cap_data__bindgen_ty_1 {
    pub permitted: __le32,
    pub inheritable: __le32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> vfs_cap_data__bindgen_ty_1`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `vfs_ns_cap_data`

```rust
struct vfs_ns_cap_data {
    pub magic_etc: __le32,
    pub data: [vfs_ns_cap_data__bindgen_ty_1; 2],
    pub rootid: __le32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> vfs_ns_cap_data`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `vfs_ns_cap_data__bindgen_ty_1`

```rust
struct vfs_ns_cap_data__bindgen_ty_1 {
    pub permitted: __le32,
    pub inheritable: __le32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> vfs_ns_cap_data__bindgen_ty_1`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `f_owner_ex`

```rust
struct f_owner_ex {
    pub type_: crate::ctypes::c_int,
    pub pid: __kernel_pid_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> f_owner_ex`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `flock`

```rust
struct flock {
    pub l_type: crate::ctypes::c_short,
    pub l_whence: crate::ctypes::c_short,
    pub l_start: __kernel_off_t,
    pub l_len: __kernel_off_t,
    pub l_pid: __kernel_pid_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> flock`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `flock64`

```rust
struct flock64 {
    pub l_type: crate::ctypes::c_short,
    pub l_whence: crate::ctypes::c_short,
    pub l_start: __kernel_loff_t,
    pub l_len: __kernel_loff_t,
    pub l_pid: __kernel_pid_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> flock64`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `open_how`

```rust
struct open_how {
    pub flags: __u64,
    pub mode: __u64,
    pub resolve: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> open_how`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `epoll_event`

```rust
struct epoll_event {
    pub events: __poll_t,
    pub data: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> epoll_event`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `epoll_params`

```rust
struct epoll_params {
    pub busy_poll_usecs: __u32,
    pub busy_poll_budget: __u16,
    pub prefer_busy_poll: __u8,
    pub __pad: __u8,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> epoll_params`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fscrypt_policy_v1`

```rust
struct fscrypt_policy_v1 {
    pub version: __u8,
    pub contents_encryption_mode: __u8,
    pub filenames_encryption_mode: __u8,
    pub flags: __u8,
    pub master_key_descriptor: [__u8; 8],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fscrypt_policy_v1`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fscrypt_key`

```rust
struct fscrypt_key {
    pub mode: __u32,
    pub raw: [__u8; 64],
    pub size: __u32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fscrypt_key`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fscrypt_policy_v2`

```rust
struct fscrypt_policy_v2 {
    pub version: __u8,
    pub contents_encryption_mode: __u8,
    pub filenames_encryption_mode: __u8,
    pub flags: __u8,
    pub log2_data_unit_size: __u8,
    pub __reserved: [__u8; 3],
    pub master_key_identifier: [__u8; 16],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fscrypt_policy_v2`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fscrypt_get_policy_ex_arg`

```rust
struct fscrypt_get_policy_ex_arg {
    pub policy_size: __u64,
    pub policy: fscrypt_get_policy_ex_arg__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fscrypt_get_policy_ex_arg`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `fscrypt_key_specifier`

```rust
struct fscrypt_key_specifier {
    pub type_: __u32,
    pub __reserved: __u32,
    pub u: fscrypt_key_specifier__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fscrypt_key_specifier`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `fscrypt_provisioning_key_payload`

```rust
struct fscrypt_provisioning_key_payload {
    pub type_: __u32,
    pub flags: __u32,
    pub raw: __IncompleteArrayField<__u8>,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fscrypt_add_key_arg`

```rust
struct fscrypt_add_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub raw_size: __u32,
    pub key_id: __u32,
    pub flags: __u32,
    pub __reserved: [__u32; 7],
    pub raw: __IncompleteArrayField<__u8>,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `fscrypt_remove_key_arg`

```rust
struct fscrypt_remove_key_arg {
    pub key_spec: fscrypt_key_specifier,
    pub removal_status_flags: __u32,
    pub __reserved: [__u32; 5],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fscrypt_remove_key_arg`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `fscrypt_get_key_status_arg`

```rust
struct fscrypt_get_key_status_arg {
    pub key_spec: fscrypt_key_specifier,
    pub __reserved: [__u32; 6],
    pub status: __u32,
    pub status_flags: __u32,
    pub user_count: __u32,
    pub __out_reserved: [__u32; 13],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fscrypt_get_key_status_arg`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `mount_attr`

```rust
struct mount_attr {
    pub attr_set: __u64,
    pub attr_clr: __u64,
    pub propagation: __u64,
    pub userns_fd: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> mount_attr`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `statmount`

```rust
struct statmount {
    pub size: __u32,
    pub mnt_opts: __u32,
    pub mask: __u64,
    pub sb_dev_major: __u32,
    pub sb_dev_minor: __u32,
    pub sb_magic: __u64,
    pub sb_flags: __u32,
    pub fs_type: __u32,
    pub mnt_id: __u64,
    pub mnt_parent_id: __u64,
    pub mnt_id_old: __u32,
    pub mnt_parent_id_old: __u32,
    pub mnt_attr: __u64,
    pub mnt_propagation: __u64,
    pub mnt_peer_group: __u64,
    pub mnt_master: __u64,
    pub propagate_from: __u64,
    pub mnt_root: __u32,
    pub mnt_point: __u32,
    pub mnt_ns_id: __u64,
    pub fs_subtype: __u32,
    pub sb_source: __u32,
    pub opt_num: __u32,
    pub opt_array: __u32,
    pub opt_sec_num: __u32,
    pub opt_sec_array: __u32,
    pub supported_mask: __u64,
    pub mnt_uidmap_num: __u32,
    pub mnt_uidmap: __u32,
    pub mnt_gidmap_num: __u32,
    pub mnt_gidmap: __u32,
    pub __spare2: [__u64; 43],
    pub str_: __IncompleteArrayField<crate::ctypes::c_char>,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `mnt_id_req`

```rust
struct mnt_id_req {
    pub size: __u32,
    pub spare: __u32,
    pub mnt_id: __u64,
    pub param: __u64,
    pub mnt_ns_id: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> mnt_id_req`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `file_clone_range`

```rust
struct file_clone_range {
    pub src_fd: __s64,
    pub src_offset: __u64,
    pub src_length: __u64,
    pub dest_offset: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> file_clone_range`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fstrim_range`

```rust
struct fstrim_range {
    pub start: __u64,
    pub len: __u64,
    pub minlen: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fstrim_range`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fsuuid2`

```rust
struct fsuuid2 {
    pub len: __u8,
    pub uuid: [__u8; 16],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fsuuid2`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fs_sysfs_path`

```rust
struct fs_sysfs_path {
    pub len: __u8,
    pub name: [__u8; 128],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fs_sysfs_path`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `file_dedupe_range_info`

```rust
struct file_dedupe_range_info {
    pub dest_fd: __s64,
    pub dest_offset: __u64,
    pub bytes_deduped: __u64,
    pub status: __s32,
    pub reserved: __u32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> file_dedupe_range_info`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `file_dedupe_range`

```rust
struct file_dedupe_range {
    pub src_offset: __u64,
    pub src_length: __u64,
    pub dest_count: __u16,
    pub reserved1: __u16,
    pub reserved2: __u32,
    pub info: __IncompleteArrayField<file_dedupe_range_info>,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `files_stat_struct`

```rust
struct files_stat_struct {
    pub nr_files: crate::ctypes::c_ulong,
    pub nr_free_files: crate::ctypes::c_ulong,
    pub max_files: crate::ctypes::c_ulong,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> files_stat_struct`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `inodes_stat_t`

```rust
struct inodes_stat_t {
    pub nr_inodes: crate::ctypes::c_long,
    pub nr_unused: crate::ctypes::c_long,
    pub dummy: [crate::ctypes::c_long; 5],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> inodes_stat_t`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fsxattr`

```rust
struct fsxattr {
    pub fsx_xflags: __u32,
    pub fsx_extsize: __u32,
    pub fsx_nextents: __u32,
    pub fsx_projid: __u32,
    pub fsx_cowextsize: __u32,
    pub fsx_pad: [crate::ctypes::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fsxattr`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `page_region`

```rust
struct page_region {
    pub start: __u64,
    pub end: __u64,
    pub categories: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> page_region`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pm_scan_arg`

```rust
struct pm_scan_arg {
    pub size: __u64,
    pub flags: __u64,
    pub start: __u64,
    pub end: __u64,
    pub walk_end: __u64,
    pub vec: __u64,
    pub vec_len: __u64,
    pub max_pages: __u64,
    pub category_inverted: __u64,
    pub category_mask: __u64,
    pub category_anyof_mask: __u64,
    pub return_mask: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> pm_scan_arg`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `procmap_query`

```rust
struct procmap_query {
    pub size: __u64,
    pub query_flags: __u64,
    pub query_addr: __u64,
    pub vma_start: __u64,
    pub vma_end: __u64,
    pub vma_flags: __u64,
    pub vma_page_size: __u64,
    pub vma_offset: __u64,
    pub inode: __u64,
    pub dev_major: __u32,
    pub dev_minor: __u32,
    pub vma_name_size: __u32,
    pub build_id_size: __u32,
    pub vma_name_addr: __u64,
    pub build_id_addr: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> procmap_query`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `futex_waitv`

```rust
struct futex_waitv {
    pub val: __u64,
    pub uaddr: __u64,
    pub flags: __u32,
    pub __reserved: __u32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> futex_waitv`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `robust_list`

```rust
struct robust_list {
    pub next: *mut robust_list,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> robust_list`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `robust_list_head`

```rust
struct robust_list_head {
    pub list: robust_list,
    pub futex_offset: crate::ctypes::c_long,
    pub list_op_pending: *mut robust_list,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> robust_list_head`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `inotify_event`

```rust
struct inotify_event {
    pub wd: __s32,
    pub mask: __u32,
    pub cookie: __u32,
    pub len: __u32,
    pub name: __IncompleteArrayField<crate::ctypes::c_char>,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `cachestat_range`

```rust
struct cachestat_range {
    pub off: __u64,
    pub len: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> cachestat_range`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `cachestat`

```rust
struct cachestat {
    pub nr_cache: __u64,
    pub nr_dirty: __u64,
    pub nr_writeback: __u64,
    pub nr_evicted: __u64,
    pub nr_recently_evicted: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> cachestat`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pollfd`

```rust
struct pollfd {
    pub fd: crate::ctypes::c_int,
    pub events: crate::ctypes::c_short,
    pub revents: crate::ctypes::c_short,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> pollfd`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `rand_pool_info`

```rust
struct rand_pool_info {
    pub entropy_count: crate::ctypes::c_int,
    pub buf_size: crate::ctypes::c_int,
    pub buf: __IncompleteArrayField<__u32>,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `vgetrandom_opaque_params`

```rust
struct vgetrandom_opaque_params {
    pub size_of_opaque_state: __u32,
    pub mmap_prot: __u32,
    pub mmap_flags: __u32,
    pub reserved: [__u32; 13],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> vgetrandom_opaque_params`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__kernel_timespec`

```rust
struct __kernel_timespec {
    pub tv_sec: __kernel_time64_t,
    pub tv_nsec: crate::ctypes::c_longlong,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __kernel_timespec`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__kernel_itimerspec`

```rust
struct __kernel_itimerspec {
    pub it_interval: __kernel_timespec,
    pub it_value: __kernel_timespec,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __kernel_itimerspec`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__kernel_old_timeval`

```rust
struct __kernel_old_timeval {
    pub tv_sec: __kernel_long_t,
    pub tv_usec: __kernel_long_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __kernel_old_timeval`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__kernel_old_timespec`

```rust
struct __kernel_old_timespec {
    pub tv_sec: __kernel_old_time_t,
    pub tv_nsec: crate::ctypes::c_long,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __kernel_old_timespec`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__kernel_old_itimerval`

```rust
struct __kernel_old_itimerval {
    pub it_interval: __kernel_old_timeval,
    pub it_value: __kernel_old_timeval,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __kernel_old_itimerval`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__kernel_sock_timeval`

```rust
struct __kernel_sock_timeval {
    pub tv_sec: __s64,
    pub tv_usec: __s64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __kernel_sock_timeval`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `rusage`

```rust
struct rusage {
    pub ru_utime: __kernel_old_timeval,
    pub ru_stime: __kernel_old_timeval,
    pub ru_maxrss: __kernel_long_t,
    pub ru_ixrss: __kernel_long_t,
    pub ru_idrss: __kernel_long_t,
    pub ru_isrss: __kernel_long_t,
    pub ru_minflt: __kernel_long_t,
    pub ru_majflt: __kernel_long_t,
    pub ru_nswap: __kernel_long_t,
    pub ru_inblock: __kernel_long_t,
    pub ru_oublock: __kernel_long_t,
    pub ru_msgsnd: __kernel_long_t,
    pub ru_msgrcv: __kernel_long_t,
    pub ru_nsignals: __kernel_long_t,
    pub ru_nvcsw: __kernel_long_t,
    pub ru_nivcsw: __kernel_long_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> rusage`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `rlimit`

```rust
struct rlimit {
    pub rlim_cur: __kernel_ulong_t,
    pub rlim_max: __kernel_ulong_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> rlimit`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `rlimit64`

```rust
struct rlimit64 {
    pub rlim_cur: __u64,
    pub rlim_max: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> rlimit64`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `clone_args`

```rust
struct clone_args {
    pub flags: __u64,
    pub pidfd: __u64,
    pub child_tid: __u64,
    pub parent_tid: __u64,
    pub exit_signal: __u64,
    pub stack: __u64,
    pub stack_size: __u64,
    pub tls: __u64,
    pub set_tid: __u64,
    pub set_tid_size: __u64,
    pub cgroup: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> clone_args`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sigaction`

```rust
struct sigaction {
    pub sa_handler: __sighandler_t,
    pub sa_flags: crate::ctypes::c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: sigset_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> sigaction`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sigaltstack`

```rust
struct sigaltstack {
    pub ss_sp: *mut crate::ctypes::c_void,
    pub ss_flags: crate::ctypes::c_int,
    pub ss_size: __kernel_size_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> sigaltstack`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__sifields__bindgen_ty_1`

```rust
struct __sifields__bindgen_ty_1 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __sifields__bindgen_ty_1`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__sifields__bindgen_ty_2`

```rust
struct __sifields__bindgen_ty_2 {
    pub _tid: __kernel_timer_t,
    pub _overrun: crate::ctypes::c_int,
    pub _sigval: sigval_t,
    pub _sys_private: crate::ctypes::c_int,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __sifields__bindgen_ty_2`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_3`

```rust
struct __sifields__bindgen_ty_3 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _sigval: sigval_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __sifields__bindgen_ty_3`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_4`

```rust
struct __sifields__bindgen_ty_4 {
    pub _pid: __kernel_pid_t,
    pub _uid: __kernel_uid32_t,
    pub _status: crate::ctypes::c_int,
    pub _utime: __kernel_clock_t,
    pub _stime: __kernel_clock_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __sifields__bindgen_ty_4`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__sifields__bindgen_ty_5`

```rust
struct __sifields__bindgen_ty_5 {
    pub _addr: *mut crate::ctypes::c_void,
    pub __bindgen_anon_1: __sifields__bindgen_ty_5__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __sifields__bindgen_ty_5`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

```rust
struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
    pub _dummy_bnd: [crate::ctypes::c_char; 8],
    pub _lower: *mut crate::ctypes::c_void,
    pub _upper: *mut crate::ctypes::c_void,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

```rust
struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2 {
    pub _dummy_pkey: [crate::ctypes::c_char; 8],
    pub _pkey: __u32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_2`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

```rust
struct __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3 {
    pub _data: crate::ctypes::c_ulong,
    pub _type: __u32,
    pub _flags: __u32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __sifields__bindgen_ty_5__bindgen_ty_1__bindgen_ty_3`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__sifields__bindgen_ty_6`

```rust
struct __sifields__bindgen_ty_6 {
    pub _band: crate::ctypes::c_long,
    pub _fd: crate::ctypes::c_int,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __sifields__bindgen_ty_6`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__sifields__bindgen_ty_7`

```rust
struct __sifields__bindgen_ty_7 {
    pub _call_addr: *mut crate::ctypes::c_void,
    pub _syscall: crate::ctypes::c_int,
    pub _arch: crate::ctypes::c_uint,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __sifields__bindgen_ty_7`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `siginfo`

```rust
struct siginfo {
    pub __bindgen_anon_1: siginfo__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> siginfo`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `siginfo__bindgen_ty_1__bindgen_ty_1`

```rust
struct siginfo__bindgen_ty_1__bindgen_ty_1 {
    pub si_signo: crate::ctypes::c_int,
    pub si_errno: crate::ctypes::c_int,
    pub si_code: crate::ctypes::c_int,
    pub _sifields: __sifields,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> siginfo__bindgen_ty_1__bindgen_ty_1`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `sigevent`

```rust
struct sigevent {
    pub sigev_value: sigval_t,
    pub sigev_signo: crate::ctypes::c_int,
    pub sigev_notify: crate::ctypes::c_int,
    pub _sigev_un: sigevent__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> sigevent`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `sigevent__bindgen_ty_1__bindgen_ty_1`

```rust
struct sigevent__bindgen_ty_1__bindgen_ty_1 {
    pub _function: ::core::option::Option<fn(sigval_t)>,
    pub _attribute: *mut crate::ctypes::c_void,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> sigevent__bindgen_ty_1__bindgen_ty_1`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `statx_timestamp`

```rust
struct statx_timestamp {
    pub tv_sec: __s64,
    pub tv_nsec: __u32,
    pub __reserved: __s32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> statx_timestamp`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `statx`

```rust
struct statx {
    pub stx_mask: __u32,
    pub stx_blksize: __u32,
    pub stx_attributes: __u64,
    pub stx_nlink: __u32,
    pub stx_uid: __u32,
    pub stx_gid: __u32,
    pub stx_mode: __u16,
    pub __spare0: [__u16; 1],
    pub stx_ino: __u64,
    pub stx_size: __u64,
    pub stx_blocks: __u64,
    pub stx_attributes_mask: __u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: __u32,
    pub stx_rdev_minor: __u32,
    pub stx_dev_major: __u32,
    pub stx_dev_minor: __u32,
    pub stx_mnt_id: __u64,
    pub stx_dio_mem_align: __u32,
    pub stx_dio_offset_align: __u32,
    pub stx_subvol: __u64,
    pub stx_atomic_write_unit_min: __u32,
    pub stx_atomic_write_unit_max: __u32,
    pub stx_atomic_write_segments_max: __u32,
    pub stx_dio_read_offset_align: __u32,
    pub stx_atomic_write_unit_max_opt: __u32,
    pub __spare2: [__u32; 1],
    pub __spare3: [__u64; 8],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> statx`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `termios`

```rust
struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 19],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> termios`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `termios2`

```rust
struct termios2 {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 19],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> termios2`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ktermios`

```rust
struct ktermios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 19],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ktermios`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `winsize`

```rust
struct winsize {
    pub ws_row: crate::ctypes::c_ushort,
    pub ws_col: crate::ctypes::c_ushort,
    pub ws_xpixel: crate::ctypes::c_ushort,
    pub ws_ypixel: crate::ctypes::c_ushort,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> winsize`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `termio`

```rust
struct termio {
    pub c_iflag: crate::ctypes::c_ushort,
    pub c_oflag: crate::ctypes::c_ushort,
    pub c_cflag: crate::ctypes::c_ushort,
    pub c_lflag: crate::ctypes::c_ushort,
    pub c_line: crate::ctypes::c_uchar,
    pub c_cc: [crate::ctypes::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> termio`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `timespec`

```rust
struct timespec {
    pub tv_sec: __kernel_old_time_t,
    pub tv_nsec: crate::ctypes::c_long,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> timespec`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `timeval`

```rust
struct timeval {
    pub tv_sec: __kernel_old_time_t,
    pub tv_usec: __kernel_suseconds_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> timeval`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `itimerspec`

```rust
struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> itimerspec`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `itimerval`

```rust
struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> itimerval`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `timezone`

```rust
struct timezone {
    pub tz_minuteswest: crate::ctypes::c_int,
    pub tz_dsttime: crate::ctypes::c_int,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> timezone`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iovec`

```rust
struct iovec {
    pub iov_base: *mut crate::ctypes::c_void,
    pub iov_len: __kernel_size_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> iovec`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `dmabuf_cmsg`

```rust
struct dmabuf_cmsg {
    pub frag_offset: __u64,
    pub frag_size: __u32,
    pub frag_token: __u32,
    pub dmabuf_id: __u32,
    pub flags: __u32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> dmabuf_cmsg`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `dmabuf_token`

```rust
struct dmabuf_token {
    pub token_start: __u32,
    pub token_count: __u32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> dmabuf_token`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xattr_args`

```rust
struct xattr_args {
    pub value: __u64,
    pub size: __u32,
    pub flags: __u32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> xattr_args`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffd_msg`

```rust
struct uffd_msg {
    pub event: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub reserved3: __u32,
    pub arg: uffd_msg__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffd_msg`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `uffd_msg__bindgen_ty_1__bindgen_ty_1`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_1 {
    pub flags: __u64,
    pub address: __u64,
    pub feat: uffd_msg__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffd_msg__bindgen_ty_1__bindgen_ty_1`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

### `uffd_msg__bindgen_ty_1__bindgen_ty_2`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_2 {
    pub ufd: __u32,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffd_msg__bindgen_ty_1__bindgen_ty_2`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffd_msg__bindgen_ty_1__bindgen_ty_3`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_3 {
    pub from: __u64,
    pub to: __u64,
    pub len: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffd_msg__bindgen_ty_1__bindgen_ty_3`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffd_msg__bindgen_ty_1__bindgen_ty_4`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_4 {
    pub start: __u64,
    pub end: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffd_msg__bindgen_ty_1__bindgen_ty_4`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffd_msg__bindgen_ty_1__bindgen_ty_5`

```rust
struct uffd_msg__bindgen_ty_1__bindgen_ty_5 {
    pub reserved1: __u64,
    pub reserved2: __u64,
    pub reserved3: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffd_msg__bindgen_ty_1__bindgen_ty_5`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffdio_api`

```rust
struct uffdio_api {
    pub api: __u64,
    pub features: __u64,
    pub ioctls: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffdio_api`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffdio_range`

```rust
struct uffdio_range {
    pub start: __u64,
    pub len: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffdio_range`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffdio_register`

```rust
struct uffdio_register {
    pub range: uffdio_range,
    pub mode: __u64,
    pub ioctls: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffdio_register`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffdio_copy`

```rust
struct uffdio_copy {
    pub dst: __u64,
    pub src: __u64,
    pub len: __u64,
    pub mode: __u64,
    pub copy: __s64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffdio_copy`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffdio_zeropage`

```rust
struct uffdio_zeropage {
    pub range: uffdio_range,
    pub mode: __u64,
    pub zeropage: __s64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffdio_zeropage`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffdio_writeprotect`

```rust
struct uffdio_writeprotect {
    pub range: uffdio_range,
    pub mode: __u64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffdio_writeprotect`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffdio_continue`

```rust
struct uffdio_continue {
    pub range: uffdio_range,
    pub mode: __u64,
    pub mapped: __s64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffdio_continue`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffdio_poison`

```rust
struct uffdio_poison {
    pub range: uffdio_range,
    pub mode: __u64,
    pub updated: __s64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffdio_poison`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uffdio_move`

```rust
struct uffdio_move {
    pub dst: __u64,
    pub src: __u64,
    pub len: __u64,
    pub mode: __u64,
    pub move_: __s64,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> uffdio_move`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `linux_dirent64`

```rust
struct linux_dirent64 {
    pub d_ino: crate::ctypes::c_ulong,
    pub d_off: crate::ctypes::c_long,
    pub d_reclen: __u16,
    pub d_type: __u8,
    pub d_name: __IncompleteArrayField<crate::ctypes::c_char>,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `stat`

```rust
struct stat {
    pub st_dev: __kernel_ulong_t,
    pub st_ino: __kernel_ulong_t,
    pub st_nlink: __kernel_ulong_t,
    pub st_mode: crate::ctypes::c_uint,
    pub st_uid: crate::ctypes::c_uint,
    pub st_gid: crate::ctypes::c_uint,
    pub __pad0: crate::ctypes::c_uint,
    pub st_rdev: __kernel_ulong_t,
    pub st_size: __kernel_long_t,
    pub st_blksize: __kernel_long_t,
    pub st_blocks: __kernel_long_t,
    pub st_atime: __kernel_ulong_t,
    pub st_atime_nsec: __kernel_ulong_t,
    pub st_mtime: __kernel_ulong_t,
    pub st_mtime_nsec: __kernel_ulong_t,
    pub st_ctime: __kernel_ulong_t,
    pub st_ctime_nsec: __kernel_ulong_t,
    pub __unused: [__kernel_long_t; 3],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> stat`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__old_kernel_stat`

```rust
struct __old_kernel_stat {
    pub st_dev: crate::ctypes::c_ushort,
    pub st_ino: crate::ctypes::c_ushort,
    pub st_mode: crate::ctypes::c_ushort,
    pub st_nlink: crate::ctypes::c_ushort,
    pub st_uid: crate::ctypes::c_ushort,
    pub st_gid: crate::ctypes::c_ushort,
    pub st_rdev: crate::ctypes::c_ushort,
    pub st_size: crate::ctypes::c_uint,
    pub st_atime: crate::ctypes::c_uint,
    pub st_mtime: crate::ctypes::c_uint,
    pub st_ctime: crate::ctypes::c_uint,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> __old_kernel_stat`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `statfs`

```rust
struct statfs {
    pub f_type: __kernel_long_t,
    pub f_bsize: __kernel_long_t,
    pub f_blocks: __kernel_long_t,
    pub f_bfree: __kernel_long_t,
    pub f_bavail: __kernel_long_t,
    pub f_files: __kernel_long_t,
    pub f_ffree: __kernel_long_t,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __kernel_long_t,
    pub f_frsize: __kernel_long_t,
    pub f_flags: __kernel_long_t,
    pub f_spare: [__kernel_long_t; 4],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> statfs`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `statfs64`

```rust
struct statfs64 {
    pub f_type: __kernel_long_t,
    pub f_bsize: __kernel_long_t,
    pub f_blocks: __u64,
    pub f_bfree: __u64,
    pub f_bavail: __u64,
    pub f_files: __u64,
    pub f_ffree: __u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __kernel_long_t,
    pub f_frsize: __kernel_long_t,
    pub f_flags: __kernel_long_t,
    pub f_spare: [__kernel_long_t; 4],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> statfs64`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `compat_statfs64`

```rust
struct compat_statfs64 {
    pub f_type: __u32,
    pub f_bsize: __u32,
    pub f_blocks: __u64,
    pub f_bfree: __u64,
    pub f_bavail: __u64,
    pub f_files: __u64,
    pub f_ffree: __u64,
    pub f_fsid: __kernel_fsid_t,
    pub f_namelen: __u32,
    pub f_frsize: __u32,
    pub f_flags: __u32,
    pub f_spare: [__u32; 4],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> compat_statfs64`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `user_desc`

```rust
struct user_desc {
    pub entry_number: crate::ctypes::c_uint,
    pub base_addr: crate::ctypes::c_uint,
    pub limit: crate::ctypes::c_uint,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1]>,
    pub __bindgen_padding_0: [u8; 3],
}
```

#### Implementations

- `fn seg_32bit(self: &Self) -> crate::ctypes::c_uint`

- `fn set_seg_32bit(self: &mut Self, val: crate::ctypes::c_uint)`

- `unsafe fn seg_32bit_raw(this: *const Self) -> crate::ctypes::c_uint`

- `unsafe fn set_seg_32bit_raw(this: *mut Self, val: crate::ctypes::c_uint)`

- `fn contents(self: &Self) -> crate::ctypes::c_uint`

- `fn set_contents(self: &mut Self, val: crate::ctypes::c_uint)`

- `unsafe fn contents_raw(this: *const Self) -> crate::ctypes::c_uint`

- `unsafe fn set_contents_raw(this: *mut Self, val: crate::ctypes::c_uint)`

- `fn read_exec_only(self: &Self) -> crate::ctypes::c_uint`

- `fn set_read_exec_only(self: &mut Self, val: crate::ctypes::c_uint)`

- `unsafe fn read_exec_only_raw(this: *const Self) -> crate::ctypes::c_uint`

- `unsafe fn set_read_exec_only_raw(this: *mut Self, val: crate::ctypes::c_uint)`

- `fn limit_in_pages(self: &Self) -> crate::ctypes::c_uint`

- `fn set_limit_in_pages(self: &mut Self, val: crate::ctypes::c_uint)`

- `unsafe fn limit_in_pages_raw(this: *const Self) -> crate::ctypes::c_uint`

- `unsafe fn set_limit_in_pages_raw(this: *mut Self, val: crate::ctypes::c_uint)`

- `fn seg_not_present(self: &Self) -> crate::ctypes::c_uint`

- `fn set_seg_not_present(self: &mut Self, val: crate::ctypes::c_uint)`

- `unsafe fn seg_not_present_raw(this: *const Self) -> crate::ctypes::c_uint`

- `unsafe fn set_seg_not_present_raw(this: *mut Self, val: crate::ctypes::c_uint)`

- `fn useable(self: &Self) -> crate::ctypes::c_uint`

- `fn set_useable(self: &mut Self, val: crate::ctypes::c_uint)`

- `unsafe fn useable_raw(this: *const Self) -> crate::ctypes::c_uint`

- `unsafe fn set_useable_raw(this: *mut Self, val: crate::ctypes::c_uint)`

- `fn lm(self: &Self) -> crate::ctypes::c_uint`

- `fn set_lm(self: &mut Self, val: crate::ctypes::c_uint)`

- `unsafe fn lm_raw(this: *const Self) -> crate::ctypes::c_uint`

- `unsafe fn set_lm_raw(this: *mut Self, val: crate::ctypes::c_uint)`

- `fn new_bitfield_1(seg_32bit: crate::ctypes::c_uint, contents: crate::ctypes::c_uint, read_exec_only: crate::ctypes::c_uint, limit_in_pages: crate::ctypes::c_uint, seg_not_present: crate::ctypes::c_uint, useable: crate::ctypes::c_uint, lm: crate::ctypes::c_uint) -> __BindgenBitfieldUnit<[u8; 1]>`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> user_desc`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `kernel_sigset_t`

```rust
struct kernel_sigset_t {
    pub sig: [crate::ctypes::c_ulong; 1],
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> kernel_sigset_t`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `kernel_sigaction`

```rust
struct kernel_sigaction {
    pub sa_handler_kernel: __kernel_sighandler_t,
    pub sa_flags: crate::ctypes::c_ulong,
    pub sa_restorer: __sigrestore_t,
    pub sa_mask: kernel_sigset_t,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> kernel_sigaction`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `fsconfig_command`

```rust
enum fsconfig_command {
    FSCONFIG_SET_FLAG,
    FSCONFIG_SET_STRING,
    FSCONFIG_SET_BINARY,
    FSCONFIG_SET_PATH,
    FSCONFIG_SET_PATH_EMPTY,
    FSCONFIG_SET_FD,
    FSCONFIG_CMD_CREATE,
    FSCONFIG_CMD_RECONFIGURE,
    FSCONFIG_CMD_CREATE_EXCL,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> fsconfig_command`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &fsconfig_command) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `procmap_query_flags`

```rust
enum procmap_query_flags {
    PROCMAP_QUERY_VMA_READABLE,
    PROCMAP_QUERY_VMA_WRITABLE,
    PROCMAP_QUERY_VMA_EXECUTABLE,
    PROCMAP_QUERY_VMA_SHARED,
    PROCMAP_QUERY_COVERING_OR_NEXT_VMA,
    PROCMAP_QUERY_FILE_BACKED_VMA,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> procmap_query_flags`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &procmap_query_flags) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `membarrier_cmd`

```rust
enum membarrier_cmd {
    MEMBARRIER_CMD_QUERY,
    MEMBARRIER_CMD_GLOBAL,
    MEMBARRIER_CMD_GLOBAL_EXPEDITED,
    MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ,
    MEMBARRIER_CMD_GET_REGISTRATIONS,
}
```

#### Implementations

- `const MEMBARRIER_CMD_SHARED: membarrier_cmd`

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> membarrier_cmd`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &membarrier_cmd) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `membarrier_cmd_flag`

```rust
enum membarrier_cmd_flag {
    MEMBARRIER_CMD_FLAG_CPU,
}
```

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> membarrier_cmd_flag`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &membarrier_cmd_flag) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Type Aliases

### `__s8`

```rust
type __s8 = crate::ctypes::c_schar;
```

### `__u8`

```rust
type __u8 = crate::ctypes::c_uchar;
```

### `__s16`

```rust
type __s16 = crate::ctypes::c_short;
```

### `__u16`

```rust
type __u16 = crate::ctypes::c_ushort;
```

### `__s32`

```rust
type __s32 = crate::ctypes::c_int;
```

### `__u32`

```rust
type __u32 = crate::ctypes::c_uint;
```

### `__s64`

```rust
type __s64 = crate::ctypes::c_longlong;
```

### `__u64`

```rust
type __u64 = crate::ctypes::c_ulonglong;
```

### `__kernel_sighandler_t`

```rust
type __kernel_sighandler_t = ::core::option::Option<fn(crate::ctypes::c_int)>;
```

### `__kernel_key_t`

```rust
type __kernel_key_t = crate::ctypes::c_int;
```

### `__kernel_mqd_t`

```rust
type __kernel_mqd_t = crate::ctypes::c_int;
```

### `__kernel_old_uid_t`

```rust
type __kernel_old_uid_t = crate::ctypes::c_ushort;
```

### `__kernel_old_gid_t`

```rust
type __kernel_old_gid_t = crate::ctypes::c_ushort;
```

### `__kernel_old_dev_t`

```rust
type __kernel_old_dev_t = crate::ctypes::c_ulong;
```

### `__kernel_long_t`

```rust
type __kernel_long_t = crate::ctypes::c_long;
```

### `__kernel_ulong_t`

```rust
type __kernel_ulong_t = crate::ctypes::c_ulong;
```

### `__kernel_ino_t`

```rust
type __kernel_ino_t = __kernel_ulong_t;
```

### `__kernel_mode_t`

```rust
type __kernel_mode_t = crate::ctypes::c_uint;
```

### `__kernel_pid_t`

```rust
type __kernel_pid_t = crate::ctypes::c_int;
```

### `__kernel_ipc_pid_t`

```rust
type __kernel_ipc_pid_t = crate::ctypes::c_int;
```

### `__kernel_uid_t`

```rust
type __kernel_uid_t = crate::ctypes::c_uint;
```

### `__kernel_gid_t`

```rust
type __kernel_gid_t = crate::ctypes::c_uint;
```

### `__kernel_suseconds_t`

```rust
type __kernel_suseconds_t = __kernel_long_t;
```

### `__kernel_daddr_t`

```rust
type __kernel_daddr_t = crate::ctypes::c_int;
```

### `__kernel_uid32_t`

```rust
type __kernel_uid32_t = crate::ctypes::c_uint;
```

### `__kernel_gid32_t`

```rust
type __kernel_gid32_t = crate::ctypes::c_uint;
```

### `__kernel_size_t`

```rust
type __kernel_size_t = __kernel_ulong_t;
```

### `__kernel_ssize_t`

```rust
type __kernel_ssize_t = __kernel_long_t;
```

### `__kernel_ptrdiff_t`

```rust
type __kernel_ptrdiff_t = __kernel_long_t;
```

### `__kernel_off_t`

```rust
type __kernel_off_t = __kernel_long_t;
```

### `__kernel_loff_t`

```rust
type __kernel_loff_t = crate::ctypes::c_longlong;
```

### `__kernel_old_time_t`

```rust
type __kernel_old_time_t = __kernel_long_t;
```

### `__kernel_time_t`

```rust
type __kernel_time_t = __kernel_long_t;
```

### `__kernel_time64_t`

```rust
type __kernel_time64_t = crate::ctypes::c_longlong;
```

### `__kernel_clock_t`

```rust
type __kernel_clock_t = __kernel_long_t;
```

### `__kernel_timer_t`

```rust
type __kernel_timer_t = crate::ctypes::c_int;
```

### `__kernel_clockid_t`

```rust
type __kernel_clockid_t = crate::ctypes::c_int;
```

### `__kernel_caddr_t`

```rust
type __kernel_caddr_t = *mut crate::ctypes::c_char;
```

### `__kernel_uid16_t`

```rust
type __kernel_uid16_t = crate::ctypes::c_ushort;
```

### `__kernel_gid16_t`

```rust
type __kernel_gid16_t = crate::ctypes::c_ushort;
```

### `__s128`

```rust
type __s128 = i128;
```

### `__u128`

```rust
type __u128 = u128;
```

### `__le16`

```rust
type __le16 = __u16;
```

### `__be16`

```rust
type __be16 = __u16;
```

### `__le32`

```rust
type __le32 = __u32;
```

### `__be32`

```rust
type __be32 = __u32;
```

### `__le64`

```rust
type __le64 = __u64;
```

### `__be64`

```rust
type __be64 = __u64;
```

### `__sum16`

```rust
type __sum16 = __u16;
```

### `__wsum`

```rust
type __wsum = __u32;
```

### `__poll_t`

```rust
type __poll_t = crate::ctypes::c_uint;
```

### `cap_user_header_t`

```rust
type cap_user_header_t = *mut __user_cap_header_struct;
```

### `cap_user_data_t`

```rust
type cap_user_data_t = *mut __user_cap_data_struct;
```

### `__kernel_rwf_t`

```rust
type __kernel_rwf_t = crate::ctypes::c_int;
```

### `sigset_t`

```rust
type sigset_t = crate::ctypes::c_ulong;
```

### `__signalfn_t`

```rust
type __signalfn_t = ::core::option::Option<fn(crate::ctypes::c_int)>;
```

### `__sighandler_t`

```rust
type __sighandler_t = __signalfn_t;
```

### `__restorefn_t`

```rust
type __restorefn_t = ::core::option::Option<fn()>;
```

### `__sigrestore_t`

```rust
type __sigrestore_t = __restorefn_t;
```

### `stack_t`

```rust
type stack_t = sigaltstack;
```

### `sigval_t`

```rust
type sigval_t = sigval;
```

### `siginfo_t`

```rust
type siginfo_t = siginfo;
```

### `sigevent_t`

```rust
type sigevent_t = sigevent;
```

### `cc_t`

```rust
type cc_t = crate::ctypes::c_uchar;
```

### `speed_t`

```rust
type speed_t = crate::ctypes::c_uint;
```

### `tcflag_t`

```rust
type tcflag_t = crate::ctypes::c_uint;
```

### `__fsword_t`

```rust
type __fsword_t = __kernel_long_t;
```

## Constants

### `LINUX_VERSION_CODE`

```rust
const LINUX_VERSION_CODE: u32 = 397_312u32;
```

### `LINUX_VERSION_MAJOR`

```rust
const LINUX_VERSION_MAJOR: u32 = 6u32;
```

### `LINUX_VERSION_PATCHLEVEL`

```rust
const LINUX_VERSION_PATCHLEVEL: u32 = 16u32;
```

### `LINUX_VERSION_SUBLEVEL`

```rust
const LINUX_VERSION_SUBLEVEL: u32 = 0u32;
```

### `__BITS_PER_LONG_LONG`

```rust
const __BITS_PER_LONG_LONG: u32 = 64u32;
```

### `__FD_SETSIZE`

```rust
const __FD_SETSIZE: u32 = 1_024u32;
```

### `_LINUX_CAPABILITY_VERSION_1`

```rust
const _LINUX_CAPABILITY_VERSION_1: u32 = 429_392_688u32;
```

### `_LINUX_CAPABILITY_U32S_1`

```rust
const _LINUX_CAPABILITY_U32S_1: u32 = 1u32;
```

### `_LINUX_CAPABILITY_VERSION_2`

```rust
const _LINUX_CAPABILITY_VERSION_2: u32 = 537_333_798u32;
```

### `_LINUX_CAPABILITY_U32S_2`

```rust
const _LINUX_CAPABILITY_U32S_2: u32 = 2u32;
```

### `_LINUX_CAPABILITY_VERSION_3`

```rust
const _LINUX_CAPABILITY_VERSION_3: u32 = 537_396_514u32;
```

### `_LINUX_CAPABILITY_U32S_3`

```rust
const _LINUX_CAPABILITY_U32S_3: u32 = 2u32;
```

### `VFS_CAP_REVISION_MASK`

```rust
const VFS_CAP_REVISION_MASK: u32 = 4_278_190_080u32;
```

### `VFS_CAP_REVISION_SHIFT`

```rust
const VFS_CAP_REVISION_SHIFT: u32 = 24u32;
```

### `VFS_CAP_FLAGS_MASK`

```rust
const VFS_CAP_FLAGS_MASK: i64 = -4_278_190_081i64;
```

### `VFS_CAP_FLAGS_EFFECTIVE`

```rust
const VFS_CAP_FLAGS_EFFECTIVE: u32 = 1u32;
```

### `VFS_CAP_REVISION_1`

```rust
const VFS_CAP_REVISION_1: u32 = 16_777_216u32;
```

### `VFS_CAP_U32_1`

```rust
const VFS_CAP_U32_1: u32 = 1u32;
```

### `VFS_CAP_REVISION_2`

```rust
const VFS_CAP_REVISION_2: u32 = 33_554_432u32;
```

### `VFS_CAP_U32_2`

```rust
const VFS_CAP_U32_2: u32 = 2u32;
```

### `VFS_CAP_REVISION_3`

```rust
const VFS_CAP_REVISION_3: u32 = 50_331_648u32;
```

### `VFS_CAP_U32_3`

```rust
const VFS_CAP_U32_3: u32 = 2u32;
```

### `VFS_CAP_U32`

```rust
const VFS_CAP_U32: u32 = 2u32;
```

### `VFS_CAP_REVISION`

```rust
const VFS_CAP_REVISION: u32 = 50_331_648u32;
```

### `_LINUX_CAPABILITY_VERSION`

```rust
const _LINUX_CAPABILITY_VERSION: u32 = 429_392_688u32;
```

### `_LINUX_CAPABILITY_U32S`

```rust
const _LINUX_CAPABILITY_U32S: u32 = 1u32;
```

### `CAP_CHOWN`

```rust
const CAP_CHOWN: u32 = 0u32;
```

### `CAP_DAC_OVERRIDE`

```rust
const CAP_DAC_OVERRIDE: u32 = 1u32;
```

### `CAP_DAC_READ_SEARCH`

```rust
const CAP_DAC_READ_SEARCH: u32 = 2u32;
```

### `CAP_FOWNER`

```rust
const CAP_FOWNER: u32 = 3u32;
```

### `CAP_FSETID`

```rust
const CAP_FSETID: u32 = 4u32;
```

### `CAP_KILL`

```rust
const CAP_KILL: u32 = 5u32;
```

### `CAP_SETGID`

```rust
const CAP_SETGID: u32 = 6u32;
```

### `CAP_SETUID`

```rust
const CAP_SETUID: u32 = 7u32;
```

### `CAP_SETPCAP`

```rust
const CAP_SETPCAP: u32 = 8u32;
```

### `CAP_LINUX_IMMUTABLE`

```rust
const CAP_LINUX_IMMUTABLE: u32 = 9u32;
```

### `CAP_NET_BIND_SERVICE`

```rust
const CAP_NET_BIND_SERVICE: u32 = 10u32;
```

### `CAP_NET_BROADCAST`

```rust
const CAP_NET_BROADCAST: u32 = 11u32;
```

### `CAP_NET_ADMIN`

```rust
const CAP_NET_ADMIN: u32 = 12u32;
```

### `CAP_NET_RAW`

```rust
const CAP_NET_RAW: u32 = 13u32;
```

### `CAP_IPC_LOCK`

```rust
const CAP_IPC_LOCK: u32 = 14u32;
```

### `CAP_IPC_OWNER`

```rust
const CAP_IPC_OWNER: u32 = 15u32;
```

### `CAP_SYS_MODULE`

```rust
const CAP_SYS_MODULE: u32 = 16u32;
```

### `CAP_SYS_RAWIO`

```rust
const CAP_SYS_RAWIO: u32 = 17u32;
```

### `CAP_SYS_CHROOT`

```rust
const CAP_SYS_CHROOT: u32 = 18u32;
```

### `CAP_SYS_PTRACE`

```rust
const CAP_SYS_PTRACE: u32 = 19u32;
```

### `CAP_SYS_PACCT`

```rust
const CAP_SYS_PACCT: u32 = 20u32;
```

### `CAP_SYS_ADMIN`

```rust
const CAP_SYS_ADMIN: u32 = 21u32;
```

### `CAP_SYS_BOOT`

```rust
const CAP_SYS_BOOT: u32 = 22u32;
```

### `CAP_SYS_NICE`

```rust
const CAP_SYS_NICE: u32 = 23u32;
```

### `CAP_SYS_RESOURCE`

```rust
const CAP_SYS_RESOURCE: u32 = 24u32;
```

### `CAP_SYS_TIME`

```rust
const CAP_SYS_TIME: u32 = 25u32;
```

### `CAP_SYS_TTY_CONFIG`

```rust
const CAP_SYS_TTY_CONFIG: u32 = 26u32;
```

### `CAP_MKNOD`

```rust
const CAP_MKNOD: u32 = 27u32;
```

### `CAP_LEASE`

```rust
const CAP_LEASE: u32 = 28u32;
```

### `CAP_AUDIT_WRITE`

```rust
const CAP_AUDIT_WRITE: u32 = 29u32;
```

### `CAP_AUDIT_CONTROL`

```rust
const CAP_AUDIT_CONTROL: u32 = 30u32;
```

### `CAP_SETFCAP`

```rust
const CAP_SETFCAP: u32 = 31u32;
```

### `CAP_MAC_OVERRIDE`

```rust
const CAP_MAC_OVERRIDE: u32 = 32u32;
```

### `CAP_MAC_ADMIN`

```rust
const CAP_MAC_ADMIN: u32 = 33u32;
```

### `CAP_SYSLOG`

```rust
const CAP_SYSLOG: u32 = 34u32;
```

### `CAP_WAKE_ALARM`

```rust
const CAP_WAKE_ALARM: u32 = 35u32;
```

### `CAP_BLOCK_SUSPEND`

```rust
const CAP_BLOCK_SUSPEND: u32 = 36u32;
```

### `CAP_AUDIT_READ`

```rust
const CAP_AUDIT_READ: u32 = 37u32;
```

### `CAP_PERFMON`

```rust
const CAP_PERFMON: u32 = 38u32;
```

### `CAP_BPF`

```rust
const CAP_BPF: u32 = 39u32;
```

### `CAP_CHECKPOINT_RESTORE`

```rust
const CAP_CHECKPOINT_RESTORE: u32 = 40u32;
```

### `CAP_LAST_CAP`

```rust
const CAP_LAST_CAP: u32 = 40u32;
```

### `O_ACCMODE`

```rust
const O_ACCMODE: u32 = 3u32;
```

### `O_RDONLY`

```rust
const O_RDONLY: u32 = 0u32;
```

### `O_WRONLY`

```rust
const O_WRONLY: u32 = 1u32;
```

### `O_RDWR`

```rust
const O_RDWR: u32 = 2u32;
```

### `O_CREAT`

```rust
const O_CREAT: u32 = 64u32;
```

### `O_EXCL`

```rust
const O_EXCL: u32 = 128u32;
```

### `O_NOCTTY`

```rust
const O_NOCTTY: u32 = 256u32;
```

### `O_TRUNC`

```rust
const O_TRUNC: u32 = 512u32;
```

### `O_APPEND`

```rust
const O_APPEND: u32 = 1_024u32;
```

### `O_NONBLOCK`

```rust
const O_NONBLOCK: u32 = 2_048u32;
```

### `O_DSYNC`

```rust
const O_DSYNC: u32 = 4_096u32;
```

### `FASYNC`

```rust
const FASYNC: u32 = 8_192u32;
```

### `O_DIRECT`

```rust
const O_DIRECT: u32 = 16_384u32;
```

### `O_LARGEFILE`

```rust
const O_LARGEFILE: u32 = 32_768u32;
```

### `O_DIRECTORY`

```rust
const O_DIRECTORY: u32 = 65_536u32;
```

### `O_NOFOLLOW`

```rust
const O_NOFOLLOW: u32 = 131_072u32;
```

### `O_NOATIME`

```rust
const O_NOATIME: u32 = 262_144u32;
```

### `O_CLOEXEC`

```rust
const O_CLOEXEC: u32 = 524_288u32;
```

### `__O_SYNC`

```rust
const __O_SYNC: u32 = 1_048_576u32;
```

### `O_SYNC`

```rust
const O_SYNC: u32 = 1_052_672u32;
```

### `O_PATH`

```rust
const O_PATH: u32 = 2_097_152u32;
```

### `__O_TMPFILE`

```rust
const __O_TMPFILE: u32 = 4_194_304u32;
```

### `O_TMPFILE`

```rust
const O_TMPFILE: u32 = 4_259_840u32;
```

### `O_NDELAY`

```rust
const O_NDELAY: u32 = 2_048u32;
```

### `F_DUPFD`

```rust
const F_DUPFD: u32 = 0u32;
```

### `F_GETFD`

```rust
const F_GETFD: u32 = 1u32;
```

### `F_SETFD`

```rust
const F_SETFD: u32 = 2u32;
```

### `F_GETFL`

```rust
const F_GETFL: u32 = 3u32;
```

### `F_SETFL`

```rust
const F_SETFL: u32 = 4u32;
```

### `F_GETLK`

```rust
const F_GETLK: u32 = 5u32;
```

### `F_SETLK`

```rust
const F_SETLK: u32 = 6u32;
```

### `F_SETLKW`

```rust
const F_SETLKW: u32 = 7u32;
```

### `F_SETOWN`

```rust
const F_SETOWN: u32 = 8u32;
```

### `F_GETOWN`

```rust
const F_GETOWN: u32 = 9u32;
```

### `F_SETSIG`

```rust
const F_SETSIG: u32 = 10u32;
```

### `F_GETSIG`

```rust
const F_GETSIG: u32 = 11u32;
```

### `F_SETOWN_EX`

```rust
const F_SETOWN_EX: u32 = 15u32;
```

### `F_GETOWN_EX`

```rust
const F_GETOWN_EX: u32 = 16u32;
```

### `F_GETOWNER_UIDS`

```rust
const F_GETOWNER_UIDS: u32 = 17u32;
```

### `F_OFD_GETLK`

```rust
const F_OFD_GETLK: u32 = 36u32;
```

### `F_OFD_SETLK`

```rust
const F_OFD_SETLK: u32 = 37u32;
```

### `F_OFD_SETLKW`

```rust
const F_OFD_SETLKW: u32 = 38u32;
```

### `F_OWNER_TID`

```rust
const F_OWNER_TID: u32 = 0u32;
```

### `F_OWNER_PID`

```rust
const F_OWNER_PID: u32 = 1u32;
```

### `F_OWNER_PGRP`

```rust
const F_OWNER_PGRP: u32 = 2u32;
```

### `FD_CLOEXEC`

```rust
const FD_CLOEXEC: u32 = 1u32;
```

### `F_RDLCK`

```rust
const F_RDLCK: u32 = 0u32;
```

### `F_WRLCK`

```rust
const F_WRLCK: u32 = 1u32;
```

### `F_UNLCK`

```rust
const F_UNLCK: u32 = 2u32;
```

### `F_EXLCK`

```rust
const F_EXLCK: u32 = 4u32;
```

### `F_SHLCK`

```rust
const F_SHLCK: u32 = 8u32;
```

### `LOCK_SH`

```rust
const LOCK_SH: u32 = 1u32;
```

### `LOCK_EX`

```rust
const LOCK_EX: u32 = 2u32;
```

### `LOCK_NB`

```rust
const LOCK_NB: u32 = 4u32;
```

### `LOCK_UN`

```rust
const LOCK_UN: u32 = 8u32;
```

### `LOCK_MAND`

```rust
const LOCK_MAND: u32 = 32u32;
```

### `LOCK_READ`

```rust
const LOCK_READ: u32 = 64u32;
```

### `LOCK_WRITE`

```rust
const LOCK_WRITE: u32 = 128u32;
```

### `LOCK_RW`

```rust
const LOCK_RW: u32 = 192u32;
```

### `F_LINUX_SPECIFIC_BASE`

```rust
const F_LINUX_SPECIFIC_BASE: u32 = 1_024u32;
```

### `RESOLVE_NO_XDEV`

```rust
const RESOLVE_NO_XDEV: u32 = 1u32;
```

### `RESOLVE_NO_MAGICLINKS`

```rust
const RESOLVE_NO_MAGICLINKS: u32 = 2u32;
```

### `RESOLVE_NO_SYMLINKS`

```rust
const RESOLVE_NO_SYMLINKS: u32 = 4u32;
```

### `RESOLVE_BENEATH`

```rust
const RESOLVE_BENEATH: u32 = 8u32;
```

### `RESOLVE_IN_ROOT`

```rust
const RESOLVE_IN_ROOT: u32 = 16u32;
```

### `RESOLVE_CACHED`

```rust
const RESOLVE_CACHED: u32 = 32u32;
```

### `F_SETLEASE`

```rust
const F_SETLEASE: u32 = 1_024u32;
```

### `F_GETLEASE`

```rust
const F_GETLEASE: u32 = 1_025u32;
```

### `F_NOTIFY`

```rust
const F_NOTIFY: u32 = 1_026u32;
```

### `F_DUPFD_QUERY`

```rust
const F_DUPFD_QUERY: u32 = 1_027u32;
```

### `F_CREATED_QUERY`

```rust
const F_CREATED_QUERY: u32 = 1_028u32;
```

### `F_CANCELLK`

```rust
const F_CANCELLK: u32 = 1_029u32;
```

### `F_DUPFD_CLOEXEC`

```rust
const F_DUPFD_CLOEXEC: u32 = 1_030u32;
```

### `F_SETPIPE_SZ`

```rust
const F_SETPIPE_SZ: u32 = 1_031u32;
```

### `F_GETPIPE_SZ`

```rust
const F_GETPIPE_SZ: u32 = 1_032u32;
```

### `F_ADD_SEALS`

```rust
const F_ADD_SEALS: u32 = 1_033u32;
```

### `F_GET_SEALS`

```rust
const F_GET_SEALS: u32 = 1_034u32;
```

### `F_SEAL_SEAL`

```rust
const F_SEAL_SEAL: u32 = 1u32;
```

### `F_SEAL_SHRINK`

```rust
const F_SEAL_SHRINK: u32 = 2u32;
```

### `F_SEAL_GROW`

```rust
const F_SEAL_GROW: u32 = 4u32;
```

### `F_SEAL_WRITE`

```rust
const F_SEAL_WRITE: u32 = 8u32;
```

### `F_SEAL_FUTURE_WRITE`

```rust
const F_SEAL_FUTURE_WRITE: u32 = 16u32;
```

### `F_SEAL_EXEC`

```rust
const F_SEAL_EXEC: u32 = 32u32;
```

### `F_GET_RW_HINT`

```rust
const F_GET_RW_HINT: u32 = 1_035u32;
```

### `F_SET_RW_HINT`

```rust
const F_SET_RW_HINT: u32 = 1_036u32;
```

### `F_GET_FILE_RW_HINT`

```rust
const F_GET_FILE_RW_HINT: u32 = 1_037u32;
```

### `F_SET_FILE_RW_HINT`

```rust
const F_SET_FILE_RW_HINT: u32 = 1_038u32;
```

### `RWH_WRITE_LIFE_NOT_SET`

```rust
const RWH_WRITE_LIFE_NOT_SET: u32 = 0u32;
```

### `RWH_WRITE_LIFE_NONE`

```rust
const RWH_WRITE_LIFE_NONE: u32 = 1u32;
```

### `RWH_WRITE_LIFE_SHORT`

```rust
const RWH_WRITE_LIFE_SHORT: u32 = 2u32;
```

### `RWH_WRITE_LIFE_MEDIUM`

```rust
const RWH_WRITE_LIFE_MEDIUM: u32 = 3u32;
```

### `RWH_WRITE_LIFE_LONG`

```rust
const RWH_WRITE_LIFE_LONG: u32 = 4u32;
```

### `RWH_WRITE_LIFE_EXTREME`

```rust
const RWH_WRITE_LIFE_EXTREME: u32 = 5u32;
```

### `RWF_WRITE_LIFE_NOT_SET`

```rust
const RWF_WRITE_LIFE_NOT_SET: u32 = 0u32;
```

### `DN_ACCESS`

```rust
const DN_ACCESS: u32 = 1u32;
```

### `DN_MODIFY`

```rust
const DN_MODIFY: u32 = 2u32;
```

### `DN_CREATE`

```rust
const DN_CREATE: u32 = 4u32;
```

### `DN_DELETE`

```rust
const DN_DELETE: u32 = 8u32;
```

### `DN_RENAME`

```rust
const DN_RENAME: u32 = 16u32;
```

### `DN_ATTRIB`

```rust
const DN_ATTRIB: u32 = 32u32;
```

### `DN_MULTISHOT`

```rust
const DN_MULTISHOT: u32 = 2_147_483_648u32;
```

### `AT_FDCWD`

```rust
const AT_FDCWD: i32 = -100i32;
```

### `AT_SYMLINK_NOFOLLOW`

```rust
const AT_SYMLINK_NOFOLLOW: u32 = 256u32;
```

### `AT_SYMLINK_FOLLOW`

```rust
const AT_SYMLINK_FOLLOW: u32 = 1_024u32;
```

### `AT_NO_AUTOMOUNT`

```rust
const AT_NO_AUTOMOUNT: u32 = 2_048u32;
```

### `AT_EMPTY_PATH`

```rust
const AT_EMPTY_PATH: u32 = 4_096u32;
```

### `AT_STATX_SYNC_TYPE`

```rust
const AT_STATX_SYNC_TYPE: u32 = 24_576u32;
```

### `AT_STATX_SYNC_AS_STAT`

```rust
const AT_STATX_SYNC_AS_STAT: u32 = 0u32;
```

### `AT_STATX_FORCE_SYNC`

```rust
const AT_STATX_FORCE_SYNC: u32 = 8_192u32;
```

### `AT_STATX_DONT_SYNC`

```rust
const AT_STATX_DONT_SYNC: u32 = 16_384u32;
```

### `AT_RECURSIVE`

```rust
const AT_RECURSIVE: u32 = 32_768u32;
```

### `AT_RENAME_NOREPLACE`

```rust
const AT_RENAME_NOREPLACE: u32 = 1u32;
```

### `AT_RENAME_EXCHANGE`

```rust
const AT_RENAME_EXCHANGE: u32 = 2u32;
```

### `AT_RENAME_WHITEOUT`

```rust
const AT_RENAME_WHITEOUT: u32 = 4u32;
```

### `AT_EACCESS`

```rust
const AT_EACCESS: u32 = 512u32;
```

### `AT_REMOVEDIR`

```rust
const AT_REMOVEDIR: u32 = 512u32;
```

### `AT_HANDLE_FID`

```rust
const AT_HANDLE_FID: u32 = 512u32;
```

### `AT_HANDLE_MNT_ID_UNIQUE`

```rust
const AT_HANDLE_MNT_ID_UNIQUE: u32 = 1u32;
```

### `AT_HANDLE_CONNECTABLE`

```rust
const AT_HANDLE_CONNECTABLE: u32 = 2u32;
```

### `AT_EXECVE_CHECK`

```rust
const AT_EXECVE_CHECK: u32 = 65_536u32;
```

### `EPOLL_CLOEXEC`

```rust
const EPOLL_CLOEXEC: u32 = 524_288u32;
```

### `EPOLL_CTL_ADD`

```rust
const EPOLL_CTL_ADD: u32 = 1u32;
```

### `EPOLL_CTL_DEL`

```rust
const EPOLL_CTL_DEL: u32 = 2u32;
```

### `EPOLL_CTL_MOD`

```rust
const EPOLL_CTL_MOD: u32 = 3u32;
```

### `EPOLL_IOC_TYPE`

```rust
const EPOLL_IOC_TYPE: u32 = 138u32;
```

### `POSIX_FADV_NORMAL`

```rust
const POSIX_FADV_NORMAL: u32 = 0u32;
```

### `POSIX_FADV_RANDOM`

```rust
const POSIX_FADV_RANDOM: u32 = 1u32;
```

### `POSIX_FADV_SEQUENTIAL`

```rust
const POSIX_FADV_SEQUENTIAL: u32 = 2u32;
```

### `POSIX_FADV_WILLNEED`

```rust
const POSIX_FADV_WILLNEED: u32 = 3u32;
```

### `POSIX_FADV_DONTNEED`

```rust
const POSIX_FADV_DONTNEED: u32 = 4u32;
```

### `POSIX_FADV_NOREUSE`

```rust
const POSIX_FADV_NOREUSE: u32 = 5u32;
```

### `FALLOC_FL_ALLOCATE_RANGE`

```rust
const FALLOC_FL_ALLOCATE_RANGE: u32 = 0u32;
```

### `FALLOC_FL_KEEP_SIZE`

```rust
const FALLOC_FL_KEEP_SIZE: u32 = 1u32;
```

### `FALLOC_FL_PUNCH_HOLE`

```rust
const FALLOC_FL_PUNCH_HOLE: u32 = 2u32;
```

### `FALLOC_FL_NO_HIDE_STALE`

```rust
const FALLOC_FL_NO_HIDE_STALE: u32 = 4u32;
```

### `FALLOC_FL_COLLAPSE_RANGE`

```rust
const FALLOC_FL_COLLAPSE_RANGE: u32 = 8u32;
```

### `FALLOC_FL_ZERO_RANGE`

```rust
const FALLOC_FL_ZERO_RANGE: u32 = 16u32;
```

### `FALLOC_FL_INSERT_RANGE`

```rust
const FALLOC_FL_INSERT_RANGE: u32 = 32u32;
```

### `FALLOC_FL_UNSHARE_RANGE`

```rust
const FALLOC_FL_UNSHARE_RANGE: u32 = 64u32;
```

### `NR_OPEN`

```rust
const NR_OPEN: u32 = 1_024u32;
```

### `NGROUPS_MAX`

```rust
const NGROUPS_MAX: u32 = 65_536u32;
```

### `ARG_MAX`

```rust
const ARG_MAX: u32 = 131_072u32;
```

### `LINK_MAX`

```rust
const LINK_MAX: u32 = 127u32;
```

### `MAX_CANON`

```rust
const MAX_CANON: u32 = 255u32;
```

### `MAX_INPUT`

```rust
const MAX_INPUT: u32 = 255u32;
```

### `NAME_MAX`

```rust
const NAME_MAX: u32 = 255u32;
```

### `PATH_MAX`

```rust
const PATH_MAX: u32 = 4_096u32;
```

### `PIPE_BUF`

```rust
const PIPE_BUF: u32 = 4_096u32;
```

### `XATTR_NAME_MAX`

```rust
const XATTR_NAME_MAX: u32 = 255u32;
```

### `XATTR_SIZE_MAX`

```rust
const XATTR_SIZE_MAX: u32 = 65_536u32;
```

### `XATTR_LIST_MAX`

```rust
const XATTR_LIST_MAX: u32 = 65_536u32;
```

### `RTSIG_MAX`

```rust
const RTSIG_MAX: u32 = 32u32;
```

### `_IOC_NRBITS`

```rust
const _IOC_NRBITS: u32 = 8u32;
```

### `_IOC_TYPEBITS`

```rust
const _IOC_TYPEBITS: u32 = 8u32;
```

### `_IOC_SIZEBITS`

```rust
const _IOC_SIZEBITS: u32 = 14u32;
```

### `_IOC_DIRBITS`

```rust
const _IOC_DIRBITS: u32 = 2u32;
```

### `_IOC_NRMASK`

```rust
const _IOC_NRMASK: u32 = 255u32;
```

### `_IOC_TYPEMASK`

```rust
const _IOC_TYPEMASK: u32 = 255u32;
```

### `_IOC_SIZEMASK`

```rust
const _IOC_SIZEMASK: u32 = 16_383u32;
```

### `_IOC_DIRMASK`

```rust
const _IOC_DIRMASK: u32 = 3u32;
```

### `_IOC_NRSHIFT`

```rust
const _IOC_NRSHIFT: u32 = 0u32;
```

### `_IOC_TYPESHIFT`

```rust
const _IOC_TYPESHIFT: u32 = 8u32;
```

### `_IOC_SIZESHIFT`

```rust
const _IOC_SIZESHIFT: u32 = 16u32;
```

### `_IOC_DIRSHIFT`

```rust
const _IOC_DIRSHIFT: u32 = 30u32;
```

### `_IOC_NONE`

```rust
const _IOC_NONE: u32 = 0u32;
```

### `_IOC_WRITE`

```rust
const _IOC_WRITE: u32 = 1u32;
```

### `_IOC_READ`

```rust
const _IOC_READ: u32 = 2u32;
```

### `IOC_IN`

```rust
const IOC_IN: u32 = 1_073_741_824u32;
```

### `IOC_OUT`

```rust
const IOC_OUT: u32 = 2_147_483_648u32;
```

### `IOC_INOUT`

```rust
const IOC_INOUT: u32 = 3_221_225_472u32;
```

### `IOCSIZE_MASK`

```rust
const IOCSIZE_MASK: u32 = 1_073_676_288u32;
```

### `IOCSIZE_SHIFT`

```rust
const IOCSIZE_SHIFT: u32 = 16u32;
```

### `FSCRYPT_POLICY_FLAGS_PAD_4`

```rust
const FSCRYPT_POLICY_FLAGS_PAD_4: u32 = 0u32;
```

### `FSCRYPT_POLICY_FLAGS_PAD_8`

```rust
const FSCRYPT_POLICY_FLAGS_PAD_8: u32 = 1u32;
```

### `FSCRYPT_POLICY_FLAGS_PAD_16`

```rust
const FSCRYPT_POLICY_FLAGS_PAD_16: u32 = 2u32;
```

### `FSCRYPT_POLICY_FLAGS_PAD_32`

```rust
const FSCRYPT_POLICY_FLAGS_PAD_32: u32 = 3u32;
```

### `FSCRYPT_POLICY_FLAGS_PAD_MASK`

```rust
const FSCRYPT_POLICY_FLAGS_PAD_MASK: u32 = 3u32;
```

### `FSCRYPT_POLICY_FLAG_DIRECT_KEY`

```rust
const FSCRYPT_POLICY_FLAG_DIRECT_KEY: u32 = 4u32;
```

### `FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64`

```rust
const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_64: u32 = 8u32;
```

### `FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32`

```rust
const FSCRYPT_POLICY_FLAG_IV_INO_LBLK_32: u32 = 16u32;
```

### `FSCRYPT_MODE_AES_256_XTS`

```rust
const FSCRYPT_MODE_AES_256_XTS: u32 = 1u32;
```

### `FSCRYPT_MODE_AES_256_CTS`

```rust
const FSCRYPT_MODE_AES_256_CTS: u32 = 4u32;
```

### `FSCRYPT_MODE_AES_128_CBC`

```rust
const FSCRYPT_MODE_AES_128_CBC: u32 = 5u32;
```

### `FSCRYPT_MODE_AES_128_CTS`

```rust
const FSCRYPT_MODE_AES_128_CTS: u32 = 6u32;
```

### `FSCRYPT_MODE_SM4_XTS`

```rust
const FSCRYPT_MODE_SM4_XTS: u32 = 7u32;
```

### `FSCRYPT_MODE_SM4_CTS`

```rust
const FSCRYPT_MODE_SM4_CTS: u32 = 8u32;
```

### `FSCRYPT_MODE_ADIANTUM`

```rust
const FSCRYPT_MODE_ADIANTUM: u32 = 9u32;
```

### `FSCRYPT_MODE_AES_256_HCTR2`

```rust
const FSCRYPT_MODE_AES_256_HCTR2: u32 = 10u32;
```

### `FSCRYPT_POLICY_V1`

```rust
const FSCRYPT_POLICY_V1: u32 = 0u32;
```

### `FSCRYPT_KEY_DESCRIPTOR_SIZE`

```rust
const FSCRYPT_KEY_DESCRIPTOR_SIZE: u32 = 8u32;
```

### `FSCRYPT_KEY_DESC_PREFIX`

```rust
const FSCRYPT_KEY_DESC_PREFIX: &[u8; 9];
```

### `FSCRYPT_KEY_DESC_PREFIX_SIZE`

```rust
const FSCRYPT_KEY_DESC_PREFIX_SIZE: u32 = 8u32;
```

### `FSCRYPT_MAX_KEY_SIZE`

```rust
const FSCRYPT_MAX_KEY_SIZE: u32 = 64u32;
```

### `FSCRYPT_POLICY_V2`

```rust
const FSCRYPT_POLICY_V2: u32 = 2u32;
```

### `FSCRYPT_KEY_IDENTIFIER_SIZE`

```rust
const FSCRYPT_KEY_IDENTIFIER_SIZE: u32 = 16u32;
```

### `FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR`

```rust
const FSCRYPT_KEY_SPEC_TYPE_DESCRIPTOR: u32 = 1u32;
```

### `FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER`

```rust
const FSCRYPT_KEY_SPEC_TYPE_IDENTIFIER: u32 = 2u32;
```

### `FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED`

```rust
const FSCRYPT_ADD_KEY_FLAG_HW_WRAPPED: u32 = 1u32;
```

### `FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY`

```rust
const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_FILES_BUSY: u32 = 1u32;
```

### `FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS`

```rust
const FSCRYPT_KEY_REMOVAL_STATUS_FLAG_OTHER_USERS: u32 = 2u32;
```

### `FSCRYPT_KEY_STATUS_ABSENT`

```rust
const FSCRYPT_KEY_STATUS_ABSENT: u32 = 1u32;
```

### `FSCRYPT_KEY_STATUS_PRESENT`

```rust
const FSCRYPT_KEY_STATUS_PRESENT: u32 = 2u32;
```

### `FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED`

```rust
const FSCRYPT_KEY_STATUS_INCOMPLETELY_REMOVED: u32 = 3u32;
```

### `FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF`

```rust
const FSCRYPT_KEY_STATUS_FLAG_ADDED_BY_SELF: u32 = 1u32;
```

### `FS_KEY_DESCRIPTOR_SIZE`

```rust
const FS_KEY_DESCRIPTOR_SIZE: u32 = 8u32;
```

### `FS_POLICY_FLAGS_PAD_4`

```rust
const FS_POLICY_FLAGS_PAD_4: u32 = 0u32;
```

### `FS_POLICY_FLAGS_PAD_8`

```rust
const FS_POLICY_FLAGS_PAD_8: u32 = 1u32;
```

### `FS_POLICY_FLAGS_PAD_16`

```rust
const FS_POLICY_FLAGS_PAD_16: u32 = 2u32;
```

### `FS_POLICY_FLAGS_PAD_32`

```rust
const FS_POLICY_FLAGS_PAD_32: u32 = 3u32;
```

### `FS_POLICY_FLAGS_PAD_MASK`

```rust
const FS_POLICY_FLAGS_PAD_MASK: u32 = 3u32;
```

### `FS_POLICY_FLAG_DIRECT_KEY`

```rust
const FS_POLICY_FLAG_DIRECT_KEY: u32 = 4u32;
```

### `FS_POLICY_FLAGS_VALID`

```rust
const FS_POLICY_FLAGS_VALID: u32 = 7u32;
```

### `FS_ENCRYPTION_MODE_INVALID`

```rust
const FS_ENCRYPTION_MODE_INVALID: u32 = 0u32;
```

### `FS_ENCRYPTION_MODE_AES_256_XTS`

```rust
const FS_ENCRYPTION_MODE_AES_256_XTS: u32 = 1u32;
```

### `FS_ENCRYPTION_MODE_AES_256_GCM`

```rust
const FS_ENCRYPTION_MODE_AES_256_GCM: u32 = 2u32;
```

### `FS_ENCRYPTION_MODE_AES_256_CBC`

```rust
const FS_ENCRYPTION_MODE_AES_256_CBC: u32 = 3u32;
```

### `FS_ENCRYPTION_MODE_AES_256_CTS`

```rust
const FS_ENCRYPTION_MODE_AES_256_CTS: u32 = 4u32;
```

### `FS_ENCRYPTION_MODE_AES_128_CBC`

```rust
const FS_ENCRYPTION_MODE_AES_128_CBC: u32 = 5u32;
```

### `FS_ENCRYPTION_MODE_AES_128_CTS`

```rust
const FS_ENCRYPTION_MODE_AES_128_CTS: u32 = 6u32;
```

### `FS_ENCRYPTION_MODE_ADIANTUM`

```rust
const FS_ENCRYPTION_MODE_ADIANTUM: u32 = 9u32;
```

### `FS_KEY_DESC_PREFIX`

```rust
const FS_KEY_DESC_PREFIX: &[u8; 9];
```

### `FS_KEY_DESC_PREFIX_SIZE`

```rust
const FS_KEY_DESC_PREFIX_SIZE: u32 = 8u32;
```

### `FS_MAX_KEY_SIZE`

```rust
const FS_MAX_KEY_SIZE: u32 = 64u32;
```

### `MS_RDONLY`

```rust
const MS_RDONLY: u32 = 1u32;
```

### `MS_NOSUID`

```rust
const MS_NOSUID: u32 = 2u32;
```

### `MS_NODEV`

```rust
const MS_NODEV: u32 = 4u32;
```

### `MS_NOEXEC`

```rust
const MS_NOEXEC: u32 = 8u32;
```

### `MS_SYNCHRONOUS`

```rust
const MS_SYNCHRONOUS: u32 = 16u32;
```

### `MS_REMOUNT`

```rust
const MS_REMOUNT: u32 = 32u32;
```

### `MS_MANDLOCK`

```rust
const MS_MANDLOCK: u32 = 64u32;
```

### `MS_DIRSYNC`

```rust
const MS_DIRSYNC: u32 = 128u32;
```

### `MS_NOSYMFOLLOW`

```rust
const MS_NOSYMFOLLOW: u32 = 256u32;
```

### `MS_NOATIME`

```rust
const MS_NOATIME: u32 = 1_024u32;
```

### `MS_NODIRATIME`

```rust
const MS_NODIRATIME: u32 = 2_048u32;
```

### `MS_BIND`

```rust
const MS_BIND: u32 = 4_096u32;
```

### `MS_MOVE`

```rust
const MS_MOVE: u32 = 8_192u32;
```

### `MS_REC`

```rust
const MS_REC: u32 = 16_384u32;
```

### `MS_VERBOSE`

```rust
const MS_VERBOSE: u32 = 32_768u32;
```

### `MS_SILENT`

```rust
const MS_SILENT: u32 = 32_768u32;
```

### `MS_POSIXACL`

```rust
const MS_POSIXACL: u32 = 65_536u32;
```

### `MS_UNBINDABLE`

```rust
const MS_UNBINDABLE: u32 = 131_072u32;
```

### `MS_PRIVATE`

```rust
const MS_PRIVATE: u32 = 262_144u32;
```

### `MS_SLAVE`

```rust
const MS_SLAVE: u32 = 524_288u32;
```

### `MS_SHARED`

```rust
const MS_SHARED: u32 = 1_048_576u32;
```

### `MS_RELATIME`

```rust
const MS_RELATIME: u32 = 2_097_152u32;
```

### `MS_KERNMOUNT`

```rust
const MS_KERNMOUNT: u32 = 4_194_304u32;
```

### `MS_I_VERSION`

```rust
const MS_I_VERSION: u32 = 8_388_608u32;
```

### `MS_STRICTATIME`

```rust
const MS_STRICTATIME: u32 = 16_777_216u32;
```

### `MS_LAZYTIME`

```rust
const MS_LAZYTIME: u32 = 33_554_432u32;
```

### `MS_SUBMOUNT`

```rust
const MS_SUBMOUNT: u32 = 67_108_864u32;
```

### `MS_NOREMOTELOCK`

```rust
const MS_NOREMOTELOCK: u32 = 134_217_728u32;
```

### `MS_NOSEC`

```rust
const MS_NOSEC: u32 = 268_435_456u32;
```

### `MS_BORN`

```rust
const MS_BORN: u32 = 536_870_912u32;
```

### `MS_ACTIVE`

```rust
const MS_ACTIVE: u32 = 1_073_741_824u32;
```

### `MS_NOUSER`

```rust
const MS_NOUSER: u32 = 2_147_483_648u32;
```

### `MS_RMT_MASK`

```rust
const MS_RMT_MASK: u32 = 41_943_121u32;
```

### `MS_MGC_VAL`

```rust
const MS_MGC_VAL: u32 = 3_236_757_504u32;
```

### `MS_MGC_MSK`

```rust
const MS_MGC_MSK: u32 = 4_294_901_760u32;
```

### `OPEN_TREE_CLONE`

```rust
const OPEN_TREE_CLONE: u32 = 1u32;
```

### `OPEN_TREE_CLOEXEC`

```rust
const OPEN_TREE_CLOEXEC: u32 = 524_288u32;
```

### `MOVE_MOUNT_F_SYMLINKS`

```rust
const MOVE_MOUNT_F_SYMLINKS: u32 = 1u32;
```

### `MOVE_MOUNT_F_AUTOMOUNTS`

```rust
const MOVE_MOUNT_F_AUTOMOUNTS: u32 = 2u32;
```

### `MOVE_MOUNT_F_EMPTY_PATH`

```rust
const MOVE_MOUNT_F_EMPTY_PATH: u32 = 4u32;
```

### `MOVE_MOUNT_T_SYMLINKS`

```rust
const MOVE_MOUNT_T_SYMLINKS: u32 = 16u32;
```

### `MOVE_MOUNT_T_AUTOMOUNTS`

```rust
const MOVE_MOUNT_T_AUTOMOUNTS: u32 = 32u32;
```

### `MOVE_MOUNT_T_EMPTY_PATH`

```rust
const MOVE_MOUNT_T_EMPTY_PATH: u32 = 64u32;
```

### `MOVE_MOUNT_SET_GROUP`

```rust
const MOVE_MOUNT_SET_GROUP: u32 = 256u32;
```

### `MOVE_MOUNT_BENEATH`

```rust
const MOVE_MOUNT_BENEATH: u32 = 512u32;
```

### `MOVE_MOUNT__MASK`

```rust
const MOVE_MOUNT__MASK: u32 = 887u32;
```

### `FSOPEN_CLOEXEC`

```rust
const FSOPEN_CLOEXEC: u32 = 1u32;
```

### `FSPICK_CLOEXEC`

```rust
const FSPICK_CLOEXEC: u32 = 1u32;
```

### `FSPICK_SYMLINK_NOFOLLOW`

```rust
const FSPICK_SYMLINK_NOFOLLOW: u32 = 2u32;
```

### `FSPICK_NO_AUTOMOUNT`

```rust
const FSPICK_NO_AUTOMOUNT: u32 = 4u32;
```

### `FSPICK_EMPTY_PATH`

```rust
const FSPICK_EMPTY_PATH: u32 = 8u32;
```

### `FSMOUNT_CLOEXEC`

```rust
const FSMOUNT_CLOEXEC: u32 = 1u32;
```

### `MOUNT_ATTR_RDONLY`

```rust
const MOUNT_ATTR_RDONLY: u32 = 1u32;
```

### `MOUNT_ATTR_NOSUID`

```rust
const MOUNT_ATTR_NOSUID: u32 = 2u32;
```

### `MOUNT_ATTR_NODEV`

```rust
const MOUNT_ATTR_NODEV: u32 = 4u32;
```

### `MOUNT_ATTR_NOEXEC`

```rust
const MOUNT_ATTR_NOEXEC: u32 = 8u32;
```

### `MOUNT_ATTR__ATIME`

```rust
const MOUNT_ATTR__ATIME: u32 = 112u32;
```

### `MOUNT_ATTR_RELATIME`

```rust
const MOUNT_ATTR_RELATIME: u32 = 0u32;
```

### `MOUNT_ATTR_NOATIME`

```rust
const MOUNT_ATTR_NOATIME: u32 = 16u32;
```

### `MOUNT_ATTR_STRICTATIME`

```rust
const MOUNT_ATTR_STRICTATIME: u32 = 32u32;
```

### `MOUNT_ATTR_NODIRATIME`

```rust
const MOUNT_ATTR_NODIRATIME: u32 = 128u32;
```

### `MOUNT_ATTR_IDMAP`

```rust
const MOUNT_ATTR_IDMAP: u32 = 1_048_576u32;
```

### `MOUNT_ATTR_NOSYMFOLLOW`

```rust
const MOUNT_ATTR_NOSYMFOLLOW: u32 = 2_097_152u32;
```

### `MOUNT_ATTR_SIZE_VER0`

```rust
const MOUNT_ATTR_SIZE_VER0: u32 = 32u32;
```

### `MNT_ID_REQ_SIZE_VER0`

```rust
const MNT_ID_REQ_SIZE_VER0: u32 = 24u32;
```

### `MNT_ID_REQ_SIZE_VER1`

```rust
const MNT_ID_REQ_SIZE_VER1: u32 = 32u32;
```

### `STATMOUNT_SB_BASIC`

```rust
const STATMOUNT_SB_BASIC: u32 = 1u32;
```

### `STATMOUNT_MNT_BASIC`

```rust
const STATMOUNT_MNT_BASIC: u32 = 2u32;
```

### `STATMOUNT_PROPAGATE_FROM`

```rust
const STATMOUNT_PROPAGATE_FROM: u32 = 4u32;
```

### `STATMOUNT_MNT_ROOT`

```rust
const STATMOUNT_MNT_ROOT: u32 = 8u32;
```

### `STATMOUNT_MNT_POINT`

```rust
const STATMOUNT_MNT_POINT: u32 = 16u32;
```

### `STATMOUNT_FS_TYPE`

```rust
const STATMOUNT_FS_TYPE: u32 = 32u32;
```

### `STATMOUNT_MNT_NS_ID`

```rust
const STATMOUNT_MNT_NS_ID: u32 = 64u32;
```

### `STATMOUNT_MNT_OPTS`

```rust
const STATMOUNT_MNT_OPTS: u32 = 128u32;
```

### `STATMOUNT_FS_SUBTYPE`

```rust
const STATMOUNT_FS_SUBTYPE: u32 = 256u32;
```

### `STATMOUNT_SB_SOURCE`

```rust
const STATMOUNT_SB_SOURCE: u32 = 512u32;
```

### `STATMOUNT_OPT_ARRAY`

```rust
const STATMOUNT_OPT_ARRAY: u32 = 1_024u32;
```

### `STATMOUNT_OPT_SEC_ARRAY`

```rust
const STATMOUNT_OPT_SEC_ARRAY: u32 = 2_048u32;
```

### `STATMOUNT_SUPPORTED_MASK`

```rust
const STATMOUNT_SUPPORTED_MASK: u32 = 4_096u32;
```

### `STATMOUNT_MNT_UIDMAP`

```rust
const STATMOUNT_MNT_UIDMAP: u32 = 8_192u32;
```

### `STATMOUNT_MNT_GIDMAP`

```rust
const STATMOUNT_MNT_GIDMAP: u32 = 16_384u32;
```

### `LSMT_ROOT`

```rust
const LSMT_ROOT: i32 = -1i32;
```

### `LISTMOUNT_REVERSE`

```rust
const LISTMOUNT_REVERSE: u32 = 1u32;
```

### `INR_OPEN_CUR`

```rust
const INR_OPEN_CUR: u32 = 1_024u32;
```

### `INR_OPEN_MAX`

```rust
const INR_OPEN_MAX: u32 = 4_096u32;
```

### `BLOCK_SIZE_BITS`

```rust
const BLOCK_SIZE_BITS: u32 = 10u32;
```

### `BLOCK_SIZE`

```rust
const BLOCK_SIZE: u32 = 1_024u32;
```

### `IO_INTEGRITY_CHK_GUARD`

```rust
const IO_INTEGRITY_CHK_GUARD: u32 = 1u32;
```

### `IO_INTEGRITY_CHK_REFTAG`

```rust
const IO_INTEGRITY_CHK_REFTAG: u32 = 2u32;
```

### `IO_INTEGRITY_CHK_APPTAG`

```rust
const IO_INTEGRITY_CHK_APPTAG: u32 = 4u32;
```

### `IO_INTEGRITY_VALID_FLAGS`

```rust
const IO_INTEGRITY_VALID_FLAGS: u32 = 7u32;
```

### `SEEK_SET`

```rust
const SEEK_SET: u32 = 0u32;
```

### `SEEK_CUR`

```rust
const SEEK_CUR: u32 = 1u32;
```

### `SEEK_END`

```rust
const SEEK_END: u32 = 2u32;
```

### `SEEK_DATA`

```rust
const SEEK_DATA: u32 = 3u32;
```

### `SEEK_HOLE`

```rust
const SEEK_HOLE: u32 = 4u32;
```

### `SEEK_MAX`

```rust
const SEEK_MAX: u32 = 4u32;
```

### `RENAME_NOREPLACE`

```rust
const RENAME_NOREPLACE: u32 = 1u32;
```

### `RENAME_EXCHANGE`

```rust
const RENAME_EXCHANGE: u32 = 2u32;
```

### `RENAME_WHITEOUT`

```rust
const RENAME_WHITEOUT: u32 = 4u32;
```

### `FILE_DEDUPE_RANGE_SAME`

```rust
const FILE_DEDUPE_RANGE_SAME: u32 = 0u32;
```

### `FILE_DEDUPE_RANGE_DIFFERS`

```rust
const FILE_DEDUPE_RANGE_DIFFERS: u32 = 1u32;
```

### `NR_FILE`

```rust
const NR_FILE: u32 = 8_192u32;
```

### `FS_XFLAG_REALTIME`

```rust
const FS_XFLAG_REALTIME: u32 = 1u32;
```

### `FS_XFLAG_PREALLOC`

```rust
const FS_XFLAG_PREALLOC: u32 = 2u32;
```

### `FS_XFLAG_IMMUTABLE`

```rust
const FS_XFLAG_IMMUTABLE: u32 = 8u32;
```

### `FS_XFLAG_APPEND`

```rust
const FS_XFLAG_APPEND: u32 = 16u32;
```

### `FS_XFLAG_SYNC`

```rust
const FS_XFLAG_SYNC: u32 = 32u32;
```

### `FS_XFLAG_NOATIME`

```rust
const FS_XFLAG_NOATIME: u32 = 64u32;
```

### `FS_XFLAG_NODUMP`

```rust
const FS_XFLAG_NODUMP: u32 = 128u32;
```

### `FS_XFLAG_RTINHERIT`

```rust
const FS_XFLAG_RTINHERIT: u32 = 256u32;
```

### `FS_XFLAG_PROJINHERIT`

```rust
const FS_XFLAG_PROJINHERIT: u32 = 512u32;
```

### `FS_XFLAG_NOSYMLINKS`

```rust
const FS_XFLAG_NOSYMLINKS: u32 = 1_024u32;
```

### `FS_XFLAG_EXTSIZE`

```rust
const FS_XFLAG_EXTSIZE: u32 = 2_048u32;
```

### `FS_XFLAG_EXTSZINHERIT`

```rust
const FS_XFLAG_EXTSZINHERIT: u32 = 4_096u32;
```

### `FS_XFLAG_NODEFRAG`

```rust
const FS_XFLAG_NODEFRAG: u32 = 8_192u32;
```

### `FS_XFLAG_FILESTREAM`

```rust
const FS_XFLAG_FILESTREAM: u32 = 16_384u32;
```

### `FS_XFLAG_DAX`

```rust
const FS_XFLAG_DAX: u32 = 32_768u32;
```

### `FS_XFLAG_COWEXTSIZE`

```rust
const FS_XFLAG_COWEXTSIZE: u32 = 65_536u32;
```

### `FS_XFLAG_HASATTR`

```rust
const FS_XFLAG_HASATTR: u32 = 2_147_483_648u32;
```

### `BMAP_IOCTL`

```rust
const BMAP_IOCTL: u32 = 1u32;
```

### `FSLABEL_MAX`

```rust
const FSLABEL_MAX: u32 = 256u32;
```

### `FS_SECRM_FL`

```rust
const FS_SECRM_FL: u32 = 1u32;
```

### `FS_UNRM_FL`

```rust
const FS_UNRM_FL: u32 = 2u32;
```

### `FS_COMPR_FL`

```rust
const FS_COMPR_FL: u32 = 4u32;
```

### `FS_SYNC_FL`

```rust
const FS_SYNC_FL: u32 = 8u32;
```

### `FS_IMMUTABLE_FL`

```rust
const FS_IMMUTABLE_FL: u32 = 16u32;
```

### `FS_APPEND_FL`

```rust
const FS_APPEND_FL: u32 = 32u32;
```

### `FS_NODUMP_FL`

```rust
const FS_NODUMP_FL: u32 = 64u32;
```

### `FS_NOATIME_FL`

```rust
const FS_NOATIME_FL: u32 = 128u32;
```

### `FS_DIRTY_FL`

```rust
const FS_DIRTY_FL: u32 = 256u32;
```

### `FS_COMPRBLK_FL`

```rust
const FS_COMPRBLK_FL: u32 = 512u32;
```

### `FS_NOCOMP_FL`

```rust
const FS_NOCOMP_FL: u32 = 1_024u32;
```

### `FS_ENCRYPT_FL`

```rust
const FS_ENCRYPT_FL: u32 = 2_048u32;
```

### `FS_BTREE_FL`

```rust
const FS_BTREE_FL: u32 = 4_096u32;
```

### `FS_INDEX_FL`

```rust
const FS_INDEX_FL: u32 = 4_096u32;
```

### `FS_IMAGIC_FL`

```rust
const FS_IMAGIC_FL: u32 = 8_192u32;
```

### `FS_JOURNAL_DATA_FL`

```rust
const FS_JOURNAL_DATA_FL: u32 = 16_384u32;
```

### `FS_NOTAIL_FL`

```rust
const FS_NOTAIL_FL: u32 = 32_768u32;
```

### `FS_DIRSYNC_FL`

```rust
const FS_DIRSYNC_FL: u32 = 65_536u32;
```

### `FS_TOPDIR_FL`

```rust
const FS_TOPDIR_FL: u32 = 131_072u32;
```

### `FS_HUGE_FILE_FL`

```rust
const FS_HUGE_FILE_FL: u32 = 262_144u32;
```

### `FS_EXTENT_FL`

```rust
const FS_EXTENT_FL: u32 = 524_288u32;
```

### `FS_VERITY_FL`

```rust
const FS_VERITY_FL: u32 = 1_048_576u32;
```

### `FS_EA_INODE_FL`

```rust
const FS_EA_INODE_FL: u32 = 2_097_152u32;
```

### `FS_EOFBLOCKS_FL`

```rust
const FS_EOFBLOCKS_FL: u32 = 4_194_304u32;
```

### `FS_NOCOW_FL`

```rust
const FS_NOCOW_FL: u32 = 8_388_608u32;
```

### `FS_DAX_FL`

```rust
const FS_DAX_FL: u32 = 33_554_432u32;
```

### `FS_INLINE_DATA_FL`

```rust
const FS_INLINE_DATA_FL: u32 = 268_435_456u32;
```

### `FS_PROJINHERIT_FL`

```rust
const FS_PROJINHERIT_FL: u32 = 536_870_912u32;
```

### `FS_CASEFOLD_FL`

```rust
const FS_CASEFOLD_FL: u32 = 1_073_741_824u32;
```

### `FS_RESERVED_FL`

```rust
const FS_RESERVED_FL: u32 = 2_147_483_648u32;
```

### `FS_FL_USER_VISIBLE`

```rust
const FS_FL_USER_VISIBLE: u32 = 253_951u32;
```

### `FS_FL_USER_MODIFIABLE`

```rust
const FS_FL_USER_MODIFIABLE: u32 = 229_631u32;
```

### `SYNC_FILE_RANGE_WAIT_BEFORE`

```rust
const SYNC_FILE_RANGE_WAIT_BEFORE: u32 = 1u32;
```

### `SYNC_FILE_RANGE_WRITE`

```rust
const SYNC_FILE_RANGE_WRITE: u32 = 2u32;
```

### `SYNC_FILE_RANGE_WAIT_AFTER`

```rust
const SYNC_FILE_RANGE_WAIT_AFTER: u32 = 4u32;
```

### `SYNC_FILE_RANGE_WRITE_AND_WAIT`

```rust
const SYNC_FILE_RANGE_WRITE_AND_WAIT: u32 = 7u32;
```

### `PROCFS_IOCTL_MAGIC`

```rust
const PROCFS_IOCTL_MAGIC: u8 = 102u8;
```

### `PAGE_IS_WPALLOWED`

```rust
const PAGE_IS_WPALLOWED: u32 = 1u32;
```

### `PAGE_IS_WRITTEN`

```rust
const PAGE_IS_WRITTEN: u32 = 2u32;
```

### `PAGE_IS_FILE`

```rust
const PAGE_IS_FILE: u32 = 4u32;
```

### `PAGE_IS_PRESENT`

```rust
const PAGE_IS_PRESENT: u32 = 8u32;
```

### `PAGE_IS_SWAPPED`

```rust
const PAGE_IS_SWAPPED: u32 = 16u32;
```

### `PAGE_IS_PFNZERO`

```rust
const PAGE_IS_PFNZERO: u32 = 32u32;
```

### `PAGE_IS_HUGE`

```rust
const PAGE_IS_HUGE: u32 = 64u32;
```

### `PAGE_IS_SOFT_DIRTY`

```rust
const PAGE_IS_SOFT_DIRTY: u32 = 128u32;
```

### `PAGE_IS_GUARD`

```rust
const PAGE_IS_GUARD: u32 = 256u32;
```

### `PM_SCAN_WP_MATCHING`

```rust
const PM_SCAN_WP_MATCHING: u32 = 1u32;
```

### `PM_SCAN_CHECK_WPASYNC`

```rust
const PM_SCAN_CHECK_WPASYNC: u32 = 2u32;
```

### `FUTEX_WAIT`

```rust
const FUTEX_WAIT: u32 = 0u32;
```

### `FUTEX_WAKE`

```rust
const FUTEX_WAKE: u32 = 1u32;
```

### `FUTEX_FD`

```rust
const FUTEX_FD: u32 = 2u32;
```

### `FUTEX_REQUEUE`

```rust
const FUTEX_REQUEUE: u32 = 3u32;
```

### `FUTEX_CMP_REQUEUE`

```rust
const FUTEX_CMP_REQUEUE: u32 = 4u32;
```

### `FUTEX_WAKE_OP`

```rust
const FUTEX_WAKE_OP: u32 = 5u32;
```

### `FUTEX_LOCK_PI`

```rust
const FUTEX_LOCK_PI: u32 = 6u32;
```

### `FUTEX_UNLOCK_PI`

```rust
const FUTEX_UNLOCK_PI: u32 = 7u32;
```

### `FUTEX_TRYLOCK_PI`

```rust
const FUTEX_TRYLOCK_PI: u32 = 8u32;
```

### `FUTEX_WAIT_BITSET`

```rust
const FUTEX_WAIT_BITSET: u32 = 9u32;
```

### `FUTEX_WAKE_BITSET`

```rust
const FUTEX_WAKE_BITSET: u32 = 10u32;
```

### `FUTEX_WAIT_REQUEUE_PI`

```rust
const FUTEX_WAIT_REQUEUE_PI: u32 = 11u32;
```

### `FUTEX_CMP_REQUEUE_PI`

```rust
const FUTEX_CMP_REQUEUE_PI: u32 = 12u32;
```

### `FUTEX_LOCK_PI2`

```rust
const FUTEX_LOCK_PI2: u32 = 13u32;
```

### `FUTEX_PRIVATE_FLAG`

```rust
const FUTEX_PRIVATE_FLAG: u32 = 128u32;
```

### `FUTEX_CLOCK_REALTIME`

```rust
const FUTEX_CLOCK_REALTIME: u32 = 256u32;
```

### `FUTEX_CMD_MASK`

```rust
const FUTEX_CMD_MASK: i32 = -385i32;
```

### `FUTEX_WAIT_PRIVATE`

```rust
const FUTEX_WAIT_PRIVATE: u32 = 128u32;
```

### `FUTEX_WAKE_PRIVATE`

```rust
const FUTEX_WAKE_PRIVATE: u32 = 129u32;
```

### `FUTEX_REQUEUE_PRIVATE`

```rust
const FUTEX_REQUEUE_PRIVATE: u32 = 131u32;
```

### `FUTEX_CMP_REQUEUE_PRIVATE`

```rust
const FUTEX_CMP_REQUEUE_PRIVATE: u32 = 132u32;
```

### `FUTEX_WAKE_OP_PRIVATE`

```rust
const FUTEX_WAKE_OP_PRIVATE: u32 = 133u32;
```

### `FUTEX_LOCK_PI_PRIVATE`

```rust
const FUTEX_LOCK_PI_PRIVATE: u32 = 134u32;
```

### `FUTEX_LOCK_PI2_PRIVATE`

```rust
const FUTEX_LOCK_PI2_PRIVATE: u32 = 141u32;
```

### `FUTEX_UNLOCK_PI_PRIVATE`

```rust
const FUTEX_UNLOCK_PI_PRIVATE: u32 = 135u32;
```

### `FUTEX_TRYLOCK_PI_PRIVATE`

```rust
const FUTEX_TRYLOCK_PI_PRIVATE: u32 = 136u32;
```

### `FUTEX_WAIT_BITSET_PRIVATE`

```rust
const FUTEX_WAIT_BITSET_PRIVATE: u32 = 137u32;
```

### `FUTEX_WAKE_BITSET_PRIVATE`

```rust
const FUTEX_WAKE_BITSET_PRIVATE: u32 = 138u32;
```

### `FUTEX_WAIT_REQUEUE_PI_PRIVATE`

```rust
const FUTEX_WAIT_REQUEUE_PI_PRIVATE: u32 = 139u32;
```

### `FUTEX_CMP_REQUEUE_PI_PRIVATE`

```rust
const FUTEX_CMP_REQUEUE_PI_PRIVATE: u32 = 140u32;
```

### `FUTEX2_SIZE_U8`

```rust
const FUTEX2_SIZE_U8: u32 = 0u32;
```

### `FUTEX2_SIZE_U16`

```rust
const FUTEX2_SIZE_U16: u32 = 1u32;
```

### `FUTEX2_SIZE_U32`

```rust
const FUTEX2_SIZE_U32: u32 = 2u32;
```

### `FUTEX2_SIZE_U64`

```rust
const FUTEX2_SIZE_U64: u32 = 3u32;
```

### `FUTEX2_NUMA`

```rust
const FUTEX2_NUMA: u32 = 4u32;
```

### `FUTEX2_MPOL`

```rust
const FUTEX2_MPOL: u32 = 8u32;
```

### `FUTEX2_PRIVATE`

```rust
const FUTEX2_PRIVATE: u32 = 128u32;
```

### `FUTEX2_SIZE_MASK`

```rust
const FUTEX2_SIZE_MASK: u32 = 3u32;
```

### `FUTEX_32`

```rust
const FUTEX_32: u32 = 2u32;
```

### `FUTEX_NO_NODE`

```rust
const FUTEX_NO_NODE: i32 = -1i32;
```

### `FUTEX_WAITV_MAX`

```rust
const FUTEX_WAITV_MAX: u32 = 128u32;
```

### `FUTEX_WAITERS`

```rust
const FUTEX_WAITERS: u32 = 2_147_483_648u32;
```

### `FUTEX_OWNER_DIED`

```rust
const FUTEX_OWNER_DIED: u32 = 1_073_741_824u32;
```

### `FUTEX_TID_MASK`

```rust
const FUTEX_TID_MASK: u32 = 1_073_741_823u32;
```

### `ROBUST_LIST_LIMIT`

```rust
const ROBUST_LIST_LIMIT: u32 = 2_048u32;
```

### `FUTEX_BITSET_MATCH_ANY`

```rust
const FUTEX_BITSET_MATCH_ANY: u32 = 4_294_967_295u32;
```

### `FUTEX_OP_SET`

```rust
const FUTEX_OP_SET: u32 = 0u32;
```

### `FUTEX_OP_ADD`

```rust
const FUTEX_OP_ADD: u32 = 1u32;
```

### `FUTEX_OP_OR`

```rust
const FUTEX_OP_OR: u32 = 2u32;
```

### `FUTEX_OP_ANDN`

```rust
const FUTEX_OP_ANDN: u32 = 3u32;
```

### `FUTEX_OP_XOR`

```rust
const FUTEX_OP_XOR: u32 = 4u32;
```

### `FUTEX_OP_OPARG_SHIFT`

```rust
const FUTEX_OP_OPARG_SHIFT: u32 = 8u32;
```

### `FUTEX_OP_CMP_EQ`

```rust
const FUTEX_OP_CMP_EQ: u32 = 0u32;
```

### `FUTEX_OP_CMP_NE`

```rust
const FUTEX_OP_CMP_NE: u32 = 1u32;
```

### `FUTEX_OP_CMP_LT`

```rust
const FUTEX_OP_CMP_LT: u32 = 2u32;
```

### `FUTEX_OP_CMP_LE`

```rust
const FUTEX_OP_CMP_LE: u32 = 3u32;
```

### `FUTEX_OP_CMP_GT`

```rust
const FUTEX_OP_CMP_GT: u32 = 4u32;
```

### `FUTEX_OP_CMP_GE`

```rust
const FUTEX_OP_CMP_GE: u32 = 5u32;
```

### `IN_ACCESS`

```rust
const IN_ACCESS: u32 = 1u32;
```

### `IN_MODIFY`

```rust
const IN_MODIFY: u32 = 2u32;
```

### `IN_ATTRIB`

```rust
const IN_ATTRIB: u32 = 4u32;
```

### `IN_CLOSE_WRITE`

```rust
const IN_CLOSE_WRITE: u32 = 8u32;
```

### `IN_CLOSE_NOWRITE`

```rust
const IN_CLOSE_NOWRITE: u32 = 16u32;
```

### `IN_OPEN`

```rust
const IN_OPEN: u32 = 32u32;
```

### `IN_MOVED_FROM`

```rust
const IN_MOVED_FROM: u32 = 64u32;
```

### `IN_MOVED_TO`

```rust
const IN_MOVED_TO: u32 = 128u32;
```

### `IN_CREATE`

```rust
const IN_CREATE: u32 = 256u32;
```

### `IN_DELETE`

```rust
const IN_DELETE: u32 = 512u32;
```

### `IN_DELETE_SELF`

```rust
const IN_DELETE_SELF: u32 = 1_024u32;
```

### `IN_MOVE_SELF`

```rust
const IN_MOVE_SELF: u32 = 2_048u32;
```

### `IN_UNMOUNT`

```rust
const IN_UNMOUNT: u32 = 8_192u32;
```

### `IN_Q_OVERFLOW`

```rust
const IN_Q_OVERFLOW: u32 = 16_384u32;
```

### `IN_IGNORED`

```rust
const IN_IGNORED: u32 = 32_768u32;
```

### `IN_CLOSE`

```rust
const IN_CLOSE: u32 = 24u32;
```

### `IN_MOVE`

```rust
const IN_MOVE: u32 = 192u32;
```

### `IN_ONLYDIR`

```rust
const IN_ONLYDIR: u32 = 16_777_216u32;
```

### `IN_DONT_FOLLOW`

```rust
const IN_DONT_FOLLOW: u32 = 33_554_432u32;
```

### `IN_EXCL_UNLINK`

```rust
const IN_EXCL_UNLINK: u32 = 67_108_864u32;
```

### `IN_MASK_CREATE`

```rust
const IN_MASK_CREATE: u32 = 268_435_456u32;
```

### `IN_MASK_ADD`

```rust
const IN_MASK_ADD: u32 = 536_870_912u32;
```

### `IN_ISDIR`

```rust
const IN_ISDIR: u32 = 1_073_741_824u32;
```

### `IN_ONESHOT`

```rust
const IN_ONESHOT: u32 = 2_147_483_648u32;
```

### `IN_ALL_EVENTS`

```rust
const IN_ALL_EVENTS: u32 = 4_095u32;
```

### `IN_CLOEXEC`

```rust
const IN_CLOEXEC: u32 = 524_288u32;
```

### `IN_NONBLOCK`

```rust
const IN_NONBLOCK: u32 = 2_048u32;
```

### `ADFS_SUPER_MAGIC`

```rust
const ADFS_SUPER_MAGIC: u32 = 44_533u32;
```

### `AFFS_SUPER_MAGIC`

```rust
const AFFS_SUPER_MAGIC: u32 = 44_543u32;
```

### `AFS_SUPER_MAGIC`

```rust
const AFS_SUPER_MAGIC: u32 = 1_397_113_167u32;
```

### `AUTOFS_SUPER_MAGIC`

```rust
const AUTOFS_SUPER_MAGIC: u32 = 391u32;
```

### `CEPH_SUPER_MAGIC`

```rust
const CEPH_SUPER_MAGIC: u32 = 12_805_120u32;
```

### `CODA_SUPER_MAGIC`

```rust
const CODA_SUPER_MAGIC: u32 = 1_937_076_805u32;
```

### `CRAMFS_MAGIC`

```rust
const CRAMFS_MAGIC: u32 = 684_539_205u32;
```

### `CRAMFS_MAGIC_WEND`

```rust
const CRAMFS_MAGIC_WEND: u32 = 1_161_678_120u32;
```

### `DEBUGFS_MAGIC`

```rust
const DEBUGFS_MAGIC: u32 = 1_684_170_528u32;
```

### `SECURITYFS_MAGIC`

```rust
const SECURITYFS_MAGIC: u32 = 1_935_894_131u32;
```

### `SELINUX_MAGIC`

```rust
const SELINUX_MAGIC: u32 = 4_185_718_668u32;
```

### `SMACK_MAGIC`

```rust
const SMACK_MAGIC: u32 = 1_128_357_203u32;
```

### `RAMFS_MAGIC`

```rust
const RAMFS_MAGIC: u32 = 2_240_043_254u32;
```

### `TMPFS_MAGIC`

```rust
const TMPFS_MAGIC: u32 = 16_914_836u32;
```

### `HUGETLBFS_MAGIC`

```rust
const HUGETLBFS_MAGIC: u32 = 2_508_478_710u32;
```

### `SQUASHFS_MAGIC`

```rust
const SQUASHFS_MAGIC: u32 = 1_936_814_952u32;
```

### `ECRYPTFS_SUPER_MAGIC`

```rust
const ECRYPTFS_SUPER_MAGIC: u32 = 61_791u32;
```

### `EFS_SUPER_MAGIC`

```rust
const EFS_SUPER_MAGIC: u32 = 4_278_867u32;
```

### `EROFS_SUPER_MAGIC_V1`

```rust
const EROFS_SUPER_MAGIC_V1: u32 = 3_774_210_530u32;
```

### `EXT2_SUPER_MAGIC`

```rust
const EXT2_SUPER_MAGIC: u32 = 61_267u32;
```

### `EXT3_SUPER_MAGIC`

```rust
const EXT3_SUPER_MAGIC: u32 = 61_267u32;
```

### `XENFS_SUPER_MAGIC`

```rust
const XENFS_SUPER_MAGIC: u32 = 2_881_100_148u32;
```

### `EXT4_SUPER_MAGIC`

```rust
const EXT4_SUPER_MAGIC: u32 = 61_267u32;
```

### `BTRFS_SUPER_MAGIC`

```rust
const BTRFS_SUPER_MAGIC: u32 = 2_435_016_766u32;
```

### `NILFS_SUPER_MAGIC`

```rust
const NILFS_SUPER_MAGIC: u32 = 13_364u32;
```

### `F2FS_SUPER_MAGIC`

```rust
const F2FS_SUPER_MAGIC: u32 = 4_076_150_800u32;
```

### `HPFS_SUPER_MAGIC`

```rust
const HPFS_SUPER_MAGIC: u32 = 4_187_351_113u32;
```

### `ISOFS_SUPER_MAGIC`

```rust
const ISOFS_SUPER_MAGIC: u32 = 38_496u32;
```

### `JFFS2_SUPER_MAGIC`

```rust
const JFFS2_SUPER_MAGIC: u32 = 29_366u32;
```

### `XFS_SUPER_MAGIC`

```rust
const XFS_SUPER_MAGIC: u32 = 1_481_003_842u32;
```

### `PSTOREFS_MAGIC`

```rust
const PSTOREFS_MAGIC: u32 = 1_634_035_564u32;
```

### `EFIVARFS_MAGIC`

```rust
const EFIVARFS_MAGIC: u32 = 3_730_735_588u32;
```

### `HOSTFS_SUPER_MAGIC`

```rust
const HOSTFS_SUPER_MAGIC: u32 = 12_648_430u32;
```

### `OVERLAYFS_SUPER_MAGIC`

```rust
const OVERLAYFS_SUPER_MAGIC: u32 = 2_035_054_128u32;
```

### `FUSE_SUPER_MAGIC`

```rust
const FUSE_SUPER_MAGIC: u32 = 1_702_057_286u32;
```

### `BCACHEFS_SUPER_MAGIC`

```rust
const BCACHEFS_SUPER_MAGIC: u32 = 3_393_526_350u32;
```

### `MINIX_SUPER_MAGIC`

```rust
const MINIX_SUPER_MAGIC: u32 = 4_991u32;
```

### `MINIX_SUPER_MAGIC2`

```rust
const MINIX_SUPER_MAGIC2: u32 = 5_007u32;
```

### `MINIX2_SUPER_MAGIC`

```rust
const MINIX2_SUPER_MAGIC: u32 = 9_320u32;
```

### `MINIX2_SUPER_MAGIC2`

```rust
const MINIX2_SUPER_MAGIC2: u32 = 9_336u32;
```

### `MINIX3_SUPER_MAGIC`

```rust
const MINIX3_SUPER_MAGIC: u32 = 19_802u32;
```

### `MSDOS_SUPER_MAGIC`

```rust
const MSDOS_SUPER_MAGIC: u32 = 19_780u32;
```

### `EXFAT_SUPER_MAGIC`

```rust
const EXFAT_SUPER_MAGIC: u32 = 538_032_816u32;
```

### `NCP_SUPER_MAGIC`

```rust
const NCP_SUPER_MAGIC: u32 = 22_092u32;
```

### `NFS_SUPER_MAGIC`

```rust
const NFS_SUPER_MAGIC: u32 = 26_985u32;
```

### `OCFS2_SUPER_MAGIC`

```rust
const OCFS2_SUPER_MAGIC: u32 = 1_952_539_503u32;
```

### `OPENPROM_SUPER_MAGIC`

```rust
const OPENPROM_SUPER_MAGIC: u32 = 40_865u32;
```

### `QNX4_SUPER_MAGIC`

```rust
const QNX4_SUPER_MAGIC: u32 = 47u32;
```

### `QNX6_SUPER_MAGIC`

```rust
const QNX6_SUPER_MAGIC: u32 = 1_746_473_250u32;
```

### `AFS_FS_MAGIC`

```rust
const AFS_FS_MAGIC: u32 = 1_799_439_955u32;
```

### `REISERFS_SUPER_MAGIC`

```rust
const REISERFS_SUPER_MAGIC: u32 = 1_382_369_651u32;
```

### `REISERFS_SUPER_MAGIC_STRING`

```rust
const REISERFS_SUPER_MAGIC_STRING: &[u8; 9];
```

### `REISER2FS_SUPER_MAGIC_STRING`

```rust
const REISER2FS_SUPER_MAGIC_STRING: &[u8; 10];
```

### `REISER2FS_JR_SUPER_MAGIC_STRING`

```rust
const REISER2FS_JR_SUPER_MAGIC_STRING: &[u8; 10];
```

### `SMB_SUPER_MAGIC`

```rust
const SMB_SUPER_MAGIC: u32 = 20_859u32;
```

### `CIFS_SUPER_MAGIC`

```rust
const CIFS_SUPER_MAGIC: u32 = 4_283_649_346u32;
```

### `SMB2_SUPER_MAGIC`

```rust
const SMB2_SUPER_MAGIC: u32 = 4_266_872_130u32;
```

### `CGROUP_SUPER_MAGIC`

```rust
const CGROUP_SUPER_MAGIC: u32 = 2_613_483u32;
```

### `CGROUP2_SUPER_MAGIC`

```rust
const CGROUP2_SUPER_MAGIC: u32 = 1_667_723_888u32;
```

### `RDTGROUP_SUPER_MAGIC`

```rust
const RDTGROUP_SUPER_MAGIC: u32 = 124_082_209u32;
```

### `STACK_END_MAGIC`

```rust
const STACK_END_MAGIC: u32 = 1_470_918_301u32;
```

### `TRACEFS_MAGIC`

```rust
const TRACEFS_MAGIC: u32 = 1_953_653_091u32;
```

### `V9FS_MAGIC`

```rust
const V9FS_MAGIC: u32 = 16_914_839u32;
```

### `BDEVFS_MAGIC`

```rust
const BDEVFS_MAGIC: u32 = 1_650_746_742u32;
```

### `DAXFS_MAGIC`

```rust
const DAXFS_MAGIC: u32 = 1_684_300_152u32;
```

### `BINFMTFS_MAGIC`

```rust
const BINFMTFS_MAGIC: u32 = 1_112_100_429u32;
```

### `DEVPTS_SUPER_MAGIC`

```rust
const DEVPTS_SUPER_MAGIC: u32 = 7_377u32;
```

### `BINDERFS_SUPER_MAGIC`

```rust
const BINDERFS_SUPER_MAGIC: u32 = 1_819_242_352u32;
```

### `FUTEXFS_SUPER_MAGIC`

```rust
const FUTEXFS_SUPER_MAGIC: u32 = 195_894_762u32;
```

### `PIPEFS_MAGIC`

```rust
const PIPEFS_MAGIC: u32 = 1_346_981_957u32;
```

### `PROC_SUPER_MAGIC`

```rust
const PROC_SUPER_MAGIC: u32 = 40_864u32;
```

### `SOCKFS_MAGIC`

```rust
const SOCKFS_MAGIC: u32 = 1_397_703_499u32;
```

### `SYSFS_MAGIC`

```rust
const SYSFS_MAGIC: u32 = 1_650_812_274u32;
```

### `USBDEVICE_SUPER_MAGIC`

```rust
const USBDEVICE_SUPER_MAGIC: u32 = 40_866u32;
```

### `MTD_INODE_FS_MAGIC`

```rust
const MTD_INODE_FS_MAGIC: u32 = 288_389_204u32;
```

### `ANON_INODE_FS_MAGIC`

```rust
const ANON_INODE_FS_MAGIC: u32 = 151_263_540u32;
```

### `BTRFS_TEST_MAGIC`

```rust
const BTRFS_TEST_MAGIC: u32 = 1_936_880_249u32;
```

### `NSFS_MAGIC`

```rust
const NSFS_MAGIC: u32 = 1_853_056_627u32;
```

### `BPF_FS_MAGIC`

```rust
const BPF_FS_MAGIC: u32 = 3_405_662_737u32;
```

### `AAFS_MAGIC`

```rust
const AAFS_MAGIC: u32 = 1_513_908_720u32;
```

### `ZONEFS_MAGIC`

```rust
const ZONEFS_MAGIC: u32 = 1_515_144_787u32;
```

### `UDF_SUPER_MAGIC`

```rust
const UDF_SUPER_MAGIC: u32 = 352_400_198u32;
```

### `DMA_BUF_MAGIC`

```rust
const DMA_BUF_MAGIC: u32 = 1_145_913_666u32;
```

### `DEVMEM_MAGIC`

```rust
const DEVMEM_MAGIC: u32 = 1_162_691_661u32;
```

### `SECRETMEM_MAGIC`

```rust
const SECRETMEM_MAGIC: u32 = 1_397_048_141u32;
```

### `PID_FS_MAGIC`

```rust
const PID_FS_MAGIC: u32 = 1_346_978_886u32;
```

### `MAP_32BIT`

```rust
const MAP_32BIT: u32 = 64u32;
```

### `MAP_ABOVE4G`

```rust
const MAP_ABOVE4G: u32 = 128u32;
```

### `PROT_READ`

```rust
const PROT_READ: u32 = 1u32;
```

### `PROT_WRITE`

```rust
const PROT_WRITE: u32 = 2u32;
```

### `PROT_EXEC`

```rust
const PROT_EXEC: u32 = 4u32;
```

### `PROT_SEM`

```rust
const PROT_SEM: u32 = 8u32;
```

### `PROT_NONE`

```rust
const PROT_NONE: u32 = 0u32;
```

### `PROT_GROWSDOWN`

```rust
const PROT_GROWSDOWN: u32 = 16_777_216u32;
```

### `PROT_GROWSUP`

```rust
const PROT_GROWSUP: u32 = 33_554_432u32;
```

### `MAP_TYPE`

```rust
const MAP_TYPE: u32 = 15u32;
```

### `MAP_FIXED`

```rust
const MAP_FIXED: u32 = 16u32;
```

### `MAP_ANONYMOUS`

```rust
const MAP_ANONYMOUS: u32 = 32u32;
```

### `MAP_POPULATE`

```rust
const MAP_POPULATE: u32 = 32_768u32;
```

### `MAP_NONBLOCK`

```rust
const MAP_NONBLOCK: u32 = 65_536u32;
```

### `MAP_STACK`

```rust
const MAP_STACK: u32 = 131_072u32;
```

### `MAP_HUGETLB`

```rust
const MAP_HUGETLB: u32 = 262_144u32;
```

### `MAP_SYNC`

```rust
const MAP_SYNC: u32 = 524_288u32;
```

### `MAP_FIXED_NOREPLACE`

```rust
const MAP_FIXED_NOREPLACE: u32 = 1_048_576u32;
```

### `MAP_UNINITIALIZED`

```rust
const MAP_UNINITIALIZED: u32 = 67_108_864u32;
```

### `MLOCK_ONFAULT`

```rust
const MLOCK_ONFAULT: u32 = 1u32;
```

### `MS_ASYNC`

```rust
const MS_ASYNC: u32 = 1u32;
```

### `MS_INVALIDATE`

```rust
const MS_INVALIDATE: u32 = 2u32;
```

### `MS_SYNC`

```rust
const MS_SYNC: u32 = 4u32;
```

### `MADV_NORMAL`

```rust
const MADV_NORMAL: u32 = 0u32;
```

### `MADV_RANDOM`

```rust
const MADV_RANDOM: u32 = 1u32;
```

### `MADV_SEQUENTIAL`

```rust
const MADV_SEQUENTIAL: u32 = 2u32;
```

### `MADV_WILLNEED`

```rust
const MADV_WILLNEED: u32 = 3u32;
```

### `MADV_DONTNEED`

```rust
const MADV_DONTNEED: u32 = 4u32;
```

### `MADV_FREE`

```rust
const MADV_FREE: u32 = 8u32;
```

### `MADV_REMOVE`

```rust
const MADV_REMOVE: u32 = 9u32;
```

### `MADV_DONTFORK`

```rust
const MADV_DONTFORK: u32 = 10u32;
```

### `MADV_DOFORK`

```rust
const MADV_DOFORK: u32 = 11u32;
```

### `MADV_HWPOISON`

```rust
const MADV_HWPOISON: u32 = 100u32;
```

### `MADV_SOFT_OFFLINE`

```rust
const MADV_SOFT_OFFLINE: u32 = 101u32;
```

### `MADV_MERGEABLE`

```rust
const MADV_MERGEABLE: u32 = 12u32;
```

### `MADV_UNMERGEABLE`

```rust
const MADV_UNMERGEABLE: u32 = 13u32;
```

### `MADV_HUGEPAGE`

```rust
const MADV_HUGEPAGE: u32 = 14u32;
```

### `MADV_NOHUGEPAGE`

```rust
const MADV_NOHUGEPAGE: u32 = 15u32;
```

### `MADV_DONTDUMP`

```rust
const MADV_DONTDUMP: u32 = 16u32;
```

### `MADV_DODUMP`

```rust
const MADV_DODUMP: u32 = 17u32;
```

### `MADV_WIPEONFORK`

```rust
const MADV_WIPEONFORK: u32 = 18u32;
```

### `MADV_KEEPONFORK`

```rust
const MADV_KEEPONFORK: u32 = 19u32;
```

### `MADV_COLD`

```rust
const MADV_COLD: u32 = 20u32;
```

### `MADV_PAGEOUT`

```rust
const MADV_PAGEOUT: u32 = 21u32;
```

### `MADV_POPULATE_READ`

```rust
const MADV_POPULATE_READ: u32 = 22u32;
```

### `MADV_POPULATE_WRITE`

```rust
const MADV_POPULATE_WRITE: u32 = 23u32;
```

### `MADV_DONTNEED_LOCKED`

```rust
const MADV_DONTNEED_LOCKED: u32 = 24u32;
```

### `MADV_COLLAPSE`

```rust
const MADV_COLLAPSE: u32 = 25u32;
```

### `MADV_GUARD_INSTALL`

```rust
const MADV_GUARD_INSTALL: u32 = 102u32;
```

### `MADV_GUARD_REMOVE`

```rust
const MADV_GUARD_REMOVE: u32 = 103u32;
```

### `MAP_FILE`

```rust
const MAP_FILE: u32 = 0u32;
```

### `PKEY_UNRESTRICTED`

```rust
const PKEY_UNRESTRICTED: u32 = 0u32;
```

### `PKEY_DISABLE_ACCESS`

```rust
const PKEY_DISABLE_ACCESS: u32 = 1u32;
```

### `PKEY_DISABLE_WRITE`

```rust
const PKEY_DISABLE_WRITE: u32 = 2u32;
```

### `PKEY_ACCESS_MASK`

```rust
const PKEY_ACCESS_MASK: u32 = 3u32;
```

### `MAP_GROWSDOWN`

```rust
const MAP_GROWSDOWN: u32 = 256u32;
```

### `MAP_DENYWRITE`

```rust
const MAP_DENYWRITE: u32 = 2_048u32;
```

### `MAP_EXECUTABLE`

```rust
const MAP_EXECUTABLE: u32 = 4_096u32;
```

### `MAP_LOCKED`

```rust
const MAP_LOCKED: u32 = 8_192u32;
```

### `MAP_NORESERVE`

```rust
const MAP_NORESERVE: u32 = 16_384u32;
```

### `MCL_CURRENT`

```rust
const MCL_CURRENT: u32 = 1u32;
```

### `MCL_FUTURE`

```rust
const MCL_FUTURE: u32 = 2u32;
```

### `MCL_ONFAULT`

```rust
const MCL_ONFAULT: u32 = 4u32;
```

### `SHADOW_STACK_SET_TOKEN`

```rust
const SHADOW_STACK_SET_TOKEN: u32 = 1u32;
```

### `SHADOW_STACK_SET_MARKER`

```rust
const SHADOW_STACK_SET_MARKER: u32 = 2u32;
```

### `HUGETLB_FLAG_ENCODE_SHIFT`

```rust
const HUGETLB_FLAG_ENCODE_SHIFT: u32 = 26u32;
```

### `HUGETLB_FLAG_ENCODE_MASK`

```rust
const HUGETLB_FLAG_ENCODE_MASK: u32 = 63u32;
```

### `HUGETLB_FLAG_ENCODE_16KB`

```rust
const HUGETLB_FLAG_ENCODE_16KB: u32 = 939_524_096u32;
```

### `HUGETLB_FLAG_ENCODE_64KB`

```rust
const HUGETLB_FLAG_ENCODE_64KB: u32 = 1_073_741_824u32;
```

### `HUGETLB_FLAG_ENCODE_512KB`

```rust
const HUGETLB_FLAG_ENCODE_512KB: u32 = 1_275_068_416u32;
```

### `HUGETLB_FLAG_ENCODE_1MB`

```rust
const HUGETLB_FLAG_ENCODE_1MB: u32 = 1_342_177_280u32;
```

### `HUGETLB_FLAG_ENCODE_2MB`

```rust
const HUGETLB_FLAG_ENCODE_2MB: u32 = 1_409_286_144u32;
```

### `HUGETLB_FLAG_ENCODE_8MB`

```rust
const HUGETLB_FLAG_ENCODE_8MB: u32 = 1_543_503_872u32;
```

### `HUGETLB_FLAG_ENCODE_16MB`

```rust
const HUGETLB_FLAG_ENCODE_16MB: u32 = 1_610_612_736u32;
```

### `HUGETLB_FLAG_ENCODE_32MB`

```rust
const HUGETLB_FLAG_ENCODE_32MB: u32 = 1_677_721_600u32;
```

### `HUGETLB_FLAG_ENCODE_256MB`

```rust
const HUGETLB_FLAG_ENCODE_256MB: u32 = 1_879_048_192u32;
```

### `HUGETLB_FLAG_ENCODE_512MB`

```rust
const HUGETLB_FLAG_ENCODE_512MB: u32 = 1_946_157_056u32;
```

### `HUGETLB_FLAG_ENCODE_1GB`

```rust
const HUGETLB_FLAG_ENCODE_1GB: u32 = 2_013_265_920u32;
```

### `HUGETLB_FLAG_ENCODE_2GB`

```rust
const HUGETLB_FLAG_ENCODE_2GB: u32 = 2_080_374_784u32;
```

### `HUGETLB_FLAG_ENCODE_16GB`

```rust
const HUGETLB_FLAG_ENCODE_16GB: u32 = 2_281_701_376u32;
```

### `MREMAP_MAYMOVE`

```rust
const MREMAP_MAYMOVE: u32 = 1u32;
```

### `MREMAP_FIXED`

```rust
const MREMAP_FIXED: u32 = 2u32;
```

### `MREMAP_DONTUNMAP`

```rust
const MREMAP_DONTUNMAP: u32 = 4u32;
```

### `OVERCOMMIT_GUESS`

```rust
const OVERCOMMIT_GUESS: u32 = 0u32;
```

### `OVERCOMMIT_ALWAYS`

```rust
const OVERCOMMIT_ALWAYS: u32 = 1u32;
```

### `OVERCOMMIT_NEVER`

```rust
const OVERCOMMIT_NEVER: u32 = 2u32;
```

### `MAP_SHARED`

```rust
const MAP_SHARED: u32 = 1u32;
```

### `MAP_PRIVATE`

```rust
const MAP_PRIVATE: u32 = 2u32;
```

### `MAP_SHARED_VALIDATE`

```rust
const MAP_SHARED_VALIDATE: u32 = 3u32;
```

### `MAP_DROPPABLE`

```rust
const MAP_DROPPABLE: u32 = 8u32;
```

### `MAP_HUGE_SHIFT`

```rust
const MAP_HUGE_SHIFT: u32 = 26u32;
```

### `MAP_HUGE_MASK`

```rust
const MAP_HUGE_MASK: u32 = 63u32;
```

### `MAP_HUGE_16KB`

```rust
const MAP_HUGE_16KB: u32 = 939_524_096u32;
```

### `MAP_HUGE_64KB`

```rust
const MAP_HUGE_64KB: u32 = 1_073_741_824u32;
```

### `MAP_HUGE_512KB`

```rust
const MAP_HUGE_512KB: u32 = 1_275_068_416u32;
```

### `MAP_HUGE_1MB`

```rust
const MAP_HUGE_1MB: u32 = 1_342_177_280u32;
```

### `MAP_HUGE_2MB`

```rust
const MAP_HUGE_2MB: u32 = 1_409_286_144u32;
```

### `MAP_HUGE_8MB`

```rust
const MAP_HUGE_8MB: u32 = 1_543_503_872u32;
```

### `MAP_HUGE_16MB`

```rust
const MAP_HUGE_16MB: u32 = 1_610_612_736u32;
```

### `MAP_HUGE_32MB`

```rust
const MAP_HUGE_32MB: u32 = 1_677_721_600u32;
```

### `MAP_HUGE_256MB`

```rust
const MAP_HUGE_256MB: u32 = 1_879_048_192u32;
```

### `MAP_HUGE_512MB`

```rust
const MAP_HUGE_512MB: u32 = 1_946_157_056u32;
```

### `MAP_HUGE_1GB`

```rust
const MAP_HUGE_1GB: u32 = 2_013_265_920u32;
```

### `MAP_HUGE_2GB`

```rust
const MAP_HUGE_2GB: u32 = 2_080_374_784u32;
```

### `MAP_HUGE_16GB`

```rust
const MAP_HUGE_16GB: u32 = 2_281_701_376u32;
```

### `POLLIN`

```rust
const POLLIN: u32 = 1u32;
```

### `POLLPRI`

```rust
const POLLPRI: u32 = 2u32;
```

### `POLLOUT`

```rust
const POLLOUT: u32 = 4u32;
```

### `POLLERR`

```rust
const POLLERR: u32 = 8u32;
```

### `POLLHUP`

```rust
const POLLHUP: u32 = 16u32;
```

### `POLLNVAL`

```rust
const POLLNVAL: u32 = 32u32;
```

### `POLLRDNORM`

```rust
const POLLRDNORM: u32 = 64u32;
```

### `POLLRDBAND`

```rust
const POLLRDBAND: u32 = 128u32;
```

### `POLLWRNORM`

```rust
const POLLWRNORM: u32 = 256u32;
```

### `POLLWRBAND`

```rust
const POLLWRBAND: u32 = 512u32;
```

### `POLLMSG`

```rust
const POLLMSG: u32 = 1_024u32;
```

### `POLLREMOVE`

```rust
const POLLREMOVE: u32 = 4_096u32;
```

### `POLLRDHUP`

```rust
const POLLRDHUP: u32 = 8_192u32;
```

### `GRND_NONBLOCK`

```rust
const GRND_NONBLOCK: u32 = 1u32;
```

### `GRND_RANDOM`

```rust
const GRND_RANDOM: u32 = 2u32;
```

### `GRND_INSECURE`

```rust
const GRND_INSECURE: u32 = 4u32;
```

### `LINUX_REBOOT_MAGIC1`

```rust
const LINUX_REBOOT_MAGIC1: u32 = 4_276_215_469u32;
```

### `LINUX_REBOOT_MAGIC2`

```rust
const LINUX_REBOOT_MAGIC2: u32 = 672_274_793u32;
```

### `LINUX_REBOOT_MAGIC2A`

```rust
const LINUX_REBOOT_MAGIC2A: u32 = 85_072_278u32;
```

### `LINUX_REBOOT_MAGIC2B`

```rust
const LINUX_REBOOT_MAGIC2B: u32 = 369_367_448u32;
```

### `LINUX_REBOOT_MAGIC2C`

```rust
const LINUX_REBOOT_MAGIC2C: u32 = 537_993_216u32;
```

### `LINUX_REBOOT_CMD_RESTART`

```rust
const LINUX_REBOOT_CMD_RESTART: u32 = 19_088_743u32;
```

### `LINUX_REBOOT_CMD_HALT`

```rust
const LINUX_REBOOT_CMD_HALT: u32 = 3_454_992_675u32;
```

### `LINUX_REBOOT_CMD_CAD_ON`

```rust
const LINUX_REBOOT_CMD_CAD_ON: u32 = 2_309_737_967u32;
```

### `LINUX_REBOOT_CMD_CAD_OFF`

```rust
const LINUX_REBOOT_CMD_CAD_OFF: u32 = 0u32;
```

### `LINUX_REBOOT_CMD_POWER_OFF`

```rust
const LINUX_REBOOT_CMD_POWER_OFF: u32 = 1_126_301_404u32;
```

### `LINUX_REBOOT_CMD_RESTART2`

```rust
const LINUX_REBOOT_CMD_RESTART2: u32 = 2_712_847_316u32;
```

### `LINUX_REBOOT_CMD_SW_SUSPEND`

```rust
const LINUX_REBOOT_CMD_SW_SUSPEND: u32 = 3_489_725_666u32;
```

### `LINUX_REBOOT_CMD_KEXEC`

```rust
const LINUX_REBOOT_CMD_KEXEC: u32 = 1_163_412_803u32;
```

### `RUSAGE_SELF`

```rust
const RUSAGE_SELF: u32 = 0u32;
```

### `RUSAGE_CHILDREN`

```rust
const RUSAGE_CHILDREN: i32 = -1i32;
```

### `RUSAGE_BOTH`

```rust
const RUSAGE_BOTH: i32 = -2i32;
```

### `RUSAGE_THREAD`

```rust
const RUSAGE_THREAD: u32 = 1u32;
```

### `RLIM64_INFINITY`

```rust
const RLIM64_INFINITY: i32 = -1i32;
```

### `PRIO_MIN`

```rust
const PRIO_MIN: i32 = -20i32;
```

### `PRIO_MAX`

```rust
const PRIO_MAX: u32 = 20u32;
```

### `PRIO_PROCESS`

```rust
const PRIO_PROCESS: u32 = 0u32;
```

### `PRIO_PGRP`

```rust
const PRIO_PGRP: u32 = 1u32;
```

### `PRIO_USER`

```rust
const PRIO_USER: u32 = 2u32;
```

### `_STK_LIM`

```rust
const _STK_LIM: u32 = 8_388_608u32;
```

### `MLOCK_LIMIT`

```rust
const MLOCK_LIMIT: u32 = 8_388_608u32;
```

### `RLIMIT_CPU`

```rust
const RLIMIT_CPU: u32 = 0u32;
```

### `RLIMIT_FSIZE`

```rust
const RLIMIT_FSIZE: u32 = 1u32;
```

### `RLIMIT_DATA`

```rust
const RLIMIT_DATA: u32 = 2u32;
```

### `RLIMIT_STACK`

```rust
const RLIMIT_STACK: u32 = 3u32;
```

### `RLIMIT_CORE`

```rust
const RLIMIT_CORE: u32 = 4u32;
```

### `RLIMIT_RSS`

```rust
const RLIMIT_RSS: u32 = 5u32;
```

### `RLIMIT_NPROC`

```rust
const RLIMIT_NPROC: u32 = 6u32;
```

### `RLIMIT_NOFILE`

```rust
const RLIMIT_NOFILE: u32 = 7u32;
```

### `RLIMIT_MEMLOCK`

```rust
const RLIMIT_MEMLOCK: u32 = 8u32;
```

### `RLIMIT_AS`

```rust
const RLIMIT_AS: u32 = 9u32;
```

### `RLIMIT_LOCKS`

```rust
const RLIMIT_LOCKS: u32 = 10u32;
```

### `RLIMIT_SIGPENDING`

```rust
const RLIMIT_SIGPENDING: u32 = 11u32;
```

### `RLIMIT_MSGQUEUE`

```rust
const RLIMIT_MSGQUEUE: u32 = 12u32;
```

### `RLIMIT_NICE`

```rust
const RLIMIT_NICE: u32 = 13u32;
```

### `RLIMIT_RTPRIO`

```rust
const RLIMIT_RTPRIO: u32 = 14u32;
```

### `RLIMIT_RTTIME`

```rust
const RLIMIT_RTTIME: u32 = 15u32;
```

### `RLIM_NLIMITS`

```rust
const RLIM_NLIMITS: u32 = 16u32;
```

### `RLIM_INFINITY`

```rust
const RLIM_INFINITY: i32 = -1i32;
```

### `CSIGNAL`

```rust
const CSIGNAL: u32 = 255u32;
```

### `CLONE_VM`

```rust
const CLONE_VM: u32 = 256u32;
```

### `CLONE_FS`

```rust
const CLONE_FS: u32 = 512u32;
```

### `CLONE_FILES`

```rust
const CLONE_FILES: u32 = 1_024u32;
```

### `CLONE_SIGHAND`

```rust
const CLONE_SIGHAND: u32 = 2_048u32;
```

### `CLONE_PIDFD`

```rust
const CLONE_PIDFD: u32 = 4_096u32;
```

### `CLONE_PTRACE`

```rust
const CLONE_PTRACE: u32 = 8_192u32;
```

### `CLONE_VFORK`

```rust
const CLONE_VFORK: u32 = 16_384u32;
```

### `CLONE_PARENT`

```rust
const CLONE_PARENT: u32 = 32_768u32;
```

### `CLONE_THREAD`

```rust
const CLONE_THREAD: u32 = 65_536u32;
```

### `CLONE_NEWNS`

```rust
const CLONE_NEWNS: u32 = 131_072u32;
```

### `CLONE_SYSVSEM`

```rust
const CLONE_SYSVSEM: u32 = 262_144u32;
```

### `CLONE_SETTLS`

```rust
const CLONE_SETTLS: u32 = 524_288u32;
```

### `CLONE_PARENT_SETTID`

```rust
const CLONE_PARENT_SETTID: u32 = 1_048_576u32;
```

### `CLONE_CHILD_CLEARTID`

```rust
const CLONE_CHILD_CLEARTID: u32 = 2_097_152u32;
```

### `CLONE_DETACHED`

```rust
const CLONE_DETACHED: u32 = 4_194_304u32;
```

### `CLONE_UNTRACED`

```rust
const CLONE_UNTRACED: u32 = 8_388_608u32;
```

### `CLONE_CHILD_SETTID`

```rust
const CLONE_CHILD_SETTID: u32 = 16_777_216u32;
```

### `CLONE_NEWCGROUP`

```rust
const CLONE_NEWCGROUP: u32 = 33_554_432u32;
```

### `CLONE_NEWUTS`

```rust
const CLONE_NEWUTS: u32 = 67_108_864u32;
```

### `CLONE_NEWIPC`

```rust
const CLONE_NEWIPC: u32 = 134_217_728u32;
```

### `CLONE_NEWUSER`

```rust
const CLONE_NEWUSER: u32 = 268_435_456u32;
```

### `CLONE_NEWPID`

```rust
const CLONE_NEWPID: u32 = 536_870_912u32;
```

### `CLONE_NEWNET`

```rust
const CLONE_NEWNET: u32 = 1_073_741_824u32;
```

### `CLONE_IO`

```rust
const CLONE_IO: u32 = 2_147_483_648u32;
```

### `CLONE_CLEAR_SIGHAND`

```rust
const CLONE_CLEAR_SIGHAND: u64 = 4_294_967_296u64;
```

### `CLONE_INTO_CGROUP`

```rust
const CLONE_INTO_CGROUP: u64 = 8_589_934_592u64;
```

### `CLONE_NEWTIME`

```rust
const CLONE_NEWTIME: u32 = 128u32;
```

### `CLONE_ARGS_SIZE_VER0`

```rust
const CLONE_ARGS_SIZE_VER0: u32 = 64u32;
```

### `CLONE_ARGS_SIZE_VER1`

```rust
const CLONE_ARGS_SIZE_VER1: u32 = 80u32;
```

### `CLONE_ARGS_SIZE_VER2`

```rust
const CLONE_ARGS_SIZE_VER2: u32 = 88u32;
```

### `SCHED_NORMAL`

```rust
const SCHED_NORMAL: u32 = 0u32;
```

### `SCHED_FIFO`

```rust
const SCHED_FIFO: u32 = 1u32;
```

### `SCHED_RR`

```rust
const SCHED_RR: u32 = 2u32;
```

### `SCHED_BATCH`

```rust
const SCHED_BATCH: u32 = 3u32;
```

### `SCHED_IDLE`

```rust
const SCHED_IDLE: u32 = 5u32;
```

### `SCHED_DEADLINE`

```rust
const SCHED_DEADLINE: u32 = 6u32;
```

### `SCHED_EXT`

```rust
const SCHED_EXT: u32 = 7u32;
```

### `SCHED_RESET_ON_FORK`

```rust
const SCHED_RESET_ON_FORK: u32 = 1_073_741_824u32;
```

### `SCHED_FLAG_RESET_ON_FORK`

```rust
const SCHED_FLAG_RESET_ON_FORK: u32 = 1u32;
```

### `SCHED_FLAG_RECLAIM`

```rust
const SCHED_FLAG_RECLAIM: u32 = 2u32;
```

### `SCHED_FLAG_DL_OVERRUN`

```rust
const SCHED_FLAG_DL_OVERRUN: u32 = 4u32;
```

### `SCHED_FLAG_KEEP_POLICY`

```rust
const SCHED_FLAG_KEEP_POLICY: u32 = 8u32;
```

### `SCHED_FLAG_KEEP_PARAMS`

```rust
const SCHED_FLAG_KEEP_PARAMS: u32 = 16u32;
```

### `SCHED_FLAG_UTIL_CLAMP_MIN`

```rust
const SCHED_FLAG_UTIL_CLAMP_MIN: u32 = 32u32;
```

### `SCHED_FLAG_UTIL_CLAMP_MAX`

```rust
const SCHED_FLAG_UTIL_CLAMP_MAX: u32 = 64u32;
```

### `SCHED_FLAG_KEEP_ALL`

```rust
const SCHED_FLAG_KEEP_ALL: u32 = 24u32;
```

### `SCHED_FLAG_UTIL_CLAMP`

```rust
const SCHED_FLAG_UTIL_CLAMP: u32 = 96u32;
```

### `SCHED_FLAG_ALL`

```rust
const SCHED_FLAG_ALL: u32 = 127u32;
```

### `NSIG`

```rust
const NSIG: u32 = 32u32;
```

### `SIGHUP`

```rust
const SIGHUP: u32 = 1u32;
```

### `SIGINT`

```rust
const SIGINT: u32 = 2u32;
```

### `SIGQUIT`

```rust
const SIGQUIT: u32 = 3u32;
```

### `SIGILL`

```rust
const SIGILL: u32 = 4u32;
```

### `SIGTRAP`

```rust
const SIGTRAP: u32 = 5u32;
```

### `SIGABRT`

```rust
const SIGABRT: u32 = 6u32;
```

### `SIGIOT`

```rust
const SIGIOT: u32 = 6u32;
```

### `SIGBUS`

```rust
const SIGBUS: u32 = 7u32;
```

### `SIGFPE`

```rust
const SIGFPE: u32 = 8u32;
```

### `SIGKILL`

```rust
const SIGKILL: u32 = 9u32;
```

### `SIGUSR1`

```rust
const SIGUSR1: u32 = 10u32;
```

### `SIGSEGV`

```rust
const SIGSEGV: u32 = 11u32;
```

### `SIGUSR2`

```rust
const SIGUSR2: u32 = 12u32;
```

### `SIGPIPE`

```rust
const SIGPIPE: u32 = 13u32;
```

### `SIGALRM`

```rust
const SIGALRM: u32 = 14u32;
```

### `SIGTERM`

```rust
const SIGTERM: u32 = 15u32;
```

### `SIGSTKFLT`

```rust
const SIGSTKFLT: u32 = 16u32;
```

### `SIGCHLD`

```rust
const SIGCHLD: u32 = 17u32;
```

### `SIGCONT`

```rust
const SIGCONT: u32 = 18u32;
```

### `SIGSTOP`

```rust
const SIGSTOP: u32 = 19u32;
```

### `SIGTSTP`

```rust
const SIGTSTP: u32 = 20u32;
```

### `SIGTTIN`

```rust
const SIGTTIN: u32 = 21u32;
```

### `SIGTTOU`

```rust
const SIGTTOU: u32 = 22u32;
```

### `SIGURG`

```rust
const SIGURG: u32 = 23u32;
```

### `SIGXCPU`

```rust
const SIGXCPU: u32 = 24u32;
```

### `SIGXFSZ`

```rust
const SIGXFSZ: u32 = 25u32;
```

### `SIGVTALRM`

```rust
const SIGVTALRM: u32 = 26u32;
```

### `SIGPROF`

```rust
const SIGPROF: u32 = 27u32;
```

### `SIGWINCH`

```rust
const SIGWINCH: u32 = 28u32;
```

### `SIGIO`

```rust
const SIGIO: u32 = 29u32;
```

### `SIGPOLL`

```rust
const SIGPOLL: u32 = 29u32;
```

### `SIGPWR`

```rust
const SIGPWR: u32 = 30u32;
```

### `SIGSYS`

```rust
const SIGSYS: u32 = 31u32;
```

### `SIGUNUSED`

```rust
const SIGUNUSED: u32 = 31u32;
```

### `SIGRTMIN`

```rust
const SIGRTMIN: u32 = 32u32;
```

### `SA_RESTORER`

```rust
const SA_RESTORER: u32 = 67_108_864u32;
```

### `MINSIGSTKSZ`

```rust
const MINSIGSTKSZ: u32 = 2_048u32;
```

### `SIGSTKSZ`

```rust
const SIGSTKSZ: u32 = 8_192u32;
```

### `SA_NOCLDSTOP`

```rust
const SA_NOCLDSTOP: u32 = 1u32;
```

### `SA_NOCLDWAIT`

```rust
const SA_NOCLDWAIT: u32 = 2u32;
```

### `SA_SIGINFO`

```rust
const SA_SIGINFO: u32 = 4u32;
```

### `SA_UNSUPPORTED`

```rust
const SA_UNSUPPORTED: u32 = 1_024u32;
```

### `SA_EXPOSE_TAGBITS`

```rust
const SA_EXPOSE_TAGBITS: u32 = 2_048u32;
```

### `SA_ONSTACK`

```rust
const SA_ONSTACK: u32 = 134_217_728u32;
```

### `SA_RESTART`

```rust
const SA_RESTART: u32 = 268_435_456u32;
```

### `SA_NODEFER`

```rust
const SA_NODEFER: u32 = 1_073_741_824u32;
```

### `SA_RESETHAND`

```rust
const SA_RESETHAND: u32 = 2_147_483_648u32;
```

### `SA_NOMASK`

```rust
const SA_NOMASK: u32 = 1_073_741_824u32;
```

### `SA_ONESHOT`

```rust
const SA_ONESHOT: u32 = 2_147_483_648u32;
```

### `SIG_BLOCK`

```rust
const SIG_BLOCK: u32 = 0u32;
```

### `SIG_UNBLOCK`

```rust
const SIG_UNBLOCK: u32 = 1u32;
```

### `SIG_SETMASK`

```rust
const SIG_SETMASK: u32 = 2u32;
```

### `SI_MAX_SIZE`

```rust
const SI_MAX_SIZE: u32 = 128u32;
```

### `SI_USER`

```rust
const SI_USER: u32 = 0u32;
```

### `SI_KERNEL`

```rust
const SI_KERNEL: u32 = 128u32;
```

### `SI_QUEUE`

```rust
const SI_QUEUE: i32 = -1i32;
```

### `SI_TIMER`

```rust
const SI_TIMER: i32 = -2i32;
```

### `SI_MESGQ`

```rust
const SI_MESGQ: i32 = -3i32;
```

### `SI_ASYNCIO`

```rust
const SI_ASYNCIO: i32 = -4i32;
```

### `SI_SIGIO`

```rust
const SI_SIGIO: i32 = -5i32;
```

### `SI_TKILL`

```rust
const SI_TKILL: i32 = -6i32;
```

### `SI_DETHREAD`

```rust
const SI_DETHREAD: i32 = -7i32;
```

### `SI_ASYNCNL`

```rust
const SI_ASYNCNL: i32 = -60i32;
```

### `ILL_ILLOPC`

```rust
const ILL_ILLOPC: u32 = 1u32;
```

### `ILL_ILLOPN`

```rust
const ILL_ILLOPN: u32 = 2u32;
```

### `ILL_ILLADR`

```rust
const ILL_ILLADR: u32 = 3u32;
```

### `ILL_ILLTRP`

```rust
const ILL_ILLTRP: u32 = 4u32;
```

### `ILL_PRVOPC`

```rust
const ILL_PRVOPC: u32 = 5u32;
```

### `ILL_PRVREG`

```rust
const ILL_PRVREG: u32 = 6u32;
```

### `ILL_COPROC`

```rust
const ILL_COPROC: u32 = 7u32;
```

### `ILL_BADSTK`

```rust
const ILL_BADSTK: u32 = 8u32;
```

### `ILL_BADIADDR`

```rust
const ILL_BADIADDR: u32 = 9u32;
```

### `__ILL_BREAK`

```rust
const __ILL_BREAK: u32 = 10u32;
```

### `__ILL_BNDMOD`

```rust
const __ILL_BNDMOD: u32 = 11u32;
```

### `NSIGILL`

```rust
const NSIGILL: u32 = 11u32;
```

### `FPE_INTDIV`

```rust
const FPE_INTDIV: u32 = 1u32;
```

### `FPE_INTOVF`

```rust
const FPE_INTOVF: u32 = 2u32;
```

### `FPE_FLTDIV`

```rust
const FPE_FLTDIV: u32 = 3u32;
```

### `FPE_FLTOVF`

```rust
const FPE_FLTOVF: u32 = 4u32;
```

### `FPE_FLTUND`

```rust
const FPE_FLTUND: u32 = 5u32;
```

### `FPE_FLTRES`

```rust
const FPE_FLTRES: u32 = 6u32;
```

### `FPE_FLTINV`

```rust
const FPE_FLTINV: u32 = 7u32;
```

### `FPE_FLTSUB`

```rust
const FPE_FLTSUB: u32 = 8u32;
```

### `__FPE_DECOVF`

```rust
const __FPE_DECOVF: u32 = 9u32;
```

### `__FPE_DECDIV`

```rust
const __FPE_DECDIV: u32 = 10u32;
```

### `__FPE_DECERR`

```rust
const __FPE_DECERR: u32 = 11u32;
```

### `__FPE_INVASC`

```rust
const __FPE_INVASC: u32 = 12u32;
```

### `__FPE_INVDEC`

```rust
const __FPE_INVDEC: u32 = 13u32;
```

### `FPE_FLTUNK`

```rust
const FPE_FLTUNK: u32 = 14u32;
```

### `FPE_CONDTRAP`

```rust
const FPE_CONDTRAP: u32 = 15u32;
```

### `NSIGFPE`

```rust
const NSIGFPE: u32 = 15u32;
```

### `SEGV_MAPERR`

```rust
const SEGV_MAPERR: u32 = 1u32;
```

### `SEGV_ACCERR`

```rust
const SEGV_ACCERR: u32 = 2u32;
```

### `SEGV_BNDERR`

```rust
const SEGV_BNDERR: u32 = 3u32;
```

### `SEGV_PKUERR`

```rust
const SEGV_PKUERR: u32 = 4u32;
```

### `SEGV_ACCADI`

```rust
const SEGV_ACCADI: u32 = 5u32;
```

### `SEGV_ADIDERR`

```rust
const SEGV_ADIDERR: u32 = 6u32;
```

### `SEGV_ADIPERR`

```rust
const SEGV_ADIPERR: u32 = 7u32;
```

### `SEGV_MTEAERR`

```rust
const SEGV_MTEAERR: u32 = 8u32;
```

### `SEGV_MTESERR`

```rust
const SEGV_MTESERR: u32 = 9u32;
```

### `SEGV_CPERR`

```rust
const SEGV_CPERR: u32 = 10u32;
```

### `NSIGSEGV`

```rust
const NSIGSEGV: u32 = 10u32;
```

### `BUS_ADRALN`

```rust
const BUS_ADRALN: u32 = 1u32;
```

### `BUS_ADRERR`

```rust
const BUS_ADRERR: u32 = 2u32;
```

### `BUS_OBJERR`

```rust
const BUS_OBJERR: u32 = 3u32;
```

### `BUS_MCEERR_AR`

```rust
const BUS_MCEERR_AR: u32 = 4u32;
```

### `BUS_MCEERR_AO`

```rust
const BUS_MCEERR_AO: u32 = 5u32;
```

### `NSIGBUS`

```rust
const NSIGBUS: u32 = 5u32;
```

### `TRAP_BRKPT`

```rust
const TRAP_BRKPT: u32 = 1u32;
```

### `TRAP_TRACE`

```rust
const TRAP_TRACE: u32 = 2u32;
```

### `TRAP_BRANCH`

```rust
const TRAP_BRANCH: u32 = 3u32;
```

### `TRAP_HWBKPT`

```rust
const TRAP_HWBKPT: u32 = 4u32;
```

### `TRAP_UNK`

```rust
const TRAP_UNK: u32 = 5u32;
```

### `TRAP_PERF`

```rust
const TRAP_PERF: u32 = 6u32;
```

### `NSIGTRAP`

```rust
const NSIGTRAP: u32 = 6u32;
```

### `TRAP_PERF_FLAG_ASYNC`

```rust
const TRAP_PERF_FLAG_ASYNC: u32 = 1u32;
```

### `CLD_EXITED`

```rust
const CLD_EXITED: u32 = 1u32;
```

### `CLD_KILLED`

```rust
const CLD_KILLED: u32 = 2u32;
```

### `CLD_DUMPED`

```rust
const CLD_DUMPED: u32 = 3u32;
```

### `CLD_TRAPPED`

```rust
const CLD_TRAPPED: u32 = 4u32;
```

### `CLD_STOPPED`

```rust
const CLD_STOPPED: u32 = 5u32;
```

### `CLD_CONTINUED`

```rust
const CLD_CONTINUED: u32 = 6u32;
```

### `NSIGCHLD`

```rust
const NSIGCHLD: u32 = 6u32;
```

### `POLL_IN`

```rust
const POLL_IN: u32 = 1u32;
```

### `POLL_OUT`

```rust
const POLL_OUT: u32 = 2u32;
```

### `POLL_MSG`

```rust
const POLL_MSG: u32 = 3u32;
```

### `POLL_ERR`

```rust
const POLL_ERR: u32 = 4u32;
```

### `POLL_PRI`

```rust
const POLL_PRI: u32 = 5u32;
```

### `POLL_HUP`

```rust
const POLL_HUP: u32 = 6u32;
```

### `NSIGPOLL`

```rust
const NSIGPOLL: u32 = 6u32;
```

### `SYS_SECCOMP`

```rust
const SYS_SECCOMP: u32 = 1u32;
```

### `SYS_USER_DISPATCH`

```rust
const SYS_USER_DISPATCH: u32 = 2u32;
```

### `NSIGSYS`

```rust
const NSIGSYS: u32 = 2u32;
```

### `EMT_TAGOVF`

```rust
const EMT_TAGOVF: u32 = 1u32;
```

### `NSIGEMT`

```rust
const NSIGEMT: u32 = 1u32;
```

### `SIGEV_SIGNAL`

```rust
const SIGEV_SIGNAL: u32 = 0u32;
```

### `SIGEV_NONE`

```rust
const SIGEV_NONE: u32 = 1u32;
```

### `SIGEV_THREAD`

```rust
const SIGEV_THREAD: u32 = 2u32;
```

### `SIGEV_THREAD_ID`

```rust
const SIGEV_THREAD_ID: u32 = 4u32;
```

### `SIGEV_MAX_SIZE`

```rust
const SIGEV_MAX_SIZE: u32 = 64u32;
```

### `SS_ONSTACK`

```rust
const SS_ONSTACK: u32 = 1u32;
```

### `SS_DISABLE`

```rust
const SS_DISABLE: u32 = 2u32;
```

### `SS_AUTODISARM`

```rust
const SS_AUTODISARM: u32 = 2_147_483_648u32;
```

### `SS_FLAG_BITS`

```rust
const SS_FLAG_BITS: u32 = 2_147_483_648u32;
```

### `S_IFMT`

```rust
const S_IFMT: u32 = 61_440u32;
```

### `S_IFSOCK`

```rust
const S_IFSOCK: u32 = 49_152u32;
```

### `S_IFLNK`

```rust
const S_IFLNK: u32 = 40_960u32;
```

### `S_IFREG`

```rust
const S_IFREG: u32 = 32_768u32;
```

### `S_IFBLK`

```rust
const S_IFBLK: u32 = 24_576u32;
```

### `S_IFDIR`

```rust
const S_IFDIR: u32 = 16_384u32;
```

### `S_IFCHR`

```rust
const S_IFCHR: u32 = 8_192u32;
```

### `S_IFIFO`

```rust
const S_IFIFO: u32 = 4_096u32;
```

### `S_ISUID`

```rust
const S_ISUID: u32 = 2_048u32;
```

### `S_ISGID`

```rust
const S_ISGID: u32 = 1_024u32;
```

### `S_ISVTX`

```rust
const S_ISVTX: u32 = 512u32;
```

### `S_IRWXU`

```rust
const S_IRWXU: u32 = 448u32;
```

### `S_IRUSR`

```rust
const S_IRUSR: u32 = 256u32;
```

### `S_IWUSR`

```rust
const S_IWUSR: u32 = 128u32;
```

### `S_IXUSR`

```rust
const S_IXUSR: u32 = 64u32;
```

### `S_IRWXG`

```rust
const S_IRWXG: u32 = 56u32;
```

### `S_IRGRP`

```rust
const S_IRGRP: u32 = 32u32;
```

### `S_IWGRP`

```rust
const S_IWGRP: u32 = 16u32;
```

### `S_IXGRP`

```rust
const S_IXGRP: u32 = 8u32;
```

### `S_IRWXO`

```rust
const S_IRWXO: u32 = 7u32;
```

### `S_IROTH`

```rust
const S_IROTH: u32 = 4u32;
```

### `S_IWOTH`

```rust
const S_IWOTH: u32 = 2u32;
```

### `S_IXOTH`

```rust
const S_IXOTH: u32 = 1u32;
```

### `STATX_TYPE`

```rust
const STATX_TYPE: u32 = 1u32;
```

### `STATX_MODE`

```rust
const STATX_MODE: u32 = 2u32;
```

### `STATX_NLINK`

```rust
const STATX_NLINK: u32 = 4u32;
```

### `STATX_UID`

```rust
const STATX_UID: u32 = 8u32;
```

### `STATX_GID`

```rust
const STATX_GID: u32 = 16u32;
```

### `STATX_ATIME`

```rust
const STATX_ATIME: u32 = 32u32;
```

### `STATX_MTIME`

```rust
const STATX_MTIME: u32 = 64u32;
```

### `STATX_CTIME`

```rust
const STATX_CTIME: u32 = 128u32;
```

### `STATX_INO`

```rust
const STATX_INO: u32 = 256u32;
```

### `STATX_SIZE`

```rust
const STATX_SIZE: u32 = 512u32;
```

### `STATX_BLOCKS`

```rust
const STATX_BLOCKS: u32 = 1_024u32;
```

### `STATX_BASIC_STATS`

```rust
const STATX_BASIC_STATS: u32 = 2_047u32;
```

### `STATX_BTIME`

```rust
const STATX_BTIME: u32 = 2_048u32;
```

### `STATX_MNT_ID`

```rust
const STATX_MNT_ID: u32 = 4_096u32;
```

### `STATX_DIOALIGN`

```rust
const STATX_DIOALIGN: u32 = 8_192u32;
```

### `STATX_MNT_ID_UNIQUE`

```rust
const STATX_MNT_ID_UNIQUE: u32 = 16_384u32;
```

### `STATX_SUBVOL`

```rust
const STATX_SUBVOL: u32 = 32_768u32;
```

### `STATX_WRITE_ATOMIC`

```rust
const STATX_WRITE_ATOMIC: u32 = 65_536u32;
```

### `STATX_DIO_READ_ALIGN`

```rust
const STATX_DIO_READ_ALIGN: u32 = 131_072u32;
```

### `STATX__RESERVED`

```rust
const STATX__RESERVED: u32 = 2_147_483_648u32;
```

### `STATX_ALL`

```rust
const STATX_ALL: u32 = 4_095u32;
```

### `STATX_ATTR_COMPRESSED`

```rust
const STATX_ATTR_COMPRESSED: u32 = 4u32;
```

### `STATX_ATTR_IMMUTABLE`

```rust
const STATX_ATTR_IMMUTABLE: u32 = 16u32;
```

### `STATX_ATTR_APPEND`

```rust
const STATX_ATTR_APPEND: u32 = 32u32;
```

### `STATX_ATTR_NODUMP`

```rust
const STATX_ATTR_NODUMP: u32 = 64u32;
```

### `STATX_ATTR_ENCRYPTED`

```rust
const STATX_ATTR_ENCRYPTED: u32 = 2_048u32;
```

### `STATX_ATTR_AUTOMOUNT`

```rust
const STATX_ATTR_AUTOMOUNT: u32 = 4_096u32;
```

### `STATX_ATTR_MOUNT_ROOT`

```rust
const STATX_ATTR_MOUNT_ROOT: u32 = 8_192u32;
```

### `STATX_ATTR_VERITY`

```rust
const STATX_ATTR_VERITY: u32 = 1_048_576u32;
```

### `STATX_ATTR_DAX`

```rust
const STATX_ATTR_DAX: u32 = 2_097_152u32;
```

### `STATX_ATTR_WRITE_ATOMIC`

```rust
const STATX_ATTR_WRITE_ATOMIC: u32 = 4_194_304u32;
```

### `IGNBRK`

```rust
const IGNBRK: u32 = 1u32;
```

### `BRKINT`

```rust
const BRKINT: u32 = 2u32;
```

### `IGNPAR`

```rust
const IGNPAR: u32 = 4u32;
```

### `PARMRK`

```rust
const PARMRK: u32 = 8u32;
```

### `INPCK`

```rust
const INPCK: u32 = 16u32;
```

### `ISTRIP`

```rust
const ISTRIP: u32 = 32u32;
```

### `INLCR`

```rust
const INLCR: u32 = 64u32;
```

### `IGNCR`

```rust
const IGNCR: u32 = 128u32;
```

### `ICRNL`

```rust
const ICRNL: u32 = 256u32;
```

### `IXANY`

```rust
const IXANY: u32 = 2_048u32;
```

### `OPOST`

```rust
const OPOST: u32 = 1u32;
```

### `OCRNL`

```rust
const OCRNL: u32 = 8u32;
```

### `ONOCR`

```rust
const ONOCR: u32 = 16u32;
```

### `ONLRET`

```rust
const ONLRET: u32 = 32u32;
```

### `OFILL`

```rust
const OFILL: u32 = 64u32;
```

### `OFDEL`

```rust
const OFDEL: u32 = 128u32;
```

### `B0`

```rust
const B0: u32 = 0u32;
```

### `B50`

```rust
const B50: u32 = 1u32;
```

### `B75`

```rust
const B75: u32 = 2u32;
```

### `B110`

```rust
const B110: u32 = 3u32;
```

### `B134`

```rust
const B134: u32 = 4u32;
```

### `B150`

```rust
const B150: u32 = 5u32;
```

### `B200`

```rust
const B200: u32 = 6u32;
```

### `B300`

```rust
const B300: u32 = 7u32;
```

### `B600`

```rust
const B600: u32 = 8u32;
```

### `B1200`

```rust
const B1200: u32 = 9u32;
```

### `B1800`

```rust
const B1800: u32 = 10u32;
```

### `B2400`

```rust
const B2400: u32 = 11u32;
```

### `B4800`

```rust
const B4800: u32 = 12u32;
```

### `B9600`

```rust
const B9600: u32 = 13u32;
```

### `B19200`

```rust
const B19200: u32 = 14u32;
```

### `B38400`

```rust
const B38400: u32 = 15u32;
```

### `EXTA`

```rust
const EXTA: u32 = 14u32;
```

### `EXTB`

```rust
const EXTB: u32 = 15u32;
```

### `ADDRB`

```rust
const ADDRB: u32 = 536_870_912u32;
```

### `CMSPAR`

```rust
const CMSPAR: u32 = 1_073_741_824u32;
```

### `CRTSCTS`

```rust
const CRTSCTS: u32 = 2_147_483_648u32;
```

### `IBSHIFT`

```rust
const IBSHIFT: u32 = 16u32;
```

### `TCOOFF`

```rust
const TCOOFF: u32 = 0u32;
```

### `TCOON`

```rust
const TCOON: u32 = 1u32;
```

### `TCIOFF`

```rust
const TCIOFF: u32 = 2u32;
```

### `TCION`

```rust
const TCION: u32 = 3u32;
```

### `TCIFLUSH`

```rust
const TCIFLUSH: u32 = 0u32;
```

### `TCOFLUSH`

```rust
const TCOFLUSH: u32 = 1u32;
```

### `TCIOFLUSH`

```rust
const TCIOFLUSH: u32 = 2u32;
```

### `NCCS`

```rust
const NCCS: u32 = 19u32;
```

### `VINTR`

```rust
const VINTR: u32 = 0u32;
```

### `VQUIT`

```rust
const VQUIT: u32 = 1u32;
```

### `VERASE`

```rust
const VERASE: u32 = 2u32;
```

### `VKILL`

```rust
const VKILL: u32 = 3u32;
```

### `VEOF`

```rust
const VEOF: u32 = 4u32;
```

### `VTIME`

```rust
const VTIME: u32 = 5u32;
```

### `VMIN`

```rust
const VMIN: u32 = 6u32;
```

### `VSWTC`

```rust
const VSWTC: u32 = 7u32;
```

### `VSTART`

```rust
const VSTART: u32 = 8u32;
```

### `VSTOP`

```rust
const VSTOP: u32 = 9u32;
```

### `VSUSP`

```rust
const VSUSP: u32 = 10u32;
```

### `VEOL`

```rust
const VEOL: u32 = 11u32;
```

### `VREPRINT`

```rust
const VREPRINT: u32 = 12u32;
```

### `VDISCARD`

```rust
const VDISCARD: u32 = 13u32;
```

### `VWERASE`

```rust
const VWERASE: u32 = 14u32;
```

### `VLNEXT`

```rust
const VLNEXT: u32 = 15u32;
```

### `VEOL2`

```rust
const VEOL2: u32 = 16u32;
```

### `IUCLC`

```rust
const IUCLC: u32 = 512u32;
```

### `IXON`

```rust
const IXON: u32 = 1_024u32;
```

### `IXOFF`

```rust
const IXOFF: u32 = 4_096u32;
```

### `IMAXBEL`

```rust
const IMAXBEL: u32 = 8_192u32;
```

### `IUTF8`

```rust
const IUTF8: u32 = 16_384u32;
```

### `OLCUC`

```rust
const OLCUC: u32 = 2u32;
```

### `ONLCR`

```rust
const ONLCR: u32 = 4u32;
```

### `NLDLY`

```rust
const NLDLY: u32 = 256u32;
```

### `NL0`

```rust
const NL0: u32 = 0u32;
```

### `NL1`

```rust
const NL1: u32 = 256u32;
```

### `CRDLY`

```rust
const CRDLY: u32 = 1_536u32;
```

### `CR0`

```rust
const CR0: u32 = 0u32;
```

### `CR1`

```rust
const CR1: u32 = 512u32;
```

### `CR2`

```rust
const CR2: u32 = 1_024u32;
```

### `CR3`

```rust
const CR3: u32 = 1_536u32;
```

### `TABDLY`

```rust
const TABDLY: u32 = 6_144u32;
```

### `TAB0`

```rust
const TAB0: u32 = 0u32;
```

### `TAB1`

```rust
const TAB1: u32 = 2_048u32;
```

### `TAB2`

```rust
const TAB2: u32 = 4_096u32;
```

### `TAB3`

```rust
const TAB3: u32 = 6_144u32;
```

### `XTABS`

```rust
const XTABS: u32 = 6_144u32;
```

### `BSDLY`

```rust
const BSDLY: u32 = 8_192u32;
```

### `BS0`

```rust
const BS0: u32 = 0u32;
```

### `BS1`

```rust
const BS1: u32 = 8_192u32;
```

### `VTDLY`

```rust
const VTDLY: u32 = 16_384u32;
```

### `VT0`

```rust
const VT0: u32 = 0u32;
```

### `VT1`

```rust
const VT1: u32 = 16_384u32;
```

### `FFDLY`

```rust
const FFDLY: u32 = 32_768u32;
```

### `FF0`

```rust
const FF0: u32 = 0u32;
```

### `FF1`

```rust
const FF1: u32 = 32_768u32;
```

### `CBAUD`

```rust
const CBAUD: u32 = 4_111u32;
```

### `CSIZE`

```rust
const CSIZE: u32 = 48u32;
```

### `CS5`

```rust
const CS5: u32 = 0u32;
```

### `CS6`

```rust
const CS6: u32 = 16u32;
```

### `CS7`

```rust
const CS7: u32 = 32u32;
```

### `CS8`

```rust
const CS8: u32 = 48u32;
```

### `CSTOPB`

```rust
const CSTOPB: u32 = 64u32;
```

### `CREAD`

```rust
const CREAD: u32 = 128u32;
```

### `PARENB`

```rust
const PARENB: u32 = 256u32;
```

### `PARODD`

```rust
const PARODD: u32 = 512u32;
```

### `HUPCL`

```rust
const HUPCL: u32 = 1_024u32;
```

### `CLOCAL`

```rust
const CLOCAL: u32 = 2_048u32;
```

### `CBAUDEX`

```rust
const CBAUDEX: u32 = 4_096u32;
```

### `BOTHER`

```rust
const BOTHER: u32 = 4_096u32;
```

### `B57600`

```rust
const B57600: u32 = 4_097u32;
```

### `B115200`

```rust
const B115200: u32 = 4_098u32;
```

### `B230400`

```rust
const B230400: u32 = 4_099u32;
```

### `B460800`

```rust
const B460800: u32 = 4_100u32;
```

### `B500000`

```rust
const B500000: u32 = 4_101u32;
```

### `B576000`

```rust
const B576000: u32 = 4_102u32;
```

### `B921600`

```rust
const B921600: u32 = 4_103u32;
```

### `B1000000`

```rust
const B1000000: u32 = 4_104u32;
```

### `B1152000`

```rust
const B1152000: u32 = 4_105u32;
```

### `B1500000`

```rust
const B1500000: u32 = 4_106u32;
```

### `B2000000`

```rust
const B2000000: u32 = 4_107u32;
```

### `B2500000`

```rust
const B2500000: u32 = 4_108u32;
```

### `B3000000`

```rust
const B3000000: u32 = 4_109u32;
```

### `B3500000`

```rust
const B3500000: u32 = 4_110u32;
```

### `B4000000`

```rust
const B4000000: u32 = 4_111u32;
```

### `CIBAUD`

```rust
const CIBAUD: u32 = 269_418_496u32;
```

### `ISIG`

```rust
const ISIG: u32 = 1u32;
```

### `ICANON`

```rust
const ICANON: u32 = 2u32;
```

### `XCASE`

```rust
const XCASE: u32 = 4u32;
```

### `ECHO`

```rust
const ECHO: u32 = 8u32;
```

### `ECHOE`

```rust
const ECHOE: u32 = 16u32;
```

### `ECHOK`

```rust
const ECHOK: u32 = 32u32;
```

### `ECHONL`

```rust
const ECHONL: u32 = 64u32;
```

### `NOFLSH`

```rust
const NOFLSH: u32 = 128u32;
```

### `TOSTOP`

```rust
const TOSTOP: u32 = 256u32;
```

### `ECHOCTL`

```rust
const ECHOCTL: u32 = 512u32;
```

### `ECHOPRT`

```rust
const ECHOPRT: u32 = 1_024u32;
```

### `ECHOKE`

```rust
const ECHOKE: u32 = 2_048u32;
```

### `FLUSHO`

```rust
const FLUSHO: u32 = 4_096u32;
```

### `PENDIN`

```rust
const PENDIN: u32 = 16_384u32;
```

### `IEXTEN`

```rust
const IEXTEN: u32 = 32_768u32;
```

### `EXTPROC`

```rust
const EXTPROC: u32 = 65_536u32;
```

### `TCSANOW`

```rust
const TCSANOW: u32 = 0u32;
```

### `TCSADRAIN`

```rust
const TCSADRAIN: u32 = 1u32;
```

### `TCSAFLUSH`

```rust
const TCSAFLUSH: u32 = 2u32;
```

### `TIOCPKT_DATA`

```rust
const TIOCPKT_DATA: u32 = 0u32;
```

### `TIOCPKT_FLUSHREAD`

```rust
const TIOCPKT_FLUSHREAD: u32 = 1u32;
```

### `TIOCPKT_FLUSHWRITE`

```rust
const TIOCPKT_FLUSHWRITE: u32 = 2u32;
```

### `TIOCPKT_STOP`

```rust
const TIOCPKT_STOP: u32 = 4u32;
```

### `TIOCPKT_START`

```rust
const TIOCPKT_START: u32 = 8u32;
```

### `TIOCPKT_NOSTOP`

```rust
const TIOCPKT_NOSTOP: u32 = 16u32;
```

### `TIOCPKT_DOSTOP`

```rust
const TIOCPKT_DOSTOP: u32 = 32u32;
```

### `TIOCPKT_IOCTL`

```rust
const TIOCPKT_IOCTL: u32 = 64u32;
```

### `TIOCSER_TEMT`

```rust
const TIOCSER_TEMT: u32 = 1u32;
```

### `NCC`

```rust
const NCC: u32 = 8u32;
```

### `TIOCM_LE`

```rust
const TIOCM_LE: u32 = 1u32;
```

### `TIOCM_DTR`

```rust
const TIOCM_DTR: u32 = 2u32;
```

### `TIOCM_RTS`

```rust
const TIOCM_RTS: u32 = 4u32;
```

### `TIOCM_ST`

```rust
const TIOCM_ST: u32 = 8u32;
```

### `TIOCM_SR`

```rust
const TIOCM_SR: u32 = 16u32;
```

### `TIOCM_CTS`

```rust
const TIOCM_CTS: u32 = 32u32;
```

### `TIOCM_CAR`

```rust
const TIOCM_CAR: u32 = 64u32;
```

### `TIOCM_RNG`

```rust
const TIOCM_RNG: u32 = 128u32;
```

### `TIOCM_DSR`

```rust
const TIOCM_DSR: u32 = 256u32;
```

### `TIOCM_CD`

```rust
const TIOCM_CD: u32 = 64u32;
```

### `TIOCM_RI`

```rust
const TIOCM_RI: u32 = 128u32;
```

### `TIOCM_OUT1`

```rust
const TIOCM_OUT1: u32 = 8_192u32;
```

### `TIOCM_OUT2`

```rust
const TIOCM_OUT2: u32 = 16_384u32;
```

### `TIOCM_LOOP`

```rust
const TIOCM_LOOP: u32 = 32_768u32;
```

### `ITIMER_REAL`

```rust
const ITIMER_REAL: u32 = 0u32;
```

### `ITIMER_VIRTUAL`

```rust
const ITIMER_VIRTUAL: u32 = 1u32;
```

### `ITIMER_PROF`

```rust
const ITIMER_PROF: u32 = 2u32;
```

### `CLOCK_REALTIME`

```rust
const CLOCK_REALTIME: u32 = 0u32;
```

### `CLOCK_MONOTONIC`

```rust
const CLOCK_MONOTONIC: u32 = 1u32;
```

### `CLOCK_PROCESS_CPUTIME_ID`

```rust
const CLOCK_PROCESS_CPUTIME_ID: u32 = 2u32;
```

### `CLOCK_THREAD_CPUTIME_ID`

```rust
const CLOCK_THREAD_CPUTIME_ID: u32 = 3u32;
```

### `CLOCK_MONOTONIC_RAW`

```rust
const CLOCK_MONOTONIC_RAW: u32 = 4u32;
```

### `CLOCK_REALTIME_COARSE`

```rust
const CLOCK_REALTIME_COARSE: u32 = 5u32;
```

### `CLOCK_MONOTONIC_COARSE`

```rust
const CLOCK_MONOTONIC_COARSE: u32 = 6u32;
```

### `CLOCK_BOOTTIME`

```rust
const CLOCK_BOOTTIME: u32 = 7u32;
```

### `CLOCK_REALTIME_ALARM`

```rust
const CLOCK_REALTIME_ALARM: u32 = 8u32;
```

### `CLOCK_BOOTTIME_ALARM`

```rust
const CLOCK_BOOTTIME_ALARM: u32 = 9u32;
```

### `CLOCK_SGI_CYCLE`

```rust
const CLOCK_SGI_CYCLE: u32 = 10u32;
```

### `CLOCK_TAI`

```rust
const CLOCK_TAI: u32 = 11u32;
```

### `MAX_CLOCKS`

```rust
const MAX_CLOCKS: u32 = 16u32;
```

### `CLOCKS_MASK`

```rust
const CLOCKS_MASK: u32 = 1u32;
```

### `CLOCKS_MONO`

```rust
const CLOCKS_MONO: u32 = 1u32;
```

### `TIMER_ABSTIME`

```rust
const TIMER_ABSTIME: u32 = 1u32;
```

### `UIO_FASTIOV`

```rust
const UIO_FASTIOV: u32 = 8u32;
```

### `UIO_MAXIOV`

```rust
const UIO_MAXIOV: u32 = 1_024u32;
```

### `__X32_SYSCALL_BIT`

```rust
const __X32_SYSCALL_BIT: u32 = 1_073_741_824u32;
```

### `__NR_read`

```rust
const __NR_read: u32 = 0u32;
```

### `__NR_write`

```rust
const __NR_write: u32 = 1u32;
```

### `__NR_open`

```rust
const __NR_open: u32 = 2u32;
```

### `__NR_close`

```rust
const __NR_close: u32 = 3u32;
```

### `__NR_stat`

```rust
const __NR_stat: u32 = 4u32;
```

### `__NR_fstat`

```rust
const __NR_fstat: u32 = 5u32;
```

### `__NR_lstat`

```rust
const __NR_lstat: u32 = 6u32;
```

### `__NR_poll`

```rust
const __NR_poll: u32 = 7u32;
```

### `__NR_lseek`

```rust
const __NR_lseek: u32 = 8u32;
```

### `__NR_mmap`

```rust
const __NR_mmap: u32 = 9u32;
```

### `__NR_mprotect`

```rust
const __NR_mprotect: u32 = 10u32;
```

### `__NR_munmap`

```rust
const __NR_munmap: u32 = 11u32;
```

### `__NR_brk`

```rust
const __NR_brk: u32 = 12u32;
```

### `__NR_rt_sigaction`

```rust
const __NR_rt_sigaction: u32 = 13u32;
```

### `__NR_rt_sigprocmask`

```rust
const __NR_rt_sigprocmask: u32 = 14u32;
```

### `__NR_rt_sigreturn`

```rust
const __NR_rt_sigreturn: u32 = 15u32;
```

### `__NR_ioctl`

```rust
const __NR_ioctl: u32 = 16u32;
```

### `__NR_pread64`

```rust
const __NR_pread64: u32 = 17u32;
```

### `__NR_pwrite64`

```rust
const __NR_pwrite64: u32 = 18u32;
```

### `__NR_readv`

```rust
const __NR_readv: u32 = 19u32;
```

### `__NR_writev`

```rust
const __NR_writev: u32 = 20u32;
```

### `__NR_access`

```rust
const __NR_access: u32 = 21u32;
```

### `__NR_pipe`

```rust
const __NR_pipe: u32 = 22u32;
```

### `__NR_select`

```rust
const __NR_select: u32 = 23u32;
```

### `__NR_sched_yield`

```rust
const __NR_sched_yield: u32 = 24u32;
```

### `__NR_mremap`

```rust
const __NR_mremap: u32 = 25u32;
```

### `__NR_msync`

```rust
const __NR_msync: u32 = 26u32;
```

### `__NR_mincore`

```rust
const __NR_mincore: u32 = 27u32;
```

### `__NR_madvise`

```rust
const __NR_madvise: u32 = 28u32;
```

### `__NR_shmget`

```rust
const __NR_shmget: u32 = 29u32;
```

### `__NR_shmat`

```rust
const __NR_shmat: u32 = 30u32;
```

### `__NR_shmctl`

```rust
const __NR_shmctl: u32 = 31u32;
```

### `__NR_dup`

```rust
const __NR_dup: u32 = 32u32;
```

### `__NR_dup2`

```rust
const __NR_dup2: u32 = 33u32;
```

### `__NR_pause`

```rust
const __NR_pause: u32 = 34u32;
```

### `__NR_nanosleep`

```rust
const __NR_nanosleep: u32 = 35u32;
```

### `__NR_getitimer`

```rust
const __NR_getitimer: u32 = 36u32;
```

### `__NR_alarm`

```rust
const __NR_alarm: u32 = 37u32;
```

### `__NR_setitimer`

```rust
const __NR_setitimer: u32 = 38u32;
```

### `__NR_getpid`

```rust
const __NR_getpid: u32 = 39u32;
```

### `__NR_sendfile`

```rust
const __NR_sendfile: u32 = 40u32;
```

### `__NR_socket`

```rust
const __NR_socket: u32 = 41u32;
```

### `__NR_connect`

```rust
const __NR_connect: u32 = 42u32;
```

### `__NR_accept`

```rust
const __NR_accept: u32 = 43u32;
```

### `__NR_sendto`

```rust
const __NR_sendto: u32 = 44u32;
```

### `__NR_recvfrom`

```rust
const __NR_recvfrom: u32 = 45u32;
```

### `__NR_sendmsg`

```rust
const __NR_sendmsg: u32 = 46u32;
```

### `__NR_recvmsg`

```rust
const __NR_recvmsg: u32 = 47u32;
```

### `__NR_shutdown`

```rust
const __NR_shutdown: u32 = 48u32;
```

### `__NR_bind`

```rust
const __NR_bind: u32 = 49u32;
```

### `__NR_listen`

```rust
const __NR_listen: u32 = 50u32;
```

### `__NR_getsockname`

```rust
const __NR_getsockname: u32 = 51u32;
```

### `__NR_getpeername`

```rust
const __NR_getpeername: u32 = 52u32;
```

### `__NR_socketpair`

```rust
const __NR_socketpair: u32 = 53u32;
```

### `__NR_setsockopt`

```rust
const __NR_setsockopt: u32 = 54u32;
```

### `__NR_getsockopt`

```rust
const __NR_getsockopt: u32 = 55u32;
```

### `__NR_clone`

```rust
const __NR_clone: u32 = 56u32;
```

### `__NR_fork`

```rust
const __NR_fork: u32 = 57u32;
```

### `__NR_vfork`

```rust
const __NR_vfork: u32 = 58u32;
```

### `__NR_execve`

```rust
const __NR_execve: u32 = 59u32;
```

### `__NR_exit`

```rust
const __NR_exit: u32 = 60u32;
```

### `__NR_wait4`

```rust
const __NR_wait4: u32 = 61u32;
```

### `__NR_kill`

```rust
const __NR_kill: u32 = 62u32;
```

### `__NR_uname`

```rust
const __NR_uname: u32 = 63u32;
```

### `__NR_semget`

```rust
const __NR_semget: u32 = 64u32;
```

### `__NR_semop`

```rust
const __NR_semop: u32 = 65u32;
```

### `__NR_semctl`

```rust
const __NR_semctl: u32 = 66u32;
```

### `__NR_shmdt`

```rust
const __NR_shmdt: u32 = 67u32;
```

### `__NR_msgget`

```rust
const __NR_msgget: u32 = 68u32;
```

### `__NR_msgsnd`

```rust
const __NR_msgsnd: u32 = 69u32;
```

### `__NR_msgrcv`

```rust
const __NR_msgrcv: u32 = 70u32;
```

### `__NR_msgctl`

```rust
const __NR_msgctl: u32 = 71u32;
```

### `__NR_fcntl`

```rust
const __NR_fcntl: u32 = 72u32;
```

### `__NR_flock`

```rust
const __NR_flock: u32 = 73u32;
```

### `__NR_fsync`

```rust
const __NR_fsync: u32 = 74u32;
```

### `__NR_fdatasync`

```rust
const __NR_fdatasync: u32 = 75u32;
```

### `__NR_truncate`

```rust
const __NR_truncate: u32 = 76u32;
```

### `__NR_ftruncate`

```rust
const __NR_ftruncate: u32 = 77u32;
```

### `__NR_getdents`

```rust
const __NR_getdents: u32 = 78u32;
```

### `__NR_getcwd`

```rust
const __NR_getcwd: u32 = 79u32;
```

### `__NR_chdir`

```rust
const __NR_chdir: u32 = 80u32;
```

### `__NR_fchdir`

```rust
const __NR_fchdir: u32 = 81u32;
```

### `__NR_rename`

```rust
const __NR_rename: u32 = 82u32;
```

### `__NR_mkdir`

```rust
const __NR_mkdir: u32 = 83u32;
```

### `__NR_rmdir`

```rust
const __NR_rmdir: u32 = 84u32;
```

### `__NR_creat`

```rust
const __NR_creat: u32 = 85u32;
```

### `__NR_link`

```rust
const __NR_link: u32 = 86u32;
```

### `__NR_unlink`

```rust
const __NR_unlink: u32 = 87u32;
```

### `__NR_symlink`

```rust
const __NR_symlink: u32 = 88u32;
```

### `__NR_readlink`

```rust
const __NR_readlink: u32 = 89u32;
```

### `__NR_chmod`

```rust
const __NR_chmod: u32 = 90u32;
```

### `__NR_fchmod`

```rust
const __NR_fchmod: u32 = 91u32;
```

### `__NR_chown`

```rust
const __NR_chown: u32 = 92u32;
```

### `__NR_fchown`

```rust
const __NR_fchown: u32 = 93u32;
```

### `__NR_lchown`

```rust
const __NR_lchown: u32 = 94u32;
```

### `__NR_umask`

```rust
const __NR_umask: u32 = 95u32;
```

### `__NR_gettimeofday`

```rust
const __NR_gettimeofday: u32 = 96u32;
```

### `__NR_getrlimit`

```rust
const __NR_getrlimit: u32 = 97u32;
```

### `__NR_getrusage`

```rust
const __NR_getrusage: u32 = 98u32;
```

### `__NR_sysinfo`

```rust
const __NR_sysinfo: u32 = 99u32;
```

### `__NR_times`

```rust
const __NR_times: u32 = 100u32;
```

### `__NR_ptrace`

```rust
const __NR_ptrace: u32 = 101u32;
```

### `__NR_getuid`

```rust
const __NR_getuid: u32 = 102u32;
```

### `__NR_syslog`

```rust
const __NR_syslog: u32 = 103u32;
```

### `__NR_getgid`

```rust
const __NR_getgid: u32 = 104u32;
```

### `__NR_setuid`

```rust
const __NR_setuid: u32 = 105u32;
```

### `__NR_setgid`

```rust
const __NR_setgid: u32 = 106u32;
```

### `__NR_geteuid`

```rust
const __NR_geteuid: u32 = 107u32;
```

### `__NR_getegid`

```rust
const __NR_getegid: u32 = 108u32;
```

### `__NR_setpgid`

```rust
const __NR_setpgid: u32 = 109u32;
```

### `__NR_getppid`

```rust
const __NR_getppid: u32 = 110u32;
```

### `__NR_getpgrp`

```rust
const __NR_getpgrp: u32 = 111u32;
```

### `__NR_setsid`

```rust
const __NR_setsid: u32 = 112u32;
```

### `__NR_setreuid`

```rust
const __NR_setreuid: u32 = 113u32;
```

### `__NR_setregid`

```rust
const __NR_setregid: u32 = 114u32;
```

### `__NR_getgroups`

```rust
const __NR_getgroups: u32 = 115u32;
```

### `__NR_setgroups`

```rust
const __NR_setgroups: u32 = 116u32;
```

### `__NR_setresuid`

```rust
const __NR_setresuid: u32 = 117u32;
```

### `__NR_getresuid`

```rust
const __NR_getresuid: u32 = 118u32;
```

### `__NR_setresgid`

```rust
const __NR_setresgid: u32 = 119u32;
```

### `__NR_getresgid`

```rust
const __NR_getresgid: u32 = 120u32;
```

### `__NR_getpgid`

```rust
const __NR_getpgid: u32 = 121u32;
```

### `__NR_setfsuid`

```rust
const __NR_setfsuid: u32 = 122u32;
```

### `__NR_setfsgid`

```rust
const __NR_setfsgid: u32 = 123u32;
```

### `__NR_getsid`

```rust
const __NR_getsid: u32 = 124u32;
```

### `__NR_capget`

```rust
const __NR_capget: u32 = 125u32;
```

### `__NR_capset`

```rust
const __NR_capset: u32 = 126u32;
```

### `__NR_rt_sigpending`

```rust
const __NR_rt_sigpending: u32 = 127u32;
```

### `__NR_rt_sigtimedwait`

```rust
const __NR_rt_sigtimedwait: u32 = 128u32;
```

### `__NR_rt_sigqueueinfo`

```rust
const __NR_rt_sigqueueinfo: u32 = 129u32;
```

### `__NR_rt_sigsuspend`

```rust
const __NR_rt_sigsuspend: u32 = 130u32;
```

### `__NR_sigaltstack`

```rust
const __NR_sigaltstack: u32 = 131u32;
```

### `__NR_utime`

```rust
const __NR_utime: u32 = 132u32;
```

### `__NR_mknod`

```rust
const __NR_mknod: u32 = 133u32;
```

### `__NR_uselib`

```rust
const __NR_uselib: u32 = 134u32;
```

### `__NR_personality`

```rust
const __NR_personality: u32 = 135u32;
```

### `__NR_ustat`

```rust
const __NR_ustat: u32 = 136u32;
```

### `__NR_statfs`

```rust
const __NR_statfs: u32 = 137u32;
```

### `__NR_fstatfs`

```rust
const __NR_fstatfs: u32 = 138u32;
```

### `__NR_sysfs`

```rust
const __NR_sysfs: u32 = 139u32;
```

### `__NR_getpriority`

```rust
const __NR_getpriority: u32 = 140u32;
```

### `__NR_setpriority`

```rust
const __NR_setpriority: u32 = 141u32;
```

### `__NR_sched_setparam`

```rust
const __NR_sched_setparam: u32 = 142u32;
```

### `__NR_sched_getparam`

```rust
const __NR_sched_getparam: u32 = 143u32;
```

### `__NR_sched_setscheduler`

```rust
const __NR_sched_setscheduler: u32 = 144u32;
```

### `__NR_sched_getscheduler`

```rust
const __NR_sched_getscheduler: u32 = 145u32;
```

### `__NR_sched_get_priority_max`

```rust
const __NR_sched_get_priority_max: u32 = 146u32;
```

### `__NR_sched_get_priority_min`

```rust
const __NR_sched_get_priority_min: u32 = 147u32;
```

### `__NR_sched_rr_get_interval`

```rust
const __NR_sched_rr_get_interval: u32 = 148u32;
```

### `__NR_mlock`

```rust
const __NR_mlock: u32 = 149u32;
```

### `__NR_munlock`

```rust
const __NR_munlock: u32 = 150u32;
```

### `__NR_mlockall`

```rust
const __NR_mlockall: u32 = 151u32;
```

### `__NR_munlockall`

```rust
const __NR_munlockall: u32 = 152u32;
```

### `__NR_vhangup`

```rust
const __NR_vhangup: u32 = 153u32;
```

### `__NR_modify_ldt`

```rust
const __NR_modify_ldt: u32 = 154u32;
```

### `__NR_pivot_root`

```rust
const __NR_pivot_root: u32 = 155u32;
```

### `__NR__sysctl`

```rust
const __NR__sysctl: u32 = 156u32;
```

### `__NR_prctl`

```rust
const __NR_prctl: u32 = 157u32;
```

### `__NR_arch_prctl`

```rust
const __NR_arch_prctl: u32 = 158u32;
```

### `__NR_adjtimex`

```rust
const __NR_adjtimex: u32 = 159u32;
```

### `__NR_setrlimit`

```rust
const __NR_setrlimit: u32 = 160u32;
```

### `__NR_chroot`

```rust
const __NR_chroot: u32 = 161u32;
```

### `__NR_sync`

```rust
const __NR_sync: u32 = 162u32;
```

### `__NR_acct`

```rust
const __NR_acct: u32 = 163u32;
```

### `__NR_settimeofday`

```rust
const __NR_settimeofday: u32 = 164u32;
```

### `__NR_mount`

```rust
const __NR_mount: u32 = 165u32;
```

### `__NR_umount2`

```rust
const __NR_umount2: u32 = 166u32;
```

### `__NR_swapon`

```rust
const __NR_swapon: u32 = 167u32;
```

### `__NR_swapoff`

```rust
const __NR_swapoff: u32 = 168u32;
```

### `__NR_reboot`

```rust
const __NR_reboot: u32 = 169u32;
```

### `__NR_sethostname`

```rust
const __NR_sethostname: u32 = 170u32;
```

### `__NR_setdomainname`

```rust
const __NR_setdomainname: u32 = 171u32;
```

### `__NR_iopl`

```rust
const __NR_iopl: u32 = 172u32;
```

### `__NR_ioperm`

```rust
const __NR_ioperm: u32 = 173u32;
```

### `__NR_create_module`

```rust
const __NR_create_module: u32 = 174u32;
```

### `__NR_init_module`

```rust
const __NR_init_module: u32 = 175u32;
```

### `__NR_delete_module`

```rust
const __NR_delete_module: u32 = 176u32;
```

### `__NR_get_kernel_syms`

```rust
const __NR_get_kernel_syms: u32 = 177u32;
```

### `__NR_query_module`

```rust
const __NR_query_module: u32 = 178u32;
```

### `__NR_quotactl`

```rust
const __NR_quotactl: u32 = 179u32;
```

### `__NR_nfsservctl`

```rust
const __NR_nfsservctl: u32 = 180u32;
```

### `__NR_getpmsg`

```rust
const __NR_getpmsg: u32 = 181u32;
```

### `__NR_putpmsg`

```rust
const __NR_putpmsg: u32 = 182u32;
```

### `__NR_afs_syscall`

```rust
const __NR_afs_syscall: u32 = 183u32;
```

### `__NR_tuxcall`

```rust
const __NR_tuxcall: u32 = 184u32;
```

### `__NR_security`

```rust
const __NR_security: u32 = 185u32;
```

### `__NR_gettid`

```rust
const __NR_gettid: u32 = 186u32;
```

### `__NR_readahead`

```rust
const __NR_readahead: u32 = 187u32;
```

### `__NR_setxattr`

```rust
const __NR_setxattr: u32 = 188u32;
```

### `__NR_lsetxattr`

```rust
const __NR_lsetxattr: u32 = 189u32;
```

### `__NR_fsetxattr`

```rust
const __NR_fsetxattr: u32 = 190u32;
```

### `__NR_getxattr`

```rust
const __NR_getxattr: u32 = 191u32;
```

### `__NR_lgetxattr`

```rust
const __NR_lgetxattr: u32 = 192u32;
```

### `__NR_fgetxattr`

```rust
const __NR_fgetxattr: u32 = 193u32;
```

### `__NR_listxattr`

```rust
const __NR_listxattr: u32 = 194u32;
```

### `__NR_llistxattr`

```rust
const __NR_llistxattr: u32 = 195u32;
```

### `__NR_flistxattr`

```rust
const __NR_flistxattr: u32 = 196u32;
```

### `__NR_removexattr`

```rust
const __NR_removexattr: u32 = 197u32;
```

### `__NR_lremovexattr`

```rust
const __NR_lremovexattr: u32 = 198u32;
```

### `__NR_fremovexattr`

```rust
const __NR_fremovexattr: u32 = 199u32;
```

### `__NR_tkill`

```rust
const __NR_tkill: u32 = 200u32;
```

### `__NR_time`

```rust
const __NR_time: u32 = 201u32;
```

### `__NR_futex`

```rust
const __NR_futex: u32 = 202u32;
```

### `__NR_sched_setaffinity`

```rust
const __NR_sched_setaffinity: u32 = 203u32;
```

### `__NR_sched_getaffinity`

```rust
const __NR_sched_getaffinity: u32 = 204u32;
```

### `__NR_set_thread_area`

```rust
const __NR_set_thread_area: u32 = 205u32;
```

### `__NR_io_setup`

```rust
const __NR_io_setup: u32 = 206u32;
```

### `__NR_io_destroy`

```rust
const __NR_io_destroy: u32 = 207u32;
```

### `__NR_io_getevents`

```rust
const __NR_io_getevents: u32 = 208u32;
```

### `__NR_io_submit`

```rust
const __NR_io_submit: u32 = 209u32;
```

### `__NR_io_cancel`

```rust
const __NR_io_cancel: u32 = 210u32;
```

### `__NR_get_thread_area`

```rust
const __NR_get_thread_area: u32 = 211u32;
```

### `__NR_lookup_dcookie`

```rust
const __NR_lookup_dcookie: u32 = 212u32;
```

### `__NR_epoll_create`

```rust
const __NR_epoll_create: u32 = 213u32;
```

### `__NR_epoll_ctl_old`

```rust
const __NR_epoll_ctl_old: u32 = 214u32;
```

### `__NR_epoll_wait_old`

```rust
const __NR_epoll_wait_old: u32 = 215u32;
```

### `__NR_remap_file_pages`

```rust
const __NR_remap_file_pages: u32 = 216u32;
```

### `__NR_getdents64`

```rust
const __NR_getdents64: u32 = 217u32;
```

### `__NR_set_tid_address`

```rust
const __NR_set_tid_address: u32 = 218u32;
```

### `__NR_restart_syscall`

```rust
const __NR_restart_syscall: u32 = 219u32;
```

### `__NR_semtimedop`

```rust
const __NR_semtimedop: u32 = 220u32;
```

### `__NR_fadvise64`

```rust
const __NR_fadvise64: u32 = 221u32;
```

### `__NR_timer_create`

```rust
const __NR_timer_create: u32 = 222u32;
```

### `__NR_timer_settime`

```rust
const __NR_timer_settime: u32 = 223u32;
```

### `__NR_timer_gettime`

```rust
const __NR_timer_gettime: u32 = 224u32;
```

### `__NR_timer_getoverrun`

```rust
const __NR_timer_getoverrun: u32 = 225u32;
```

### `__NR_timer_delete`

```rust
const __NR_timer_delete: u32 = 226u32;
```

### `__NR_clock_settime`

```rust
const __NR_clock_settime: u32 = 227u32;
```

### `__NR_clock_gettime`

```rust
const __NR_clock_gettime: u32 = 228u32;
```

### `__NR_clock_getres`

```rust
const __NR_clock_getres: u32 = 229u32;
```

### `__NR_clock_nanosleep`

```rust
const __NR_clock_nanosleep: u32 = 230u32;
```

### `__NR_exit_group`

```rust
const __NR_exit_group: u32 = 231u32;
```

### `__NR_epoll_wait`

```rust
const __NR_epoll_wait: u32 = 232u32;
```

### `__NR_epoll_ctl`

```rust
const __NR_epoll_ctl: u32 = 233u32;
```

### `__NR_tgkill`

```rust
const __NR_tgkill: u32 = 234u32;
```

### `__NR_utimes`

```rust
const __NR_utimes: u32 = 235u32;
```

### `__NR_vserver`

```rust
const __NR_vserver: u32 = 236u32;
```

### `__NR_mbind`

```rust
const __NR_mbind: u32 = 237u32;
```

### `__NR_set_mempolicy`

```rust
const __NR_set_mempolicy: u32 = 238u32;
```

### `__NR_get_mempolicy`

```rust
const __NR_get_mempolicy: u32 = 239u32;
```

### `__NR_mq_open`

```rust
const __NR_mq_open: u32 = 240u32;
```

### `__NR_mq_unlink`

```rust
const __NR_mq_unlink: u32 = 241u32;
```

### `__NR_mq_timedsend`

```rust
const __NR_mq_timedsend: u32 = 242u32;
```

### `__NR_mq_timedreceive`

```rust
const __NR_mq_timedreceive: u32 = 243u32;
```

### `__NR_mq_notify`

```rust
const __NR_mq_notify: u32 = 244u32;
```

### `__NR_mq_getsetattr`

```rust
const __NR_mq_getsetattr: u32 = 245u32;
```

### `__NR_kexec_load`

```rust
const __NR_kexec_load: u32 = 246u32;
```

### `__NR_waitid`

```rust
const __NR_waitid: u32 = 247u32;
```

### `__NR_add_key`

```rust
const __NR_add_key: u32 = 248u32;
```

### `__NR_request_key`

```rust
const __NR_request_key: u32 = 249u32;
```

### `__NR_keyctl`

```rust
const __NR_keyctl: u32 = 250u32;
```

### `__NR_ioprio_set`

```rust
const __NR_ioprio_set: u32 = 251u32;
```

### `__NR_ioprio_get`

```rust
const __NR_ioprio_get: u32 = 252u32;
```

### `__NR_inotify_init`

```rust
const __NR_inotify_init: u32 = 253u32;
```

### `__NR_inotify_add_watch`

```rust
const __NR_inotify_add_watch: u32 = 254u32;
```

### `__NR_inotify_rm_watch`

```rust
const __NR_inotify_rm_watch: u32 = 255u32;
```

### `__NR_migrate_pages`

```rust
const __NR_migrate_pages: u32 = 256u32;
```

### `__NR_openat`

```rust
const __NR_openat: u32 = 257u32;
```

### `__NR_mkdirat`

```rust
const __NR_mkdirat: u32 = 258u32;
```

### `__NR_mknodat`

```rust
const __NR_mknodat: u32 = 259u32;
```

### `__NR_fchownat`

```rust
const __NR_fchownat: u32 = 260u32;
```

### `__NR_futimesat`

```rust
const __NR_futimesat: u32 = 261u32;
```

### `__NR_newfstatat`

```rust
const __NR_newfstatat: u32 = 262u32;
```

### `__NR_unlinkat`

```rust
const __NR_unlinkat: u32 = 263u32;
```

### `__NR_renameat`

```rust
const __NR_renameat: u32 = 264u32;
```

### `__NR_linkat`

```rust
const __NR_linkat: u32 = 265u32;
```

### `__NR_symlinkat`

```rust
const __NR_symlinkat: u32 = 266u32;
```

### `__NR_readlinkat`

```rust
const __NR_readlinkat: u32 = 267u32;
```

### `__NR_fchmodat`

```rust
const __NR_fchmodat: u32 = 268u32;
```

### `__NR_faccessat`

```rust
const __NR_faccessat: u32 = 269u32;
```

### `__NR_pselect6`

```rust
const __NR_pselect6: u32 = 270u32;
```

### `__NR_ppoll`

```rust
const __NR_ppoll: u32 = 271u32;
```

### `__NR_unshare`

```rust
const __NR_unshare: u32 = 272u32;
```

### `__NR_set_robust_list`

```rust
const __NR_set_robust_list: u32 = 273u32;
```

### `__NR_get_robust_list`

```rust
const __NR_get_robust_list: u32 = 274u32;
```

### `__NR_splice`

```rust
const __NR_splice: u32 = 275u32;
```

### `__NR_tee`

```rust
const __NR_tee: u32 = 276u32;
```

### `__NR_sync_file_range`

```rust
const __NR_sync_file_range: u32 = 277u32;
```

### `__NR_vmsplice`

```rust
const __NR_vmsplice: u32 = 278u32;
```

### `__NR_move_pages`

```rust
const __NR_move_pages: u32 = 279u32;
```

### `__NR_utimensat`

```rust
const __NR_utimensat: u32 = 280u32;
```

### `__NR_epoll_pwait`

```rust
const __NR_epoll_pwait: u32 = 281u32;
```

### `__NR_signalfd`

```rust
const __NR_signalfd: u32 = 282u32;
```

### `__NR_timerfd_create`

```rust
const __NR_timerfd_create: u32 = 283u32;
```

### `__NR_eventfd`

```rust
const __NR_eventfd: u32 = 284u32;
```

### `__NR_fallocate`

```rust
const __NR_fallocate: u32 = 285u32;
```

### `__NR_timerfd_settime`

```rust
const __NR_timerfd_settime: u32 = 286u32;
```

### `__NR_timerfd_gettime`

```rust
const __NR_timerfd_gettime: u32 = 287u32;
```

### `__NR_accept4`

```rust
const __NR_accept4: u32 = 288u32;
```

### `__NR_signalfd4`

```rust
const __NR_signalfd4: u32 = 289u32;
```

### `__NR_eventfd2`

```rust
const __NR_eventfd2: u32 = 290u32;
```

### `__NR_epoll_create1`

```rust
const __NR_epoll_create1: u32 = 291u32;
```

### `__NR_dup3`

```rust
const __NR_dup3: u32 = 292u32;
```

### `__NR_pipe2`

```rust
const __NR_pipe2: u32 = 293u32;
```

### `__NR_inotify_init1`

```rust
const __NR_inotify_init1: u32 = 294u32;
```

### `__NR_preadv`

```rust
const __NR_preadv: u32 = 295u32;
```

### `__NR_pwritev`

```rust
const __NR_pwritev: u32 = 296u32;
```

### `__NR_rt_tgsigqueueinfo`

```rust
const __NR_rt_tgsigqueueinfo: u32 = 297u32;
```

### `__NR_perf_event_open`

```rust
const __NR_perf_event_open: u32 = 298u32;
```

### `__NR_recvmmsg`

```rust
const __NR_recvmmsg: u32 = 299u32;
```

### `__NR_fanotify_init`

```rust
const __NR_fanotify_init: u32 = 300u32;
```

### `__NR_fanotify_mark`

```rust
const __NR_fanotify_mark: u32 = 301u32;
```

### `__NR_prlimit64`

```rust
const __NR_prlimit64: u32 = 302u32;
```

### `__NR_name_to_handle_at`

```rust
const __NR_name_to_handle_at: u32 = 303u32;
```

### `__NR_open_by_handle_at`

```rust
const __NR_open_by_handle_at: u32 = 304u32;
```

### `__NR_clock_adjtime`

```rust
const __NR_clock_adjtime: u32 = 305u32;
```

### `__NR_syncfs`

```rust
const __NR_syncfs: u32 = 306u32;
```

### `__NR_sendmmsg`

```rust
const __NR_sendmmsg: u32 = 307u32;
```

### `__NR_setns`

```rust
const __NR_setns: u32 = 308u32;
```

### `__NR_getcpu`

```rust
const __NR_getcpu: u32 = 309u32;
```

### `__NR_process_vm_readv`

```rust
const __NR_process_vm_readv: u32 = 310u32;
```

### `__NR_process_vm_writev`

```rust
const __NR_process_vm_writev: u32 = 311u32;
```

### `__NR_kcmp`

```rust
const __NR_kcmp: u32 = 312u32;
```

### `__NR_finit_module`

```rust
const __NR_finit_module: u32 = 313u32;
```

### `__NR_sched_setattr`

```rust
const __NR_sched_setattr: u32 = 314u32;
```

### `__NR_sched_getattr`

```rust
const __NR_sched_getattr: u32 = 315u32;
```

### `__NR_renameat2`

```rust
const __NR_renameat2: u32 = 316u32;
```

### `__NR_seccomp`

```rust
const __NR_seccomp: u32 = 317u32;
```

### `__NR_getrandom`

```rust
const __NR_getrandom: u32 = 318u32;
```

### `__NR_memfd_create`

```rust
const __NR_memfd_create: u32 = 319u32;
```

### `__NR_kexec_file_load`

```rust
const __NR_kexec_file_load: u32 = 320u32;
```

### `__NR_bpf`

```rust
const __NR_bpf: u32 = 321u32;
```

### `__NR_execveat`

```rust
const __NR_execveat: u32 = 322u32;
```

### `__NR_userfaultfd`

```rust
const __NR_userfaultfd: u32 = 323u32;
```

### `__NR_membarrier`

```rust
const __NR_membarrier: u32 = 324u32;
```

### `__NR_mlock2`

```rust
const __NR_mlock2: u32 = 325u32;
```

### `__NR_copy_file_range`

```rust
const __NR_copy_file_range: u32 = 326u32;
```

### `__NR_preadv2`

```rust
const __NR_preadv2: u32 = 327u32;
```

### `__NR_pwritev2`

```rust
const __NR_pwritev2: u32 = 328u32;
```

### `__NR_pkey_mprotect`

```rust
const __NR_pkey_mprotect: u32 = 329u32;
```

### `__NR_pkey_alloc`

```rust
const __NR_pkey_alloc: u32 = 330u32;
```

### `__NR_pkey_free`

```rust
const __NR_pkey_free: u32 = 331u32;
```

### `__NR_statx`

```rust
const __NR_statx: u32 = 332u32;
```

### `__NR_io_pgetevents`

```rust
const __NR_io_pgetevents: u32 = 333u32;
```

### `__NR_rseq`

```rust
const __NR_rseq: u32 = 334u32;
```

### `__NR_uretprobe`

```rust
const __NR_uretprobe: u32 = 335u32;
```

### `__NR_pidfd_send_signal`

```rust
const __NR_pidfd_send_signal: u32 = 424u32;
```

### `__NR_io_uring_setup`

```rust
const __NR_io_uring_setup: u32 = 425u32;
```

### `__NR_io_uring_enter`

```rust
const __NR_io_uring_enter: u32 = 426u32;
```

### `__NR_io_uring_register`

```rust
const __NR_io_uring_register: u32 = 427u32;
```

### `__NR_open_tree`

```rust
const __NR_open_tree: u32 = 428u32;
```

### `__NR_move_mount`

```rust
const __NR_move_mount: u32 = 429u32;
```

### `__NR_fsopen`

```rust
const __NR_fsopen: u32 = 430u32;
```

### `__NR_fsconfig`

```rust
const __NR_fsconfig: u32 = 431u32;
```

### `__NR_fsmount`

```rust
const __NR_fsmount: u32 = 432u32;
```

### `__NR_fspick`

```rust
const __NR_fspick: u32 = 433u32;
```

### `__NR_pidfd_open`

```rust
const __NR_pidfd_open: u32 = 434u32;
```

### `__NR_clone3`

```rust
const __NR_clone3: u32 = 435u32;
```

### `__NR_close_range`

```rust
const __NR_close_range: u32 = 436u32;
```

### `__NR_openat2`

```rust
const __NR_openat2: u32 = 437u32;
```

### `__NR_pidfd_getfd`

```rust
const __NR_pidfd_getfd: u32 = 438u32;
```

### `__NR_faccessat2`

```rust
const __NR_faccessat2: u32 = 439u32;
```

### `__NR_process_madvise`

```rust
const __NR_process_madvise: u32 = 440u32;
```

### `__NR_epoll_pwait2`

```rust
const __NR_epoll_pwait2: u32 = 441u32;
```

### `__NR_mount_setattr`

```rust
const __NR_mount_setattr: u32 = 442u32;
```

### `__NR_quotactl_fd`

```rust
const __NR_quotactl_fd: u32 = 443u32;
```

### `__NR_landlock_create_ruleset`

```rust
const __NR_landlock_create_ruleset: u32 = 444u32;
```

### `__NR_landlock_add_rule`

```rust
const __NR_landlock_add_rule: u32 = 445u32;
```

### `__NR_landlock_restrict_self`

```rust
const __NR_landlock_restrict_self: u32 = 446u32;
```

### `__NR_memfd_secret`

```rust
const __NR_memfd_secret: u32 = 447u32;
```

### `__NR_process_mrelease`

```rust
const __NR_process_mrelease: u32 = 448u32;
```

### `__NR_futex_waitv`

```rust
const __NR_futex_waitv: u32 = 449u32;
```

### `__NR_set_mempolicy_home_node`

```rust
const __NR_set_mempolicy_home_node: u32 = 450u32;
```

### `__NR_cachestat`

```rust
const __NR_cachestat: u32 = 451u32;
```

### `__NR_fchmodat2`

```rust
const __NR_fchmodat2: u32 = 452u32;
```

### `__NR_map_shadow_stack`

```rust
const __NR_map_shadow_stack: u32 = 453u32;
```

### `__NR_futex_wake`

```rust
const __NR_futex_wake: u32 = 454u32;
```

### `__NR_futex_wait`

```rust
const __NR_futex_wait: u32 = 455u32;
```

### `__NR_futex_requeue`

```rust
const __NR_futex_requeue: u32 = 456u32;
```

### `__NR_statmount`

```rust
const __NR_statmount: u32 = 457u32;
```

### `__NR_listmount`

```rust
const __NR_listmount: u32 = 458u32;
```

### `__NR_lsm_get_self_attr`

```rust
const __NR_lsm_get_self_attr: u32 = 459u32;
```

### `__NR_lsm_set_self_attr`

```rust
const __NR_lsm_set_self_attr: u32 = 460u32;
```

### `__NR_lsm_list_modules`

```rust
const __NR_lsm_list_modules: u32 = 461u32;
```

### `__NR_mseal`

```rust
const __NR_mseal: u32 = 462u32;
```

### `__NR_setxattrat`

```rust
const __NR_setxattrat: u32 = 463u32;
```

### `__NR_getxattrat`

```rust
const __NR_getxattrat: u32 = 464u32;
```

### `__NR_listxattrat`

```rust
const __NR_listxattrat: u32 = 465u32;
```

### `__NR_removexattrat`

```rust
const __NR_removexattrat: u32 = 466u32;
```

### `__NR_open_tree_attr`

```rust
const __NR_open_tree_attr: u32 = 467u32;
```

### `WNOHANG`

```rust
const WNOHANG: u32 = 1u32;
```

### `WUNTRACED`

```rust
const WUNTRACED: u32 = 2u32;
```

### `WSTOPPED`

```rust
const WSTOPPED: u32 = 2u32;
```

### `WEXITED`

```rust
const WEXITED: u32 = 4u32;
```

### `WCONTINUED`

```rust
const WCONTINUED: u32 = 8u32;
```

### `WNOWAIT`

```rust
const WNOWAIT: u32 = 16_777_216u32;
```

### `__WNOTHREAD`

```rust
const __WNOTHREAD: u32 = 536_870_912u32;
```

### `__WALL`

```rust
const __WALL: u32 = 1_073_741_824u32;
```

### `__WCLONE`

```rust
const __WCLONE: u32 = 2_147_483_648u32;
```

### `P_ALL`

```rust
const P_ALL: u32 = 0u32;
```

### `P_PID`

```rust
const P_PID: u32 = 1u32;
```

### `P_PGID`

```rust
const P_PGID: u32 = 2u32;
```

### `P_PIDFD`

```rust
const P_PIDFD: u32 = 3u32;
```

### `XATTR_CREATE`

```rust
const XATTR_CREATE: u32 = 1u32;
```

### `XATTR_REPLACE`

```rust
const XATTR_REPLACE: u32 = 2u32;
```

### `XATTR_OS2_PREFIX`

```rust
const XATTR_OS2_PREFIX: &[u8; 5];
```

### `XATTR_MAC_OSX_PREFIX`

```rust
const XATTR_MAC_OSX_PREFIX: &[u8; 5];
```

### `XATTR_BTRFS_PREFIX`

```rust
const XATTR_BTRFS_PREFIX: &[u8; 7];
```

### `XATTR_HURD_PREFIX`

```rust
const XATTR_HURD_PREFIX: &[u8; 5];
```

### `XATTR_SECURITY_PREFIX`

```rust
const XATTR_SECURITY_PREFIX: &[u8; 10];
```

### `XATTR_SYSTEM_PREFIX`

```rust
const XATTR_SYSTEM_PREFIX: &[u8; 8];
```

### `XATTR_TRUSTED_PREFIX`

```rust
const XATTR_TRUSTED_PREFIX: &[u8; 9];
```

### `XATTR_USER_PREFIX`

```rust
const XATTR_USER_PREFIX: &[u8; 6];
```

### `XATTR_EVM_SUFFIX`

```rust
const XATTR_EVM_SUFFIX: &[u8; 4];
```

### `XATTR_NAME_EVM`

```rust
const XATTR_NAME_EVM: &[u8; 13];
```

### `XATTR_IMA_SUFFIX`

```rust
const XATTR_IMA_SUFFIX: &[u8; 4];
```

### `XATTR_NAME_IMA`

```rust
const XATTR_NAME_IMA: &[u8; 13];
```

### `XATTR_SELINUX_SUFFIX`

```rust
const XATTR_SELINUX_SUFFIX: &[u8; 8];
```

### `XATTR_NAME_SELINUX`

```rust
const XATTR_NAME_SELINUX: &[u8; 17];
```

### `XATTR_SMACK_SUFFIX`

```rust
const XATTR_SMACK_SUFFIX: &[u8; 8];
```

### `XATTR_SMACK_IPIN`

```rust
const XATTR_SMACK_IPIN: &[u8; 12];
```

### `XATTR_SMACK_IPOUT`

```rust
const XATTR_SMACK_IPOUT: &[u8; 13];
```

### `XATTR_SMACK_EXEC`

```rust
const XATTR_SMACK_EXEC: &[u8; 12];
```

### `XATTR_SMACK_TRANSMUTE`

```rust
const XATTR_SMACK_TRANSMUTE: &[u8; 17];
```

### `XATTR_SMACK_MMAP`

```rust
const XATTR_SMACK_MMAP: &[u8; 12];
```

### `XATTR_NAME_SMACK`

```rust
const XATTR_NAME_SMACK: &[u8; 17];
```

### `XATTR_NAME_SMACKIPIN`

```rust
const XATTR_NAME_SMACKIPIN: &[u8; 21];
```

### `XATTR_NAME_SMACKIPOUT`

```rust
const XATTR_NAME_SMACKIPOUT: &[u8; 22];
```

### `XATTR_NAME_SMACKEXEC`

```rust
const XATTR_NAME_SMACKEXEC: &[u8; 21];
```

### `XATTR_NAME_SMACKTRANSMUTE`

```rust
const XATTR_NAME_SMACKTRANSMUTE: &[u8; 26];
```

### `XATTR_NAME_SMACKMMAP`

```rust
const XATTR_NAME_SMACKMMAP: &[u8; 21];
```

### `XATTR_APPARMOR_SUFFIX`

```rust
const XATTR_APPARMOR_SUFFIX: &[u8; 9];
```

### `XATTR_NAME_APPARMOR`

```rust
const XATTR_NAME_APPARMOR: &[u8; 18];
```

### `XATTR_CAPS_SUFFIX`

```rust
const XATTR_CAPS_SUFFIX: &[u8; 11];
```

### `XATTR_NAME_CAPS`

```rust
const XATTR_NAME_CAPS: &[u8; 20];
```

### `XATTR_BPF_LSM_SUFFIX`

```rust
const XATTR_BPF_LSM_SUFFIX: &[u8; 5];
```

### `XATTR_NAME_BPF_LSM`

```rust
const XATTR_NAME_BPF_LSM: &[u8; 14];
```

### `XATTR_POSIX_ACL_ACCESS`

```rust
const XATTR_POSIX_ACL_ACCESS: &[u8; 17];
```

### `XATTR_NAME_POSIX_ACL_ACCESS`

```rust
const XATTR_NAME_POSIX_ACL_ACCESS: &[u8; 24];
```

### `XATTR_POSIX_ACL_DEFAULT`

```rust
const XATTR_POSIX_ACL_DEFAULT: &[u8; 18];
```

### `XATTR_NAME_POSIX_ACL_DEFAULT`

```rust
const XATTR_NAME_POSIX_ACL_DEFAULT: &[u8; 25];
```

### `MFD_CLOEXEC`

```rust
const MFD_CLOEXEC: u32 = 1u32;
```

### `MFD_ALLOW_SEALING`

```rust
const MFD_ALLOW_SEALING: u32 = 2u32;
```

### `MFD_HUGETLB`

```rust
const MFD_HUGETLB: u32 = 4u32;
```

### `MFD_NOEXEC_SEAL`

```rust
const MFD_NOEXEC_SEAL: u32 = 8u32;
```

### `MFD_EXEC`

```rust
const MFD_EXEC: u32 = 16u32;
```

### `MFD_HUGE_SHIFT`

```rust
const MFD_HUGE_SHIFT: u32 = 26u32;
```

### `MFD_HUGE_MASK`

```rust
const MFD_HUGE_MASK: u32 = 63u32;
```

### `MFD_HUGE_64KB`

```rust
const MFD_HUGE_64KB: u32 = 1_073_741_824u32;
```

### `MFD_HUGE_512KB`

```rust
const MFD_HUGE_512KB: u32 = 1_275_068_416u32;
```

### `MFD_HUGE_1MB`

```rust
const MFD_HUGE_1MB: u32 = 1_342_177_280u32;
```

### `MFD_HUGE_2MB`

```rust
const MFD_HUGE_2MB: u32 = 1_409_286_144u32;
```

### `MFD_HUGE_8MB`

```rust
const MFD_HUGE_8MB: u32 = 1_543_503_872u32;
```

### `MFD_HUGE_16MB`

```rust
const MFD_HUGE_16MB: u32 = 1_610_612_736u32;
```

### `MFD_HUGE_32MB`

```rust
const MFD_HUGE_32MB: u32 = 1_677_721_600u32;
```

### `MFD_HUGE_256MB`

```rust
const MFD_HUGE_256MB: u32 = 1_879_048_192u32;
```

### `MFD_HUGE_512MB`

```rust
const MFD_HUGE_512MB: u32 = 1_946_157_056u32;
```

### `MFD_HUGE_1GB`

```rust
const MFD_HUGE_1GB: u32 = 2_013_265_920u32;
```

### `MFD_HUGE_2GB`

```rust
const MFD_HUGE_2GB: u32 = 2_080_374_784u32;
```

### `MFD_HUGE_16GB`

```rust
const MFD_HUGE_16GB: u32 = 2_281_701_376u32;
```

### `TFD_TIMER_ABSTIME`

```rust
const TFD_TIMER_ABSTIME: u32 = 1u32;
```

### `TFD_TIMER_CANCEL_ON_SET`

```rust
const TFD_TIMER_CANCEL_ON_SET: u32 = 2u32;
```

### `TFD_CLOEXEC`

```rust
const TFD_CLOEXEC: u32 = 524_288u32;
```

### `TFD_NONBLOCK`

```rust
const TFD_NONBLOCK: u32 = 2_048u32;
```

### `USERFAULTFD_IOC`

```rust
const USERFAULTFD_IOC: u32 = 170u32;
```

### `_UFFDIO_REGISTER`

```rust
const _UFFDIO_REGISTER: u32 = 0u32;
```

### `_UFFDIO_UNREGISTER`

```rust
const _UFFDIO_UNREGISTER: u32 = 1u32;
```

### `_UFFDIO_WAKE`

```rust
const _UFFDIO_WAKE: u32 = 2u32;
```

### `_UFFDIO_COPY`

```rust
const _UFFDIO_COPY: u32 = 3u32;
```

### `_UFFDIO_ZEROPAGE`

```rust
const _UFFDIO_ZEROPAGE: u32 = 4u32;
```

### `_UFFDIO_MOVE`

```rust
const _UFFDIO_MOVE: u32 = 5u32;
```

### `_UFFDIO_WRITEPROTECT`

```rust
const _UFFDIO_WRITEPROTECT: u32 = 6u32;
```

### `_UFFDIO_CONTINUE`

```rust
const _UFFDIO_CONTINUE: u32 = 7u32;
```

### `_UFFDIO_POISON`

```rust
const _UFFDIO_POISON: u32 = 8u32;
```

### `_UFFDIO_API`

```rust
const _UFFDIO_API: u32 = 63u32;
```

### `UFFDIO`

```rust
const UFFDIO: u32 = 170u32;
```

### `UFFD_EVENT_PAGEFAULT`

```rust
const UFFD_EVENT_PAGEFAULT: u32 = 18u32;
```

### `UFFD_EVENT_FORK`

```rust
const UFFD_EVENT_FORK: u32 = 19u32;
```

### `UFFD_EVENT_REMAP`

```rust
const UFFD_EVENT_REMAP: u32 = 20u32;
```

### `UFFD_EVENT_REMOVE`

```rust
const UFFD_EVENT_REMOVE: u32 = 21u32;
```

### `UFFD_EVENT_UNMAP`

```rust
const UFFD_EVENT_UNMAP: u32 = 22u32;
```

### `UFFD_PAGEFAULT_FLAG_WRITE`

```rust
const UFFD_PAGEFAULT_FLAG_WRITE: u32 = 1u32;
```

### `UFFD_PAGEFAULT_FLAG_WP`

```rust
const UFFD_PAGEFAULT_FLAG_WP: u32 = 2u32;
```

### `UFFD_PAGEFAULT_FLAG_MINOR`

```rust
const UFFD_PAGEFAULT_FLAG_MINOR: u32 = 4u32;
```

### `UFFD_FEATURE_PAGEFAULT_FLAG_WP`

```rust
const UFFD_FEATURE_PAGEFAULT_FLAG_WP: u32 = 1u32;
```

### `UFFD_FEATURE_EVENT_FORK`

```rust
const UFFD_FEATURE_EVENT_FORK: u32 = 2u32;
```

### `UFFD_FEATURE_EVENT_REMAP`

```rust
const UFFD_FEATURE_EVENT_REMAP: u32 = 4u32;
```

### `UFFD_FEATURE_EVENT_REMOVE`

```rust
const UFFD_FEATURE_EVENT_REMOVE: u32 = 8u32;
```

### `UFFD_FEATURE_MISSING_HUGETLBFS`

```rust
const UFFD_FEATURE_MISSING_HUGETLBFS: u32 = 16u32;
```

### `UFFD_FEATURE_MISSING_SHMEM`

```rust
const UFFD_FEATURE_MISSING_SHMEM: u32 = 32u32;
```

### `UFFD_FEATURE_EVENT_UNMAP`

```rust
const UFFD_FEATURE_EVENT_UNMAP: u32 = 64u32;
```

### `UFFD_FEATURE_SIGBUS`

```rust
const UFFD_FEATURE_SIGBUS: u32 = 128u32;
```

### `UFFD_FEATURE_THREAD_ID`

```rust
const UFFD_FEATURE_THREAD_ID: u32 = 256u32;
```

### `UFFD_FEATURE_MINOR_HUGETLBFS`

```rust
const UFFD_FEATURE_MINOR_HUGETLBFS: u32 = 512u32;
```

### `UFFD_FEATURE_MINOR_SHMEM`

```rust
const UFFD_FEATURE_MINOR_SHMEM: u32 = 1_024u32;
```

### `UFFD_FEATURE_EXACT_ADDRESS`

```rust
const UFFD_FEATURE_EXACT_ADDRESS: u32 = 2_048u32;
```

### `UFFD_FEATURE_WP_HUGETLBFS_SHMEM`

```rust
const UFFD_FEATURE_WP_HUGETLBFS_SHMEM: u32 = 4_096u32;
```

### `UFFD_FEATURE_WP_UNPOPULATED`

```rust
const UFFD_FEATURE_WP_UNPOPULATED: u32 = 8_192u32;
```

### `UFFD_FEATURE_POISON`

```rust
const UFFD_FEATURE_POISON: u32 = 16_384u32;
```

### `UFFD_FEATURE_WP_ASYNC`

```rust
const UFFD_FEATURE_WP_ASYNC: u32 = 32_768u32;
```

### `UFFD_FEATURE_MOVE`

```rust
const UFFD_FEATURE_MOVE: u32 = 65_536u32;
```

### `UFFD_USER_MODE_ONLY`

```rust
const UFFD_USER_MODE_ONLY: u32 = 1u32;
```

### `DT_UNKNOWN`

```rust
const DT_UNKNOWN: u32 = 0u32;
```

### `DT_FIFO`

```rust
const DT_FIFO: u32 = 1u32;
```

### `DT_CHR`

```rust
const DT_CHR: u32 = 2u32;
```

### `DT_DIR`

```rust
const DT_DIR: u32 = 4u32;
```

### `DT_BLK`

```rust
const DT_BLK: u32 = 6u32;
```

### `DT_REG`

```rust
const DT_REG: u32 = 8u32;
```

### `DT_LNK`

```rust
const DT_LNK: u32 = 10u32;
```

### `DT_SOCK`

```rust
const DT_SOCK: u32 = 12u32;
```

### `STAT_HAVE_NSEC`

```rust
const STAT_HAVE_NSEC: u32 = 1u32;
```

### `F_OK`

```rust
const F_OK: u32 = 0u32;
```

### `R_OK`

```rust
const R_OK: u32 = 4u32;
```

### `W_OK`

```rust
const W_OK: u32 = 2u32;
```

### `X_OK`

```rust
const X_OK: u32 = 1u32;
```

### `UTIME_NOW`

```rust
const UTIME_NOW: u32 = 1_073_741_823u32;
```

### `UTIME_OMIT`

```rust
const UTIME_OMIT: u32 = 1_073_741_822u32;
```

### `MNT_FORCE`

```rust
const MNT_FORCE: u32 = 1u32;
```

### `MNT_DETACH`

```rust
const MNT_DETACH: u32 = 2u32;
```

### `MNT_EXPIRE`

```rust
const MNT_EXPIRE: u32 = 4u32;
```

### `UMOUNT_NOFOLLOW`

```rust
const UMOUNT_NOFOLLOW: u32 = 8u32;
```

### `UMOUNT_UNUSED`

```rust
const UMOUNT_UNUSED: u32 = 2_147_483_648u32;
```

### `STDIN_FILENO`

```rust
const STDIN_FILENO: u32 = 0u32;
```

### `STDOUT_FILENO`

```rust
const STDOUT_FILENO: u32 = 1u32;
```

### `STDERR_FILENO`

```rust
const STDERR_FILENO: u32 = 2u32;
```

### `RWF_HIPRI`

```rust
const RWF_HIPRI: u32 = 1u32;
```

### `RWF_DSYNC`

```rust
const RWF_DSYNC: u32 = 2u32;
```

### `RWF_SYNC`

```rust
const RWF_SYNC: u32 = 4u32;
```

### `RWF_NOWAIT`

```rust
const RWF_NOWAIT: u32 = 8u32;
```

### `RWF_APPEND`

```rust
const RWF_APPEND: u32 = 16u32;
```

### `EFD_SEMAPHORE`

```rust
const EFD_SEMAPHORE: u32 = 1u32;
```

### `EFD_CLOEXEC`

```rust
const EFD_CLOEXEC: u32 = 524_288u32;
```

### `EFD_NONBLOCK`

```rust
const EFD_NONBLOCK: u32 = 2_048u32;
```

### `EPOLLIN`

```rust
const EPOLLIN: u32 = 1u32;
```

### `EPOLLPRI`

```rust
const EPOLLPRI: u32 = 2u32;
```

### `EPOLLOUT`

```rust
const EPOLLOUT: u32 = 4u32;
```

### `EPOLLERR`

```rust
const EPOLLERR: u32 = 8u32;
```

### `EPOLLHUP`

```rust
const EPOLLHUP: u32 = 16u32;
```

### `EPOLLNVAL`

```rust
const EPOLLNVAL: u32 = 32u32;
```

### `EPOLLRDNORM`

```rust
const EPOLLRDNORM: u32 = 64u32;
```

### `EPOLLRDBAND`

```rust
const EPOLLRDBAND: u32 = 128u32;
```

### `EPOLLWRNORM`

```rust
const EPOLLWRNORM: u32 = 256u32;
```

### `EPOLLWRBAND`

```rust
const EPOLLWRBAND: u32 = 512u32;
```

### `EPOLLMSG`

```rust
const EPOLLMSG: u32 = 1_024u32;
```

### `EPOLLRDHUP`

```rust
const EPOLLRDHUP: u32 = 8_192u32;
```

### `EPOLLEXCLUSIVE`

```rust
const EPOLLEXCLUSIVE: u32 = 268_435_456u32;
```

### `EPOLLWAKEUP`

```rust
const EPOLLWAKEUP: u32 = 536_870_912u32;
```

### `EPOLLONESHOT`

```rust
const EPOLLONESHOT: u32 = 1_073_741_824u32;
```

### `EPOLLET`

```rust
const EPOLLET: u32 = 2_147_483_648u32;
```

### `TFD_SHARED_FCNTL_FLAGS`

```rust
const TFD_SHARED_FCNTL_FLAGS: u32 = 526_336u32;
```

### `TFD_CREATE_FLAGS`

```rust
const TFD_CREATE_FLAGS: u32 = 526_336u32;
```

### `TFD_SETTIME_FLAGS`

```rust
const TFD_SETTIME_FLAGS: u32 = 1u32;
```

### `ARCH_SET_FS`

```rust
const ARCH_SET_FS: u32 = 4_098u32;
```

### `UFFD_API`

```rust
const UFFD_API: u32 = 170u32;
```

### `UFFDIO_REGISTER_MODE_MISSING`

```rust
const UFFDIO_REGISTER_MODE_MISSING: u32 = 1u32;
```

### `UFFDIO_REGISTER_MODE_WP`

```rust
const UFFDIO_REGISTER_MODE_WP: u32 = 2u32;
```

### `UFFDIO_REGISTER_MODE_MINOR`

```rust
const UFFDIO_REGISTER_MODE_MINOR: u32 = 4u32;
```

### `UFFDIO_COPY_MODE_DONTWAKE`

```rust
const UFFDIO_COPY_MODE_DONTWAKE: u32 = 1u32;
```

### `UFFDIO_COPY_MODE_WP`

```rust
const UFFDIO_COPY_MODE_WP: u32 = 2u32;
```

### `UFFDIO_ZEROPAGE_MODE_DONTWAKE`

```rust
const UFFDIO_ZEROPAGE_MODE_DONTWAKE: u32 = 1u32;
```

### `SPLICE_F_MOVE`

```rust
const SPLICE_F_MOVE: u32 = 1u32;
```

### `SPLICE_F_NONBLOCK`

```rust
const SPLICE_F_NONBLOCK: u32 = 2u32;
```

### `SPLICE_F_MORE`

```rust
const SPLICE_F_MORE: u32 = 4u32;
```

### `SPLICE_F_GIFT`

```rust
const SPLICE_F_GIFT: u32 = 8u32;
```

### `_NSIG`

```rust
const _NSIG: u32 = 64u32;
```

