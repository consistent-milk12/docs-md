*[libc](../../../../../../index.md) / [unix](../../../../../index.md) / [linux_like](../../../../index.md) / [linux](../../../index.md) / [gnu](../../index.md) / [b64](../index.md) / [x86_64](index.md)*

---

# Module `x86_64`

x86_64-specific definitions for 64-bit linux-like values

## Modules

- [`not_x32`](not_x32/index.md) - 

## Structs

### `sigaction`

```rust
struct sigaction {
    pub sa_sigaction: crate::sighandler_t,
    pub sa_mask: crate::sigset_t,
    pub sa_flags: crate::c_int,
    pub sa_restorer: core::option::Option<fn()>,
}
```

#### Trait Implementations

##### `impl Clone for sigaction`

- `fn clone(self: &Self) -> sigaction` — [`sigaction`](../index.md)

##### `impl Copy for sigaction`

##### `impl Debug for sigaction`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `statfs`

```rust
struct statfs {
    pub f_type: crate::__fsword_t,
    pub f_bsize: crate::__fsword_t,
    pub f_blocks: crate::fsblkcnt_t,
    pub f_bfree: crate::fsblkcnt_t,
    pub f_bavail: crate::fsblkcnt_t,
    pub f_files: crate::fsfilcnt_t,
    pub f_ffree: crate::fsfilcnt_t,
    pub f_fsid: crate::fsid_t,
    pub f_namelen: crate::__fsword_t,
    pub f_frsize: crate::__fsword_t,
    f_spare: [crate::__fsword_t; 5],
}
```

#### Trait Implementations

##### `impl Clone for statfs`

- `fn clone(self: &Self) -> statfs` — [`statfs`](../index.md)

##### `impl Copy for statfs`

##### `impl Debug for statfs`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `flock`

```rust
struct flock {
    pub l_type: crate::c_short,
    pub l_whence: crate::c_short,
    pub l_start: crate::off_t,
    pub l_len: crate::off_t,
    pub l_pid: crate::pid_t,
}
```

#### Trait Implementations

##### `impl Clone for flock`

- `fn clone(self: &Self) -> flock` — [`flock`](../index.md)

##### `impl Copy for flock`

##### `impl Debug for flock`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `flock64`

```rust
struct flock64 {
    pub l_type: crate::c_short,
    pub l_whence: crate::c_short,
    pub l_start: crate::off64_t,
    pub l_len: crate::off64_t,
    pub l_pid: crate::pid_t,
}
```

#### Trait Implementations

##### `impl Clone for flock64`

- `fn clone(self: &Self) -> flock64` — [`flock64`](../index.md)

##### `impl Copy for flock64`

##### `impl Debug for flock64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `siginfo_t`

```rust
struct siginfo_t {
    pub si_signo: crate::c_int,
    pub si_errno: crate::c_int,
    pub si_code: crate::c_int,
    _align: [u64; 0],
    // [REDACTED: Private Fields]
}
```

#### Implementations

- `unsafe fn si_addr(self: &Self) -> *mut c_void`

- `unsafe fn si_value(self: &Self) -> crate::sigval` — [`sigval`](../../../../../../index.md)

#### Trait Implementations

##### `impl Clone for siginfo_t`

- `fn clone(self: &Self) -> siginfo_t` — [`siginfo_t`](../index.md)

##### `impl Copy for siginfo_t`

##### `impl Debug for siginfo_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `stack_t`

```rust
struct stack_t {
    pub ss_sp: *mut crate::c_void,
    pub ss_flags: crate::c_int,
    pub ss_size: crate::size_t,
}
```

#### Trait Implementations

##### `impl Clone for stack_t`

- `fn clone(self: &Self) -> stack_t` — [`stack_t`](../index.md)

##### `impl Copy for stack_t`

##### `impl Debug for stack_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `stat`

```rust
struct stat {
    pub st_dev: crate::dev_t,
    pub st_ino: crate::ino_t,
    pub st_nlink: crate::nlink_t,
    pub st_mode: crate::mode_t,
    pub st_uid: crate::uid_t,
    pub st_gid: crate::gid_t,
    __pad0: crate::types::Padding<crate::c_int>,
    pub st_rdev: crate::dev_t,
    pub st_size: crate::off_t,
    pub st_blksize: crate::blksize_t,
    pub st_blocks: crate::blkcnt_t,
    pub st_atime: crate::time_t,
    pub st_atime_nsec: i64,
    pub st_mtime: crate::time_t,
    pub st_mtime_nsec: i64,
    pub st_ctime: crate::time_t,
    pub st_ctime_nsec: i64,
    __unused: crate::types::Padding<[i64; 3]>,
}
```

#### Trait Implementations

##### `impl Clone for stat`

- `fn clone(self: &Self) -> stat` — [`stat`](../index.md)

##### `impl Copy for stat`

##### `impl Debug for stat`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `stat64`

```rust
struct stat64 {
    pub st_dev: crate::dev_t,
    pub st_ino: crate::ino64_t,
    pub st_nlink: crate::nlink_t,
    pub st_mode: crate::mode_t,
    pub st_uid: crate::uid_t,
    pub st_gid: crate::gid_t,
    __pad0: crate::types::Padding<crate::c_int>,
    pub st_rdev: crate::dev_t,
    pub st_size: crate::off_t,
    pub st_blksize: crate::blksize_t,
    pub st_blocks: crate::blkcnt64_t,
    pub st_atime: crate::time_t,
    pub st_atime_nsec: i64,
    pub st_mtime: crate::time_t,
    pub st_mtime_nsec: i64,
    pub st_ctime: crate::time_t,
    pub st_ctime_nsec: i64,
    __reserved: crate::types::Padding<[i64; 3]>,
}
```

#### Trait Implementations

##### `impl Clone for stat64`

- `fn clone(self: &Self) -> stat64` — [`stat64`](../index.md)

##### `impl Copy for stat64`

##### `impl Debug for stat64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `statfs64`

```rust
struct statfs64 {
    pub f_type: crate::__fsword_t,
    pub f_bsize: crate::__fsword_t,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_fsid: crate::fsid_t,
    pub f_namelen: crate::__fsword_t,
    pub f_frsize: crate::__fsword_t,
    pub f_flags: crate::__fsword_t,
    pub f_spare: [crate::__fsword_t; 4],
}
```

#### Trait Implementations

##### `impl Clone for statfs64`

- `fn clone(self: &Self) -> statfs64` — [`statfs64`](../index.md)

##### `impl Copy for statfs64`

##### `impl Debug for statfs64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `statvfs64`

```rust
struct statvfs64 {
    pub f_bsize: crate::c_ulong,
    pub f_frsize: crate::c_ulong,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_favail: u64,
    pub f_fsid: crate::c_ulong,
    pub f_flag: crate::c_ulong,
    pub f_namemax: crate::c_ulong,
    __f_spare: [crate::c_int; 6],
}
```

#### Trait Implementations

##### `impl Clone for statvfs64`

- `fn clone(self: &Self) -> statvfs64` — [`statvfs64`](../index.md)

##### `impl Copy for statvfs64`

##### `impl Debug for statvfs64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pthread_attr_t`

```rust
struct pthread_attr_t {
    __size: [u64; 7],
}
```

#### Trait Implementations

##### `impl Clone for pthread_attr_t`

- `fn clone(self: &Self) -> pthread_attr_t` — [`pthread_attr_t`](../index.md)

##### `impl Copy for pthread_attr_t`

##### `impl Debug for pthread_attr_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `_libc_fpxreg`

```rust
struct _libc_fpxreg {
    pub significand: [u16; 4],
    pub exponent: u16,
    __private: [u16; 3],
}
```

#### Trait Implementations

##### `impl Clone for _libc_fpxreg`

- `fn clone(self: &Self) -> _libc_fpxreg` — [`_libc_fpxreg`](../index.md)

##### `impl Copy for _libc_fpxreg`

##### `impl Debug for _libc_fpxreg`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `_libc_xmmreg`

```rust
struct _libc_xmmreg {
    pub element: [u32; 4],
}
```

#### Trait Implementations

##### `impl Clone for _libc_xmmreg`

- `fn clone(self: &Self) -> _libc_xmmreg` — [`_libc_xmmreg`](../index.md)

##### `impl Copy for _libc_xmmreg`

##### `impl Debug for _libc_xmmreg`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `_libc_fpstate`

```rust
struct _libc_fpstate {
    pub cwd: u16,
    pub swd: u16,
    pub ftw: u16,
    pub fop: u16,
    pub rip: u64,
    pub rdp: u64,
    pub mxcsr: u32,
    pub mxcr_mask: u32,
    pub _st: [_libc_fpxreg; 8],
    pub _xmm: [_libc_xmmreg; 16],
    __private: [u64; 12],
}
```

#### Trait Implementations

##### `impl Clone for _libc_fpstate`

- `fn clone(self: &Self) -> _libc_fpstate` — [`_libc_fpstate`](../index.md)

##### `impl Copy for _libc_fpstate`

##### `impl Debug for _libc_fpstate`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `user_regs_struct`

```rust
struct user_regs_struct {
    pub r15: crate::c_ulonglong,
    pub r14: crate::c_ulonglong,
    pub r13: crate::c_ulonglong,
    pub r12: crate::c_ulonglong,
    pub rbp: crate::c_ulonglong,
    pub rbx: crate::c_ulonglong,
    pub r11: crate::c_ulonglong,
    pub r10: crate::c_ulonglong,
    pub r9: crate::c_ulonglong,
    pub r8: crate::c_ulonglong,
    pub rax: crate::c_ulonglong,
    pub rcx: crate::c_ulonglong,
    pub rdx: crate::c_ulonglong,
    pub rsi: crate::c_ulonglong,
    pub rdi: crate::c_ulonglong,
    pub orig_rax: crate::c_ulonglong,
    pub rip: crate::c_ulonglong,
    pub cs: crate::c_ulonglong,
    pub eflags: crate::c_ulonglong,
    pub rsp: crate::c_ulonglong,
    pub ss: crate::c_ulonglong,
    pub fs_base: crate::c_ulonglong,
    pub gs_base: crate::c_ulonglong,
    pub ds: crate::c_ulonglong,
    pub es: crate::c_ulonglong,
    pub fs: crate::c_ulonglong,
    pub gs: crate::c_ulonglong,
}
```

#### Trait Implementations

##### `impl Clone for user_regs_struct`

- `fn clone(self: &Self) -> user_regs_struct` — [`user_regs_struct`](../index.md)

##### `impl Copy for user_regs_struct`

##### `impl Debug for user_regs_struct`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `user`

```rust
struct user {
    pub regs: user_regs_struct,
    pub u_fpvalid: crate::c_int,
    pub i387: user_fpregs_struct,
    pub u_tsize: crate::c_ulonglong,
    pub u_dsize: crate::c_ulonglong,
    pub u_ssize: crate::c_ulonglong,
    pub start_code: crate::c_ulonglong,
    pub start_stack: crate::c_ulonglong,
    pub signal: crate::c_longlong,
    __reserved: crate::types::Padding<crate::c_int>,
    pub u_ar0: *mut user_regs_struct,
    pub u_fpstate: *mut user_fpregs_struct,
    pub magic: crate::c_ulonglong,
    pub u_comm: [crate::c_char; 32],
    pub u_debugreg: [crate::c_ulonglong; 8],
}
```

#### Trait Implementations

##### `impl Clone for user`

- `fn clone(self: &Self) -> user` — [`user`](../index.md)

##### `impl Copy for user`

