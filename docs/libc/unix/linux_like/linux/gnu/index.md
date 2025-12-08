*[libc](../../../../index.md) / [unix](../../../index.md) / [linux_like](../../index.md) / [linux](../index.md) / [gnu](index.md)*

---

# Module `gnu`

## Modules

- [`b64`](b64/index.md) - 64-bit specific definitions for linux-like values
- [`x86_64`](x86_64/index.md) - x86_64-specific definitions for 64-bit linux-like values

## Structs

### `aiocb`

```rust
struct aiocb {
    pub aio_fildes: crate::c_int,
    pub aio_lio_opcode: crate::c_int,
    pub aio_reqprio: crate::c_int,
    pub aio_buf: *mut crate::c_void,
    pub aio_nbytes: crate::size_t,
    pub aio_sigevent: crate::sigevent,
    __next_prio: *mut aiocb,
    __abs_prio: crate::c_int,
    __policy: crate::c_int,
    __error_code: crate::c_int,
    __return_value: crate::ssize_t,
    pub aio_offset: off_t,
    __glibc_reserved: [crate::c_char; 32],
}
```

#### Trait Implementations

##### `impl Clone for aiocb`

- `fn clone(self: &Self) -> aiocb` — [`aiocb`](../index.md)

##### `impl Copy for aiocb`

##### `impl Debug for aiocb`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__exit_status`

```rust
struct __exit_status {
    pub e_termination: crate::c_short,
    pub e_exit: crate::c_short,
}
```

#### Trait Implementations

##### `impl Clone for __exit_status`

- `fn clone(self: &Self) -> __exit_status` — [`__exit_status`](../index.md)

##### `impl Copy for __exit_status`

##### `impl Debug for __exit_status`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__timeval`

```rust
struct __timeval {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
```

#### Trait Implementations

##### `impl Clone for __timeval`

- `fn clone(self: &Self) -> __timeval` — [`__timeval`](../index.md)

##### `impl Copy for __timeval`

##### `impl Debug for __timeval`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `glob64_t`

```rust
struct glob64_t {
    pub gl_pathc: crate::size_t,
    pub gl_pathv: *mut *mut crate::c_char,
    pub gl_offs: crate::size_t,
    pub gl_flags: crate::c_int,
    __unused1: crate::types::Padding<*mut crate::c_void>,
    __unused2: crate::types::Padding<*mut crate::c_void>,
    __unused3: crate::types::Padding<*mut crate::c_void>,
    __unused4: crate::types::Padding<*mut crate::c_void>,
    __unused5: crate::types::Padding<*mut crate::c_void>,
}
```

#### Trait Implementations

##### `impl Clone for glob64_t`

- `fn clone(self: &Self) -> glob64_t` — [`glob64_t`](../index.md)

##### `impl Copy for glob64_t`

##### `impl Debug for glob64_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `msghdr`

```rust
struct msghdr {
    pub msg_name: *mut crate::c_void,
    pub msg_namelen: crate::socklen_t,
    pub msg_iov: *mut crate::iovec,
    pub msg_iovlen: crate::size_t,
    pub msg_control: *mut crate::c_void,
    pub msg_controllen: crate::size_t,
    pub msg_flags: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for msghdr`

- `fn clone(self: &Self) -> msghdr` — [`msghdr`](../index.md)

##### `impl Copy for msghdr`

##### `impl Debug for msghdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `cmsghdr`

```rust
struct cmsghdr {
    pub cmsg_len: crate::size_t,
    pub cmsg_level: crate::c_int,
    pub cmsg_type: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for cmsghdr`

- `fn clone(self: &Self) -> cmsghdr` — [`cmsghdr`](../index.md)

##### `impl Copy for cmsghdr`

##### `impl Debug for cmsghdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `termios`

```rust
struct termios {
    pub c_iflag: crate::tcflag_t,
    pub c_oflag: crate::tcflag_t,
    pub c_cflag: crate::tcflag_t,
    pub c_lflag: crate::tcflag_t,
    pub c_line: crate::cc_t,
    pub c_cc: [crate::cc_t; 32],
    pub c_ispeed: crate::speed_t,
    pub c_ospeed: crate::speed_t,
}
```

#### Trait Implementations

##### `impl Clone for termios`

- `fn clone(self: &Self) -> termios` — [`termios`](../index.md)

##### `impl Copy for termios`

##### `impl Debug for termios`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `mallinfo`

```rust
struct mallinfo {
    pub arena: crate::c_int,
    pub ordblks: crate::c_int,
    pub smblks: crate::c_int,
    pub hblks: crate::c_int,
    pub hblkhd: crate::c_int,
    pub usmblks: crate::c_int,
    pub fsmblks: crate::c_int,
    pub uordblks: crate::c_int,
    pub fordblks: crate::c_int,
    pub keepcost: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for mallinfo`

- `fn clone(self: &Self) -> mallinfo` — [`mallinfo`](../index.md)

##### `impl Copy for mallinfo`

##### `impl Debug for mallinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `mallinfo2`

```rust
struct mallinfo2 {
    pub arena: crate::size_t,
    pub ordblks: crate::size_t,
    pub smblks: crate::size_t,
    pub hblks: crate::size_t,
    pub hblkhd: crate::size_t,
    pub usmblks: crate::size_t,
    pub fsmblks: crate::size_t,
    pub uordblks: crate::size_t,
    pub fordblks: crate::size_t,
    pub keepcost: crate::size_t,
}
```

#### Trait Implementations

##### `impl Clone for mallinfo2`

- `fn clone(self: &Self) -> mallinfo2` — [`mallinfo2`](../index.md)

##### `impl Copy for mallinfo2`

##### `impl Debug for mallinfo2`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `nl_pktinfo`

```rust
struct nl_pktinfo {
    pub group: u32,
}
```

#### Trait Implementations

##### `impl Clone for nl_pktinfo`

- `fn clone(self: &Self) -> nl_pktinfo` — [`nl_pktinfo`](../index.md)

##### `impl Copy for nl_pktinfo`

##### `impl Debug for nl_pktinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `nl_mmap_req`

```rust
struct nl_mmap_req {
    pub nm_block_size: crate::c_uint,
    pub nm_block_nr: crate::c_uint,
    pub nm_frame_size: crate::c_uint,
    pub nm_frame_nr: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for nl_mmap_req`

- `fn clone(self: &Self) -> nl_mmap_req` — [`nl_mmap_req`](../index.md)

##### `impl Copy for nl_mmap_req`

##### `impl Debug for nl_mmap_req`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `nl_mmap_hdr`

```rust
struct nl_mmap_hdr {
    pub nm_status: crate::c_uint,
    pub nm_len: crate::c_uint,
    pub nm_group: u32,
    pub nm_pid: u32,
    pub nm_uid: u32,
    pub nm_gid: u32,
}
```

#### Trait Implementations

##### `impl Clone for nl_mmap_hdr`

- `fn clone(self: &Self) -> nl_mmap_hdr` — [`nl_mmap_hdr`](../index.md)

##### `impl Copy for nl_mmap_hdr`

##### `impl Debug for nl_mmap_hdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ntptimeval`

```rust
struct ntptimeval {
    pub time: crate::timeval,
    pub maxerror: crate::c_long,
    pub esterror: crate::c_long,
    pub tai: crate::c_long,
    pub __glibc_reserved1: crate::c_long,
    pub __glibc_reserved2: crate::c_long,
    pub __glibc_reserved3: crate::c_long,
    pub __glibc_reserved4: crate::c_long,
}
```

#### Trait Implementations

##### `impl Clone for ntptimeval`

- `fn clone(self: &Self) -> ntptimeval` — [`ntptimeval`](../index.md)

##### `impl Copy for ntptimeval`

##### `impl Debug for ntptimeval`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `regex_t`

```rust
struct regex_t {
    __buffer: *mut crate::c_void,
    __allocated: crate::size_t,
    __used: crate::size_t,
    __syntax: crate::c_ulong,
    __fastmap: *mut crate::c_char,
    __translate: *mut crate::c_char,
    __re_nsub: crate::size_t,
    __bitfield: u8,
}
```

#### Trait Implementations

##### `impl Clone for regex_t`

- `fn clone(self: &Self) -> regex_t` — [`regex_t`](../index.md)

##### `impl Copy for regex_t`

##### `impl Debug for regex_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Elf64_Chdr`

```rust
struct Elf64_Chdr {
    pub ch_type: crate::Elf64_Word,
    pub ch_reserved: crate::Elf64_Word,
    pub ch_size: crate::Elf64_Xword,
    pub ch_addralign: crate::Elf64_Xword,
}
```

#### Trait Implementations

##### `impl Clone for Elf64_Chdr`

- `fn clone(self: &Self) -> Elf64_Chdr` — [`Elf64_Chdr`](../index.md)

##### `impl Copy for Elf64_Chdr`

##### `impl Debug for Elf64_Chdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Elf32_Chdr`

```rust
struct Elf32_Chdr {
    pub ch_type: crate::Elf32_Word,
    pub ch_size: crate::Elf32_Word,
    pub ch_addralign: crate::Elf32_Word,
}
```

#### Trait Implementations

##### `impl Clone for Elf32_Chdr`

- `fn clone(self: &Self) -> Elf32_Chdr` — [`Elf32_Chdr`](../index.md)

##### `impl Copy for Elf32_Chdr`

##### `impl Debug for Elf32_Chdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `seminfo`

