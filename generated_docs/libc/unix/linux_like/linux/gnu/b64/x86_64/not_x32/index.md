*[libc](../../../../../../../index.md) / [unix](../../../../../../index.md) / [linux_like](../../../../../index.md) / [linux](../../../../index.md) / [gnu](../../../index.md) / [b64](../../index.md) / [x86_64](../index.md) / [not_x32](index.md)*

---

# Module `not_x32`

## Structs

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

- `fn clone(self: &Self) -> statvfs` — [`statvfs`](../index.md)

##### `impl Copy for statvfs`

##### `impl Debug for statvfs`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `sysctl`

```rust
unsafe fn sysctl(name: *mut c_int, namelen: c_int, oldp: *mut c_void, oldlenp: *mut size_t, newp: *mut c_void, newlen: size_t) -> c_int
```

## Constants

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

