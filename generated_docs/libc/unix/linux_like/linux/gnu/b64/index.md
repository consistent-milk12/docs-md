*[libc](../../../../../index.md) / [unix](../../../../index.md) / [linux_like](../../../index.md) / [linux](../../index.md) / [gnu](../index.md) / [b64](index.md)*

---

# Module `b64`

64-bit specific definitions for linux-like values

## Contents

- [Modules](#modules)
  - [`x86_64`](#x86_64)
  - [`not_x32`](#not_x32)
- [Structs](#structs)
  - [`sigset_t`](#sigset_t)
  - [`sysinfo`](#sysinfo)
  - [`msqid_ds`](#msqid_ds)
  - [`semid_ds`](#semid_ds)
  - [`timex`](#timex)
  - [`sigaction`](#sigaction)
  - [`statfs`](#statfs)
  - [`flock`](#flock)
  - [`flock64`](#flock64)
  - [`siginfo_t`](#siginfo_t)
  - [`stack_t`](#stack_t)
  - [`stat`](#stat)
  - [`stat64`](#stat64)
  - [`statfs64`](#statfs64)
  - [`statvfs64`](#statvfs64)
  - [`pthread_attr_t`](#pthread_attr_t)
  - [`_libc_fpxreg`](#_libc_fpxreg)
  - [`_libc_xmmreg`](#_libc_xmmreg)
  - [`_libc_fpstate`](#_libc_fpstate)
  - [`user_regs_struct`](#user_regs_struct)
  - [`user`](#user)
  - [`mcontext_t`](#mcontext_t)
  - [`ipc_perm`](#ipc_perm)
  - [`shmid_ds`](#shmid_ds)
  - [`ptrace_rseq_configuration`](#ptrace_rseq_configuration)
  - [`clone_args`](#clone_args)
  - [`user_fpregs_struct`](#user_fpregs_struct)
  - [`ucontext_t`](#ucontext_t)
  - [`max_align_t`](#max_align_t)
- [Functions](#functions)
  - [`getcontext`](#getcontext)
  - [`setcontext`](#setcontext)
  - [`makecontext`](#makecontext)
  - [`swapcontext`](#swapcontext)
- [Type Aliases](#type-aliases)
  - [`ino_t`](#ino_t)
  - [`off_t`](#off_t)
  - [`blkcnt_t`](#blkcnt_t)
  - [`shmatt_t`](#shmatt_t)
  - [`msgqnum_t`](#msgqnum_t)
  - [`msglen_t`](#msglen_t)
  - [`fsblkcnt_t`](#fsblkcnt_t)
  - [`fsfilcnt_t`](#fsfilcnt_t)
  - [`rlim_t`](#rlim_t)
  - [`__syscall_ulong_t`](#__syscall_ulong_t)
  - [`__fsword_t`](#__fsword_t)
  - [`clock_t`](#clock_t)
  - [`time_t`](#time_t)
  - [`wchar_t`](#wchar_t)
  - [`nlink_t`](#nlink_t)
  - [`blksize_t`](#blksize_t)
  - [`greg_t`](#greg_t)
  - [`suseconds_t`](#suseconds_t)
  - [`__u64`](#__u64)
  - [`__s64`](#__s64)
- [Constants](#constants)
  - [`__SIZEOF_PTHREAD_RWLOCKATTR_T`](#__sizeof_pthread_rwlockattr_t)
  - [`O_LARGEFILE`](#o_largefile)
  - [`POSIX_FADV_DONTNEED`](#posix_fadv_dontneed)
  - [`POSIX_FADV_NOREUSE`](#posix_fadv_noreuse)
  - [`VEOF`](#veof)
  - [`RTLD_DEEPBIND`](#rtld_deepbind)
  - [`RTLD_GLOBAL`](#rtld_global)
  - [`RTLD_NOLOAD`](#rtld_noload)
  - [`O_APPEND`](#o_append)
  - [`O_CREAT`](#o_creat)
  - [`O_EXCL`](#o_excl)
  - [`O_NOCTTY`](#o_noctty)
  - [`O_NONBLOCK`](#o_nonblock)
  - [`O_SYNC`](#o_sync)
  - [`O_RSYNC`](#o_rsync)
  - [`O_DSYNC`](#o_dsync)
  - [`O_FSYNC`](#o_fsync)
  - [`O_NOATIME`](#o_noatime)
  - [`O_PATH`](#o_path)
  - [`O_TMPFILE`](#o_tmpfile)
  - [`MADV_SOFT_OFFLINE`](#madv_soft_offline)
  - [`MAP_GROWSDOWN`](#map_growsdown)
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
  - [`SOCK_STREAM`](#sock_stream)
  - [`SOCK_DGRAM`](#sock_dgram)
  - [`SA_ONSTACK`](#sa_onstack)
  - [`SA_SIGINFO`](#sa_siginfo)
  - [`SA_NOCLDWAIT`](#sa_nocldwait)
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
  - [`SIG_SETMASK`](#sig_setmask)
  - [`SIG_BLOCK`](#sig_block)
  - [`SIG_UNBLOCK`](#sig_unblock)
  - [`POLLWRNORM`](#pollwrnorm)
  - [`POLLWRBAND`](#pollwrband)
  - [`O_ASYNC`](#o_async)
  - [`O_NDELAY`](#o_ndelay)
  - [`PTRACE_DETACH`](#ptrace_detach)
  - [`PTRACE_GET_RSEQ_CONFIGURATION`](#ptrace_get_rseq_configuration)
  - [`EFD_NONBLOCK`](#efd_nonblock)
  - [`F_GETLK`](#f_getlk)
  - [`F_GETOWN`](#f_getown)
  - [`F_SETOWN`](#f_setown)
  - [`F_SETLK`](#f_setlk)
  - [`F_SETLKW`](#f_setlkw)
  - [`F_OFD_GETLK`](#f_ofd_getlk)
  - [`F_OFD_SETLK`](#f_ofd_setlk)
  - [`F_OFD_SETLKW`](#f_ofd_setlkw)
  - [`F_RDLCK`](#f_rdlck)
  - [`F_WRLCK`](#f_wrlck)
  - [`F_UNLCK`](#f_unlck)
  - [`SFD_NONBLOCK`](#sfd_nonblock)
  - [`TCSANOW`](#tcsanow)
  - [`TCSADRAIN`](#tcsadrain)
  - [`TCSAFLUSH`](#tcsaflush)
  - [`SFD_CLOEXEC`](#sfd_cloexec)
  - [`NCCS`](#nccs)
  - [`O_TRUNC`](#o_trunc)
  - [`O_CLOEXEC`](#o_cloexec)
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
  - [`SA_NODEFER`](#sa_nodefer)
  - [`SA_RESETHAND`](#sa_resethand)
  - [`SA_RESTART`](#sa_restart)
  - [`SA_NOCLDSTOP`](#sa_nocldstop)
  - [`EPOLL_CLOEXEC`](#epoll_cloexec)
  - [`EFD_CLOEXEC`](#efd_cloexec)
  - [`__SIZEOF_PTHREAD_CONDATTR_T`](#__sizeof_pthread_condattr_t)
  - [`__SIZEOF_PTHREAD_MUTEXATTR_T`](#__sizeof_pthread_mutexattr_t)
  - [`__SIZEOF_PTHREAD_BARRIERATTR_T`](#__sizeof_pthread_barrierattr_t)
  - [`O_DIRECT`](#o_direct)
  - [`O_DIRECTORY`](#o_directory)
  - [`O_NOFOLLOW`](#o_nofollow)
  - [`MAP_HUGETLB`](#map_hugetlb)
  - [`MAP_LOCKED`](#map_locked)
  - [`MAP_NORESERVE`](#map_noreserve)
  - [`MAP_32BIT`](#map_32bit)
  - [`MAP_ANON`](#map_anon)
  - [`MAP_ANONYMOUS`](#map_anonymous)
  - [`MAP_DENYWRITE`](#map_denywrite)
  - [`MAP_EXECUTABLE`](#map_executable)
  - [`MAP_POPULATE`](#map_populate)
  - [`MAP_NONBLOCK`](#map_nonblock)
  - [`MAP_STACK`](#map_stack)
  - [`MAP_SYNC`](#map_sync)
  - [`EDEADLOCK`](#edeadlock)
  - [`EUCLEAN`](#euclean)
  - [`ENOTNAM`](#enotnam)
  - [`ENAVAIL`](#enavail)
  - [`EISNAM`](#eisnam)
  - [`EREMOTEIO`](#eremoteio)
  - [`PTRACE_GETFPREGS`](#ptrace_getfpregs)
  - [`PTRACE_SETFPREGS`](#ptrace_setfpregs)
  - [`PTRACE_GETFPXREGS`](#ptrace_getfpxregs)
  - [`PTRACE_SETFPXREGS`](#ptrace_setfpxregs)
  - [`PTRACE_GETREGS`](#ptrace_getregs)
  - [`PTRACE_SETREGS`](#ptrace_setregs)
  - [`PTRACE_PEEKSIGINFO_SHARED`](#ptrace_peeksiginfo_shared)
  - [`PTRACE_SYSEMU`](#ptrace_sysemu)
  - [`PTRACE_SYSEMU_SINGLESTEP`](#ptrace_sysemu_singlestep)
  - [`PR_GET_SPECULATION_CTRL`](#pr_get_speculation_ctrl)
  - [`PR_SET_SPECULATION_CTRL`](#pr_set_speculation_ctrl)
  - [`PR_SPEC_NOT_AFFECTED`](#pr_spec_not_affected)
  - [`PR_SPEC_PRCTL`](#pr_spec_prctl)
  - [`PR_SPEC_ENABLE`](#pr_spec_enable)
  - [`PR_SPEC_DISABLE`](#pr_spec_disable)
  - [`PR_SPEC_FORCE_DISABLE`](#pr_spec_force_disable)
  - [`PR_SPEC_DISABLE_NOEXEC`](#pr_spec_disable_noexec)
  - [`PR_SPEC_STORE_BYPASS`](#pr_spec_store_bypass)
  - [`PR_SPEC_INDIRECT_BRANCH`](#pr_spec_indirect_branch)
  - [`MCL_CURRENT`](#mcl_current)
  - [`MCL_FUTURE`](#mcl_future)
  - [`MCL_ONFAULT`](#mcl_onfault)
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
  - [`ORIG_RAX`](#orig_rax)
  - [`RIP`](#rip)
  - [`CS`](#cs)
  - [`EFLAGS`](#eflags)
  - [`RSP`](#rsp)
  - [`SS`](#ss)
  - [`FS_BASE`](#fs_base)
  - [`GS_BASE`](#gs_base)
  - [`DS`](#ds)
  - [`ES`](#es)
  - [`FS`](#fs)
  - [`GS`](#gs)
  - [`REG_R8`](#reg_r8)
  - [`REG_R9`](#reg_r9)
  - [`REG_R10`](#reg_r10)
  - [`REG_R11`](#reg_r11)
  - [`REG_R12`](#reg_r12)
  - [`REG_R13`](#reg_r13)
  - [`REG_R14`](#reg_r14)
  - [`REG_R15`](#reg_r15)
  - [`REG_RDI`](#reg_rdi)
  - [`REG_RSI`](#reg_rsi)
  - [`REG_RBP`](#reg_rbp)
  - [`REG_RBX`](#reg_rbx)
  - [`REG_RDX`](#reg_rdx)
  - [`REG_RAX`](#reg_rax)
  - [`REG_RCX`](#reg_rcx)
  - [`REG_RSP`](#reg_rsp)
  - [`REG_RIP`](#reg_rip)
  - [`REG_EFL`](#reg_efl)
  - [`REG_CSGSFS`](#reg_csgsfs)
  - [`REG_ERR`](#reg_err)
  - [`REG_TRAPNO`](#reg_trapno)
  - [`REG_OLDMASK`](#reg_oldmask)
  - [`REG_CR2`](#reg_cr2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`x86_64`](#x86_64) | mod | x86_64-specific definitions for 64-bit linux-like values |
| [`not_x32`](#not_x32) | mod |  |
| [`sigset_t`](#sigset_t) | struct |  |
| [`sysinfo`](#sysinfo) | struct |  |
| [`msqid_ds`](#msqid_ds) | struct |  |
| [`semid_ds`](#semid_ds) | struct |  |
| [`timex`](#timex) | struct |  |
| [`sigaction`](#sigaction) | struct |  |
| [`statfs`](#statfs) | struct |  |
| [`flock`](#flock) | struct |  |
| [`flock64`](#flock64) | struct |  |
| [`siginfo_t`](#siginfo_t) | struct |  |
| [`stack_t`](#stack_t) | struct |  |
| [`stat`](#stat) | struct |  |
| [`stat64`](#stat64) | struct |  |
| [`statfs64`](#statfs64) | struct |  |
| [`statvfs64`](#statvfs64) | struct |  |
| [`pthread_attr_t`](#pthread_attr_t) | struct |  |
| [`_libc_fpxreg`](#_libc_fpxreg) | struct |  |
| [`_libc_xmmreg`](#_libc_xmmreg) | struct |  |
| [`_libc_fpstate`](#_libc_fpstate) | struct |  |
| [`user_regs_struct`](#user_regs_struct) | struct |  |
| [`user`](#user) | struct |  |
| [`mcontext_t`](#mcontext_t) | struct |  |
| [`ipc_perm`](#ipc_perm) | struct |  |
| [`shmid_ds`](#shmid_ds) | struct |  |
| [`ptrace_rseq_configuration`](#ptrace_rseq_configuration) | struct |  |
| [`clone_args`](#clone_args) | struct |  |
| [`user_fpregs_struct`](#user_fpregs_struct) | struct |  |
| [`ucontext_t`](#ucontext_t) | struct |  |
| [`max_align_t`](#max_align_t) | struct |  |
| [`getcontext`](#getcontext) | fn |  |
| [`setcontext`](#setcontext) | fn |  |
| [`makecontext`](#makecontext) | fn |  |
| [`swapcontext`](#swapcontext) | fn |  |
| [`ino_t`](#ino_t) | type |  |
| [`off_t`](#off_t) | type |  |
| [`blkcnt_t`](#blkcnt_t) | type |  |
| [`shmatt_t`](#shmatt_t) | type |  |
| [`msgqnum_t`](#msgqnum_t) | type |  |
| [`msglen_t`](#msglen_t) | type |  |
| [`fsblkcnt_t`](#fsblkcnt_t) | type |  |
| [`fsfilcnt_t`](#fsfilcnt_t) | type |  |
| [`rlim_t`](#rlim_t) | type |  |
| [`__syscall_ulong_t`](#__syscall_ulong_t) | type |  |
| [`__fsword_t`](#__fsword_t) | type |  |
| [`clock_t`](#clock_t) | type |  |
| [`time_t`](#time_t) | type |  |
| [`wchar_t`](#wchar_t) | type |  |
| [`nlink_t`](#nlink_t) | type |  |
| [`blksize_t`](#blksize_t) | type |  |
| [`greg_t`](#greg_t) | type |  |
| [`suseconds_t`](#suseconds_t) | type |  |
| [`__u64`](#__u64) | type |  |
| [`__s64`](#__s64) | type |  |
| [`__SIZEOF_PTHREAD_RWLOCKATTR_T`](#__sizeof_pthread_rwlockattr_t) | const |  |
| [`O_LARGEFILE`](#o_largefile) | const |  |
| [`POSIX_FADV_DONTNEED`](#posix_fadv_dontneed) | const |  |
| [`POSIX_FADV_NOREUSE`](#posix_fadv_noreuse) | const |  |
| [`VEOF`](#veof) | const |  |
| [`RTLD_DEEPBIND`](#rtld_deepbind) | const |  |
| [`RTLD_GLOBAL`](#rtld_global) | const |  |
| [`RTLD_NOLOAD`](#rtld_noload) | const |  |
| [`O_APPEND`](#o_append) | const |  |
| [`O_CREAT`](#o_creat) | const |  |
| [`O_EXCL`](#o_excl) | const |  |
| [`O_NOCTTY`](#o_noctty) | const |  |
| [`O_NONBLOCK`](#o_nonblock) | const |  |
| [`O_SYNC`](#o_sync) | const |  |
| [`O_RSYNC`](#o_rsync) | const |  |
| [`O_DSYNC`](#o_dsync) | const |  |
| [`O_FSYNC`](#o_fsync) | const |  |
| [`O_NOATIME`](#o_noatime) | const |  |
| [`O_PATH`](#o_path) | const |  |
| [`O_TMPFILE`](#o_tmpfile) | const |  |
| [`MADV_SOFT_OFFLINE`](#madv_soft_offline) | const |  |
| [`MAP_GROWSDOWN`](#map_growsdown) | const |  |
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
| [`SOCK_STREAM`](#sock_stream) | const |  |
| [`SOCK_DGRAM`](#sock_dgram) | const |  |
| [`SA_ONSTACK`](#sa_onstack) | const |  |
| [`SA_SIGINFO`](#sa_siginfo) | const |  |
| [`SA_NOCLDWAIT`](#sa_nocldwait) | const |  |
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
| [`SIG_SETMASK`](#sig_setmask) | const |  |
| [`SIG_BLOCK`](#sig_block) | const |  |
| [`SIG_UNBLOCK`](#sig_unblock) | const |  |
| [`POLLWRNORM`](#pollwrnorm) | const |  |
| [`POLLWRBAND`](#pollwrband) | const |  |
| [`O_ASYNC`](#o_async) | const |  |
| [`O_NDELAY`](#o_ndelay) | const |  |
| [`PTRACE_DETACH`](#ptrace_detach) | const |  |
| [`PTRACE_GET_RSEQ_CONFIGURATION`](#ptrace_get_rseq_configuration) | const |  |
| [`EFD_NONBLOCK`](#efd_nonblock) | const |  |
| [`F_GETLK`](#f_getlk) | const |  |
| [`F_GETOWN`](#f_getown) | const |  |
| [`F_SETOWN`](#f_setown) | const |  |
| [`F_SETLK`](#f_setlk) | const |  |
| [`F_SETLKW`](#f_setlkw) | const |  |
| [`F_OFD_GETLK`](#f_ofd_getlk) | const |  |
| [`F_OFD_SETLK`](#f_ofd_setlk) | const |  |
| [`F_OFD_SETLKW`](#f_ofd_setlkw) | const |  |
| [`F_RDLCK`](#f_rdlck) | const |  |
| [`F_WRLCK`](#f_wrlck) | const |  |
| [`F_UNLCK`](#f_unlck) | const |  |
| [`SFD_NONBLOCK`](#sfd_nonblock) | const |  |
| [`TCSANOW`](#tcsanow) | const |  |
| [`TCSADRAIN`](#tcsadrain) | const |  |
| [`TCSAFLUSH`](#tcsaflush) | const |  |
| [`SFD_CLOEXEC`](#sfd_cloexec) | const |  |
| [`NCCS`](#nccs) | const |  |
| [`O_TRUNC`](#o_trunc) | const |  |
| [`O_CLOEXEC`](#o_cloexec) | const |  |
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
| [`SA_NODEFER`](#sa_nodefer) | const |  |
| [`SA_RESETHAND`](#sa_resethand) | const |  |
| [`SA_RESTART`](#sa_restart) | const |  |
| [`SA_NOCLDSTOP`](#sa_nocldstop) | const |  |
| [`EPOLL_CLOEXEC`](#epoll_cloexec) | const |  |
| [`EFD_CLOEXEC`](#efd_cloexec) | const |  |
| [`__SIZEOF_PTHREAD_CONDATTR_T`](#__sizeof_pthread_condattr_t) | const |  |
| [`__SIZEOF_PTHREAD_MUTEXATTR_T`](#__sizeof_pthread_mutexattr_t) | const |  |
| [`__SIZEOF_PTHREAD_BARRIERATTR_T`](#__sizeof_pthread_barrierattr_t) | const |  |
| [`O_DIRECT`](#o_direct) | const |  |
| [`O_DIRECTORY`](#o_directory) | const |  |
| [`O_NOFOLLOW`](#o_nofollow) | const |  |
| [`MAP_HUGETLB`](#map_hugetlb) | const |  |
| [`MAP_LOCKED`](#map_locked) | const |  |
| [`MAP_NORESERVE`](#map_noreserve) | const |  |
| [`MAP_32BIT`](#map_32bit) | const |  |
| [`MAP_ANON`](#map_anon) | const |  |
| [`MAP_ANONYMOUS`](#map_anonymous) | const |  |
| [`MAP_DENYWRITE`](#map_denywrite) | const |  |
| [`MAP_EXECUTABLE`](#map_executable) | const |  |
| [`MAP_POPULATE`](#map_populate) | const |  |
| [`MAP_NONBLOCK`](#map_nonblock) | const |  |
| [`MAP_STACK`](#map_stack) | const |  |
| [`MAP_SYNC`](#map_sync) | const |  |
| [`EDEADLOCK`](#edeadlock) | const |  |
| [`EUCLEAN`](#euclean) | const |  |
| [`ENOTNAM`](#enotnam) | const |  |
| [`ENAVAIL`](#enavail) | const |  |
| [`EISNAM`](#eisnam) | const |  |
| [`EREMOTEIO`](#eremoteio) | const |  |
| [`PTRACE_GETFPREGS`](#ptrace_getfpregs) | const |  |
| [`PTRACE_SETFPREGS`](#ptrace_setfpregs) | const |  |
| [`PTRACE_GETFPXREGS`](#ptrace_getfpxregs) | const |  |
| [`PTRACE_SETFPXREGS`](#ptrace_setfpxregs) | const |  |
| [`PTRACE_GETREGS`](#ptrace_getregs) | const |  |
| [`PTRACE_SETREGS`](#ptrace_setregs) | const |  |
| [`PTRACE_PEEKSIGINFO_SHARED`](#ptrace_peeksiginfo_shared) | const |  |
| [`PTRACE_SYSEMU`](#ptrace_sysemu) | const |  |
| [`PTRACE_SYSEMU_SINGLESTEP`](#ptrace_sysemu_singlestep) | const |  |
| [`PR_GET_SPECULATION_CTRL`](#pr_get_speculation_ctrl) | const |  |
| [`PR_SET_SPECULATION_CTRL`](#pr_set_speculation_ctrl) | const |  |
| [`PR_SPEC_NOT_AFFECTED`](#pr_spec_not_affected) | const |  |
| [`PR_SPEC_PRCTL`](#pr_spec_prctl) | const |  |
| [`PR_SPEC_ENABLE`](#pr_spec_enable) | const |  |
| [`PR_SPEC_DISABLE`](#pr_spec_disable) | const |  |
| [`PR_SPEC_FORCE_DISABLE`](#pr_spec_force_disable) | const |  |
| [`PR_SPEC_DISABLE_NOEXEC`](#pr_spec_disable_noexec) | const |  |
| [`PR_SPEC_STORE_BYPASS`](#pr_spec_store_bypass) | const |  |
| [`PR_SPEC_INDIRECT_BRANCH`](#pr_spec_indirect_branch) | const |  |
| [`MCL_CURRENT`](#mcl_current) | const |  |
| [`MCL_FUTURE`](#mcl_future) | const |  |
| [`MCL_ONFAULT`](#mcl_onfault) | const |  |
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
| [`ORIG_RAX`](#orig_rax) | const |  |
| [`RIP`](#rip) | const |  |
| [`CS`](#cs) | const |  |
| [`EFLAGS`](#eflags) | const |  |
| [`RSP`](#rsp) | const |  |
| [`SS`](#ss) | const |  |
| [`FS_BASE`](#fs_base) | const |  |
| [`GS_BASE`](#gs_base) | const |  |
| [`DS`](#ds) | const |  |
| [`ES`](#es) | const |  |
| [`FS`](#fs) | const |  |
| [`GS`](#gs) | const |  |
| [`REG_R8`](#reg_r8) | const |  |
| [`REG_R9`](#reg_r9) | const |  |
| [`REG_R10`](#reg_r10) | const |  |
| [`REG_R11`](#reg_r11) | const |  |
| [`REG_R12`](#reg_r12) | const |  |
| [`REG_R13`](#reg_r13) | const |  |
| [`REG_R14`](#reg_r14) | const |  |
| [`REG_R15`](#reg_r15) | const |  |
| [`REG_RDI`](#reg_rdi) | const |  |
| [`REG_RSI`](#reg_rsi) | const |  |
| [`REG_RBP`](#reg_rbp) | const |  |
| [`REG_RBX`](#reg_rbx) | const |  |
| [`REG_RDX`](#reg_rdx) | const |  |
| [`REG_RAX`](#reg_rax) | const |  |
| [`REG_RCX`](#reg_rcx) | const |  |
| [`REG_RSP`](#reg_rsp) | const |  |
| [`REG_RIP`](#reg_rip) | const |  |
| [`REG_EFL`](#reg_efl) | const |  |
| [`REG_CSGSFS`](#reg_csgsfs) | const |  |
| [`REG_ERR`](#reg_err) | const |  |
| [`REG_TRAPNO`](#reg_trapno) | const |  |
| [`REG_OLDMASK`](#reg_oldmask) | const |  |
| [`REG_CR2`](#reg_cr2) | const |  |

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

#### Trait Implementations

##### `impl Clone for sigset_t`

- <span id="sigset-t-clone"></span>`fn clone(&self) -> sigset_t` — [`sigset_t`](../index.md)

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

#### Trait Implementations

##### `impl Clone for sysinfo`

- <span id="sysinfo-clone"></span>`fn clone(&self) -> sysinfo` — [`sysinfo`](../index.md)

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

#### Trait Implementations

##### `impl Clone for msqid_ds`

- <span id="msqid-ds-clone"></span>`fn clone(&self) -> msqid_ds` — [`msqid_ds`](../index.md)

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

#### Trait Implementations

##### `impl Clone for semid_ds`

- <span id="semid-ds-clone"></span>`fn clone(&self) -> semid_ds` — [`semid_ds`](../index.md)

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

#### Trait Implementations

##### `impl Clone for timex`

- <span id="timex-clone"></span>`fn clone(&self) -> timex` — [`timex`](../index.md)

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

#### Implementations

- <span id="siginfo-t-si-addr"></span>`unsafe fn si_addr(&self) -> *mut c_void` — [`c_void`](../../../../../index.md)

- <span id="siginfo-t-si-value"></span>`unsafe fn si_value(&self) -> crate::sigval` — [`sigval`](../../../../../index.md)

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