##### `impl Debug for user`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `mcontext_t`

```rust
struct mcontext_t {
    pub gregs: [greg_t; 23],
    pub fpregs: *mut _libc_fpstate,
    __private: [u64; 8],
}
```

#### Trait Implementations

##### `impl Clone for mcontext_t`

- `fn clone(self: &Self) -> mcontext_t` — [`mcontext_t`](../index.md)

##### `impl Copy for mcontext_t`

##### `impl Debug for mcontext_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ipc_perm`

```rust
struct ipc_perm {
    pub __key: crate::key_t,
    pub uid: crate::uid_t,
    pub gid: crate::gid_t,
    pub cuid: crate::uid_t,
    pub cgid: crate::gid_t,
    pub mode: crate::c_ushort,
    __pad1: crate::types::Padding<crate::c_ushort>,
    pub __seq: crate::c_ushort,
    __pad2: crate::types::Padding<crate::c_ushort>,
    __unused1: crate::types::Padding<u64>,
    __unused2: crate::types::Padding<u64>,
}
```

#### Trait Implementations

##### `impl Clone for ipc_perm`

- `fn clone(self: &Self) -> ipc_perm` — [`ipc_perm`](../index.md)

##### `impl Copy for ipc_perm`

##### `impl Debug for ipc_perm`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `shmid_ds`

```rust
struct shmid_ds {
    pub shm_perm: crate::ipc_perm,
    pub shm_segsz: crate::size_t,
    pub shm_atime: crate::time_t,
    pub shm_dtime: crate::time_t,
    pub shm_ctime: crate::time_t,
    pub shm_cpid: crate::pid_t,
    pub shm_lpid: crate::pid_t,
    pub shm_nattch: crate::shmatt_t,
    __unused4: crate::types::Padding<u64>,
    __unused5: crate::types::Padding<u64>,
}
```

#### Trait Implementations

##### `impl Clone for shmid_ds`

- `fn clone(self: &Self) -> shmid_ds` — [`shmid_ds`](../index.md)

##### `impl Copy for shmid_ds`

##### `impl Debug for shmid_ds`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptrace_rseq_configuration`

```rust
struct ptrace_rseq_configuration {
    pub rseq_abi_pointer: crate::__u64,
    pub rseq_abi_size: crate::__u32,
    pub signature: crate::__u32,
    pub flags: crate::__u32,
    pub pad: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for ptrace_rseq_configuration`

- `fn clone(self: &Self) -> ptrace_rseq_configuration` — [`ptrace_rseq_configuration`](../index.md)

##### `impl Copy for ptrace_rseq_configuration`

##### `impl Debug for ptrace_rseq_configuration`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `clone_args`

```rust
struct clone_args {
    pub flags: crate::c_ulonglong,
    pub pidfd: crate::c_ulonglong,
    pub child_tid: crate::c_ulonglong,
    pub parent_tid: crate::c_ulonglong,
    pub exit_signal: crate::c_ulonglong,
    pub stack: crate::c_ulonglong,
    pub stack_size: crate::c_ulonglong,
    pub tls: crate::c_ulonglong,
    pub set_tid: crate::c_ulonglong,
    pub set_tid_size: crate::c_ulonglong,
    pub cgroup: crate::c_ulonglong,
}
```

#### Trait Implementations

##### `impl Clone for clone_args`

- `fn clone(self: &Self) -> clone_args` — [`clone_args`](../index.md)

##### `impl Copy for clone_args`

##### `impl Debug for clone_args`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `user_fpregs_struct`

```rust
struct user_fpregs_struct {
    pub cwd: crate::c_ushort,
    pub swd: crate::c_ushort,
    pub ftw: crate::c_ushort,
    pub fop: crate::c_ushort,
    pub rip: crate::c_ulonglong,
    pub rdp: crate::c_ulonglong,
    pub mxcsr: crate::c_uint,
    pub mxcr_mask: crate::c_uint,
    pub st_space: [crate::c_uint; 32],
    pub xmm_space: [crate::c_uint; 64],
    padding: [crate::c_uint; 24],
}
```

#### Trait Implementations

##### `impl Clone for user_fpregs_struct`

- `fn clone(self: &Self) -> user_fpregs_struct` — [`user_fpregs_struct`](../index.md)

##### `impl Copy for user_fpregs_struct`

##### `impl Debug for user_fpregs_struct`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ucontext_t`

```rust
struct ucontext_t {
    pub uc_flags: crate::c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: crate::stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: crate::sigset_t,
    __private: [u8; 512],
    __ssp: [crate::c_ulonglong; 4],
}
```

#### Trait Implementations

##### `impl Clone for ucontext_t`

- `fn clone(self: &Self) -> ucontext_t` — [`ucontext_t`](../index.md)

##### `impl Copy for ucontext_t`

##### `impl Debug for ucontext_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `max_align_t`

```rust
struct max_align_t {
    priv_: [f64; 4],
}
```

#### Trait Implementations

##### `impl Clone for max_align_t`

- `fn clone(self: &Self) -> max_align_t` — [`max_align_t`](../index.md)

##### `impl Copy for max_align_t`

##### `impl Debug for max_align_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `statvfs`

```rust
struct statvfs {
    pub f_bsize: crate::c_ulong,
    pub f_frsize: crate::c_ulong,
    pub f_blocks: crate::fsblkcnt_t,
    pub f_bfree: crate::fsblkcnt_t,
    pub f_bavail: crate::fsblkcnt_t,
    pub f_files: crate::fsfilcnt_t,
    pub f_ffree: crate::fsfilcnt_t,
    pub f_favail: crate::fsfilcnt_t,
    pub f_fsid: crate::c_ulong,
    pub f_flag: crate::c_ulong,
    pub f_namemax: crate::c_ulong,
    __f_spare: [crate::c_int; 6],
}
```

#### Trait Implementations

##### `impl Clone for statvfs`

