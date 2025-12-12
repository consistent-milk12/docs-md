*[libc](../../../../../index.md) / [unix](../../../../index.md) / [linux_like](../../../index.md) / [linux](../../index.md) / [gnu](../index.md) / [b64](index.md)*

---

# Module `b64`

64-bit specific definitions for linux-like values

## Contents

- [Modules](#modules)
  - [`x86_64`](#x86-64)
  - [`not_x32`](#not-x32)
- [Structs](#structs)
  - [`sigset_t`](#sigset-t)
  - [`sysinfo`](#sysinfo)
  - [`msqid_ds`](#msqid-ds)
  - [`semid_ds`](#semid-ds)
  - [`timex`](#timex)
  - [`sigaction`](#sigaction)
  - [`statfs`](#statfs)
  - [`flock`](#flock)
  - [`flock64`](#flock64)
  - [`siginfo_t`](#siginfo-t)
  - [`stack_t`](#stack-t)
  - [`stat`](#stat)
  - [`stat64`](#stat64)
  - [`statfs64`](#statfs64)
  - [`statvfs64`](#statvfs64)
  - [`pthread_attr_t`](#pthread-attr-t)
  - [`_libc_fpxreg`](#libc-fpxreg)
  - [`_libc_xmmreg`](#libc-xmmreg)
  - [`_libc_fpstate`](#libc-fpstate)
  - [`user_regs_struct`](#user-regs-struct)
  - [`user`](#user)
  - [`mcontext_t`](#mcontext-t)
  - [`ipc_perm`](#ipc-perm)
  - [`shmid_ds`](#shmid-ds)
  - [`ptrace_rseq_configuration`](#ptrace-rseq-configuration)
  - [`clone_args`](#clone-args)
  - [`user_fpregs_struct`](#user-fpregs-struct)
  - [`ucontext_t`](#ucontext-t)
  - [`max_align_t`](#max-align-t)
- [Functions](#functions)
  - [`getcontext`](#getcontext)
  - [`setcontext`](#setcontext)
  - [`makecontext`](#makecontext)
  - [`swapcontext`](#swapcontext)
- [Type Aliases](#type-aliases)
  - [`ino_t`](#ino-t)
  - [`off_t`](#off-t)
  - [`blkcnt_t`](#blkcnt-t)
  - [`shmatt_t`](#shmatt-t)
  - [`msgqnum_t`](#msgqnum-t)
  - [`msglen_t`](#msglen-t)
  - [`fsblkcnt_t`](#fsblkcnt-t)
  - [`fsfilcnt_t`](#fsfilcnt-t)
  - [`rlim_t`](#rlim-t)
  - [`__syscall_ulong_t`](#syscall-ulong-t)
  - [`__fsword_t`](#fsword-t)
  - [`clock_t`](#clock-t)
  - [`time_t`](#time-t)
  - [`wchar_t`](#wchar-t)
  - [`nlink_t`](#nlink-t)
  - [`blksize_t`](#blksize-t)
  - [`greg_t`](#greg-t)
  - [`suseconds_t`](#suseconds-t)
  - [`__u64`](#u64)
  - [`__s64`](#s64)
- [Constants](#constants)
  - [`__SIZEOF_PTHREAD_RWLOCKATTR_T`](#sizeof-pthread-rwlockattr-t)
  - [`O_LARGEFILE`](#o-largefile)
  - [`POSIX_FADV_DONTNEED`](#posix-fadv-dontneed)
  - [`POSIX_FADV_NOREUSE`](#posix-fadv-noreuse)
  - [`VEOF`](#veof)
  - [`RTLD_DEEPBIND`](#rtld-deepbind)
  - [`RTLD_GLOBAL`](#rtld-global)
  - [`RTLD_NOLOAD`](#rtld-noload)
  - [`O_APPEND`](#o-append)
  - [`O_CREAT`](#o-creat)
  - [`O_EXCL`](#o-excl)
  - [`O_NOCTTY`](#o-noctty)
  - [`O_NONBLOCK`](#o-nonblock)
  - [`O_SYNC`](#o-sync)
  - [`O_RSYNC`](#o-rsync)
  - [`O_DSYNC`](#o-dsync)
  - [`O_FSYNC`](#o-fsync)
  - [`O_NOATIME`](#o-noatime)
  - [`O_PATH`](#o-path)
  - [`O_TMPFILE`](#o-tmpfile)
  - [`MADV_SOFT_OFFLINE`](#madv-soft-offline)
  - [`MAP_GROWSDOWN`](#map-growsdown)
  - [`EDEADLK`](#edeadlk)
  - [`ENAMETOOLONG`](#enametoolong)
  - [`ENOLCK`](#enolck)
  - [`ENOSYS`](#enosys)
  - [`ENOTEMPTY`](#enotempty)
  - [`ELOOP`](#eloop)
  - [`ENOMSG`](#enomsg)
  - [`EIDRM`](#eidrm)
  - [`ECHRNG`](#echrng)
  - [`EL2NSYNC`](#el2nsync)
  - [`EL3HLT`](#el3hlt)
  - [`EL3RST`](#el3rst)
  - [`ELNRNG`](#elnrng)
  - [`EUNATCH`](#eunatch)
  - [`ENOCSI`](#enocsi)
  - [`EL2HLT`](#el2hlt)
  - [`EBADE`](#ebade)
  - [`EBADR`](#ebadr)
  - [`EXFULL`](#exfull)
  - [`ENOANO`](#enoano)
  - [`EBADRQC`](#ebadrqc)
  - [`EBADSLT`](#ebadslt)
  - [`EMULTIHOP`](#emultihop)
  - [`EOVERFLOW`](#eoverflow)
  - [`ENOTUNIQ`](#enotuniq)
  - [`EBADFD`](#ebadfd)
  - [`EBADMSG`](#ebadmsg)
  - [`EREMCHG`](#eremchg)
  - [`ELIBACC`](#elibacc)
  - [`ELIBBAD`](#elibbad)
  - [`ELIBSCN`](#elibscn)
  - [`ELIBMAX`](#elibmax)
  - [`ELIBEXEC`](#elibexec)
  - [`EILSEQ`](#eilseq)
  - [`ERESTART`](#erestart)
  - [`ESTRPIPE`](#estrpipe)
  - [`EUSERS`](#eusers)
  - [`ENOTSOCK`](#enotsock)
  - [`EDESTADDRREQ`](#edestaddrreq)
  - [`EMSGSIZE`](#emsgsize)
  - [`EPROTOTYPE`](#eprototype)
  - [`ENOPROTOOPT`](#enoprotoopt)
  - [`EPROTONOSUPPORT`](#eprotonosupport)
  - [`ESOCKTNOSUPPORT`](#esocktnosupport)
  - [`EOPNOTSUPP`](#eopnotsupp)
  - [`EPFNOSUPPORT`](#epfnosupport)
  - [`EAFNOSUPPORT`](#eafnosupport)
  - [`EADDRINUSE`](#eaddrinuse)
  - [`EADDRNOTAVAIL`](#eaddrnotavail)
  - [`ENETDOWN`](#enetdown)
  - [`ENETUNREACH`](#enetunreach)
  - [`ENETRESET`](#enetreset)
  - [`ECONNABORTED`](#econnaborted)
  - [`ECONNRESET`](#econnreset)
  - [`ENOBUFS`](#enobufs)
  - [`EISCONN`](#eisconn)
  - [`ENOTCONN`](#enotconn)
  - [`ESHUTDOWN`](#eshutdown)
  - [`ETOOMANYREFS`](#etoomanyrefs)
  - [`ETIMEDOUT`](#etimedout)
  - [`ECONNREFUSED`](#econnrefused)
  - [`EHOSTDOWN`](#ehostdown)
  - [`EHOSTUNREACH`](#ehostunreach)
  - [`EALREADY`](#ealready)
  - [`EINPROGRESS`](#einprogress)
  - [`ESTALE`](#estale)
  - [`EDQUOT`](#edquot)
  - [`ENOMEDIUM`](#enomedium)
  - [`EMEDIUMTYPE`](#emediumtype)
  - [`ECANCELED`](#ecanceled)
  - [`ENOKEY`](#enokey)
  - [`EKEYEXPIRED`](#ekeyexpired)
  - [`EKEYREVOKED`](#ekeyrevoked)
  - [`EKEYREJECTED`](#ekeyrejected)
  - [`EOWNERDEAD`](#eownerdead)
  - [`ENOTRECOVERABLE`](#enotrecoverable)
  - [`EHWPOISON`](#ehwpoison)
  - [`ERFKILL`](#erfkill)
  - [`SOCK_STREAM`](#sock-stream)
  - [`SOCK_DGRAM`](#sock-dgram)
  - [`SA_ONSTACK`](#sa-onstack)
  - [`SA_SIGINFO`](#sa-siginfo)
  - [`SA_NOCLDWAIT`](#sa-nocldwait)
  - [`SIGTTIN`](#sigttin)
  - [`SIGTTOU`](#sigttou)
  - [`SIGXCPU`](#sigxcpu)
  - [`SIGXFSZ`](#sigxfsz)
  - [`SIGVTALRM`](#sigvtalrm)
  - [`SIGPROF`](#sigprof)
  - [`SIGWINCH`](#sigwinch)
  - [`SIGCHLD`](#sigchld)
  - [`SIGBUS`](#sigbus)
  - [`SIGUSR1`](#sigusr1)
  - [`SIGUSR2`](#sigusr2)
  - [`SIGCONT`](#sigcont)
  - [`SIGSTOP`](#sigstop)
  - [`SIGTSTP`](#sigtstp)
  - [`SIGURG`](#sigurg)
  - [`SIGIO`](#sigio)
  - [`SIGSYS`](#sigsys)
  - [`SIGSTKFLT`](#sigstkflt)
  - [`SIGUNUSED`](#sigunused)
  - [`SIGPOLL`](#sigpoll)
  - [`SIGPWR`](#sigpwr)
  - [`SIG_SETMASK`](#sig-setmask)
  - [`SIG_BLOCK`](#sig-block)
  - [`SIG_UNBLOCK`](#sig-unblock)
  - [`POLLWRNORM`](#pollwrnorm)
  - [`POLLWRBAND`](#pollwrband)
  - [`O_ASYNC`](#o-async)
  - [`O_NDELAY`](#o-ndelay)
  - [`PTRACE_DETACH`](#ptrace-detach)
  - [`PTRACE_GET_RSEQ_CONFIGURATION`](#ptrace-get-rseq-configuration)
  - [`EFD_NONBLOCK`](#efd-nonblock)
  - [`F_GETLK`](#f-getlk)
  - [`F_GETOWN`](#f-getown)
  - [`F_SETOWN`](#f-setown)
  - [`F_SETLK`](#f-setlk)
  - [`F_SETLKW`](#f-setlkw)
  - [`F_OFD_GETLK`](#f-ofd-getlk)
  - [`F_OFD_SETLK`](#f-ofd-setlk)
  - [`F_OFD_SETLKW`](#f-ofd-setlkw)
  - [`F_RDLCK`](#f-rdlck)
  - [`F_WRLCK`](#f-wrlck)
  - [`F_UNLCK`](#f-unlck)
  - [`SFD_NONBLOCK`](#sfd-nonblock)
  - [`TCSANOW`](#tcsanow)
  - [`TCSADRAIN`](#tcsadrain)
  - [`TCSAFLUSH`](#tcsaflush)
  - [`SFD_CLOEXEC`](#sfd-cloexec)
  - [`NCCS`](#nccs)
  - [`O_TRUNC`](#o-trunc)
  - [`O_CLOEXEC`](#o-cloexec)
  - [`EBFONT`](#ebfont)
  - [`ENOSTR`](#enostr)
  - [`ENODATA`](#enodata)
  - [`ETIME`](#etime)
  - [`ENOSR`](#enosr)
  - [`ENONET`](#enonet)
  - [`ENOPKG`](#enopkg)
  - [`EREMOTE`](#eremote)
  - [`ENOLINK`](#enolink)
  - [`EADV`](#eadv)
  - [`ESRMNT`](#esrmnt)
  - [`ECOMM`](#ecomm)
  - [`EPROTO`](#eproto)
  - [`EDOTDOT`](#edotdot)
  - [`SA_NODEFER`](#sa-nodefer)
  - [`SA_RESETHAND`](#sa-resethand)
  - [`SA_RESTART`](#sa-restart)
  - [`SA_NOCLDSTOP`](#sa-nocldstop)
  - [`EPOLL_CLOEXEC`](#epoll-cloexec)
  - [`EFD_CLOEXEC`](#efd-cloexec)
  - [`__SIZEOF_PTHREAD_CONDATTR_T`](#sizeof-pthread-condattr-t)
  - [`__SIZEOF_PTHREAD_MUTEXATTR_T`](#sizeof-pthread-mutexattr-t)
  - [`__SIZEOF_PTHREAD_BARRIERATTR_T`](#sizeof-pthread-barrierattr-t)
  - [`O_DIRECT`](#o-direct)
  - [`O_DIRECTORY`](#o-directory)
  - [`O_NOFOLLOW`](#o-nofollow)
  - [`MAP_HUGETLB`](#map-hugetlb)
  - [`MAP_LOCKED`](#map-locked)
  - [`MAP_NORESERVE`](#map-noreserve)
  - [`MAP_32BIT`](#map-32bit)
  - [`MAP_ANON`](#map-anon)
  - [`MAP_ANONYMOUS`](#map-anonymous)
  - [`MAP_DENYWRITE`](#map-denywrite)
  - [`MAP_EXECUTABLE`](#map-executable)
  - [`MAP_POPULATE`](#map-populate)
  - [`MAP_NONBLOCK`](#map-nonblock)
  - [`MAP_STACK`](#map-stack)
  - [`MAP_SYNC`](#map-sync)
  - [`EDEADLOCK`](#edeadlock)
  - [`EUCLEAN`](#euclean)
  - [`ENOTNAM`](#enotnam)
  - [`ENAVAIL`](#enavail)
  - [`EISNAM`](#eisnam)
  - [`EREMOTEIO`](#eremoteio)
  - [`PTRACE_GETFPREGS`](#ptrace-getfpregs)
  - [`PTRACE_SETFPREGS`](#ptrace-setfpregs)
  - [`PTRACE_GETFPXREGS`](#ptrace-getfpxregs)
  - [`PTRACE_SETFPXREGS`](#ptrace-setfpxregs)
  - [`PTRACE_GETREGS`](#ptrace-getregs)
  - [`PTRACE_SETREGS`](#ptrace-setregs)
  - [`PTRACE_PEEKSIGINFO_SHARED`](#ptrace-peeksiginfo-shared)
  - [`PTRACE_SYSEMU`](#ptrace-sysemu)
  - [`PTRACE_SYSEMU_SINGLESTEP`](#ptrace-sysemu-singlestep)
  - [`PR_GET_SPECULATION_CTRL`](#pr-get-speculation-ctrl)
  - [`PR_SET_SPECULATION_CTRL`](#pr-set-speculation-ctrl)
  - [`PR_SPEC_NOT_AFFECTED`](#pr-spec-not-affected)
  - [`PR_SPEC_PRCTL`](#pr-spec-prctl)
  - [`PR_SPEC_ENABLE`](#pr-spec-enable)
  - [`PR_SPEC_DISABLE`](#pr-spec-disable)
  - [`PR_SPEC_FORCE_DISABLE`](#pr-spec-force-disable)
  - [`PR_SPEC_DISABLE_NOEXEC`](#pr-spec-disable-noexec)
  - [`PR_SPEC_STORE_BYPASS`](#pr-spec-store-bypass)
  - [`PR_SPEC_INDIRECT_BRANCH`](#pr-spec-indirect-branch)
  - [`MCL_CURRENT`](#mcl-current)
  - [`MCL_FUTURE`](#mcl-future)
  - [`MCL_ONFAULT`](#mcl-onfault)
  - [`SIGSTKSZ`](#sigstksz)
  - [`MINSIGSTKSZ`](#minsigstksz)
  - [`CBAUD`](#cbaud)
  - [`TAB1`](#tab1)
  - [`TAB2`](#tab2)
  - [`TAB3`](#tab3)
  - [`CR1`](#cr1)
  - [`CR2`](#cr2)
  - [`CR3`](#cr3)
  - [`FF1`](#ff1)
  - [`BS1`](#bs1)
  - [`VT1`](#vt1)
  - [`VWERASE`](#vwerase)
  - [`VREPRINT`](#vreprint)
  - [`VSUSP`](#vsusp)
  - [`VSTART`](#vstart)
  - [`VSTOP`](#vstop)
  - [`VDISCARD`](#vdiscard)
  - [`VTIME`](#vtime)
  - [`IXON`](#ixon)
  - [`IXOFF`](#ixoff)
  - [`ONLCR`](#onlcr)
  - [`CSIZE`](#csize)
  - [`CS6`](#cs6)
  - [`CS7`](#cs7)
  - [`CS8`](#cs8)
  - [`CSTOPB`](#cstopb)
  - [`CREAD`](#cread)
  - [`PARENB`](#parenb)
  - [`PARODD`](#parodd)
  - [`HUPCL`](#hupcl)
  - [`CLOCAL`](#clocal)
  - [`ECHOKE`](#echoke)
  - [`ECHOE`](#echoe)
  - [`ECHOK`](#echok)
  - [`ECHONL`](#echonl)
  - [`ECHOPRT`](#echoprt)
  - [`ECHOCTL`](#echoctl)
  - [`ISIG`](#isig)
  - [`ICANON`](#icanon)
  - [`PENDIN`](#pendin)
  - [`NOFLSH`](#noflsh)
  - [`CIBAUD`](#cibaud)
  - [`CBAUDEX`](#cbaudex)
  - [`VSWTC`](#vswtc)
  - [`OLCUC`](#olcuc)
  - [`NLDLY`](#nldly)
  - [`CRDLY`](#crdly)
  - [`TABDLY`](#tabdly)
  - [`BSDLY`](#bsdly)
  - [`FFDLY`](#ffdly)
  - [`VTDLY`](#vtdly)
  - [`XTABS`](#xtabs)
  - [`B0`](#b0)
  - [`B50`](#b50)
  - [`B75`](#b75)
  - [`B110`](#b110)
  - [`B134`](#b134)
  - [`B150`](#b150)
  - [`B200`](#b200)
  - [`B300`](#b300)
  - [`B600`](#b600)
  - [`B1200`](#b1200)
  - [`B1800`](#b1800)
  - [`B2400`](#b2400)
  - [`B4800`](#b4800)
  - [`B9600`](#b9600)
  - [`B19200`](#b19200)
  - [`B38400`](#b38400)
  - [`EXTA`](#exta)
  - [`EXTB`](#extb)
  - [`B57600`](#b57600)
  - [`B115200`](#b115200)
  - [`B230400`](#b230400)
  - [`B460800`](#b460800)
  - [`B500000`](#b500000)
  - [`B576000`](#b576000)
  - [`B921600`](#b921600)
  - [`B1000000`](#b1000000)
  - [`B1152000`](#b1152000)
  - [`B1500000`](#b1500000)
  - [`B2000000`](#b2000000)
  - [`B2500000`](#b2500000)
  - [`B3000000`](#b3000000)
  - [`B3500000`](#b3500000)
  - [`B4000000`](#b4000000)
  - [`VEOL`](#veol)
  - [`VEOL2`](#veol2)
  - [`VMIN`](#vmin)
  - [`IEXTEN`](#iexten)
  - [`TOSTOP`](#tostop)
  - [`FLUSHO`](#flusho)
  - [`EXTPROC`](#extproc)
  - [`R15`](#r15)
  - [`R14`](#r14)
  - [`R13`](#r13)
  - [`R12`](#r12)
  - [`RBP`](#rbp)
  - [`RBX`](#rbx)
  - [`R11`](#r11)
  - [`R10`](#r10)
  - [`R9`](#r9)
  - [`R8`](#r8)
  - [`RAX`](#rax)
  - [`RCX`](#rcx)
  - [`RDX`](#rdx)
  - [`RSI`](#rsi)
  - [`RDI`](#rdi)
  - [`ORIG_RAX`](#orig-rax)
  - [`RIP`](#rip)
  - [`CS`](#cs)
  - [`EFLAGS`](#eflags)
  - [`RSP`](#rsp)
  - [`SS`](#ss)
  - [`FS_BASE`](#fs-base)
  - [`GS_BASE`](#gs-base)
  - [`DS`](#ds)
  - [`ES`](#es)
  - [`FS`](#fs)
  - [`GS`](#gs)
  - [`REG_R8`](#reg-r8)
  - [`REG_R9`](#reg-r9)
  - [`REG_R10`](#reg-r10)
  - [`REG_R11`](#reg-r11)
  - [`REG_R12`](#reg-r12)
  - [`REG_R13`](#reg-r13)
  - [`REG_R14`](#reg-r14)
  - [`REG_R15`](#reg-r15)
  - [`REG_RDI`](#reg-rdi)
  - [`REG_RSI`](#reg-rsi)
  - [`REG_RBP`](#reg-rbp)
  - [`REG_RBX`](#reg-rbx)
  - [`REG_RDX`](#reg-rdx)
  - [`REG_RAX`](#reg-rax)
  - [`REG_RCX`](#reg-rcx)
  - [`REG_RSP`](#reg-rsp)
  - [`REG_RIP`](#reg-rip)
  - [`REG_EFL`](#reg-efl)
  - [`REG_CSGSFS`](#reg-csgsfs)
  - [`REG_ERR`](#reg-err)
  - [`REG_TRAPNO`](#reg-trapno)
  - [`REG_OLDMASK`](#reg-oldmask)
  - [`REG_CR2`](#reg-cr2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`x86_64`](#x86-64) | mod | x86_64-specific definitions for 64-bit linux-like values |
| [`not_x32`](#not-x32) | mod |  |
| [`sigset_t`](#sigset-t) | struct |  |
| [`sysinfo`](#sysinfo) | struct |  |
| [`msqid_ds`](#msqid-ds) | struct |  |
| [`semid_ds`](#semid-ds) | struct |  |
| [`timex`](#timex) | struct |  |
| [`sigaction`](#sigaction) | struct |  |
| [`statfs`](#statfs) | struct |  |
| [`flock`](#flock) | struct |  |
| [`flock64`](#flock64) | struct |  |
| [`siginfo_t`](#siginfo-t) | struct |  |
| [`stack_t`](#stack-t) | struct |  |
| [`stat`](#stat) | struct |  |
| [`stat64`](#stat64) | struct |  |
| [`statfs64`](#statfs64) | struct |  |
| [`statvfs64`](#statvfs64) | struct |  |
| [`pthread_attr_t`](#pthread-attr-t) | struct |  |
| [`_libc_fpxreg`](#libc-fpxreg) | struct |  |
| [`_libc_xmmreg`](#libc-xmmreg) | struct |  |
| [`_libc_fpstate`](#libc-fpstate) | struct |  |
| [`user_regs_struct`](#user-regs-struct) | struct |  |
| [`user`](#user) | struct |  |
| [`mcontext_t`](#mcontext-t) | struct |  |
| [`ipc_perm`](#ipc-perm) | struct |  |
| [`shmid_ds`](#shmid-ds) | struct |  |
| [`ptrace_rseq_configuration`](#ptrace-rseq-configuration) | struct |  |
| [`clone_args`](#clone-args) | struct |  |
| [`user_fpregs_struct`](#user-fpregs-struct) | struct |  |
| [`ucontext_t`](#ucontext-t) | struct |  |
| [`max_align_t`](#max-align-t) | struct |  |
| [`getcontext`](#getcontext) | fn |  |
| [`setcontext`](#setcontext) | fn |  |
| [`makecontext`](#makecontext) | fn |  |
| [`swapcontext`](#swapcontext) | fn |  |
| [`ino_t`](#ino-t) | type |  |
| [`off_t`](#off-t) | type |  |
| [`blkcnt_t`](#blkcnt-t) | type |  |
| [`shmatt_t`](#shmatt-t) | type |  |
| [`msgqnum_t`](#msgqnum-t) | type |  |
| [`msglen_t`](#msglen-t) | type |  |
| [`fsblkcnt_t`](#fsblkcnt-t) | type |  |
| [`fsfilcnt_t`](#fsfilcnt-t) | type |  |
| [`rlim_t`](#rlim-t) | type |  |
| [`__syscall_ulong_t`](#syscall-ulong-t) | type |  |
| [`__fsword_t`](#fsword-t) | type |  |
| [`clock_t`](#clock-t) | type |  |
| [`time_t`](#time-t) | type |  |
| [`wchar_t`](#wchar-t) | type |  |
| [`nlink_t`](#nlink-t) | type |  |
| [`blksize_t`](#blksize-t) | type |  |
| [`greg_t`](#greg-t) | type |  |
| [`suseconds_t`](#suseconds-t) | type |  |
| [`__u64`](#u64) | type |  |
| [`__s64`](#s64) | type |  |
| [`__SIZEOF_PTHREAD_RWLOCKATTR_T`](#sizeof-pthread-rwlockattr-t) | const |  |
| [`O_LARGEFILE`](#o-largefile) | const |  |
| [`POSIX_FADV_DONTNEED`](#posix-fadv-dontneed) | const |  |
| [`POSIX_FADV_NOREUSE`](#posix-fadv-noreuse) | const |  |
| [`VEOF`](#veof) | const |  |
| [`RTLD_DEEPBIND`](#rtld-deepbind) | const |  |
| [`RTLD_GLOBAL`](#rtld-global) | const |  |
| [`RTLD_NOLOAD`](#rtld-noload) | const |  |
| [`O_APPEND`](#o-append) | const |  |
| [`O_CREAT`](#o-creat) | const |  |
| [`O_EXCL`](#o-excl) | const |  |
| [`O_NOCTTY`](#o-noctty) | const |  |
| [`O_NONBLOCK`](#o-nonblock) | const |  |
| [`O_SYNC`](#o-sync) | const |  |
| [`O_RSYNC`](#o-rsync) | const |  |
| [`O_DSYNC`](#o-dsync) | const |  |
| [`O_FSYNC`](#o-fsync) | const |  |
| [`O_NOATIME`](#o-noatime) | const |  |
| [`O_PATH`](#o-path) | const |  |
| [`O_TMPFILE`](#o-tmpfile) | const |  |
| [`MADV_SOFT_OFFLINE`](#madv-soft-offline) | const |  |
| [`MAP_GROWSDOWN`](#map-growsdown) | const |  |
| [`EDEADLK`](#edeadlk) | const |  |
| [`ENAMETOOLONG`](#enametoolong) | const |  |
| [`ENOLCK`](#enolck) | const |  |
| [`ENOSYS`](#enosys) | const |  |
| [`ENOTEMPTY`](#enotempty) | const |  |
| [`ELOOP`](#eloop) | const |  |
| [`ENOMSG`](#enomsg) | const |  |
| [`EIDRM`](#eidrm) | const |  |
| [`ECHRNG`](#echrng) | const |  |
| [`EL2NSYNC`](#el2nsync) | const |  |
| [`EL3HLT`](#el3hlt) | const |  |
| [`EL3RST`](#el3rst) | const |  |
| [`ELNRNG`](#elnrng) | const |  |
| [`EUNATCH`](#eunatch) | const |  |
| [`ENOCSI`](#enocsi) | const |  |
| [`EL2HLT`](#el2hlt) | const |  |
| [`EBADE`](#ebade) | const |  |
| [`EBADR`](#ebadr) | const |  |
| [`EXFULL`](#exfull) | const |  |
| [`ENOANO`](#enoano) | const |  |
| [`EBADRQC`](#ebadrqc) | const |  |
| [`EBADSLT`](#ebadslt) | const |  |
| [`EMULTIHOP`](#emultihop) | const |  |
| [`EOVERFLOW`](#eoverflow) | const |  |
| [`ENOTUNIQ`](#enotuniq) | const |  |
| [`EBADFD`](#ebadfd) | const |  |
| [`EBADMSG`](#ebadmsg) | const |  |
| [`EREMCHG`](#eremchg) | const |  |
| [`ELIBACC`](#elibacc) | const |  |
| [`ELIBBAD`](#elibbad) | const |  |
| [`ELIBSCN`](#elibscn) | const |  |
| [`ELIBMAX`](#elibmax) | const |  |
| [`ELIBEXEC`](#elibexec) | const |  |
| [`EILSEQ`](#eilseq) | const |  |
| [`ERESTART`](#erestart) | const |  |
| [`ESTRPIPE`](#estrpipe) | const |  |
| [`EUSERS`](#eusers) | const |  |
| [`ENOTSOCK`](#enotsock) | const |  |
| [`EDESTADDRREQ`](#edestaddrreq) | const |  |
| [`EMSGSIZE`](#emsgsize) | const |  |
| [`EPROTOTYPE`](#eprototype) | const |  |
| [`ENOPROTOOPT`](#enoprotoopt) | const |  |
| [`EPROTONOSUPPORT`](#eprotonosupport) | const |  |
| [`ESOCKTNOSUPPORT`](#esocktnosupport) | const |  |
| [`EOPNOTSUPP`](#eopnotsupp) | const |  |
| [`EPFNOSUPPORT`](#epfnosupport) | const |  |
| [`EAFNOSUPPORT`](#eafnosupport) | const |  |
| [`EADDRINUSE`](#eaddrinuse) | const |  |
| [`EADDRNOTAVAIL`](#eaddrnotavail) | const |  |
| [`ENETDOWN`](#enetdown) | const |  |
| [`ENETUNREACH`](#enetunreach) | const |  |
| [`ENETRESET`](#enetreset) | const |  |
| [`ECONNABORTED`](#econnaborted) | const |  |
| [`ECONNRESET`](#econnreset) | const |  |
| [`ENOBUFS`](#enobufs) | const |  |
| [`EISCONN`](#eisconn) | const |  |
| [`ENOTCONN`](#enotconn) | const |  |
| [`ESHUTDOWN`](#eshutdown) | const |  |
| [`ETOOMANYREFS`](#etoomanyrefs) | const |  |
| [`ETIMEDOUT`](#etimedout) | const |  |
| [`ECONNREFUSED`](#econnrefused) | const |  |
| [`EHOSTDOWN`](#ehostdown) | const |  |
| [`EHOSTUNREACH`](#ehostunreach) | const |  |
| [`EALREADY`](#ealready) | const |  |
| [`EINPROGRESS`](#einprogress) | const |  |
| [`ESTALE`](#estale) | const |  |
| [`EDQUOT`](#edquot) | const |  |
| [`ENOMEDIUM`](#enomedium) | const |  |
| [`EMEDIUMTYPE`](#emediumtype) | const |  |
| [`ECANCELED`](#ecanceled) | const |  |
| [`ENOKEY`](#enokey) | const |  |
| [`EKEYEXPIRED`](#ekeyexpired) | const |  |
| [`EKEYREVOKED`](#ekeyrevoked) | const |  |
| [`EKEYREJECTED`](#ekeyrejected) | const |  |
| [`EOWNERDEAD`](#eownerdead) | const |  |
| [`ENOTRECOVERABLE`](#enotrecoverable) | const |  |
| [`EHWPOISON`](#ehwpoison) | const |  |
| [`ERFKILL`](#erfkill) | const |  |
| [`SOCK_STREAM`](#sock-stream) | const |  |
| [`SOCK_DGRAM`](#sock-dgram) | const |  |
| [`SA_ONSTACK`](#sa-onstack) | const |  |
| [`SA_SIGINFO`](#sa-siginfo) | const |  |
| [`SA_NOCLDWAIT`](#sa-nocldwait) | const |  |
| [`SIGTTIN`](#sigttin) | const |  |
| [`SIGTTOU`](#sigttou) | const |  |
| [`SIGXCPU`](#sigxcpu) | const |  |
| [`SIGXFSZ`](#sigxfsz) | const |  |
| [`SIGVTALRM`](#sigvtalrm) | const |  |
| [`SIGPROF`](#sigprof) | const |  |
| [`SIGWINCH`](#sigwinch) | const |  |
| [`SIGCHLD`](#sigchld) | const |  |
| [`SIGBUS`](#sigbus) | const |  |
| [`SIGUSR1`](#sigusr1) | const |  |
| [`SIGUSR2`](#sigusr2) | const |  |
| [`SIGCONT`](#sigcont) | const |  |
| [`SIGSTOP`](#sigstop) | const |  |
| [`SIGTSTP`](#sigtstp) | const |  |
| [`SIGURG`](#sigurg) | const |  |
| [`SIGIO`](#sigio) | const |  |
| [`SIGSYS`](#sigsys) | const |  |
| [`SIGSTKFLT`](#sigstkflt) | const |  |
| [`SIGUNUSED`](#sigunused) | const |  |
| [`SIGPOLL`](#sigpoll) | const |  |
| [`SIGPWR`](#sigpwr) | const |  |
| [`SIG_SETMASK`](#sig-setmask) | const |  |
| [`SIG_BLOCK`](#sig-block) | const |  |
| [`SIG_UNBLOCK`](#sig-unblock) | const |  |
| [`POLLWRNORM`](#pollwrnorm) | const |  |
| [`POLLWRBAND`](#pollwrband) | const |  |
| [`O_ASYNC`](#o-async) | const |  |
| [`O_NDELAY`](#o-ndelay) | const |  |
| [`PTRACE_DETACH`](#ptrace-detach) | const |  |
| [`PTRACE_GET_RSEQ_CONFIGURATION`](#ptrace-get-rseq-configuration) | const |  |
| [`EFD_NONBLOCK`](#efd-nonblock) | const |  |
| [`F_GETLK`](#f-getlk) | const |  |
| [`F_GETOWN`](#f-getown) | const |  |
| [`F_SETOWN`](#f-setown) | const |  |
| [`F_SETLK`](#f-setlk) | const |  |
| [`F_SETLKW`](#f-setlkw) | const |  |
| [`F_OFD_GETLK`](#f-ofd-getlk) | const |  |
| [`F_OFD_SETLK`](#f-ofd-setlk) | const |  |
| [`F_OFD_SETLKW`](#f-ofd-setlkw) | const |  |
| [`F_RDLCK`](#f-rdlck) | const |  |
| [`F_WRLCK`](#f-wrlck) | const |  |
| [`F_UNLCK`](#f-unlck) | const |  |
| [`SFD_NONBLOCK`](#sfd-nonblock) | const |  |
| [`TCSANOW`](#tcsanow) | const |  |
| [`TCSADRAIN`](#tcsadrain) | const |  |
| [`TCSAFLUSH`](#tcsaflush) | const |  |
| [`SFD_CLOEXEC`](#sfd-cloexec) | const |  |
| [`NCCS`](#nccs) | const |  |
| [`O_TRUNC`](#o-trunc) | const |  |
| [`O_CLOEXEC`](#o-cloexec) | const |  |
| [`EBFONT`](#ebfont) | const |  |
| [`ENOSTR`](#enostr) | const |  |
| [`ENODATA`](#enodata) | const |  |
| [`ETIME`](#etime) | const |  |
| [`ENOSR`](#enosr) | const |  |
| [`ENONET`](#enonet) | const |  |
| [`ENOPKG`](#enopkg) | const |  |
| [`EREMOTE`](#eremote) | const |  |
| [`ENOLINK`](#enolink) | const |  |
| [`EADV`](#eadv) | const |  |
| [`ESRMNT`](#esrmnt) | const |  |
| [`ECOMM`](#ecomm) | const |  |
| [`EPROTO`](#eproto) | const |  |
| [`EDOTDOT`](#edotdot) | const |  |
| [`SA_NODEFER`](#sa-nodefer) | const |  |
| [`SA_RESETHAND`](#sa-resethand) | const |  |
| [`SA_RESTART`](#sa-restart) | const |  |
| [`SA_NOCLDSTOP`](#sa-nocldstop) | const |  |
| [`EPOLL_CLOEXEC`](#epoll-cloexec) | const |  |
| [`EFD_CLOEXEC`](#efd-cloexec) | const |  |
| [`__SIZEOF_PTHREAD_CONDATTR_T`](#sizeof-pthread-condattr-t) | const |  |
| [`__SIZEOF_PTHREAD_MUTEXATTR_T`](#sizeof-pthread-mutexattr-t) | const |  |
| [`__SIZEOF_PTHREAD_BARRIERATTR_T`](#sizeof-pthread-barrierattr-t) | const |  |
| [`O_DIRECT`](#o-direct) | const |  |
| [`O_DIRECTORY`](#o-directory) | const |  |
| [`O_NOFOLLOW`](#o-nofollow) | const |  |
| [`MAP_HUGETLB`](#map-hugetlb) | const |  |
| [`MAP_LOCKED`](#map-locked) | const |  |
| [`MAP_NORESERVE`](#map-noreserve) | const |  |
| [`MAP_32BIT`](#map-32bit) | const |  |
| [`MAP_ANON`](#map-anon) | const |  |
| [`MAP_ANONYMOUS`](#map-anonymous) | const |  |
| [`MAP_DENYWRITE`](#map-denywrite) | const |  |
| [`MAP_EXECUTABLE`](#map-executable) | const |  |
| [`MAP_POPULATE`](#map-populate) | const |  |
| [`MAP_NONBLOCK`](#map-nonblock) | const |  |
| [`MAP_STACK`](#map-stack) | const |  |
| [`MAP_SYNC`](#map-sync) | const |  |
| [`EDEADLOCK`](#edeadlock) | const |  |
| [`EUCLEAN`](#euclean) | const |  |
| [`ENOTNAM`](#enotnam) | const |  |
| [`ENAVAIL`](#enavail) | const |  |
| [`EISNAM`](#eisnam) | const |  |
| [`EREMOTEIO`](#eremoteio) | const |  |
| [`PTRACE_GETFPREGS`](#ptrace-getfpregs) | const |  |
| [`PTRACE_SETFPREGS`](#ptrace-setfpregs) | const |  |
| [`PTRACE_GETFPXREGS`](#ptrace-getfpxregs) | const |  |
| [`PTRACE_SETFPXREGS`](#ptrace-setfpxregs) | const |  |
| [`PTRACE_GETREGS`](#ptrace-getregs) | const |  |
| [`PTRACE_SETREGS`](#ptrace-setregs) | const |  |
| [`PTRACE_PEEKSIGINFO_SHARED`](#ptrace-peeksiginfo-shared) | const |  |
| [`PTRACE_SYSEMU`](#ptrace-sysemu) | const |  |
| [`PTRACE_SYSEMU_SINGLESTEP`](#ptrace-sysemu-singlestep) | const |  |
| [`PR_GET_SPECULATION_CTRL`](#pr-get-speculation-ctrl) | const |  |
| [`PR_SET_SPECULATION_CTRL`](#pr-set-speculation-ctrl) | const |  |
| [`PR_SPEC_NOT_AFFECTED`](#pr-spec-not-affected) | const |  |
| [`PR_SPEC_PRCTL`](#pr-spec-prctl) | const |  |
| [`PR_SPEC_ENABLE`](#pr-spec-enable) | const |  |
| [`PR_SPEC_DISABLE`](#pr-spec-disable) | const |  |
| [`PR_SPEC_FORCE_DISABLE`](#pr-spec-force-disable) | const |  |
| [`PR_SPEC_DISABLE_NOEXEC`](#pr-spec-disable-noexec) | const |  |
| [`PR_SPEC_STORE_BYPASS`](#pr-spec-store-bypass) | const |  |
| [`PR_SPEC_INDIRECT_BRANCH`](#pr-spec-indirect-branch) | const |  |
| [`MCL_CURRENT`](#mcl-current) | const |  |
| [`MCL_FUTURE`](#mcl-future) | const |  |
| [`MCL_ONFAULT`](#mcl-onfault) | const |  |
| [`SIGSTKSZ`](#sigstksz) | const |  |
| [`MINSIGSTKSZ`](#minsigstksz) | const |  |
| [`CBAUD`](#cbaud) | const |  |
| [`TAB1`](#tab1) | const |  |
| [`TAB2`](#tab2) | const |  |
| [`TAB3`](#tab3) | const |  |
| [`CR1`](#cr1) | const |  |
| [`CR2`](#cr2) | const |  |
| [`CR3`](#cr3) | const |  |
| [`FF1`](#ff1) | const |  |
| [`BS1`](#bs1) | const |  |
| [`VT1`](#vt1) | const |  |
| [`VWERASE`](#vwerase) | const |  |
| [`VREPRINT`](#vreprint) | const |  |
| [`VSUSP`](#vsusp) | const |  |
| [`VSTART`](#vstart) | const |  |
| [`VSTOP`](#vstop) | const |  |
| [`VDISCARD`](#vdiscard) | const |  |
| [`VTIME`](#vtime) | const |  |
| [`IXON`](#ixon) | const |  |
| [`IXOFF`](#ixoff) | const |  |
| [`ONLCR`](#onlcr) | const |  |
| [`CSIZE`](#csize) | const |  |
| [`CS6`](#cs6) | const |  |
| [`CS7`](#cs7) | const |  |
| [`CS8`](#cs8) | const |  |
| [`CSTOPB`](#cstopb) | const |  |
| [`CREAD`](#cread) | const |  |
| [`PARENB`](#parenb) | const |  |
| [`PARODD`](#parodd) | const |  |
| [`HUPCL`](#hupcl) | const |  |
| [`CLOCAL`](#clocal) | const |  |
| [`ECHOKE`](#echoke) | const |  |
| [`ECHOE`](#echoe) | const |  |
| [`ECHOK`](#echok) | const |  |
| [`ECHONL`](#echonl) | const |  |
| [`ECHOPRT`](#echoprt) | const |  |
| [`ECHOCTL`](#echoctl) | const |  |
| [`ISIG`](#isig) | const |  |
| [`ICANON`](#icanon) | const |  |
| [`PENDIN`](#pendin) | const |  |
| [`NOFLSH`](#noflsh) | const |  |
| [`CIBAUD`](#cibaud) | const |  |
| [`CBAUDEX`](#cbaudex) | const |  |
| [`VSWTC`](#vswtc) | const |  |
| [`OLCUC`](#olcuc) | const |  |
| [`NLDLY`](#nldly) | const |  |
| [`CRDLY`](#crdly) | const |  |
| [`TABDLY`](#tabdly) | const |  |
| [`BSDLY`](#bsdly) | const |  |
| [`FFDLY`](#ffdly) | const |  |
| [`VTDLY`](#vtdly) | const |  |
| [`XTABS`](#xtabs) | const |  |
| [`B0`](#b0) | const |  |
| [`B50`](#b50) | const |  |
| [`B75`](#b75) | const |  |
| [`B110`](#b110) | const |  |
| [`B134`](#b134) | const |  |
| [`B150`](#b150) | const |  |
| [`B200`](#b200) | const |  |
| [`B300`](#b300) | const |  |
| [`B600`](#b600) | const |  |
| [`B1200`](#b1200) | const |  |
| [`B1800`](#b1800) | const |  |
| [`B2400`](#b2400) | const |  |
| [`B4800`](#b4800) | const |  |
| [`B9600`](#b9600) | const |  |
| [`B19200`](#b19200) | const |  |
| [`B38400`](#b38400) | const |  |
| [`EXTA`](#exta) | const |  |
| [`EXTB`](#extb) | const |  |
| [`B57600`](#b57600) | const |  |
| [`B115200`](#b115200) | const |  |
| [`B230400`](#b230400) | const |  |
| [`B460800`](#b460800) | const |  |
| [`B500000`](#b500000) | const |  |
| [`B576000`](#b576000) | const |  |
| [`B921600`](#b921600) | const |  |
| [`B1000000`](#b1000000) | const |  |
| [`B1152000`](#b1152000) | const |  |
| [`B1500000`](#b1500000) | const |  |
| [`B2000000`](#b2000000) | const |  |
| [`B2500000`](#b2500000) | const |  |
| [`B3000000`](#b3000000) | const |  |
| [`B3500000`](#b3500000) | const |  |
| [`B4000000`](#b4000000) | const |  |
| [`VEOL`](#veol) | const |  |
| [`VEOL2`](#veol2) | const |  |
| [`VMIN`](#vmin) | const |  |
| [`IEXTEN`](#iexten) | const |  |
| [`TOSTOP`](#tostop) | const |  |
| [`FLUSHO`](#flusho) | const |  |
| [`EXTPROC`](#extproc) | const |  |
| [`R15`](#r15) | const |  |
| [`R14`](#r14) | const |  |
| [`R13`](#r13) | const |  |
| [`R12`](#r12) | const |  |
| [`RBP`](#rbp) | const |  |
| [`RBX`](#rbx) | const |  |
| [`R11`](#r11) | const |  |
| [`R10`](#r10) | const |  |
| [`R9`](#r9) | const |  |
| [`R8`](#r8) | const |  |
| [`RAX`](#rax) | const |  |
| [`RCX`](#rcx) | const |  |
| [`RDX`](#rdx) | const |  |
| [`RSI`](#rsi) | const |  |
| [`RDI`](#rdi) | const |  |
| [`ORIG_RAX`](#orig-rax) | const |  |
| [`RIP`](#rip) | const |  |
| [`CS`](#cs) | const |  |
| [`EFLAGS`](#eflags) | const |  |
| [`RSP`](#rsp) | const |  |
| [`SS`](#ss) | const |  |
| [`FS_BASE`](#fs-base) | const |  |
| [`GS_BASE`](#gs-base) | const |  |
| [`DS`](#ds) | const |  |
| [`ES`](#es) | const |  |
| [`FS`](#fs) | const |  |
| [`GS`](#gs) | const |  |
| [`REG_R8`](#reg-r8) | const |  |
| [`REG_R9`](#reg-r9) | const |  |
| [`REG_R10`](#reg-r10) | const |  |
| [`REG_R11`](#reg-r11) | const |  |
| [`REG_R12`](#reg-r12) | const |  |
| [`REG_R13`](#reg-r13) | const |  |
| [`REG_R14`](#reg-r14) | const |  |
| [`REG_R15`](#reg-r15) | const |  |
| [`REG_RDI`](#reg-rdi) | const |  |
| [`REG_RSI`](#reg-rsi) | const |  |
| [`REG_RBP`](#reg-rbp) | const |  |
| [`REG_RBX`](#reg-rbx) | const |  |
| [`REG_RDX`](#reg-rdx) | const |  |
| [`REG_RAX`](#reg-rax) | const |  |
| [`REG_RCX`](#reg-rcx) | const |  |
| [`REG_RSP`](#reg-rsp) | const |  |
| [`REG_RIP`](#reg-rip) | const |  |
| [`REG_EFL`](#reg-efl) | const |  |
| [`REG_CSGSFS`](#reg-csgsfs) | const |  |
| [`REG_ERR`](#reg-err) | const |  |
| [`REG_TRAPNO`](#reg-trapno) | const |  |
| [`REG_OLDMASK`](#reg-oldmask) | const |  |
| [`REG_CR2`](#reg-cr2) | const |  |

## Modules

- [`x86_64`](x86_64/index.md) — x86_64-specific definitions for 64-bit linux-like values
- [`not_x32`](not_x32/index.md)

## Structs

### `sigset_t`

```rust
struct sigset_t {
    __val: [u64; 16],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:31-179`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L31-L179)*

#### Trait Implementations

##### `impl Clone for sigset_t`

- <span id="sigset-t-clone"></span>`fn clone(&self) -> sigset_t` — [`sigset_t`](../index.md#sigset-t)

##### `impl Copy for sigset_t`

##### `impl Debug for sigset_t`

- <span id="sigset-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:31-179`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L31-L179)*

#### Trait Implementations

##### `impl Clone for sysinfo`

- <span id="sysinfo-clone"></span>`fn clone(&self) -> sysinfo` — [`sysinfo`](../index.md#sysinfo)

##### `impl Copy for sysinfo`

##### `impl Debug for sysinfo`

- <span id="sysinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:31-179`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L31-L179)*

#### Trait Implementations

##### `impl Clone for msqid_ds`

- <span id="msqid-ds-clone"></span>`fn clone(&self) -> msqid_ds` — [`msqid_ds`](../index.md#msqid-ds)

##### `impl Copy for msqid_ds`

##### `impl Debug for msqid_ds`

- <span id="msqid-ds-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:31-179`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L31-L179)*

#### Trait Implementations

##### `impl Clone for semid_ds`

- <span id="semid-ds-clone"></span>`fn clone(&self) -> semid_ds` — [`semid_ds`](../index.md#semid-ds)

##### `impl Copy for semid_ds`

##### `impl Debug for semid_ds`

- <span id="semid-ds-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:31-179`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L31-L179)*

#### Trait Implementations

##### `impl Clone for timex`

- <span id="timex-clone"></span>`fn clone(&self) -> timex` — [`timex`](../index.md#timex)

##### `impl Copy for timex`

##### `impl Debug for timex`

- <span id="timex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sigaction`

```rust
struct sigaction {
    pub sa_sigaction: crate::sighandler_t,
    pub sa_mask: crate::sigset_t,
    pub sa_flags: crate::c_int,
    pub sa_restorer: core::option::Option<fn()>,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for sigaction`

- <span id="sigaction-clone"></span>`fn clone(&self) -> sigaction` — [`sigaction`](#sigaction)

##### `impl Copy for sigaction`

##### `impl Debug for sigaction`

- <span id="sigaction-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for statfs`

- <span id="statfs-clone"></span>`fn clone(&self) -> statfs` — [`statfs`](#statfs)

##### `impl Copy for statfs`

##### `impl Debug for statfs`

- <span id="statfs-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for flock`

- <span id="flock-clone"></span>`fn clone(&self) -> flock` — [`flock`](#flock)

##### `impl Copy for flock`

##### `impl Debug for flock`

- <span id="flock-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for flock64`

- <span id="flock64-clone"></span>`fn clone(&self) -> flock64` — [`flock64`](#flock64)

##### `impl Copy for flock64`

##### `impl Debug for flock64`

- <span id="flock64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Implementations

- <span id="siginfo-t-si-addr"></span>`unsafe fn si_addr(&self) -> *mut c_void` — [`c_void`](../../../../../index.md#c-void)

- <span id="siginfo-t-si-value"></span>`unsafe fn si_value(&self) -> crate::sigval` — [`sigval`](../../../../../index.md#sigval)

#### Trait Implementations

##### `impl Clone for siginfo_t`

- <span id="siginfo-t-clone"></span>`fn clone(&self) -> siginfo_t` — [`siginfo_t`](#siginfo-t)

##### `impl Copy for siginfo_t`

##### `impl Debug for siginfo_t`

- <span id="siginfo-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `stack_t`

```rust
struct stack_t {
    pub ss_sp: *mut crate::c_void,
    pub ss_flags: crate::c_int,
    pub ss_size: crate::size_t,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for stack_t`

- <span id="stack-t-clone"></span>`fn clone(&self) -> stack_t` — [`stack_t`](#stack-t)

##### `impl Copy for stack_t`

##### `impl Debug for stack_t`

- <span id="stack-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for stat`

- <span id="stat-clone"></span>`fn clone(&self) -> stat` — [`stat`](#stat)

##### `impl Copy for stat`

##### `impl Debug for stat`

- <span id="stat-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for stat64`

- <span id="stat64-clone"></span>`fn clone(&self) -> stat64` — [`stat64`](#stat64)

##### `impl Copy for stat64`

##### `impl Debug for stat64`

- <span id="stat64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for statfs64`

- <span id="statfs64-clone"></span>`fn clone(&self) -> statfs64` — [`statfs64`](#statfs64)

##### `impl Copy for statfs64`

##### `impl Debug for statfs64`

- <span id="statfs64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for statvfs64`

- <span id="statvfs64-clone"></span>`fn clone(&self) -> statvfs64` — [`statvfs64`](#statvfs64)

##### `impl Copy for statvfs64`

##### `impl Debug for statvfs64`

- <span id="statvfs64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pthread_attr_t`

```rust
struct pthread_attr_t {
    __size: [u64; 7],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for pthread_attr_t`

- <span id="pthread-attr-t-clone"></span>`fn clone(&self) -> pthread_attr_t` — [`pthread_attr_t`](#pthread-attr-t)

##### `impl Copy for pthread_attr_t`

##### `impl Debug for pthread_attr_t`

- <span id="pthread-attr-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `_libc_fpxreg`

```rust
struct _libc_fpxreg {
    pub significand: [u16; 4],
    pub exponent: u16,
    __private: [u16; 3],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for _libc_fpxreg`

- <span id="libc-fpxreg-clone"></span>`fn clone(&self) -> _libc_fpxreg` — [`_libc_fpxreg`](#libc-fpxreg)

##### `impl Copy for _libc_fpxreg`

##### `impl Debug for _libc_fpxreg`

- <span id="libc-fpxreg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `_libc_xmmreg`

```rust
struct _libc_xmmreg {
    pub element: [u32; 4],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for _libc_xmmreg`

- <span id="libc-xmmreg-clone"></span>`fn clone(&self) -> _libc_xmmreg` — [`_libc_xmmreg`](#libc-xmmreg)

##### `impl Copy for _libc_xmmreg`

##### `impl Debug for _libc_xmmreg`

- <span id="libc-xmmreg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for _libc_fpstate`

- <span id="libc-fpstate-clone"></span>`fn clone(&self) -> _libc_fpstate` — [`_libc_fpstate`](#libc-fpstate)

##### `impl Copy for _libc_fpstate`

##### `impl Debug for _libc_fpstate`

- <span id="libc-fpstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for user_regs_struct`

- <span id="user-regs-struct-clone"></span>`fn clone(&self) -> user_regs_struct` — [`user_regs_struct`](#user-regs-struct)

##### `impl Copy for user_regs_struct`

##### `impl Debug for user_regs_struct`

- <span id="user-regs-struct-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for user`

- <span id="user-clone"></span>`fn clone(&self) -> user` — [`user`](#user)

##### `impl Copy for user`

##### `impl Debug for user`

- <span id="user-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mcontext_t`

```rust
struct mcontext_t {
    pub gregs: [greg_t; 23],
    pub fpregs: *mut _libc_fpstate,
    __private: [u64; 8],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for mcontext_t`

- <span id="mcontext-t-clone"></span>`fn clone(&self) -> mcontext_t` — [`mcontext_t`](#mcontext-t)

##### `impl Copy for mcontext_t`

##### `impl Debug for mcontext_t`

- <span id="mcontext-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for ipc_perm`

- <span id="ipc-perm-clone"></span>`fn clone(&self) -> ipc_perm` — [`ipc_perm`](#ipc-perm)

##### `impl Copy for ipc_perm`

##### `impl Debug for ipc_perm`

- <span id="ipc-perm-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for shmid_ds`

- <span id="shmid-ds-clone"></span>`fn clone(&self) -> shmid_ds` — [`shmid_ds`](#shmid-ds)

##### `impl Copy for shmid_ds`

##### `impl Debug for shmid_ds`

- <span id="shmid-ds-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for ptrace_rseq_configuration`

- <span id="ptrace-rseq-configuration-clone"></span>`fn clone(&self) -> ptrace_rseq_configuration` — [`ptrace_rseq_configuration`](#ptrace-rseq-configuration)

##### `impl Copy for ptrace_rseq_configuration`

##### `impl Debug for ptrace_rseq_configuration`

- <span id="ptrace-rseq-configuration-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for clone_args`

- <span id="clone-args-clone"></span>`fn clone(&self) -> clone_args` — [`clone_args`](#clone-args)

##### `impl Copy for clone_args`

##### `impl Debug for clone_args`

- <span id="clone-args-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for user_fpregs_struct`

- <span id="user-fpregs-struct-clone"></span>`fn clone(&self) -> user_fpregs_struct` — [`user_fpregs_struct`](#user-fpregs-struct)

##### `impl Copy for user_fpregs_struct`

##### `impl Debug for user_fpregs_struct`

- <span id="user-fpregs-struct-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for ucontext_t`

- <span id="ucontext-t-clone"></span>`fn clone(&self) -> ucontext_t` — [`ucontext_t`](#ucontext-t)

##### `impl Copy for ucontext_t`

##### `impl Debug for ucontext_t`

- <span id="ucontext-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `max_align_t`

```rust
struct max_align_t {
    priv_: [f64; 4],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:318-323`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L318-L323)*

#### Trait Implementations

##### `impl Clone for max_align_t`

- <span id="max-align-t-clone"></span>`fn clone(&self) -> max_align_t` — [`max_align_t`](#max-align-t)

##### `impl Copy for max_align_t`

##### `impl Debug for max_align_t`

- <span id="max-align-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `getcontext`

```rust
unsafe fn getcontext(ucp: *mut ucontext_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:729`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L729)*

### `setcontext`

```rust
unsafe fn setcontext(ucp: *const ucontext_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:730`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L730)*

### `makecontext`

```rust
unsafe fn makecontext(ucp: *mut ucontext_t, func: fn(), argc: c_int)
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:731`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L731)*

### `swapcontext`

```rust
unsafe fn swapcontext(uocp: *mut ucontext_t, ucp: *const ucontext_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:732`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L732)*

## Type Aliases

### `ino_t`

```rust
type ino_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:5`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L5)*

### `off_t`

```rust
type off_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:6`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L6)*

### `blkcnt_t`

```rust
type blkcnt_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:7`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L7)*

### `shmatt_t`

```rust
type shmatt_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:8`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L8)*

### `msgqnum_t`

```rust
type msgqnum_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:9`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L9)*

### `msglen_t`

```rust
type msglen_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:10`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L10)*

### `fsblkcnt_t`

```rust
type fsblkcnt_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:11`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L11)*

### `fsfilcnt_t`

```rust
type fsfilcnt_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:12`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L12)*

### `rlim_t`

```rust
type rlim_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:13`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L13)*

### `__syscall_ulong_t`

```rust
type __syscall_ulong_t = crate::c_ulong;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:17`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L17)*

### `__fsword_t`

```rust
type __fsword_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:25`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L25)*

### `clock_t`

```rust
type clock_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:26`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L26)*

### `time_t`

```rust
type time_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:27`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L27)*

### `wchar_t`

```rust
type wchar_t = i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:9`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L9)*

### `nlink_t`

```rust
type nlink_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:10`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L10)*

### `blksize_t`

```rust
type blksize_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:11`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L11)*

### `greg_t`

```rust
type greg_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:12`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L12)*

### `suseconds_t`

```rust
type suseconds_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:13`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L13)*

### `__u64`

```rust
type __u64 = crate::c_ulonglong;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:14`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L14)*

### `__s64`

```rust
type __s64 = crate::c_longlong;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:15`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L15)*

## Constants

### `__SIZEOF_PTHREAD_RWLOCKATTR_T`
```rust
const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:181`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L181)*

### `O_LARGEFILE`
```rust
const O_LARGEFILE: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs:183`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/mod.rs#L183)*

### `POSIX_FADV_DONTNEED`
```rust
const POSIX_FADV_DONTNEED: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:325`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L325)*

### `POSIX_FADV_NOREUSE`
```rust
const POSIX_FADV_NOREUSE: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:326`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L326)*

### `VEOF`
```rust
const VEOF: usize = 4usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:328`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L328)*

### `RTLD_DEEPBIND`
```rust
const RTLD_DEEPBIND: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:329`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L329)*

### `RTLD_GLOBAL`
```rust
const RTLD_GLOBAL: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:330`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L330)*

### `RTLD_NOLOAD`
```rust
const RTLD_NOLOAD: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:331`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L331)*

### `O_APPEND`
```rust
const O_APPEND: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:333`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L333)*

### `O_CREAT`
```rust
const O_CREAT: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:334`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L334)*

### `O_EXCL`
```rust
const O_EXCL: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:335`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L335)*

### `O_NOCTTY`
```rust
const O_NOCTTY: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:336`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L336)*

### `O_NONBLOCK`
```rust
const O_NONBLOCK: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:337`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L337)*

### `O_SYNC`
```rust
const O_SYNC: crate::c_int = 1_052_672i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:338`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L338)*

### `O_RSYNC`
```rust
const O_RSYNC: crate::c_int = 1_052_672i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:339`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L339)*

### `O_DSYNC`
```rust
const O_DSYNC: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:340`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L340)*

### `O_FSYNC`
```rust
const O_FSYNC: crate::c_int = 1_052_672i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:341`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L341)*

### `O_NOATIME`
```rust
const O_NOATIME: crate::c_int = 262_144i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:342`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L342)*

### `O_PATH`
```rust
const O_PATH: crate::c_int = 2_097_152i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:343`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L343)*

### `O_TMPFILE`
```rust
const O_TMPFILE: crate::c_int = 4_259_840i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:344`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L344)*

### `MADV_SOFT_OFFLINE`
```rust
const MADV_SOFT_OFFLINE: crate::c_int = 101i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:346`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L346)*

### `MAP_GROWSDOWN`
```rust
const MAP_GROWSDOWN: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:347`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L347)*

### `EDEADLK`
```rust
const EDEADLK: crate::c_int = 35i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:349`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L349)*

### `ENAMETOOLONG`
```rust
const ENAMETOOLONG: crate::c_int = 36i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:350`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L350)*

### `ENOLCK`
```rust
const ENOLCK: crate::c_int = 37i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:351`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L351)*

### `ENOSYS`
```rust
const ENOSYS: crate::c_int = 38i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:352`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L352)*

### `ENOTEMPTY`
```rust
const ENOTEMPTY: crate::c_int = 39i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:353`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L353)*

### `ELOOP`
```rust
const ELOOP: crate::c_int = 40i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:354`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L354)*

### `ENOMSG`
```rust
const ENOMSG: crate::c_int = 42i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:355`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L355)*

### `EIDRM`
```rust
const EIDRM: crate::c_int = 43i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:356`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L356)*

### `ECHRNG`
```rust
const ECHRNG: crate::c_int = 44i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:357`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L357)*

### `EL2NSYNC`
```rust
const EL2NSYNC: crate::c_int = 45i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:358`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L358)*

### `EL3HLT`
```rust
const EL3HLT: crate::c_int = 46i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:359`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L359)*

### `EL3RST`
```rust
const EL3RST: crate::c_int = 47i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:360`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L360)*

### `ELNRNG`
```rust
const ELNRNG: crate::c_int = 48i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:361`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L361)*

### `EUNATCH`
```rust
const EUNATCH: crate::c_int = 49i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:362`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L362)*

### `ENOCSI`
```rust
const ENOCSI: crate::c_int = 50i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:363`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L363)*

### `EL2HLT`
```rust
const EL2HLT: crate::c_int = 51i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:364`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L364)*

### `EBADE`
```rust
const EBADE: crate::c_int = 52i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:365`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L365)*

### `EBADR`
```rust
const EBADR: crate::c_int = 53i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:366`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L366)*

### `EXFULL`
```rust
const EXFULL: crate::c_int = 54i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:367`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L367)*

### `ENOANO`
```rust
const ENOANO: crate::c_int = 55i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:368`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L368)*

### `EBADRQC`
```rust
const EBADRQC: crate::c_int = 56i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:369`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L369)*

### `EBADSLT`
```rust
const EBADSLT: crate::c_int = 57i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:370`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L370)*

### `EMULTIHOP`
```rust
const EMULTIHOP: crate::c_int = 72i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:371`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L371)*

### `EOVERFLOW`
```rust
const EOVERFLOW: crate::c_int = 75i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:372`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L372)*

### `ENOTUNIQ`
```rust
const ENOTUNIQ: crate::c_int = 76i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:373`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L373)*

### `EBADFD`
```rust
const EBADFD: crate::c_int = 77i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:374`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L374)*

### `EBADMSG`
```rust
const EBADMSG: crate::c_int = 74i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:375`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L375)*

### `EREMCHG`
```rust
const EREMCHG: crate::c_int = 78i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:376`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L376)*

### `ELIBACC`
```rust
const ELIBACC: crate::c_int = 79i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:377`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L377)*

### `ELIBBAD`
```rust
const ELIBBAD: crate::c_int = 80i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:378`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L378)*

### `ELIBSCN`
```rust
const ELIBSCN: crate::c_int = 81i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:379`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L379)*

### `ELIBMAX`
```rust
const ELIBMAX: crate::c_int = 82i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:380`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L380)*

### `ELIBEXEC`
```rust
const ELIBEXEC: crate::c_int = 83i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:381`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L381)*

### `EILSEQ`
```rust
const EILSEQ: crate::c_int = 84i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:382`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L382)*

### `ERESTART`
```rust
const ERESTART: crate::c_int = 85i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:383`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L383)*

### `ESTRPIPE`
```rust
const ESTRPIPE: crate::c_int = 86i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:384`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L384)*

### `EUSERS`
```rust
const EUSERS: crate::c_int = 87i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:385`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L385)*

### `ENOTSOCK`
```rust
const ENOTSOCK: crate::c_int = 88i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:386`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L386)*

### `EDESTADDRREQ`
```rust
const EDESTADDRREQ: crate::c_int = 89i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:387`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L387)*

### `EMSGSIZE`
```rust
const EMSGSIZE: crate::c_int = 90i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:388`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L388)*

### `EPROTOTYPE`
```rust
const EPROTOTYPE: crate::c_int = 91i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:389`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L389)*

### `ENOPROTOOPT`
```rust
const ENOPROTOOPT: crate::c_int = 92i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:390`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L390)*

### `EPROTONOSUPPORT`
```rust
const EPROTONOSUPPORT: crate::c_int = 93i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:391`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L391)*

### `ESOCKTNOSUPPORT`
```rust
const ESOCKTNOSUPPORT: crate::c_int = 94i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:392`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L392)*

### `EOPNOTSUPP`
```rust
const EOPNOTSUPP: crate::c_int = 95i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:393`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L393)*

### `EPFNOSUPPORT`
```rust
const EPFNOSUPPORT: crate::c_int = 96i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:394`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L394)*

### `EAFNOSUPPORT`
```rust
const EAFNOSUPPORT: crate::c_int = 97i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:395`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L395)*

### `EADDRINUSE`
```rust
const EADDRINUSE: crate::c_int = 98i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:396`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L396)*

### `EADDRNOTAVAIL`
```rust
const EADDRNOTAVAIL: crate::c_int = 99i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:397`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L397)*

### `ENETDOWN`
```rust
const ENETDOWN: crate::c_int = 100i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:398`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L398)*

### `ENETUNREACH`
```rust
const ENETUNREACH: crate::c_int = 101i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:399`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L399)*

### `ENETRESET`
```rust
const ENETRESET: crate::c_int = 102i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:400`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L400)*

### `ECONNABORTED`
```rust
const ECONNABORTED: crate::c_int = 103i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:401`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L401)*

### `ECONNRESET`
```rust
const ECONNRESET: crate::c_int = 104i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:402`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L402)*

### `ENOBUFS`
```rust
const ENOBUFS: crate::c_int = 105i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:403`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L403)*

### `EISCONN`
```rust
const EISCONN: crate::c_int = 106i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:404`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L404)*

### `ENOTCONN`
```rust
const ENOTCONN: crate::c_int = 107i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:405`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L405)*

### `ESHUTDOWN`
```rust
const ESHUTDOWN: crate::c_int = 108i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:406`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L406)*

### `ETOOMANYREFS`
```rust
const ETOOMANYREFS: crate::c_int = 109i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:407`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L407)*

### `ETIMEDOUT`
```rust
const ETIMEDOUT: crate::c_int = 110i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:408`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L408)*

### `ECONNREFUSED`
```rust
const ECONNREFUSED: crate::c_int = 111i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:409`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L409)*

### `EHOSTDOWN`
```rust
const EHOSTDOWN: crate::c_int = 112i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:410`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L410)*

### `EHOSTUNREACH`
```rust
const EHOSTUNREACH: crate::c_int = 113i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:411`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L411)*

### `EALREADY`
```rust
const EALREADY: crate::c_int = 114i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:412`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L412)*

### `EINPROGRESS`
```rust
const EINPROGRESS: crate::c_int = 115i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:413`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L413)*

### `ESTALE`
```rust
const ESTALE: crate::c_int = 116i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:414`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L414)*

### `EDQUOT`
```rust
const EDQUOT: crate::c_int = 122i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:415`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L415)*

### `ENOMEDIUM`
```rust
const ENOMEDIUM: crate::c_int = 123i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:416`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L416)*

### `EMEDIUMTYPE`
```rust
const EMEDIUMTYPE: crate::c_int = 124i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:417`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L417)*

### `ECANCELED`
```rust
const ECANCELED: crate::c_int = 125i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:418`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L418)*

### `ENOKEY`
```rust
const ENOKEY: crate::c_int = 126i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:419`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L419)*

### `EKEYEXPIRED`
```rust
const EKEYEXPIRED: crate::c_int = 127i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:420`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L420)*

### `EKEYREVOKED`
```rust
const EKEYREVOKED: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:421`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L421)*

### `EKEYREJECTED`
```rust
const EKEYREJECTED: crate::c_int = 129i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:422`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L422)*

### `EOWNERDEAD`
```rust
const EOWNERDEAD: crate::c_int = 130i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:423`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L423)*

### `ENOTRECOVERABLE`
```rust
const ENOTRECOVERABLE: crate::c_int = 131i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:424`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L424)*

### `EHWPOISON`
```rust
const EHWPOISON: crate::c_int = 133i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:425`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L425)*

### `ERFKILL`
```rust
const ERFKILL: crate::c_int = 132i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:426`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L426)*

### `SOCK_STREAM`
```rust
const SOCK_STREAM: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:428`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L428)*

### `SOCK_DGRAM`
```rust
const SOCK_DGRAM: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:429`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L429)*

### `SA_ONSTACK`
```rust
const SA_ONSTACK: crate::c_int = 134_217_728i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:431`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L431)*

### `SA_SIGINFO`
```rust
const SA_SIGINFO: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:432`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L432)*

### `SA_NOCLDWAIT`
```rust
const SA_NOCLDWAIT: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:433`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L433)*

### `SIGTTIN`
```rust
const SIGTTIN: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:435`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L435)*

### `SIGTTOU`
```rust
const SIGTTOU: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:436`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L436)*

### `SIGXCPU`
```rust
const SIGXCPU: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:437`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L437)*

### `SIGXFSZ`
```rust
const SIGXFSZ: crate::c_int = 25i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:438`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L438)*

### `SIGVTALRM`
```rust
const SIGVTALRM: crate::c_int = 26i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:439`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L439)*

### `SIGPROF`
```rust
const SIGPROF: crate::c_int = 27i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:440`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L440)*

### `SIGWINCH`
```rust
const SIGWINCH: crate::c_int = 28i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:441`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L441)*

### `SIGCHLD`
```rust
const SIGCHLD: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:442`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L442)*

### `SIGBUS`
```rust
const SIGBUS: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:443`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L443)*

### `SIGUSR1`
```rust
const SIGUSR1: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:444`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L444)*

### `SIGUSR2`
```rust
const SIGUSR2: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:445`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L445)*

### `SIGCONT`
```rust
const SIGCONT: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:446`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L446)*

### `SIGSTOP`
```rust
const SIGSTOP: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:447`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L447)*

### `SIGTSTP`
```rust
const SIGTSTP: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:448`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L448)*

### `SIGURG`
```rust
const SIGURG: crate::c_int = 23i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:449`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L449)*

### `SIGIO`
```rust
const SIGIO: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:450`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L450)*

### `SIGSYS`
```rust
const SIGSYS: crate::c_int = 31i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:451`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L451)*

### `SIGSTKFLT`
```rust
const SIGSTKFLT: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:452`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L452)*

### `SIGUNUSED`
```rust
const SIGUNUSED: crate::c_int = 31i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:454`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L454)*

### `SIGPOLL`
```rust
const SIGPOLL: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:455`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L455)*

### `SIGPWR`
```rust
const SIGPWR: crate::c_int = 30i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:456`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L456)*

### `SIG_SETMASK`
```rust
const SIG_SETMASK: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:457`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L457)*

### `SIG_BLOCK`
```rust
const SIG_BLOCK: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:458`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L458)*

### `SIG_UNBLOCK`
```rust
const SIG_UNBLOCK: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:459`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L459)*

### `POLLWRNORM`
```rust
const POLLWRNORM: crate::c_short = 256i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:461`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L461)*

### `POLLWRBAND`
```rust
const POLLWRBAND: crate::c_short = 512i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:462`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L462)*

### `O_ASYNC`
```rust
const O_ASYNC: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:464`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L464)*

### `O_NDELAY`
```rust
const O_NDELAY: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:465`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L465)*

### `PTRACE_DETACH`
```rust
const PTRACE_DETACH: crate::c_uint = 17u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:467`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L467)*

### `PTRACE_GET_RSEQ_CONFIGURATION`
```rust
const PTRACE_GET_RSEQ_CONFIGURATION: crate::c_uint = 16_911u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:468`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L468)*

### `EFD_NONBLOCK`
```rust
const EFD_NONBLOCK: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:470`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L470)*

### `F_GETLK`
```rust
const F_GETLK: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:472`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L472)*

### `F_GETOWN`
```rust
const F_GETOWN: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:473`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L473)*

### `F_SETOWN`
```rust
const F_SETOWN: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:474`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L474)*

### `F_SETLK`
```rust
const F_SETLK: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:475`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L475)*

### `F_SETLKW`
```rust
const F_SETLKW: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:476`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L476)*

### `F_OFD_GETLK`
```rust
const F_OFD_GETLK: crate::c_int = 36i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:477`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L477)*

### `F_OFD_SETLK`
```rust
const F_OFD_SETLK: crate::c_int = 37i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:478`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L478)*

### `F_OFD_SETLKW`
```rust
const F_OFD_SETLKW: crate::c_int = 38i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:479`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L479)*

### `F_RDLCK`
```rust
const F_RDLCK: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:481`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L481)*

### `F_WRLCK`
```rust
const F_WRLCK: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:482`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L482)*

### `F_UNLCK`
```rust
const F_UNLCK: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:483`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L483)*

### `SFD_NONBLOCK`
```rust
const SFD_NONBLOCK: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:485`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L485)*

### `TCSANOW`
```rust
const TCSANOW: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:487`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L487)*

### `TCSADRAIN`
```rust
const TCSADRAIN: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:488`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L488)*

### `TCSAFLUSH`
```rust
const TCSAFLUSH: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:489`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L489)*

### `SFD_CLOEXEC`
```rust
const SFD_CLOEXEC: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:491`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L491)*

### `NCCS`
```rust
const NCCS: usize = 32usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:493`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L493)*

### `O_TRUNC`
```rust
const O_TRUNC: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:495`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L495)*

### `O_CLOEXEC`
```rust
const O_CLOEXEC: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:497`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L497)*

### `EBFONT`
```rust
const EBFONT: crate::c_int = 59i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:499`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L499)*

### `ENOSTR`
```rust
const ENOSTR: crate::c_int = 60i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:500`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L500)*

### `ENODATA`
```rust
const ENODATA: crate::c_int = 61i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:501`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L501)*

### `ETIME`
```rust
const ETIME: crate::c_int = 62i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:502`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L502)*

### `ENOSR`
```rust
const ENOSR: crate::c_int = 63i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:503`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L503)*

### `ENONET`
```rust
const ENONET: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:504`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L504)*

### `ENOPKG`
```rust
const ENOPKG: crate::c_int = 65i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:505`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L505)*

### `EREMOTE`
```rust
const EREMOTE: crate::c_int = 66i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:506`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L506)*

### `ENOLINK`
```rust
const ENOLINK: crate::c_int = 67i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:507`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L507)*

### `EADV`
```rust
const EADV: crate::c_int = 68i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:508`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L508)*

### `ESRMNT`
```rust
const ESRMNT: crate::c_int = 69i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:509`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L509)*

### `ECOMM`
```rust
const ECOMM: crate::c_int = 70i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:510`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L510)*

### `EPROTO`
```rust
const EPROTO: crate::c_int = 71i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:511`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L511)*

### `EDOTDOT`
```rust
const EDOTDOT: crate::c_int = 73i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:512`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L512)*

### `SA_NODEFER`
```rust
const SA_NODEFER: crate::c_int = 1_073_741_824i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:514`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L514)*

### `SA_RESETHAND`
```rust
const SA_RESETHAND: crate::c_int = -2_147_483_648i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:515`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L515)*

### `SA_RESTART`
```rust
const SA_RESTART: crate::c_int = 268_435_456i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:516`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L516)*

### `SA_NOCLDSTOP`
```rust
const SA_NOCLDSTOP: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:517`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L517)*

### `EPOLL_CLOEXEC`
```rust
const EPOLL_CLOEXEC: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:519`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L519)*

### `EFD_CLOEXEC`
```rust
const EFD_CLOEXEC: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:521`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L521)*

### `__SIZEOF_PTHREAD_CONDATTR_T`
```rust
const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:523`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L523)*

### `__SIZEOF_PTHREAD_MUTEXATTR_T`
```rust
const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:524`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L524)*

### `__SIZEOF_PTHREAD_BARRIERATTR_T`
```rust
const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:525`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L525)*

### `O_DIRECT`
```rust
const O_DIRECT: crate::c_int = 16_384i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:527`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L527)*

### `O_DIRECTORY`
```rust
const O_DIRECTORY: crate::c_int = 65_536i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:528`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L528)*

### `O_NOFOLLOW`
```rust
const O_NOFOLLOW: crate::c_int = 131_072i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:529`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L529)*

### `MAP_HUGETLB`
```rust
const MAP_HUGETLB: crate::c_int = 262_144i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:531`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L531)*

### `MAP_LOCKED`
```rust
const MAP_LOCKED: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:532`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L532)*

### `MAP_NORESERVE`
```rust
const MAP_NORESERVE: crate::c_int = 16_384i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:533`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L533)*

### `MAP_32BIT`
```rust
const MAP_32BIT: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:534`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L534)*

### `MAP_ANON`
```rust
const MAP_ANON: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:535`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L535)*

### `MAP_ANONYMOUS`
```rust
const MAP_ANONYMOUS: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:536`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L536)*

### `MAP_DENYWRITE`
```rust
const MAP_DENYWRITE: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:537`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L537)*

### `MAP_EXECUTABLE`
```rust
const MAP_EXECUTABLE: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:538`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L538)*

### `MAP_POPULATE`
```rust
const MAP_POPULATE: crate::c_int = 32_768i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:539`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L539)*

### `MAP_NONBLOCK`
```rust
const MAP_NONBLOCK: crate::c_int = 65_536i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:540`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L540)*

### `MAP_STACK`
```rust
const MAP_STACK: crate::c_int = 131_072i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:541`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L541)*

### `MAP_SYNC`
```rust
const MAP_SYNC: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:542`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L542)*

### `EDEADLOCK`
```rust
const EDEADLOCK: crate::c_int = 35i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:544`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L544)*

### `EUCLEAN`
```rust
const EUCLEAN: crate::c_int = 117i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:545`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L545)*

### `ENOTNAM`
```rust
const ENOTNAM: crate::c_int = 118i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:546`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L546)*

### `ENAVAIL`
```rust
const ENAVAIL: crate::c_int = 119i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:547`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L547)*

### `EISNAM`
```rust
const EISNAM: crate::c_int = 120i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:548`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L548)*

### `EREMOTEIO`
```rust
const EREMOTEIO: crate::c_int = 121i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:549`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L549)*

### `PTRACE_GETFPREGS`
```rust
const PTRACE_GETFPREGS: crate::c_uint = 14u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:551`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L551)*

### `PTRACE_SETFPREGS`
```rust
const PTRACE_SETFPREGS: crate::c_uint = 15u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:552`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L552)*

### `PTRACE_GETFPXREGS`
```rust
const PTRACE_GETFPXREGS: crate::c_uint = 18u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:553`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L553)*

### `PTRACE_SETFPXREGS`
```rust
const PTRACE_SETFPXREGS: crate::c_uint = 19u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:554`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L554)*

### `PTRACE_GETREGS`
```rust
const PTRACE_GETREGS: crate::c_uint = 12u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:555`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L555)*

### `PTRACE_SETREGS`
```rust
const PTRACE_SETREGS: crate::c_uint = 13u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:556`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L556)*

### `PTRACE_PEEKSIGINFO_SHARED`
```rust
const PTRACE_PEEKSIGINFO_SHARED: crate::c_uint = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:557`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L557)*

### `PTRACE_SYSEMU`
```rust
const PTRACE_SYSEMU: crate::c_uint = 31u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:558`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L558)*

### `PTRACE_SYSEMU_SINGLESTEP`
```rust
const PTRACE_SYSEMU_SINGLESTEP: crate::c_uint = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:559`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L559)*

### `PR_GET_SPECULATION_CTRL`
```rust
const PR_GET_SPECULATION_CTRL: crate::c_int = 52i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:561`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L561)*

### `PR_SET_SPECULATION_CTRL`
```rust
const PR_SET_SPECULATION_CTRL: crate::c_int = 53i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:562`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L562)*

### `PR_SPEC_NOT_AFFECTED`
```rust
const PR_SPEC_NOT_AFFECTED: crate::c_uint = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:563`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L563)*

### `PR_SPEC_PRCTL`
```rust
const PR_SPEC_PRCTL: crate::c_uint = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:564`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L564)*

### `PR_SPEC_ENABLE`
```rust
const PR_SPEC_ENABLE: crate::c_uint = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:565`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L565)*

### `PR_SPEC_DISABLE`
```rust
const PR_SPEC_DISABLE: crate::c_uint = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:566`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L566)*

### `PR_SPEC_FORCE_DISABLE`
```rust
const PR_SPEC_FORCE_DISABLE: crate::c_uint = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:567`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L567)*

### `PR_SPEC_DISABLE_NOEXEC`
```rust
const PR_SPEC_DISABLE_NOEXEC: crate::c_uint = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:568`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L568)*

### `PR_SPEC_STORE_BYPASS`
```rust
const PR_SPEC_STORE_BYPASS: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:569`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L569)*

### `PR_SPEC_INDIRECT_BRANCH`
```rust
const PR_SPEC_INDIRECT_BRANCH: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:570`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L570)*

### `MCL_CURRENT`
```rust
const MCL_CURRENT: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:574`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L574)*

### `MCL_FUTURE`
```rust
const MCL_FUTURE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:575`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L575)*

### `MCL_ONFAULT`
```rust
const MCL_ONFAULT: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:576`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L576)*

### `SIGSTKSZ`
```rust
const SIGSTKSZ: crate::size_t = 8_192usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:578`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L578)*

### `MINSIGSTKSZ`
```rust
const MINSIGSTKSZ: crate::size_t = 2_048usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:579`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L579)*

### `CBAUD`
```rust
const CBAUD: crate::tcflag_t = 4_111u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:580`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L580)*

### `TAB1`
```rust
const TAB1: crate::tcflag_t = 2_048u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:581`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L581)*

### `TAB2`
```rust
const TAB2: crate::tcflag_t = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:582`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L582)*

### `TAB3`
```rust
const TAB3: crate::tcflag_t = 6_144u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:583`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L583)*

### `CR1`
```rust
const CR1: crate::tcflag_t = 512u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:584`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L584)*

### `CR2`
```rust
const CR2: crate::tcflag_t = 1_024u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:585`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L585)*

### `CR3`
```rust
const CR3: crate::tcflag_t = 1_536u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:586`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L586)*

### `FF1`
```rust
const FF1: crate::tcflag_t = 32_768u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:587`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L587)*

### `BS1`
```rust
const BS1: crate::tcflag_t = 8_192u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:588`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L588)*

### `VT1`
```rust
const VT1: crate::tcflag_t = 16_384u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:589`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L589)*

### `VWERASE`
```rust
const VWERASE: usize = 14usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:590`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L590)*

### `VREPRINT`
```rust
const VREPRINT: usize = 12usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:591`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L591)*

### `VSUSP`
```rust
const VSUSP: usize = 10usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:592`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L592)*

### `VSTART`
```rust
const VSTART: usize = 8usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:593`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L593)*

### `VSTOP`
```rust
const VSTOP: usize = 9usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:594`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L594)*

### `VDISCARD`
```rust
const VDISCARD: usize = 13usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:595`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L595)*

### `VTIME`
```rust
const VTIME: usize = 5usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:596`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L596)*

### `IXON`
```rust
const IXON: crate::tcflag_t = 1_024u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:597`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L597)*

### `IXOFF`
```rust
const IXOFF: crate::tcflag_t = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:598`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L598)*

### `ONLCR`
```rust
const ONLCR: crate::tcflag_t = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:599`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L599)*

### `CSIZE`
```rust
const CSIZE: crate::tcflag_t = 48u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:600`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L600)*

### `CS6`
```rust
const CS6: crate::tcflag_t = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:601`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L601)*

### `CS7`
```rust
const CS7: crate::tcflag_t = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:602`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L602)*

### `CS8`
```rust
const CS8: crate::tcflag_t = 48u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:603`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L603)*

### `CSTOPB`
```rust
const CSTOPB: crate::tcflag_t = 64u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:604`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L604)*

### `CREAD`
```rust
const CREAD: crate::tcflag_t = 128u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:605`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L605)*

### `PARENB`
```rust
const PARENB: crate::tcflag_t = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:606`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L606)*

### `PARODD`
```rust
const PARODD: crate::tcflag_t = 512u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:607`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L607)*

### `HUPCL`
```rust
const HUPCL: crate::tcflag_t = 1_024u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:608`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L608)*

### `CLOCAL`
```rust
const CLOCAL: crate::tcflag_t = 2_048u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:609`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L609)*

### `ECHOKE`
```rust
const ECHOKE: crate::tcflag_t = 2_048u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:610`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L610)*

### `ECHOE`
```rust
const ECHOE: crate::tcflag_t = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:611`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L611)*

### `ECHOK`
```rust
const ECHOK: crate::tcflag_t = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:612`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L612)*

### `ECHONL`
```rust
const ECHONL: crate::tcflag_t = 64u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:613`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L613)*

### `ECHOPRT`
```rust
const ECHOPRT: crate::tcflag_t = 1_024u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:614`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L614)*

### `ECHOCTL`
```rust
const ECHOCTL: crate::tcflag_t = 512u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:615`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L615)*

### `ISIG`
```rust
const ISIG: crate::tcflag_t = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:616`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L616)*

### `ICANON`
```rust
const ICANON: crate::tcflag_t = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:617`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L617)*

### `PENDIN`
```rust
const PENDIN: crate::tcflag_t = 16_384u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:618`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L618)*

### `NOFLSH`
```rust
const NOFLSH: crate::tcflag_t = 128u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:619`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L619)*

### `CIBAUD`
```rust
const CIBAUD: crate::tcflag_t = 269_418_496u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:620`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L620)*

### `CBAUDEX`
```rust
const CBAUDEX: crate::tcflag_t = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:621`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L621)*

### `VSWTC`
```rust
const VSWTC: usize = 7usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:622`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L622)*

### `OLCUC`
```rust
const OLCUC: crate::tcflag_t = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:623`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L623)*

### `NLDLY`
```rust
const NLDLY: crate::tcflag_t = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:624`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L624)*

### `CRDLY`
```rust
const CRDLY: crate::tcflag_t = 1_536u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:625`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L625)*

### `TABDLY`
```rust
const TABDLY: crate::tcflag_t = 6_144u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:626`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L626)*

### `BSDLY`
```rust
const BSDLY: crate::tcflag_t = 8_192u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:627`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L627)*

### `FFDLY`
```rust
const FFDLY: crate::tcflag_t = 32_768u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:628`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L628)*

### `VTDLY`
```rust
const VTDLY: crate::tcflag_t = 16_384u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:629`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L629)*

### `XTABS`
```rust
const XTABS: crate::tcflag_t = 6_144u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:630`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L630)*

### `B0`
```rust
const B0: crate::speed_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:632`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L632)*

### `B50`
```rust
const B50: crate::speed_t = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:633`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L633)*

### `B75`
```rust
const B75: crate::speed_t = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:634`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L634)*

### `B110`
```rust
const B110: crate::speed_t = 3u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:635`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L635)*

### `B134`
```rust
const B134: crate::speed_t = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:636`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L636)*

### `B150`
```rust
const B150: crate::speed_t = 5u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:637`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L637)*

### `B200`
```rust
const B200: crate::speed_t = 6u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:638`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L638)*

### `B300`
```rust
const B300: crate::speed_t = 7u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:639`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L639)*

### `B600`
```rust
const B600: crate::speed_t = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:640`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L640)*

### `B1200`
```rust
const B1200: crate::speed_t = 9u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:641`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L641)*

### `B1800`
```rust
const B1800: crate::speed_t = 10u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:642`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L642)*

### `B2400`
```rust
const B2400: crate::speed_t = 11u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:643`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L643)*

### `B4800`
```rust
const B4800: crate::speed_t = 12u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:644`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L644)*

### `B9600`
```rust
const B9600: crate::speed_t = 13u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:645`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L645)*

### `B19200`
```rust
const B19200: crate::speed_t = 14u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:646`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L646)*

### `B38400`
```rust
const B38400: crate::speed_t = 15u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:647`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L647)*

### `EXTA`
```rust
const EXTA: crate::speed_t = 14u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:648`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L648)*

### `EXTB`
```rust
const EXTB: crate::speed_t = 15u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:649`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L649)*

### `B57600`
```rust
const B57600: crate::speed_t = 4_097u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:650`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L650)*

### `B115200`
```rust
const B115200: crate::speed_t = 4_098u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:651`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L651)*

### `B230400`
```rust
const B230400: crate::speed_t = 4_099u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:652`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L652)*

### `B460800`
```rust
const B460800: crate::speed_t = 4_100u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:653`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L653)*

### `B500000`
```rust
const B500000: crate::speed_t = 4_101u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:654`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L654)*

### `B576000`
```rust
const B576000: crate::speed_t = 4_102u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:655`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L655)*

### `B921600`
```rust
const B921600: crate::speed_t = 4_103u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:656`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L656)*

### `B1000000`
```rust
const B1000000: crate::speed_t = 4_104u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:657`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L657)*

### `B1152000`
```rust
const B1152000: crate::speed_t = 4_105u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:658`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L658)*

### `B1500000`
```rust
const B1500000: crate::speed_t = 4_106u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:659`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L659)*

### `B2000000`
```rust
const B2000000: crate::speed_t = 4_107u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:660`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L660)*

### `B2500000`
```rust
const B2500000: crate::speed_t = 4_108u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:661`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L661)*

### `B3000000`
```rust
const B3000000: crate::speed_t = 4_109u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:662`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L662)*

### `B3500000`
```rust
const B3500000: crate::speed_t = 4_110u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:663`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L663)*

### `B4000000`
```rust
const B4000000: crate::speed_t = 4_111u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:664`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L664)*

### `VEOL`
```rust
const VEOL: usize = 11usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:666`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L666)*

### `VEOL2`
```rust
const VEOL2: usize = 16usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:667`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L667)*

### `VMIN`
```rust
const VMIN: usize = 6usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:668`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L668)*

### `IEXTEN`
```rust
const IEXTEN: crate::tcflag_t = 32_768u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:669`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L669)*

### `TOSTOP`
```rust
const TOSTOP: crate::tcflag_t = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:670`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L670)*

### `FLUSHO`
```rust
const FLUSHO: crate::tcflag_t = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:671`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L671)*

### `EXTPROC`
```rust
const EXTPROC: crate::tcflag_t = 65_536u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:672`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L672)*

### `R15`
```rust
const R15: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:675`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L675)*

### `R14`
```rust
const R14: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:676`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L676)*

### `R13`
```rust
const R13: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:677`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L677)*

### `R12`
```rust
const R12: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:678`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L678)*

### `RBP`
```rust
const RBP: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:679`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L679)*

### `RBX`
```rust
const RBX: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:680`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L680)*

### `R11`
```rust
const R11: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:681`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L681)*

### `R10`
```rust
const R10: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:682`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L682)*

### `R9`
```rust
const R9: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:683`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L683)*

### `R8`
```rust
const R8: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:684`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L684)*

### `RAX`
```rust
const RAX: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:685`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L685)*

### `RCX`
```rust
const RCX: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:686`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L686)*

### `RDX`
```rust
const RDX: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:687`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L687)*

### `RSI`
```rust
const RSI: crate::c_int = 13i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:688`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L688)*

### `RDI`
```rust
const RDI: crate::c_int = 14i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:689`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L689)*

### `ORIG_RAX`
```rust
const ORIG_RAX: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:690`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L690)*

### `RIP`
```rust
const RIP: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:691`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L691)*

### `CS`
```rust
const CS: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:692`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L692)*

### `EFLAGS`
```rust
const EFLAGS: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:693`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L693)*

### `RSP`
```rust
const RSP: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:694`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L694)*

### `SS`
```rust
const SS: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:695`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L695)*

### `FS_BASE`
```rust
const FS_BASE: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:696`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L696)*

### `GS_BASE`
```rust
const GS_BASE: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:697`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L697)*

### `DS`
```rust
const DS: crate::c_int = 23i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:698`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L698)*

### `ES`
```rust
const ES: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:699`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L699)*

### `FS`
```rust
const FS: crate::c_int = 25i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:700`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L700)*

### `GS`
```rust
const GS: crate::c_int = 26i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:701`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L701)*

### `REG_R8`
```rust
const REG_R8: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:704`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L704)*

### `REG_R9`
```rust
const REG_R9: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:705`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L705)*

### `REG_R10`
```rust
const REG_R10: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:706`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L706)*

### `REG_R11`
```rust
const REG_R11: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:707`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L707)*

### `REG_R12`
```rust
const REG_R12: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:708`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L708)*

### `REG_R13`
```rust
const REG_R13: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:709`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L709)*

### `REG_R14`
```rust
const REG_R14: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:710`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L710)*

### `REG_R15`
```rust
const REG_R15: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:711`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L711)*

### `REG_RDI`
```rust
const REG_RDI: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:712`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L712)*

### `REG_RSI`
```rust
const REG_RSI: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:713`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L713)*

### `REG_RBP`
```rust
const REG_RBP: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:714`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L714)*

### `REG_RBX`
```rust
const REG_RBX: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:715`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L715)*

### `REG_RDX`
```rust
const REG_RDX: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:716`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L716)*

### `REG_RAX`
```rust
const REG_RAX: crate::c_int = 13i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:717`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L717)*

### `REG_RCX`
```rust
const REG_RCX: crate::c_int = 14i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:718`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L718)*

### `REG_RSP`
```rust
const REG_RSP: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:719`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L719)*

### `REG_RIP`
```rust
const REG_RIP: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:720`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L720)*

### `REG_EFL`
```rust
const REG_EFL: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:721`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L721)*

### `REG_CSGSFS`
```rust
const REG_CSGSFS: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:722`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L722)*

### `REG_ERR`
```rust
const REG_ERR: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:723`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L723)*

### `REG_TRAPNO`
```rust
const REG_TRAPNO: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:724`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L724)*

### `REG_OLDMASK`
```rust
const REG_OLDMASK: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:725`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L725)*

### `REG_CR2`
```rust
const REG_CR2: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:726`](../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L726)*

