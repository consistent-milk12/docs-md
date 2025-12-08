*[libc](../../../../../index.md) / [unix](../../../../index.md) / [linux_like](../../../index.md) / [linux](../../index.md) / [gnu](../index.md) / [b64](index.md)*

---

# Module `b64`

64-bit specific definitions for linux-like values

## Modules

- [`x86_64`](x86_64/index.md) - x86_64-specific definitions for 64-bit linux-like values
- [`not_x32`](not_x32/index.md) - 

## Structs

### `sigset_t`

```rust
struct sigset_t {
    __val: [u64; 16],
}
```

#### Trait Implementations

##### `impl Clone for sigset_t`

- `fn clone(self: &Self) -> sigset_t` — [`sigset_t`](../index.md)

##### `impl Copy for sigset_t`

##### `impl Debug for sigset_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sysinfo`

```rust
struct sysinfo {
    pub uptime: i64,
    pub loads: [u64; 3],
    pub totalram: u64,
    pub freeram: u64,
    pub sharedram: u64,
    pub bufferram: u64,
    pub totalswap: u64,
    pub freeswap: u64,
    pub procs: crate::c_ushort,
    pub pad: crate::c_ushort,
    pub totalhigh: u64,
    pub freehigh: u64,
    pub mem_unit: crate::c_uint,
    pub _f: [crate::c_char; 0],
}
```

#### Trait Implementations

##### `impl Clone for sysinfo`

- `fn clone(self: &Self) -> sysinfo` — [`sysinfo`](../index.md)

##### `impl Copy for sysinfo`

##### `impl Debug for sysinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `msqid_ds`

```rust
struct msqid_ds {
    pub msg_perm: crate::ipc_perm,
    pub msg_stime: crate::time_t,
    pub msg_rtime: crate::time_t,
    pub msg_ctime: crate::time_t,
    pub __msg_cbytes: u64,
    pub msg_qnum: crate::msgqnum_t,
    pub msg_qbytes: crate::msglen_t,
    pub msg_lspid: crate::pid_t,
    pub msg_lrpid: crate::pid_t,
    __glibc_reserved4: u64,
    __glibc_reserved5: u64,
}
```

#### Trait Implementations

##### `impl Clone for msqid_ds`

- `fn clone(self: &Self) -> msqid_ds` — [`msqid_ds`](../index.md)

##### `impl Copy for msqid_ds`

##### `impl Debug for msqid_ds`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `semid_ds`

```rust
struct semid_ds {
    pub sem_perm: ipc_perm,
    pub sem_otime: crate::time_t,
    __reserved: crate::types::Padding<crate::__syscall_ulong_t>,
    pub sem_ctime: crate::time_t,
    __reserved2: crate::types::Padding<crate::__syscall_ulong_t>,
    pub sem_nsems: crate::__syscall_ulong_t,
    __glibc_reserved3: crate::__syscall_ulong_t,
    __glibc_reserved4: crate::__syscall_ulong_t,
}
```

#### Trait Implementations

##### `impl Clone for semid_ds`

- `fn clone(self: &Self) -> semid_ds` — [`semid_ds`](../index.md)

##### `impl Copy for semid_ds`

##### `impl Debug for semid_ds`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `timex`

```rust
struct timex {
    pub modes: crate::c_uint,
    pub offset: crate::c_long,
    pub freq: crate::c_long,
    pub maxerror: crate::c_long,
    pub esterror: crate::c_long,
    pub status: crate::c_int,
    pub constant: crate::c_long,
    pub precision: crate::c_long,
    pub tolerance: crate::c_long,
    pub time: crate::timeval,
    pub tick: crate::c_long,
    pub ppsfreq: crate::c_long,
    pub jitter: crate::c_long,
    pub shift: crate::c_int,
    pub stabil: crate::c_long,
    pub jitcnt: crate::c_long,
    pub calcnt: crate::c_long,
    pub errcnt: crate::c_long,
    pub stbcnt: crate::c_long,
    pub tai: crate::c_int,
    pub __unused1: i32,
    pub __unused2: i32,
    pub __unused3: i32,
    pub __unused4: i32,
    pub __unused5: i32,
    pub __unused6: i32,
    pub __unused7: i32,
    pub __unused8: i32,
    pub __unused9: i32,
    pub __unused10: i32,
    pub __unused11: i32,
}
```

#### Trait Implementations

##### `impl Clone for timex`

- `fn clone(self: &Self) -> timex` — [`timex`](../index.md)

##### `impl Copy for timex`

