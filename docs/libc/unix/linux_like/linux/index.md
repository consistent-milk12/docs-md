*[libc](../../../index.md) / [unix](../../index.md) / [linux_like](../index.md) / [linux](index.md)*

---

# Module `linux`

Linux-specific definitions for linux-like values

## Modules

- [`gnu`](gnu/index.md) - 
- [`arch`](arch/index.md) - 
- [`b64`](b64/index.md) - 64-bit specific definitions for linux-like values
- [`generic`](generic/index.md) - 

## Structs

### `glob_t`

```rust
struct glob_t {
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

##### `impl Clone for glob_t`

- `fn clone(self: &Self) -> glob_t` — [`glob_t`](../index.md)

##### `impl Copy for glob_t`

##### `impl Debug for glob_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `passwd`

```rust
struct passwd {
    pub pw_name: *mut crate::c_char,
    pub pw_passwd: *mut crate::c_char,
    pub pw_uid: crate::uid_t,
    pub pw_gid: crate::gid_t,
    pub pw_gecos: *mut crate::c_char,
    pub pw_dir: *mut crate::c_char,
    pub pw_shell: *mut crate::c_char,
}
```

#### Trait Implementations

##### `impl Clone for passwd`

- `fn clone(self: &Self) -> passwd` — [`passwd`](../index.md)

##### `impl Copy for passwd`

##### `impl Debug for passwd`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `spwd`

```rust
struct spwd {
    pub sp_namp: *mut crate::c_char,
    pub sp_pwdp: *mut crate::c_char,
    pub sp_lstchg: crate::c_long,
    pub sp_min: crate::c_long,
    pub sp_max: crate::c_long,
    pub sp_warn: crate::c_long,
    pub sp_inact: crate::c_long,
    pub sp_expire: crate::c_long,
    pub sp_flag: crate::c_ulong,
}
```

#### Trait Implementations

##### `impl Clone for spwd`

- `fn clone(self: &Self) -> spwd` — [`spwd`](../index.md)

##### `impl Copy for spwd`

##### `impl Debug for spwd`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `dqblk`

```rust
struct dqblk {
    pub dqb_bhardlimit: u64,
    pub dqb_bsoftlimit: u64,
    pub dqb_curspace: u64,
    pub dqb_ihardlimit: u64,
    pub dqb_isoftlimit: u64,
    pub dqb_curinodes: u64,
    pub dqb_btime: u64,
    pub dqb_itime: u64,
    pub dqb_valid: u32,
}
```

#### Trait Implementations

##### `impl Clone for dqblk`

- `fn clone(self: &Self) -> dqblk` — [`dqblk`](../index.md)

##### `impl Copy for dqblk`

##### `impl Debug for dqblk`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `signalfd_siginfo`

```rust
struct signalfd_siginfo {
    pub ssi_signo: u32,
    pub ssi_errno: i32,
    pub ssi_code: i32,
    pub ssi_pid: u32,
    pub ssi_uid: u32,
    pub ssi_fd: i32,
    pub ssi_tid: u32,
    pub ssi_band: u32,
    pub ssi_overrun: u32,
    pub ssi_trapno: u32,
    pub ssi_status: i32,
    pub ssi_int: i32,
    pub ssi_ptr: u64,
    pub ssi_utime: u64,
    pub ssi_stime: u64,
    pub ssi_addr: u64,
    pub ssi_addr_lsb: u16,
    _pad2: crate::types::Padding<u16>,
    pub ssi_syscall: i32,
    pub ssi_call_addr: u64,
    pub ssi_arch: u32,
    _pad: crate::types::Padding<[u8; 28]>,
}
```

#### Trait Implementations

##### `impl Clone for signalfd_siginfo`

- `fn clone(self: &Self) -> signalfd_siginfo` — [`signalfd_siginfo`](../index.md)

##### `impl Copy for signalfd_siginfo`

##### `impl Debug for signalfd_siginfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `itimerspec`

```rust
struct itimerspec {
    pub it_interval: crate::timespec,
    pub it_value: crate::timespec,
}
```

#### Trait Implementations

##### `impl Clone for itimerspec`

- `fn clone(self: &Self) -> itimerspec` — [`itimerspec`](../index.md)

##### `impl Copy for itimerspec`

##### `impl Debug for itimerspec`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fsid_t`

```rust
struct fsid_t {
    __val: [crate::c_int; 2],
}
```

#### Trait Implementations

##### `impl Clone for fsid_t`

- `fn clone(self: &Self) -> fsid_t` — [`fsid_t`](../index.md)

##### `impl Copy for fsid_t`

##### `impl Debug for fsid_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fanout_args`

```rust
struct fanout_args {
    pub id: __u16,
    pub type_flags: __u16,
    pub max_num_members: __u32,
}
```

#### Trait Implementations

##### `impl Clone for fanout_args`

- `fn clone(self: &Self) -> fanout_args` — [`fanout_args`](../index.md)

##### `impl Copy for fanout_args`

##### `impl Debug for fanout_args`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `packet_mreq`

```rust
struct packet_mreq {
    pub mr_ifindex: crate::c_int,
    pub mr_type: crate::c_ushort,
    pub mr_alen: crate::c_ushort,
    pub mr_address: [crate::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for packet_mreq`

- `fn clone(self: &Self) -> packet_mreq` — [`packet_mreq`](../index.md)

##### `impl Copy for packet_mreq`

##### `impl Debug for packet_mreq`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr_pkt`

```rust
struct sockaddr_pkt {
    pub spkt_family: crate::c_ushort,
    pub spkt_device: [crate::c_uchar; 14],
    pub spkt_protocol: crate::c_ushort,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_pkt`

- `fn clone(self: &Self) -> sockaddr_pkt` — [`sockaddr_pkt`](../index.md)

##### `impl Copy for sockaddr_pkt`

##### `impl Debug for sockaddr_pkt`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket_auxdata`

```rust
struct tpacket_auxdata {
    pub tp_status: __u32,
    pub tp_len: __u32,
    pub tp_snaplen: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
    pub tp_vlan_tci: __u16,
    pub tp_vlan_tpid: __u16,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_auxdata`

- `fn clone(self: &Self) -> tpacket_auxdata` — [`tpacket_auxdata`](../index.md)

##### `impl Copy for tpacket_auxdata`

##### `impl Debug for tpacket_auxdata`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket_hdr`

```rust
struct tpacket_hdr {
    pub tp_status: crate::c_ulong,
    pub tp_len: crate::c_uint,
    pub tp_snaplen: crate::c_uint,
    pub tp_mac: crate::c_ushort,
    pub tp_net: crate::c_ushort,
    pub tp_sec: crate::c_uint,
    pub tp_usec: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_hdr`

- `fn clone(self: &Self) -> tpacket_hdr` — [`tpacket_hdr`](../index.md)

##### `impl Copy for tpacket_hdr`

##### `impl Debug for tpacket_hdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket_hdr_variant1`

```rust
struct tpacket_hdr_variant1 {
    pub tp_rxhash: __u32,
    pub tp_vlan_tci: __u32,
    pub tp_vlan_tpid: __u16,
    pub tp_padding: __u16,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_hdr_variant1`

- `fn clone(self: &Self) -> tpacket_hdr_variant1` — [`tpacket_hdr_variant1`](../index.md)

##### `impl Copy for tpacket_hdr_variant1`

##### `impl Debug for tpacket_hdr_variant1`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket2_hdr`

```rust
struct tpacket2_hdr {
    pub tp_status: __u32,
    pub tp_len: __u32,
    pub tp_snaplen: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
    pub tp_sec: __u32,
    pub tp_nsec: __u32,
    pub tp_vlan_tci: __u16,
    pub tp_vlan_tpid: __u16,
    pub tp_padding: [__u8; 4],
}
```

#### Trait Implementations

##### `impl Clone for tpacket2_hdr`

- `fn clone(self: &Self) -> tpacket2_hdr` — [`tpacket2_hdr`](../index.md)

##### `impl Copy for tpacket2_hdr`

##### `impl Debug for tpacket2_hdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket_req`

```rust
struct tpacket_req {
    pub tp_block_size: crate::c_uint,
    pub tp_block_nr: crate::c_uint,
    pub tp_frame_size: crate::c_uint,
    pub tp_frame_nr: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_req`

- `fn clone(self: &Self) -> tpacket_req` — [`tpacket_req`](../index.md)

##### `impl Copy for tpacket_req`

##### `impl Debug for tpacket_req`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket_req3`

```rust
struct tpacket_req3 {
    pub tp_block_size: crate::c_uint,
    pub tp_block_nr: crate::c_uint,
    pub tp_frame_size: crate::c_uint,
    pub tp_frame_nr: crate::c_uint,
    pub tp_retire_blk_tov: crate::c_uint,
    pub tp_sizeof_priv: crate::c_uint,
    pub tp_feature_req_word: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_req3`

- `fn clone(self: &Self) -> tpacket_req3` — [`tpacket_req3`](../index.md)

##### `impl Copy for tpacket_req3`

##### `impl Debug for tpacket_req3`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket_rollover_stats`

```rust
struct tpacket_rollover_stats {
    pub tp_all: crate::__u64,
    pub tp_huge: crate::__u64,
    pub tp_failed: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_rollover_stats`

- `fn clone(self: &Self) -> tpacket_rollover_stats` — [`tpacket_rollover_stats`](../index.md)

##### `impl Copy for tpacket_rollover_stats`

##### `impl Debug for tpacket_rollover_stats`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket_stats`

```rust
struct tpacket_stats {
    pub tp_packets: crate::c_uint,
    pub tp_drops: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_stats`

- `fn clone(self: &Self) -> tpacket_stats` — [`tpacket_stats`](../index.md)

##### `impl Copy for tpacket_stats`

##### `impl Debug for tpacket_stats`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket_stats_v3`

```rust
struct tpacket_stats_v3 {
    pub tp_packets: crate::c_uint,
    pub tp_drops: crate::c_uint,
    pub tp_freeze_q_cnt: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_stats_v3`

- `fn clone(self: &Self) -> tpacket_stats_v3` — [`tpacket_stats_v3`](../index.md)

##### `impl Copy for tpacket_stats_v3`

##### `impl Debug for tpacket_stats_v3`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket3_hdr`

```rust
struct tpacket3_hdr {
    pub tp_next_offset: __u32,
    pub tp_sec: __u32,
    pub tp_nsec: __u32,
    pub tp_snaplen: __u32,
    pub tp_len: __u32,
    pub tp_status: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
    pub hv1: crate::tpacket_hdr_variant1,
    pub tp_padding: [__u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for tpacket3_hdr`

- `fn clone(self: &Self) -> tpacket3_hdr` — [`tpacket3_hdr`](../index.md)

##### `impl Copy for tpacket3_hdr`

##### `impl Debug for tpacket3_hdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket_bd_ts`

```rust
struct tpacket_bd_ts {
    pub ts_sec: crate::c_uint,
    pub ts_usec: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_bd_ts`

- `fn clone(self: &Self) -> tpacket_bd_ts` — [`tpacket_bd_ts`](../index.md)

##### `impl Copy for tpacket_bd_ts`

##### `impl Debug for tpacket_bd_ts`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket_hdr_v1`

```rust
struct tpacket_hdr_v1 {
    pub block_status: __u32,
    pub num_pkts: __u32,
    pub offset_to_first_pkt: __u32,
    pub blk_len: __u32,
    pub seq_num: crate::__u64,
    pub ts_first_pkt: crate::tpacket_bd_ts,
    pub ts_last_pkt: crate::tpacket_bd_ts,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_hdr_v1`

- `fn clone(self: &Self) -> tpacket_hdr_v1` — [`tpacket_hdr_v1`](../index.md)

##### `impl Copy for tpacket_hdr_v1`

##### `impl Debug for tpacket_hdr_v1`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `cpu_set_t`

```rust
struct cpu_set_t {
    bits: [u64; 16],
}
```

#### Trait Implementations

##### `impl Clone for cpu_set_t`

- `fn clone(self: &Self) -> cpu_set_t` — [`cpu_set_t`](../index.md)

##### `impl Copy for cpu_set_t`

##### `impl Debug for cpu_set_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `if_nameindex`

```rust
struct if_nameindex {
    pub if_index: crate::c_uint,
    pub if_name: *mut crate::c_char,
}
```

#### Trait Implementations

##### `impl Clone for if_nameindex`

- `fn clone(self: &Self) -> if_nameindex` — [`if_nameindex`](../index.md)

##### `impl Copy for if_nameindex`

##### `impl Debug for if_nameindex`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `msginfo`

```rust
struct msginfo {
    pub msgpool: crate::c_int,
    pub msgmap: crate::c_int,
    pub msgmax: crate::c_int,
    pub msgmnb: crate::c_int,
    pub msgmni: crate::c_int,
    pub msgssz: crate::c_int,
    pub msgtql: crate::c_int,
    pub msgseg: crate::c_ushort,
}
```

#### Trait Implementations

##### `impl Clone for msginfo`

- `fn clone(self: &Self) -> msginfo` — [`msginfo`](../index.md)

##### `impl Copy for msginfo`

##### `impl Debug for msginfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sembuf`

```rust
struct sembuf {
    pub sem_num: crate::c_ushort,
    pub sem_op: crate::c_short,
    pub sem_flg: crate::c_short,
}
```

#### Trait Implementations

##### `impl Clone for sembuf`

- `fn clone(self: &Self) -> sembuf` — [`sembuf`](../index.md)

##### `impl Copy for sembuf`

##### `impl Debug for sembuf`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `input_event`

```rust
struct input_event {
    pub time: crate::timeval,
    pub type_: __u16,
    pub code: __u16,
    pub value: __s32,
}
```

#### Trait Implementations

##### `impl Clone for input_event`

- `fn clone(self: &Self) -> input_event` — [`input_event`](../index.md)

##### `impl Copy for input_event`

##### `impl Debug for input_event`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `input_id`

```rust
struct input_id {
    pub bustype: __u16,
    pub vendor: __u16,
    pub product: __u16,
    pub version: __u16,
}
```

#### Trait Implementations

##### `impl Clone for input_id`

- `fn clone(self: &Self) -> input_id` — [`input_id`](../index.md)

##### `impl Copy for input_id`

##### `impl Debug for input_id`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `input_absinfo`

```rust
struct input_absinfo {
    pub value: __s32,
    pub minimum: __s32,
    pub maximum: __s32,
    pub fuzz: __s32,
    pub flat: __s32,
    pub resolution: __s32,
}
```

#### Trait Implementations

##### `impl Clone for input_absinfo`

- `fn clone(self: &Self) -> input_absinfo` — [`input_absinfo`](../index.md)

##### `impl Copy for input_absinfo`

##### `impl Debug for input_absinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `input_keymap_entry`

```rust
struct input_keymap_entry {
    pub flags: __u8,
    pub len: __u8,
    pub index: __u16,
    pub keycode: __u32,
    pub scancode: [__u8; 32],
}
```

#### Trait Implementations

##### `impl Clone for input_keymap_entry`

- `fn clone(self: &Self) -> input_keymap_entry` — [`input_keymap_entry`](../index.md)

##### `impl Copy for input_keymap_entry`

##### `impl Debug for input_keymap_entry`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `input_mask`

```rust
struct input_mask {
    pub type_: __u32,
    pub codes_size: __u32,
    pub codes_ptr: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for input_mask`

- `fn clone(self: &Self) -> input_mask` — [`input_mask`](../index.md)

##### `impl Copy for input_mask`

##### `impl Debug for input_mask`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ff_replay`

```rust
struct ff_replay {
    pub length: __u16,
    pub delay: __u16,
}
```

#### Trait Implementations

##### `impl Clone for ff_replay`

- `fn clone(self: &Self) -> ff_replay` — [`ff_replay`](../index.md)

##### `impl Copy for ff_replay`

##### `impl Debug for ff_replay`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ff_trigger`

```rust
struct ff_trigger {
    pub button: __u16,
    pub interval: __u16,
}
```

#### Trait Implementations

##### `impl Clone for ff_trigger`

- `fn clone(self: &Self) -> ff_trigger` — [`ff_trigger`](../index.md)

##### `impl Copy for ff_trigger`

##### `impl Debug for ff_trigger`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ff_envelope`

```rust
struct ff_envelope {
    pub attack_length: __u16,
    pub attack_level: __u16,
    pub fade_length: __u16,
    pub fade_level: __u16,
}
```

#### Trait Implementations

##### `impl Clone for ff_envelope`

- `fn clone(self: &Self) -> ff_envelope` — [`ff_envelope`](../index.md)

##### `impl Copy for ff_envelope`

##### `impl Debug for ff_envelope`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ff_constant_effect`

```rust
struct ff_constant_effect {
    pub level: __s16,
    pub envelope: ff_envelope,
}
```

#### Trait Implementations

##### `impl Clone for ff_constant_effect`

- `fn clone(self: &Self) -> ff_constant_effect` — [`ff_constant_effect`](../index.md)

##### `impl Copy for ff_constant_effect`

##### `impl Debug for ff_constant_effect`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ff_ramp_effect`

```rust
struct ff_ramp_effect {
    pub start_level: __s16,
    pub end_level: __s16,
    pub envelope: ff_envelope,
}
```

#### Trait Implementations

##### `impl Clone for ff_ramp_effect`

- `fn clone(self: &Self) -> ff_ramp_effect` — [`ff_ramp_effect`](../index.md)

##### `impl Copy for ff_ramp_effect`

##### `impl Debug for ff_ramp_effect`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ff_condition_effect`

```rust
struct ff_condition_effect {
    pub right_saturation: __u16,
    pub left_saturation: __u16,
    pub right_coeff: __s16,
    pub left_coeff: __s16,
    pub deadband: __u16,
    pub center: __s16,
}
```

#### Trait Implementations

##### `impl Clone for ff_condition_effect`

- `fn clone(self: &Self) -> ff_condition_effect` — [`ff_condition_effect`](../index.md)

##### `impl Copy for ff_condition_effect`

##### `impl Debug for ff_condition_effect`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ff_periodic_effect`

```rust
struct ff_periodic_effect {
    pub waveform: __u16,
    pub period: __u16,
    pub magnitude: __s16,
    pub offset: __s16,
    pub phase: __u16,
    pub envelope: ff_envelope,
    pub custom_len: __u32,
    pub custom_data: *mut __s16,
}
```

#### Trait Implementations

##### `impl Clone for ff_periodic_effect`

- `fn clone(self: &Self) -> ff_periodic_effect` — [`ff_periodic_effect`](../index.md)

##### `impl Copy for ff_periodic_effect`

##### `impl Debug for ff_periodic_effect`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ff_rumble_effect`

```rust
struct ff_rumble_effect {
    pub strong_magnitude: __u16,
    pub weak_magnitude: __u16,
}
```

#### Trait Implementations

##### `impl Clone for ff_rumble_effect`

- `fn clone(self: &Self) -> ff_rumble_effect` — [`ff_rumble_effect`](../index.md)

##### `impl Copy for ff_rumble_effect`

##### `impl Debug for ff_rumble_effect`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ff_effect`

```rust
struct ff_effect {
    pub type_: __u16,
    pub id: __s16,
    pub direction: __u16,
    pub trigger: ff_trigger,
    pub replay: ff_replay,
    pub u: [u64; 4],
}
```

#### Trait Implementations

##### `impl Clone for ff_effect`

- `fn clone(self: &Self) -> ff_effect` — [`ff_effect`](../index.md)

##### `impl Copy for ff_effect`

##### `impl Debug for ff_effect`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uinput_ff_upload`

```rust
struct uinput_ff_upload {
    pub request_id: __u32,
    pub retval: __s32,
    pub effect: ff_effect,
    pub old: ff_effect,
}
```

#### Trait Implementations

##### `impl Clone for uinput_ff_upload`

- `fn clone(self: &Self) -> uinput_ff_upload` — [`uinput_ff_upload`](../index.md)

##### `impl Copy for uinput_ff_upload`

##### `impl Debug for uinput_ff_upload`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uinput_ff_erase`

```rust
struct uinput_ff_erase {
    pub request_id: __u32,
    pub retval: __s32,
    pub effect_id: __u32,
}
```

#### Trait Implementations

##### `impl Clone for uinput_ff_erase`

- `fn clone(self: &Self) -> uinput_ff_erase` — [`uinput_ff_erase`](../index.md)

##### `impl Copy for uinput_ff_erase`

##### `impl Debug for uinput_ff_erase`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uinput_abs_setup`

```rust
struct uinput_abs_setup {
    pub code: __u16,
    pub absinfo: input_absinfo,
}
```

#### Trait Implementations

##### `impl Clone for uinput_abs_setup`

- `fn clone(self: &Self) -> uinput_abs_setup` — [`uinput_abs_setup`](../index.md)

##### `impl Copy for uinput_abs_setup`

##### `impl Debug for uinput_abs_setup`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `dl_phdr_info`

```rust
struct dl_phdr_info {
    pub dlpi_addr: Elf64_Addr,
    pub dlpi_name: *const crate::c_char,
    pub dlpi_phdr: *const Elf64_Phdr,
    pub dlpi_phnum: Elf64_Half,
    pub dlpi_adds: crate::c_ulonglong,
    pub dlpi_subs: crate::c_ulonglong,
    pub dlpi_tls_modid: crate::size_t,
    pub dlpi_tls_data: *mut crate::c_void,
}
```

#### Trait Implementations

##### `impl Clone for dl_phdr_info`

- `fn clone(self: &Self) -> dl_phdr_info` — [`dl_phdr_info`](../index.md)

##### `impl Copy for dl_phdr_info`

##### `impl Debug for dl_phdr_info`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Elf32_Ehdr`

```rust
struct Elf32_Ehdr {
    pub e_ident: [crate::c_uchar; 16],
    pub e_type: Elf32_Half,
    pub e_machine: Elf32_Half,
    pub e_version: Elf32_Word,
    pub e_entry: Elf32_Addr,
    pub e_phoff: Elf32_Off,
    pub e_shoff: Elf32_Off,
    pub e_flags: Elf32_Word,
    pub e_ehsize: Elf32_Half,
    pub e_phentsize: Elf32_Half,
    pub e_phnum: Elf32_Half,
    pub e_shentsize: Elf32_Half,
    pub e_shnum: Elf32_Half,
    pub e_shstrndx: Elf32_Half,
}
```

#### Trait Implementations

##### `impl Clone for Elf32_Ehdr`

- `fn clone(self: &Self) -> Elf32_Ehdr` — [`Elf32_Ehdr`](../index.md)

##### `impl Copy for Elf32_Ehdr`

##### `impl Debug for Elf32_Ehdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Elf64_Ehdr`

```rust
struct Elf64_Ehdr {
    pub e_ident: [crate::c_uchar; 16],
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr,
    pub e_phoff: Elf64_Off,
    pub e_shoff: Elf64_Off,
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}
```

#### Trait Implementations

##### `impl Clone for Elf64_Ehdr`

- `fn clone(self: &Self) -> Elf64_Ehdr` — [`Elf64_Ehdr`](../index.md)

##### `impl Copy for Elf64_Ehdr`

##### `impl Debug for Elf64_Ehdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Elf32_Sym`

```rust
struct Elf32_Sym {
    pub st_name: Elf32_Word,
    pub st_value: Elf32_Addr,
    pub st_size: Elf32_Word,
    pub st_info: crate::c_uchar,
    pub st_other: crate::c_uchar,
    pub st_shndx: Elf32_Section,
}
```

#### Trait Implementations

##### `impl Clone for Elf32_Sym`

- `fn clone(self: &Self) -> Elf32_Sym` — [`Elf32_Sym`](../index.md)

##### `impl Copy for Elf32_Sym`

##### `impl Debug for Elf32_Sym`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Elf64_Sym`

```rust
struct Elf64_Sym {
    pub st_name: Elf64_Word,
    pub st_info: crate::c_uchar,
    pub st_other: crate::c_uchar,
    pub st_shndx: Elf64_Section,
    pub st_value: Elf64_Addr,
    pub st_size: Elf64_Xword,
}
```

#### Trait Implementations

##### `impl Clone for Elf64_Sym`

- `fn clone(self: &Self) -> Elf64_Sym` — [`Elf64_Sym`](../index.md)

##### `impl Copy for Elf64_Sym`

##### `impl Debug for Elf64_Sym`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Elf32_Phdr`

```rust
struct Elf32_Phdr {
    pub p_type: Elf32_Word,
    pub p_offset: Elf32_Off,
    pub p_vaddr: Elf32_Addr,
    pub p_paddr: Elf32_Addr,
    pub p_filesz: Elf32_Word,
    pub p_memsz: Elf32_Word,
    pub p_flags: Elf32_Word,
    pub p_align: Elf32_Word,
}
```

#### Trait Implementations

##### `impl Clone for Elf32_Phdr`

- `fn clone(self: &Self) -> Elf32_Phdr` — [`Elf32_Phdr`](../index.md)

##### `impl Copy for Elf32_Phdr`

##### `impl Debug for Elf32_Phdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Elf64_Phdr`

```rust
struct Elf64_Phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}
```

#### Trait Implementations

##### `impl Clone for Elf64_Phdr`

- `fn clone(self: &Self) -> Elf64_Phdr` — [`Elf64_Phdr`](../index.md)

##### `impl Copy for Elf64_Phdr`

##### `impl Debug for Elf64_Phdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Elf32_Shdr`

```rust
struct Elf32_Shdr {
    pub sh_name: Elf32_Word,
    pub sh_type: Elf32_Word,
    pub sh_flags: Elf32_Word,
    pub sh_addr: Elf32_Addr,
    pub sh_offset: Elf32_Off,
    pub sh_size: Elf32_Word,
    pub sh_link: Elf32_Word,
    pub sh_info: Elf32_Word,
    pub sh_addralign: Elf32_Word,
    pub sh_entsize: Elf32_Word,
}
```

#### Trait Implementations

##### `impl Clone for Elf32_Shdr`

- `fn clone(self: &Self) -> Elf32_Shdr` — [`Elf32_Shdr`](../index.md)

##### `impl Copy for Elf32_Shdr`

##### `impl Debug for Elf32_Shdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Elf64_Shdr`

```rust
struct Elf64_Shdr {
    pub sh_name: Elf64_Word,
    pub sh_type: Elf64_Word,
    pub sh_flags: Elf64_Xword,
    pub sh_addr: Elf64_Addr,
    pub sh_offset: Elf64_Off,
    pub sh_size: Elf64_Xword,
    pub sh_link: Elf64_Word,
    pub sh_info: Elf64_Word,
    pub sh_addralign: Elf64_Xword,
    pub sh_entsize: Elf64_Xword,
}
```

#### Trait Implementations

##### `impl Clone for Elf64_Shdr`

- `fn clone(self: &Self) -> Elf64_Shdr` — [`Elf64_Shdr`](../index.md)

##### `impl Copy for Elf64_Shdr`

##### `impl Debug for Elf64_Shdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__c_anonymous_elf32_rel`

```rust
struct __c_anonymous_elf32_rel {
    pub r_offset: Elf32_Addr,
    pub r_info: Elf32_Word,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_elf32_rel`

- `fn clone(self: &Self) -> __c_anonymous_elf32_rel` — [`__c_anonymous_elf32_rel`](../index.md)

##### `impl Copy for __c_anonymous_elf32_rel`

##### `impl Debug for __c_anonymous_elf32_rel`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__c_anonymous_elf64_rel`

```rust
struct __c_anonymous_elf64_rel {
    pub r_offset: Elf64_Addr,
    pub r_info: Elf64_Xword,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_elf64_rel`

- `fn clone(self: &Self) -> __c_anonymous_elf64_rel` — [`__c_anonymous_elf64_rel`](../index.md)

##### `impl Copy for __c_anonymous_elf64_rel`

##### `impl Debug for __c_anonymous_elf64_rel`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__c_anonymous__kernel_fsid_t`

```rust
struct __c_anonymous__kernel_fsid_t {
    pub val: [crate::c_int; 2],
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous__kernel_fsid_t`

- `fn clone(self: &Self) -> __c_anonymous__kernel_fsid_t` — [`__c_anonymous__kernel_fsid_t`](../index.md)

##### `impl Copy for __c_anonymous__kernel_fsid_t`

##### `impl Debug for __c_anonymous__kernel_fsid_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ucred`

```rust
struct ucred {
    pub pid: crate::pid_t,
    pub uid: crate::uid_t,
    pub gid: crate::gid_t,
}
```

#### Trait Implementations

##### `impl Clone for ucred`

- `fn clone(self: &Self) -> ucred` — [`ucred`](../index.md)

##### `impl Copy for ucred`

##### `impl Debug for ucred`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `mntent`

```rust
struct mntent {
    pub mnt_fsname: *mut crate::c_char,
    pub mnt_dir: *mut crate::c_char,
    pub mnt_type: *mut crate::c_char,
    pub mnt_opts: *mut crate::c_char,
    pub mnt_freq: crate::c_int,
    pub mnt_passno: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for mntent`

- `fn clone(self: &Self) -> mntent` — [`mntent`](../index.md)

##### `impl Copy for mntent`

##### `impl Debug for mntent`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `posix_spawn_file_actions_t`

```rust
struct posix_spawn_file_actions_t {
    __allocated: crate::c_int,
    __used: crate::c_int,
    __actions: *mut crate::c_int,
    __pad: crate::types::Padding<[crate::c_int; 16]>,
}
```

#### Trait Implementations

##### `impl Clone for posix_spawn_file_actions_t`

- `fn clone(self: &Self) -> posix_spawn_file_actions_t` — [`posix_spawn_file_actions_t`](../index.md)

##### `impl Copy for posix_spawn_file_actions_t`

##### `impl Debug for posix_spawn_file_actions_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `posix_spawnattr_t`

```rust
struct posix_spawnattr_t {
    __flags: crate::c_short,
    __pgrp: crate::pid_t,
    __sd: crate::sigset_t,
    __ss: crate::sigset_t,
    __sp: crate::sched_param,
    __policy: crate::c_int,
    __pad: crate::types::Padding<[crate::c_int; 16]>,
}
```

#### Trait Implementations

##### `impl Clone for posix_spawnattr_t`

- `fn clone(self: &Self) -> posix_spawnattr_t` — [`posix_spawnattr_t`](../index.md)

##### `impl Copy for posix_spawnattr_t`

##### `impl Debug for posix_spawnattr_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `genlmsghdr`

```rust
struct genlmsghdr {
    pub cmd: u8,
    pub version: u8,
    pub reserved: u16,
}
```

#### Trait Implementations

##### `impl Clone for genlmsghdr`

- `fn clone(self: &Self) -> genlmsghdr` — [`genlmsghdr`](../index.md)

##### `impl Copy for genlmsghdr`

##### `impl Debug for genlmsghdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `in6_pktinfo`

```rust
struct in6_pktinfo {
    pub ipi6_addr: crate::in6_addr,
    pub ipi6_ifindex: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for in6_pktinfo`

- `fn clone(self: &Self) -> in6_pktinfo` — [`in6_pktinfo`](../index.md)

##### `impl Copy for in6_pktinfo`

##### `impl Debug for in6_pktinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `arpd_request`

```rust
struct arpd_request {
    pub req: crate::c_ushort,
    pub ip: u32,
    pub dev: crate::c_ulong,
    pub stamp: crate::c_ulong,
    pub updated: crate::c_ulong,
    pub ha: [crate::c_uchar; 7],
}
```

#### Trait Implementations

##### `impl Clone for arpd_request`

- `fn clone(self: &Self) -> arpd_request` — [`arpd_request`](../index.md)

##### `impl Copy for arpd_request`

##### `impl Debug for arpd_request`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `inotify_event`

```rust
struct inotify_event {
    pub wd: crate::c_int,
    pub mask: u32,
    pub cookie: u32,
    pub len: u32,
}
```

#### Trait Implementations

##### `impl Clone for inotify_event`

- `fn clone(self: &Self) -> inotify_event` — [`inotify_event`](../index.md)

##### `impl Copy for inotify_event`

##### `impl Debug for inotify_event`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fanotify_response`

```rust
struct fanotify_response {
    pub fd: crate::c_int,
    pub response: __u32,
}
```

#### Trait Implementations

##### `impl Clone for fanotify_response`

- `fn clone(self: &Self) -> fanotify_response` — [`fanotify_response`](../index.md)

##### `impl Copy for fanotify_response`

##### `impl Debug for fanotify_response`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fanotify_event_info_header`

```rust
struct fanotify_event_info_header {
    pub info_type: __u8,
    pub pad: __u8,
    pub len: __u16,
}
```

#### Trait Implementations

##### `impl Clone for fanotify_event_info_header`

- `fn clone(self: &Self) -> fanotify_event_info_header` — [`fanotify_event_info_header`](../index.md)

##### `impl Copy for fanotify_event_info_header`

##### `impl Debug for fanotify_event_info_header`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fanotify_event_info_fid`

```rust
struct fanotify_event_info_fid {
    pub hdr: fanotify_event_info_header,
    pub fsid: crate::__kernel_fsid_t,
    pub handle: [crate::c_uchar; 0],
}
```

#### Trait Implementations

##### `impl Clone for fanotify_event_info_fid`

- `fn clone(self: &Self) -> fanotify_event_info_fid` — [`fanotify_event_info_fid`](../index.md)

##### `impl Copy for fanotify_event_info_fid`

##### `impl Debug for fanotify_event_info_fid`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr_vm`

```rust
struct sockaddr_vm {
    pub svm_family: crate::sa_family_t,
    pub svm_reserved1: crate::c_ushort,
    pub svm_port: crate::c_uint,
    pub svm_cid: crate::c_uint,
    pub svm_zero: [u8; 4],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_vm`

- `fn clone(self: &Self) -> sockaddr_vm` — [`sockaddr_vm`](../index.md)

##### `impl Copy for sockaddr_vm`

##### `impl Debug for sockaddr_vm`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `regmatch_t`

```rust
struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
```

#### Trait Implementations

##### `impl Clone for regmatch_t`

- `fn clone(self: &Self) -> regmatch_t` — [`regmatch_t`](../index.md)

##### `impl Copy for regmatch_t`

##### `impl Debug for regmatch_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sock_extended_err`

```rust
struct sock_extended_err {
    pub ee_errno: u32,
    pub ee_origin: u8,
    pub ee_type: u8,
    pub ee_code: u8,
    pub ee_pad: u8,
    pub ee_info: u32,
    pub ee_data: u32,
}
```

#### Trait Implementations

##### `impl Clone for sock_extended_err`

- `fn clone(self: &Self) -> sock_extended_err` — [`sock_extended_err`](../index.md)

##### `impl Copy for sock_extended_err`

##### `impl Debug for sock_extended_err`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `seccomp_data`

```rust
struct seccomp_data {
    pub nr: crate::c_int,
    pub arch: __u32,
    pub instruction_pointer: crate::__u64,
    pub args: [crate::__u64; 6],
}
```

#### Trait Implementations

##### `impl Clone for seccomp_data`

- `fn clone(self: &Self) -> seccomp_data` — [`seccomp_data`](../index.md)

##### `impl Copy for seccomp_data`

##### `impl Debug for seccomp_data`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `seccomp_notif_sizes`

```rust
struct seccomp_notif_sizes {
    pub seccomp_notif: __u16,
    pub seccomp_notif_resp: __u16,
    pub seccomp_data: __u16,
}
```

#### Trait Implementations

##### `impl Clone for seccomp_notif_sizes`

- `fn clone(self: &Self) -> seccomp_notif_sizes` — [`seccomp_notif_sizes`](../index.md)

##### `impl Copy for seccomp_notif_sizes`

##### `impl Debug for seccomp_notif_sizes`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `seccomp_notif`

```rust
struct seccomp_notif {
    pub id: crate::__u64,
    pub pid: __u32,
    pub flags: __u32,
    pub data: seccomp_data,
}
```

#### Trait Implementations

##### `impl Clone for seccomp_notif`

- `fn clone(self: &Self) -> seccomp_notif` — [`seccomp_notif`](../index.md)

##### `impl Copy for seccomp_notif`

##### `impl Debug for seccomp_notif`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `seccomp_notif_resp`

```rust
struct seccomp_notif_resp {
    pub id: crate::__u64,
    pub val: crate::__s64,
    pub error: __s32,
    pub flags: __u32,
}
```

#### Trait Implementations

##### `impl Clone for seccomp_notif_resp`

- `fn clone(self: &Self) -> seccomp_notif_resp` — [`seccomp_notif_resp`](../index.md)

##### `impl Copy for seccomp_notif_resp`

##### `impl Debug for seccomp_notif_resp`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `seccomp_notif_addfd`

```rust
struct seccomp_notif_addfd {
    pub id: crate::__u64,
    pub flags: __u32,
    pub srcfd: __u32,
    pub newfd: __u32,
    pub newfd_flags: __u32,
}
```

#### Trait Implementations

##### `impl Clone for seccomp_notif_addfd`

- `fn clone(self: &Self) -> seccomp_notif_addfd` — [`seccomp_notif_addfd`](../index.md)

##### `impl Copy for seccomp_notif_addfd`

##### `impl Debug for seccomp_notif_addfd`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `nlmsghdr`

```rust
struct nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}
```

#### Trait Implementations

##### `impl Clone for nlmsghdr`

- `fn clone(self: &Self) -> nlmsghdr` — [`nlmsghdr`](../index.md)

##### `impl Copy for nlmsghdr`

##### `impl Debug for nlmsghdr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `nlmsgerr`

```rust
struct nlmsgerr {
    pub error: crate::c_int,
    pub msg: nlmsghdr,
}
```

#### Trait Implementations

##### `impl Clone for nlmsgerr`

- `fn clone(self: &Self) -> nlmsgerr` — [`nlmsgerr`](../index.md)

##### `impl Copy for nlmsgerr`

##### `impl Debug for nlmsgerr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `nlattr`

```rust
struct nlattr {
    pub nla_len: u16,
    pub nla_type: u16,
}
```

#### Trait Implementations

##### `impl Clone for nlattr`

- `fn clone(self: &Self) -> nlattr` — [`nlattr`](../index.md)

##### `impl Copy for nlattr`

##### `impl Debug for nlattr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__c_anonymous_ifru_map`

```rust
struct __c_anonymous_ifru_map {
    pub mem_start: crate::c_ulong,
    pub mem_end: crate::c_ulong,
    pub base_addr: crate::c_ushort,
    pub irq: crate::c_uchar,
    pub dma: crate::c_uchar,
    pub port: crate::c_uchar,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_ifru_map`

- `fn clone(self: &Self) -> __c_anonymous_ifru_map` — [`__c_anonymous_ifru_map`](../index.md)

##### `impl Copy for __c_anonymous_ifru_map`

##### `impl Debug for __c_anonymous_ifru_map`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `in6_ifreq`

```rust
struct in6_ifreq {
    pub ifr6_addr: crate::in6_addr,
    pub ifr6_prefixlen: u32,
    pub ifr6_ifindex: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for in6_ifreq`

- `fn clone(self: &Self) -> in6_ifreq` — [`in6_ifreq`](../index.md)

##### `impl Copy for in6_ifreq`

##### `impl Debug for in6_ifreq`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `option`

```rust
struct option {
    pub name: *const crate::c_char,
    pub has_arg: crate::c_int,
    pub flag: *mut crate::c_int,
    pub val: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for option`

- `fn clone(self: &Self) -> option` — [`option`](../index.md)

##### `impl Copy for option`

##### `impl Debug for option`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `open_how`

```rust
struct open_how {
    pub flags: crate::__u64,
    pub mode: crate::__u64,
    pub resolve: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for open_how`

- `fn clone(self: &Self) -> open_how` — [`open_how`](../index.md)

##### `impl Copy for open_how`

##### `impl Debug for open_how`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptp_clock_time`

```rust
struct ptp_clock_time {
    pub sec: crate::__s64,
    pub nsec: __u32,
    pub reserved: __u32,
}
```

#### Trait Implementations

##### `impl Clone for ptp_clock_time`

- `fn clone(self: &Self) -> ptp_clock_time` — [`ptp_clock_time`](../index.md)

##### `impl Copy for ptp_clock_time`

##### `impl Debug for ptp_clock_time`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptp_extts_request`

```rust
struct ptp_extts_request {
    pub index: crate::c_uint,
    pub flags: crate::c_uint,
    pub rsv: [crate::c_uint; 2],
}
```

#### Trait Implementations

##### `impl Clone for ptp_extts_request`

- `fn clone(self: &Self) -> ptp_extts_request` — [`ptp_extts_request`](../index.md)

##### `impl Copy for ptp_extts_request`

##### `impl Debug for ptp_extts_request`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptp_sys_offset_extended`

```rust
struct ptp_sys_offset_extended {
    pub n_samples: crate::c_uint,
    pub clockid: __kernel_clockid_t,
    pub rsv: [crate::c_uint; 2],
    pub ts: [[ptp_clock_time; 3]; 25],
}
```

#### Trait Implementations

##### `impl Clone for ptp_sys_offset_extended`

- `fn clone(self: &Self) -> ptp_sys_offset_extended` — [`ptp_sys_offset_extended`](../index.md)

##### `impl Copy for ptp_sys_offset_extended`

##### `impl Debug for ptp_sys_offset_extended`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptp_sys_offset_precise`

```rust
struct ptp_sys_offset_precise {
    pub device: ptp_clock_time,
    pub sys_realtime: ptp_clock_time,
    pub sys_monoraw: ptp_clock_time,
    pub rsv: [crate::c_uint; 4],
}
```

#### Trait Implementations

##### `impl Clone for ptp_sys_offset_precise`

- `fn clone(self: &Self) -> ptp_sys_offset_precise` — [`ptp_sys_offset_precise`](../index.md)

##### `impl Copy for ptp_sys_offset_precise`

##### `impl Debug for ptp_sys_offset_precise`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptp_extts_event`

```rust
struct ptp_extts_event {
    pub t: ptp_clock_time,
    index: crate::c_uint,
    flags: crate::c_uint,
    rsv: [crate::c_uint; 2],
}
```

#### Trait Implementations

##### `impl Clone for ptp_extts_event`

- `fn clone(self: &Self) -> ptp_extts_event` — [`ptp_extts_event`](../index.md)

##### `impl Copy for ptp_extts_event`

##### `impl Debug for ptp_extts_event`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sctp_initmsg`

```rust
struct sctp_initmsg {
    pub sinit_num_ostreams: __u16,
    pub sinit_max_instreams: __u16,
    pub sinit_max_attempts: __u16,
    pub sinit_max_init_timeo: __u16,
}
```

#### Trait Implementations

##### `impl Clone for sctp_initmsg`

- `fn clone(self: &Self) -> sctp_initmsg` — [`sctp_initmsg`](../index.md)

##### `impl Copy for sctp_initmsg`

##### `impl Debug for sctp_initmsg`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sctp_sndrcvinfo`

```rust
struct sctp_sndrcvinfo {
    pub sinfo_stream: __u16,
    pub sinfo_ssn: __u16,
    pub sinfo_flags: __u16,
    pub sinfo_ppid: __u32,
    pub sinfo_context: __u32,
    pub sinfo_timetolive: __u32,
    pub sinfo_tsn: __u32,
    pub sinfo_cumtsn: __u32,
    pub sinfo_assoc_id: crate::sctp_assoc_t,
}
```

#### Trait Implementations

##### `impl Clone for sctp_sndrcvinfo`

- `fn clone(self: &Self) -> sctp_sndrcvinfo` — [`sctp_sndrcvinfo`](../index.md)

##### `impl Copy for sctp_sndrcvinfo`

##### `impl Debug for sctp_sndrcvinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sctp_sndinfo`

```rust
struct sctp_sndinfo {
    pub snd_sid: __u16,
    pub snd_flags: __u16,
    pub snd_ppid: __u32,
    pub snd_context: __u32,
    pub snd_assoc_id: crate::sctp_assoc_t,
}
```

#### Trait Implementations

##### `impl Clone for sctp_sndinfo`

- `fn clone(self: &Self) -> sctp_sndinfo` — [`sctp_sndinfo`](../index.md)

##### `impl Copy for sctp_sndinfo`

##### `impl Debug for sctp_sndinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sctp_rcvinfo`

```rust
struct sctp_rcvinfo {
    pub rcv_sid: __u16,
    pub rcv_ssn: __u16,
    pub rcv_flags: __u16,
    pub rcv_ppid: __u32,
    pub rcv_tsn: __u32,
    pub rcv_cumtsn: __u32,
    pub rcv_context: __u32,
    pub rcv_assoc_id: crate::sctp_assoc_t,
}
```

#### Trait Implementations

##### `impl Clone for sctp_rcvinfo`

- `fn clone(self: &Self) -> sctp_rcvinfo` — [`sctp_rcvinfo`](../index.md)

##### `impl Copy for sctp_rcvinfo`

##### `impl Debug for sctp_rcvinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sctp_nxtinfo`

```rust
struct sctp_nxtinfo {
    pub nxt_sid: __u16,
    pub nxt_flags: __u16,
    pub nxt_ppid: __u32,
    pub nxt_length: __u32,
    pub nxt_assoc_id: crate::sctp_assoc_t,
}
```

#### Trait Implementations

##### `impl Clone for sctp_nxtinfo`

- `fn clone(self: &Self) -> sctp_nxtinfo` — [`sctp_nxtinfo`](../index.md)

##### `impl Copy for sctp_nxtinfo`

##### `impl Debug for sctp_nxtinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sctp_prinfo`

```rust
struct sctp_prinfo {
    pub pr_policy: __u16,
    pub pr_value: __u32,
}
```

#### Trait Implementations

##### `impl Clone for sctp_prinfo`

- `fn clone(self: &Self) -> sctp_prinfo` — [`sctp_prinfo`](../index.md)

##### `impl Copy for sctp_prinfo`

##### `impl Debug for sctp_prinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sctp_authinfo`

```rust
struct sctp_authinfo {
    pub auth_keynumber: __u16,
}
```

#### Trait Implementations

##### `impl Clone for sctp_authinfo`

- `fn clone(self: &Self) -> sctp_authinfo` — [`sctp_authinfo`](../index.md)

##### `impl Copy for sctp_authinfo`

##### `impl Debug for sctp_authinfo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `rlimit64`

```rust
struct rlimit64 {
    pub rlim_cur: rlim64_t,
    pub rlim_max: rlim64_t,
}
```

#### Trait Implementations

##### `impl Clone for rlimit64`

- `fn clone(self: &Self) -> rlimit64` — [`rlimit64`](../index.md)

##### `impl Copy for rlimit64`

##### `impl Debug for rlimit64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tls_crypto_info`

```rust
struct tls_crypto_info {
    pub version: __u16,
    pub cipher_type: __u16,
}
```

#### Trait Implementations

##### `impl Clone for tls_crypto_info`

- `fn clone(self: &Self) -> tls_crypto_info` — [`tls_crypto_info`](../index.md)

##### `impl Copy for tls_crypto_info`

##### `impl Debug for tls_crypto_info`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tls12_crypto_info_aes_gcm_128`

```rust
struct tls12_crypto_info_aes_gcm_128 {
    pub info: tls_crypto_info,
    pub iv: [crate::c_uchar; 8],
    pub key: [crate::c_uchar; 16],
    pub salt: [crate::c_uchar; 4],
    pub rec_seq: [crate::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_aes_gcm_128`

- `fn clone(self: &Self) -> tls12_crypto_info_aes_gcm_128` — [`tls12_crypto_info_aes_gcm_128`](../index.md)

##### `impl Copy for tls12_crypto_info_aes_gcm_128`

##### `impl Debug for tls12_crypto_info_aes_gcm_128`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tls12_crypto_info_aes_gcm_256`

```rust
struct tls12_crypto_info_aes_gcm_256 {
    pub info: tls_crypto_info,
    pub iv: [crate::c_uchar; 8],
    pub key: [crate::c_uchar; 32],
    pub salt: [crate::c_uchar; 4],
    pub rec_seq: [crate::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_aes_gcm_256`

- `fn clone(self: &Self) -> tls12_crypto_info_aes_gcm_256` — [`tls12_crypto_info_aes_gcm_256`](../index.md)

##### `impl Copy for tls12_crypto_info_aes_gcm_256`

##### `impl Debug for tls12_crypto_info_aes_gcm_256`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tls12_crypto_info_aes_ccm_128`

```rust
struct tls12_crypto_info_aes_ccm_128 {
    pub info: tls_crypto_info,
    pub iv: [crate::c_uchar; 8],
    pub key: [crate::c_uchar; 16],
    pub salt: [crate::c_uchar; 4],
    pub rec_seq: [crate::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_aes_ccm_128`

- `fn clone(self: &Self) -> tls12_crypto_info_aes_ccm_128` — [`tls12_crypto_info_aes_ccm_128`](../index.md)

##### `impl Copy for tls12_crypto_info_aes_ccm_128`

##### `impl Debug for tls12_crypto_info_aes_ccm_128`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tls12_crypto_info_chacha20_poly1305`

```rust
struct tls12_crypto_info_chacha20_poly1305 {
    pub info: tls_crypto_info,
    pub iv: [crate::c_uchar; 12],
    pub key: [crate::c_uchar; 32],
    pub salt: [crate::c_uchar; 0],
    pub rec_seq: [crate::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_chacha20_poly1305`

- `fn clone(self: &Self) -> tls12_crypto_info_chacha20_poly1305` — [`tls12_crypto_info_chacha20_poly1305`](../index.md)

##### `impl Copy for tls12_crypto_info_chacha20_poly1305`

##### `impl Debug for tls12_crypto_info_chacha20_poly1305`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tls12_crypto_info_sm4_gcm`

```rust
struct tls12_crypto_info_sm4_gcm {
    pub info: tls_crypto_info,
    pub iv: [crate::c_uchar; 8],
    pub key: [crate::c_uchar; 16],
    pub salt: [crate::c_uchar; 4],
    pub rec_seq: [crate::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_sm4_gcm`

- `fn clone(self: &Self) -> tls12_crypto_info_sm4_gcm` — [`tls12_crypto_info_sm4_gcm`](../index.md)

##### `impl Copy for tls12_crypto_info_sm4_gcm`

##### `impl Debug for tls12_crypto_info_sm4_gcm`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tls12_crypto_info_sm4_ccm`

```rust
struct tls12_crypto_info_sm4_ccm {
    pub info: tls_crypto_info,
    pub iv: [crate::c_uchar; 8],
    pub key: [crate::c_uchar; 16],
    pub salt: [crate::c_uchar; 4],
    pub rec_seq: [crate::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_sm4_ccm`

- `fn clone(self: &Self) -> tls12_crypto_info_sm4_ccm` — [`tls12_crypto_info_sm4_ccm`](../index.md)

##### `impl Copy for tls12_crypto_info_sm4_ccm`

##### `impl Debug for tls12_crypto_info_sm4_ccm`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tls12_crypto_info_aria_gcm_128`

```rust
struct tls12_crypto_info_aria_gcm_128 {
    pub info: tls_crypto_info,
    pub iv: [crate::c_uchar; 8],
    pub key: [crate::c_uchar; 16],
    pub salt: [crate::c_uchar; 4],
    pub rec_seq: [crate::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_aria_gcm_128`

- `fn clone(self: &Self) -> tls12_crypto_info_aria_gcm_128` — [`tls12_crypto_info_aria_gcm_128`](../index.md)

##### `impl Copy for tls12_crypto_info_aria_gcm_128`

##### `impl Debug for tls12_crypto_info_aria_gcm_128`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tls12_crypto_info_aria_gcm_256`

```rust
struct tls12_crypto_info_aria_gcm_256 {
    pub info: tls_crypto_info,
    pub iv: [crate::c_uchar; 8],
    pub key: [crate::c_uchar; 32],
    pub salt: [crate::c_uchar; 4],
    pub rec_seq: [crate::c_uchar; 8],
}
```

#### Trait Implementations

##### `impl Clone for tls12_crypto_info_aria_gcm_256`

- `fn clone(self: &Self) -> tls12_crypto_info_aria_gcm_256` — [`tls12_crypto_info_aria_gcm_256`](../index.md)

##### `impl Copy for tls12_crypto_info_aria_gcm_256`

##### `impl Debug for tls12_crypto_info_aria_gcm_256`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_param`

```rust
struct iw_param {
    pub value: __s32,
    pub fixed: __u8,
    pub disabled: __u8,
    pub flags: __u16,
}
```

#### Trait Implementations

##### `impl Clone for iw_param`

- `fn clone(self: &Self) -> iw_param` — [`iw_param`](../index.md)

##### `impl Copy for iw_param`

##### `impl Debug for iw_param`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_point`

```rust
struct iw_point {
    pub pointer: *mut crate::c_void,
    pub length: __u16,
    pub flags: __u16,
}
```

#### Trait Implementations

##### `impl Clone for iw_point`

- `fn clone(self: &Self) -> iw_point` — [`iw_point`](../index.md)

##### `impl Copy for iw_point`

##### `impl Debug for iw_point`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_freq`

```rust
struct iw_freq {
    pub m: __s32,
    pub e: __s16,
    pub i: __u8,
    pub flags: __u8,
}
```

#### Trait Implementations

##### `impl Clone for iw_freq`

- `fn clone(self: &Self) -> iw_freq` — [`iw_freq`](../index.md)

##### `impl Copy for iw_freq`

##### `impl Debug for iw_freq`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_quality`

```rust
struct iw_quality {
    pub qual: __u8,
    pub level: __u8,
    pub noise: __u8,
    pub updated: __u8,
}
```

#### Trait Implementations

##### `impl Clone for iw_quality`

- `fn clone(self: &Self) -> iw_quality` — [`iw_quality`](../index.md)

##### `impl Copy for iw_quality`

##### `impl Debug for iw_quality`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_discarded`

```rust
struct iw_discarded {
    pub nwid: __u32,
    pub code: __u32,
    pub fragment: __u32,
    pub retries: __u32,
    pubmisc: __u32,
}
```

#### Trait Implementations

##### `impl Clone for iw_discarded`

- `fn clone(self: &Self) -> iw_discarded` — [`iw_discarded`](../index.md)

##### `impl Copy for iw_discarded`

##### `impl Debug for iw_discarded`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_missed`

```rust
struct iw_missed {
    pub beacon: __u32,
}
```

#### Trait Implementations

##### `impl Clone for iw_missed`

- `fn clone(self: &Self) -> iw_missed` — [`iw_missed`](../index.md)

##### `impl Copy for iw_missed`

##### `impl Debug for iw_missed`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_scan_req`

```rust
struct iw_scan_req {
    pub scan_type: __u8,
    pub essid_len: __u8,
    pub num_channels: __u8,
    pub flags: __u8,
    pub bssid: crate::sockaddr,
    pub essid: [__u8; 32],
    pub min_channel_time: __u32,
    pub max_channel_time: __u32,
    pub channel_list: [iw_freq; 32],
}
```

#### Trait Implementations

##### `impl Clone for iw_scan_req`

- `fn clone(self: &Self) -> iw_scan_req` — [`iw_scan_req`](../index.md)

##### `impl Copy for iw_scan_req`

##### `impl Debug for iw_scan_req`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_encode_ext`

```rust
struct iw_encode_ext {
    pub ext_flags: __u32,
    pub tx_seq: [__u8; 8],
    pub rx_seq: [__u8; 8],
    pub addr: crate::sockaddr,
    pub alg: __u16,
    pub key_len: __u16,
    pub key: [__u8; 0],
}
```

#### Trait Implementations

##### `impl Clone for iw_encode_ext`

- `fn clone(self: &Self) -> iw_encode_ext` — [`iw_encode_ext`](../index.md)

##### `impl Copy for iw_encode_ext`

##### `impl Debug for iw_encode_ext`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_pmksa`

```rust
struct iw_pmksa {
    pub cmd: __u32,
    pub bssid: crate::sockaddr,
    pub pmkid: [__u8; 16],
}
```

#### Trait Implementations

##### `impl Clone for iw_pmksa`

- `fn clone(self: &Self) -> iw_pmksa` — [`iw_pmksa`](../index.md)

##### `impl Copy for iw_pmksa`

##### `impl Debug for iw_pmksa`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_pmkid_cand`

```rust
struct iw_pmkid_cand {
    pub flags: __u32,
    pub index: __u32,
    pub bssid: crate::sockaddr,
}
```

#### Trait Implementations

##### `impl Clone for iw_pmkid_cand`

- `fn clone(self: &Self) -> iw_pmkid_cand` — [`iw_pmkid_cand`](../index.md)

##### `impl Copy for iw_pmkid_cand`

##### `impl Debug for iw_pmkid_cand`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_statistics`

```rust
struct iw_statistics {
    pub status: __u16,
    pub qual: iw_quality,
    pub discard: iw_discarded,
    pub miss: iw_missed,
}
```

#### Trait Implementations

##### `impl Clone for iw_statistics`

- `fn clone(self: &Self) -> iw_statistics` — [`iw_statistics`](../index.md)

##### `impl Copy for iw_statistics`

##### `impl Debug for iw_statistics`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_range`

```rust
struct iw_range {
    pub throughput: __u32,
    pub min_nwid: __u32,
    pub max_nwid: __u32,
    pub old_num_channels: __u16,
    pub old_num_frequency: __u8,
    pub scan_capa: __u8,
    pub event_capa: [__u32; 6],
    pub sensitivity: __s32,
    pub max_qual: iw_quality,
    pub avg_qual: iw_quality,
    pub num_bitrates: __u8,
    pub bitrate: [__s32; 32],
    pub min_rts: __s32,
    pub max_rts: __s32,
    pub min_frag: __s32,
    pub max_frag: __s32,
    pub min_pmp: __s32,
    pub max_pmp: __s32,
    pub min_pmt: __s32,
    pub max_pmt: __s32,
    pub pmp_flags: __u16,
    pub pmt_flags: __u16,
    pub pm_capa: __u16,
    pub encoding_size: [__u16; 8],
    pub num_encoding_sizes: __u8,
    pub max_encoding_tokens: __u8,
    pub encoding_login_index: __u8,
    pub txpower_capa: __u16,
    pub num_txpower: __u8,
    pub txpower: [__s32; 8],
    pub we_version_compiled: __u8,
    pub we_version_source: __u8,
    pub retry_capa: __u16,
    pub retry_flags: __u16,
    pub r_time_flags: __u16,
    pub min_retry: __s32,
    pub max_retry: __s32,
    pub min_r_time: __s32,
    pub max_r_time: __s32,
    pub num_channels: __u16,
    pub num_frequency: __u8,
    pub freq: [iw_freq; 32],
    pub enc_capa: __u32,
}
```

#### Trait Implementations

##### `impl Clone for iw_range`

- `fn clone(self: &Self) -> iw_range` — [`iw_range`](../index.md)

##### `impl Copy for iw_range`

##### `impl Debug for iw_range`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_priv_args`

```rust
struct iw_priv_args {
    pub cmd: __u32,
    pub set_args: __u16,
    pub get_args: __u16,
    pub name: [crate::c_char; 16],
}
```

#### Trait Implementations

##### `impl Clone for iw_priv_args`

- `fn clone(self: &Self) -> iw_priv_args` — [`iw_priv_args`](../index.md)

##### `impl Copy for iw_priv_args`

##### `impl Debug for iw_priv_args`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `epoll_params`

```rust
struct epoll_params {
    pub busy_poll_usecs: u32,
    pub busy_poll_budget: u16,
    pub prefer_busy_poll: u8,
    pub __pad: u8,
}
```

#### Trait Implementations

##### `impl Clone for epoll_params`

- `fn clone(self: &Self) -> epoll_params` — [`epoll_params`](../index.md)

##### `impl Copy for epoll_params`

##### `impl Debug for epoll_params`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pthread_mutexattr_t`

```rust
struct pthread_mutexattr_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_mutexattr_t`

- `fn clone(self: &Self) -> pthread_mutexattr_t` — [`pthread_mutexattr_t`](../index.md)

##### `impl Copy for pthread_mutexattr_t`

##### `impl Debug for pthread_mutexattr_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pthread_rwlockattr_t`

```rust
struct pthread_rwlockattr_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_rwlockattr_t`

- `fn clone(self: &Self) -> pthread_rwlockattr_t` — [`pthread_rwlockattr_t`](../index.md)

##### `impl Copy for pthread_rwlockattr_t`

##### `impl Debug for pthread_rwlockattr_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pthread_condattr_t`

```rust
struct pthread_condattr_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_condattr_t`

- `fn clone(self: &Self) -> pthread_condattr_t` — [`pthread_condattr_t`](../index.md)

##### `impl Copy for pthread_condattr_t`

##### `impl Debug for pthread_condattr_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pthread_barrierattr_t`

```rust
struct pthread_barrierattr_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_barrierattr_t`

- `fn clone(self: &Self) -> pthread_barrierattr_t` — [`pthread_barrierattr_t`](../index.md)

##### `impl Copy for pthread_barrierattr_t`

##### `impl Debug for pthread_barrierattr_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `fanotify_event_metadata`

```rust
struct fanotify_event_metadata {
    pub event_len: __u32,
    pub vers: __u8,
    pub reserved: __u8,
    pub metadata_len: __u16,
    pub mask: __u64,
    pub fd: crate::c_int,
    pub pid: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for fanotify_event_metadata`

- `fn clone(self: &Self) -> fanotify_event_metadata` — [`fanotify_event_metadata`](../index.md)

##### `impl Copy for fanotify_event_metadata`

##### `impl Debug for fanotify_event_metadata`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptp_sys_offset`

```rust
struct ptp_sys_offset {
    pub n_samples: crate::c_uint,
    pub rsv: [crate::c_uint; 3],
    pub ts: [ptp_clock_time; 51],
}
```

#### Trait Implementations

##### `impl Clone for ptp_sys_offset`

- `fn clone(self: &Self) -> ptp_sys_offset` — [`ptp_sys_offset`](../index.md)

##### `impl Copy for ptp_sys_offset`

##### `impl Debug for ptp_sys_offset`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptp_pin_desc`

```rust
struct ptp_pin_desc {
    pub name: [crate::c_char; 64],
    pub index: crate::c_uint,
    pub func: crate::c_uint,
    pub chan: crate::c_uint,
    pub rsv: [crate::c_uint; 5],
}
```

#### Trait Implementations

##### `impl Clone for ptp_pin_desc`

- `fn clone(self: &Self) -> ptp_pin_desc` — [`ptp_pin_desc`](../index.md)

##### `impl Copy for ptp_pin_desc`

##### `impl Debug for ptp_pin_desc`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptp_clock_caps`

```rust
struct ptp_clock_caps {
    pub max_adj: crate::c_int,
    pub n_alarm: crate::c_int,
    pub n_ext_ts: crate::c_int,
    pub n_per_out: crate::c_int,
    pub pps: crate::c_int,
    pub n_pins: crate::c_int,
    pub cross_timestamping: crate::c_int,
    pub adjust_phase: crate::c_int,
    pub max_phase_adj: crate::c_int,
    pub rsv: [crate::c_int; 11],
}
```

#### Trait Implementations

##### `impl Clone for ptp_clock_caps`

- `fn clone(self: &Self) -> ptp_clock_caps` — [`ptp_clock_caps`](../index.md)

##### `impl Copy for ptp_clock_caps`

##### `impl Debug for ptp_clock_caps`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr_xdp`

```rust
struct sockaddr_xdp {
    pub sxdp_family: crate::__u16,
    pub sxdp_flags: crate::__u16,
    pub sxdp_ifindex: crate::__u32,
    pub sxdp_queue_id: crate::__u32,
    pub sxdp_shared_umem_fd: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_xdp`

- `fn clone(self: &Self) -> sockaddr_xdp` — [`sockaddr_xdp`](../index.md)

##### `impl Copy for sockaddr_xdp`

##### `impl Debug for sockaddr_xdp`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xdp_ring_offset`

```rust
struct xdp_ring_offset {
    pub producer: crate::__u64,
    pub consumer: crate::__u64,
    pub desc: crate::__u64,
    pub flags: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for xdp_ring_offset`

- `fn clone(self: &Self) -> xdp_ring_offset` — [`xdp_ring_offset`](../index.md)

##### `impl Copy for xdp_ring_offset`

##### `impl Debug for xdp_ring_offset`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xdp_mmap_offsets`

```rust
struct xdp_mmap_offsets {
    pub rx: xdp_ring_offset,
    pub tx: xdp_ring_offset,
    pub fr: xdp_ring_offset,
    pub cr: xdp_ring_offset,
}
```

#### Trait Implementations

##### `impl Clone for xdp_mmap_offsets`

- `fn clone(self: &Self) -> xdp_mmap_offsets` — [`xdp_mmap_offsets`](../index.md)

##### `impl Copy for xdp_mmap_offsets`

##### `impl Debug for xdp_mmap_offsets`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xdp_ring_offset_v1`

```rust
struct xdp_ring_offset_v1 {
    pub producer: crate::__u64,
    pub consumer: crate::__u64,
    pub desc: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for xdp_ring_offset_v1`

- `fn clone(self: &Self) -> xdp_ring_offset_v1` — [`xdp_ring_offset_v1`](../index.md)

##### `impl Copy for xdp_ring_offset_v1`

##### `impl Debug for xdp_ring_offset_v1`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xdp_mmap_offsets_v1`

```rust
struct xdp_mmap_offsets_v1 {
    pub rx: xdp_ring_offset_v1,
    pub tx: xdp_ring_offset_v1,
    pub fr: xdp_ring_offset_v1,
    pub cr: xdp_ring_offset_v1,
}
```

#### Trait Implementations

##### `impl Clone for xdp_mmap_offsets_v1`

- `fn clone(self: &Self) -> xdp_mmap_offsets_v1` — [`xdp_mmap_offsets_v1`](../index.md)

##### `impl Copy for xdp_mmap_offsets_v1`

##### `impl Debug for xdp_mmap_offsets_v1`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xdp_umem_reg`

```rust
struct xdp_umem_reg {
    pub addr: crate::__u64,
    pub len: crate::__u64,
    pub chunk_size: crate::__u32,
    pub headroom: crate::__u32,
    pub flags: crate::__u32,
    pub tx_metadata_len: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for xdp_umem_reg`

- `fn clone(self: &Self) -> xdp_umem_reg` — [`xdp_umem_reg`](../index.md)

##### `impl Copy for xdp_umem_reg`

##### `impl Debug for xdp_umem_reg`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xdp_umem_reg_v1`

```rust
struct xdp_umem_reg_v1 {
    pub addr: crate::__u64,
    pub len: crate::__u64,
    pub chunk_size: crate::__u32,
    pub headroom: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for xdp_umem_reg_v1`

- `fn clone(self: &Self) -> xdp_umem_reg_v1` — [`xdp_umem_reg_v1`](../index.md)

##### `impl Copy for xdp_umem_reg_v1`

##### `impl Debug for xdp_umem_reg_v1`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xdp_statistics`

```rust
struct xdp_statistics {
    pub rx_dropped: crate::__u64,
    pub rx_invalid_descs: crate::__u64,
    pub tx_invalid_descs: crate::__u64,
    pub rx_ring_full: crate::__u64,
    pub rx_fill_ring_empty_descs: crate::__u64,
    pub tx_ring_empty_descs: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for xdp_statistics`

- `fn clone(self: &Self) -> xdp_statistics` — [`xdp_statistics`](../index.md)

##### `impl Copy for xdp_statistics`

##### `impl Debug for xdp_statistics`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xdp_statistics_v1`

```rust
struct xdp_statistics_v1 {
    pub rx_dropped: crate::__u64,
    pub rx_invalid_descs: crate::__u64,
    pub tx_invalid_descs: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for xdp_statistics_v1`

- `fn clone(self: &Self) -> xdp_statistics_v1` — [`xdp_statistics_v1`](../index.md)

##### `impl Copy for xdp_statistics_v1`

##### `impl Debug for xdp_statistics_v1`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xdp_options`

```rust
struct xdp_options {
    pub flags: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for xdp_options`

- `fn clone(self: &Self) -> xdp_options` — [`xdp_options`](../index.md)

##### `impl Copy for xdp_options`

##### `impl Debug for xdp_options`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xdp_desc`

```rust
struct xdp_desc {
    pub addr: crate::__u64,
    pub len: crate::__u32,
    pub options: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for xdp_desc`

- `fn clone(self: &Self) -> xdp_desc` — [`xdp_desc`](../index.md)

##### `impl Copy for xdp_desc`

##### `impl Debug for xdp_desc`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xsk_tx_metadata_completion`

```rust
struct xsk_tx_metadata_completion {
    pub tx_timestamp: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for xsk_tx_metadata_completion`

- `fn clone(self: &Self) -> xsk_tx_metadata_completion` — [`xsk_tx_metadata_completion`](../index.md)

##### `impl Copy for xsk_tx_metadata_completion`

##### `impl Debug for xsk_tx_metadata_completion`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xsk_tx_metadata_request`

```rust
struct xsk_tx_metadata_request {
    pub csum_start: __u16,
    pub csum_offset: __u16,
}
```

#### Trait Implementations

##### `impl Clone for xsk_tx_metadata_request`

- `fn clone(self: &Self) -> xsk_tx_metadata_request` — [`xsk_tx_metadata_request`](../index.md)

##### `impl Copy for xsk_tx_metadata_request`

##### `impl Debug for xsk_tx_metadata_request`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `mount_attr`

```rust
struct mount_attr {
    pub attr_set: crate::__u64,
    pub attr_clr: crate::__u64,
    pub propagation: crate::__u64,
    pub userns_fd: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for mount_attr`

- `fn clone(self: &Self) -> mount_attr` — [`mount_attr`](../index.md)

##### `impl Copy for mount_attr`

##### `impl Debug for mount_attr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `mnt_ns_info`

```rust
struct mnt_ns_info {
    pub size: crate::__u32,
    pub nr_mounts: crate::__u32,
    pub mnt_ns_id: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for mnt_ns_info`

- `fn clone(self: &Self) -> mnt_ns_info` — [`mnt_ns_info`](../index.md)

##### `impl Copy for mnt_ns_info`

##### `impl Debug for mnt_ns_info`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pidfd_info`

```rust
struct pidfd_info {
    pub mask: crate::__u64,
    pub cgroupid: crate::__u64,
    pub pid: crate::__u32,
    pub tgid: crate::__u32,
    pub ppid: crate::__u32,
    pub ruid: crate::__u32,
    pub rgid: crate::__u32,
    pub euid: crate::__u32,
    pub egid: crate::__u32,
    pub suid: crate::__u32,
    pub sgid: crate::__u32,
    pub fsuid: crate::__u32,
    pub fsgid: crate::__u32,
    pub exit_code: crate::__s32,
}
```

#### Trait Implementations

##### `impl Clone for pidfd_info`

- `fn clone(self: &Self) -> pidfd_info` — [`pidfd_info`](../index.md)

##### `impl Copy for pidfd_info`

##### `impl Debug for pidfd_info`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `dmabuf_cmsg`

```rust
struct dmabuf_cmsg {
    pub frag_offset: crate::__u64,
    pub frag_size: crate::__u32,
    pub frag_token: crate::__u32,
    pub dmabuf_id: crate::__u32,
    pub flags: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for dmabuf_cmsg`

- `fn clone(self: &Self) -> dmabuf_cmsg` — [`dmabuf_cmsg`](../index.md)

##### `impl Copy for dmabuf_cmsg`

##### `impl Debug for dmabuf_cmsg`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `dmabuf_token`

```rust
struct dmabuf_token {
    pub token_start: crate::__u32,
    pub token_count: crate::__u32,
}
```

#### Trait Implementations

##### `impl Clone for dmabuf_token`

- `fn clone(self: &Self) -> dmabuf_token` — [`dmabuf_token`](../index.md)

##### `impl Copy for dmabuf_token`

##### `impl Debug for dmabuf_token`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr_nl`

```rust
struct sockaddr_nl {
    pub nl_family: crate::sa_family_t,
    nl_pad: crate::types::Padding<crate::c_ushort>,
    pub nl_pid: u32,
    pub nl_groups: u32,
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_nl`

- `fn clone(self: &Self) -> sockaddr_nl` — [`sockaddr_nl`](../index.md)

##### `impl Copy for sockaddr_nl`

##### `impl Debug for sockaddr_nl`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `dirent`

```rust
struct dirent {
    pub d_ino: crate::ino_t,
    pub d_off: off_t,
    pub d_reclen: crate::c_ushort,
    pub d_type: crate::c_uchar,
    pub d_name: [crate::c_char; 256],
}
```

#### Trait Implementations

##### `impl Clone for dirent`

- `fn clone(self: &Self) -> dirent` — [`dirent`](../index.md)

##### `impl Copy for dirent`

##### `impl Debug for dirent`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `dirent64`

```rust
struct dirent64 {
    pub d_ino: crate::ino64_t,
    pub d_off: off64_t,
    pub d_reclen: crate::c_ushort,
    pub d_type: crate::c_uchar,
    pub d_name: [crate::c_char; 256],
}
```

#### Trait Implementations

##### `impl Clone for dirent64`

- `fn clone(self: &Self) -> dirent64` — [`dirent64`](../index.md)

##### `impl Copy for dirent64`

##### `impl Debug for dirent64`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sockaddr_alg`

```rust
struct sockaddr_alg {
    pub salg_family: crate::sa_family_t,
    pub salg_type: [crate::c_uchar; 14],
    pub salg_feat: u32,
    pub salg_mask: u32,
    pub salg_name: [crate::c_uchar; 64],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_alg`

- `fn clone(self: &Self) -> sockaddr_alg` — [`sockaddr_alg`](../index.md)

##### `impl Copy for sockaddr_alg`

##### `impl Debug for sockaddr_alg`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uinput_setup`

```rust
struct uinput_setup {
    pub id: input_id,
    pub name: [crate::c_char; 80],
    pub ff_effects_max: __u32,
}
```

#### Trait Implementations

##### `impl Clone for uinput_setup`

- `fn clone(self: &Self) -> uinput_setup` — [`uinput_setup`](../index.md)

##### `impl Copy for uinput_setup`

##### `impl Debug for uinput_setup`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `uinput_user_dev`

```rust
struct uinput_user_dev {
    pub name: [crate::c_char; 80],
    pub id: input_id,
    pub ff_effects_max: __u32,
    pub absmax: [__s32; 64],
    pub absmin: [__s32; 64],
    pub absfuzz: [__s32; 64],
    pub absflat: [__s32; 64],
}
```

#### Trait Implementations

##### `impl Clone for uinput_user_dev`

- `fn clone(self: &Self) -> uinput_user_dev` — [`uinput_user_dev`](../index.md)

##### `impl Copy for uinput_user_dev`

##### `impl Debug for uinput_user_dev`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `mq_attr`

```rust
struct mq_attr {
    pub mq_flags: crate::c_long,
    pub mq_maxmsg: crate::c_long,
    pub mq_msgsize: crate::c_long,
    pub mq_curmsgs: crate::c_long,
    pad: [crate::c_long; 4],
}
```

#### Trait Implementations

##### `impl Clone for mq_attr`

- `fn clone(self: &Self) -> mq_attr` — [`mq_attr`](../index.md)

##### `impl Copy for mq_attr`

##### `impl Debug for mq_attr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `hwtstamp_config`

```rust
struct hwtstamp_config {
    pub flags: crate::c_int,
    pub tx_type: crate::c_int,
    pub rx_filter: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for hwtstamp_config`

- `fn clone(self: &Self) -> hwtstamp_config` — [`hwtstamp_config`](../index.md)

##### `impl Copy for hwtstamp_config`

##### `impl Debug for hwtstamp_config`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sched_attr`

```rust
struct sched_attr {
    pub size: __u32,
    pub sched_policy: __u32,
    pub sched_flags: crate::__u64,
    pub sched_nice: __s32,
    pub sched_priority: __u32,
    pub sched_runtime: crate::__u64,
    pub sched_deadline: crate::__u64,
    pub sched_period: crate::__u64,
}
```

#### Trait Implementations

##### `impl Clone for sched_attr`

- `fn clone(self: &Self) -> sched_attr` — [`sched_attr`](../index.md)

##### `impl Copy for sched_attr`

##### `impl Debug for sched_attr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pthread_cond_t`

```rust
struct pthread_cond_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_cond_t`

- `fn clone(self: &Self) -> pthread_cond_t` — [`pthread_cond_t`](../index.md)

##### `impl Copy for pthread_cond_t`

##### `impl Debug for pthread_cond_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pthread_mutex_t`

```rust
struct pthread_mutex_t {
    // [REDACTED: Private Fields]
}
```

#### Trait Implementations

##### `impl Clone for pthread_mutex_t`

- `fn clone(self: &Self) -> pthread_mutex_t` — [`pthread_mutex_t`](../index.md)

##### `impl Copy for pthread_mutex_t`

##### `impl Debug for pthread_mutex_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pthread_rwlock_t`

```rust
struct pthread_rwlock_t {
    size: [u8; 56],
}
```

#### Trait Implementations

##### `impl Clone for pthread_rwlock_t`

- `fn clone(self: &Self) -> pthread_rwlock_t` — [`pthread_rwlock_t`](../index.md)

##### `impl Copy for pthread_rwlock_t`

##### `impl Debug for pthread_rwlock_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `pthread_barrier_t`

```rust
struct pthread_barrier_t {
    size: [u8; 32],
}
```

#### Trait Implementations

##### `impl Clone for pthread_barrier_t`

- `fn clone(self: &Self) -> pthread_barrier_t` — [`pthread_barrier_t`](../index.md)

##### `impl Copy for pthread_barrier_t`

##### `impl Debug for pthread_barrier_t`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_thrspy`

```rust
struct iw_thrspy {
    pub addr: crate::sockaddr,
    pub qual: iw_quality,
    pub low: iw_quality,
    pub high: iw_quality,
}
```

#### Trait Implementations

##### `impl Clone for iw_thrspy`

- `fn clone(self: &Self) -> iw_thrspy` — [`iw_thrspy`](../index.md)

##### `impl Copy for iw_thrspy`

##### `impl Debug for iw_thrspy`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_mlme`

```rust
struct iw_mlme {
    pub cmd: __u16,
    pub reason_code: __u16,
    pub addr: crate::sockaddr,
}
```

#### Trait Implementations

##### `impl Clone for iw_mlme`

- `fn clone(self: &Self) -> iw_mlme` — [`iw_mlme`](../index.md)

##### `impl Copy for iw_mlme`

##### `impl Debug for iw_mlme`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_michaelmicfailure`

```rust
struct iw_michaelmicfailure {
    pub flags: __u32,
    pub src_addr: crate::sockaddr,
    pub tsc: [__u8; 8],
}
```

#### Trait Implementations

##### `impl Clone for iw_michaelmicfailure`

- `fn clone(self: &Self) -> iw_michaelmicfailure` — [`iw_michaelmicfailure`](../index.md)

##### `impl Copy for iw_michaelmicfailure`

##### `impl Debug for iw_michaelmicfailure`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__c_anonymous_elf32_rela`

```rust
struct __c_anonymous_elf32_rela {
    pub r_offset: Elf32_Addr,
    pub r_info: Elf32_Word,
    pub r_addend: Elf32_Sword,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_elf32_rela`

- `fn clone(self: &Self) -> __c_anonymous_elf32_rela` — [`__c_anonymous_elf32_rela`](../index.md)

##### `impl Copy for __c_anonymous_elf32_rela`

##### `impl Debug for __c_anonymous_elf32_rela`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `__c_anonymous_elf64_rela`

```rust
struct __c_anonymous_elf64_rela {
    pub r_offset: Elf64_Addr,
    pub r_info: Elf64_Xword,
    pub r_addend: Elf64_Sxword,
}
```

#### Trait Implementations

##### `impl Clone for __c_anonymous_elf64_rela`

- `fn clone(self: &Self) -> __c_anonymous_elf64_rela` — [`__c_anonymous_elf64_rela`](../index.md)

##### `impl Copy for __c_anonymous_elf64_rela`

##### `impl Debug for __c_anonymous_elf64_rela`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `af_alg_iv`

```rust
struct af_alg_iv {
    pub ivlen: u32,
    pub iv: [crate::c_uchar; 0],
}
```

WARNING: The `PartialEq`, `Eq` and `Hash` implementations of this
type are unsound and will be removed in the future.

#### Trait Implementations

##### `impl Clone for af_alg_iv`

- `fn clone(self: &Self) -> af_alg_iv` — [`af_alg_iv`](../index.md)

##### `impl Copy for af_alg_iv`

##### `impl Debug for af_alg_iv`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ifreq`

```rust
struct ifreq {
    pub ifr_name: [crate::c_char; 16],
    pub ifr_ifru: __c_anonymous_ifr_ifru,
}
```

#### Fields

- **`ifr_name`**: `[crate::c_char; 16]`

  interface name, e.g. "en0"

#### Trait Implementations

##### `impl Clone for ifreq`

- `fn clone(self: &Self) -> ifreq` — [`ifreq`](../index.md)

##### `impl Copy for ifreq`

##### `impl Debug for ifreq`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ifconf`

```rust
struct ifconf {
    pub ifc_len: crate::c_int,
    pub ifc_ifcu: __c_anonymous_ifc_ifcu,
}
```

Structure used in SIOCGIFCONF request.  Used to retrieve interface configuration for
machine (useful for programs which must know all networks accessible).

#### Fields

- **`ifc_len`**: `crate::c_int`

  Size of buffer

#### Trait Implementations

##### `impl Clone for ifconf`

- `fn clone(self: &Self) -> ifconf` — [`ifconf`](../index.md)

##### `impl Copy for ifconf`

##### `impl Debug for ifconf`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `tpacket_block_desc`

```rust
struct tpacket_block_desc {
    pub version: __u32,
    pub offset_to_priv: __u32,
    pub hdr: crate::tpacket_bd_header_u,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_block_desc`

- `fn clone(self: &Self) -> tpacket_block_desc` — [`tpacket_block_desc`](../index.md)

##### `impl Copy for tpacket_block_desc`

##### `impl Debug for tpacket_block_desc`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `sock_txtime`

```rust
struct sock_txtime {
    pub clockid: crate::clockid_t,
    pub flags: __u32,
}
```

#### Trait Implementations

##### `impl Clone for sock_txtime`

- `fn clone(self: &Self) -> sock_txtime` — [`sock_txtime`](../index.md)

##### `impl Copy for sock_txtime`

##### `impl Debug for sock_txtime`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iw_event`

```rust
struct iw_event {
    pub len: __u16,
    pub cmd: __u16,
    pub u: iwreq_data,
}
```

#### Trait Implementations

##### `impl Clone for iw_event`

- `fn clone(self: &Self) -> iw_event` — [`iw_event`](../index.md)

##### `impl Copy for iw_event`

##### `impl Debug for iw_event`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `iwreq`

```rust
struct iwreq {
    pub ifr_ifrn: __c_anonymous_iwreq,
    pub u: iwreq_data,
}
```

#### Trait Implementations

##### `impl Clone for iwreq`

- `fn clone(self: &Self) -> iwreq` — [`iwreq`](../index.md)

##### `impl Copy for iwreq`

##### `impl Debug for iwreq`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ptp_perout_request`

```rust
struct ptp_perout_request {
    pub anonymous_1: __c_anonymous_ptp_perout_request_1,
    pub period: ptp_clock_time,
    pub index: crate::c_uint,
    pub flags: crate::c_uint,
    pub anonymous_2: __c_anonymous_ptp_perout_request_2,
}
```

#### Trait Implementations

##### `impl Clone for ptp_perout_request`

- `fn clone(self: &Self) -> ptp_perout_request` — [`ptp_perout_request`](../index.md)

##### `impl Copy for ptp_perout_request`

##### `impl Debug for ptp_perout_request`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `xsk_tx_metadata`

```rust
struct xsk_tx_metadata {
    pub flags: crate::__u64,
    pub xsk_tx_metadata_union: __c_anonymous_xsk_tx_metadata_union,
}
```

#### Trait Implementations

##### `impl Clone for xsk_tx_metadata`

- `fn clone(self: &Self) -> xsk_tx_metadata` — [`xsk_tx_metadata`](../index.md)

##### `impl Copy for xsk_tx_metadata`

##### `impl Debug for xsk_tx_metadata`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn clone(self: &Self) -> aiocb` — [`aiocb`](#aiocb)

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

- `fn clone(self: &Self) -> __exit_status` — [`__exit_status`](#exit-status)

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

- `fn clone(self: &Self) -> __timeval` — [`__timeval`](#timeval)

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

- `fn clone(self: &Self) -> glob64_t` — [`glob64_t`](#glob64-t)

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

- `fn clone(self: &Self) -> msghdr` — [`msghdr`](#msghdr)

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

- `fn clone(self: &Self) -> cmsghdr` — [`cmsghdr`](#cmsghdr)

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

- `fn clone(self: &Self) -> termios` — [`termios`](#termios)

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

- `fn clone(self: &Self) -> mallinfo` — [`mallinfo`](#mallinfo)

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

- `fn clone(self: &Self) -> mallinfo2` — [`mallinfo2`](#mallinfo2)

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

- `fn clone(self: &Self) -> nl_pktinfo` — [`nl_pktinfo`](#nl-pktinfo)

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

- `fn clone(self: &Self) -> nl_mmap_req` — [`nl_mmap_req`](#nl-mmap-req)

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

- `fn clone(self: &Self) -> nl_mmap_hdr` — [`nl_mmap_hdr`](#nl-mmap-hdr)

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

- `fn clone(self: &Self) -> ntptimeval` — [`ntptimeval`](#ntptimeval)

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

- `fn clone(self: &Self) -> regex_t` — [`regex_t`](#regex-t)

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

- `fn clone(self: &Self) -> Elf64_Chdr` — [`Elf64_Chdr`](#elf64-chdr)

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

- `fn clone(self: &Self) -> Elf32_Chdr` — [`Elf32_Chdr`](#elf32-chdr)

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

- `fn clone(self: &Self) -> seminfo` — [`seminfo`](#seminfo)

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

- `fn clone(self: &Self) -> ptrace_peeksiginfo_args` — [`ptrace_peeksiginfo_args`](#ptrace-peeksiginfo-args)

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

- `fn clone(self: &Self) -> __c_anonymous_ptrace_syscall_info_entry` — [`__c_anonymous_ptrace_syscall_info_entry`](#c-anonymous-ptrace-syscall-info-entry)

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

- `fn clone(self: &Self) -> __c_anonymous_ptrace_syscall_info_exit` — [`__c_anonymous_ptrace_syscall_info_exit`](#c-anonymous-ptrace-syscall-info-exit)

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

- `fn clone(self: &Self) -> __c_anonymous_ptrace_syscall_info_seccomp` — [`__c_anonymous_ptrace_syscall_info_seccomp`](#c-anonymous-ptrace-syscall-info-seccomp)

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

- `fn clone(self: &Self) -> ptrace_syscall_info` — [`ptrace_syscall_info`](#ptrace-syscall-info)

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

- `fn clone(self: &Self) -> ptrace_sud_config` — [`ptrace_sud_config`](#ptrace-sud-config)

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

- `fn clone(self: &Self) -> iocb` — [`iocb`](#iocb)

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

- `fn clone(self: &Self) -> tcp_info` — [`tcp_info`](#tcp-info)

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

- `fn clone(self: &Self) -> fanotify_event_info_pidfd` — [`fanotify_event_info_pidfd`](#fanotify-event-info-pidfd)

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

- `fn clone(self: &Self) -> fanotify_event_info_error` — [`fanotify_event_info_error`](#fanotify-event-info-error)

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

- `fn clone(self: &Self) -> sem_t` — [`sem_t`](#sem-t)

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

- `fn clone(self: &Self) -> mbstate_t` — [`mbstate_t`](#mbstate-t)

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

- `fn clone(self: &Self) -> fpos64_t` — [`fpos64_t`](#fpos64-t)

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

- `fn clone(self: &Self) -> fpos_t` — [`fpos_t`](#fpos-t)

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

- `fn clone(self: &Self) -> timespec` — [`timespec`](#timespec)

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

- `fn clone(self: &Self) -> utmpx` — [`utmpx`](#utmpx)

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

- `fn clone(self: &Self) -> sifields_sigchld` — [`sifields_sigchld`](gnu/index.md)

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

- `fn clone(self: &Self) -> siginfo_f` — [`siginfo_f`](gnu/index.md)

##### `impl Copy for siginfo_f`

##### `impl Debug for siginfo_f`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Enums

### `tpacket_versions`

```rust
enum tpacket_versions {
    TPACKET_V1,
    TPACKET_V2,
    TPACKET_V3,
}
```

#### Trait Implementations

##### `impl Clone for tpacket_versions`

- `fn clone(self: &Self) -> tpacket_versions` — [`tpacket_versions`](../index.md)

##### `impl Copy for tpacket_versions`

##### `impl Debug for tpacket_versions`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `iopl`

```rust
unsafe fn iopl(level: c_int) -> c_int
```

### `ioperm`

```rust
unsafe fn ioperm(from: c_ulong, num: c_ulong, turn_on: c_int) -> c_int
```

### `aio_read`

```rust
unsafe fn aio_read(aiocbp: *mut aiocb) -> c_int
```

### `aio_write`

```rust
unsafe fn aio_write(aiocbp: *mut aiocb) -> c_int
```

### `aio_fsync`

```rust
unsafe fn aio_fsync(op: c_int, aiocbp: *mut aiocb) -> c_int
```

### `aio_error`

```rust
unsafe fn aio_error(aiocbp: *const aiocb) -> c_int
```

### `aio_return`

```rust
unsafe fn aio_return(aiocbp: *mut aiocb) -> ssize_t
```

### `aio_suspend`

```rust
unsafe fn aio_suspend(aiocb_list: *const *const aiocb, nitems: c_int, timeout: *const crate::timespec) -> c_int
```

### `aio_cancel`

```rust
unsafe fn aio_cancel(fd: c_int, aiocbp: *mut aiocb) -> c_int
```

### `lio_listio`

```rust
unsafe fn lio_listio(mode: c_int, aiocb_list: *const *mut aiocb, nitems: c_int, sevp: *mut crate::sigevent) -> c_int
```

### `pwritev`

```rust
unsafe fn pwritev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t) -> ssize_t
```

### `preadv`

```rust
unsafe fn preadv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t) -> ssize_t
```

### `getnameinfo`

```rust
unsafe fn getnameinfo(sa: *const crate::sockaddr, salen: crate::socklen_t, host: *mut c_char, hostlen: crate::socklen_t, serv: *mut c_char, servlen: crate::socklen_t, flags: c_int) -> c_int
```

### `getloadavg`

```rust
unsafe fn getloadavg(loadavg: *mut c_double, nelem: c_int) -> c_int
```

### `process_vm_readv`

```rust
unsafe fn process_vm_readv(pid: crate::pid_t, local_iov: *const crate::iovec, liovcnt: c_ulong, remote_iov: *const crate::iovec, riovcnt: c_ulong, flags: c_ulong) -> isize
```

### `process_vm_writev`

```rust
unsafe fn process_vm_writev(pid: crate::pid_t, local_iov: *const crate::iovec, liovcnt: c_ulong, remote_iov: *const crate::iovec, riovcnt: c_ulong, flags: c_ulong) -> isize
```

### `futimes`

```rust
unsafe fn futimes(fd: c_int, times: *const crate::timeval) -> c_int
```

### `getspnam_r`

```rust
unsafe fn getspnam_r(name: *const c_char, spbuf: *mut spwd, buf: *mut c_char, buflen: size_t, spbufp: *mut *mut spwd) -> c_int
```

### `mq_open`

```rust
unsafe fn mq_open(name: *const c_char, oflag: c_int) -> crate::mqd_t
```

### `mq_close`

```rust
unsafe fn mq_close(mqd: crate::mqd_t) -> c_int
```

### `mq_unlink`

```rust
unsafe fn mq_unlink(name: *const c_char) -> c_int
```

### `mq_receive`

```rust
unsafe fn mq_receive(mqd: crate::mqd_t, msg_ptr: *mut c_char, msg_len: size_t, msg_prio: *mut c_uint) -> ssize_t
```

### `mq_timedreceive`

```rust
unsafe fn mq_timedreceive(mqd: crate::mqd_t, msg_ptr: *mut c_char, msg_len: size_t, msg_prio: *mut c_uint, abs_timeout: *const crate::timespec) -> ssize_t
```

### `mq_send`

```rust
unsafe fn mq_send(mqd: crate::mqd_t, msg_ptr: *const c_char, msg_len: size_t, msg_prio: c_uint) -> c_int
```

### `mq_timedsend`

```rust
unsafe fn mq_timedsend(mqd: crate::mqd_t, msg_ptr: *const c_char, msg_len: size_t, msg_prio: c_uint, abs_timeout: *const crate::timespec) -> c_int
```

### `mq_getattr`

```rust
unsafe fn mq_getattr(mqd: crate::mqd_t, attr: *mut crate::mq_attr) -> c_int
```

### `mq_setattr`

```rust
unsafe fn mq_setattr(mqd: crate::mqd_t, newattr: *const crate::mq_attr, oldattr: *mut crate::mq_attr) -> c_int
```

### `strerror_r`

```rust
unsafe fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```

### `abs`

```rust
unsafe fn abs(i: c_int) -> c_int
```

### `labs`

```rust
unsafe fn labs(i: c_long) -> c_long
```

### `rand`

```rust
unsafe fn rand() -> c_int
```

### `srand`

```rust
unsafe fn srand(seed: c_uint)
```

### `drand48`

```rust
unsafe fn drand48() -> c_double
```

### `erand48`

```rust
unsafe fn erand48(xseed: *mut c_ushort) -> c_double
```

### `lrand48`

```rust
unsafe fn lrand48() -> c_long
```

### `nrand48`

```rust
unsafe fn nrand48(xseed: *mut c_ushort) -> c_long
```

### `mrand48`

```rust
unsafe fn mrand48() -> c_long
```

### `jrand48`

```rust
unsafe fn jrand48(xseed: *mut c_ushort) -> c_long
```

### `srand48`

```rust
unsafe fn srand48(seed: c_long)
```

### `seed48`

```rust
unsafe fn seed48(xseed: *mut c_ushort) -> *mut c_ushort
```

### `lcong48`

```rust
unsafe fn lcong48(p: *mut c_ushort)
```

### `lutimes`

```rust
unsafe fn lutimes(file: *const c_char, times: *const crate::timeval) -> c_int
```

### `setpwent`

```rust
unsafe fn setpwent()
```

### `endpwent`

```rust
unsafe fn endpwent()
```

### `getpwent`

```rust
unsafe fn getpwent() -> *mut passwd
```

### `setgrent`

```rust
unsafe fn setgrent()
```

### `endgrent`

```rust
unsafe fn endgrent()
```

### `getgrent`

```rust
unsafe fn getgrent() -> *mut crate::group
```

### `setspent`

```rust
unsafe fn setspent()
```

### `endspent`

```rust
unsafe fn endspent()
```

### `getspent`

```rust
unsafe fn getspent() -> *mut spwd
```

### `getspnam`

```rust
unsafe fn getspnam(name: *const c_char) -> *mut spwd
```

### `shm_open`

```rust
unsafe fn shm_open(name: *const c_char, oflag: c_int, mode: mode_t) -> c_int
```

### `shm_unlink`

```rust
unsafe fn shm_unlink(name: *const c_char) -> c_int
```

### `shmget`

```rust
unsafe fn shmget(key: crate::key_t, size: size_t, shmflg: c_int) -> c_int
```

### `shmat`

```rust
unsafe fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void
```

### `shmdt`

```rust
unsafe fn shmdt(shmaddr: *const c_void) -> c_int
```

### `shmctl`

```rust
unsafe fn shmctl(shmid: c_int, cmd: c_int, buf: *mut crate::shmid_ds) -> c_int
```

### `ftok`

```rust
unsafe fn ftok(pathname: *const c_char, proj_id: c_int) -> crate::key_t
```

### `semget`

```rust
unsafe fn semget(key: crate::key_t, nsems: c_int, semflag: c_int) -> c_int
```

### `semop`

```rust
unsafe fn semop(semid: c_int, sops: *mut crate::sembuf, nsops: size_t) -> c_int
```

### `semctl`

```rust
unsafe fn semctl(semid: c_int, semnum: c_int, cmd: c_int) -> c_int
```

### `msgctl`

```rust
unsafe fn msgctl(msqid: c_int, cmd: c_int, buf: *mut msqid_ds) -> c_int
```

### `msgget`

```rust
unsafe fn msgget(key: crate::key_t, msgflg: c_int) -> c_int
```

### `msgrcv`

```rust
unsafe fn msgrcv(msqid: c_int, msgp: *mut c_void, msgsz: size_t, msgtyp: c_long, msgflg: c_int) -> ssize_t
```

### `msgsnd`

```rust
unsafe fn msgsnd(msqid: c_int, msgp: *const c_void, msgsz: size_t, msgflg: c_int) -> c_int
```

### `mprotect`

```rust
unsafe fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int
```

### `__errno_location`

```rust
unsafe fn __errno_location() -> *mut c_int
```

### `fallocate`

```rust
unsafe fn fallocate(fd: c_int, mode: c_int, offset: off_t, len: off_t) -> c_int
```

### `posix_fallocate`

```rust
unsafe fn posix_fallocate(fd: c_int, offset: off_t, len: off_t) -> c_int
```

### `readahead`

```rust
unsafe fn readahead(fd: c_int, offset: off64_t, count: size_t) -> ssize_t
```

### `getxattr`

```rust
unsafe fn getxattr(path: *const c_char, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t
```

### `lgetxattr`

```rust
unsafe fn lgetxattr(path: *const c_char, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t
```

### `fgetxattr`

```rust
unsafe fn fgetxattr(filedes: c_int, name: *const c_char, value: *mut c_void, size: size_t) -> ssize_t
```

### `setxattr`

```rust
unsafe fn setxattr(path: *const c_char, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int
```

### `lsetxattr`

```rust
unsafe fn lsetxattr(path: *const c_char, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int
```

### `fsetxattr`

```rust
unsafe fn fsetxattr(filedes: c_int, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> c_int
```

### `listxattr`

```rust
unsafe fn listxattr(path: *const c_char, list: *mut c_char, size: size_t) -> ssize_t
```

### `llistxattr`

```rust
unsafe fn llistxattr(path: *const c_char, list: *mut c_char, size: size_t) -> ssize_t
```

### `flistxattr`

```rust
unsafe fn flistxattr(filedes: c_int, list: *mut c_char, size: size_t) -> ssize_t
```

### `removexattr`

```rust
unsafe fn removexattr(path: *const c_char, name: *const c_char) -> c_int
```

### `lremovexattr`

```rust
unsafe fn lremovexattr(path: *const c_char, name: *const c_char) -> c_int
```

### `fremovexattr`

```rust
unsafe fn fremovexattr(filedes: c_int, name: *const c_char) -> c_int
```

### `signalfd`

```rust
unsafe fn signalfd(fd: c_int, mask: *const crate::sigset_t, flags: c_int) -> c_int
```

### `timerfd_create`

```rust
unsafe fn timerfd_create(clockid: crate::clockid_t, flags: c_int) -> c_int
```

### `timerfd_gettime`

```rust
unsafe fn timerfd_gettime(fd: c_int, curr_value: *mut itimerspec) -> c_int
```

### `timerfd_settime`

```rust
unsafe fn timerfd_settime(fd: c_int, flags: c_int, new_value: *const itimerspec, old_value: *mut itimerspec) -> c_int
```

### `quotactl`

```rust
unsafe fn quotactl(cmd: c_int, special: *const c_char, id: c_int, data: *mut c_char) -> c_int
```

### `epoll_pwait`

```rust
unsafe fn epoll_pwait(epfd: c_int, events: *mut crate::epoll_event, maxevents: c_int, timeout: c_int, sigmask: *const crate::sigset_t) -> c_int
```

### `dup3`

```rust
unsafe fn dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int
```

### `sigtimedwait`

```rust
unsafe fn sigtimedwait(set: *const sigset_t, info: *mut siginfo_t, timeout: *const crate::timespec) -> c_int
```

### `sigwaitinfo`

```rust
unsafe fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> c_int
```

### `nl_langinfo_l`

```rust
unsafe fn nl_langinfo_l(item: crate::nl_item, locale: crate::locale_t) -> *mut c_char
```

### `accept4`

```rust
unsafe fn accept4(fd: c_int, addr: *mut crate::sockaddr, len: *mut crate::socklen_t, flg: c_int) -> c_int
```

### `reboot`

```rust
unsafe fn reboot(how_to: c_int) -> c_int
```

### `setfsgid`

```rust
unsafe fn setfsgid(gid: crate::gid_t) -> c_int
```

### `setfsuid`

```rust
unsafe fn setfsuid(uid: crate::uid_t) -> c_int
```

### `mkfifoat`

```rust
unsafe fn mkfifoat(dirfd: c_int, pathname: *const c_char, mode: mode_t) -> c_int
```

### `if_nameindex`

```rust
unsafe fn if_nameindex() -> *mut if_nameindex
```

### `if_freenameindex`

```rust
unsafe fn if_freenameindex(ptr: *mut if_nameindex)
```

### `sync_file_range`

```rust
unsafe fn sync_file_range(fd: c_int, offset: off64_t, nbytes: off64_t, flags: c_uint) -> c_int
```

### `mremap`

```rust
unsafe fn mremap(addr: *mut c_void, len: size_t, new_len: size_t, flags: c_int) -> *mut c_void
```

### `glob`

```rust
unsafe fn glob(pattern: *const c_char, flags: c_int, errfunc: Option<fn(*const c_char, c_int) -> c_int>, pglob: *mut crate::glob_t) -> c_int
```

### `globfree`

```rust
unsafe fn globfree(pglob: *mut crate::glob_t)
```

### `posix_madvise`

```rust
unsafe fn posix_madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int
```

### `seekdir`

```rust
unsafe fn seekdir(dirp: *mut crate::DIR, loc: c_long)
```

### `telldir`

```rust
unsafe fn telldir(dirp: *mut crate::DIR) -> c_long
```

### `madvise`

```rust
unsafe fn madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int
```

### `msync`

```rust
unsafe fn msync(addr: *mut c_void, len: size_t, flags: c_int) -> c_int
```

### `remap_file_pages`

```rust
unsafe fn remap_file_pages(addr: *mut c_void, size: size_t, prot: c_int, pgoff: size_t, flags: c_int) -> c_int
```

### `recvfrom`

```rust
unsafe fn recvfrom(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int, addr: *mut crate::sockaddr, addrlen: *mut crate::socklen_t) -> ssize_t
```

### `mkstemps`

```rust
unsafe fn mkstemps(template: *mut c_char, suffixlen: c_int) -> c_int
```

### `nl_langinfo`

```rust
unsafe fn nl_langinfo(item: crate::nl_item) -> *mut c_char
```

### `vhangup`

```rust
unsafe fn vhangup() -> c_int
```

### `sync`

```rust
unsafe fn sync()
```

### `syncfs`

```rust
unsafe fn syncfs(fd: c_int) -> c_int
```

### `syscall`

```rust
unsafe fn syscall(num: c_long) -> c_long
```

### `sched_getaffinity`

```rust
unsafe fn sched_getaffinity(pid: crate::pid_t, cpusetsize: size_t, cpuset: *mut cpu_set_t) -> c_int
```

### `sched_setaffinity`

```rust
unsafe fn sched_setaffinity(pid: crate::pid_t, cpusetsize: size_t, cpuset: *const cpu_set_t) -> c_int
```

### `epoll_create`

```rust
unsafe fn epoll_create(size: c_int) -> c_int
```

### `epoll_create1`

```rust
unsafe fn epoll_create1(flags: c_int) -> c_int
```

### `epoll_wait`

```rust
unsafe fn epoll_wait(epfd: c_int, events: *mut crate::epoll_event, maxevents: c_int, timeout: c_int) -> c_int
```

### `epoll_ctl`

```rust
unsafe fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut crate::epoll_event) -> c_int
```

### `unshare`

```rust
unsafe fn unshare(flags: c_int) -> c_int
```

### `umount`

```rust
unsafe fn umount(target: *const c_char) -> c_int
```

### `sched_get_priority_max`

```rust
unsafe fn sched_get_priority_max(policy: c_int) -> c_int
```

### `tee`

```rust
unsafe fn tee(fd_in: c_int, fd_out: c_int, len: size_t, flags: c_uint) -> ssize_t
```

### `settimeofday`

```rust
unsafe fn settimeofday(tv: *const crate::timeval, tz: *const crate::timezone) -> c_int
```

### `splice`

```rust
unsafe fn splice(fd_in: c_int, off_in: *mut crate::loff_t, fd_out: c_int, off_out: *mut crate::loff_t, len: size_t, flags: c_uint) -> ssize_t
```

### `eventfd`

```rust
unsafe fn eventfd(initval: c_uint, flags: c_int) -> c_int
```

### `eventfd_read`

```rust
unsafe fn eventfd_read(fd: c_int, value: *mut eventfd_t) -> c_int
```

### `eventfd_write`

```rust
unsafe fn eventfd_write(fd: c_int, value: eventfd_t) -> c_int
```

### `sched_rr_get_interval`

```rust
unsafe fn sched_rr_get_interval(pid: crate::pid_t, tp: *mut crate::timespec) -> c_int
```

### `sem_timedwait`

```rust
unsafe fn sem_timedwait(sem: *mut sem_t, abstime: *const crate::timespec) -> c_int
```

### `sem_getvalue`

```rust
unsafe fn sem_getvalue(sem: *mut sem_t, sval: *mut c_int) -> c_int
```

### `sched_setparam`

```rust
unsafe fn sched_setparam(pid: crate::pid_t, param: *const crate::sched_param) -> c_int
```

### `setns`

```rust
unsafe fn setns(fd: c_int, nstype: c_int) -> c_int
```

### `swapoff`

```rust
unsafe fn swapoff(path: *const c_char) -> c_int
```

### `vmsplice`

```rust
unsafe fn vmsplice(fd: c_int, iov: *const crate::iovec, nr_segs: size_t, flags: c_uint) -> ssize_t
```

### `mount`

```rust
unsafe fn mount(src: *const c_char, target: *const c_char, fstype: *const c_char, flags: c_ulong, data: *const c_void) -> c_int
```

### `personality`

```rust
unsafe fn personality(persona: c_ulong) -> c_int
```

### `prctl`

```rust
unsafe fn prctl(option: c_int) -> c_int
```

### `sched_getparam`

```rust
unsafe fn sched_getparam(pid: crate::pid_t, param: *mut crate::sched_param) -> c_int
```

### `ppoll`

```rust
unsafe fn ppoll(fds: *mut crate::pollfd, nfds: nfds_t, timeout: *const crate::timespec, sigmask: *const sigset_t) -> c_int
```

### `clone`

```rust
unsafe fn clone(cb: fn(*mut c_void) -> c_int, child_stack: *mut c_void, flags: c_int, arg: *mut c_void) -> c_int
```

### `sched_getscheduler`

```rust
unsafe fn sched_getscheduler(pid: crate::pid_t) -> c_int
```

### `clock_nanosleep`

```rust
unsafe fn clock_nanosleep(clk_id: crate::clockid_t, flags: c_int, rqtp: *const crate::timespec, rmtp: *mut crate::timespec) -> c_int
```

### `sethostname`

```rust
unsafe fn sethostname(name: *const c_char, len: size_t) -> c_int
```

### `sched_get_priority_min`

```rust
unsafe fn sched_get_priority_min(policy: c_int) -> c_int
```

### `sysinfo`

```rust
unsafe fn sysinfo(info: *mut crate::sysinfo) -> c_int
```

### `umount2`

```rust
unsafe fn umount2(target: *const c_char, flags: c_int) -> c_int
```

### `swapon`

```rust
unsafe fn swapon(path: *const c_char, swapflags: c_int) -> c_int
```

### `sched_setscheduler`

```rust
unsafe fn sched_setscheduler(pid: crate::pid_t, policy: c_int, param: *const crate::sched_param) -> c_int
```

### `sendfile`

```rust
unsafe fn sendfile(out_fd: c_int, in_fd: c_int, offset: *mut off_t, count: size_t) -> ssize_t
```

### `sigsuspend`

```rust
unsafe fn sigsuspend(mask: *const crate::sigset_t) -> c_int
```

### `getgrgid_r`

```rust
unsafe fn getgrgid_r(gid: crate::gid_t, grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```

### `sigaltstack`

```rust
unsafe fn sigaltstack(ss: *const stack_t, oss: *mut stack_t) -> c_int
```

### `sem_close`

```rust
unsafe fn sem_close(sem: *mut sem_t) -> c_int
```

### `getdtablesize`

```rust
unsafe fn getdtablesize() -> c_int
```

### `getgrnam_r`

```rust
unsafe fn getgrnam_r(name: *const c_char, grp: *mut crate::group, buf: *mut c_char, buflen: size_t, result: *mut *mut crate::group) -> c_int
```

### `initgroups`

```rust
unsafe fn initgroups(user: *const c_char, group: crate::gid_t) -> c_int
```

### `sem_open`

```rust
unsafe fn sem_open(name: *const c_char, oflag: c_int) -> *mut sem_t
```

### `getgrnam`

```rust
unsafe fn getgrnam(name: *const c_char) -> *mut crate::group
```

### `sem_unlink`

```rust
unsafe fn sem_unlink(name: *const c_char) -> c_int
```

### `daemon`

```rust
unsafe fn daemon(nochdir: c_int, noclose: c_int) -> c_int
```

### `getpwnam_r`

```rust
unsafe fn getpwnam_r(name: *const c_char, pwd: *mut passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut passwd) -> c_int
```

### `getpwuid_r`

```rust
unsafe fn getpwuid_r(uid: crate::uid_t, pwd: *mut passwd, buf: *mut c_char, buflen: size_t, result: *mut *mut passwd) -> c_int
```

### `sigwait`

```rust
unsafe fn sigwait(set: *const sigset_t, sig: *mut c_int) -> c_int
```

### `getgrgid`

```rust
unsafe fn getgrgid(gid: crate::gid_t) -> *mut crate::group
```

### `getgrouplist`

```rust
unsafe fn getgrouplist(user: *const c_char, group: crate::gid_t, groups: *mut crate::gid_t, ngroups: *mut c_int) -> c_int
```

### `popen`

```rust
unsafe fn popen(command: *const c_char, mode: *const c_char) -> *mut crate::FILE
```

### `faccessat`

```rust
unsafe fn faccessat(dirfd: c_int, pathname: *const c_char, mode: c_int, flags: c_int) -> c_int
```

### `dl_iterate_phdr`

```rust
unsafe fn dl_iterate_phdr(callback: Option<fn(*mut crate::dl_phdr_info, size_t, *mut c_void) -> c_int>, data: *mut c_void) -> c_int
```

### `setmntent`

```rust
unsafe fn setmntent(filename: *const c_char, ty: *const c_char) -> *mut crate::FILE
```

### `getmntent`

```rust
unsafe fn getmntent(stream: *mut crate::FILE) -> *mut crate::mntent
```

### `addmntent`

```rust
unsafe fn addmntent(stream: *mut crate::FILE, mnt: *const crate::mntent) -> c_int
```

### `endmntent`

```rust
unsafe fn endmntent(streamp: *mut crate::FILE) -> c_int
```

### `hasmntopt`

```rust
unsafe fn hasmntopt(mnt: *const crate::mntent, opt: *const c_char) -> *mut c_char
```

### `posix_spawn`

```rust
unsafe fn posix_spawn(pid: *mut crate::pid_t, path: *const c_char, file_actions: *const crate::posix_spawn_file_actions_t, attrp: *const crate::posix_spawnattr_t, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int
```

### `posix_spawnp`

```rust
unsafe fn posix_spawnp(pid: *mut crate::pid_t, file: *const c_char, file_actions: *const crate::posix_spawn_file_actions_t, attrp: *const crate::posix_spawnattr_t, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int
```

### `posix_spawnattr_init`

```rust
unsafe fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> c_int
```

### `posix_spawnattr_destroy`

```rust
unsafe fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> c_int
```

### `posix_spawnattr_getsigdefault`

```rust
unsafe fn posix_spawnattr_getsigdefault(attr: *const posix_spawnattr_t, default: *mut crate::sigset_t) -> c_int
```

### `posix_spawnattr_setsigdefault`

```rust
unsafe fn posix_spawnattr_setsigdefault(attr: *mut posix_spawnattr_t, default: *const crate::sigset_t) -> c_int
```

### `posix_spawnattr_getsigmask`

```rust
unsafe fn posix_spawnattr_getsigmask(attr: *const posix_spawnattr_t, default: *mut crate::sigset_t) -> c_int
```

### `posix_spawnattr_setsigmask`

```rust
unsafe fn posix_spawnattr_setsigmask(attr: *mut posix_spawnattr_t, default: *const crate::sigset_t) -> c_int
```

### `posix_spawnattr_getflags`

```rust
unsafe fn posix_spawnattr_getflags(attr: *const posix_spawnattr_t, flags: *mut c_short) -> c_int
```

### `posix_spawnattr_setflags`

```rust
unsafe fn posix_spawnattr_setflags(attr: *mut posix_spawnattr_t, flags: c_short) -> c_int
```

### `posix_spawnattr_getpgroup`

```rust
unsafe fn posix_spawnattr_getpgroup(attr: *const posix_spawnattr_t, flags: *mut crate::pid_t) -> c_int
```

### `posix_spawnattr_setpgroup`

```rust
unsafe fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t, flags: crate::pid_t) -> c_int
```

### `posix_spawnattr_getschedpolicy`

```rust
unsafe fn posix_spawnattr_getschedpolicy(attr: *const posix_spawnattr_t, flags: *mut c_int) -> c_int
```

### `posix_spawnattr_setschedpolicy`

```rust
unsafe fn posix_spawnattr_setschedpolicy(attr: *mut posix_spawnattr_t, flags: c_int) -> c_int
```

### `posix_spawnattr_getschedparam`

```rust
unsafe fn posix_spawnattr_getschedparam(attr: *const posix_spawnattr_t, param: *mut crate::sched_param) -> c_int
```

### `posix_spawnattr_setschedparam`

```rust
unsafe fn posix_spawnattr_setschedparam(attr: *mut posix_spawnattr_t, param: *const crate::sched_param) -> c_int
```

### `posix_spawn_file_actions_init`

```rust
unsafe fn posix_spawn_file_actions_init(actions: *mut posix_spawn_file_actions_t) -> c_int
```

### `posix_spawn_file_actions_destroy`

```rust
unsafe fn posix_spawn_file_actions_destroy(actions: *mut posix_spawn_file_actions_t) -> c_int
```

### `posix_spawn_file_actions_addopen`

```rust
unsafe fn posix_spawn_file_actions_addopen(actions: *mut posix_spawn_file_actions_t, fd: c_int, path: *const c_char, oflag: c_int, mode: mode_t) -> c_int
```

### `posix_spawn_file_actions_addclose`

```rust
unsafe fn posix_spawn_file_actions_addclose(actions: *mut posix_spawn_file_actions_t, fd: c_int) -> c_int
```

### `posix_spawn_file_actions_adddup2`

```rust
unsafe fn posix_spawn_file_actions_adddup2(actions: *mut posix_spawn_file_actions_t, fd: c_int, newfd: c_int) -> c_int
```

### `fread_unlocked`

```rust
unsafe fn fread_unlocked(buf: *mut c_void, size: size_t, nobj: size_t, stream: *mut crate::FILE) -> size_t
```

### `inotify_rm_watch`

```rust
unsafe fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int
```

### `inotify_init`

```rust
unsafe fn inotify_init() -> c_int
```

### `inotify_init1`

```rust
unsafe fn inotify_init1(flags: c_int) -> c_int
```

### `inotify_add_watch`

```rust
unsafe fn inotify_add_watch(fd: c_int, path: *const c_char, mask: u32) -> c_int
```

### `fanotify_init`

```rust
unsafe fn fanotify_init(flags: c_uint, event_f_flags: c_uint) -> c_int
```

### `regcomp`

```rust
unsafe fn regcomp(preg: *mut crate::regex_t, pattern: *const c_char, cflags: c_int) -> c_int
```

### `regexec`

```rust
unsafe fn regexec(preg: *const crate::regex_t, input: *const c_char, nmatch: size_t, pmatch: *mut regmatch_t, eflags: c_int) -> c_int
```

### `regerror`

```rust
unsafe fn regerror(errcode: c_int, preg: *const crate::regex_t, errbuf: *mut c_char, errbuf_size: size_t) -> size_t
```

### `regfree`

```rust
unsafe fn regfree(preg: *mut crate::regex_t)
```

### `iconv_open`

```rust
unsafe fn iconv_open(tocode: *const c_char, fromcode: *const c_char) -> iconv_t
```

### `iconv`

```rust
unsafe fn iconv(cd: iconv_t, inbuf: *mut *mut c_char, inbytesleft: *mut size_t, outbuf: *mut *mut c_char, outbytesleft: *mut size_t) -> size_t
```

### `iconv_close`

```rust
unsafe fn iconv_close(cd: iconv_t) -> c_int
```

### `gettid`

```rust
unsafe fn gettid() -> crate::pid_t
```

### `timer_create`

```rust
unsafe fn timer_create(clockid: crate::clockid_t, sevp: *mut crate::sigevent, timerid: *mut crate::timer_t) -> c_int
```

### `timer_delete`

```rust
unsafe fn timer_delete(timerid: crate::timer_t) -> c_int
```

### `timer_getoverrun`

```rust
unsafe fn timer_getoverrun(timerid: crate::timer_t) -> c_int
```

### `timer_gettime`

```rust
unsafe fn timer_gettime(timerid: crate::timer_t, curr_value: *mut crate::itimerspec) -> c_int
```

### `timer_settime`

```rust
unsafe fn timer_settime(timerid: crate::timer_t, flags: c_int, new_value: *const crate::itimerspec, old_value: *mut crate::itimerspec) -> c_int
```

### `gethostid`

```rust
unsafe fn gethostid() -> c_long
```

### `memmem`

```rust
unsafe fn memmem(haystack: *const c_void, haystacklen: size_t, needle: *const c_void, needlelen: size_t) -> *mut c_void
```

### `sched_getcpu`

```rust
unsafe fn sched_getcpu() -> c_int
```

### `getopt_long`

```rust
unsafe fn getopt_long(argc: c_int, argv: *const *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int
```

### `copy_file_range`

```rust
unsafe fn copy_file_range(fd_in: c_int, off_in: *mut off64_t, fd_out: c_int, off_out: *mut off64_t, len: size_t, flags: c_uint) -> ssize_t
```

### `klogctl`

```rust
unsafe fn klogctl(syslog_type: c_int, bufp: *mut c_char, len: c_int) -> c_int
```

### `fallocate64`

```rust
unsafe fn fallocate64(fd: c_int, mode: c_int, offset: off64_t, len: off64_t) -> c_int
```

### `fgetpos64`

```rust
unsafe fn fgetpos64(stream: *mut crate::FILE, ptr: *mut fpos64_t) -> c_int
```

### `fopen64`

```rust
unsafe fn fopen64(filename: *const c_char, mode: *const c_char) -> *mut crate::FILE
```

### `freopen64`

```rust
unsafe fn freopen64(filename: *const c_char, mode: *const c_char, file: *mut crate::FILE) -> *mut crate::FILE
```

### `fseeko64`

```rust
unsafe fn fseeko64(stream: *mut crate::FILE, offset: off64_t, whence: c_int) -> c_int
```

### `fsetpos64`

```rust
unsafe fn fsetpos64(stream: *mut crate::FILE, ptr: *const fpos64_t) -> c_int
```

### `ftello64`

```rust
unsafe fn ftello64(stream: *mut crate::FILE) -> off64_t
```

### `posix_fallocate64`

```rust
unsafe fn posix_fallocate64(fd: c_int, offset: off64_t, len: off64_t) -> c_int
```

### `sendfile64`

```rust
unsafe fn sendfile64(out_fd: c_int, in_fd: c_int, offset: *mut off64_t, count: size_t) -> ssize_t
```

### `tmpfile64`

```rust
unsafe fn tmpfile64() -> *mut crate::FILE
```

### `issecure_mask`

```rust
const fn issecure_mask(x: crate::c_int) -> crate::c_int
```

### `FUTEX_OP`

```rust
fn FUTEX_OP(op: crate::c_int, oparg: crate::c_int, cmp: crate::c_int, cmparg: crate::c_int) -> crate::c_int
```

### `NLA_ALIGN`

```rust
unsafe fn NLA_ALIGN(len: crate::c_int) -> crate::c_int
```

### `CMSG_NXTHDR`

```rust
unsafe fn CMSG_NXTHDR(mhdr: *const crate::msghdr, cmsg: *const crate::cmsghdr) -> *mut crate::cmsghdr
```

### `CPU_ALLOC_SIZE`

```rust
unsafe fn CPU_ALLOC_SIZE(count: crate::c_int) -> crate::size_t
```

### `CPU_ZERO`

```rust
unsafe fn CPU_ZERO(cpuset: &mut cpu_set_t)
```

### `CPU_SET`

```rust
unsafe fn CPU_SET(cpu: usize, cpuset: &mut cpu_set_t)
```

### `CPU_CLR`

```rust
unsafe fn CPU_CLR(cpu: usize, cpuset: &mut cpu_set_t)
```

### `CPU_ISSET`

```rust
unsafe fn CPU_ISSET(cpu: usize, cpuset: &cpu_set_t) -> bool
```

### `CPU_COUNT_S`

```rust
unsafe fn CPU_COUNT_S(size: usize, cpuset: &cpu_set_t) -> crate::c_int
```

### `CPU_COUNT`

```rust
unsafe fn CPU_COUNT(cpuset: &cpu_set_t) -> crate::c_int
```

### `CPU_EQUAL`

```rust
unsafe fn CPU_EQUAL(set1: &cpu_set_t, set2: &cpu_set_t) -> bool
```

### `SCTP_PR_INDEX`

```rust
unsafe fn SCTP_PR_INDEX(policy: crate::c_int) -> crate::c_int
```

### `SCTP_PR_POLICY`

```rust
unsafe fn SCTP_PR_POLICY(policy: crate::c_int) -> crate::c_int
```

### `SCTP_PR_SET_POLICY`

```rust
unsafe fn SCTP_PR_SET_POLICY(flags: &mut crate::c_int, policy: crate::c_int)
```

### `IPTOS_TOS`

```rust
unsafe fn IPTOS_TOS(tos: u8) -> u8
```

### `IPTOS_PREC`

```rust
unsafe fn IPTOS_PREC(tos: u8) -> u8
```

### `RT_TOS`

```rust
unsafe fn RT_TOS(tos: u8) -> u8
```

### `RT_ADDRCLASS`

```rust
unsafe fn RT_ADDRCLASS(flags: u32) -> u32
```

### `RT_LOCALADDR`

```rust
unsafe fn RT_LOCALADDR(flags: u32) -> bool
```

### `SO_EE_OFFENDER`

```rust
unsafe fn SO_EE_OFFENDER(ee: *const crate::sock_extended_err) -> *mut crate::sockaddr
```

### `TPACKET_ALIGN`

```rust
unsafe fn TPACKET_ALIGN(x: usize) -> usize
```

### `BPF_CLASS`

```rust
unsafe fn BPF_CLASS(code: __u32) -> __u32
```

### `BPF_SIZE`

```rust
unsafe fn BPF_SIZE(code: __u32) -> __u32
```

### `BPF_MODE`

```rust
unsafe fn BPF_MODE(code: __u32) -> __u32
```

### `BPF_OP`

```rust
unsafe fn BPF_OP(code: __u32) -> __u32
```

### `BPF_SRC`

```rust
unsafe fn BPF_SRC(code: __u32) -> __u32
```

### `BPF_RVAL`

```rust
unsafe fn BPF_RVAL(code: __u32) -> __u32
```

### `BPF_MISCOP`

```rust
unsafe fn BPF_MISCOP(code: __u32) -> __u32
```

### `BPF_STMT`

```rust
unsafe fn BPF_STMT(code: __u16, k: __u32) -> crate::sock_filter
```

### `BPF_JUMP`

```rust
unsafe fn BPF_JUMP(code: __u16, k: __u32, jt: __u8, jf: __u8) -> crate::sock_filter
```

### `ELF32_R_SYM`

```rust
unsafe fn ELF32_R_SYM(val: Elf32_Word) -> Elf32_Word
```

### `ELF32_R_TYPE`

```rust
unsafe fn ELF32_R_TYPE(val: Elf32_Word) -> Elf32_Word
```

### `ELF32_R_INFO`

```rust
unsafe fn ELF32_R_INFO(sym: Elf32_Word, t: Elf32_Word) -> Elf32_Word
```

### `ELF64_R_SYM`

```rust
unsafe fn ELF64_R_SYM(val: Elf64_Xword) -> Elf64_Xword
```

### `ELF64_R_TYPE`

```rust
unsafe fn ELF64_R_TYPE(val: Elf64_Xword) -> Elf64_Xword
```

### `ELF64_R_INFO`

```rust
unsafe fn ELF64_R_INFO(sym: Elf64_Xword, t: Elf64_Xword) -> Elf64_Xword
```

### `makedev`

```rust
const fn makedev(major: crate::c_uint, minor: crate::c_uint) -> crate::dev_t
```

### `major`

```rust
const fn major(dev: crate::dev_t) -> crate::c_uint
```

### `minor`

```rust
const fn minor(dev: crate::dev_t) -> crate::c_uint
```

### `SCTP_PR_TTL_ENABLED`

```rust
const fn SCTP_PR_TTL_ENABLED(policy: crate::c_int) -> bool
```

### `SCTP_PR_RTX_ENABLED`

```rust
const fn SCTP_PR_RTX_ENABLED(policy: crate::c_int) -> bool
```

### `SCTP_PR_PRIO_ENABLED`

```rust
const fn SCTP_PR_PRIO_ENABLED(policy: crate::c_int) -> bool
```

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

### `useconds_t`

```rust
type useconds_t = u32;
```

### `dev_t`

```rust
type dev_t = u64;
```

### `socklen_t`

```rust
type socklen_t = u32;
```

### `mode_t`

```rust
type mode_t = u32;
```

### `ino64_t`

```rust
type ino64_t = u64;
```

### `off64_t`

```rust
type off64_t = i64;
```

### `blkcnt64_t`

```rust
type blkcnt64_t = i64;
```

### `rlim64_t`

```rust
type rlim64_t = u64;
```

### `mqd_t`

```rust
type mqd_t = crate::c_int;
```

### `nfds_t`

```rust
type nfds_t = crate::c_ulong;
```

### `nl_item`

```rust
type nl_item = crate::c_int;
```

### `idtype_t`

```rust
type idtype_t = crate::c_uint;
```

### `loff_t`

```rust
type loff_t = crate::c_longlong;
```

### `pthread_key_t`

```rust
type pthread_key_t = crate::c_uint;
```

### `pthread_once_t`

```rust
type pthread_once_t = crate::c_int;
```

### `pthread_spinlock_t`

```rust
type pthread_spinlock_t = crate::c_int;
```

### `__kernel_fsid_t`

```rust
type __kernel_fsid_t = __c_anonymous__kernel_fsid_t;
```

### `__kernel_clockid_t`

```rust
type __kernel_clockid_t = crate::c_int;
```

### `__u8`

```rust
type __u8 = crate::c_uchar;
```

### `__u16`

```rust
type __u16 = crate::c_ushort;
```

### `__s16`

```rust
type __s16 = crate::c_short;
```

### `__u32`

```rust
type __u32 = crate::c_uint;
```

### `__s32`

```rust
type __s32 = crate::c_int;
```

### `Elf32_Half`

```rust
type Elf32_Half = u16;
```

### `Elf32_Word`

```rust
type Elf32_Word = u32;
```

### `Elf32_Off`

```rust
type Elf32_Off = u32;
```

### `Elf32_Addr`

```rust
type Elf32_Addr = u32;
```

### `Elf32_Xword`

```rust
type Elf32_Xword = u64;
```

### `Elf32_Sword`

```rust
type Elf32_Sword = i32;
```

### `Elf64_Half`

```rust
type Elf64_Half = u16;
```

### `Elf64_Word`

```rust
type Elf64_Word = u32;
```

### `Elf64_Off`

```rust
type Elf64_Off = u64;
```

### `Elf64_Addr`

```rust
type Elf64_Addr = u64;
```

### `Elf64_Xword`

```rust
type Elf64_Xword = u64;
```

### `Elf64_Sxword`

```rust
type Elf64_Sxword = i64;
```

### `Elf64_Sword`

```rust
type Elf64_Sword = i32;
```

### `Elf32_Section`

```rust
type Elf32_Section = u16;
```

### `Elf64_Section`

```rust
type Elf64_Section = u16;
```

### `Elf32_Relr`

```rust
type Elf32_Relr = Elf32_Word;
```

### `Elf64_Relr`

```rust
type Elf64_Relr = Elf32_Xword;
```

### `Elf32_Rel`

```rust
type Elf32_Rel = __c_anonymous_elf32_rel;
```

### `Elf64_Rel`

```rust
type Elf64_Rel = __c_anonymous_elf64_rel;
```

### `Elf32_Rela`

```rust
type Elf32_Rela = __c_anonymous_elf32_rela;
```

### `Elf64_Rela`

```rust
type Elf64_Rela = __c_anonymous_elf64_rela;
```

### `iconv_t`

```rust
type iconv_t = *mut crate::c_void;
```

### `sctp_assoc_t`

```rust
type sctp_assoc_t = __s32;
```

### `eventfd_t`

```rust
type eventfd_t = u64;
```

### `pid_type`

```rust
type pid_type = crate::c_uint;
```

### `proc_cn_mcast_op`

```rust
type proc_cn_mcast_op = crate::c_uint;
```

### `proc_cn_event`

```rust
type proc_cn_event = crate::c_uint;
```

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

## Constants

### `PIDTYPE_PID`

```rust
const PIDTYPE_PID: pid_type = 0u32;
```

### `PIDTYPE_TGID`

```rust
const PIDTYPE_TGID: pid_type = 1u32;
```

### `PIDTYPE_PGID`

```rust
const PIDTYPE_PGID: pid_type = 2u32;
```

### `PIDTYPE_SID`

```rust
const PIDTYPE_SID: pid_type = 3u32;
```

### `PIDTYPE_MAX`

```rust
const PIDTYPE_MAX: pid_type = 4u32;
```

### `ABDAY_1`

```rust
const ABDAY_1: crate::nl_item = 131_072i32;
```

### `ABDAY_2`

```rust
const ABDAY_2: crate::nl_item = 131_073i32;
```

### `ABDAY_3`

```rust
const ABDAY_3: crate::nl_item = 131_074i32;
```

### `ABDAY_4`

```rust
const ABDAY_4: crate::nl_item = 131_075i32;
```

### `ABDAY_5`

```rust
const ABDAY_5: crate::nl_item = 131_076i32;
```

### `ABDAY_6`

```rust
const ABDAY_6: crate::nl_item = 131_077i32;
```

### `ABDAY_7`

```rust
const ABDAY_7: crate::nl_item = 131_078i32;
```

### `DAY_1`

```rust
const DAY_1: crate::nl_item = 131_079i32;
```

### `DAY_2`

```rust
const DAY_2: crate::nl_item = 131_080i32;
```

### `DAY_3`

```rust
const DAY_3: crate::nl_item = 131_081i32;
```

### `DAY_4`

```rust
const DAY_4: crate::nl_item = 131_082i32;
```

### `DAY_5`

```rust
const DAY_5: crate::nl_item = 131_083i32;
```

### `DAY_6`

```rust
const DAY_6: crate::nl_item = 131_084i32;
```

### `DAY_7`

```rust
const DAY_7: crate::nl_item = 131_085i32;
```

### `ABMON_1`

```rust
const ABMON_1: crate::nl_item = 131_086i32;
```

### `ABMON_2`

```rust
const ABMON_2: crate::nl_item = 131_087i32;
```

### `ABMON_3`

```rust
const ABMON_3: crate::nl_item = 131_088i32;
```

### `ABMON_4`

```rust
const ABMON_4: crate::nl_item = 131_089i32;
```

### `ABMON_5`

```rust
const ABMON_5: crate::nl_item = 131_090i32;
```

### `ABMON_6`

```rust
const ABMON_6: crate::nl_item = 131_091i32;
```

### `ABMON_7`

```rust
const ABMON_7: crate::nl_item = 131_092i32;
```

### `ABMON_8`

```rust
const ABMON_8: crate::nl_item = 131_093i32;
```

### `ABMON_9`

```rust
const ABMON_9: crate::nl_item = 131_094i32;
```

### `ABMON_10`

```rust
const ABMON_10: crate::nl_item = 131_095i32;
```

### `ABMON_11`

```rust
const ABMON_11: crate::nl_item = 131_096i32;
```

### `ABMON_12`

```rust
const ABMON_12: crate::nl_item = 131_097i32;
```

### `MON_1`

```rust
const MON_1: crate::nl_item = 131_098i32;
```

### `MON_2`

```rust
const MON_2: crate::nl_item = 131_099i32;
```

### `MON_3`

```rust
const MON_3: crate::nl_item = 131_100i32;
```

### `MON_4`

```rust
const MON_4: crate::nl_item = 131_101i32;
```

### `MON_5`

```rust
const MON_5: crate::nl_item = 131_102i32;
```

### `MON_6`

```rust
const MON_6: crate::nl_item = 131_103i32;
```

### `MON_7`

```rust
const MON_7: crate::nl_item = 131_104i32;
```

### `MON_8`

```rust
const MON_8: crate::nl_item = 131_105i32;
```

### `MON_9`

```rust
const MON_9: crate::nl_item = 131_106i32;
```

### `MON_10`

```rust
const MON_10: crate::nl_item = 131_107i32;
```

### `MON_11`

```rust
const MON_11: crate::nl_item = 131_108i32;
```

### `MON_12`

```rust
const MON_12: crate::nl_item = 131_109i32;
```

### `AM_STR`

```rust
const AM_STR: crate::nl_item = 131_110i32;
```

### `PM_STR`

```rust
const PM_STR: crate::nl_item = 131_111i32;
```

### `D_T_FMT`

```rust
const D_T_FMT: crate::nl_item = 131_112i32;
```

### `D_FMT`

```rust
const D_FMT: crate::nl_item = 131_113i32;
```

### `T_FMT`

```rust
const T_FMT: crate::nl_item = 131_114i32;
```

### `T_FMT_AMPM`

```rust
const T_FMT_AMPM: crate::nl_item = 131_115i32;
```

### `ERA`

```rust
const ERA: crate::nl_item = 131_116i32;
```

### `ERA_D_FMT`

```rust
const ERA_D_FMT: crate::nl_item = 131_118i32;
```

### `ALT_DIGITS`

```rust
const ALT_DIGITS: crate::nl_item = 131_119i32;
```

### `ERA_D_T_FMT`

```rust
const ERA_D_T_FMT: crate::nl_item = 131_120i32;
```

### `ERA_T_FMT`

```rust
const ERA_T_FMT: crate::nl_item = 131_121i32;
```

### `CODESET`

```rust
const CODESET: crate::nl_item = 14i32;
```

### `CRNCYSTR`

```rust
const CRNCYSTR: crate::nl_item = 262_159i32;
```

### `RADIXCHAR`

```rust
const RADIXCHAR: crate::nl_item = 65_536i32;
```

### `THOUSEP`

```rust
const THOUSEP: crate::nl_item = 65_537i32;
```

### `YESEXPR`

```rust
const YESEXPR: crate::nl_item = 327_680i32;
```

### `NOEXPR`

```rust
const NOEXPR: crate::nl_item = 327_681i32;
```

### `YESSTR`

```rust
const YESSTR: crate::nl_item = 327_682i32;
```

### `NOSTR`

```rust
const NOSTR: crate::nl_item = 327_683i32;
```

### `RUSAGE_CHILDREN`

```rust
const RUSAGE_CHILDREN: crate::c_int = -1i32;
```

### `L_tmpnam`

```rust
const L_tmpnam: crate::c_uint = 20u32;
```

### `_PC_LINK_MAX`

```rust
const _PC_LINK_MAX: crate::c_int = 0i32;
```

### `_PC_MAX_CANON`

```rust
const _PC_MAX_CANON: crate::c_int = 1i32;
```

### `_PC_MAX_INPUT`

```rust
const _PC_MAX_INPUT: crate::c_int = 2i32;
```

### `_PC_NAME_MAX`

```rust
const _PC_NAME_MAX: crate::c_int = 3i32;
```

### `_PC_PATH_MAX`

```rust
const _PC_PATH_MAX: crate::c_int = 4i32;
```

### `_PC_PIPE_BUF`

```rust
const _PC_PIPE_BUF: crate::c_int = 5i32;
```

### `_PC_CHOWN_RESTRICTED`

```rust
const _PC_CHOWN_RESTRICTED: crate::c_int = 6i32;
```

### `_PC_NO_TRUNC`

```rust
const _PC_NO_TRUNC: crate::c_int = 7i32;
```

### `_PC_VDISABLE`

```rust
const _PC_VDISABLE: crate::c_int = 8i32;
```

### `_PC_SYNC_IO`

```rust
const _PC_SYNC_IO: crate::c_int = 9i32;
```

### `_PC_ASYNC_IO`

```rust
const _PC_ASYNC_IO: crate::c_int = 10i32;
```

### `_PC_PRIO_IO`

```rust
const _PC_PRIO_IO: crate::c_int = 11i32;
```

### `_PC_SOCK_MAXBUF`

```rust
const _PC_SOCK_MAXBUF: crate::c_int = 12i32;
```

### `_PC_FILESIZEBITS`

```rust
const _PC_FILESIZEBITS: crate::c_int = 13i32;
```

### `_PC_REC_INCR_XFER_SIZE`

```rust
const _PC_REC_INCR_XFER_SIZE: crate::c_int = 14i32;
```

### `_PC_REC_MAX_XFER_SIZE`

```rust
const _PC_REC_MAX_XFER_SIZE: crate::c_int = 15i32;
```

### `_PC_REC_MIN_XFER_SIZE`

```rust
const _PC_REC_MIN_XFER_SIZE: crate::c_int = 16i32;
```

### `_PC_REC_XFER_ALIGN`

```rust
const _PC_REC_XFER_ALIGN: crate::c_int = 17i32;
```

### `_PC_ALLOC_SIZE_MIN`

```rust
const _PC_ALLOC_SIZE_MIN: crate::c_int = 18i32;
```

### `_PC_SYMLINK_MAX`

```rust
const _PC_SYMLINK_MAX: crate::c_int = 19i32;
```

### `_PC_2_SYMLINKS`

```rust
const _PC_2_SYMLINKS: crate::c_int = 20i32;
```

### `MS_NOUSER`

```rust
const MS_NOUSER: crate::c_ulong = 18_446_744_071_562_067_968u64;
```

### `_SC_ARG_MAX`

```rust
const _SC_ARG_MAX: crate::c_int = 0i32;
```

### `_SC_CHILD_MAX`

```rust
const _SC_CHILD_MAX: crate::c_int = 1i32;
```

### `_SC_CLK_TCK`

```rust
const _SC_CLK_TCK: crate::c_int = 2i32;
```

### `_SC_NGROUPS_MAX`

```rust
const _SC_NGROUPS_MAX: crate::c_int = 3i32;
```

### `_SC_OPEN_MAX`

```rust
const _SC_OPEN_MAX: crate::c_int = 4i32;
```

### `_SC_STREAM_MAX`

```rust
const _SC_STREAM_MAX: crate::c_int = 5i32;
```

### `_SC_TZNAME_MAX`

```rust
const _SC_TZNAME_MAX: crate::c_int = 6i32;
```

### `_SC_JOB_CONTROL`

```rust
const _SC_JOB_CONTROL: crate::c_int = 7i32;
```

### `_SC_SAVED_IDS`

```rust
const _SC_SAVED_IDS: crate::c_int = 8i32;
```

### `_SC_REALTIME_SIGNALS`

```rust
const _SC_REALTIME_SIGNALS: crate::c_int = 9i32;
```

### `_SC_PRIORITY_SCHEDULING`

```rust
const _SC_PRIORITY_SCHEDULING: crate::c_int = 10i32;
```

### `_SC_TIMERS`

```rust
const _SC_TIMERS: crate::c_int = 11i32;
```

### `_SC_ASYNCHRONOUS_IO`

```rust
const _SC_ASYNCHRONOUS_IO: crate::c_int = 12i32;
```

### `_SC_PRIORITIZED_IO`

```rust
const _SC_PRIORITIZED_IO: crate::c_int = 13i32;
```

### `_SC_SYNCHRONIZED_IO`

```rust
const _SC_SYNCHRONIZED_IO: crate::c_int = 14i32;
```

### `_SC_FSYNC`

```rust
const _SC_FSYNC: crate::c_int = 15i32;
```

### `_SC_MAPPED_FILES`

```rust
const _SC_MAPPED_FILES: crate::c_int = 16i32;
```

### `_SC_MEMLOCK`

```rust
const _SC_MEMLOCK: crate::c_int = 17i32;
```

### `_SC_MEMLOCK_RANGE`

```rust
const _SC_MEMLOCK_RANGE: crate::c_int = 18i32;
```

### `_SC_MEMORY_PROTECTION`

```rust
const _SC_MEMORY_PROTECTION: crate::c_int = 19i32;
```

### `_SC_MESSAGE_PASSING`

```rust
const _SC_MESSAGE_PASSING: crate::c_int = 20i32;
```

### `_SC_SEMAPHORES`

```rust
const _SC_SEMAPHORES: crate::c_int = 21i32;
```

### `_SC_SHARED_MEMORY_OBJECTS`

```rust
const _SC_SHARED_MEMORY_OBJECTS: crate::c_int = 22i32;
```

### `_SC_AIO_LISTIO_MAX`

```rust
const _SC_AIO_LISTIO_MAX: crate::c_int = 23i32;
```

### `_SC_AIO_MAX`

```rust
const _SC_AIO_MAX: crate::c_int = 24i32;
```

### `_SC_AIO_PRIO_DELTA_MAX`

```rust
const _SC_AIO_PRIO_DELTA_MAX: crate::c_int = 25i32;
```

### `_SC_DELAYTIMER_MAX`

```rust
const _SC_DELAYTIMER_MAX: crate::c_int = 26i32;
```

### `_SC_MQ_OPEN_MAX`

```rust
const _SC_MQ_OPEN_MAX: crate::c_int = 27i32;
```

### `_SC_MQ_PRIO_MAX`

```rust
const _SC_MQ_PRIO_MAX: crate::c_int = 28i32;
```

### `_SC_VERSION`

```rust
const _SC_VERSION: crate::c_int = 29i32;
```

### `_SC_PAGESIZE`

```rust
const _SC_PAGESIZE: crate::c_int = 30i32;
```

### `_SC_PAGE_SIZE`

```rust
const _SC_PAGE_SIZE: crate::c_int = 30i32;
```

### `_SC_RTSIG_MAX`

```rust
const _SC_RTSIG_MAX: crate::c_int = 31i32;
```

### `_SC_SEM_NSEMS_MAX`

```rust
const _SC_SEM_NSEMS_MAX: crate::c_int = 32i32;
```

### `_SC_SEM_VALUE_MAX`

```rust
const _SC_SEM_VALUE_MAX: crate::c_int = 33i32;
```

### `_SC_SIGQUEUE_MAX`

```rust
const _SC_SIGQUEUE_MAX: crate::c_int = 34i32;
```

### `_SC_TIMER_MAX`

```rust
const _SC_TIMER_MAX: crate::c_int = 35i32;
```

### `_SC_BC_BASE_MAX`

```rust
const _SC_BC_BASE_MAX: crate::c_int = 36i32;
```

### `_SC_BC_DIM_MAX`

```rust
const _SC_BC_DIM_MAX: crate::c_int = 37i32;
```

### `_SC_BC_SCALE_MAX`

```rust
const _SC_BC_SCALE_MAX: crate::c_int = 38i32;
```

### `_SC_BC_STRING_MAX`

```rust
const _SC_BC_STRING_MAX: crate::c_int = 39i32;
```

### `_SC_COLL_WEIGHTS_MAX`

```rust
const _SC_COLL_WEIGHTS_MAX: crate::c_int = 40i32;
```

### `_SC_EXPR_NEST_MAX`

```rust
const _SC_EXPR_NEST_MAX: crate::c_int = 42i32;
```

### `_SC_LINE_MAX`

```rust
const _SC_LINE_MAX: crate::c_int = 43i32;
```

### `_SC_RE_DUP_MAX`

```rust
const _SC_RE_DUP_MAX: crate::c_int = 44i32;
```

### `_SC_2_VERSION`

```rust
const _SC_2_VERSION: crate::c_int = 46i32;
```

### `_SC_2_C_BIND`

```rust
const _SC_2_C_BIND: crate::c_int = 47i32;
```

### `_SC_2_C_DEV`

```rust
const _SC_2_C_DEV: crate::c_int = 48i32;
```

### `_SC_2_FORT_DEV`

```rust
const _SC_2_FORT_DEV: crate::c_int = 49i32;
```

### `_SC_2_FORT_RUN`

```rust
const _SC_2_FORT_RUN: crate::c_int = 50i32;
```

### `_SC_2_SW_DEV`

```rust
const _SC_2_SW_DEV: crate::c_int = 51i32;
```

### `_SC_2_LOCALEDEF`

```rust
const _SC_2_LOCALEDEF: crate::c_int = 52i32;
```

### `_SC_UIO_MAXIOV`

```rust
const _SC_UIO_MAXIOV: crate::c_int = 60i32;
```

### `_SC_IOV_MAX`

```rust
const _SC_IOV_MAX: crate::c_int = 60i32;
```

### `_SC_THREADS`

```rust
const _SC_THREADS: crate::c_int = 67i32;
```

### `_SC_THREAD_SAFE_FUNCTIONS`

```rust
const _SC_THREAD_SAFE_FUNCTIONS: crate::c_int = 68i32;
```

### `_SC_GETGR_R_SIZE_MAX`

```rust
const _SC_GETGR_R_SIZE_MAX: crate::c_int = 69i32;
```

### `_SC_GETPW_R_SIZE_MAX`

```rust
const _SC_GETPW_R_SIZE_MAX: crate::c_int = 70i32;
```

### `_SC_LOGIN_NAME_MAX`

```rust
const _SC_LOGIN_NAME_MAX: crate::c_int = 71i32;
```

### `_SC_TTY_NAME_MAX`

```rust
const _SC_TTY_NAME_MAX: crate::c_int = 72i32;
```

### `_SC_THREAD_DESTRUCTOR_ITERATIONS`

```rust
const _SC_THREAD_DESTRUCTOR_ITERATIONS: crate::c_int = 73i32;
```

### `_SC_THREAD_KEYS_MAX`

```rust
const _SC_THREAD_KEYS_MAX: crate::c_int = 74i32;
```

### `_SC_THREAD_STACK_MIN`

```rust
const _SC_THREAD_STACK_MIN: crate::c_int = 75i32;
```

### `_SC_THREAD_THREADS_MAX`

```rust
const _SC_THREAD_THREADS_MAX: crate::c_int = 76i32;
```

### `_SC_THREAD_ATTR_STACKADDR`

```rust
const _SC_THREAD_ATTR_STACKADDR: crate::c_int = 77i32;
```

### `_SC_THREAD_ATTR_STACKSIZE`

```rust
const _SC_THREAD_ATTR_STACKSIZE: crate::c_int = 78i32;
```

### `_SC_THREAD_PRIORITY_SCHEDULING`

```rust
const _SC_THREAD_PRIORITY_SCHEDULING: crate::c_int = 79i32;
```

### `_SC_THREAD_PRIO_INHERIT`

```rust
const _SC_THREAD_PRIO_INHERIT: crate::c_int = 80i32;
```

### `_SC_THREAD_PRIO_PROTECT`

```rust
const _SC_THREAD_PRIO_PROTECT: crate::c_int = 81i32;
```

### `_SC_THREAD_PROCESS_SHARED`

```rust
const _SC_THREAD_PROCESS_SHARED: crate::c_int = 82i32;
```

### `_SC_NPROCESSORS_CONF`

```rust
const _SC_NPROCESSORS_CONF: crate::c_int = 83i32;
```

### `_SC_NPROCESSORS_ONLN`

```rust
const _SC_NPROCESSORS_ONLN: crate::c_int = 84i32;
```

### `_SC_PHYS_PAGES`

```rust
const _SC_PHYS_PAGES: crate::c_int = 85i32;
```

### `_SC_AVPHYS_PAGES`

```rust
const _SC_AVPHYS_PAGES: crate::c_int = 86i32;
```

### `_SC_ATEXIT_MAX`

```rust
const _SC_ATEXIT_MAX: crate::c_int = 87i32;
```

### `_SC_PASS_MAX`

```rust
const _SC_PASS_MAX: crate::c_int = 88i32;
```

### `_SC_XOPEN_VERSION`

```rust
const _SC_XOPEN_VERSION: crate::c_int = 89i32;
```

### `_SC_XOPEN_XCU_VERSION`

```rust
const _SC_XOPEN_XCU_VERSION: crate::c_int = 90i32;
```

### `_SC_XOPEN_UNIX`

```rust
const _SC_XOPEN_UNIX: crate::c_int = 91i32;
```

### `_SC_XOPEN_CRYPT`

```rust
const _SC_XOPEN_CRYPT: crate::c_int = 92i32;
```

### `_SC_XOPEN_ENH_I18N`

```rust
const _SC_XOPEN_ENH_I18N: crate::c_int = 93i32;
```

### `_SC_XOPEN_SHM`

```rust
const _SC_XOPEN_SHM: crate::c_int = 94i32;
```

### `_SC_2_CHAR_TERM`

```rust
const _SC_2_CHAR_TERM: crate::c_int = 95i32;
```

### `_SC_2_UPE`

```rust
const _SC_2_UPE: crate::c_int = 97i32;
```

### `_SC_XOPEN_XPG2`

```rust
const _SC_XOPEN_XPG2: crate::c_int = 98i32;
```

### `_SC_XOPEN_XPG3`

```rust
const _SC_XOPEN_XPG3: crate::c_int = 99i32;
```

### `_SC_XOPEN_XPG4`

```rust
const _SC_XOPEN_XPG4: crate::c_int = 100i32;
```

### `_SC_NZERO`

```rust
const _SC_NZERO: crate::c_int = 109i32;
```

### `_SC_XBS5_ILP32_OFF32`

```rust
const _SC_XBS5_ILP32_OFF32: crate::c_int = 125i32;
```

### `_SC_XBS5_ILP32_OFFBIG`

```rust
const _SC_XBS5_ILP32_OFFBIG: crate::c_int = 126i32;
```

### `_SC_XBS5_LP64_OFF64`

```rust
const _SC_XBS5_LP64_OFF64: crate::c_int = 127i32;
```

### `_SC_XBS5_LPBIG_OFFBIG`

```rust
const _SC_XBS5_LPBIG_OFFBIG: crate::c_int = 128i32;
```

### `_SC_XOPEN_LEGACY`

```rust
const _SC_XOPEN_LEGACY: crate::c_int = 129i32;
```

### `_SC_XOPEN_REALTIME`

```rust
const _SC_XOPEN_REALTIME: crate::c_int = 130i32;
```

### `_SC_XOPEN_REALTIME_THREADS`

```rust
const _SC_XOPEN_REALTIME_THREADS: crate::c_int = 131i32;
```

### `_SC_ADVISORY_INFO`

```rust
const _SC_ADVISORY_INFO: crate::c_int = 132i32;
```

### `_SC_BARRIERS`

```rust
const _SC_BARRIERS: crate::c_int = 133i32;
```

### `_SC_CLOCK_SELECTION`

```rust
const _SC_CLOCK_SELECTION: crate::c_int = 137i32;
```

### `_SC_CPUTIME`

```rust
const _SC_CPUTIME: crate::c_int = 138i32;
```

### `_SC_THREAD_CPUTIME`

```rust
const _SC_THREAD_CPUTIME: crate::c_int = 139i32;
```

### `_SC_MONOTONIC_CLOCK`

```rust
const _SC_MONOTONIC_CLOCK: crate::c_int = 149i32;
```

### `_SC_READER_WRITER_LOCKS`

```rust
const _SC_READER_WRITER_LOCKS: crate::c_int = 153i32;
```

### `_SC_SPIN_LOCKS`

```rust
const _SC_SPIN_LOCKS: crate::c_int = 154i32;
```

### `_SC_REGEXP`

```rust
const _SC_REGEXP: crate::c_int = 155i32;
```

### `_SC_SHELL`

```rust
const _SC_SHELL: crate::c_int = 157i32;
```

### `_SC_SPAWN`

```rust
const _SC_SPAWN: crate::c_int = 159i32;
```

### `_SC_SPORADIC_SERVER`

```rust
const _SC_SPORADIC_SERVER: crate::c_int = 160i32;
```

### `_SC_THREAD_SPORADIC_SERVER`

```rust
const _SC_THREAD_SPORADIC_SERVER: crate::c_int = 161i32;
```

### `_SC_TIMEOUTS`

```rust
const _SC_TIMEOUTS: crate::c_int = 164i32;
```

### `_SC_TYPED_MEMORY_OBJECTS`

```rust
const _SC_TYPED_MEMORY_OBJECTS: crate::c_int = 165i32;
```

### `_SC_2_PBS`

```rust
const _SC_2_PBS: crate::c_int = 168i32;
```

### `_SC_2_PBS_ACCOUNTING`

```rust
const _SC_2_PBS_ACCOUNTING: crate::c_int = 169i32;
```

### `_SC_2_PBS_LOCATE`

```rust
const _SC_2_PBS_LOCATE: crate::c_int = 170i32;
```

### `_SC_2_PBS_MESSAGE`

```rust
const _SC_2_PBS_MESSAGE: crate::c_int = 171i32;
```

### `_SC_2_PBS_TRACK`

```rust
const _SC_2_PBS_TRACK: crate::c_int = 172i32;
```

### `_SC_SYMLOOP_MAX`

```rust
const _SC_SYMLOOP_MAX: crate::c_int = 173i32;
```

### `_SC_STREAMS`

```rust
const _SC_STREAMS: crate::c_int = 174i32;
```

### `_SC_2_PBS_CHECKPOINT`

```rust
const _SC_2_PBS_CHECKPOINT: crate::c_int = 175i32;
```

### `_SC_V6_ILP32_OFF32`

```rust
const _SC_V6_ILP32_OFF32: crate::c_int = 176i32;
```

### `_SC_V6_ILP32_OFFBIG`

```rust
const _SC_V6_ILP32_OFFBIG: crate::c_int = 177i32;
```

### `_SC_V6_LP64_OFF64`

```rust
const _SC_V6_LP64_OFF64: crate::c_int = 178i32;
```

### `_SC_V6_LPBIG_OFFBIG`

```rust
const _SC_V6_LPBIG_OFFBIG: crate::c_int = 179i32;
```

### `_SC_HOST_NAME_MAX`

```rust
const _SC_HOST_NAME_MAX: crate::c_int = 180i32;
```

### `_SC_TRACE`

```rust
const _SC_TRACE: crate::c_int = 181i32;
```

### `_SC_TRACE_EVENT_FILTER`

```rust
const _SC_TRACE_EVENT_FILTER: crate::c_int = 182i32;
```

### `_SC_TRACE_INHERIT`

```rust
const _SC_TRACE_INHERIT: crate::c_int = 183i32;
```

### `_SC_TRACE_LOG`

```rust
const _SC_TRACE_LOG: crate::c_int = 184i32;
```

### `_SC_IPV6`

```rust
const _SC_IPV6: crate::c_int = 235i32;
```

### `_SC_RAW_SOCKETS`

```rust
const _SC_RAW_SOCKETS: crate::c_int = 236i32;
```

### `_SC_V7_ILP32_OFF32`

```rust
const _SC_V7_ILP32_OFF32: crate::c_int = 237i32;
```

### `_SC_V7_ILP32_OFFBIG`

```rust
const _SC_V7_ILP32_OFFBIG: crate::c_int = 238i32;
```

### `_SC_V7_LP64_OFF64`

```rust
const _SC_V7_LP64_OFF64: crate::c_int = 239i32;
```

### `_SC_V7_LPBIG_OFFBIG`

```rust
const _SC_V7_LPBIG_OFFBIG: crate::c_int = 240i32;
```

### `_SC_SS_REPL_MAX`

```rust
const _SC_SS_REPL_MAX: crate::c_int = 241i32;
```

### `_SC_TRACE_EVENT_NAME_MAX`

```rust
const _SC_TRACE_EVENT_NAME_MAX: crate::c_int = 242i32;
```

### `_SC_TRACE_NAME_MAX`

```rust
const _SC_TRACE_NAME_MAX: crate::c_int = 243i32;
```

### `_SC_TRACE_SYS_MAX`

```rust
const _SC_TRACE_SYS_MAX: crate::c_int = 244i32;
```

### `_SC_TRACE_USER_EVENT_MAX`

```rust
const _SC_TRACE_USER_EVENT_MAX: crate::c_int = 245i32;
```

### `_SC_XOPEN_STREAMS`

```rust
const _SC_XOPEN_STREAMS: crate::c_int = 246i32;
```

### `_SC_THREAD_ROBUST_PRIO_INHERIT`

```rust
const _SC_THREAD_ROBUST_PRIO_INHERIT: crate::c_int = 247i32;
```

### `_SC_THREAD_ROBUST_PRIO_PROTECT`

```rust
const _SC_THREAD_ROBUST_PRIO_PROTECT: crate::c_int = 248i32;
```

### `_CS_PATH`

```rust
const _CS_PATH: crate::c_int = 0i32;
```

### `_CS_POSIX_V6_WIDTH_RESTRICTED_ENVS`

```rust
const _CS_POSIX_V6_WIDTH_RESTRICTED_ENVS: crate::c_int = 1i32;
```

### `_CS_POSIX_V5_WIDTH_RESTRICTED_ENVS`

```rust
const _CS_POSIX_V5_WIDTH_RESTRICTED_ENVS: crate::c_int = 4i32;
```

### `_CS_POSIX_V7_WIDTH_RESTRICTED_ENVS`

```rust
const _CS_POSIX_V7_WIDTH_RESTRICTED_ENVS: crate::c_int = 5i32;
```

### `_CS_POSIX_V6_ILP32_OFF32_CFLAGS`

```rust
const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: crate::c_int = 1_116i32;
```

### `_CS_POSIX_V6_ILP32_OFF32_LDFLAGS`

```rust
const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: crate::c_int = 1_117i32;
```

### `_CS_POSIX_V6_ILP32_OFF32_LIBS`

```rust
const _CS_POSIX_V6_ILP32_OFF32_LIBS: crate::c_int = 1_118i32;
```

### `_CS_POSIX_V6_ILP32_OFF32_LINTFLAGS`

```rust
const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: crate::c_int = 1_119i32;
```

### `_CS_POSIX_V6_ILP32_OFFBIG_CFLAGS`

```rust
const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: crate::c_int = 1_120i32;
```

### `_CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS`

```rust
const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: crate::c_int = 1_121i32;
```

### `_CS_POSIX_V6_ILP32_OFFBIG_LIBS`

```rust
const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: crate::c_int = 1_122i32;
```

### `_CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS`

```rust
const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: crate::c_int = 1_123i32;
```

### `_CS_POSIX_V6_LP64_OFF64_CFLAGS`

```rust
const _CS_POSIX_V6_LP64_OFF64_CFLAGS: crate::c_int = 1_124i32;
```

### `_CS_POSIX_V6_LP64_OFF64_LDFLAGS`

```rust
const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: crate::c_int = 1_125i32;
```

### `_CS_POSIX_V6_LP64_OFF64_LIBS`

```rust
const _CS_POSIX_V6_LP64_OFF64_LIBS: crate::c_int = 1_126i32;
```

### `_CS_POSIX_V6_LP64_OFF64_LINTFLAGS`

```rust
const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: crate::c_int = 1_127i32;
```

### `_CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS`

```rust
const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: crate::c_int = 1_128i32;
```

### `_CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS`

```rust
const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: crate::c_int = 1_129i32;
```

### `_CS_POSIX_V6_LPBIG_OFFBIG_LIBS`

```rust
const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: crate::c_int = 1_130i32;
```

### `_CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS`

```rust
const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: crate::c_int = 1_131i32;
```

### `_CS_POSIX_V7_ILP32_OFF32_CFLAGS`

```rust
const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: crate::c_int = 1_132i32;
```

### `_CS_POSIX_V7_ILP32_OFF32_LDFLAGS`

```rust
const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: crate::c_int = 1_133i32;
```

### `_CS_POSIX_V7_ILP32_OFF32_LIBS`

```rust
const _CS_POSIX_V7_ILP32_OFF32_LIBS: crate::c_int = 1_134i32;
```

### `_CS_POSIX_V7_ILP32_OFF32_LINTFLAGS`

```rust
const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: crate::c_int = 1_135i32;
```

### `_CS_POSIX_V7_ILP32_OFFBIG_CFLAGS`

```rust
const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: crate::c_int = 1_136i32;
```

### `_CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS`

```rust
const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: crate::c_int = 1_137i32;
```

### `_CS_POSIX_V7_ILP32_OFFBIG_LIBS`

```rust
const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: crate::c_int = 1_138i32;
```

### `_CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS`

```rust
const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: crate::c_int = 1_139i32;
```

### `_CS_POSIX_V7_LP64_OFF64_CFLAGS`

```rust
const _CS_POSIX_V7_LP64_OFF64_CFLAGS: crate::c_int = 1_140i32;
```

### `_CS_POSIX_V7_LP64_OFF64_LDFLAGS`

```rust
const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: crate::c_int = 1_141i32;
```

### `_CS_POSIX_V7_LP64_OFF64_LIBS`

```rust
const _CS_POSIX_V7_LP64_OFF64_LIBS: crate::c_int = 1_142i32;
```

### `_CS_POSIX_V7_LP64_OFF64_LINTFLAGS`

```rust
const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: crate::c_int = 1_143i32;
```

### `_CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS`

```rust
const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: crate::c_int = 1_144i32;
```

### `_CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS`

```rust
const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: crate::c_int = 1_145i32;
```

### `_CS_POSIX_V7_LPBIG_OFFBIG_LIBS`

```rust
const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: crate::c_int = 1_146i32;
```

### `_CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS`

```rust
const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: crate::c_int = 1_147i32;
```

### `RLIM_SAVED_MAX`

```rust
const RLIM_SAVED_MAX: crate::rlim_t = 18_446_744_073_709_551_615u64;
```

### `RLIM_SAVED_CUR`

```rust
const RLIM_SAVED_CUR: crate::rlim_t = 18_446_744_073_709_551_615u64;
```

### `EI_NIDENT`

```rust
const EI_NIDENT: usize = 16usize;
```

### `EI_MAG0`

```rust
const EI_MAG0: usize = 0usize;
```

### `ELFMAG0`

```rust
const ELFMAG0: u8 = 127u8;
```

### `EI_MAG1`

```rust
const EI_MAG1: usize = 1usize;
```

### `ELFMAG1`

```rust
const ELFMAG1: u8 = 69u8;
```

### `EI_MAG2`

```rust
const EI_MAG2: usize = 2usize;
```

### `ELFMAG2`

```rust
const ELFMAG2: u8 = 76u8;
```

### `EI_MAG3`

```rust
const EI_MAG3: usize = 3usize;
```

### `ELFMAG3`

```rust
const ELFMAG3: u8 = 70u8;
```

### `SELFMAG`

```rust
const SELFMAG: usize = 4usize;
```

### `EI_CLASS`

```rust
const EI_CLASS: usize = 4usize;
```

### `ELFCLASSNONE`

```rust
const ELFCLASSNONE: u8 = 0u8;
```

### `ELFCLASS32`

```rust
const ELFCLASS32: u8 = 1u8;
```

### `ELFCLASS64`

```rust
const ELFCLASS64: u8 = 2u8;
```

### `ELFCLASSNUM`

```rust
const ELFCLASSNUM: usize = 3usize;
```

### `EI_DATA`

```rust
const EI_DATA: usize = 5usize;
```

### `ELFDATANONE`

```rust
const ELFDATANONE: u8 = 0u8;
```

### `ELFDATA2LSB`

```rust
const ELFDATA2LSB: u8 = 1u8;
```

### `ELFDATA2MSB`

```rust
const ELFDATA2MSB: u8 = 2u8;
```

### `ELFDATANUM`

```rust
const ELFDATANUM: usize = 3usize;
```

### `EI_VERSION`

```rust
const EI_VERSION: usize = 6usize;
```

### `EI_OSABI`

```rust
const EI_OSABI: usize = 7usize;
```

### `ELFOSABI_NONE`

```rust
const ELFOSABI_NONE: u8 = 0u8;
```

### `ELFOSABI_SYSV`

```rust
const ELFOSABI_SYSV: u8 = 0u8;
```

### `ELFOSABI_HPUX`

```rust
const ELFOSABI_HPUX: u8 = 1u8;
```

### `ELFOSABI_NETBSD`

```rust
const ELFOSABI_NETBSD: u8 = 2u8;
```

### `ELFOSABI_GNU`

```rust
const ELFOSABI_GNU: u8 = 3u8;
```

### `ELFOSABI_LINUX`

```rust
const ELFOSABI_LINUX: u8 = 3u8;
```

### `ELFOSABI_SOLARIS`

```rust
const ELFOSABI_SOLARIS: u8 = 6u8;
```

### `ELFOSABI_AIX`

```rust
const ELFOSABI_AIX: u8 = 7u8;
```

### `ELFOSABI_IRIX`

```rust
const ELFOSABI_IRIX: u8 = 8u8;
```

### `ELFOSABI_FREEBSD`

```rust
const ELFOSABI_FREEBSD: u8 = 9u8;
```

### `ELFOSABI_TRU64`

```rust
const ELFOSABI_TRU64: u8 = 10u8;
```

### `ELFOSABI_MODESTO`

```rust
const ELFOSABI_MODESTO: u8 = 11u8;
```

### `ELFOSABI_OPENBSD`

```rust
const ELFOSABI_OPENBSD: u8 = 12u8;
```

### `ELFOSABI_ARM`

```rust
const ELFOSABI_ARM: u8 = 97u8;
```

### `ELFOSABI_STANDALONE`

```rust
const ELFOSABI_STANDALONE: u8 = 255u8;
```

### `EI_ABIVERSION`

```rust
const EI_ABIVERSION: usize = 8usize;
```

### `EI_PAD`

```rust
const EI_PAD: usize = 9usize;
```

### `ET_NONE`

```rust
const ET_NONE: u16 = 0u16;
```

### `ET_REL`

```rust
const ET_REL: u16 = 1u16;
```

### `ET_EXEC`

```rust
const ET_EXEC: u16 = 2u16;
```

### `ET_DYN`

```rust
const ET_DYN: u16 = 3u16;
```

### `ET_CORE`

```rust
const ET_CORE: u16 = 4u16;
```

### `ET_NUM`

```rust
const ET_NUM: u16 = 5u16;
```

### `ET_LOOS`

```rust
const ET_LOOS: u16 = 65_024u16;
```

### `ET_HIOS`

```rust
const ET_HIOS: u16 = 65_279u16;
```

### `ET_LOPROC`

```rust
const ET_LOPROC: u16 = 65_280u16;
```

### `ET_HIPROC`

```rust
const ET_HIPROC: u16 = 65_535u16;
```

### `EM_NONE`

```rust
const EM_NONE: u16 = 0u16;
```

### `EM_M32`

```rust
const EM_M32: u16 = 1u16;
```

### `EM_SPARC`

```rust
const EM_SPARC: u16 = 2u16;
```

### `EM_386`

```rust
const EM_386: u16 = 3u16;
```

### `EM_68K`

```rust
const EM_68K: u16 = 4u16;
```

### `EM_88K`

```rust
const EM_88K: u16 = 5u16;
```

### `EM_860`

```rust
const EM_860: u16 = 7u16;
```

### `EM_MIPS`

```rust
const EM_MIPS: u16 = 8u16;
```

### `EM_S370`

```rust
const EM_S370: u16 = 9u16;
```

### `EM_MIPS_RS3_LE`

```rust
const EM_MIPS_RS3_LE: u16 = 10u16;
```

### `EM_PARISC`

```rust
const EM_PARISC: u16 = 15u16;
```

### `EM_VPP500`

```rust
const EM_VPP500: u16 = 17u16;
```

### `EM_SPARC32PLUS`

```rust
const EM_SPARC32PLUS: u16 = 18u16;
```

### `EM_960`

```rust
const EM_960: u16 = 19u16;
```

### `EM_PPC`

```rust
const EM_PPC: u16 = 20u16;
```

### `EM_PPC64`

```rust
const EM_PPC64: u16 = 21u16;
```

### `EM_S390`

```rust
const EM_S390: u16 = 22u16;
```

### `EM_V800`

```rust
const EM_V800: u16 = 36u16;
```

### `EM_FR20`

```rust
const EM_FR20: u16 = 37u16;
```

### `EM_RH32`

```rust
const EM_RH32: u16 = 38u16;
```

### `EM_RCE`

```rust
const EM_RCE: u16 = 39u16;
```

### `EM_ARM`

```rust
const EM_ARM: u16 = 40u16;
```

### `EM_FAKE_ALPHA`

```rust
const EM_FAKE_ALPHA: u16 = 41u16;
```

### `EM_SH`

```rust
const EM_SH: u16 = 42u16;
```

### `EM_SPARCV9`

```rust
const EM_SPARCV9: u16 = 43u16;
```

### `EM_TRICORE`

```rust
const EM_TRICORE: u16 = 44u16;
```

### `EM_ARC`

```rust
const EM_ARC: u16 = 45u16;
```

### `EM_H8_300`

```rust
const EM_H8_300: u16 = 46u16;
```

### `EM_H8_300H`

```rust
const EM_H8_300H: u16 = 47u16;
```

### `EM_H8S`

```rust
const EM_H8S: u16 = 48u16;
```

### `EM_H8_500`

```rust
const EM_H8_500: u16 = 49u16;
```

### `EM_IA_64`

```rust
const EM_IA_64: u16 = 50u16;
```

### `EM_MIPS_X`

```rust
const EM_MIPS_X: u16 = 51u16;
```

### `EM_COLDFIRE`

```rust
const EM_COLDFIRE: u16 = 52u16;
```

### `EM_68HC12`

```rust
const EM_68HC12: u16 = 53u16;
```

### `EM_MMA`

```rust
const EM_MMA: u16 = 54u16;
```

### `EM_PCP`

```rust
const EM_PCP: u16 = 55u16;
```

### `EM_NCPU`

```rust
const EM_NCPU: u16 = 56u16;
```

### `EM_NDR1`

```rust
const EM_NDR1: u16 = 57u16;
```

### `EM_STARCORE`

```rust
const EM_STARCORE: u16 = 58u16;
```

### `EM_ME16`

```rust
const EM_ME16: u16 = 59u16;
```

### `EM_ST100`

```rust
const EM_ST100: u16 = 60u16;
```

### `EM_TINYJ`

```rust
const EM_TINYJ: u16 = 61u16;
```

### `EM_X86_64`

```rust
const EM_X86_64: u16 = 62u16;
```

### `EM_PDSP`

```rust
const EM_PDSP: u16 = 63u16;
```

### `EM_FX66`

```rust
const EM_FX66: u16 = 66u16;
```

### `EM_ST9PLUS`

```rust
const EM_ST9PLUS: u16 = 67u16;
```

### `EM_ST7`

```rust
const EM_ST7: u16 = 68u16;
```

### `EM_68HC16`

```rust
const EM_68HC16: u16 = 69u16;
```

### `EM_68HC11`

```rust
const EM_68HC11: u16 = 70u16;
```

### `EM_68HC08`

```rust
const EM_68HC08: u16 = 71u16;
```

### `EM_68HC05`

```rust
const EM_68HC05: u16 = 72u16;
```

### `EM_SVX`

```rust
const EM_SVX: u16 = 73u16;
```

### `EM_ST19`

```rust
const EM_ST19: u16 = 74u16;
```

### `EM_VAX`

```rust
const EM_VAX: u16 = 75u16;
```

### `EM_CRIS`

```rust
const EM_CRIS: u16 = 76u16;
```

### `EM_JAVELIN`

```rust
const EM_JAVELIN: u16 = 77u16;
```

### `EM_FIREPATH`

```rust
const EM_FIREPATH: u16 = 78u16;
```

### `EM_ZSP`

```rust
const EM_ZSP: u16 = 79u16;
```

### `EM_MMIX`

```rust
const EM_MMIX: u16 = 80u16;
```

### `EM_HUANY`

```rust
const EM_HUANY: u16 = 81u16;
```

### `EM_PRISM`

```rust
const EM_PRISM: u16 = 82u16;
```

### `EM_AVR`

```rust
const EM_AVR: u16 = 83u16;
```

### `EM_FR30`

```rust
const EM_FR30: u16 = 84u16;
```

### `EM_D10V`

```rust
const EM_D10V: u16 = 85u16;
```

### `EM_D30V`

```rust
const EM_D30V: u16 = 86u16;
```

### `EM_V850`

```rust
const EM_V850: u16 = 87u16;
```

### `EM_M32R`

```rust
const EM_M32R: u16 = 88u16;
```

### `EM_MN10300`

```rust
const EM_MN10300: u16 = 89u16;
```

### `EM_MN10200`

```rust
const EM_MN10200: u16 = 90u16;
```

### `EM_PJ`

```rust
const EM_PJ: u16 = 91u16;
```

### `EM_OPENRISC`

```rust
const EM_OPENRISC: u16 = 92u16;
```

### `EM_ARC_A5`

```rust
const EM_ARC_A5: u16 = 93u16;
```

### `EM_XTENSA`

```rust
const EM_XTENSA: u16 = 94u16;
```

### `EM_AARCH64`

```rust
const EM_AARCH64: u16 = 183u16;
```

### `EM_TILEPRO`

```rust
const EM_TILEPRO: u16 = 188u16;
```

### `EM_TILEGX`

```rust
const EM_TILEGX: u16 = 191u16;
```

### `EM_RISCV`

```rust
const EM_RISCV: u16 = 243u16;
```

### `EM_ALPHA`

```rust
const EM_ALPHA: u16 = 36_902u16;
```

### `EV_NONE`

```rust
const EV_NONE: u32 = 0u32;
```

### `EV_CURRENT`

```rust
const EV_CURRENT: u32 = 1u32;
```

### `EV_NUM`

```rust
const EV_NUM: u32 = 2u32;
```

### `PT_NULL`

```rust
const PT_NULL: u32 = 0u32;
```

### `PT_LOAD`

```rust
const PT_LOAD: u32 = 1u32;
```

### `PT_DYNAMIC`

```rust
const PT_DYNAMIC: u32 = 2u32;
```

### `PT_INTERP`

```rust
const PT_INTERP: u32 = 3u32;
```

### `PT_NOTE`

```rust
const PT_NOTE: u32 = 4u32;
```

### `PT_SHLIB`

```rust
const PT_SHLIB: u32 = 5u32;
```

### `PT_PHDR`

```rust
const PT_PHDR: u32 = 6u32;
```

### `PT_TLS`

```rust
const PT_TLS: u32 = 7u32;
```

### `PT_NUM`

```rust
const PT_NUM: u32 = 8u32;
```

### `PT_LOOS`

```rust
const PT_LOOS: u32 = 1_610_612_736u32;
```

### `PT_GNU_EH_FRAME`

```rust
const PT_GNU_EH_FRAME: u32 = 1_685_382_480u32;
```

### `PT_GNU_STACK`

```rust
const PT_GNU_STACK: u32 = 1_685_382_481u32;
```

### `PT_GNU_RELRO`

```rust
const PT_GNU_RELRO: u32 = 1_685_382_482u32;
```

### `PT_LOSUNW`

```rust
const PT_LOSUNW: u32 = 1_879_048_186u32;
```

### `PT_SUNWBSS`

```rust
const PT_SUNWBSS: u32 = 1_879_048_186u32;
```

### `PT_SUNWSTACK`

```rust
const PT_SUNWSTACK: u32 = 1_879_048_187u32;
```

### `PT_HISUNW`

```rust
const PT_HISUNW: u32 = 1_879_048_191u32;
```

### `PT_HIOS`

```rust
const PT_HIOS: u32 = 1_879_048_191u32;
```

### `PT_LOPROC`

```rust
const PT_LOPROC: u32 = 1_879_048_192u32;
```

### `PT_HIPROC`

```rust
const PT_HIPROC: u32 = 2_147_483_647u32;
```

### `PF_X`

```rust
const PF_X: u32 = 1u32;
```

### `PF_W`

```rust
const PF_W: u32 = 2u32;
```

### `PF_R`

```rust
const PF_R: u32 = 4u32;
```

### `PF_MASKOS`

```rust
const PF_MASKOS: u32 = 267_386_880u32;
```

### `PF_MASKPROC`

```rust
const PF_MASKPROC: u32 = 4_026_531_840u32;
```

### `AT_NULL`

```rust
const AT_NULL: crate::c_ulong = 0u64;
```

### `AT_IGNORE`

```rust
const AT_IGNORE: crate::c_ulong = 1u64;
```

### `AT_EXECFD`

```rust
const AT_EXECFD: crate::c_ulong = 2u64;
```

### `AT_PHDR`

```rust
const AT_PHDR: crate::c_ulong = 3u64;
```

### `AT_PHENT`

```rust
const AT_PHENT: crate::c_ulong = 4u64;
```

### `AT_PHNUM`

```rust
const AT_PHNUM: crate::c_ulong = 5u64;
```

### `AT_PAGESZ`

```rust
const AT_PAGESZ: crate::c_ulong = 6u64;
```

### `AT_BASE`

```rust
const AT_BASE: crate::c_ulong = 7u64;
```

### `AT_FLAGS`

```rust
const AT_FLAGS: crate::c_ulong = 8u64;
```

### `AT_ENTRY`

```rust
const AT_ENTRY: crate::c_ulong = 9u64;
```

### `AT_NOTELF`

```rust
const AT_NOTELF: crate::c_ulong = 10u64;
```

### `AT_UID`

```rust
const AT_UID: crate::c_ulong = 11u64;
```

### `AT_EUID`

```rust
const AT_EUID: crate::c_ulong = 12u64;
```

### `AT_GID`

```rust
const AT_GID: crate::c_ulong = 13u64;
```

### `AT_EGID`

```rust
const AT_EGID: crate::c_ulong = 14u64;
```

### `AT_PLATFORM`

```rust
const AT_PLATFORM: crate::c_ulong = 15u64;
```

### `AT_HWCAP`

```rust
const AT_HWCAP: crate::c_ulong = 16u64;
```

### `AT_CLKTCK`

```rust
const AT_CLKTCK: crate::c_ulong = 17u64;
```

### `AT_SECURE`

```rust
const AT_SECURE: crate::c_ulong = 23u64;
```

### `AT_BASE_PLATFORM`

```rust
const AT_BASE_PLATFORM: crate::c_ulong = 24u64;
```

### `AT_RANDOM`

```rust
const AT_RANDOM: crate::c_ulong = 25u64;
```

### `AT_HWCAP2`

```rust
const AT_HWCAP2: crate::c_ulong = 26u64;
```

### `AT_HWCAP3`

```rust
const AT_HWCAP3: crate::c_ulong = 29u64;
```

### `AT_HWCAP4`

```rust
const AT_HWCAP4: crate::c_ulong = 30u64;
```

### `AT_EXECFN`

```rust
const AT_EXECFN: crate::c_ulong = 31u64;
```

### `AT_SYSINFO_EHDR`

```rust
const AT_SYSINFO_EHDR: crate::c_ulong = 33u64;
```

### `AT_MINSIGSTKSZ`

```rust
const AT_MINSIGSTKSZ: crate::c_ulong = 51u64;
```

### `GLOB_ERR`

```rust
const GLOB_ERR: crate::c_int = 1i32;
```

### `GLOB_MARK`

```rust
const GLOB_MARK: crate::c_int = 2i32;
```

### `GLOB_NOSORT`

```rust
const GLOB_NOSORT: crate::c_int = 4i32;
```

### `GLOB_DOOFFS`

```rust
const GLOB_DOOFFS: crate::c_int = 8i32;
```

### `GLOB_NOCHECK`

```rust
const GLOB_NOCHECK: crate::c_int = 16i32;
```

### `GLOB_APPEND`

```rust
const GLOB_APPEND: crate::c_int = 32i32;
```

### `GLOB_NOESCAPE`

```rust
const GLOB_NOESCAPE: crate::c_int = 64i32;
```

### `GLOB_NOSPACE`

```rust
const GLOB_NOSPACE: crate::c_int = 1i32;
```

### `GLOB_ABORTED`

```rust
const GLOB_ABORTED: crate::c_int = 2i32;
```

### `GLOB_NOMATCH`

```rust
const GLOB_NOMATCH: crate::c_int = 3i32;
```

### `POSIX_MADV_NORMAL`

```rust
const POSIX_MADV_NORMAL: crate::c_int = 0i32;
```

### `POSIX_MADV_RANDOM`

```rust
const POSIX_MADV_RANDOM: crate::c_int = 1i32;
```

### `POSIX_MADV_SEQUENTIAL`

```rust
const POSIX_MADV_SEQUENTIAL: crate::c_int = 2i32;
```

### `POSIX_MADV_WILLNEED`

```rust
const POSIX_MADV_WILLNEED: crate::c_int = 3i32;
```

### `POSIX_SPAWN_USEVFORK`

```rust
const POSIX_SPAWN_USEVFORK: crate::c_int = 64i32;
```

### `POSIX_SPAWN_SETSID`

```rust
const POSIX_SPAWN_SETSID: crate::c_int = 128i32;
```

### `S_IEXEC`

```rust
const S_IEXEC: mode_t = 64u32;
```

### `S_IWRITE`

```rust
const S_IWRITE: mode_t = 128u32;
```

### `S_IREAD`

```rust
const S_IREAD: mode_t = 256u32;
```

### `F_LOCK`

```rust
const F_LOCK: crate::c_int = 1i32;
```

### `F_TEST`

```rust
const F_TEST: crate::c_int = 3i32;
```

### `F_TLOCK`

```rust
const F_TLOCK: crate::c_int = 2i32;
```

### `F_ULOCK`

```rust
const F_ULOCK: crate::c_int = 0i32;
```

### `F_SEAL_FUTURE_WRITE`

```rust
const F_SEAL_FUTURE_WRITE: crate::c_int = 16i32;
```

### `F_SEAL_EXEC`

```rust
const F_SEAL_EXEC: crate::c_int = 32i32;
```

### `IFF_LOWER_UP`

```rust
const IFF_LOWER_UP: crate::c_int = 65_536i32;
```

### `IFF_DORMANT`

```rust
const IFF_DORMANT: crate::c_int = 131_072i32;
```

### `IFF_ECHO`

```rust
const IFF_ECHO: crate::c_int = 262_144i32;
```

### `IFA_UNSPEC`

```rust
const IFA_UNSPEC: crate::c_ushort = 0u16;
```

### `IFA_ADDRESS`

```rust
const IFA_ADDRESS: crate::c_ushort = 1u16;
```

### `IFA_LOCAL`

```rust
const IFA_LOCAL: crate::c_ushort = 2u16;
```

### `IFA_LABEL`

```rust
const IFA_LABEL: crate::c_ushort = 3u16;
```

### `IFA_BROADCAST`

```rust
const IFA_BROADCAST: crate::c_ushort = 4u16;
```

### `IFA_ANYCAST`

```rust
const IFA_ANYCAST: crate::c_ushort = 5u16;
```

### `IFA_CACHEINFO`

```rust
const IFA_CACHEINFO: crate::c_ushort = 6u16;
```

### `IFA_MULTICAST`

```rust
const IFA_MULTICAST: crate::c_ushort = 7u16;
```

### `IFA_FLAGS`

```rust
const IFA_FLAGS: crate::c_ushort = 8u16;
```

### `IFA_F_SECONDARY`

```rust
const IFA_F_SECONDARY: u32 = 1u32;
```

### `IFA_F_TEMPORARY`

```rust
const IFA_F_TEMPORARY: u32 = 1u32;
```

### `IFA_F_NODAD`

```rust
const IFA_F_NODAD: u32 = 2u32;
```

### `IFA_F_OPTIMISTIC`

```rust
const IFA_F_OPTIMISTIC: u32 = 4u32;
```

### `IFA_F_DADFAILED`

```rust
const IFA_F_DADFAILED: u32 = 8u32;
```

### `IFA_F_HOMEADDRESS`

```rust
const IFA_F_HOMEADDRESS: u32 = 16u32;
```

### `IFA_F_DEPRECATED`

```rust
const IFA_F_DEPRECATED: u32 = 32u32;
```

### `IFA_F_TENTATIVE`

```rust
const IFA_F_TENTATIVE: u32 = 64u32;
```

### `IFA_F_PERMANENT`

```rust
const IFA_F_PERMANENT: u32 = 128u32;
```

### `IFA_F_MANAGETEMPADDR`

```rust
const IFA_F_MANAGETEMPADDR: u32 = 256u32;
```

### `IFA_F_NOPREFIXROUTE`

```rust
const IFA_F_NOPREFIXROUTE: u32 = 512u32;
```

### `IFA_F_MCAUTOJOIN`

```rust
const IFA_F_MCAUTOJOIN: u32 = 1_024u32;
```

### `IFA_F_STABLE_PRIVACY`

```rust
const IFA_F_STABLE_PRIVACY: u32 = 2_048u32;
```

### `RWF_HIPRI`

```rust
const RWF_HIPRI: crate::c_int = 1i32;
```

### `RWF_DSYNC`

```rust
const RWF_DSYNC: crate::c_int = 2i32;
```

### `RWF_SYNC`

```rust
const RWF_SYNC: crate::c_int = 4i32;
```

### `RWF_NOWAIT`

```rust
const RWF_NOWAIT: crate::c_int = 8i32;
```

### `RWF_APPEND`

```rust
const RWF_APPEND: crate::c_int = 16i32;
```

### `RWF_NOAPPEND`

```rust
const RWF_NOAPPEND: crate::c_int = 32i32;
```

### `RWF_ATOMIC`

```rust
const RWF_ATOMIC: crate::c_int = 64i32;
```

### `RWF_DONTCACHE`

```rust
const RWF_DONTCACHE: crate::c_int = 128i32;
```

### `IFLA_UNSPEC`

```rust
const IFLA_UNSPEC: crate::c_ushort = 0u16;
```

### `IFLA_ADDRESS`

```rust
const IFLA_ADDRESS: crate::c_ushort = 1u16;
```

### `IFLA_BROADCAST`

```rust
const IFLA_BROADCAST: crate::c_ushort = 2u16;
```

### `IFLA_IFNAME`

```rust
const IFLA_IFNAME: crate::c_ushort = 3u16;
```

### `IFLA_MTU`

```rust
const IFLA_MTU: crate::c_ushort = 4u16;
```

### `IFLA_LINK`

```rust
const IFLA_LINK: crate::c_ushort = 5u16;
```

### `IFLA_QDISC`

```rust
const IFLA_QDISC: crate::c_ushort = 6u16;
```

### `IFLA_STATS`

```rust
const IFLA_STATS: crate::c_ushort = 7u16;
```

### `IFLA_COST`

```rust
const IFLA_COST: crate::c_ushort = 8u16;
```

### `IFLA_PRIORITY`

```rust
const IFLA_PRIORITY: crate::c_ushort = 9u16;
```

### `IFLA_MASTER`

```rust
const IFLA_MASTER: crate::c_ushort = 10u16;
```

### `IFLA_WIRELESS`

```rust
const IFLA_WIRELESS: crate::c_ushort = 11u16;
```

### `IFLA_PROTINFO`

```rust
const IFLA_PROTINFO: crate::c_ushort = 12u16;
```

### `IFLA_TXQLEN`

```rust
const IFLA_TXQLEN: crate::c_ushort = 13u16;
```

### `IFLA_MAP`

```rust
const IFLA_MAP: crate::c_ushort = 14u16;
```

### `IFLA_WEIGHT`

```rust
const IFLA_WEIGHT: crate::c_ushort = 15u16;
```

### `IFLA_OPERSTATE`

```rust
const IFLA_OPERSTATE: crate::c_ushort = 16u16;
```

### `IFLA_LINKMODE`

```rust
const IFLA_LINKMODE: crate::c_ushort = 17u16;
```

### `IFLA_LINKINFO`

```rust
const IFLA_LINKINFO: crate::c_ushort = 18u16;
```

### `IFLA_NET_NS_PID`

```rust
const IFLA_NET_NS_PID: crate::c_ushort = 19u16;
```

### `IFLA_IFALIAS`

```rust
const IFLA_IFALIAS: crate::c_ushort = 20u16;
```

### `IFLA_NUM_VF`

```rust
const IFLA_NUM_VF: crate::c_ushort = 21u16;
```

### `IFLA_VFINFO_LIST`

```rust
const IFLA_VFINFO_LIST: crate::c_ushort = 22u16;
```

### `IFLA_STATS64`

```rust
const IFLA_STATS64: crate::c_ushort = 23u16;
```

### `IFLA_VF_PORTS`

```rust
const IFLA_VF_PORTS: crate::c_ushort = 24u16;
```

### `IFLA_PORT_SELF`

```rust
const IFLA_PORT_SELF: crate::c_ushort = 25u16;
```

### `IFLA_AF_SPEC`

```rust
const IFLA_AF_SPEC: crate::c_ushort = 26u16;
```

### `IFLA_GROUP`

```rust
const IFLA_GROUP: crate::c_ushort = 27u16;
```

### `IFLA_NET_NS_FD`

```rust
const IFLA_NET_NS_FD: crate::c_ushort = 28u16;
```

### `IFLA_EXT_MASK`

```rust
const IFLA_EXT_MASK: crate::c_ushort = 29u16;
```

### `IFLA_PROMISCUITY`

```rust
const IFLA_PROMISCUITY: crate::c_ushort = 30u16;
```

### `IFLA_NUM_TX_QUEUES`

```rust
const IFLA_NUM_TX_QUEUES: crate::c_ushort = 31u16;
```

### `IFLA_NUM_RX_QUEUES`

```rust
const IFLA_NUM_RX_QUEUES: crate::c_ushort = 32u16;
```

### `IFLA_CARRIER`

```rust
const IFLA_CARRIER: crate::c_ushort = 33u16;
```

### `IFLA_PHYS_PORT_ID`

```rust
const IFLA_PHYS_PORT_ID: crate::c_ushort = 34u16;
```

### `IFLA_CARRIER_CHANGES`

```rust
const IFLA_CARRIER_CHANGES: crate::c_ushort = 35u16;
```

### `IFLA_PHYS_SWITCH_ID`

```rust
const IFLA_PHYS_SWITCH_ID: crate::c_ushort = 36u16;
```

### `IFLA_LINK_NETNSID`

```rust
const IFLA_LINK_NETNSID: crate::c_ushort = 37u16;
```

### `IFLA_PHYS_PORT_NAME`

```rust
const IFLA_PHYS_PORT_NAME: crate::c_ushort = 38u16;
```

### `IFLA_PROTO_DOWN`

```rust
const IFLA_PROTO_DOWN: crate::c_ushort = 39u16;
```

### `IFLA_GSO_MAX_SEGS`

```rust
const IFLA_GSO_MAX_SEGS: crate::c_ushort = 40u16;
```

### `IFLA_GSO_MAX_SIZE`

```rust
const IFLA_GSO_MAX_SIZE: crate::c_ushort = 41u16;
```

### `IFLA_PAD`

```rust
const IFLA_PAD: crate::c_ushort = 42u16;
```

### `IFLA_XDP`

```rust
const IFLA_XDP: crate::c_ushort = 43u16;
```

### `IFLA_EVENT`

```rust
const IFLA_EVENT: crate::c_ushort = 44u16;
```

### `IFLA_NEW_NETNSID`

```rust
const IFLA_NEW_NETNSID: crate::c_ushort = 45u16;
```

### `IFLA_IF_NETNSID`

```rust
const IFLA_IF_NETNSID: crate::c_ushort = 46u16;
```

### `IFLA_TARGET_NETNSID`

```rust
const IFLA_TARGET_NETNSID: crate::c_ushort = 46u16;
```

### `IFLA_CARRIER_UP_COUNT`

```rust
const IFLA_CARRIER_UP_COUNT: crate::c_ushort = 47u16;
```

### `IFLA_CARRIER_DOWN_COUNT`

```rust
const IFLA_CARRIER_DOWN_COUNT: crate::c_ushort = 48u16;
```

### `IFLA_NEW_IFINDEX`

```rust
const IFLA_NEW_IFINDEX: crate::c_ushort = 49u16;
```

### `IFLA_MIN_MTU`

```rust
const IFLA_MIN_MTU: crate::c_ushort = 50u16;
```

### `IFLA_MAX_MTU`

```rust
const IFLA_MAX_MTU: crate::c_ushort = 51u16;
```

### `IFLA_PROP_LIST`

```rust
const IFLA_PROP_LIST: crate::c_ushort = 52u16;
```

### `IFLA_ALT_IFNAME`

```rust
const IFLA_ALT_IFNAME: crate::c_ushort = 53u16;
```

### `IFLA_PERM_ADDRESS`

```rust
const IFLA_PERM_ADDRESS: crate::c_ushort = 54u16;
```

### `IFLA_PROTO_DOWN_REASON`

```rust
const IFLA_PROTO_DOWN_REASON: crate::c_ushort = 55u16;
```

### `IFLA_PARENT_DEV_NAME`

```rust
const IFLA_PARENT_DEV_NAME: crate::c_ushort = 56u16;
```

### `IFLA_PARENT_DEV_BUS_NAME`

```rust
const IFLA_PARENT_DEV_BUS_NAME: crate::c_ushort = 57u16;
```

### `IFLA_GRO_MAX_SIZE`

```rust
const IFLA_GRO_MAX_SIZE: crate::c_ushort = 58u16;
```

### `IFLA_TSO_MAX_SIZE`

```rust
const IFLA_TSO_MAX_SIZE: crate::c_ushort = 59u16;
```

### `IFLA_TSO_MAX_SEGS`

```rust
const IFLA_TSO_MAX_SEGS: crate::c_ushort = 60u16;
```

### `IFLA_ALLMULTI`

```rust
const IFLA_ALLMULTI: crate::c_ushort = 61u16;
```

### `IFLA_INFO_UNSPEC`

```rust
const IFLA_INFO_UNSPEC: crate::c_ushort = 0u16;
```

### `IFLA_INFO_KIND`

```rust
const IFLA_INFO_KIND: crate::c_ushort = 1u16;
```

### `IFLA_INFO_DATA`

```rust
const IFLA_INFO_DATA: crate::c_ushort = 2u16;
```

### `IFLA_INFO_XSTATS`

```rust
const IFLA_INFO_XSTATS: crate::c_ushort = 3u16;
```

### `IFLA_INFO_SLAVE_KIND`

```rust
const IFLA_INFO_SLAVE_KIND: crate::c_ushort = 4u16;
```

### `IFLA_INFO_SLAVE_DATA`

```rust
const IFLA_INFO_SLAVE_DATA: crate::c_ushort = 5u16;
```

### `SEEK_DATA`

```rust
const SEEK_DATA: crate::c_int = 3i32;
```

### `SEEK_HOLE`

```rust
const SEEK_HOLE: crate::c_int = 4i32;
```

### `ST_RDONLY`

```rust
const ST_RDONLY: crate::c_ulong = 1u64;
```

### `ST_NOSUID`

```rust
const ST_NOSUID: crate::c_ulong = 2u64;
```

### `ST_NODEV`

```rust
const ST_NODEV: crate::c_ulong = 4u64;
```

### `ST_NOEXEC`

```rust
const ST_NOEXEC: crate::c_ulong = 8u64;
```

### `ST_SYNCHRONOUS`

```rust
const ST_SYNCHRONOUS: crate::c_ulong = 16u64;
```

### `ST_MANDLOCK`

```rust
const ST_MANDLOCK: crate::c_ulong = 64u64;
```

### `ST_WRITE`

```rust
const ST_WRITE: crate::c_ulong = 128u64;
```

### `ST_APPEND`

```rust
const ST_APPEND: crate::c_ulong = 256u64;
```

### `ST_IMMUTABLE`

```rust
const ST_IMMUTABLE: crate::c_ulong = 512u64;
```

### `ST_NOATIME`

```rust
const ST_NOATIME: crate::c_ulong = 1_024u64;
```

### `ST_NODIRATIME`

```rust
const ST_NODIRATIME: crate::c_ulong = 2_048u64;
```

### `RTLD_NEXT`

```rust
const RTLD_NEXT: *mut crate::c_void = {0xffffffffffffffff as *mut core::ffi::c_void};
```

### `RTLD_DEFAULT`

```rust
const RTLD_DEFAULT: *mut crate::c_void = {0x0 as *mut core::ffi::c_void};
```

### `RTLD_NODELETE`

```rust
const RTLD_NODELETE: crate::c_int = 4_096i32;
```

### `RTLD_NOW`

```rust
const RTLD_NOW: crate::c_int = 2i32;
```

### `AT_EACCESS`

```rust
const AT_EACCESS: crate::c_int = 512i32;
```

### `MPOL_DEFAULT`

```rust
const MPOL_DEFAULT: crate::c_int = 0i32;
```

### `MPOL_PREFERRED`

```rust
const MPOL_PREFERRED: crate::c_int = 1i32;
```

### `MPOL_BIND`

```rust
const MPOL_BIND: crate::c_int = 2i32;
```

### `MPOL_INTERLEAVE`

```rust
const MPOL_INTERLEAVE: crate::c_int = 3i32;
```

### `MPOL_LOCAL`

```rust
const MPOL_LOCAL: crate::c_int = 4i32;
```

### `MPOL_F_NUMA_BALANCING`

```rust
const MPOL_F_NUMA_BALANCING: crate::c_int = 8_192i32;
```

### `MPOL_F_RELATIVE_NODES`

```rust
const MPOL_F_RELATIVE_NODES: crate::c_int = 16_384i32;
```

### `MPOL_F_STATIC_NODES`

```rust
const MPOL_F_STATIC_NODES: crate::c_int = 32_768i32;
```

### `MEMBARRIER_CMD_QUERY`

```rust
const MEMBARRIER_CMD_QUERY: crate::c_int = 0i32;
```

### `MEMBARRIER_CMD_GLOBAL`

```rust
const MEMBARRIER_CMD_GLOBAL: crate::c_int = 1i32;
```

### `MEMBARRIER_CMD_GLOBAL_EXPEDITED`

```rust
const MEMBARRIER_CMD_GLOBAL_EXPEDITED: crate::c_int = 2i32;
```

### `MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED`

```rust
const MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED: crate::c_int = 4i32;
```

### `MEMBARRIER_CMD_PRIVATE_EXPEDITED`

```rust
const MEMBARRIER_CMD_PRIVATE_EXPEDITED: crate::c_int = 8i32;
```

### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED`

```rust
const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED: crate::c_int = 16i32;
```

### `MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE`

```rust
const MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE: crate::c_int = 32i32;
```

### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE`

```rust
const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE: crate::c_int = 64i32;
```

### `MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ`

```rust
const MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ: crate::c_int = 128i32;
```

### `MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ`

```rust
const MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ: crate::c_int = 256i32;
```

### `PTHREAD_MUTEX_INITIALIZER`

```rust
const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t;
```

### `PTHREAD_COND_INITIALIZER`

```rust
const PTHREAD_COND_INITIALIZER: pthread_cond_t;
```

### `PTHREAD_RWLOCK_INITIALIZER`

```rust
const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t;
```

### `PTHREAD_BARRIER_SERIAL_THREAD`

```rust
const PTHREAD_BARRIER_SERIAL_THREAD: crate::c_int = -1i32;
```

### `PTHREAD_ONCE_INIT`

```rust
const PTHREAD_ONCE_INIT: pthread_once_t = 0i32;
```

### `PTHREAD_MUTEX_NORMAL`

```rust
const PTHREAD_MUTEX_NORMAL: crate::c_int = 0i32;
```

### `PTHREAD_MUTEX_RECURSIVE`

```rust
const PTHREAD_MUTEX_RECURSIVE: crate::c_int = 1i32;
```

### `PTHREAD_MUTEX_ERRORCHECK`

```rust
const PTHREAD_MUTEX_ERRORCHECK: crate::c_int = 2i32;
```

### `PTHREAD_MUTEX_DEFAULT`

```rust
const PTHREAD_MUTEX_DEFAULT: crate::c_int = 0i32;
```

### `PTHREAD_MUTEX_STALLED`

```rust
const PTHREAD_MUTEX_STALLED: crate::c_int = 0i32;
```

### `PTHREAD_MUTEX_ROBUST`

```rust
const PTHREAD_MUTEX_ROBUST: crate::c_int = 1i32;
```

### `PTHREAD_PRIO_NONE`

```rust
const PTHREAD_PRIO_NONE: crate::c_int = 0i32;
```

### `PTHREAD_PRIO_INHERIT`

```rust
const PTHREAD_PRIO_INHERIT: crate::c_int = 1i32;
```

### `PTHREAD_PRIO_PROTECT`

```rust
const PTHREAD_PRIO_PROTECT: crate::c_int = 2i32;
```

### `PTHREAD_PROCESS_PRIVATE`

```rust
const PTHREAD_PROCESS_PRIVATE: crate::c_int = 0i32;
```

### `PTHREAD_PROCESS_SHARED`

```rust
const PTHREAD_PROCESS_SHARED: crate::c_int = 1i32;
```

### `PTHREAD_INHERIT_SCHED`

```rust
const PTHREAD_INHERIT_SCHED: crate::c_int = 0i32;
```

### `PTHREAD_EXPLICIT_SCHED`

```rust
const PTHREAD_EXPLICIT_SCHED: crate::c_int = 1i32;
```

### `__SIZEOF_PTHREAD_COND_T`

```rust
const __SIZEOF_PTHREAD_COND_T: usize = 48usize;
```

### `RENAME_NOREPLACE`

```rust
const RENAME_NOREPLACE: crate::c_uint = 1u32;
```

### `RENAME_EXCHANGE`

```rust
const RENAME_EXCHANGE: crate::c_uint = 2u32;
```

### `RENAME_WHITEOUT`

```rust
const RENAME_WHITEOUT: crate::c_uint = 4u32;
```

### `IPPROTO_MAX`

```rust
const IPPROTO_MAX: crate::c_int = 256i32;
```

### `IPC_PRIVATE`

```rust
const IPC_PRIVATE: crate::key_t = 0i32;
```

### `IPC_CREAT`

```rust
const IPC_CREAT: crate::c_int = 512i32;
```

### `IPC_EXCL`

```rust
const IPC_EXCL: crate::c_int = 1_024i32;
```

### `IPC_NOWAIT`

```rust
const IPC_NOWAIT: crate::c_int = 2_048i32;
```

### `IPC_RMID`

```rust
const IPC_RMID: crate::c_int = 0i32;
```

### `IPC_SET`

```rust
const IPC_SET: crate::c_int = 1i32;
```

### `IPC_STAT`

```rust
const IPC_STAT: crate::c_int = 2i32;
```

### `IPC_INFO`

```rust
const IPC_INFO: crate::c_int = 3i32;
```

### `MSG_STAT`

```rust
const MSG_STAT: crate::c_int = 11i32;
```

### `MSG_INFO`

```rust
const MSG_INFO: crate::c_int = 12i32;
```

### `MSG_NOTIFICATION`

```rust
const MSG_NOTIFICATION: crate::c_int = 32_768i32;
```

### `MSG_NOERROR`

```rust
const MSG_NOERROR: crate::c_int = 4_096i32;
```

### `MSG_EXCEPT`

```rust
const MSG_EXCEPT: crate::c_int = 8_192i32;
```

### `MSG_ZEROCOPY`

```rust
const MSG_ZEROCOPY: crate::c_int = 67_108_864i32;
```

### `SEM_UNDO`

```rust
const SEM_UNDO: crate::c_int = 4_096i32;
```

### `GETPID`

```rust
const GETPID: crate::c_int = 11i32;
```

### `GETVAL`

```rust
const GETVAL: crate::c_int = 12i32;
```

### `GETALL`

```rust
const GETALL: crate::c_int = 13i32;
```

### `GETNCNT`

```rust
const GETNCNT: crate::c_int = 14i32;
```

### `GETZCNT`

```rust
const GETZCNT: crate::c_int = 15i32;
```

### `SETVAL`

```rust
const SETVAL: crate::c_int = 16i32;
```

### `SETALL`

```rust
const SETALL: crate::c_int = 17i32;
```

### `SEM_STAT`

```rust
const SEM_STAT: crate::c_int = 18i32;
```

### `SEM_INFO`

```rust
const SEM_INFO: crate::c_int = 19i32;
```

### `SEM_STAT_ANY`

```rust
const SEM_STAT_ANY: crate::c_int = 20i32;
```

### `SHM_R`

```rust
const SHM_R: crate::c_int = 256i32;
```

### `SHM_W`

```rust
const SHM_W: crate::c_int = 128i32;
```

### `SHM_RDONLY`

```rust
const SHM_RDONLY: crate::c_int = 4_096i32;
```

### `SHM_RND`

```rust
const SHM_RND: crate::c_int = 8_192i32;
```

### `SHM_REMAP`

```rust
const SHM_REMAP: crate::c_int = 16_384i32;
```

### `SHM_LOCK`

```rust
const SHM_LOCK: crate::c_int = 11i32;
```

### `SHM_UNLOCK`

```rust
const SHM_UNLOCK: crate::c_int = 12i32;
```

### `SHM_HUGETLB`

```rust
const SHM_HUGETLB: crate::c_int = 2_048i32;
```

### `SHM_NORESERVE`

```rust
const SHM_NORESERVE: crate::c_int = 4_096i32;
```

### `QFMT_VFS_OLD`

```rust
const QFMT_VFS_OLD: crate::c_int = 1i32;
```

### `QFMT_VFS_V0`

```rust
const QFMT_VFS_V0: crate::c_int = 2i32;
```

### `QFMT_VFS_V1`

```rust
const QFMT_VFS_V1: crate::c_int = 4i32;
```

### `EFD_SEMAPHORE`

```rust
const EFD_SEMAPHORE: crate::c_int = 1i32;
```

### `LOG_NFACILITIES`

```rust
const LOG_NFACILITIES: crate::c_int = 24i32;
```

### `SEM_FAILED`

```rust
const SEM_FAILED: *mut crate::sem_t = {0x0 as *mut unix::linux_like::linux::gnu::sem_t};
```

### `RB_AUTOBOOT`

```rust
const RB_AUTOBOOT: crate::c_int = 19_088_743i32;
```

### `RB_HALT_SYSTEM`

```rust
const RB_HALT_SYSTEM: crate::c_int = -839_974_621i32;
```

### `RB_ENABLE_CAD`

```rust
const RB_ENABLE_CAD: crate::c_int = -1_985_229_329i32;
```

### `RB_DISABLE_CAD`

```rust
const RB_DISABLE_CAD: crate::c_int = 0i32;
```

### `RB_POWER_OFF`

```rust
const RB_POWER_OFF: crate::c_int = 1_126_301_404i32;
```

### `RB_SW_SUSPEND`

```rust
const RB_SW_SUSPEND: crate::c_int = -805_241_630i32;
```

### `RB_KEXEC`

```rust
const RB_KEXEC: crate::c_int = 1_163_412_803i32;
```

### `AI_PASSIVE`

```rust
const AI_PASSIVE: crate::c_int = 1i32;
```

### `AI_CANONNAME`

```rust
const AI_CANONNAME: crate::c_int = 2i32;
```

### `AI_NUMERICHOST`

```rust
const AI_NUMERICHOST: crate::c_int = 4i32;
```

### `AI_V4MAPPED`

```rust
const AI_V4MAPPED: crate::c_int = 8i32;
```

### `AI_ALL`

```rust
const AI_ALL: crate::c_int = 16i32;
```

### `AI_ADDRCONFIG`

```rust
const AI_ADDRCONFIG: crate::c_int = 32i32;
```

### `AI_NUMERICSERV`

```rust
const AI_NUMERICSERV: crate::c_int = 1_024i32;
```

### `EAI_BADFLAGS`

```rust
const EAI_BADFLAGS: crate::c_int = -1i32;
```

### `EAI_NONAME`

```rust
const EAI_NONAME: crate::c_int = -2i32;
```

### `EAI_AGAIN`

```rust
const EAI_AGAIN: crate::c_int = -3i32;
```

### `EAI_FAIL`

```rust
const EAI_FAIL: crate::c_int = -4i32;
```

### `EAI_NODATA`

```rust
const EAI_NODATA: crate::c_int = -5i32;
```

### `EAI_FAMILY`

```rust
const EAI_FAMILY: crate::c_int = -6i32;
```

### `EAI_SOCKTYPE`

```rust
const EAI_SOCKTYPE: crate::c_int = -7i32;
```

### `EAI_SERVICE`

```rust
const EAI_SERVICE: crate::c_int = -8i32;
```

### `EAI_MEMORY`

```rust
const EAI_MEMORY: crate::c_int = -10i32;
```

### `EAI_SYSTEM`

```rust
const EAI_SYSTEM: crate::c_int = -11i32;
```

### `EAI_OVERFLOW`

```rust
const EAI_OVERFLOW: crate::c_int = -12i32;
```

### `NI_NUMERICHOST`

```rust
const NI_NUMERICHOST: crate::c_int = 1i32;
```

### `NI_NUMERICSERV`

```rust
const NI_NUMERICSERV: crate::c_int = 2i32;
```

### `NI_NOFQDN`

```rust
const NI_NOFQDN: crate::c_int = 4i32;
```

### `NI_NAMEREQD`

```rust
const NI_NAMEREQD: crate::c_int = 8i32;
```

### `NI_DGRAM`

```rust
const NI_DGRAM: crate::c_int = 16i32;
```

### `NI_IDN`

```rust
const NI_IDN: crate::c_int = 32i32;
```

### `SYNC_FILE_RANGE_WAIT_BEFORE`

```rust
const SYNC_FILE_RANGE_WAIT_BEFORE: crate::c_uint = 1u32;
```

### `SYNC_FILE_RANGE_WRITE`

```rust
const SYNC_FILE_RANGE_WRITE: crate::c_uint = 2u32;
```

### `SYNC_FILE_RANGE_WAIT_AFTER`

```rust
const SYNC_FILE_RANGE_WAIT_AFTER: crate::c_uint = 4u32;
```

### `AIO_CANCELED`

```rust
const AIO_CANCELED: crate::c_int = 0i32;
```

### `AIO_NOTCANCELED`

```rust
const AIO_NOTCANCELED: crate::c_int = 1i32;
```

### `AIO_ALLDONE`

```rust
const AIO_ALLDONE: crate::c_int = 2i32;
```

### `LIO_READ`

```rust
const LIO_READ: crate::c_int = 0i32;
```

### `LIO_WRITE`

```rust
const LIO_WRITE: crate::c_int = 1i32;
```

### `LIO_NOP`

```rust
const LIO_NOP: crate::c_int = 2i32;
```

### `LIO_WAIT`

```rust
const LIO_WAIT: crate::c_int = 0i32;
```

### `LIO_NOWAIT`

```rust
const LIO_NOWAIT: crate::c_int = 1i32;
```

### `RUSAGE_THREAD`

```rust
const RUSAGE_THREAD: crate::c_int = 1i32;
```

### `MSG_COPY`

```rust
const MSG_COPY: crate::c_int = 16_384i32;
```

### `SHM_EXEC`

```rust
const SHM_EXEC: crate::c_int = 32_768i32;
```

### `IPV6_MULTICAST_ALL`

```rust
const IPV6_MULTICAST_ALL: crate::c_int = 29i32;
```

### `IPV6_ROUTER_ALERT_ISOLATE`

```rust
const IPV6_ROUTER_ALERT_ISOLATE: crate::c_int = 30i32;
```

### `PACKET_MR_UNICAST`

```rust
const PACKET_MR_UNICAST: crate::c_int = 3i32;
```

### `PTRACE_EVENT_STOP`

```rust
const PTRACE_EVENT_STOP: crate::c_int = 128i32;
```

### `UDP_SEGMENT`

```rust
const UDP_SEGMENT: crate::c_int = 103i32;
```

### `UDP_GRO`

```rust
const UDP_GRO: crate::c_int = 104i32;
```

### `MREMAP_MAYMOVE`

```rust
const MREMAP_MAYMOVE: crate::c_int = 1i32;
```

### `MREMAP_FIXED`

```rust
const MREMAP_FIXED: crate::c_int = 2i32;
```

### `MREMAP_DONTUNMAP`

```rust
const MREMAP_DONTUNMAP: crate::c_int = 4i32;
```

### `NSIO`

```rust
const NSIO: crate::c_uint = 183u32;
```

### `NS_GET_USERNS`

```rust
const NS_GET_USERNS: crate::c_ulong = 46_849u64;
```

### `NS_GET_PARENT`

```rust
const NS_GET_PARENT: crate::c_ulong = 46_850u64;
```

### `NS_GET_NSTYPE`

```rust
const NS_GET_NSTYPE: crate::c_ulong = 46_851u64;
```

### `NS_GET_OWNER_UID`

```rust
const NS_GET_OWNER_UID: crate::c_ulong = 46_852u64;
```

### `NS_GET_MNTNS_ID`

```rust
const NS_GET_MNTNS_ID: crate::c_ulong = 2_148_054_789u64;
```

### `NS_GET_PID_FROM_PIDNS`

```rust
const NS_GET_PID_FROM_PIDNS: crate::c_ulong = 2_147_792_646u64;
```

### `NS_GET_TGID_FROM_PIDNS`

```rust
const NS_GET_TGID_FROM_PIDNS: crate::c_ulong = 2_147_792_647u64;
```

### `NS_GET_PID_IN_PIDNS`

```rust
const NS_GET_PID_IN_PIDNS: crate::c_ulong = 2_147_792_648u64;
```

### `NS_GET_TGID_IN_PIDNS`

```rust
const NS_GET_TGID_IN_PIDNS: crate::c_ulong = 2_147_792_649u64;
```

### `MNT_NS_INFO_SIZE_VER0`

```rust
const MNT_NS_INFO_SIZE_VER0: crate::c_ulong = 16u64;
```

### `NS_MNT_GET_INFO`

```rust
const NS_MNT_GET_INFO: crate::c_ulong = 2_148_579_082u64;
```

### `NS_MNT_GET_NEXT`

```rust
const NS_MNT_GET_NEXT: crate::c_ulong = 2_148_579_083u64;
```

### `NS_MNT_GET_PREV`

```rust
const NS_MNT_GET_PREV: crate::c_ulong = 2_148_579_084u64;
```

### `PIDFD_NONBLOCK`

```rust
const PIDFD_NONBLOCK: crate::c_uint = 2_048u32;
```

### `PIDFD_THREAD`

```rust
const PIDFD_THREAD: crate::c_uint = 128u32;
```

### `PIDFD_SIGNAL_THREAD`

```rust
const PIDFD_SIGNAL_THREAD: crate::c_uint = 1u32;
```

### `PIDFD_SIGNAL_THREAD_GROUP`

```rust
const PIDFD_SIGNAL_THREAD_GROUP: crate::c_uint = 2u32;
```

### `PIDFD_SIGNAL_PROCESS_GROUP`

```rust
const PIDFD_SIGNAL_PROCESS_GROUP: crate::c_uint = 4u32;
```

### `PIDFD_INFO_PID`

```rust
const PIDFD_INFO_PID: crate::c_uint = 1u32;
```

### `PIDFD_INFO_CREDS`

```rust
const PIDFD_INFO_CREDS: crate::c_uint = 2u32;
```

### `PIDFD_INFO_CGROUPID`

```rust
const PIDFD_INFO_CGROUPID: crate::c_uint = 4u32;
```

### `PIDFD_INFO_EXIT`

```rust
const PIDFD_INFO_EXIT: crate::c_uint = 8u32;
```

### `PIDFD_INFO_SIZE_VER0`

```rust
const PIDFD_INFO_SIZE_VER0: crate::c_uint = 64u32;
```

### `PIDFS_IOCTL_MAGIC`

```rust
const PIDFS_IOCTL_MAGIC: crate::c_uint = 255u32;
```

### `PIDFD_GET_CGROUP_NAMESPACE`

```rust
const PIDFD_GET_CGROUP_NAMESPACE: crate::c_ulong = 65_281u64;
```

### `PIDFD_GET_IPC_NAMESPACE`

```rust
const PIDFD_GET_IPC_NAMESPACE: crate::c_ulong = 65_282u64;
```

### `PIDFD_GET_MNT_NAMESPACE`

```rust
const PIDFD_GET_MNT_NAMESPACE: crate::c_ulong = 65_283u64;
```

### `PIDFD_GET_NET_NAMESPACE`

```rust
const PIDFD_GET_NET_NAMESPACE: crate::c_ulong = 65_284u64;
```

### `PIDFD_GET_PID_NAMESPACE`

```rust
const PIDFD_GET_PID_NAMESPACE: crate::c_ulong = 65_285u64;
```

### `PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE`

```rust
const PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE: crate::c_ulong = 65_286u64;
```

### `PIDFD_GET_TIME_NAMESPACE`

```rust
const PIDFD_GET_TIME_NAMESPACE: crate::c_ulong = 65_287u64;
```

### `PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE`

```rust
const PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE: crate::c_ulong = 65_288u64;
```

### `PIDFD_GET_USER_NAMESPACE`

```rust
const PIDFD_GET_USER_NAMESPACE: crate::c_ulong = 65_289u64;
```

### `PIDFD_GET_UTS_NAMESPACE`

```rust
const PIDFD_GET_UTS_NAMESPACE: crate::c_ulong = 65_290u64;
```

### `PIDFD_GET_INFO`

```rust
const PIDFD_GET_INFO: crate::c_ulong = 3_225_485_067u64;
```

### `PR_SET_PDEATHSIG`

```rust
const PR_SET_PDEATHSIG: crate::c_int = 1i32;
```

### `PR_GET_PDEATHSIG`

```rust
const PR_GET_PDEATHSIG: crate::c_int = 2i32;
```

### `PR_GET_DUMPABLE`

```rust
const PR_GET_DUMPABLE: crate::c_int = 3i32;
```

### `PR_SET_DUMPABLE`

```rust
const PR_SET_DUMPABLE: crate::c_int = 4i32;
```

### `PR_GET_UNALIGN`

```rust
const PR_GET_UNALIGN: crate::c_int = 5i32;
```

### `PR_SET_UNALIGN`

```rust
const PR_SET_UNALIGN: crate::c_int = 6i32;
```

### `PR_UNALIGN_NOPRINT`

```rust
const PR_UNALIGN_NOPRINT: crate::c_int = 1i32;
```

### `PR_UNALIGN_SIGBUS`

```rust
const PR_UNALIGN_SIGBUS: crate::c_int = 2i32;
```

### `PR_GET_KEEPCAPS`

```rust
const PR_GET_KEEPCAPS: crate::c_int = 7i32;
```

### `PR_SET_KEEPCAPS`

```rust
const PR_SET_KEEPCAPS: crate::c_int = 8i32;
```

### `PR_GET_FPEMU`

```rust
const PR_GET_FPEMU: crate::c_int = 9i32;
```

### `PR_SET_FPEMU`

```rust
const PR_SET_FPEMU: crate::c_int = 10i32;
```

### `PR_FPEMU_NOPRINT`

```rust
const PR_FPEMU_NOPRINT: crate::c_int = 1i32;
```

### `PR_FPEMU_SIGFPE`

```rust
const PR_FPEMU_SIGFPE: crate::c_int = 2i32;
```

### `PR_GET_FPEXC`

```rust
const PR_GET_FPEXC: crate::c_int = 11i32;
```

### `PR_SET_FPEXC`

```rust
const PR_SET_FPEXC: crate::c_int = 12i32;
```

### `PR_FP_EXC_SW_ENABLE`

```rust
const PR_FP_EXC_SW_ENABLE: crate::c_int = 128i32;
```

### `PR_FP_EXC_DIV`

```rust
const PR_FP_EXC_DIV: crate::c_int = 65_536i32;
```

### `PR_FP_EXC_OVF`

```rust
const PR_FP_EXC_OVF: crate::c_int = 131_072i32;
```

### `PR_FP_EXC_UND`

```rust
const PR_FP_EXC_UND: crate::c_int = 262_144i32;
```

### `PR_FP_EXC_RES`

```rust
const PR_FP_EXC_RES: crate::c_int = 524_288i32;
```

### `PR_FP_EXC_INV`

```rust
const PR_FP_EXC_INV: crate::c_int = 1_048_576i32;
```

### `PR_FP_EXC_DISABLED`

```rust
const PR_FP_EXC_DISABLED: crate::c_int = 0i32;
```

### `PR_FP_EXC_NONRECOV`

```rust
const PR_FP_EXC_NONRECOV: crate::c_int = 1i32;
```

### `PR_FP_EXC_ASYNC`

```rust
const PR_FP_EXC_ASYNC: crate::c_int = 2i32;
```

### `PR_FP_EXC_PRECISE`

```rust
const PR_FP_EXC_PRECISE: crate::c_int = 3i32;
```

### `PR_GET_TIMING`

```rust
const PR_GET_TIMING: crate::c_int = 13i32;
```

### `PR_SET_TIMING`

```rust
const PR_SET_TIMING: crate::c_int = 14i32;
```

### `PR_TIMING_STATISTICAL`

```rust
const PR_TIMING_STATISTICAL: crate::c_int = 0i32;
```

### `PR_TIMING_TIMESTAMP`

```rust
const PR_TIMING_TIMESTAMP: crate::c_int = 1i32;
```

### `PR_SET_NAME`

```rust
const PR_SET_NAME: crate::c_int = 15i32;
```

### `PR_GET_NAME`

```rust
const PR_GET_NAME: crate::c_int = 16i32;
```

### `PR_GET_ENDIAN`

```rust
const PR_GET_ENDIAN: crate::c_int = 19i32;
```

### `PR_SET_ENDIAN`

```rust
const PR_SET_ENDIAN: crate::c_int = 20i32;
```

### `PR_ENDIAN_BIG`

```rust
const PR_ENDIAN_BIG: crate::c_int = 0i32;
```

### `PR_ENDIAN_LITTLE`

```rust
const PR_ENDIAN_LITTLE: crate::c_int = 1i32;
```

### `PR_ENDIAN_PPC_LITTLE`

```rust
const PR_ENDIAN_PPC_LITTLE: crate::c_int = 2i32;
```

### `PR_GET_SECCOMP`

```rust
const PR_GET_SECCOMP: crate::c_int = 21i32;
```

### `PR_SET_SECCOMP`

```rust
const PR_SET_SECCOMP: crate::c_int = 22i32;
```

### `PR_CAPBSET_READ`

```rust
const PR_CAPBSET_READ: crate::c_int = 23i32;
```

### `PR_CAPBSET_DROP`

```rust
const PR_CAPBSET_DROP: crate::c_int = 24i32;
```

### `PR_GET_TSC`

```rust
const PR_GET_TSC: crate::c_int = 25i32;
```

### `PR_SET_TSC`

```rust
const PR_SET_TSC: crate::c_int = 26i32;
```

### `PR_TSC_ENABLE`

```rust
const PR_TSC_ENABLE: crate::c_int = 1i32;
```

### `PR_TSC_SIGSEGV`

```rust
const PR_TSC_SIGSEGV: crate::c_int = 2i32;
```

### `PR_GET_SECUREBITS`

```rust
const PR_GET_SECUREBITS: crate::c_int = 27i32;
```

### `PR_SET_SECUREBITS`

```rust
const PR_SET_SECUREBITS: crate::c_int = 28i32;
```

### `PR_SET_TIMERSLACK`

```rust
const PR_SET_TIMERSLACK: crate::c_int = 29i32;
```

### `PR_GET_TIMERSLACK`

```rust
const PR_GET_TIMERSLACK: crate::c_int = 30i32;
```

### `PR_TASK_PERF_EVENTS_DISABLE`

```rust
const PR_TASK_PERF_EVENTS_DISABLE: crate::c_int = 31i32;
```

### `PR_TASK_PERF_EVENTS_ENABLE`

```rust
const PR_TASK_PERF_EVENTS_ENABLE: crate::c_int = 32i32;
```

### `PR_MCE_KILL`

```rust
const PR_MCE_KILL: crate::c_int = 33i32;
```

### `PR_MCE_KILL_CLEAR`

```rust
const PR_MCE_KILL_CLEAR: crate::c_int = 0i32;
```

### `PR_MCE_KILL_SET`

```rust
const PR_MCE_KILL_SET: crate::c_int = 1i32;
```

### `PR_MCE_KILL_LATE`

```rust
const PR_MCE_KILL_LATE: crate::c_int = 0i32;
```

### `PR_MCE_KILL_EARLY`

```rust
const PR_MCE_KILL_EARLY: crate::c_int = 1i32;
```

### `PR_MCE_KILL_DEFAULT`

```rust
const PR_MCE_KILL_DEFAULT: crate::c_int = 2i32;
```

### `PR_MCE_KILL_GET`

```rust
const PR_MCE_KILL_GET: crate::c_int = 34i32;
```

### `PR_SET_MM`

```rust
const PR_SET_MM: crate::c_int = 35i32;
```

### `PR_SET_MM_START_CODE`

```rust
const PR_SET_MM_START_CODE: crate::c_int = 1i32;
```

### `PR_SET_MM_END_CODE`

```rust
const PR_SET_MM_END_CODE: crate::c_int = 2i32;
```

### `PR_SET_MM_START_DATA`

```rust
const PR_SET_MM_START_DATA: crate::c_int = 3i32;
```

### `PR_SET_MM_END_DATA`

```rust
const PR_SET_MM_END_DATA: crate::c_int = 4i32;
```

### `PR_SET_MM_START_STACK`

```rust
const PR_SET_MM_START_STACK: crate::c_int = 5i32;
```

### `PR_SET_MM_START_BRK`

```rust
const PR_SET_MM_START_BRK: crate::c_int = 6i32;
```

### `PR_SET_MM_BRK`

```rust
const PR_SET_MM_BRK: crate::c_int = 7i32;
```

### `PR_SET_MM_ARG_START`

```rust
const PR_SET_MM_ARG_START: crate::c_int = 8i32;
```

### `PR_SET_MM_ARG_END`

```rust
const PR_SET_MM_ARG_END: crate::c_int = 9i32;
```

### `PR_SET_MM_ENV_START`

```rust
const PR_SET_MM_ENV_START: crate::c_int = 10i32;
```

### `PR_SET_MM_ENV_END`

```rust
const PR_SET_MM_ENV_END: crate::c_int = 11i32;
```

### `PR_SET_MM_AUXV`

```rust
const PR_SET_MM_AUXV: crate::c_int = 12i32;
```

### `PR_SET_MM_EXE_FILE`

```rust
const PR_SET_MM_EXE_FILE: crate::c_int = 13i32;
```

### `PR_SET_MM_MAP`

```rust
const PR_SET_MM_MAP: crate::c_int = 14i32;
```

### `PR_SET_MM_MAP_SIZE`

```rust
const PR_SET_MM_MAP_SIZE: crate::c_int = 15i32;
```

### `PR_SET_PTRACER`

```rust
const PR_SET_PTRACER: crate::c_int = 1_499_557_217i32;
```

### `PR_SET_PTRACER_ANY`

```rust
const PR_SET_PTRACER_ANY: crate::c_ulong = 18_446_744_073_709_551_615u64;
```

### `PR_SET_CHILD_SUBREAPER`

```rust
const PR_SET_CHILD_SUBREAPER: crate::c_int = 36i32;
```

### `PR_GET_CHILD_SUBREAPER`

```rust
const PR_GET_CHILD_SUBREAPER: crate::c_int = 37i32;
```

### `PR_SET_NO_NEW_PRIVS`

```rust
const PR_SET_NO_NEW_PRIVS: crate::c_int = 38i32;
```

### `PR_GET_NO_NEW_PRIVS`

```rust
const PR_GET_NO_NEW_PRIVS: crate::c_int = 39i32;
```

### `PR_SET_MDWE`

```rust
const PR_SET_MDWE: crate::c_int = 65i32;
```

### `PR_GET_MDWE`

```rust
const PR_GET_MDWE: crate::c_int = 66i32;
```

### `PR_MDWE_REFUSE_EXEC_GAIN`

```rust
const PR_MDWE_REFUSE_EXEC_GAIN: crate::c_uint = 1u32;
```

### `PR_MDWE_NO_INHERIT`

```rust
const PR_MDWE_NO_INHERIT: crate::c_uint = 2u32;
```

### `PR_GET_TID_ADDRESS`

```rust
const PR_GET_TID_ADDRESS: crate::c_int = 40i32;
```

### `PR_SET_THP_DISABLE`

```rust
const PR_SET_THP_DISABLE: crate::c_int = 41i32;
```

### `PR_GET_THP_DISABLE`

```rust
const PR_GET_THP_DISABLE: crate::c_int = 42i32;
```

### `PR_MPX_ENABLE_MANAGEMENT`

```rust
const PR_MPX_ENABLE_MANAGEMENT: crate::c_int = 43i32;
```

### `PR_MPX_DISABLE_MANAGEMENT`

```rust
const PR_MPX_DISABLE_MANAGEMENT: crate::c_int = 44i32;
```

### `PR_SET_FP_MODE`

```rust
const PR_SET_FP_MODE: crate::c_int = 45i32;
```

### `PR_GET_FP_MODE`

```rust
const PR_GET_FP_MODE: crate::c_int = 46i32;
```

### `PR_FP_MODE_FR`

```rust
const PR_FP_MODE_FR: crate::c_int = 1i32;
```

### `PR_FP_MODE_FRE`

```rust
const PR_FP_MODE_FRE: crate::c_int = 2i32;
```

### `PR_CAP_AMBIENT`

```rust
const PR_CAP_AMBIENT: crate::c_int = 47i32;
```

### `PR_CAP_AMBIENT_IS_SET`

```rust
const PR_CAP_AMBIENT_IS_SET: crate::c_int = 1i32;
```

### `PR_CAP_AMBIENT_RAISE`

```rust
const PR_CAP_AMBIENT_RAISE: crate::c_int = 2i32;
```

### `PR_CAP_AMBIENT_LOWER`

```rust
const PR_CAP_AMBIENT_LOWER: crate::c_int = 3i32;
```

### `PR_CAP_AMBIENT_CLEAR_ALL`

```rust
const PR_CAP_AMBIENT_CLEAR_ALL: crate::c_int = 4i32;
```

### `PR_SET_VMA`

```rust
const PR_SET_VMA: crate::c_int = 1_398_164_801i32;
```

### `PR_SET_VMA_ANON_NAME`

```rust
const PR_SET_VMA_ANON_NAME: crate::c_int = 0i32;
```

### `PR_SCHED_CORE`

```rust
const PR_SCHED_CORE: crate::c_int = 62i32;
```

### `PR_SCHED_CORE_GET`

```rust
const PR_SCHED_CORE_GET: crate::c_int = 0i32;
```

### `PR_SCHED_CORE_CREATE`

```rust
const PR_SCHED_CORE_CREATE: crate::c_int = 1i32;
```

### `PR_SCHED_CORE_SHARE_TO`

```rust
const PR_SCHED_CORE_SHARE_TO: crate::c_int = 2i32;
```

### `PR_SCHED_CORE_SHARE_FROM`

```rust
const PR_SCHED_CORE_SHARE_FROM: crate::c_int = 3i32;
```

### `PR_SCHED_CORE_MAX`

```rust
const PR_SCHED_CORE_MAX: crate::c_int = 4i32;
```

### `PR_SCHED_CORE_SCOPE_THREAD`

```rust
const PR_SCHED_CORE_SCOPE_THREAD: crate::c_int = 0i32;
```

### `PR_SCHED_CORE_SCOPE_THREAD_GROUP`

```rust
const PR_SCHED_CORE_SCOPE_THREAD_GROUP: crate::c_int = 1i32;
```

### `PR_SCHED_CORE_SCOPE_PROCESS_GROUP`

```rust
const PR_SCHED_CORE_SCOPE_PROCESS_GROUP: crate::c_int = 2i32;
```

### `GRND_NONBLOCK`

```rust
const GRND_NONBLOCK: crate::c_uint = 1u32;
```

### `GRND_RANDOM`

```rust
const GRND_RANDOM: crate::c_uint = 2u32;
```

### `GRND_INSECURE`

```rust
const GRND_INSECURE: crate::c_uint = 4u32;
```

### `SECCOMP_MODE_DISABLED`

```rust
const SECCOMP_MODE_DISABLED: crate::c_uint = 0u32;
```

### `SECCOMP_MODE_STRICT`

```rust
const SECCOMP_MODE_STRICT: crate::c_uint = 1u32;
```

### `SECCOMP_MODE_FILTER`

```rust
const SECCOMP_MODE_FILTER: crate::c_uint = 2u32;
```

### `SECCOMP_SET_MODE_STRICT`

```rust
const SECCOMP_SET_MODE_STRICT: crate::c_uint = 0u32;
```

### `SECCOMP_SET_MODE_FILTER`

```rust
const SECCOMP_SET_MODE_FILTER: crate::c_uint = 1u32;
```

### `SECCOMP_GET_ACTION_AVAIL`

```rust
const SECCOMP_GET_ACTION_AVAIL: crate::c_uint = 2u32;
```

### `SECCOMP_GET_NOTIF_SIZES`

```rust
const SECCOMP_GET_NOTIF_SIZES: crate::c_uint = 3u32;
```

### `SECCOMP_FILTER_FLAG_TSYNC`

```rust
const SECCOMP_FILTER_FLAG_TSYNC: crate::c_ulong = 1u64;
```

### `SECCOMP_FILTER_FLAG_LOG`

```rust
const SECCOMP_FILTER_FLAG_LOG: crate::c_ulong = 2u64;
```

### `SECCOMP_FILTER_FLAG_SPEC_ALLOW`

```rust
const SECCOMP_FILTER_FLAG_SPEC_ALLOW: crate::c_ulong = 4u64;
```

### `SECCOMP_FILTER_FLAG_NEW_LISTENER`

```rust
const SECCOMP_FILTER_FLAG_NEW_LISTENER: crate::c_ulong = 8u64;
```

### `SECCOMP_FILTER_FLAG_TSYNC_ESRCH`

```rust
const SECCOMP_FILTER_FLAG_TSYNC_ESRCH: crate::c_ulong = 16u64;
```

### `SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV`

```rust
const SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV: crate::c_ulong = 32u64;
```

### `SECCOMP_RET_KILL_PROCESS`

```rust
const SECCOMP_RET_KILL_PROCESS: crate::c_uint = 2_147_483_648u32;
```

### `SECCOMP_RET_KILL_THREAD`

```rust
const SECCOMP_RET_KILL_THREAD: crate::c_uint = 0u32;
```

### `SECCOMP_RET_KILL`

```rust
const SECCOMP_RET_KILL: crate::c_uint = 0u32;
```

### `SECCOMP_RET_TRAP`

```rust
const SECCOMP_RET_TRAP: crate::c_uint = 196_608u32;
```

### `SECCOMP_RET_ERRNO`

```rust
const SECCOMP_RET_ERRNO: crate::c_uint = 327_680u32;
```

### `SECCOMP_RET_USER_NOTIF`

```rust
const SECCOMP_RET_USER_NOTIF: crate::c_uint = 2_143_289_344u32;
```

### `SECCOMP_RET_TRACE`

```rust
const SECCOMP_RET_TRACE: crate::c_uint = 2_146_435_072u32;
```

### `SECCOMP_RET_LOG`

```rust
const SECCOMP_RET_LOG: crate::c_uint = 2_147_221_504u32;
```

### `SECCOMP_RET_ALLOW`

```rust
const SECCOMP_RET_ALLOW: crate::c_uint = 2_147_418_112u32;
```

### `SECCOMP_RET_ACTION_FULL`

```rust
const SECCOMP_RET_ACTION_FULL: crate::c_uint = 4_294_901_760u32;
```

### `SECCOMP_RET_ACTION`

```rust
const SECCOMP_RET_ACTION: crate::c_uint = 2_147_418_112u32;
```

### `SECCOMP_RET_DATA`

```rust
const SECCOMP_RET_DATA: crate::c_uint = 65_535u32;
```

### `SECCOMP_USER_NOTIF_FLAG_CONTINUE`

```rust
const SECCOMP_USER_NOTIF_FLAG_CONTINUE: crate::c_ulong = 1u64;
```

### `SECCOMP_ADDFD_FLAG_SETFD`

```rust
const SECCOMP_ADDFD_FLAG_SETFD: crate::c_ulong = 1u64;
```

### `SECCOMP_ADDFD_FLAG_SEND`

```rust
const SECCOMP_ADDFD_FLAG_SEND: crate::c_ulong = 2u64;
```

### `ITIMER_REAL`

```rust
const ITIMER_REAL: crate::c_int = 0i32;
```

### `ITIMER_VIRTUAL`

```rust
const ITIMER_VIRTUAL: crate::c_int = 1i32;
```

### `ITIMER_PROF`

```rust
const ITIMER_PROF: crate::c_int = 2i32;
```

### `TFD_CLOEXEC`

```rust
const TFD_CLOEXEC: crate::c_int = 524_288i32;
```

### `TFD_NONBLOCK`

```rust
const TFD_NONBLOCK: crate::c_int = 2_048i32;
```

### `TFD_TIMER_ABSTIME`

```rust
const TFD_TIMER_ABSTIME: crate::c_int = 1i32;
```

### `TFD_TIMER_CANCEL_ON_SET`

```rust
const TFD_TIMER_CANCEL_ON_SET: crate::c_int = 2i32;
```

### `_POSIX_VDISABLE`

```rust
const _POSIX_VDISABLE: crate::cc_t = 0u8;
```

### `FALLOC_FL_KEEP_SIZE`

```rust
const FALLOC_FL_KEEP_SIZE: crate::c_int = 1i32;
```

### `FALLOC_FL_PUNCH_HOLE`

```rust
const FALLOC_FL_PUNCH_HOLE: crate::c_int = 2i32;
```

### `FALLOC_FL_COLLAPSE_RANGE`

```rust
const FALLOC_FL_COLLAPSE_RANGE: crate::c_int = 8i32;
```

### `FALLOC_FL_ZERO_RANGE`

```rust
const FALLOC_FL_ZERO_RANGE: crate::c_int = 16i32;
```

### `FALLOC_FL_INSERT_RANGE`

```rust
const FALLOC_FL_INSERT_RANGE: crate::c_int = 32i32;
```

### `FALLOC_FL_UNSHARE_RANGE`

```rust
const FALLOC_FL_UNSHARE_RANGE: crate::c_int = 64i32;
```

### `ENOATTR`

```rust
const ENOATTR: crate::c_int = 61i32;
```

### `SO_ORIGINAL_DST`

```rust
const SO_ORIGINAL_DST: crate::c_int = 80i32;
```

### `IP_RECVFRAGSIZE`

```rust
const IP_RECVFRAGSIZE: crate::c_int = 25i32;
```

### `IPV6_FLOWINFO`

```rust
const IPV6_FLOWINFO: crate::c_int = 11i32;
```

### `IPV6_FLOWLABEL_MGR`

```rust
const IPV6_FLOWLABEL_MGR: crate::c_int = 32i32;
```

### `IPV6_FLOWINFO_SEND`

```rust
const IPV6_FLOWINFO_SEND: crate::c_int = 33i32;
```

### `IPV6_RECVFRAGSIZE`

```rust
const IPV6_RECVFRAGSIZE: crate::c_int = 77i32;
```

### `IPV6_FREEBIND`

```rust
const IPV6_FREEBIND: crate::c_int = 78i32;
```

### `IPV6_FLOWINFO_FLOWLABEL`

```rust
const IPV6_FLOWINFO_FLOWLABEL: crate::c_int = 1_048_575i32;
```

### `IPV6_FLOWINFO_PRIORITY`

```rust
const IPV6_FLOWINFO_PRIORITY: crate::c_int = 267_386_880i32;
```

### `IPV6_RTHDR_LOOSE`

```rust
const IPV6_RTHDR_LOOSE: crate::c_int = 0i32;
```

### `IPV6_RTHDR_STRICT`

```rust
const IPV6_RTHDR_STRICT: crate::c_int = 1i32;
```

### `SK_MEMINFO_RMEM_ALLOC`

```rust
const SK_MEMINFO_RMEM_ALLOC: crate::c_int = 0i32;
```

### `SK_MEMINFO_RCVBUF`

```rust
const SK_MEMINFO_RCVBUF: crate::c_int = 1i32;
```

### `SK_MEMINFO_WMEM_ALLOC`

```rust
const SK_MEMINFO_WMEM_ALLOC: crate::c_int = 2i32;
```

### `SK_MEMINFO_SNDBUF`

```rust
const SK_MEMINFO_SNDBUF: crate::c_int = 3i32;
```

### `SK_MEMINFO_FWD_ALLOC`

```rust
const SK_MEMINFO_FWD_ALLOC: crate::c_int = 4i32;
```

### `SK_MEMINFO_WMEM_QUEUED`

```rust
const SK_MEMINFO_WMEM_QUEUED: crate::c_int = 5i32;
```

### `SK_MEMINFO_OPTMEM`

```rust
const SK_MEMINFO_OPTMEM: crate::c_int = 6i32;
```

### `SK_MEMINFO_BACKLOG`

```rust
const SK_MEMINFO_BACKLOG: crate::c_int = 7i32;
```

### `SK_MEMINFO_DROPS`

```rust
const SK_MEMINFO_DROPS: crate::c_int = 8i32;
```

### `IUTF8`

```rust
const IUTF8: crate::tcflag_t = 16_384u32;
```

### `CMSPAR`

```rust
const CMSPAR: crate::tcflag_t = 1_073_741_824u32;
```

### `MFD_CLOEXEC`

```rust
const MFD_CLOEXEC: crate::c_uint = 1u32;
```

### `MFD_ALLOW_SEALING`

```rust
const MFD_ALLOW_SEALING: crate::c_uint = 2u32;
```

### `MFD_HUGETLB`

```rust
const MFD_HUGETLB: crate::c_uint = 4u32;
```

### `MFD_NOEXEC_SEAL`

```rust
const MFD_NOEXEC_SEAL: crate::c_uint = 8u32;
```

### `MFD_EXEC`

```rust
const MFD_EXEC: crate::c_uint = 16u32;
```

### `MFD_HUGE_64KB`

```rust
const MFD_HUGE_64KB: crate::c_uint = 1_073_741_824u32;
```

### `MFD_HUGE_512KB`

```rust
const MFD_HUGE_512KB: crate::c_uint = 1_275_068_416u32;
```

### `MFD_HUGE_1MB`

```rust
const MFD_HUGE_1MB: crate::c_uint = 1_342_177_280u32;
```

### `MFD_HUGE_2MB`

```rust
const MFD_HUGE_2MB: crate::c_uint = 1_409_286_144u32;
```

### `MFD_HUGE_8MB`

```rust
const MFD_HUGE_8MB: crate::c_uint = 1_543_503_872u32;
```

### `MFD_HUGE_16MB`

```rust
const MFD_HUGE_16MB: crate::c_uint = 1_610_612_736u32;
```

### `MFD_HUGE_32MB`

```rust
const MFD_HUGE_32MB: crate::c_uint = 1_677_721_600u32;
```

### `MFD_HUGE_256MB`

```rust
const MFD_HUGE_256MB: crate::c_uint = 1_879_048_192u32;
```

### `MFD_HUGE_512MB`

```rust
const MFD_HUGE_512MB: crate::c_uint = 1_946_157_056u32;
```

### `MFD_HUGE_1GB`

```rust
const MFD_HUGE_1GB: crate::c_uint = 2_013_265_920u32;
```

### `MFD_HUGE_2GB`

```rust
const MFD_HUGE_2GB: crate::c_uint = 2_080_374_784u32;
```

### `MFD_HUGE_16GB`

```rust
const MFD_HUGE_16GB: crate::c_uint = 2_281_701_376u32;
```

### `MFD_HUGE_MASK`

```rust
const MFD_HUGE_MASK: crate::c_uint = 63u32;
```

### `MFD_HUGE_SHIFT`

```rust
const MFD_HUGE_SHIFT: crate::c_uint = 26u32;
```

### `CLOSE_RANGE_UNSHARE`

```rust
const CLOSE_RANGE_UNSHARE: crate::c_uint = 2u32;
```

### `CLOSE_RANGE_CLOEXEC`

```rust
const CLOSE_RANGE_CLOEXEC: crate::c_uint = 4u32;
```

### `SKF_AD_OFF`

```rust
const SKF_AD_OFF: crate::c_int = -4_096i32;
```

### `SKF_AD_PROTOCOL`

```rust
const SKF_AD_PROTOCOL: crate::c_int = 0i32;
```

### `SKF_AD_PKTTYPE`

```rust
const SKF_AD_PKTTYPE: crate::c_int = 4i32;
```

### `SKF_AD_IFINDEX`

```rust
const SKF_AD_IFINDEX: crate::c_int = 8i32;
```

### `SKF_AD_NLATTR`

```rust
const SKF_AD_NLATTR: crate::c_int = 12i32;
```

### `SKF_AD_NLATTR_NEST`

```rust
const SKF_AD_NLATTR_NEST: crate::c_int = 16i32;
```

### `SKF_AD_MARK`

```rust
const SKF_AD_MARK: crate::c_int = 20i32;
```

### `SKF_AD_QUEUE`

```rust
const SKF_AD_QUEUE: crate::c_int = 24i32;
```

### `SKF_AD_HATYPE`

```rust
const SKF_AD_HATYPE: crate::c_int = 28i32;
```

### `SKF_AD_RXHASH`

```rust
const SKF_AD_RXHASH: crate::c_int = 32i32;
```

### `SKF_AD_CPU`

```rust
const SKF_AD_CPU: crate::c_int = 36i32;
```

### `SKF_AD_ALU_XOR_X`

```rust
const SKF_AD_ALU_XOR_X: crate::c_int = 40i32;
```

### `SKF_AD_VLAN_TAG`

```rust
const SKF_AD_VLAN_TAG: crate::c_int = 44i32;
```

### `SKF_AD_VLAN_TAG_PRESENT`

```rust
const SKF_AD_VLAN_TAG_PRESENT: crate::c_int = 48i32;
```

### `SKF_AD_PAY_OFFSET`

```rust
const SKF_AD_PAY_OFFSET: crate::c_int = 52i32;
```

### `SKF_AD_RANDOM`

```rust
const SKF_AD_RANDOM: crate::c_int = 56i32;
```

### `SKF_AD_VLAN_TPID`

```rust
const SKF_AD_VLAN_TPID: crate::c_int = 60i32;
```

### `SKF_AD_MAX`

```rust
const SKF_AD_MAX: crate::c_int = 64i32;
```

### `SKF_NET_OFF`

```rust
const SKF_NET_OFF: crate::c_int = -1_048_576i32;
```

### `SKF_LL_OFF`

```rust
const SKF_LL_OFF: crate::c_int = -2_097_152i32;
```

### `BPF_NET_OFF`

```rust
const BPF_NET_OFF: crate::c_int = -1_048_576i32;
```

### `BPF_LL_OFF`

```rust
const BPF_LL_OFF: crate::c_int = -2_097_152i32;
```

### `BPF_MEMWORDS`

```rust
const BPF_MEMWORDS: crate::c_int = 16i32;
```

### `BPF_MAXINSNS`

```rust
const BPF_MAXINSNS: crate::c_int = 4_096i32;
```

### `BPF_LD`

```rust
const BPF_LD: __u32 = 0u32;
```

### `BPF_LDX`

```rust
const BPF_LDX: __u32 = 1u32;
```

### `BPF_ST`

```rust
const BPF_ST: __u32 = 2u32;
```

### `BPF_STX`

```rust
const BPF_STX: __u32 = 3u32;
```

### `BPF_ALU`

```rust
const BPF_ALU: __u32 = 4u32;
```

### `BPF_JMP`

```rust
const BPF_JMP: __u32 = 5u32;
```

### `BPF_RET`

```rust
const BPF_RET: __u32 = 6u32;
```

### `BPF_MISC`

```rust
const BPF_MISC: __u32 = 7u32;
```

### `BPF_W`

```rust
const BPF_W: __u32 = 0u32;
```

### `BPF_H`

```rust
const BPF_H: __u32 = 8u32;
```

### `BPF_B`

```rust
const BPF_B: __u32 = 16u32;
```

### `BPF_IMM`

```rust
const BPF_IMM: __u32 = 0u32;
```

### `BPF_ABS`

```rust
const BPF_ABS: __u32 = 32u32;
```

### `BPF_IND`

```rust
const BPF_IND: __u32 = 64u32;
```

### `BPF_MEM`

```rust
const BPF_MEM: __u32 = 96u32;
```

### `BPF_LEN`

```rust
const BPF_LEN: __u32 = 128u32;
```

### `BPF_MSH`

```rust
const BPF_MSH: __u32 = 160u32;
```

### `BPF_ADD`

```rust
const BPF_ADD: __u32 = 0u32;
```

### `BPF_SUB`

```rust
const BPF_SUB: __u32 = 16u32;
```

### `BPF_MUL`

```rust
const BPF_MUL: __u32 = 32u32;
```

### `BPF_DIV`

```rust
const BPF_DIV: __u32 = 48u32;
```

### `BPF_OR`

```rust
const BPF_OR: __u32 = 64u32;
```

### `BPF_AND`

```rust
const BPF_AND: __u32 = 80u32;
```

### `BPF_LSH`

```rust
const BPF_LSH: __u32 = 96u32;
```

### `BPF_RSH`

```rust
const BPF_RSH: __u32 = 112u32;
```

### `BPF_NEG`

```rust
const BPF_NEG: __u32 = 128u32;
```

### `BPF_MOD`

```rust
const BPF_MOD: __u32 = 144u32;
```

### `BPF_XOR`

```rust
const BPF_XOR: __u32 = 160u32;
```

### `BPF_JA`

```rust
const BPF_JA: __u32 = 0u32;
```

### `BPF_JEQ`

```rust
const BPF_JEQ: __u32 = 16u32;
```

### `BPF_JGT`

```rust
const BPF_JGT: __u32 = 32u32;
```

### `BPF_JGE`

```rust
const BPF_JGE: __u32 = 48u32;
```

### `BPF_JSET`

```rust
const BPF_JSET: __u32 = 64u32;
```

### `BPF_K`

```rust
const BPF_K: __u32 = 0u32;
```

### `BPF_X`

```rust
const BPF_X: __u32 = 8u32;
```

### `BPF_A`

```rust
const BPF_A: __u32 = 16u32;
```

### `BPF_TAX`

```rust
const BPF_TAX: __u32 = 0u32;
```

### `BPF_TXA`

```rust
const BPF_TXA: __u32 = 128u32;
```

### `RESOLVE_NO_XDEV`

```rust
const RESOLVE_NO_XDEV: crate::__u64 = 1u64;
```

### `RESOLVE_NO_MAGICLINKS`

```rust
const RESOLVE_NO_MAGICLINKS: crate::__u64 = 2u64;
```

### `RESOLVE_NO_SYMLINKS`

```rust
const RESOLVE_NO_SYMLINKS: crate::__u64 = 4u64;
```

### `RESOLVE_BENEATH`

```rust
const RESOLVE_BENEATH: crate::__u64 = 8u64;
```

### `RESOLVE_IN_ROOT`

```rust
const RESOLVE_IN_ROOT: crate::__u64 = 16u64;
```

### `RESOLVE_CACHED`

```rust
const RESOLVE_CACHED: crate::__u64 = 32u64;
```

### `ETH_ALEN`

```rust
const ETH_ALEN: crate::c_int = 6i32;
```

### `ETH_HLEN`

```rust
const ETH_HLEN: crate::c_int = 14i32;
```

### `ETH_ZLEN`

```rust
const ETH_ZLEN: crate::c_int = 60i32;
```

### `ETH_DATA_LEN`

```rust
const ETH_DATA_LEN: crate::c_int = 1_500i32;
```

### `ETH_FRAME_LEN`

```rust
const ETH_FRAME_LEN: crate::c_int = 1_514i32;
```

### `ETH_FCS_LEN`

```rust
const ETH_FCS_LEN: crate::c_int = 4i32;
```

### `ETH_P_LOOP`

```rust
const ETH_P_LOOP: crate::c_int = 96i32;
```

### `ETH_P_PUP`

```rust
const ETH_P_PUP: crate::c_int = 512i32;
```

### `ETH_P_PUPAT`

```rust
const ETH_P_PUPAT: crate::c_int = 513i32;
```

### `ETH_P_IP`

```rust
const ETH_P_IP: crate::c_int = 2_048i32;
```

### `ETH_P_X25`

```rust
const ETH_P_X25: crate::c_int = 2_053i32;
```

### `ETH_P_ARP`

```rust
const ETH_P_ARP: crate::c_int = 2_054i32;
```

### `ETH_P_BPQ`

```rust
const ETH_P_BPQ: crate::c_int = 2_303i32;
```

### `ETH_P_IEEEPUP`

```rust
const ETH_P_IEEEPUP: crate::c_int = 2_560i32;
```

### `ETH_P_IEEEPUPAT`

```rust
const ETH_P_IEEEPUPAT: crate::c_int = 2_561i32;
```

### `ETH_P_BATMAN`

```rust
const ETH_P_BATMAN: crate::c_int = 17_157i32;
```

### `ETH_P_DEC`

```rust
const ETH_P_DEC: crate::c_int = 24_576i32;
```

### `ETH_P_DNA_DL`

```rust
const ETH_P_DNA_DL: crate::c_int = 24_577i32;
```

### `ETH_P_DNA_RC`

```rust
const ETH_P_DNA_RC: crate::c_int = 24_578i32;
```

### `ETH_P_DNA_RT`

```rust
const ETH_P_DNA_RT: crate::c_int = 24_579i32;
```

### `ETH_P_LAT`

```rust
const ETH_P_LAT: crate::c_int = 24_580i32;
```

### `ETH_P_DIAG`

```rust
const ETH_P_DIAG: crate::c_int = 24_581i32;
```

### `ETH_P_CUST`

```rust
const ETH_P_CUST: crate::c_int = 24_582i32;
```

### `ETH_P_SCA`

```rust
const ETH_P_SCA: crate::c_int = 24_583i32;
```

### `ETH_P_TEB`

```rust
const ETH_P_TEB: crate::c_int = 25_944i32;
```

### `ETH_P_RARP`

```rust
const ETH_P_RARP: crate::c_int = 32_821i32;
```

### `ETH_P_ATALK`

```rust
const ETH_P_ATALK: crate::c_int = 32_923i32;
```

### `ETH_P_AARP`

```rust
const ETH_P_AARP: crate::c_int = 33_011i32;
```

### `ETH_P_8021Q`

```rust
const ETH_P_8021Q: crate::c_int = 33_024i32;
```

### `ETH_P_IPX`

```rust
const ETH_P_IPX: crate::c_int = 33_079i32;
```

### `ETH_P_IPV6`

```rust
const ETH_P_IPV6: crate::c_int = 34_525i32;
```

### `ETH_P_PAUSE`

```rust
const ETH_P_PAUSE: crate::c_int = 34_824i32;
```

### `ETH_P_SLOW`

```rust
const ETH_P_SLOW: crate::c_int = 34_825i32;
```

### `ETH_P_WCCP`

```rust
const ETH_P_WCCP: crate::c_int = 34_878i32;
```

### `ETH_P_MPLS_UC`

```rust
const ETH_P_MPLS_UC: crate::c_int = 34_887i32;
```

### `ETH_P_MPLS_MC`

```rust
const ETH_P_MPLS_MC: crate::c_int = 34_888i32;
```

### `ETH_P_ATMMPOA`

```rust
const ETH_P_ATMMPOA: crate::c_int = 34_892i32;
```

### `ETH_P_PPP_DISC`

```rust
const ETH_P_PPP_DISC: crate::c_int = 34_915i32;
```

### `ETH_P_PPP_SES`

```rust
const ETH_P_PPP_SES: crate::c_int = 34_916i32;
```

### `ETH_P_LINK_CTL`

```rust
const ETH_P_LINK_CTL: crate::c_int = 34_924i32;
```

### `ETH_P_ATMFATE`

```rust
const ETH_P_ATMFATE: crate::c_int = 34_948i32;
```

### `ETH_P_PAE`

```rust
const ETH_P_PAE: crate::c_int = 34_958i32;
```

### `ETH_P_AOE`

```rust
const ETH_P_AOE: crate::c_int = 34_978i32;
```

### `ETH_P_8021AD`

```rust
const ETH_P_8021AD: crate::c_int = 34_984i32;
```

### `ETH_P_802_EX1`

```rust
const ETH_P_802_EX1: crate::c_int = 34_997i32;
```

### `ETH_P_TIPC`

```rust
const ETH_P_TIPC: crate::c_int = 35_018i32;
```

### `ETH_P_MACSEC`

```rust
const ETH_P_MACSEC: crate::c_int = 35_045i32;
```

### `ETH_P_8021AH`

```rust
const ETH_P_8021AH: crate::c_int = 35_047i32;
```

### `ETH_P_MVRP`

```rust
const ETH_P_MVRP: crate::c_int = 35_061i32;
```

### `ETH_P_1588`

```rust
const ETH_P_1588: crate::c_int = 35_063i32;
```

### `ETH_P_PRP`

```rust
const ETH_P_PRP: crate::c_int = 35_067i32;
```

### `ETH_P_FCOE`

```rust
const ETH_P_FCOE: crate::c_int = 35_078i32;
```

### `ETH_P_TDLS`

```rust
const ETH_P_TDLS: crate::c_int = 35_085i32;
```

### `ETH_P_FIP`

```rust
const ETH_P_FIP: crate::c_int = 35_092i32;
```

### `ETH_P_80221`

```rust
const ETH_P_80221: crate::c_int = 35_095i32;
```

### `ETH_P_LOOPBACK`

```rust
const ETH_P_LOOPBACK: crate::c_int = 36_864i32;
```

### `ETH_P_QINQ1`

```rust
const ETH_P_QINQ1: crate::c_int = 37_120i32;
```

### `ETH_P_QINQ2`

```rust
const ETH_P_QINQ2: crate::c_int = 37_376i32;
```

### `ETH_P_QINQ3`

```rust
const ETH_P_QINQ3: crate::c_int = 37_632i32;
```

### `ETH_P_EDSA`

```rust
const ETH_P_EDSA: crate::c_int = 56_026i32;
```

### `ETH_P_AF_IUCV`

```rust
const ETH_P_AF_IUCV: crate::c_int = 64_507i32;
```

### `ETH_P_802_3_MIN`

```rust
const ETH_P_802_3_MIN: crate::c_int = 1_536i32;
```

### `ETH_P_802_3`

```rust
const ETH_P_802_3: crate::c_int = 1i32;
```

### `ETH_P_AX25`

```rust
const ETH_P_AX25: crate::c_int = 2i32;
```

### `ETH_P_ALL`

```rust
const ETH_P_ALL: crate::c_int = 3i32;
```

### `ETH_P_802_2`

```rust
const ETH_P_802_2: crate::c_int = 4i32;
```

### `ETH_P_SNAP`

```rust
const ETH_P_SNAP: crate::c_int = 5i32;
```

### `ETH_P_DDCMP`

```rust
const ETH_P_DDCMP: crate::c_int = 6i32;
```

### `ETH_P_WAN_PPP`

```rust
const ETH_P_WAN_PPP: crate::c_int = 7i32;
```

### `ETH_P_PPP_MP`

```rust
const ETH_P_PPP_MP: crate::c_int = 8i32;
```

### `ETH_P_LOCALTALK`

```rust
const ETH_P_LOCALTALK: crate::c_int = 9i32;
```

### `ETH_P_CANFD`

```rust
const ETH_P_CANFD: crate::c_int = 13i32;
```

### `ETH_P_PPPTALK`

```rust
const ETH_P_PPPTALK: crate::c_int = 16i32;
```

### `ETH_P_TR_802_2`

```rust
const ETH_P_TR_802_2: crate::c_int = 17i32;
```

### `ETH_P_MOBITEX`

```rust
const ETH_P_MOBITEX: crate::c_int = 21i32;
```

### `ETH_P_CONTROL`

```rust
const ETH_P_CONTROL: crate::c_int = 22i32;
```

### `ETH_P_IRDA`

```rust
const ETH_P_IRDA: crate::c_int = 23i32;
```

### `ETH_P_ECONET`

```rust
const ETH_P_ECONET: crate::c_int = 24i32;
```

### `ETH_P_HDLC`

```rust
const ETH_P_HDLC: crate::c_int = 25i32;
```

### `ETH_P_ARCNET`

```rust
const ETH_P_ARCNET: crate::c_int = 26i32;
```

### `ETH_P_DSA`

```rust
const ETH_P_DSA: crate::c_int = 27i32;
```

### `ETH_P_TRAILER`

```rust
const ETH_P_TRAILER: crate::c_int = 28i32;
```

### `ETH_P_PHONET`

```rust
const ETH_P_PHONET: crate::c_int = 245i32;
```

### `ETH_P_IEEE802154`

```rust
const ETH_P_IEEE802154: crate::c_int = 246i32;
```

### `ETH_P_CAIF`

```rust
const ETH_P_CAIF: crate::c_int = 247i32;
```

### `POSIX_SPAWN_RESETIDS`

```rust
const POSIX_SPAWN_RESETIDS: crate::c_int = 1i32;
```

### `POSIX_SPAWN_SETPGROUP`

```rust
const POSIX_SPAWN_SETPGROUP: crate::c_int = 2i32;
```

### `POSIX_SPAWN_SETSIGDEF`

```rust
const POSIX_SPAWN_SETSIGDEF: crate::c_int = 4i32;
```

### `POSIX_SPAWN_SETSIGMASK`

```rust
const POSIX_SPAWN_SETSIGMASK: crate::c_int = 8i32;
```

### `POSIX_SPAWN_SETSCHEDPARAM`

```rust
const POSIX_SPAWN_SETSCHEDPARAM: crate::c_int = 16i32;
```

### `POSIX_SPAWN_SETSCHEDULER`

```rust
const POSIX_SPAWN_SETSCHEDULER: crate::c_int = 32i32;
```

### `NLMSG_NOOP`

```rust
const NLMSG_NOOP: crate::c_int = 1i32;
```

### `NLMSG_ERROR`

```rust
const NLMSG_ERROR: crate::c_int = 2i32;
```

### `NLMSG_DONE`

```rust
const NLMSG_DONE: crate::c_int = 3i32;
```

### `NLMSG_OVERRUN`

```rust
const NLMSG_OVERRUN: crate::c_int = 4i32;
```

### `NLMSG_MIN_TYPE`

```rust
const NLMSG_MIN_TYPE: crate::c_int = 16i32;
```

### `NFNLGRP_NONE`

```rust
const NFNLGRP_NONE: crate::c_int = 0i32;
```

### `NFNLGRP_CONNTRACK_NEW`

```rust
const NFNLGRP_CONNTRACK_NEW: crate::c_int = 1i32;
```

### `NFNLGRP_CONNTRACK_UPDATE`

```rust
const NFNLGRP_CONNTRACK_UPDATE: crate::c_int = 2i32;
```

### `NFNLGRP_CONNTRACK_DESTROY`

```rust
const NFNLGRP_CONNTRACK_DESTROY: crate::c_int = 3i32;
```

### `NFNLGRP_CONNTRACK_EXP_NEW`

```rust
const NFNLGRP_CONNTRACK_EXP_NEW: crate::c_int = 4i32;
```

### `NFNLGRP_CONNTRACK_EXP_UPDATE`

```rust
const NFNLGRP_CONNTRACK_EXP_UPDATE: crate::c_int = 5i32;
```

### `NFNLGRP_CONNTRACK_EXP_DESTROY`

```rust
const NFNLGRP_CONNTRACK_EXP_DESTROY: crate::c_int = 6i32;
```

### `NFNLGRP_NFTABLES`

```rust
const NFNLGRP_NFTABLES: crate::c_int = 7i32;
```

### `NFNLGRP_ACCT_QUOTA`

```rust
const NFNLGRP_ACCT_QUOTA: crate::c_int = 8i32;
```

### `NFNLGRP_NFTRACE`

```rust
const NFNLGRP_NFTRACE: crate::c_int = 9i32;
```

### `NFNETLINK_V0`

```rust
const NFNETLINK_V0: crate::c_int = 0i32;
```

### `NFNL_SUBSYS_NONE`

```rust
const NFNL_SUBSYS_NONE: crate::c_int = 0i32;
```

### `NFNL_SUBSYS_CTNETLINK`

```rust
const NFNL_SUBSYS_CTNETLINK: crate::c_int = 1i32;
```

### `NFNL_SUBSYS_CTNETLINK_EXP`

```rust
const NFNL_SUBSYS_CTNETLINK_EXP: crate::c_int = 2i32;
```

### `NFNL_SUBSYS_QUEUE`

```rust
const NFNL_SUBSYS_QUEUE: crate::c_int = 3i32;
```

### `NFNL_SUBSYS_ULOG`

```rust
const NFNL_SUBSYS_ULOG: crate::c_int = 4i32;
```

### `NFNL_SUBSYS_OSF`

```rust
const NFNL_SUBSYS_OSF: crate::c_int = 5i32;
```

### `NFNL_SUBSYS_IPSET`

```rust
const NFNL_SUBSYS_IPSET: crate::c_int = 6i32;
```

### `NFNL_SUBSYS_ACCT`

```rust
const NFNL_SUBSYS_ACCT: crate::c_int = 7i32;
```

### `NFNL_SUBSYS_CTNETLINK_TIMEOUT`

```rust
const NFNL_SUBSYS_CTNETLINK_TIMEOUT: crate::c_int = 8i32;
```

### `NFNL_SUBSYS_CTHELPER`

```rust
const NFNL_SUBSYS_CTHELPER: crate::c_int = 9i32;
```

### `NFNL_SUBSYS_NFTABLES`

```rust
const NFNL_SUBSYS_NFTABLES: crate::c_int = 10i32;
```

### `NFNL_SUBSYS_NFT_COMPAT`

```rust
const NFNL_SUBSYS_NFT_COMPAT: crate::c_int = 11i32;
```

### `NFNL_SUBSYS_HOOK`

```rust
const NFNL_SUBSYS_HOOK: crate::c_int = 12i32;
```

### `NFNL_SUBSYS_COUNT`

```rust
const NFNL_SUBSYS_COUNT: crate::c_int = 13i32;
```

### `NFNL_MSG_BATCH_BEGIN`

```rust
const NFNL_MSG_BATCH_BEGIN: crate::c_int = 16i32;
```

### `NFNL_MSG_BATCH_END`

```rust
const NFNL_MSG_BATCH_END: crate::c_int = 17i32;
```

### `NFNL_BATCH_UNSPEC`

```rust
const NFNL_BATCH_UNSPEC: crate::c_int = 0i32;
```

### `NFNL_BATCH_GENID`

```rust
const NFNL_BATCH_GENID: crate::c_int = 1i32;
```

### `NFULNL_MSG_PACKET`

```rust
const NFULNL_MSG_PACKET: crate::c_int = 0i32;
```

### `NFULNL_MSG_CONFIG`

```rust
const NFULNL_MSG_CONFIG: crate::c_int = 1i32;
```

### `NFULA_VLAN_UNSPEC`

```rust
const NFULA_VLAN_UNSPEC: crate::c_int = 0i32;
```

### `NFULA_VLAN_PROTO`

```rust
const NFULA_VLAN_PROTO: crate::c_int = 1i32;
```

### `NFULA_VLAN_TCI`

```rust
const NFULA_VLAN_TCI: crate::c_int = 2i32;
```

### `NFULA_UNSPEC`

```rust
const NFULA_UNSPEC: crate::c_int = 0i32;
```

### `NFULA_PACKET_HDR`

```rust
const NFULA_PACKET_HDR: crate::c_int = 1i32;
```

### `NFULA_MARK`

```rust
const NFULA_MARK: crate::c_int = 2i32;
```

### `NFULA_TIMESTAMP`

```rust
const NFULA_TIMESTAMP: crate::c_int = 3i32;
```

### `NFULA_IFINDEX_INDEV`

```rust
const NFULA_IFINDEX_INDEV: crate::c_int = 4i32;
```

### `NFULA_IFINDEX_OUTDEV`

```rust
const NFULA_IFINDEX_OUTDEV: crate::c_int = 5i32;
```

### `NFULA_IFINDEX_PHYSINDEV`

```rust
const NFULA_IFINDEX_PHYSINDEV: crate::c_int = 6i32;
```

### `NFULA_IFINDEX_PHYSOUTDEV`

```rust
const NFULA_IFINDEX_PHYSOUTDEV: crate::c_int = 7i32;
```

### `NFULA_HWADDR`

```rust
const NFULA_HWADDR: crate::c_int = 8i32;
```

### `NFULA_PAYLOAD`

```rust
const NFULA_PAYLOAD: crate::c_int = 9i32;
```

### `NFULA_PREFIX`

```rust
const NFULA_PREFIX: crate::c_int = 10i32;
```

### `NFULA_UID`

```rust
const NFULA_UID: crate::c_int = 11i32;
```

### `NFULA_SEQ`

```rust
const NFULA_SEQ: crate::c_int = 12i32;
```

### `NFULA_SEQ_GLOBAL`

```rust
const NFULA_SEQ_GLOBAL: crate::c_int = 13i32;
```

### `NFULA_GID`

```rust
const NFULA_GID: crate::c_int = 14i32;
```

### `NFULA_HWTYPE`

```rust
const NFULA_HWTYPE: crate::c_int = 15i32;
```

### `NFULA_HWHEADER`

```rust
const NFULA_HWHEADER: crate::c_int = 16i32;
```

### `NFULA_HWLEN`

```rust
const NFULA_HWLEN: crate::c_int = 17i32;
```

### `NFULA_CT`

```rust
const NFULA_CT: crate::c_int = 18i32;
```

### `NFULA_CT_INFO`

```rust
const NFULA_CT_INFO: crate::c_int = 19i32;
```

### `NFULA_VLAN`

```rust
const NFULA_VLAN: crate::c_int = 20i32;
```

### `NFULA_L2HDR`

```rust
const NFULA_L2HDR: crate::c_int = 21i32;
```

### `NFULNL_CFG_CMD_NONE`

```rust
const NFULNL_CFG_CMD_NONE: crate::c_int = 0i32;
```

### `NFULNL_CFG_CMD_BIND`

```rust
const NFULNL_CFG_CMD_BIND: crate::c_int = 1i32;
```

### `NFULNL_CFG_CMD_UNBIND`

```rust
const NFULNL_CFG_CMD_UNBIND: crate::c_int = 2i32;
```

### `NFULNL_CFG_CMD_PF_BIND`

```rust
const NFULNL_CFG_CMD_PF_BIND: crate::c_int = 3i32;
```

### `NFULNL_CFG_CMD_PF_UNBIND`

```rust
const NFULNL_CFG_CMD_PF_UNBIND: crate::c_int = 4i32;
```

### `NFULA_CFG_UNSPEC`

```rust
const NFULA_CFG_UNSPEC: crate::c_int = 0i32;
```

### `NFULA_CFG_CMD`

```rust
const NFULA_CFG_CMD: crate::c_int = 1i32;
```

### `NFULA_CFG_MODE`

```rust
const NFULA_CFG_MODE: crate::c_int = 2i32;
```

### `NFULA_CFG_NLBUFSIZ`

```rust
const NFULA_CFG_NLBUFSIZ: crate::c_int = 3i32;
```

### `NFULA_CFG_TIMEOUT`

```rust
const NFULA_CFG_TIMEOUT: crate::c_int = 4i32;
```

### `NFULA_CFG_QTHRESH`

```rust
const NFULA_CFG_QTHRESH: crate::c_int = 5i32;
```

### `NFULA_CFG_FLAGS`

```rust
const NFULA_CFG_FLAGS: crate::c_int = 6i32;
```

### `NFULNL_COPY_NONE`

```rust
const NFULNL_COPY_NONE: crate::c_int = 0i32;
```

### `NFULNL_COPY_META`

```rust
const NFULNL_COPY_META: crate::c_int = 1i32;
```

### `NFULNL_COPY_PACKET`

```rust
const NFULNL_COPY_PACKET: crate::c_int = 2i32;
```

### `NFULNL_CFG_F_SEQ`

```rust
const NFULNL_CFG_F_SEQ: crate::c_int = 1i32;
```

### `NFULNL_CFG_F_SEQ_GLOBAL`

```rust
const NFULNL_CFG_F_SEQ_GLOBAL: crate::c_int = 2i32;
```

### `NFULNL_CFG_F_CONNTRACK`

```rust
const NFULNL_CFG_F_CONNTRACK: crate::c_int = 4i32;
```

### `NFQNL_MSG_PACKET`

```rust
const NFQNL_MSG_PACKET: crate::c_int = 0i32;
```

### `NFQNL_MSG_VERDICT`

```rust
const NFQNL_MSG_VERDICT: crate::c_int = 1i32;
```

### `NFQNL_MSG_CONFIG`

```rust
const NFQNL_MSG_CONFIG: crate::c_int = 2i32;
```

### `NFQNL_MSG_VERDICT_BATCH`

```rust
const NFQNL_MSG_VERDICT_BATCH: crate::c_int = 3i32;
```

### `NFQA_UNSPEC`

```rust
const NFQA_UNSPEC: crate::c_int = 0i32;
```

### `NFQA_PACKET_HDR`

```rust
const NFQA_PACKET_HDR: crate::c_int = 1i32;
```

### `NFQA_VERDICT_HDR`

```rust
const NFQA_VERDICT_HDR: crate::c_int = 2i32;
```

### `NFQA_MARK`

```rust
const NFQA_MARK: crate::c_int = 3i32;
```

### `NFQA_TIMESTAMP`

```rust
const NFQA_TIMESTAMP: crate::c_int = 4i32;
```

### `NFQA_IFINDEX_INDEV`

```rust
const NFQA_IFINDEX_INDEV: crate::c_int = 5i32;
```

### `NFQA_IFINDEX_OUTDEV`

```rust
const NFQA_IFINDEX_OUTDEV: crate::c_int = 6i32;
```

### `NFQA_IFINDEX_PHYSINDEV`

```rust
const NFQA_IFINDEX_PHYSINDEV: crate::c_int = 7i32;
```

### `NFQA_IFINDEX_PHYSOUTDEV`

```rust
const NFQA_IFINDEX_PHYSOUTDEV: crate::c_int = 8i32;
```

### `NFQA_HWADDR`

```rust
const NFQA_HWADDR: crate::c_int = 9i32;
```

### `NFQA_PAYLOAD`

```rust
const NFQA_PAYLOAD: crate::c_int = 10i32;
```

### `NFQA_CT`

```rust
const NFQA_CT: crate::c_int = 11i32;
```

### `NFQA_CT_INFO`

```rust
const NFQA_CT_INFO: crate::c_int = 12i32;
```

### `NFQA_CAP_LEN`

```rust
const NFQA_CAP_LEN: crate::c_int = 13i32;
```

### `NFQA_SKB_INFO`

```rust
const NFQA_SKB_INFO: crate::c_int = 14i32;
```

### `NFQA_EXP`

```rust
const NFQA_EXP: crate::c_int = 15i32;
```

### `NFQA_UID`

```rust
const NFQA_UID: crate::c_int = 16i32;
```

### `NFQA_GID`

```rust
const NFQA_GID: crate::c_int = 17i32;
```

### `NFQA_SECCTX`

```rust
const NFQA_SECCTX: crate::c_int = 18i32;
```

### `NFQA_VLAN`

```rust
const NFQA_VLAN: crate::c_int = 19i32;
```

### `NFQA_L2HDR`

```rust
const NFQA_L2HDR: crate::c_int = 20i32;
```

### `NFQA_PRIORITY`

```rust
const NFQA_PRIORITY: crate::c_int = 21i32;
```

### `NFQA_VLAN_UNSPEC`

```rust
const NFQA_VLAN_UNSPEC: crate::c_int = 0i32;
```

### `NFQA_VLAN_PROTO`

```rust
const NFQA_VLAN_PROTO: crate::c_int = 1i32;
```

### `NFQA_VLAN_TCI`

```rust
const NFQA_VLAN_TCI: crate::c_int = 2i32;
```

### `NFQNL_CFG_CMD_NONE`

```rust
const NFQNL_CFG_CMD_NONE: crate::c_int = 0i32;
```

### `NFQNL_CFG_CMD_BIND`

```rust
const NFQNL_CFG_CMD_BIND: crate::c_int = 1i32;
```

### `NFQNL_CFG_CMD_UNBIND`

```rust
const NFQNL_CFG_CMD_UNBIND: crate::c_int = 2i32;
```

### `NFQNL_CFG_CMD_PF_BIND`

```rust
const NFQNL_CFG_CMD_PF_BIND: crate::c_int = 3i32;
```

### `NFQNL_CFG_CMD_PF_UNBIND`

```rust
const NFQNL_CFG_CMD_PF_UNBIND: crate::c_int = 4i32;
```

### `NFQNL_COPY_NONE`

```rust
const NFQNL_COPY_NONE: crate::c_int = 0i32;
```

### `NFQNL_COPY_META`

```rust
const NFQNL_COPY_META: crate::c_int = 1i32;
```

### `NFQNL_COPY_PACKET`

```rust
const NFQNL_COPY_PACKET: crate::c_int = 2i32;
```

### `NFQA_CFG_UNSPEC`

```rust
const NFQA_CFG_UNSPEC: crate::c_int = 0i32;
```

### `NFQA_CFG_CMD`

```rust
const NFQA_CFG_CMD: crate::c_int = 1i32;
```

### `NFQA_CFG_PARAMS`

```rust
const NFQA_CFG_PARAMS: crate::c_int = 2i32;
```

### `NFQA_CFG_QUEUE_MAXLEN`

```rust
const NFQA_CFG_QUEUE_MAXLEN: crate::c_int = 3i32;
```

### `NFQA_CFG_MASK`

```rust
const NFQA_CFG_MASK: crate::c_int = 4i32;
```

### `NFQA_CFG_FLAGS`

```rust
const NFQA_CFG_FLAGS: crate::c_int = 5i32;
```

### `NFQA_CFG_F_FAIL_OPEN`

```rust
const NFQA_CFG_F_FAIL_OPEN: crate::c_int = 1i32;
```

### `NFQA_CFG_F_CONNTRACK`

```rust
const NFQA_CFG_F_CONNTRACK: crate::c_int = 2i32;
```

### `NFQA_CFG_F_GSO`

```rust
const NFQA_CFG_F_GSO: crate::c_int = 4i32;
```

### `NFQA_CFG_F_UID_GID`

```rust
const NFQA_CFG_F_UID_GID: crate::c_int = 8i32;
```

### `NFQA_CFG_F_SECCTX`

```rust
const NFQA_CFG_F_SECCTX: crate::c_int = 16i32;
```

### `NFQA_CFG_F_MAX`

```rust
const NFQA_CFG_F_MAX: crate::c_int = 32i32;
```

### `NFQA_SKB_CSUMNOTREADY`

```rust
const NFQA_SKB_CSUMNOTREADY: crate::c_int = 1i32;
```

### `NFQA_SKB_GSO`

```rust
const NFQA_SKB_GSO: crate::c_int = 2i32;
```

### `NFQA_SKB_CSUM_NOTVERIFIED`

```rust
const NFQA_SKB_CSUM_NOTVERIFIED: crate::c_int = 4i32;
```

### `GENL_NAMSIZ`

```rust
const GENL_NAMSIZ: crate::c_int = 16i32;
```

### `GENL_MIN_ID`

```rust
const GENL_MIN_ID: crate::c_int = 16i32;
```

### `GENL_MAX_ID`

```rust
const GENL_MAX_ID: crate::c_int = 1_023i32;
```

### `GENL_ADMIN_PERM`

```rust
const GENL_ADMIN_PERM: crate::c_int = 1i32;
```

### `GENL_CMD_CAP_DO`

```rust
const GENL_CMD_CAP_DO: crate::c_int = 2i32;
```

### `GENL_CMD_CAP_DUMP`

```rust
const GENL_CMD_CAP_DUMP: crate::c_int = 4i32;
```

### `GENL_CMD_CAP_HASPOL`

```rust
const GENL_CMD_CAP_HASPOL: crate::c_int = 8i32;
```

### `GENL_ID_CTRL`

```rust
const GENL_ID_CTRL: crate::c_int = 16i32;
```

### `CTRL_CMD_UNSPEC`

```rust
const CTRL_CMD_UNSPEC: crate::c_int = 0i32;
```

### `CTRL_CMD_NEWFAMILY`

```rust
const CTRL_CMD_NEWFAMILY: crate::c_int = 1i32;
```

### `CTRL_CMD_DELFAMILY`

```rust
const CTRL_CMD_DELFAMILY: crate::c_int = 2i32;
```

### `CTRL_CMD_GETFAMILY`

```rust
const CTRL_CMD_GETFAMILY: crate::c_int = 3i32;
```

### `CTRL_CMD_NEWOPS`

```rust
const CTRL_CMD_NEWOPS: crate::c_int = 4i32;
```

### `CTRL_CMD_DELOPS`

```rust
const CTRL_CMD_DELOPS: crate::c_int = 5i32;
```

### `CTRL_CMD_GETOPS`

```rust
const CTRL_CMD_GETOPS: crate::c_int = 6i32;
```

### `CTRL_CMD_NEWMCAST_GRP`

```rust
const CTRL_CMD_NEWMCAST_GRP: crate::c_int = 7i32;
```

### `CTRL_CMD_DELMCAST_GRP`

```rust
const CTRL_CMD_DELMCAST_GRP: crate::c_int = 8i32;
```

### `CTRL_CMD_GETMCAST_GRP`

```rust
const CTRL_CMD_GETMCAST_GRP: crate::c_int = 9i32;
```

### `CTRL_ATTR_UNSPEC`

```rust
const CTRL_ATTR_UNSPEC: crate::c_int = 0i32;
```

### `CTRL_ATTR_FAMILY_ID`

```rust
const CTRL_ATTR_FAMILY_ID: crate::c_int = 1i32;
```

### `CTRL_ATTR_FAMILY_NAME`

```rust
const CTRL_ATTR_FAMILY_NAME: crate::c_int = 2i32;
```

### `CTRL_ATTR_VERSION`

```rust
const CTRL_ATTR_VERSION: crate::c_int = 3i32;
```

### `CTRL_ATTR_HDRSIZE`

```rust
const CTRL_ATTR_HDRSIZE: crate::c_int = 4i32;
```

### `CTRL_ATTR_MAXATTR`

```rust
const CTRL_ATTR_MAXATTR: crate::c_int = 5i32;
```

### `CTRL_ATTR_OPS`

```rust
const CTRL_ATTR_OPS: crate::c_int = 6i32;
```

### `CTRL_ATTR_MCAST_GROUPS`

```rust
const CTRL_ATTR_MCAST_GROUPS: crate::c_int = 7i32;
```

### `CTRL_ATTR_OP_UNSPEC`

```rust
const CTRL_ATTR_OP_UNSPEC: crate::c_int = 0i32;
```

### `CTRL_ATTR_OP_ID`

```rust
const CTRL_ATTR_OP_ID: crate::c_int = 1i32;
```

### `CTRL_ATTR_OP_FLAGS`

```rust
const CTRL_ATTR_OP_FLAGS: crate::c_int = 2i32;
```

### `CTRL_ATTR_MCAST_GRP_UNSPEC`

```rust
const CTRL_ATTR_MCAST_GRP_UNSPEC: crate::c_int = 0i32;
```

### `CTRL_ATTR_MCAST_GRP_NAME`

```rust
const CTRL_ATTR_MCAST_GRP_NAME: crate::c_int = 1i32;
```

### `CTRL_ATTR_MCAST_GRP_ID`

```rust
const CTRL_ATTR_MCAST_GRP_ID: crate::c_int = 2i32;
```

### `PACKET_HOST`

```rust
const PACKET_HOST: crate::c_uchar = 0u8;
```

### `PACKET_BROADCAST`

```rust
const PACKET_BROADCAST: crate::c_uchar = 1u8;
```

### `PACKET_MULTICAST`

```rust
const PACKET_MULTICAST: crate::c_uchar = 2u8;
```

### `PACKET_OTHERHOST`

```rust
const PACKET_OTHERHOST: crate::c_uchar = 3u8;
```

### `PACKET_OUTGOING`

```rust
const PACKET_OUTGOING: crate::c_uchar = 4u8;
```

### `PACKET_LOOPBACK`

```rust
const PACKET_LOOPBACK: crate::c_uchar = 5u8;
```

### `PACKET_USER`

```rust
const PACKET_USER: crate::c_uchar = 6u8;
```

### `PACKET_KERNEL`

```rust
const PACKET_KERNEL: crate::c_uchar = 7u8;
```

### `PACKET_ADD_MEMBERSHIP`

```rust
const PACKET_ADD_MEMBERSHIP: crate::c_int = 1i32;
```

### `PACKET_DROP_MEMBERSHIP`

```rust
const PACKET_DROP_MEMBERSHIP: crate::c_int = 2i32;
```

### `PACKET_RECV_OUTPUT`

```rust
const PACKET_RECV_OUTPUT: crate::c_int = 3i32;
```

### `PACKET_RX_RING`

```rust
const PACKET_RX_RING: crate::c_int = 5i32;
```

### `PACKET_STATISTICS`

```rust
const PACKET_STATISTICS: crate::c_int = 6i32;
```

### `PACKET_COPY_THRESH`

```rust
const PACKET_COPY_THRESH: crate::c_int = 7i32;
```

### `PACKET_AUXDATA`

```rust
const PACKET_AUXDATA: crate::c_int = 8i32;
```

### `PACKET_ORIGDEV`

```rust
const PACKET_ORIGDEV: crate::c_int = 9i32;
```

### `PACKET_VERSION`

```rust
const PACKET_VERSION: crate::c_int = 10i32;
```

### `PACKET_HDRLEN`

```rust
const PACKET_HDRLEN: crate::c_int = 11i32;
```

### `PACKET_RESERVE`

```rust
const PACKET_RESERVE: crate::c_int = 12i32;
```

### `PACKET_TX_RING`

```rust
const PACKET_TX_RING: crate::c_int = 13i32;
```

### `PACKET_LOSS`

```rust
const PACKET_LOSS: crate::c_int = 14i32;
```

### `PACKET_VNET_HDR`

```rust
const PACKET_VNET_HDR: crate::c_int = 15i32;
```

### `PACKET_TX_TIMESTAMP`

```rust
const PACKET_TX_TIMESTAMP: crate::c_int = 16i32;
```

### `PACKET_TIMESTAMP`

```rust
const PACKET_TIMESTAMP: crate::c_int = 17i32;
```

### `PACKET_FANOUT`

```rust
const PACKET_FANOUT: crate::c_int = 18i32;
```

### `PACKET_TX_HAS_OFF`

```rust
const PACKET_TX_HAS_OFF: crate::c_int = 19i32;
```

### `PACKET_QDISC_BYPASS`

```rust
const PACKET_QDISC_BYPASS: crate::c_int = 20i32;
```

### `PACKET_ROLLOVER_STATS`

```rust
const PACKET_ROLLOVER_STATS: crate::c_int = 21i32;
```

### `PACKET_FANOUT_DATA`

```rust
const PACKET_FANOUT_DATA: crate::c_int = 22i32;
```

### `PACKET_IGNORE_OUTGOING`

```rust
const PACKET_IGNORE_OUTGOING: crate::c_int = 23i32;
```

### `PACKET_VNET_HDR_SZ`

```rust
const PACKET_VNET_HDR_SZ: crate::c_int = 24i32;
```

### `PACKET_FANOUT_HASH`

```rust
const PACKET_FANOUT_HASH: crate::c_uint = 0u32;
```

### `PACKET_FANOUT_LB`

```rust
const PACKET_FANOUT_LB: crate::c_uint = 1u32;
```

### `PACKET_FANOUT_CPU`

```rust
const PACKET_FANOUT_CPU: crate::c_uint = 2u32;
```

### `PACKET_FANOUT_ROLLOVER`

```rust
const PACKET_FANOUT_ROLLOVER: crate::c_uint = 3u32;
```

### `PACKET_FANOUT_RND`

```rust
const PACKET_FANOUT_RND: crate::c_uint = 4u32;
```

### `PACKET_FANOUT_QM`

```rust
const PACKET_FANOUT_QM: crate::c_uint = 5u32;
```

### `PACKET_FANOUT_CBPF`

```rust
const PACKET_FANOUT_CBPF: crate::c_uint = 6u32;
```

### `PACKET_FANOUT_EBPF`

```rust
const PACKET_FANOUT_EBPF: crate::c_uint = 7u32;
```

### `PACKET_FANOUT_FLAG_ROLLOVER`

```rust
const PACKET_FANOUT_FLAG_ROLLOVER: crate::c_uint = 4_096u32;
```

### `PACKET_FANOUT_FLAG_UNIQUEID`

```rust
const PACKET_FANOUT_FLAG_UNIQUEID: crate::c_uint = 8_192u32;
```

### `PACKET_FANOUT_FLAG_IGNORE_OUTGOING`

```rust
const PACKET_FANOUT_FLAG_IGNORE_OUTGOING: crate::c_uint = 16_384u32;
```

### `PACKET_FANOUT_FLAG_DEFRAG`

```rust
const PACKET_FANOUT_FLAG_DEFRAG: crate::c_uint = 32_768u32;
```

### `PACKET_MR_MULTICAST`

```rust
const PACKET_MR_MULTICAST: crate::c_int = 0i32;
```

### `PACKET_MR_PROMISC`

```rust
const PACKET_MR_PROMISC: crate::c_int = 1i32;
```

### `PACKET_MR_ALLMULTI`

```rust
const PACKET_MR_ALLMULTI: crate::c_int = 2i32;
```

### `TP_STATUS_KERNEL`

```rust
const TP_STATUS_KERNEL: __u32 = 0u32;
```

### `TP_STATUS_USER`

```rust
const TP_STATUS_USER: __u32 = 1u32;
```

### `TP_STATUS_COPY`

```rust
const TP_STATUS_COPY: __u32 = 2u32;
```

### `TP_STATUS_LOSING`

```rust
const TP_STATUS_LOSING: __u32 = 4u32;
```

### `TP_STATUS_CSUMNOTREADY`

```rust
const TP_STATUS_CSUMNOTREADY: __u32 = 8u32;
```

### `TP_STATUS_VLAN_VALID`

```rust
const TP_STATUS_VLAN_VALID: __u32 = 16u32;
```

### `TP_STATUS_BLK_TMO`

```rust
const TP_STATUS_BLK_TMO: __u32 = 32u32;
```

### `TP_STATUS_VLAN_TPID_VALID`

```rust
const TP_STATUS_VLAN_TPID_VALID: __u32 = 64u32;
```

### `TP_STATUS_CSUM_VALID`

```rust
const TP_STATUS_CSUM_VALID: __u32 = 128u32;
```

### `TP_STATUS_AVAILABLE`

```rust
const TP_STATUS_AVAILABLE: __u32 = 0u32;
```

### `TP_STATUS_SEND_REQUEST`

```rust
const TP_STATUS_SEND_REQUEST: __u32 = 1u32;
```

### `TP_STATUS_SENDING`

```rust
const TP_STATUS_SENDING: __u32 = 2u32;
```

### `TP_STATUS_WRONG_FORMAT`

```rust
const TP_STATUS_WRONG_FORMAT: __u32 = 4u32;
```

### `TP_STATUS_TS_SOFTWARE`

```rust
const TP_STATUS_TS_SOFTWARE: __u32 = 536_870_912u32;
```

### `TP_STATUS_TS_SYS_HARDWARE`

```rust
const TP_STATUS_TS_SYS_HARDWARE: __u32 = 1_073_741_824u32;
```

### `TP_STATUS_TS_RAW_HARDWARE`

```rust
const TP_STATUS_TS_RAW_HARDWARE: __u32 = 2_147_483_648u32;
```

### `TP_FT_REQ_FILL_RXHASH`

```rust
const TP_FT_REQ_FILL_RXHASH: __u32 = 1u32;
```

### `TPACKET_ALIGNMENT`

```rust
const TPACKET_ALIGNMENT: usize = 16usize;
```

### `TPACKET_HDRLEN`

```rust
const TPACKET_HDRLEN: usize = 52usize;
```

### `TPACKET2_HDRLEN`

```rust
const TPACKET2_HDRLEN: usize = 52usize;
```

### `TPACKET3_HDRLEN`

```rust
const TPACKET3_HDRLEN: usize = 68usize;
```

### `NF_DROP`

```rust
const NF_DROP: crate::c_int = 0i32;
```

### `NF_ACCEPT`

```rust
const NF_ACCEPT: crate::c_int = 1i32;
```

### `NF_STOLEN`

```rust
const NF_STOLEN: crate::c_int = 2i32;
```

### `NF_QUEUE`

```rust
const NF_QUEUE: crate::c_int = 3i32;
```

### `NF_REPEAT`

```rust
const NF_REPEAT: crate::c_int = 4i32;
```

### `NF_STOP`

```rust
const NF_STOP: crate::c_int = 5i32;
```

### `NF_MAX_VERDICT`

```rust
const NF_MAX_VERDICT: crate::c_int = 5i32;
```

### `NF_VERDICT_MASK`

```rust
const NF_VERDICT_MASK: crate::c_int = 255i32;
```

### `NF_VERDICT_FLAG_QUEUE_BYPASS`

```rust
const NF_VERDICT_FLAG_QUEUE_BYPASS: crate::c_int = 32_768i32;
```

### `NF_VERDICT_QMASK`

```rust
const NF_VERDICT_QMASK: crate::c_int = -65_536i32;
```

### `NF_VERDICT_QBITS`

```rust
const NF_VERDICT_QBITS: crate::c_int = 16i32;
```

### `NF_VERDICT_BITS`

```rust
const NF_VERDICT_BITS: crate::c_int = 16i32;
```

### `NF_INET_PRE_ROUTING`

```rust
const NF_INET_PRE_ROUTING: crate::c_int = 0i32;
```

### `NF_INET_LOCAL_IN`

```rust
const NF_INET_LOCAL_IN: crate::c_int = 1i32;
```

### `NF_INET_FORWARD`

```rust
const NF_INET_FORWARD: crate::c_int = 2i32;
```

### `NF_INET_LOCAL_OUT`

```rust
const NF_INET_LOCAL_OUT: crate::c_int = 3i32;
```

### `NF_INET_POST_ROUTING`

```rust
const NF_INET_POST_ROUTING: crate::c_int = 4i32;
```

### `NF_INET_NUMHOOKS`

```rust
const NF_INET_NUMHOOKS: crate::c_int = 5i32;
```

### `NF_INET_INGRESS`

```rust
const NF_INET_INGRESS: crate::c_int = 5i32;
```

### `NF_NETDEV_INGRESS`

```rust
const NF_NETDEV_INGRESS: crate::c_int = 0i32;
```

### `NF_NETDEV_EGRESS`

```rust
const NF_NETDEV_EGRESS: crate::c_int = 1i32;
```

### `NF_NETDEV_NUMHOOKS`

```rust
const NF_NETDEV_NUMHOOKS: crate::c_int = 2i32;
```

### `NFPROTO_UNSPEC`

```rust
const NFPROTO_UNSPEC: crate::c_int = 0i32;
```

### `NFPROTO_INET`

```rust
const NFPROTO_INET: crate::c_int = 1i32;
```

### `NFPROTO_IPV4`

```rust
const NFPROTO_IPV4: crate::c_int = 2i32;
```

### `NFPROTO_ARP`

```rust
const NFPROTO_ARP: crate::c_int = 3i32;
```

### `NFPROTO_NETDEV`

```rust
const NFPROTO_NETDEV: crate::c_int = 5i32;
```

### `NFPROTO_BRIDGE`

```rust
const NFPROTO_BRIDGE: crate::c_int = 7i32;
```

### `NFPROTO_IPV6`

```rust
const NFPROTO_IPV6: crate::c_int = 10i32;
```

### `NFPROTO_DECNET`

```rust
const NFPROTO_DECNET: crate::c_int = 12i32;
```

### `NFPROTO_NUMPROTO`

```rust
const NFPROTO_NUMPROTO: crate::c_int = 13i32;
```

### `NF_ARP`

```rust
const NF_ARP: crate::c_int = 0i32;
```

### `NF_ARP_IN`

```rust
const NF_ARP_IN: crate::c_int = 0i32;
```

### `NF_ARP_OUT`

```rust
const NF_ARP_OUT: crate::c_int = 1i32;
```

### `NF_ARP_FORWARD`

```rust
const NF_ARP_FORWARD: crate::c_int = 2i32;
```

### `NF_ARP_NUMHOOKS`

```rust
const NF_ARP_NUMHOOKS: crate::c_int = 3i32;
```

### `NF_BR_PRE_ROUTING`

```rust
const NF_BR_PRE_ROUTING: crate::c_int = 0i32;
```

### `NF_BR_LOCAL_IN`

```rust
const NF_BR_LOCAL_IN: crate::c_int = 1i32;
```

### `NF_BR_FORWARD`

```rust
const NF_BR_FORWARD: crate::c_int = 2i32;
```

### `NF_BR_LOCAL_OUT`

```rust
const NF_BR_LOCAL_OUT: crate::c_int = 3i32;
```

### `NF_BR_POST_ROUTING`

```rust
const NF_BR_POST_ROUTING: crate::c_int = 4i32;
```

### `NF_BR_BROUTING`

```rust
const NF_BR_BROUTING: crate::c_int = 5i32;
```

### `NF_BR_NUMHOOKS`

```rust
const NF_BR_NUMHOOKS: crate::c_int = 6i32;
```

### `NF_BR_PRI_FIRST`

```rust
const NF_BR_PRI_FIRST: crate::c_int = -2_147_483_648i32;
```

### `NF_BR_PRI_NAT_DST_BRIDGED`

```rust
const NF_BR_PRI_NAT_DST_BRIDGED: crate::c_int = -300i32;
```

### `NF_BR_PRI_FILTER_BRIDGED`

```rust
const NF_BR_PRI_FILTER_BRIDGED: crate::c_int = -200i32;
```

### `NF_BR_PRI_BRNF`

```rust
const NF_BR_PRI_BRNF: crate::c_int = 0i32;
```

### `NF_BR_PRI_NAT_DST_OTHER`

```rust
const NF_BR_PRI_NAT_DST_OTHER: crate::c_int = 100i32;
```

### `NF_BR_PRI_FILTER_OTHER`

```rust
const NF_BR_PRI_FILTER_OTHER: crate::c_int = 200i32;
```

### `NF_BR_PRI_NAT_SRC`

```rust
const NF_BR_PRI_NAT_SRC: crate::c_int = 300i32;
```

### `NF_BR_PRI_LAST`

```rust
const NF_BR_PRI_LAST: crate::c_int = 2_147_483_647i32;
```

### `NF_IP_PRE_ROUTING`

```rust
const NF_IP_PRE_ROUTING: crate::c_int = 0i32;
```

### `NF_IP_LOCAL_IN`

```rust
const NF_IP_LOCAL_IN: crate::c_int = 1i32;
```

### `NF_IP_FORWARD`

```rust
const NF_IP_FORWARD: crate::c_int = 2i32;
```

### `NF_IP_LOCAL_OUT`

```rust
const NF_IP_LOCAL_OUT: crate::c_int = 3i32;
```

### `NF_IP_POST_ROUTING`

```rust
const NF_IP_POST_ROUTING: crate::c_int = 4i32;
```

### `NF_IP_NUMHOOKS`

```rust
const NF_IP_NUMHOOKS: crate::c_int = 5i32;
```

### `NF_IP_PRI_FIRST`

```rust
const NF_IP_PRI_FIRST: crate::c_int = -2_147_483_648i32;
```

### `NF_IP_PRI_RAW_BEFORE_DEFRAG`

```rust
const NF_IP_PRI_RAW_BEFORE_DEFRAG: crate::c_int = -450i32;
```

### `NF_IP_PRI_CONNTRACK_DEFRAG`

```rust
const NF_IP_PRI_CONNTRACK_DEFRAG: crate::c_int = -400i32;
```

### `NF_IP_PRI_RAW`

```rust
const NF_IP_PRI_RAW: crate::c_int = -300i32;
```

### `NF_IP_PRI_SELINUX_FIRST`

```rust
const NF_IP_PRI_SELINUX_FIRST: crate::c_int = -225i32;
```

### `NF_IP_PRI_CONNTRACK`

```rust
const NF_IP_PRI_CONNTRACK: crate::c_int = -200i32;
```

### `NF_IP_PRI_MANGLE`

```rust
const NF_IP_PRI_MANGLE: crate::c_int = -150i32;
```

### `NF_IP_PRI_NAT_DST`

```rust
const NF_IP_PRI_NAT_DST: crate::c_int = -100i32;
```

### `NF_IP_PRI_FILTER`

```rust
const NF_IP_PRI_FILTER: crate::c_int = 0i32;
```

### `NF_IP_PRI_SECURITY`

```rust
const NF_IP_PRI_SECURITY: crate::c_int = 50i32;
```

### `NF_IP_PRI_NAT_SRC`

```rust
const NF_IP_PRI_NAT_SRC: crate::c_int = 100i32;
```

### `NF_IP_PRI_SELINUX_LAST`

```rust
const NF_IP_PRI_SELINUX_LAST: crate::c_int = 225i32;
```

### `NF_IP_PRI_CONNTRACK_HELPER`

```rust
const NF_IP_PRI_CONNTRACK_HELPER: crate::c_int = 300i32;
```

### `NF_IP_PRI_CONNTRACK_CONFIRM`

```rust
const NF_IP_PRI_CONNTRACK_CONFIRM: crate::c_int = 2_147_483_647i32;
```

### `NF_IP_PRI_LAST`

```rust
const NF_IP_PRI_LAST: crate::c_int = 2_147_483_647i32;
```

### `NF_IP6_PRE_ROUTING`

```rust
const NF_IP6_PRE_ROUTING: crate::c_int = 0i32;
```

### `NF_IP6_LOCAL_IN`

```rust
const NF_IP6_LOCAL_IN: crate::c_int = 1i32;
```

### `NF_IP6_FORWARD`

```rust
const NF_IP6_FORWARD: crate::c_int = 2i32;
```

### `NF_IP6_LOCAL_OUT`

```rust
const NF_IP6_LOCAL_OUT: crate::c_int = 3i32;
```

### `NF_IP6_POST_ROUTING`

```rust
const NF_IP6_POST_ROUTING: crate::c_int = 4i32;
```

### `NF_IP6_NUMHOOKS`

```rust
const NF_IP6_NUMHOOKS: crate::c_int = 5i32;
```

### `NF_IP6_PRI_FIRST`

```rust
const NF_IP6_PRI_FIRST: crate::c_int = -2_147_483_648i32;
```

### `NF_IP6_PRI_RAW_BEFORE_DEFRAG`

```rust
const NF_IP6_PRI_RAW_BEFORE_DEFRAG: crate::c_int = -450i32;
```

### `NF_IP6_PRI_CONNTRACK_DEFRAG`

```rust
const NF_IP6_PRI_CONNTRACK_DEFRAG: crate::c_int = -400i32;
```

### `NF_IP6_PRI_RAW`

```rust
const NF_IP6_PRI_RAW: crate::c_int = -300i32;
```

### `NF_IP6_PRI_SELINUX_FIRST`

```rust
const NF_IP6_PRI_SELINUX_FIRST: crate::c_int = -225i32;
```

### `NF_IP6_PRI_CONNTRACK`

```rust
const NF_IP6_PRI_CONNTRACK: crate::c_int = -200i32;
```

### `NF_IP6_PRI_MANGLE`

```rust
const NF_IP6_PRI_MANGLE: crate::c_int = -150i32;
```

### `NF_IP6_PRI_NAT_DST`

```rust
const NF_IP6_PRI_NAT_DST: crate::c_int = -100i32;
```

### `NF_IP6_PRI_FILTER`

```rust
const NF_IP6_PRI_FILTER: crate::c_int = 0i32;
```

### `NF_IP6_PRI_SECURITY`

```rust
const NF_IP6_PRI_SECURITY: crate::c_int = 50i32;
```

### `NF_IP6_PRI_NAT_SRC`

```rust
const NF_IP6_PRI_NAT_SRC: crate::c_int = 100i32;
```

### `NF_IP6_PRI_SELINUX_LAST`

```rust
const NF_IP6_PRI_SELINUX_LAST: crate::c_int = 225i32;
```

### `NF_IP6_PRI_CONNTRACK_HELPER`

```rust
const NF_IP6_PRI_CONNTRACK_HELPER: crate::c_int = 300i32;
```

### `NF_IP6_PRI_LAST`

```rust
const NF_IP6_PRI_LAST: crate::c_int = 2_147_483_647i32;
```

### `IP6T_SO_ORIGINAL_DST`

```rust
const IP6T_SO_ORIGINAL_DST: crate::c_int = 80i32;
```

### `SIOCADDRT`

```rust
const SIOCADDRT: crate::c_ulong = 35_083u64;
```

### `SIOCDELRT`

```rust
const SIOCDELRT: crate::c_ulong = 35_084u64;
```

### `SIOCGIFNAME`

```rust
const SIOCGIFNAME: crate::c_ulong = 35_088u64;
```

### `SIOCSIFLINK`

```rust
const SIOCSIFLINK: crate::c_ulong = 35_089u64;
```

### `SIOCGIFCONF`

```rust
const SIOCGIFCONF: crate::c_ulong = 35_090u64;
```

### `SIOCGIFFLAGS`

```rust
const SIOCGIFFLAGS: crate::c_ulong = 35_091u64;
```

### `SIOCSIFFLAGS`

```rust
const SIOCSIFFLAGS: crate::c_ulong = 35_092u64;
```

### `SIOCGIFADDR`

```rust
const SIOCGIFADDR: crate::c_ulong = 35_093u64;
```

### `SIOCSIFADDR`

```rust
const SIOCSIFADDR: crate::c_ulong = 35_094u64;
```

### `SIOCGIFDSTADDR`

```rust
const SIOCGIFDSTADDR: crate::c_ulong = 35_095u64;
```

### `SIOCSIFDSTADDR`

```rust
const SIOCSIFDSTADDR: crate::c_ulong = 35_096u64;
```

### `SIOCGIFBRDADDR`

```rust
const SIOCGIFBRDADDR: crate::c_ulong = 35_097u64;
```

### `SIOCSIFBRDADDR`

```rust
const SIOCSIFBRDADDR: crate::c_ulong = 35_098u64;
```

### `SIOCGIFNETMASK`

```rust
const SIOCGIFNETMASK: crate::c_ulong = 35_099u64;
```

### `SIOCSIFNETMASK`

```rust
const SIOCSIFNETMASK: crate::c_ulong = 35_100u64;
```

### `SIOCGIFMETRIC`

```rust
const SIOCGIFMETRIC: crate::c_ulong = 35_101u64;
```

### `SIOCSIFMETRIC`

```rust
const SIOCSIFMETRIC: crate::c_ulong = 35_102u64;
```

### `SIOCGIFMEM`

```rust
const SIOCGIFMEM: crate::c_ulong = 35_103u64;
```

### `SIOCSIFMEM`

```rust
const SIOCSIFMEM: crate::c_ulong = 35_104u64;
```

### `SIOCGIFMTU`

```rust
const SIOCGIFMTU: crate::c_ulong = 35_105u64;
```

### `SIOCSIFMTU`

```rust
const SIOCSIFMTU: crate::c_ulong = 35_106u64;
```

### `SIOCSIFNAME`

```rust
const SIOCSIFNAME: crate::c_ulong = 35_107u64;
```

### `SIOCSIFHWADDR`

```rust
const SIOCSIFHWADDR: crate::c_ulong = 35_108u64;
```

### `SIOCGIFENCAP`

```rust
const SIOCGIFENCAP: crate::c_ulong = 35_109u64;
```

### `SIOCSIFENCAP`

```rust
const SIOCSIFENCAP: crate::c_ulong = 35_110u64;
```

### `SIOCGIFHWADDR`

```rust
const SIOCGIFHWADDR: crate::c_ulong = 35_111u64;
```

### `SIOCGIFSLAVE`

```rust
const SIOCGIFSLAVE: crate::c_ulong = 35_113u64;
```

### `SIOCSIFSLAVE`

```rust
const SIOCSIFSLAVE: crate::c_ulong = 35_120u64;
```

### `SIOCADDMULTI`

```rust
const SIOCADDMULTI: crate::c_ulong = 35_121u64;
```

### `SIOCDELMULTI`

```rust
const SIOCDELMULTI: crate::c_ulong = 35_122u64;
```

### `SIOCGIFINDEX`

```rust
const SIOCGIFINDEX: crate::c_ulong = 35_123u64;
```

### `SIOGIFINDEX`

```rust
const SIOGIFINDEX: crate::c_ulong = 35_123u64;
```

### `SIOCSIFPFLAGS`

```rust
const SIOCSIFPFLAGS: crate::c_ulong = 35_124u64;
```

### `SIOCGIFPFLAGS`

```rust
const SIOCGIFPFLAGS: crate::c_ulong = 35_125u64;
```

### `SIOCDIFADDR`

```rust
const SIOCDIFADDR: crate::c_ulong = 35_126u64;
```

### `SIOCSIFHWBROADCAST`

```rust
const SIOCSIFHWBROADCAST: crate::c_ulong = 35_127u64;
```

### `SIOCGIFCOUNT`

```rust
const SIOCGIFCOUNT: crate::c_ulong = 35_128u64;
```

### `SIOCGIFBR`

```rust
const SIOCGIFBR: crate::c_ulong = 35_136u64;
```

### `SIOCSIFBR`

```rust
const SIOCSIFBR: crate::c_ulong = 35_137u64;
```

### `SIOCGIFTXQLEN`

```rust
const SIOCGIFTXQLEN: crate::c_ulong = 35_138u64;
```

### `SIOCSIFTXQLEN`

```rust
const SIOCSIFTXQLEN: crate::c_ulong = 35_139u64;
```

### `SIOCETHTOOL`

```rust
const SIOCETHTOOL: crate::c_ulong = 35_142u64;
```

### `SIOCGMIIPHY`

```rust
const SIOCGMIIPHY: crate::c_ulong = 35_143u64;
```

### `SIOCGMIIREG`

```rust
const SIOCGMIIREG: crate::c_ulong = 35_144u64;
```

### `SIOCSMIIREG`

```rust
const SIOCSMIIREG: crate::c_ulong = 35_145u64;
```

### `SIOCWANDEV`

```rust
const SIOCWANDEV: crate::c_ulong = 35_146u64;
```

### `SIOCOUTQNSD`

```rust
const SIOCOUTQNSD: crate::c_ulong = 35_147u64;
```

### `SIOCGSKNS`

```rust
const SIOCGSKNS: crate::c_ulong = 35_148u64;
```

### `SIOCDARP`

```rust
const SIOCDARP: crate::c_ulong = 35_155u64;
```

### `SIOCGARP`

```rust
const SIOCGARP: crate::c_ulong = 35_156u64;
```

### `SIOCSARP`

```rust
const SIOCSARP: crate::c_ulong = 35_157u64;
```

### `SIOCDRARP`

```rust
const SIOCDRARP: crate::c_ulong = 35_168u64;
```

### `SIOCGRARP`

```rust
const SIOCGRARP: crate::c_ulong = 35_169u64;
```

### `SIOCSRARP`

```rust
const SIOCSRARP: crate::c_ulong = 35_170u64;
```

### `SIOCGIFMAP`

```rust
const SIOCGIFMAP: crate::c_ulong = 35_184u64;
```

### `SIOCSIFMAP`

```rust
const SIOCSIFMAP: crate::c_ulong = 35_185u64;
```

### `SIOCSHWTSTAMP`

```rust
const SIOCSHWTSTAMP: crate::c_ulong = 35_248u64;
```

### `SIOCGHWTSTAMP`

```rust
const SIOCGHWTSTAMP: crate::c_ulong = 35_249u64;
```

### `WIRELESS_EXT`

```rust
const WIRELESS_EXT: crate::c_ulong = 22u64;
```

### `SIOCSIWCOMMIT`

```rust
const SIOCSIWCOMMIT: crate::c_ulong = 35_584u64;
```

### `SIOCGIWNAME`

```rust
const SIOCGIWNAME: crate::c_ulong = 35_585u64;
```

### `SIOCSIWNWID`

```rust
const SIOCSIWNWID: crate::c_ulong = 35_586u64;
```

### `SIOCGIWNWID`

```rust
const SIOCGIWNWID: crate::c_ulong = 35_587u64;
```

### `SIOCSIWFREQ`

```rust
const SIOCSIWFREQ: crate::c_ulong = 35_588u64;
```

### `SIOCGIWFREQ`

```rust
const SIOCGIWFREQ: crate::c_ulong = 35_589u64;
```

### `SIOCSIWMODE`

```rust
const SIOCSIWMODE: crate::c_ulong = 35_590u64;
```

### `SIOCGIWMODE`

```rust
const SIOCGIWMODE: crate::c_ulong = 35_591u64;
```

### `SIOCSIWSENS`

```rust
const SIOCSIWSENS: crate::c_ulong = 35_592u64;
```

### `SIOCGIWSENS`

```rust
const SIOCGIWSENS: crate::c_ulong = 35_593u64;
```

### `SIOCSIWRANGE`

```rust
const SIOCSIWRANGE: crate::c_ulong = 35_594u64;
```

### `SIOCGIWRANGE`

```rust
const SIOCGIWRANGE: crate::c_ulong = 35_595u64;
```

### `SIOCSIWPRIV`

```rust
const SIOCSIWPRIV: crate::c_ulong = 35_596u64;
```

### `SIOCGIWPRIV`

```rust
const SIOCGIWPRIV: crate::c_ulong = 35_597u64;
```

### `SIOCSIWSTATS`

```rust
const SIOCSIWSTATS: crate::c_ulong = 35_598u64;
```

### `SIOCGIWSTATS`

```rust
const SIOCGIWSTATS: crate::c_ulong = 35_599u64;
```

### `SIOCSIWSPY`

```rust
const SIOCSIWSPY: crate::c_ulong = 35_600u64;
```

### `SIOCGIWSPY`

```rust
const SIOCGIWSPY: crate::c_ulong = 35_601u64;
```

### `SIOCSIWTHRSPY`

```rust
const SIOCSIWTHRSPY: crate::c_ulong = 35_602u64;
```

### `SIOCGIWTHRSPY`

```rust
const SIOCGIWTHRSPY: crate::c_ulong = 35_603u64;
```

### `SIOCSIWAP`

```rust
const SIOCSIWAP: crate::c_ulong = 35_604u64;
```

### `SIOCGIWAP`

```rust
const SIOCGIWAP: crate::c_ulong = 35_605u64;
```

### `SIOCGIWAPLIST`

```rust
const SIOCGIWAPLIST: crate::c_ulong = 35_607u64;
```

### `SIOCSIWSCAN`

```rust
const SIOCSIWSCAN: crate::c_ulong = 35_608u64;
```

### `SIOCGIWSCAN`

```rust
const SIOCGIWSCAN: crate::c_ulong = 35_609u64;
```

### `SIOCSIWESSID`

```rust
const SIOCSIWESSID: crate::c_ulong = 35_610u64;
```

### `SIOCGIWESSID`

```rust
const SIOCGIWESSID: crate::c_ulong = 35_611u64;
```

### `SIOCSIWNICKN`

```rust
const SIOCSIWNICKN: crate::c_ulong = 35_612u64;
```

### `SIOCGIWNICKN`

```rust
const SIOCGIWNICKN: crate::c_ulong = 35_613u64;
```

### `SIOCSIWRATE`

```rust
const SIOCSIWRATE: crate::c_ulong = 35_616u64;
```

### `SIOCGIWRATE`

```rust
const SIOCGIWRATE: crate::c_ulong = 35_617u64;
```

### `SIOCSIWRTS`

```rust
const SIOCSIWRTS: crate::c_ulong = 35_618u64;
```

### `SIOCGIWRTS`

```rust
const SIOCGIWRTS: crate::c_ulong = 35_619u64;
```

### `SIOCSIWFRAG`

```rust
const SIOCSIWFRAG: crate::c_ulong = 35_620u64;
```

### `SIOCGIWFRAG`

```rust
const SIOCGIWFRAG: crate::c_ulong = 35_621u64;
```

### `SIOCSIWTXPOW`

```rust
const SIOCSIWTXPOW: crate::c_ulong = 35_622u64;
```

### `SIOCGIWTXPOW`

```rust
const SIOCGIWTXPOW: crate::c_ulong = 35_623u64;
```

### `SIOCSIWRETRY`

```rust
const SIOCSIWRETRY: crate::c_ulong = 35_624u64;
```

### `SIOCGIWRETRY`

```rust
const SIOCGIWRETRY: crate::c_ulong = 35_625u64;
```

### `SIOCSIWENCODE`

```rust
const SIOCSIWENCODE: crate::c_ulong = 35_626u64;
```

### `SIOCGIWENCODE`

```rust
const SIOCGIWENCODE: crate::c_ulong = 35_627u64;
```

### `SIOCSIWPOWER`

```rust
const SIOCSIWPOWER: crate::c_ulong = 35_628u64;
```

### `SIOCGIWPOWER`

```rust
const SIOCGIWPOWER: crate::c_ulong = 35_629u64;
```

### `SIOCSIWGENIE`

```rust
const SIOCSIWGENIE: crate::c_ulong = 35_632u64;
```

### `SIOCGIWGENIE`

```rust
const SIOCGIWGENIE: crate::c_ulong = 35_633u64;
```

### `SIOCSIWMLME`

```rust
const SIOCSIWMLME: crate::c_ulong = 35_606u64;
```

### `SIOCSIWAUTH`

```rust
const SIOCSIWAUTH: crate::c_ulong = 35_634u64;
```

### `SIOCGIWAUTH`

```rust
const SIOCGIWAUTH: crate::c_ulong = 35_635u64;
```

### `SIOCSIWENCODEEXT`

```rust
const SIOCSIWENCODEEXT: crate::c_ulong = 35_636u64;
```

### `SIOCGIWENCODEEXT`

```rust
const SIOCGIWENCODEEXT: crate::c_ulong = 35_637u64;
```

### `SIOCSIWPMKSA`

```rust
const SIOCSIWPMKSA: crate::c_ulong = 35_638u64;
```

### `SIOCIWFIRSTPRIV`

```rust
const SIOCIWFIRSTPRIV: crate::c_ulong = 35_808u64;
```

### `SIOCIWLASTPRIV`

```rust
const SIOCIWLASTPRIV: crate::c_ulong = 35_839u64;
```

### `SIOCIWFIRST`

```rust
const SIOCIWFIRST: crate::c_ulong = 35_584u64;
```

### `SIOCIWLAST`

```rust
const SIOCIWLAST: crate::c_ulong = 35_839u64;
```

### `IWEVTXDROP`

```rust
const IWEVTXDROP: crate::c_ulong = 35_840u64;
```

### `IWEVQUAL`

```rust
const IWEVQUAL: crate::c_ulong = 35_841u64;
```

### `IWEVCUSTOM`

```rust
const IWEVCUSTOM: crate::c_ulong = 35_842u64;
```

### `IWEVREGISTERED`

```rust
const IWEVREGISTERED: crate::c_ulong = 35_843u64;
```

### `IWEVEXPIRED`

```rust
const IWEVEXPIRED: crate::c_ulong = 35_844u64;
```

### `IWEVGENIE`

```rust
const IWEVGENIE: crate::c_ulong = 35_845u64;
```

### `IWEVMICHAELMICFAILURE`

```rust
const IWEVMICHAELMICFAILURE: crate::c_ulong = 35_846u64;
```

### `IWEVASSOCREQIE`

```rust
const IWEVASSOCREQIE: crate::c_ulong = 35_847u64;
```

### `IWEVASSOCRESPIE`

```rust
const IWEVASSOCRESPIE: crate::c_ulong = 35_848u64;
```

### `IWEVPMKIDCAND`

```rust
const IWEVPMKIDCAND: crate::c_ulong = 35_849u64;
```

### `IWEVFIRST`

```rust
const IWEVFIRST: crate::c_ulong = 35_840u64;
```

### `IW_PRIV_TYPE_MASK`

```rust
const IW_PRIV_TYPE_MASK: crate::c_ulong = 28_672u64;
```

### `IW_PRIV_TYPE_NONE`

```rust
const IW_PRIV_TYPE_NONE: crate::c_ulong = 0u64;
```

### `IW_PRIV_TYPE_BYTE`

```rust
const IW_PRIV_TYPE_BYTE: crate::c_ulong = 4_096u64;
```

### `IW_PRIV_TYPE_CHAR`

```rust
const IW_PRIV_TYPE_CHAR: crate::c_ulong = 8_192u64;
```

### `IW_PRIV_TYPE_INT`

```rust
const IW_PRIV_TYPE_INT: crate::c_ulong = 16_384u64;
```

### `IW_PRIV_TYPE_FLOAT`

```rust
const IW_PRIV_TYPE_FLOAT: crate::c_ulong = 20_480u64;
```

### `IW_PRIV_TYPE_ADDR`

```rust
const IW_PRIV_TYPE_ADDR: crate::c_ulong = 24_576u64;
```

### `IW_PRIV_SIZE_FIXED`

```rust
const IW_PRIV_SIZE_FIXED: crate::c_ulong = 2_048u64;
```

### `IW_PRIV_SIZE_MASK`

```rust
const IW_PRIV_SIZE_MASK: crate::c_ulong = 2_047u64;
```

### `IW_MAX_FREQUENCIES`

```rust
const IW_MAX_FREQUENCIES: usize = 32usize;
```

### `IW_MAX_BITRATES`

```rust
const IW_MAX_BITRATES: usize = 32usize;
```

### `IW_MAX_TXPOWER`

```rust
const IW_MAX_TXPOWER: usize = 8usize;
```

### `IW_MAX_SPY`

```rust
const IW_MAX_SPY: usize = 8usize;
```

### `IW_MAX_AP`

```rust
const IW_MAX_AP: usize = 64usize;
```

### `IW_ESSID_MAX_SIZE`

```rust
const IW_ESSID_MAX_SIZE: usize = 32usize;
```

### `IW_MODE_AUTO`

```rust
const IW_MODE_AUTO: usize = 0usize;
```

### `IW_MODE_ADHOC`

```rust
const IW_MODE_ADHOC: usize = 1usize;
```

### `IW_MODE_INFRA`

```rust
const IW_MODE_INFRA: usize = 2usize;
```

### `IW_MODE_MASTER`

```rust
const IW_MODE_MASTER: usize = 3usize;
```

### `IW_MODE_REPEAT`

```rust
const IW_MODE_REPEAT: usize = 4usize;
```

### `IW_MODE_SECOND`

```rust
const IW_MODE_SECOND: usize = 5usize;
```

### `IW_MODE_MONITOR`

```rust
const IW_MODE_MONITOR: usize = 6usize;
```

### `IW_MODE_MESH`

```rust
const IW_MODE_MESH: usize = 7usize;
```

### `IW_QUAL_QUAL_UPDATED`

```rust
const IW_QUAL_QUAL_UPDATED: crate::c_ulong = 1u64;
```

### `IW_QUAL_LEVEL_UPDATED`

```rust
const IW_QUAL_LEVEL_UPDATED: crate::c_ulong = 2u64;
```

### `IW_QUAL_NOISE_UPDATED`

```rust
const IW_QUAL_NOISE_UPDATED: crate::c_ulong = 4u64;
```

### `IW_QUAL_ALL_UPDATED`

```rust
const IW_QUAL_ALL_UPDATED: crate::c_ulong = 7u64;
```

### `IW_QUAL_DBM`

```rust
const IW_QUAL_DBM: crate::c_ulong = 8u64;
```

### `IW_QUAL_QUAL_INVALID`

```rust
const IW_QUAL_QUAL_INVALID: crate::c_ulong = 16u64;
```

### `IW_QUAL_LEVEL_INVALID`

```rust
const IW_QUAL_LEVEL_INVALID: crate::c_ulong = 32u64;
```

### `IW_QUAL_NOISE_INVALID`

```rust
const IW_QUAL_NOISE_INVALID: crate::c_ulong = 64u64;
```

### `IW_QUAL_RCPI`

```rust
const IW_QUAL_RCPI: crate::c_ulong = 128u64;
```

### `IW_QUAL_ALL_INVALID`

```rust
const IW_QUAL_ALL_INVALID: crate::c_ulong = 112u64;
```

### `IW_FREQ_AUTO`

```rust
const IW_FREQ_AUTO: crate::c_ulong = 0u64;
```

### `IW_FREQ_FIXED`

```rust
const IW_FREQ_FIXED: crate::c_ulong = 1u64;
```

### `IW_MAX_ENCODING_SIZES`

```rust
const IW_MAX_ENCODING_SIZES: usize = 8usize;
```

### `IW_ENCODING_TOKEN_MAX`

```rust
const IW_ENCODING_TOKEN_MAX: usize = 64usize;
```

### `IW_ENCODE_INDEX`

```rust
const IW_ENCODE_INDEX: crate::c_ulong = 255u64;
```

### `IW_ENCODE_FLAGS`

```rust
const IW_ENCODE_FLAGS: crate::c_ulong = 65_280u64;
```

### `IW_ENCODE_MODE`

```rust
const IW_ENCODE_MODE: crate::c_ulong = 61_440u64;
```

### `IW_ENCODE_DISABLED`

```rust
const IW_ENCODE_DISABLED: crate::c_ulong = 32_768u64;
```

### `IW_ENCODE_ENABLED`

```rust
const IW_ENCODE_ENABLED: crate::c_ulong = 0u64;
```

### `IW_ENCODE_RESTRICTED`

```rust
const IW_ENCODE_RESTRICTED: crate::c_ulong = 16_384u64;
```

### `IW_ENCODE_OPEN`

```rust
const IW_ENCODE_OPEN: crate::c_ulong = 8_192u64;
```

### `IW_ENCODE_NOKEY`

```rust
const IW_ENCODE_NOKEY: crate::c_ulong = 2_048u64;
```

### `IW_ENCODE_TEMP`

```rust
const IW_ENCODE_TEMP: crate::c_ulong = 1_024u64;
```

### `IW_POWER_ON`

```rust
const IW_POWER_ON: crate::c_ulong = 0u64;
```

### `IW_POWER_TYPE`

```rust
const IW_POWER_TYPE: crate::c_ulong = 61_440u64;
```

### `IW_POWER_PERIOD`

```rust
const IW_POWER_PERIOD: crate::c_ulong = 4_096u64;
```

### `IW_POWER_TIMEOUT`

```rust
const IW_POWER_TIMEOUT: crate::c_ulong = 8_192u64;
```

### `IW_POWER_MODE`

```rust
const IW_POWER_MODE: crate::c_ulong = 3_840u64;
```

### `IW_POWER_UNICAST_R`

```rust
const IW_POWER_UNICAST_R: crate::c_ulong = 256u64;
```

### `IW_POWER_MULTICAST_R`

```rust
const IW_POWER_MULTICAST_R: crate::c_ulong = 512u64;
```

### `IW_POWER_ALL_R`

```rust
const IW_POWER_ALL_R: crate::c_ulong = 768u64;
```

### `IW_POWER_FORCE_S`

```rust
const IW_POWER_FORCE_S: crate::c_ulong = 1_024u64;
```

### `IW_POWER_REPEATER`

```rust
const IW_POWER_REPEATER: crate::c_ulong = 2_048u64;
```

### `IW_POWER_MODIFIER`

```rust
const IW_POWER_MODIFIER: crate::c_ulong = 15u64;
```

### `IW_POWER_MIN`

```rust
const IW_POWER_MIN: crate::c_ulong = 1u64;
```

### `IW_POWER_MAX`

```rust
const IW_POWER_MAX: crate::c_ulong = 2u64;
```

### `IW_POWER_RELATIVE`

```rust
const IW_POWER_RELATIVE: crate::c_ulong = 4u64;
```

### `IW_TXPOW_TYPE`

```rust
const IW_TXPOW_TYPE: crate::c_ulong = 255u64;
```

### `IW_TXPOW_DBM`

```rust
const IW_TXPOW_DBM: crate::c_ulong = 0u64;
```

### `IW_TXPOW_MWATT`

```rust
const IW_TXPOW_MWATT: crate::c_ulong = 1u64;
```

### `IW_TXPOW_RELATIVE`

```rust
const IW_TXPOW_RELATIVE: crate::c_ulong = 2u64;
```

### `IW_TXPOW_RANGE`

```rust
const IW_TXPOW_RANGE: crate::c_ulong = 4_096u64;
```

### `IW_RETRY_ON`

```rust
const IW_RETRY_ON: crate::c_ulong = 0u64;
```

### `IW_RETRY_TYPE`

```rust
const IW_RETRY_TYPE: crate::c_ulong = 61_440u64;
```

### `IW_RETRY_LIMIT`

```rust
const IW_RETRY_LIMIT: crate::c_ulong = 4_096u64;
```

### `IW_RETRY_LIFETIME`

```rust
const IW_RETRY_LIFETIME: crate::c_ulong = 8_192u64;
```

### `IW_RETRY_MODIFIER`

```rust
const IW_RETRY_MODIFIER: crate::c_ulong = 255u64;
```

### `IW_RETRY_MIN`

```rust
const IW_RETRY_MIN: crate::c_ulong = 1u64;
```

### `IW_RETRY_MAX`

```rust
const IW_RETRY_MAX: crate::c_ulong = 2u64;
```

### `IW_RETRY_RELATIVE`

```rust
const IW_RETRY_RELATIVE: crate::c_ulong = 4u64;
```

### `IW_RETRY_SHORT`

```rust
const IW_RETRY_SHORT: crate::c_ulong = 16u64;
```

### `IW_RETRY_LONG`

```rust
const IW_RETRY_LONG: crate::c_ulong = 32u64;
```

### `IW_SCAN_DEFAULT`

```rust
const IW_SCAN_DEFAULT: crate::c_ulong = 0u64;
```

### `IW_SCAN_ALL_ESSID`

```rust
const IW_SCAN_ALL_ESSID: crate::c_ulong = 1u64;
```

### `IW_SCAN_THIS_ESSID`

```rust
const IW_SCAN_THIS_ESSID: crate::c_ulong = 2u64;
```

### `IW_SCAN_ALL_FREQ`

```rust
const IW_SCAN_ALL_FREQ: crate::c_ulong = 4u64;
```

### `IW_SCAN_THIS_FREQ`

```rust
const IW_SCAN_THIS_FREQ: crate::c_ulong = 8u64;
```

### `IW_SCAN_ALL_MODE`

```rust
const IW_SCAN_ALL_MODE: crate::c_ulong = 16u64;
```

### `IW_SCAN_THIS_MODE`

```rust
const IW_SCAN_THIS_MODE: crate::c_ulong = 32u64;
```

### `IW_SCAN_ALL_RATE`

```rust
const IW_SCAN_ALL_RATE: crate::c_ulong = 64u64;
```

### `IW_SCAN_THIS_RATE`

```rust
const IW_SCAN_THIS_RATE: crate::c_ulong = 128u64;
```

### `IW_SCAN_TYPE_ACTIVE`

```rust
const IW_SCAN_TYPE_ACTIVE: usize = 0usize;
```

### `IW_SCAN_TYPE_PASSIVE`

```rust
const IW_SCAN_TYPE_PASSIVE: usize = 1usize;
```

### `IW_SCAN_MAX_DATA`

```rust
const IW_SCAN_MAX_DATA: usize = 4_096usize;
```

### `IW_SCAN_CAPA_NONE`

```rust
const IW_SCAN_CAPA_NONE: crate::c_ulong = 0u64;
```

### `IW_SCAN_CAPA_ESSID`

```rust
const IW_SCAN_CAPA_ESSID: crate::c_ulong = 1u64;
```

### `IW_SCAN_CAPA_BSSID`

```rust
const IW_SCAN_CAPA_BSSID: crate::c_ulong = 2u64;
```

### `IW_SCAN_CAPA_CHANNEL`

```rust
const IW_SCAN_CAPA_CHANNEL: crate::c_ulong = 4u64;
```

### `IW_SCAN_CAPA_MODE`

```rust
const IW_SCAN_CAPA_MODE: crate::c_ulong = 8u64;
```

### `IW_SCAN_CAPA_RATE`

```rust
const IW_SCAN_CAPA_RATE: crate::c_ulong = 16u64;
```

### `IW_SCAN_CAPA_TYPE`

```rust
const IW_SCAN_CAPA_TYPE: crate::c_ulong = 32u64;
```

### `IW_SCAN_CAPA_TIME`

```rust
const IW_SCAN_CAPA_TIME: crate::c_ulong = 64u64;
```

### `IW_CUSTOM_MAX`

```rust
const IW_CUSTOM_MAX: crate::c_ulong = 256u64;
```

### `IW_GENERIC_IE_MAX`

```rust
const IW_GENERIC_IE_MAX: crate::c_ulong = 1_024u64;
```

### `IW_MLME_DEAUTH`

```rust
const IW_MLME_DEAUTH: crate::c_ulong = 0u64;
```

### `IW_MLME_DISASSOC`

```rust
const IW_MLME_DISASSOC: crate::c_ulong = 1u64;
```

### `IW_MLME_AUTH`

```rust
const IW_MLME_AUTH: crate::c_ulong = 2u64;
```

### `IW_MLME_ASSOC`

```rust
const IW_MLME_ASSOC: crate::c_ulong = 3u64;
```

### `IW_AUTH_INDEX`

```rust
const IW_AUTH_INDEX: crate::c_ulong = 4_095u64;
```

### `IW_AUTH_FLAGS`

```rust
const IW_AUTH_FLAGS: crate::c_ulong = 61_440u64;
```

### `IW_AUTH_WPA_VERSION`

```rust
const IW_AUTH_WPA_VERSION: usize = 0usize;
```

### `IW_AUTH_CIPHER_PAIRWISE`

```rust
const IW_AUTH_CIPHER_PAIRWISE: usize = 1usize;
```

### `IW_AUTH_CIPHER_GROUP`

```rust
const IW_AUTH_CIPHER_GROUP: usize = 2usize;
```

### `IW_AUTH_KEY_MGMT`

```rust
const IW_AUTH_KEY_MGMT: usize = 3usize;
```

### `IW_AUTH_TKIP_COUNTERMEASURES`

```rust
const IW_AUTH_TKIP_COUNTERMEASURES: usize = 4usize;
```

### `IW_AUTH_DROP_UNENCRYPTED`

```rust
const IW_AUTH_DROP_UNENCRYPTED: usize = 5usize;
```

### `IW_AUTH_80211_AUTH_ALG`

```rust
const IW_AUTH_80211_AUTH_ALG: usize = 6usize;
```

### `IW_AUTH_WPA_ENABLED`

```rust
const IW_AUTH_WPA_ENABLED: usize = 7usize;
```

### `IW_AUTH_RX_UNENCRYPTED_EAPOL`

```rust
const IW_AUTH_RX_UNENCRYPTED_EAPOL: usize = 8usize;
```

### `IW_AUTH_ROAMING_CONTROL`

```rust
const IW_AUTH_ROAMING_CONTROL: usize = 9usize;
```

### `IW_AUTH_PRIVACY_INVOKED`

```rust
const IW_AUTH_PRIVACY_INVOKED: usize = 10usize;
```

### `IW_AUTH_CIPHER_GROUP_MGMT`

```rust
const IW_AUTH_CIPHER_GROUP_MGMT: usize = 11usize;
```

### `IW_AUTH_MFP`

```rust
const IW_AUTH_MFP: usize = 12usize;
```

### `IW_AUTH_WPA_VERSION_DISABLED`

```rust
const IW_AUTH_WPA_VERSION_DISABLED: crate::c_ulong = 1u64;
```

### `IW_AUTH_WPA_VERSION_WPA`

```rust
const IW_AUTH_WPA_VERSION_WPA: crate::c_ulong = 2u64;
```

### `IW_AUTH_WPA_VERSION_WPA2`

```rust
const IW_AUTH_WPA_VERSION_WPA2: crate::c_ulong = 4u64;
```

### `IW_AUTH_CIPHER_NONE`

```rust
const IW_AUTH_CIPHER_NONE: crate::c_ulong = 1u64;
```

### `IW_AUTH_CIPHER_WEP40`

```rust
const IW_AUTH_CIPHER_WEP40: crate::c_ulong = 2u64;
```

### `IW_AUTH_CIPHER_TKIP`

```rust
const IW_AUTH_CIPHER_TKIP: crate::c_ulong = 4u64;
```

### `IW_AUTH_CIPHER_CCMP`

```rust
const IW_AUTH_CIPHER_CCMP: crate::c_ulong = 8u64;
```

### `IW_AUTH_CIPHER_WEP104`

```rust
const IW_AUTH_CIPHER_WEP104: crate::c_ulong = 16u64;
```

### `IW_AUTH_CIPHER_AES_CMAC`

```rust
const IW_AUTH_CIPHER_AES_CMAC: crate::c_ulong = 32u64;
```

### `IW_AUTH_KEY_MGMT_802_1X`

```rust
const IW_AUTH_KEY_MGMT_802_1X: usize = 1usize;
```

### `IW_AUTH_KEY_MGMT_PSK`

```rust
const IW_AUTH_KEY_MGMT_PSK: usize = 2usize;
```

### `IW_AUTH_ALG_OPEN_SYSTEM`

```rust
const IW_AUTH_ALG_OPEN_SYSTEM: crate::c_ulong = 1u64;
```

### `IW_AUTH_ALG_SHARED_KEY`

```rust
const IW_AUTH_ALG_SHARED_KEY: crate::c_ulong = 2u64;
```

### `IW_AUTH_ALG_LEAP`

```rust
const IW_AUTH_ALG_LEAP: crate::c_ulong = 4u64;
```

### `IW_AUTH_ROAMING_ENABLE`

```rust
const IW_AUTH_ROAMING_ENABLE: usize = 0usize;
```

### `IW_AUTH_ROAMING_DISABLE`

```rust
const IW_AUTH_ROAMING_DISABLE: usize = 1usize;
```

### `IW_AUTH_MFP_DISABLED`

```rust
const IW_AUTH_MFP_DISABLED: usize = 0usize;
```

### `IW_AUTH_MFP_OPTIONAL`

```rust
const IW_AUTH_MFP_OPTIONAL: usize = 1usize;
```

### `IW_AUTH_MFP_REQUIRED`

```rust
const IW_AUTH_MFP_REQUIRED: usize = 2usize;
```

### `IW_ENCODE_SEQ_MAX_SIZE`

```rust
const IW_ENCODE_SEQ_MAX_SIZE: usize = 8usize;
```

### `IW_ENCODE_ALG_NONE`

```rust
const IW_ENCODE_ALG_NONE: usize = 0usize;
```

### `IW_ENCODE_ALG_WEP`

```rust
const IW_ENCODE_ALG_WEP: usize = 1usize;
```

### `IW_ENCODE_ALG_TKIP`

```rust
const IW_ENCODE_ALG_TKIP: usize = 2usize;
```

### `IW_ENCODE_ALG_CCMP`

```rust
const IW_ENCODE_ALG_CCMP: usize = 3usize;
```

### `IW_ENCODE_ALG_PMK`

```rust
const IW_ENCODE_ALG_PMK: usize = 4usize;
```

### `IW_ENCODE_ALG_AES_CMAC`

```rust
const IW_ENCODE_ALG_AES_CMAC: usize = 5usize;
```

### `IW_ENCODE_EXT_TX_SEQ_VALID`

```rust
const IW_ENCODE_EXT_TX_SEQ_VALID: crate::c_ulong = 1u64;
```

### `IW_ENCODE_EXT_RX_SEQ_VALID`

```rust
const IW_ENCODE_EXT_RX_SEQ_VALID: crate::c_ulong = 2u64;
```

### `IW_ENCODE_EXT_GROUP_KEY`

```rust
const IW_ENCODE_EXT_GROUP_KEY: crate::c_ulong = 4u64;
```

### `IW_ENCODE_EXT_SET_TX_KEY`

```rust
const IW_ENCODE_EXT_SET_TX_KEY: crate::c_ulong = 8u64;
```

### `IW_MICFAILURE_KEY_ID`

```rust
const IW_MICFAILURE_KEY_ID: crate::c_ulong = 3u64;
```

### `IW_MICFAILURE_GROUP`

```rust
const IW_MICFAILURE_GROUP: crate::c_ulong = 4u64;
```

### `IW_MICFAILURE_PAIRWISE`

```rust
const IW_MICFAILURE_PAIRWISE: crate::c_ulong = 8u64;
```

### `IW_MICFAILURE_STAKEY`

```rust
const IW_MICFAILURE_STAKEY: crate::c_ulong = 16u64;
```

### `IW_MICFAILURE_COUNT`

```rust
const IW_MICFAILURE_COUNT: crate::c_ulong = 96u64;
```

### `IW_ENC_CAPA_WPA`

```rust
const IW_ENC_CAPA_WPA: crate::c_ulong = 1u64;
```

### `IW_ENC_CAPA_WPA2`

```rust
const IW_ENC_CAPA_WPA2: crate::c_ulong = 2u64;
```

### `IW_ENC_CAPA_CIPHER_TKIP`

```rust
const IW_ENC_CAPA_CIPHER_TKIP: crate::c_ulong = 4u64;
```

### `IW_ENC_CAPA_CIPHER_CCMP`

```rust
const IW_ENC_CAPA_CIPHER_CCMP: crate::c_ulong = 8u64;
```

### `IW_ENC_CAPA_4WAY_HANDSHAKE`

```rust
const IW_ENC_CAPA_4WAY_HANDSHAKE: crate::c_ulong = 16u64;
```

### `IW_EVENT_CAPA_K_0`

```rust
const IW_EVENT_CAPA_K_0: crate::c_ulong = 67_108_944u64;
```

### `IW_EVENT_CAPA_K_1`

```rust
const IW_EVENT_CAPA_K_1: crate::c_ulong = 1_024u64;
```

### `IW_PMKSA_ADD`

```rust
const IW_PMKSA_ADD: usize = 1usize;
```

### `IW_PMKSA_REMOVE`

```rust
const IW_PMKSA_REMOVE: usize = 2usize;
```

### `IW_PMKSA_FLUSH`

```rust
const IW_PMKSA_FLUSH: usize = 3usize;
```

### `IW_PMKID_LEN`

```rust
const IW_PMKID_LEN: usize = 16usize;
```

### `IW_PMKID_CAND_PREAUTH`

```rust
const IW_PMKID_CAND_PREAUTH: crate::c_ulong = 1u64;
```

### `IW_EV_LCP_PK_LEN`

```rust
const IW_EV_LCP_PK_LEN: usize = 4usize;
```

### `IW_EV_CHAR_PK_LEN`

```rust
const IW_EV_CHAR_PK_LEN: usize = 20usize;
```

### `IW_EV_UINT_PK_LEN`

```rust
const IW_EV_UINT_PK_LEN: usize = 8usize;
```

### `IW_EV_FREQ_PK_LEN`

```rust
const IW_EV_FREQ_PK_LEN: usize = 12usize;
```

### `IW_EV_PARAM_PK_LEN`

```rust
const IW_EV_PARAM_PK_LEN: usize = 12usize;
```

### `IW_EV_ADDR_PK_LEN`

```rust
const IW_EV_ADDR_PK_LEN: usize = 20usize;
```

### `IW_EV_QUAL_PK_LEN`

```rust
const IW_EV_QUAL_PK_LEN: usize = 8usize;
```

### `IW_EV_POINT_PK_LEN`

```rust
const IW_EV_POINT_PK_LEN: usize = 8usize;
```

### `IPTOS_TOS_MASK`

```rust
const IPTOS_TOS_MASK: u8 = 30u8;
```

### `IPTOS_PREC_MASK`

```rust
const IPTOS_PREC_MASK: u8 = 224u8;
```

### `IPTOS_ECN_NOT_ECT`

```rust
const IPTOS_ECN_NOT_ECT: u8 = 0u8;
```

### `RTF_UP`

```rust
const RTF_UP: crate::c_ushort = 1u16;
```

### `RTF_GATEWAY`

```rust
const RTF_GATEWAY: crate::c_ushort = 2u16;
```

### `RTF_HOST`

```rust
const RTF_HOST: crate::c_ushort = 4u16;
```

### `RTF_REINSTATE`

```rust
const RTF_REINSTATE: crate::c_ushort = 8u16;
```

### `RTF_DYNAMIC`

```rust
const RTF_DYNAMIC: crate::c_ushort = 16u16;
```

### `RTF_MODIFIED`

```rust
const RTF_MODIFIED: crate::c_ushort = 32u16;
```

### `RTF_MTU`

```rust
const RTF_MTU: crate::c_ushort = 64u16;
```

### `RTF_MSS`

```rust
const RTF_MSS: crate::c_ushort = 64u16;
```

### `RTF_WINDOW`

```rust
const RTF_WINDOW: crate::c_ushort = 128u16;
```

### `RTF_IRTT`

```rust
const RTF_IRTT: crate::c_ushort = 256u16;
```

### `RTF_REJECT`

```rust
const RTF_REJECT: crate::c_ushort = 512u16;
```

### `RTF_STATIC`

```rust
const RTF_STATIC: crate::c_ushort = 1_024u16;
```

### `RTF_XRESOLVE`

```rust
const RTF_XRESOLVE: crate::c_ushort = 2_048u16;
```

### `RTF_NOFORWARD`

```rust
const RTF_NOFORWARD: crate::c_ushort = 4_096u16;
```

### `RTF_THROW`

```rust
const RTF_THROW: crate::c_ushort = 8_192u16;
```

### `RTF_NOPMTUDISC`

```rust
const RTF_NOPMTUDISC: crate::c_ushort = 16_384u16;
```

### `RTF_DEFAULT`

```rust
const RTF_DEFAULT: u32 = 65_536u32;
```

### `RTF_ALLONLINK`

```rust
const RTF_ALLONLINK: u32 = 131_072u32;
```

### `RTF_ADDRCONF`

```rust
const RTF_ADDRCONF: u32 = 262_144u32;
```

### `RTF_LINKRT`

```rust
const RTF_LINKRT: u32 = 1_048_576u32;
```

### `RTF_NONEXTHOP`

```rust
const RTF_NONEXTHOP: u32 = 2_097_152u32;
```

### `RTF_CACHE`

```rust
const RTF_CACHE: u32 = 16_777_216u32;
```

### `RTF_FLOW`

```rust
const RTF_FLOW: u32 = 33_554_432u32;
```

### `RTF_POLICY`

```rust
const RTF_POLICY: u32 = 67_108_864u32;
```

### `RTCF_VALVE`

```rust
const RTCF_VALVE: u32 = 2_097_152u32;
```

### `RTCF_MASQ`

```rust
const RTCF_MASQ: u32 = 4_194_304u32;
```

### `RTCF_NAT`

```rust
const RTCF_NAT: u32 = 8_388_608u32;
```

### `RTCF_DOREDIRECT`

```rust
const RTCF_DOREDIRECT: u32 = 16_777_216u32;
```

### `RTCF_LOG`

```rust
const RTCF_LOG: u32 = 33_554_432u32;
```

### `RTCF_DIRECTSRC`

```rust
const RTCF_DIRECTSRC: u32 = 67_108_864u32;
```

### `RTF_LOCAL`

```rust
const RTF_LOCAL: u32 = 2_147_483_648u32;
```

### `RTF_INTERFACE`

```rust
const RTF_INTERFACE: u32 = 1_073_741_824u32;
```

### `RTF_MULTICAST`

```rust
const RTF_MULTICAST: u32 = 536_870_912u32;
```

### `RTF_BROADCAST`

```rust
const RTF_BROADCAST: u32 = 268_435_456u32;
```

### `RTF_NAT`

```rust
const RTF_NAT: u32 = 134_217_728u32;
```

### `RTF_ADDRCLASSMASK`

```rust
const RTF_ADDRCLASSMASK: u32 = 4_160_749_568u32;
```

### `RT_CLASS_UNSPEC`

```rust
const RT_CLASS_UNSPEC: u8 = 0u8;
```

### `RT_CLASS_DEFAULT`

```rust
const RT_CLASS_DEFAULT: u8 = 253u8;
```

### `RT_CLASS_MAIN`

```rust
const RT_CLASS_MAIN: u8 = 254u8;
```

### `RT_CLASS_LOCAL`

```rust
const RT_CLASS_LOCAL: u8 = 255u8;
```

### `RT_CLASS_MAX`

```rust
const RT_CLASS_MAX: u8 = 255u8;
```

### `NUD_NONE`

```rust
const NUD_NONE: u16 = 0u16;
```

### `NUD_INCOMPLETE`

```rust
const NUD_INCOMPLETE: u16 = 1u16;
```

### `NUD_REACHABLE`

```rust
const NUD_REACHABLE: u16 = 2u16;
```

### `NUD_STALE`

```rust
const NUD_STALE: u16 = 4u16;
```

### `NUD_DELAY`

```rust
const NUD_DELAY: u16 = 8u16;
```

### `NUD_PROBE`

```rust
const NUD_PROBE: u16 = 16u16;
```

### `NUD_FAILED`

```rust
const NUD_FAILED: u16 = 32u16;
```

### `NUD_NOARP`

```rust
const NUD_NOARP: u16 = 64u16;
```

### `NUD_PERMANENT`

```rust
const NUD_PERMANENT: u16 = 128u16;
```

### `NTF_USE`

```rust
const NTF_USE: u8 = 1u8;
```

### `NTF_SELF`

```rust
const NTF_SELF: u8 = 2u8;
```

### `NTF_MASTER`

```rust
const NTF_MASTER: u8 = 4u8;
```

### `NTF_PROXY`

```rust
const NTF_PROXY: u8 = 8u8;
```

### `NTF_ROUTER`

```rust
const NTF_ROUTER: u8 = 128u8;
```

### `NDA_UNSPEC`

```rust
const NDA_UNSPEC: crate::c_ushort = 0u16;
```

### `NDA_DST`

```rust
const NDA_DST: crate::c_ushort = 1u16;
```

### `NDA_LLADDR`

```rust
const NDA_LLADDR: crate::c_ushort = 2u16;
```

### `NDA_CACHEINFO`

```rust
const NDA_CACHEINFO: crate::c_ushort = 3u16;
```

### `NDA_PROBES`

```rust
const NDA_PROBES: crate::c_ushort = 4u16;
```

### `NDA_VLAN`

```rust
const NDA_VLAN: crate::c_ushort = 5u16;
```

### `NDA_PORT`

```rust
const NDA_PORT: crate::c_ushort = 6u16;
```

### `NDA_VNI`

```rust
const NDA_VNI: crate::c_ushort = 7u16;
```

### `NDA_IFINDEX`

```rust
const NDA_IFINDEX: crate::c_ushort = 8u16;
```

### `NLA_ALIGNTO`

```rust
const NLA_ALIGNTO: crate::c_int = 4i32;
```

### `NETLINK_ROUTE`

```rust
const NETLINK_ROUTE: crate::c_int = 0i32;
```

### `NETLINK_UNUSED`

```rust
const NETLINK_UNUSED: crate::c_int = 1i32;
```

### `NETLINK_USERSOCK`

```rust
const NETLINK_USERSOCK: crate::c_int = 2i32;
```

### `NETLINK_FIREWALL`

```rust
const NETLINK_FIREWALL: crate::c_int = 3i32;
```

### `NETLINK_SOCK_DIAG`

```rust
const NETLINK_SOCK_DIAG: crate::c_int = 4i32;
```

### `NETLINK_NFLOG`

```rust
const NETLINK_NFLOG: crate::c_int = 5i32;
```

### `NETLINK_XFRM`

```rust
const NETLINK_XFRM: crate::c_int = 6i32;
```

### `NETLINK_SELINUX`

```rust
const NETLINK_SELINUX: crate::c_int = 7i32;
```

### `NETLINK_ISCSI`

```rust
const NETLINK_ISCSI: crate::c_int = 8i32;
```

### `NETLINK_AUDIT`

```rust
const NETLINK_AUDIT: crate::c_int = 9i32;
```

### `NETLINK_FIB_LOOKUP`

```rust
const NETLINK_FIB_LOOKUP: crate::c_int = 10i32;
```

### `NETLINK_CONNECTOR`

```rust
const NETLINK_CONNECTOR: crate::c_int = 11i32;
```

### `NETLINK_NETFILTER`

```rust
const NETLINK_NETFILTER: crate::c_int = 12i32;
```

### `NETLINK_IP6_FW`

```rust
const NETLINK_IP6_FW: crate::c_int = 13i32;
```

### `NETLINK_DNRTMSG`

```rust
const NETLINK_DNRTMSG: crate::c_int = 14i32;
```

### `NETLINK_KOBJECT_UEVENT`

```rust
const NETLINK_KOBJECT_UEVENT: crate::c_int = 15i32;
```

### `NETLINK_GENERIC`

```rust
const NETLINK_GENERIC: crate::c_int = 16i32;
```

### `NETLINK_SCSITRANSPORT`

```rust
const NETLINK_SCSITRANSPORT: crate::c_int = 18i32;
```

### `NETLINK_ECRYPTFS`

```rust
const NETLINK_ECRYPTFS: crate::c_int = 19i32;
```

### `NETLINK_RDMA`

```rust
const NETLINK_RDMA: crate::c_int = 20i32;
```

### `NETLINK_CRYPTO`

```rust
const NETLINK_CRYPTO: crate::c_int = 21i32;
```

### `NETLINK_INET_DIAG`

```rust
const NETLINK_INET_DIAG: crate::c_int = 4i32;
```

### `NLM_F_REQUEST`

```rust
const NLM_F_REQUEST: crate::c_int = 1i32;
```

### `NLM_F_MULTI`

```rust
const NLM_F_MULTI: crate::c_int = 2i32;
```

### `NLM_F_ACK`

```rust
const NLM_F_ACK: crate::c_int = 4i32;
```

### `NLM_F_ECHO`

```rust
const NLM_F_ECHO: crate::c_int = 8i32;
```

### `NLM_F_DUMP_INTR`

```rust
const NLM_F_DUMP_INTR: crate::c_int = 16i32;
```

### `NLM_F_DUMP_FILTERED`

```rust
const NLM_F_DUMP_FILTERED: crate::c_int = 32i32;
```

### `NLM_F_ROOT`

```rust
const NLM_F_ROOT: crate::c_int = 256i32;
```

### `NLM_F_MATCH`

```rust
const NLM_F_MATCH: crate::c_int = 512i32;
```

### `NLM_F_ATOMIC`

```rust
const NLM_F_ATOMIC: crate::c_int = 1_024i32;
```

### `NLM_F_DUMP`

```rust
const NLM_F_DUMP: crate::c_int = 768i32;
```

### `NLM_F_REPLACE`

```rust
const NLM_F_REPLACE: crate::c_int = 256i32;
```

### `NLM_F_EXCL`

```rust
const NLM_F_EXCL: crate::c_int = 512i32;
```

### `NLM_F_CREATE`

```rust
const NLM_F_CREATE: crate::c_int = 1_024i32;
```

### `NLM_F_APPEND`

```rust
const NLM_F_APPEND: crate::c_int = 2_048i32;
```

### `NLM_F_NONREC`

```rust
const NLM_F_NONREC: crate::c_int = 256i32;
```

### `NLM_F_BULK`

```rust
const NLM_F_BULK: crate::c_int = 512i32;
```

### `NLM_F_CAPPED`

```rust
const NLM_F_CAPPED: crate::c_int = 256i32;
```

### `NLM_F_ACK_TLVS`

```rust
const NLM_F_ACK_TLVS: crate::c_int = 512i32;
```

### `NETLINK_ADD_MEMBERSHIP`

```rust
const NETLINK_ADD_MEMBERSHIP: crate::c_int = 1i32;
```

### `NETLINK_DROP_MEMBERSHIP`

```rust
const NETLINK_DROP_MEMBERSHIP: crate::c_int = 2i32;
```

### `NETLINK_PKTINFO`

```rust
const NETLINK_PKTINFO: crate::c_int = 3i32;
```

### `NETLINK_BROADCAST_ERROR`

```rust
const NETLINK_BROADCAST_ERROR: crate::c_int = 4i32;
```

### `NETLINK_NO_ENOBUFS`

```rust
const NETLINK_NO_ENOBUFS: crate::c_int = 5i32;
```

### `NETLINK_RX_RING`

```rust
const NETLINK_RX_RING: crate::c_int = 6i32;
```

### `NETLINK_TX_RING`

```rust
const NETLINK_TX_RING: crate::c_int = 7i32;
```

### `NETLINK_LISTEN_ALL_NSID`

```rust
const NETLINK_LISTEN_ALL_NSID: crate::c_int = 8i32;
```

### `NETLINK_LIST_MEMBERSHIPS`

```rust
const NETLINK_LIST_MEMBERSHIPS: crate::c_int = 9i32;
```

### `NETLINK_CAP_ACK`

```rust
const NETLINK_CAP_ACK: crate::c_int = 10i32;
```

### `NETLINK_EXT_ACK`

```rust
const NETLINK_EXT_ACK: crate::c_int = 11i32;
```

### `NETLINK_GET_STRICT_CHK`

```rust
const NETLINK_GET_STRICT_CHK: crate::c_int = 12i32;
```

### `NLA_F_NESTED`

```rust
const NLA_F_NESTED: crate::c_int = 32_768i32;
```

### `NLA_F_NET_BYTEORDER`

```rust
const NLA_F_NET_BYTEORDER: crate::c_int = 16_384i32;
```

### `NLA_TYPE_MASK`

```rust
const NLA_TYPE_MASK: crate::c_int = -49_153i32;
```

### `TCA_UNSPEC`

```rust
const TCA_UNSPEC: crate::c_ushort = 0u16;
```

### `TCA_KIND`

```rust
const TCA_KIND: crate::c_ushort = 1u16;
```

### `TCA_OPTIONS`

```rust
const TCA_OPTIONS: crate::c_ushort = 2u16;
```

### `TCA_STATS`

```rust
const TCA_STATS: crate::c_ushort = 3u16;
```

### `TCA_XSTATS`

```rust
const TCA_XSTATS: crate::c_ushort = 4u16;
```

### `TCA_RATE`

```rust
const TCA_RATE: crate::c_ushort = 5u16;
```

### `TCA_FCNT`

```rust
const TCA_FCNT: crate::c_ushort = 6u16;
```

### `TCA_STATS2`

```rust
const TCA_STATS2: crate::c_ushort = 7u16;
```

### `TCA_STAB`

```rust
const TCA_STAB: crate::c_ushort = 8u16;
```

### `RTM_NEWLINK`

```rust
const RTM_NEWLINK: u16 = 16u16;
```

### `RTM_DELLINK`

```rust
const RTM_DELLINK: u16 = 17u16;
```

### `RTM_GETLINK`

```rust
const RTM_GETLINK: u16 = 18u16;
```

### `RTM_SETLINK`

```rust
const RTM_SETLINK: u16 = 19u16;
```

### `RTM_NEWADDR`

```rust
const RTM_NEWADDR: u16 = 20u16;
```

### `RTM_DELADDR`

```rust
const RTM_DELADDR: u16 = 21u16;
```

### `RTM_GETADDR`

```rust
const RTM_GETADDR: u16 = 22u16;
```

### `RTM_NEWROUTE`

```rust
const RTM_NEWROUTE: u16 = 24u16;
```

### `RTM_DELROUTE`

```rust
const RTM_DELROUTE: u16 = 25u16;
```

### `RTM_GETROUTE`

```rust
const RTM_GETROUTE: u16 = 26u16;
```

### `RTM_NEWNEIGH`

```rust
const RTM_NEWNEIGH: u16 = 28u16;
```

### `RTM_DELNEIGH`

```rust
const RTM_DELNEIGH: u16 = 29u16;
```

### `RTM_GETNEIGH`

```rust
const RTM_GETNEIGH: u16 = 30u16;
```

### `RTM_NEWRULE`

```rust
const RTM_NEWRULE: u16 = 32u16;
```

### `RTM_DELRULE`

```rust
const RTM_DELRULE: u16 = 33u16;
```

### `RTM_GETRULE`

```rust
const RTM_GETRULE: u16 = 34u16;
```

### `RTM_NEWQDISC`

```rust
const RTM_NEWQDISC: u16 = 36u16;
```

### `RTM_DELQDISC`

```rust
const RTM_DELQDISC: u16 = 37u16;
```

### `RTM_GETQDISC`

```rust
const RTM_GETQDISC: u16 = 38u16;
```

### `RTM_NEWTCLASS`

```rust
const RTM_NEWTCLASS: u16 = 40u16;
```

### `RTM_DELTCLASS`

```rust
const RTM_DELTCLASS: u16 = 41u16;
```

### `RTM_GETTCLASS`

```rust
const RTM_GETTCLASS: u16 = 42u16;
```

### `RTM_NEWTFILTER`

```rust
const RTM_NEWTFILTER: u16 = 44u16;
```

### `RTM_DELTFILTER`

```rust
const RTM_DELTFILTER: u16 = 45u16;
```

### `RTM_GETTFILTER`

```rust
const RTM_GETTFILTER: u16 = 46u16;
```

### `RTM_NEWACTION`

```rust
const RTM_NEWACTION: u16 = 48u16;
```

### `RTM_DELACTION`

```rust
const RTM_DELACTION: u16 = 49u16;
```

### `RTM_GETACTION`

```rust
const RTM_GETACTION: u16 = 50u16;
```

### `RTM_NEWPREFIX`

```rust
const RTM_NEWPREFIX: u16 = 52u16;
```

### `RTM_GETMULTICAST`

```rust
const RTM_GETMULTICAST: u16 = 58u16;
```

### `RTM_GETANYCAST`

```rust
const RTM_GETANYCAST: u16 = 62u16;
```

### `RTM_NEWNEIGHTBL`

```rust
const RTM_NEWNEIGHTBL: u16 = 64u16;
```

### `RTM_GETNEIGHTBL`

```rust
const RTM_GETNEIGHTBL: u16 = 66u16;
```

### `RTM_SETNEIGHTBL`

```rust
const RTM_SETNEIGHTBL: u16 = 67u16;
```

### `RTM_NEWNDUSEROPT`

```rust
const RTM_NEWNDUSEROPT: u16 = 68u16;
```

### `RTM_NEWADDRLABEL`

```rust
const RTM_NEWADDRLABEL: u16 = 72u16;
```

### `RTM_DELADDRLABEL`

```rust
const RTM_DELADDRLABEL: u16 = 73u16;
```

### `RTM_GETADDRLABEL`

```rust
const RTM_GETADDRLABEL: u16 = 74u16;
```

### `RTM_GETDCB`

```rust
const RTM_GETDCB: u16 = 78u16;
```

### `RTM_SETDCB`

```rust
const RTM_SETDCB: u16 = 79u16;
```

### `RTM_NEWNETCONF`

```rust
const RTM_NEWNETCONF: u16 = 80u16;
```

### `RTM_GETNETCONF`

```rust
const RTM_GETNETCONF: u16 = 82u16;
```

### `RTM_NEWMDB`

```rust
const RTM_NEWMDB: u16 = 84u16;
```

### `RTM_DELMDB`

```rust
const RTM_DELMDB: u16 = 85u16;
```

### `RTM_GETMDB`

```rust
const RTM_GETMDB: u16 = 86u16;
```

### `RTM_NEWNSID`

```rust
const RTM_NEWNSID: u16 = 88u16;
```

### `RTM_DELNSID`

```rust
const RTM_DELNSID: u16 = 89u16;
```

### `RTM_GETNSID`

```rust
const RTM_GETNSID: u16 = 90u16;
```

### `RTM_F_NOTIFY`

```rust
const RTM_F_NOTIFY: crate::c_uint = 256u32;
```

### `RTM_F_CLONED`

```rust
const RTM_F_CLONED: crate::c_uint = 512u32;
```

### `RTM_F_EQUALIZE`

```rust
const RTM_F_EQUALIZE: crate::c_uint = 1_024u32;
```

### `RTM_F_PREFIX`

```rust
const RTM_F_PREFIX: crate::c_uint = 2_048u32;
```

### `RTA_UNSPEC`

```rust
const RTA_UNSPEC: crate::c_ushort = 0u16;
```

### `RTA_DST`

```rust
const RTA_DST: crate::c_ushort = 1u16;
```

### `RTA_SRC`

```rust
const RTA_SRC: crate::c_ushort = 2u16;
```

### `RTA_IIF`

```rust
const RTA_IIF: crate::c_ushort = 3u16;
```

### `RTA_OIF`

```rust
const RTA_OIF: crate::c_ushort = 4u16;
```

### `RTA_GATEWAY`

```rust
const RTA_GATEWAY: crate::c_ushort = 5u16;
```

### `RTA_PRIORITY`

```rust
const RTA_PRIORITY: crate::c_ushort = 6u16;
```

### `RTA_PREFSRC`

```rust
const RTA_PREFSRC: crate::c_ushort = 7u16;
```

### `RTA_METRICS`

```rust
const RTA_METRICS: crate::c_ushort = 8u16;
```

### `RTA_MULTIPATH`

```rust
const RTA_MULTIPATH: crate::c_ushort = 9u16;
```

### `RTA_PROTOINFO`

```rust
const RTA_PROTOINFO: crate::c_ushort = 10u16;
```

### `RTA_FLOW`

```rust
const RTA_FLOW: crate::c_ushort = 11u16;
```

### `RTA_CACHEINFO`

```rust
const RTA_CACHEINFO: crate::c_ushort = 12u16;
```

### `RTA_SESSION`

```rust
const RTA_SESSION: crate::c_ushort = 13u16;
```

### `RTA_MP_ALGO`

```rust
const RTA_MP_ALGO: crate::c_ushort = 14u16;
```

### `RTA_TABLE`

```rust
const RTA_TABLE: crate::c_ushort = 15u16;
```

### `RTA_MARK`

```rust
const RTA_MARK: crate::c_ushort = 16u16;
```

### `RTA_MFC_STATS`

```rust
const RTA_MFC_STATS: crate::c_ushort = 17u16;
```

### `RTN_UNSPEC`

```rust
const RTN_UNSPEC: crate::c_uchar = 0u8;
```

### `RTN_UNICAST`

```rust
const RTN_UNICAST: crate::c_uchar = 1u8;
```

### `RTN_LOCAL`

```rust
const RTN_LOCAL: crate::c_uchar = 2u8;
```

### `RTN_BROADCAST`

```rust
const RTN_BROADCAST: crate::c_uchar = 3u8;
```

### `RTN_ANYCAST`

```rust
const RTN_ANYCAST: crate::c_uchar = 4u8;
```

### `RTN_MULTICAST`

```rust
const RTN_MULTICAST: crate::c_uchar = 5u8;
```

### `RTN_BLACKHOLE`

```rust
const RTN_BLACKHOLE: crate::c_uchar = 6u8;
```

### `RTN_UNREACHABLE`

```rust
const RTN_UNREACHABLE: crate::c_uchar = 7u8;
```

### `RTN_PROHIBIT`

```rust
const RTN_PROHIBIT: crate::c_uchar = 8u8;
```

### `RTN_THROW`

```rust
const RTN_THROW: crate::c_uchar = 9u8;
```

### `RTN_NAT`

```rust
const RTN_NAT: crate::c_uchar = 10u8;
```

### `RTN_XRESOLVE`

```rust
const RTN_XRESOLVE: crate::c_uchar = 11u8;
```

### `RTPROT_UNSPEC`

```rust
const RTPROT_UNSPEC: crate::c_uchar = 0u8;
```

### `RTPROT_REDIRECT`

```rust
const RTPROT_REDIRECT: crate::c_uchar = 1u8;
```

### `RTPROT_KERNEL`

```rust
const RTPROT_KERNEL: crate::c_uchar = 2u8;
```

### `RTPROT_BOOT`

```rust
const RTPROT_BOOT: crate::c_uchar = 3u8;
```

### `RTPROT_STATIC`

```rust
const RTPROT_STATIC: crate::c_uchar = 4u8;
```

### `RT_SCOPE_UNIVERSE`

```rust
const RT_SCOPE_UNIVERSE: crate::c_uchar = 0u8;
```

### `RT_SCOPE_SITE`

```rust
const RT_SCOPE_SITE: crate::c_uchar = 200u8;
```

### `RT_SCOPE_LINK`

```rust
const RT_SCOPE_LINK: crate::c_uchar = 253u8;
```

### `RT_SCOPE_HOST`

```rust
const RT_SCOPE_HOST: crate::c_uchar = 254u8;
```

### `RT_SCOPE_NOWHERE`

```rust
const RT_SCOPE_NOWHERE: crate::c_uchar = 255u8;
```

### `RT_TABLE_UNSPEC`

```rust
const RT_TABLE_UNSPEC: crate::c_uchar = 0u8;
```

### `RT_TABLE_COMPAT`

```rust
const RT_TABLE_COMPAT: crate::c_uchar = 252u8;
```

### `RT_TABLE_DEFAULT`

```rust
const RT_TABLE_DEFAULT: crate::c_uchar = 253u8;
```

### `RT_TABLE_MAIN`

```rust
const RT_TABLE_MAIN: crate::c_uchar = 254u8;
```

### `RT_TABLE_LOCAL`

```rust
const RT_TABLE_LOCAL: crate::c_uchar = 255u8;
```

### `RTMSG_OVERRUN`

```rust
const RTMSG_OVERRUN: u32 = 4u32;
```

### `RTMSG_NEWDEVICE`

```rust
const RTMSG_NEWDEVICE: u32 = 17u32;
```

### `RTMSG_DELDEVICE`

```rust
const RTMSG_DELDEVICE: u32 = 18u32;
```

### `RTMSG_NEWROUTE`

```rust
const RTMSG_NEWROUTE: u32 = 33u32;
```

### `RTMSG_DELROUTE`

```rust
const RTMSG_DELROUTE: u32 = 34u32;
```

### `RTMSG_NEWRULE`

```rust
const RTMSG_NEWRULE: u32 = 49u32;
```

### `RTMSG_DELRULE`

```rust
const RTMSG_DELRULE: u32 = 50u32;
```

### `RTMSG_CONTROL`

```rust
const RTMSG_CONTROL: u32 = 64u32;
```

### `RTMSG_AR_FAILED`

```rust
const RTMSG_AR_FAILED: u32 = 81u32;
```

### `MAX_ADDR_LEN`

```rust
const MAX_ADDR_LEN: usize = 7usize;
```

### `ARPD_UPDATE`

```rust
const ARPD_UPDATE: crate::c_ushort = 1u16;
```

### `ARPD_LOOKUP`

```rust
const ARPD_LOOKUP: crate::c_ushort = 2u16;
```

### `ARPD_FLUSH`

```rust
const ARPD_FLUSH: crate::c_ushort = 3u16;
```

### `ATF_MAGIC`

```rust
const ATF_MAGIC: crate::c_int = 128i32;
```

### `RTEXT_FILTER_VF`

```rust
const RTEXT_FILTER_VF: crate::c_int = 1i32;
```

### `RTEXT_FILTER_BRVLAN`

```rust
const RTEXT_FILTER_BRVLAN: crate::c_int = 2i32;
```

### `RTEXT_FILTER_BRVLAN_COMPRESSED`

```rust
const RTEXT_FILTER_BRVLAN_COMPRESSED: crate::c_int = 4i32;
```

### `RTEXT_FILTER_SKIP_STATS`

```rust
const RTEXT_FILTER_SKIP_STATS: crate::c_int = 8i32;
```

### `RTEXT_FILTER_MRP`

```rust
const RTEXT_FILTER_MRP: crate::c_int = 16i32;
```

### `RTEXT_FILTER_CFM_CONFIG`

```rust
const RTEXT_FILTER_CFM_CONFIG: crate::c_int = 32i32;
```

### `RTEXT_FILTER_CFM_STATUS`

```rust
const RTEXT_FILTER_CFM_STATUS: crate::c_int = 64i32;
```

### `RTMGRP_LINK`

```rust
const RTMGRP_LINK: crate::c_int = 1i32;
```

### `RTMGRP_NOTIFY`

```rust
const RTMGRP_NOTIFY: crate::c_int = 2i32;
```

### `RTMGRP_NEIGH`

```rust
const RTMGRP_NEIGH: crate::c_int = 4i32;
```

### `RTMGRP_TC`

```rust
const RTMGRP_TC: crate::c_int = 8i32;
```

### `RTMGRP_IPV4_IFADDR`

```rust
const RTMGRP_IPV4_IFADDR: crate::c_int = 16i32;
```

### `RTMGRP_IPV4_MROUTE`

```rust
const RTMGRP_IPV4_MROUTE: crate::c_int = 32i32;
```

### `RTMGRP_IPV4_ROUTE`

```rust
const RTMGRP_IPV4_ROUTE: crate::c_int = 64i32;
```

### `RTMGRP_IPV4_RULE`

```rust
const RTMGRP_IPV4_RULE: crate::c_int = 128i32;
```

### `RTMGRP_IPV6_IFADDR`

```rust
const RTMGRP_IPV6_IFADDR: crate::c_int = 256i32;
```

### `RTMGRP_IPV6_MROUTE`

```rust
const RTMGRP_IPV6_MROUTE: crate::c_int = 512i32;
```

### `RTMGRP_IPV6_ROUTE`

```rust
const RTMGRP_IPV6_ROUTE: crate::c_int = 1_024i32;
```

### `RTMGRP_IPV6_IFINFO`

```rust
const RTMGRP_IPV6_IFINFO: crate::c_int = 2_048i32;
```

### `RTMGRP_DECnet_IFADDR`

```rust
const RTMGRP_DECnet_IFADDR: crate::c_int = 4_096i32;
```

### `RTMGRP_DECnet_ROUTE`

```rust
const RTMGRP_DECnet_ROUTE: crate::c_int = 16_384i32;
```

### `RTMGRP_IPV6_PREFIX`

```rust
const RTMGRP_IPV6_PREFIX: crate::c_int = 131_072i32;
```

### `RTNLGRP_NONE`

```rust
const RTNLGRP_NONE: crate::c_uint = 0u32;
```

### `RTNLGRP_LINK`

```rust
const RTNLGRP_LINK: crate::c_uint = 1u32;
```

### `RTNLGRP_NOTIFY`

```rust
const RTNLGRP_NOTIFY: crate::c_uint = 2u32;
```

### `RTNLGRP_NEIGH`

```rust
const RTNLGRP_NEIGH: crate::c_uint = 3u32;
```

### `RTNLGRP_TC`

```rust
const RTNLGRP_TC: crate::c_uint = 4u32;
```

### `RTNLGRP_IPV4_IFADDR`

```rust
const RTNLGRP_IPV4_IFADDR: crate::c_uint = 5u32;
```

### `RTNLGRP_IPV4_MROUTE`

```rust
const RTNLGRP_IPV4_MROUTE: crate::c_uint = 6u32;
```

### `RTNLGRP_IPV4_ROUTE`

```rust
const RTNLGRP_IPV4_ROUTE: crate::c_uint = 7u32;
```

### `RTNLGRP_IPV4_RULE`

```rust
const RTNLGRP_IPV4_RULE: crate::c_uint = 8u32;
```

### `RTNLGRP_IPV6_IFADDR`

```rust
const RTNLGRP_IPV6_IFADDR: crate::c_uint = 9u32;
```

### `RTNLGRP_IPV6_MROUTE`

```rust
const RTNLGRP_IPV6_MROUTE: crate::c_uint = 10u32;
```

### `RTNLGRP_IPV6_ROUTE`

```rust
const RTNLGRP_IPV6_ROUTE: crate::c_uint = 11u32;
```

### `RTNLGRP_IPV6_IFINFO`

```rust
const RTNLGRP_IPV6_IFINFO: crate::c_uint = 12u32;
```

### `RTNLGRP_DECnet_IFADDR`

```rust
const RTNLGRP_DECnet_IFADDR: crate::c_uint = 13u32;
```

### `RTNLGRP_NOP2`

```rust
const RTNLGRP_NOP2: crate::c_uint = 14u32;
```

### `RTNLGRP_DECnet_ROUTE`

```rust
const RTNLGRP_DECnet_ROUTE: crate::c_uint = 15u32;
```

### `RTNLGRP_DECnet_RULE`

```rust
const RTNLGRP_DECnet_RULE: crate::c_uint = 16u32;
```

### `RTNLGRP_NOP4`

```rust
const RTNLGRP_NOP4: crate::c_uint = 17u32;
```

### `RTNLGRP_IPV6_PREFIX`

```rust
const RTNLGRP_IPV6_PREFIX: crate::c_uint = 18u32;
```

### `RTNLGRP_IPV6_RULE`

```rust
const RTNLGRP_IPV6_RULE: crate::c_uint = 19u32;
```

### `RTNLGRP_ND_USEROPT`

```rust
const RTNLGRP_ND_USEROPT: crate::c_uint = 20u32;
```

### `RTNLGRP_PHONET_IFADDR`

```rust
const RTNLGRP_PHONET_IFADDR: crate::c_uint = 21u32;
```

### `RTNLGRP_PHONET_ROUTE`

```rust
const RTNLGRP_PHONET_ROUTE: crate::c_uint = 22u32;
```

### `RTNLGRP_DCB`

```rust
const RTNLGRP_DCB: crate::c_uint = 23u32;
```

### `RTNLGRP_IPV4_NETCONF`

```rust
const RTNLGRP_IPV4_NETCONF: crate::c_uint = 24u32;
```

### `RTNLGRP_IPV6_NETCONF`

```rust
const RTNLGRP_IPV6_NETCONF: crate::c_uint = 25u32;
```

### `RTNLGRP_MDB`

```rust
const RTNLGRP_MDB: crate::c_uint = 26u32;
```

### `RTNLGRP_MPLS_ROUTE`

```rust
const RTNLGRP_MPLS_ROUTE: crate::c_uint = 27u32;
```

### `RTNLGRP_NSID`

```rust
const RTNLGRP_NSID: crate::c_uint = 28u32;
```

### `RTNLGRP_MPLS_NETCONF`

```rust
const RTNLGRP_MPLS_NETCONF: crate::c_uint = 29u32;
```

### `RTNLGRP_IPV4_MROUTE_R`

```rust
const RTNLGRP_IPV4_MROUTE_R: crate::c_uint = 30u32;
```

### `RTNLGRP_IPV6_MROUTE_R`

```rust
const RTNLGRP_IPV6_MROUTE_R: crate::c_uint = 31u32;
```

### `RTNLGRP_NEXTHOP`

```rust
const RTNLGRP_NEXTHOP: crate::c_uint = 32u32;
```

### `RTNLGRP_BRVLAN`

```rust
const RTNLGRP_BRVLAN: crate::c_uint = 33u32;
```

### `RTNLGRP_MCTP_IFADDR`

```rust
const RTNLGRP_MCTP_IFADDR: crate::c_uint = 34u32;
```

### `RTNLGRP_TUNNEL`

```rust
const RTNLGRP_TUNNEL: crate::c_uint = 35u32;
```

### `RTNLGRP_STATS`

```rust
const RTNLGRP_STATS: crate::c_uint = 36u32;
```

### `PROC_CN_MCAST_LISTEN`

```rust
const PROC_CN_MCAST_LISTEN: proc_cn_mcast_op = 1u32;
```

### `PROC_CN_MCAST_IGNORE`

```rust
const PROC_CN_MCAST_IGNORE: proc_cn_mcast_op = 2u32;
```

### `PROC_EVENT_NONE`

```rust
const PROC_EVENT_NONE: proc_cn_event = 0u32;
```

### `PROC_EVENT_FORK`

```rust
const PROC_EVENT_FORK: proc_cn_event = 1u32;
```

### `PROC_EVENT_EXEC`

```rust
const PROC_EVENT_EXEC: proc_cn_event = 2u32;
```

### `PROC_EVENT_UID`

```rust
const PROC_EVENT_UID: proc_cn_event = 4u32;
```

### `PROC_EVENT_GID`

```rust
const PROC_EVENT_GID: proc_cn_event = 64u32;
```

### `PROC_EVENT_SID`

```rust
const PROC_EVENT_SID: proc_cn_event = 128u32;
```

### `PROC_EVENT_PTRACE`

```rust
const PROC_EVENT_PTRACE: proc_cn_event = 256u32;
```

### `PROC_EVENT_COMM`

```rust
const PROC_EVENT_COMM: proc_cn_event = 512u32;
```

### `PROC_EVENT_NONZERO_EXIT`

```rust
const PROC_EVENT_NONZERO_EXIT: proc_cn_event = 536_870_912u32;
```

### `PROC_EVENT_COREDUMP`

```rust
const PROC_EVENT_COREDUMP: proc_cn_event = 1_073_741_824u32;
```

### `PROC_EVENT_EXIT`

```rust
const PROC_EVENT_EXIT: proc_cn_event = 2_147_483_648u32;
```

### `CN_IDX_PROC`

```rust
const CN_IDX_PROC: crate::c_uint = 1u32;
```

### `CN_VAL_PROC`

```rust
const CN_VAL_PROC: crate::c_uint = 1u32;
```

### `CN_IDX_CIFS`

```rust
const CN_IDX_CIFS: crate::c_uint = 2u32;
```

### `CN_VAL_CIFS`

```rust
const CN_VAL_CIFS: crate::c_uint = 1u32;
```

### `CN_W1_IDX`

```rust
const CN_W1_IDX: crate::c_uint = 3u32;
```

### `CN_W1_VAL`

```rust
const CN_W1_VAL: crate::c_uint = 1u32;
```

### `CN_IDX_V86D`

```rust
const CN_IDX_V86D: crate::c_uint = 4u32;
```

### `CN_VAL_V86D_UVESAFB`

```rust
const CN_VAL_V86D_UVESAFB: crate::c_uint = 1u32;
```

### `CN_IDX_BB`

```rust
const CN_IDX_BB: crate::c_uint = 5u32;
```

### `CN_DST_IDX`

```rust
const CN_DST_IDX: crate::c_uint = 6u32;
```

### `CN_DST_VAL`

```rust
const CN_DST_VAL: crate::c_uint = 1u32;
```

### `CN_IDX_DM`

```rust
const CN_IDX_DM: crate::c_uint = 7u32;
```

### `CN_VAL_DM_USERSPACE_LOG`

```rust
const CN_VAL_DM_USERSPACE_LOG: crate::c_uint = 1u32;
```

### `CN_IDX_DRBD`

```rust
const CN_IDX_DRBD: crate::c_uint = 8u32;
```

### `CN_VAL_DRBD`

```rust
const CN_VAL_DRBD: crate::c_uint = 1u32;
```

### `CN_KVP_IDX`

```rust
const CN_KVP_IDX: crate::c_uint = 9u32;
```

### `CN_KVP_VAL`

```rust
const CN_KVP_VAL: crate::c_uint = 1u32;
```

### `CN_VSS_IDX`

```rust
const CN_VSS_IDX: crate::c_uint = 10u32;
```

### `CN_VSS_VAL`

```rust
const CN_VSS_VAL: crate::c_uint = 1u32;
```

### `MODULE_INIT_IGNORE_MODVERSIONS`

```rust
const MODULE_INIT_IGNORE_MODVERSIONS: crate::c_uint = 1u32;
```

### `MODULE_INIT_IGNORE_VERMAGIC`

```rust
const MODULE_INIT_IGNORE_VERMAGIC: crate::c_uint = 2u32;
```

### `SOF_TIMESTAMPING_TX_HARDWARE`

```rust
const SOF_TIMESTAMPING_TX_HARDWARE: crate::c_uint = 1u32;
```

### `SOF_TIMESTAMPING_TX_SOFTWARE`

```rust
const SOF_TIMESTAMPING_TX_SOFTWARE: crate::c_uint = 2u32;
```

### `SOF_TIMESTAMPING_RX_HARDWARE`

```rust
const SOF_TIMESTAMPING_RX_HARDWARE: crate::c_uint = 4u32;
```

### `SOF_TIMESTAMPING_RX_SOFTWARE`

```rust
const SOF_TIMESTAMPING_RX_SOFTWARE: crate::c_uint = 8u32;
```

### `SOF_TIMESTAMPING_SOFTWARE`

```rust
const SOF_TIMESTAMPING_SOFTWARE: crate::c_uint = 16u32;
```

### `SOF_TIMESTAMPING_SYS_HARDWARE`

```rust
const SOF_TIMESTAMPING_SYS_HARDWARE: crate::c_uint = 32u32;
```

### `SOF_TIMESTAMPING_RAW_HARDWARE`

```rust
const SOF_TIMESTAMPING_RAW_HARDWARE: crate::c_uint = 64u32;
```

### `SOF_TIMESTAMPING_OPT_ID`

```rust
const SOF_TIMESTAMPING_OPT_ID: crate::c_uint = 128u32;
```

### `SOF_TIMESTAMPING_TX_SCHED`

```rust
const SOF_TIMESTAMPING_TX_SCHED: crate::c_uint = 256u32;
```

### `SOF_TIMESTAMPING_TX_ACK`

```rust
const SOF_TIMESTAMPING_TX_ACK: crate::c_uint = 512u32;
```

### `SOF_TIMESTAMPING_OPT_CMSG`

```rust
const SOF_TIMESTAMPING_OPT_CMSG: crate::c_uint = 1_024u32;
```

### `SOF_TIMESTAMPING_OPT_TSONLY`

```rust
const SOF_TIMESTAMPING_OPT_TSONLY: crate::c_uint = 2_048u32;
```

### `SOF_TIMESTAMPING_OPT_STATS`

```rust
const SOF_TIMESTAMPING_OPT_STATS: crate::c_uint = 4_096u32;
```

### `SOF_TIMESTAMPING_OPT_PKTINFO`

```rust
const SOF_TIMESTAMPING_OPT_PKTINFO: crate::c_uint = 8_192u32;
```

### `SOF_TIMESTAMPING_OPT_TX_SWHW`

```rust
const SOF_TIMESTAMPING_OPT_TX_SWHW: crate::c_uint = 16_384u32;
```

### `SOF_TIMESTAMPING_BIND_PHC`

```rust
const SOF_TIMESTAMPING_BIND_PHC: crate::c_uint = 32_768u32;
```

### `SOF_TIMESTAMPING_OPT_ID_TCP`

```rust
const SOF_TIMESTAMPING_OPT_ID_TCP: crate::c_uint = 65_536u32;
```

### `SOF_TIMESTAMPING_OPT_RX_FILTER`

```rust
const SOF_TIMESTAMPING_OPT_RX_FILTER: crate::c_uint = 131_072u32;
```

### `SOF_TXTIME_DEADLINE_MODE`

```rust
const SOF_TXTIME_DEADLINE_MODE: u32 = 1u32;
```

### `SOF_TXTIME_REPORT_ERRORS`

```rust
const SOF_TXTIME_REPORT_ERRORS: u32 = 2u32;
```

### `HWTSTAMP_TX_OFF`

```rust
const HWTSTAMP_TX_OFF: crate::c_uint = 0u32;
```

### `HWTSTAMP_TX_ON`

```rust
const HWTSTAMP_TX_ON: crate::c_uint = 1u32;
```

### `HWTSTAMP_TX_ONESTEP_SYNC`

```rust
const HWTSTAMP_TX_ONESTEP_SYNC: crate::c_uint = 2u32;
```

### `HWTSTAMP_TX_ONESTEP_P2P`

```rust
const HWTSTAMP_TX_ONESTEP_P2P: crate::c_uint = 3u32;
```

### `HWTSTAMP_FILTER_NONE`

```rust
const HWTSTAMP_FILTER_NONE: crate::c_uint = 0u32;
```

### `HWTSTAMP_FILTER_ALL`

```rust
const HWTSTAMP_FILTER_ALL: crate::c_uint = 1u32;
```

### `HWTSTAMP_FILTER_SOME`

```rust
const HWTSTAMP_FILTER_SOME: crate::c_uint = 2u32;
```

### `HWTSTAMP_FILTER_PTP_V1_L4_EVENT`

```rust
const HWTSTAMP_FILTER_PTP_V1_L4_EVENT: crate::c_uint = 3u32;
```

### `HWTSTAMP_FILTER_PTP_V1_L4_SYNC`

```rust
const HWTSTAMP_FILTER_PTP_V1_L4_SYNC: crate::c_uint = 4u32;
```

### `HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ`

```rust
const HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ: crate::c_uint = 5u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L4_EVENT`

```rust
const HWTSTAMP_FILTER_PTP_V2_L4_EVENT: crate::c_uint = 6u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L4_SYNC`

```rust
const HWTSTAMP_FILTER_PTP_V2_L4_SYNC: crate::c_uint = 7u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ`

```rust
const HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ: crate::c_uint = 8u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L2_EVENT`

```rust
const HWTSTAMP_FILTER_PTP_V2_L2_EVENT: crate::c_uint = 9u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L2_SYNC`

```rust
const HWTSTAMP_FILTER_PTP_V2_L2_SYNC: crate::c_uint = 10u32;
```

### `HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ`

```rust
const HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ: crate::c_uint = 11u32;
```

### `HWTSTAMP_FILTER_PTP_V2_EVENT`

```rust
const HWTSTAMP_FILTER_PTP_V2_EVENT: crate::c_uint = 12u32;
```

### `HWTSTAMP_FILTER_PTP_V2_SYNC`

```rust
const HWTSTAMP_FILTER_PTP_V2_SYNC: crate::c_uint = 13u32;
```

### `HWTSTAMP_FILTER_PTP_V2_DELAY_REQ`

```rust
const HWTSTAMP_FILTER_PTP_V2_DELAY_REQ: crate::c_uint = 14u32;
```

### `HWTSTAMP_FILTER_NTP_ALL`

```rust
const HWTSTAMP_FILTER_NTP_ALL: crate::c_uint = 15u32;
```

### `PTP_MAX_SAMPLES`

```rust
const PTP_MAX_SAMPLES: crate::c_uint = 25u32;
```

### `PTP_CLK_MAGIC`

```rust
const PTP_CLK_MAGIC: u32 = 61u32;
```

### `PTP_CLOCK_GETCAPS`

```rust
const PTP_CLOCK_GETCAPS: crate::c_ulong = 2_152_742_145u64;
```

### `PTP_EXTTS_REQUEST`

```rust
const PTP_EXTTS_REQUEST: crate::c_ulong = 1_074_806_018u64;
```

### `PTP_PEROUT_REQUEST`

```rust
const PTP_PEROUT_REQUEST: crate::c_ulong = 1_077_427_459u64;
```

### `PTP_ENABLE_PPS`

```rust
const PTP_ENABLE_PPS: crate::c_ulong = 1_074_019_588u64;
```

### `PTP_SYS_OFFSET`

```rust
const PTP_SYS_OFFSET: crate::c_ulong = 1_128_283_397u64;
```

### `PTP_PIN_GETFUNC`

```rust
const PTP_PIN_GETFUNC: crate::c_ulong = 3_227_532_550u64;
```

### `PTP_PIN_SETFUNC`

```rust
const PTP_PIN_SETFUNC: crate::c_ulong = 1_080_048_903u64;
```

### `PTP_SYS_OFFSET_PRECISE`

```rust
const PTP_SYS_OFFSET_PRECISE: crate::c_ulong = 3_225_435_400u64;
```

### `PTP_SYS_OFFSET_EXTENDED`

```rust
const PTP_SYS_OFFSET_EXTENDED: crate::c_ulong = 3_300_932_873u64;
```

### `PTP_CLOCK_GETCAPS2`

```rust
const PTP_CLOCK_GETCAPS2: crate::c_ulong = 2_152_742_154u64;
```

### `PTP_EXTTS_REQUEST2`

```rust
const PTP_EXTTS_REQUEST2: crate::c_ulong = 1_074_806_027u64;
```

### `PTP_PEROUT_REQUEST2`

```rust
const PTP_PEROUT_REQUEST2: crate::c_ulong = 1_077_427_468u64;
```

### `PTP_ENABLE_PPS2`

```rust
const PTP_ENABLE_PPS2: crate::c_ulong = 1_074_019_597u64;
```

### `PTP_SYS_OFFSET2`

```rust
const PTP_SYS_OFFSET2: crate::c_ulong = 1_128_283_406u64;
```

### `PTP_PIN_GETFUNC2`

```rust
const PTP_PIN_GETFUNC2: crate::c_ulong = 3_227_532_559u64;
```

### `PTP_PIN_SETFUNC2`

```rust
const PTP_PIN_SETFUNC2: crate::c_ulong = 1_080_048_912u64;
```

### `PTP_SYS_OFFSET_PRECISE2`

```rust
const PTP_SYS_OFFSET_PRECISE2: crate::c_ulong = 3_225_435_409u64;
```

### `PTP_SYS_OFFSET_EXTENDED2`

```rust
const PTP_SYS_OFFSET_EXTENDED2: crate::c_ulong = 3_300_932_882u64;
```

### `PTP_PF_NONE`

```rust
const PTP_PF_NONE: crate::c_uint = 0u32;
```

### `PTP_PF_EXTTS`

```rust
const PTP_PF_EXTTS: crate::c_uint = 1u32;
```

### `PTP_PF_PEROUT`

```rust
const PTP_PF_PEROUT: crate::c_uint = 2u32;
```

### `PTP_PF_PHYSYNC`

```rust
const PTP_PF_PHYSYNC: crate::c_uint = 3u32;
```

### `TLS_TX`

```rust
const TLS_TX: crate::c_int = 1i32;
```

### `TLS_RX`

```rust
const TLS_RX: crate::c_int = 2i32;
```

### `TLS_TX_ZEROCOPY_RO`

```rust
const TLS_TX_ZEROCOPY_RO: crate::c_int = 3i32;
```

### `TLS_RX_EXPECT_NO_PAD`

```rust
const TLS_RX_EXPECT_NO_PAD: crate::c_int = 4i32;
```

### `TLS_1_2_VERSION_MAJOR`

```rust
const TLS_1_2_VERSION_MAJOR: __u8 = 3u8;
```

### `TLS_1_2_VERSION_MINOR`

```rust
const TLS_1_2_VERSION_MINOR: __u8 = 3u8;
```

### `TLS_1_2_VERSION`

```rust
const TLS_1_2_VERSION: __u16 = 771u16;
```

### `TLS_1_3_VERSION_MAJOR`

```rust
const TLS_1_3_VERSION_MAJOR: __u8 = 3u8;
```

### `TLS_1_3_VERSION_MINOR`

```rust
const TLS_1_3_VERSION_MINOR: __u8 = 4u8;
```

### `TLS_1_3_VERSION`

```rust
const TLS_1_3_VERSION: __u16 = 772u16;
```

### `TLS_CIPHER_AES_GCM_128`

```rust
const TLS_CIPHER_AES_GCM_128: __u16 = 51u16;
```

### `TLS_CIPHER_AES_GCM_128_IV_SIZE`

```rust
const TLS_CIPHER_AES_GCM_128_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_AES_GCM_128_KEY_SIZE`

```rust
const TLS_CIPHER_AES_GCM_128_KEY_SIZE: usize = 16usize;
```

### `TLS_CIPHER_AES_GCM_128_SALT_SIZE`

```rust
const TLS_CIPHER_AES_GCM_128_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_AES_GCM_128_TAG_SIZE`

```rust
const TLS_CIPHER_AES_GCM_128_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE`

```rust
const TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_AES_GCM_256`

```rust
const TLS_CIPHER_AES_GCM_256: __u16 = 52u16;
```

### `TLS_CIPHER_AES_GCM_256_IV_SIZE`

```rust
const TLS_CIPHER_AES_GCM_256_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_AES_GCM_256_KEY_SIZE`

```rust
const TLS_CIPHER_AES_GCM_256_KEY_SIZE: usize = 32usize;
```

### `TLS_CIPHER_AES_GCM_256_SALT_SIZE`

```rust
const TLS_CIPHER_AES_GCM_256_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_AES_GCM_256_TAG_SIZE`

```rust
const TLS_CIPHER_AES_GCM_256_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE`

```rust
const TLS_CIPHER_AES_GCM_256_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_AES_CCM_128`

```rust
const TLS_CIPHER_AES_CCM_128: __u16 = 53u16;
```

### `TLS_CIPHER_AES_CCM_128_IV_SIZE`

```rust
const TLS_CIPHER_AES_CCM_128_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_AES_CCM_128_KEY_SIZE`

```rust
const TLS_CIPHER_AES_CCM_128_KEY_SIZE: usize = 16usize;
```

### `TLS_CIPHER_AES_CCM_128_SALT_SIZE`

```rust
const TLS_CIPHER_AES_CCM_128_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_AES_CCM_128_TAG_SIZE`

```rust
const TLS_CIPHER_AES_CCM_128_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE`

```rust
const TLS_CIPHER_AES_CCM_128_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_CHACHA20_POLY1305`

```rust
const TLS_CIPHER_CHACHA20_POLY1305: __u16 = 54u16;
```

### `TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE`

```rust
const TLS_CIPHER_CHACHA20_POLY1305_IV_SIZE: usize = 12usize;
```

### `TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE`

```rust
const TLS_CIPHER_CHACHA20_POLY1305_KEY_SIZE: usize = 32usize;
```

### `TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE`

```rust
const TLS_CIPHER_CHACHA20_POLY1305_SALT_SIZE: usize = 0usize;
```

### `TLS_CIPHER_CHACHA20_POLY1305_TAG_SIZE`

```rust
const TLS_CIPHER_CHACHA20_POLY1305_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE`

```rust
const TLS_CIPHER_CHACHA20_POLY1305_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_SM4_GCM`

```rust
const TLS_CIPHER_SM4_GCM: __u16 = 55u16;
```

### `TLS_CIPHER_SM4_GCM_IV_SIZE`

```rust
const TLS_CIPHER_SM4_GCM_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_SM4_GCM_KEY_SIZE`

```rust
const TLS_CIPHER_SM4_GCM_KEY_SIZE: usize = 16usize;
```

### `TLS_CIPHER_SM4_GCM_SALT_SIZE`

```rust
const TLS_CIPHER_SM4_GCM_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_SM4_GCM_TAG_SIZE`

```rust
const TLS_CIPHER_SM4_GCM_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE`

```rust
const TLS_CIPHER_SM4_GCM_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_SM4_CCM`

```rust
const TLS_CIPHER_SM4_CCM: __u16 = 56u16;
```

### `TLS_CIPHER_SM4_CCM_IV_SIZE`

```rust
const TLS_CIPHER_SM4_CCM_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_SM4_CCM_KEY_SIZE`

```rust
const TLS_CIPHER_SM4_CCM_KEY_SIZE: usize = 16usize;
```

### `TLS_CIPHER_SM4_CCM_SALT_SIZE`

```rust
const TLS_CIPHER_SM4_CCM_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_SM4_CCM_TAG_SIZE`

```rust
const TLS_CIPHER_SM4_CCM_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE`

```rust
const TLS_CIPHER_SM4_CCM_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_ARIA_GCM_128`

```rust
const TLS_CIPHER_ARIA_GCM_128: __u16 = 57u16;
```

### `TLS_CIPHER_ARIA_GCM_128_IV_SIZE`

```rust
const TLS_CIPHER_ARIA_GCM_128_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_ARIA_GCM_128_KEY_SIZE`

```rust
const TLS_CIPHER_ARIA_GCM_128_KEY_SIZE: usize = 16usize;
```

### `TLS_CIPHER_ARIA_GCM_128_SALT_SIZE`

```rust
const TLS_CIPHER_ARIA_GCM_128_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_ARIA_GCM_128_TAG_SIZE`

```rust
const TLS_CIPHER_ARIA_GCM_128_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_ARIA_GCM_128_REC_SEQ_SIZE`

```rust
const TLS_CIPHER_ARIA_GCM_128_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_CIPHER_ARIA_GCM_256`

```rust
const TLS_CIPHER_ARIA_GCM_256: __u16 = 58u16;
```

### `TLS_CIPHER_ARIA_GCM_256_IV_SIZE`

```rust
const TLS_CIPHER_ARIA_GCM_256_IV_SIZE: usize = 8usize;
```

### `TLS_CIPHER_ARIA_GCM_256_KEY_SIZE`

```rust
const TLS_CIPHER_ARIA_GCM_256_KEY_SIZE: usize = 32usize;
```

### `TLS_CIPHER_ARIA_GCM_256_SALT_SIZE`

```rust
const TLS_CIPHER_ARIA_GCM_256_SALT_SIZE: usize = 4usize;
```

### `TLS_CIPHER_ARIA_GCM_256_TAG_SIZE`

```rust
const TLS_CIPHER_ARIA_GCM_256_TAG_SIZE: usize = 16usize;
```

### `TLS_CIPHER_ARIA_GCM_256_REC_SEQ_SIZE`

```rust
const TLS_CIPHER_ARIA_GCM_256_REC_SEQ_SIZE: usize = 8usize;
```

### `TLS_SET_RECORD_TYPE`

```rust
const TLS_SET_RECORD_TYPE: crate::c_int = 1i32;
```

### `TLS_GET_RECORD_TYPE`

```rust
const TLS_GET_RECORD_TYPE: crate::c_int = 2i32;
```

### `SOL_TLS`

```rust
const SOL_TLS: crate::c_int = 282i32;
```

### `TLS_INFO_UNSPEC`

```rust
const TLS_INFO_UNSPEC: crate::c_int = 0i32;
```

### `TLS_INFO_VERSION`

```rust
const TLS_INFO_VERSION: crate::c_int = 1i32;
```

### `TLS_INFO_CIPHER`

```rust
const TLS_INFO_CIPHER: crate::c_int = 2i32;
```

### `TLS_INFO_TXCONF`

```rust
const TLS_INFO_TXCONF: crate::c_int = 3i32;
```

### `TLS_INFO_RXCONF`

```rust
const TLS_INFO_RXCONF: crate::c_int = 4i32;
```

### `TLS_INFO_ZC_RO_TX`

```rust
const TLS_INFO_ZC_RO_TX: crate::c_int = 5i32;
```

### `TLS_INFO_RX_NO_PAD`

```rust
const TLS_INFO_RX_NO_PAD: crate::c_int = 6i32;
```

### `TLS_CONF_BASE`

```rust
const TLS_CONF_BASE: crate::c_int = 1i32;
```

### `TLS_CONF_SW`

```rust
const TLS_CONF_SW: crate::c_int = 2i32;
```

### `TLS_CONF_HW`

```rust
const TLS_CONF_HW: crate::c_int = 3i32;
```

### `TLS_CONF_HW_RECORD`

```rust
const TLS_CONF_HW_RECORD: crate::c_int = 4i32;
```

### `ALG_SET_KEY`

```rust
const ALG_SET_KEY: crate::c_int = 1i32;
```

### `ALG_SET_IV`

```rust
const ALG_SET_IV: crate::c_int = 2i32;
```

### `ALG_SET_OP`

```rust
const ALG_SET_OP: crate::c_int = 3i32;
```

### `ALG_SET_AEAD_ASSOCLEN`

```rust
const ALG_SET_AEAD_ASSOCLEN: crate::c_int = 4i32;
```

### `ALG_SET_AEAD_AUTHSIZE`

```rust
const ALG_SET_AEAD_AUTHSIZE: crate::c_int = 5i32;
```

### `ALG_SET_DRBG_ENTROPY`

```rust
const ALG_SET_DRBG_ENTROPY: crate::c_int = 6i32;
```

### `ALG_SET_KEY_BY_KEY_SERIAL`

```rust
const ALG_SET_KEY_BY_KEY_SERIAL: crate::c_int = 7i32;
```

### `ALG_OP_DECRYPT`

```rust
const ALG_OP_DECRYPT: crate::c_int = 0i32;
```

### `ALG_OP_ENCRYPT`

```rust
const ALG_OP_ENCRYPT: crate::c_int = 1i32;
```

### `IF_OPER_UNKNOWN`

```rust
const IF_OPER_UNKNOWN: crate::c_int = 0i32;
```

### `IF_OPER_NOTPRESENT`

```rust
const IF_OPER_NOTPRESENT: crate::c_int = 1i32;
```

### `IF_OPER_DOWN`

```rust
const IF_OPER_DOWN: crate::c_int = 2i32;
```

### `IF_OPER_LOWERLAYERDOWN`

```rust
const IF_OPER_LOWERLAYERDOWN: crate::c_int = 3i32;
```

### `IF_OPER_TESTING`

```rust
const IF_OPER_TESTING: crate::c_int = 4i32;
```

### `IF_OPER_DORMANT`

```rust
const IF_OPER_DORMANT: crate::c_int = 5i32;
```

### `IF_OPER_UP`

```rust
const IF_OPER_UP: crate::c_int = 6i32;
```

### `IF_LINK_MODE_DEFAULT`

```rust
const IF_LINK_MODE_DEFAULT: crate::c_int = 0i32;
```

### `IF_LINK_MODE_DORMANT`

```rust
const IF_LINK_MODE_DORMANT: crate::c_int = 1i32;
```

### `IF_LINK_MODE_TESTING`

```rust
const IF_LINK_MODE_TESTING: crate::c_int = 2i32;
```

### `UDP_CORK`

```rust
const UDP_CORK: crate::c_int = 1i32;
```

### `UDP_ENCAP`

```rust
const UDP_ENCAP: crate::c_int = 100i32;
```

### `UDP_NO_CHECK6_TX`

```rust
const UDP_NO_CHECK6_TX: crate::c_int = 101i32;
```

### `UDP_NO_CHECK6_RX`

```rust
const UDP_NO_CHECK6_RX: crate::c_int = 102i32;
```

### `MAP_SHARED_VALIDATE`

```rust
const MAP_SHARED_VALIDATE: crate::c_int = 3i32;
```

### `MAP_DROPPABLE`

```rust
const MAP_DROPPABLE: crate::c_int = 8i32;
```

### `MAP_FIXED_NOREPLACE`

```rust
const MAP_FIXED_NOREPLACE: crate::c_int = 1_048_576i32;
```

### `MLOCK_ONFAULT`

```rust
const MLOCK_ONFAULT: crate::c_uint = 1u32;
```

### `VMADDR_CID_ANY`

```rust
const VMADDR_CID_ANY: crate::c_uint = 4_294_967_295u32;
```

### `VMADDR_CID_HYPERVISOR`

```rust
const VMADDR_CID_HYPERVISOR: crate::c_uint = 0u32;
```

### `VMADDR_CID_RESERVED`

```rust
const VMADDR_CID_RESERVED: crate::c_uint = 1u32;
```

### `VMADDR_CID_LOCAL`

```rust
const VMADDR_CID_LOCAL: crate::c_uint = 1u32;
```

### `VMADDR_CID_HOST`

```rust
const VMADDR_CID_HOST: crate::c_uint = 2u32;
```

### `VMADDR_PORT_ANY`

```rust
const VMADDR_PORT_ANY: crate::c_uint = 4_294_967_295u32;
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

### `IN_CLOSE`

```rust
const IN_CLOSE: u32 = 24u32;
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

### `IN_MOVE`

```rust
const IN_MOVE: u32 = 192u32;
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

### `SECURE_NOROOT`

```rust
const SECURE_NOROOT: crate::c_int = 0i32;
```

### `SECURE_NOROOT_LOCKED`

```rust
const SECURE_NOROOT_LOCKED: crate::c_int = 1i32;
```

### `SECBIT_NOROOT`

```rust
const SECBIT_NOROOT: crate::c_int = 1i32;
```

### `SECBIT_NOROOT_LOCKED`

```rust
const SECBIT_NOROOT_LOCKED: crate::c_int = 2i32;
```

### `SECURE_NO_SETUID_FIXUP`

```rust
const SECURE_NO_SETUID_FIXUP: crate::c_int = 2i32;
```

### `SECURE_NO_SETUID_FIXUP_LOCKED`

```rust
const SECURE_NO_SETUID_FIXUP_LOCKED: crate::c_int = 3i32;
```

### `SECBIT_NO_SETUID_FIXUP`

```rust
const SECBIT_NO_SETUID_FIXUP: crate::c_int = 4i32;
```

### `SECBIT_NO_SETUID_FIXUP_LOCKED`

```rust
const SECBIT_NO_SETUID_FIXUP_LOCKED: crate::c_int = 8i32;
```

### `SECURE_KEEP_CAPS`

```rust
const SECURE_KEEP_CAPS: crate::c_int = 4i32;
```

### `SECURE_KEEP_CAPS_LOCKED`

```rust
const SECURE_KEEP_CAPS_LOCKED: crate::c_int = 5i32;
```

### `SECBIT_KEEP_CAPS`

```rust
const SECBIT_KEEP_CAPS: crate::c_int = 16i32;
```

### `SECBIT_KEEP_CAPS_LOCKED`

```rust
const SECBIT_KEEP_CAPS_LOCKED: crate::c_int = 32i32;
```

### `SECURE_NO_CAP_AMBIENT_RAISE`

```rust
const SECURE_NO_CAP_AMBIENT_RAISE: crate::c_int = 6i32;
```

### `SECURE_NO_CAP_AMBIENT_RAISE_LOCKED`

```rust
const SECURE_NO_CAP_AMBIENT_RAISE_LOCKED: crate::c_int = 7i32;
```

### `SECBIT_NO_CAP_AMBIENT_RAISE`

```rust
const SECBIT_NO_CAP_AMBIENT_RAISE: crate::c_int = 64i32;
```

### `SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED`

```rust
const SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED: crate::c_int = 128i32;
```

### `SECURE_EXEC_RESTRICT_FILE`

```rust
const SECURE_EXEC_RESTRICT_FILE: crate::c_int = 8i32;
```

### `SECURE_EXEC_RESTRICT_FILE_LOCKED`

```rust
const SECURE_EXEC_RESTRICT_FILE_LOCKED: crate::c_int = 9i32;
```

### `SECBIT_EXEC_RESTRICT_FILE`

```rust
const SECBIT_EXEC_RESTRICT_FILE: crate::c_int = 256i32;
```

### `SECBIT_EXEC_RESTRICT_FILE_LOCKED`

```rust
const SECBIT_EXEC_RESTRICT_FILE_LOCKED: crate::c_int = 512i32;
```

### `SECURE_EXEC_DENY_INTERACTIVE`

```rust
const SECURE_EXEC_DENY_INTERACTIVE: crate::c_int = 10i32;
```

### `SECURE_EXEC_DENY_INTERACTIVE_LOCKED`

```rust
const SECURE_EXEC_DENY_INTERACTIVE_LOCKED: crate::c_int = 11i32;
```

### `SECBIT_EXEC_DENY_INTERACTIVE`

```rust
const SECBIT_EXEC_DENY_INTERACTIVE: crate::c_int = 1_024i32;
```

### `SECBIT_EXEC_DENY_INTERACTIVE_LOCKED`

```rust
const SECBIT_EXEC_DENY_INTERACTIVE_LOCKED: crate::c_int = 2_048i32;
```

### `SECUREBITS_DEFAULT`

```rust
const SECUREBITS_DEFAULT: crate::c_int = 0i32;
```

### `SECURE_ALL_BITS`

```rust
const SECURE_ALL_BITS: crate::c_int = 1_365i32;
```

### `SECURE_ALL_LOCKS`

```rust
const SECURE_ALL_LOCKS: crate::c_int = 2_730i32;
```

### `SECURE_ALL_UNPRIVILEGED`

```rust
const SECURE_ALL_UNPRIVILEGED: crate::c_int = 1_280i32;
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
const IN_CLOEXEC: crate::c_int = 524_288i32;
```

### `IN_NONBLOCK`

```rust
const IN_NONBLOCK: crate::c_int = 2_048i32;
```

### `OPEN_TREE_CLONE`

```rust
const OPEN_TREE_CLONE: crate::c_uint = 1u32;
```

### `OPEN_TREE_CLOEXEC`

```rust
const OPEN_TREE_CLOEXEC: crate::c_uint = 524_288u32;
```

### `NFT_TABLE_MAXNAMELEN`

```rust
const NFT_TABLE_MAXNAMELEN: crate::c_int = 256i32;
```

### `NFT_CHAIN_MAXNAMELEN`

```rust
const NFT_CHAIN_MAXNAMELEN: crate::c_int = 256i32;
```

### `NFT_SET_MAXNAMELEN`

```rust
const NFT_SET_MAXNAMELEN: crate::c_int = 256i32;
```

### `NFT_OBJ_MAXNAMELEN`

```rust
const NFT_OBJ_MAXNAMELEN: crate::c_int = 256i32;
```

### `NFT_USERDATA_MAXLEN`

```rust
const NFT_USERDATA_MAXLEN: crate::c_int = 256i32;
```

### `NFT_REG_VERDICT`

```rust
const NFT_REG_VERDICT: crate::c_int = 0i32;
```

### `NFT_REG_1`

```rust
const NFT_REG_1: crate::c_int = 1i32;
```

### `NFT_REG_2`

```rust
const NFT_REG_2: crate::c_int = 2i32;
```

### `NFT_REG_3`

```rust
const NFT_REG_3: crate::c_int = 3i32;
```

### `NFT_REG_4`

```rust
const NFT_REG_4: crate::c_int = 4i32;
```

### `__NFT_REG_MAX`

```rust
const __NFT_REG_MAX: crate::c_int = 5i32;
```

### `NFT_REG32_00`

```rust
const NFT_REG32_00: crate::c_int = 8i32;
```

### `NFT_REG32_01`

```rust
const NFT_REG32_01: crate::c_int = 9i32;
```

### `NFT_REG32_02`

```rust
const NFT_REG32_02: crate::c_int = 10i32;
```

### `NFT_REG32_03`

```rust
const NFT_REG32_03: crate::c_int = 11i32;
```

### `NFT_REG32_04`

```rust
const NFT_REG32_04: crate::c_int = 12i32;
```

### `NFT_REG32_05`

```rust
const NFT_REG32_05: crate::c_int = 13i32;
```

### `NFT_REG32_06`

```rust
const NFT_REG32_06: crate::c_int = 14i32;
```

### `NFT_REG32_07`

```rust
const NFT_REG32_07: crate::c_int = 15i32;
```

### `NFT_REG32_08`

```rust
const NFT_REG32_08: crate::c_int = 16i32;
```

### `NFT_REG32_09`

```rust
const NFT_REG32_09: crate::c_int = 17i32;
```

### `NFT_REG32_10`

```rust
const NFT_REG32_10: crate::c_int = 18i32;
```

### `NFT_REG32_11`

```rust
const NFT_REG32_11: crate::c_int = 19i32;
```

### `NFT_REG32_12`

```rust
const NFT_REG32_12: crate::c_int = 20i32;
```

### `NFT_REG32_13`

```rust
const NFT_REG32_13: crate::c_int = 21i32;
```

### `NFT_REG32_14`

```rust
const NFT_REG32_14: crate::c_int = 22i32;
```

### `NFT_REG32_15`

```rust
const NFT_REG32_15: crate::c_int = 23i32;
```

### `NFT_REG_SIZE`

```rust
const NFT_REG_SIZE: crate::c_int = 16i32;
```

### `NFT_REG32_SIZE`

```rust
const NFT_REG32_SIZE: crate::c_int = 4i32;
```

### `NFT_CONTINUE`

```rust
const NFT_CONTINUE: crate::c_int = -1i32;
```

### `NFT_BREAK`

```rust
const NFT_BREAK: crate::c_int = -2i32;
```

### `NFT_JUMP`

```rust
const NFT_JUMP: crate::c_int = -3i32;
```

### `NFT_GOTO`

```rust
const NFT_GOTO: crate::c_int = -4i32;
```

### `NFT_RETURN`

```rust
const NFT_RETURN: crate::c_int = -5i32;
```

### `NFT_MSG_NEWTABLE`

```rust
const NFT_MSG_NEWTABLE: crate::c_int = 0i32;
```

### `NFT_MSG_GETTABLE`

```rust
const NFT_MSG_GETTABLE: crate::c_int = 1i32;
```

### `NFT_MSG_DELTABLE`

```rust
const NFT_MSG_DELTABLE: crate::c_int = 2i32;
```

### `NFT_MSG_NEWCHAIN`

```rust
const NFT_MSG_NEWCHAIN: crate::c_int = 3i32;
```

### `NFT_MSG_GETCHAIN`

```rust
const NFT_MSG_GETCHAIN: crate::c_int = 4i32;
```

### `NFT_MSG_DELCHAIN`

```rust
const NFT_MSG_DELCHAIN: crate::c_int = 5i32;
```

### `NFT_MSG_NEWRULE`

```rust
const NFT_MSG_NEWRULE: crate::c_int = 6i32;
```

### `NFT_MSG_GETRULE`

```rust
const NFT_MSG_GETRULE: crate::c_int = 7i32;
```

### `NFT_MSG_DELRULE`

```rust
const NFT_MSG_DELRULE: crate::c_int = 8i32;
```

### `NFT_MSG_NEWSET`

```rust
const NFT_MSG_NEWSET: crate::c_int = 9i32;
```

### `NFT_MSG_GETSET`

```rust
const NFT_MSG_GETSET: crate::c_int = 10i32;
```

### `NFT_MSG_DELSET`

```rust
const NFT_MSG_DELSET: crate::c_int = 11i32;
```

### `NFT_MSG_NEWSETELEM`

```rust
const NFT_MSG_NEWSETELEM: crate::c_int = 12i32;
```

### `NFT_MSG_GETSETELEM`

```rust
const NFT_MSG_GETSETELEM: crate::c_int = 13i32;
```

### `NFT_MSG_DELSETELEM`

```rust
const NFT_MSG_DELSETELEM: crate::c_int = 14i32;
```

### `NFT_MSG_NEWGEN`

```rust
const NFT_MSG_NEWGEN: crate::c_int = 15i32;
```

### `NFT_MSG_GETGEN`

```rust
const NFT_MSG_GETGEN: crate::c_int = 16i32;
```

### `NFT_MSG_TRACE`

```rust
const NFT_MSG_TRACE: crate::c_int = 17i32;
```

### `NFT_MSG_NEWOBJ`

```rust
const NFT_MSG_NEWOBJ: crate::c_int = 18i32;
```

### `NFT_MSG_GETOBJ`

```rust
const NFT_MSG_GETOBJ: crate::c_int = 19i32;
```

### `NFT_MSG_DELOBJ`

```rust
const NFT_MSG_DELOBJ: crate::c_int = 20i32;
```

### `NFT_MSG_GETOBJ_RESET`

```rust
const NFT_MSG_GETOBJ_RESET: crate::c_int = 21i32;
```

### `NFT_MSG_MAX`

```rust
const NFT_MSG_MAX: crate::c_int = 34i32;
```

### `NFT_SET_ANONYMOUS`

```rust
const NFT_SET_ANONYMOUS: crate::c_int = 1i32;
```

### `NFT_SET_CONSTANT`

```rust
const NFT_SET_CONSTANT: crate::c_int = 2i32;
```

### `NFT_SET_INTERVAL`

```rust
const NFT_SET_INTERVAL: crate::c_int = 4i32;
```

### `NFT_SET_MAP`

```rust
const NFT_SET_MAP: crate::c_int = 8i32;
```

### `NFT_SET_TIMEOUT`

```rust
const NFT_SET_TIMEOUT: crate::c_int = 16i32;
```

### `NFT_SET_EVAL`

```rust
const NFT_SET_EVAL: crate::c_int = 32i32;
```

### `NFT_SET_POL_PERFORMANCE`

```rust
const NFT_SET_POL_PERFORMANCE: crate::c_int = 0i32;
```

### `NFT_SET_POL_MEMORY`

```rust
const NFT_SET_POL_MEMORY: crate::c_int = 1i32;
```

### `NFT_SET_ELEM_INTERVAL_END`

```rust
const NFT_SET_ELEM_INTERVAL_END: crate::c_int = 1i32;
```

### `NFT_DATA_VALUE`

```rust
const NFT_DATA_VALUE: crate::c_uint = 0u32;
```

### `NFT_DATA_VERDICT`

```rust
const NFT_DATA_VERDICT: crate::c_uint = 4_294_967_040u32;
```

### `NFT_DATA_RESERVED_MASK`

```rust
const NFT_DATA_RESERVED_MASK: crate::c_uint = 4_294_967_040u32;
```

### `NFT_DATA_VALUE_MAXLEN`

```rust
const NFT_DATA_VALUE_MAXLEN: crate::c_int = 64i32;
```

### `NFT_BYTEORDER_NTOH`

```rust
const NFT_BYTEORDER_NTOH: crate::c_int = 0i32;
```

### `NFT_BYTEORDER_HTON`

```rust
const NFT_BYTEORDER_HTON: crate::c_int = 1i32;
```

### `NFT_CMP_EQ`

```rust
const NFT_CMP_EQ: crate::c_int = 0i32;
```

### `NFT_CMP_NEQ`

```rust
const NFT_CMP_NEQ: crate::c_int = 1i32;
```

### `NFT_CMP_LT`

```rust
const NFT_CMP_LT: crate::c_int = 2i32;
```

### `NFT_CMP_LTE`

```rust
const NFT_CMP_LTE: crate::c_int = 3i32;
```

### `NFT_CMP_GT`

```rust
const NFT_CMP_GT: crate::c_int = 4i32;
```

### `NFT_CMP_GTE`

```rust
const NFT_CMP_GTE: crate::c_int = 5i32;
```

### `NFT_RANGE_EQ`

```rust
const NFT_RANGE_EQ: crate::c_int = 0i32;
```

### `NFT_RANGE_NEQ`

```rust
const NFT_RANGE_NEQ: crate::c_int = 1i32;
```

### `NFT_LOOKUP_F_INV`

```rust
const NFT_LOOKUP_F_INV: crate::c_int = 1i32;
```

### `NFT_DYNSET_OP_ADD`

```rust
const NFT_DYNSET_OP_ADD: crate::c_int = 0i32;
```

### `NFT_DYNSET_OP_UPDATE`

```rust
const NFT_DYNSET_OP_UPDATE: crate::c_int = 1i32;
```

### `NFT_DYNSET_F_INV`

```rust
const NFT_DYNSET_F_INV: crate::c_int = 1i32;
```

### `NFT_PAYLOAD_LL_HEADER`

```rust
const NFT_PAYLOAD_LL_HEADER: crate::c_int = 0i32;
```

### `NFT_PAYLOAD_NETWORK_HEADER`

```rust
const NFT_PAYLOAD_NETWORK_HEADER: crate::c_int = 1i32;
```

### `NFT_PAYLOAD_TRANSPORT_HEADER`

```rust
const NFT_PAYLOAD_TRANSPORT_HEADER: crate::c_int = 2i32;
```

### `NFT_PAYLOAD_CSUM_NONE`

```rust
const NFT_PAYLOAD_CSUM_NONE: crate::c_int = 0i32;
```

### `NFT_PAYLOAD_CSUM_INET`

```rust
const NFT_PAYLOAD_CSUM_INET: crate::c_int = 1i32;
```

### `NFT_META_LEN`

```rust
const NFT_META_LEN: crate::c_int = 0i32;
```

### `NFT_META_PROTOCOL`

```rust
const NFT_META_PROTOCOL: crate::c_int = 1i32;
```

### `NFT_META_PRIORITY`

```rust
const NFT_META_PRIORITY: crate::c_int = 2i32;
```

### `NFT_META_MARK`

```rust
const NFT_META_MARK: crate::c_int = 3i32;
```

### `NFT_META_IIF`

```rust
const NFT_META_IIF: crate::c_int = 4i32;
```

### `NFT_META_OIF`

```rust
const NFT_META_OIF: crate::c_int = 5i32;
```

### `NFT_META_IIFNAME`

```rust
const NFT_META_IIFNAME: crate::c_int = 6i32;
```

### `NFT_META_OIFNAME`

```rust
const NFT_META_OIFNAME: crate::c_int = 7i32;
```

### `NFT_META_IIFTYPE`

```rust
const NFT_META_IIFTYPE: crate::c_int = 8i32;
```

### `NFT_META_OIFTYPE`

```rust
const NFT_META_OIFTYPE: crate::c_int = 9i32;
```

### `NFT_META_SKUID`

```rust
const NFT_META_SKUID: crate::c_int = 10i32;
```

### `NFT_META_SKGID`

```rust
const NFT_META_SKGID: crate::c_int = 11i32;
```

### `NFT_META_NFTRACE`

```rust
const NFT_META_NFTRACE: crate::c_int = 12i32;
```

### `NFT_META_RTCLASSID`

```rust
const NFT_META_RTCLASSID: crate::c_int = 13i32;
```

### `NFT_META_SECMARK`

```rust
const NFT_META_SECMARK: crate::c_int = 14i32;
```

### `NFT_META_NFPROTO`

```rust
const NFT_META_NFPROTO: crate::c_int = 15i32;
```

### `NFT_META_L4PROTO`

```rust
const NFT_META_L4PROTO: crate::c_int = 16i32;
```

### `NFT_META_BRI_IIFNAME`

```rust
const NFT_META_BRI_IIFNAME: crate::c_int = 17i32;
```

### `NFT_META_BRI_OIFNAME`

```rust
const NFT_META_BRI_OIFNAME: crate::c_int = 18i32;
```

### `NFT_META_PKTTYPE`

```rust
const NFT_META_PKTTYPE: crate::c_int = 19i32;
```

### `NFT_META_CPU`

```rust
const NFT_META_CPU: crate::c_int = 20i32;
```

### `NFT_META_IIFGROUP`

```rust
const NFT_META_IIFGROUP: crate::c_int = 21i32;
```

### `NFT_META_OIFGROUP`

```rust
const NFT_META_OIFGROUP: crate::c_int = 22i32;
```

### `NFT_META_CGROUP`

```rust
const NFT_META_CGROUP: crate::c_int = 23i32;
```

### `NFT_META_PRANDOM`

```rust
const NFT_META_PRANDOM: crate::c_int = 24i32;
```

### `NFT_CT_STATE`

```rust
const NFT_CT_STATE: crate::c_int = 0i32;
```

### `NFT_CT_DIRECTION`

```rust
const NFT_CT_DIRECTION: crate::c_int = 1i32;
```

### `NFT_CT_STATUS`

```rust
const NFT_CT_STATUS: crate::c_int = 2i32;
```

### `NFT_CT_MARK`

```rust
const NFT_CT_MARK: crate::c_int = 3i32;
```

### `NFT_CT_SECMARK`

```rust
const NFT_CT_SECMARK: crate::c_int = 4i32;
```

### `NFT_CT_EXPIRATION`

```rust
const NFT_CT_EXPIRATION: crate::c_int = 5i32;
```

### `NFT_CT_HELPER`

```rust
const NFT_CT_HELPER: crate::c_int = 6i32;
```

### `NFT_CT_L3PROTOCOL`

```rust
const NFT_CT_L3PROTOCOL: crate::c_int = 7i32;
```

### `NFT_CT_SRC`

```rust
const NFT_CT_SRC: crate::c_int = 8i32;
```

### `NFT_CT_DST`

```rust
const NFT_CT_DST: crate::c_int = 9i32;
```

### `NFT_CT_PROTOCOL`

```rust
const NFT_CT_PROTOCOL: crate::c_int = 10i32;
```

### `NFT_CT_PROTO_SRC`

```rust
const NFT_CT_PROTO_SRC: crate::c_int = 11i32;
```

### `NFT_CT_PROTO_DST`

```rust
const NFT_CT_PROTO_DST: crate::c_int = 12i32;
```

### `NFT_CT_LABELS`

```rust
const NFT_CT_LABELS: crate::c_int = 13i32;
```

### `NFT_CT_PKTS`

```rust
const NFT_CT_PKTS: crate::c_int = 14i32;
```

### `NFT_CT_BYTES`

```rust
const NFT_CT_BYTES: crate::c_int = 15i32;
```

### `NFT_CT_AVGPKT`

```rust
const NFT_CT_AVGPKT: crate::c_int = 16i32;
```

### `NFT_CT_ZONE`

```rust
const NFT_CT_ZONE: crate::c_int = 17i32;
```

### `NFT_CT_EVENTMASK`

```rust
const NFT_CT_EVENTMASK: crate::c_int = 18i32;
```

### `NFT_CT_SRC_IP`

```rust
const NFT_CT_SRC_IP: crate::c_int = 19i32;
```

### `NFT_CT_DST_IP`

```rust
const NFT_CT_DST_IP: crate::c_int = 20i32;
```

### `NFT_CT_SRC_IP6`

```rust
const NFT_CT_SRC_IP6: crate::c_int = 21i32;
```

### `NFT_CT_DST_IP6`

```rust
const NFT_CT_DST_IP6: crate::c_int = 22i32;
```

### `NFT_LIMIT_PKTS`

```rust
const NFT_LIMIT_PKTS: crate::c_int = 0i32;
```

### `NFT_LIMIT_PKT_BYTES`

```rust
const NFT_LIMIT_PKT_BYTES: crate::c_int = 1i32;
```

### `NFT_LIMIT_F_INV`

```rust
const NFT_LIMIT_F_INV: crate::c_int = 1i32;
```

### `NFT_QUEUE_FLAG_BYPASS`

```rust
const NFT_QUEUE_FLAG_BYPASS: crate::c_int = 1i32;
```

### `NFT_QUEUE_FLAG_CPU_FANOUT`

```rust
const NFT_QUEUE_FLAG_CPU_FANOUT: crate::c_int = 2i32;
```

### `NFT_QUEUE_FLAG_MASK`

```rust
const NFT_QUEUE_FLAG_MASK: crate::c_int = 3i32;
```

### `NFT_QUOTA_F_INV`

```rust
const NFT_QUOTA_F_INV: crate::c_int = 1i32;
```

### `NFT_REJECT_ICMP_UNREACH`

```rust
const NFT_REJECT_ICMP_UNREACH: crate::c_int = 0i32;
```

### `NFT_REJECT_TCP_RST`

```rust
const NFT_REJECT_TCP_RST: crate::c_int = 1i32;
```

### `NFT_REJECT_ICMPX_UNREACH`

```rust
const NFT_REJECT_ICMPX_UNREACH: crate::c_int = 2i32;
```

### `NFT_REJECT_ICMPX_NO_ROUTE`

```rust
const NFT_REJECT_ICMPX_NO_ROUTE: crate::c_int = 0i32;
```

### `NFT_REJECT_ICMPX_PORT_UNREACH`

```rust
const NFT_REJECT_ICMPX_PORT_UNREACH: crate::c_int = 1i32;
```

### `NFT_REJECT_ICMPX_HOST_UNREACH`

```rust
const NFT_REJECT_ICMPX_HOST_UNREACH: crate::c_int = 2i32;
```

### `NFT_REJECT_ICMPX_ADMIN_PROHIBITED`

```rust
const NFT_REJECT_ICMPX_ADMIN_PROHIBITED: crate::c_int = 3i32;
```

### `NFT_NAT_SNAT`

```rust
const NFT_NAT_SNAT: crate::c_int = 0i32;
```

### `NFT_NAT_DNAT`

```rust
const NFT_NAT_DNAT: crate::c_int = 1i32;
```

### `NFT_TRACETYPE_UNSPEC`

```rust
const NFT_TRACETYPE_UNSPEC: crate::c_int = 0i32;
```

### `NFT_TRACETYPE_POLICY`

```rust
const NFT_TRACETYPE_POLICY: crate::c_int = 1i32;
```

### `NFT_TRACETYPE_RETURN`

```rust
const NFT_TRACETYPE_RETURN: crate::c_int = 2i32;
```

### `NFT_TRACETYPE_RULE`

```rust
const NFT_TRACETYPE_RULE: crate::c_int = 3i32;
```

### `NFT_NG_INCREMENTAL`

```rust
const NFT_NG_INCREMENTAL: crate::c_int = 0i32;
```

### `NFT_NG_RANDOM`

```rust
const NFT_NG_RANDOM: crate::c_int = 1i32;
```

### `FF_MAX`

```rust
const FF_MAX: __u16 = 127u16;
```

### `FF_CNT`

```rust
const FF_CNT: usize = 128usize;
```

### `INPUT_PROP_POINTER`

```rust
const INPUT_PROP_POINTER: __u16 = 0u16;
```

### `INPUT_PROP_DIRECT`

```rust
const INPUT_PROP_DIRECT: __u16 = 1u16;
```

### `INPUT_PROP_BUTTONPAD`

```rust
const INPUT_PROP_BUTTONPAD: __u16 = 2u16;
```

### `INPUT_PROP_SEMI_MT`

```rust
const INPUT_PROP_SEMI_MT: __u16 = 3u16;
```

### `INPUT_PROP_TOPBUTTONPAD`

```rust
const INPUT_PROP_TOPBUTTONPAD: __u16 = 4u16;
```

### `INPUT_PROP_POINTING_STICK`

```rust
const INPUT_PROP_POINTING_STICK: __u16 = 5u16;
```

### `INPUT_PROP_ACCELEROMETER`

```rust
const INPUT_PROP_ACCELEROMETER: __u16 = 6u16;
```

### `INPUT_PROP_MAX`

```rust
const INPUT_PROP_MAX: __u16 = 31u16;
```

### `INPUT_PROP_CNT`

```rust
const INPUT_PROP_CNT: usize = 32usize;
```

### `EV_MAX`

```rust
const EV_MAX: __u16 = 31u16;
```

### `EV_CNT`

```rust
const EV_CNT: usize = 32usize;
```

### `SYN_MAX`

```rust
const SYN_MAX: __u16 = 15u16;
```

### `SYN_CNT`

```rust
const SYN_CNT: usize = 16usize;
```

### `KEY_MAX`

```rust
const KEY_MAX: __u16 = 767u16;
```

### `KEY_CNT`

```rust
const KEY_CNT: usize = 768usize;
```

### `REL_MAX`

```rust
const REL_MAX: __u16 = 15u16;
```

### `REL_CNT`

```rust
const REL_CNT: usize = 16usize;
```

### `ABS_MAX`

```rust
const ABS_MAX: __u16 = 63u16;
```

### `ABS_CNT`

```rust
const ABS_CNT: usize = 64usize;
```

### `SW_MAX`

```rust
const SW_MAX: __u16 = 16u16;
```

### `SW_CNT`

```rust
const SW_CNT: usize = 17usize;
```

### `MSC_MAX`

```rust
const MSC_MAX: __u16 = 7u16;
```

### `MSC_CNT`

```rust
const MSC_CNT: usize = 8usize;
```

### `LED_MAX`

```rust
const LED_MAX: __u16 = 15u16;
```

### `LED_CNT`

```rust
const LED_CNT: usize = 16usize;
```

### `REP_MAX`

```rust
const REP_MAX: __u16 = 1u16;
```

### `REP_CNT`

```rust
const REP_CNT: usize = 2usize;
```

### `SND_MAX`

```rust
const SND_MAX: __u16 = 7u16;
```

### `SND_CNT`

```rust
const SND_CNT: usize = 8usize;
```

### `UINPUT_VERSION`

```rust
const UINPUT_VERSION: crate::c_uint = 5u32;
```

### `UINPUT_MAX_NAME_SIZE`

```rust
const UINPUT_MAX_NAME_SIZE: usize = 80usize;
```

### `FAN_ACCESS`

```rust
const FAN_ACCESS: u64 = 1u64;
```

### `FAN_MODIFY`

```rust
const FAN_MODIFY: u64 = 2u64;
```

### `FAN_ATTRIB`

```rust
const FAN_ATTRIB: u64 = 4u64;
```

### `FAN_CLOSE_WRITE`

```rust
const FAN_CLOSE_WRITE: u64 = 8u64;
```

### `FAN_CLOSE_NOWRITE`

```rust
const FAN_CLOSE_NOWRITE: u64 = 16u64;
```

### `FAN_OPEN`

```rust
const FAN_OPEN: u64 = 32u64;
```

### `FAN_MOVED_FROM`

```rust
const FAN_MOVED_FROM: u64 = 64u64;
```

### `FAN_MOVED_TO`

```rust
const FAN_MOVED_TO: u64 = 128u64;
```

### `FAN_CREATE`

```rust
const FAN_CREATE: u64 = 256u64;
```

### `FAN_DELETE`

```rust
const FAN_DELETE: u64 = 512u64;
```

### `FAN_DELETE_SELF`

```rust
const FAN_DELETE_SELF: u64 = 1_024u64;
```

### `FAN_MOVE_SELF`

```rust
const FAN_MOVE_SELF: u64 = 2_048u64;
```

### `FAN_OPEN_EXEC`

```rust
const FAN_OPEN_EXEC: u64 = 4_096u64;
```

### `FAN_Q_OVERFLOW`

```rust
const FAN_Q_OVERFLOW: u64 = 16_384u64;
```

### `FAN_FS_ERROR`

```rust
const FAN_FS_ERROR: u64 = 32_768u64;
```

### `FAN_OPEN_PERM`

```rust
const FAN_OPEN_PERM: u64 = 65_536u64;
```

### `FAN_ACCESS_PERM`

```rust
const FAN_ACCESS_PERM: u64 = 131_072u64;
```

### `FAN_OPEN_EXEC_PERM`

```rust
const FAN_OPEN_EXEC_PERM: u64 = 262_144u64;
```

### `FAN_EVENT_ON_CHILD`

```rust
const FAN_EVENT_ON_CHILD: u64 = 134_217_728u64;
```

### `FAN_RENAME`

```rust
const FAN_RENAME: u64 = 268_435_456u64;
```

### `FAN_ONDIR`

```rust
const FAN_ONDIR: u64 = 1_073_741_824u64;
```

### `FAN_CLOSE`

```rust
const FAN_CLOSE: u64 = 24u64;
```

### `FAN_MOVE`

```rust
const FAN_MOVE: u64 = 192u64;
```

### `FAN_CLOEXEC`

```rust
const FAN_CLOEXEC: crate::c_uint = 1u32;
```

### `FAN_NONBLOCK`

```rust
const FAN_NONBLOCK: crate::c_uint = 2u32;
```

### `FAN_CLASS_NOTIF`

```rust
const FAN_CLASS_NOTIF: crate::c_uint = 0u32;
```

### `FAN_CLASS_CONTENT`

```rust
const FAN_CLASS_CONTENT: crate::c_uint = 4u32;
```

### `FAN_CLASS_PRE_CONTENT`

```rust
const FAN_CLASS_PRE_CONTENT: crate::c_uint = 8u32;
```

### `FAN_UNLIMITED_QUEUE`

```rust
const FAN_UNLIMITED_QUEUE: crate::c_uint = 16u32;
```

### `FAN_UNLIMITED_MARKS`

```rust
const FAN_UNLIMITED_MARKS: crate::c_uint = 32u32;
```

### `FAN_ENABLE_AUDIT`

```rust
const FAN_ENABLE_AUDIT: crate::c_uint = 64u32;
```

### `FAN_REPORT_PIDFD`

```rust
const FAN_REPORT_PIDFD: crate::c_uint = 128u32;
```

### `FAN_REPORT_TID`

```rust
const FAN_REPORT_TID: crate::c_uint = 256u32;
```

### `FAN_REPORT_FID`

```rust
const FAN_REPORT_FID: crate::c_uint = 512u32;
```

### `FAN_REPORT_DIR_FID`

```rust
const FAN_REPORT_DIR_FID: crate::c_uint = 1_024u32;
```

### `FAN_REPORT_NAME`

```rust
const FAN_REPORT_NAME: crate::c_uint = 2_048u32;
```

### `FAN_REPORT_TARGET_FID`

```rust
const FAN_REPORT_TARGET_FID: crate::c_uint = 4_096u32;
```

### `FAN_REPORT_DFID_NAME`

```rust
const FAN_REPORT_DFID_NAME: crate::c_uint = 3_072u32;
```

### `FAN_REPORT_DFID_NAME_TARGET`

```rust
const FAN_REPORT_DFID_NAME_TARGET: crate::c_uint = 7_680u32;
```

### `FAN_MARK_ADD`

```rust
const FAN_MARK_ADD: crate::c_uint = 1u32;
```

### `FAN_MARK_REMOVE`

```rust
const FAN_MARK_REMOVE: crate::c_uint = 2u32;
```

### `FAN_MARK_DONT_FOLLOW`

```rust
const FAN_MARK_DONT_FOLLOW: crate::c_uint = 4u32;
```

### `FAN_MARK_ONLYDIR`

```rust
const FAN_MARK_ONLYDIR: crate::c_uint = 8u32;
```

### `FAN_MARK_IGNORED_MASK`

```rust
const FAN_MARK_IGNORED_MASK: crate::c_uint = 32u32;
```

### `FAN_MARK_IGNORED_SURV_MODIFY`

```rust
const FAN_MARK_IGNORED_SURV_MODIFY: crate::c_uint = 64u32;
```

### `FAN_MARK_FLUSH`

```rust
const FAN_MARK_FLUSH: crate::c_uint = 128u32;
```

### `FAN_MARK_EVICTABLE`

```rust
const FAN_MARK_EVICTABLE: crate::c_uint = 512u32;
```

### `FAN_MARK_IGNORE`

```rust
const FAN_MARK_IGNORE: crate::c_uint = 1_024u32;
```

### `FAN_MARK_INODE`

```rust
const FAN_MARK_INODE: crate::c_uint = 0u32;
```

### `FAN_MARK_MOUNT`

```rust
const FAN_MARK_MOUNT: crate::c_uint = 16u32;
```

### `FAN_MARK_FILESYSTEM`

```rust
const FAN_MARK_FILESYSTEM: crate::c_uint = 256u32;
```

### `FAN_MARK_IGNORE_SURV`

```rust
const FAN_MARK_IGNORE_SURV: crate::c_uint = 1_088u32;
```

### `FANOTIFY_METADATA_VERSION`

```rust
const FANOTIFY_METADATA_VERSION: u8 = 3u8;
```

### `FAN_EVENT_INFO_TYPE_FID`

```rust
const FAN_EVENT_INFO_TYPE_FID: u8 = 1u8;
```

### `FAN_EVENT_INFO_TYPE_DFID_NAME`

```rust
const FAN_EVENT_INFO_TYPE_DFID_NAME: u8 = 2u8;
```

### `FAN_EVENT_INFO_TYPE_DFID`

```rust
const FAN_EVENT_INFO_TYPE_DFID: u8 = 3u8;
```

### `FAN_EVENT_INFO_TYPE_PIDFD`

```rust
const FAN_EVENT_INFO_TYPE_PIDFD: u8 = 4u8;
```

### `FAN_EVENT_INFO_TYPE_ERROR`

```rust
const FAN_EVENT_INFO_TYPE_ERROR: u8 = 5u8;
```

### `FAN_EVENT_INFO_TYPE_OLD_DFID_NAME`

```rust
const FAN_EVENT_INFO_TYPE_OLD_DFID_NAME: u8 = 10u8;
```

### `FAN_EVENT_INFO_TYPE_NEW_DFID_NAME`

```rust
const FAN_EVENT_INFO_TYPE_NEW_DFID_NAME: u8 = 12u8;
```

### `FAN_RESPONSE_INFO_NONE`

```rust
const FAN_RESPONSE_INFO_NONE: u8 = 0u8;
```

### `FAN_RESPONSE_INFO_AUDIT_RULE`

```rust
const FAN_RESPONSE_INFO_AUDIT_RULE: u8 = 1u8;
```

### `FAN_ALLOW`

```rust
const FAN_ALLOW: u32 = 1u32;
```

### `FAN_DENY`

```rust
const FAN_DENY: u32 = 2u32;
```

### `FAN_AUDIT`

```rust
const FAN_AUDIT: u32 = 16u32;
```

### `FAN_INFO`

```rust
const FAN_INFO: u32 = 32u32;
```

### `FAN_NOFD`

```rust
const FAN_NOFD: crate::c_int = -1i32;
```

### `FAN_NOPIDFD`

```rust
const FAN_NOPIDFD: crate::c_int = -1i32;
```

### `FAN_EPIDFD`

```rust
const FAN_EPIDFD: crate::c_int = -2i32;
```

### `FUTEX_WAIT`

```rust
const FUTEX_WAIT: crate::c_int = 0i32;
```

### `FUTEX_WAKE`

```rust
const FUTEX_WAKE: crate::c_int = 1i32;
```

### `FUTEX_FD`

```rust
const FUTEX_FD: crate::c_int = 2i32;
```

### `FUTEX_REQUEUE`

```rust
const FUTEX_REQUEUE: crate::c_int = 3i32;
```

### `FUTEX_CMP_REQUEUE`

```rust
const FUTEX_CMP_REQUEUE: crate::c_int = 4i32;
```

### `FUTEX_WAKE_OP`

```rust
const FUTEX_WAKE_OP: crate::c_int = 5i32;
```

### `FUTEX_LOCK_PI`

```rust
const FUTEX_LOCK_PI: crate::c_int = 6i32;
```

### `FUTEX_UNLOCK_PI`

```rust
const FUTEX_UNLOCK_PI: crate::c_int = 7i32;
```

### `FUTEX_TRYLOCK_PI`

```rust
const FUTEX_TRYLOCK_PI: crate::c_int = 8i32;
```

### `FUTEX_WAIT_BITSET`

```rust
const FUTEX_WAIT_BITSET: crate::c_int = 9i32;
```

### `FUTEX_WAKE_BITSET`

```rust
const FUTEX_WAKE_BITSET: crate::c_int = 10i32;
```

### `FUTEX_WAIT_REQUEUE_PI`

```rust
const FUTEX_WAIT_REQUEUE_PI: crate::c_int = 11i32;
```

### `FUTEX_CMP_REQUEUE_PI`

```rust
const FUTEX_CMP_REQUEUE_PI: crate::c_int = 12i32;
```

### `FUTEX_LOCK_PI2`

```rust
const FUTEX_LOCK_PI2: crate::c_int = 13i32;
```

### `FUTEX_PRIVATE_FLAG`

```rust
const FUTEX_PRIVATE_FLAG: crate::c_int = 128i32;
```

### `FUTEX_CLOCK_REALTIME`

```rust
const FUTEX_CLOCK_REALTIME: crate::c_int = 256i32;
```

### `FUTEX_CMD_MASK`

```rust
const FUTEX_CMD_MASK: crate::c_int = -385i32;
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

### `FUTEX_BITSET_MATCH_ANY`

```rust
const FUTEX_BITSET_MATCH_ANY: crate::c_int = -1i32;
```

### `FUTEX_OP_SET`

```rust
const FUTEX_OP_SET: crate::c_int = 0i32;
```

### `FUTEX_OP_ADD`

```rust
const FUTEX_OP_ADD: crate::c_int = 1i32;
```

### `FUTEX_OP_OR`

```rust
const FUTEX_OP_OR: crate::c_int = 2i32;
```

### `FUTEX_OP_ANDN`

```rust
const FUTEX_OP_ANDN: crate::c_int = 3i32;
```

### `FUTEX_OP_XOR`

```rust
const FUTEX_OP_XOR: crate::c_int = 4i32;
```

### `FUTEX_OP_OPARG_SHIFT`

```rust
const FUTEX_OP_OPARG_SHIFT: crate::c_int = 8i32;
```

### `FUTEX_OP_CMP_EQ`

```rust
const FUTEX_OP_CMP_EQ: crate::c_int = 0i32;
```

### `FUTEX_OP_CMP_NE`

```rust
const FUTEX_OP_CMP_NE: crate::c_int = 1i32;
```

### `FUTEX_OP_CMP_LT`

```rust
const FUTEX_OP_CMP_LT: crate::c_int = 2i32;
```

### `FUTEX_OP_CMP_LE`

```rust
const FUTEX_OP_CMP_LE: crate::c_int = 3i32;
```

### `FUTEX_OP_CMP_GT`

```rust
const FUTEX_OP_CMP_GT: crate::c_int = 4i32;
```

### `FUTEX_OP_CMP_GE`

```rust
const FUTEX_OP_CMP_GE: crate::c_int = 5i32;
```

### `KEXEC_ON_CRASH`

```rust
const KEXEC_ON_CRASH: crate::c_int = 1i32;
```

### `KEXEC_PRESERVE_CONTEXT`

```rust
const KEXEC_PRESERVE_CONTEXT: crate::c_int = 2i32;
```

### `KEXEC_ARCH_MASK`

```rust
const KEXEC_ARCH_MASK: crate::c_int = -65_536i32;
```

### `KEXEC_FILE_UNLOAD`

```rust
const KEXEC_FILE_UNLOAD: crate::c_int = 1i32;
```

### `KEXEC_FILE_ON_CRASH`

```rust
const KEXEC_FILE_ON_CRASH: crate::c_int = 2i32;
```

### `KEXEC_FILE_NO_INITRAMFS`

```rust
const KEXEC_FILE_NO_INITRAMFS: crate::c_int = 4i32;
```

### `LINUX_REBOOT_MAGIC1`

```rust
const LINUX_REBOOT_MAGIC1: crate::c_int = -18_751_827i32;
```

### `LINUX_REBOOT_MAGIC2`

```rust
const LINUX_REBOOT_MAGIC2: crate::c_int = 672_274_793i32;
```

### `LINUX_REBOOT_MAGIC2A`

```rust
const LINUX_REBOOT_MAGIC2A: crate::c_int = 85_072_278i32;
```

### `LINUX_REBOOT_MAGIC2B`

```rust
const LINUX_REBOOT_MAGIC2B: crate::c_int = 369_367_448i32;
```

### `LINUX_REBOOT_MAGIC2C`

```rust
const LINUX_REBOOT_MAGIC2C: crate::c_int = 537_993_216i32;
```

### `LINUX_REBOOT_CMD_RESTART`

```rust
const LINUX_REBOOT_CMD_RESTART: crate::c_int = 19_088_743i32;
```

### `LINUX_REBOOT_CMD_HALT`

```rust
const LINUX_REBOOT_CMD_HALT: crate::c_int = -839_974_621i32;
```

### `LINUX_REBOOT_CMD_CAD_ON`

```rust
const LINUX_REBOOT_CMD_CAD_ON: crate::c_int = -1_985_229_329i32;
```

### `LINUX_REBOOT_CMD_CAD_OFF`

```rust
const LINUX_REBOOT_CMD_CAD_OFF: crate::c_int = 0i32;
```

### `LINUX_REBOOT_CMD_POWER_OFF`

```rust
const LINUX_REBOOT_CMD_POWER_OFF: crate::c_int = 1_126_301_404i32;
```

### `LINUX_REBOOT_CMD_RESTART2`

```rust
const LINUX_REBOOT_CMD_RESTART2: crate::c_int = -1_582_119_980i32;
```

### `LINUX_REBOOT_CMD_SW_SUSPEND`

```rust
const LINUX_REBOOT_CMD_SW_SUSPEND: crate::c_int = -805_241_630i32;
```

### `LINUX_REBOOT_CMD_KEXEC`

```rust
const LINUX_REBOOT_CMD_KEXEC: crate::c_int = 1_163_412_803i32;
```

### `REG_EXTENDED`

```rust
const REG_EXTENDED: crate::c_int = 1i32;
```

### `REG_ICASE`

```rust
const REG_ICASE: crate::c_int = 2i32;
```

### `REG_NEWLINE`

```rust
const REG_NEWLINE: crate::c_int = 4i32;
```

### `REG_NOSUB`

```rust
const REG_NOSUB: crate::c_int = 8i32;
```

### `REG_NOTBOL`

```rust
const REG_NOTBOL: crate::c_int = 1i32;
```

### `REG_NOTEOL`

```rust
const REG_NOTEOL: crate::c_int = 2i32;
```

### `REG_ENOSYS`

```rust
const REG_ENOSYS: crate::c_int = -1i32;
```

### `REG_NOMATCH`

```rust
const REG_NOMATCH: crate::c_int = 1i32;
```

### `REG_BADPAT`

```rust
const REG_BADPAT: crate::c_int = 2i32;
```

### `REG_ECOLLATE`

```rust
const REG_ECOLLATE: crate::c_int = 3i32;
```

### `REG_ECTYPE`

```rust
const REG_ECTYPE: crate::c_int = 4i32;
```

### `REG_EESCAPE`

```rust
const REG_EESCAPE: crate::c_int = 5i32;
```

### `REG_ESUBREG`

```rust
const REG_ESUBREG: crate::c_int = 6i32;
```

### `REG_EBRACK`

```rust
const REG_EBRACK: crate::c_int = 7i32;
```

### `REG_EPAREN`

```rust
const REG_EPAREN: crate::c_int = 8i32;
```

### `REG_EBRACE`

```rust
const REG_EBRACE: crate::c_int = 9i32;
```

### `REG_BADBR`

```rust
const REG_BADBR: crate::c_int = 10i32;
```

### `REG_ERANGE`

```rust
const REG_ERANGE: crate::c_int = 11i32;
```

### `REG_ESPACE`

```rust
const REG_ESPACE: crate::c_int = 12i32;
```

### `REG_BADRPT`

```rust
const REG_BADRPT: crate::c_int = 13i32;
```

### `SO_EE_ORIGIN_NONE`

```rust
const SO_EE_ORIGIN_NONE: u8 = 0u8;
```

### `SO_EE_ORIGIN_LOCAL`

```rust
const SO_EE_ORIGIN_LOCAL: u8 = 1u8;
```

### `SO_EE_ORIGIN_ICMP`

```rust
const SO_EE_ORIGIN_ICMP: u8 = 2u8;
```

### `SO_EE_ORIGIN_ICMP6`

```rust
const SO_EE_ORIGIN_ICMP6: u8 = 3u8;
```

### `SO_EE_ORIGIN_TXSTATUS`

```rust
const SO_EE_ORIGIN_TXSTATUS: u8 = 4u8;
```

### `SO_EE_ORIGIN_TIMESTAMPING`

```rust
const SO_EE_ORIGIN_TIMESTAMPING: u8 = 4u8;
```

### `EPERM`

```rust
const EPERM: crate::c_int = 1i32;
```

### `ENOENT`

```rust
const ENOENT: crate::c_int = 2i32;
```

### `ESRCH`

```rust
const ESRCH: crate::c_int = 3i32;
```

### `EINTR`

```rust
const EINTR: crate::c_int = 4i32;
```

### `EIO`

```rust
const EIO: crate::c_int = 5i32;
```

### `ENXIO`

```rust
const ENXIO: crate::c_int = 6i32;
```

### `E2BIG`

```rust
const E2BIG: crate::c_int = 7i32;
```

### `ENOEXEC`

```rust
const ENOEXEC: crate::c_int = 8i32;
```

### `EBADF`

```rust
const EBADF: crate::c_int = 9i32;
```

### `ECHILD`

```rust
const ECHILD: crate::c_int = 10i32;
```

### `EAGAIN`

```rust
const EAGAIN: crate::c_int = 11i32;
```

### `ENOMEM`

```rust
const ENOMEM: crate::c_int = 12i32;
```

### `EACCES`

```rust
const EACCES: crate::c_int = 13i32;
```

### `EFAULT`

```rust
const EFAULT: crate::c_int = 14i32;
```

### `ENOTBLK`

```rust
const ENOTBLK: crate::c_int = 15i32;
```

### `EBUSY`

```rust
const EBUSY: crate::c_int = 16i32;
```

### `EEXIST`

```rust
const EEXIST: crate::c_int = 17i32;
```

### `EXDEV`

```rust
const EXDEV: crate::c_int = 18i32;
```

### `ENODEV`

```rust
const ENODEV: crate::c_int = 19i32;
```

### `ENOTDIR`

```rust
const ENOTDIR: crate::c_int = 20i32;
```

### `EISDIR`

```rust
const EISDIR: crate::c_int = 21i32;
```

### `EINVAL`

```rust
const EINVAL: crate::c_int = 22i32;
```

### `ENFILE`

```rust
const ENFILE: crate::c_int = 23i32;
```

### `EMFILE`

```rust
const EMFILE: crate::c_int = 24i32;
```

### `ENOTTY`

```rust
const ENOTTY: crate::c_int = 25i32;
```

### `ETXTBSY`

```rust
const ETXTBSY: crate::c_int = 26i32;
```

### `EFBIG`

```rust
const EFBIG: crate::c_int = 27i32;
```

### `ENOSPC`

```rust
const ENOSPC: crate::c_int = 28i32;
```

### `ESPIPE`

```rust
const ESPIPE: crate::c_int = 29i32;
```

### `EROFS`

```rust
const EROFS: crate::c_int = 30i32;
```

### `EMLINK`

```rust
const EMLINK: crate::c_int = 31i32;
```

### `EPIPE`

```rust
const EPIPE: crate::c_int = 32i32;
```

### `EDOM`

```rust
const EDOM: crate::c_int = 33i32;
```

### `ERANGE`

```rust
const ERANGE: crate::c_int = 34i32;
```

### `EWOULDBLOCK`

```rust
const EWOULDBLOCK: crate::c_int = 11i32;
```

### `SCTP_FUTURE_ASSOC`

```rust
const SCTP_FUTURE_ASSOC: crate::c_int = 0i32;
```

### `SCTP_CURRENT_ASSOC`

```rust
const SCTP_CURRENT_ASSOC: crate::c_int = 1i32;
```

### `SCTP_ALL_ASSOC`

```rust
const SCTP_ALL_ASSOC: crate::c_int = 2i32;
```

### `SCTP_RTOINFO`

```rust
const SCTP_RTOINFO: crate::c_int = 0i32;
```

### `SCTP_ASSOCINFO`

```rust
const SCTP_ASSOCINFO: crate::c_int = 1i32;
```

### `SCTP_INITMSG`

```rust
const SCTP_INITMSG: crate::c_int = 2i32;
```

### `SCTP_NODELAY`

```rust
const SCTP_NODELAY: crate::c_int = 3i32;
```

### `SCTP_AUTOCLOSE`

```rust
const SCTP_AUTOCLOSE: crate::c_int = 4i32;
```

### `SCTP_SET_PEER_PRIMARY_ADDR`

```rust
const SCTP_SET_PEER_PRIMARY_ADDR: crate::c_int = 5i32;
```

### `SCTP_PRIMARY_ADDR`

```rust
const SCTP_PRIMARY_ADDR: crate::c_int = 6i32;
```

### `SCTP_ADAPTATION_LAYER`

```rust
const SCTP_ADAPTATION_LAYER: crate::c_int = 7i32;
```

### `SCTP_DISABLE_FRAGMENTS`

```rust
const SCTP_DISABLE_FRAGMENTS: crate::c_int = 8i32;
```

### `SCTP_PEER_ADDR_PARAMS`

```rust
const SCTP_PEER_ADDR_PARAMS: crate::c_int = 9i32;
```

### `SCTP_DEFAULT_SEND_PARAM`

```rust
const SCTP_DEFAULT_SEND_PARAM: crate::c_int = 10i32;
```

### `SCTP_EVENTS`

```rust
const SCTP_EVENTS: crate::c_int = 11i32;
```

### `SCTP_I_WANT_MAPPED_V4_ADDR`

```rust
const SCTP_I_WANT_MAPPED_V4_ADDR: crate::c_int = 12i32;
```

### `SCTP_MAXSEG`

```rust
const SCTP_MAXSEG: crate::c_int = 13i32;
```

### `SCTP_STATUS`

```rust
const SCTP_STATUS: crate::c_int = 14i32;
```

### `SCTP_GET_PEER_ADDR_INFO`

```rust
const SCTP_GET_PEER_ADDR_INFO: crate::c_int = 15i32;
```

### `SCTP_DELAYED_ACK_TIME`

```rust
const SCTP_DELAYED_ACK_TIME: crate::c_int = 16i32;
```

### `SCTP_DELAYED_ACK`

```rust
const SCTP_DELAYED_ACK: crate::c_int = 16i32;
```

### `SCTP_DELAYED_SACK`

```rust
const SCTP_DELAYED_SACK: crate::c_int = 16i32;
```

### `SCTP_CONTEXT`

```rust
const SCTP_CONTEXT: crate::c_int = 17i32;
```

### `SCTP_FRAGMENT_INTERLEAVE`

```rust
const SCTP_FRAGMENT_INTERLEAVE: crate::c_int = 18i32;
```

### `SCTP_PARTIAL_DELIVERY_POINT`

```rust
const SCTP_PARTIAL_DELIVERY_POINT: crate::c_int = 19i32;
```

### `SCTP_MAX_BURST`

```rust
const SCTP_MAX_BURST: crate::c_int = 20i32;
```

### `SCTP_AUTH_CHUNK`

```rust
const SCTP_AUTH_CHUNK: crate::c_int = 21i32;
```

### `SCTP_HMAC_IDENT`

```rust
const SCTP_HMAC_IDENT: crate::c_int = 22i32;
```

### `SCTP_AUTH_KEY`

```rust
const SCTP_AUTH_KEY: crate::c_int = 23i32;
```

### `SCTP_AUTH_ACTIVE_KEY`

```rust
const SCTP_AUTH_ACTIVE_KEY: crate::c_int = 24i32;
```

### `SCTP_AUTH_DELETE_KEY`

```rust
const SCTP_AUTH_DELETE_KEY: crate::c_int = 25i32;
```

### `SCTP_PEER_AUTH_CHUNKS`

```rust
const SCTP_PEER_AUTH_CHUNKS: crate::c_int = 26i32;
```

### `SCTP_LOCAL_AUTH_CHUNKS`

```rust
const SCTP_LOCAL_AUTH_CHUNKS: crate::c_int = 27i32;
```

### `SCTP_GET_ASSOC_NUMBER`

```rust
const SCTP_GET_ASSOC_NUMBER: crate::c_int = 28i32;
```

### `SCTP_GET_ASSOC_ID_LIST`

```rust
const SCTP_GET_ASSOC_ID_LIST: crate::c_int = 29i32;
```

### `SCTP_AUTO_ASCONF`

```rust
const SCTP_AUTO_ASCONF: crate::c_int = 30i32;
```

### `SCTP_PEER_ADDR_THLDS`

```rust
const SCTP_PEER_ADDR_THLDS: crate::c_int = 31i32;
```

### `SCTP_RECVRCVINFO`

```rust
const SCTP_RECVRCVINFO: crate::c_int = 32i32;
```

### `SCTP_RECVNXTINFO`

```rust
const SCTP_RECVNXTINFO: crate::c_int = 33i32;
```

### `SCTP_DEFAULT_SNDINFO`

```rust
const SCTP_DEFAULT_SNDINFO: crate::c_int = 34i32;
```

### `SCTP_AUTH_DEACTIVATE_KEY`

```rust
const SCTP_AUTH_DEACTIVATE_KEY: crate::c_int = 35i32;
```

### `SCTP_REUSE_PORT`

```rust
const SCTP_REUSE_PORT: crate::c_int = 36i32;
```

### `SCTP_PEER_ADDR_THLDS_V2`

```rust
const SCTP_PEER_ADDR_THLDS_V2: crate::c_int = 37i32;
```

### `SCTP_PR_SCTP_NONE`

```rust
const SCTP_PR_SCTP_NONE: crate::c_int = 0i32;
```

### `SCTP_PR_SCTP_TTL`

```rust
const SCTP_PR_SCTP_TTL: crate::c_int = 16i32;
```

### `SCTP_PR_SCTP_RTX`

```rust
const SCTP_PR_SCTP_RTX: crate::c_int = 32i32;
```

### `SCTP_PR_SCTP_PRIO`

```rust
const SCTP_PR_SCTP_PRIO: crate::c_int = 48i32;
```

### `SCTP_PR_SCTP_MAX`

```rust
const SCTP_PR_SCTP_MAX: crate::c_int = 48i32;
```

### `SCTP_PR_SCTP_MASK`

```rust
const SCTP_PR_SCTP_MASK: crate::c_int = 48i32;
```

### `SCTP_ENABLE_RESET_STREAM_REQ`

```rust
const SCTP_ENABLE_RESET_STREAM_REQ: crate::c_int = 1i32;
```

### `SCTP_ENABLE_RESET_ASSOC_REQ`

```rust
const SCTP_ENABLE_RESET_ASSOC_REQ: crate::c_int = 2i32;
```

### `SCTP_ENABLE_CHANGE_ASSOC_REQ`

```rust
const SCTP_ENABLE_CHANGE_ASSOC_REQ: crate::c_int = 4i32;
```

### `SCTP_ENABLE_STRRESET_MASK`

```rust
const SCTP_ENABLE_STRRESET_MASK: crate::c_int = 7i32;
```

### `SCTP_STREAM_RESET_INCOMING`

```rust
const SCTP_STREAM_RESET_INCOMING: crate::c_int = 1i32;
```

### `SCTP_STREAM_RESET_OUTGOING`

```rust
const SCTP_STREAM_RESET_OUTGOING: crate::c_int = 2i32;
```

### `SCTP_INIT`

```rust
const SCTP_INIT: crate::c_int = 0i32;
```

### `SCTP_SNDRCV`

```rust
const SCTP_SNDRCV: crate::c_int = 1i32;
```

### `SCTP_SNDINFO`

```rust
const SCTP_SNDINFO: crate::c_int = 2i32;
```

### `SCTP_RCVINFO`

```rust
const SCTP_RCVINFO: crate::c_int = 3i32;
```

### `SCTP_NXTINFO`

```rust
const SCTP_NXTINFO: crate::c_int = 4i32;
```

### `SCTP_PRINFO`

```rust
const SCTP_PRINFO: crate::c_int = 5i32;
```

### `SCTP_AUTHINFO`

```rust
const SCTP_AUTHINFO: crate::c_int = 6i32;
```

### `SCTP_DSTADDRV4`

```rust
const SCTP_DSTADDRV4: crate::c_int = 7i32;
```

### `SCTP_DSTADDRV6`

```rust
const SCTP_DSTADDRV6: crate::c_int = 8i32;
```

### `SCTP_UNORDERED`

```rust
const SCTP_UNORDERED: crate::c_int = 1i32;
```

### `SCTP_ADDR_OVER`

```rust
const SCTP_ADDR_OVER: crate::c_int = 2i32;
```

### `SCTP_ABORT`

```rust
const SCTP_ABORT: crate::c_int = 4i32;
```

### `SCTP_SACK_IMMEDIATELY`

```rust
const SCTP_SACK_IMMEDIATELY: crate::c_int = 8i32;
```

### `SCTP_SENDALL`

```rust
const SCTP_SENDALL: crate::c_int = 64i32;
```

### `SCTP_PR_SCTP_ALL`

```rust
const SCTP_PR_SCTP_ALL: crate::c_int = 128i32;
```

### `SCTP_NOTIFICATION`

```rust
const SCTP_NOTIFICATION: crate::c_int = 32_768i32;
```

### `SCTP_EOF`

```rust
const SCTP_EOF: crate::c_int = 512i32;
```

### `DCCP_SOCKOPT_PACKET_SIZE`

```rust
const DCCP_SOCKOPT_PACKET_SIZE: crate::c_int = 1i32;
```

### `DCCP_SOCKOPT_SERVICE`

```rust
const DCCP_SOCKOPT_SERVICE: crate::c_int = 2i32;
```

### `DCCP_SOCKOPT_CHANGE_L`

```rust
const DCCP_SOCKOPT_CHANGE_L: crate::c_int = 3i32;
```

### `DCCP_SOCKOPT_CHANGE_R`

```rust
const DCCP_SOCKOPT_CHANGE_R: crate::c_int = 4i32;
```

### `DCCP_SOCKOPT_GET_CUR_MPS`

```rust
const DCCP_SOCKOPT_GET_CUR_MPS: crate::c_int = 5i32;
```

### `DCCP_SOCKOPT_SERVER_TIMEWAIT`

```rust
const DCCP_SOCKOPT_SERVER_TIMEWAIT: crate::c_int = 6i32;
```

### `DCCP_SOCKOPT_SEND_CSCOV`

```rust
const DCCP_SOCKOPT_SEND_CSCOV: crate::c_int = 10i32;
```

### `DCCP_SOCKOPT_RECV_CSCOV`

```rust
const DCCP_SOCKOPT_RECV_CSCOV: crate::c_int = 11i32;
```

### `DCCP_SOCKOPT_AVAILABLE_CCIDS`

```rust
const DCCP_SOCKOPT_AVAILABLE_CCIDS: crate::c_int = 12i32;
```

### `DCCP_SOCKOPT_CCID`

```rust
const DCCP_SOCKOPT_CCID: crate::c_int = 13i32;
```

### `DCCP_SOCKOPT_TX_CCID`

```rust
const DCCP_SOCKOPT_TX_CCID: crate::c_int = 14i32;
```

### `DCCP_SOCKOPT_RX_CCID`

```rust
const DCCP_SOCKOPT_RX_CCID: crate::c_int = 15i32;
```

### `DCCP_SOCKOPT_QPOLICY_ID`

```rust
const DCCP_SOCKOPT_QPOLICY_ID: crate::c_int = 16i32;
```

### `DCCP_SOCKOPT_QPOLICY_TXQLEN`

```rust
const DCCP_SOCKOPT_QPOLICY_TXQLEN: crate::c_int = 17i32;
```

### `DCCP_SOCKOPT_CCID_RX_INFO`

```rust
const DCCP_SOCKOPT_CCID_RX_INFO: crate::c_int = 128i32;
```

### `DCCP_SOCKOPT_CCID_TX_INFO`

```rust
const DCCP_SOCKOPT_CCID_TX_INFO: crate::c_int = 192i32;
```

### `DCCP_SERVICE_LIST_MAX_LEN`

```rust
const DCCP_SERVICE_LIST_MAX_LEN: crate::c_int = 32i32;
```

maximum number of services provided on the same listening port

### `CTL_KERN`

```rust
const CTL_KERN: crate::c_int = 1i32;
```

### `CTL_VM`

```rust
const CTL_VM: crate::c_int = 2i32;
```

### `CTL_NET`

```rust
const CTL_NET: crate::c_int = 3i32;
```

### `CTL_FS`

```rust
const CTL_FS: crate::c_int = 5i32;
```

### `CTL_DEBUG`

```rust
const CTL_DEBUG: crate::c_int = 6i32;
```

### `CTL_DEV`

```rust
const CTL_DEV: crate::c_int = 7i32;
```

### `CTL_BUS`

```rust
const CTL_BUS: crate::c_int = 8i32;
```

### `CTL_ABI`

```rust
const CTL_ABI: crate::c_int = 9i32;
```

### `CTL_CPU`

```rust
const CTL_CPU: crate::c_int = 10i32;
```

### `CTL_BUS_ISA`

```rust
const CTL_BUS_ISA: crate::c_int = 1i32;
```

### `INOTIFY_MAX_USER_INSTANCES`

```rust
const INOTIFY_MAX_USER_INSTANCES: crate::c_int = 1i32;
```

### `INOTIFY_MAX_USER_WATCHES`

```rust
const INOTIFY_MAX_USER_WATCHES: crate::c_int = 2i32;
```

### `INOTIFY_MAX_QUEUED_EVENTS`

```rust
const INOTIFY_MAX_QUEUED_EVENTS: crate::c_int = 3i32;
```

### `KERN_OSTYPE`

```rust
const KERN_OSTYPE: crate::c_int = 1i32;
```

### `KERN_OSRELEASE`

```rust
const KERN_OSRELEASE: crate::c_int = 2i32;
```

### `KERN_OSREV`

```rust
const KERN_OSREV: crate::c_int = 3i32;
```

### `KERN_VERSION`

```rust
const KERN_VERSION: crate::c_int = 4i32;
```

### `KERN_SECUREMASK`

```rust
const KERN_SECUREMASK: crate::c_int = 5i32;
```

### `KERN_PROF`

```rust
const KERN_PROF: crate::c_int = 6i32;
```

### `KERN_NODENAME`

```rust
const KERN_NODENAME: crate::c_int = 7i32;
```

### `KERN_DOMAINNAME`

```rust
const KERN_DOMAINNAME: crate::c_int = 8i32;
```

### `KERN_PANIC`

```rust
const KERN_PANIC: crate::c_int = 15i32;
```

### `KERN_REALROOTDEV`

```rust
const KERN_REALROOTDEV: crate::c_int = 16i32;
```

### `KERN_SPARC_REBOOT`

```rust
const KERN_SPARC_REBOOT: crate::c_int = 21i32;
```

### `KERN_CTLALTDEL`

```rust
const KERN_CTLALTDEL: crate::c_int = 22i32;
```

### `KERN_PRINTK`

```rust
const KERN_PRINTK: crate::c_int = 23i32;
```

### `KERN_NAMETRANS`

```rust
const KERN_NAMETRANS: crate::c_int = 24i32;
```

### `KERN_PPC_HTABRECLAIM`

```rust
const KERN_PPC_HTABRECLAIM: crate::c_int = 25i32;
```

### `KERN_PPC_ZEROPAGED`

```rust
const KERN_PPC_ZEROPAGED: crate::c_int = 26i32;
```

### `KERN_PPC_POWERSAVE_NAP`

```rust
const KERN_PPC_POWERSAVE_NAP: crate::c_int = 27i32;
```

### `KERN_MODPROBE`

```rust
const KERN_MODPROBE: crate::c_int = 28i32;
```

### `KERN_SG_BIG_BUFF`

```rust
const KERN_SG_BIG_BUFF: crate::c_int = 29i32;
```

### `KERN_ACCT`

```rust
const KERN_ACCT: crate::c_int = 30i32;
```

### `KERN_PPC_L2CR`

```rust
const KERN_PPC_L2CR: crate::c_int = 31i32;
```

### `KERN_RTSIGNR`

```rust
const KERN_RTSIGNR: crate::c_int = 32i32;
```

### `KERN_RTSIGMAX`

```rust
const KERN_RTSIGMAX: crate::c_int = 33i32;
```

### `KERN_SHMMAX`

```rust
const KERN_SHMMAX: crate::c_int = 34i32;
```

### `KERN_MSGMAX`

```rust
const KERN_MSGMAX: crate::c_int = 35i32;
```

### `KERN_MSGMNB`

```rust
const KERN_MSGMNB: crate::c_int = 36i32;
```

### `KERN_MSGPOOL`

```rust
const KERN_MSGPOOL: crate::c_int = 37i32;
```

### `KERN_SYSRQ`

```rust
const KERN_SYSRQ: crate::c_int = 38i32;
```

### `KERN_MAX_THREADS`

```rust
const KERN_MAX_THREADS: crate::c_int = 39i32;
```

### `KERN_RANDOM`

```rust
const KERN_RANDOM: crate::c_int = 40i32;
```

### `KERN_SHMALL`

```rust
const KERN_SHMALL: crate::c_int = 41i32;
```

### `KERN_MSGMNI`

```rust
const KERN_MSGMNI: crate::c_int = 42i32;
```

### `KERN_SEM`

```rust
const KERN_SEM: crate::c_int = 43i32;
```

### `KERN_SPARC_STOP_A`

```rust
const KERN_SPARC_STOP_A: crate::c_int = 44i32;
```

### `KERN_SHMMNI`

```rust
const KERN_SHMMNI: crate::c_int = 45i32;
```

### `KERN_OVERFLOWUID`

```rust
const KERN_OVERFLOWUID: crate::c_int = 46i32;
```

### `KERN_OVERFLOWGID`

```rust
const KERN_OVERFLOWGID: crate::c_int = 47i32;
```

### `KERN_SHMPATH`

```rust
const KERN_SHMPATH: crate::c_int = 48i32;
```

### `KERN_HOTPLUG`

```rust
const KERN_HOTPLUG: crate::c_int = 49i32;
```

### `KERN_IEEE_EMULATION_WARNINGS`

```rust
const KERN_IEEE_EMULATION_WARNINGS: crate::c_int = 50i32;
```

### `KERN_S390_USER_DEBUG_LOGGING`

```rust
const KERN_S390_USER_DEBUG_LOGGING: crate::c_int = 51i32;
```

### `KERN_CORE_USES_PID`

```rust
const KERN_CORE_USES_PID: crate::c_int = 52i32;
```

### `KERN_TAINTED`

```rust
const KERN_TAINTED: crate::c_int = 53i32;
```

### `KERN_CADPID`

```rust
const KERN_CADPID: crate::c_int = 54i32;
```

### `KERN_PIDMAX`

```rust
const KERN_PIDMAX: crate::c_int = 55i32;
```

### `KERN_CORE_PATTERN`

```rust
const KERN_CORE_PATTERN: crate::c_int = 56i32;
```

### `KERN_PANIC_ON_OOPS`

```rust
const KERN_PANIC_ON_OOPS: crate::c_int = 57i32;
```

### `KERN_HPPA_PWRSW`

```rust
const KERN_HPPA_PWRSW: crate::c_int = 58i32;
```

### `KERN_HPPA_UNALIGNED`

```rust
const KERN_HPPA_UNALIGNED: crate::c_int = 59i32;
```

### `KERN_PRINTK_RATELIMIT`

```rust
const KERN_PRINTK_RATELIMIT: crate::c_int = 60i32;
```

### `KERN_PRINTK_RATELIMIT_BURST`

```rust
const KERN_PRINTK_RATELIMIT_BURST: crate::c_int = 61i32;
```

### `KERN_PTY`

```rust
const KERN_PTY: crate::c_int = 62i32;
```

### `KERN_NGROUPS_MAX`

```rust
const KERN_NGROUPS_MAX: crate::c_int = 63i32;
```

### `KERN_SPARC_SCONS_PWROFF`

```rust
const KERN_SPARC_SCONS_PWROFF: crate::c_int = 64i32;
```

### `KERN_HZ_TIMER`

```rust
const KERN_HZ_TIMER: crate::c_int = 65i32;
```

### `KERN_UNKNOWN_NMI_PANIC`

```rust
const KERN_UNKNOWN_NMI_PANIC: crate::c_int = 66i32;
```

### `KERN_BOOTLOADER_TYPE`

```rust
const KERN_BOOTLOADER_TYPE: crate::c_int = 67i32;
```

### `KERN_RANDOMIZE`

```rust
const KERN_RANDOMIZE: crate::c_int = 68i32;
```

### `KERN_SETUID_DUMPABLE`

```rust
const KERN_SETUID_DUMPABLE: crate::c_int = 69i32;
```

### `KERN_SPIN_RETRY`

```rust
const KERN_SPIN_RETRY: crate::c_int = 70i32;
```

### `KERN_ACPI_VIDEO_FLAGS`

```rust
const KERN_ACPI_VIDEO_FLAGS: crate::c_int = 71i32;
```

### `KERN_IA64_UNALIGNED`

```rust
const KERN_IA64_UNALIGNED: crate::c_int = 72i32;
```

### `KERN_COMPAT_LOG`

```rust
const KERN_COMPAT_LOG: crate::c_int = 73i32;
```

### `KERN_MAX_LOCK_DEPTH`

```rust
const KERN_MAX_LOCK_DEPTH: crate::c_int = 74i32;
```

### `KERN_NMI_WATCHDOG`

```rust
const KERN_NMI_WATCHDOG: crate::c_int = 75i32;
```

### `KERN_PANIC_ON_NMI`

```rust
const KERN_PANIC_ON_NMI: crate::c_int = 76i32;
```

### `VM_OVERCOMMIT_MEMORY`

```rust
const VM_OVERCOMMIT_MEMORY: crate::c_int = 5i32;
```

### `VM_PAGE_CLUSTER`

```rust
const VM_PAGE_CLUSTER: crate::c_int = 10i32;
```

### `VM_DIRTY_BACKGROUND`

```rust
const VM_DIRTY_BACKGROUND: crate::c_int = 11i32;
```

### `VM_DIRTY_RATIO`

```rust
const VM_DIRTY_RATIO: crate::c_int = 12i32;
```

### `VM_DIRTY_WB_CS`

```rust
const VM_DIRTY_WB_CS: crate::c_int = 13i32;
```

### `VM_DIRTY_EXPIRE_CS`

```rust
const VM_DIRTY_EXPIRE_CS: crate::c_int = 14i32;
```

### `VM_NR_PDFLUSH_THREADS`

```rust
const VM_NR_PDFLUSH_THREADS: crate::c_int = 15i32;
```

### `VM_OVERCOMMIT_RATIO`

```rust
const VM_OVERCOMMIT_RATIO: crate::c_int = 16i32;
```

### `VM_PAGEBUF`

```rust
const VM_PAGEBUF: crate::c_int = 17i32;
```

### `VM_HUGETLB_PAGES`

```rust
const VM_HUGETLB_PAGES: crate::c_int = 18i32;
```

### `VM_SWAPPINESS`

```rust
const VM_SWAPPINESS: crate::c_int = 19i32;
```

### `VM_LOWMEM_RESERVE_RATIO`

```rust
const VM_LOWMEM_RESERVE_RATIO: crate::c_int = 20i32;
```

### `VM_MIN_FREE_KBYTES`

```rust
const VM_MIN_FREE_KBYTES: crate::c_int = 21i32;
```

### `VM_MAX_MAP_COUNT`

```rust
const VM_MAX_MAP_COUNT: crate::c_int = 22i32;
```

### `VM_LAPTOP_MODE`

```rust
const VM_LAPTOP_MODE: crate::c_int = 23i32;
```

### `VM_BLOCK_DUMP`

```rust
const VM_BLOCK_DUMP: crate::c_int = 24i32;
```

### `VM_HUGETLB_GROUP`

```rust
const VM_HUGETLB_GROUP: crate::c_int = 25i32;
```

### `VM_VFS_CACHE_PRESSURE`

```rust
const VM_VFS_CACHE_PRESSURE: crate::c_int = 26i32;
```

### `VM_LEGACY_VA_LAYOUT`

```rust
const VM_LEGACY_VA_LAYOUT: crate::c_int = 27i32;
```

### `VM_SWAP_TOKEN_TIMEOUT`

```rust
const VM_SWAP_TOKEN_TIMEOUT: crate::c_int = 28i32;
```

### `VM_DROP_PAGECACHE`

```rust
const VM_DROP_PAGECACHE: crate::c_int = 29i32;
```

### `VM_PERCPU_PAGELIST_FRACTION`

```rust
const VM_PERCPU_PAGELIST_FRACTION: crate::c_int = 30i32;
```

### `VM_ZONE_RECLAIM_MODE`

```rust
const VM_ZONE_RECLAIM_MODE: crate::c_int = 31i32;
```

### `VM_MIN_UNMAPPED`

```rust
const VM_MIN_UNMAPPED: crate::c_int = 32i32;
```

### `VM_PANIC_ON_OOM`

```rust
const VM_PANIC_ON_OOM: crate::c_int = 33i32;
```

### `VM_VDSO_ENABLED`

```rust
const VM_VDSO_ENABLED: crate::c_int = 34i32;
```

### `VM_MIN_SLAB`

```rust
const VM_MIN_SLAB: crate::c_int = 35i32;
```

### `NET_CORE`

```rust
const NET_CORE: crate::c_int = 1i32;
```

### `NET_ETHER`

```rust
const NET_ETHER: crate::c_int = 2i32;
```

### `NET_802`

```rust
const NET_802: crate::c_int = 3i32;
```

### `NET_UNIX`

```rust
const NET_UNIX: crate::c_int = 4i32;
```

### `NET_IPV4`

```rust
const NET_IPV4: crate::c_int = 5i32;
```

### `NET_IPX`

```rust
const NET_IPX: crate::c_int = 6i32;
```

### `NET_ATALK`

```rust
const NET_ATALK: crate::c_int = 7i32;
```

### `NET_NETROM`

```rust
const NET_NETROM: crate::c_int = 8i32;
```

### `NET_AX25`

```rust
const NET_AX25: crate::c_int = 9i32;
```

### `NET_BRIDGE`

```rust
const NET_BRIDGE: crate::c_int = 10i32;
```

### `NET_ROSE`

```rust
const NET_ROSE: crate::c_int = 11i32;
```

### `NET_IPV6`

```rust
const NET_IPV6: crate::c_int = 12i32;
```

### `NET_X25`

```rust
const NET_X25: crate::c_int = 13i32;
```

### `NET_TR`

```rust
const NET_TR: crate::c_int = 14i32;
```

### `NET_DECNET`

```rust
const NET_DECNET: crate::c_int = 15i32;
```

### `NET_ECONET`

```rust
const NET_ECONET: crate::c_int = 16i32;
```

### `NET_SCTP`

```rust
const NET_SCTP: crate::c_int = 17i32;
```

### `NET_LLC`

```rust
const NET_LLC: crate::c_int = 18i32;
```

### `NET_NETFILTER`

```rust
const NET_NETFILTER: crate::c_int = 19i32;
```

### `NET_DCCP`

```rust
const NET_DCCP: crate::c_int = 20i32;
```

### `NET_IRDA`

```rust
const NET_IRDA: crate::c_int = 412i32;
```

### `PF_VCPU`

```rust
const PF_VCPU: crate::c_int = 1i32;
```

I'm a virtual CPU.

### `PF_IDLE`

```rust
const PF_IDLE: crate::c_int = 2i32;
```

I am an IDLE thread.

### `PF_EXITING`

```rust
const PF_EXITING: crate::c_int = 4i32;
```

Getting shut down.

### `PF_POSTCOREDUMP`

```rust
const PF_POSTCOREDUMP: crate::c_int = 8i32;
```

Coredumps should ignore this task.

### `PF_IO_WORKER`

```rust
const PF_IO_WORKER: crate::c_int = 16i32;
```

Task is an IO worker.

### `PF_WQ_WORKER`

```rust
const PF_WQ_WORKER: crate::c_int = 32i32;
```

I'm a workqueue worker.

### `PF_FORKNOEXEC`

```rust
const PF_FORKNOEXEC: crate::c_int = 64i32;
```

Forked but didn't exec.

### `PF_MCE_PROCESS`

```rust
const PF_MCE_PROCESS: crate::c_int = 128i32;
```

Process policy on mce errors.

### `PF_SUPERPRIV`

```rust
const PF_SUPERPRIV: crate::c_int = 256i32;
```

Used super-user privileges.

### `PF_DUMPCORE`

```rust
const PF_DUMPCORE: crate::c_int = 512i32;
```

Dumped core.

### `PF_SIGNALED`

```rust
const PF_SIGNALED: crate::c_int = 1_024i32;
```

Killed by a signal.

### `PF_MEMALLOC`

```rust
const PF_MEMALLOC: crate::c_int = 2_048i32;
```

Allocating memory to free memory.

See `memalloc_noreclaim_save()`.

### `PF_NPROC_EXCEEDED`

```rust
const PF_NPROC_EXCEEDED: crate::c_int = 4_096i32;
```

`set_user()` noticed that `RLIMIT_NPROC` was exceeded.

### `PF_USED_MATH`

```rust
const PF_USED_MATH: crate::c_int = 8_192i32;
```

If unset the fpu must be initialized before use.

### `PF_USER_WORKER`

```rust
const PF_USER_WORKER: crate::c_int = 16_384i32;
```

Kernel thread cloned from userspace thread.

### `PF_NOFREEZE`

```rust
const PF_NOFREEZE: crate::c_int = 32_768i32;
```

This thread should not be frozen.

### `PF_KSWAPD`

```rust
const PF_KSWAPD: crate::c_int = 131_072i32;
```

I am `kswapd`.

### `PF_MEMALLOC_NOFS`

```rust
const PF_MEMALLOC_NOFS: crate::c_int = 262_144i32;
```

All allocations inherit `GFP_NOFS`.

See `memalloc_nfs_save()`.

### `PF_MEMALLOC_NOIO`

```rust
const PF_MEMALLOC_NOIO: crate::c_int = 524_288i32;
```

All allocations inherit `GFP_NOIO`.

See `memalloc_noio_save()`.

### `PF_LOCAL_THROTTLE`

```rust
const PF_LOCAL_THROTTLE: crate::c_int = 1_048_576i32;
```

Throttle writes only against the bdi I write to, I am cleaning
dirty pages from some other bdi.

### `PF_KTHREAD`

```rust
const PF_KTHREAD: crate::c_int = 2_097_152i32;
```

I am a kernel thread.

### `PF_RANDOMIZE`

```rust
const PF_RANDOMIZE: crate::c_int = 4_194_304i32;
```

Randomize virtual address space.

### `PF_NO_SETAFFINITY`

```rust
const PF_NO_SETAFFINITY: crate::c_int = 67_108_864i32;
```

Userland is not allowed to meddle with `cpus_mask`.

### `PF_MCE_EARLY`

```rust
const PF_MCE_EARLY: crate::c_int = 134_217_728i32;
```

Early kill for mce process policy.

### `PF_MEMALLOC_PIN`

```rust
const PF_MEMALLOC_PIN: crate::c_int = 268_435_456i32;
```

Allocations constrained to zones which allow long term pinning.

See `memalloc_pin_save()`.

### `PF_BLOCK_TS`

```rust
const PF_BLOCK_TS: crate::c_int = 536_870_912i32;
```

Plug has ts that needs updating.

### `PF_SUSPEND_TASK`

```rust
const PF_SUSPEND_TASK: crate::c_int = -2_147_483_648i32;
```

This thread called `freeze_processes()` and should not be frozen.

### `PF_SUSPEND_TASK_UINT`

```rust
const PF_SUSPEND_TASK_UINT: crate::c_uint = 2_147_483_648u32;
```

### `CSIGNAL`

```rust
const CSIGNAL: crate::c_int = 255i32;
```

### `SCHED_NORMAL`

```rust
const SCHED_NORMAL: crate::c_int = 0i32;
```

### `SCHED_OTHER`

```rust
const SCHED_OTHER: crate::c_int = 0i32;
```

### `SCHED_FIFO`

```rust
const SCHED_FIFO: crate::c_int = 1i32;
```

### `SCHED_RR`

```rust
const SCHED_RR: crate::c_int = 2i32;
```

### `SCHED_BATCH`

```rust
const SCHED_BATCH: crate::c_int = 3i32;
```

### `SCHED_IDLE`

```rust
const SCHED_IDLE: crate::c_int = 5i32;
```

### `SCHED_DEADLINE`

```rust
const SCHED_DEADLINE: crate::c_int = 6i32;
```

### `SCHED_RESET_ON_FORK`

```rust
const SCHED_RESET_ON_FORK: crate::c_int = 1_073_741_824i32;
```

### `CLONE_PIDFD`

```rust
const CLONE_PIDFD: crate::c_int = 4_096i32;
```

### `SCHED_FLAG_RESET_ON_FORK`

```rust
const SCHED_FLAG_RESET_ON_FORK: crate::c_int = 1i32;
```

### `SCHED_FLAG_RECLAIM`

```rust
const SCHED_FLAG_RECLAIM: crate::c_int = 2i32;
```

### `SCHED_FLAG_DL_OVERRUN`

```rust
const SCHED_FLAG_DL_OVERRUN: crate::c_int = 4i32;
```

### `SCHED_FLAG_KEEP_POLICY`

```rust
const SCHED_FLAG_KEEP_POLICY: crate::c_int = 8i32;
```

### `SCHED_FLAG_KEEP_PARAMS`

```rust
const SCHED_FLAG_KEEP_PARAMS: crate::c_int = 16i32;
```

### `SCHED_FLAG_UTIL_CLAMP_MIN`

```rust
const SCHED_FLAG_UTIL_CLAMP_MIN: crate::c_int = 32i32;
```

### `SCHED_FLAG_UTIL_CLAMP_MAX`

```rust
const SCHED_FLAG_UTIL_CLAMP_MAX: crate::c_int = 64i32;
```

### `XDP_SHARED_UMEM`

```rust
const XDP_SHARED_UMEM: crate::__u16 = 1u16;
```

### `XDP_COPY`

```rust
const XDP_COPY: crate::__u16 = 2u16;
```

### `XDP_ZEROCOPY`

```rust
const XDP_ZEROCOPY: crate::__u16 = 4u16;
```

### `XDP_USE_NEED_WAKEUP`

```rust
const XDP_USE_NEED_WAKEUP: crate::__u16 = 8u16;
```

### `XDP_USE_SG`

```rust
const XDP_USE_SG: crate::__u16 = 16u16;
```

### `XDP_UMEM_UNALIGNED_CHUNK_FLAG`

```rust
const XDP_UMEM_UNALIGNED_CHUNK_FLAG: crate::__u32 = 1u32;
```

### `XDP_RING_NEED_WAKEUP`

```rust
const XDP_RING_NEED_WAKEUP: crate::__u32 = 1u32;
```

### `XDP_MMAP_OFFSETS`

```rust
const XDP_MMAP_OFFSETS: crate::c_int = 1i32;
```

### `XDP_RX_RING`

```rust
const XDP_RX_RING: crate::c_int = 2i32;
```

### `XDP_TX_RING`

```rust
const XDP_TX_RING: crate::c_int = 3i32;
```

### `XDP_UMEM_REG`

```rust
const XDP_UMEM_REG: crate::c_int = 4i32;
```

### `XDP_UMEM_FILL_RING`

```rust
const XDP_UMEM_FILL_RING: crate::c_int = 5i32;
```

### `XDP_UMEM_COMPLETION_RING`

```rust
const XDP_UMEM_COMPLETION_RING: crate::c_int = 6i32;
```

### `XDP_STATISTICS`

```rust
const XDP_STATISTICS: crate::c_int = 7i32;
```

### `XDP_OPTIONS`

```rust
const XDP_OPTIONS: crate::c_int = 8i32;
```

### `XDP_OPTIONS_ZEROCOPY`

```rust
const XDP_OPTIONS_ZEROCOPY: crate::__u32 = 1u32;
```

### `XDP_PGOFF_RX_RING`

```rust
const XDP_PGOFF_RX_RING: crate::off_t = 0i64;
```

### `XDP_PGOFF_TX_RING`

```rust
const XDP_PGOFF_TX_RING: crate::off_t = 2_147_483_648i64;
```

### `XDP_UMEM_PGOFF_FILL_RING`

```rust
const XDP_UMEM_PGOFF_FILL_RING: crate::c_ulonglong = 4_294_967_296u64;
```

### `XDP_UMEM_PGOFF_COMPLETION_RING`

```rust
const XDP_UMEM_PGOFF_COMPLETION_RING: crate::c_ulonglong = 6_442_450_944u64;
```

### `XSK_UNALIGNED_BUF_OFFSET_SHIFT`

```rust
const XSK_UNALIGNED_BUF_OFFSET_SHIFT: crate::c_int = 48i32;
```

### `XSK_UNALIGNED_BUF_ADDR_MASK`

```rust
const XSK_UNALIGNED_BUF_ADDR_MASK: crate::c_ulonglong = 281_474_976_710_655u64;
```

### `XDP_PKT_CONTD`

```rust
const XDP_PKT_CONTD: crate::__u32 = 1u32;
```

### `XDP_UMEM_TX_SW_CSUM`

```rust
const XDP_UMEM_TX_SW_CSUM: crate::__u32 = 2u32;
```

### `XDP_UMEM_TX_METADATA_LEN`

```rust
const XDP_UMEM_TX_METADATA_LEN: crate::__u32 = 4u32;
```

### `XDP_TXMD_FLAGS_TIMESTAMP`

```rust
const XDP_TXMD_FLAGS_TIMESTAMP: crate::__u32 = 1u32;
```

### `XDP_TXMD_FLAGS_CHECKSUM`

```rust
const XDP_TXMD_FLAGS_CHECKSUM: crate::__u32 = 2u32;
```

### `XDP_TX_METADATA`

```rust
const XDP_TX_METADATA: crate::__u32 = 2u32;
```

### `SOL_XDP`

```rust
const SOL_XDP: crate::c_int = 283i32;
```

### `MOUNT_ATTR_RDONLY`

```rust
const MOUNT_ATTR_RDONLY: crate::__u64 = 1u64;
```

### `MOUNT_ATTR_NOSUID`

```rust
const MOUNT_ATTR_NOSUID: crate::__u64 = 2u64;
```

### `MOUNT_ATTR_NODEV`

```rust
const MOUNT_ATTR_NODEV: crate::__u64 = 4u64;
```

### `MOUNT_ATTR_NOEXEC`

```rust
const MOUNT_ATTR_NOEXEC: crate::__u64 = 8u64;
```

### `MOUNT_ATTR__ATIME`

```rust
const MOUNT_ATTR__ATIME: crate::__u64 = 112u64;
```

### `MOUNT_ATTR_RELATIME`

```rust
const MOUNT_ATTR_RELATIME: crate::__u64 = 0u64;
```

### `MOUNT_ATTR_NOATIME`

```rust
const MOUNT_ATTR_NOATIME: crate::__u64 = 16u64;
```

### `MOUNT_ATTR_STRICTATIME`

```rust
const MOUNT_ATTR_STRICTATIME: crate::__u64 = 32u64;
```

### `MOUNT_ATTR_NODIRATIME`

```rust
const MOUNT_ATTR_NODIRATIME: crate::__u64 = 128u64;
```

### `MOUNT_ATTR_IDMAP`

```rust
const MOUNT_ATTR_IDMAP: crate::__u64 = 1_048_576u64;
```

### `MOUNT_ATTR_NOSYMFOLLOW`

```rust
const MOUNT_ATTR_NOSYMFOLLOW: crate::__u64 = 2_097_152u64;
```

### `MOUNT_ATTR_SIZE_VER0`

```rust
const MOUNT_ATTR_SIZE_VER0: crate::c_int = 32i32;
```

### `NT_PRSTATUS`

```rust
const NT_PRSTATUS: crate::c_int = 1i32;
```

### `NT_PRFPREG`

```rust
const NT_PRFPREG: crate::c_int = 2i32;
```

### `NT_FPREGSET`

```rust
const NT_FPREGSET: crate::c_int = 2i32;
```

### `NT_PRPSINFO`

```rust
const NT_PRPSINFO: crate::c_int = 3i32;
```

### `NT_PRXREG`

```rust
const NT_PRXREG: crate::c_int = 4i32;
```

### `NT_TASKSTRUCT`

```rust
const NT_TASKSTRUCT: crate::c_int = 4i32;
```

### `NT_PLATFORM`

```rust
const NT_PLATFORM: crate::c_int = 5i32;
```

### `NT_AUXV`

```rust
const NT_AUXV: crate::c_int = 6i32;
```

### `NT_GWINDOWS`

```rust
const NT_GWINDOWS: crate::c_int = 7i32;
```

### `NT_ASRS`

```rust
const NT_ASRS: crate::c_int = 8i32;
```

### `NT_PSTATUS`

```rust
const NT_PSTATUS: crate::c_int = 10i32;
```

### `NT_PSINFO`

```rust
const NT_PSINFO: crate::c_int = 13i32;
```

### `NT_PRCRED`

```rust
const NT_PRCRED: crate::c_int = 14i32;
```

### `NT_UTSNAME`

```rust
const NT_UTSNAME: crate::c_int = 15i32;
```

### `NT_LWPSTATUS`

```rust
const NT_LWPSTATUS: crate::c_int = 16i32;
```

### `NT_LWPSINFO`

```rust
const NT_LWPSINFO: crate::c_int = 17i32;
```

### `NT_PRFPXREG`

```rust
const NT_PRFPXREG: crate::c_int = 20i32;
```

### `SCHED_FLAG_KEEP_ALL`

```rust
const SCHED_FLAG_KEEP_ALL: crate::c_int = 24i32;
```

### `SCHED_FLAG_UTIL_CLAMP`

```rust
const SCHED_FLAG_UTIL_CLAMP: crate::c_int = 96i32;
```

### `SCHED_FLAG_ALL`

```rust
const SCHED_FLAG_ALL: crate::c_int = 127i32;
```

### `EPIOCSPARAMS`

```rust
const EPIOCSPARAMS: crate::c_ulong = 1_074_301_441u64;
```

### `EPIOCGPARAMS`

```rust
const EPIOCGPARAMS: crate::c_ulong = 2_148_043_266u64;
```

### `SI_DETHREAD`

```rust
const SI_DETHREAD: crate::c_int = -7i32;
```

### `TRAP_PERF`

```rust
const TRAP_PERF: crate::c_int = 6i32;
```

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

