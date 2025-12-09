*[libc](../../../../../../index.md) / [unix](../../../../../index.md) / [linux_like](../../../../index.md) / [linux](../../../index.md) / [gnu](../../index.md) / [b64](../index.md) / [x86_64](index.md)*

---

# Module `x86_64`

x86_64-specific definitions for 64-bit linux-like values

## Contents

- [Modules](#modules)
  - [`not_x32`](#not_x32)
- [Structs](#structs)
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
  - [`statvfs`](#statvfs)
- [Functions](#functions)
  - [`getcontext`](#getcontext)
  - [`setcontext`](#setcontext)
  - [`makecontext`](#makecontext)
  - [`swapcontext`](#swapcontext)
  - [`sysctl`](#sysctl)
- [Type Aliases](#type-aliases)
  - [`wchar_t`](#wchar_t)
  - [`nlink_t`](#nlink_t)
  - [`blksize_t`](#blksize_t)
  - [`greg_t`](#greg_t)
  - [`suseconds_t`](#suseconds_t)
  - [`__u64`](#__u64)
  - [`__s64`](#__s64)
- [Constants](#constants)
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
  - [`__SIZEOF_PTHREAD_MUTEX_T`](#__sizeof_pthread_mutex_t)
  - [`__SIZEOF_PTHREAD_RWLOCK_T`](#__sizeof_pthread_rwlock_t)
  - [`__SIZEOF_PTHREAD_BARRIER_T`](#__sizeof_pthread_barrier_t)
  - [`PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP`](#pthread_recursive_mutex_initializer_np)
  - [`PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP`](#pthread_errorcheck_mutex_initializer_np)
  - [`PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP`](#pthread_adaptive_mutex_initializer_np)
  - [`SYS_read`](#sys_read)
  - [`SYS_write`](#sys_write)
  - [`SYS_open`](#sys_open)
  - [`SYS_close`](#sys_close)
  - [`SYS_stat`](#sys_stat)
  - [`SYS_fstat`](#sys_fstat)
  - [`SYS_lstat`](#sys_lstat)
  - [`SYS_poll`](#sys_poll)
  - [`SYS_lseek`](#sys_lseek)
  - [`SYS_mmap`](#sys_mmap)
  - [`SYS_mprotect`](#sys_mprotect)
  - [`SYS_munmap`](#sys_munmap)
  - [`SYS_brk`](#sys_brk)
  - [`SYS_rt_sigaction`](#sys_rt_sigaction)
  - [`SYS_rt_sigprocmask`](#sys_rt_sigprocmask)
  - [`SYS_rt_sigreturn`](#sys_rt_sigreturn)
  - [`SYS_ioctl`](#sys_ioctl)
  - [`SYS_pread64`](#sys_pread64)
  - [`SYS_pwrite64`](#sys_pwrite64)
  - [`SYS_readv`](#sys_readv)
  - [`SYS_writev`](#sys_writev)
  - [`SYS_access`](#sys_access)
  - [`SYS_pipe`](#sys_pipe)
  - [`SYS_select`](#sys_select)
  - [`SYS_sched_yield`](#sys_sched_yield)
  - [`SYS_mremap`](#sys_mremap)
  - [`SYS_msync`](#sys_msync)
  - [`SYS_mincore`](#sys_mincore)
  - [`SYS_madvise`](#sys_madvise)
  - [`SYS_shmget`](#sys_shmget)
  - [`SYS_shmat`](#sys_shmat)
  - [`SYS_shmctl`](#sys_shmctl)
  - [`SYS_dup`](#sys_dup)
  - [`SYS_dup2`](#sys_dup2)
  - [`SYS_pause`](#sys_pause)
  - [`SYS_nanosleep`](#sys_nanosleep)
  - [`SYS_getitimer`](#sys_getitimer)
  - [`SYS_alarm`](#sys_alarm)
  - [`SYS_setitimer`](#sys_setitimer)
  - [`SYS_getpid`](#sys_getpid)
  - [`SYS_sendfile`](#sys_sendfile)
  - [`SYS_socket`](#sys_socket)
  - [`SYS_connect`](#sys_connect)
  - [`SYS_accept`](#sys_accept)
  - [`SYS_sendto`](#sys_sendto)
  - [`SYS_recvfrom`](#sys_recvfrom)
  - [`SYS_sendmsg`](#sys_sendmsg)
  - [`SYS_recvmsg`](#sys_recvmsg)
  - [`SYS_shutdown`](#sys_shutdown)
  - [`SYS_bind`](#sys_bind)
  - [`SYS_listen`](#sys_listen)
  - [`SYS_getsockname`](#sys_getsockname)
  - [`SYS_getpeername`](#sys_getpeername)
  - [`SYS_socketpair`](#sys_socketpair)
  - [`SYS_setsockopt`](#sys_setsockopt)
  - [`SYS_getsockopt`](#sys_getsockopt)
  - [`SYS_clone`](#sys_clone)
  - [`SYS_fork`](#sys_fork)
  - [`SYS_vfork`](#sys_vfork)
  - [`SYS_execve`](#sys_execve)
  - [`SYS_exit`](#sys_exit)
  - [`SYS_wait4`](#sys_wait4)
  - [`SYS_kill`](#sys_kill)
  - [`SYS_uname`](#sys_uname)
  - [`SYS_semget`](#sys_semget)
  - [`SYS_semop`](#sys_semop)
  - [`SYS_semctl`](#sys_semctl)
  - [`SYS_shmdt`](#sys_shmdt)
  - [`SYS_msgget`](#sys_msgget)
  - [`SYS_msgsnd`](#sys_msgsnd)
  - [`SYS_msgrcv`](#sys_msgrcv)
  - [`SYS_msgctl`](#sys_msgctl)
  - [`SYS_fcntl`](#sys_fcntl)
  - [`SYS_flock`](#sys_flock)
  - [`SYS_fsync`](#sys_fsync)
  - [`SYS_fdatasync`](#sys_fdatasync)
  - [`SYS_truncate`](#sys_truncate)
  - [`SYS_ftruncate`](#sys_ftruncate)
  - [`SYS_getdents`](#sys_getdents)
  - [`SYS_getcwd`](#sys_getcwd)
  - [`SYS_chdir`](#sys_chdir)
  - [`SYS_fchdir`](#sys_fchdir)
  - [`SYS_rename`](#sys_rename)
  - [`SYS_mkdir`](#sys_mkdir)
  - [`SYS_rmdir`](#sys_rmdir)
  - [`SYS_creat`](#sys_creat)
  - [`SYS_link`](#sys_link)
  - [`SYS_unlink`](#sys_unlink)
  - [`SYS_symlink`](#sys_symlink)
  - [`SYS_readlink`](#sys_readlink)
  - [`SYS_chmod`](#sys_chmod)
  - [`SYS_fchmod`](#sys_fchmod)
  - [`SYS_chown`](#sys_chown)
  - [`SYS_fchown`](#sys_fchown)
  - [`SYS_lchown`](#sys_lchown)
  - [`SYS_umask`](#sys_umask)
  - [`SYS_gettimeofday`](#sys_gettimeofday)
  - [`SYS_getrlimit`](#sys_getrlimit)
  - [`SYS_getrusage`](#sys_getrusage)
  - [`SYS_sysinfo`](#sys_sysinfo)
  - [`SYS_times`](#sys_times)
  - [`SYS_ptrace`](#sys_ptrace)
  - [`SYS_getuid`](#sys_getuid)
  - [`SYS_syslog`](#sys_syslog)
  - [`SYS_getgid`](#sys_getgid)
  - [`SYS_setuid`](#sys_setuid)
  - [`SYS_setgid`](#sys_setgid)
  - [`SYS_geteuid`](#sys_geteuid)
  - [`SYS_getegid`](#sys_getegid)
  - [`SYS_setpgid`](#sys_setpgid)
  - [`SYS_getppid`](#sys_getppid)
  - [`SYS_getpgrp`](#sys_getpgrp)
  - [`SYS_setsid`](#sys_setsid)
  - [`SYS_setreuid`](#sys_setreuid)
  - [`SYS_setregid`](#sys_setregid)
  - [`SYS_getgroups`](#sys_getgroups)
  - [`SYS_setgroups`](#sys_setgroups)
  - [`SYS_setresuid`](#sys_setresuid)
  - [`SYS_getresuid`](#sys_getresuid)
  - [`SYS_setresgid`](#sys_setresgid)
  - [`SYS_getresgid`](#sys_getresgid)
  - [`SYS_getpgid`](#sys_getpgid)
  - [`SYS_setfsuid`](#sys_setfsuid)
  - [`SYS_setfsgid`](#sys_setfsgid)
  - [`SYS_getsid`](#sys_getsid)
  - [`SYS_capget`](#sys_capget)
  - [`SYS_capset`](#sys_capset)
  - [`SYS_rt_sigpending`](#sys_rt_sigpending)
  - [`SYS_rt_sigtimedwait`](#sys_rt_sigtimedwait)
  - [`SYS_rt_sigqueueinfo`](#sys_rt_sigqueueinfo)
  - [`SYS_rt_sigsuspend`](#sys_rt_sigsuspend)
  - [`SYS_sigaltstack`](#sys_sigaltstack)
  - [`SYS_utime`](#sys_utime)
  - [`SYS_mknod`](#sys_mknod)
  - [`SYS_uselib`](#sys_uselib)
  - [`SYS_personality`](#sys_personality)
  - [`SYS_ustat`](#sys_ustat)
  - [`SYS_statfs`](#sys_statfs)
  - [`SYS_fstatfs`](#sys_fstatfs)
  - [`SYS_sysfs`](#sys_sysfs)
  - [`SYS_getpriority`](#sys_getpriority)
  - [`SYS_setpriority`](#sys_setpriority)
  - [`SYS_sched_setparam`](#sys_sched_setparam)
  - [`SYS_sched_getparam`](#sys_sched_getparam)
  - [`SYS_sched_setscheduler`](#sys_sched_setscheduler)
  - [`SYS_sched_getscheduler`](#sys_sched_getscheduler)
  - [`SYS_sched_get_priority_max`](#sys_sched_get_priority_max)
  - [`SYS_sched_get_priority_min`](#sys_sched_get_priority_min)
  - [`SYS_sched_rr_get_interval`](#sys_sched_rr_get_interval)
  - [`SYS_mlock`](#sys_mlock)
  - [`SYS_munlock`](#sys_munlock)
  - [`SYS_mlockall`](#sys_mlockall)
  - [`SYS_munlockall`](#sys_munlockall)
  - [`SYS_vhangup`](#sys_vhangup)
  - [`SYS_modify_ldt`](#sys_modify_ldt)
  - [`SYS_pivot_root`](#sys_pivot_root)
  - [`SYS__sysctl`](#sys__sysctl)
  - [`SYS_prctl`](#sys_prctl)
  - [`SYS_arch_prctl`](#sys_arch_prctl)
  - [`SYS_adjtimex`](#sys_adjtimex)
  - [`SYS_setrlimit`](#sys_setrlimit)
  - [`SYS_chroot`](#sys_chroot)
  - [`SYS_sync`](#sys_sync)
  - [`SYS_acct`](#sys_acct)
  - [`SYS_settimeofday`](#sys_settimeofday)
  - [`SYS_mount`](#sys_mount)
  - [`SYS_umount2`](#sys_umount2)
  - [`SYS_swapon`](#sys_swapon)
  - [`SYS_swapoff`](#sys_swapoff)
  - [`SYS_reboot`](#sys_reboot)
  - [`SYS_sethostname`](#sys_sethostname)
  - [`SYS_setdomainname`](#sys_setdomainname)
  - [`SYS_iopl`](#sys_iopl)
  - [`SYS_ioperm`](#sys_ioperm)
  - [`SYS_create_module`](#sys_create_module)
  - [`SYS_init_module`](#sys_init_module)
  - [`SYS_delete_module`](#sys_delete_module)
  - [`SYS_get_kernel_syms`](#sys_get_kernel_syms)
  - [`SYS_query_module`](#sys_query_module)
  - [`SYS_quotactl`](#sys_quotactl)
  - [`SYS_nfsservctl`](#sys_nfsservctl)
  - [`SYS_getpmsg`](#sys_getpmsg)
  - [`SYS_putpmsg`](#sys_putpmsg)
  - [`SYS_afs_syscall`](#sys_afs_syscall)
  - [`SYS_tuxcall`](#sys_tuxcall)
  - [`SYS_security`](#sys_security)
  - [`SYS_gettid`](#sys_gettid)
  - [`SYS_readahead`](#sys_readahead)
  - [`SYS_setxattr`](#sys_setxattr)
  - [`SYS_lsetxattr`](#sys_lsetxattr)
  - [`SYS_fsetxattr`](#sys_fsetxattr)
  - [`SYS_getxattr`](#sys_getxattr)
  - [`SYS_lgetxattr`](#sys_lgetxattr)
  - [`SYS_fgetxattr`](#sys_fgetxattr)
  - [`SYS_listxattr`](#sys_listxattr)
  - [`SYS_llistxattr`](#sys_llistxattr)
  - [`SYS_flistxattr`](#sys_flistxattr)
  - [`SYS_removexattr`](#sys_removexattr)
  - [`SYS_lremovexattr`](#sys_lremovexattr)
  - [`SYS_fremovexattr`](#sys_fremovexattr)
  - [`SYS_tkill`](#sys_tkill)
  - [`SYS_time`](#sys_time)
  - [`SYS_futex`](#sys_futex)
  - [`SYS_sched_setaffinity`](#sys_sched_setaffinity)
  - [`SYS_sched_getaffinity`](#sys_sched_getaffinity)
  - [`SYS_set_thread_area`](#sys_set_thread_area)
  - [`SYS_io_setup`](#sys_io_setup)
  - [`SYS_io_destroy`](#sys_io_destroy)
  - [`SYS_io_getevents`](#sys_io_getevents)
  - [`SYS_io_submit`](#sys_io_submit)
  - [`SYS_io_cancel`](#sys_io_cancel)
  - [`SYS_get_thread_area`](#sys_get_thread_area)
  - [`SYS_lookup_dcookie`](#sys_lookup_dcookie)
  - [`SYS_epoll_create`](#sys_epoll_create)
  - [`SYS_epoll_ctl_old`](#sys_epoll_ctl_old)
  - [`SYS_epoll_wait_old`](#sys_epoll_wait_old)
  - [`SYS_remap_file_pages`](#sys_remap_file_pages)
  - [`SYS_getdents64`](#sys_getdents64)
  - [`SYS_set_tid_address`](#sys_set_tid_address)
  - [`SYS_restart_syscall`](#sys_restart_syscall)
  - [`SYS_semtimedop`](#sys_semtimedop)
  - [`SYS_fadvise64`](#sys_fadvise64)
  - [`SYS_timer_create`](#sys_timer_create)
  - [`SYS_timer_settime`](#sys_timer_settime)
  - [`SYS_timer_gettime`](#sys_timer_gettime)
  - [`SYS_timer_getoverrun`](#sys_timer_getoverrun)
  - [`SYS_timer_delete`](#sys_timer_delete)
  - [`SYS_clock_settime`](#sys_clock_settime)
  - [`SYS_clock_gettime`](#sys_clock_gettime)
  - [`SYS_clock_getres`](#sys_clock_getres)
  - [`SYS_clock_nanosleep`](#sys_clock_nanosleep)
  - [`SYS_exit_group`](#sys_exit_group)
  - [`SYS_epoll_wait`](#sys_epoll_wait)
  - [`SYS_epoll_ctl`](#sys_epoll_ctl)
  - [`SYS_tgkill`](#sys_tgkill)
  - [`SYS_utimes`](#sys_utimes)
  - [`SYS_vserver`](#sys_vserver)
  - [`SYS_mbind`](#sys_mbind)
  - [`SYS_set_mempolicy`](#sys_set_mempolicy)
  - [`SYS_get_mempolicy`](#sys_get_mempolicy)
  - [`SYS_mq_open`](#sys_mq_open)
  - [`SYS_mq_unlink`](#sys_mq_unlink)
  - [`SYS_mq_timedsend`](#sys_mq_timedsend)
  - [`SYS_mq_timedreceive`](#sys_mq_timedreceive)
  - [`SYS_mq_notify`](#sys_mq_notify)
  - [`SYS_mq_getsetattr`](#sys_mq_getsetattr)
  - [`SYS_kexec_load`](#sys_kexec_load)
  - [`SYS_waitid`](#sys_waitid)
  - [`SYS_add_key`](#sys_add_key)
  - [`SYS_request_key`](#sys_request_key)
  - [`SYS_keyctl`](#sys_keyctl)
  - [`SYS_ioprio_set`](#sys_ioprio_set)
  - [`SYS_ioprio_get`](#sys_ioprio_get)
  - [`SYS_inotify_init`](#sys_inotify_init)
  - [`SYS_inotify_add_watch`](#sys_inotify_add_watch)
  - [`SYS_inotify_rm_watch`](#sys_inotify_rm_watch)
  - [`SYS_migrate_pages`](#sys_migrate_pages)
  - [`SYS_openat`](#sys_openat)
  - [`SYS_mkdirat`](#sys_mkdirat)
  - [`SYS_mknodat`](#sys_mknodat)
  - [`SYS_fchownat`](#sys_fchownat)
  - [`SYS_futimesat`](#sys_futimesat)
  - [`SYS_newfstatat`](#sys_newfstatat)
  - [`SYS_unlinkat`](#sys_unlinkat)
  - [`SYS_renameat`](#sys_renameat)
  - [`SYS_linkat`](#sys_linkat)
  - [`SYS_symlinkat`](#sys_symlinkat)
  - [`SYS_readlinkat`](#sys_readlinkat)
  - [`SYS_fchmodat`](#sys_fchmodat)
  - [`SYS_faccessat`](#sys_faccessat)
  - [`SYS_pselect6`](#sys_pselect6)
  - [`SYS_ppoll`](#sys_ppoll)
  - [`SYS_unshare`](#sys_unshare)
  - [`SYS_set_robust_list`](#sys_set_robust_list)
  - [`SYS_get_robust_list`](#sys_get_robust_list)
  - [`SYS_splice`](#sys_splice)
  - [`SYS_tee`](#sys_tee)
  - [`SYS_sync_file_range`](#sys_sync_file_range)
  - [`SYS_vmsplice`](#sys_vmsplice)
  - [`SYS_move_pages`](#sys_move_pages)
  - [`SYS_utimensat`](#sys_utimensat)
  - [`SYS_epoll_pwait`](#sys_epoll_pwait)
  - [`SYS_signalfd`](#sys_signalfd)
  - [`SYS_timerfd_create`](#sys_timerfd_create)
  - [`SYS_eventfd`](#sys_eventfd)
  - [`SYS_fallocate`](#sys_fallocate)
  - [`SYS_timerfd_settime`](#sys_timerfd_settime)
  - [`SYS_timerfd_gettime`](#sys_timerfd_gettime)
  - [`SYS_accept4`](#sys_accept4)
  - [`SYS_signalfd4`](#sys_signalfd4)
  - [`SYS_eventfd2`](#sys_eventfd2)
  - [`SYS_epoll_create1`](#sys_epoll_create1)
  - [`SYS_dup3`](#sys_dup3)
  - [`SYS_pipe2`](#sys_pipe2)
  - [`SYS_inotify_init1`](#sys_inotify_init1)
  - [`SYS_preadv`](#sys_preadv)
  - [`SYS_pwritev`](#sys_pwritev)
  - [`SYS_rt_tgsigqueueinfo`](#sys_rt_tgsigqueueinfo)
  - [`SYS_perf_event_open`](#sys_perf_event_open)
  - [`SYS_recvmmsg`](#sys_recvmmsg)
  - [`SYS_fanotify_init`](#sys_fanotify_init)
  - [`SYS_fanotify_mark`](#sys_fanotify_mark)
  - [`SYS_prlimit64`](#sys_prlimit64)
  - [`SYS_name_to_handle_at`](#sys_name_to_handle_at)
  - [`SYS_open_by_handle_at`](#sys_open_by_handle_at)
  - [`SYS_clock_adjtime`](#sys_clock_adjtime)
  - [`SYS_syncfs`](#sys_syncfs)
  - [`SYS_sendmmsg`](#sys_sendmmsg)
  - [`SYS_setns`](#sys_setns)
  - [`SYS_getcpu`](#sys_getcpu)
  - [`SYS_process_vm_readv`](#sys_process_vm_readv)
  - [`SYS_process_vm_writev`](#sys_process_vm_writev)
  - [`SYS_kcmp`](#sys_kcmp)
  - [`SYS_finit_module`](#sys_finit_module)
  - [`SYS_sched_setattr`](#sys_sched_setattr)
  - [`SYS_sched_getattr`](#sys_sched_getattr)
  - [`SYS_renameat2`](#sys_renameat2)
  - [`SYS_seccomp`](#sys_seccomp)
  - [`SYS_getrandom`](#sys_getrandom)
  - [`SYS_memfd_create`](#sys_memfd_create)
  - [`SYS_kexec_file_load`](#sys_kexec_file_load)
  - [`SYS_bpf`](#sys_bpf)
  - [`SYS_execveat`](#sys_execveat)
  - [`SYS_userfaultfd`](#sys_userfaultfd)
  - [`SYS_membarrier`](#sys_membarrier)
  - [`SYS_mlock2`](#sys_mlock2)
  - [`SYS_copy_file_range`](#sys_copy_file_range)
  - [`SYS_preadv2`](#sys_preadv2)
  - [`SYS_pwritev2`](#sys_pwritev2)
  - [`SYS_pkey_mprotect`](#sys_pkey_mprotect)
  - [`SYS_pkey_alloc`](#sys_pkey_alloc)
  - [`SYS_pkey_free`](#sys_pkey_free)
  - [`SYS_statx`](#sys_statx)
  - [`SYS_rseq`](#sys_rseq)
  - [`SYS_pidfd_send_signal`](#sys_pidfd_send_signal)
  - [`SYS_io_uring_setup`](#sys_io_uring_setup)
  - [`SYS_io_uring_enter`](#sys_io_uring_enter)
  - [`SYS_io_uring_register`](#sys_io_uring_register)
  - [`SYS_open_tree`](#sys_open_tree)
  - [`SYS_move_mount`](#sys_move_mount)
  - [`SYS_fsopen`](#sys_fsopen)
  - [`SYS_fsconfig`](#sys_fsconfig)
  - [`SYS_fsmount`](#sys_fsmount)
  - [`SYS_fspick`](#sys_fspick)
  - [`SYS_pidfd_open`](#sys_pidfd_open)
  - [`SYS_clone3`](#sys_clone3)
  - [`SYS_close_range`](#sys_close_range)
  - [`SYS_openat2`](#sys_openat2)
  - [`SYS_pidfd_getfd`](#sys_pidfd_getfd)
  - [`SYS_faccessat2`](#sys_faccessat2)
  - [`SYS_process_madvise`](#sys_process_madvise)
  - [`SYS_epoll_pwait2`](#sys_epoll_pwait2)
  - [`SYS_mount_setattr`](#sys_mount_setattr)
  - [`SYS_quotactl_fd`](#sys_quotactl_fd)
  - [`SYS_landlock_create_ruleset`](#sys_landlock_create_ruleset)
  - [`SYS_landlock_add_rule`](#sys_landlock_add_rule)
  - [`SYS_landlock_restrict_self`](#sys_landlock_restrict_self)
  - [`SYS_memfd_secret`](#sys_memfd_secret)
  - [`SYS_process_mrelease`](#sys_process_mrelease)
  - [`SYS_futex_waitv`](#sys_futex_waitv)
  - [`SYS_set_mempolicy_home_node`](#sys_set_mempolicy_home_node)
  - [`SYS_fchmodat2`](#sys_fchmodat2)
  - [`SYS_mseal`](#sys_mseal)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`not_x32`](#not_x32) | mod |  |
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
| [`statvfs`](#statvfs) | struct |  |
| [`getcontext`](#getcontext) | fn |  |
| [`setcontext`](#setcontext) | fn |  |
| [`makecontext`](#makecontext) | fn |  |
| [`swapcontext`](#swapcontext) | fn |  |
| [`sysctl`](#sysctl) | fn |  |
| [`wchar_t`](#wchar_t) | type |  |
| [`nlink_t`](#nlink_t) | type |  |
| [`blksize_t`](#blksize_t) | type |  |
| [`greg_t`](#greg_t) | type |  |
| [`suseconds_t`](#suseconds_t) | type |  |
| [`__u64`](#__u64) | type |  |
| [`__s64`](#__s64) | type |  |
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
| [`__SIZEOF_PTHREAD_MUTEX_T`](#__sizeof_pthread_mutex_t) | const |  |
| [`__SIZEOF_PTHREAD_RWLOCK_T`](#__sizeof_pthread_rwlock_t) | const |  |
| [`__SIZEOF_PTHREAD_BARRIER_T`](#__sizeof_pthread_barrier_t) | const |  |
| [`PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP`](#pthread_recursive_mutex_initializer_np) | const |  |
| [`PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP`](#pthread_errorcheck_mutex_initializer_np) | const |  |
| [`PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP`](#pthread_adaptive_mutex_initializer_np) | const |  |
| [`SYS_read`](#sys_read) | const |  |
| [`SYS_write`](#sys_write) | const |  |
| [`SYS_open`](#sys_open) | const |  |
| [`SYS_close`](#sys_close) | const |  |
| [`SYS_stat`](#sys_stat) | const |  |
| [`SYS_fstat`](#sys_fstat) | const |  |
| [`SYS_lstat`](#sys_lstat) | const |  |
| [`SYS_poll`](#sys_poll) | const |  |
| [`SYS_lseek`](#sys_lseek) | const |  |
| [`SYS_mmap`](#sys_mmap) | const |  |
| [`SYS_mprotect`](#sys_mprotect) | const |  |
| [`SYS_munmap`](#sys_munmap) | const |  |
| [`SYS_brk`](#sys_brk) | const |  |
| [`SYS_rt_sigaction`](#sys_rt_sigaction) | const |  |
| [`SYS_rt_sigprocmask`](#sys_rt_sigprocmask) | const |  |
| [`SYS_rt_sigreturn`](#sys_rt_sigreturn) | const |  |
| [`SYS_ioctl`](#sys_ioctl) | const |  |
| [`SYS_pread64`](#sys_pread64) | const |  |
| [`SYS_pwrite64`](#sys_pwrite64) | const |  |
| [`SYS_readv`](#sys_readv) | const |  |
| [`SYS_writev`](#sys_writev) | const |  |
| [`SYS_access`](#sys_access) | const |  |
| [`SYS_pipe`](#sys_pipe) | const |  |
| [`SYS_select`](#sys_select) | const |  |
| [`SYS_sched_yield`](#sys_sched_yield) | const |  |
| [`SYS_mremap`](#sys_mremap) | const |  |
| [`SYS_msync`](#sys_msync) | const |  |
| [`SYS_mincore`](#sys_mincore) | const |  |
| [`SYS_madvise`](#sys_madvise) | const |  |
| [`SYS_shmget`](#sys_shmget) | const |  |
| [`SYS_shmat`](#sys_shmat) | const |  |
| [`SYS_shmctl`](#sys_shmctl) | const |  |
| [`SYS_dup`](#sys_dup) | const |  |
| [`SYS_dup2`](#sys_dup2) | const |  |
| [`SYS_pause`](#sys_pause) | const |  |
| [`SYS_nanosleep`](#sys_nanosleep) | const |  |
| [`SYS_getitimer`](#sys_getitimer) | const |  |
| [`SYS_alarm`](#sys_alarm) | const |  |
| [`SYS_setitimer`](#sys_setitimer) | const |  |
| [`SYS_getpid`](#sys_getpid) | const |  |
| [`SYS_sendfile`](#sys_sendfile) | const |  |
| [`SYS_socket`](#sys_socket) | const |  |
| [`SYS_connect`](#sys_connect) | const |  |
| [`SYS_accept`](#sys_accept) | const |  |
| [`SYS_sendto`](#sys_sendto) | const |  |
| [`SYS_recvfrom`](#sys_recvfrom) | const |  |
| [`SYS_sendmsg`](#sys_sendmsg) | const |  |
| [`SYS_recvmsg`](#sys_recvmsg) | const |  |
| [`SYS_shutdown`](#sys_shutdown) | const |  |
| [`SYS_bind`](#sys_bind) | const |  |
| [`SYS_listen`](#sys_listen) | const |  |
| [`SYS_getsockname`](#sys_getsockname) | const |  |
| [`SYS_getpeername`](#sys_getpeername) | const |  |
| [`SYS_socketpair`](#sys_socketpair) | const |  |
| [`SYS_setsockopt`](#sys_setsockopt) | const |  |
| [`SYS_getsockopt`](#sys_getsockopt) | const |  |
| [`SYS_clone`](#sys_clone) | const |  |
| [`SYS_fork`](#sys_fork) | const |  |
| [`SYS_vfork`](#sys_vfork) | const |  |
| [`SYS_execve`](#sys_execve) | const |  |
| [`SYS_exit`](#sys_exit) | const |  |
| [`SYS_wait4`](#sys_wait4) | const |  |
| [`SYS_kill`](#sys_kill) | const |  |
| [`SYS_uname`](#sys_uname) | const |  |
| [`SYS_semget`](#sys_semget) | const |  |
| [`SYS_semop`](#sys_semop) | const |  |
| [`SYS_semctl`](#sys_semctl) | const |  |
| [`SYS_shmdt`](#sys_shmdt) | const |  |
| [`SYS_msgget`](#sys_msgget) | const |  |
| [`SYS_msgsnd`](#sys_msgsnd) | const |  |
| [`SYS_msgrcv`](#sys_msgrcv) | const |  |
| [`SYS_msgctl`](#sys_msgctl) | const |  |
| [`SYS_fcntl`](#sys_fcntl) | const |  |
| [`SYS_flock`](#sys_flock) | const |  |
| [`SYS_fsync`](#sys_fsync) | const |  |
| [`SYS_fdatasync`](#sys_fdatasync) | const |  |
| [`SYS_truncate`](#sys_truncate) | const |  |
| [`SYS_ftruncate`](#sys_ftruncate) | const |  |
| [`SYS_getdents`](#sys_getdents) | const |  |
| [`SYS_getcwd`](#sys_getcwd) | const |  |
| [`SYS_chdir`](#sys_chdir) | const |  |
| [`SYS_fchdir`](#sys_fchdir) | const |  |
| [`SYS_rename`](#sys_rename) | const |  |
| [`SYS_mkdir`](#sys_mkdir) | const |  |
| [`SYS_rmdir`](#sys_rmdir) | const |  |
| [`SYS_creat`](#sys_creat) | const |  |
| [`SYS_link`](#sys_link) | const |  |
| [`SYS_unlink`](#sys_unlink) | const |  |
| [`SYS_symlink`](#sys_symlink) | const |  |
| [`SYS_readlink`](#sys_readlink) | const |  |
| [`SYS_chmod`](#sys_chmod) | const |  |
| [`SYS_fchmod`](#sys_fchmod) | const |  |
| [`SYS_chown`](#sys_chown) | const |  |
| [`SYS_fchown`](#sys_fchown) | const |  |
| [`SYS_lchown`](#sys_lchown) | const |  |
| [`SYS_umask`](#sys_umask) | const |  |
| [`SYS_gettimeofday`](#sys_gettimeofday) | const |  |
| [`SYS_getrlimit`](#sys_getrlimit) | const |  |
| [`SYS_getrusage`](#sys_getrusage) | const |  |
| [`SYS_sysinfo`](#sys_sysinfo) | const |  |
| [`SYS_times`](#sys_times) | const |  |
| [`SYS_ptrace`](#sys_ptrace) | const |  |
| [`SYS_getuid`](#sys_getuid) | const |  |
| [`SYS_syslog`](#sys_syslog) | const |  |
| [`SYS_getgid`](#sys_getgid) | const |  |
| [`SYS_setuid`](#sys_setuid) | const |  |
| [`SYS_setgid`](#sys_setgid) | const |  |
| [`SYS_geteuid`](#sys_geteuid) | const |  |
| [`SYS_getegid`](#sys_getegid) | const |  |
| [`SYS_setpgid`](#sys_setpgid) | const |  |
| [`SYS_getppid`](#sys_getppid) | const |  |
| [`SYS_getpgrp`](#sys_getpgrp) | const |  |
| [`SYS_setsid`](#sys_setsid) | const |  |
| [`SYS_setreuid`](#sys_setreuid) | const |  |
| [`SYS_setregid`](#sys_setregid) | const |  |
| [`SYS_getgroups`](#sys_getgroups) | const |  |
| [`SYS_setgroups`](#sys_setgroups) | const |  |
| [`SYS_setresuid`](#sys_setresuid) | const |  |
| [`SYS_getresuid`](#sys_getresuid) | const |  |
| [`SYS_setresgid`](#sys_setresgid) | const |  |
| [`SYS_getresgid`](#sys_getresgid) | const |  |
| [`SYS_getpgid`](#sys_getpgid) | const |  |
| [`SYS_setfsuid`](#sys_setfsuid) | const |  |
| [`SYS_setfsgid`](#sys_setfsgid) | const |  |
| [`SYS_getsid`](#sys_getsid) | const |  |
| [`SYS_capget`](#sys_capget) | const |  |
| [`SYS_capset`](#sys_capset) | const |  |
| [`SYS_rt_sigpending`](#sys_rt_sigpending) | const |  |
| [`SYS_rt_sigtimedwait`](#sys_rt_sigtimedwait) | const |  |
| [`SYS_rt_sigqueueinfo`](#sys_rt_sigqueueinfo) | const |  |
| [`SYS_rt_sigsuspend`](#sys_rt_sigsuspend) | const |  |
| [`SYS_sigaltstack`](#sys_sigaltstack) | const |  |
| [`SYS_utime`](#sys_utime) | const |  |
| [`SYS_mknod`](#sys_mknod) | const |  |
| [`SYS_uselib`](#sys_uselib) | const |  |
| [`SYS_personality`](#sys_personality) | const |  |
| [`SYS_ustat`](#sys_ustat) | const |  |
| [`SYS_statfs`](#sys_statfs) | const |  |
| [`SYS_fstatfs`](#sys_fstatfs) | const |  |
| [`SYS_sysfs`](#sys_sysfs) | const |  |
| [`SYS_getpriority`](#sys_getpriority) | const |  |
| [`SYS_setpriority`](#sys_setpriority) | const |  |
| [`SYS_sched_setparam`](#sys_sched_setparam) | const |  |
| [`SYS_sched_getparam`](#sys_sched_getparam) | const |  |
| [`SYS_sched_setscheduler`](#sys_sched_setscheduler) | const |  |
| [`SYS_sched_getscheduler`](#sys_sched_getscheduler) | const |  |
| [`SYS_sched_get_priority_max`](#sys_sched_get_priority_max) | const |  |
| [`SYS_sched_get_priority_min`](#sys_sched_get_priority_min) | const |  |
| [`SYS_sched_rr_get_interval`](#sys_sched_rr_get_interval) | const |  |
| [`SYS_mlock`](#sys_mlock) | const |  |
| [`SYS_munlock`](#sys_munlock) | const |  |
| [`SYS_mlockall`](#sys_mlockall) | const |  |
| [`SYS_munlockall`](#sys_munlockall) | const |  |
| [`SYS_vhangup`](#sys_vhangup) | const |  |
| [`SYS_modify_ldt`](#sys_modify_ldt) | const |  |
| [`SYS_pivot_root`](#sys_pivot_root) | const |  |
| [`SYS__sysctl`](#sys__sysctl) | const |  |
| [`SYS_prctl`](#sys_prctl) | const |  |
| [`SYS_arch_prctl`](#sys_arch_prctl) | const |  |
| [`SYS_adjtimex`](#sys_adjtimex) | const |  |
| [`SYS_setrlimit`](#sys_setrlimit) | const |  |
| [`SYS_chroot`](#sys_chroot) | const |  |
| [`SYS_sync`](#sys_sync) | const |  |
| [`SYS_acct`](#sys_acct) | const |  |
| [`SYS_settimeofday`](#sys_settimeofday) | const |  |
| [`SYS_mount`](#sys_mount) | const |  |
| [`SYS_umount2`](#sys_umount2) | const |  |
| [`SYS_swapon`](#sys_swapon) | const |  |
| [`SYS_swapoff`](#sys_swapoff) | const |  |
| [`SYS_reboot`](#sys_reboot) | const |  |
| [`SYS_sethostname`](#sys_sethostname) | const |  |
| [`SYS_setdomainname`](#sys_setdomainname) | const |  |
| [`SYS_iopl`](#sys_iopl) | const |  |
| [`SYS_ioperm`](#sys_ioperm) | const |  |
| [`SYS_create_module`](#sys_create_module) | const |  |
| [`SYS_init_module`](#sys_init_module) | const |  |
| [`SYS_delete_module`](#sys_delete_module) | const |  |
| [`SYS_get_kernel_syms`](#sys_get_kernel_syms) | const |  |
| [`SYS_query_module`](#sys_query_module) | const |  |
| [`SYS_quotactl`](#sys_quotactl) | const |  |
| [`SYS_nfsservctl`](#sys_nfsservctl) | const |  |
| [`SYS_getpmsg`](#sys_getpmsg) | const |  |
| [`SYS_putpmsg`](#sys_putpmsg) | const |  |
| [`SYS_afs_syscall`](#sys_afs_syscall) | const |  |
| [`SYS_tuxcall`](#sys_tuxcall) | const |  |
| [`SYS_security`](#sys_security) | const |  |
| [`SYS_gettid`](#sys_gettid) | const |  |
| [`SYS_readahead`](#sys_readahead) | const |  |
| [`SYS_setxattr`](#sys_setxattr) | const |  |
| [`SYS_lsetxattr`](#sys_lsetxattr) | const |  |
| [`SYS_fsetxattr`](#sys_fsetxattr) | const |  |
| [`SYS_getxattr`](#sys_getxattr) | const |  |
| [`SYS_lgetxattr`](#sys_lgetxattr) | const |  |
| [`SYS_fgetxattr`](#sys_fgetxattr) | const |  |
| [`SYS_listxattr`](#sys_listxattr) | const |  |
| [`SYS_llistxattr`](#sys_llistxattr) | const |  |
| [`SYS_flistxattr`](#sys_flistxattr) | const |  |
| [`SYS_removexattr`](#sys_removexattr) | const |  |
| [`SYS_lremovexattr`](#sys_lremovexattr) | const |  |
| [`SYS_fremovexattr`](#sys_fremovexattr) | const |  |
| [`SYS_tkill`](#sys_tkill) | const |  |
| [`SYS_time`](#sys_time) | const |  |
| [`SYS_futex`](#sys_futex) | const |  |
| [`SYS_sched_setaffinity`](#sys_sched_setaffinity) | const |  |
| [`SYS_sched_getaffinity`](#sys_sched_getaffinity) | const |  |
| [`SYS_set_thread_area`](#sys_set_thread_area) | const |  |
| [`SYS_io_setup`](#sys_io_setup) | const |  |
| [`SYS_io_destroy`](#sys_io_destroy) | const |  |
| [`SYS_io_getevents`](#sys_io_getevents) | const |  |
| [`SYS_io_submit`](#sys_io_submit) | const |  |
| [`SYS_io_cancel`](#sys_io_cancel) | const |  |
| [`SYS_get_thread_area`](#sys_get_thread_area) | const |  |
| [`SYS_lookup_dcookie`](#sys_lookup_dcookie) | const |  |
| [`SYS_epoll_create`](#sys_epoll_create) | const |  |
| [`SYS_epoll_ctl_old`](#sys_epoll_ctl_old) | const |  |
| [`SYS_epoll_wait_old`](#sys_epoll_wait_old) | const |  |
| [`SYS_remap_file_pages`](#sys_remap_file_pages) | const |  |
| [`SYS_getdents64`](#sys_getdents64) | const |  |
| [`SYS_set_tid_address`](#sys_set_tid_address) | const |  |
| [`SYS_restart_syscall`](#sys_restart_syscall) | const |  |
| [`SYS_semtimedop`](#sys_semtimedop) | const |  |
| [`SYS_fadvise64`](#sys_fadvise64) | const |  |
| [`SYS_timer_create`](#sys_timer_create) | const |  |
| [`SYS_timer_settime`](#sys_timer_settime) | const |  |
| [`SYS_timer_gettime`](#sys_timer_gettime) | const |  |
| [`SYS_timer_getoverrun`](#sys_timer_getoverrun) | const |  |
| [`SYS_timer_delete`](#sys_timer_delete) | const |  |
| [`SYS_clock_settime`](#sys_clock_settime) | const |  |
| [`SYS_clock_gettime`](#sys_clock_gettime) | const |  |
| [`SYS_clock_getres`](#sys_clock_getres) | const |  |
| [`SYS_clock_nanosleep`](#sys_clock_nanosleep) | const |  |
| [`SYS_exit_group`](#sys_exit_group) | const |  |
| [`SYS_epoll_wait`](#sys_epoll_wait) | const |  |
| [`SYS_epoll_ctl`](#sys_epoll_ctl) | const |  |
| [`SYS_tgkill`](#sys_tgkill) | const |  |
| [`SYS_utimes`](#sys_utimes) | const |  |
| [`SYS_vserver`](#sys_vserver) | const |  |
| [`SYS_mbind`](#sys_mbind) | const |  |
| [`SYS_set_mempolicy`](#sys_set_mempolicy) | const |  |
| [`SYS_get_mempolicy`](#sys_get_mempolicy) | const |  |
| [`SYS_mq_open`](#sys_mq_open) | const |  |
| [`SYS_mq_unlink`](#sys_mq_unlink) | const |  |
| [`SYS_mq_timedsend`](#sys_mq_timedsend) | const |  |
| [`SYS_mq_timedreceive`](#sys_mq_timedreceive) | const |  |
| [`SYS_mq_notify`](#sys_mq_notify) | const |  |
| [`SYS_mq_getsetattr`](#sys_mq_getsetattr) | const |  |
| [`SYS_kexec_load`](#sys_kexec_load) | const |  |
| [`SYS_waitid`](#sys_waitid) | const |  |
| [`SYS_add_key`](#sys_add_key) | const |  |
| [`SYS_request_key`](#sys_request_key) | const |  |
| [`SYS_keyctl`](#sys_keyctl) | const |  |
| [`SYS_ioprio_set`](#sys_ioprio_set) | const |  |
| [`SYS_ioprio_get`](#sys_ioprio_get) | const |  |
| [`SYS_inotify_init`](#sys_inotify_init) | const |  |
| [`SYS_inotify_add_watch`](#sys_inotify_add_watch) | const |  |
| [`SYS_inotify_rm_watch`](#sys_inotify_rm_watch) | const |  |
| [`SYS_migrate_pages`](#sys_migrate_pages) | const |  |
| [`SYS_openat`](#sys_openat) | const |  |
| [`SYS_mkdirat`](#sys_mkdirat) | const |  |
| [`SYS_mknodat`](#sys_mknodat) | const |  |
| [`SYS_fchownat`](#sys_fchownat) | const |  |
| [`SYS_futimesat`](#sys_futimesat) | const |  |
| [`SYS_newfstatat`](#sys_newfstatat) | const |  |
| [`SYS_unlinkat`](#sys_unlinkat) | const |  |
| [`SYS_renameat`](#sys_renameat) | const |  |
| [`SYS_linkat`](#sys_linkat) | const |  |
| [`SYS_symlinkat`](#sys_symlinkat) | const |  |
| [`SYS_readlinkat`](#sys_readlinkat) | const |  |
| [`SYS_fchmodat`](#sys_fchmodat) | const |  |
| [`SYS_faccessat`](#sys_faccessat) | const |  |
| [`SYS_pselect6`](#sys_pselect6) | const |  |
| [`SYS_ppoll`](#sys_ppoll) | const |  |
| [`SYS_unshare`](#sys_unshare) | const |  |
| [`SYS_set_robust_list`](#sys_set_robust_list) | const |  |
| [`SYS_get_robust_list`](#sys_get_robust_list) | const |  |
| [`SYS_splice`](#sys_splice) | const |  |
| [`SYS_tee`](#sys_tee) | const |  |
| [`SYS_sync_file_range`](#sys_sync_file_range) | const |  |
| [`SYS_vmsplice`](#sys_vmsplice) | const |  |
| [`SYS_move_pages`](#sys_move_pages) | const |  |
| [`SYS_utimensat`](#sys_utimensat) | const |  |
| [`SYS_epoll_pwait`](#sys_epoll_pwait) | const |  |
| [`SYS_signalfd`](#sys_signalfd) | const |  |
| [`SYS_timerfd_create`](#sys_timerfd_create) | const |  |
| [`SYS_eventfd`](#sys_eventfd) | const |  |
| [`SYS_fallocate`](#sys_fallocate) | const |  |
| [`SYS_timerfd_settime`](#sys_timerfd_settime) | const |  |
| [`SYS_timerfd_gettime`](#sys_timerfd_gettime) | const |  |
| [`SYS_accept4`](#sys_accept4) | const |  |
| [`SYS_signalfd4`](#sys_signalfd4) | const |  |
| [`SYS_eventfd2`](#sys_eventfd2) | const |  |
| [`SYS_epoll_create1`](#sys_epoll_create1) | const |  |
| [`SYS_dup3`](#sys_dup3) | const |  |
| [`SYS_pipe2`](#sys_pipe2) | const |  |
| [`SYS_inotify_init1`](#sys_inotify_init1) | const |  |
| [`SYS_preadv`](#sys_preadv) | const |  |
| [`SYS_pwritev`](#sys_pwritev) | const |  |
| [`SYS_rt_tgsigqueueinfo`](#sys_rt_tgsigqueueinfo) | const |  |
| [`SYS_perf_event_open`](#sys_perf_event_open) | const |  |
| [`SYS_recvmmsg`](#sys_recvmmsg) | const |  |
| [`SYS_fanotify_init`](#sys_fanotify_init) | const |  |
| [`SYS_fanotify_mark`](#sys_fanotify_mark) | const |  |
| [`SYS_prlimit64`](#sys_prlimit64) | const |  |
| [`SYS_name_to_handle_at`](#sys_name_to_handle_at) | const |  |
| [`SYS_open_by_handle_at`](#sys_open_by_handle_at) | const |  |
| [`SYS_clock_adjtime`](#sys_clock_adjtime) | const |  |
| [`SYS_syncfs`](#sys_syncfs) | const |  |
| [`SYS_sendmmsg`](#sys_sendmmsg) | const |  |
| [`SYS_setns`](#sys_setns) | const |  |
| [`SYS_getcpu`](#sys_getcpu) | const |  |
| [`SYS_process_vm_readv`](#sys_process_vm_readv) | const |  |
| [`SYS_process_vm_writev`](#sys_process_vm_writev) | const |  |
| [`SYS_kcmp`](#sys_kcmp) | const |  |
| [`SYS_finit_module`](#sys_finit_module) | const |  |
| [`SYS_sched_setattr`](#sys_sched_setattr) | const |  |
| [`SYS_sched_getattr`](#sys_sched_getattr) | const |  |
| [`SYS_renameat2`](#sys_renameat2) | const |  |
| [`SYS_seccomp`](#sys_seccomp) | const |  |
| [`SYS_getrandom`](#sys_getrandom) | const |  |
| [`SYS_memfd_create`](#sys_memfd_create) | const |  |
| [`SYS_kexec_file_load`](#sys_kexec_file_load) | const |  |
| [`SYS_bpf`](#sys_bpf) | const |  |
| [`SYS_execveat`](#sys_execveat) | const |  |
| [`SYS_userfaultfd`](#sys_userfaultfd) | const |  |
| [`SYS_membarrier`](#sys_membarrier) | const |  |
| [`SYS_mlock2`](#sys_mlock2) | const |  |
| [`SYS_copy_file_range`](#sys_copy_file_range) | const |  |
| [`SYS_preadv2`](#sys_preadv2) | const |  |
| [`SYS_pwritev2`](#sys_pwritev2) | const |  |
| [`SYS_pkey_mprotect`](#sys_pkey_mprotect) | const |  |
| [`SYS_pkey_alloc`](#sys_pkey_alloc) | const |  |
| [`SYS_pkey_free`](#sys_pkey_free) | const |  |
| [`SYS_statx`](#sys_statx) | const |  |
| [`SYS_rseq`](#sys_rseq) | const |  |
| [`SYS_pidfd_send_signal`](#sys_pidfd_send_signal) | const |  |
| [`SYS_io_uring_setup`](#sys_io_uring_setup) | const |  |
| [`SYS_io_uring_enter`](#sys_io_uring_enter) | const |  |
| [`SYS_io_uring_register`](#sys_io_uring_register) | const |  |
| [`SYS_open_tree`](#sys_open_tree) | const |  |
| [`SYS_move_mount`](#sys_move_mount) | const |  |
| [`SYS_fsopen`](#sys_fsopen) | const |  |
| [`SYS_fsconfig`](#sys_fsconfig) | const |  |
| [`SYS_fsmount`](#sys_fsmount) | const |  |
| [`SYS_fspick`](#sys_fspick) | const |  |
| [`SYS_pidfd_open`](#sys_pidfd_open) | const |  |
| [`SYS_clone3`](#sys_clone3) | const |  |
| [`SYS_close_range`](#sys_close_range) | const |  |
| [`SYS_openat2`](#sys_openat2) | const |  |
| [`SYS_pidfd_getfd`](#sys_pidfd_getfd) | const |  |
| [`SYS_faccessat2`](#sys_faccessat2) | const |  |
| [`SYS_process_madvise`](#sys_process_madvise) | const |  |
| [`SYS_epoll_pwait2`](#sys_epoll_pwait2) | const |  |
| [`SYS_mount_setattr`](#sys_mount_setattr) | const |  |
| [`SYS_quotactl_fd`](#sys_quotactl_fd) | const |  |
| [`SYS_landlock_create_ruleset`](#sys_landlock_create_ruleset) | const |  |
| [`SYS_landlock_add_rule`](#sys_landlock_add_rule) | const |  |
| [`SYS_landlock_restrict_self`](#sys_landlock_restrict_self) | const |  |
| [`SYS_memfd_secret`](#sys_memfd_secret) | const |  |
| [`SYS_process_mrelease`](#sys_process_mrelease) | const |  |
| [`SYS_futex_waitv`](#sys_futex_waitv) | const |  |
| [`SYS_set_mempolicy_home_node`](#sys_set_mempolicy_home_node) | const |  |
| [`SYS_fchmodat2`](#sys_fchmodat2) | const |  |
| [`SYS_mseal`](#sys_mseal) | const |  |

## Modules

- [`not_x32`](not_x32/index.md)

## Structs

### `sigaction`

```rust
struct sigaction {
    pub sa_sigaction: crate::sighandler_t,
    pub sa_mask: crate::sigset_t,
    pub sa_flags: crate::c_int,
    pub sa_restorer: core::option::Option<fn()>,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for sigaction`

- <span id="sigaction-clone"></span>`fn clone(&self) -> sigaction` — [`sigaction`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for statfs`

- <span id="statfs-clone"></span>`fn clone(&self) -> statfs` — [`statfs`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for flock`

- <span id="flock-clone"></span>`fn clone(&self) -> flock` — [`flock`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for flock64`

- <span id="flock64-clone"></span>`fn clone(&self) -> flock64` — [`flock64`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Implementations

- <span id="siginfo-t-si-addr"></span>`unsafe fn si_addr(&self) -> *mut c_void` — [`c_void`](../../../../../../index.md)

- <span id="siginfo-t-si-value"></span>`unsafe fn si_value(&self) -> crate::sigval` — [`sigval`](../../../../../../index.md)

#### Trait Implementations

##### `impl Clone for siginfo_t`

- <span id="siginfo-t-clone"></span>`fn clone(&self) -> siginfo_t` — [`siginfo_t`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for stack_t`

- <span id="stack-t-clone"></span>`fn clone(&self) -> stack_t` — [`stack_t`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for stat`

- <span id="stat-clone"></span>`fn clone(&self) -> stat` — [`stat`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for stat64`

- <span id="stat64-clone"></span>`fn clone(&self) -> stat64` — [`stat64`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for statfs64`

- <span id="statfs64-clone"></span>`fn clone(&self) -> statfs64` — [`statfs64`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for statvfs64`

- <span id="statvfs64-clone"></span>`fn clone(&self) -> statvfs64` — [`statvfs64`](../index.md)

##### `impl Copy for statvfs64`

##### `impl Debug for statvfs64`

- <span id="statvfs64-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `pthread_attr_t`

```rust
struct pthread_attr_t {
    __size: [u64; 7],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for pthread_attr_t`

- <span id="pthread-attr-t-clone"></span>`fn clone(&self) -> pthread_attr_t` — [`pthread_attr_t`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for _libc_fpxreg`

- <span id="libc-fpxreg-clone"></span>`fn clone(&self) -> _libc_fpxreg` — [`_libc_fpxreg`](../index.md)

##### `impl Copy for _libc_fpxreg`

##### `impl Debug for _libc_fpxreg`

- <span id="libc-fpxreg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `_libc_xmmreg`

```rust
struct _libc_xmmreg {
    pub element: [u32; 4],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for _libc_xmmreg`

- <span id="libc-xmmreg-clone"></span>`fn clone(&self) -> _libc_xmmreg` — [`_libc_xmmreg`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for _libc_fpstate`

- <span id="libc-fpstate-clone"></span>`fn clone(&self) -> _libc_fpstate` — [`_libc_fpstate`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for user_regs_struct`

- <span id="user-regs-struct-clone"></span>`fn clone(&self) -> user_regs_struct` — [`user_regs_struct`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for user`

- <span id="user-clone"></span>`fn clone(&self) -> user` — [`user`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for mcontext_t`

- <span id="mcontext-t-clone"></span>`fn clone(&self) -> mcontext_t` — [`mcontext_t`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for ipc_perm`

- <span id="ipc-perm-clone"></span>`fn clone(&self) -> ipc_perm` — [`ipc_perm`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for shmid_ds`

- <span id="shmid-ds-clone"></span>`fn clone(&self) -> shmid_ds` — [`shmid_ds`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for ptrace_rseq_configuration`

- <span id="ptrace-rseq-configuration-clone"></span>`fn clone(&self) -> ptrace_rseq_configuration` — [`ptrace_rseq_configuration`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for clone_args`

- <span id="clone-args-clone"></span>`fn clone(&self) -> clone_args` — [`clone_args`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for user_fpregs_struct`

- <span id="user-fpregs-struct-clone"></span>`fn clone(&self) -> user_fpregs_struct` — [`user_fpregs_struct`](../index.md)

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:17-316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L17-L316)*

#### Trait Implementations

##### `impl Clone for ucontext_t`

- <span id="ucontext-t-clone"></span>`fn clone(&self) -> ucontext_t` — [`ucontext_t`](../index.md)

##### `impl Copy for ucontext_t`

##### `impl Debug for ucontext_t`

- <span id="ucontext-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `max_align_t`

```rust
struct max_align_t {
    priv_: [f64; 4],
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:318-323`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L318-L323)*

#### Trait Implementations

##### `impl Clone for max_align_t`

- <span id="max-align-t-clone"></span>`fn clone(&self) -> max_align_t` — [`max_align_t`](../index.md)

##### `impl Copy for max_align_t`

##### `impl Debug for max_align_t`

- <span id="max-align-t-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:4-19`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L4-L19)*

#### Trait Implementations

##### `impl Clone for statvfs`

- <span id="statvfs-clone"></span>`fn clone(&self) -> statvfs` — [`statvfs`](#statvfs)

##### `impl Copy for statvfs`

##### `impl Debug for statvfs`

- <span id="statvfs-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `getcontext`

```rust
unsafe fn getcontext(ucp: *mut ucontext_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:729`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L729)*

### `setcontext`

```rust
unsafe fn setcontext(ucp: *const ucontext_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:730`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L730)*

### `makecontext`

```rust
unsafe fn makecontext(ucp: *mut ucontext_t, func: fn(), argc: c_int)
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:731`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L731)*

### `swapcontext`

```rust
unsafe fn swapcontext(uocp: *mut ucontext_t, ucp: *const ucontext_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:732`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L732)*

### `sysctl`

```rust
unsafe fn sysctl(name: *mut c_int, namelen: c_int, oldp: *mut c_void, oldlenp: *mut size_t, newp: *mut c_void, newlen: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:438-445`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L438-L445)*

## Type Aliases

### `wchar_t`

```rust
type wchar_t = i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:9`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L9)*

### `nlink_t`

```rust
type nlink_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:10`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L10)*

### `blksize_t`

```rust
type blksize_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:11`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L11)*

### `greg_t`

```rust
type greg_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:12`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L12)*

### `suseconds_t`

```rust
type suseconds_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:13`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L13)*

### `__u64`

```rust
type __u64 = crate::c_ulonglong;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:14`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L14)*

### `__s64`

```rust
type __s64 = crate::c_longlong;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:15`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L15)*

## Constants

### `POSIX_FADV_DONTNEED`
```rust
const POSIX_FADV_DONTNEED: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:325`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L325)*

### `POSIX_FADV_NOREUSE`
```rust
const POSIX_FADV_NOREUSE: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:326`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L326)*

### `VEOF`
```rust
const VEOF: usize = 4usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:328`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L328)*

### `RTLD_DEEPBIND`
```rust
const RTLD_DEEPBIND: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:329`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L329)*

### `RTLD_GLOBAL`
```rust
const RTLD_GLOBAL: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:330`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L330)*

### `RTLD_NOLOAD`
```rust
const RTLD_NOLOAD: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:331`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L331)*

### `O_APPEND`
```rust
const O_APPEND: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:333`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L333)*

### `O_CREAT`
```rust
const O_CREAT: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:334`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L334)*

### `O_EXCL`
```rust
const O_EXCL: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:335`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L335)*

### `O_NOCTTY`
```rust
const O_NOCTTY: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:336`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L336)*

### `O_NONBLOCK`
```rust
const O_NONBLOCK: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:337`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L337)*

### `O_SYNC`
```rust
const O_SYNC: crate::c_int = 1_052_672i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:338`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L338)*

### `O_RSYNC`
```rust
const O_RSYNC: crate::c_int = 1_052_672i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:339`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L339)*

### `O_DSYNC`
```rust
const O_DSYNC: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:340`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L340)*

### `O_FSYNC`
```rust
const O_FSYNC: crate::c_int = 1_052_672i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:341`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L341)*

### `O_NOATIME`
```rust
const O_NOATIME: crate::c_int = 262_144i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:342`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L342)*

### `O_PATH`
```rust
const O_PATH: crate::c_int = 2_097_152i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:343`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L343)*

### `O_TMPFILE`
```rust
const O_TMPFILE: crate::c_int = 4_259_840i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:344`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L344)*

### `MADV_SOFT_OFFLINE`
```rust
const MADV_SOFT_OFFLINE: crate::c_int = 101i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:346`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L346)*

### `MAP_GROWSDOWN`
```rust
const MAP_GROWSDOWN: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:347`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L347)*

### `EDEADLK`
```rust
const EDEADLK: crate::c_int = 35i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:349`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L349)*

### `ENAMETOOLONG`
```rust
const ENAMETOOLONG: crate::c_int = 36i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:350`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L350)*

### `ENOLCK`
```rust
const ENOLCK: crate::c_int = 37i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:351`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L351)*

### `ENOSYS`
```rust
const ENOSYS: crate::c_int = 38i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:352`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L352)*

### `ENOTEMPTY`
```rust
const ENOTEMPTY: crate::c_int = 39i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:353`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L353)*

### `ELOOP`
```rust
const ELOOP: crate::c_int = 40i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:354`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L354)*

### `ENOMSG`
```rust
const ENOMSG: crate::c_int = 42i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:355`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L355)*

### `EIDRM`
```rust
const EIDRM: crate::c_int = 43i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:356`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L356)*

### `ECHRNG`
```rust
const ECHRNG: crate::c_int = 44i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:357`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L357)*

### `EL2NSYNC`
```rust
const EL2NSYNC: crate::c_int = 45i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:358`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L358)*

### `EL3HLT`
```rust
const EL3HLT: crate::c_int = 46i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:359`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L359)*

### `EL3RST`
```rust
const EL3RST: crate::c_int = 47i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:360`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L360)*

### `ELNRNG`
```rust
const ELNRNG: crate::c_int = 48i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:361`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L361)*

### `EUNATCH`
```rust
const EUNATCH: crate::c_int = 49i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:362`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L362)*

### `ENOCSI`
```rust
const ENOCSI: crate::c_int = 50i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:363`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L363)*

### `EL2HLT`
```rust
const EL2HLT: crate::c_int = 51i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:364`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L364)*

### `EBADE`
```rust
const EBADE: crate::c_int = 52i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:365`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L365)*

### `EBADR`
```rust
const EBADR: crate::c_int = 53i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:366`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L366)*

### `EXFULL`
```rust
const EXFULL: crate::c_int = 54i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:367`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L367)*

### `ENOANO`
```rust
const ENOANO: crate::c_int = 55i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:368`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L368)*

### `EBADRQC`
```rust
const EBADRQC: crate::c_int = 56i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:369`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L369)*

### `EBADSLT`
```rust
const EBADSLT: crate::c_int = 57i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:370`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L370)*

### `EMULTIHOP`
```rust
const EMULTIHOP: crate::c_int = 72i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:371`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L371)*

### `EOVERFLOW`
```rust
const EOVERFLOW: crate::c_int = 75i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:372`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L372)*

### `ENOTUNIQ`
```rust
const ENOTUNIQ: crate::c_int = 76i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:373`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L373)*

### `EBADFD`
```rust
const EBADFD: crate::c_int = 77i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:374`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L374)*

### `EBADMSG`
```rust
const EBADMSG: crate::c_int = 74i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:375`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L375)*

### `EREMCHG`
```rust
const EREMCHG: crate::c_int = 78i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:376`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L376)*

### `ELIBACC`
```rust
const ELIBACC: crate::c_int = 79i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:377`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L377)*

### `ELIBBAD`
```rust
const ELIBBAD: crate::c_int = 80i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:378`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L378)*

### `ELIBSCN`
```rust
const ELIBSCN: crate::c_int = 81i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:379`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L379)*

### `ELIBMAX`
```rust
const ELIBMAX: crate::c_int = 82i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:380`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L380)*

### `ELIBEXEC`
```rust
const ELIBEXEC: crate::c_int = 83i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:381`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L381)*

### `EILSEQ`
```rust
const EILSEQ: crate::c_int = 84i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:382`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L382)*

### `ERESTART`
```rust
const ERESTART: crate::c_int = 85i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:383`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L383)*

### `ESTRPIPE`
```rust
const ESTRPIPE: crate::c_int = 86i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:384`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L384)*

### `EUSERS`
```rust
const EUSERS: crate::c_int = 87i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:385`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L385)*

### `ENOTSOCK`
```rust
const ENOTSOCK: crate::c_int = 88i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:386`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L386)*

### `EDESTADDRREQ`
```rust
const EDESTADDRREQ: crate::c_int = 89i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:387`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L387)*

### `EMSGSIZE`
```rust
const EMSGSIZE: crate::c_int = 90i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:388`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L388)*

### `EPROTOTYPE`
```rust
const EPROTOTYPE: crate::c_int = 91i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:389`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L389)*

### `ENOPROTOOPT`
```rust
const ENOPROTOOPT: crate::c_int = 92i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:390`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L390)*

### `EPROTONOSUPPORT`
```rust
const EPROTONOSUPPORT: crate::c_int = 93i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:391`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L391)*

### `ESOCKTNOSUPPORT`
```rust
const ESOCKTNOSUPPORT: crate::c_int = 94i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:392`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L392)*

### `EOPNOTSUPP`
```rust
const EOPNOTSUPP: crate::c_int = 95i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:393`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L393)*

### `EPFNOSUPPORT`
```rust
const EPFNOSUPPORT: crate::c_int = 96i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:394`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L394)*

### `EAFNOSUPPORT`
```rust
const EAFNOSUPPORT: crate::c_int = 97i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:395`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L395)*

### `EADDRINUSE`
```rust
const EADDRINUSE: crate::c_int = 98i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:396`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L396)*

### `EADDRNOTAVAIL`
```rust
const EADDRNOTAVAIL: crate::c_int = 99i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:397`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L397)*

### `ENETDOWN`
```rust
const ENETDOWN: crate::c_int = 100i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:398`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L398)*

### `ENETUNREACH`
```rust
const ENETUNREACH: crate::c_int = 101i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:399`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L399)*

### `ENETRESET`
```rust
const ENETRESET: crate::c_int = 102i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:400`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L400)*

### `ECONNABORTED`
```rust
const ECONNABORTED: crate::c_int = 103i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:401`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L401)*

### `ECONNRESET`
```rust
const ECONNRESET: crate::c_int = 104i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:402`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L402)*

### `ENOBUFS`
```rust
const ENOBUFS: crate::c_int = 105i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:403`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L403)*

### `EISCONN`
```rust
const EISCONN: crate::c_int = 106i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:404`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L404)*

### `ENOTCONN`
```rust
const ENOTCONN: crate::c_int = 107i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:405`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L405)*

### `ESHUTDOWN`
```rust
const ESHUTDOWN: crate::c_int = 108i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:406`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L406)*

### `ETOOMANYREFS`
```rust
const ETOOMANYREFS: crate::c_int = 109i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:407`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L407)*

### `ETIMEDOUT`
```rust
const ETIMEDOUT: crate::c_int = 110i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:408`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L408)*

### `ECONNREFUSED`
```rust
const ECONNREFUSED: crate::c_int = 111i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:409`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L409)*

### `EHOSTDOWN`
```rust
const EHOSTDOWN: crate::c_int = 112i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:410`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L410)*

### `EHOSTUNREACH`
```rust
const EHOSTUNREACH: crate::c_int = 113i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:411`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L411)*

### `EALREADY`
```rust
const EALREADY: crate::c_int = 114i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:412`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L412)*

### `EINPROGRESS`
```rust
const EINPROGRESS: crate::c_int = 115i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:413`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L413)*

### `ESTALE`
```rust
const ESTALE: crate::c_int = 116i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:414`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L414)*

### `EDQUOT`
```rust
const EDQUOT: crate::c_int = 122i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:415`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L415)*

### `ENOMEDIUM`
```rust
const ENOMEDIUM: crate::c_int = 123i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:416`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L416)*

### `EMEDIUMTYPE`
```rust
const EMEDIUMTYPE: crate::c_int = 124i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:417`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L417)*

### `ECANCELED`
```rust
const ECANCELED: crate::c_int = 125i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:418`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L418)*

### `ENOKEY`
```rust
const ENOKEY: crate::c_int = 126i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:419`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L419)*

### `EKEYEXPIRED`
```rust
const EKEYEXPIRED: crate::c_int = 127i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:420`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L420)*

### `EKEYREVOKED`
```rust
const EKEYREVOKED: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:421`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L421)*

### `EKEYREJECTED`
```rust
const EKEYREJECTED: crate::c_int = 129i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:422`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L422)*

### `EOWNERDEAD`
```rust
const EOWNERDEAD: crate::c_int = 130i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:423`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L423)*

### `ENOTRECOVERABLE`
```rust
const ENOTRECOVERABLE: crate::c_int = 131i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:424`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L424)*

### `EHWPOISON`
```rust
const EHWPOISON: crate::c_int = 133i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:425`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L425)*

### `ERFKILL`
```rust
const ERFKILL: crate::c_int = 132i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:426`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L426)*

### `SOCK_STREAM`
```rust
const SOCK_STREAM: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:428`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L428)*

### `SOCK_DGRAM`
```rust
const SOCK_DGRAM: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:429`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L429)*

### `SA_ONSTACK`
```rust
const SA_ONSTACK: crate::c_int = 134_217_728i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:431`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L431)*

### `SA_SIGINFO`
```rust
const SA_SIGINFO: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:432`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L432)*

### `SA_NOCLDWAIT`
```rust
const SA_NOCLDWAIT: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:433`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L433)*

### `SIGTTIN`
```rust
const SIGTTIN: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:435`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L435)*

### `SIGTTOU`
```rust
const SIGTTOU: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:436`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L436)*

### `SIGXCPU`
```rust
const SIGXCPU: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:437`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L437)*

### `SIGXFSZ`
```rust
const SIGXFSZ: crate::c_int = 25i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:438`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L438)*

### `SIGVTALRM`
```rust
const SIGVTALRM: crate::c_int = 26i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:439`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L439)*

### `SIGPROF`
```rust
const SIGPROF: crate::c_int = 27i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:440`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L440)*

### `SIGWINCH`
```rust
const SIGWINCH: crate::c_int = 28i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:441`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L441)*

### `SIGCHLD`
```rust
const SIGCHLD: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:442`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L442)*

### `SIGBUS`
```rust
const SIGBUS: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:443`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L443)*

### `SIGUSR1`
```rust
const SIGUSR1: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:444`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L444)*

### `SIGUSR2`
```rust
const SIGUSR2: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:445`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L445)*

### `SIGCONT`
```rust
const SIGCONT: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:446`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L446)*

### `SIGSTOP`
```rust
const SIGSTOP: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:447`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L447)*

### `SIGTSTP`
```rust
const SIGTSTP: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:448`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L448)*

### `SIGURG`
```rust
const SIGURG: crate::c_int = 23i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:449`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L449)*

### `SIGIO`
```rust
const SIGIO: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:450`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L450)*

### `SIGSYS`
```rust
const SIGSYS: crate::c_int = 31i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:451`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L451)*

### `SIGSTKFLT`
```rust
const SIGSTKFLT: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:452`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L452)*

### `SIGUNUSED`
```rust
const SIGUNUSED: crate::c_int = 31i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:454`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L454)*

### `SIGPOLL`
```rust
const SIGPOLL: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:455`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L455)*

### `SIGPWR`
```rust
const SIGPWR: crate::c_int = 30i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:456`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L456)*

### `SIG_SETMASK`
```rust
const SIG_SETMASK: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:457`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L457)*

### `SIG_BLOCK`
```rust
const SIG_BLOCK: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:458`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L458)*

### `SIG_UNBLOCK`
```rust
const SIG_UNBLOCK: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:459`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L459)*

### `POLLWRNORM`
```rust
const POLLWRNORM: crate::c_short = 256i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:461`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L461)*

### `POLLWRBAND`
```rust
const POLLWRBAND: crate::c_short = 512i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:462`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L462)*

### `O_ASYNC`
```rust
const O_ASYNC: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:464`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L464)*

### `O_NDELAY`
```rust
const O_NDELAY: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:465`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L465)*

### `PTRACE_DETACH`
```rust
const PTRACE_DETACH: crate::c_uint = 17u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:467`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L467)*

### `PTRACE_GET_RSEQ_CONFIGURATION`
```rust
const PTRACE_GET_RSEQ_CONFIGURATION: crate::c_uint = 16_911u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:468`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L468)*

### `EFD_NONBLOCK`
```rust
const EFD_NONBLOCK: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:470`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L470)*

### `F_GETLK`
```rust
const F_GETLK: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:472`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L472)*

### `F_GETOWN`
```rust
const F_GETOWN: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:473`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L473)*

### `F_SETOWN`
```rust
const F_SETOWN: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:474`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L474)*

### `F_SETLK`
```rust
const F_SETLK: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:475`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L475)*

### `F_SETLKW`
```rust
const F_SETLKW: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:476`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L476)*

### `F_OFD_GETLK`
```rust
const F_OFD_GETLK: crate::c_int = 36i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:477`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L477)*

### `F_OFD_SETLK`
```rust
const F_OFD_SETLK: crate::c_int = 37i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:478`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L478)*

### `F_OFD_SETLKW`
```rust
const F_OFD_SETLKW: crate::c_int = 38i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:479`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L479)*

### `F_RDLCK`
```rust
const F_RDLCK: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:481`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L481)*

### `F_WRLCK`
```rust
const F_WRLCK: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:482`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L482)*

### `F_UNLCK`
```rust
const F_UNLCK: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:483`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L483)*

### `SFD_NONBLOCK`
```rust
const SFD_NONBLOCK: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:485`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L485)*

### `TCSANOW`
```rust
const TCSANOW: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:487`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L487)*

### `TCSADRAIN`
```rust
const TCSADRAIN: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:488`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L488)*

### `TCSAFLUSH`
```rust
const TCSAFLUSH: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:489`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L489)*

### `SFD_CLOEXEC`
```rust
const SFD_CLOEXEC: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:491`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L491)*

### `NCCS`
```rust
const NCCS: usize = 32usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:493`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L493)*

### `O_TRUNC`
```rust
const O_TRUNC: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:495`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L495)*

### `O_CLOEXEC`
```rust
const O_CLOEXEC: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:497`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L497)*

### `EBFONT`
```rust
const EBFONT: crate::c_int = 59i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:499`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L499)*

### `ENOSTR`
```rust
const ENOSTR: crate::c_int = 60i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:500`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L500)*

### `ENODATA`
```rust
const ENODATA: crate::c_int = 61i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:501`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L501)*

### `ETIME`
```rust
const ETIME: crate::c_int = 62i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:502`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L502)*

### `ENOSR`
```rust
const ENOSR: crate::c_int = 63i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:503`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L503)*

### `ENONET`
```rust
const ENONET: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:504`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L504)*

### `ENOPKG`
```rust
const ENOPKG: crate::c_int = 65i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:505`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L505)*

### `EREMOTE`
```rust
const EREMOTE: crate::c_int = 66i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:506`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L506)*

### `ENOLINK`
```rust
const ENOLINK: crate::c_int = 67i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:507`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L507)*

### `EADV`
```rust
const EADV: crate::c_int = 68i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:508`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L508)*

### `ESRMNT`
```rust
const ESRMNT: crate::c_int = 69i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:509`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L509)*

### `ECOMM`
```rust
const ECOMM: crate::c_int = 70i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:510`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L510)*

### `EPROTO`
```rust
const EPROTO: crate::c_int = 71i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:511`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L511)*

### `EDOTDOT`
```rust
const EDOTDOT: crate::c_int = 73i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:512`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L512)*

### `SA_NODEFER`
```rust
const SA_NODEFER: crate::c_int = 1_073_741_824i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:514`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L514)*

### `SA_RESETHAND`
```rust
const SA_RESETHAND: crate::c_int = -2_147_483_648i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:515`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L515)*

### `SA_RESTART`
```rust
const SA_RESTART: crate::c_int = 268_435_456i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:516`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L516)*

### `SA_NOCLDSTOP`
```rust
const SA_NOCLDSTOP: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:517`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L517)*

### `EPOLL_CLOEXEC`
```rust
const EPOLL_CLOEXEC: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:519`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L519)*

### `EFD_CLOEXEC`
```rust
const EFD_CLOEXEC: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:521`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L521)*

### `__SIZEOF_PTHREAD_CONDATTR_T`
```rust
const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:523`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L523)*

### `__SIZEOF_PTHREAD_MUTEXATTR_T`
```rust
const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:524`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L524)*

### `__SIZEOF_PTHREAD_BARRIERATTR_T`
```rust
const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:525`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L525)*

### `O_DIRECT`
```rust
const O_DIRECT: crate::c_int = 16_384i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:527`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L527)*

### `O_DIRECTORY`
```rust
const O_DIRECTORY: crate::c_int = 65_536i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:528`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L528)*

### `O_NOFOLLOW`
```rust
const O_NOFOLLOW: crate::c_int = 131_072i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:529`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L529)*

### `MAP_HUGETLB`
```rust
const MAP_HUGETLB: crate::c_int = 262_144i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:531`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L531)*

### `MAP_LOCKED`
```rust
const MAP_LOCKED: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:532`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L532)*

### `MAP_NORESERVE`
```rust
const MAP_NORESERVE: crate::c_int = 16_384i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:533`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L533)*

### `MAP_32BIT`
```rust
const MAP_32BIT: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:534`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L534)*

### `MAP_ANON`
```rust
const MAP_ANON: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:535`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L535)*

### `MAP_ANONYMOUS`
```rust
const MAP_ANONYMOUS: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:536`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L536)*

### `MAP_DENYWRITE`
```rust
const MAP_DENYWRITE: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:537`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L537)*

### `MAP_EXECUTABLE`
```rust
const MAP_EXECUTABLE: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:538`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L538)*

### `MAP_POPULATE`
```rust
const MAP_POPULATE: crate::c_int = 32_768i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:539`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L539)*

### `MAP_NONBLOCK`
```rust
const MAP_NONBLOCK: crate::c_int = 65_536i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:540`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L540)*

### `MAP_STACK`
```rust
const MAP_STACK: crate::c_int = 131_072i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:541`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L541)*

### `MAP_SYNC`
```rust
const MAP_SYNC: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:542`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L542)*

### `EDEADLOCK`
```rust
const EDEADLOCK: crate::c_int = 35i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:544`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L544)*

### `EUCLEAN`
```rust
const EUCLEAN: crate::c_int = 117i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:545`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L545)*

### `ENOTNAM`
```rust
const ENOTNAM: crate::c_int = 118i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:546`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L546)*

### `ENAVAIL`
```rust
const ENAVAIL: crate::c_int = 119i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:547`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L547)*

### `EISNAM`
```rust
const EISNAM: crate::c_int = 120i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:548`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L548)*

### `EREMOTEIO`
```rust
const EREMOTEIO: crate::c_int = 121i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:549`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L549)*

### `PTRACE_GETFPREGS`
```rust
const PTRACE_GETFPREGS: crate::c_uint = 14u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:551`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L551)*

### `PTRACE_SETFPREGS`
```rust
const PTRACE_SETFPREGS: crate::c_uint = 15u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:552`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L552)*

### `PTRACE_GETFPXREGS`
```rust
const PTRACE_GETFPXREGS: crate::c_uint = 18u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:553`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L553)*

### `PTRACE_SETFPXREGS`
```rust
const PTRACE_SETFPXREGS: crate::c_uint = 19u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:554`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L554)*

### `PTRACE_GETREGS`
```rust
const PTRACE_GETREGS: crate::c_uint = 12u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:555`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L555)*

### `PTRACE_SETREGS`
```rust
const PTRACE_SETREGS: crate::c_uint = 13u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:556`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L556)*

### `PTRACE_PEEKSIGINFO_SHARED`
```rust
const PTRACE_PEEKSIGINFO_SHARED: crate::c_uint = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:557`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L557)*

### `PTRACE_SYSEMU`
```rust
const PTRACE_SYSEMU: crate::c_uint = 31u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:558`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L558)*

### `PTRACE_SYSEMU_SINGLESTEP`
```rust
const PTRACE_SYSEMU_SINGLESTEP: crate::c_uint = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:559`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L559)*

### `PR_GET_SPECULATION_CTRL`
```rust
const PR_GET_SPECULATION_CTRL: crate::c_int = 52i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:561`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L561)*

### `PR_SET_SPECULATION_CTRL`
```rust
const PR_SET_SPECULATION_CTRL: crate::c_int = 53i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:562`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L562)*

### `PR_SPEC_NOT_AFFECTED`
```rust
const PR_SPEC_NOT_AFFECTED: crate::c_uint = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:563`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L563)*

### `PR_SPEC_PRCTL`
```rust
const PR_SPEC_PRCTL: crate::c_uint = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:564`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L564)*

### `PR_SPEC_ENABLE`
```rust
const PR_SPEC_ENABLE: crate::c_uint = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:565`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L565)*

### `PR_SPEC_DISABLE`
```rust
const PR_SPEC_DISABLE: crate::c_uint = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:566`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L566)*

### `PR_SPEC_FORCE_DISABLE`
```rust
const PR_SPEC_FORCE_DISABLE: crate::c_uint = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:567`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L567)*

### `PR_SPEC_DISABLE_NOEXEC`
```rust
const PR_SPEC_DISABLE_NOEXEC: crate::c_uint = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:568`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L568)*

### `PR_SPEC_STORE_BYPASS`
```rust
const PR_SPEC_STORE_BYPASS: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:569`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L569)*

### `PR_SPEC_INDIRECT_BRANCH`
```rust
const PR_SPEC_INDIRECT_BRANCH: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:570`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L570)*

### `MCL_CURRENT`
```rust
const MCL_CURRENT: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:574`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L574)*

### `MCL_FUTURE`
```rust
const MCL_FUTURE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:575`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L575)*

### `MCL_ONFAULT`
```rust
const MCL_ONFAULT: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:576`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L576)*

### `SIGSTKSZ`
```rust
const SIGSTKSZ: crate::size_t = 8_192usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:578`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L578)*

### `MINSIGSTKSZ`
```rust
const MINSIGSTKSZ: crate::size_t = 2_048usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:579`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L579)*

### `CBAUD`
```rust
const CBAUD: crate::tcflag_t = 4_111u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:580`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L580)*

### `TAB1`
```rust
const TAB1: crate::tcflag_t = 2_048u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:581`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L581)*

### `TAB2`
```rust
const TAB2: crate::tcflag_t = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:582`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L582)*

### `TAB3`
```rust
const TAB3: crate::tcflag_t = 6_144u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:583`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L583)*

### `CR1`
```rust
const CR1: crate::tcflag_t = 512u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:584`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L584)*

### `CR2`
```rust
const CR2: crate::tcflag_t = 1_024u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:585`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L585)*

### `CR3`
```rust
const CR3: crate::tcflag_t = 1_536u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:586`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L586)*

### `FF1`
```rust
const FF1: crate::tcflag_t = 32_768u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:587`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L587)*

### `BS1`
```rust
const BS1: crate::tcflag_t = 8_192u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:588`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L588)*

### `VT1`
```rust
const VT1: crate::tcflag_t = 16_384u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:589`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L589)*

### `VWERASE`
```rust
const VWERASE: usize = 14usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:590`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L590)*

### `VREPRINT`
```rust
const VREPRINT: usize = 12usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:591`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L591)*

### `VSUSP`
```rust
const VSUSP: usize = 10usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:592`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L592)*

### `VSTART`
```rust
const VSTART: usize = 8usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:593`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L593)*

### `VSTOP`
```rust
const VSTOP: usize = 9usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:594`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L594)*

### `VDISCARD`
```rust
const VDISCARD: usize = 13usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:595`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L595)*

### `VTIME`
```rust
const VTIME: usize = 5usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:596`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L596)*

### `IXON`
```rust
const IXON: crate::tcflag_t = 1_024u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:597`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L597)*

### `IXOFF`
```rust
const IXOFF: crate::tcflag_t = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:598`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L598)*

### `ONLCR`
```rust
const ONLCR: crate::tcflag_t = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:599`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L599)*

### `CSIZE`
```rust
const CSIZE: crate::tcflag_t = 48u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:600`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L600)*

### `CS6`
```rust
const CS6: crate::tcflag_t = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:601`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L601)*

### `CS7`
```rust
const CS7: crate::tcflag_t = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:602`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L602)*

### `CS8`
```rust
const CS8: crate::tcflag_t = 48u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:603`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L603)*

### `CSTOPB`
```rust
const CSTOPB: crate::tcflag_t = 64u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:604`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L604)*

### `CREAD`
```rust
const CREAD: crate::tcflag_t = 128u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:605`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L605)*

### `PARENB`
```rust
const PARENB: crate::tcflag_t = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:606`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L606)*

### `PARODD`
```rust
const PARODD: crate::tcflag_t = 512u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:607`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L607)*

### `HUPCL`
```rust
const HUPCL: crate::tcflag_t = 1_024u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:608`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L608)*

### `CLOCAL`
```rust
const CLOCAL: crate::tcflag_t = 2_048u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:609`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L609)*

### `ECHOKE`
```rust
const ECHOKE: crate::tcflag_t = 2_048u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:610`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L610)*

### `ECHOE`
```rust
const ECHOE: crate::tcflag_t = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:611`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L611)*

### `ECHOK`
```rust
const ECHOK: crate::tcflag_t = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:612`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L612)*

### `ECHONL`
```rust
const ECHONL: crate::tcflag_t = 64u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:613`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L613)*

### `ECHOPRT`
```rust
const ECHOPRT: crate::tcflag_t = 1_024u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:614`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L614)*

### `ECHOCTL`
```rust
const ECHOCTL: crate::tcflag_t = 512u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:615`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L615)*

### `ISIG`
```rust
const ISIG: crate::tcflag_t = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:616`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L616)*

### `ICANON`
```rust
const ICANON: crate::tcflag_t = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:617`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L617)*

### `PENDIN`
```rust
const PENDIN: crate::tcflag_t = 16_384u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:618`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L618)*

### `NOFLSH`
```rust
const NOFLSH: crate::tcflag_t = 128u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:619`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L619)*

### `CIBAUD`
```rust
const CIBAUD: crate::tcflag_t = 269_418_496u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:620`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L620)*

### `CBAUDEX`
```rust
const CBAUDEX: crate::tcflag_t = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:621`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L621)*

### `VSWTC`
```rust
const VSWTC: usize = 7usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:622`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L622)*

### `OLCUC`
```rust
const OLCUC: crate::tcflag_t = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:623`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L623)*

### `NLDLY`
```rust
const NLDLY: crate::tcflag_t = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:624`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L624)*

### `CRDLY`
```rust
const CRDLY: crate::tcflag_t = 1_536u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:625`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L625)*

### `TABDLY`
```rust
const TABDLY: crate::tcflag_t = 6_144u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:626`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L626)*

### `BSDLY`
```rust
const BSDLY: crate::tcflag_t = 8_192u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:627`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L627)*

### `FFDLY`
```rust
const FFDLY: crate::tcflag_t = 32_768u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:628`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L628)*

### `VTDLY`
```rust
const VTDLY: crate::tcflag_t = 16_384u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:629`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L629)*

### `XTABS`
```rust
const XTABS: crate::tcflag_t = 6_144u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:630`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L630)*

### `B0`
```rust
const B0: crate::speed_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:632`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L632)*

### `B50`
```rust
const B50: crate::speed_t = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:633`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L633)*

### `B75`
```rust
const B75: crate::speed_t = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:634`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L634)*

### `B110`
```rust
const B110: crate::speed_t = 3u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:635`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L635)*

### `B134`
```rust
const B134: crate::speed_t = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:636`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L636)*

### `B150`
```rust
const B150: crate::speed_t = 5u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:637`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L637)*

### `B200`
```rust
const B200: crate::speed_t = 6u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:638`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L638)*

### `B300`
```rust
const B300: crate::speed_t = 7u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:639`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L639)*

### `B600`
```rust
const B600: crate::speed_t = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:640`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L640)*

### `B1200`
```rust
const B1200: crate::speed_t = 9u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:641`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L641)*

### `B1800`
```rust
const B1800: crate::speed_t = 10u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:642`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L642)*

### `B2400`
```rust
const B2400: crate::speed_t = 11u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:643`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L643)*

### `B4800`
```rust
const B4800: crate::speed_t = 12u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:644`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L644)*

### `B9600`
```rust
const B9600: crate::speed_t = 13u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:645`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L645)*

### `B19200`
```rust
const B19200: crate::speed_t = 14u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:646`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L646)*

### `B38400`
```rust
const B38400: crate::speed_t = 15u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:647`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L647)*

### `EXTA`
```rust
const EXTA: crate::speed_t = 14u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:648`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L648)*

### `EXTB`
```rust
const EXTB: crate::speed_t = 15u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:649`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L649)*

### `B57600`
```rust
const B57600: crate::speed_t = 4_097u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:650`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L650)*

### `B115200`
```rust
const B115200: crate::speed_t = 4_098u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:651`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L651)*

### `B230400`
```rust
const B230400: crate::speed_t = 4_099u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:652`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L652)*

### `B460800`
```rust
const B460800: crate::speed_t = 4_100u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:653`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L653)*

### `B500000`
```rust
const B500000: crate::speed_t = 4_101u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:654`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L654)*

### `B576000`
```rust
const B576000: crate::speed_t = 4_102u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:655`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L655)*

### `B921600`
```rust
const B921600: crate::speed_t = 4_103u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:656`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L656)*

### `B1000000`
```rust
const B1000000: crate::speed_t = 4_104u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:657`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L657)*

### `B1152000`
```rust
const B1152000: crate::speed_t = 4_105u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:658`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L658)*

### `B1500000`
```rust
const B1500000: crate::speed_t = 4_106u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:659`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L659)*

### `B2000000`
```rust
const B2000000: crate::speed_t = 4_107u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:660`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L660)*

### `B2500000`
```rust
const B2500000: crate::speed_t = 4_108u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:661`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L661)*

### `B3000000`
```rust
const B3000000: crate::speed_t = 4_109u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:662`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L662)*

### `B3500000`
```rust
const B3500000: crate::speed_t = 4_110u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:663`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L663)*

### `B4000000`
```rust
const B4000000: crate::speed_t = 4_111u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:664`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L664)*

### `VEOL`
```rust
const VEOL: usize = 11usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:666`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L666)*

### `VEOL2`
```rust
const VEOL2: usize = 16usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:667`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L667)*

### `VMIN`
```rust
const VMIN: usize = 6usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:668`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L668)*

### `IEXTEN`
```rust
const IEXTEN: crate::tcflag_t = 32_768u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:669`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L669)*

### `TOSTOP`
```rust
const TOSTOP: crate::tcflag_t = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:670`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L670)*

### `FLUSHO`
```rust
const FLUSHO: crate::tcflag_t = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:671`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L671)*

### `EXTPROC`
```rust
const EXTPROC: crate::tcflag_t = 65_536u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:672`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L672)*

### `R15`
```rust
const R15: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:675`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L675)*

### `R14`
```rust
const R14: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:676`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L676)*

### `R13`
```rust
const R13: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:677`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L677)*

### `R12`
```rust
const R12: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:678`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L678)*

### `RBP`
```rust
const RBP: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:679`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L679)*

### `RBX`
```rust
const RBX: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:680`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L680)*

### `R11`
```rust
const R11: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:681`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L681)*

### `R10`
```rust
const R10: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:682`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L682)*

### `R9`
```rust
const R9: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:683`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L683)*

### `R8`
```rust
const R8: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:684`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L684)*

### `RAX`
```rust
const RAX: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:685`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L685)*

### `RCX`
```rust
const RCX: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:686`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L686)*

### `RDX`
```rust
const RDX: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:687`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L687)*

### `RSI`
```rust
const RSI: crate::c_int = 13i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:688`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L688)*

### `RDI`
```rust
const RDI: crate::c_int = 14i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:689`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L689)*

### `ORIG_RAX`
```rust
const ORIG_RAX: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:690`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L690)*

### `RIP`
```rust
const RIP: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:691`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L691)*

### `CS`
```rust
const CS: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:692`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L692)*

### `EFLAGS`
```rust
const EFLAGS: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:693`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L693)*

### `RSP`
```rust
const RSP: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:694`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L694)*

### `SS`
```rust
const SS: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:695`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L695)*

### `FS_BASE`
```rust
const FS_BASE: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:696`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L696)*

### `GS_BASE`
```rust
const GS_BASE: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:697`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L697)*

### `DS`
```rust
const DS: crate::c_int = 23i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:698`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L698)*

### `ES`
```rust
const ES: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:699`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L699)*

### `FS`
```rust
const FS: crate::c_int = 25i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:700`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L700)*

### `GS`
```rust
const GS: crate::c_int = 26i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:701`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L701)*

### `REG_R8`
```rust
const REG_R8: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:704`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L704)*

### `REG_R9`
```rust
const REG_R9: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:705`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L705)*

### `REG_R10`
```rust
const REG_R10: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:706`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L706)*

### `REG_R11`
```rust
const REG_R11: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:707`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L707)*

### `REG_R12`
```rust
const REG_R12: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:708`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L708)*

### `REG_R13`
```rust
const REG_R13: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:709`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L709)*

### `REG_R14`
```rust
const REG_R14: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:710`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L710)*

### `REG_R15`
```rust
const REG_R15: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:711`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L711)*

### `REG_RDI`
```rust
const REG_RDI: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:712`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L712)*

### `REG_RSI`
```rust
const REG_RSI: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:713`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L713)*

### `REG_RBP`
```rust
const REG_RBP: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:714`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L714)*

### `REG_RBX`
```rust
const REG_RBX: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:715`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L715)*

### `REG_RDX`
```rust
const REG_RDX: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:716`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L716)*

### `REG_RAX`
```rust
const REG_RAX: crate::c_int = 13i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:717`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L717)*

### `REG_RCX`
```rust
const REG_RCX: crate::c_int = 14i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:718`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L718)*

### `REG_RSP`
```rust
const REG_RSP: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:719`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L719)*

### `REG_RIP`
```rust
const REG_RIP: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:720`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L720)*

### `REG_EFL`
```rust
const REG_EFL: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:721`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L721)*

### `REG_CSGSFS`
```rust
const REG_CSGSFS: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:722`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L722)*

### `REG_ERR`
```rust
const REG_ERR: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:723`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L723)*

### `REG_TRAPNO`
```rust
const REG_TRAPNO: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:724`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L724)*

### `REG_OLDMASK`
```rust
const REG_OLDMASK: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:725`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L725)*

### `REG_CR2`
```rust
const REG_CR2: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs:726`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs#L726)*

### `__SIZEOF_PTHREAD_MUTEX_T`
```rust
const __SIZEOF_PTHREAD_MUTEX_T: usize = 40usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:21`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L21)*

### `__SIZEOF_PTHREAD_RWLOCK_T`
```rust
const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:22`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L22)*

### `__SIZEOF_PTHREAD_BARRIER_T`
```rust
const __SIZEOF_PTHREAD_BARRIER_T: usize = 32usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:23`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L23)*

### `PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP`
```rust
const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:26-31`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L26-L31)*

### `PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP`
```rust
const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:33-38`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L33-L38)*

### `PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP`
```rust
const PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP: crate::pthread_mutex_t;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:40-45`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L40-L45)*

### `SYS_read`
```rust
const SYS_read: crate::c_long = 0i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:70`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L70)*

### `SYS_write`
```rust
const SYS_write: crate::c_long = 1i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:71`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L71)*

### `SYS_open`
```rust
const SYS_open: crate::c_long = 2i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:72`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L72)*

### `SYS_close`
```rust
const SYS_close: crate::c_long = 3i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:73`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L73)*

### `SYS_stat`
```rust
const SYS_stat: crate::c_long = 4i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:74`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L74)*

### `SYS_fstat`
```rust
const SYS_fstat: crate::c_long = 5i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:75`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L75)*

### `SYS_lstat`
```rust
const SYS_lstat: crate::c_long = 6i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:76`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L76)*

### `SYS_poll`
```rust
const SYS_poll: crate::c_long = 7i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:77`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L77)*

### `SYS_lseek`
```rust
const SYS_lseek: crate::c_long = 8i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:78`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L78)*

### `SYS_mmap`
```rust
const SYS_mmap: crate::c_long = 9i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:79`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L79)*

### `SYS_mprotect`
```rust
const SYS_mprotect: crate::c_long = 10i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:80`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L80)*

### `SYS_munmap`
```rust
const SYS_munmap: crate::c_long = 11i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:81`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L81)*

### `SYS_brk`
```rust
const SYS_brk: crate::c_long = 12i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:82`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L82)*

### `SYS_rt_sigaction`
```rust
const SYS_rt_sigaction: crate::c_long = 13i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:83`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L83)*

### `SYS_rt_sigprocmask`
```rust
const SYS_rt_sigprocmask: crate::c_long = 14i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:84`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L84)*

### `SYS_rt_sigreturn`
```rust
const SYS_rt_sigreturn: crate::c_long = 15i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:85`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L85)*

### `SYS_ioctl`
```rust
const SYS_ioctl: crate::c_long = 16i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:86`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L86)*

### `SYS_pread64`
```rust
const SYS_pread64: crate::c_long = 17i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:87`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L87)*

### `SYS_pwrite64`
```rust
const SYS_pwrite64: crate::c_long = 18i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:88`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L88)*

### `SYS_readv`
```rust
const SYS_readv: crate::c_long = 19i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:89`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L89)*

### `SYS_writev`
```rust
const SYS_writev: crate::c_long = 20i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:90`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L90)*

### `SYS_access`
```rust
const SYS_access: crate::c_long = 21i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:91`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L91)*

### `SYS_pipe`
```rust
const SYS_pipe: crate::c_long = 22i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:92`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L92)*

### `SYS_select`
```rust
const SYS_select: crate::c_long = 23i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:93`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L93)*

### `SYS_sched_yield`
```rust
const SYS_sched_yield: crate::c_long = 24i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:94`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L94)*

### `SYS_mremap`
```rust
const SYS_mremap: crate::c_long = 25i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:95`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L95)*

### `SYS_msync`
```rust
const SYS_msync: crate::c_long = 26i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:96`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L96)*

### `SYS_mincore`
```rust
const SYS_mincore: crate::c_long = 27i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:97`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L97)*

### `SYS_madvise`
```rust
const SYS_madvise: crate::c_long = 28i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:98`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L98)*

### `SYS_shmget`
```rust
const SYS_shmget: crate::c_long = 29i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:99`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L99)*

### `SYS_shmat`
```rust
const SYS_shmat: crate::c_long = 30i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:100`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L100)*

### `SYS_shmctl`
```rust
const SYS_shmctl: crate::c_long = 31i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:101`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L101)*

### `SYS_dup`
```rust
const SYS_dup: crate::c_long = 32i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:102`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L102)*

### `SYS_dup2`
```rust
const SYS_dup2: crate::c_long = 33i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:103`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L103)*

### `SYS_pause`
```rust
const SYS_pause: crate::c_long = 34i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:104`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L104)*

### `SYS_nanosleep`
```rust
const SYS_nanosleep: crate::c_long = 35i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:105`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L105)*

### `SYS_getitimer`
```rust
const SYS_getitimer: crate::c_long = 36i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:106`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L106)*

### `SYS_alarm`
```rust
const SYS_alarm: crate::c_long = 37i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:107`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L107)*

### `SYS_setitimer`
```rust
const SYS_setitimer: crate::c_long = 38i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:108`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L108)*

### `SYS_getpid`
```rust
const SYS_getpid: crate::c_long = 39i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:109`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L109)*

### `SYS_sendfile`
```rust
const SYS_sendfile: crate::c_long = 40i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:110`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L110)*

### `SYS_socket`
```rust
const SYS_socket: crate::c_long = 41i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:111`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L111)*

### `SYS_connect`
```rust
const SYS_connect: crate::c_long = 42i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:112`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L112)*

### `SYS_accept`
```rust
const SYS_accept: crate::c_long = 43i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:113`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L113)*

### `SYS_sendto`
```rust
const SYS_sendto: crate::c_long = 44i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:114`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L114)*

### `SYS_recvfrom`
```rust
const SYS_recvfrom: crate::c_long = 45i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:115`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L115)*

### `SYS_sendmsg`
```rust
const SYS_sendmsg: crate::c_long = 46i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:116`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L116)*

### `SYS_recvmsg`
```rust
const SYS_recvmsg: crate::c_long = 47i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:117`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L117)*

### `SYS_shutdown`
```rust
const SYS_shutdown: crate::c_long = 48i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:118`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L118)*

### `SYS_bind`
```rust
const SYS_bind: crate::c_long = 49i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:119`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L119)*

### `SYS_listen`
```rust
const SYS_listen: crate::c_long = 50i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:120`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L120)*

### `SYS_getsockname`
```rust
const SYS_getsockname: crate::c_long = 51i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:121`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L121)*

### `SYS_getpeername`
```rust
const SYS_getpeername: crate::c_long = 52i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:122`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L122)*

### `SYS_socketpair`
```rust
const SYS_socketpair: crate::c_long = 53i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:123`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L123)*

### `SYS_setsockopt`
```rust
const SYS_setsockopt: crate::c_long = 54i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:124`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L124)*

### `SYS_getsockopt`
```rust
const SYS_getsockopt: crate::c_long = 55i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:125`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L125)*

### `SYS_clone`
```rust
const SYS_clone: crate::c_long = 56i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:126`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L126)*

### `SYS_fork`
```rust
const SYS_fork: crate::c_long = 57i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:127`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L127)*

### `SYS_vfork`
```rust
const SYS_vfork: crate::c_long = 58i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:128`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L128)*

### `SYS_execve`
```rust
const SYS_execve: crate::c_long = 59i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:129`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L129)*

### `SYS_exit`
```rust
const SYS_exit: crate::c_long = 60i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:130`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L130)*

### `SYS_wait4`
```rust
const SYS_wait4: crate::c_long = 61i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:131`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L131)*

### `SYS_kill`
```rust
const SYS_kill: crate::c_long = 62i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:132`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L132)*

### `SYS_uname`
```rust
const SYS_uname: crate::c_long = 63i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:133`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L133)*

### `SYS_semget`
```rust
const SYS_semget: crate::c_long = 64i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:134`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L134)*

### `SYS_semop`
```rust
const SYS_semop: crate::c_long = 65i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:135`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L135)*

### `SYS_semctl`
```rust
const SYS_semctl: crate::c_long = 66i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:136`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L136)*

### `SYS_shmdt`
```rust
const SYS_shmdt: crate::c_long = 67i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:137`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L137)*

### `SYS_msgget`
```rust
const SYS_msgget: crate::c_long = 68i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:138`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L138)*

### `SYS_msgsnd`
```rust
const SYS_msgsnd: crate::c_long = 69i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:139`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L139)*

### `SYS_msgrcv`
```rust
const SYS_msgrcv: crate::c_long = 70i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:140`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L140)*

### `SYS_msgctl`
```rust
const SYS_msgctl: crate::c_long = 71i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:141`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L141)*

### `SYS_fcntl`
```rust
const SYS_fcntl: crate::c_long = 72i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:142`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L142)*

### `SYS_flock`
```rust
const SYS_flock: crate::c_long = 73i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:143`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L143)*

### `SYS_fsync`
```rust
const SYS_fsync: crate::c_long = 74i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:144`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L144)*

### `SYS_fdatasync`
```rust
const SYS_fdatasync: crate::c_long = 75i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:145`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L145)*

### `SYS_truncate`
```rust
const SYS_truncate: crate::c_long = 76i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:146`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L146)*

### `SYS_ftruncate`
```rust
const SYS_ftruncate: crate::c_long = 77i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:147`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L147)*

### `SYS_getdents`
```rust
const SYS_getdents: crate::c_long = 78i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:148`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L148)*

### `SYS_getcwd`
```rust
const SYS_getcwd: crate::c_long = 79i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:149`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L149)*

### `SYS_chdir`
```rust
const SYS_chdir: crate::c_long = 80i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:150`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L150)*

### `SYS_fchdir`
```rust
const SYS_fchdir: crate::c_long = 81i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:151`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L151)*

### `SYS_rename`
```rust
const SYS_rename: crate::c_long = 82i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:152`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L152)*

### `SYS_mkdir`
```rust
const SYS_mkdir: crate::c_long = 83i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:153`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L153)*

### `SYS_rmdir`
```rust
const SYS_rmdir: crate::c_long = 84i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:154`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L154)*

### `SYS_creat`
```rust
const SYS_creat: crate::c_long = 85i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:155`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L155)*

### `SYS_link`
```rust
const SYS_link: crate::c_long = 86i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:156`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L156)*

### `SYS_unlink`
```rust
const SYS_unlink: crate::c_long = 87i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:157`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L157)*

### `SYS_symlink`
```rust
const SYS_symlink: crate::c_long = 88i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:158`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L158)*

### `SYS_readlink`
```rust
const SYS_readlink: crate::c_long = 89i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:159`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L159)*

### `SYS_chmod`
```rust
const SYS_chmod: crate::c_long = 90i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:160`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L160)*

### `SYS_fchmod`
```rust
const SYS_fchmod: crate::c_long = 91i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:161`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L161)*

### `SYS_chown`
```rust
const SYS_chown: crate::c_long = 92i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:162`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L162)*

### `SYS_fchown`
```rust
const SYS_fchown: crate::c_long = 93i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:163`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L163)*

### `SYS_lchown`
```rust
const SYS_lchown: crate::c_long = 94i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:164`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L164)*

### `SYS_umask`
```rust
const SYS_umask: crate::c_long = 95i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:165`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L165)*

### `SYS_gettimeofday`
```rust
const SYS_gettimeofday: crate::c_long = 96i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:166`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L166)*

### `SYS_getrlimit`
```rust
const SYS_getrlimit: crate::c_long = 97i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:167`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L167)*

### `SYS_getrusage`
```rust
const SYS_getrusage: crate::c_long = 98i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:168`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L168)*

### `SYS_sysinfo`
```rust
const SYS_sysinfo: crate::c_long = 99i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:169`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L169)*

### `SYS_times`
```rust
const SYS_times: crate::c_long = 100i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:170`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L170)*

### `SYS_ptrace`
```rust
const SYS_ptrace: crate::c_long = 101i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:171`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L171)*

### `SYS_getuid`
```rust
const SYS_getuid: crate::c_long = 102i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:172`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L172)*

### `SYS_syslog`
```rust
const SYS_syslog: crate::c_long = 103i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:173`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L173)*

### `SYS_getgid`
```rust
const SYS_getgid: crate::c_long = 104i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:174`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L174)*

### `SYS_setuid`
```rust
const SYS_setuid: crate::c_long = 105i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:175`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L175)*

### `SYS_setgid`
```rust
const SYS_setgid: crate::c_long = 106i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:176`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L176)*

### `SYS_geteuid`
```rust
const SYS_geteuid: crate::c_long = 107i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:177`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L177)*

### `SYS_getegid`
```rust
const SYS_getegid: crate::c_long = 108i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:178`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L178)*

### `SYS_setpgid`
```rust
const SYS_setpgid: crate::c_long = 109i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:179`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L179)*

### `SYS_getppid`
```rust
const SYS_getppid: crate::c_long = 110i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:180`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L180)*

### `SYS_getpgrp`
```rust
const SYS_getpgrp: crate::c_long = 111i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:181`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L181)*

### `SYS_setsid`
```rust
const SYS_setsid: crate::c_long = 112i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:182`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L182)*

### `SYS_setreuid`
```rust
const SYS_setreuid: crate::c_long = 113i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:183`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L183)*

### `SYS_setregid`
```rust
const SYS_setregid: crate::c_long = 114i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:184`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L184)*

### `SYS_getgroups`
```rust
const SYS_getgroups: crate::c_long = 115i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:185`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L185)*

### `SYS_setgroups`
```rust
const SYS_setgroups: crate::c_long = 116i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:186`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L186)*

### `SYS_setresuid`
```rust
const SYS_setresuid: crate::c_long = 117i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:187`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L187)*

### `SYS_getresuid`
```rust
const SYS_getresuid: crate::c_long = 118i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:188`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L188)*

### `SYS_setresgid`
```rust
const SYS_setresgid: crate::c_long = 119i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:189`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L189)*

### `SYS_getresgid`
```rust
const SYS_getresgid: crate::c_long = 120i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:190`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L190)*

### `SYS_getpgid`
```rust
const SYS_getpgid: crate::c_long = 121i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:191`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L191)*

### `SYS_setfsuid`
```rust
const SYS_setfsuid: crate::c_long = 122i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:192`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L192)*

### `SYS_setfsgid`
```rust
const SYS_setfsgid: crate::c_long = 123i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:193`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L193)*

### `SYS_getsid`
```rust
const SYS_getsid: crate::c_long = 124i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:194`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L194)*

### `SYS_capget`
```rust
const SYS_capget: crate::c_long = 125i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:195`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L195)*

### `SYS_capset`
```rust
const SYS_capset: crate::c_long = 126i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:196`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L196)*

### `SYS_rt_sigpending`
```rust
const SYS_rt_sigpending: crate::c_long = 127i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:197`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L197)*

### `SYS_rt_sigtimedwait`
```rust
const SYS_rt_sigtimedwait: crate::c_long = 128i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:198`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L198)*

### `SYS_rt_sigqueueinfo`
```rust
const SYS_rt_sigqueueinfo: crate::c_long = 129i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:199`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L199)*

### `SYS_rt_sigsuspend`
```rust
const SYS_rt_sigsuspend: crate::c_long = 130i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:200`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L200)*

### `SYS_sigaltstack`
```rust
const SYS_sigaltstack: crate::c_long = 131i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:201`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L201)*

### `SYS_utime`
```rust
const SYS_utime: crate::c_long = 132i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:202`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L202)*

### `SYS_mknod`
```rust
const SYS_mknod: crate::c_long = 133i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:203`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L203)*

### `SYS_uselib`
```rust
const SYS_uselib: crate::c_long = 134i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:204`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L204)*

### `SYS_personality`
```rust
const SYS_personality: crate::c_long = 135i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:205`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L205)*

### `SYS_ustat`
```rust
const SYS_ustat: crate::c_long = 136i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:206`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L206)*

### `SYS_statfs`
```rust
const SYS_statfs: crate::c_long = 137i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:207`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L207)*

### `SYS_fstatfs`
```rust
const SYS_fstatfs: crate::c_long = 138i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:208`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L208)*

### `SYS_sysfs`
```rust
const SYS_sysfs: crate::c_long = 139i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:209`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L209)*

### `SYS_getpriority`
```rust
const SYS_getpriority: crate::c_long = 140i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:210`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L210)*

### `SYS_setpriority`
```rust
const SYS_setpriority: crate::c_long = 141i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:211`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L211)*

### `SYS_sched_setparam`
```rust
const SYS_sched_setparam: crate::c_long = 142i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:212`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L212)*

### `SYS_sched_getparam`
```rust
const SYS_sched_getparam: crate::c_long = 143i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:213`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L213)*

### `SYS_sched_setscheduler`
```rust
const SYS_sched_setscheduler: crate::c_long = 144i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:214`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L214)*

### `SYS_sched_getscheduler`
```rust
const SYS_sched_getscheduler: crate::c_long = 145i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:215`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L215)*

### `SYS_sched_get_priority_max`
```rust
const SYS_sched_get_priority_max: crate::c_long = 146i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:216`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L216)*

### `SYS_sched_get_priority_min`
```rust
const SYS_sched_get_priority_min: crate::c_long = 147i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:217`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L217)*

### `SYS_sched_rr_get_interval`
```rust
const SYS_sched_rr_get_interval: crate::c_long = 148i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:218`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L218)*

### `SYS_mlock`
```rust
const SYS_mlock: crate::c_long = 149i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:219`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L219)*

### `SYS_munlock`
```rust
const SYS_munlock: crate::c_long = 150i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:220`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L220)*

### `SYS_mlockall`
```rust
const SYS_mlockall: crate::c_long = 151i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:221`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L221)*

### `SYS_munlockall`
```rust
const SYS_munlockall: crate::c_long = 152i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:222`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L222)*

### `SYS_vhangup`
```rust
const SYS_vhangup: crate::c_long = 153i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:223`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L223)*

### `SYS_modify_ldt`
```rust
const SYS_modify_ldt: crate::c_long = 154i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:224`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L224)*

### `SYS_pivot_root`
```rust
const SYS_pivot_root: crate::c_long = 155i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:225`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L225)*

### `SYS__sysctl`
```rust
const SYS__sysctl: crate::c_long = 156i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:226`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L226)*

### `SYS_prctl`
```rust
const SYS_prctl: crate::c_long = 157i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:227`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L227)*

### `SYS_arch_prctl`
```rust
const SYS_arch_prctl: crate::c_long = 158i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:228`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L228)*

### `SYS_adjtimex`
```rust
const SYS_adjtimex: crate::c_long = 159i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:229`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L229)*

### `SYS_setrlimit`
```rust
const SYS_setrlimit: crate::c_long = 160i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:230`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L230)*

### `SYS_chroot`
```rust
const SYS_chroot: crate::c_long = 161i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:231`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L231)*

### `SYS_sync`
```rust
const SYS_sync: crate::c_long = 162i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:232`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L232)*

### `SYS_acct`
```rust
const SYS_acct: crate::c_long = 163i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:233`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L233)*

### `SYS_settimeofday`
```rust
const SYS_settimeofday: crate::c_long = 164i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:234`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L234)*

### `SYS_mount`
```rust
const SYS_mount: crate::c_long = 165i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:235`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L235)*

### `SYS_umount2`
```rust
const SYS_umount2: crate::c_long = 166i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:236`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L236)*

### `SYS_swapon`
```rust
const SYS_swapon: crate::c_long = 167i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:237`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L237)*

### `SYS_swapoff`
```rust
const SYS_swapoff: crate::c_long = 168i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:238`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L238)*

### `SYS_reboot`
```rust
const SYS_reboot: crate::c_long = 169i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:239`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L239)*

### `SYS_sethostname`
```rust
const SYS_sethostname: crate::c_long = 170i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:240`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L240)*

### `SYS_setdomainname`
```rust
const SYS_setdomainname: crate::c_long = 171i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:241`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L241)*

### `SYS_iopl`
```rust
const SYS_iopl: crate::c_long = 172i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:242`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L242)*

### `SYS_ioperm`
```rust
const SYS_ioperm: crate::c_long = 173i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:243`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L243)*

### `SYS_create_module`
```rust
const SYS_create_module: crate::c_long = 174i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:245`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L245)*

### `SYS_init_module`
```rust
const SYS_init_module: crate::c_long = 175i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:246`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L246)*

### `SYS_delete_module`
```rust
const SYS_delete_module: crate::c_long = 176i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:247`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L247)*

### `SYS_get_kernel_syms`
```rust
const SYS_get_kernel_syms: crate::c_long = 177i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:249`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L249)*

### `SYS_query_module`
```rust
const SYS_query_module: crate::c_long = 178i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:251`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L251)*

### `SYS_quotactl`
```rust
const SYS_quotactl: crate::c_long = 179i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:252`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L252)*

### `SYS_nfsservctl`
```rust
const SYS_nfsservctl: crate::c_long = 180i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:253`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L253)*

### `SYS_getpmsg`
```rust
const SYS_getpmsg: crate::c_long = 181i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:254`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L254)*

### `SYS_putpmsg`
```rust
const SYS_putpmsg: crate::c_long = 182i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:255`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L255)*

### `SYS_afs_syscall`
```rust
const SYS_afs_syscall: crate::c_long = 183i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:256`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L256)*

### `SYS_tuxcall`
```rust
const SYS_tuxcall: crate::c_long = 184i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:257`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L257)*

### `SYS_security`
```rust
const SYS_security: crate::c_long = 185i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:258`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L258)*

### `SYS_gettid`
```rust
const SYS_gettid: crate::c_long = 186i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:259`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L259)*

### `SYS_readahead`
```rust
const SYS_readahead: crate::c_long = 187i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:260`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L260)*

### `SYS_setxattr`
```rust
const SYS_setxattr: crate::c_long = 188i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:261`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L261)*

### `SYS_lsetxattr`
```rust
const SYS_lsetxattr: crate::c_long = 189i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:262`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L262)*

### `SYS_fsetxattr`
```rust
const SYS_fsetxattr: crate::c_long = 190i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:263`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L263)*

### `SYS_getxattr`
```rust
const SYS_getxattr: crate::c_long = 191i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:264`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L264)*

### `SYS_lgetxattr`
```rust
const SYS_lgetxattr: crate::c_long = 192i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:265`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L265)*

### `SYS_fgetxattr`
```rust
const SYS_fgetxattr: crate::c_long = 193i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:266`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L266)*

### `SYS_listxattr`
```rust
const SYS_listxattr: crate::c_long = 194i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:267`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L267)*

### `SYS_llistxattr`
```rust
const SYS_llistxattr: crate::c_long = 195i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:268`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L268)*

### `SYS_flistxattr`
```rust
const SYS_flistxattr: crate::c_long = 196i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:269`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L269)*

### `SYS_removexattr`
```rust
const SYS_removexattr: crate::c_long = 197i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:270`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L270)*

### `SYS_lremovexattr`
```rust
const SYS_lremovexattr: crate::c_long = 198i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:271`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L271)*

### `SYS_fremovexattr`
```rust
const SYS_fremovexattr: crate::c_long = 199i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:272`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L272)*

### `SYS_tkill`
```rust
const SYS_tkill: crate::c_long = 200i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:273`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L273)*

### `SYS_time`
```rust
const SYS_time: crate::c_long = 201i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:274`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L274)*

### `SYS_futex`
```rust
const SYS_futex: crate::c_long = 202i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:275`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L275)*

### `SYS_sched_setaffinity`
```rust
const SYS_sched_setaffinity: crate::c_long = 203i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:276`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L276)*

### `SYS_sched_getaffinity`
```rust
const SYS_sched_getaffinity: crate::c_long = 204i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:277`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L277)*

### `SYS_set_thread_area`
```rust
const SYS_set_thread_area: crate::c_long = 205i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:278`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L278)*

### `SYS_io_setup`
```rust
const SYS_io_setup: crate::c_long = 206i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:279`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L279)*

### `SYS_io_destroy`
```rust
const SYS_io_destroy: crate::c_long = 207i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:280`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L280)*

### `SYS_io_getevents`
```rust
const SYS_io_getevents: crate::c_long = 208i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:281`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L281)*

### `SYS_io_submit`
```rust
const SYS_io_submit: crate::c_long = 209i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:282`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L282)*

### `SYS_io_cancel`
```rust
const SYS_io_cancel: crate::c_long = 210i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:283`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L283)*

### `SYS_get_thread_area`
```rust
const SYS_get_thread_area: crate::c_long = 211i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:284`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L284)*

### `SYS_lookup_dcookie`
```rust
const SYS_lookup_dcookie: crate::c_long = 212i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:285`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L285)*

### `SYS_epoll_create`
```rust
const SYS_epoll_create: crate::c_long = 213i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:286`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L286)*

### `SYS_epoll_ctl_old`
```rust
const SYS_epoll_ctl_old: crate::c_long = 214i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:287`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L287)*

### `SYS_epoll_wait_old`
```rust
const SYS_epoll_wait_old: crate::c_long = 215i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:288`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L288)*

### `SYS_remap_file_pages`
```rust
const SYS_remap_file_pages: crate::c_long = 216i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:289`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L289)*

### `SYS_getdents64`
```rust
const SYS_getdents64: crate::c_long = 217i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:290`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L290)*

### `SYS_set_tid_address`
```rust
const SYS_set_tid_address: crate::c_long = 218i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:291`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L291)*

### `SYS_restart_syscall`
```rust
const SYS_restart_syscall: crate::c_long = 219i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:292`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L292)*

### `SYS_semtimedop`
```rust
const SYS_semtimedop: crate::c_long = 220i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:293`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L293)*

### `SYS_fadvise64`
```rust
const SYS_fadvise64: crate::c_long = 221i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:294`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L294)*

### `SYS_timer_create`
```rust
const SYS_timer_create: crate::c_long = 222i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:295`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L295)*

### `SYS_timer_settime`
```rust
const SYS_timer_settime: crate::c_long = 223i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:296`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L296)*

### `SYS_timer_gettime`
```rust
const SYS_timer_gettime: crate::c_long = 224i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:297`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L297)*

### `SYS_timer_getoverrun`
```rust
const SYS_timer_getoverrun: crate::c_long = 225i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:298`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L298)*

### `SYS_timer_delete`
```rust
const SYS_timer_delete: crate::c_long = 226i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:299`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L299)*

### `SYS_clock_settime`
```rust
const SYS_clock_settime: crate::c_long = 227i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:300`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L300)*

### `SYS_clock_gettime`
```rust
const SYS_clock_gettime: crate::c_long = 228i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:301`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L301)*

### `SYS_clock_getres`
```rust
const SYS_clock_getres: crate::c_long = 229i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:302`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L302)*

### `SYS_clock_nanosleep`
```rust
const SYS_clock_nanosleep: crate::c_long = 230i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:303`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L303)*

### `SYS_exit_group`
```rust
const SYS_exit_group: crate::c_long = 231i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:304`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L304)*

### `SYS_epoll_wait`
```rust
const SYS_epoll_wait: crate::c_long = 232i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:305`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L305)*

### `SYS_epoll_ctl`
```rust
const SYS_epoll_ctl: crate::c_long = 233i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:306`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L306)*

### `SYS_tgkill`
```rust
const SYS_tgkill: crate::c_long = 234i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:307`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L307)*

### `SYS_utimes`
```rust
const SYS_utimes: crate::c_long = 235i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:308`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L308)*

### `SYS_vserver`
```rust
const SYS_vserver: crate::c_long = 236i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:309`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L309)*

### `SYS_mbind`
```rust
const SYS_mbind: crate::c_long = 237i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:310`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L310)*

### `SYS_set_mempolicy`
```rust
const SYS_set_mempolicy: crate::c_long = 238i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:311`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L311)*

### `SYS_get_mempolicy`
```rust
const SYS_get_mempolicy: crate::c_long = 239i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:312`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L312)*

### `SYS_mq_open`
```rust
const SYS_mq_open: crate::c_long = 240i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:313`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L313)*

### `SYS_mq_unlink`
```rust
const SYS_mq_unlink: crate::c_long = 241i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:314`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L314)*

### `SYS_mq_timedsend`
```rust
const SYS_mq_timedsend: crate::c_long = 242i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:315`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L315)*

### `SYS_mq_timedreceive`
```rust
const SYS_mq_timedreceive: crate::c_long = 243i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:316`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L316)*

### `SYS_mq_notify`
```rust
const SYS_mq_notify: crate::c_long = 244i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:317`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L317)*

### `SYS_mq_getsetattr`
```rust
const SYS_mq_getsetattr: crate::c_long = 245i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:318`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L318)*

### `SYS_kexec_load`
```rust
const SYS_kexec_load: crate::c_long = 246i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:319`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L319)*

### `SYS_waitid`
```rust
const SYS_waitid: crate::c_long = 247i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:320`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L320)*

### `SYS_add_key`
```rust
const SYS_add_key: crate::c_long = 248i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:321`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L321)*

### `SYS_request_key`
```rust
const SYS_request_key: crate::c_long = 249i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:322`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L322)*

### `SYS_keyctl`
```rust
const SYS_keyctl: crate::c_long = 250i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:323`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L323)*

### `SYS_ioprio_set`
```rust
const SYS_ioprio_set: crate::c_long = 251i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:324`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L324)*

### `SYS_ioprio_get`
```rust
const SYS_ioprio_get: crate::c_long = 252i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:325`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L325)*

### `SYS_inotify_init`
```rust
const SYS_inotify_init: crate::c_long = 253i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:326`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L326)*

### `SYS_inotify_add_watch`
```rust
const SYS_inotify_add_watch: crate::c_long = 254i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:327`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L327)*

### `SYS_inotify_rm_watch`
```rust
const SYS_inotify_rm_watch: crate::c_long = 255i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:328`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L328)*

### `SYS_migrate_pages`
```rust
const SYS_migrate_pages: crate::c_long = 256i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:329`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L329)*

### `SYS_openat`
```rust
const SYS_openat: crate::c_long = 257i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:330`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L330)*

### `SYS_mkdirat`
```rust
const SYS_mkdirat: crate::c_long = 258i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:331`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L331)*

### `SYS_mknodat`
```rust
const SYS_mknodat: crate::c_long = 259i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:332`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L332)*

### `SYS_fchownat`
```rust
const SYS_fchownat: crate::c_long = 260i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:333`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L333)*

### `SYS_futimesat`
```rust
const SYS_futimesat: crate::c_long = 261i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:334`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L334)*

### `SYS_newfstatat`
```rust
const SYS_newfstatat: crate::c_long = 262i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:335`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L335)*

### `SYS_unlinkat`
```rust
const SYS_unlinkat: crate::c_long = 263i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:336`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L336)*

### `SYS_renameat`
```rust
const SYS_renameat: crate::c_long = 264i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:337`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L337)*

### `SYS_linkat`
```rust
const SYS_linkat: crate::c_long = 265i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:338`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L338)*

### `SYS_symlinkat`
```rust
const SYS_symlinkat: crate::c_long = 266i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:339`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L339)*

### `SYS_readlinkat`
```rust
const SYS_readlinkat: crate::c_long = 267i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:340`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L340)*

### `SYS_fchmodat`
```rust
const SYS_fchmodat: crate::c_long = 268i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:341`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L341)*

### `SYS_faccessat`
```rust
const SYS_faccessat: crate::c_long = 269i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:342`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L342)*

### `SYS_pselect6`
```rust
const SYS_pselect6: crate::c_long = 270i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:343`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L343)*

### `SYS_ppoll`
```rust
const SYS_ppoll: crate::c_long = 271i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:344`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L344)*

### `SYS_unshare`
```rust
const SYS_unshare: crate::c_long = 272i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:345`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L345)*

### `SYS_set_robust_list`
```rust
const SYS_set_robust_list: crate::c_long = 273i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:346`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L346)*

### `SYS_get_robust_list`
```rust
const SYS_get_robust_list: crate::c_long = 274i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:347`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L347)*

### `SYS_splice`
```rust
const SYS_splice: crate::c_long = 275i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:348`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L348)*

### `SYS_tee`
```rust
const SYS_tee: crate::c_long = 276i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:349`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L349)*

### `SYS_sync_file_range`
```rust
const SYS_sync_file_range: crate::c_long = 277i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:350`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L350)*

### `SYS_vmsplice`
```rust
const SYS_vmsplice: crate::c_long = 278i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:351`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L351)*

### `SYS_move_pages`
```rust
const SYS_move_pages: crate::c_long = 279i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:352`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L352)*

### `SYS_utimensat`
```rust
const SYS_utimensat: crate::c_long = 280i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:353`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L353)*

### `SYS_epoll_pwait`
```rust
const SYS_epoll_pwait: crate::c_long = 281i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:354`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L354)*

### `SYS_signalfd`
```rust
const SYS_signalfd: crate::c_long = 282i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:355`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L355)*

### `SYS_timerfd_create`
```rust
const SYS_timerfd_create: crate::c_long = 283i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:356`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L356)*

### `SYS_eventfd`
```rust
const SYS_eventfd: crate::c_long = 284i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:357`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L357)*

### `SYS_fallocate`
```rust
const SYS_fallocate: crate::c_long = 285i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:358`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L358)*

### `SYS_timerfd_settime`
```rust
const SYS_timerfd_settime: crate::c_long = 286i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:359`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L359)*

### `SYS_timerfd_gettime`
```rust
const SYS_timerfd_gettime: crate::c_long = 287i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:360`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L360)*

### `SYS_accept4`
```rust
const SYS_accept4: crate::c_long = 288i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:361`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L361)*

### `SYS_signalfd4`
```rust
const SYS_signalfd4: crate::c_long = 289i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:362`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L362)*

### `SYS_eventfd2`
```rust
const SYS_eventfd2: crate::c_long = 290i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:363`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L363)*

### `SYS_epoll_create1`
```rust
const SYS_epoll_create1: crate::c_long = 291i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:364`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L364)*

### `SYS_dup3`
```rust
const SYS_dup3: crate::c_long = 292i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:365`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L365)*

### `SYS_pipe2`
```rust
const SYS_pipe2: crate::c_long = 293i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:366`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L366)*

### `SYS_inotify_init1`
```rust
const SYS_inotify_init1: crate::c_long = 294i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:367`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L367)*

### `SYS_preadv`
```rust
const SYS_preadv: crate::c_long = 295i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:368`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L368)*

### `SYS_pwritev`
```rust
const SYS_pwritev: crate::c_long = 296i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:369`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L369)*

### `SYS_rt_tgsigqueueinfo`
```rust
const SYS_rt_tgsigqueueinfo: crate::c_long = 297i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:370`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L370)*

### `SYS_perf_event_open`
```rust
const SYS_perf_event_open: crate::c_long = 298i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:371`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L371)*

### `SYS_recvmmsg`
```rust
const SYS_recvmmsg: crate::c_long = 299i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:372`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L372)*

### `SYS_fanotify_init`
```rust
const SYS_fanotify_init: crate::c_long = 300i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:373`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L373)*

### `SYS_fanotify_mark`
```rust
const SYS_fanotify_mark: crate::c_long = 301i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:374`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L374)*

### `SYS_prlimit64`
```rust
const SYS_prlimit64: crate::c_long = 302i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:375`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L375)*

### `SYS_name_to_handle_at`
```rust
const SYS_name_to_handle_at: crate::c_long = 303i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:376`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L376)*

### `SYS_open_by_handle_at`
```rust
const SYS_open_by_handle_at: crate::c_long = 304i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:377`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L377)*

### `SYS_clock_adjtime`
```rust
const SYS_clock_adjtime: crate::c_long = 305i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:378`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L378)*

### `SYS_syncfs`
```rust
const SYS_syncfs: crate::c_long = 306i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:379`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L379)*

### `SYS_sendmmsg`
```rust
const SYS_sendmmsg: crate::c_long = 307i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:380`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L380)*

### `SYS_setns`
```rust
const SYS_setns: crate::c_long = 308i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:381`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L381)*

### `SYS_getcpu`
```rust
const SYS_getcpu: crate::c_long = 309i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:382`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L382)*

### `SYS_process_vm_readv`
```rust
const SYS_process_vm_readv: crate::c_long = 310i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:383`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L383)*

### `SYS_process_vm_writev`
```rust
const SYS_process_vm_writev: crate::c_long = 311i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:384`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L384)*

### `SYS_kcmp`
```rust
const SYS_kcmp: crate::c_long = 312i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:385`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L385)*

### `SYS_finit_module`
```rust
const SYS_finit_module: crate::c_long = 313i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:386`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L386)*

### `SYS_sched_setattr`
```rust
const SYS_sched_setattr: crate::c_long = 314i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:387`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L387)*

### `SYS_sched_getattr`
```rust
const SYS_sched_getattr: crate::c_long = 315i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:388`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L388)*

### `SYS_renameat2`
```rust
const SYS_renameat2: crate::c_long = 316i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:389`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L389)*

### `SYS_seccomp`
```rust
const SYS_seccomp: crate::c_long = 317i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:390`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L390)*

### `SYS_getrandom`
```rust
const SYS_getrandom: crate::c_long = 318i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:391`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L391)*

### `SYS_memfd_create`
```rust
const SYS_memfd_create: crate::c_long = 319i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:392`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L392)*

### `SYS_kexec_file_load`
```rust
const SYS_kexec_file_load: crate::c_long = 320i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:393`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L393)*

### `SYS_bpf`
```rust
const SYS_bpf: crate::c_long = 321i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:394`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L394)*

### `SYS_execveat`
```rust
const SYS_execveat: crate::c_long = 322i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:395`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L395)*

### `SYS_userfaultfd`
```rust
const SYS_userfaultfd: crate::c_long = 323i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:396`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L396)*

### `SYS_membarrier`
```rust
const SYS_membarrier: crate::c_long = 324i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:397`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L397)*

### `SYS_mlock2`
```rust
const SYS_mlock2: crate::c_long = 325i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:398`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L398)*

### `SYS_copy_file_range`
```rust
const SYS_copy_file_range: crate::c_long = 326i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:399`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L399)*

### `SYS_preadv2`
```rust
const SYS_preadv2: crate::c_long = 327i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:400`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L400)*

### `SYS_pwritev2`
```rust
const SYS_pwritev2: crate::c_long = 328i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:401`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L401)*

### `SYS_pkey_mprotect`
```rust
const SYS_pkey_mprotect: crate::c_long = 329i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:402`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L402)*

### `SYS_pkey_alloc`
```rust
const SYS_pkey_alloc: crate::c_long = 330i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:403`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L403)*

### `SYS_pkey_free`
```rust
const SYS_pkey_free: crate::c_long = 331i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:404`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L404)*

### `SYS_statx`
```rust
const SYS_statx: crate::c_long = 332i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:405`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L405)*

### `SYS_rseq`
```rust
const SYS_rseq: crate::c_long = 334i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:406`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L406)*

### `SYS_pidfd_send_signal`
```rust
const SYS_pidfd_send_signal: crate::c_long = 424i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:407`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L407)*

### `SYS_io_uring_setup`
```rust
const SYS_io_uring_setup: crate::c_long = 425i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:408`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L408)*

### `SYS_io_uring_enter`
```rust
const SYS_io_uring_enter: crate::c_long = 426i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:409`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L409)*

### `SYS_io_uring_register`
```rust
const SYS_io_uring_register: crate::c_long = 427i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:410`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L410)*

### `SYS_open_tree`
```rust
const SYS_open_tree: crate::c_long = 428i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:411`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L411)*

### `SYS_move_mount`
```rust
const SYS_move_mount: crate::c_long = 429i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:412`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L412)*

### `SYS_fsopen`
```rust
const SYS_fsopen: crate::c_long = 430i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:413`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L413)*

### `SYS_fsconfig`
```rust
const SYS_fsconfig: crate::c_long = 431i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:414`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L414)*

### `SYS_fsmount`
```rust
const SYS_fsmount: crate::c_long = 432i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:415`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L415)*

### `SYS_fspick`
```rust
const SYS_fspick: crate::c_long = 433i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:416`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L416)*

### `SYS_pidfd_open`
```rust
const SYS_pidfd_open: crate::c_long = 434i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:417`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L417)*

### `SYS_clone3`
```rust
const SYS_clone3: crate::c_long = 435i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:418`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L418)*

### `SYS_close_range`
```rust
const SYS_close_range: crate::c_long = 436i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:419`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L419)*

### `SYS_openat2`
```rust
const SYS_openat2: crate::c_long = 437i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:420`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L420)*

### `SYS_pidfd_getfd`
```rust
const SYS_pidfd_getfd: crate::c_long = 438i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:421`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L421)*

### `SYS_faccessat2`
```rust
const SYS_faccessat2: crate::c_long = 439i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:422`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L422)*

### `SYS_process_madvise`
```rust
const SYS_process_madvise: crate::c_long = 440i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:423`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L423)*

### `SYS_epoll_pwait2`
```rust
const SYS_epoll_pwait2: crate::c_long = 441i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:424`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L424)*

### `SYS_mount_setattr`
```rust
const SYS_mount_setattr: crate::c_long = 442i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:425`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L425)*

### `SYS_quotactl_fd`
```rust
const SYS_quotactl_fd: crate::c_long = 443i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:426`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L426)*

### `SYS_landlock_create_ruleset`
```rust
const SYS_landlock_create_ruleset: crate::c_long = 444i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:427`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L427)*

### `SYS_landlock_add_rule`
```rust
const SYS_landlock_add_rule: crate::c_long = 445i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:428`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L428)*

### `SYS_landlock_restrict_self`
```rust
const SYS_landlock_restrict_self: crate::c_long = 446i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:429`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L429)*

### `SYS_memfd_secret`
```rust
const SYS_memfd_secret: crate::c_long = 447i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:430`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L430)*

### `SYS_process_mrelease`
```rust
const SYS_process_mrelease: crate::c_long = 448i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:431`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L431)*

### `SYS_futex_waitv`
```rust
const SYS_futex_waitv: crate::c_long = 449i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:432`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L432)*

### `SYS_set_mempolicy_home_node`
```rust
const SYS_set_mempolicy_home_node: crate::c_long = 450i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:433`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L433)*

### `SYS_fchmodat2`
```rust
const SYS_fchmodat2: crate::c_long = 452i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:434`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L434)*

### `SYS_mseal`
```rust
const SYS_mseal: crate::c_long = 462i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs:435`](../../../../../../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs#L435)*