##### `impl Debug for timex`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn clone(self: &Self) -> sigaction` — [`sigaction`](#sigaction)

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

- `fn clone(self: &Self) -> statfs` — [`statfs`](#statfs)

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

- `fn clone(self: &Self) -> flock` — [`flock`](#flock)

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

- `fn clone(self: &Self) -> flock64` — [`flock64`](#flock64)

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

- `unsafe fn sifields(self: &Self) -> &sifields` — [`sifields`](../../../../../index.md)

- `unsafe fn si_pid(self: &Self) -> crate::pid_t` — [`pid_t`](../../../../../index.md)

- `unsafe fn si_uid(self: &Self) -> crate::uid_t` — [`uid_t`](../../../../../index.md)

- `unsafe fn si_status(self: &Self) -> c_int` — [`c_int`](../../../../../index.md)

- `unsafe fn si_utime(self: &Self) -> c_long` — [`c_long`](../../../../../index.md)

- `unsafe fn si_stime(self: &Self) -> c_long` — [`c_long`](../../../../../index.md)

#### Trait Implementations

##### `impl Clone for siginfo_t`

- `fn clone(self: &Self) -> siginfo_t` — [`siginfo_t`](#siginfo-t)

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

- `fn clone(self: &Self) -> stack_t` — [`stack_t`](#stack-t)

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

- `fn clone(self: &Self) -> stat` — [`stat`](#stat)

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

- `fn clone(self: &Self) -> stat64` — [`stat64`](#stat64)

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

- `fn clone(self: &Self) -> statfs64` — [`statfs64`](#statfs64)

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

- `fn clone(self: &Self) -> statvfs64` — [`statvfs64`](#statvfs64)

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

- `fn clone(self: &Self) -> pthread_attr_t` — [`pthread_attr_t`](#pthread-attr-t)

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

- `fn clone(self: &Self) -> _libc_fpxreg` — [`_libc_fpxreg`](#libc-fpxreg)

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

- `fn clone(self: &Self) -> _libc_xmmreg` — [`_libc_xmmreg`](#libc-xmmreg)

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

- `fn clone(self: &Self) -> _libc_fpstate` — [`_libc_fpstate`](#libc-fpstate)

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

- `fn clone(self: &Self) -> user_regs_struct` — [`user_regs_struct`](#user-regs-struct)

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

- `fn clone(self: &Self) -> user` — [`user`](#user)

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

- `fn clone(self: &Self) -> mcontext_t` — [`mcontext_t`](#mcontext-t)

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

- `fn clone(self: &Self) -> ipc_perm` — [`ipc_perm`](#ipc-perm)

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

- `fn clone(self: &Self) -> shmid_ds` — [`shmid_ds`](#shmid-ds)

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

- `fn clone(self: &Self) -> ptrace_rseq_configuration` — [`ptrace_rseq_configuration`](#ptrace-rseq-configuration)

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

- `fn clone(self: &Self) -> clone_args` — [`clone_args`](#clone-args)

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

- `fn clone(self: &Self) -> user_fpregs_struct` — [`user_fpregs_struct`](#user-fpregs-struct)

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

- `fn clone(self: &Self) -> ucontext_t` — [`ucontext_t`](#ucontext-t)

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

- `fn clone(self: &Self) -> max_align_t` — [`max_align_t`](#max-align-t)

##### `impl Copy for max_align_t`

##### `impl Debug for max_align_t`

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

## Type Aliases

### `ino_t`

```rust
type ino_t = u64;
```

### `off_t`

```rust
type off_t = i64;
```

### `blkcnt_t`

```rust
type blkcnt_t = i64;
```

### `shmatt_t`

```rust
type shmatt_t = u64;
```

### `msgqnum_t`

```rust
type msgqnum_t = u64;
```

### `msglen_t`

```rust
type msglen_t = u64;
```

### `fsblkcnt_t`

```rust
type fsblkcnt_t = u64;
```

### `fsfilcnt_t`

```rust
type fsfilcnt_t = u64;
```

### `rlim_t`

```rust
type rlim_t = u64;
```

### `__syscall_ulong_t`

```rust
type __syscall_ulong_t = crate::c_ulong;
```

### `__fsword_t`

```rust
type __fsword_t = i64;
```

### `clock_t`

```rust
type clock_t = i64;
```

### `time_t`

```rust
type time_t = i64;
```

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

### `__SIZEOF_PTHREAD_RWLOCKATTR_T`

```rust
const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8usize;
```

### `O_LARGEFILE`

```rust
const O_LARGEFILE: crate::c_int = 0i32;
```

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