```rust
struct seminfo {
    pub semmap: crate::c_int,
    pub semmni: crate::c_int,
    pub semmns: crate::c_int,
    pub semmnu: crate::c_int,
    pub semmsl: crate::c_int,
    pub semopm: crate::c_int,
    pub semume: crate::c_int,
    pub semusz: crate::c_int,
    pub semvmx: crate::c_int,
    pub semaem: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for seminfo`

- `fn clone(self: &Self) -> seminfo` — [`seminfo`](../index.md)

##### `impl Copy for seminfo`

##### `impl Debug for seminfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptrace_peeksiginfo_args`

```rust
struct ptrace_peeksiginfo_args {
    pub off: crate::__u64,
    pub flags: crate::__u32,
    pub nr: crate::__s32,
}
```

#### Trait Implementations

##### `impl Clone for ptrace_peeksiginfo_args`

- `fn clone(self: &Self) -> ptrace_peeksiginfo_args` — [`ptrace_peeksiginfo_args`](../index.md)

##### `impl Copy for ptrace_peeksiginfo_args`

##### `impl Debug for ptrace_peeksiginfo_args`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__c_anonymous_ptrace_syscall_info_entry`

```rust
struct __c_anonymous_ptrace_syscall_info_entry {
    pub nr: crate::__u64,
    pub args: [crate::__u64; 6],
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_ptrace_syscall_info_entry`

- `fn clone(self: &Self) -> __c_anonymous_ptrace_syscall_info_entry` — [`__c_anonymous_ptrace_syscall_info_entry`](../index.md)

##### `impl Copy for __c_anonymous_ptrace_syscall_info_entry`

##### `impl Debug for __c_anonymous_ptrace_syscall_info_entry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__c_anonymous_ptrace_syscall_info_exit`

```rust
struct __c_anonymous_ptrace_syscall_info_exit {
    pub sval: crate::__s64,
    pub is_error: crate::__u8,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_ptrace_syscall_info_exit`

- `fn clone(self: &Self) -> __c_anonymous_ptrace_syscall_info_exit` — [`__c_anonymous_ptrace_syscall_info_exit`](../index.md)

##### `impl Copy for __c_anonymous_ptrace_syscall_info_exit`

##### `impl Debug for __c_anonymous_ptrace_syscall_info_exit`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__c_anonymous_ptrace_syscall_info_seccomp`

```rust
struct __c_anonymous_ptrace_syscall_info_seccomp {
    pub nr: crate::__u64,
    pub args: [crate::__u64; 6],
    pub ret_data: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_ptrace_syscall_info_seccomp`

- `fn clone(self: &Self) -> __c_anonymous_ptrace_syscall_info_seccomp` — [`__c_anonymous_ptrace_syscall_info_seccomp`](../index.md)

##### `impl Copy for __c_anonymous_ptrace_syscall_info_seccomp`

##### `impl Debug for __c_anonymous_ptrace_syscall_info_seccomp`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptrace_syscall_info`

```rust
struct ptrace_syscall_info {
    pub op: crate::__u8,
    pub pad: [crate::__u8; 3],
    pub arch: crate::__u32,
    pub instruction_pointer: crate::__u64,
    pub stack_pointer: crate::__u64,
    pub u: __c_anonymous_ptrace_syscall_info_data,
}
```

#### Trait Implementations

##### `impl Clone for ptrace_syscall_info`

- `fn clone(self: &Self) -> ptrace_syscall_info` — [`ptrace_syscall_info`](../index.md)

##### `impl Copy for ptrace_syscall_info`

##### `impl Debug for ptrace_syscall_info`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptrace_sud_config`

```rust
struct ptrace_sud_config {
    pub mode: crate::__u64,
    pub selector: crate::__u64,
    pub offset: crate::__u64,
    pub len: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for ptrace_sud_config`

- `fn clone(self: &Self) -> ptrace_sud_config` — [`ptrace_sud_config`](../index.md)

##### `impl Copy for ptrace_sud_config`

##### `impl Debug for ptrace_sud_config`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iocb`

```rust
struct iocb {
    pub aio_data: crate::__u64,
    pub aio_key: crate::__u32,
    pub aio_rw_flags: crate::__kernel_rwf_t,
    pub aio_lio_opcode: crate::__u16,
    pub aio_reqprio: crate::__s16,
    pub aio_fildes: crate::__u32,
    pub aio_buf: crate::__u64,
    pub aio_nbytes: crate::__u64,
    pub aio_offset: crate::__s64,
    aio_reserved2: crate::__u64,
    pub aio_flags: crate::__u32,
    pub aio_resfd: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for iocb`

- `fn clone(self: &Self) -> iocb` — [`iocb`](../index.md)

##### `impl Copy for iocb`

##### `impl Debug for iocb`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tcp_info`

```rust
struct tcp_info {
    pub tcpi_state: u8,
    pub tcpi_ca_state: u8,
    pub tcpi_retransmits: u8,
    pub tcpi_probes: u8,
    pub tcpi_backoff: u8,
    pub tcpi_options: u8,
    pub tcpi_snd_rcv_wscale: u8,
    pub tcpi_rto: u32,
    pub tcpi_ato: u32,
    pub tcpi_snd_mss: u32,
    pub tcpi_rcv_mss: u32,
    pub tcpi_unacked: u32,
    pub tcpi_sacked: u32,
    pub tcpi_lost: u32,
    pub tcpi_retrans: u32,
    pub tcpi_fackets: u32,
    pub tcpi_last_data_sent: u32,
    pub tcpi_last_ack_sent: u32,
    pub tcpi_last_data_recv: u32,
    pub tcpi_last_ack_recv: u32,
    pub tcpi_pmtu: u32,
    pub tcpi_rcv_ssthresh: u32,
    pub tcpi_rtt: u32,
    pub tcpi_rttvar: u32,
    pub tcpi_snd_ssthresh: u32,
    pub tcpi_snd_cwnd: u32,
    pub tcpi_advmss: u32,
    pub tcpi_reordering: u32,
    pub tcpi_rcv_rtt: u32,
    pub tcpi_rcv_space: u32,
    pub tcpi_total_retrans: u32,
}
```

#### Fields

- **`tcpi_snd_rcv_wscale`**: `u8`

  This contains the bitfields `tcpi_snd_wscale` and `tcpi_rcv_wscale`.
  Each is 4 bits.

#### Trait Implementations

##### `impl Clone for tcp_info`

- `fn clone(self: &Self) -> tcp_info` — [`tcp_info`](../index.md)

##### `impl Copy for tcp_info`

##### `impl Debug for tcp_info`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fanotify_event_info_pidfd`

```rust
struct fanotify_event_info_pidfd {
    pub hdr: crate::fanotify_event_info_header,
    pub pidfd: crate::__s32,
}
```

#### Trait Implementations

##### `impl Clone for fanotify_event_info_pidfd`

- `fn clone(self: &Self) -> fanotify_event_info_pidfd` — [`fanotify_event_info_pidfd`](../index.md)

##### `impl Copy for fanotify_event_info_pidfd`

##### `impl Debug for fanotify_event_info_pidfd`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fanotify_event_info_error`

```rust
struct fanotify_event_info_error {
    pub hdr: crate::fanotify_event_info_header,
    pub error: crate::__s32,
    pub error_count: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for fanotify_event_info_error`

- `fn clone(self: &Self) -> fanotify_event_info_error` — [`fanotify_event_info_error`](../index.md)

##### `impl Copy for fanotify_event_info_error`

##### `impl Debug for fanotify_event_info_error`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sem_t`

```rust
struct sem_t {
    __size: [crate::c_char; 32],
}
```

#### Trait Implementations

##### `impl Clone for sem_t`

- `fn clone(self: &Self) -> sem_t` — [`sem_t`](../index.md)

##### `impl Copy for sem_t`

##### `impl Debug for sem_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `mbstate_t`

```rust
struct mbstate_t {
    __count: crate::c_int,
    __wchb: [crate::c_char; 4],
}
```

#### Trait Implementations

##### `impl Clone for mbstate_t`

- `fn clone(self: &Self) -> mbstate_t` — [`mbstate_t`](../index.md)

##### `impl Copy for mbstate_t`

##### `impl Debug for mbstate_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fpos64_t`

```rust
struct fpos64_t {
    __pos: crate::off64_t,
    __state: crate::mbstate_t,
}
```

#### Trait Implementations

##### `impl Clone for fpos64_t`

- `fn clone(self: &Self) -> fpos64_t` — [`fpos64_t`](../index.md)

##### `impl Copy for fpos64_t`

##### `impl Debug for fpos64_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fpos_t`

```rust
struct fpos_t {
    __pos: off_t,
    __state: crate::mbstate_t,
}
```

#### Trait Implementations

##### `impl Clone for fpos_t`

- `fn clone(self: &Self) -> fpos_t` — [`fpos_t`](../index.md)

##### `impl Copy for fpos_t`

##### `impl Debug for fpos_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `timespec`

```rust
struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: crate::c_long,
}
```

#### Trait Implementations

##### `impl Clone for timespec`

- `fn clone(self: &Self) -> timespec` — [`timespec`](../index.md)

##### `impl Copy for timespec`

##### `impl Debug for timespec`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `utmpx`

```rust
struct utmpx {
    pub ut_type: crate::c_short,
    pub ut_pid: crate::pid_t,
    pub ut_line: [crate::c_char; 32],
    pub ut_id: [crate::c_char; 4],
    pub ut_user: [crate::c_char; 32],
    pub ut_host: [crate::c_char; 256],
    pub ut_exit: __exit_status,
    pub ut_session: i32,
    pub ut_tv: __timeval,
    pub ut_addr_v6: [i32; 4],
    __glibc_reserved: [crate::c_char; 20],
}
```

