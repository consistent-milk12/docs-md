*[rustix](../../index.md) / [backend](../index.md) / [c](index.md)*

---

# Module `c`

Adapt the Linux API to resemble a POSIX-style libc API.

The linux_raw backend doesn't use actual libc; this just defines certain
types that are convenient to have defined.

## Type Aliases

### `size_t`

```rust
type size_t = usize;
```

## Constants

### `SIGHUP`

```rust
const SIGHUP: c_int = 1i32;
```

### `SIGINT`

```rust
const SIGINT: c_int = 2i32;
```

### `SIGQUIT`

```rust
const SIGQUIT: c_int = 3i32;
```

### `SIGILL`

```rust
const SIGILL: c_int = 4i32;
```

### `SIGTRAP`

```rust
const SIGTRAP: c_int = 5i32;
```

### `SIGABRT`

```rust
const SIGABRT: c_int = 6i32;
```

### `SIGBUS`

```rust
const SIGBUS: c_int = 7i32;
```

### `SIGFPE`

```rust
const SIGFPE: c_int = 8i32;
```

### `SIGKILL`

```rust
const SIGKILL: c_int = 9i32;
```

### `SIGUSR1`

```rust
const SIGUSR1: c_int = 10i32;
```

### `SIGSEGV`

```rust
const SIGSEGV: c_int = 11i32;
```

### `SIGUSR2`

```rust
const SIGUSR2: c_int = 12i32;
```

### `SIGPIPE`

```rust
const SIGPIPE: c_int = 13i32;
```

### `SIGALRM`

```rust
const SIGALRM: c_int = 14i32;
```

### `SIGTERM`

```rust
const SIGTERM: c_int = 15i32;
```

### `SIGSTKFLT`

```rust
const SIGSTKFLT: c_int = 16i32;
```

### `SIGCHLD`

```rust
const SIGCHLD: c_int = 17i32;
```

### `SIGCONT`

```rust
const SIGCONT: c_int = 18i32;
```

### `SIGSTOP`

```rust
const SIGSTOP: c_int = 19i32;
```

### `SIGTSTP`

```rust
const SIGTSTP: c_int = 20i32;
```

### `SIGTTIN`

```rust
const SIGTTIN: c_int = 21i32;
```

### `SIGTTOU`

```rust
const SIGTTOU: c_int = 22i32;
```

### `SIGURG`

```rust
const SIGURG: c_int = 23i32;
```

### `SIGXCPU`

```rust
const SIGXCPU: c_int = 24i32;
```

### `SIGXFSZ`

```rust
const SIGXFSZ: c_int = 25i32;
```

### `SIGVTALRM`

```rust
const SIGVTALRM: c_int = 26i32;
```

### `SIGPROF`

```rust
const SIGPROF: c_int = 27i32;
```

### `SIGWINCH`

```rust
const SIGWINCH: c_int = 28i32;
```

### `SIGIO`

```rust
const SIGIO: c_int = 29i32;
```

### `SIGPWR`

```rust
const SIGPWR: c_int = 30i32;
```

### `SIGSYS`

```rust
const SIGSYS: c_int = 31i32;
```

### `PIPE_BUF`

```rust
const PIPE_BUF: usize = 4_096usize;
```

### `CLOCK_MONOTONIC`

```rust
const CLOCK_MONOTONIC: c_int = 1i32;
```

### `CLOCK_REALTIME`

```rust
const CLOCK_REALTIME: c_int = 0i32;
```

### `CLOCK_MONOTONIC_RAW`

```rust
const CLOCK_MONOTONIC_RAW: c_int = 4i32;
```

### `CLOCK_MONOTONIC_COARSE`

```rust
const CLOCK_MONOTONIC_COARSE: c_int = 6i32;
```

### `CLOCK_REALTIME_COARSE`

```rust
const CLOCK_REALTIME_COARSE: c_int = 5i32;
```

### `CLOCK_THREAD_CPUTIME_ID`

```rust
const CLOCK_THREAD_CPUTIME_ID: c_int = 3i32;
```

### `CLOCK_PROCESS_CPUTIME_ID`

```rust
const CLOCK_PROCESS_CPUTIME_ID: c_int = 2i32;
```

