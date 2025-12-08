*[libc](../index.md) / [unix](index.md)*

---

# Module `unix`

Definitions found commonly among almost all Unix derivatives

More functions and definitions can be found in the more specific modules
according to the platform in question.

## Modules

- [`linux_like`](linux_like/index.md) - 
- [`linux`](linux/index.md) - Linux-specific definitions for linux-like values

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

- `fn clone(self: &Self) -> group` — [`group`](../index.md)

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

- `fn clone(self: &Self) -> utimbuf` — [`utimbuf`](../index.md)

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

- `fn clone(self: &Self) -> timeval` — [`timeval`](../index.md)

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

- `fn clone(self: &Self) -> rlimit` — [`rlimit`](../index.md)

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

- `fn clone(self: &Self) -> rusage` — [`rusage`](../index.md)

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

- `fn clone(self: &Self) -> ipv6_mreq` — [`ipv6_mreq`](../index.md)

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

- `fn clone(self: &Self) -> hostent` — [`hostent`](../index.md)

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

- `fn clone(self: &Self) -> iovec` — [`iovec`](../index.md)

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

- `fn clone(self: &Self) -> pollfd` — [`pollfd`](../index.md)

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

- `fn clone(self: &Self) -> winsize` — [`winsize`](../index.md)

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

- `fn clone(self: &Self) -> linger` — [`linger`](../index.md)

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

- `fn clone(self: &Self) -> sigval` — [`sigval`](../index.md)

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

- `fn clone(self: &Self) -> itimerval` — [`itimerval`](../index.md)

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

- `fn clone(self: &Self) -> tms` — [`tms`](../index.md)

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

- `fn clone(self: &Self) -> servent` — [`servent`](../index.md)

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

- `fn clone(self: &Self) -> protoent` — [`protoent`](../index.md)

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

- `fn clone(self: &Self) -> in6_addr` — [`in6_addr`](../index.md)

##### `impl Copy for in6_addr`

##### `impl Debug for in6_addr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `in_addr`

```rust
struct in_addr {
    pub s_addr: crate::in_addr_t,
}
```

#### Trait Implementations

##### `impl Clone for in_addr`

