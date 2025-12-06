# Crate `libc`

libc - Raw FFI bindings to platforms' system libraries

## Structs

### `group`

```rust
struct group {
    pub gr_name: *mut crate::c_char,
    pub gr_passwd: *mut crate::c_char,
    pub gr_gid: crate::gid_t,
    pub gr_mem: *mut *mut crate::c_char,
}
```

#### Trait Implementations

##### `impl Clone for group`

- `fn clone(self: &Self) -> group` — [`group`](#group)

##### `impl Copy for group`

##### `impl Debug for group`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `utimbuf`

```rust
struct utimbuf {
    pub actime: time_t,
    pub modtime: time_t,
}
```

#### Trait Implementations

##### `impl Clone for utimbuf`

- `fn clone(self: &Self) -> utimbuf` — [`utimbuf`](#utimbuf)

##### `impl Copy for utimbuf`

##### `impl Debug for utimbuf`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `timeval`

```rust
struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: crate::suseconds_t,
}
```

#### Trait Implementations

##### `impl Clone for timeval`

- `fn clone(self: &Self) -> timeval` — [`timeval`](#timeval)

##### `impl Copy for timeval`

##### `impl Debug for timeval`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `rlimit`

```rust
struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
```

#### Trait Implementations

##### `impl Clone for rlimit`

- `fn clone(self: &Self) -> rlimit` — [`rlimit`](#rlimit)

##### `impl Copy for rlimit`

##### `impl Debug for rlimit`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `rusage`

```rust
struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub ru_maxrss: crate::c_long,
    pub ru_ixrss: crate::c_long,
    pub ru_idrss: crate::c_long,
    pub ru_isrss: crate::c_long,
    pub ru_minflt: crate::c_long,
    pub ru_majflt: crate::c_long,
    pub ru_nswap: crate::c_long,
    pub ru_inblock: crate::c_long,
    pub ru_oublock: crate::c_long,
    pub ru_msgsnd: crate::c_long,
    pub ru_msgrcv: crate::c_long,
    pub ru_nsignals: crate::c_long,
    pub ru_nvcsw: crate::c_long,
    pub ru_nivcsw: crate::c_long,
}
```

#### Trait Implementations

##### `impl Clone for rusage`

- `fn clone(self: &Self) -> rusage` — [`rusage`](#rusage)

##### `impl Copy for rusage`

##### `impl Debug for rusage`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ipv6_mreq`

```rust
struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for ipv6_mreq`

- `fn clone(self: &Self) -> ipv6_mreq` — [`ipv6_mreq`](#ipv6-mreq)

##### `impl Copy for ipv6_mreq`

##### `impl Debug for ipv6_mreq`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `hostent`

```rust
struct hostent {
    pub h_name: *mut crate::c_char,
    pub h_aliases: *mut *mut crate::c_char,
    pub h_addrtype: crate::c_int,
    pub h_length: crate::c_int,
    pub h_addr_list: *mut *mut crate::c_char,
}
```

#### Trait Implementations

##### `impl Clone for hostent`

- `fn clone(self: &Self) -> hostent` — [`hostent`](#hostent)

##### `impl Copy for hostent`

##### `impl Debug for hostent`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iovec`

```rust
struct iovec {
    pub iov_base: *mut crate::c_void,
    pub iov_len: size_t,
}
```

#### Trait Implementations

##### `impl Clone for iovec`

- `fn clone(self: &Self) -> iovec` — [`iovec`](#iovec)

##### `impl Copy for iovec`

##### `impl Debug for iovec`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pollfd`

```rust
struct pollfd {
    pub fd: crate::c_int,
    pub events: crate::c_short,
    pub revents: crate::c_short,
}
```

#### Trait Implementations

##### `impl Clone for pollfd`

- `fn clone(self: &Self) -> pollfd` — [`pollfd`](#pollfd)

##### `impl Copy for pollfd`

##### `impl Debug for pollfd`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `winsize`

```rust
struct winsize {
    pub ws_row: crate::c_ushort,
    pub ws_col: crate::c_ushort,
    pub ws_xpixel: crate::c_ushort,
    pub ws_ypixel: crate::c_ushort,
}
```

#### Trait Implementations

##### `impl Clone for winsize`

- `fn clone(self: &Self) -> winsize` — [`winsize`](#winsize)

##### `impl Copy for winsize`

##### `impl Debug for winsize`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `linger`

```rust
struct linger {
    pub l_onoff: crate::c_int,
    pub l_linger: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for linger`

- `fn clone(self: &Self) -> linger` — [`linger`](#linger)

##### `impl Copy for linger`

##### `impl Debug for linger`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sigval`

```rust
struct sigval {
    pub sival_ptr: *mut crate::c_void,
}
```

#### Trait Implementations

##### `impl Clone for sigval`

- `fn clone(self: &Self) -> sigval` — [`sigval`](#sigval)

##### `impl Copy for sigval`

##### `impl Debug for sigval`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `itimerval`

```rust
struct itimerval {
    pub it_interval: crate::timeval,
    pub it_value: crate::timeval,
}
```

#### Trait Implementations

##### `impl Clone for itimerval`

- `fn clone(self: &Self) -> itimerval` — [`itimerval`](#itimerval)

##### `impl Copy for itimerval`

##### `impl Debug for itimerval`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tms`

```rust
struct tms {
    pub tms_utime: crate::clock_t,
    pub tms_stime: crate::clock_t,
    pub tms_cutime: crate::clock_t,
    pub tms_cstime: crate::clock_t,
}
```

#### Trait Implementations

##### `impl Clone for tms`

- `fn clone(self: &Self) -> tms` — [`tms`](#tms)

##### `impl Copy for tms`

##### `impl Debug for tms`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `servent`

```rust
struct servent {
    pub s_name: *mut crate::c_char,
    pub s_aliases: *mut *mut crate::c_char,
    pub s_port: crate::c_int,
    pub s_proto: *mut crate::c_char,
}
```

#### Trait Implementations

##### `impl Clone for servent`

- `fn clone(self: &Self) -> servent` — [`servent`](#servent)

##### `impl Copy for servent`

##### `impl Debug for servent`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `protoent`

```rust
struct protoent {
    pub p_name: *mut crate::c_char,
    pub p_aliases: *mut *mut crate::c_char,
    pub p_proto: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for protoent`

- `fn clone(self: &Self) -> protoent` — [`protoent`](#protoent)

##### `impl Copy for protoent`

##### `impl Debug for protoent`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `in6_addr`

```rust
struct in6_addr {
    pub s6_addr: [u8; 16],
}
```

#### Trait Implementations

##### `impl Clone for in6_addr`

- `fn clone(self: &Self) -> in6_addr` — [`in6_addr`](#in6-addr)

##### `impl Copy for in6_addr`

##### `impl Debug for in6_addr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `DIR`

```rust
enum DIR {
}
```

#### Trait Implementations

##### `impl Clone for DIR`

- `fn clone(self: &Self) -> DIR` — [`DIR`](#dir)

##### `impl Copy for DIR`

##### `impl Debug for DIR`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FILE`

```rust
enum FILE {
}
```

#### Trait Implementations

##### `impl Clone for FILE`

- `fn clone(self: &Self) -> FILE` — [`FILE`](#file)

##### `impl Copy for FILE`

##### `impl Debug for FILE`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `isalnum`

```rust
unsafe fn isalnum(c: c_int) -> c_int
```

### `isalpha`

```rust
unsafe fn isalpha(c: c_int) -> c_int
```

### `iscntrl`

```rust
unsafe fn iscntrl(c: c_int) -> c_int
```

### `isdigit`

```rust
unsafe fn isdigit(c: c_int) -> c_int
```

### `isgraph`

```rust
unsafe fn isgraph(c: c_int) -> c_int
```

### `islower`

```rust
unsafe fn islower(c: c_int) -> c_int
```

### `isprint`

```rust
unsafe fn isprint(c: c_int) -> c_int
```

### `ispunct`

```rust
unsafe fn ispunct(c: c_int) -> c_int
```

### `isspace`

```rust
unsafe fn isspace(c: c_int) -> c_int
```

### `isupper`

```rust
unsafe fn isupper(c: c_int) -> c_int
```

### `isxdigit`

```rust
unsafe fn isxdigit(c: c_int) -> c_int
```

### `isblank`

```rust
unsafe fn isblank(c: c_int) -> c_int
```

### `tolower`

```rust
unsafe fn tolower(c: c_int) -> c_int
```

### `toupper`

```rust
unsafe fn toupper(c: c_int) -> c_int
```

### `qsort`

```rust
unsafe fn qsort(base: *mut c_void, num: size_t, size: size_t, compar: Option<fn(*const c_void, *const c_void) -> c_int>)
```

### `bsearch`

```rust
unsafe fn bsearch(key: *const c_void, base: *const c_void, num: size_t, size: size_t, compar: Option<fn(*const c_void, *const c_void) -> c_int>) -> *mut c_void
```

### `fopen`

```rust
unsafe fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE
```

### `freopen`

```rust
unsafe fn freopen(filename: *const c_char, mode: *const c_char, file: *mut FILE) -> *mut FILE
```

### `fflush`

```rust
unsafe fn fflush(file: *mut FILE) -> c_int
```

### `fclose`

```rust
unsafe fn fclose(file: *mut FILE) -> c_int
```

### `remove`

```rust
unsafe fn remove(filename: *const c_char) -> c_int
```

### `rename`

```rust
unsafe fn rename(oldname: *const c_char, newname: *const c_char) -> c_int
```

### `tmpfile`

```rust
unsafe fn tmpfile() -> *mut FILE
```

### `setvbuf`

```rust
unsafe fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int, size: size_t) -> c_int
```

### `setbuf`

```rust
unsafe fn setbuf(stream: *mut FILE, buf: *mut c_char)
```

### `getchar`

```rust
unsafe fn getchar() -> c_int
```

### `putchar`

```rust
unsafe fn putchar(c: c_int) -> c_int
```

### `fgetc`

```rust
unsafe fn fgetc(stream: *mut FILE) -> c_int
```

### `fgets`

```rust
unsafe fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char
```

### `fputc`

```rust
unsafe fn fputc(c: c_int, stream: *mut FILE) -> c_int
```

### `fputs`

```rust
unsafe fn fputs(s: *const c_char, stream: *mut FILE) -> c_int
```

### `puts`

```rust
unsafe fn puts(s: *const c_char) -> c_int
```

### `ungetc`

```rust
unsafe fn ungetc(c: c_int, stream: *mut FILE) -> c_int
```

### `fread`

```rust
unsafe fn fread(ptr: *mut c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t
```

### `fwrite`

```rust
unsafe fn fwrite(ptr: *const c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t
```

### `fseek`

```rust
unsafe fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int
```

### `ftell`

```rust
unsafe fn ftell(stream: *mut FILE) -> c_long
```

### `rewind`

```rust
unsafe fn rewind(stream: *mut FILE)
```

### `fgetpos`

```rust
unsafe fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int
```

### `fsetpos`

```rust
unsafe fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int
```

### `feof`

```rust
unsafe fn feof(stream: *mut FILE) -> c_int
```

### `ferror`

```rust
unsafe fn ferror(stream: *mut FILE) -> c_int
```

### `clearerr`

```rust
unsafe fn clearerr(stream: *mut FILE)
```

### `perror`

```rust
unsafe fn perror(s: *const c_char)
```

### `atof`

```rust
unsafe fn atof(s: *const c_char) -> c_double
```

### `atoi`

```rust
unsafe fn atoi(s: *const c_char) -> c_int
```

### `atol`

```rust
unsafe fn atol(s: *const c_char) -> c_long
```

### `atoll`

```rust
unsafe fn atoll(s: *const c_char) -> c_longlong
```

### `strtod`

```rust
unsafe fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double
```

### `strtof`

```rust
unsafe fn strtof(s: *const c_char, endp: *mut *mut c_char) -> c_float
```

### `strtol`

```rust
unsafe fn strtol(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_long
```

### `strtoll`

```rust
unsafe fn strtoll(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_longlong
```

### `strtoul`

```rust
unsafe fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong
```

### `strtoull`

```rust
unsafe fn strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulonglong
```

### `calloc`

```rust
unsafe fn calloc(nobj: size_t, size: size_t) -> *mut c_void
```

### `malloc`

```rust
unsafe fn malloc(size: size_t) -> *mut c_void
```

### `realloc`

```rust
unsafe fn realloc(p: *mut c_void, size: size_t) -> *mut c_void
```

### `free`

```rust
unsafe fn free(p: *mut c_void)
```

### `abort`

```rust
unsafe fn abort() -> never
```

### `exit`

```rust
unsafe fn exit(status: c_int) -> never
```

### `_exit`

```rust
unsafe fn _exit(status: c_int) -> never
```

### `system`

```rust
unsafe fn system(s: *const c_char) -> c_int
```

### `getenv`

```rust
unsafe fn getenv(s: *const c_char) -> *mut c_char
```

### `strcpy`

```rust
unsafe fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char
```

### `strncpy`

```rust
unsafe fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char
```

### `stpcpy`

```rust
unsafe fn stpcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char
```

### `strcat`

```rust
unsafe fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char
```

### `strncat`

```rust
unsafe fn strncat(s: *mut c_char, ct: *const c_char, n: size_t) -> *mut c_char
```

### `strcmp`

```rust
unsafe fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int
```

### `strncmp`

```rust
unsafe fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int
```

### `strcoll`

```rust
unsafe fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int
```

### `strchr`

```rust
unsafe fn strchr(cs: *const c_char, c: c_int) -> *mut c_char
```

### `strrchr`

```rust
unsafe fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char
```

### `strspn`

```rust
unsafe fn strspn(cs: *const c_char, ct: *const c_char) -> size_t
```

### `strcspn`

```rust
unsafe fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t
```

### `strdup`

```rust
unsafe fn strdup(cs: *const c_char) -> *mut c_char
```

### `strndup`

```rust
unsafe fn strndup(cs: *const c_char, n: size_t) -> *mut c_char
```

### `strpbrk`

```rust
unsafe fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

### `strstr`

```rust
unsafe fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

### `strcasecmp`

```rust
unsafe fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int
```

### `strncasecmp`

```rust
unsafe fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int
```

### `strlen`

```rust
unsafe fn strlen(cs: *const c_char) -> size_t
```

### `strnlen`

```rust
unsafe fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t
```

### `strerror`

```rust
unsafe fn strerror(n: c_int) -> *mut c_char
```

### `strtok`

```rust
unsafe fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char
```

### `strtok_r`

```rust
unsafe fn strtok_r(s: *mut c_char, t: *const c_char, p: *mut *mut c_char) -> *mut c_char
```

### `strxfrm`

```rust
unsafe fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t
```

### `strsignal`

```rust
unsafe fn strsignal(sig: c_int) -> *mut c_char
```

### `wcslen`

```rust
unsafe fn wcslen(buf: *const wchar_t) -> size_t
```

### `wcstombs`

```rust
unsafe fn wcstombs(dest: *mut c_char, src: *const wchar_t, n: size_t) -> size_t
```

### `memchr`

```rust
unsafe fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

### `wmemchr`

```rust
unsafe fn wmemchr(cx: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t
```

### `memcmp`

```rust
unsafe fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int
```

### `memcpy`

```rust
unsafe fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```

### `memmove`

```rust
unsafe fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```

### `memset`

```rust
unsafe fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void
```

### `memccpy`

```rust
unsafe fn memccpy(dest: *mut c_void, src: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

### `getpwnam`

```rust
unsafe fn getpwnam(name: *const c_char) -> *mut passwd
```

### `getpwuid`

```rust
unsafe fn getpwuid(uid: crate::uid_t) -> *mut passwd
```

### `fprintf`

```rust
unsafe fn fprintf(stream: *mut crate::FILE, format: *const c_char) -> c_int
```

### `printf`

```rust
unsafe fn printf(format: *const c_char) -> c_int
```

### `snprintf`

```rust
unsafe fn snprintf(s: *mut c_char, n: size_t, format: *const c_char) -> c_int
```

### `sprintf`

```rust
unsafe fn sprintf(s: *mut c_char, format: *const c_char) -> c_int
```

### `fscanf`

```rust
unsafe fn fscanf(stream: *mut crate::FILE, format: *const c_char) -> c_int
```

### `scanf`

```rust
unsafe fn scanf(format: *const c_char) -> c_int
```

### `sscanf`

```rust
unsafe fn sscanf(s: *const c_char, format: *const c_char) -> c_int
```

### `getchar_unlocked`

```rust
unsafe fn getchar_unlocked() -> c_int
```

### `putchar_unlocked`

```rust
unsafe fn putchar_unlocked(c: c_int) -> c_int
```

### `socket`

```rust
unsafe fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int
```

### `connect`

```rust
unsafe fn connect(socket: c_int, address: *const sockaddr, len: socklen_t) -> c_int
```

### `listen`

```rust
unsafe fn listen(socket: c_int, backlog: c_int) -> c_int
```

### `accept`

```rust
unsafe fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

### `getpeername`

```rust
unsafe fn getpeername(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

### `getsockname`

```rust
unsafe fn getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

### `setsockopt`

```rust
unsafe fn setsockopt(socket: c_int, level: c_int, name: c_int, value: *const c_void, option_len: socklen_t) -> c_int
```

### `socketpair`

```rust
unsafe fn socketpair(domain: c_int, type_: c_int, protocol: c_int, socket_vector: *mut c_int) -> c_int
```

### `sendto`

```rust
unsafe fn sendto(socket: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, addrlen: socklen_t) -> ssize_t
```

### `shutdown`

```rust
unsafe fn shutdown(socket: c_int, how: c_int) -> c_int
```

### `chmod`

```rust
unsafe fn chmod(path: *const c_char, mode: mode_t) -> c_int
```

### `fchmod`

```rust
unsafe fn fchmod(fd: c_int, mode: mode_t) -> c_int
```

### `fstat`

```rust
unsafe fn fstat(fildes: c_int, buf: *mut stat) -> c_int
```

### `mkdir`

```rust
unsafe fn mkdir(path: *const c_char, mode: mode_t) -> c_int
```

### `stat`

```rust
unsafe fn stat(path: *const c_char, buf: *mut stat) -> c_int
```

### `pclose`

```rust
unsafe fn pclose(stream: *mut crate::FILE) -> c_int
```

### `fdopen`

```rust
unsafe fn fdopen(fd: c_int, mode: *const c_char) -> *mut crate::FILE
```

### `fileno`

```rust
unsafe fn fileno(stream: *mut crate::FILE) -> c_int
```

### `open`

```rust
unsafe fn open(path: *const c_char, oflag: c_int) -> c_int
```

### `creat`

```rust
unsafe fn creat(path: *const c_char, mode: mode_t) -> c_int
```

### `fcntl`

```rust
unsafe fn fcntl(fd: c_int, cmd: c_int) -> c_int
```

### `opendir`

```rust
unsafe fn opendir(dirname: *const c_char) -> *mut crate::DIR
```

### `readdir`

```rust
unsafe fn readdir(dirp: *mut crate::DIR) -> *mut crate::dirent
```

### `closedir`

```rust
unsafe fn closedir(dirp: *mut crate::DIR) -> c_int
```

### `rewinddir`

```rust
unsafe fn rewinddir(dirp: *mut crate::DIR)
```

### `fchmodat`

```rust
unsafe fn fchmodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, flags: c_int) -> c_int
```

### `fchown`

```rust
unsafe fn fchown(fd: c_int, owner: crate::uid_t, group: crate::gid_t) -> c_int
```

### `fchownat`

```rust
unsafe fn fchownat(dirfd: c_int, pathname: *const c_char, owner: crate::uid_t, group: crate::gid_t, flags: c_int) -> c_int
```

### `fstatat`

```rust
unsafe fn fstatat(dirfd: c_int, pathname: *const c_char, buf: *mut stat, flags: c_int) -> c_int
```

### `linkat`

```rust
unsafe fn linkat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_int) -> c_int
```

### `renameat`

```rust
unsafe fn renameat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char) -> c_int
```

### `symlinkat`

```rust
unsafe fn symlinkat(target: *const c_char, newdirfd: c_int, linkpath: *const c_char) -> c_int
```

### `unlinkat`

```rust
unsafe fn unlinkat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int
```

### `access`

```rust
unsafe fn access(path: *const c_char, amode: c_int) -> c_int
```

### `alarm`

```rust
unsafe fn alarm(seconds: c_uint) -> c_uint
```

### `chdir`

```rust
unsafe fn chdir(dir: *const c_char) -> c_int
```

### `fchdir`

```rust
unsafe fn fchdir(dirfd: c_int) -> c_int
```

### `chown`

```rust
unsafe fn chown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int
```

### `lchown`

```rust
unsafe fn lchown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int
```

### `close`

```rust
unsafe fn close(fd: c_int) -> c_int
```

### `dup`

```rust
unsafe fn dup(fd: c_int) -> c_int
```

### `dup2`

```rust
unsafe fn dup2(src: c_int, dst: c_int) -> c_int
```

### `execl`

```rust
unsafe fn execl(path: *const c_char, arg0: *const c_char) -> c_int
```

### `execle`

```rust
unsafe fn execle(path: *const c_char, arg0: *const c_char) -> c_int
```

### `execlp`

```rust
unsafe fn execlp(file: *const c_char, arg0: *const c_char) -> c_int
```

### `execv`

```rust
unsafe fn execv(prog: *const c_char, argv: *const *const c_char) -> c_int
```

### `execve`

```rust
unsafe fn execve(prog: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

### `execvp`

```rust
unsafe fn execvp(c: *const c_char, argv: *const *const c_char) -> c_int
```

### `fork`

```rust
unsafe fn fork() -> pid_t
```

### `fpathconf`

```rust
unsafe fn fpathconf(filedes: c_int, name: c_int) -> c_long
```

### `getcwd`

```rust
unsafe fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char
```

### `getegid`

```rust
unsafe fn getegid() -> gid_t
```

### `geteuid`

```rust
unsafe fn geteuid() -> uid_t
```

### `getgid`

```rust
unsafe fn getgid() -> gid_t
```

### `getgroups`

```rust
unsafe fn getgroups(ngroups_max: c_int, groups: *mut gid_t) -> c_int
```

### `getlogin`

```rust
unsafe fn getlogin() -> *mut c_char
```

### `getopt`

```rust
unsafe fn getopt(argc: c_int, argv: *const *mut c_char, optstr: *const c_char) -> c_int
```

### `getpgid`

```rust
unsafe fn getpgid(pid: pid_t) -> pid_t
```

### `getpgrp`

```rust
unsafe fn getpgrp() -> pid_t
```

### `getpid`

```rust
unsafe fn getpid() -> pid_t
```

### `getppid`

```rust
unsafe fn getppid() -> pid_t
```

### `getuid`

```rust
unsafe fn getuid() -> uid_t
```

### `isatty`

```rust
unsafe fn isatty(fd: c_int) -> c_int
```

### `link`

```rust
unsafe fn link(src: *const c_char, dst: *const c_char) -> c_int
```

### `lseek`

```rust
unsafe fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t
```

### `pathconf`

```rust
unsafe fn pathconf(path: *const c_char, name: c_int) -> c_long
```

### `pipe`

```rust
unsafe fn pipe(fds: *mut c_int) -> c_int
```

### `posix_memalign`

```rust
unsafe fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int
```

### `aligned_alloc`

```rust
unsafe fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void
```

### `read`

```rust
unsafe fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t
```

### `rmdir`

```rust
unsafe fn rmdir(path: *const c_char) -> c_int
```

### `seteuid`

```rust
unsafe fn seteuid(uid: uid_t) -> c_int
```

### `setegid`

```rust
unsafe fn setegid(gid: gid_t) -> c_int
```

### `setgid`

```rust
unsafe fn setgid(gid: gid_t) -> c_int
```

### `setpgid`

```rust
unsafe fn setpgid(pid: pid_t, pgid: pid_t) -> c_int
```

### `setsid`

```rust
unsafe fn setsid() -> pid_t
```

### `setuid`

```rust
unsafe fn setuid(uid: uid_t) -> c_int
```

### `setreuid`

```rust
unsafe fn setreuid(ruid: uid_t, euid: uid_t) -> c_int
```

### `setregid`

```rust
unsafe fn setregid(rgid: gid_t, egid: gid_t) -> c_int
```

### `sleep`

```rust
unsafe fn sleep(secs: c_uint) -> c_uint
```

### `nanosleep`

```rust
unsafe fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int
```

### `tcgetpgrp`

```rust
unsafe fn tcgetpgrp(fd: c_int) -> pid_t
```

### `tcsetpgrp`

```rust
unsafe fn tcsetpgrp(fd: c_int, pgrp: crate::pid_t) -> c_int
```

### `ttyname`

```rust
unsafe fn ttyname(fd: c_int) -> *mut c_char
```

### `ttyname_r`

```rust
unsafe fn ttyname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```

### `unlink`

```rust
unsafe fn unlink(c: *const c_char) -> c_int
```

### `wait`

```rust
unsafe fn wait(status: *mut c_int) -> pid_t
```

### `waitpid`

```rust
unsafe fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t
```

### `write`

```rust
unsafe fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t
```

### `pread`

```rust
unsafe fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t
```

### `pwrite`

```rust
unsafe fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t
```

### `umask`

```rust
unsafe fn umask(mask: mode_t) -> mode_t
```

### `utime`

```rust
unsafe fn utime(file: *const c_char, buf: *const utimbuf) -> c_int
```

### `kill`

```rust
unsafe fn kill(pid: pid_t, sig: c_int) -> c_int
```

### `killpg`

```rust
unsafe fn killpg(pgrp: pid_t, sig: c_int) -> c_int
```

### `mlock`

```rust
unsafe fn mlock(addr: *const c_void, len: size_t) -> c_int
```

### `munlock`

```rust
unsafe fn munlock(addr: *const c_void, len: size_t) -> c_int
```

### `mlockall`

```rust
unsafe fn mlockall(flags: c_int) -> c_int
```

### `munlockall`

```rust
unsafe fn munlockall() -> c_int
```

### `mmap`

```rust
unsafe fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *mut c_void
```

### `munmap`

```rust
unsafe fn munmap(addr: *mut c_void, len: size_t) -> c_int
```

### `if_nametoindex`

```rust
unsafe fn if_nametoindex(ifname: *const c_char) -> c_uint
```

### `if_indextoname`

```rust
unsafe fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char
```

### `lstat`

```rust
unsafe fn lstat(path: *const c_char, buf: *mut stat) -> c_int
```

### `fsync`

```rust
unsafe fn fsync(fd: c_int) -> c_int
```

### `setenv`

```rust
unsafe fn setenv(name: *const c_char, val: *const c_char, overwrite: c_int) -> c_int
```

### `unsetenv`

```rust
unsafe fn unsetenv(name: *const c_char) -> c_int
```

### `symlink`

```rust
unsafe fn symlink(path1: *const c_char, path2: *const c_char) -> c_int
```

### `truncate`

```rust
unsafe fn truncate(path: *const c_char, length: off_t) -> c_int
```

### `ftruncate`

```rust
unsafe fn ftruncate(fd: c_int, length: off_t) -> c_int
```

### `signal`

```rust
unsafe fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t
```

### `getrusage`

```rust
unsafe fn getrusage(resource: c_int, usage: *mut rusage) -> c_int
```

### `realpath`

```rust
unsafe fn realpath(pathname: *const c_char, resolved: *mut c_char) -> *mut c_char
```

### `times`

```rust
unsafe fn times(buf: *mut crate::tms) -> crate::clock_t
```

### `pthread_self`

```rust
unsafe fn pthread_self() -> crate::pthread_t
```

### `pthread_equal`

```rust
unsafe fn pthread_equal(t1: crate::pthread_t, t2: crate::pthread_t) -> c_int
```

### `pthread_join`

```rust
unsafe fn pthread_join(native: crate::pthread_t, value: *mut *mut c_void) -> c_int
```

### `pthread_exit`

```rust
unsafe fn pthread_exit(value: *mut c_void) -> never
```

### `pthread_attr_init`

```rust
unsafe fn pthread_attr_init(attr: *mut crate::pthread_attr_t) -> c_int
```

### `pthread_attr_destroy`

```rust
unsafe fn pthread_attr_destroy(attr: *mut crate::pthread_attr_t) -> c_int
```

### `pthread_attr_getstacksize`

```rust
unsafe fn pthread_attr_getstacksize(attr: *const crate::pthread_attr_t, stacksize: *mut size_t) -> c_int
```

### `pthread_attr_setstacksize`

```rust
unsafe fn pthread_attr_setstacksize(attr: *mut crate::pthread_attr_t, stack_size: size_t) -> c_int
```

### `pthread_attr_setdetachstate`

```rust
unsafe fn pthread_attr_setdetachstate(attr: *mut crate::pthread_attr_t, state: c_int) -> c_int
```

### `pthread_detach`

```rust
unsafe fn pthread_detach(thread: crate::pthread_t) -> c_int
```

### `sched_yield`

```rust
unsafe fn sched_yield() -> c_int
```

### `pthread_key_create`

```rust
unsafe fn pthread_key_create(key: *mut crate::pthread_key_t, dtor: Option<fn(*mut c_void)>) -> c_int
```

### `pthread_key_delete`

```rust
unsafe fn pthread_key_delete(key: crate::pthread_key_t) -> c_int
```

### `pthread_getspecific`

```rust
unsafe fn pthread_getspecific(key: crate::pthread_key_t) -> *mut c_void
```

### `pthread_setspecific`

```rust
unsafe fn pthread_setspecific(key: crate::pthread_key_t, value: *const c_void) -> c_int
```

### `pthread_mutex_init`

```rust
unsafe fn pthread_mutex_init(lock: *mut crate::pthread_mutex_t, attr: *const crate::pthread_mutexattr_t) -> c_int
```

### `pthread_mutex_destroy`

```rust
unsafe fn pthread_mutex_destroy(lock: *mut crate::pthread_mutex_t) -> c_int
```

### `pthread_mutex_lock`

```rust
unsafe fn pthread_mutex_lock(lock: *mut crate::pthread_mutex_t) -> c_int
```

### `pthread_mutex_trylock`

```rust
unsafe fn pthread_mutex_trylock(lock: *mut crate::pthread_mutex_t) -> c_int
```

### `pthread_mutex_unlock`

```rust
unsafe fn pthread_mutex_unlock(lock: *mut crate::pthread_mutex_t) -> c_int
```

### `pthread_mutexattr_init`

```rust
unsafe fn pthread_mutexattr_init(attr: *mut crate::pthread_mutexattr_t) -> c_int
```

### `pthread_mutexattr_destroy`

```rust
unsafe fn pthread_mutexattr_destroy(attr: *mut crate::pthread_mutexattr_t) -> c_int
```

### `pthread_mutexattr_settype`

```rust
unsafe fn pthread_mutexattr_settype(attr: *mut crate::pthread_mutexattr_t, _type: c_int) -> c_int
```

### `pthread_cond_init`

```rust
unsafe fn pthread_cond_init(cond: *mut crate::pthread_cond_t, attr: *const crate::pthread_condattr_t) -> c_int
```

### `pthread_cond_wait`

```rust
unsafe fn pthread_cond_wait(cond: *mut crate::pthread_cond_t, lock: *mut crate::pthread_mutex_t) -> c_int
```

### `pthread_cond_timedwait`

```rust
unsafe fn pthread_cond_timedwait(cond: *mut crate::pthread_cond_t, lock: *mut crate::pthread_mutex_t, abstime: *const crate::timespec) -> c_int
```

### `pthread_cond_signal`

```rust
unsafe fn pthread_cond_signal(cond: *mut crate::pthread_cond_t) -> c_int
```

### `pthread_cond_broadcast`

```rust
unsafe fn pthread_cond_broadcast(cond: *mut crate::pthread_cond_t) -> c_int
```

### `pthread_cond_destroy`

```rust
unsafe fn pthread_cond_destroy(cond: *mut crate::pthread_cond_t) -> c_int
```

### `pthread_condattr_init`

```rust
unsafe fn pthread_condattr_init(attr: *mut crate::pthread_condattr_t) -> c_int
```

### `pthread_condattr_destroy`

```rust
unsafe fn pthread_condattr_destroy(attr: *mut crate::pthread_condattr_t) -> c_int
```

### `pthread_rwlock_init`

```rust
unsafe fn pthread_rwlock_init(lock: *mut crate::pthread_rwlock_t, attr: *const crate::pthread_rwlockattr_t) -> c_int
```

### `pthread_rwlock_destroy`

```rust
unsafe fn pthread_rwlock_destroy(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlock_rdlock`

```rust
unsafe fn pthread_rwlock_rdlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlock_tryrdlock`

```rust
unsafe fn pthread_rwlock_tryrdlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlock_wrlock`

```rust
unsafe fn pthread_rwlock_wrlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlock_trywrlock`

```rust
unsafe fn pthread_rwlock_trywrlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlock_unlock`

```rust
unsafe fn pthread_rwlock_unlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

### `pthread_rwlockattr_init`

```rust
unsafe fn pthread_rwlockattr_init(attr: *mut crate::pthread_rwlockattr_t) -> c_int
```

### `pthread_rwlockattr_destroy`

```rust
unsafe fn pthread_rwlockattr_destroy(attr: *mut crate::pthread_rwlockattr_t) -> c_int
```

### `getsockopt`

```rust
unsafe fn getsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut crate::socklen_t) -> c_int
```

### `raise`

```rust
unsafe fn raise(signum: c_int) -> c_int
```

### `utimes`

```rust
unsafe fn utimes(filename: *const c_char, times: *const crate::timeval) -> c_int
```

### `dlopen`

```rust
unsafe fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void
```

### `dlerror`

```rust
unsafe fn dlerror() -> *mut c_char
```

### `dlsym`

```rust
unsafe fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void
```

### `dlclose`

```rust
unsafe fn dlclose(handle: *mut c_void) -> c_int
```

### `getaddrinfo`

```rust
unsafe fn getaddrinfo(node: *const c_char, service: *const c_char, hints: *const addrinfo, res: *mut *mut addrinfo) -> c_int
```

### `freeaddrinfo`

```rust
unsafe fn freeaddrinfo(res: *mut addrinfo)
```

### `hstrerror`

```rust
unsafe fn hstrerror(errcode: c_int) -> *const c_char
```

### `gai_strerror`

```rust
unsafe fn gai_strerror(errcode: c_int) -> *const c_char
```

### `res_init`

```rust
unsafe fn res_init() -> c_int
```

### `gmtime_r`

```rust
unsafe fn gmtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm
```

### `localtime_r`

```rust
unsafe fn localtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm
```

### `mktime`

```rust
unsafe fn mktime(tm: *mut tm) -> time_t
```

### `time`

```rust
unsafe fn time(time: *mut time_t) -> time_t
```

### `gmtime`

```rust
unsafe fn gmtime(time_p: *const time_t) -> *mut tm
```

### `localtime`

```rust
unsafe fn localtime(time_p: *const time_t) -> *mut tm
```

### `difftime`

```rust
unsafe fn difftime(time1: time_t, time0: time_t) -> c_double
```

### `timegm`

```rust
unsafe fn timegm(tm: *mut crate::tm) -> time_t
```

### `mknod`

```rust
unsafe fn mknod(pathname: *const c_char, mode: mode_t, dev: crate::dev_t) -> c_int
```

### `gethostname`

```rust
unsafe fn gethostname(name: *mut c_char, len: size_t) -> c_int
```

### `endservent`

```rust
unsafe fn endservent()
```

### `getservbyname`

```rust
unsafe fn getservbyname(name: *const c_char, proto: *const c_char) -> *mut servent
```

### `getservbyport`

```rust
unsafe fn getservbyport(port: c_int, proto: *const c_char) -> *mut servent
```

### `getservent`

```rust
unsafe fn getservent() -> *mut servent
```

### `setservent`

```rust
unsafe fn setservent(stayopen: c_int)
```

### `getprotobyname`

```rust
unsafe fn getprotobyname(name: *const c_char) -> *mut protoent
```

### `getprotobynumber`

```rust
unsafe fn getprotobynumber(proto: c_int) -> *mut protoent
```

### `chroot`

```rust
unsafe fn chroot(name: *const c_char) -> c_int
```

### `usleep`

```rust
unsafe fn usleep(secs: c_uint) -> c_int
```

### `send`

```rust
unsafe fn send(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t
```

### `recv`

```rust
unsafe fn recv(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t
```

### `putenv`

```rust
unsafe fn putenv(string: *mut c_char) -> c_int
```

### `poll`

```rust
unsafe fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int
```

### `select`

```rust
unsafe fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *mut timeval) -> c_int
```

### `setlocale`

```rust
unsafe fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char
```

### `localeconv`

```rust
unsafe fn localeconv() -> *mut lconv
```

### `sem_wait`

```rust
unsafe fn sem_wait(sem: *mut sem_t) -> c_int
```

### `sem_trywait`

```rust
unsafe fn sem_trywait(sem: *mut sem_t) -> c_int
```

### `sem_post`

```rust
unsafe fn sem_post(sem: *mut sem_t) -> c_int
```

### `statvfs`

```rust
unsafe fn statvfs(path: *const c_char, buf: *mut crate::statvfs) -> c_int
```

### `fstatvfs`

```rust
unsafe fn fstatvfs(fd: c_int, buf: *mut crate::statvfs) -> c_int
```

### `sigemptyset`

```rust
unsafe fn sigemptyset(set: *mut sigset_t) -> c_int
```

### `sigaddset`

```rust
unsafe fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int
```

### `sigfillset`

```rust
unsafe fn sigfillset(set: *mut sigset_t) -> c_int
```

### `sigdelset`

```rust
unsafe fn sigdelset(set: *mut sigset_t, signum: c_int) -> c_int
```

### `sigismember`

```rust
unsafe fn sigismember(set: *const sigset_t, signum: c_int) -> c_int
```

### `sigprocmask`

```rust
unsafe fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int
```

### `sigpending`

```rust
unsafe fn sigpending(set: *mut sigset_t) -> c_int
```

### `sysconf`

```rust
unsafe fn sysconf(name: c_int) -> c_long
```

### `mkfifo`

```rust
unsafe fn mkfifo(path: *const c_char, mode: mode_t) -> c_int
```

### `fseeko`

```rust
unsafe fn fseeko(stream: *mut crate::FILE, offset: off_t, whence: c_int) -> c_int
```

### `ftello`

```rust
unsafe fn ftello(stream: *mut crate::FILE) -> off_t
```

### `tcdrain`

```rust
unsafe fn tcdrain(fd: c_int) -> c_int
```

### `cfgetispeed`

```rust
unsafe fn cfgetispeed(termios: *const crate::termios) -> crate::speed_t
```

### `cfgetospeed`

```rust
unsafe fn cfgetospeed(termios: *const crate::termios) -> crate::speed_t
```

### `cfsetispeed`

```rust
unsafe fn cfsetispeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

### `cfsetospeed`

```rust
unsafe fn cfsetospeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

### `tcgetattr`

```rust
unsafe fn tcgetattr(fd: c_int, termios: *mut crate::termios) -> c_int
```

### `tcsetattr`

```rust
unsafe fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const crate::termios) -> c_int
```

### `tcflow`

```rust
unsafe fn tcflow(fd: c_int, action: c_int) -> c_int
```

### `tcflush`

```rust
unsafe fn tcflush(fd: c_int, action: c_int) -> c_int
```

### `tcgetsid`

```rust
unsafe fn tcgetsid(fd: c_int) -> crate::pid_t
```

### `tcsendbreak`

```rust
unsafe fn tcsendbreak(fd: c_int, duration: c_int) -> c_int
```

### `mkstemp`

```rust
unsafe fn mkstemp(template: *mut c_char) -> c_int
```

### `mkdtemp`

```rust
unsafe fn mkdtemp(template: *mut c_char) -> *mut c_char
```

### `tmpnam`

```rust
unsafe fn tmpnam(ptr: *mut c_char) -> *mut c_char
```

### `openlog`

```rust
unsafe fn openlog(ident: *const c_char, logopt: c_int, facility: c_int)
```

### `closelog`

```rust
unsafe fn closelog()
```

### `setlogmask`

```rust
unsafe fn setlogmask(maskpri: c_int) -> c_int
```

### `syslog`

```rust
unsafe fn syslog(priority: c_int, message: *const c_char)
```

### `nice`

```rust
unsafe fn nice(incr: c_int) -> c_int
```

### `grantpt`

```rust
unsafe fn grantpt(fd: c_int) -> c_int
```

### `posix_openpt`

```rust
unsafe fn posix_openpt(flags: c_int) -> c_int
```

### `ptsname`

```rust
unsafe fn ptsname(fd: c_int) -> *mut c_char
```

### `unlockpt`

```rust
unsafe fn unlockpt(fd: c_int) -> c_int
```

### `strcasestr`

```rust
unsafe fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

### `getline`

```rust
unsafe fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t
```

### `lockf`

```rust
unsafe fn lockf(fd: c_int, cmd: c_int, len: off_t) -> c_int
```

### `adjtime`

```rust
unsafe fn adjtime(delta: *const timeval, olddelta: *mut timeval) -> c_int
```

### `stpncpy`

```rust
unsafe fn stpncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char
```

### `sigqueue`

```rust
unsafe fn sigqueue(pid: pid_t, sig: c_int, value: crate::sigval) -> c_int
```

### `confstr`

```rust
unsafe fn confstr(name: c_int, buf: *mut c_char, len: size_t) -> size_t
```

### `dladdr`

```rust
unsafe fn dladdr(addr: *const c_void, info: *mut Dl_info) -> c_int
```

### `flock`

```rust
unsafe fn flock(fd: c_int, operation: c_int) -> c_int
```

### `open_wmemstream`

```rust
unsafe fn open_wmemstream(ptr: *mut *mut wchar_t, sizeloc: *mut size_t) -> *mut FILE
```

### `getsid`

```rust
unsafe fn getsid(pid: pid_t) -> pid_t
```

### `pause`

```rust
unsafe fn pause() -> c_int
```

### `mkdirat`

```rust
unsafe fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: mode_t) -> c_int
```

### `openat`

```rust
unsafe fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int
```

### `fdopendir`

```rust
unsafe fn fdopendir(fd: c_int) -> *mut crate::DIR
```

### `readdir_r`

```rust
unsafe fn readdir_r(dirp: *mut crate::DIR, entry: *mut crate::dirent, result: *mut *mut crate::dirent) -> c_int
```

The 64-bit libc on Solaris and illumos only has readdir_r. If a
32-bit Solaris or illumos target is ever created, it should use
__posix_readdir_r. See libc(3LIB) on Solaris or illumos:
https://illumos.org/man/3lib/libc
https://docs.oracle.com/cd/E36784_01/html/E36873/libc-3lib.html
https://www.unix.com/man-page/opensolaris/3LIB/libc/

### `readlinkat`

```rust
unsafe fn readlinkat(dirfd: c_int, pathname: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t
```

### `fmemopen`

```rust
unsafe fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE
```

### `open_memstream`

```rust
unsafe fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE
```

### `atexit`

```rust
unsafe fn atexit(cb: fn()) -> c_int
```

### `sigaction`

```rust
unsafe fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int
```

### `readlink`

```rust
unsafe fn readlink(path: *const c_char, buf: *mut c_char, bufsz: size_t) -> ssize_t
```

### `pselect`

```rust
unsafe fn pselect(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *const timespec, sigmask: *const sigset_t) -> c_int
```

### `cfmakeraw`

```rust
unsafe fn cfmakeraw(termios: *mut crate::termios)
```

### `cfsetspeed`

```rust
unsafe fn cfsetspeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

### `fnmatch`

```rust
unsafe fn fnmatch(pattern: *const c_char, name: *const c_char, flags: c_int) -> c_int
```

### `htonl`

```rust
const fn htonl(hostlong: u32) -> u32
```

### `htons`

```rust
const fn htons(hostshort: u16) -> u16
```

### `ntohl`

```rust
const fn ntohl(netlong: u32) -> u32
```

### `ntohs`

```rust
const fn ntohs(netshort: u16) -> u16
```

## Type Aliases

### `c_schar`

```rust
type c_schar = i8;
```

### `c_uchar`

```rust
type c_uchar = u8;
```

### `c_short`

```rust
type c_short = i16;
```

### `c_ushort`

```rust
type c_ushort = u16;
```

### `c_longlong`

```rust
type c_longlong = i64;
```

### `c_ulonglong`

```rust
type c_ulonglong = u64;
```

### `c_float`

```rust
type c_float = f32;
```

### `c_double`

```rust
type c_double = f64;
```

### `c_char`

```rust
type c_char = i8;
```

### `c_int`

```rust
type c_int = i32;
```

### `c_uint`

```rust
type c_uint = u32;
```

### `c_long`

```rust
type c_long = i64;
```

### `c_ulong`

```rust
type c_ulong = u64;
```

### `int8_t`

```rust
type int8_t = i8;
```

### `int16_t`

```rust
type int16_t = i16;
```

### `int32_t`

```rust
type int32_t = i32;
```

### `int64_t`

```rust
type int64_t = i64;
```

### `uint8_t`

```rust
type uint8_t = u8;
```

### `uint16_t`

```rust
type uint16_t = u16;
```

### `uint32_t`

```rust
type uint32_t = u32;
```

### `uint64_t`

```rust
type uint64_t = u64;
```

### `intmax_t`

```rust
type intmax_t = i64;
```

### `uintmax_t`

```rust
type uintmax_t = u64;
```

### `size_t`

```rust
type size_t = usize;
```

### `ptrdiff_t`

```rust
type ptrdiff_t = isize;
```

### `intptr_t`

```rust
type intptr_t = isize;
```

### `uintptr_t`

```rust
type uintptr_t = usize;
```

### `ssize_t`

```rust
type ssize_t = isize;
```

### `pid_t`

```rust
type pid_t = i32;
```

### `in_addr_t`

```rust
type in_addr_t = u32;
```

### `in_port_t`

```rust
type in_port_t = u16;
```

### `sighandler_t`

```rust
type sighandler_t = size_t;
```

### `cc_t`

```rust
type cc_t = crate::c_uchar;
```

### `uid_t`

```rust
type uid_t = u32;
```

### `gid_t`

```rust
type gid_t = u32;
```

### `locale_t`

```rust
type locale_t = *mut crate::c_void;
```

## Constants

### `INT_MIN`

```rust
const INT_MIN: crate::c_int = -2_147_483_648i32;
```

### `INT_MAX`

```rust
const INT_MAX: crate::c_int = 2_147_483_647i32;
```

### `SIG_DFL`

```rust
const SIG_DFL: sighandler_t = 0usize;
```

### `SIG_IGN`

```rust
const SIG_IGN: sighandler_t = 1usize;
```

### `SIG_ERR`

```rust
const SIG_ERR: sighandler_t = 18_446_744_073_709_551_615usize;
```

### `DT_UNKNOWN`

```rust
const DT_UNKNOWN: u8 = 0u8;
```

### `DT_FIFO`

```rust
const DT_FIFO: u8 = 1u8;
```

### `DT_CHR`

```rust
const DT_CHR: u8 = 2u8;
```

### `DT_DIR`

```rust
const DT_DIR: u8 = 4u8;
```

### `DT_BLK`

```rust
const DT_BLK: u8 = 6u8;
```

### `DT_REG`

```rust
const DT_REG: u8 = 8u8;
```

### `DT_LNK`

```rust
const DT_LNK: u8 = 10u8;
```

### `DT_SOCK`

```rust
const DT_SOCK: u8 = 12u8;
```

### `FD_CLOEXEC`

```rust
const FD_CLOEXEC: crate::c_int = 1i32;
```

### `USRQUOTA`

```rust
const USRQUOTA: crate::c_int = 0i32;
```

### `GRPQUOTA`

```rust
const GRPQUOTA: crate::c_int = 1i32;
```

### `SIGIOT`

```rust
const SIGIOT: crate::c_int = 6i32;
```

### `S_ISUID`

```rust
const S_ISUID: mode_t = 2_048u32;
```

### `S_ISGID`

```rust
const S_ISGID: mode_t = 1_024u32;
```

### `S_ISVTX`

```rust
const S_ISVTX: mode_t = 512u32;
```

### `IF_NAMESIZE`

```rust
const IF_NAMESIZE: size_t = 16usize;
```

### `IFNAMSIZ`

```rust
const IFNAMSIZ: size_t = 16usize;
```

### `LOG_EMERG`

```rust
const LOG_EMERG: crate::c_int = 0i32;
```

### `LOG_ALERT`

```rust
const LOG_ALERT: crate::c_int = 1i32;
```

### `LOG_CRIT`

```rust
const LOG_CRIT: crate::c_int = 2i32;
```

### `LOG_ERR`

```rust
const LOG_ERR: crate::c_int = 3i32;
```

### `LOG_WARNING`

```rust
const LOG_WARNING: crate::c_int = 4i32;
```

### `LOG_NOTICE`

```rust
const LOG_NOTICE: crate::c_int = 5i32;
```

### `LOG_INFO`

```rust
const LOG_INFO: crate::c_int = 6i32;
```

### `LOG_DEBUG`

```rust
const LOG_DEBUG: crate::c_int = 7i32;
```

### `LOG_KERN`

```rust
const LOG_KERN: crate::c_int = 0i32;
```

### `LOG_USER`

```rust
const LOG_USER: crate::c_int = 8i32;
```

### `LOG_MAIL`

```rust
const LOG_MAIL: crate::c_int = 16i32;
```

### `LOG_DAEMON`

```rust
const LOG_DAEMON: crate::c_int = 24i32;
```

### `LOG_AUTH`

```rust
const LOG_AUTH: crate::c_int = 32i32;
```

### `LOG_SYSLOG`

```rust
const LOG_SYSLOG: crate::c_int = 40i32;
```

### `LOG_LPR`

```rust
const LOG_LPR: crate::c_int = 48i32;
```

### `LOG_NEWS`

```rust
const LOG_NEWS: crate::c_int = 56i32;
```

### `LOG_UUCP`

```rust
const LOG_UUCP: crate::c_int = 64i32;
```

### `LOG_LOCAL0`

```rust
const LOG_LOCAL0: crate::c_int = 128i32;
```

### `LOG_LOCAL1`

```rust
const LOG_LOCAL1: crate::c_int = 136i32;
```

### `LOG_LOCAL2`

```rust
const LOG_LOCAL2: crate::c_int = 144i32;
```

### `LOG_LOCAL3`

```rust
const LOG_LOCAL3: crate::c_int = 152i32;
```

### `LOG_LOCAL4`

```rust
const LOG_LOCAL4: crate::c_int = 160i32;
```

### `LOG_LOCAL5`

```rust
const LOG_LOCAL5: crate::c_int = 168i32;
```

### `LOG_LOCAL6`

```rust
const LOG_LOCAL6: crate::c_int = 176i32;
```

### `LOG_LOCAL7`

```rust
const LOG_LOCAL7: crate::c_int = 184i32;
```

### `LOG_PID`

```rust
const LOG_PID: crate::c_int = 1i32;
```

### `LOG_CONS`

```rust
const LOG_CONS: crate::c_int = 2i32;
```

### `LOG_ODELAY`

```rust
const LOG_ODELAY: crate::c_int = 4i32;
```

### `LOG_NDELAY`

```rust
const LOG_NDELAY: crate::c_int = 8i32;
```

### `LOG_NOWAIT`

```rust
const LOG_NOWAIT: crate::c_int = 16i32;
```

### `LOG_PRIMASK`

```rust
const LOG_PRIMASK: crate::c_int = 7i32;
```

### `LOG_FACMASK`

```rust
const LOG_FACMASK: crate::c_int = 1_016i32;
```

### `PRIO_MIN`

```rust
const PRIO_MIN: crate::c_int = -20i32;
```

### `PRIO_MAX`

```rust
const PRIO_MAX: crate::c_int = 20i32;
```

### `IPPROTO_ICMP`

```rust
const IPPROTO_ICMP: crate::c_int = 1i32;
```

### `IPPROTO_ICMPV6`

```rust
const IPPROTO_ICMPV6: crate::c_int = 58i32;
```

### `IPPROTO_TCP`

```rust
const IPPROTO_TCP: crate::c_int = 6i32;
```

### `IPPROTO_UDP`

```rust
const IPPROTO_UDP: crate::c_int = 17i32;
```

### `IPPROTO_IP`

```rust
const IPPROTO_IP: crate::c_int = 0i32;
```

### `IPPROTO_IPV6`

```rust
const IPPROTO_IPV6: crate::c_int = 41i32;
```

### `INADDR_LOOPBACK`

```rust
const INADDR_LOOPBACK: in_addr_t = 2_130_706_433u32;
```

### `INADDR_ANY`

```rust
const INADDR_ANY: in_addr_t = 0u32;
```

### `INADDR_BROADCAST`

```rust
const INADDR_BROADCAST: in_addr_t = 4_294_967_295u32;
```

### `INADDR_NONE`

```rust
const INADDR_NONE: in_addr_t = 4_294_967_295u32;
```

### `IN6ADDR_LOOPBACK_INIT`

```rust
const IN6ADDR_LOOPBACK_INIT: in6_addr;
```

### `IN6ADDR_ANY_INIT`

```rust
const IN6ADDR_ANY_INIT: in6_addr;
```

### `ARPOP_REQUEST`

```rust
const ARPOP_REQUEST: u16 = 1u16;
```

### `ARPOP_REPLY`

```rust
const ARPOP_REPLY: u16 = 2u16;
```

### `ATF_COM`

```rust
const ATF_COM: crate::c_int = 2i32;
```

### `ATF_PERM`

```rust
const ATF_PERM: crate::c_int = 4i32;
```

### `ATF_PUBL`

```rust
const ATF_PUBL: crate::c_int = 8i32;
```

### `ATF_USETRAILERS`

```rust
const ATF_USETRAILERS: crate::c_int = 16i32;
```

### `FNM_PERIOD`

```rust
const FNM_PERIOD: crate::c_int = 4i32;
```

### `FNM_NOMATCH`

```rust
const FNM_NOMATCH: crate::c_int = 1i32;
```

### `FNM_CASEFOLD`

```rust
const FNM_CASEFOLD: crate::c_int = 16i32;
```

### `FNM_PATHNAME`

```rust
const FNM_PATHNAME: crate::c_int = 1i32;
```

### `FNM_NOESCAPE`

```rust
const FNM_NOESCAPE: crate::c_int = 2i32;
```