#### Trait Implementations

##### `impl Clone for utmpx`

- `fn clone(self: &Self) -> utmpx` — [`utmpx`](../index.md)

##### `impl Copy for utmpx`

##### `impl Debug for utmpx`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sifields_sigchld`

```rust
struct sifields_sigchld {
    si_pid: crate::pid_t,
    si_uid: crate::uid_t,
    si_status: crate::c_int,
    si_utime: crate::c_long,
    si_stime: crate::c_long,
}
```

#### Trait Implementations

##### `impl Clone for sifields_sigchld`

- `fn clone(self: &Self) -> sifields_sigchld` — [`sifields_sigchld`](#sifields-sigchld)

##### `impl Copy for sifields_sigchld`

##### `impl Debug for sifields_sigchld`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `siginfo_f`

```rust
struct siginfo_f {
    _siginfo_base: [crate::c_int; 3],
    sifields: sifields,
}
```

#### Trait Implementations

##### `impl Clone for siginfo_f`

- `fn clone(self: &Self) -> siginfo_f` — [`siginfo_f`](#siginfo-f)

##### `impl Copy for siginfo_f`

##### `impl Debug for siginfo_f`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sigset_t`

```rust
struct sigset_t {
    __val: [u64; 16],
}
```

#### Trait Implementations

##### `impl Clone for sigset_t`

- `fn clone(self: &Self) -> sigset_t` — [`sigset_t`](#sigset-t)

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

- `fn clone(self: &Self) -> sysinfo` — [`sysinfo`](#sysinfo)

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

- `fn clone(self: &Self) -> msqid_ds` — [`msqid_ds`](#msqid-ds)

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

- `fn clone(self: &Self) -> semid_ds` — [`semid_ds`](#semid-ds)

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

- `fn clone(self: &Self) -> timex` — [`timex`](#timex)

##### `impl Copy for timex`

##### `impl Debug for timex`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `fgetspent_r`

```rust
unsafe fn fgetspent_r(fp: *mut crate::FILE, spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```

### `sgetspent_r`

```rust
unsafe fn sgetspent_r(s: *const c_char, spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```

### `getspent_r`

```rust
unsafe fn getspent_r(spbuf: *mut crate::spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut crate::spwd) -> c_int
```

### `qsort_r`

```rust
unsafe fn qsort_r(base: *mut c_void, num: size_t, size: size_t, compar: Option<fn(*const c_void, *const c_void, *mut c_void) -> c_int>, arg: *mut c_void)
```

### `sendmmsg`

```rust
unsafe fn sendmmsg(sockfd: c_int, msgvec: *mut crate::mmsghdr, vlen: c_uint, flags: c_int) -> c_int
```

### `recvmmsg`

```rust
unsafe fn recvmmsg(sockfd: c_int, msgvec: *mut crate::mmsghdr, vlen: c_uint, flags: c_int, timeout: *mut crate::timespec) -> c_int
```

### `getrlimit64`

```rust
unsafe fn getrlimit64(resource: crate::__rlimit_resource_t, rlim: *mut crate::rlimit64) -> c_int
```

### `setrlimit64`

```rust
unsafe fn setrlimit64(resource: crate::__rlimit_resource_t, rlim: *const crate::rlimit64) -> c_int
```

### `getrlimit`

```rust
unsafe fn getrlimit(resource: crate::__rlimit_resource_t, rlim: *mut crate::rlimit) -> c_int
```

### `setrlimit`

```rust
unsafe fn setrlimit(resource: crate::__rlimit_resource_t, rlim: *const crate::rlimit) -> c_int
```

### `prlimit`

```rust
unsafe fn prlimit(pid: crate::pid_t, resource: crate::__rlimit_resource_t, new_limit: *const crate::rlimit, old_limit: *mut crate::rlimit) -> c_int
```

### `prlimit64`

```rust
unsafe fn prlimit64(pid: crate::pid_t, resource: crate::__rlimit_resource_t, new_limit: *const crate::rlimit64, old_limit: *mut crate::rlimit64) -> c_int
```

### `utmpname`

```rust
unsafe fn utmpname(file: *const c_char) -> c_int
```

### `utmpxname`

```rust
unsafe fn utmpxname(file: *const c_char) -> c_int
```

### `getutxent`

```rust
unsafe fn getutxent() -> *mut utmpx
```

### `getutxid`

```rust
unsafe fn getutxid(ut: *const utmpx) -> *mut utmpx
```

### `getutxline`

```rust
unsafe fn getutxline(ut: *const utmpx) -> *mut utmpx
```

### `pututxline`

```rust
unsafe fn pututxline(ut: *const utmpx) -> *mut utmpx
```

### `setutxent`

```rust
unsafe fn setutxent()
```

### `endutxent`

```rust
unsafe fn endutxent()
```

### `getpt`

```rust
unsafe fn getpt() -> c_int
```

### `mallopt`

```rust
unsafe fn mallopt(param: c_int, value: c_int) -> c_int
```

### `gettimeofday`

```rust
unsafe fn gettimeofday(tp: *mut crate::timeval, tz: *mut crate::timezone) -> c_int
```

### `getentropy`

```rust
unsafe fn getentropy(buf: *mut c_void, buflen: size_t) -> c_int
```

### `getrandom`

```rust
unsafe fn getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> ssize_t
```

### `getauxval`

```rust
unsafe fn getauxval(type_: c_ulong) -> c_ulong
```

### `adjtimex`

```rust
unsafe fn adjtimex(buf: *mut timex) -> c_int
```

### `ntp_adjtime`

```rust
unsafe fn ntp_adjtime(buf: *mut timex) -> c_int
```

### `ntp_gettime`

```rust
unsafe fn ntp_gettime(buf: *mut ntptimeval) -> c_int
```

### `clock_adjtime`

```rust
unsafe fn clock_adjtime(clk_id: crate::clockid_t, buf: *mut crate::timex) -> c_int
```

### `fanotify_mark`

```rust
unsafe fn fanotify_mark(fd: c_int, flags: c_uint, mask: u64, dirfd: c_int, path: *const c_char) -> c_int
```

### `preadv2`

```rust
unsafe fn preadv2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> ssize_t
```

### `pwritev2`

```rust
unsafe fn pwritev2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> ssize_t
```

### `preadv64v2`

```rust
unsafe fn preadv64v2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t, flags: c_int) -> ssize_t
```

### `pwritev64v2`

```rust
unsafe fn pwritev64v2(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t, flags: c_int) -> ssize_t
```

### `renameat2`

```rust
unsafe fn renameat2(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_uint) -> c_int
```

### `explicit_bzero`

```rust
unsafe fn explicit_bzero(s: *mut c_void, len: size_t)
```

### `reallocarray`

```rust
unsafe fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void
```

### `ctermid`

```rust
unsafe fn ctermid(s: *mut c_char) -> *mut c_char
```

### `backtrace`

```rust
unsafe fn backtrace(buf: *mut *mut c_void, sz: c_int) -> c_int
```

### `backtrace_symbols`

```rust
unsafe fn backtrace_symbols(buffer: *const *mut c_void, len: c_int) -> *mut *mut c_char
```

### `backtrace_symbols_fd`

```rust
unsafe fn backtrace_symbols_fd(buffer: *const *mut c_void, len: c_int, fd: c_int)
```

### `glob64`

```rust
unsafe fn glob64(pattern: *const c_char, flags: c_int, errfunc: Option<fn(*const c_char, c_int) -> c_int>, pglob: *mut glob64_t) -> c_int
```

### `globfree64`

```rust
unsafe fn globfree64(pglob: *mut glob64_t)
```

### `ptrace`

```rust
unsafe fn ptrace(request: c_uint) -> c_long
```

### `pthread_attr_getaffinity_np`

```rust
unsafe fn pthread_attr_getaffinity_np(attr: *const crate::pthread_attr_t, cpusetsize: size_t, cpuset: *mut crate::cpu_set_t) -> c_int
```

### `pthread_attr_setaffinity_np`

```rust
unsafe fn pthread_attr_setaffinity_np(attr: *mut crate::pthread_attr_t, cpusetsize: size_t, cpuset: *const crate::cpu_set_t) -> c_int
```

### `getpriority`

```rust
unsafe fn getpriority(which: crate::__priority_which_t, who: crate::id_t) -> c_int
```

### `setpriority`

```rust
unsafe fn setpriority(which: crate::__priority_which_t, who: crate::id_t, prio: c_int) -> c_int
```

### `pthread_rwlockattr_getkind_np`

```rust
unsafe fn pthread_rwlockattr_getkind_np(attr: *const crate::pthread_rwlockattr_t, val: *mut c_int) -> c_int
```

### `pthread_rwlockattr_setkind_np`

```rust
unsafe fn pthread_rwlockattr_setkind_np(attr: *mut crate::pthread_rwlockattr_t, val: c_int) -> c_int
```

### `pthread_sigqueue`

```rust
unsafe fn pthread_sigqueue(thread: crate::pthread_t, sig: c_int, value: crate::sigval) -> c_int
```

### `mallinfo`

```rust
unsafe fn mallinfo() -> crate::mallinfo
```

### `mallinfo2`

```rust
unsafe fn mallinfo2() -> crate::mallinfo2
```

### `malloc_stats`

```rust
unsafe fn malloc_stats()
```

### `malloc_info`

```rust
unsafe fn malloc_info(options: c_int, stream: *mut crate::FILE) -> c_int
```

### `malloc_usable_size`

```rust
unsafe fn malloc_usable_size(ptr: *mut c_void) -> size_t
```

### `getpwent_r`

```rust
unsafe fn getpwent_r(pwd: *mut crate::passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::passwd) -> c_int
```

### `getgrent_r`

```rust
unsafe fn getgrent_r(grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```

### `fgetpwent_r`

```rust
unsafe fn fgetpwent_r(stream: *mut crate::FILE, pwd: *mut crate::passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::passwd) -> c_int
```

### `fgetgrent_r`

```rust
unsafe fn fgetgrent_r(stream: *mut crate::FILE, grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```

### `putpwent`

```rust
unsafe fn putpwent(p: *const crate::passwd, stream: *mut crate::FILE) -> c_int
```

### `putgrent`

```rust
unsafe fn putgrent(grp: *const crate::group, stream: *mut crate::FILE) -> c_int
```

### `sethostid`

```rust
unsafe fn sethostid(hostid: c_long) -> c_int
```

### `memfd_create`

```rust
unsafe fn memfd_create(name: *const c_char, flags: c_uint) -> c_int
```

### `mlock2`

```rust
unsafe fn mlock2(addr: *const c_void, len: size_t, flags: c_uint) -> c_int
```

### `euidaccess`

```rust
unsafe fn euidaccess(pathname: *const c_char, mode: c_int) -> c_int
```

### `eaccess`

```rust
unsafe fn eaccess(pathname: *const c_char, mode: c_int) -> c_int
```

### `asctime_r`

```rust
unsafe fn asctime_r(tm: *const crate::tm, buf: *mut c_char) -> *mut c_char
```

### `ctime_r`

```rust
unsafe fn ctime_r(timep: *const time_t, buf: *mut c_char) -> *mut c_char
```

### `dirname`

```rust
unsafe fn dirname(path: *mut c_char) -> *mut c_char
```

### `posix_basename`

```rust
unsafe fn posix_basename(path: *mut c_char) -> *mut c_char
```

POSIX version of `basename(3)`, defined in `libgen.h`.

### `gnu_basename`

```rust
unsafe fn gnu_basename(path: *const c_char) -> *mut c_char
```

GNU version of `basename(3)`, defined in `string.h`.

### `dlmopen`

```rust
unsafe fn dlmopen(lmid: Lmid_t, filename: *const c_char, flag: c_int) -> *mut c_void
```

### `dlinfo`

```rust
unsafe fn dlinfo(handle: *mut c_void, request: c_int, info: *mut c_void) -> c_int
```

### `dladdr1`

```rust
unsafe fn dladdr1(addr: *const c_void, info: *mut crate::Dl_info, extra_info: *mut *mut c_void, flags: c_int) -> c_int
```

### `dlvsym`

```rust
unsafe fn dlvsym(handle: *mut c_void, symbol: *const c_char, version: *const c_char) -> *mut c_void
```

### `malloc_trim`

```rust
unsafe fn malloc_trim(__pad: size_t) -> c_int
```

### `gnu_get_libc_release`

```rust
unsafe fn gnu_get_libc_release() -> *const c_char
```

### `gnu_get_libc_version`

```rust
unsafe fn gnu_get_libc_version() -> *const c_char
```

### `posix_spawn_file_actions_addchdir_np`

```rust
unsafe fn posix_spawn_file_actions_addchdir_np(actions: *mut crate::posix_spawn_file_actions_t, path: *const c_char) -> c_int
```

### `posix_spawn_file_actions_addfchdir_np`

```rust
unsafe fn posix_spawn_file_actions_addfchdir_np(actions: *mut crate::posix_spawn_file_actions_t, fd: c_int) -> c_int
```

### `posix_spawn_file_actions_addclosefrom_np`

```rust
unsafe fn posix_spawn_file_actions_addclosefrom_np(actions: *mut crate::posix_spawn_file_actions_t, from: c_int) -> c_int
```

### `posix_spawn_file_actions_addtcsetpgrp_np`

```rust
unsafe fn posix_spawn_file_actions_addtcsetpgrp_np(actions: *mut crate::posix_spawn_file_actions_t, tcfd: c_int) -> c_int
```

### `getmntent_r`

```rust
unsafe fn getmntent_r(stream: *mut crate::FILE, mntbuf: *mut crate::mntent, buf: *mut c_char, buflen: c_int) -> *mut crate::mntent
```

### `execveat`

```rust
unsafe fn execveat(dirfd: c_int, pathname: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char, flags: c_int) -> c_int
```

### `close_range`

```rust
unsafe fn close_range(first: c_uint, last: c_uint, flags: c_int) -> c_int
```

### `mq_notify`

```rust
unsafe fn mq_notify(mqdes: crate::mqd_t, sevp: *const crate::sigevent) -> c_int
```

### `epoll_pwait2`

```rust
unsafe fn epoll_pwait2(epfd: c_int, events: *mut crate::epoll_event, maxevents: c_int, timeout: *const crate::timespec, sigmask: *const crate::sigset_t) -> c_int
```

### `mempcpy`

```rust
unsafe fn mempcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```

## Type Aliases

### `pthread_t`

```rust
type pthread_t = crate::c_ulong;
```

### `__priority_which_t`

```rust
type __priority_which_t = crate::c_uint;
```

### `__rlimit_resource_t`

```rust
type __rlimit_resource_t = crate::c_uint;
```

### `Lmid_t`

```rust
type Lmid_t = crate::c_long;
```

### `regoff_t`

```rust
type regoff_t = crate::c_int;
```

### `__kernel_rwf_t`

```rust
type __kernel_rwf_t = crate::c_int;
```

### `Ioctl`

```rust
type Ioctl = crate::c_ulong;
```

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

## Constants

### `HUGETLB_FLAG_ENCODE_SHIFT`

```rust
const HUGETLB_FLAG_ENCODE_SHIFT: crate::c_int = 26i32;
```

### `HUGETLB_FLAG_ENCODE_MASK`

```rust
const HUGETLB_FLAG_ENCODE_MASK: crate::c_int = 63i32;
```

### `HUGETLB_FLAG_ENCODE_64KB`

```rust
const HUGETLB_FLAG_ENCODE_64KB: crate::c_int = 1_073_741_824i32;
```

### `HUGETLB_FLAG_ENCODE_512KB`

```rust
const HUGETLB_FLAG_ENCODE_512KB: crate::c_int = 1_275_068_416i32;
```

### `HUGETLB_FLAG_ENCODE_1MB`

```rust
const HUGETLB_FLAG_ENCODE_1MB: crate::c_int = 1_342_177_280i32;
```

### `HUGETLB_FLAG_ENCODE_2MB`

```rust
const HUGETLB_FLAG_ENCODE_2MB: crate::c_int = 1_409_286_144i32;
```

### `HUGETLB_FLAG_ENCODE_8MB`

```rust
const HUGETLB_FLAG_ENCODE_8MB: crate::c_int = 1_543_503_872i32;
```

### `HUGETLB_FLAG_ENCODE_16MB`

```rust
const HUGETLB_FLAG_ENCODE_16MB: crate::c_int = 1_610_612_736i32;
```

### `HUGETLB_FLAG_ENCODE_32MB`

```rust
const HUGETLB_FLAG_ENCODE_32MB: crate::c_int = 1_677_721_600i32;
```

### `HUGETLB_FLAG_ENCODE_256MB`

```rust
const HUGETLB_FLAG_ENCODE_256MB: crate::c_int = 1_879_048_192i32;
```

### `HUGETLB_FLAG_ENCODE_512MB`

```rust
const HUGETLB_FLAG_ENCODE_512MB: crate::c_int = 1_946_157_056i32;
```

### `HUGETLB_FLAG_ENCODE_1GB`

```rust
const HUGETLB_FLAG_ENCODE_1GB: crate::c_int = 2_013_265_920i32;
```

### `HUGETLB_FLAG_ENCODE_2GB`

```rust
const HUGETLB_FLAG_ENCODE_2GB: crate::c_int = 2_080_374_784i32;
```

### `HUGETLB_FLAG_ENCODE_16GB`

```rust
const HUGETLB_FLAG_ENCODE_16GB: crate::c_int = -2_013_265_920i32;
```

### `MAP_HUGE_SHIFT`

```rust
const MAP_HUGE_SHIFT: crate::c_int = 26i32;
```

### `MAP_HUGE_MASK`

```rust
const MAP_HUGE_MASK: crate::c_int = 63i32;
```

### `MAP_HUGE_64KB`

```rust
const MAP_HUGE_64KB: crate::c_int = 1_073_741_824i32;
```

### `MAP_HUGE_512KB`

```rust
const MAP_HUGE_512KB: crate::c_int = 1_275_068_416i32;
```

### `MAP_HUGE_1MB`

```rust
const MAP_HUGE_1MB: crate::c_int = 1_342_177_280i32;
```

### `MAP_HUGE_2MB`

```rust
const MAP_HUGE_2MB: crate::c_int = 1_409_286_144i32;
```

### `MAP_HUGE_8MB`

```rust
const MAP_HUGE_8MB: crate::c_int = 1_543_503_872i32;
```

### `MAP_HUGE_16MB`

```rust
const MAP_HUGE_16MB: crate::c_int = 1_610_612_736i32;
```

### `MAP_HUGE_32MB`

```rust
const MAP_HUGE_32MB: crate::c_int = 1_677_721_600i32;
```

### `MAP_HUGE_256MB`

```rust
const MAP_HUGE_256MB: crate::c_int = 1_879_048_192i32;
```

### `MAP_HUGE_512MB`

```rust
const MAP_HUGE_512MB: crate::c_int = 1_946_157_056i32;
```

### `MAP_HUGE_1GB`

```rust
const MAP_HUGE_1GB: crate::c_int = 2_013_265_920i32;
```

### `MAP_HUGE_2GB`

```rust
const MAP_HUGE_2GB: crate::c_int = 2_080_374_784i32;
```

### `MAP_HUGE_16GB`

```rust
const MAP_HUGE_16GB: crate::c_int = -2_013_265_920i32;
```

### `PRIO_PROCESS`

```rust
const PRIO_PROCESS: crate::__priority_which_t = 0u32;
```

### `PRIO_PGRP`

```rust
const PRIO_PGRP: crate::__priority_which_t = 1u32;
```

### `PRIO_USER`

```rust
const PRIO_USER: crate::__priority_which_t = 2u32;
```

### `MS_RMT_MASK`

```rust
const MS_RMT_MASK: crate::c_ulong = 41_943_121u64;
```

### `__UT_LINESIZE`

```rust
const __UT_LINESIZE: usize = 32usize;
```

### `__UT_NAMESIZE`

```rust
const __UT_NAMESIZE: usize = 32usize;
```

### `__UT_HOSTSIZE`

```rust
const __UT_HOSTSIZE: usize = 256usize;
```

### `EMPTY`

```rust
const EMPTY: crate::c_short = 0i16;
```

### `RUN_LVL`

```rust
const RUN_LVL: crate::c_short = 1i16;
```

### `BOOT_TIME`

```rust
const BOOT_TIME: crate::c_short = 2i16;
```

### `NEW_TIME`

```rust
const NEW_TIME: crate::c_short = 3i16;
```

### `OLD_TIME`

```rust
const OLD_TIME: crate::c_short = 4i16;
```

### `INIT_PROCESS`

```rust
const INIT_PROCESS: crate::c_short = 5i16;
```

### `LOGIN_PROCESS`

```rust
const LOGIN_PROCESS: crate::c_short = 6i16;
```

### `USER_PROCESS`

```rust
const USER_PROCESS: crate::c_short = 7i16;
```

### `DEAD_PROCESS`

```rust
const DEAD_PROCESS: crate::c_short = 8i16;
```

### `ACCOUNTING`

```rust
const ACCOUNTING: crate::c_short = 9i16;
```

### `LM_ID_BASE`

```rust
const LM_ID_BASE: crate::c_long = 0i64;
```

### `LM_ID_NEWLM`

```rust
const LM_ID_NEWLM: crate::c_long = -1i64;
```

### `RTLD_DI_LMID`

```rust
const RTLD_DI_LMID: crate::c_int = 1i32;
```

### `RTLD_DI_LINKMAP`

```rust
const RTLD_DI_LINKMAP: crate::c_int = 2i32;
```

### `RTLD_DI_CONFIGADDR`

```rust
const RTLD_DI_CONFIGADDR: crate::c_int = 3i32;
```

### `RTLD_DI_SERINFO`

```rust
const RTLD_DI_SERINFO: crate::c_int = 4i32;
```

### `RTLD_DI_SERINFOSIZE`

```rust
const RTLD_DI_SERINFOSIZE: crate::c_int = 5i32;
```

### `RTLD_DI_ORIGIN`

```rust
const RTLD_DI_ORIGIN: crate::c_int = 6i32;
```

### `RTLD_DI_PROFILENAME`

```rust
const RTLD_DI_PROFILENAME: crate::c_int = 7i32;
```

### `RTLD_DI_PROFILEOUT`

```rust
const RTLD_DI_PROFILEOUT: crate::c_int = 8i32;
```

### `RTLD_DI_TLS_MODID`

```rust
const RTLD_DI_TLS_MODID: crate::c_int = 9i32;
```

### `RTLD_DI_TLS_DATA`

```rust
const RTLD_DI_TLS_DATA: crate::c_int = 10i32;
```

### `SOCK_NONBLOCK`

```rust
const SOCK_NONBLOCK: crate::c_int = 2_048i32;
```

### `SOL_RXRPC`

```rust
const SOL_RXRPC: crate::c_int = 272i32;
```

### `SOL_PPPOL2TP`

```rust
const SOL_PPPOL2TP: crate::c_int = 273i32;
```

### `SOL_PNPIPE`

```rust
const SOL_PNPIPE: crate::c_int = 275i32;
```

### `SOL_RDS`

```rust
const SOL_RDS: crate::c_int = 276i32;
```

### `SOL_IUCV`

```rust
const SOL_IUCV: crate::c_int = 277i32;
```

### `SOL_CAIF`

```rust
const SOL_CAIF: crate::c_int = 278i32;
```

### `SOL_NFC`

```rust
const SOL_NFC: crate::c_int = 280i32;
```

### `MSG_TRYHARD`

```rust
const MSG_TRYHARD: crate::c_int = 4i32;
```

### `LC_PAPER`

```rust
const LC_PAPER: crate::c_int = 7i32;
```

### `LC_NAME`

```rust
const LC_NAME: crate::c_int = 8i32;
```

### `LC_ADDRESS`

```rust
const LC_ADDRESS: crate::c_int = 9i32;
```

### `LC_TELEPHONE`

```rust
const LC_TELEPHONE: crate::c_int = 10i32;
```

### `LC_MEASUREMENT`

```rust
const LC_MEASUREMENT: crate::c_int = 11i32;
```

### `LC_IDENTIFICATION`

```rust
const LC_IDENTIFICATION: crate::c_int = 12i32;
```

### `LC_PAPER_MASK`

```rust
const LC_PAPER_MASK: crate::c_int = 128i32;
```

### `LC_NAME_MASK`

```rust
const LC_NAME_MASK: crate::c_int = 256i32;
```

### `LC_ADDRESS_MASK`

```rust
const LC_ADDRESS_MASK: crate::c_int = 512i32;
```

### `LC_TELEPHONE_MASK`

```rust
const LC_TELEPHONE_MASK: crate::c_int = 1_024i32;
```

### `LC_MEASUREMENT_MASK`

```rust
const LC_MEASUREMENT_MASK: crate::c_int = 2_048i32;
```

### `LC_IDENTIFICATION_MASK`

```rust
const LC_IDENTIFICATION_MASK: crate::c_int = 4_096i32;
```

### `LC_ALL_MASK`

```rust
const LC_ALL_MASK: crate::c_int = 8_127i32;
```

### `ENOTSUP`

```rust
const ENOTSUP: crate::c_int = 95i32;
```

### `SOCK_SEQPACKET`

```rust
const SOCK_SEQPACKET: crate::c_int = 5i32;
```

### `SOCK_DCCP`

```rust
const SOCK_DCCP: crate::c_int = 6i32;
```

### `SOCK_PACKET`

```rust
const SOCK_PACKET: crate::c_int = 10i32;
```

### `AF_IB`

```rust
const AF_IB: crate::c_int = 27i32;
```

### `AF_MPLS`

```rust
const AF_MPLS: crate::c_int = 28i32;
```

### `AF_NFC`

```rust
const AF_NFC: crate::c_int = 39i32;
```

### `AF_VSOCK`

```rust
const AF_VSOCK: crate::c_int = 40i32;
```

### `AF_XDP`

```rust
const AF_XDP: crate::c_int = 44i32;
```

### `PF_IB`

```rust
const PF_IB: crate::c_int = 27i32;
```

### `PF_MPLS`

```rust
const PF_MPLS: crate::c_int = 28i32;
```

### `PF_NFC`

```rust
const PF_NFC: crate::c_int = 39i32;
```

### `PF_VSOCK`

```rust
const PF_VSOCK: crate::c_int = 40i32;
```

### `PF_XDP`

```rust
const PF_XDP: crate::c_int = 44i32;
```

### `SIGEV_THREAD_ID`

```rust
const SIGEV_THREAD_ID: crate::c_int = 4i32;
```

### `BUFSIZ`

```rust
const BUFSIZ: crate::c_uint = 8_192u32;
```

### `TMP_MAX`

```rust
const TMP_MAX: crate::c_uint = 238_328u32;
```

### `FOPEN_MAX`

```rust
const FOPEN_MAX: crate::c_uint = 16u32;
```

### `FILENAME_MAX`

```rust
const FILENAME_MAX: crate::c_uint = 4_096u32;
```

### `POSIX_MADV_DONTNEED`

```rust
const POSIX_MADV_DONTNEED: crate::c_int = 4i32;
```

### `_CS_GNU_LIBC_VERSION`

```rust
const _CS_GNU_LIBC_VERSION: crate::c_int = 2i32;
```

### `_CS_GNU_LIBPTHREAD_VERSION`

```rust
const _CS_GNU_LIBPTHREAD_VERSION: crate::c_int = 3i32;
```

### `_CS_V6_ENV`

```rust
const _CS_V6_ENV: crate::c_int = 1_148i32;
```

### `_CS_V7_ENV`

```rust
const _CS_V7_ENV: crate::c_int = 1_149i32;
```

### `_SC_EQUIV_CLASS_MAX`

```rust
const _SC_EQUIV_CLASS_MAX: crate::c_int = 41i32;
```

### `_SC_CHARCLASS_NAME_MAX`

```rust
const _SC_CHARCLASS_NAME_MAX: crate::c_int = 45i32;
```

### `_SC_PII`

```rust
const _SC_PII: crate::c_int = 53i32;
```

### `_SC_PII_XTI`

```rust
const _SC_PII_XTI: crate::c_int = 54i32;
```

### `_SC_PII_SOCKET`

```rust
const _SC_PII_SOCKET: crate::c_int = 55i32;
```

### `_SC_PII_INTERNET`

```rust
const _SC_PII_INTERNET: crate::c_int = 56i32;
```

### `_SC_PII_OSI`

```rust
const _SC_PII_OSI: crate::c_int = 57i32;
```

### `_SC_POLL`

```rust
const _SC_POLL: crate::c_int = 58i32;
```

### `_SC_SELECT`

```rust
const _SC_SELECT: crate::c_int = 59i32;
```

### `_SC_PII_INTERNET_STREAM`

```rust
const _SC_PII_INTERNET_STREAM: crate::c_int = 61i32;
```

### `_SC_PII_INTERNET_DGRAM`

```rust
const _SC_PII_INTERNET_DGRAM: crate::c_int = 62i32;
```

### `_SC_PII_OSI_COTS`

```rust
const _SC_PII_OSI_COTS: crate::c_int = 63i32;
```

### `_SC_PII_OSI_CLTS`

```rust
const _SC_PII_OSI_CLTS: crate::c_int = 64i32;
```

### `_SC_PII_OSI_M`

```rust
const _SC_PII_OSI_M: crate::c_int = 65i32;
```

### `_SC_T_IOV_MAX`

```rust
const _SC_T_IOV_MAX: crate::c_int = 66i32;
```

### `_SC_2_C_VERSION`

```rust
const _SC_2_C_VERSION: crate::c_int = 96i32;
```

### `_SC_CHAR_BIT`

```rust
const _SC_CHAR_BIT: crate::c_int = 101i32;
```

### `_SC_CHAR_MAX`

```rust
const _SC_CHAR_MAX: crate::c_int = 102i32;
```

### `_SC_CHAR_MIN`

```rust
const _SC_CHAR_MIN: crate::c_int = 103i32;
```

### `_SC_INT_MAX`

```rust
const _SC_INT_MAX: crate::c_int = 104i32;
```

### `_SC_INT_MIN`

```rust
const _SC_INT_MIN: crate::c_int = 105i32;
```

### `_SC_LONG_BIT`

```rust
const _SC_LONG_BIT: crate::c_int = 106i32;
```

### `_SC_WORD_BIT`

```rust
const _SC_WORD_BIT: crate::c_int = 107i32;
```

### `_SC_MB_LEN_MAX`

```rust
const _SC_MB_LEN_MAX: crate::c_int = 108i32;
```

### `_SC_SSIZE_MAX`

```rust
const _SC_SSIZE_MAX: crate::c_int = 110i32;
```

### `_SC_SCHAR_MAX`

```rust
const _SC_SCHAR_MAX: crate::c_int = 111i32;
```

### `_SC_SCHAR_MIN`

```rust
const _SC_SCHAR_MIN: crate::c_int = 112i32;
```

### `_SC_SHRT_MAX`

```rust
const _SC_SHRT_MAX: crate::c_int = 113i32;
```

### `_SC_SHRT_MIN`

```rust
const _SC_SHRT_MIN: crate::c_int = 114i32;
```

### `_SC_UCHAR_MAX`

```rust
const _SC_UCHAR_MAX: crate::c_int = 115i32;
```

### `_SC_UINT_MAX`

```rust
const _SC_UINT_MAX: crate::c_int = 116i32;
```

### `_SC_ULONG_MAX`

```rust
const _SC_ULONG_MAX: crate::c_int = 117i32;
```

### `_SC_USHRT_MAX`

```rust
const _SC_USHRT_MAX: crate::c_int = 118i32;
```

### `_SC_NL_ARGMAX`

```rust
const _SC_NL_ARGMAX: crate::c_int = 119i32;
```

### `_SC_NL_LANGMAX`

```rust
const _SC_NL_LANGMAX: crate::c_int = 120i32;
```

### `_SC_NL_MSGMAX`

```rust
const _SC_NL_MSGMAX: crate::c_int = 121i32;
```

### `_SC_NL_NMAX`

```rust
const _SC_NL_NMAX: crate::c_int = 122i32;
```

### `_SC_NL_SETMAX`

```rust
const _SC_NL_SETMAX: crate::c_int = 123i32;
```

### `_SC_NL_TEXTMAX`

```rust
const _SC_NL_TEXTMAX: crate::c_int = 124i32;
```

### `_SC_BASE`

```rust
const _SC_BASE: crate::c_int = 134i32;
```

### `_SC_C_LANG_SUPPORT`

```rust
const _SC_C_LANG_SUPPORT: crate::c_int = 135i32;
```

### `_SC_C_LANG_SUPPORT_R`

```rust
const _SC_C_LANG_SUPPORT_R: crate::c_int = 136i32;
```

### `_SC_DEVICE_IO`

```rust
const _SC_DEVICE_IO: crate::c_int = 140i32;
```

### `_SC_DEVICE_SPECIFIC`

```rust
const _SC_DEVICE_SPECIFIC: crate::c_int = 141i32;
```

### `_SC_DEVICE_SPECIFIC_R`

```rust
const _SC_DEVICE_SPECIFIC_R: crate::c_int = 142i32;
```

### `_SC_FD_MGMT`

```rust
const _SC_FD_MGMT: crate::c_int = 143i32;
```

### `_SC_FIFO`

```rust
const _SC_FIFO: crate::c_int = 144i32;
```

### `_SC_PIPE`

```rust
const _SC_PIPE: crate::c_int = 145i32;
```

### `_SC_FILE_ATTRIBUTES`

```rust
const _SC_FILE_ATTRIBUTES: crate::c_int = 146i32;
```

### `_SC_FILE_LOCKING`

```rust
const _SC_FILE_LOCKING: crate::c_int = 147i32;
```

### `_SC_FILE_SYSTEM`

```rust
const _SC_FILE_SYSTEM: crate::c_int = 148i32;
```

### `_SC_MULTI_PROCESS`

```rust
const _SC_MULTI_PROCESS: crate::c_int = 150i32;
```

### `_SC_SINGLE_PROCESS`

```rust
const _SC_SINGLE_PROCESS: crate::c_int = 151i32;
```

### `_SC_NETWORKING`

```rust
const _SC_NETWORKING: crate::c_int = 152i32;
```

### `_SC_REGEX_VERSION`

```rust
const _SC_REGEX_VERSION: crate::c_int = 156i32;
```

### `_SC_SIGNALS`

```rust
const _SC_SIGNALS: crate::c_int = 158i32;
```

### `_SC_SYSTEM_DATABASE`

```rust
const _SC_SYSTEM_DATABASE: crate::c_int = 162i32;
```

### `_SC_SYSTEM_DATABASE_R`

```rust
const _SC_SYSTEM_DATABASE_R: crate::c_int = 163i32;
```

### `_SC_USER_GROUPS`

```rust
const _SC_USER_GROUPS: crate::c_int = 166i32;
```

### `_SC_USER_GROUPS_R`

```rust
const _SC_USER_GROUPS_R: crate::c_int = 167i32;
```

### `_SC_LEVEL1_ICACHE_SIZE`

```rust
const _SC_LEVEL1_ICACHE_SIZE: crate::c_int = 185i32;
```

### `_SC_LEVEL1_ICACHE_ASSOC`

```rust
const _SC_LEVEL1_ICACHE_ASSOC: crate::c_int = 186i32;
```

### `_SC_LEVEL1_ICACHE_LINESIZE`

```rust
const _SC_LEVEL1_ICACHE_LINESIZE: crate::c_int = 187i32;
```

### `_SC_LEVEL1_DCACHE_SIZE`

```rust
const _SC_LEVEL1_DCACHE_SIZE: crate::c_int = 188i32;
```

### `_SC_LEVEL1_DCACHE_ASSOC`

```rust
const _SC_LEVEL1_DCACHE_ASSOC: crate::c_int = 189i32;
```

### `_SC_LEVEL1_DCACHE_LINESIZE`

```rust
const _SC_LEVEL1_DCACHE_LINESIZE: crate::c_int = 190i32;
```

### `_SC_LEVEL2_CACHE_SIZE`

```rust
const _SC_LEVEL2_CACHE_SIZE: crate::c_int = 191i32;
```

### `_SC_LEVEL2_CACHE_ASSOC`

```rust
const _SC_LEVEL2_CACHE_ASSOC: crate::c_int = 192i32;
```

### `_SC_LEVEL2_CACHE_LINESIZE`

```rust
const _SC_LEVEL2_CACHE_LINESIZE: crate::c_int = 193i32;
```

### `_SC_LEVEL3_CACHE_SIZE`

```rust
const _SC_LEVEL3_CACHE_SIZE: crate::c_int = 194i32;
```

### `_SC_LEVEL3_CACHE_ASSOC`

```rust
const _SC_LEVEL3_CACHE_ASSOC: crate::c_int = 195i32;
```

### `_SC_LEVEL3_CACHE_LINESIZE`

```rust
const _SC_LEVEL3_CACHE_LINESIZE: crate::c_int = 196i32;
```

### `_SC_LEVEL4_CACHE_SIZE`

```rust
const _SC_LEVEL4_CACHE_SIZE: crate::c_int = 197i32;
```

### `_SC_LEVEL4_CACHE_ASSOC`

```rust
const _SC_LEVEL4_CACHE_ASSOC: crate::c_int = 198i32;
```

### `_SC_LEVEL4_CACHE_LINESIZE`

```rust
const _SC_LEVEL4_CACHE_LINESIZE: crate::c_int = 199i32;
```

### `O_ACCMODE`

```rust
const O_ACCMODE: crate::c_int = 3i32;
```

### `ST_RELATIME`

```rust
const ST_RELATIME: crate::c_ulong = 4_096u64;
```

### `NI_MAXHOST`

```rust
const NI_MAXHOST: crate::socklen_t = 1_025u32;
```

### `BINDERFS_SUPER_MAGIC`

```rust
const BINDERFS_SUPER_MAGIC: crate::c_long = 1_819_242_352i64;
```

### `XFS_SUPER_MAGIC`

```rust
const XFS_SUPER_MAGIC: crate::c_long = 1_481_003_842i64;
```

### `CPU_SETSIZE`

```rust
const CPU_SETSIZE: crate::c_int = 1_024i32;
```

### `PTRACE_TRACEME`

```rust
const PTRACE_TRACEME: crate::c_uint = 0u32;
```

### `PTRACE_PEEKTEXT`

```rust
const PTRACE_PEEKTEXT: crate::c_uint = 1u32;
```

### `PTRACE_PEEKDATA`

```rust
const PTRACE_PEEKDATA: crate::c_uint = 2u32;
```

### `PTRACE_PEEKUSER`

```rust
const PTRACE_PEEKUSER: crate::c_uint = 3u32;
```

### `PTRACE_POKETEXT`

```rust
const PTRACE_POKETEXT: crate::c_uint = 4u32;
```

### `PTRACE_POKEDATA`

```rust
const PTRACE_POKEDATA: crate::c_uint = 5u32;
```

### `PTRACE_POKEUSER`

```rust
const PTRACE_POKEUSER: crate::c_uint = 6u32;
```

### `PTRACE_CONT`

```rust
const PTRACE_CONT: crate::c_uint = 7u32;
```

### `PTRACE_KILL`

```rust
const PTRACE_KILL: crate::c_uint = 8u32;
```

### `PTRACE_SINGLESTEP`

```rust
const PTRACE_SINGLESTEP: crate::c_uint = 9u32;
```

### `PTRACE_ATTACH`

```rust
const PTRACE_ATTACH: crate::c_uint = 16u32;
```

### `PTRACE_SYSCALL`

```rust
const PTRACE_SYSCALL: crate::c_uint = 24u32;
```

### `PTRACE_SETOPTIONS`

```rust
const PTRACE_SETOPTIONS: crate::c_uint = 16_896u32;
```

### `PTRACE_GETEVENTMSG`

```rust
const PTRACE_GETEVENTMSG: crate::c_uint = 16_897u32;
```

### `PTRACE_GETSIGINFO`

```rust
const PTRACE_GETSIGINFO: crate::c_uint = 16_898u32;
```

### `PTRACE_SETSIGINFO`

```rust
const PTRACE_SETSIGINFO: crate::c_uint = 16_899u32;
```

### `PTRACE_GETREGSET`

```rust
const PTRACE_GETREGSET: crate::c_uint = 16_900u32;
```

### `PTRACE_SETREGSET`

```rust
const PTRACE_SETREGSET: crate::c_uint = 16_901u32;
```

### `PTRACE_SEIZE`

```rust
const PTRACE_SEIZE: crate::c_uint = 16_902u32;
```

### `PTRACE_INTERRUPT`

```rust
const PTRACE_INTERRUPT: crate::c_uint = 16_903u32;
```

### `PTRACE_LISTEN`

```rust
const PTRACE_LISTEN: crate::c_uint = 16_904u32;
```

### `PTRACE_PEEKSIGINFO`

```rust
const PTRACE_PEEKSIGINFO: crate::c_uint = 16_905u32;
```

### `PTRACE_GETSIGMASK`

```rust
const PTRACE_GETSIGMASK: crate::c_uint = 16_906u32;
```

### `PTRACE_SETSIGMASK`

```rust
const PTRACE_SETSIGMASK: crate::c_uint = 16_907u32;
```

### `PTRACE_GET_SYSCALL_INFO`

```rust
const PTRACE_GET_SYSCALL_INFO: crate::c_uint = 16_910u32;
```

### `PTRACE_SYSCALL_INFO_NONE`

```rust
const PTRACE_SYSCALL_INFO_NONE: crate::__u8 = 0u8;
```

### `PTRACE_SYSCALL_INFO_ENTRY`

```rust
const PTRACE_SYSCALL_INFO_ENTRY: crate::__u8 = 1u8;
```

### `PTRACE_SYSCALL_INFO_EXIT`

```rust
const PTRACE_SYSCALL_INFO_EXIT: crate::__u8 = 2u8;
```

### `PTRACE_SYSCALL_INFO_SECCOMP`

```rust
const PTRACE_SYSCALL_INFO_SECCOMP: crate::__u8 = 3u8;
```

### `PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG`

```rust
const PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG: crate::__u8 = 16u8;
```

### `PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG`

```rust
const PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG: crate::__u8 = 17u8;
```

### `TCA_PAD`

```rust
const TCA_PAD: crate::c_ushort = 9u16;
```

### `TCA_DUMP_INVISIBLE`

```rust
const TCA_DUMP_INVISIBLE: crate::c_ushort = 10u16;
```

### `TCA_CHAIN`

```rust
const TCA_CHAIN: crate::c_ushort = 11u16;
```

### `TCA_HW_OFFLOAD`

```rust
const TCA_HW_OFFLOAD: crate::c_ushort = 12u16;
```

### `RTM_DELNETCONF`

```rust
const RTM_DELNETCONF: u16 = 81u16;
```

### `RTM_NEWSTATS`

```rust
const RTM_NEWSTATS: u16 = 92u16;
```

### `RTM_GETSTATS`

```rust
const RTM_GETSTATS: u16 = 94u16;
```

### `RTM_NEWCACHEREPORT`

```rust
const RTM_NEWCACHEREPORT: u16 = 96u16;
```

### `RTM_F_LOOKUP_TABLE`

```rust
const RTM_F_LOOKUP_TABLE: crate::c_uint = 4_096u32;
```

### `RTM_F_FIB_MATCH`

```rust
const RTM_F_FIB_MATCH: crate::c_uint = 8_192u32;
```

### `RTA_VIA`

```rust
const RTA_VIA: crate::c_ushort = 18u16;
```

### `RTA_NEWDST`

```rust
const RTA_NEWDST: crate::c_ushort = 19u16;
```

### `RTA_PREF`

```rust
const RTA_PREF: crate::c_ushort = 20u16;
```

### `RTA_ENCAP_TYPE`

```rust
const RTA_ENCAP_TYPE: crate::c_ushort = 21u16;
```

### `RTA_ENCAP`

```rust
const RTA_ENCAP: crate::c_ushort = 22u16;
```

### `RTA_EXPIRES`

```rust
const RTA_EXPIRES: crate::c_ushort = 23u16;
```

### `RTA_PAD`

```rust
const RTA_PAD: crate::c_ushort = 24u16;
```

### `RTA_UID`

```rust
const RTA_UID: crate::c_ushort = 25u16;
```

### `RTA_TTL_PROPAGATE`

```rust
const RTA_TTL_PROPAGATE: crate::c_ushort = 26u16;
```

### `NTF_EXT_LEARNED`

```rust
const NTF_EXT_LEARNED: u8 = 16u8;
```

### `NTF_OFFLOADED`

```rust
const NTF_OFFLOADED: u8 = 32u8;
```

### `NDA_MASTER`

```rust
const NDA_MASTER: crate::c_ushort = 9u16;
```

### `NDA_LINK_NETNSID`

```rust
const NDA_LINK_NETNSID: crate::c_ushort = 10u16;
```

### `NDA_SRC_VNI`

```rust
const NDA_SRC_VNI: crate::c_ushort = 11u16;
```

### `UNAME26`

```rust
const UNAME26: crate::c_int = 131_072i32;
```

### `FDPIC_FUNCPTRS`

```rust
const FDPIC_FUNCPTRS: crate::c_int = 524_288i32;
```

### `MAX_LINKS`

```rust
const MAX_LINKS: crate::c_int = 32i32;
```

### `GENL_UNS_ADMIN_PERM`

```rust
const GENL_UNS_ADMIN_PERM: crate::c_int = 16i32;
```

### `GENL_ID_VFS_DQUOT`

```rust
const GENL_ID_VFS_DQUOT: crate::c_int = 17i32;
```

### `GENL_ID_PMCRAID`

```rust
const GENL_ID_PMCRAID: crate::c_int = 18i32;
```

### `ELFOSABI_ARM_AEABI`

```rust
const ELFOSABI_ARM_AEABI: u8 = 64u8;
```

### `CLONE_NEWTIME`

```rust
const CLONE_NEWTIME: crate::c_int = 128i32;
```

### `CLONE_CLEAR_SIGHAND`

```rust
const CLONE_CLEAR_SIGHAND: crate::c_int = 0i32;
```

### `CLONE_INTO_CGROUP`

```rust
const CLONE_INTO_CGROUP: crate::c_int = 0i32;
```

### `M_MXFAST`

```rust
const M_MXFAST: crate::c_int = 1i32;
```

### `M_NLBLKS`

```rust
const M_NLBLKS: crate::c_int = 2i32;
```

### `M_GRAIN`

```rust
const M_GRAIN: crate::c_int = 3i32;
```

### `M_KEEP`

```rust
const M_KEEP: crate::c_int = 4i32;
```

### `M_TRIM_THRESHOLD`

```rust
const M_TRIM_THRESHOLD: crate::c_int = -1i32;
```

### `M_TOP_PAD`

```rust
const M_TOP_PAD: crate::c_int = -2i32;
```

### `M_MMAP_THRESHOLD`

```rust
const M_MMAP_THRESHOLD: crate::c_int = -3i32;
```

### `M_MMAP_MAX`

```rust
const M_MMAP_MAX: crate::c_int = -4i32;
```

### `M_CHECK_ACTION`

```rust
const M_CHECK_ACTION: crate::c_int = -5i32;
```

### `M_PERTURB`

```rust
const M_PERTURB: crate::c_int = -6i32;
```

### `M_ARENA_TEST`

```rust
const M_ARENA_TEST: crate::c_int = -7i32;
```

### `M_ARENA_MAX`

```rust
const M_ARENA_MAX: crate::c_int = -8i32;
```

### `SOMAXCONN`

```rust
const SOMAXCONN: crate::c_int = 4_096i32;
```

### `MOVE_MOUNT_F_SYMLINKS`

```rust
const MOVE_MOUNT_F_SYMLINKS: crate::c_uint = 1u32;
```

### `MOVE_MOUNT_F_AUTOMOUNTS`

```rust
const MOVE_MOUNT_F_AUTOMOUNTS: crate::c_uint = 2u32;
```

### `MOVE_MOUNT_F_EMPTY_PATH`

```rust
const MOVE_MOUNT_F_EMPTY_PATH: crate::c_uint = 4u32;
```

### `MOVE_MOUNT_T_SYMLINKS`

```rust
const MOVE_MOUNT_T_SYMLINKS: crate::c_uint = 16u32;
```

### `MOVE_MOUNT_T_AUTOMOUNTS`

```rust
const MOVE_MOUNT_T_AUTOMOUNTS: crate::c_uint = 32u32;
```

### `MOVE_MOUNT_T_EMPTY_PATH`

```rust
const MOVE_MOUNT_T_EMPTY_PATH: crate::c_uint = 64u32;
```

### `MOVE_MOUNT_SET_GROUP`

```rust
const MOVE_MOUNT_SET_GROUP: crate::c_uint = 256u32;
```

### `MOVE_MOUNT_BENEATH`

```rust
const MOVE_MOUNT_BENEATH: crate::c_uint = 512u32;
```

### `ADJ_OFFSET`

```rust
const ADJ_OFFSET: crate::c_uint = 1u32;
```

### `ADJ_FREQUENCY`

```rust
const ADJ_FREQUENCY: crate::c_uint = 2u32;
```

### `ADJ_MAXERROR`

```rust
const ADJ_MAXERROR: crate::c_uint = 4u32;
```

### `ADJ_ESTERROR`

```rust
const ADJ_ESTERROR: crate::c_uint = 8u32;
```

### `ADJ_STATUS`

```rust
const ADJ_STATUS: crate::c_uint = 16u32;
```

### `ADJ_TIMECONST`

```rust
const ADJ_TIMECONST: crate::c_uint = 32u32;
```

### `ADJ_TAI`

```rust
const ADJ_TAI: crate::c_uint = 128u32;
```

### `ADJ_SETOFFSET`

```rust
const ADJ_SETOFFSET: crate::c_uint = 256u32;
```

### `ADJ_MICRO`

```rust
const ADJ_MICRO: crate::c_uint = 4_096u32;
```

### `ADJ_NANO`

```rust
const ADJ_NANO: crate::c_uint = 8_192u32;
```

### `ADJ_TICK`

```rust
const ADJ_TICK: crate::c_uint = 16_384u32;
```

### `ADJ_OFFSET_SINGLESHOT`

```rust
const ADJ_OFFSET_SINGLESHOT: crate::c_uint = 32_769u32;
```

### `ADJ_OFFSET_SS_READ`

```rust
const ADJ_OFFSET_SS_READ: crate::c_uint = 40_961u32;
```

### `MOD_OFFSET`

```rust
const MOD_OFFSET: crate::c_uint = 1u32;
```

### `MOD_FREQUENCY`

```rust
const MOD_FREQUENCY: crate::c_uint = 2u32;
```

### `MOD_MAXERROR`

```rust
const MOD_MAXERROR: crate::c_uint = 4u32;
```

### `MOD_ESTERROR`

```rust
const MOD_ESTERROR: crate::c_uint = 8u32;
```

### `MOD_STATUS`

```rust
const MOD_STATUS: crate::c_uint = 16u32;
```

### `MOD_TIMECONST`

```rust
const MOD_TIMECONST: crate::c_uint = 32u32;
```

### `MOD_CLKB`

```rust
const MOD_CLKB: crate::c_uint = 16_384u32;
```

### `MOD_CLKA`

```rust
const MOD_CLKA: crate::c_uint = 32_769u32;
```

### `MOD_TAI`

```rust
const MOD_TAI: crate::c_uint = 128u32;
```

### `MOD_MICRO`

```rust
const MOD_MICRO: crate::c_uint = 4_096u32;
```

### `MOD_NANO`

```rust
const MOD_NANO: crate::c_uint = 8_192u32;
```

### `STA_PLL`

```rust
const STA_PLL: crate::c_int = 1i32;
```

### `STA_PPSFREQ`

```rust
const STA_PPSFREQ: crate::c_int = 2i32;
```

### `STA_PPSTIME`

```rust
const STA_PPSTIME: crate::c_int = 4i32;
```

### `STA_FLL`

```rust
const STA_FLL: crate::c_int = 8i32;
```

### `STA_INS`

```rust
const STA_INS: crate::c_int = 16i32;
```

### `STA_DEL`

```rust
const STA_DEL: crate::c_int = 32i32;
```

### `STA_UNSYNC`

```rust
const STA_UNSYNC: crate::c_int = 64i32;
```

### `STA_FREQHOLD`

```rust
const STA_FREQHOLD: crate::c_int = 128i32;
```

### `STA_PPSSIGNAL`

```rust
const STA_PPSSIGNAL: crate::c_int = 256i32;
```

### `STA_PPSJITTER`

```rust
const STA_PPSJITTER: crate::c_int = 512i32;
```

### `STA_PPSWANDER`

```rust
const STA_PPSWANDER: crate::c_int = 1_024i32;
```

### `STA_PPSERROR`

```rust
const STA_PPSERROR: crate::c_int = 2_048i32;
```

### `STA_CLOCKERR`

```rust
const STA_CLOCKERR: crate::c_int = 4_096i32;
```

### `STA_NANO`

```rust
const STA_NANO: crate::c_int = 8_192i32;
```

### `STA_MODE`

```rust
const STA_MODE: crate::c_int = 16_384i32;
```

### `STA_CLK`

```rust
const STA_CLK: crate::c_int = 32_768i32;
```

### `STA_RONLY`

```rust
const STA_RONLY: crate::c_int = 65_280i32;
```

### `NTP_API`

```rust
const NTP_API: crate::c_int = 4i32;
```

### `TIME_OK`

```rust
const TIME_OK: crate::c_int = 0i32;
```

### `TIME_INS`

```rust
const TIME_INS: crate::c_int = 1i32;
```

### `TIME_DEL`

```rust
const TIME_DEL: crate::c_int = 2i32;
```

### `TIME_OOP`

```rust
const TIME_OOP: crate::c_int = 3i32;
```

### `TIME_WAIT`

```rust
const TIME_WAIT: crate::c_int = 4i32;
```

### `TIME_ERROR`

```rust
const TIME_ERROR: crate::c_int = 5i32;
```

### `TIME_BAD`

```rust
const TIME_BAD: crate::c_int = 5i32;
```

### `MAXTC`

```rust
const MAXTC: crate::c_long = 6i64;
```

### `GLOB_PERIOD`

```rust
const GLOB_PERIOD: crate::c_int = 128i32;
```

### `GLOB_ALTDIRFUNC`

```rust
const GLOB_ALTDIRFUNC: crate::c_int = 512i32;
```

### `GLOB_BRACE`

```rust
const GLOB_BRACE: crate::c_int = 1_024i32;
```

### `GLOB_NOMAGIC`

```rust
const GLOB_NOMAGIC: crate::c_int = 2_048i32;
```

### `GLOB_TILDE`

```rust
const GLOB_TILDE: crate::c_int = 4_096i32;
```

### `GLOB_ONLYDIR`

```rust
const GLOB_ONLYDIR: crate::c_int = 8_192i32;
```

### `GLOB_TILDE_CHECK`

```rust
const GLOB_TILDE_CHECK: crate::c_int = 16_384i32;
```

### `MADV_COLLAPSE`

```rust
const MADV_COLLAPSE: crate::c_int = 25i32;
```

### `PTHREAD_STACK_MIN`

```rust
const PTHREAD_STACK_MIN: crate::size_t = 16_384usize;
```

### `PTHREAD_MUTEX_ADAPTIVE_NP`

```rust
const PTHREAD_MUTEX_ADAPTIVE_NP: crate::c_int = 3i32;
```

### `REG_STARTEND`

```rust
const REG_STARTEND: crate::c_int = 4i32;
```

### `REG_EEND`

```rust
const REG_EEND: crate::c_int = 14i32;
```

### `REG_ESIZE`

```rust
const REG_ESIZE: crate::c_int = 15i32;
```

### `REG_ERPAREN`

```rust
const REG_ERPAREN: crate::c_int = 16i32;
```

### `__SIZEOF_PTHREAD_RWLOCKATTR_T`

```rust
const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8usize;
```

### `O_LARGEFILE`

```rust
const O_LARGEFILE: crate::c_int = 0i32;
```