- `fn clone(self: &Self) -> in_addr` — [`in_addr`](#in-addr)

##### `impl Copy for in_addr`

##### `impl Debug for in_addr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ip_mreq`

```rust
struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
```

#### Trait Implementations

##### `impl Clone for ip_mreq`

- `fn clone(self: &Self) -> ip_mreq` — [`ip_mreq`](#ip-mreq)

##### `impl Copy for ip_mreq`

##### `impl Debug for ip_mreq`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ip_mreqn`

```rust
struct ip_mreqn {
    pub imr_multiaddr: in_addr,
    pub imr_address: in_addr,
    pub imr_ifindex: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for ip_mreqn`

- `fn clone(self: &Self) -> ip_mreqn` — [`ip_mreqn`](#ip-mreqn)

##### `impl Copy for ip_mreqn`

##### `impl Debug for ip_mreqn`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ip_mreq_source`

```rust
struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}
```

#### Trait Implementations

##### `impl Clone for ip_mreq_source`

- `fn clone(self: &Self) -> ip_mreq_source` — [`ip_mreq_source`](#ip-mreq-source)

##### `impl Copy for ip_mreq_source`

##### `impl Debug for ip_mreq_source`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr`

```rust
struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [crate::c_char; 14],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr`

- `fn clone(self: &Self) -> sockaddr` — [`sockaddr`](#sockaddr)

##### `impl Copy for sockaddr`

##### `impl Debug for sockaddr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr_in`

```rust
struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: crate::in_port_t,
    pub sin_addr: crate::in_addr,
    pub sin_zero: [u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_in`

- `fn clone(self: &Self) -> sockaddr_in` — [`sockaddr_in`](#sockaddr-in)

##### `impl Copy for sockaddr_in`

##### `impl Debug for sockaddr_in`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr_in6`

```rust
struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: crate::in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: crate::in6_addr,
    pub sin6_scope_id: u32,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_in6`

- `fn clone(self: &Self) -> sockaddr_in6` — [`sockaddr_in6`](#sockaddr-in6)

##### `impl Copy for sockaddr_in6`

##### `impl Debug for sockaddr_in6`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `addrinfo`

```rust
struct addrinfo {
    pub ai_flags: crate::c_int,
    pub ai_family: crate::c_int,
    pub ai_socktype: crate::c_int,
    pub ai_protocol: crate::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut crate::sockaddr,
    pub ai_canonname: *mut crate::c_char,
    pub ai_next: *mut addrinfo,
}
```

#### Trait Implementations

##### `impl Clone for addrinfo`

- `fn clone(self: &Self) -> addrinfo` — [`addrinfo`](#addrinfo)

##### `impl Copy for addrinfo`

##### `impl Debug for addrinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr_ll`

```rust
struct sockaddr_ll {
    pub sll_family: crate::c_ushort,
    pub sll_protocol: crate::c_ushort,
    pub sll_ifindex: crate::c_int,
    pub sll_hatype: crate::c_ushort,
    pub sll_pkttype: crate::c_uchar,
    pub sll_halen: crate::c_uchar,
    pub sll_addr: [crate::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_ll`

- `fn clone(self: &Self) -> sockaddr_ll` — [`sockaddr_ll`](#sockaddr-ll)

##### `impl Copy for sockaddr_ll`

##### `impl Debug for sockaddr_ll`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fd_set`

```rust
struct fd_set {
    fds_bits: [crate::c_ulong; 16],
}
```

#### Trait Implementations

##### `impl Clone for fd_set`

- `fn clone(self: &Self) -> fd_set` — [`fd_set`](#fd-set)

##### `impl Copy for fd_set`

##### `impl Debug for fd_set`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tm`

```rust
struct tm {
    pub tm_sec: crate::c_int,
    pub tm_min: crate::c_int,
    pub tm_hour: crate::c_int,
    pub tm_mday: crate::c_int,
    pub tm_mon: crate::c_int,
    pub tm_year: crate::c_int,
    pub tm_wday: crate::c_int,
    pub tm_yday: crate::c_int,
    pub tm_isdst: crate::c_int,
    pub tm_gmtoff: crate::c_long,
    pub tm_zone: *const crate::c_char,
}
```

#### Trait Implementations

##### `impl Clone for tm`

- `fn clone(self: &Self) -> tm` — [`tm`](#tm)

##### `impl Copy for tm`

##### `impl Debug for tm`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sched_param`

```rust
struct sched_param {
    pub sched_priority: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for sched_param`

- `fn clone(self: &Self) -> sched_param` — [`sched_param`](#sched-param)

##### `impl Copy for sched_param`

##### `impl Debug for sched_param`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Dl_info`

```rust
struct Dl_info {
    pub dli_fname: *const crate::c_char,
    pub dli_fbase: *mut crate::c_void,
    pub dli_sname: *const crate::c_char,
    pub dli_saddr: *mut crate::c_void,
}
```

#### Trait Implementations

##### `impl Clone for Dl_info`

- `fn clone(self: &Self) -> Dl_info` — [`Dl_info`](#dl-info)

##### `impl Copy for Dl_info`

##### `impl Debug for Dl_info`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `lconv`

```rust
struct lconv {
    pub decimal_point: *mut crate::c_char,
    pub thousands_sep: *mut crate::c_char,
    pub grouping: *mut crate::c_char,
    pub int_curr_symbol: *mut crate::c_char,
    pub currency_symbol: *mut crate::c_char,
    pub mon_decimal_point: *mut crate::c_char,
    pub mon_thousands_sep: *mut crate::c_char,
    pub mon_grouping: *mut crate::c_char,
    pub positive_sign: *mut crate::c_char,
    pub negative_sign: *mut crate::c_char,
    pub int_frac_digits: crate::c_char,
    pub frac_digits: crate::c_char,
    pub p_cs_precedes: crate::c_char,
    pub p_sep_by_space: crate::c_char,
    pub n_cs_precedes: crate::c_char,
    pub n_sep_by_space: crate::c_char,
    pub p_sign_posn: crate::c_char,
    pub n_sign_posn: crate::c_char,
    pub int_p_cs_precedes: crate::c_char,
    pub int_p_sep_by_space: crate::c_char,
    pub int_n_cs_precedes: crate::c_char,
    pub int_n_sep_by_space: crate::c_char,
    pub int_p_sign_posn: crate::c_char,
    pub int_n_sign_posn: crate::c_char,
}
```

#### Trait Implementations

##### `impl Clone for lconv`

- `fn clone(self: &Self) -> lconv` — [`lconv`](#lconv)

##### `impl Copy for lconv`

##### `impl Debug for lconv`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `in_pktinfo`

```rust
struct in_pktinfo {
    pub ipi_ifindex: crate::c_int,
    pub ipi_spec_dst: crate::in_addr,
    pub ipi_addr: crate::in_addr,
}
```

#### Trait Implementations

##### `impl Clone for in_pktinfo`

- `fn clone(self: &Self) -> in_pktinfo` — [`in_pktinfo`](#in-pktinfo)

##### `impl Copy for in_pktinfo`

##### `impl Debug for in_pktinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ifaddrs`

```rust
struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut crate::c_char,
    pub ifa_flags: crate::c_uint,
    pub ifa_addr: *mut crate::sockaddr,
    pub ifa_netmask: *mut crate::sockaddr,
    pub ifa_ifu: *mut crate::sockaddr,
    pub ifa_data: *mut crate::c_void,
}
```

#### Trait Implementations

##### `impl Clone for ifaddrs`

- `fn clone(self: &Self) -> ifaddrs` — [`ifaddrs`](#ifaddrs)

##### `impl Copy for ifaddrs`

##### `impl Debug for ifaddrs`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `in6_rtmsg`

```rust
struct in6_rtmsg {
    rtmsg_dst: crate::in6_addr,
    rtmsg_src: crate::in6_addr,
    rtmsg_gateway: crate::in6_addr,
    rtmsg_type: u32,
    rtmsg_dst_len: u16,
    rtmsg_src_len: u16,
    rtmsg_metric: u32,
    rtmsg_info: crate::c_ulong,
    rtmsg_flags: u32,
    rtmsg_ifindex: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for in6_rtmsg`

- `fn clone(self: &Self) -> in6_rtmsg` — [`in6_rtmsg`](#in6-rtmsg)

##### `impl Copy for in6_rtmsg`

##### `impl Debug for in6_rtmsg`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `arpreq`

```rust
struct arpreq {
    pub arp_pa: crate::sockaddr,
    pub arp_ha: crate::sockaddr,
    pub arp_flags: crate::c_int,
    pub arp_netmask: crate::sockaddr,
    pub arp_dev: [crate::c_char; 16],
}
```

#### Trait Implementations

##### `impl Clone for arpreq`

- `fn clone(self: &Self) -> arpreq` — [`arpreq`](#arpreq)

##### `impl Copy for arpreq`

##### `impl Debug for arpreq`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `arpreq_old`

```rust
struct arpreq_old {
    pub arp_pa: crate::sockaddr,
    pub arp_ha: crate::sockaddr,
    pub arp_flags: crate::c_int,
    pub arp_netmask: crate::sockaddr,
}
```

#### Trait Implementations

##### `impl Clone for arpreq_old`

- `fn clone(self: &Self) -> arpreq_old` — [`arpreq_old`](#arpreq-old)

##### `impl Copy for arpreq_old`

##### `impl Debug for arpreq_old`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `arphdr`

```rust
struct arphdr {
    pub ar_hrd: u16,
    pub ar_pro: u16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: u16,
}
```

#### Trait Implementations

##### `impl Clone for arphdr`

- `fn clone(self: &Self) -> arphdr` — [`arphdr`](#arphdr)

##### `impl Copy for arphdr`

##### `impl Debug for arphdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `mmsghdr`

```rust
struct mmsghdr {
    pub msg_hdr: crate::msghdr,
    pub msg_len: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for mmsghdr`

- `fn clone(self: &Self) -> mmsghdr` — [`mmsghdr`](#mmsghdr)

##### `impl Copy for mmsghdr`

##### `impl Debug for mmsghdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr_un`

```rust
struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [crate::c_char; 108],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_un`

- `fn clone(self: &Self) -> sockaddr_un` — [`sockaddr_un`](#sockaddr-un)

##### `impl Copy for sockaddr_un`

##### `impl Debug for sockaddr_un`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr_storage`

```rust
struct sockaddr_storage {
    pub ss_family: sa_family_t,
    __ss_pad2: [u8; 118],
    __ss_align: crate::size_t,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_storage`

- `fn clone(self: &Self) -> sockaddr_storage` — [`sockaddr_storage`](#sockaddr-storage)

##### `impl Copy for sockaddr_storage`

##### `impl Debug for sockaddr_storage`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `utsname`

```rust
struct utsname {
    pub sysname: [crate::c_char; 65],
    pub nodename: [crate::c_char; 65],
    pub release: [crate::c_char; 65],
    pub version: [crate::c_char; 65],
    pub machine: [crate::c_char; 65],
    pub domainname: [crate::c_char; 65],
}
```

#### Trait Implementations

##### `impl Clone for utsname`

- `fn clone(self: &Self) -> utsname` — [`utsname`](#utsname)

##### `impl Copy for utsname`

##### `impl Debug for utsname`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `file_clone_range`

```rust
struct file_clone_range {
    pub src_fd: crate::__s64,
    pub src_offset: crate::__u64,
    pub src_length: crate::__u64,
    pub dest_offset: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for file_clone_range`

- `fn clone(self: &Self) -> file_clone_range` — [`file_clone_range`](#file-clone-range)

##### `impl Copy for file_clone_range`

##### `impl Debug for file_clone_range`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sock_filter`

```rust
struct sock_filter {
    pub code: __u16,
    pub jt: __u8,
    pub jf: __u8,
    pub k: __u32,
}
```

#### Trait Implementations

##### `impl Clone for sock_filter`

- `fn clone(self: &Self) -> sock_filter` — [`sock_filter`](#sock-filter)

##### `impl Copy for sock_filter`

##### `impl Debug for sock_filter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sock_fprog`

```rust
struct sock_fprog {
    pub len: crate::c_ushort,
    pub filter: *mut sock_filter,
}
```

#### Trait Implementations

##### `impl Clone for sock_fprog`

- `fn clone(self: &Self) -> sock_fprog` — [`sock_fprog`](#sock-fprog)

##### `impl Copy for sock_fprog`

##### `impl Debug for sock_fprog`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `statx`

```rust
struct statx {
    pub stx_mask: crate::__u32,
    pub stx_blksize: crate::__u32,
    pub stx_attributes: crate::__u64,
    pub stx_nlink: crate::__u32,
    pub stx_uid: crate::__u32,
    pub stx_gid: crate::__u32,
    pub stx_mode: crate::__u16,
    __statx_pad1: crate::types::Padding<[crate::__u16; 1]>,
    pub stx_ino: crate::__u64,
    pub stx_size: crate::__u64,
    pub stx_blocks: crate::__u64,
    pub stx_attributes_mask: crate::__u64,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: crate::__u32,
    pub stx_rdev_minor: crate::__u32,
    pub stx_dev_major: crate::__u32,
    pub stx_dev_minor: crate::__u32,
    pub stx_mnt_id: crate::__u64,
    pub stx_dio_mem_align: crate::__u32,
    pub stx_dio_offset_align: crate::__u32,
    __statx_pad3: crate::types::Padding<[crate::__u64; 12]>,
}
```

#### Trait Implementations

##### `impl Clone for statx`

- `fn clone(self: &Self) -> statx` — [`statx`](#statx)

##### `impl Copy for statx`

##### `impl Debug for statx`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `statx_timestamp`

```rust
struct statx_timestamp {
    pub tv_sec: crate::__s64,
    pub tv_nsec: crate::__u32,
    __statx_timestamp_pad1: crate::types::Padding<[crate::__s32; 1]>,
}
```

#### Trait Implementations

##### `impl Clone for statx_timestamp`

- `fn clone(self: &Self) -> statx_timestamp` — [`statx_timestamp`](#statx-timestamp)

##### `impl Copy for statx_timestamp`

##### `impl Debug for statx_timestamp`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `epoll_event`

```rust
struct epoll_event {
    pub events: u32,
    pub u64: u64,
}
```

#### Trait Implementations

##### `impl Clone for epoll_event`

- `fn clone(self: &Self) -> epoll_event` — [`epoll_event`](#epoll-event)

##### `impl Copy for epoll_event`

##### `impl Debug for epoll_event`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sigevent`

```rust
struct sigevent {
    pub sigev_value: crate::sigval,
    pub sigev_signo: crate::c_int,
    pub sigev_notify: crate::c_int,
    pub sigev_notify_thread_id: crate::c_int,
    __unused1: crate::types::Padding<[crate::c_int; 11]>,
}
```

#### Trait Implementations

##### `impl Clone for sigevent`

- `fn clone(self: &Self) -> sigevent` — [`sigevent`](#sigevent)

##### `impl Copy for sigevent`

##### `impl Debug for sigevent`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `DIR`

```rust
enum DIR {
}
```

#### Trait Implementations

##### `impl Clone for DIR`

- `fn clone(self: &Self) -> DIR` — [`DIR`](../index.md)

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

- `fn clone(self: &Self) -> FILE` — [`FILE`](../index.md)

##### `impl Copy for FILE`

##### `impl Debug for FILE`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `timezone`

```rust
enum timezone {
}
```

#### Trait Implementations

##### `impl Clone for timezone`

- `fn clone(self: &Self) -> timezone` — [`timezone`](#timezone)

##### `impl Copy for timezone`

##### `impl Debug for timezone`

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

### `ioctl`

```rust
unsafe fn ioctl(fd: c_int, request: crate::c_ulong) -> c_int
```

### `sem_destroy`

```rust
unsafe fn sem_destroy(sem: *mut sem_t) -> c_int
```

### `sem_init`

```rust
unsafe fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int
```

### `fdatasync`

```rust
unsafe fn fdatasync(fd: c_int) -> c_int
```

### `mincore`

```rust
unsafe fn mincore(addr: *mut c_void, len: size_t, vec: *mut c_uchar) -> c_int
```

### `clock_getres`

```rust
unsafe fn clock_getres(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int
```

### `clock_gettime`

```rust
unsafe fn clock_gettime(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int
```

### `clock_settime`

```rust
unsafe fn clock_settime(clk_id: crate::clockid_t, tp: *const crate::timespec) -> c_int
```

### `clock_getcpuclockid`

```rust
unsafe fn clock_getcpuclockid(pid: crate::pid_t, clk_id: *mut crate::clockid_t) -> c_int
```

### `dirfd`

```rust
unsafe fn dirfd(dirp: *mut crate::DIR) -> c_int
```

### `memalign`

```rust
unsafe fn memalign(align: size_t, size: size_t) -> *mut c_void
```

### `setgroups`

```rust
unsafe fn setgroups(ngroups: size_t, ptr: *const crate::gid_t) -> c_int
```

### `pipe2`

```rust
unsafe fn pipe2(fds: *mut c_int, flags: c_int) -> c_int
```

### `statfs`

```rust
unsafe fn statfs(path: *const c_char, buf: *mut statfs) -> c_int
```

### `fstatfs`

```rust
unsafe fn fstatfs(fd: c_int, buf: *mut statfs) -> c_int
```

### `memrchr`

```rust
unsafe fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

### `posix_fadvise`

```rust
unsafe fn posix_fadvise(fd: c_int, offset: off_t, len: off_t, advise: c_int) -> c_int
```

### `futimens`

```rust
unsafe fn futimens(fd: c_int, times: *const crate::timespec) -> c_int
```

### `utimensat`

```rust
unsafe fn utimensat(dirfd: c_int, path: *const c_char, times: *const crate::timespec, flag: c_int) -> c_int
```

### `duplocale`

```rust
unsafe fn duplocale(base: crate::locale_t) -> crate::locale_t
```

### `freelocale`

```rust
unsafe fn freelocale(loc: crate::locale_t)
```

### `newlocale`

```rust
unsafe fn newlocale(mask: c_int, locale: *const c_char, base: crate::locale_t) -> crate::locale_t
```

### `uselocale`

```rust
unsafe fn uselocale(loc: crate::locale_t) -> crate::locale_t
```

### `mknodat`

```rust
unsafe fn mknodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, dev: dev_t) -> c_int
```

### `ptsname_r`

```rust
unsafe fn ptsname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```

### `clearenv`

```rust
unsafe fn clearenv() -> c_int
```

### `waitid`

```rust
unsafe fn waitid(idtype: idtype_t, id: id_t, infop: *mut crate::siginfo_t, options: c_int) -> c_int
```

### `getresuid`

```rust
unsafe fn getresuid(ruid: *mut crate::uid_t, euid: *mut crate::uid_t, suid: *mut crate::uid_t) -> c_int
```

### `getresgid`

```rust
unsafe fn getresgid(rgid: *mut crate::gid_t, egid: *mut crate::gid_t, sgid: *mut crate::gid_t) -> c_int
```

### `acct`

```rust
unsafe fn acct(filename: *const c_char) -> c_int
```

### `brk`

```rust
unsafe fn brk(addr: *mut c_void) -> c_int
```

### `sbrk`

```rust
unsafe fn sbrk(increment: intptr_t) -> *mut c_void
```

### `vfork`

```rust
unsafe fn vfork() -> crate::pid_t
```

### `setresgid`

```rust
unsafe fn setresgid(rgid: crate::gid_t, egid: crate::gid_t, sgid: crate::gid_t) -> c_int
```

### `setresuid`

```rust
unsafe fn setresuid(ruid: crate::uid_t, euid: crate::uid_t, suid: crate::uid_t) -> c_int
```

### `wait4`

```rust
unsafe fn wait4(pid: crate::pid_t, status: *mut c_int, options: c_int, rusage: *mut crate::rusage) -> crate::pid_t
```

### `login_tty`

```rust
unsafe fn login_tty(fd: c_int) -> c_int
```

### `execvpe`

```rust
unsafe fn execvpe(file: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

### `fexecve`

```rust
unsafe fn fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

### `getifaddrs`

```rust
unsafe fn getifaddrs(ifap: *mut *mut crate::ifaddrs) -> c_int
```

### `freeifaddrs`

```rust
unsafe fn freeifaddrs(ifa: *mut crate::ifaddrs)
```

### `bind`

```rust
unsafe fn bind(socket: c_int, address: *const crate::sockaddr, address_len: crate::socklen_t) -> c_int
```

### `writev`

```rust
unsafe fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t
```

### `readv`

```rust
unsafe fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t
```

### `sendmsg`

```rust
unsafe fn sendmsg(fd: c_int, msg: *const crate::msghdr, flags: c_int) -> ssize_t
```

### `recvmsg`

```rust
unsafe fn recvmsg(fd: c_int, msg: *mut crate::msghdr, flags: c_int) -> ssize_t
```

### `uname`

```rust
unsafe fn uname(buf: *mut crate::utsname) -> c_int
```

### `strchrnul`

```rust
unsafe fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char
```

### `strftime`

```rust
unsafe fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const crate::tm) -> size_t
```

### `strftime_l`

```rust
unsafe fn strftime_l(s: *mut c_char, max: size_t, format: *const c_char, tm: *const crate::tm, locale: crate::locale_t) -> size_t
```

### `strptime`

```rust
unsafe fn strptime(s: *const c_char, format: *const c_char, tm: *mut crate::tm) -> *mut c_char
```

### `mkostemp`

```rust
unsafe fn mkostemp(template: *mut c_char, flags: c_int) -> c_int
```

### `mkostemps`

```rust
unsafe fn mkostemps(template: *mut c_char, suffixlen: c_int, flags: c_int) -> c_int
```

### `getdomainname`

```rust
unsafe fn getdomainname(name: *mut c_char, len: size_t) -> c_int
```

### `setdomainname`

```rust
unsafe fn setdomainname(name: *const c_char, len: size_t) -> c_int
```

### `fstatfs64`

```rust
unsafe fn fstatfs64(fd: c_int, buf: *mut statfs64) -> c_int
```

### `statvfs64`

```rust
unsafe fn statvfs64(path: *const c_char, buf: *mut statvfs64) -> c_int
```

### `fstatvfs64`

```rust
unsafe fn fstatvfs64(fd: c_int, buf: *mut statvfs64) -> c_int
```

### `statfs64`

```rust
unsafe fn statfs64(path: *const c_char, buf: *mut statfs64) -> c_int
```

### `creat64`

```rust
unsafe fn creat64(path: *const c_char, mode: mode_t) -> c_int
```

### `fstat64`

```rust
unsafe fn fstat64(fildes: c_int, buf: *mut stat64) -> c_int
```

### `fstatat64`

```rust
unsafe fn fstatat64(dirfd: c_int, pathname: *const c_char, buf: *mut stat64, flags: c_int) -> c_int
```

### `ftruncate64`

```rust
unsafe fn ftruncate64(fd: c_int, length: off64_t) -> c_int
```

### `lseek64`

```rust
unsafe fn lseek64(fd: c_int, offset: off64_t, whence: c_int) -> off64_t
```

### `lstat64`

```rust
unsafe fn lstat64(path: *const c_char, buf: *mut stat64) -> c_int
```

### `mmap64`

```rust
unsafe fn mmap64(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off64_t) -> *mut c_void
```

### `open64`

```rust
unsafe fn open64(path: *const c_char, oflag: c_int) -> c_int
```

### `openat64`

```rust
unsafe fn openat64(fd: c_int, path: *const c_char, oflag: c_int) -> c_int
```

### `posix_fadvise64`

```rust
unsafe fn posix_fadvise64(fd: c_int, offset: off64_t, len: off64_t, advise: c_int) -> c_int
```

### `pread64`

```rust
unsafe fn pread64(fd: c_int, buf: *mut c_void, count: size_t, offset: off64_t) -> ssize_t
```

### `pwrite64`

```rust
unsafe fn pwrite64(fd: c_int, buf: *const c_void, count: size_t, offset: off64_t) -> ssize_t
```

### `readdir64`

```rust
unsafe fn readdir64(dirp: *mut crate::DIR) -> *mut crate::dirent64
```

### `readdir64_r`

```rust
unsafe fn readdir64_r(dirp: *mut crate::DIR, entry: *mut crate::dirent64, result: *mut *mut crate::dirent64) -> c_int
```

### `stat64`

```rust
unsafe fn stat64(path: *const c_char, buf: *mut stat64) -> c_int
```

### `truncate64`

```rust
unsafe fn truncate64(path: *const c_char, length: off64_t) -> c_int
```

### `preadv64`

```rust
unsafe fn preadv64(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t) -> ssize_t
```

### `pwritev64`

```rust
unsafe fn pwritev64(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t) -> ssize_t
```

### `forkpty`

```rust
unsafe fn forkpty(amaster: *mut c_int, name: *mut c_char, termp: *const termios, winp: *const crate::winsize) -> crate::pid_t
```

### `openpty`

```rust
unsafe fn openpty(amaster: *mut c_int, aslave: *mut c_int, name: *mut c_char, termp: *const termios, winp: *const crate::winsize) -> c_int
```

### `statx`

```rust
unsafe fn statx(dirfd: c_int, pathname: *const c_char, flags: c_int, mask: c_uint, statxbuf: *mut statx) -> c_int
```

### `_IOC`

```rust
const fn _IOC(dir: u32, ty: u32, nr: u32, size: usize) -> crate::c_ulong
```

Build an ioctl number, analogous to the C macro of the same name.

### `_IO`

```rust
const fn _IO(ty: u32, nr: u32) -> crate::c_ulong
```

Build an ioctl number for an argumentless ioctl.

### `_IOR`

```rust
const fn _IOR<T>(ty: u32, nr: u32) -> crate::c_ulong
```

Build an ioctl number for an read-only ioctl.

### `_IOW`

```rust
const fn _IOW<T>(ty: u32, nr: u32) -> crate::c_ulong
```

Build an ioctl number for an write-only ioctl.

### `_IOWR`

```rust
const fn _IOWR<T>(ty: u32, nr: u32) -> crate::c_ulong
```

Build an ioctl number for a read-write ioctl.

### `CMSG_ALIGN`

```rust
const fn CMSG_ALIGN(len: usize) -> usize
```

### `CMSG_FIRSTHDR`

```rust
unsafe fn CMSG_FIRSTHDR(mhdr: *const crate::msghdr) -> *mut crate::cmsghdr
```

### `CMSG_DATA`

```rust
unsafe fn CMSG_DATA(cmsg: *const crate::cmsghdr) -> *mut crate::c_uchar
```

### `CMSG_SPACE`

```rust
const unsafe fn CMSG_SPACE(length: crate::c_uint) -> crate::c_uint
```

### `CMSG_LEN`

```rust
const unsafe fn CMSG_LEN(length: crate::c_uint) -> crate::c_uint
```

### `FD_CLR`

```rust
unsafe fn FD_CLR(fd: crate::c_int, set: *mut fd_set)
```

### `FD_ISSET`

```rust
unsafe fn FD_ISSET(fd: crate::c_int, set: *const fd_set) -> bool
```

### `FD_SET`

```rust
unsafe fn FD_SET(fd: crate::c_int, set: *mut fd_set)
```

### `FD_ZERO`

```rust
unsafe fn FD_ZERO(set: *mut fd_set)
```

### `SIGRTMAX`

```rust
fn SIGRTMAX() -> crate::c_int
```

### `SIGRTMIN`

```rust
fn SIGRTMIN() -> crate::c_int
```

### `WIFSTOPPED`

```rust
const fn WIFSTOPPED(status: crate::c_int) -> bool
```

### `WSTOPSIG`

```rust
const fn WSTOPSIG(status: crate::c_int) -> crate::c_int
```

### `WIFCONTINUED`

```rust
const fn WIFCONTINUED(status: crate::c_int) -> bool
```

### `WIFSIGNALED`

```rust
const fn WIFSIGNALED(status: crate::c_int) -> bool
```

### `WTERMSIG`

```rust
const fn WTERMSIG(status: crate::c_int) -> crate::c_int
```

### `WIFEXITED`

```rust
const fn WIFEXITED(status: crate::c_int) -> bool
```

### `WEXITSTATUS`

```rust
const fn WEXITSTATUS(status: crate::c_int) -> crate::c_int
```

### `WCOREDUMP`

```rust
const fn WCOREDUMP(status: crate::c_int) -> bool
```

### `W_EXITCODE`

```rust
const fn W_EXITCODE(ret: crate::c_int, sig: crate::c_int) -> crate::c_int
```

### `W_STOPCODE`

```rust
const fn W_STOPCODE(sig: crate::c_int) -> crate::c_int
```

### `QCMD`

```rust
const fn QCMD(cmd: crate::c_int, type_: crate::c_int) -> crate::c_int
```

### `IPOPT_COPIED`

```rust
const fn IPOPT_COPIED(o: u8) -> u8
```

### `IPOPT_CLASS`

```rust
const fn IPOPT_CLASS(o: u8) -> u8
```

### `IPOPT_NUMBER`

```rust
const fn IPOPT_NUMBER(o: u8) -> u8
```

### `IPTOS_ECN`

```rust
const fn IPTOS_ECN(x: u8) -> u8
```

### `KERNEL_VERSION`

```rust
const fn KERNEL_VERSION(a: u32, b: u32, c: u32) -> u32
```

## Type Aliases

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

### `sa_family_t`

```rust
type sa_family_t = u16;
```

### `speed_t`

```rust
type speed_t = crate::c_uint;
```

### `tcflag_t`

```rust
type tcflag_t = crate::c_uint;
```

### `clockid_t`

```rust
type clockid_t = crate::c_int;
```

### `timer_t`

```rust
type timer_t = *mut crate::c_void;
```

### `key_t`

```rust
type key_t = crate::c_int;
```

### `id_t`

```rust
type id_t = crate::c_uint;
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

### `ULONG_SIZE`

```rust
const ULONG_SIZE: usize = 64usize;
```

### `EXIT_FAILURE`

```rust
const EXIT_FAILURE: crate::c_int = 1i32;
```

### `EXIT_SUCCESS`

```rust
const EXIT_SUCCESS: crate::c_int = 0i32;
```

### `RAND_MAX`

```rust
const RAND_MAX: crate::c_int = 2_147_483_647i32;
```

### `EOF`

```rust
const EOF: crate::c_int = -1i32;
```

### `SEEK_SET`

```rust
const SEEK_SET: crate::c_int = 0i32;
```

### `SEEK_CUR`

```rust
const SEEK_CUR: crate::c_int = 1i32;
```

### `SEEK_END`

```rust
const SEEK_END: crate::c_int = 2i32;
```

### `_IOFBF`

```rust
const _IOFBF: crate::c_int = 0i32;
```

### `_IONBF`

```rust
const _IONBF: crate::c_int = 2i32;
```

### `_IOLBF`

```rust
const _IOLBF: crate::c_int = 1i32;
```

### `F_DUPFD`

```rust
const F_DUPFD: crate::c_int = 0i32;
```

### `F_GETFD`

```rust
const F_GETFD: crate::c_int = 1i32;
```

### `F_SETFD`

```rust
const F_SETFD: crate::c_int = 2i32;
```

### `F_GETFL`

```rust
const F_GETFL: crate::c_int = 3i32;
```

### `F_SETFL`

```rust
const F_SETFL: crate::c_int = 4i32;
```

### `F_SETLEASE`

```rust
const F_SETLEASE: crate::c_int = 1_024i32;
```

### `F_GETLEASE`

```rust
const F_GETLEASE: crate::c_int = 1_025i32;
```

### `F_NOTIFY`

```rust
const F_NOTIFY: crate::c_int = 1_026i32;
```

### `F_CANCELLK`

```rust
const F_CANCELLK: crate::c_int = 1_029i32;
```

### `F_DUPFD_CLOEXEC`

```rust
const F_DUPFD_CLOEXEC: crate::c_int = 1_030i32;
```

### `F_SETPIPE_SZ`

```rust
const F_SETPIPE_SZ: crate::c_int = 1_031i32;
```

### `F_GETPIPE_SZ`

```rust
const F_GETPIPE_SZ: crate::c_int = 1_032i32;
```

### `F_ADD_SEALS`

```rust
const F_ADD_SEALS: crate::c_int = 1_033i32;
```

### `F_GET_SEALS`

```rust
const F_GET_SEALS: crate::c_int = 1_034i32;
```

### `F_SEAL_SEAL`

```rust
const F_SEAL_SEAL: crate::c_int = 1i32;
```

### `F_SEAL_SHRINK`

```rust
const F_SEAL_SHRINK: crate::c_int = 2i32;
```

### `F_SEAL_GROW`

```rust
const F_SEAL_GROW: crate::c_int = 4i32;
```

### `F_SEAL_WRITE`

```rust
const F_SEAL_WRITE: crate::c_int = 8i32;
```

### `SIGTRAP`

```rust
const SIGTRAP: crate::c_int = 5i32;
```

### `PTHREAD_CREATE_JOINABLE`

```rust
const PTHREAD_CREATE_JOINABLE: crate::c_int = 0i32;
```

### `PTHREAD_CREATE_DETACHED`

```rust
const PTHREAD_CREATE_DETACHED: crate::c_int = 1i32;
```

### `CLOCK_REALTIME`

```rust
const CLOCK_REALTIME: crate::clockid_t = 0i32;
```

### `CLOCK_MONOTONIC`

```rust
const CLOCK_MONOTONIC: crate::clockid_t = 1i32;
```

### `CLOCK_PROCESS_CPUTIME_ID`

```rust
const CLOCK_PROCESS_CPUTIME_ID: crate::clockid_t = 2i32;
```

### `CLOCK_THREAD_CPUTIME_ID`

```rust
const CLOCK_THREAD_CPUTIME_ID: crate::clockid_t = 3i32;
```

### `CLOCK_MONOTONIC_RAW`

```rust
const CLOCK_MONOTONIC_RAW: crate::clockid_t = 4i32;
```

### `CLOCK_REALTIME_COARSE`

```rust
const CLOCK_REALTIME_COARSE: crate::clockid_t = 5i32;
```

### `CLOCK_MONOTONIC_COARSE`

```rust
const CLOCK_MONOTONIC_COARSE: crate::clockid_t = 6i32;
```

### `CLOCK_BOOTTIME`

```rust
const CLOCK_BOOTTIME: crate::clockid_t = 7i32;
```

### `CLOCK_REALTIME_ALARM`

```rust
const CLOCK_REALTIME_ALARM: crate::clockid_t = 8i32;
```

### `CLOCK_BOOTTIME_ALARM`

```rust
const CLOCK_BOOTTIME_ALARM: crate::clockid_t = 9i32;
```

### `CLOCK_TAI`

```rust
const CLOCK_TAI: crate::clockid_t = 11i32;
```

### `TIMER_ABSTIME`

```rust
const TIMER_ABSTIME: crate::c_int = 1i32;
```

### `RUSAGE_SELF`

```rust
const RUSAGE_SELF: crate::c_int = 0i32;
```

### `O_RDONLY`

```rust
const O_RDONLY: crate::c_int = 0i32;
```

### `O_WRONLY`

```rust
const O_WRONLY: crate::c_int = 1i32;
```

### `O_RDWR`

```rust
const O_RDWR: crate::c_int = 2i32;
```

### `SOCK_CLOEXEC`

```rust
const SOCK_CLOEXEC: crate::c_int = 524_288i32;
```

### `S_IFIFO`

```rust
const S_IFIFO: mode_t = 4_096u32;
```

### `S_IFCHR`

```rust
const S_IFCHR: mode_t = 8_192u32;
```

### `S_IFBLK`

```rust
const S_IFBLK: mode_t = 24_576u32;
```

### `S_IFDIR`

```rust
const S_IFDIR: mode_t = 16_384u32;
```

### `S_IFREG`

```rust
const S_IFREG: mode_t = 32_768u32;
```

### `S_IFLNK`

```rust
const S_IFLNK: mode_t = 40_960u32;
```

### `S_IFSOCK`

```rust
const S_IFSOCK: mode_t = 49_152u32;
```

### `S_IFMT`

```rust
const S_IFMT: mode_t = 61_440u32;
```

### `S_IRWXU`

```rust
const S_IRWXU: mode_t = 448u32;
```

### `S_IXUSR`

```rust
const S_IXUSR: mode_t = 64u32;
```

### `S_IWUSR`

```rust
const S_IWUSR: mode_t = 128u32;
```

### `S_IRUSR`

```rust
const S_IRUSR: mode_t = 256u32;
```

### `S_IRWXG`

```rust
const S_IRWXG: mode_t = 56u32;
```

### `S_IXGRP`

```rust
const S_IXGRP: mode_t = 8u32;
```

### `S_IWGRP`

```rust
const S_IWGRP: mode_t = 16u32;
```

### `S_IRGRP`

```rust
const S_IRGRP: mode_t = 32u32;
```

### `S_IRWXO`

```rust
const S_IRWXO: mode_t = 7u32;
```

### `S_IXOTH`

```rust
const S_IXOTH: mode_t = 1u32;
```

### `S_IWOTH`

```rust
const S_IWOTH: mode_t = 2u32;
```

### `S_IROTH`

```rust
const S_IROTH: mode_t = 4u32;
```

### `F_OK`

```rust
const F_OK: crate::c_int = 0i32;
```

### `R_OK`

```rust
const R_OK: crate::c_int = 4i32;
```

### `W_OK`

```rust
const W_OK: crate::c_int = 2i32;
```

### `X_OK`

```rust
const X_OK: crate::c_int = 1i32;
```

### `SIGHUP`

```rust
const SIGHUP: crate::c_int = 1i32;
```

### `SIGINT`

```rust
const SIGINT: crate::c_int = 2i32;
```

### `SIGQUIT`

```rust
const SIGQUIT: crate::c_int = 3i32;
```

### `SIGILL`

```rust
const SIGILL: crate::c_int = 4i32;
```

### `SIGABRT`

```rust
const SIGABRT: crate::c_int = 6i32;
```

### `SIGFPE`

```rust
const SIGFPE: crate::c_int = 8i32;
```

### `SIGKILL`

```rust
const SIGKILL: crate::c_int = 9i32;
```

### `SIGSEGV`

```rust
const SIGSEGV: crate::c_int = 11i32;
```

### `SIGPIPE`

```rust
const SIGPIPE: crate::c_int = 13i32;
```

### `SIGALRM`

```rust
const SIGALRM: crate::c_int = 14i32;
```

### `SIGTERM`

```rust
const SIGTERM: crate::c_int = 15i32;
```

### `PROT_NONE`

```rust
const PROT_NONE: crate::c_int = 0i32;
```

### `PROT_READ`

```rust
const PROT_READ: crate::c_int = 1i32;
```

### `PROT_WRITE`

```rust
const PROT_WRITE: crate::c_int = 2i32;
```

### `PROT_EXEC`

```rust
const PROT_EXEC: crate::c_int = 4i32;
```

### `XATTR_CREATE`

```rust
const XATTR_CREATE: crate::c_int = 1i32;
```

### `XATTR_REPLACE`

```rust
const XATTR_REPLACE: crate::c_int = 2i32;
```

### `RLIM64_INFINITY`

```rust
const RLIM64_INFINITY: crate::rlim64_t = 18_446_744_073_709_551_615u64;
```

### `LC_CTYPE`

```rust
const LC_CTYPE: crate::c_int = 0i32;
```

### `LC_NUMERIC`

```rust
const LC_NUMERIC: crate::c_int = 1i32;
```

### `LC_TIME`

```rust
const LC_TIME: crate::c_int = 2i32;
```

### `LC_COLLATE`

```rust
const LC_COLLATE: crate::c_int = 3i32;
```

### `LC_MONETARY`

```rust
const LC_MONETARY: crate::c_int = 4i32;
```

### `LC_MESSAGES`

```rust
const LC_MESSAGES: crate::c_int = 5i32;
```

### `LC_ALL`

```rust
const LC_ALL: crate::c_int = 6i32;
```

### `LC_CTYPE_MASK`

```rust
const LC_CTYPE_MASK: crate::c_int = 1i32;
```

### `LC_NUMERIC_MASK`

```rust
const LC_NUMERIC_MASK: crate::c_int = 2i32;
```

### `LC_TIME_MASK`

```rust
const LC_TIME_MASK: crate::c_int = 4i32;
```

### `LC_COLLATE_MASK`

```rust
const LC_COLLATE_MASK: crate::c_int = 8i32;
```

### `LC_MONETARY_MASK`

```rust
const LC_MONETARY_MASK: crate::c_int = 16i32;
```

### `LC_MESSAGES_MASK`

```rust
const LC_MESSAGES_MASK: crate::c_int = 32i32;
```

### `MAP_FILE`

```rust
const MAP_FILE: crate::c_int = 0i32;
```

### `MAP_SHARED`

```rust
const MAP_SHARED: crate::c_int = 1i32;
```

### `MAP_PRIVATE`

```rust
const MAP_PRIVATE: crate::c_int = 2i32;
```

### `MAP_FIXED`

```rust
const MAP_FIXED: crate::c_int = 16i32;
```

### `MAP_FAILED`

```rust
const MAP_FAILED: *mut crate::c_void = {0xffffffffffffffff as *mut core::ffi::c_void};
```

### `MS_ASYNC`

```rust
const MS_ASYNC: crate::c_int = 1i32;
```

### `MS_INVALIDATE`

```rust
const MS_INVALIDATE: crate::c_int = 2i32;
```

### `MS_SYNC`

```rust
const MS_SYNC: crate::c_int = 4i32;
```

### `MS_RDONLY`

```rust
const MS_RDONLY: crate::c_ulong = 1u64;
```

### `MS_NOSUID`

```rust
const MS_NOSUID: crate::c_ulong = 2u64;
```

### `MS_NODEV`

```rust
const MS_NODEV: crate::c_ulong = 4u64;
```

### `MS_NOEXEC`

```rust
const MS_NOEXEC: crate::c_ulong = 8u64;
```

### `MS_SYNCHRONOUS`

```rust
const MS_SYNCHRONOUS: crate::c_ulong = 16u64;
```

### `MS_REMOUNT`

```rust
const MS_REMOUNT: crate::c_ulong = 32u64;
```

### `MS_MANDLOCK`

```rust
const MS_MANDLOCK: crate::c_ulong = 64u64;
```

### `MS_DIRSYNC`

```rust
const MS_DIRSYNC: crate::c_ulong = 128u64;
```

### `MS_NOSYMFOLLOW`

```rust
const MS_NOSYMFOLLOW: crate::c_ulong = 256u64;
```

### `MS_NOATIME`

```rust
const MS_NOATIME: crate::c_ulong = 1_024u64;
```

### `MS_NODIRATIME`

```rust
const MS_NODIRATIME: crate::c_ulong = 2_048u64;
```

### `MS_BIND`

```rust
const MS_BIND: crate::c_ulong = 4_096u64;
```

### `MS_MOVE`

```rust
const MS_MOVE: crate::c_ulong = 8_192u64;
```

### `MS_REC`

```rust
const MS_REC: crate::c_ulong = 16_384u64;
```

### `MS_SILENT`

```rust
const MS_SILENT: crate::c_ulong = 32_768u64;
```

### `MS_POSIXACL`

```rust
const MS_POSIXACL: crate::c_ulong = 65_536u64;
```

### `MS_UNBINDABLE`

```rust
const MS_UNBINDABLE: crate::c_ulong = 131_072u64;
```

### `MS_PRIVATE`

```rust
const MS_PRIVATE: crate::c_ulong = 262_144u64;
```

### `MS_SLAVE`

```rust
const MS_SLAVE: crate::c_ulong = 524_288u64;
```

### `MS_SHARED`

```rust
const MS_SHARED: crate::c_ulong = 1_048_576u64;
```

### `MS_RELATIME`

```rust
const MS_RELATIME: crate::c_ulong = 2_097_152u64;
```

### `MS_KERNMOUNT`

```rust
const MS_KERNMOUNT: crate::c_ulong = 4_194_304u64;
```

### `MS_I_VERSION`

```rust
const MS_I_VERSION: crate::c_ulong = 8_388_608u64;
```

### `MS_STRICTATIME`

```rust
const MS_STRICTATIME: crate::c_ulong = 16_777_216u64;
```

### `MS_LAZYTIME`

```rust
const MS_LAZYTIME: crate::c_ulong = 33_554_432u64;
```

### `MS_ACTIVE`

```rust
const MS_ACTIVE: crate::c_ulong = 1_073_741_824u64;
```

### `MS_MGC_VAL`

```rust
const MS_MGC_VAL: crate::c_ulong = 3_236_757_504u64;
```

### `MS_MGC_MSK`

```rust
const MS_MGC_MSK: crate::c_ulong = 4_294_901_760u64;
```

### `SCM_RIGHTS`

```rust
const SCM_RIGHTS: crate::c_int = 1i32;
```

### `SCM_CREDENTIALS`

```rust
const SCM_CREDENTIALS: crate::c_int = 2i32;
```

### `PROT_GROWSDOWN`

```rust
const PROT_GROWSDOWN: crate::c_int = 16_777_216i32;
```

### `PROT_GROWSUP`

```rust
const PROT_GROWSUP: crate::c_int = 33_554_432i32;
```

### `MAP_TYPE`

```rust
const MAP_TYPE: crate::c_int = 15i32;
```

### `MADV_NORMAL`

```rust
const MADV_NORMAL: crate::c_int = 0i32;
```

### `MADV_RANDOM`

```rust
const MADV_RANDOM: crate::c_int = 1i32;
```

### `MADV_SEQUENTIAL`

```rust
const MADV_SEQUENTIAL: crate::c_int = 2i32;
```

### `MADV_WILLNEED`

```rust
const MADV_WILLNEED: crate::c_int = 3i32;
```

### `MADV_DONTNEED`

```rust
const MADV_DONTNEED: crate::c_int = 4i32;
```

### `MADV_FREE`

```rust
const MADV_FREE: crate::c_int = 8i32;
```

### `MADV_REMOVE`

```rust
const MADV_REMOVE: crate::c_int = 9i32;
```

### `MADV_DONTFORK`

```rust
const MADV_DONTFORK: crate::c_int = 10i32;
```

### `MADV_DOFORK`

```rust
const MADV_DOFORK: crate::c_int = 11i32;
```

### `MADV_MERGEABLE`

```rust
const MADV_MERGEABLE: crate::c_int = 12i32;
```

### `MADV_UNMERGEABLE`

```rust
const MADV_UNMERGEABLE: crate::c_int = 13i32;
```

### `MADV_HUGEPAGE`

```rust
const MADV_HUGEPAGE: crate::c_int = 14i32;
```

### `MADV_NOHUGEPAGE`

```rust
const MADV_NOHUGEPAGE: crate::c_int = 15i32;
```

### `MADV_DONTDUMP`

```rust
const MADV_DONTDUMP: crate::c_int = 16i32;
```

### `MADV_DODUMP`

```rust
const MADV_DODUMP: crate::c_int = 17i32;
```

### `MADV_WIPEONFORK`

```rust
const MADV_WIPEONFORK: crate::c_int = 18i32;
```

### `MADV_KEEPONFORK`

```rust
const MADV_KEEPONFORK: crate::c_int = 19i32;
```

### `MADV_COLD`

```rust
const MADV_COLD: crate::c_int = 20i32;
```

### `MADV_PAGEOUT`

```rust
const MADV_PAGEOUT: crate::c_int = 21i32;
```

### `MADV_HWPOISON`

```rust
const MADV_HWPOISON: crate::c_int = 100i32;
```

### `MADV_POPULATE_READ`

```rust
const MADV_POPULATE_READ: crate::c_int = 22i32;
```

### `MADV_POPULATE_WRITE`

```rust
const MADV_POPULATE_WRITE: crate::c_int = 23i32;
```

### `MADV_DONTNEED_LOCKED`

```rust
const MADV_DONTNEED_LOCKED: crate::c_int = 24i32;
```

### `IFF_UP`

```rust
const IFF_UP: crate::c_int = 1i32;
```

### `IFF_BROADCAST`

```rust
const IFF_BROADCAST: crate::c_int = 2i32;
```

### `IFF_DEBUG`

```rust
const IFF_DEBUG: crate::c_int = 4i32;
```

### `IFF_LOOPBACK`

```rust
const IFF_LOOPBACK: crate::c_int = 8i32;
```

### `IFF_POINTOPOINT`

```rust
const IFF_POINTOPOINT: crate::c_int = 16i32;
```

### `IFF_NOTRAILERS`

```rust
const IFF_NOTRAILERS: crate::c_int = 32i32;
```

### `IFF_RUNNING`

```rust
const IFF_RUNNING: crate::c_int = 64i32;
```

### `IFF_NOARP`

```rust
const IFF_NOARP: crate::c_int = 128i32;
```

### `IFF_PROMISC`

```rust
const IFF_PROMISC: crate::c_int = 256i32;
```

### `IFF_ALLMULTI`

```rust
const IFF_ALLMULTI: crate::c_int = 512i32;
```

### `IFF_MASTER`

```rust
const IFF_MASTER: crate::c_int = 1_024i32;
```

### `IFF_SLAVE`

```rust
const IFF_SLAVE: crate::c_int = 2_048i32;
```

### `IFF_MULTICAST`

```rust
const IFF_MULTICAST: crate::c_int = 4_096i32;
```

### `IFF_PORTSEL`

```rust
const IFF_PORTSEL: crate::c_int = 8_192i32;
```

### `IFF_AUTOMEDIA`

```rust
const IFF_AUTOMEDIA: crate::c_int = 16_384i32;
```

### `IFF_DYNAMIC`

```rust
const IFF_DYNAMIC: crate::c_int = 32_768i32;
```

### `SOL_IP`

```rust
const SOL_IP: crate::c_int = 0i32;
```

### `SOL_TCP`

```rust
const SOL_TCP: crate::c_int = 6i32;
```

### `SOL_UDP`

```rust
const SOL_UDP: crate::c_int = 17i32;
```

### `SOL_IPV6`

```rust
const SOL_IPV6: crate::c_int = 41i32;
```

### `SOL_ICMPV6`

```rust
const SOL_ICMPV6: crate::c_int = 58i32;
```

### `SOL_RAW`

```rust
const SOL_RAW: crate::c_int = 255i32;
```

### `SOL_DECNET`

```rust
const SOL_DECNET: crate::c_int = 261i32;
```

### `SOL_X25`

```rust
const SOL_X25: crate::c_int = 262i32;
```

### `SOL_PACKET`

```rust
const SOL_PACKET: crate::c_int = 263i32;
```

### `SOL_ATM`

```rust
const SOL_ATM: crate::c_int = 264i32;
```

### `SOL_AAL`

```rust
const SOL_AAL: crate::c_int = 265i32;
```

### `SOL_IRDA`

```rust
const SOL_IRDA: crate::c_int = 266i32;
```

### `SOL_NETBEUI`

```rust
const SOL_NETBEUI: crate::c_int = 267i32;
```

### `SOL_LLC`

```rust
const SOL_LLC: crate::c_int = 268i32;
```

### `SOL_DCCP`

```rust
const SOL_DCCP: crate::c_int = 269i32;
```

### `SOL_NETLINK`

```rust
const SOL_NETLINK: crate::c_int = 270i32;
```

### `SOL_TIPC`

```rust
const SOL_TIPC: crate::c_int = 271i32;
```

### `SOL_BLUETOOTH`

```rust
const SOL_BLUETOOTH: crate::c_int = 274i32;
```

### `SOL_ALG`

```rust
const SOL_ALG: crate::c_int = 279i32;
```

### `AF_UNSPEC`

```rust
const AF_UNSPEC: crate::c_int = 0i32;
```

### `AF_UNIX`

```rust
const AF_UNIX: crate::c_int = 1i32;
```

### `AF_LOCAL`

```rust
const AF_LOCAL: crate::c_int = 1i32;
```

### `AF_INET`

```rust
const AF_INET: crate::c_int = 2i32;
```

### `AF_AX25`

```rust
const AF_AX25: crate::c_int = 3i32;
```

### `AF_IPX`

```rust
const AF_IPX: crate::c_int = 4i32;
```

### `AF_APPLETALK`

```rust
const AF_APPLETALK: crate::c_int = 5i32;
```

### `AF_NETROM`

```rust
const AF_NETROM: crate::c_int = 6i32;
```

### `AF_BRIDGE`

```rust
const AF_BRIDGE: crate::c_int = 7i32;
```

### `AF_ATMPVC`

```rust
const AF_ATMPVC: crate::c_int = 8i32;
```

### `AF_X25`

```rust
const AF_X25: crate::c_int = 9i32;
```

### `AF_INET6`

```rust
const AF_INET6: crate::c_int = 10i32;
```

### `AF_ROSE`

```rust
const AF_ROSE: crate::c_int = 11i32;
```

### `AF_DECnet`

```rust
const AF_DECnet: crate::c_int = 12i32;
```

### `AF_NETBEUI`

```rust
const AF_NETBEUI: crate::c_int = 13i32;
```

### `AF_SECURITY`

```rust
const AF_SECURITY: crate::c_int = 14i32;
```

### `AF_KEY`

```rust
const AF_KEY: crate::c_int = 15i32;
```

### `AF_NETLINK`

```rust
const AF_NETLINK: crate::c_int = 16i32;
```

### `AF_ROUTE`

```rust
const AF_ROUTE: crate::c_int = 16i32;
```

### `AF_PACKET`

```rust
const AF_PACKET: crate::c_int = 17i32;
```

### `AF_ASH`

```rust
const AF_ASH: crate::c_int = 18i32;
```

### `AF_ECONET`

```rust
const AF_ECONET: crate::c_int = 19i32;
```

### `AF_ATMSVC`

```rust
const AF_ATMSVC: crate::c_int = 20i32;
```

### `AF_RDS`

```rust
const AF_RDS: crate::c_int = 21i32;
```

### `AF_SNA`

```rust
const AF_SNA: crate::c_int = 22i32;
```

### `AF_IRDA`

```rust
const AF_IRDA: crate::c_int = 23i32;
```

### `AF_PPPOX`

```rust
const AF_PPPOX: crate::c_int = 24i32;
```

### `AF_WANPIPE`

```rust
const AF_WANPIPE: crate::c_int = 25i32;
```

### `AF_LLC`

```rust
const AF_LLC: crate::c_int = 26i32;
```

### `AF_CAN`

```rust
const AF_CAN: crate::c_int = 29i32;
```

### `AF_TIPC`

```rust
const AF_TIPC: crate::c_int = 30i32;
```

### `AF_BLUETOOTH`

```rust
const AF_BLUETOOTH: crate::c_int = 31i32;
```

### `AF_IUCV`

```rust
const AF_IUCV: crate::c_int = 32i32;
```

### `AF_RXRPC`

```rust
const AF_RXRPC: crate::c_int = 33i32;
```

### `AF_ISDN`

```rust
const AF_ISDN: crate::c_int = 34i32;
```

### `AF_PHONET`

```rust
const AF_PHONET: crate::c_int = 35i32;
```

### `AF_IEEE802154`

```rust
const AF_IEEE802154: crate::c_int = 36i32;
```

### `AF_CAIF`

```rust
const AF_CAIF: crate::c_int = 37i32;
```

### `AF_ALG`

```rust
const AF_ALG: crate::c_int = 38i32;
```

### `PF_UNSPEC`

```rust
const PF_UNSPEC: crate::c_int = 0i32;
```

### `PF_UNIX`

```rust
const PF_UNIX: crate::c_int = 1i32;
```

### `PF_LOCAL`

```rust
const PF_LOCAL: crate::c_int = 1i32;
```

### `PF_INET`

```rust
const PF_INET: crate::c_int = 2i32;
```

### `PF_AX25`

```rust
const PF_AX25: crate::c_int = 3i32;
```

### `PF_IPX`

```rust
const PF_IPX: crate::c_int = 4i32;
```

### `PF_APPLETALK`

```rust
const PF_APPLETALK: crate::c_int = 5i32;
```

### `PF_NETROM`

```rust
const PF_NETROM: crate::c_int = 6i32;
```

### `PF_BRIDGE`

```rust
const PF_BRIDGE: crate::c_int = 7i32;
```

### `PF_ATMPVC`

```rust
const PF_ATMPVC: crate::c_int = 8i32;
```

### `PF_X25`

```rust
const PF_X25: crate::c_int = 9i32;
```

### `PF_INET6`

```rust
const PF_INET6: crate::c_int = 10i32;
```

### `PF_ROSE`

```rust
const PF_ROSE: crate::c_int = 11i32;
```

### `PF_DECnet`

```rust
const PF_DECnet: crate::c_int = 12i32;
```

### `PF_NETBEUI`

```rust
const PF_NETBEUI: crate::c_int = 13i32;
```

### `PF_SECURITY`

```rust
const PF_SECURITY: crate::c_int = 14i32;
```

### `PF_KEY`

```rust
const PF_KEY: crate::c_int = 15i32;
```

### `PF_NETLINK`

```rust
const PF_NETLINK: crate::c_int = 16i32;
```

### `PF_ROUTE`

```rust
const PF_ROUTE: crate::c_int = 16i32;
```

### `PF_PACKET`

```rust
const PF_PACKET: crate::c_int = 17i32;
```

### `PF_ASH`

```rust
const PF_ASH: crate::c_int = 18i32;
```

### `PF_ECONET`

```rust
const PF_ECONET: crate::c_int = 19i32;
```

### `PF_ATMSVC`

```rust
const PF_ATMSVC: crate::c_int = 20i32;
```

### `PF_RDS`

```rust
const PF_RDS: crate::c_int = 21i32;
```

### `PF_SNA`

```rust
const PF_SNA: crate::c_int = 22i32;
```

### `PF_IRDA`

```rust
const PF_IRDA: crate::c_int = 23i32;
```

### `PF_PPPOX`

```rust
const PF_PPPOX: crate::c_int = 24i32;
```

### `PF_WANPIPE`

```rust
const PF_WANPIPE: crate::c_int = 25i32;
```

### `PF_LLC`

```rust
const PF_LLC: crate::c_int = 26i32;
```

### `PF_CAN`

```rust
const PF_CAN: crate::c_int = 29i32;
```

### `PF_TIPC`

```rust
const PF_TIPC: crate::c_int = 30i32;
```

### `PF_BLUETOOTH`

```rust
const PF_BLUETOOTH: crate::c_int = 31i32;
```

### `PF_IUCV`

```rust
const PF_IUCV: crate::c_int = 32i32;
```

### `PF_RXRPC`

```rust
const PF_RXRPC: crate::c_int = 33i32;
```

### `PF_ISDN`

```rust
const PF_ISDN: crate::c_int = 34i32;
```

### `PF_PHONET`

```rust
const PF_PHONET: crate::c_int = 35i32;
```

### `PF_IEEE802154`

```rust
const PF_IEEE802154: crate::c_int = 36i32;
```

### `PF_CAIF`

```rust
const PF_CAIF: crate::c_int = 37i32;
```

### `PF_ALG`

```rust
const PF_ALG: crate::c_int = 38i32;
```

### `MSG_OOB`

```rust
const MSG_OOB: crate::c_int = 1i32;
```

### `MSG_PEEK`

```rust
const MSG_PEEK: crate::c_int = 2i32;
```

### `MSG_DONTROUTE`

```rust
const MSG_DONTROUTE: crate::c_int = 4i32;
```

### `MSG_CTRUNC`

```rust
const MSG_CTRUNC: crate::c_int = 8i32;
```

### `MSG_TRUNC`

```rust
const MSG_TRUNC: crate::c_int = 32i32;
```

### `MSG_DONTWAIT`

```rust
const MSG_DONTWAIT: crate::c_int = 64i32;
```

### `MSG_EOR`

```rust
const MSG_EOR: crate::c_int = 128i32;
```

### `MSG_WAITALL`

```rust
const MSG_WAITALL: crate::c_int = 256i32;
```

### `MSG_FIN`

```rust
const MSG_FIN: crate::c_int = 512i32;
```

### `MSG_SYN`

```rust
const MSG_SYN: crate::c_int = 1_024i32;
```

### `MSG_CONFIRM`

```rust
const MSG_CONFIRM: crate::c_int = 2_048i32;
```

### `MSG_RST`

```rust
const MSG_RST: crate::c_int = 4_096i32;
```

### `MSG_ERRQUEUE`

```rust
const MSG_ERRQUEUE: crate::c_int = 8_192i32;
```

### `MSG_NOSIGNAL`

```rust
const MSG_NOSIGNAL: crate::c_int = 16_384i32;
```

### `MSG_MORE`

```rust
const MSG_MORE: crate::c_int = 32_768i32;
```

### `MSG_WAITFORONE`

```rust
const MSG_WAITFORONE: crate::c_int = 65_536i32;
```

### `MSG_FASTOPEN`

```rust
const MSG_FASTOPEN: crate::c_int = 536_870_912i32;
```

### `MSG_CMSG_CLOEXEC`

```rust
const MSG_CMSG_CLOEXEC: crate::c_int = 1_073_741_824i32;
```

### `SCM_TIMESTAMP`

```rust
const SCM_TIMESTAMP: crate::c_int = 29i32;
```

### `SOCK_RAW`

```rust
const SOCK_RAW: crate::c_int = 3i32;
```

### `SOCK_RDM`

```rust
const SOCK_RDM: crate::c_int = 4i32;
```

### `IP_TOS`

```rust
const IP_TOS: crate::c_int = 1i32;
```

### `IP_TTL`

```rust
const IP_TTL: crate::c_int = 2i32;
```

### `IP_HDRINCL`

```rust
const IP_HDRINCL: crate::c_int = 3i32;
```

### `IP_OPTIONS`

```rust
const IP_OPTIONS: crate::c_int = 4i32;
```

### `IP_ROUTER_ALERT`

```rust
const IP_ROUTER_ALERT: crate::c_int = 5i32;
```

### `IP_RECVOPTS`

```rust
const IP_RECVOPTS: crate::c_int = 6i32;
```

### `IP_RETOPTS`

```rust
const IP_RETOPTS: crate::c_int = 7i32;
```

### `IP_PKTINFO`

```rust
const IP_PKTINFO: crate::c_int = 8i32;
```

### `IP_PKTOPTIONS`

```rust
const IP_PKTOPTIONS: crate::c_int = 9i32;
```

### `IP_MTU_DISCOVER`

```rust
const IP_MTU_DISCOVER: crate::c_int = 10i32;
```

### `IP_RECVERR`

```rust
const IP_RECVERR: crate::c_int = 11i32;
```

### `IP_RECVTTL`

```rust
const IP_RECVTTL: crate::c_int = 12i32;
```

### `IP_RECVTOS`

```rust
const IP_RECVTOS: crate::c_int = 13i32;
```

### `IP_MTU`

```rust
const IP_MTU: crate::c_int = 14i32;
```

### `IP_FREEBIND`

```rust
const IP_FREEBIND: crate::c_int = 15i32;
```

### `IP_IPSEC_POLICY`

```rust
const IP_IPSEC_POLICY: crate::c_int = 16i32;
```

### `IP_XFRM_POLICY`

```rust
const IP_XFRM_POLICY: crate::c_int = 17i32;
```

### `IP_PASSSEC`

```rust
const IP_PASSSEC: crate::c_int = 18i32;
```

### `IP_TRANSPARENT`

```rust
const IP_TRANSPARENT: crate::c_int = 19i32;
```

### `IP_ORIGDSTADDR`

```rust
const IP_ORIGDSTADDR: crate::c_int = 20i32;
```

### `IP_RECVORIGDSTADDR`

```rust
const IP_RECVORIGDSTADDR: crate::c_int = 20i32;
```

### `IP_MINTTL`

```rust
const IP_MINTTL: crate::c_int = 21i32;
```

### `IP_NODEFRAG`

```rust
const IP_NODEFRAG: crate::c_int = 22i32;
```

### `IP_CHECKSUM`

```rust
const IP_CHECKSUM: crate::c_int = 23i32;
```

### `IP_BIND_ADDRESS_NO_PORT`

```rust
const IP_BIND_ADDRESS_NO_PORT: crate::c_int = 24i32;
```

### `IP_MULTICAST_IF`

```rust
const IP_MULTICAST_IF: crate::c_int = 32i32;
```

### `IP_MULTICAST_TTL`

```rust
const IP_MULTICAST_TTL: crate::c_int = 33i32;
```

### `IP_MULTICAST_LOOP`

```rust
const IP_MULTICAST_LOOP: crate::c_int = 34i32;
```

### `IP_ADD_MEMBERSHIP`

```rust
const IP_ADD_MEMBERSHIP: crate::c_int = 35i32;
```

### `IP_DROP_MEMBERSHIP`

```rust
const IP_DROP_MEMBERSHIP: crate::c_int = 36i32;
```

### `IP_UNBLOCK_SOURCE`

```rust
const IP_UNBLOCK_SOURCE: crate::c_int = 37i32;
```

### `IP_BLOCK_SOURCE`

```rust
const IP_BLOCK_SOURCE: crate::c_int = 38i32;
```

### `IP_ADD_SOURCE_MEMBERSHIP`

```rust
const IP_ADD_SOURCE_MEMBERSHIP: crate::c_int = 39i32;
```

### `IP_DROP_SOURCE_MEMBERSHIP`

```rust
const IP_DROP_SOURCE_MEMBERSHIP: crate::c_int = 40i32;
```

### `IP_MSFILTER`

```rust
const IP_MSFILTER: crate::c_int = 41i32;
```

### `IP_MULTICAST_ALL`

```rust
const IP_MULTICAST_ALL: crate::c_int = 49i32;
```

### `IP_UNICAST_IF`

```rust
const IP_UNICAST_IF: crate::c_int = 50i32;
```

### `IP_DEFAULT_MULTICAST_TTL`

```rust
const IP_DEFAULT_MULTICAST_TTL: crate::c_int = 1i32;
```

### `IP_DEFAULT_MULTICAST_LOOP`

```rust
const IP_DEFAULT_MULTICAST_LOOP: crate::c_int = 1i32;
```

### `IP_PMTUDISC_DONT`

```rust
const IP_PMTUDISC_DONT: crate::c_int = 0i32;
```

### `IP_PMTUDISC_WANT`

```rust
const IP_PMTUDISC_WANT: crate::c_int = 1i32;
```

### `IP_PMTUDISC_DO`

```rust
const IP_PMTUDISC_DO: crate::c_int = 2i32;
```

### `IP_PMTUDISC_PROBE`

```rust
const IP_PMTUDISC_PROBE: crate::c_int = 3i32;
```

### `IP_PMTUDISC_INTERFACE`

```rust
const IP_PMTUDISC_INTERFACE: crate::c_int = 4i32;
```

### `IP_PMTUDISC_OMIT`

```rust
const IP_PMTUDISC_OMIT: crate::c_int = 5i32;
```

### `IPPROTO_HOPOPTS`

```rust
const IPPROTO_HOPOPTS: crate::c_int = 0i32;
```

Hop-by-hop option header

### `IPPROTO_IGMP`

```rust
const IPPROTO_IGMP: crate::c_int = 2i32;
```

group mgmt protocol

### `IPPROTO_IPIP`

```rust
const IPPROTO_IPIP: crate::c_int = 4i32;
```

for compatibility

### `IPPROTO_EGP`

```rust
const IPPROTO_EGP: crate::c_int = 8i32;
```

exterior gateway protocol

### `IPPROTO_PUP`

```rust
const IPPROTO_PUP: crate::c_int = 12i32;
```

pup

### `IPPROTO_IDP`

```rust
const IPPROTO_IDP: crate::c_int = 22i32;
```

xns idp

### `IPPROTO_TP`

```rust
const IPPROTO_TP: crate::c_int = 29i32;
```

tp-4 w/ class negotiation

### `IPPROTO_DCCP`

```rust
const IPPROTO_DCCP: crate::c_int = 33i32;
```

DCCP

### `IPPROTO_ROUTING`

```rust
const IPPROTO_ROUTING: crate::c_int = 43i32;
```

IP6 routing header

### `IPPROTO_FRAGMENT`

```rust
const IPPROTO_FRAGMENT: crate::c_int = 44i32;
```

IP6 fragmentation header

### `IPPROTO_RSVP`

```rust
const IPPROTO_RSVP: crate::c_int = 46i32;
```

resource reservation

### `IPPROTO_GRE`

```rust
const IPPROTO_GRE: crate::c_int = 47i32;
```

General Routing Encap.

### `IPPROTO_ESP`

```rust
const IPPROTO_ESP: crate::c_int = 50i32;
```

IP6 Encap Sec. Payload

### `IPPROTO_AH`

```rust
const IPPROTO_AH: crate::c_int = 51i32;
```

IP6 Auth Header

### `IPPROTO_NONE`

```rust
const IPPROTO_NONE: crate::c_int = 59i32;
```

IP6 no next header

### `IPPROTO_DSTOPTS`

```rust
const IPPROTO_DSTOPTS: crate::c_int = 60i32;
```

IP6 destination option

### `IPPROTO_MTP`

```rust
const IPPROTO_MTP: crate::c_int = 92i32;
```

### `IPPROTO_ENCAP`

```rust
const IPPROTO_ENCAP: crate::c_int = 98i32;
```

encapsulation header

### `IPPROTO_PIM`

```rust
const IPPROTO_PIM: crate::c_int = 103i32;
```

Protocol indep. multicast

### `IPPROTO_COMP`

```rust
const IPPROTO_COMP: crate::c_int = 108i32;
```

IP Payload Comp. Protocol

### `IPPROTO_SCTP`

```rust
const IPPROTO_SCTP: crate::c_int = 132i32;
```

SCTP

### `IPPROTO_MH`

```rust
const IPPROTO_MH: crate::c_int = 135i32;
```

### `IPPROTO_UDPLITE`

```rust
const IPPROTO_UDPLITE: crate::c_int = 136i32;
```

### `IPPROTO_RAW`

```rust
const IPPROTO_RAW: crate::c_int = 255i32;
```

raw IP packet

### `IPPROTO_BEETPH`

```rust
const IPPROTO_BEETPH: crate::c_int = 94i32;
```

### `IPPROTO_MPLS`

```rust
const IPPROTO_MPLS: crate::c_int = 137i32;
```

### `IPPROTO_MPTCP`

```rust
const IPPROTO_MPTCP: crate::c_int = 262i32;
```

Multipath TCP

### `IPPROTO_ETHERNET`

```rust
const IPPROTO_ETHERNET: crate::c_int = 143i32;
```

Ethernet-within-IPv6 encapsulation.

### `MCAST_EXCLUDE`

```rust
const MCAST_EXCLUDE: crate::c_int = 0i32;
```

### `MCAST_INCLUDE`

```rust
const MCAST_INCLUDE: crate::c_int = 1i32;
```

### `MCAST_JOIN_GROUP`

```rust
const MCAST_JOIN_GROUP: crate::c_int = 42i32;
```

### `MCAST_BLOCK_SOURCE`

```rust
const MCAST_BLOCK_SOURCE: crate::c_int = 43i32;
```

### `MCAST_UNBLOCK_SOURCE`

```rust
const MCAST_UNBLOCK_SOURCE: crate::c_int = 44i32;
```

### `MCAST_LEAVE_GROUP`

```rust
const MCAST_LEAVE_GROUP: crate::c_int = 45i32;
```

### `MCAST_JOIN_SOURCE_GROUP`

```rust
const MCAST_JOIN_SOURCE_GROUP: crate::c_int = 46i32;
```

### `MCAST_LEAVE_SOURCE_GROUP`

```rust
const MCAST_LEAVE_SOURCE_GROUP: crate::c_int = 47i32;
```

### `MCAST_MSFILTER`

```rust
const MCAST_MSFILTER: crate::c_int = 48i32;
```

### `IPV6_ADDRFORM`

```rust
const IPV6_ADDRFORM: crate::c_int = 1i32;
```

### `IPV6_2292PKTINFO`

```rust
const IPV6_2292PKTINFO: crate::c_int = 2i32;
```

### `IPV6_2292HOPOPTS`

```rust
const IPV6_2292HOPOPTS: crate::c_int = 3i32;
```

### `IPV6_2292DSTOPTS`

```rust
const IPV6_2292DSTOPTS: crate::c_int = 4i32;
```

### `IPV6_2292RTHDR`

```rust
const IPV6_2292RTHDR: crate::c_int = 5i32;
```

### `IPV6_2292PKTOPTIONS`

```rust
const IPV6_2292PKTOPTIONS: crate::c_int = 6i32;
```

### `IPV6_CHECKSUM`

```rust
const IPV6_CHECKSUM: crate::c_int = 7i32;
```

### `IPV6_2292HOPLIMIT`

```rust
const IPV6_2292HOPLIMIT: crate::c_int = 8i32;
```

### `IPV6_NEXTHOP`

```rust
const IPV6_NEXTHOP: crate::c_int = 9i32;
```

### `IPV6_AUTHHDR`

```rust
const IPV6_AUTHHDR: crate::c_int = 10i32;
```

### `IPV6_UNICAST_HOPS`

```rust
const IPV6_UNICAST_HOPS: crate::c_int = 16i32;
```

### `IPV6_MULTICAST_IF`

```rust
const IPV6_MULTICAST_IF: crate::c_int = 17i32;
```

### `IPV6_MULTICAST_HOPS`

```rust
const IPV6_MULTICAST_HOPS: crate::c_int = 18i32;
```

### `IPV6_MULTICAST_LOOP`

```rust
const IPV6_MULTICAST_LOOP: crate::c_int = 19i32;
```

### `IPV6_ADD_MEMBERSHIP`

```rust
const IPV6_ADD_MEMBERSHIP: crate::c_int = 20i32;
```

### `IPV6_DROP_MEMBERSHIP`

```rust
const IPV6_DROP_MEMBERSHIP: crate::c_int = 21i32;
```

### `IPV6_ROUTER_ALERT`

```rust
const IPV6_ROUTER_ALERT: crate::c_int = 22i32;
```

### `IPV6_MTU_DISCOVER`

```rust
const IPV6_MTU_DISCOVER: crate::c_int = 23i32;
```

### `IPV6_MTU`

```rust
const IPV6_MTU: crate::c_int = 24i32;
```

### `IPV6_RECVERR`

```rust
const IPV6_RECVERR: crate::c_int = 25i32;
```

### `IPV6_V6ONLY`

```rust
const IPV6_V6ONLY: crate::c_int = 26i32;
```

### `IPV6_JOIN_ANYCAST`

```rust
const IPV6_JOIN_ANYCAST: crate::c_int = 27i32;
```

### `IPV6_LEAVE_ANYCAST`

```rust
const IPV6_LEAVE_ANYCAST: crate::c_int = 28i32;
```

### `IPV6_IPSEC_POLICY`

```rust
const IPV6_IPSEC_POLICY: crate::c_int = 34i32;
```

### `IPV6_XFRM_POLICY`

```rust
const IPV6_XFRM_POLICY: crate::c_int = 35i32;
```

### `IPV6_HDRINCL`

```rust
const IPV6_HDRINCL: crate::c_int = 36i32;
```

### `IPV6_RECVPKTINFO`

```rust
const IPV6_RECVPKTINFO: crate::c_int = 49i32;
```

### `IPV6_PKTINFO`

```rust
const IPV6_PKTINFO: crate::c_int = 50i32;
```

### `IPV6_RECVHOPLIMIT`

```rust
const IPV6_RECVHOPLIMIT: crate::c_int = 51i32;
```

### `IPV6_HOPLIMIT`

```rust
const IPV6_HOPLIMIT: crate::c_int = 52i32;
```

### `IPV6_RECVHOPOPTS`

```rust
const IPV6_RECVHOPOPTS: crate::c_int = 53i32;
```

### `IPV6_HOPOPTS`

```rust
const IPV6_HOPOPTS: crate::c_int = 54i32;
```

### `IPV6_RTHDRDSTOPTS`

```rust
const IPV6_RTHDRDSTOPTS: crate::c_int = 55i32;
```

### `IPV6_RECVRTHDR`

```rust
const IPV6_RECVRTHDR: crate::c_int = 56i32;
```

### `IPV6_RTHDR`

```rust
const IPV6_RTHDR: crate::c_int = 57i32;
```

### `IPV6_RECVDSTOPTS`

```rust
const IPV6_RECVDSTOPTS: crate::c_int = 58i32;
```

### `IPV6_DSTOPTS`

```rust
const IPV6_DSTOPTS: crate::c_int = 59i32;
```

### `IPV6_RECVPATHMTU`

```rust
const IPV6_RECVPATHMTU: crate::c_int = 60i32;
```

### `IPV6_PATHMTU`

```rust
const IPV6_PATHMTU: crate::c_int = 61i32;
```

### `IPV6_DONTFRAG`

```rust
const IPV6_DONTFRAG: crate::c_int = 62i32;
```

### `IPV6_RECVTCLASS`

```rust
const IPV6_RECVTCLASS: crate::c_int = 66i32;
```

### `IPV6_TCLASS`

```rust
const IPV6_TCLASS: crate::c_int = 67i32;
```

### `IPV6_AUTOFLOWLABEL`

```rust
const IPV6_AUTOFLOWLABEL: crate::c_int = 70i32;
```

### `IPV6_ADDR_PREFERENCES`

```rust
const IPV6_ADDR_PREFERENCES: crate::c_int = 72i32;
```

### `IPV6_MINHOPCOUNT`

```rust
const IPV6_MINHOPCOUNT: crate::c_int = 73i32;
```

### `IPV6_ORIGDSTADDR`

```rust
const IPV6_ORIGDSTADDR: crate::c_int = 74i32;
```

### `IPV6_RECVORIGDSTADDR`

```rust
const IPV6_RECVORIGDSTADDR: crate::c_int = 74i32;
```

### `IPV6_TRANSPARENT`

```rust
const IPV6_TRANSPARENT: crate::c_int = 75i32;
```

### `IPV6_UNICAST_IF`

```rust
const IPV6_UNICAST_IF: crate::c_int = 76i32;
```

### `IPV6_PREFER_SRC_TMP`

```rust
const IPV6_PREFER_SRC_TMP: crate::c_int = 1i32;
```

### `IPV6_PREFER_SRC_PUBLIC`

```rust
const IPV6_PREFER_SRC_PUBLIC: crate::c_int = 2i32;
```

### `IPV6_PREFER_SRC_PUBTMP_DEFAULT`

```rust
const IPV6_PREFER_SRC_PUBTMP_DEFAULT: crate::c_int = 256i32;
```

### `IPV6_PREFER_SRC_COA`

```rust
const IPV6_PREFER_SRC_COA: crate::c_int = 4i32;
```

### `IPV6_PREFER_SRC_HOME`

```rust
const IPV6_PREFER_SRC_HOME: crate::c_int = 1_024i32;
```

### `IPV6_PREFER_SRC_CGA`

```rust
const IPV6_PREFER_SRC_CGA: crate::c_int = 8i32;
```

### `IPV6_PREFER_SRC_NONCGA`

```rust
const IPV6_PREFER_SRC_NONCGA: crate::c_int = 2_048i32;
```

### `IPV6_PMTUDISC_DONT`

```rust
const IPV6_PMTUDISC_DONT: crate::c_int = 0i32;
```

### `IPV6_PMTUDISC_WANT`

```rust
const IPV6_PMTUDISC_WANT: crate::c_int = 1i32;
```

### `IPV6_PMTUDISC_DO`

```rust
const IPV6_PMTUDISC_DO: crate::c_int = 2i32;
```

### `IPV6_PMTUDISC_PROBE`

```rust
const IPV6_PMTUDISC_PROBE: crate::c_int = 3i32;
```

### `IPV6_PMTUDISC_INTERFACE`

```rust
const IPV6_PMTUDISC_INTERFACE: crate::c_int = 4i32;
```

### `IPV6_PMTUDISC_OMIT`

```rust
const IPV6_PMTUDISC_OMIT: crate::c_int = 5i32;
```

### `TCP_NODELAY`

```rust
const TCP_NODELAY: crate::c_int = 1i32;
```

### `TCP_MAXSEG`

```rust
const TCP_MAXSEG: crate::c_int = 2i32;
```

### `TCP_CORK`

```rust
const TCP_CORK: crate::c_int = 3i32;
```

### `TCP_KEEPIDLE`

```rust
const TCP_KEEPIDLE: crate::c_int = 4i32;
```

### `TCP_KEEPINTVL`

```rust
const TCP_KEEPINTVL: crate::c_int = 5i32;
```

### `TCP_KEEPCNT`

```rust
const TCP_KEEPCNT: crate::c_int = 6i32;
```

### `TCP_SYNCNT`

```rust
const TCP_SYNCNT: crate::c_int = 7i32;
```

### `TCP_LINGER2`

```rust
const TCP_LINGER2: crate::c_int = 8i32;
```

### `TCP_DEFER_ACCEPT`

```rust
const TCP_DEFER_ACCEPT: crate::c_int = 9i32;
```

### `TCP_WINDOW_CLAMP`

```rust
const TCP_WINDOW_CLAMP: crate::c_int = 10i32;
```

### `TCP_INFO`

```rust
const TCP_INFO: crate::c_int = 11i32;
```

### `TCP_QUICKACK`

```rust
const TCP_QUICKACK: crate::c_int = 12i32;
```

### `TCP_CONGESTION`

```rust
const TCP_CONGESTION: crate::c_int = 13i32;
```

### `TCP_MD5SIG`

```rust
const TCP_MD5SIG: crate::c_int = 14i32;
```

### `TCP_COOKIE_TRANSACTIONS`

```rust
const TCP_COOKIE_TRANSACTIONS: crate::c_int = 15i32;
```

### `TCP_THIN_LINEAR_TIMEOUTS`

```rust
const TCP_THIN_LINEAR_TIMEOUTS: crate::c_int = 16i32;
```

### `TCP_THIN_DUPACK`

```rust
const TCP_THIN_DUPACK: crate::c_int = 17i32;
```

### `TCP_USER_TIMEOUT`

```rust
const TCP_USER_TIMEOUT: crate::c_int = 18i32;
```

### `TCP_REPAIR`

```rust
const TCP_REPAIR: crate::c_int = 19i32;
```

### `TCP_REPAIR_QUEUE`

```rust
const TCP_REPAIR_QUEUE: crate::c_int = 20i32;
```

### `TCP_QUEUE_SEQ`

```rust
const TCP_QUEUE_SEQ: crate::c_int = 21i32;
```

### `TCP_REPAIR_OPTIONS`

```rust
const TCP_REPAIR_OPTIONS: crate::c_int = 22i32;
```

### `TCP_FASTOPEN`

```rust
const TCP_FASTOPEN: crate::c_int = 23i32;
```

### `TCP_TIMESTAMP`

```rust
const TCP_TIMESTAMP: crate::c_int = 24i32;
```

### `TCP_NOTSENT_LOWAT`

```rust
const TCP_NOTSENT_LOWAT: crate::c_int = 25i32;
```

### `TCP_CC_INFO`

```rust
const TCP_CC_INFO: crate::c_int = 26i32;
```

### `TCP_SAVE_SYN`

```rust
const TCP_SAVE_SYN: crate::c_int = 27i32;
```

### `TCP_SAVED_SYN`

```rust
const TCP_SAVED_SYN: crate::c_int = 28i32;
```

### `TCP_REPAIR_WINDOW`

```rust
const TCP_REPAIR_WINDOW: crate::c_int = 29i32;
```

### `TCP_FASTOPEN_CONNECT`

```rust
const TCP_FASTOPEN_CONNECT: crate::c_int = 30i32;
```

### `TCP_ULP`

```rust
const TCP_ULP: crate::c_int = 31i32;
```

### `TCP_MD5SIG_EXT`

```rust
const TCP_MD5SIG_EXT: crate::c_int = 32i32;
```

### `TCP_FASTOPEN_KEY`

```rust
const TCP_FASTOPEN_KEY: crate::c_int = 33i32;
```

### `TCP_FASTOPEN_NO_COOKIE`

```rust
const TCP_FASTOPEN_NO_COOKIE: crate::c_int = 34i32;
```

### `TCP_ZEROCOPY_RECEIVE`

```rust
const TCP_ZEROCOPY_RECEIVE: crate::c_int = 35i32;
```

### `TCP_INQ`

```rust
const TCP_INQ: crate::c_int = 36i32;
```

### `TCP_CM_INQ`

```rust
const TCP_CM_INQ: crate::c_int = 36i32;
```

### `TCP_MD5SIG_MAXKEYLEN`

```rust
const TCP_MD5SIG_MAXKEYLEN: usize = 80usize;
```

### `SO_DEBUG`

```rust
const SO_DEBUG: crate::c_int = 1i32;
```

### `SHUT_RD`

```rust
const SHUT_RD: crate::c_int = 0i32;
```

### `SHUT_WR`

```rust
const SHUT_WR: crate::c_int = 1i32;
```

### `SHUT_RDWR`

```rust
const SHUT_RDWR: crate::c_int = 2i32;
```

### `LOCK_SH`

```rust
const LOCK_SH: crate::c_int = 1i32;
```

### `LOCK_EX`

```rust
const LOCK_EX: crate::c_int = 2i32;
```

### `LOCK_NB`

```rust
const LOCK_NB: crate::c_int = 4i32;
```

### `LOCK_UN`

```rust
const LOCK_UN: crate::c_int = 8i32;
```

### `SS_ONSTACK`

```rust
const SS_ONSTACK: crate::c_int = 1i32;
```

### `SS_DISABLE`

```rust
const SS_DISABLE: crate::c_int = 2i32;
```

### `PATH_MAX`

```rust
const PATH_MAX: crate::c_int = 4_096i32;
```

### `UIO_MAXIOV`

```rust
const UIO_MAXIOV: crate::c_int = 1_024i32;
```

### `FD_SETSIZE`

```rust
const FD_SETSIZE: usize = 1_024usize;
```

### `EPOLLIN`

```rust
const EPOLLIN: crate::c_int = 1i32;
```

### `EPOLLPRI`

```rust
const EPOLLPRI: crate::c_int = 2i32;
```

### `EPOLLOUT`

```rust
const EPOLLOUT: crate::c_int = 4i32;
```

### `EPOLLERR`

```rust
const EPOLLERR: crate::c_int = 8i32;
```

### `EPOLLHUP`

```rust
const EPOLLHUP: crate::c_int = 16i32;
```

### `EPOLLRDNORM`

```rust
const EPOLLRDNORM: crate::c_int = 64i32;
```

### `EPOLLRDBAND`

```rust
const EPOLLRDBAND: crate::c_int = 128i32;
```

### `EPOLLWRNORM`

```rust
const EPOLLWRNORM: crate::c_int = 256i32;
```

### `EPOLLWRBAND`

```rust
const EPOLLWRBAND: crate::c_int = 512i32;
```

### `EPOLLMSG`

```rust
const EPOLLMSG: crate::c_int = 1_024i32;
```

### `EPOLLRDHUP`

```rust
const EPOLLRDHUP: crate::c_int = 8_192i32;
```

### `EPOLLEXCLUSIVE`

```rust
const EPOLLEXCLUSIVE: crate::c_int = 268_435_456i32;
```

### `EPOLLWAKEUP`

```rust
const EPOLLWAKEUP: crate::c_int = 536_870_912i32;
```

### `EPOLLONESHOT`

```rust
const EPOLLONESHOT: crate::c_int = 1_073_741_824i32;
```

### `EPOLLET`

```rust
const EPOLLET: crate::c_int = -2_147_483_648i32;
```

### `EPOLL_CTL_ADD`

```rust
const EPOLL_CTL_ADD: crate::c_int = 1i32;
```

### `EPOLL_CTL_MOD`

```rust
const EPOLL_CTL_MOD: crate::c_int = 3i32;
```

### `EPOLL_CTL_DEL`

```rust
const EPOLL_CTL_DEL: crate::c_int = 2i32;
```

### `MNT_FORCE`

```rust
const MNT_FORCE: crate::c_int = 1i32;
```

### `MNT_DETACH`

```rust
const MNT_DETACH: crate::c_int = 2i32;
```

### `MNT_EXPIRE`

```rust
const MNT_EXPIRE: crate::c_int = 4i32;
```

### `UMOUNT_NOFOLLOW`

```rust
const UMOUNT_NOFOLLOW: crate::c_int = 8i32;
```

### `Q_GETFMT`

```rust
const Q_GETFMT: crate::c_int = 8_388_612i32;
```

### `Q_GETINFO`

```rust
const Q_GETINFO: crate::c_int = 8_388_613i32;
```

### `Q_SETINFO`

```rust
const Q_SETINFO: crate::c_int = 8_388_614i32;
```

### `QIF_BLIMITS`

```rust
const QIF_BLIMITS: u32 = 1u32;
```

### `QIF_SPACE`

```rust
const QIF_SPACE: u32 = 2u32;
```

### `QIF_ILIMITS`

```rust
const QIF_ILIMITS: u32 = 4u32;
```

### `QIF_INODES`

```rust
const QIF_INODES: u32 = 8u32;
```

### `QIF_BTIME`

```rust
const QIF_BTIME: u32 = 16u32;
```

### `QIF_ITIME`

```rust
const QIF_ITIME: u32 = 32u32;
```

### `QIF_LIMITS`

```rust
const QIF_LIMITS: u32 = 5u32;
```

### `QIF_USAGE`

```rust
const QIF_USAGE: u32 = 10u32;
```

### `QIF_TIMES`

```rust
const QIF_TIMES: u32 = 48u32;
```

### `QIF_ALL`

```rust
const QIF_ALL: u32 = 63u32;
```

### `Q_SYNC`

```rust
const Q_SYNC: crate::c_int = 8_388_609i32;
```

### `Q_QUOTAON`

```rust
const Q_QUOTAON: crate::c_int = 8_388_610i32;
```

### `Q_QUOTAOFF`

```rust
const Q_QUOTAOFF: crate::c_int = 8_388_611i32;
```

### `Q_GETQUOTA`

```rust
const Q_GETQUOTA: crate::c_int = 8_388_615i32;
```

### `Q_SETQUOTA`

```rust
const Q_SETQUOTA: crate::c_int = 8_388_616i32;
```

### `TCIOFF`

```rust
const TCIOFF: crate::c_int = 2i32;
```

### `TCION`

```rust
const TCION: crate::c_int = 3i32;
```

### `TCOOFF`

```rust
const TCOOFF: crate::c_int = 0i32;
```

### `TCOON`

```rust
const TCOON: crate::c_int = 1i32;
```

### `TCIFLUSH`

```rust
const TCIFLUSH: crate::c_int = 0i32;
```

### `TCOFLUSH`

```rust
const TCOFLUSH: crate::c_int = 1i32;
```

### `TCIOFLUSH`

```rust
const TCIOFLUSH: crate::c_int = 2i32;
```

### `NL0`

```rust
const NL0: crate::tcflag_t = 0u32;
```

### `NL1`

```rust
const NL1: crate::tcflag_t = 256u32;
```

### `TAB0`

```rust
const TAB0: crate::tcflag_t = 0u32;
```

### `CR0`

```rust
const CR0: crate::tcflag_t = 0u32;
```

### `FF0`

```rust
const FF0: crate::tcflag_t = 0u32;
```

### `BS0`

```rust
const BS0: crate::tcflag_t = 0u32;
```

### `VT0`

```rust
const VT0: crate::tcflag_t = 0u32;
```

### `VERASE`

```rust
const VERASE: usize = 2usize;
```

### `VKILL`

```rust
const VKILL: usize = 3usize;
```

### `VINTR`

```rust
const VINTR: usize = 0usize;
```

### `VQUIT`

```rust
const VQUIT: usize = 1usize;
```

### `VLNEXT`

```rust
const VLNEXT: usize = 15usize;
```

### `IGNBRK`

```rust
const IGNBRK: crate::tcflag_t = 1u32;
```

### `BRKINT`

```rust
const BRKINT: crate::tcflag_t = 2u32;
```

### `IGNPAR`

```rust
const IGNPAR: crate::tcflag_t = 4u32;
```

### `PARMRK`

```rust
const PARMRK: crate::tcflag_t = 8u32;
```

### `INPCK`

```rust
const INPCK: crate::tcflag_t = 16u32;
```

### `ISTRIP`

```rust
const ISTRIP: crate::tcflag_t = 32u32;
```

### `INLCR`

```rust
const INLCR: crate::tcflag_t = 64u32;
```

### `IGNCR`

```rust
const IGNCR: crate::tcflag_t = 128u32;
```

### `ICRNL`

```rust
const ICRNL: crate::tcflag_t = 256u32;
```

### `IXANY`

```rust
const IXANY: crate::tcflag_t = 2_048u32;
```

### `IMAXBEL`

```rust
const IMAXBEL: crate::tcflag_t = 8_192u32;
```

### `OPOST`

```rust
const OPOST: crate::tcflag_t = 1u32;
```

### `CS5`

```rust
const CS5: crate::tcflag_t = 0u32;
```

### `CRTSCTS`

```rust
const CRTSCTS: crate::tcflag_t = 2_147_483_648u32;
```

### `ECHO`

```rust
const ECHO: crate::tcflag_t = 8u32;
```

### `OCRNL`

```rust
const OCRNL: crate::tcflag_t = 8u32;
```

### `ONOCR`

```rust
const ONOCR: crate::tcflag_t = 16u32;
```

### `ONLRET`

```rust
const ONLRET: crate::tcflag_t = 32u32;
```

### `OFILL`

```rust
const OFILL: crate::tcflag_t = 64u32;
```

### `OFDEL`

```rust
const OFDEL: crate::tcflag_t = 128u32;
```

### `CLONE_VM`

```rust
const CLONE_VM: crate::c_int = 256i32;
```

### `CLONE_FS`

```rust
const CLONE_FS: crate::c_int = 512i32;
```

### `CLONE_FILES`

```rust
const CLONE_FILES: crate::c_int = 1_024i32;
```

### `CLONE_SIGHAND`

```rust
const CLONE_SIGHAND: crate::c_int = 2_048i32;
```

### `CLONE_PTRACE`

```rust
const CLONE_PTRACE: crate::c_int = 8_192i32;
```

### `CLONE_VFORK`

```rust
const CLONE_VFORK: crate::c_int = 16_384i32;
```

### `CLONE_PARENT`

```rust
const CLONE_PARENT: crate::c_int = 32_768i32;
```

### `CLONE_THREAD`

```rust
const CLONE_THREAD: crate::c_int = 65_536i32;
```

### `CLONE_NEWNS`

```rust
const CLONE_NEWNS: crate::c_int = 131_072i32;
```

### `CLONE_SYSVSEM`

```rust
const CLONE_SYSVSEM: crate::c_int = 262_144i32;
```

### `CLONE_SETTLS`

```rust
const CLONE_SETTLS: crate::c_int = 524_288i32;
```

### `CLONE_PARENT_SETTID`

```rust
const CLONE_PARENT_SETTID: crate::c_int = 1_048_576i32;
```

### `CLONE_CHILD_CLEARTID`

```rust
const CLONE_CHILD_CLEARTID: crate::c_int = 2_097_152i32;
```

### `CLONE_DETACHED`

```rust
const CLONE_DETACHED: crate::c_int = 4_194_304i32;
```

### `CLONE_UNTRACED`

```rust
const CLONE_UNTRACED: crate::c_int = 8_388_608i32;
```

### `CLONE_CHILD_SETTID`

```rust
const CLONE_CHILD_SETTID: crate::c_int = 16_777_216i32;
```

### `CLONE_NEWCGROUP`

```rust
const CLONE_NEWCGROUP: crate::c_int = 33_554_432i32;
```

### `CLONE_NEWUTS`

```rust
const CLONE_NEWUTS: crate::c_int = 67_108_864i32;
```

### `CLONE_NEWIPC`

```rust
const CLONE_NEWIPC: crate::c_int = 134_217_728i32;
```

### `CLONE_NEWUSER`

```rust
const CLONE_NEWUSER: crate::c_int = 268_435_456i32;
```

### `CLONE_NEWPID`

```rust
const CLONE_NEWPID: crate::c_int = 536_870_912i32;
```

### `CLONE_NEWNET`

```rust
const CLONE_NEWNET: crate::c_int = 1_073_741_824i32;
```

### `CLONE_IO`

```rust
const CLONE_IO: crate::c_int = -2_147_483_648i32;
```

### `WNOHANG`

```rust
const WNOHANG: crate::c_int = 1i32;
```

### `WUNTRACED`

```rust
const WUNTRACED: crate::c_int = 2i32;
```

### `WSTOPPED`

```rust
const WSTOPPED: crate::c_int = 2i32;
```

### `WEXITED`

```rust
const WEXITED: crate::c_int = 4i32;
```

### `WCONTINUED`

```rust
const WCONTINUED: crate::c_int = 8i32;
```

### `WNOWAIT`

```rust
const WNOWAIT: crate::c_int = 16_777_216i32;
```

### `ADDR_NO_RANDOMIZE`

```rust
const ADDR_NO_RANDOMIZE: crate::c_int = 262_144i32;
```

### `MMAP_PAGE_ZERO`

```rust
const MMAP_PAGE_ZERO: crate::c_int = 1_048_576i32;
```

### `ADDR_COMPAT_LAYOUT`

```rust
const ADDR_COMPAT_LAYOUT: crate::c_int = 2_097_152i32;
```

### `READ_IMPLIES_EXEC`

```rust
const READ_IMPLIES_EXEC: crate::c_int = 4_194_304i32;
```

### `ADDR_LIMIT_32BIT`

```rust
const ADDR_LIMIT_32BIT: crate::c_int = 8_388_608i32;
```

### `SHORT_INODE`

```rust
const SHORT_INODE: crate::c_int = 16_777_216i32;
```

### `WHOLE_SECONDS`

```rust
const WHOLE_SECONDS: crate::c_int = 33_554_432i32;
```

### `STICKY_TIMEOUTS`

```rust
const STICKY_TIMEOUTS: crate::c_int = 67_108_864i32;
```

### `ADDR_LIMIT_3GB`

```rust
const ADDR_LIMIT_3GB: crate::c_int = 134_217_728i32;
```

### `PTRACE_O_TRACESYSGOOD`

```rust
const PTRACE_O_TRACESYSGOOD: crate::c_int = 1i32;
```

### `PTRACE_O_TRACEFORK`

```rust
const PTRACE_O_TRACEFORK: crate::c_int = 2i32;
```

### `PTRACE_O_TRACEVFORK`

```rust
const PTRACE_O_TRACEVFORK: crate::c_int = 4i32;
```

### `PTRACE_O_TRACECLONE`

```rust
const PTRACE_O_TRACECLONE: crate::c_int = 8i32;
```

### `PTRACE_O_TRACEEXEC`

```rust
const PTRACE_O_TRACEEXEC: crate::c_int = 16i32;
```

### `PTRACE_O_TRACEVFORKDONE`

```rust
const PTRACE_O_TRACEVFORKDONE: crate::c_int = 32i32;
```

### `PTRACE_O_TRACEEXIT`

```rust
const PTRACE_O_TRACEEXIT: crate::c_int = 64i32;
```

### `PTRACE_O_TRACESECCOMP`

```rust
const PTRACE_O_TRACESECCOMP: crate::c_int = 128i32;
```

### `PTRACE_O_SUSPEND_SECCOMP`

```rust
const PTRACE_O_SUSPEND_SECCOMP: crate::c_int = 2_097_152i32;
```

### `PTRACE_O_EXITKILL`

```rust
const PTRACE_O_EXITKILL: crate::c_int = 1_048_576i32;
```

### `PTRACE_O_MASK`

```rust
const PTRACE_O_MASK: crate::c_int = 3_145_983i32;
```

### `PTRACE_EVENT_FORK`

```rust
const PTRACE_EVENT_FORK: crate::c_int = 1i32;
```

### `PTRACE_EVENT_VFORK`

```rust
const PTRACE_EVENT_VFORK: crate::c_int = 2i32;
```

### `PTRACE_EVENT_CLONE`

```rust
const PTRACE_EVENT_CLONE: crate::c_int = 3i32;
```

### `PTRACE_EVENT_EXEC`

```rust
const PTRACE_EVENT_EXEC: crate::c_int = 4i32;
```

### `PTRACE_EVENT_VFORK_DONE`

```rust
const PTRACE_EVENT_VFORK_DONE: crate::c_int = 5i32;
```

### `PTRACE_EVENT_EXIT`

```rust
const PTRACE_EVENT_EXIT: crate::c_int = 6i32;
```

### `PTRACE_EVENT_SECCOMP`

```rust
const PTRACE_EVENT_SECCOMP: crate::c_int = 7i32;
```

### `__WNOTHREAD`

```rust
const __WNOTHREAD: crate::c_int = 536_870_912i32;
```

### `__WALL`

```rust
const __WALL: crate::c_int = 1_073_741_824i32;
```

### `__WCLONE`

```rust
const __WCLONE: crate::c_int = -2_147_483_648i32;
```

### `SPLICE_F_MOVE`

```rust
const SPLICE_F_MOVE: crate::c_uint = 1u32;
```

### `SPLICE_F_NONBLOCK`

```rust
const SPLICE_F_NONBLOCK: crate::c_uint = 2u32;
```

### `SPLICE_F_MORE`

```rust
const SPLICE_F_MORE: crate::c_uint = 4u32;
```

### `SPLICE_F_GIFT`

```rust
const SPLICE_F_GIFT: crate::c_uint = 8u32;
```

### `RTLD_LOCAL`

```rust
const RTLD_LOCAL: crate::c_int = 0i32;
```

### `RTLD_LAZY`

```rust
const RTLD_LAZY: crate::c_int = 1i32;
```

### `POSIX_FADV_NORMAL`

```rust
const POSIX_FADV_NORMAL: crate::c_int = 0i32;
```

### `POSIX_FADV_RANDOM`

```rust
const POSIX_FADV_RANDOM: crate::c_int = 1i32;
```

### `POSIX_FADV_SEQUENTIAL`

```rust
const POSIX_FADV_SEQUENTIAL: crate::c_int = 2i32;
```

### `POSIX_FADV_WILLNEED`

```rust
const POSIX_FADV_WILLNEED: crate::c_int = 3i32;
```

### `AT_FDCWD`

```rust
const AT_FDCWD: crate::c_int = -100i32;
```

### `AT_SYMLINK_NOFOLLOW`

```rust
const AT_SYMLINK_NOFOLLOW: crate::c_int = 256i32;
```

### `AT_REMOVEDIR`

```rust
const AT_REMOVEDIR: crate::c_int = 512i32;
```

### `AT_SYMLINK_FOLLOW`

```rust
const AT_SYMLINK_FOLLOW: crate::c_int = 1_024i32;
```

### `AT_NO_AUTOMOUNT`

```rust
const AT_NO_AUTOMOUNT: crate::c_int = 2_048i32;
```

### `AT_EMPTY_PATH`

```rust
const AT_EMPTY_PATH: crate::c_int = 4_096i32;
```

### `AT_RECURSIVE`

```rust
const AT_RECURSIVE: crate::c_int = 32_768i32;
```

### `LOG_CRON`

```rust
const LOG_CRON: crate::c_int = 72i32;
```

### `LOG_AUTHPRIV`

```rust
const LOG_AUTHPRIV: crate::c_int = 80i32;
```

### `LOG_FTP`

```rust
const LOG_FTP: crate::c_int = 88i32;
```

### `LOG_PERROR`

```rust
const LOG_PERROR: crate::c_int = 32i32;
```

### `PIPE_BUF`

```rust
const PIPE_BUF: usize = 4_096usize;
```

### `SI_LOAD_SHIFT`

```rust
const SI_LOAD_SHIFT: crate::c_uint = 16u32;
```

### `SI_USER`

```rust
const SI_USER: crate::c_int = 0i32;
```

### `SI_KERNEL`

```rust
const SI_KERNEL: crate::c_int = 128i32;
```

### `SI_QUEUE`

```rust
const SI_QUEUE: crate::c_int = -1i32;
```

### `SI_TIMER`

```rust
const SI_TIMER: crate::c_int = -2i32;
```

### `SI_MESGQ`

```rust
const SI_MESGQ: crate::c_int = -3i32;
```

### `SI_ASYNCIO`

```rust
const SI_ASYNCIO: crate::c_int = -4i32;
```

### `SI_SIGIO`

```rust
const SI_SIGIO: crate::c_int = -5i32;
```

### `SI_TKILL`

```rust
const SI_TKILL: crate::c_int = -6i32;
```

### `SI_ASYNCNL`

```rust
const SI_ASYNCNL: crate::c_int = -60i32;
```

### `BUS_ADRALN`

```rust
const BUS_ADRALN: crate::c_int = 1i32;
```

### `BUS_ADRERR`

```rust
const BUS_ADRERR: crate::c_int = 2i32;
```

### `BUS_OBJERR`

```rust
const BUS_OBJERR: crate::c_int = 3i32;
```

### `BUS_MCEERR_AR`

```rust
const BUS_MCEERR_AR: crate::c_int = 4i32;
```

### `BUS_MCEERR_AO`

```rust
const BUS_MCEERR_AO: crate::c_int = 5i32;
```

### `TRAP_BRKPT`

```rust
const TRAP_BRKPT: crate::c_int = 1i32;
```

### `TRAP_TRACE`

```rust
const TRAP_TRACE: crate::c_int = 2i32;
```

### `TRAP_BRANCH`

```rust
const TRAP_BRANCH: crate::c_int = 3i32;
```

### `TRAP_HWBKPT`

```rust
const TRAP_HWBKPT: crate::c_int = 4i32;
```

### `TRAP_UNK`

```rust
const TRAP_UNK: crate::c_int = 5i32;
```

### `CLD_EXITED`

```rust
const CLD_EXITED: crate::c_int = 1i32;
```

### `CLD_KILLED`

```rust
const CLD_KILLED: crate::c_int = 2i32;
```

### `CLD_DUMPED`

```rust
const CLD_DUMPED: crate::c_int = 3i32;
```

### `CLD_TRAPPED`

```rust
const CLD_TRAPPED: crate::c_int = 4i32;
```

### `CLD_STOPPED`

```rust
const CLD_STOPPED: crate::c_int = 5i32;
```

### `CLD_CONTINUED`

```rust
const CLD_CONTINUED: crate::c_int = 6i32;
```

### `SIGEV_SIGNAL`

```rust
const SIGEV_SIGNAL: crate::c_int = 0i32;
```

### `SIGEV_NONE`

```rust
const SIGEV_NONE: crate::c_int = 1i32;
```

### `SIGEV_THREAD`

```rust
const SIGEV_THREAD: crate::c_int = 2i32;
```

### `P_ALL`

```rust
const P_ALL: idtype_t = 0u32;
```

### `P_PID`

```rust
const P_PID: idtype_t = 1u32;
```

### `P_PGID`

```rust
const P_PGID: idtype_t = 2u32;
```

### `P_PIDFD`

```rust
const P_PIDFD: idtype_t = 3u32;
```

### `UTIME_OMIT`

```rust
const UTIME_OMIT: crate::c_long = 1_073_741_822i64;
```

### `UTIME_NOW`

```rust
const UTIME_NOW: crate::c_long = 1_073_741_823i64;
```

### `POLLIN`

```rust
const POLLIN: crate::c_short = 1i16;
```

### `POLLPRI`

```rust
const POLLPRI: crate::c_short = 2i16;
```

### `POLLOUT`

```rust
const POLLOUT: crate::c_short = 4i16;
```

### `POLLERR`

```rust
const POLLERR: crate::c_short = 8i16;
```

### `POLLHUP`

```rust
const POLLHUP: crate::c_short = 16i16;
```

### `POLLNVAL`

```rust
const POLLNVAL: crate::c_short = 32i16;
```

### `POLLRDNORM`

```rust
const POLLRDNORM: crate::c_short = 64i16;
```

### `POLLRDBAND`

```rust
const POLLRDBAND: crate::c_short = 128i16;
```

### `POLLRDHUP`

```rust
const POLLRDHUP: crate::c_short = 8_192i16;
```

### `IPTOS_LOWDELAY`

```rust
const IPTOS_LOWDELAY: u8 = 16u8;
```

### `IPTOS_THROUGHPUT`

```rust
const IPTOS_THROUGHPUT: u8 = 8u8;
```

### `IPTOS_RELIABILITY`

```rust
const IPTOS_RELIABILITY: u8 = 4u8;
```

### `IPTOS_MINCOST`

```rust
const IPTOS_MINCOST: u8 = 2u8;
```

### `IPTOS_PREC_NETCONTROL`

```rust
const IPTOS_PREC_NETCONTROL: u8 = 224u8;
```

### `IPTOS_PREC_INTERNETCONTROL`

```rust
const IPTOS_PREC_INTERNETCONTROL: u8 = 192u8;
```

### `IPTOS_PREC_CRITIC_ECP`

```rust
const IPTOS_PREC_CRITIC_ECP: u8 = 160u8;
```

### `IPTOS_PREC_FLASHOVERRIDE`

```rust
const IPTOS_PREC_FLASHOVERRIDE: u8 = 128u8;
```

### `IPTOS_PREC_FLASH`

```rust
const IPTOS_PREC_FLASH: u8 = 96u8;
```

### `IPTOS_PREC_IMMEDIATE`

```rust
const IPTOS_PREC_IMMEDIATE: u8 = 64u8;
```

### `IPTOS_PREC_PRIORITY`

```rust
const IPTOS_PREC_PRIORITY: u8 = 32u8;
```

### `IPTOS_PREC_ROUTINE`

```rust
const IPTOS_PREC_ROUTINE: u8 = 0u8;
```

### `IPTOS_ECN_MASK`

```rust
const IPTOS_ECN_MASK: u8 = 3u8;
```

### `IPTOS_ECN_ECT1`

```rust
const IPTOS_ECN_ECT1: u8 = 1u8;
```

### `IPTOS_ECN_ECT0`

```rust
const IPTOS_ECN_ECT0: u8 = 2u8;
```

### `IPTOS_ECN_CE`

```rust
const IPTOS_ECN_CE: u8 = 3u8;
```

### `IPOPT_COPY`

```rust
const IPOPT_COPY: u8 = 128u8;
```

### `IPOPT_CLASS_MASK`

```rust
const IPOPT_CLASS_MASK: u8 = 96u8;
```

### `IPOPT_NUMBER_MASK`

```rust
const IPOPT_NUMBER_MASK: u8 = 31u8;
```

### `IPOPT_CONTROL`

```rust
const IPOPT_CONTROL: u8 = 0u8;
```

### `IPOPT_RESERVED1`

```rust
const IPOPT_RESERVED1: u8 = 32u8;
```

### `IPOPT_MEASUREMENT`

```rust
const IPOPT_MEASUREMENT: u8 = 64u8;
```

### `IPOPT_RESERVED2`

```rust
const IPOPT_RESERVED2: u8 = 96u8;
```

### `IPOPT_END`

```rust
const IPOPT_END: u8 = 0u8;
```

### `IPOPT_NOOP`

```rust
const IPOPT_NOOP: u8 = 1u8;
```

### `IPOPT_SEC`

```rust
const IPOPT_SEC: u8 = 130u8;
```

### `IPOPT_LSRR`

```rust
const IPOPT_LSRR: u8 = 131u8;
```

### `IPOPT_TIMESTAMP`

```rust
const IPOPT_TIMESTAMP: u8 = 68u8;
```

### `IPOPT_RR`

```rust
const IPOPT_RR: u8 = 7u8;
```

### `IPOPT_SID`

```rust
const IPOPT_SID: u8 = 136u8;
```

### `IPOPT_SSRR`

```rust
const IPOPT_SSRR: u8 = 137u8;
```

### `IPOPT_RA`

```rust
const IPOPT_RA: u8 = 148u8;
```

### `IPVERSION`

```rust
const IPVERSION: u8 = 4u8;
```

### `MAXTTL`

```rust
const MAXTTL: u8 = 255u8;
```

### `IPDEFTTL`

```rust
const IPDEFTTL: u8 = 64u8;
```

### `IPOPT_OPTVAL`

```rust
const IPOPT_OPTVAL: u8 = 0u8;
```

### `IPOPT_OLEN`

```rust
const IPOPT_OLEN: u8 = 1u8;
```

### `IPOPT_OFFSET`

```rust
const IPOPT_OFFSET: u8 = 2u8;
```

### `IPOPT_MINOFF`

```rust
const IPOPT_MINOFF: u8 = 4u8;
```

### `MAX_IPOPTLEN`

```rust
const MAX_IPOPTLEN: u8 = 40u8;
```

### `IPOPT_NOP`

```rust
const IPOPT_NOP: u8 = 1u8;
```

### `IPOPT_EOL`

```rust
const IPOPT_EOL: u8 = 0u8;
```

### `IPOPT_TS`

```rust
const IPOPT_TS: u8 = 68u8;
```

### `IPOPT_TS_TSONLY`

```rust
const IPOPT_TS_TSONLY: u8 = 0u8;
```

### `IPOPT_TS_TSANDADDR`

```rust
const IPOPT_TS_TSANDADDR: u8 = 1u8;
```

### `IPOPT_TS_PRESPEC`

```rust
const IPOPT_TS_PRESPEC: u8 = 3u8;
```

### `ARPOP_RREQUEST`

```rust
const ARPOP_RREQUEST: u16 = 3u16;
```

### `ARPOP_RREPLY`

```rust
const ARPOP_RREPLY: u16 = 4u16;
```

### `ARPOP_InREQUEST`

```rust
const ARPOP_InREQUEST: u16 = 8u16;
```

### `ARPOP_InREPLY`

```rust
const ARPOP_InREPLY: u16 = 9u16;
```

### `ARPOP_NAK`

```rust
const ARPOP_NAK: u16 = 10u16;
```

### `ATF_NETMASK`

```rust
const ATF_NETMASK: crate::c_int = 32i32;
```

### `ATF_DONTPUB`

```rust
const ATF_DONTPUB: crate::c_int = 64i32;
```

### `ARPHRD_NETROM`

```rust
const ARPHRD_NETROM: u16 = 0u16;
```

### `ARPHRD_ETHER`

```rust
const ARPHRD_ETHER: u16 = 1u16;
```

### `ARPHRD_EETHER`

```rust
const ARPHRD_EETHER: u16 = 2u16;
```

### `ARPHRD_AX25`

```rust
const ARPHRD_AX25: u16 = 3u16;
```

### `ARPHRD_PRONET`

```rust
const ARPHRD_PRONET: u16 = 4u16;
```

### `ARPHRD_CHAOS`

```rust
const ARPHRD_CHAOS: u16 = 5u16;
```

### `ARPHRD_IEEE802`

```rust
const ARPHRD_IEEE802: u16 = 6u16;
```

### `ARPHRD_ARCNET`

```rust
const ARPHRD_ARCNET: u16 = 7u16;
```

### `ARPHRD_APPLETLK`

```rust
const ARPHRD_APPLETLK: u16 = 8u16;
```

### `ARPHRD_DLCI`

```rust
const ARPHRD_DLCI: u16 = 15u16;
```

### `ARPHRD_ATM`

```rust
const ARPHRD_ATM: u16 = 19u16;
```

### `ARPHRD_METRICOM`

```rust
const ARPHRD_METRICOM: u16 = 23u16;
```

### `ARPHRD_IEEE1394`

```rust
const ARPHRD_IEEE1394: u16 = 24u16;
```

### `ARPHRD_EUI64`

```rust
const ARPHRD_EUI64: u16 = 27u16;
```

### `ARPHRD_INFINIBAND`

```rust
const ARPHRD_INFINIBAND: u16 = 32u16;
```

### `ARPHRD_SLIP`

```rust
const ARPHRD_SLIP: u16 = 256u16;
```

### `ARPHRD_CSLIP`

```rust
const ARPHRD_CSLIP: u16 = 257u16;
```

### `ARPHRD_SLIP6`

```rust
const ARPHRD_SLIP6: u16 = 258u16;
```

### `ARPHRD_CSLIP6`

```rust
const ARPHRD_CSLIP6: u16 = 259u16;
```

### `ARPHRD_RSRVD`

```rust
const ARPHRD_RSRVD: u16 = 260u16;
```

### `ARPHRD_ADAPT`

```rust
const ARPHRD_ADAPT: u16 = 264u16;
```

### `ARPHRD_ROSE`

```rust
const ARPHRD_ROSE: u16 = 270u16;
```

### `ARPHRD_X25`

```rust
const ARPHRD_X25: u16 = 271u16;
```

### `ARPHRD_HWX25`

```rust
const ARPHRD_HWX25: u16 = 272u16;
```

### `ARPHRD_CAN`

```rust
const ARPHRD_CAN: u16 = 280u16;
```

### `ARPHRD_PPP`

```rust
const ARPHRD_PPP: u16 = 512u16;
```

### `ARPHRD_CISCO`

```rust
const ARPHRD_CISCO: u16 = 513u16;
```

### `ARPHRD_HDLC`

```rust
const ARPHRD_HDLC: u16 = 513u16;
```

### `ARPHRD_LAPB`

```rust
const ARPHRD_LAPB: u16 = 516u16;
```

### `ARPHRD_DDCMP`

```rust
const ARPHRD_DDCMP: u16 = 517u16;
```

### `ARPHRD_RAWHDLC`

```rust
const ARPHRD_RAWHDLC: u16 = 518u16;
```

### `ARPHRD_TUNNEL`

```rust
const ARPHRD_TUNNEL: u16 = 768u16;
```

### `ARPHRD_TUNNEL6`

```rust
const ARPHRD_TUNNEL6: u16 = 769u16;
```

### `ARPHRD_FRAD`

```rust
const ARPHRD_FRAD: u16 = 770u16;
```

### `ARPHRD_SKIP`

```rust
const ARPHRD_SKIP: u16 = 771u16;
```

### `ARPHRD_LOOPBACK`

```rust
const ARPHRD_LOOPBACK: u16 = 772u16;
```

### `ARPHRD_LOCALTLK`

```rust
const ARPHRD_LOCALTLK: u16 = 773u16;
```

### `ARPHRD_FDDI`

```rust
const ARPHRD_FDDI: u16 = 774u16;
```

### `ARPHRD_BIF`

```rust
const ARPHRD_BIF: u16 = 775u16;
```

### `ARPHRD_SIT`

```rust
const ARPHRD_SIT: u16 = 776u16;
```

### `ARPHRD_IPDDP`

```rust
const ARPHRD_IPDDP: u16 = 777u16;
```

### `ARPHRD_IPGRE`

```rust
const ARPHRD_IPGRE: u16 = 778u16;
```

### `ARPHRD_PIMREG`

```rust
const ARPHRD_PIMREG: u16 = 779u16;
```

### `ARPHRD_HIPPI`

```rust
const ARPHRD_HIPPI: u16 = 780u16;
```

### `ARPHRD_ASH`

```rust
const ARPHRD_ASH: u16 = 781u16;
```

### `ARPHRD_ECONET`

```rust
const ARPHRD_ECONET: u16 = 782u16;
```

### `ARPHRD_IRDA`

```rust
const ARPHRD_IRDA: u16 = 783u16;
```

### `ARPHRD_FCPP`

```rust
const ARPHRD_FCPP: u16 = 784u16;
```

### `ARPHRD_FCAL`

```rust
const ARPHRD_FCAL: u16 = 785u16;
```

### `ARPHRD_FCPL`

```rust
const ARPHRD_FCPL: u16 = 786u16;
```

### `ARPHRD_FCFABRIC`

```rust
const ARPHRD_FCFABRIC: u16 = 787u16;
```

### `ARPHRD_IEEE802_TR`

```rust
const ARPHRD_IEEE802_TR: u16 = 800u16;
```

### `ARPHRD_IEEE80211`

```rust
const ARPHRD_IEEE80211: u16 = 801u16;
```

### `ARPHRD_IEEE80211_PRISM`

```rust
const ARPHRD_IEEE80211_PRISM: u16 = 802u16;
```

### `ARPHRD_IEEE80211_RADIOTAP`

```rust
const ARPHRD_IEEE80211_RADIOTAP: u16 = 803u16;
```

### `ARPHRD_IEEE802154`

```rust
const ARPHRD_IEEE802154: u16 = 804u16;
```

### `ARPHRD_VOID`

```rust
const ARPHRD_VOID: u16 = 65_535u16;
```

### `ARPHRD_NONE`

```rust
const ARPHRD_NONE: u16 = 65_534u16;
```

### `IFF_TUN`

```rust
const IFF_TUN: crate::c_int = 1i32;
```

### `IFF_TAP`

```rust
const IFF_TAP: crate::c_int = 2i32;
```

### `IFF_NAPI`

```rust
const IFF_NAPI: crate::c_int = 16i32;
```

### `IFF_NAPI_FRAGS`

```rust
const IFF_NAPI_FRAGS: crate::c_int = 32i32;
```

### `IFF_NO_CARRIER`

```rust
const IFF_NO_CARRIER: crate::c_int = 64i32;
```

### `IFF_NO_PI`

```rust
const IFF_NO_PI: crate::c_int = 4_096i32;
```

### `TUN_READQ_SIZE`

```rust
const TUN_READQ_SIZE: crate::c_short = 500i16;
```

### `TUN_TUN_DEV`

```rust
const TUN_TUN_DEV: crate::c_short = 1i16;
```

### `TUN_TAP_DEV`

```rust
const TUN_TAP_DEV: crate::c_short = 2i16;
```

### `TUN_TYPE_MASK`

```rust
const TUN_TYPE_MASK: crate::c_short = 15i16;
```

### `IFF_ONE_QUEUE`

```rust
const IFF_ONE_QUEUE: crate::c_int = 8_192i32;
```

### `IFF_VNET_HDR`

```rust
const IFF_VNET_HDR: crate::c_int = 16_384i32;
```

### `IFF_TUN_EXCL`

```rust
const IFF_TUN_EXCL: crate::c_int = 32_768i32;
```

### `IFF_MULTI_QUEUE`

```rust
const IFF_MULTI_QUEUE: crate::c_int = 256i32;
```

### `IFF_ATTACH_QUEUE`

```rust
const IFF_ATTACH_QUEUE: crate::c_int = 512i32;
```

### `IFF_DETACH_QUEUE`

```rust
const IFF_DETACH_QUEUE: crate::c_int = 1_024i32;
```

### `IFF_PERSIST`

```rust
const IFF_PERSIST: crate::c_int = 2_048i32;
```

### `IFF_NOFILTER`

```rust
const IFF_NOFILTER: crate::c_int = 4_096i32;
```

### `TUN_TX_TIMESTAMP`

```rust
const TUN_TX_TIMESTAMP: crate::c_int = 1i32;
```

### `TUN_F_CSUM`

```rust
const TUN_F_CSUM: crate::c_uint = 1u32;
```

### `TUN_F_TSO4`

```rust
const TUN_F_TSO4: crate::c_uint = 2u32;
```

### `TUN_F_TSO6`

```rust
const TUN_F_TSO6: crate::c_uint = 4u32;
```

### `TUN_F_TSO_ECN`

```rust
const TUN_F_TSO_ECN: crate::c_uint = 8u32;
```

### `TUN_F_UFO`

```rust
const TUN_F_UFO: crate::c_uint = 16u32;
```

### `TUN_F_USO4`

```rust
const TUN_F_USO4: crate::c_uint = 32u32;
```

### `TUN_F_USO6`

```rust
const TUN_F_USO6: crate::c_uint = 64u32;
```

### `TUN_PKT_STRIP`

```rust
const TUN_PKT_STRIP: crate::c_int = 1i32;
```

### `TUN_FLT_ALLMULTI`

```rust
const TUN_FLT_ALLMULTI: crate::c_int = 1i32;
```

### `T_TYPE`

```rust
const T_TYPE: u32 = 84u32;
```

### `TUNSETNOCSUM`

```rust
const TUNSETNOCSUM: crate::c_ulong = 1_074_025_672u64;
```

### `TUNSETDEBUG`

```rust
const TUNSETDEBUG: crate::c_ulong = 1_074_025_673u64;
```

### `TUNSETIFF`

```rust
const TUNSETIFF: crate::c_ulong = 1_074_025_674u64;
```

### `TUNSETPERSIST`

```rust
const TUNSETPERSIST: crate::c_ulong = 1_074_025_675u64;
```

### `TUNSETOWNER`

```rust
const TUNSETOWNER: crate::c_ulong = 1_074_025_676u64;
```

### `TUNSETLINK`

```rust
const TUNSETLINK: crate::c_ulong = 1_074_025_677u64;
```

### `TUNSETGROUP`

```rust
const TUNSETGROUP: crate::c_ulong = 1_074_025_678u64;
```

### `TUNGETFEATURES`

```rust
const TUNGETFEATURES: crate::c_ulong = 2_147_767_503u64;
```

### `TUNSETOFFLOAD`

```rust
const TUNSETOFFLOAD: crate::c_ulong = 1_074_025_680u64;
```

### `TUNSETTXFILTER`

```rust
const TUNSETTXFILTER: crate::c_ulong = 1_074_025_681u64;
```

### `TUNGETIFF`

```rust
const TUNGETIFF: crate::c_ulong = 2_147_767_506u64;
```

### `TUNGETSNDBUF`

```rust
const TUNGETSNDBUF: crate::c_ulong = 2_147_767_507u64;
```

### `TUNSETSNDBUF`

```rust
const TUNSETSNDBUF: crate::c_ulong = 1_074_025_684u64;
```

### `TUNATTACHFILTER`

```rust
const TUNATTACHFILTER: crate::c_ulong = 1_074_812_117u64;
```

### `TUNDETACHFILTER`

```rust
const TUNDETACHFILTER: crate::c_ulong = 1_074_812_118u64;
```

### `TUNGETVNETHDRSZ`

```rust
const TUNGETVNETHDRSZ: crate::c_ulong = 2_147_767_511u64;
```

### `TUNSETVNETHDRSZ`

```rust
const TUNSETVNETHDRSZ: crate::c_ulong = 1_074_025_688u64;
```

### `TUNSETQUEUE`

```rust
const TUNSETQUEUE: crate::c_ulong = 1_074_025_689u64;
```

### `TUNSETIFINDEX`

```rust
const TUNSETIFINDEX: crate::c_ulong = 1_074_025_690u64;
```

### `TUNGETFILTER`

```rust
const TUNGETFILTER: crate::c_ulong = 2_148_553_947u64;
```

### `TUNSETVNETLE`

```rust
const TUNSETVNETLE: crate::c_ulong = 1_074_025_692u64;
```

### `TUNGETVNETLE`

```rust
const TUNGETVNETLE: crate::c_ulong = 2_147_767_517u64;
```

### `TUNSETVNETBE`

```rust
const TUNSETVNETBE: crate::c_ulong = 1_074_025_694u64;
```

### `TUNGETVNETBE`

```rust
const TUNGETVNETBE: crate::c_ulong = 2_147_767_519u64;
```

### `TUNSETSTEERINGEBPF`

```rust
const TUNSETSTEERINGEBPF: crate::c_ulong = 2_147_767_520u64;
```

### `TUNSETFILTEREBPF`

```rust
const TUNSETFILTEREBPF: crate::c_ulong = 2_147_767_521u64;
```

### `TUNSETCARRIER`

```rust
const TUNSETCARRIER: crate::c_ulong = 1_074_025_698u64;
```

### `TUNGETDEVNETNS`

```rust
const TUNGETDEVNETNS: crate::c_ulong = 21_731u64;
```

### `FS_IOC_GETFLAGS`

```rust
const FS_IOC_GETFLAGS: crate::c_ulong = 2_148_034_049u64;
```

### `FS_IOC_SETFLAGS`

```rust
const FS_IOC_SETFLAGS: crate::c_ulong = 1_074_292_226u64;
```

### `FS_IOC_GETVERSION`

```rust
const FS_IOC_GETVERSION: crate::c_ulong = 2_148_038_145u64;
```

### `FS_IOC_SETVERSION`

```rust
const FS_IOC_SETVERSION: crate::c_ulong = 1_074_296_322u64;
```

### `FS_IOC32_GETFLAGS`

```rust
const FS_IOC32_GETFLAGS: crate::c_ulong = 2_147_771_905u64;
```

### `FS_IOC32_SETFLAGS`

```rust
const FS_IOC32_SETFLAGS: crate::c_ulong = 1_074_030_082u64;
```

### `FS_IOC32_GETVERSION`

```rust
const FS_IOC32_GETVERSION: crate::c_ulong = 2_147_776_001u64;
```

### `FS_IOC32_SETVERSION`

```rust
const FS_IOC32_SETVERSION: crate::c_ulong = 1_074_034_178u64;
```

### `FICLONE`

```rust
const FICLONE: crate::c_ulong = 1_074_041_865u64;
```

### `FICLONERANGE`

```rust
const FICLONERANGE: crate::c_ulong = 1_075_876_877u64;
```

### `ADFS_SUPER_MAGIC`

```rust
const ADFS_SUPER_MAGIC: crate::c_long = 44_533i64;
```

### `AFFS_SUPER_MAGIC`

```rust
const AFFS_SUPER_MAGIC: crate::c_long = 44_543i64;
```

### `AFS_SUPER_MAGIC`

```rust
const AFS_SUPER_MAGIC: crate::c_long = 1_397_113_167i64;
```

### `AUTOFS_SUPER_MAGIC`

```rust
const AUTOFS_SUPER_MAGIC: crate::c_long = 391i64;
```

### `BPF_FS_MAGIC`

```rust
const BPF_FS_MAGIC: crate::c_long = 3_405_662_737i64;
```

### `BTRFS_SUPER_MAGIC`

```rust
const BTRFS_SUPER_MAGIC: crate::c_long = 2_435_016_766i64;
```

### `CGROUP2_SUPER_MAGIC`

```rust
const CGROUP2_SUPER_MAGIC: crate::c_long = 1_667_723_888i64;
```

### `CGROUP_SUPER_MAGIC`

```rust
const CGROUP_SUPER_MAGIC: crate::c_long = 2_613_483i64;
```

### `CODA_SUPER_MAGIC`

```rust
const CODA_SUPER_MAGIC: crate::c_long = 1_937_076_805i64;
```

### `CRAMFS_MAGIC`

```rust
const CRAMFS_MAGIC: crate::c_long = 684_539_205i64;
```

### `DEBUGFS_MAGIC`

```rust
const DEBUGFS_MAGIC: crate::c_long = 1_684_170_528i64;
```

### `DEVPTS_SUPER_MAGIC`

```rust
const DEVPTS_SUPER_MAGIC: crate::c_long = 7_377i64;
```

### `ECRYPTFS_SUPER_MAGIC`

```rust
const ECRYPTFS_SUPER_MAGIC: crate::c_long = 61_791i64;
```

### `EFS_SUPER_MAGIC`

```rust
const EFS_SUPER_MAGIC: crate::c_long = 4_278_867i64;
```

### `EXT2_SUPER_MAGIC`

```rust
const EXT2_SUPER_MAGIC: crate::c_long = 61_267i64;
```

### `EXT3_SUPER_MAGIC`

```rust
const EXT3_SUPER_MAGIC: crate::c_long = 61_267i64;
```

### `EXT4_SUPER_MAGIC`

```rust
const EXT4_SUPER_MAGIC: crate::c_long = 61_267i64;
```

### `F2FS_SUPER_MAGIC`

```rust
const F2FS_SUPER_MAGIC: crate::c_long = 4_076_150_800i64;
```

### `FUSE_SUPER_MAGIC`

```rust
const FUSE_SUPER_MAGIC: crate::c_long = 1_702_057_286i64;
```

### `FUTEXFS_SUPER_MAGIC`

```rust
const FUTEXFS_SUPER_MAGIC: crate::c_long = 195_894_762i64;
```

### `HOSTFS_SUPER_MAGIC`

```rust
const HOSTFS_SUPER_MAGIC: crate::c_long = 12_648_430i64;
```

### `HPFS_SUPER_MAGIC`

```rust
const HPFS_SUPER_MAGIC: crate::c_long = 4_187_351_113i64;
```

### `HUGETLBFS_MAGIC`

```rust
const HUGETLBFS_MAGIC: crate::c_long = 2_508_478_710i64;
```

### `ISOFS_SUPER_MAGIC`

```rust
const ISOFS_SUPER_MAGIC: crate::c_long = 38_496i64;
```

### `JFFS2_SUPER_MAGIC`

```rust
const JFFS2_SUPER_MAGIC: crate::c_long = 29_366i64;
```

### `MINIX2_SUPER_MAGIC2`

```rust
const MINIX2_SUPER_MAGIC2: crate::c_long = 9_336i64;
```

### `MINIX2_SUPER_MAGIC`

```rust
const MINIX2_SUPER_MAGIC: crate::c_long = 9_320i64;
```

### `MINIX3_SUPER_MAGIC`

```rust
const MINIX3_SUPER_MAGIC: crate::c_long = 19_802i64;
```

### `MINIX_SUPER_MAGIC2`

```rust
const MINIX_SUPER_MAGIC2: crate::c_long = 5_007i64;
```

### `MINIX_SUPER_MAGIC`

```rust
const MINIX_SUPER_MAGIC: crate::c_long = 4_991i64;
```

### `MSDOS_SUPER_MAGIC`

```rust
const MSDOS_SUPER_MAGIC: crate::c_long = 19_780i64;
```

### `NCP_SUPER_MAGIC`

```rust
const NCP_SUPER_MAGIC: crate::c_long = 22_092i64;
```

### `NFS_SUPER_MAGIC`

```rust
const NFS_SUPER_MAGIC: crate::c_long = 26_985i64;
```

### `NILFS_SUPER_MAGIC`

```rust
const NILFS_SUPER_MAGIC: crate::c_long = 13_364i64;
```

### `OCFS2_SUPER_MAGIC`

```rust
const OCFS2_SUPER_MAGIC: crate::c_long = 1_952_539_503i64;
```

### `OPENPROM_SUPER_MAGIC`

```rust
const OPENPROM_SUPER_MAGIC: crate::c_long = 40_865i64;
```

### `OVERLAYFS_SUPER_MAGIC`

```rust
const OVERLAYFS_SUPER_MAGIC: crate::c_long = 2_035_054_128i64;
```

### `PROC_SUPER_MAGIC`

```rust
const PROC_SUPER_MAGIC: crate::c_long = 40_864i64;
```

### `QNX4_SUPER_MAGIC`

```rust
const QNX4_SUPER_MAGIC: crate::c_long = 47i64;
```

### `QNX6_SUPER_MAGIC`

```rust
const QNX6_SUPER_MAGIC: crate::c_long = 1_746_473_250i64;
```

### `RDTGROUP_SUPER_MAGIC`

```rust
const RDTGROUP_SUPER_MAGIC: crate::c_long = 124_082_209i64;
```

### `REISERFS_SUPER_MAGIC`

```rust
const REISERFS_SUPER_MAGIC: crate::c_long = 1_382_369_651i64;
```

### `SECURITYFS_MAGIC`

```rust
const SECURITYFS_MAGIC: crate::c_long = 1_935_894_131i64;
```

### `SELINUX_MAGIC`

```rust
const SELINUX_MAGIC: crate::c_long = 4_185_718_668i64;
```

### `SMACK_MAGIC`

```rust
const SMACK_MAGIC: crate::c_long = 1_128_357_203i64;
```

### `SMB_SUPER_MAGIC`

```rust
const SMB_SUPER_MAGIC: crate::c_long = 20_859i64;
```

### `SYSFS_MAGIC`

```rust
const SYSFS_MAGIC: crate::c_long = 1_650_812_274i64;
```

### `TMPFS_MAGIC`

```rust
const TMPFS_MAGIC: crate::c_long = 16_914_836i64;
```

### `TRACEFS_MAGIC`

```rust
const TRACEFS_MAGIC: crate::c_long = 1_953_653_091i64;
```

### `UDF_SUPER_MAGIC`

```rust
const UDF_SUPER_MAGIC: crate::c_long = 352_400_198i64;
```

### `USBDEVICE_SUPER_MAGIC`

```rust
const USBDEVICE_SUPER_MAGIC: crate::c_long = 40_866i64;
```

### `XENFS_SUPER_MAGIC`

```rust
const XENFS_SUPER_MAGIC: crate::c_long = 2_881_100_148i64;
```

### `NSFS_MAGIC`

```rust
const NSFS_MAGIC: crate::c_long = 1_853_056_627i64;
```

### `AT_STATX_SYNC_TYPE`

```rust
const AT_STATX_SYNC_TYPE: crate::c_int = 24_576i32;
```

### `AT_STATX_SYNC_AS_STAT`

```rust
const AT_STATX_SYNC_AS_STAT: crate::c_int = 0i32;
```

### `AT_STATX_FORCE_SYNC`

```rust
const AT_STATX_FORCE_SYNC: crate::c_int = 8_192i32;
```

### `AT_STATX_DONT_SYNC`

```rust
const AT_STATX_DONT_SYNC: crate::c_int = 16_384i32;
```

### `STATX_TYPE`

```rust
const STATX_TYPE: crate::c_uint = 1u32;
```

### `STATX_MODE`

```rust
const STATX_MODE: crate::c_uint = 2u32;
```

### `STATX_NLINK`

```rust
const STATX_NLINK: crate::c_uint = 4u32;
```

### `STATX_UID`

```rust
const STATX_UID: crate::c_uint = 8u32;
```

### `STATX_GID`

```rust
const STATX_GID: crate::c_uint = 16u32;
```

### `STATX_ATIME`

```rust
const STATX_ATIME: crate::c_uint = 32u32;
```

### `STATX_MTIME`

```rust
const STATX_MTIME: crate::c_uint = 64u32;
```

### `STATX_CTIME`

```rust
const STATX_CTIME: crate::c_uint = 128u32;
```

### `STATX_INO`

```rust
const STATX_INO: crate::c_uint = 256u32;
```

### `STATX_SIZE`

```rust
const STATX_SIZE: crate::c_uint = 512u32;
```

### `STATX_BLOCKS`

```rust
const STATX_BLOCKS: crate::c_uint = 1_024u32;
```

### `STATX_BASIC_STATS`

```rust
const STATX_BASIC_STATS: crate::c_uint = 2_047u32;
```

### `STATX_BTIME`

```rust
const STATX_BTIME: crate::c_uint = 2_048u32;
```

### `STATX_ALL`

```rust
const STATX_ALL: crate::c_uint = 4_095u32;
```

### `STATX_MNT_ID`

```rust
const STATX_MNT_ID: crate::c_uint = 4_096u32;
```

### `STATX_DIOALIGN`

```rust
const STATX_DIOALIGN: crate::c_uint = 8_192u32;
```

### `STATX__RESERVED`

```rust
const STATX__RESERVED: crate::c_int = -2_147_483_648i32;
```

### `STATX_ATTR_COMPRESSED`

```rust
const STATX_ATTR_COMPRESSED: crate::c_int = 4i32;
```

### `STATX_ATTR_IMMUTABLE`

```rust
const STATX_ATTR_IMMUTABLE: crate::c_int = 16i32;
```

### `STATX_ATTR_APPEND`

```rust
const STATX_ATTR_APPEND: crate::c_int = 32i32;
```

### `STATX_ATTR_NODUMP`

```rust
const STATX_ATTR_NODUMP: crate::c_int = 64i32;
```

### `STATX_ATTR_ENCRYPTED`

```rust
const STATX_ATTR_ENCRYPTED: crate::c_int = 2_048i32;
```

### `STATX_ATTR_AUTOMOUNT`

```rust
const STATX_ATTR_AUTOMOUNT: crate::c_int = 4_096i32;
```

### `STATX_ATTR_MOUNT_ROOT`

```rust
const STATX_ATTR_MOUNT_ROOT: crate::c_int = 8_192i32;
```

### `STATX_ATTR_VERITY`

```rust
const STATX_ATTR_VERITY: crate::c_int = 1_048_576i32;
```

### `STATX_ATTR_DAX`

```rust
const STATX_ATTR_DAX: crate::c_int = 2_097_152i32;
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

