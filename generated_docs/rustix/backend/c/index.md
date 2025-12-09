*[rustix](../../index.md) / [backend](../index.md) / [c](index.md)*

---

# Module `c`

Adapt the Linux API to resemble a POSIX-style libc API.

The linux_raw backend doesn't use actual libc; this just defines certain
types that are convenient to have defined.

## Contents

- [Type Aliases](#type-aliases)
  - [`size_t`](#size_t)
- [Constants](#constants)
  - [`SIGHUP`](#sighup)
  - [`SIGINT`](#sigint)
  - [`SIGQUIT`](#sigquit)
  - [`SIGILL`](#sigill)
  - [`SIGTRAP`](#sigtrap)
  - [`SIGABRT`](#sigabrt)
  - [`SIGBUS`](#sigbus)
  - [`SIGFPE`](#sigfpe)
  - [`SIGKILL`](#sigkill)
  - [`SIGUSR1`](#sigusr1)
  - [`SIGSEGV`](#sigsegv)
  - [`SIGUSR2`](#sigusr2)
  - [`SIGPIPE`](#sigpipe)
  - [`SIGALRM`](#sigalrm)
  - [`SIGTERM`](#sigterm)
  - [`SIGSTKFLT`](#sigstkflt)
  - [`SIGCHLD`](#sigchld)
  - [`SIGCONT`](#sigcont)
  - [`SIGSTOP`](#sigstop)
  - [`SIGTSTP`](#sigtstp)
  - [`SIGTTIN`](#sigttin)
  - [`SIGTTOU`](#sigttou)
  - [`SIGURG`](#sigurg)
  - [`SIGXCPU`](#sigxcpu)
  - [`SIGXFSZ`](#sigxfsz)
  - [`SIGVTALRM`](#sigvtalrm)
  - [`SIGPROF`](#sigprof)
  - [`SIGWINCH`](#sigwinch)
  - [`SIGIO`](#sigio)
  - [`SIGPWR`](#sigpwr)
  - [`SIGSYS`](#sigsys)
  - [`PIPE_BUF`](#pipe_buf)
  - [`CLOCK_MONOTONIC`](#clock_monotonic)
  - [`CLOCK_REALTIME`](#clock_realtime)
  - [`CLOCK_MONOTONIC_RAW`](#clock_monotonic_raw)
  - [`CLOCK_MONOTONIC_COARSE`](#clock_monotonic_coarse)
  - [`CLOCK_REALTIME_COARSE`](#clock_realtime_coarse)
  - [`CLOCK_THREAD_CPUTIME_ID`](#clock_thread_cputime_id)
  - [`CLOCK_PROCESS_CPUTIME_ID`](#clock_process_cputime_id)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`size_t`](#size_t) | type |  |
| [`SIGHUP`](#sighup) | const |  |
| [`SIGINT`](#sigint) | const |  |
| [`SIGQUIT`](#sigquit) | const |  |
| [`SIGILL`](#sigill) | const |  |
| [`SIGTRAP`](#sigtrap) | const |  |
| [`SIGABRT`](#sigabrt) | const |  |
| [`SIGBUS`](#sigbus) | const |  |
| [`SIGFPE`](#sigfpe) | const |  |
| [`SIGKILL`](#sigkill) | const |  |
| [`SIGUSR1`](#sigusr1) | const |  |
| [`SIGSEGV`](#sigsegv) | const |  |
| [`SIGUSR2`](#sigusr2) | const |  |
| [`SIGPIPE`](#sigpipe) | const |  |
| [`SIGALRM`](#sigalrm) | const |  |
| [`SIGTERM`](#sigterm) | const |  |
| [`SIGSTKFLT`](#sigstkflt) | const |  |
| [`SIGCHLD`](#sigchld) | const |  |
| [`SIGCONT`](#sigcont) | const |  |
| [`SIGSTOP`](#sigstop) | const |  |
| [`SIGTSTP`](#sigtstp) | const |  |
| [`SIGTTIN`](#sigttin) | const |  |
| [`SIGTTOU`](#sigttou) | const |  |
| [`SIGURG`](#sigurg) | const |  |
| [`SIGXCPU`](#sigxcpu) | const |  |
| [`SIGXFSZ`](#sigxfsz) | const |  |
| [`SIGVTALRM`](#sigvtalrm) | const |  |
| [`SIGPROF`](#sigprof) | const |  |
| [`SIGWINCH`](#sigwinch) | const |  |
| [`SIGIO`](#sigio) | const |  |
| [`SIGPWR`](#sigpwr) | const |  |
| [`SIGSYS`](#sigsys) | const |  |
| [`PIPE_BUF`](#pipe_buf) | const |  |
| [`CLOCK_MONOTONIC`](#clock_monotonic) | const |  |
| [`CLOCK_REALTIME`](#clock_realtime) | const |  |
| [`CLOCK_MONOTONIC_RAW`](#clock_monotonic_raw) | const |  |
| [`CLOCK_MONOTONIC_COARSE`](#clock_monotonic_coarse) | const |  |
| [`CLOCK_REALTIME_COARSE`](#clock_realtime_coarse) | const |  |
| [`CLOCK_THREAD_CPUTIME_ID`](#clock_thread_cputime_id) | const |  |
| [`CLOCK_PROCESS_CPUTIME_ID`](#clock_process_cputime_id) | const |  |

## Type Aliases

### `size_t`

```rust
type size_t = usize;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:9`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L9)*

## Constants

### `SIGHUP`
```rust
const SIGHUP: c_int = 1i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:256`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L256)*

### `SIGINT`
```rust
const SIGINT: c_int = 2i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:257`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L257)*

### `SIGQUIT`
```rust
const SIGQUIT: c_int = 3i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:258`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L258)*

### `SIGILL`
```rust
const SIGILL: c_int = 4i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:259`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L259)*

### `SIGTRAP`
```rust
const SIGTRAP: c_int = 5i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:260`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L260)*

### `SIGABRT`
```rust
const SIGABRT: c_int = 6i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:261`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L261)*

### `SIGBUS`
```rust
const SIGBUS: c_int = 7i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:262`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L262)*

### `SIGFPE`
```rust
const SIGFPE: c_int = 8i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:263`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L263)*

### `SIGKILL`
```rust
const SIGKILL: c_int = 9i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:264`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L264)*

### `SIGUSR1`
```rust
const SIGUSR1: c_int = 10i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:265`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L265)*

### `SIGSEGV`
```rust
const SIGSEGV: c_int = 11i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:266`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L266)*

### `SIGUSR2`
```rust
const SIGUSR2: c_int = 12i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:267`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L267)*

### `SIGPIPE`
```rust
const SIGPIPE: c_int = 13i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:268`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L268)*

### `SIGALRM`
```rust
const SIGALRM: c_int = 14i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:269`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L269)*

### `SIGTERM`
```rust
const SIGTERM: c_int = 15i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:270`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L270)*

### `SIGSTKFLT`
```rust
const SIGSTKFLT: c_int = 16i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:279`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L279)*

### `SIGCHLD`
```rust
const SIGCHLD: c_int = 17i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:280`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L280)*

### `SIGCONT`
```rust
const SIGCONT: c_int = 18i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:281`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L281)*

### `SIGSTOP`
```rust
const SIGSTOP: c_int = 19i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:282`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L282)*

### `SIGTSTP`
```rust
const SIGTSTP: c_int = 20i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:283`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L283)*

### `SIGTTIN`
```rust
const SIGTTIN: c_int = 21i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:284`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L284)*

### `SIGTTOU`
```rust
const SIGTTOU: c_int = 22i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:285`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L285)*

### `SIGURG`
```rust
const SIGURG: c_int = 23i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:286`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L286)*

### `SIGXCPU`
```rust
const SIGXCPU: c_int = 24i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:287`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L287)*

### `SIGXFSZ`
```rust
const SIGXFSZ: c_int = 25i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:288`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L288)*

### `SIGVTALRM`
```rust
const SIGVTALRM: c_int = 26i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:289`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L289)*

### `SIGPROF`
```rust
const SIGPROF: c_int = 27i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:290`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L290)*

### `SIGWINCH`
```rust
const SIGWINCH: c_int = 28i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:291`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L291)*

### `SIGIO`
```rust
const SIGIO: c_int = 29i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:292`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L292)*

### `SIGPWR`
```rust
const SIGPWR: c_int = 30i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:293`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L293)*

### `SIGSYS`
```rust
const SIGSYS: c_int = 31i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:294`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L294)*

### `PIPE_BUF`
```rust
const PIPE_BUF: usize = 4_096usize;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:312`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L312)*

### `CLOCK_MONOTONIC`
```rust
const CLOCK_MONOTONIC: c_int = 1i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:314`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L314)*

### `CLOCK_REALTIME`
```rust
const CLOCK_REALTIME: c_int = 0i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:315`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L315)*

### `CLOCK_MONOTONIC_RAW`
```rust
const CLOCK_MONOTONIC_RAW: c_int = 4i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:316`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L316)*

### `CLOCK_MONOTONIC_COARSE`
```rust
const CLOCK_MONOTONIC_COARSE: c_int = 6i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:317-318`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L317-L318)*

### `CLOCK_REALTIME_COARSE`
```rust
const CLOCK_REALTIME_COARSE: c_int = 5i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:319`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L319)*

### `CLOCK_THREAD_CPUTIME_ID`
```rust
const CLOCK_THREAD_CPUTIME_ID: c_int = 3i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:320-321`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L320-L321)*

### `CLOCK_PROCESS_CPUTIME_ID`
```rust
const CLOCK_PROCESS_CPUTIME_ID: c_int = 2i32;
```

*Defined in [`rustix-1.1.2/src/backend/linux_raw/c.rs:322-323`](../../../../.source_1765210505/rustix-1.1.2/src/backend/linux_raw/c.rs#L322-L323)*