- `fn clone(self: &Self) -> statvfs` — [`statvfs`](#statvfs)

##### `impl Copy for statvfs`

##### `impl Debug for statvfs`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `getcontext`

```rust
unsafe fn getcontext(ucp: *mut ucontext_t) -> c_int
```

### `setcontext`

```rust
unsafe fn setcontext(ucp: *const ucontext_t) -> c_int
```

### `makecontext`

```rust
unsafe fn makecontext(ucp: *mut ucontext_t, func: fn(), argc: c_int)
```

### `swapcontext`

```rust
unsafe fn swapcontext(uocp: *mut ucontext_t, ucp: *const ucontext_t) -> c_int
```

### `sysctl`

```rust
unsafe fn sysctl(name: *mut c_int, namelen: c_int, oldp: *mut c_void, oldlenp: *mut size_t, newp: *mut c_void, newlen: size_t) -> c_int
```

## Type Aliases

### `wchar_t`

```rust
type wchar_t = i32;
```

### `nlink_t`

```rust
type nlink_t = u64;
```

### `blksize_t`

```rust
type blksize_t = i64;
```

### `greg_t`

```rust
type greg_t = i64;
```

### `suseconds_t`

```rust
type suseconds_t = i64;
```

### `__u64`

```rust
type __u64 = crate::c_ulonglong;
```

### `__s64`

```rust
type __s64 = crate::c_longlong;
```

## Constants

### `POSIX_FADV_DONTNEED`

```rust
const POSIX_FADV_DONTNEED: crate::c_int = 4i32;
```

### `POSIX_FADV_NOREUSE`

```rust
const POSIX_FADV_NOREUSE: crate::c_int = 5i32;
```

### `VEOF`

```rust
const VEOF: usize = 4usize;
```

### `RTLD_DEEPBIND`

```rust
const RTLD_DEEPBIND: crate::c_int = 8i32;
```

### `RTLD_GLOBAL`

```rust
const RTLD_GLOBAL: crate::c_int = 256i32;
```

### `RTLD_NOLOAD`

```rust
const RTLD_NOLOAD: crate::c_int = 4i32;
```

### `O_APPEND`

```rust
const O_APPEND: crate::c_int = 1_024i32;
```

### `O_CREAT`

```rust
const O_CREAT: crate::c_int = 64i32;
```

### `O_EXCL`

```rust
const O_EXCL: crate::c_int = 128i32;
```

### `O_NOCTTY`

```rust
const O_NOCTTY: crate::c_int = 256i32;
```

### `O_NONBLOCK`

```rust
const O_NONBLOCK: crate::c_int = 2_048i32;
```

### `O_SYNC`

```rust
const O_SYNC: crate::c_int = 1_052_672i32;
```

### `O_RSYNC`

```rust
const O_RSYNC: crate::c_int = 1_052_672i32;
```

### `O_DSYNC`

```rust
const O_DSYNC: crate::c_int = 4_096i32;
```

### `O_FSYNC`

```rust
const O_FSYNC: crate::c_int = 1_052_672i32;
```

### `O_NOATIME`

```rust
const O_NOATIME: crate::c_int = 262_144i32;
```

### `O_PATH`

```rust
const O_PATH: crate::c_int = 2_097_152i32;
```

### `O_TMPFILE`

```rust
const O_TMPFILE: crate::c_int = 4_259_840i32;
```

### `MADV_SOFT_OFFLINE`

```rust
const MADV_SOFT_OFFLINE: crate::c_int = 101i32;
```

### `MAP_GROWSDOWN`

```rust
const MAP_GROWSDOWN: crate::c_int = 256i32;
```

### `EDEADLK`

```rust
const EDEADLK: crate::c_int = 35i32;
```

### `ENAMETOOLONG`

```rust
const ENAMETOOLONG: crate::c_int = 36i32;
```

### `ENOLCK`

```rust
const ENOLCK: crate::c_int = 37i32;
```

### `ENOSYS`

```rust
const ENOSYS: crate::c_int = 38i32;
```

### `ENOTEMPTY`

```rust
const ENOTEMPTY: crate::c_int = 39i32;
```

### `ELOOP`

```rust
const ELOOP: crate::c_int = 40i32;
```

### `ENOMSG`

```rust
const ENOMSG: crate::c_int = 42i32;
```

### `EIDRM`

```rust
const EIDRM: crate::c_int = 43i32;
```

### `ECHRNG`

```rust
const ECHRNG: crate::c_int = 44i32;
```

### `EL2NSYNC`

```rust
const EL2NSYNC: crate::c_int = 45i32;
```

### `EL3HLT`

```rust
const EL3HLT: crate::c_int = 46i32;
```

### `EL3RST`

```rust
const EL3RST: crate::c_int = 47i32;
```

### `ELNRNG`

```rust
const ELNRNG: crate::c_int = 48i32;
```

### `EUNATCH`

```rust
const EUNATCH: crate::c_int = 49i32;
```

### `ENOCSI`

```rust
const ENOCSI: crate::c_int = 50i32;
```

### `EL2HLT`

```rust
const EL2HLT: crate::c_int = 51i32;
```

### `EBADE`

```rust
const EBADE: crate::c_int = 52i32;
```

### `EBADR`

```rust
const EBADR: crate::c_int = 53i32;
```

### `EXFULL`

```rust
const EXFULL: crate::c_int = 54i32;
```

### `ENOANO`

```rust
const ENOANO: crate::c_int = 55i32;
```

### `EBADRQC`

```rust
const EBADRQC: crate::c_int = 56i32;
```

### `EBADSLT`

```rust
const EBADSLT: crate::c_int = 57i32;
```

### `EMULTIHOP`

```rust
const EMULTIHOP: crate::c_int = 72i32;
```

### `EOVERFLOW`

```rust
const EOVERFLOW: crate::c_int = 75i32;
```

### `ENOTUNIQ`

```rust
const ENOTUNIQ: crate::c_int = 76i32;
```

### `EBADFD`

```rust
const EBADFD: crate::c_int = 77i32;
```

### `EBADMSG`

```rust
const EBADMSG: crate::c_int = 74i32;
```

### `EREMCHG`

```rust
const EREMCHG: crate::c_int = 78i32;
```

### `ELIBACC`

```rust
const ELIBACC: crate::c_int = 79i32;
```

### `ELIBBAD`

```rust
const ELIBBAD: crate::c_int = 80i32;
```

### `ELIBSCN`

```rust
const ELIBSCN: crate::c_int = 81i32;
```

### `ELIBMAX`

```rust
const ELIBMAX: crate::c_int = 82i32;
```

### `ELIBEXEC`

```rust
const ELIBEXEC: crate::c_int = 83i32;
```

### `EILSEQ`

```rust
const EILSEQ: crate::c_int = 84i32;
```

### `ERESTART`

```rust
const ERESTART: crate::c_int = 85i32;
```

### `ESTRPIPE`

```rust
const ESTRPIPE: crate::c_int = 86i32;
```

### `EUSERS`

```rust
const EUSERS: crate::c_int = 87i32;
```

### `ENOTSOCK`

```rust
const ENOTSOCK: crate::c_int = 88i32;
```

### `EDESTADDRREQ`

```rust
const EDESTADDRREQ: crate::c_int = 89i32;
```

### `EMSGSIZE`

```rust
const EMSGSIZE: crate::c_int = 90i32;
```

### `EPROTOTYPE`

```rust
const EPROTOTYPE: crate::c_int = 91i32;
```

### `ENOPROTOOPT`

```rust
const ENOPROTOOPT: crate::c_int = 92i32;
```

### `EPROTONOSUPPORT`

```rust
const EPROTONOSUPPORT: crate::c_int = 93i32;
```

### `ESOCKTNOSUPPORT`

```rust
const ESOCKTNOSUPPORT: crate::c_int = 94i32;
```

### `EOPNOTSUPP`

```rust
const EOPNOTSUPP: crate::c_int = 95i32;
```

### `EPFNOSUPPORT`

```rust
const EPFNOSUPPORT: crate::c_int = 96i32;
```

### `EAFNOSUPPORT`

```rust
const EAFNOSUPPORT: crate::c_int = 97i32;
```

### `EADDRINUSE`

```rust
const EADDRINUSE: crate::c_int = 98i32;
```

### `EADDRNOTAVAIL`

```rust
const EADDRNOTAVAIL: crate::c_int = 99i32;
```

### `ENETDOWN`

```rust
const ENETDOWN: crate::c_int = 100i32;
```

### `ENETUNREACH`

```rust
const ENETUNREACH: crate::c_int = 101i32;
```

### `ENETRESET`

```rust
const ENETRESET: crate::c_int = 102i32;
```

### `ECONNABORTED`

```rust
const ECONNABORTED: crate::c_int = 103i32;
```

### `ECONNRESET`

```rust
const ECONNRESET: crate::c_int = 104i32;
```

### `ENOBUFS`

```rust
const ENOBUFS: crate::c_int = 105i32;
```

### `EISCONN`

```rust
const EISCONN: crate::c_int = 106i32;
```

### `ENOTCONN`

```rust
const ENOTCONN: crate::c_int = 107i32;
```

### `ESHUTDOWN`

```rust
const ESHUTDOWN: crate::c_int = 108i32;
```

### `ETOOMANYREFS`

```rust
const ETOOMANYREFS: crate::c_int = 109i32;
```

### `ETIMEDOUT`

```rust
const ETIMEDOUT: crate::c_int = 110i32;
```

### `ECONNREFUSED`

```rust
const ECONNREFUSED: crate::c_int = 111i32;
```

### `EHOSTDOWN`

```rust
const EHOSTDOWN: crate::c_int = 112i32;
```

### `EHOSTUNREACH`

```rust
const EHOSTUNREACH: crate::c_int = 113i32;
```

### `EALREADY`

```rust
const EALREADY: crate::c_int = 114i32;
```

### `EINPROGRESS`

```rust
const EINPROGRESS: crate::c_int = 115i32;
```

### `ESTALE`

```rust
const ESTALE: crate::c_int = 116i32;
```

### `EDQUOT`

```rust
const EDQUOT: crate::c_int = 122i32;
```

### `ENOMEDIUM`

```rust
const ENOMEDIUM: crate::c_int = 123i32;
```

### `EMEDIUMTYPE`

```rust
const EMEDIUMTYPE: crate::c_int = 124i32;
```

### `ECANCELED`

```rust
const ECANCELED: crate::c_int = 125i32;
```

### `ENOKEY`

```rust
const ENOKEY: crate::c_int = 126i32;
```

### `EKEYEXPIRED`

```rust
const EKEYEXPIRED: crate::c_int = 127i32;
```

### `EKEYREVOKED`

```rust
const EKEYREVOKED: crate::c_int = 128i32;
```

### `EKEYREJECTED`

```rust
const EKEYREJECTED: crate::c_int = 129i32;
```

### `EOWNERDEAD`

```rust
const EOWNERDEAD: crate::c_int = 130i32;
```

### `ENOTRECOVERABLE`

```rust
const ENOTRECOVERABLE: crate::c_int = 131i32;
```

### `EHWPOISON`

```rust
const EHWPOISON: crate::c_int = 133i32;
```

### `ERFKILL`

```rust
const ERFKILL: crate::c_int = 132i32;
```

### `SOCK_STREAM`

```rust
const SOCK_STREAM: crate::c_int = 1i32;
```

### `SOCK_DGRAM`

```rust
const SOCK_DGRAM: crate::c_int = 2i32;
```

### `SA_ONSTACK`

```rust
const SA_ONSTACK: crate::c_int = 134_217_728i32;
```

### `SA_SIGINFO`

```rust
const SA_SIGINFO: crate::c_int = 4i32;
```

### `SA_NOCLDWAIT`

```rust
const SA_NOCLDWAIT: crate::c_int = 2i32;
```

### `SIGTTIN`

```rust
const SIGTTIN: crate::c_int = 21i32;
```

### `SIGTTOU`

```rust
const SIGTTOU: crate::c_int = 22i32;
```

### `SIGXCPU`

```rust
const SIGXCPU: crate::c_int = 24i32;
```

### `SIGXFSZ`

```rust
const SIGXFSZ: crate::c_int = 25i32;
```

### `SIGVTALRM`

```rust
const SIGVTALRM: crate::c_int = 26i32;
```

### `SIGPROF`

```rust
const SIGPROF: crate::c_int = 27i32;
```

### `SIGWINCH`

```rust
const SIGWINCH: crate::c_int = 28i32;
```

### `SIGCHLD`

```rust
const SIGCHLD: crate::c_int = 17i32;
```

### `SIGBUS`

```rust
const SIGBUS: crate::c_int = 7i32;
```

### `SIGUSR1`

```rust
const SIGUSR1: crate::c_int = 10i32;
```

### `SIGUSR2`

```rust
const SIGUSR2: crate::c_int = 12i32;
```

### `SIGCONT`

```rust
const SIGCONT: crate::c_int = 18i32;
```

### `SIGSTOP`

```rust
const SIGSTOP: crate::c_int = 19i32;
```

### `SIGTSTP`

```rust
const SIGTSTP: crate::c_int = 20i32;
```

### `SIGURG`

```rust
const SIGURG: crate::c_int = 23i32;
```

### `SIGIO`

```rust
const SIGIO: crate::c_int = 29i32;
```

### `SIGSYS`

```rust
const SIGSYS: crate::c_int = 31i32;
```

### `SIGSTKFLT`

```rust
const SIGSTKFLT: crate::c_int = 16i32;
```

### `SIGUNUSED`

```rust
const SIGUNUSED: crate::c_int = 31i32;
```

### `SIGPOLL`

```rust
const SIGPOLL: crate::c_int = 29i32;
```

### `SIGPWR`

```rust
const SIGPWR: crate::c_int = 30i32;
```

### `SIG_SETMASK`

```rust
const SIG_SETMASK: crate::c_int = 2i32;
```

### `SIG_BLOCK`

```rust
const SIG_BLOCK: crate::c_int = 0i32;
```

### `SIG_UNBLOCK`

```rust
const SIG_UNBLOCK: crate::c_int = 1i32;
```

### `POLLWRNORM`

```rust
const POLLWRNORM: crate::c_short = 256i16;
```

### `POLLWRBAND`

```rust
const POLLWRBAND: crate::c_short = 512i16;
```

### `O_ASYNC`

```rust
const O_ASYNC: crate::c_int = 8_192i32;
```

### `O_NDELAY`

```rust
const O_NDELAY: crate::c_int = 2_048i32;
```

### `PTRACE_DETACH`

```rust
const PTRACE_DETACH: crate::c_uint = 17u32;
```

### `PTRACE_GET_RSEQ_CONFIGURATION`

```rust
const PTRACE_GET_RSEQ_CONFIGURATION: crate::c_uint = 16_911u32;
```

### `EFD_NONBLOCK`

```rust
const EFD_NONBLOCK: crate::c_int = 2_048i32;
```

### `F_GETLK`

```rust
const F_GETLK: crate::c_int = 5i32;
```

### `F_GETOWN`

```rust
const F_GETOWN: crate::c_int = 9i32;
```

### `F_SETOWN`

```rust
const F_SETOWN: crate::c_int = 8i32;
```

### `F_SETLK`

```rust
const F_SETLK: crate::c_int = 6i32;
```

### `F_SETLKW`

```rust
const F_SETLKW: crate::c_int = 7i32;
```

### `F_OFD_GETLK`

```rust
const F_OFD_GETLK: crate::c_int = 36i32;
```

### `F_OFD_SETLK`

```rust
const F_OFD_SETLK: crate::c_int = 37i32;
```

### `F_OFD_SETLKW`

```rust
const F_OFD_SETLKW: crate::c_int = 38i32;
```

### `F_RDLCK`

```rust
const F_RDLCK: crate::c_int = 0i32;
```

### `F_WRLCK`

```rust
const F_WRLCK: crate::c_int = 1i32;
```

### `F_UNLCK`

```rust
const F_UNLCK: crate::c_int = 2i32;
```

### `SFD_NONBLOCK`

```rust
const SFD_NONBLOCK: crate::c_int = 2_048i32;
```

### `TCSANOW`

```rust
const TCSANOW: crate::c_int = 0i32;
```

### `TCSADRAIN`

```rust
const TCSADRAIN: crate::c_int = 1i32;
```

### `TCSAFLUSH`

```rust
const TCSAFLUSH: crate::c_int = 2i32;
```

### `SFD_CLOEXEC`

```rust
const SFD_CLOEXEC: crate::c_int = 524_288i32;
```

### `NCCS`

```rust
const NCCS: usize = 32usize;
```

### `O_TRUNC`

```rust
const O_TRUNC: crate::c_int = 512i32;
```

### `O_CLOEXEC`

```rust
const O_CLOEXEC: crate::c_int = 524_288i32;
```

### `EBFONT`

```rust
const EBFONT: crate::c_int = 59i32;
```

### `ENOSTR`

```rust
const ENOSTR: crate::c_int = 60i32;
```

### `ENODATA`

```rust
const ENODATA: crate::c_int = 61i32;
```

### `ETIME`

```rust
const ETIME: crate::c_int = 62i32;
```

### `ENOSR`

```rust
const ENOSR: crate::c_int = 63i32;
```

### `ENONET`

```rust
const ENONET: crate::c_int = 64i32;
```

### `ENOPKG`

```rust
const ENOPKG: crate::c_int = 65i32;
```

### `EREMOTE`

```rust
const EREMOTE: crate::c_int = 66i32;
```

### `ENOLINK`

```rust
const ENOLINK: crate::c_int = 67i32;
```

### `EADV`

```rust
const EADV: crate::c_int = 68i32;
```

### `ESRMNT`

```rust
const ESRMNT: crate::c_int = 69i32;
```

### `ECOMM`

```rust
const ECOMM: crate::c_int = 70i32;
```

### `EPROTO`

```rust
const EPROTO: crate::c_int = 71i32;
```

### `EDOTDOT`

```rust
const EDOTDOT: crate::c_int = 73i32;
```

### `SA_NODEFER`

```rust
const SA_NODEFER: crate::c_int = 1_073_741_824i32;
```

### `SA_RESETHAND`

```rust
const SA_RESETHAND: crate::c_int = -2_147_483_648i32;
```

### `SA_RESTART`

```rust
const SA_RESTART: crate::c_int = 268_435_456i32;
```

### `SA_NOCLDSTOP`

```rust
const SA_NOCLDSTOP: crate::c_int = 1i32;
```

### `EPOLL_CLOEXEC`

```rust
const EPOLL_CLOEXEC: crate::c_int = 524_288i32;
```

### `EFD_CLOEXEC`

```rust
const EFD_CLOEXEC: crate::c_int = 524_288i32;
```

### `__SIZEOF_PTHREAD_CONDATTR_T`

```rust
const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4usize;
```

### `__SIZEOF_PTHREAD_MUTEXATTR_T`

```rust
const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4usize;
```

### `__SIZEOF_PTHREAD_BARRIERATTR_T`

```rust
const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4usize;
```

### `O_DIRECT`

```rust
const O_DIRECT: crate::c_int = 16_384i32;
```

### `O_DIRECTORY`

```rust
const O_DIRECTORY: crate::c_int = 65_536i32;
```

### `O_NOFOLLOW`

```rust
const O_NOFOLLOW: crate::c_int = 131_072i32;
```

### `MAP_HUGETLB`

```rust
const MAP_HUGETLB: crate::c_int = 262_144i32;
```

### `MAP_LOCKED`

```rust
const MAP_LOCKED: crate::c_int = 8_192i32;
```

### `MAP_NORESERVE`

```rust
const MAP_NORESERVE: crate::c_int = 16_384i32;
```

### `MAP_32BIT`

```rust
const MAP_32BIT: crate::c_int = 64i32;
```

### `MAP_ANON`

```rust
const MAP_ANON: crate::c_int = 32i32;
```

### `MAP_ANONYMOUS`

```rust
const MAP_ANONYMOUS: crate::c_int = 32i32;
```

### `MAP_DENYWRITE`

```rust
const MAP_DENYWRITE: crate::c_int = 2_048i32;
```

### `MAP_EXECUTABLE`

```rust
const MAP_EXECUTABLE: crate::c_int = 4_096i32;
```

### `MAP_POPULATE`

```rust
const MAP_POPULATE: crate::c_int = 32_768i32;
```

### `MAP_NONBLOCK`

```rust
const MAP_NONBLOCK: crate::c_int = 65_536i32;
```

### `MAP_STACK`

```rust
const MAP_STACK: crate::c_int = 131_072i32;
```

### `MAP_SYNC`

```rust
const MAP_SYNC: crate::c_int = 524_288i32;
```

### `EDEADLOCK`

```rust
const EDEADLOCK: crate::c_int = 35i32;
```

### `EUCLEAN`

```rust
const EUCLEAN: crate::c_int = 117i32;
```

### `ENOTNAM`

```rust
const ENOTNAM: crate::c_int = 118i32;
```

### `ENAVAIL`

```rust
const ENAVAIL: crate::c_int = 119i32;
```

### `EISNAM`

```rust
const EISNAM: crate::c_int = 120i32;
```

### `EREMOTEIO`

```rust
const EREMOTEIO: crate::c_int = 121i32;
```

### `PTRACE_GETFPREGS`

```rust
const PTRACE_GETFPREGS: crate::c_uint = 14u32;
```

### `PTRACE_SETFPREGS`

```rust
const PTRACE_SETFPREGS: crate::c_uint = 15u32;
```

### `PTRACE_GETFPXREGS`

```rust
const PTRACE_GETFPXREGS: crate::c_uint = 18u32;
```

### `PTRACE_SETFPXREGS`

```rust
const PTRACE_SETFPXREGS: crate::c_uint = 19u32;
```

### `PTRACE_GETREGS`

```rust
const PTRACE_GETREGS: crate::c_uint = 12u32;
```

### `PTRACE_SETREGS`

```rust
const PTRACE_SETREGS: crate::c_uint = 13u32;
```

### `PTRACE_PEEKSIGINFO_SHARED`

```rust
const PTRACE_PEEKSIGINFO_SHARED: crate::c_uint = 1u32;
```

### `PTRACE_SYSEMU`

```rust
const PTRACE_SYSEMU: crate::c_uint = 31u32;
```

### `PTRACE_SYSEMU_SINGLESTEP`

```rust
const PTRACE_SYSEMU_SINGLESTEP: crate::c_uint = 32u32;
```

### `PR_GET_SPECULATION_CTRL`

```rust
const PR_GET_SPECULATION_CTRL: crate::c_int = 52i32;
```

### `PR_SET_SPECULATION_CTRL`

```rust
const PR_SET_SPECULATION_CTRL: crate::c_int = 53i32;
```

### `PR_SPEC_NOT_AFFECTED`

```rust
const PR_SPEC_NOT_AFFECTED: crate::c_uint = 0u32;
```

### `PR_SPEC_PRCTL`

```rust
const PR_SPEC_PRCTL: crate::c_uint = 1u32;
```

### `PR_SPEC_ENABLE`

```rust
const PR_SPEC_ENABLE: crate::c_uint = 2u32;
```

### `PR_SPEC_DISABLE`

```rust
const PR_SPEC_DISABLE: crate::c_uint = 4u32;
```

### `PR_SPEC_FORCE_DISABLE`

```rust
const PR_SPEC_FORCE_DISABLE: crate::c_uint = 8u32;
```

### `PR_SPEC_DISABLE_NOEXEC`

```rust
const PR_SPEC_DISABLE_NOEXEC: crate::c_uint = 16u32;
```

### `PR_SPEC_STORE_BYPASS`

```rust
const PR_SPEC_STORE_BYPASS: crate::c_int = 0i32;
```

### `PR_SPEC_INDIRECT_BRANCH`

```rust
const PR_SPEC_INDIRECT_BRANCH: crate::c_int = 1i32;
```

### `MCL_CURRENT`

```rust
const MCL_CURRENT: crate::c_int = 1i32;
```

### `MCL_FUTURE`

```rust
const MCL_FUTURE: crate::c_int = 2i32;
```

### `MCL_ONFAULT`

```rust
const MCL_ONFAULT: crate::c_int = 4i32;
```

### `SIGSTKSZ`

```rust
const SIGSTKSZ: crate::size_t = 8_192usize;
```

### `MINSIGSTKSZ`

```rust
const MINSIGSTKSZ: crate::size_t = 2_048usize;
```

### `CBAUD`

```rust
const CBAUD: crate::tcflag_t = 4_111u32;
```

### `TAB1`

```rust
const TAB1: crate::tcflag_t = 2_048u32;
```

### `TAB2`

```rust
const TAB2: crate::tcflag_t = 4_096u32;
```

### `TAB3`

```rust
const TAB3: crate::tcflag_t = 6_144u32;
```

### `CR1`

```rust
const CR1: crate::tcflag_t = 512u32;
```

### `CR2`

```rust
const CR2: crate::tcflag_t = 1_024u32;
```

### `CR3`

```rust
const CR3: crate::tcflag_t = 1_536u32;
```

### `FF1`

```rust
const FF1: crate::tcflag_t = 32_768u32;
```

### `BS1`

```rust
const BS1: crate::tcflag_t = 8_192u32;
```

### `VT1`

```rust
const VT1: crate::tcflag_t = 16_384u32;
```

### `VWERASE`

```rust
const VWERASE: usize = 14usize;
```

### `VREPRINT`

```rust
const VREPRINT: usize = 12usize;
```

### `VSUSP`

```rust
const VSUSP: usize = 10usize;
```

### `VSTART`

```rust
const VSTART: usize = 8usize;
```

### `VSTOP`

```rust
const VSTOP: usize = 9usize;
```

### `VDISCARD`

```rust
const VDISCARD: usize = 13usize;
```

### `VTIME`

```rust
const VTIME: usize = 5usize;
```

### `IXON`

```rust
const IXON: crate::tcflag_t = 1_024u32;
```

### `IXOFF`

```rust
const IXOFF: crate::tcflag_t = 4_096u32;
```

### `ONLCR`

```rust
const ONLCR: crate::tcflag_t = 4u32;
```

### `CSIZE`

```rust
const CSIZE: crate::tcflag_t = 48u32;
```

### `CS6`

```rust
const CS6: crate::tcflag_t = 16u32;
```

### `CS7`

```rust
const CS7: crate::tcflag_t = 32u32;
```

### `CS8`

```rust
const CS8: crate::tcflag_t = 48u32;
```

### `CSTOPB`

```rust
const CSTOPB: crate::tcflag_t = 64u32;
```

### `CREAD`

```rust
const CREAD: crate::tcflag_t = 128u32;
```

### `PARENB`

```rust
const PARENB: crate::tcflag_t = 256u32;
```

### `PARODD`

```rust
const PARODD: crate::tcflag_t = 512u32;
```

### `HUPCL`

```rust
const HUPCL: crate::tcflag_t = 1_024u32;
```

### `CLOCAL`

```rust
const CLOCAL: crate::tcflag_t = 2_048u32;
```

### `ECHOKE`

```rust
const ECHOKE: crate::tcflag_t = 2_048u32;
```

### `ECHOE`

```rust
const ECHOE: crate::tcflag_t = 16u32;
```

### `ECHOK`

```rust
const ECHOK: crate::tcflag_t = 32u32;
```

### `ECHONL`

```rust
const ECHONL: crate::tcflag_t = 64u32;
```

### `ECHOPRT`

```rust
const ECHOPRT: crate::tcflag_t = 1_024u32;
```

### `ECHOCTL`

```rust
const ECHOCTL: crate::tcflag_t = 512u32;
```

### `ISIG`

```rust
const ISIG: crate::tcflag_t = 1u32;
```

### `ICANON`

```rust
const ICANON: crate::tcflag_t = 2u32;
```

### `PENDIN`

```rust
const PENDIN: crate::tcflag_t = 16_384u32;
```

### `NOFLSH`

```rust
const NOFLSH: crate::tcflag_t = 128u32;
```

### `CIBAUD`

```rust
const CIBAUD: crate::tcflag_t = 269_418_496u32;
```

### `CBAUDEX`

```rust
const CBAUDEX: crate::tcflag_t = 4_096u32;
```

### `VSWTC`

```rust
const VSWTC: usize = 7usize;
```

### `OLCUC`

```rust
const OLCUC: crate::tcflag_t = 2u32;
```

### `NLDLY`

```rust
const NLDLY: crate::tcflag_t = 256u32;
```

### `CRDLY`

```rust
const CRDLY: crate::tcflag_t = 1_536u32;
```

### `TABDLY`

```rust
const TABDLY: crate::tcflag_t = 6_144u32;
```

### `BSDLY`

```rust
const BSDLY: crate::tcflag_t = 8_192u32;
```

### `FFDLY`

```rust
const FFDLY: crate::tcflag_t = 32_768u32;
```

### `VTDLY`

```rust
const VTDLY: crate::tcflag_t = 16_384u32;
```

### `XTABS`

```rust
const XTABS: crate::tcflag_t = 6_144u32;
```

### `B0`

```rust
const B0: crate::speed_t = 0u32;
```

### `B50`

```rust
const B50: crate::speed_t = 1u32;
```

### `B75`

```rust
const B75: crate::speed_t = 2u32;
```

### `B110`

```rust
const B110: crate::speed_t = 3u32;
```

### `B134`

```rust
const B134: crate::speed_t = 4u32;
```

### `B150`

```rust
const B150: crate::speed_t = 5u32;
```

### `B200`

```rust
const B200: crate::speed_t = 6u32;
```

### `B300`

```rust
const B300: crate::speed_t = 7u32;
```

### `B600`

```rust
const B600: crate::speed_t = 8u32;
```

### `B1200`

```rust
const B1200: crate::speed_t = 9u32;
```

### `B1800`

```rust
const B1800: crate::speed_t = 10u32;
```

### `B2400`

```rust
const B2400: crate::speed_t = 11u32;
```

### `B4800`

```rust
const B4800: crate::speed_t = 12u32;
```

### `B9600`

```rust
const B9600: crate::speed_t = 13u32;
```

### `B19200`

```rust
const B19200: crate::speed_t = 14u32;
```

### `B38400`

```rust
const B38400: crate::speed_t = 15u32;
```

### `EXTA`

```rust
const EXTA: crate::speed_t = 14u32;
```

### `EXTB`

```rust
const EXTB: crate::speed_t = 15u32;
```

### `B57600`

```rust
const B57600: crate::speed_t = 4_097u32;
```

### `B115200`

```rust
const B115200: crate::speed_t = 4_098u32;
```

### `B230400`

```rust
const B230400: crate::speed_t = 4_099u32;
```

### `B460800`

```rust
const B460800: crate::speed_t = 4_100u32;
```

### `B500000`

```rust
const B500000: crate::speed_t = 4_101u32;
```

### `B576000`

```rust
const B576000: crate::speed_t = 4_102u32;
```

### `B921600`

```rust
const B921600: crate::speed_t = 4_103u32;
```

### `B1000000`

```rust
const B1000000: crate::speed_t = 4_104u32;
```

### `B1152000`

```rust
const B1152000: crate::speed_t = 4_105u32;
```

### `B1500000`

```rust
const B1500000: crate::speed_t = 4_106u32;
```

### `B2000000`

```rust
const B2000000: crate::speed_t = 4_107u32;
```

### `B2500000`

```rust
const B2500000: crate::speed_t = 4_108u32;
```

### `B3000000`

```rust
const B3000000: crate::speed_t = 4_109u32;
```

### `B3500000`

```rust
const B3500000: crate::speed_t = 4_110u32;
```

### `B4000000`

```rust
const B4000000: crate::speed_t = 4_111u32;
```

### `VEOL`

```rust
const VEOL: usize = 11usize;
```

### `VEOL2`

```rust
const VEOL2: usize = 16usize;
```

### `VMIN`

```rust
const VMIN: usize = 6usize;
```

### `IEXTEN`

```rust
const IEXTEN: crate::tcflag_t = 32_768u32;
```

### `TOSTOP`

```rust
const TOSTOP: crate::tcflag_t = 256u32;
```

### `FLUSHO`

```rust
const FLUSHO: crate::tcflag_t = 4_096u32;
```

### `EXTPROC`

```rust
const EXTPROC: crate::tcflag_t = 65_536u32;
```

### `R15`

```rust
const R15: crate::c_int = 0i32;
```

### `R14`

```rust
const R14: crate::c_int = 1i32;
```

### `R13`

```rust
const R13: crate::c_int = 2i32;
```

### `R12`

```rust
const R12: crate::c_int = 3i32;
```

### `RBP`

```rust
const RBP: crate::c_int = 4i32;
```

### `RBX`

```rust
const RBX: crate::c_int = 5i32;
```

### `R11`

```rust
const R11: crate::c_int = 6i32;
```

### `R10`

```rust
const R10: crate::c_int = 7i32;
```

### `R9`

```rust
const R9: crate::c_int = 8i32;
```

### `R8`

```rust
const R8: crate::c_int = 9i32;
```

### `RAX`

```rust
const RAX: crate::c_int = 10i32;
```

### `RCX`

```rust
const RCX: crate::c_int = 11i32;
```

### `RDX`

```rust
const RDX: crate::c_int = 12i32;
```

### `RSI`

```rust
const RSI: crate::c_int = 13i32;
```

### `RDI`

```rust
const RDI: crate::c_int = 14i32;
```

### `ORIG_RAX`

```rust
const ORIG_RAX: crate::c_int = 15i32;
```

### `RIP`

```rust
const RIP: crate::c_int = 16i32;
```

### `CS`

```rust
const CS: crate::c_int = 17i32;
```

### `EFLAGS`

```rust
const EFLAGS: crate::c_int = 18i32;
```

### `RSP`

```rust
const RSP: crate::c_int = 19i32;
```

### `SS`

```rust
const SS: crate::c_int = 20i32;
```

### `FS_BASE`

```rust
const FS_BASE: crate::c_int = 21i32;
```

### `GS_BASE`

```rust
const GS_BASE: crate::c_int = 22i32;
```

### `DS`

```rust
const DS: crate::c_int = 23i32;
```

### `ES`

```rust
const ES: crate::c_int = 24i32;
```

### `FS`

```rust
const FS: crate::c_int = 25i32;
```

### `GS`

```rust
const GS: crate::c_int = 26i32;
```

### `REG_R8`

```rust
const REG_R8: crate::c_int = 0i32;
```

### `REG_R9`

```rust
const REG_R9: crate::c_int = 1i32;
```

### `REG_R10`

```rust
const REG_R10: crate::c_int = 2i32;
```

### `REG_R11`

```rust
const REG_R11: crate::c_int = 3i32;
```

### `REG_R12`

```rust
const REG_R12: crate::c_int = 4i32;
```

### `REG_R13`

```rust
const REG_R13: crate::c_int = 5i32;
```

### `REG_R14`

```rust
const REG_R14: crate::c_int = 6i32;
```

### `REG_R15`

```rust
const REG_R15: crate::c_int = 7i32;
```

### `REG_RDI`

```rust
const REG_RDI: crate::c_int = 8i32;
```

### `REG_RSI`

```rust
const REG_RSI: crate::c_int = 9i32;
```

### `REG_RBP`

```rust
const REG_RBP: crate::c_int = 10i32;
```

### `REG_RBX`

```rust
const REG_RBX: crate::c_int = 11i32;
```

### `REG_RDX`

```rust
const REG_RDX: crate::c_int = 12i32;
```

### `REG_RAX`

```rust
const REG_RAX: crate::c_int = 13i32;
```

### `REG_RCX`

```rust
const REG_RCX: crate::c_int = 14i32;
```

### `REG_RSP`

```rust
const REG_RSP: crate::c_int = 15i32;
```

### `REG_RIP`

```rust
const REG_RIP: crate::c_int = 16i32;
```

### `REG_EFL`

```rust
const REG_EFL: crate::c_int = 17i32;
```

### `REG_CSGSFS`

```rust
const REG_CSGSFS: crate::c_int = 18i32;
```

### `REG_ERR`

```rust
const REG_ERR: crate::c_int = 19i32;
```

### `REG_TRAPNO`

```rust
const REG_TRAPNO: crate::c_int = 20i32;
```

### `REG_OLDMASK`

```rust
const REG_OLDMASK: crate::c_int = 21i32;
```

### `REG_CR2`

```rust
const REG_CR2: crate::c_int = 22i32;
```

### `__SIZEOF_PTHREAD_MUTEX_T`

```rust
const __SIZEOF_PTHREAD_MUTEX_T: usize = 40usize;
```

### `__SIZEOF_PTHREAD_RWLOCK_T`

```rust
const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56usize;
```

### `__SIZEOF_PTHREAD_BARRIER_T`

```rust
const __SIZEOF_PTHREAD_BARRIER_T: usize = 32usize;
```

### `PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP`

```rust
const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

### `PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP`

```rust
const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

### `PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP`

```rust
const PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

### `SYS_read`

```rust
const SYS_read: crate::c_long = 0i64;
```

### `SYS_write`

```rust
const SYS_write: crate::c_long = 1i64;
```

### `SYS_open`

```rust
const SYS_open: crate::c_long = 2i64;
```

### `SYS_close`

```rust
const SYS_close: crate::c_long = 3i64;
```

### `SYS_stat`

```rust
const SYS_stat: crate::c_long = 4i64;
```

### `SYS_fstat`

```rust
const SYS_fstat: crate::c_long = 5i64;
```

### `SYS_lstat`

```rust
const SYS_lstat: crate::c_long = 6i64;
```

### `SYS_poll`

```rust
const SYS_poll: crate::c_long = 7i64;
```

### `SYS_lseek`

```rust
const SYS_lseek: crate::c_long = 8i64;
```

### `SYS_mmap`

```rust
const SYS_mmap: crate::c_long = 9i64;
```

### `SYS_mprotect`

```rust
const SYS_mprotect: crate::c_long = 10i64;
```

### `SYS_munmap`

```rust
const SYS_munmap: crate::c_long = 11i64;
```

### `SYS_brk`

```rust
const SYS_brk: crate::c_long = 12i64;
```

### `SYS_rt_sigaction`

```rust
const SYS_rt_sigaction: crate::c_long = 13i64;
```

### `SYS_rt_sigprocmask`

```rust
const SYS_rt_sigprocmask: crate::c_long = 14i64;
```

### `SYS_rt_sigreturn`

```rust
const SYS_rt_sigreturn: crate::c_long = 15i64;
```

### `SYS_ioctl`

```rust
const SYS_ioctl: crate::c_long = 16i64;
```

### `SYS_pread64`

```rust
const SYS_pread64: crate::c_long = 17i64;
```

### `SYS_pwrite64`

```rust
const SYS_pwrite64: crate::c_long = 18i64;
```

### `SYS_readv`

```rust
const SYS_readv: crate::c_long = 19i64;
```

### `SYS_writev`

```rust
const SYS_writev: crate::c_long = 20i64;
```

### `SYS_access`

```rust
const SYS_access: crate::c_long = 21i64;
```

### `SYS_pipe`

```rust
const SYS_pipe: crate::c_long = 22i64;
```

### `SYS_select`

```rust
const SYS_select: crate::c_long = 23i64;
```

### `SYS_sched_yield`

```rust
const SYS_sched_yield: crate::c_long = 24i64;
```

### `SYS_mremap`

```rust
const SYS_mremap: crate::c_long = 25i64;
```

### `SYS_msync`

```rust
const SYS_msync: crate::c_long = 26i64;
```

### `SYS_mincore`

```rust
const SYS_mincore: crate::c_long = 27i64;
```

### `SYS_madvise`

```rust
const SYS_madvise: crate::c_long = 28i64;
```

### `SYS_shmget`

```rust
const SYS_shmget: crate::c_long = 29i64;
```

### `SYS_shmat`

```rust
const SYS_shmat: crate::c_long = 30i64;
```

### `SYS_shmctl`

```rust
const SYS_shmctl: crate::c_long = 31i64;
```

### `SYS_dup`

```rust
const SYS_dup: crate::c_long = 32i64;
```

### `SYS_dup2`

```rust
const SYS_dup2: crate::c_long = 33i64;
```

### `SYS_pause`

```rust
const SYS_pause: crate::c_long = 34i64;
```

### `SYS_nanosleep`

```rust
const SYS_nanosleep: crate::c_long = 35i64;
```

### `SYS_getitimer`

```rust
const SYS_getitimer: crate::c_long = 36i64;
```

### `SYS_alarm`

```rust
const SYS_alarm: crate::c_long = 37i64;
```

### `SYS_setitimer`

```rust
const SYS_setitimer: crate::c_long = 38i64;
```

### `SYS_getpid`

```rust
const SYS_getpid: crate::c_long = 39i64;
```

### `SYS_sendfile`

```rust
const SYS_sendfile: crate::c_long = 40i64;
```

### `SYS_socket`

```rust
const SYS_socket: crate::c_long = 41i64;
```

### `SYS_connect`

```rust
const SYS_connect: crate::c_long = 42i64;
```

### `SYS_accept`

```rust
const SYS_accept: crate::c_long = 43i64;
```

### `SYS_sendto`

```rust
const SYS_sendto: crate::c_long = 44i64;
```

### `SYS_recvfrom`

```rust
const SYS_recvfrom: crate::c_long = 45i64;
```

### `SYS_sendmsg`

```rust
const SYS_sendmsg: crate::c_long = 46i64;
```

### `SYS_recvmsg`

```rust
const SYS_recvmsg: crate::c_long = 47i64;
```

### `SYS_shutdown`

```rust
const SYS_shutdown: crate::c_long = 48i64;
```

### `SYS_bind`

```rust
const SYS_bind: crate::c_long = 49i64;
```

### `SYS_listen`

```rust
const SYS_listen: crate::c_long = 50i64;
```

### `SYS_getsockname`

```rust
const SYS_getsockname: crate::c_long = 51i64;
```

### `SYS_getpeername`

```rust
const SYS_getpeername: crate::c_long = 52i64;
```

### `SYS_socketpair`

```rust
const SYS_socketpair: crate::c_long = 53i64;
```

### `SYS_setsockopt`

```rust
const SYS_setsockopt: crate::c_long = 54i64;
```

### `SYS_getsockopt`

```rust
const SYS_getsockopt: crate::c_long = 55i64;
```

### `SYS_clone`

```rust
const SYS_clone: crate::c_long = 56i64;
```

### `SYS_fork`

```rust
const SYS_fork: crate::c_long = 57i64;
```

### `SYS_vfork`

```rust
const SYS_vfork: crate::c_long = 58i64;
```

### `SYS_execve`

```rust
const SYS_execve: crate::c_long = 59i64;
```

### `SYS_exit`

```rust
const SYS_exit: crate::c_long = 60i64;
```

### `SYS_wait4`

```rust
const SYS_wait4: crate::c_long = 61i64;
```

### `SYS_kill`

```rust
const SYS_kill: crate::c_long = 62i64;
```

### `SYS_uname`

```rust
const SYS_uname: crate::c_long = 63i64;
```

### `SYS_semget`

```rust
const SYS_semget: crate::c_long = 64i64;
```

### `SYS_semop`

```rust
const SYS_semop: crate::c_long = 65i64;
```

### `SYS_semctl`

```rust
const SYS_semctl: crate::c_long = 66i64;
```

### `SYS_shmdt`

```rust
const SYS_shmdt: crate::c_long = 67i64;
```

### `SYS_msgget`

```rust
const SYS_msgget: crate::c_long = 68i64;
```

### `SYS_msgsnd`

```rust
const SYS_msgsnd: crate::c_long = 69i64;
```

### `SYS_msgrcv`

```rust
const SYS_msgrcv: crate::c_long = 70i64;
```

### `SYS_msgctl`

```rust
const SYS_msgctl: crate::c_long = 71i64;
```

### `SYS_fcntl`

```rust
const SYS_fcntl: crate::c_long = 72i64;
```

### `SYS_flock`

```rust
const SYS_flock: crate::c_long = 73i64;
```

### `SYS_fsync`

```rust
const SYS_fsync: crate::c_long = 74i64;
```

### `SYS_fdatasync`

```rust
const SYS_fdatasync: crate::c_long = 75i64;
```

### `SYS_truncate`

```rust
const SYS_truncate: crate::c_long = 76i64;
```

### `SYS_ftruncate`

```rust
const SYS_ftruncate: crate::c_long = 77i64;
```

### `SYS_getdents`

```rust
const SYS_getdents: crate::c_long = 78i64;
```

### `SYS_getcwd`

```rust
const SYS_getcwd: crate::c_long = 79i64;
```

### `SYS_chdir`

```rust
const SYS_chdir: crate::c_long = 80i64;
```

### `SYS_fchdir`

```rust
const SYS_fchdir: crate::c_long = 81i64;
```

### `SYS_rename`

```rust
const SYS_rename: crate::c_long = 82i64;
```

### `SYS_mkdir`

```rust
const SYS_mkdir: crate::c_long = 83i64;
```

### `SYS_rmdir`

```rust
const SYS_rmdir: crate::c_long = 84i64;
```

### `SYS_creat`

```rust
const SYS_creat: crate::c_long = 85i64;
```

### `SYS_link`

```rust
const SYS_link: crate::c_long = 86i64;
```

### `SYS_unlink`

```rust
const SYS_unlink: crate::c_long = 87i64;
```

### `SYS_symlink`

```rust
const SYS_symlink: crate::c_long = 88i64;
```

### `SYS_readlink`

```rust
const SYS_readlink: crate::c_long = 89i64;
```

### `SYS_chmod`

```rust
const SYS_chmod: crate::c_long = 90i64;
```

### `SYS_fchmod`

```rust
const SYS_fchmod: crate::c_long = 91i64;
```

### `SYS_chown`

```rust
const SYS_chown: crate::c_long = 92i64;
```

### `SYS_fchown`

```rust
const SYS_fchown: crate::c_long = 93i64;
```

### `SYS_lchown`

```rust
const SYS_lchown: crate::c_long = 94i64;
```

### `SYS_umask`

```rust
const SYS_umask: crate::c_long = 95i64;
```

### `SYS_gettimeofday`

```rust
const SYS_gettimeofday: crate::c_long = 96i64;
```

### `SYS_getrlimit`

```rust
const SYS_getrlimit: crate::c_long = 97i64;
```

### `SYS_getrusage`

```rust
const SYS_getrusage: crate::c_long = 98i64;
```

### `SYS_sysinfo`

```rust
const SYS_sysinfo: crate::c_long = 99i64;
```

### `SYS_times`

```rust
const SYS_times: crate::c_long = 100i64;
```

### `SYS_ptrace`

```rust
const SYS_ptrace: crate::c_long = 101i64;
```

### `SYS_getuid`

```rust
const SYS_getuid: crate::c_long = 102i64;
```

### `SYS_syslog`

```rust
const SYS_syslog: crate::c_long = 103i64;
```

### `SYS_getgid`

```rust
const SYS_getgid: crate::c_long = 104i64;
```

### `SYS_setuid`

```rust
const SYS_setuid: crate::c_long = 105i64;
```

### `SYS_setgid`

```rust
const SYS_setgid: crate::c_long = 106i64;
```

### `SYS_geteuid`

```rust
const SYS_geteuid: crate::c_long = 107i64;
```

### `SYS_getegid`

```rust
const SYS_getegid: crate::c_long = 108i64;
```

### `SYS_setpgid`

```rust
const SYS_setpgid: crate::c_long = 109i64;
```

### `SYS_getppid`

```rust
const SYS_getppid: crate::c_long = 110i64;
```

### `SYS_getpgrp`

```rust
const SYS_getpgrp: crate::c_long = 111i64;
```

### `SYS_setsid`

```rust
const SYS_setsid: crate::c_long = 112i64;
```

### `SYS_setreuid`

```rust
const SYS_setreuid: crate::c_long = 113i64;
```

### `SYS_setregid`

```rust
const SYS_setregid: crate::c_long = 114i64;
```

### `SYS_getgroups`

```rust
const SYS_getgroups: crate::c_long = 115i64;
```

### `SYS_setgroups`

```rust
const SYS_setgroups: crate::c_long = 116i64;
```

### `SYS_setresuid`

```rust
const SYS_setresuid: crate::c_long = 117i64;
```

### `SYS_getresuid`

```rust
const SYS_getresuid: crate::c_long = 118i64;
```

### `SYS_setresgid`

```rust
const SYS_setresgid: crate::c_long = 119i64;
```

### `SYS_getresgid`

```rust
const SYS_getresgid: crate::c_long = 120i64;
```

### `SYS_getpgid`

```rust
const SYS_getpgid: crate::c_long = 121i64;
```

### `SYS_setfsuid`

```rust
const SYS_setfsuid: crate::c_long = 122i64;
```

### `SYS_setfsgid`

```rust
const SYS_setfsgid: crate::c_long = 123i64;
```

### `SYS_getsid`

```rust
const SYS_getsid: crate::c_long = 124i64;
```

### `SYS_capget`

```rust
const SYS_capget: crate::c_long = 125i64;
```

### `SYS_capset`

```rust
const SYS_capset: crate::c_long = 126i64;
```

### `SYS_rt_sigpending`

```rust
const SYS_rt_sigpending: crate::c_long = 127i64;
```

### `SYS_rt_sigtimedwait`

```rust
const SYS_rt_sigtimedwait: crate::c_long = 128i64;
```

### `SYS_rt_sigqueueinfo`

```rust
const SYS_rt_sigqueueinfo: crate::c_long = 129i64;
```

### `SYS_rt_sigsuspend`

```rust
const SYS_rt_sigsuspend: crate::c_long = 130i64;
```

### `SYS_sigaltstack`

```rust
const SYS_sigaltstack: crate::c_long = 131i64;
```

### `SYS_utime`

```rust
const SYS_utime: crate::c_long = 132i64;
```

### `SYS_mknod`

```rust
const SYS_mknod: crate::c_long = 133i64;
```

### `SYS_uselib`

```rust
const SYS_uselib: crate::c_long = 134i64;
```

### `SYS_personality`

```rust
const SYS_personality: crate::c_long = 135i64;
```

### `SYS_ustat`

```rust
const SYS_ustat: crate::c_long = 136i64;
```

### `SYS_statfs`

```rust
const SYS_statfs: crate::c_long = 137i64;
```

### `SYS_fstatfs`

```rust
const SYS_fstatfs: crate::c_long = 138i64;
```

### `SYS_sysfs`

```rust
const SYS_sysfs: crate::c_long = 139i64;
```

### `SYS_getpriority`

```rust
const SYS_getpriority: crate::c_long = 140i64;
```

### `SYS_setpriority`

```rust
const SYS_setpriority: crate::c_long = 141i64;
```

### `SYS_sched_setparam`

```rust
const SYS_sched_setparam: crate::c_long = 142i64;
```

### `SYS_sched_getparam`

```rust
const SYS_sched_getparam: crate::c_long = 143i64;
```

### `SYS_sched_setscheduler`

```rust
const SYS_sched_setscheduler: crate::c_long = 144i64;
```

### `SYS_sched_getscheduler`

```rust
const SYS_sched_getscheduler: crate::c_long = 145i64;
```

### `SYS_sched_get_priority_max`

```rust
const SYS_sched_get_priority_max: crate::c_long = 146i64;
```

### `SYS_sched_get_priority_min`

```rust
const SYS_sched_get_priority_min: crate::c_long = 147i64;
```

### `SYS_sched_rr_get_interval`

```rust
const SYS_sched_rr_get_interval: crate::c_long = 148i64;
```

### `SYS_mlock`

```rust
const SYS_mlock: crate::c_long = 149i64;
```

### `SYS_munlock`

```rust
const SYS_munlock: crate::c_long = 150i64;
```

### `SYS_mlockall`

```rust
const SYS_mlockall: crate::c_long = 151i64;
```

### `SYS_munlockall`

```rust
const SYS_munlockall: crate::c_long = 152i64;
```

### `SYS_vhangup`

```rust
const SYS_vhangup: crate::c_long = 153i64;
```

### `SYS_modify_ldt`

```rust
const SYS_modify_ldt: crate::c_long = 154i64;
```

### `SYS_pivot_root`

```rust
const SYS_pivot_root: crate::c_long = 155i64;
```

### `SYS__sysctl`

```rust
const SYS__sysctl: crate::c_long = 156i64;
```

### `SYS_prctl`

```rust
const SYS_prctl: crate::c_long = 157i64;
```

### `SYS_arch_prctl`

```rust
const SYS_arch_prctl: crate::c_long = 158i64;
```

### `SYS_adjtimex`

```rust
const SYS_adjtimex: crate::c_long = 159i64;
```

### `SYS_setrlimit`

```rust
const SYS_setrlimit: crate::c_long = 160i64;
```

### `SYS_chroot`

```rust
const SYS_chroot: crate::c_long = 161i64;
```

### `SYS_sync`

```rust
const SYS_sync: crate::c_long = 162i64;
```

### `SYS_acct`

```rust
const SYS_acct: crate::c_long = 163i64;
```

### `SYS_settimeofday`

```rust
const SYS_settimeofday: crate::c_long = 164i64;
```

### `SYS_mount`

```rust
const SYS_mount: crate::c_long = 165i64;
```

### `SYS_umount2`

```rust
const SYS_umount2: crate::c_long = 166i64;
```

### `SYS_swapon`

```rust
const SYS_swapon: crate::c_long = 167i64;
```

### `SYS_swapoff`

```rust
const SYS_swapoff: crate::c_long = 168i64;
```

### `SYS_reboot`

```rust
const SYS_reboot: crate::c_long = 169i64;
```

### `SYS_sethostname`

```rust
const SYS_sethostname: crate::c_long = 170i64;
```

### `SYS_setdomainname`

```rust
const SYS_setdomainname: crate::c_long = 171i64;
```

### `SYS_iopl`

```rust
const SYS_iopl: crate::c_long = 172i64;
```

### `SYS_ioperm`

```rust
const SYS_ioperm: crate::c_long = 173i64;
```

### `SYS_create_module`

```rust
const SYS_create_module: crate::c_long = 174i64;
```

### `SYS_init_module`

```rust
const SYS_init_module: crate::c_long = 175i64;
```

### `SYS_delete_module`

```rust
const SYS_delete_module: crate::c_long = 176i64;
```

### `SYS_get_kernel_syms`

```rust
const SYS_get_kernel_syms: crate::c_long = 177i64;
```

### `SYS_query_module`

```rust
const SYS_query_module: crate::c_long = 178i64;
```

### `SYS_quotactl`

```rust
const SYS_quotactl: crate::c_long = 179i64;
```

### `SYS_nfsservctl`

```rust
const SYS_nfsservctl: crate::c_long = 180i64;
```

### `SYS_getpmsg`

```rust
const SYS_getpmsg: crate::c_long = 181i64;
```

### `SYS_putpmsg`

```rust
const SYS_putpmsg: crate::c_long = 182i64;
```

### `SYS_afs_syscall`

```rust
const SYS_afs_syscall: crate::c_long = 183i64;
```

### `SYS_tuxcall`

```rust
const SYS_tuxcall: crate::c_long = 184i64;
```

### `SYS_security`

```rust
const SYS_security: crate::c_long = 185i64;
```

### `SYS_gettid`

```rust
const SYS_gettid: crate::c_long = 186i64;
```

### `SYS_readahead`

```rust
const SYS_readahead: crate::c_long = 187i64;
```

### `SYS_setxattr`

```rust
const SYS_setxattr: crate::c_long = 188i64;
```

### `SYS_lsetxattr`

```rust
const SYS_lsetxattr: crate::c_long = 189i64;
```

### `SYS_fsetxattr`

```rust
const SYS_fsetxattr: crate::c_long = 190i64;
```

### `SYS_getxattr`

```rust
const SYS_getxattr: crate::c_long = 191i64;
```

### `SYS_lgetxattr`

```rust
const SYS_lgetxattr: crate::c_long = 192i64;
```

### `SYS_fgetxattr`

```rust
const SYS_fgetxattr: crate::c_long = 193i64;
```

### `SYS_listxattr`

```rust
const SYS_listxattr: crate::c_long = 194i64;
```

### `SYS_llistxattr`

```rust
const SYS_llistxattr: crate::c_long = 195i64;
```

### `SYS_flistxattr`

```rust
const SYS_flistxattr: crate::c_long = 196i64;
```

### `SYS_removexattr`

```rust
const SYS_removexattr: crate::c_long = 197i64;
```

### `SYS_lremovexattr`

```rust
const SYS_lremovexattr: crate::c_long = 198i64;
```

### `SYS_fremovexattr`

```rust
const SYS_fremovexattr: crate::c_long = 199i64;
```

### `SYS_tkill`

```rust
const SYS_tkill: crate::c_long = 200i64;
```

### `SYS_time`

```rust
const SYS_time: crate::c_long = 201i64;
```

### `SYS_futex`

```rust
const SYS_futex: crate::c_long = 202i64;
```

### `SYS_sched_setaffinity`

```rust
const SYS_sched_setaffinity: crate::c_long = 203i64;
```

### `SYS_sched_getaffinity`

```rust
const SYS_sched_getaffinity: crate::c_long = 204i64;
```

### `SYS_set_thread_area`

```rust
const SYS_set_thread_area: crate::c_long = 205i64;
```

### `SYS_io_setup`

```rust
const SYS_io_setup: crate::c_long = 206i64;
```

### `SYS_io_destroy`

```rust
const SYS_io_destroy: crate::c_long = 207i64;
```

### `SYS_io_getevents`

```rust
const SYS_io_getevents: crate::c_long = 208i64;
```

### `SYS_io_submit`

```rust
const SYS_io_submit: crate::c_long = 209i64;
```

### `SYS_io_cancel`

```rust
const SYS_io_cancel: crate::c_long = 210i64;
```

### `SYS_get_thread_area`

```rust
const SYS_get_thread_area: crate::c_long = 211i64;
```

### `SYS_lookup_dcookie`

```rust
const SYS_lookup_dcookie: crate::c_long = 212i64;
```

### `SYS_epoll_create`

```rust
const SYS_epoll_create: crate::c_long = 213i64;
```

### `SYS_epoll_ctl_old`

```rust
const SYS_epoll_ctl_old: crate::c_long = 214i64;
```

### `SYS_epoll_wait_old`

```rust
const SYS_epoll_wait_old: crate::c_long = 215i64;
```

### `SYS_remap_file_pages`

```rust
const SYS_remap_file_pages: crate::c_long = 216i64;
```

### `SYS_getdents64`

```rust
const SYS_getdents64: crate::c_long = 217i64;
```

### `SYS_set_tid_address`

```rust
const SYS_set_tid_address: crate::c_long = 218i64;
```

### `SYS_restart_syscall`

```rust
const SYS_restart_syscall: crate::c_long = 219i64;
```

### `SYS_semtimedop`

```rust
const SYS_semtimedop: crate::c_long = 220i64;
```

### `SYS_fadvise64`

```rust
const SYS_fadvise64: crate::c_long = 221i64;
```

### `SYS_timer_create`

```rust
const SYS_timer_create: crate::c_long = 222i64;
```

### `SYS_timer_settime`

```rust
const SYS_timer_settime: crate::c_long = 223i64;
```

### `SYS_timer_gettime`

```rust
const SYS_timer_gettime: crate::c_long = 224i64;
```

### `SYS_timer_getoverrun`

```rust
const SYS_timer_getoverrun: crate::c_long = 225i64;
```

### `SYS_timer_delete`

```rust
const SYS_timer_delete: crate::c_long = 226i64;
```

### `SYS_clock_settime`

```rust
const SYS_clock_settime: crate::c_long = 227i64;
```

### `SYS_clock_gettime`

```rust
const SYS_clock_gettime: crate::c_long = 228i64;
```

### `SYS_clock_getres`

```rust
const SYS_clock_getres: crate::c_long = 229i64;
```

### `SYS_clock_nanosleep`

```rust
const SYS_clock_nanosleep: crate::c_long = 230i64;
```

### `SYS_exit_group`

```rust
const SYS_exit_group: crate::c_long = 231i64;
```

### `SYS_epoll_wait`

```rust
const SYS_epoll_wait: crate::c_long = 232i64;
```

### `SYS_epoll_ctl`

```rust
const SYS_epoll_ctl: crate::c_long = 233i64;
```

### `SYS_tgkill`

```rust
const SYS_tgkill: crate::c_long = 234i64;
```

### `SYS_utimes`

```rust
const SYS_utimes: crate::c_long = 235i64;
```

### `SYS_vserver`

```rust
const SYS_vserver: crate::c_long = 236i64;
```

### `SYS_mbind`

```rust
const SYS_mbind: crate::c_long = 237i64;
```

### `SYS_set_mempolicy`

```rust
const SYS_set_mempolicy: crate::c_long = 238i64;
```

### `SYS_get_mempolicy`

```rust
const SYS_get_mempolicy: crate::c_long = 239i64;
```

### `SYS_mq_open`

```rust
const SYS_mq_open: crate::c_long = 240i64;
```

### `SYS_mq_unlink`

```rust
const SYS_mq_unlink: crate::c_long = 241i64;
```

### `SYS_mq_timedsend`

```rust
const SYS_mq_timedsend: crate::c_long = 242i64;
```

### `SYS_mq_timedreceive`

```rust
const SYS_mq_timedreceive: crate::c_long = 243i64;
```

### `SYS_mq_notify`

```rust
const SYS_mq_notify: crate::c_long = 244i64;
```

### `SYS_mq_getsetattr`

```rust
const SYS_mq_getsetattr: crate::c_long = 245i64;
```

### `SYS_kexec_load`

```rust
const SYS_kexec_load: crate::c_long = 246i64;
```

### `SYS_waitid`

```rust
const SYS_waitid: crate::c_long = 247i64;
```

### `SYS_add_key`

```rust
const SYS_add_key: crate::c_long = 248i64;
```

### `SYS_request_key`

```rust
const SYS_request_key: crate::c_long = 249i64;
```

### `SYS_keyctl`

```rust
const SYS_keyctl: crate::c_long = 250i64;
```

### `SYS_ioprio_set`

```rust
const SYS_ioprio_set: crate::c_long = 251i64;
```

### `SYS_ioprio_get`

```rust
const SYS_ioprio_get: crate::c_long = 252i64;
```

### `SYS_inotify_init`

```rust
const SYS_inotify_init: crate::c_long = 253i64;
```

### `SYS_inotify_add_watch`

```rust
const SYS_inotify_add_watch: crate::c_long = 254i64;
```

### `SYS_inotify_rm_watch`

```rust
const SYS_inotify_rm_watch: crate::c_long = 255i64;
```

### `SYS_migrate_pages`

```rust
const SYS_migrate_pages: crate::c_long = 256i64;
```

### `SYS_openat`

```rust
const SYS_openat: crate::c_long = 257i64;
```

### `SYS_mkdirat`

```rust
const SYS_mkdirat: crate::c_long = 258i64;
```

### `SYS_mknodat`

```rust
const SYS_mknodat: crate::c_long = 259i64;
```

### `SYS_fchownat`

```rust
const SYS_fchownat: crate::c_long = 260i64;
```

### `SYS_futimesat`

```rust
const SYS_futimesat: crate::c_long = 261i64;
```

### `SYS_newfstatat`

```rust
const SYS_newfstatat: crate::c_long = 262i64;
```

### `SYS_unlinkat`

```rust
const SYS_unlinkat: crate::c_long = 263i64;
```

### `SYS_renameat`

```rust
const SYS_renameat: crate::c_long = 264i64;
```

### `SYS_linkat`

```rust
const SYS_linkat: crate::c_long = 265i64;
```

### `SYS_symlinkat`

```rust
const SYS_symlinkat: crate::c_long = 266i64;
```

### `SYS_readlinkat`

```rust
const SYS_readlinkat: crate::c_long = 267i64;
```

### `SYS_fchmodat`

```rust
const SYS_fchmodat: crate::c_long = 268i64;
```

### `SYS_faccessat`

```rust
const SYS_faccessat: crate::c_long = 269i64;
```

### `SYS_pselect6`

```rust
const SYS_pselect6: crate::c_long = 270i64;
```

### `SYS_ppoll`

```rust
const SYS_ppoll: crate::c_long = 271i64;
```

### `SYS_unshare`

```rust
const SYS_unshare: crate::c_long = 272i64;
```

### `SYS_set_robust_list`

```rust
const SYS_set_robust_list: crate::c_long = 273i64;
```

### `SYS_get_robust_list`

```rust
const SYS_get_robust_list: crate::c_long = 274i64;
```

### `SYS_splice`

```rust
const SYS_splice: crate::c_long = 275i64;
```

### `SYS_tee`

```rust
const SYS_tee: crate::c_long = 276i64;
```

### `SYS_sync_file_range`

```rust
const SYS_sync_file_range: crate::c_long = 277i64;
```

### `SYS_vmsplice`

```rust
const SYS_vmsplice: crate::c_long = 278i64;
```

### `SYS_move_pages`

```rust
const SYS_move_pages: crate::c_long = 279i64;
```

### `SYS_utimensat`

```rust
const SYS_utimensat: crate::c_long = 280i64;
```

### `SYS_epoll_pwait`

```rust
const SYS_epoll_pwait: crate::c_long = 281i64;
```

### `SYS_signalfd`

```rust
const SYS_signalfd: crate::c_long = 282i64;
```

### `SYS_timerfd_create`

```rust
const SYS_timerfd_create: crate::c_long = 283i64;
```

### `SYS_eventfd`

```rust
const SYS_eventfd: crate::c_long = 284i64;
```

### `SYS_fallocate`

```rust
const SYS_fallocate: crate::c_long = 285i64;
```

### `SYS_timerfd_settime`

```rust
const SYS_timerfd_settime: crate::c_long = 286i64;
```

### `SYS_timerfd_gettime`

```rust
const SYS_timerfd_gettime: crate::c_long = 287i64;
```

### `SYS_accept4`

```rust
const SYS_accept4: crate::c_long = 288i64;
```

### `SYS_signalfd4`

```rust
const SYS_signalfd4: crate::c_long = 289i64;
```

### `SYS_eventfd2`

```rust
const SYS_eventfd2: crate::c_long = 290i64;
```

### `SYS_epoll_create1`

```rust
const SYS_epoll_create1: crate::c_long = 291i64;
```

### `SYS_dup3`

```rust
const SYS_dup3: crate::c_long = 292i64;
```

### `SYS_pipe2`

```rust
const SYS_pipe2: crate::c_long = 293i64;
```

### `SYS_inotify_init1`

```rust
const SYS_inotify_init1: crate::c_long = 294i64;
```

### `SYS_preadv`

```rust
const SYS_preadv: crate::c_long = 295i64;
```

### `SYS_pwritev`

```rust
const SYS_pwritev: crate::c_long = 296i64;
```

### `SYS_rt_tgsigqueueinfo`

```rust
const SYS_rt_tgsigqueueinfo: crate::c_long = 297i64;
```

### `SYS_perf_event_open`

```rust
const SYS_perf_event_open: crate::c_long = 298i64;
```

### `SYS_recvmmsg`

```rust
const SYS_recvmmsg: crate::c_long = 299i64;
```

### `SYS_fanotify_init`

```rust
const SYS_fanotify_init: crate::c_long = 300i64;
```

### `SYS_fanotify_mark`

```rust
const SYS_fanotify_mark: crate::c_long = 301i64;
```

### `SYS_prlimit64`

```rust
const SYS_prlimit64: crate::c_long = 302i64;
```

### `SYS_name_to_handle_at`

```rust
const SYS_name_to_handle_at: crate::c_long = 303i64;
```

### `SYS_open_by_handle_at`

```rust
const SYS_open_by_handle_at: crate::c_long = 304i64;
```

### `SYS_clock_adjtime`

```rust
const SYS_clock_adjtime: crate::c_long = 305i64;
```

### `SYS_syncfs`

```rust
const SYS_syncfs: crate::c_long = 306i64;
```

### `SYS_sendmmsg`

```rust
const SYS_sendmmsg: crate::c_long = 307i64;
```

### `SYS_setns`

```rust
const SYS_setns: crate::c_long = 308i64;
```

### `SYS_getcpu`

```rust
const SYS_getcpu: crate::c_long = 309i64;
```

### `SYS_process_vm_readv`

```rust
const SYS_process_vm_readv: crate::c_long = 310i64;
```

### `SYS_process_vm_writev`

```rust
const SYS_process_vm_writev: crate::c_long = 311i64;
```

### `SYS_kcmp`

```rust
const SYS_kcmp: crate::c_long = 312i64;
```

### `SYS_finit_module`

```rust
const SYS_finit_module: crate::c_long = 313i64;
```

### `SYS_sched_setattr`

```rust
const SYS_sched_setattr: crate::c_long = 314i64;
```

### `SYS_sched_getattr`

```rust
const SYS_sched_getattr: crate::c_long = 315i64;
```

### `SYS_renameat2`

```rust
const SYS_renameat2: crate::c_long = 316i64;
```

### `SYS_seccomp`

```rust
const SYS_seccomp: crate::c_long = 317i64;
```

### `SYS_getrandom`

```rust
const SYS_getrandom: crate::c_long = 318i64;
```

### `SYS_memfd_create`

```rust
const SYS_memfd_create: crate::c_long = 319i64;
```

### `SYS_kexec_file_load`

```rust
const SYS_kexec_file_load: crate::c_long = 320i64;
```

### `SYS_bpf`

```rust
const SYS_bpf: crate::c_long = 321i64;
```

### `SYS_execveat`

```rust
const SYS_execveat: crate::c_long = 322i64;
```

### `SYS_userfaultfd`

```rust
const SYS_userfaultfd: crate::c_long = 323i64;
```

### `SYS_membarrier`

```rust
const SYS_membarrier: crate::c_long = 324i64;
```

### `SYS_mlock2`

```rust
const SYS_mlock2: crate::c_long = 325i64;
```

### `SYS_copy_file_range`

```rust
const SYS_copy_file_range: crate::c_long = 326i64;
```

### `SYS_preadv2`

```rust
const SYS_preadv2: crate::c_long = 327i64;
```

### `SYS_pwritev2`

```rust
const SYS_pwritev2: crate::c_long = 328i64;
```

### `SYS_pkey_mprotect`

```rust
const SYS_pkey_mprotect: crate::c_long = 329i64;
```

### `SYS_pkey_alloc`

```rust
const SYS_pkey_alloc: crate::c_long = 330i64;
```

### `SYS_pkey_free`

```rust
const SYS_pkey_free: crate::c_long = 331i64;
```

### `SYS_statx`

```rust
const SYS_statx: crate::c_long = 332i64;
```

### `SYS_rseq`

```rust
const SYS_rseq: crate::c_long = 334i64;
```

### `SYS_pidfd_send_signal`

```rust
const SYS_pidfd_send_signal: crate::c_long = 424i64;
```

### `SYS_io_uring_setup`

```rust
const SYS_io_uring_setup: crate::c_long = 425i64;
```

### `SYS_io_uring_enter`

```rust
const SYS_io_uring_enter: crate::c_long = 426i64;
```

### `SYS_io_uring_register`

```rust
const SYS_io_uring_register: crate::c_long = 427i64;
```

### `SYS_open_tree`

```rust
const SYS_open_tree: crate::c_long = 428i64;
```

### `SYS_move_mount`

```rust
const SYS_move_mount: crate::c_long = 429i64;
```

### `SYS_fsopen`

```rust
const SYS_fsopen: crate::c_long = 430i64;
```

### `SYS_fsconfig`

```rust
const SYS_fsconfig: crate::c_long = 431i64;
```

### `SYS_fsmount`

```rust
const SYS_fsmount: crate::c_long = 432i64;
```

### `SYS_fspick`

```rust
const SYS_fspick: crate::c_long = 433i64;
```

### `SYS_pidfd_open`

```rust
const SYS_pidfd_open: crate::c_long = 434i64;
```

### `SYS_clone3`

```rust
const SYS_clone3: crate::c_long = 435i64;
```

### `SYS_close_range`

```rust
const SYS_close_range: crate::c_long = 436i64;
```

### `SYS_openat2`

```rust
const SYS_openat2: crate::c_long = 437i64;
```

### `SYS_pidfd_getfd`

```rust
const SYS_pidfd_getfd: crate::c_long = 438i64;
```

### `SYS_faccessat2`

```rust
const SYS_faccessat2: crate::c_long = 439i64;
```

### `SYS_process_madvise`

```rust
const SYS_process_madvise: crate::c_long = 440i64;
```

### `SYS_epoll_pwait2`

```rust
const SYS_epoll_pwait2: crate::c_long = 441i64;
```

### `SYS_mount_setattr`

```rust
const SYS_mount_setattr: crate::c_long = 442i64;
```

### `SYS_quotactl_fd`

```rust
const SYS_quotactl_fd: crate::c_long = 443i64;
```

### `SYS_landlock_create_ruleset`

```rust
const SYS_landlock_create_ruleset: crate::c_long = 444i64;
```

### `SYS_landlock_add_rule`

```rust
const SYS_landlock_add_rule: crate::c_long = 445i64;
```

### `SYS_landlock_restrict_self`

```rust
const SYS_landlock_restrict_self: crate::c_long = 446i64;
```

### `SYS_memfd_secret`

```rust
const SYS_memfd_secret: crate::c_long = 447i64;
```

### `SYS_process_mrelease`

```rust
const SYS_process_mrelease: crate::c_long = 448i64;
```

### `SYS_futex_waitv`

```rust
const SYS_futex_waitv: crate::c_long = 449i64;
```

### `SYS_set_mempolicy_home_node`

```rust
const SYS_set_mempolicy_home_node: crate::c_long = 450i64;
```

### `SYS_fchmodat2`

```rust
const SYS_fchmodat2: crate::c_long = 452i64;
```

### `SYS_mseal`

```rust
const SYS_mseal: crate::c_long = 462i64;
```

