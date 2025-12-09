*[libc](../index.md) / [unix](index.md)*

---

# Module `unix`

Definitions found commonly among almost all Unix derivatives

More functions and definitions can be found in the more specific modules
according to the platform in question.

## Contents

- [Modules](#modules)
  - [`linux_like`](#linux_like)
  - [`linux`](#linux)
- [Structs](#structs)
  - [`group`](#group)
  - [`utimbuf`](#utimbuf)
  - [`timeval`](#timeval)
  - [`rlimit`](#rlimit)
  - [`rusage`](#rusage)
  - [`ipv6_mreq`](#ipv6_mreq)
  - [`hostent`](#hostent)
  - [`iovec`](#iovec)
  - [`pollfd`](#pollfd)
  - [`winsize`](#winsize)
  - [`linger`](#linger)
  - [`sigval`](#sigval)
  - [`itimerval`](#itimerval)
  - [`tms`](#tms)
  - [`servent`](#servent)
  - [`protoent`](#protoent)
  - [`in6_addr`](#in6_addr)
  - [`in_addr`](#in_addr)
  - [`ip_mreq`](#ip_mreq)
  - [`ip_mreqn`](#ip_mreqn)
  - [`ip_mreq_source`](#ip_mreq_source)
  - [`sockaddr`](#sockaddr)
  - [`sockaddr_in`](#sockaddr_in)
  - [`sockaddr_in6`](#sockaddr_in6)
  - [`addrinfo`](#addrinfo)
  - [`sockaddr_ll`](#sockaddr_ll)
  - [`fd_set`](#fd_set)
  - [`tm`](#tm)
  - [`sched_param`](#sched_param)
  - [`Dl_info`](#dl_info)
  - [`lconv`](#lconv)
  - [`in_pktinfo`](#in_pktinfo)
  - [`ifaddrs`](#ifaddrs)
  - [`in6_rtmsg`](#in6_rtmsg)
  - [`arpreq`](#arpreq)
  - [`arpreq_old`](#arpreq_old)
  - [`arphdr`](#arphdr)
  - [`mmsghdr`](#mmsghdr)
  - [`sockaddr_un`](#sockaddr_un)
  - [`sockaddr_storage`](#sockaddr_storage)
  - [`utsname`](#utsname)
  - [`file_clone_range`](#file_clone_range)
  - [`sock_filter`](#sock_filter)
  - [`sock_fprog`](#sock_fprog)
  - [`statx`](#statx)
  - [`statx_timestamp`](#statx_timestamp)
  - [`epoll_event`](#epoll_event)
  - [`sigevent`](#sigevent)
- [Enums](#enums)
  - [`DIR`](#dir)
  - [`FILE`](#file)
  - [`timezone`](#timezone)
- [Functions](#functions)
  - [`isalnum`](#isalnum)
  - [`isalpha`](#isalpha)
  - [`iscntrl`](#iscntrl)
  - [`isdigit`](#isdigit)
  - [`isgraph`](#isgraph)
  - [`islower`](#islower)
  - [`isprint`](#isprint)
  - [`ispunct`](#ispunct)
  - [`isspace`](#isspace)
  - [`isupper`](#isupper)
  - [`isxdigit`](#isxdigit)
  - [`isblank`](#isblank)
  - [`tolower`](#tolower)
  - [`toupper`](#toupper)
  - [`qsort`](#qsort)
  - [`bsearch`](#bsearch)
  - [`fopen`](#fopen)
  - [`freopen`](#freopen)
  - [`fflush`](#fflush)
  - [`fclose`](#fclose)
  - [`remove`](#remove)
  - [`rename`](#rename)
  - [`tmpfile`](#tmpfile)
  - [`setvbuf`](#setvbuf)
  - [`setbuf`](#setbuf)
  - [`getchar`](#getchar)
  - [`putchar`](#putchar)
  - [`fgetc`](#fgetc)
  - [`fgets`](#fgets)
  - [`fputc`](#fputc)
  - [`fputs`](#fputs)
  - [`puts`](#puts)
  - [`ungetc`](#ungetc)
  - [`fread`](#fread)
  - [`fwrite`](#fwrite)
  - [`fseek`](#fseek)
  - [`ftell`](#ftell)
  - [`rewind`](#rewind)
  - [`fgetpos`](#fgetpos)
  - [`fsetpos`](#fsetpos)
  - [`feof`](#feof)
  - [`ferror`](#ferror)
  - [`clearerr`](#clearerr)
  - [`perror`](#perror)
  - [`atof`](#atof)
  - [`atoi`](#atoi)
  - [`atol`](#atol)
  - [`atoll`](#atoll)
  - [`strtod`](#strtod)
  - [`strtof`](#strtof)
  - [`strtol`](#strtol)
  - [`strtoll`](#strtoll)
  - [`strtoul`](#strtoul)
  - [`strtoull`](#strtoull)
  - [`calloc`](#calloc)
  - [`malloc`](#malloc)
  - [`realloc`](#realloc)
  - [`free`](#free)
  - [`abort`](#abort)
  - [`exit`](#exit)
  - [`_exit`](#_exit)
  - [`system`](#system)
  - [`getenv`](#getenv)
  - [`strcpy`](#strcpy)
  - [`strncpy`](#strncpy)
  - [`stpcpy`](#stpcpy)
  - [`strcat`](#strcat)
  - [`strncat`](#strncat)
  - [`strcmp`](#strcmp)
  - [`strncmp`](#strncmp)
  - [`strcoll`](#strcoll)
  - [`strchr`](#strchr)
  - [`strrchr`](#strrchr)
  - [`strspn`](#strspn)
  - [`strcspn`](#strcspn)
  - [`strdup`](#strdup)
  - [`strndup`](#strndup)
  - [`strpbrk`](#strpbrk)
  - [`strstr`](#strstr)
  - [`strcasecmp`](#strcasecmp)
  - [`strncasecmp`](#strncasecmp)
  - [`strlen`](#strlen)
  - [`strnlen`](#strnlen)
  - [`strerror`](#strerror)
  - [`strtok`](#strtok)
  - [`strtok_r`](#strtok_r)
  - [`strxfrm`](#strxfrm)
  - [`strsignal`](#strsignal)
  - [`wcslen`](#wcslen)
  - [`wcstombs`](#wcstombs)
  - [`memchr`](#memchr)
  - [`wmemchr`](#wmemchr)
  - [`memcmp`](#memcmp)
  - [`memcpy`](#memcpy)
  - [`memmove`](#memmove)
  - [`memset`](#memset)
  - [`memccpy`](#memccpy)
  - [`getpwnam`](#getpwnam)
  - [`getpwuid`](#getpwuid)
  - [`fprintf`](#fprintf)
  - [`printf`](#printf)
  - [`snprintf`](#snprintf)
  - [`sprintf`](#sprintf)
  - [`fscanf`](#fscanf)
  - [`scanf`](#scanf)
  - [`sscanf`](#sscanf)
  - [`getchar_unlocked`](#getchar_unlocked)
  - [`putchar_unlocked`](#putchar_unlocked)
  - [`socket`](#socket)
  - [`connect`](#connect)
  - [`listen`](#listen)
  - [`accept`](#accept)
  - [`getpeername`](#getpeername)
  - [`getsockname`](#getsockname)
  - [`setsockopt`](#setsockopt)
  - [`socketpair`](#socketpair)
  - [`sendto`](#sendto)
  - [`shutdown`](#shutdown)
  - [`chmod`](#chmod)
  - [`fchmod`](#fchmod)
  - [`fstat`](#fstat)
  - [`mkdir`](#mkdir)
  - [`stat`](#stat)
  - [`pclose`](#pclose)
  - [`fdopen`](#fdopen)
  - [`fileno`](#fileno)
  - [`open`](#open)
  - [`creat`](#creat)
  - [`fcntl`](#fcntl)
  - [`opendir`](#opendir)
  - [`readdir`](#readdir)
  - [`closedir`](#closedir)
  - [`rewinddir`](#rewinddir)
  - [`fchmodat`](#fchmodat)
  - [`fchown`](#fchown)
  - [`fchownat`](#fchownat)
  - [`fstatat`](#fstatat)
  - [`linkat`](#linkat)
  - [`renameat`](#renameat)
  - [`symlinkat`](#symlinkat)
  - [`unlinkat`](#unlinkat)
  - [`access`](#access)
  - [`alarm`](#alarm)
  - [`chdir`](#chdir)
  - [`fchdir`](#fchdir)
  - [`chown`](#chown)
  - [`lchown`](#lchown)
  - [`close`](#close)
  - [`dup`](#dup)
  - [`dup2`](#dup2)
  - [`execl`](#execl)
  - [`execle`](#execle)
  - [`execlp`](#execlp)
  - [`execv`](#execv)
  - [`execve`](#execve)
  - [`execvp`](#execvp)
  - [`fork`](#fork)
  - [`fpathconf`](#fpathconf)
  - [`getcwd`](#getcwd)
  - [`getegid`](#getegid)
  - [`geteuid`](#geteuid)
  - [`getgid`](#getgid)
  - [`getgroups`](#getgroups)
  - [`getlogin`](#getlogin)
  - [`getopt`](#getopt)
  - [`getpgid`](#getpgid)
  - [`getpgrp`](#getpgrp)
  - [`getpid`](#getpid)
  - [`getppid`](#getppid)
  - [`getuid`](#getuid)
  - [`isatty`](#isatty)
  - [`link`](#link)
  - [`lseek`](#lseek)
  - [`pathconf`](#pathconf)
  - [`pipe`](#pipe)
  - [`posix_memalign`](#posix_memalign)
  - [`aligned_alloc`](#aligned_alloc)
  - [`read`](#read)
  - [`rmdir`](#rmdir)
  - [`seteuid`](#seteuid)
  - [`setegid`](#setegid)
  - [`setgid`](#setgid)
  - [`setpgid`](#setpgid)
  - [`setsid`](#setsid)
  - [`setuid`](#setuid)
  - [`setreuid`](#setreuid)
  - [`setregid`](#setregid)
  - [`sleep`](#sleep)
  - [`nanosleep`](#nanosleep)
  - [`tcgetpgrp`](#tcgetpgrp)
  - [`tcsetpgrp`](#tcsetpgrp)
  - [`ttyname`](#ttyname)
  - [`ttyname_r`](#ttyname_r)
  - [`unlink`](#unlink)
  - [`wait`](#wait)
  - [`waitpid`](#waitpid)
  - [`write`](#write)
  - [`pread`](#pread)
  - [`pwrite`](#pwrite)
  - [`umask`](#umask)
  - [`utime`](#utime)
  - [`kill`](#kill)
  - [`killpg`](#killpg)
  - [`mlock`](#mlock)
  - [`munlock`](#munlock)
  - [`mlockall`](#mlockall)
  - [`munlockall`](#munlockall)
  - [`mmap`](#mmap)
  - [`munmap`](#munmap)
  - [`if_nametoindex`](#if_nametoindex)
  - [`if_indextoname`](#if_indextoname)
  - [`lstat`](#lstat)
  - [`fsync`](#fsync)
  - [`setenv`](#setenv)
  - [`unsetenv`](#unsetenv)
  - [`symlink`](#symlink)
  - [`truncate`](#truncate)
  - [`ftruncate`](#ftruncate)
  - [`signal`](#signal)
  - [`getrusage`](#getrusage)
  - [`realpath`](#realpath)
  - [`times`](#times)
  - [`pthread_self`](#pthread_self)
  - [`pthread_equal`](#pthread_equal)
  - [`pthread_join`](#pthread_join)
  - [`pthread_exit`](#pthread_exit)
  - [`pthread_attr_init`](#pthread_attr_init)
  - [`pthread_attr_destroy`](#pthread_attr_destroy)
  - [`pthread_attr_getstacksize`](#pthread_attr_getstacksize)
  - [`pthread_attr_setstacksize`](#pthread_attr_setstacksize)
  - [`pthread_attr_setdetachstate`](#pthread_attr_setdetachstate)
  - [`pthread_detach`](#pthread_detach)
  - [`sched_yield`](#sched_yield)
  - [`pthread_key_create`](#pthread_key_create)
  - [`pthread_key_delete`](#pthread_key_delete)
  - [`pthread_getspecific`](#pthread_getspecific)
  - [`pthread_setspecific`](#pthread_setspecific)
  - [`pthread_mutex_init`](#pthread_mutex_init)
  - [`pthread_mutex_destroy`](#pthread_mutex_destroy)
  - [`pthread_mutex_lock`](#pthread_mutex_lock)
  - [`pthread_mutex_trylock`](#pthread_mutex_trylock)
  - [`pthread_mutex_unlock`](#pthread_mutex_unlock)
  - [`pthread_mutexattr_init`](#pthread_mutexattr_init)
  - [`pthread_mutexattr_destroy`](#pthread_mutexattr_destroy)
  - [`pthread_mutexattr_settype`](#pthread_mutexattr_settype)
  - [`pthread_cond_init`](#pthread_cond_init)
  - [`pthread_cond_wait`](#pthread_cond_wait)
  - [`pthread_cond_timedwait`](#pthread_cond_timedwait)
  - [`pthread_cond_signal`](#pthread_cond_signal)
  - [`pthread_cond_broadcast`](#pthread_cond_broadcast)
  - [`pthread_cond_destroy`](#pthread_cond_destroy)
  - [`pthread_condattr_init`](#pthread_condattr_init)
  - [`pthread_condattr_destroy`](#pthread_condattr_destroy)
  - [`pthread_rwlock_init`](#pthread_rwlock_init)
  - [`pthread_rwlock_destroy`](#pthread_rwlock_destroy)
  - [`pthread_rwlock_rdlock`](#pthread_rwlock_rdlock)
  - [`pthread_rwlock_tryrdlock`](#pthread_rwlock_tryrdlock)
  - [`pthread_rwlock_wrlock`](#pthread_rwlock_wrlock)
  - [`pthread_rwlock_trywrlock`](#pthread_rwlock_trywrlock)
  - [`pthread_rwlock_unlock`](#pthread_rwlock_unlock)
  - [`pthread_rwlockattr_init`](#pthread_rwlockattr_init)
  - [`pthread_rwlockattr_destroy`](#pthread_rwlockattr_destroy)
  - [`getsockopt`](#getsockopt)
  - [`raise`](#raise)
  - [`utimes`](#utimes)
  - [`dlopen`](#dlopen)
  - [`dlerror`](#dlerror)
  - [`dlsym`](#dlsym)
  - [`dlclose`](#dlclose)
  - [`getaddrinfo`](#getaddrinfo)
  - [`freeaddrinfo`](#freeaddrinfo)
  - [`hstrerror`](#hstrerror)
  - [`gai_strerror`](#gai_strerror)
  - [`res_init`](#res_init)
  - [`gmtime_r`](#gmtime_r)
  - [`localtime_r`](#localtime_r)
  - [`mktime`](#mktime)
  - [`time`](#time)
  - [`gmtime`](#gmtime)
  - [`localtime`](#localtime)
  - [`difftime`](#difftime)
  - [`timegm`](#timegm)
  - [`mknod`](#mknod)
  - [`gethostname`](#gethostname)
  - [`endservent`](#endservent)
  - [`getservbyname`](#getservbyname)
  - [`getservbyport`](#getservbyport)
  - [`getservent`](#getservent)
  - [`setservent`](#setservent)
  - [`getprotobyname`](#getprotobyname)
  - [`getprotobynumber`](#getprotobynumber)
  - [`chroot`](#chroot)
  - [`usleep`](#usleep)
  - [`send`](#send)
  - [`recv`](#recv)
  - [`putenv`](#putenv)
  - [`poll`](#poll)
  - [`select`](#select)
  - [`setlocale`](#setlocale)
  - [`localeconv`](#localeconv)
  - [`sem_wait`](#sem_wait)
  - [`sem_trywait`](#sem_trywait)
  - [`sem_post`](#sem_post)
  - [`statvfs`](#statvfs)
  - [`fstatvfs`](#fstatvfs)
  - [`sigemptyset`](#sigemptyset)
  - [`sigaddset`](#sigaddset)
  - [`sigfillset`](#sigfillset)
  - [`sigdelset`](#sigdelset)
  - [`sigismember`](#sigismember)
  - [`sigprocmask`](#sigprocmask)
  - [`sigpending`](#sigpending)
  - [`sysconf`](#sysconf)
  - [`mkfifo`](#mkfifo)
  - [`fseeko`](#fseeko)
  - [`ftello`](#ftello)
  - [`tcdrain`](#tcdrain)
  - [`cfgetispeed`](#cfgetispeed)
  - [`cfgetospeed`](#cfgetospeed)
  - [`cfsetispeed`](#cfsetispeed)
  - [`cfsetospeed`](#cfsetospeed)
  - [`tcgetattr`](#tcgetattr)
  - [`tcsetattr`](#tcsetattr)
  - [`tcflow`](#tcflow)
  - [`tcflush`](#tcflush)
  - [`tcgetsid`](#tcgetsid)
  - [`tcsendbreak`](#tcsendbreak)
  - [`mkstemp`](#mkstemp)
  - [`mkdtemp`](#mkdtemp)
  - [`tmpnam`](#tmpnam)
  - [`openlog`](#openlog)
  - [`closelog`](#closelog)
  - [`setlogmask`](#setlogmask)
  - [`syslog`](#syslog)
  - [`nice`](#nice)
  - [`grantpt`](#grantpt)
  - [`posix_openpt`](#posix_openpt)
  - [`ptsname`](#ptsname)
  - [`unlockpt`](#unlockpt)
  - [`strcasestr`](#strcasestr)
  - [`getline`](#getline)
  - [`lockf`](#lockf)
  - [`adjtime`](#adjtime)
  - [`stpncpy`](#stpncpy)
  - [`sigqueue`](#sigqueue)
  - [`confstr`](#confstr)
  - [`dladdr`](#dladdr)
  - [`flock`](#flock)
  - [`open_wmemstream`](#open_wmemstream)
  - [`getsid`](#getsid)
  - [`pause`](#pause)
  - [`mkdirat`](#mkdirat)
  - [`openat`](#openat)
  - [`fdopendir`](#fdopendir)
  - [`readdir_r`](#readdir_r)
  - [`readlinkat`](#readlinkat)
  - [`fmemopen`](#fmemopen)
  - [`open_memstream`](#open_memstream)
  - [`atexit`](#atexit)
  - [`sigaction`](#sigaction)
  - [`readlink`](#readlink)
  - [`pselect`](#pselect)
  - [`cfmakeraw`](#cfmakeraw)
  - [`cfsetspeed`](#cfsetspeed)
  - [`fnmatch`](#fnmatch)
  - [`htonl`](#htonl)
  - [`htons`](#htons)
  - [`ntohl`](#ntohl)
  - [`ntohs`](#ntohs)
  - [`ioctl`](#ioctl)
  - [`sem_destroy`](#sem_destroy)
  - [`sem_init`](#sem_init)
  - [`fdatasync`](#fdatasync)
  - [`mincore`](#mincore)
  - [`clock_getres`](#clock_getres)
  - [`clock_gettime`](#clock_gettime)
  - [`clock_settime`](#clock_settime)
  - [`clock_getcpuclockid`](#clock_getcpuclockid)
  - [`dirfd`](#dirfd)
  - [`memalign`](#memalign)
  - [`setgroups`](#setgroups)
  - [`pipe2`](#pipe2)
  - [`statfs`](#statfs)
  - [`fstatfs`](#fstatfs)
  - [`memrchr`](#memrchr)
  - [`posix_fadvise`](#posix_fadvise)
  - [`futimens`](#futimens)
  - [`utimensat`](#utimensat)
  - [`duplocale`](#duplocale)
  - [`freelocale`](#freelocale)
  - [`newlocale`](#newlocale)
  - [`uselocale`](#uselocale)
  - [`mknodat`](#mknodat)
  - [`ptsname_r`](#ptsname_r)
  - [`clearenv`](#clearenv)
  - [`waitid`](#waitid)
  - [`getresuid`](#getresuid)
  - [`getresgid`](#getresgid)
  - [`acct`](#acct)
  - [`brk`](#brk)
  - [`sbrk`](#sbrk)
  - [`vfork`](#vfork)
  - [`setresgid`](#setresgid)
  - [`setresuid`](#setresuid)
  - [`wait4`](#wait4)
  - [`login_tty`](#login_tty)
  - [`execvpe`](#execvpe)
  - [`fexecve`](#fexecve)
  - [`getifaddrs`](#getifaddrs)
  - [`freeifaddrs`](#freeifaddrs)
  - [`bind`](#bind)
  - [`writev`](#writev)
  - [`readv`](#readv)
  - [`sendmsg`](#sendmsg)
  - [`recvmsg`](#recvmsg)
  - [`uname`](#uname)
  - [`strchrnul`](#strchrnul)
  - [`strftime`](#strftime)
  - [`strftime_l`](#strftime_l)
  - [`strptime`](#strptime)
  - [`mkostemp`](#mkostemp)
  - [`mkostemps`](#mkostemps)
  - [`getdomainname`](#getdomainname)
  - [`setdomainname`](#setdomainname)
  - [`fstatfs64`](#fstatfs64)
  - [`statvfs64`](#statvfs64)
  - [`fstatvfs64`](#fstatvfs64)
  - [`statfs64`](#statfs64)
  - [`creat64`](#creat64)
  - [`fstat64`](#fstat64)
  - [`fstatat64`](#fstatat64)
  - [`ftruncate64`](#ftruncate64)
  - [`lseek64`](#lseek64)
  - [`lstat64`](#lstat64)
  - [`mmap64`](#mmap64)
  - [`open64`](#open64)
  - [`openat64`](#openat64)
  - [`posix_fadvise64`](#posix_fadvise64)
  - [`pread64`](#pread64)
  - [`pwrite64`](#pwrite64)
  - [`readdir64`](#readdir64)
  - [`readdir64_r`](#readdir64_r)
  - [`stat64`](#stat64)
  - [`truncate64`](#truncate64)
  - [`preadv64`](#preadv64)
  - [`pwritev64`](#pwritev64)
  - [`forkpty`](#forkpty)
  - [`openpty`](#openpty)
  - [`statx`](#statx)
  - [`_IOC`](#_ioc)
  - [`_IO`](#_io)
  - [`_IOR`](#_ior)
  - [`_IOW`](#_iow)
  - [`_IOWR`](#_iowr)
  - [`CMSG_ALIGN`](#cmsg_align)
  - [`CMSG_FIRSTHDR`](#cmsg_firsthdr)
  - [`CMSG_DATA`](#cmsg_data)
  - [`CMSG_SPACE`](#cmsg_space)
  - [`CMSG_LEN`](#cmsg_len)
  - [`FD_CLR`](#fd_clr)
  - [`FD_ISSET`](#fd_isset)
  - [`FD_SET`](#fd_set)
  - [`FD_ZERO`](#fd_zero)
  - [`SIGRTMAX`](#sigrtmax)
  - [`SIGRTMIN`](#sigrtmin)
  - [`WIFSTOPPED`](#wifstopped)
  - [`WSTOPSIG`](#wstopsig)
  - [`WIFCONTINUED`](#wifcontinued)
  - [`WIFSIGNALED`](#wifsignaled)
  - [`WTERMSIG`](#wtermsig)
  - [`WIFEXITED`](#wifexited)
  - [`WEXITSTATUS`](#wexitstatus)
  - [`WCOREDUMP`](#wcoredump)
  - [`W_EXITCODE`](#w_exitcode)
  - [`W_STOPCODE`](#w_stopcode)
  - [`QCMD`](#qcmd)
  - [`IPOPT_COPIED`](#ipopt_copied)
  - [`IPOPT_CLASS`](#ipopt_class)
  - [`IPOPT_NUMBER`](#ipopt_number)
  - [`IPTOS_ECN`](#iptos_ecn)
  - [`KERNEL_VERSION`](#kernel_version)
- [Type Aliases](#type-aliases)
  - [`intmax_t`](#intmax_t)
  - [`uintmax_t`](#uintmax_t)
  - [`size_t`](#size_t)
  - [`ptrdiff_t`](#ptrdiff_t)
  - [`intptr_t`](#intptr_t)
  - [`uintptr_t`](#uintptr_t)
  - [`ssize_t`](#ssize_t)
  - [`pid_t`](#pid_t)
  - [`in_addr_t`](#in_addr_t)
  - [`in_port_t`](#in_port_t)
  - [`sighandler_t`](#sighandler_t)
  - [`cc_t`](#cc_t)
  - [`uid_t`](#uid_t)
  - [`gid_t`](#gid_t)
  - [`locale_t`](#locale_t)
  - [`sa_family_t`](#sa_family_t)
  - [`speed_t`](#speed_t)
  - [`tcflag_t`](#tcflag_t)
  - [`clockid_t`](#clockid_t)
  - [`timer_t`](#timer_t)
  - [`key_t`](#key_t)
  - [`id_t`](#id_t)
- [Constants](#constants)
  - [`INT_MIN`](#int_min)
  - [`INT_MAX`](#int_max)
  - [`SIG_DFL`](#sig_dfl)
  - [`SIG_IGN`](#sig_ign)
  - [`SIG_ERR`](#sig_err)
  - [`DT_UNKNOWN`](#dt_unknown)
  - [`DT_FIFO`](#dt_fifo)
  - [`DT_CHR`](#dt_chr)
  - [`DT_DIR`](#dt_dir)
  - [`DT_BLK`](#dt_blk)
  - [`DT_REG`](#dt_reg)
  - [`DT_LNK`](#dt_lnk)
  - [`DT_SOCK`](#dt_sock)
  - [`FD_CLOEXEC`](#fd_cloexec)
  - [`USRQUOTA`](#usrquota)
  - [`GRPQUOTA`](#grpquota)
  - [`SIGIOT`](#sigiot)
  - [`S_ISUID`](#s_isuid)
  - [`S_ISGID`](#s_isgid)
  - [`S_ISVTX`](#s_isvtx)
  - [`IF_NAMESIZE`](#if_namesize)
  - [`IFNAMSIZ`](#ifnamsiz)
  - [`LOG_EMERG`](#log_emerg)
  - [`LOG_ALERT`](#log_alert)
  - [`LOG_CRIT`](#log_crit)
  - [`LOG_ERR`](#log_err)
  - [`LOG_WARNING`](#log_warning)
  - [`LOG_NOTICE`](#log_notice)
  - [`LOG_INFO`](#log_info)
  - [`LOG_DEBUG`](#log_debug)
  - [`LOG_KERN`](#log_kern)
  - [`LOG_USER`](#log_user)
  - [`LOG_MAIL`](#log_mail)
  - [`LOG_DAEMON`](#log_daemon)
  - [`LOG_AUTH`](#log_auth)
  - [`LOG_SYSLOG`](#log_syslog)
  - [`LOG_LPR`](#log_lpr)
  - [`LOG_NEWS`](#log_news)
  - [`LOG_UUCP`](#log_uucp)
  - [`LOG_LOCAL0`](#log_local0)
  - [`LOG_LOCAL1`](#log_local1)
  - [`LOG_LOCAL2`](#log_local2)
  - [`LOG_LOCAL3`](#log_local3)
  - [`LOG_LOCAL4`](#log_local4)
  - [`LOG_LOCAL5`](#log_local5)
  - [`LOG_LOCAL6`](#log_local6)
  - [`LOG_LOCAL7`](#log_local7)
  - [`LOG_PID`](#log_pid)
  - [`LOG_CONS`](#log_cons)
  - [`LOG_ODELAY`](#log_odelay)
  - [`LOG_NDELAY`](#log_ndelay)
  - [`LOG_NOWAIT`](#log_nowait)
  - [`LOG_PRIMASK`](#log_primask)
  - [`LOG_FACMASK`](#log_facmask)
  - [`PRIO_MIN`](#prio_min)
  - [`PRIO_MAX`](#prio_max)
  - [`IPPROTO_ICMP`](#ipproto_icmp)
  - [`IPPROTO_ICMPV6`](#ipproto_icmpv6)
  - [`IPPROTO_TCP`](#ipproto_tcp)
  - [`IPPROTO_UDP`](#ipproto_udp)
  - [`IPPROTO_IP`](#ipproto_ip)
  - [`IPPROTO_IPV6`](#ipproto_ipv6)
  - [`INADDR_LOOPBACK`](#inaddr_loopback)
  - [`INADDR_ANY`](#inaddr_any)
  - [`INADDR_BROADCAST`](#inaddr_broadcast)
  - [`INADDR_NONE`](#inaddr_none)
  - [`IN6ADDR_LOOPBACK_INIT`](#in6addr_loopback_init)
  - [`IN6ADDR_ANY_INIT`](#in6addr_any_init)
  - [`ARPOP_REQUEST`](#arpop_request)
  - [`ARPOP_REPLY`](#arpop_reply)
  - [`ATF_COM`](#atf_com)
  - [`ATF_PERM`](#atf_perm)
  - [`ATF_PUBL`](#atf_publ)
  - [`ATF_USETRAILERS`](#atf_usetrailers)
  - [`FNM_PERIOD`](#fnm_period)
  - [`FNM_NOMATCH`](#fnm_nomatch)
  - [`FNM_CASEFOLD`](#fnm_casefold)
  - [`FNM_PATHNAME`](#fnm_pathname)
  - [`FNM_NOESCAPE`](#fnm_noescape)
  - [`ULONG_SIZE`](#ulong_size)
  - [`EXIT_FAILURE`](#exit_failure)
  - [`EXIT_SUCCESS`](#exit_success)
  - [`RAND_MAX`](#rand_max)
  - [`EOF`](#eof)
  - [`SEEK_SET`](#seek_set)
  - [`SEEK_CUR`](#seek_cur)
  - [`SEEK_END`](#seek_end)
  - [`_IOFBF`](#_iofbf)
  - [`_IONBF`](#_ionbf)
  - [`_IOLBF`](#_iolbf)
  - [`F_DUPFD`](#f_dupfd)
  - [`F_GETFD`](#f_getfd)
  - [`F_SETFD`](#f_setfd)
  - [`F_GETFL`](#f_getfl)
  - [`F_SETFL`](#f_setfl)
  - [`F_SETLEASE`](#f_setlease)
  - [`F_GETLEASE`](#f_getlease)
  - [`F_NOTIFY`](#f_notify)
  - [`F_CANCELLK`](#f_cancellk)
  - [`F_DUPFD_CLOEXEC`](#f_dupfd_cloexec)
  - [`F_SETPIPE_SZ`](#f_setpipe_sz)
  - [`F_GETPIPE_SZ`](#f_getpipe_sz)
  - [`F_ADD_SEALS`](#f_add_seals)
  - [`F_GET_SEALS`](#f_get_seals)
  - [`F_SEAL_SEAL`](#f_seal_seal)
  - [`F_SEAL_SHRINK`](#f_seal_shrink)
  - [`F_SEAL_GROW`](#f_seal_grow)
  - [`F_SEAL_WRITE`](#f_seal_write)
  - [`SIGTRAP`](#sigtrap)
  - [`PTHREAD_CREATE_JOINABLE`](#pthread_create_joinable)
  - [`PTHREAD_CREATE_DETACHED`](#pthread_create_detached)
  - [`CLOCK_REALTIME`](#clock_realtime)
  - [`CLOCK_MONOTONIC`](#clock_monotonic)
  - [`CLOCK_PROCESS_CPUTIME_ID`](#clock_process_cputime_id)
  - [`CLOCK_THREAD_CPUTIME_ID`](#clock_thread_cputime_id)
  - [`CLOCK_MONOTONIC_RAW`](#clock_monotonic_raw)
  - [`CLOCK_REALTIME_COARSE`](#clock_realtime_coarse)
  - [`CLOCK_MONOTONIC_COARSE`](#clock_monotonic_coarse)
  - [`CLOCK_BOOTTIME`](#clock_boottime)
  - [`CLOCK_REALTIME_ALARM`](#clock_realtime_alarm)
  - [`CLOCK_BOOTTIME_ALARM`](#clock_boottime_alarm)
  - [`CLOCK_TAI`](#clock_tai)
  - [`TIMER_ABSTIME`](#timer_abstime)
  - [`RUSAGE_SELF`](#rusage_self)
  - [`O_RDONLY`](#o_rdonly)
  - [`O_WRONLY`](#o_wronly)
  - [`O_RDWR`](#o_rdwr)
  - [`SOCK_CLOEXEC`](#sock_cloexec)
  - [`S_IFIFO`](#s_ififo)
  - [`S_IFCHR`](#s_ifchr)
  - [`S_IFBLK`](#s_ifblk)
  - [`S_IFDIR`](#s_ifdir)
  - [`S_IFREG`](#s_ifreg)
  - [`S_IFLNK`](#s_iflnk)
  - [`S_IFSOCK`](#s_ifsock)
  - [`S_IFMT`](#s_ifmt)
  - [`S_IRWXU`](#s_irwxu)
  - [`S_IXUSR`](#s_ixusr)
  - [`S_IWUSR`](#s_iwusr)
  - [`S_IRUSR`](#s_irusr)
  - [`S_IRWXG`](#s_irwxg)
  - [`S_IXGRP`](#s_ixgrp)
  - [`S_IWGRP`](#s_iwgrp)
  - [`S_IRGRP`](#s_irgrp)
  - [`S_IRWXO`](#s_irwxo)
  - [`S_IXOTH`](#s_ixoth)
  - [`S_IWOTH`](#s_iwoth)
  - [`S_IROTH`](#s_iroth)
  - [`F_OK`](#f_ok)
  - [`R_OK`](#r_ok)
  - [`W_OK`](#w_ok)
  - [`X_OK`](#x_ok)
  - [`SIGHUP`](#sighup)
  - [`SIGINT`](#sigint)
  - [`SIGQUIT`](#sigquit)
  - [`SIGILL`](#sigill)
  - [`SIGABRT`](#sigabrt)
  - [`SIGFPE`](#sigfpe)
  - [`SIGKILL`](#sigkill)
  - [`SIGSEGV`](#sigsegv)
  - [`SIGPIPE`](#sigpipe)
  - [`SIGALRM`](#sigalrm)
  - [`SIGTERM`](#sigterm)
  - [`PROT_NONE`](#prot_none)
  - [`PROT_READ`](#prot_read)
  - [`PROT_WRITE`](#prot_write)
  - [`PROT_EXEC`](#prot_exec)
  - [`XATTR_CREATE`](#xattr_create)
  - [`XATTR_REPLACE`](#xattr_replace)
  - [`RLIM64_INFINITY`](#rlim64_infinity)
  - [`LC_CTYPE`](#lc_ctype)
  - [`LC_NUMERIC`](#lc_numeric)
  - [`LC_TIME`](#lc_time)
  - [`LC_COLLATE`](#lc_collate)
  - [`LC_MONETARY`](#lc_monetary)
  - [`LC_MESSAGES`](#lc_messages)
  - [`LC_ALL`](#lc_all)
  - [`LC_CTYPE_MASK`](#lc_ctype_mask)
  - [`LC_NUMERIC_MASK`](#lc_numeric_mask)
  - [`LC_TIME_MASK`](#lc_time_mask)
  - [`LC_COLLATE_MASK`](#lc_collate_mask)
  - [`LC_MONETARY_MASK`](#lc_monetary_mask)
  - [`LC_MESSAGES_MASK`](#lc_messages_mask)
  - [`MAP_FILE`](#map_file)
  - [`MAP_SHARED`](#map_shared)
  - [`MAP_PRIVATE`](#map_private)
  - [`MAP_FIXED`](#map_fixed)
  - [`MAP_FAILED`](#map_failed)
  - [`MS_ASYNC`](#ms_async)
  - [`MS_INVALIDATE`](#ms_invalidate)
  - [`MS_SYNC`](#ms_sync)
  - [`MS_RDONLY`](#ms_rdonly)
  - [`MS_NOSUID`](#ms_nosuid)
  - [`MS_NODEV`](#ms_nodev)
  - [`MS_NOEXEC`](#ms_noexec)
  - [`MS_SYNCHRONOUS`](#ms_synchronous)
  - [`MS_REMOUNT`](#ms_remount)
  - [`MS_MANDLOCK`](#ms_mandlock)
  - [`MS_DIRSYNC`](#ms_dirsync)
  - [`MS_NOSYMFOLLOW`](#ms_nosymfollow)
  - [`MS_NOATIME`](#ms_noatime)
  - [`MS_NODIRATIME`](#ms_nodiratime)
  - [`MS_BIND`](#ms_bind)
  - [`MS_MOVE`](#ms_move)
  - [`MS_REC`](#ms_rec)
  - [`MS_SILENT`](#ms_silent)
  - [`MS_POSIXACL`](#ms_posixacl)
  - [`MS_UNBINDABLE`](#ms_unbindable)
  - [`MS_PRIVATE`](#ms_private)
  - [`MS_SLAVE`](#ms_slave)
  - [`MS_SHARED`](#ms_shared)
  - [`MS_RELATIME`](#ms_relatime)
  - [`MS_KERNMOUNT`](#ms_kernmount)
  - [`MS_I_VERSION`](#ms_i_version)
  - [`MS_STRICTATIME`](#ms_strictatime)
  - [`MS_LAZYTIME`](#ms_lazytime)
  - [`MS_ACTIVE`](#ms_active)
  - [`MS_MGC_VAL`](#ms_mgc_val)
  - [`MS_MGC_MSK`](#ms_mgc_msk)
  - [`SCM_RIGHTS`](#scm_rights)
  - [`SCM_CREDENTIALS`](#scm_credentials)
  - [`PROT_GROWSDOWN`](#prot_growsdown)
  - [`PROT_GROWSUP`](#prot_growsup)
  - [`MAP_TYPE`](#map_type)
  - [`MADV_NORMAL`](#madv_normal)
  - [`MADV_RANDOM`](#madv_random)
  - [`MADV_SEQUENTIAL`](#madv_sequential)
  - [`MADV_WILLNEED`](#madv_willneed)
  - [`MADV_DONTNEED`](#madv_dontneed)
  - [`MADV_FREE`](#madv_free)
  - [`MADV_REMOVE`](#madv_remove)
  - [`MADV_DONTFORK`](#madv_dontfork)
  - [`MADV_DOFORK`](#madv_dofork)
  - [`MADV_MERGEABLE`](#madv_mergeable)
  - [`MADV_UNMERGEABLE`](#madv_unmergeable)
  - [`MADV_HUGEPAGE`](#madv_hugepage)
  - [`MADV_NOHUGEPAGE`](#madv_nohugepage)
  - [`MADV_DONTDUMP`](#madv_dontdump)
  - [`MADV_DODUMP`](#madv_dodump)
  - [`MADV_WIPEONFORK`](#madv_wipeonfork)
  - [`MADV_KEEPONFORK`](#madv_keeponfork)
  - [`MADV_COLD`](#madv_cold)
  - [`MADV_PAGEOUT`](#madv_pageout)
  - [`MADV_HWPOISON`](#madv_hwpoison)
  - [`MADV_POPULATE_READ`](#madv_populate_read)
  - [`MADV_POPULATE_WRITE`](#madv_populate_write)
  - [`MADV_DONTNEED_LOCKED`](#madv_dontneed_locked)
  - [`IFF_UP`](#iff_up)
  - [`IFF_BROADCAST`](#iff_broadcast)
  - [`IFF_DEBUG`](#iff_debug)
  - [`IFF_LOOPBACK`](#iff_loopback)
  - [`IFF_POINTOPOINT`](#iff_pointopoint)
  - [`IFF_NOTRAILERS`](#iff_notrailers)
  - [`IFF_RUNNING`](#iff_running)
  - [`IFF_NOARP`](#iff_noarp)
  - [`IFF_PROMISC`](#iff_promisc)
  - [`IFF_ALLMULTI`](#iff_allmulti)
  - [`IFF_MASTER`](#iff_master)
  - [`IFF_SLAVE`](#iff_slave)
  - [`IFF_MULTICAST`](#iff_multicast)
  - [`IFF_PORTSEL`](#iff_portsel)
  - [`IFF_AUTOMEDIA`](#iff_automedia)
  - [`IFF_DYNAMIC`](#iff_dynamic)
  - [`SOL_IP`](#sol_ip)
  - [`SOL_TCP`](#sol_tcp)
  - [`SOL_UDP`](#sol_udp)
  - [`SOL_IPV6`](#sol_ipv6)
  - [`SOL_ICMPV6`](#sol_icmpv6)
  - [`SOL_RAW`](#sol_raw)
  - [`SOL_DECNET`](#sol_decnet)
  - [`SOL_X25`](#sol_x25)
  - [`SOL_PACKET`](#sol_packet)
  - [`SOL_ATM`](#sol_atm)
  - [`SOL_AAL`](#sol_aal)
  - [`SOL_IRDA`](#sol_irda)
  - [`SOL_NETBEUI`](#sol_netbeui)
  - [`SOL_LLC`](#sol_llc)
  - [`SOL_DCCP`](#sol_dccp)
  - [`SOL_NETLINK`](#sol_netlink)
  - [`SOL_TIPC`](#sol_tipc)
  - [`SOL_BLUETOOTH`](#sol_bluetooth)
  - [`SOL_ALG`](#sol_alg)
  - [`AF_UNSPEC`](#af_unspec)
  - [`AF_UNIX`](#af_unix)
  - [`AF_LOCAL`](#af_local)
  - [`AF_INET`](#af_inet)
  - [`AF_AX25`](#af_ax25)
  - [`AF_IPX`](#af_ipx)
  - [`AF_APPLETALK`](#af_appletalk)
  - [`AF_NETROM`](#af_netrom)
  - [`AF_BRIDGE`](#af_bridge)
  - [`AF_ATMPVC`](#af_atmpvc)
  - [`AF_X25`](#af_x25)
  - [`AF_INET6`](#af_inet6)
  - [`AF_ROSE`](#af_rose)
  - [`AF_DECnet`](#af_decnet)
  - [`AF_NETBEUI`](#af_netbeui)
  - [`AF_SECURITY`](#af_security)
  - [`AF_KEY`](#af_key)
  - [`AF_NETLINK`](#af_netlink)
  - [`AF_ROUTE`](#af_route)
  - [`AF_PACKET`](#af_packet)
  - [`AF_ASH`](#af_ash)
  - [`AF_ECONET`](#af_econet)
  - [`AF_ATMSVC`](#af_atmsvc)
  - [`AF_RDS`](#af_rds)
  - [`AF_SNA`](#af_sna)
  - [`AF_IRDA`](#af_irda)
  - [`AF_PPPOX`](#af_pppox)
  - [`AF_WANPIPE`](#af_wanpipe)
  - [`AF_LLC`](#af_llc)
  - [`AF_CAN`](#af_can)
  - [`AF_TIPC`](#af_tipc)
  - [`AF_BLUETOOTH`](#af_bluetooth)
  - [`AF_IUCV`](#af_iucv)
  - [`AF_RXRPC`](#af_rxrpc)
  - [`AF_ISDN`](#af_isdn)
  - [`AF_PHONET`](#af_phonet)
  - [`AF_IEEE802154`](#af_ieee802154)
  - [`AF_CAIF`](#af_caif)
  - [`AF_ALG`](#af_alg)
  - [`PF_UNSPEC`](#pf_unspec)
  - [`PF_UNIX`](#pf_unix)
  - [`PF_LOCAL`](#pf_local)
  - [`PF_INET`](#pf_inet)
  - [`PF_AX25`](#pf_ax25)
  - [`PF_IPX`](#pf_ipx)
  - [`PF_APPLETALK`](#pf_appletalk)
  - [`PF_NETROM`](#pf_netrom)
  - [`PF_BRIDGE`](#pf_bridge)
  - [`PF_ATMPVC`](#pf_atmpvc)
  - [`PF_X25`](#pf_x25)
  - [`PF_INET6`](#pf_inet6)
  - [`PF_ROSE`](#pf_rose)
  - [`PF_DECnet`](#pf_decnet)
  - [`PF_NETBEUI`](#pf_netbeui)
  - [`PF_SECURITY`](#pf_security)
  - [`PF_KEY`](#pf_key)
  - [`PF_NETLINK`](#pf_netlink)
  - [`PF_ROUTE`](#pf_route)
  - [`PF_PACKET`](#pf_packet)
  - [`PF_ASH`](#pf_ash)
  - [`PF_ECONET`](#pf_econet)
  - [`PF_ATMSVC`](#pf_atmsvc)
  - [`PF_RDS`](#pf_rds)
  - [`PF_SNA`](#pf_sna)
  - [`PF_IRDA`](#pf_irda)
  - [`PF_PPPOX`](#pf_pppox)
  - [`PF_WANPIPE`](#pf_wanpipe)
  - [`PF_LLC`](#pf_llc)
  - [`PF_CAN`](#pf_can)
  - [`PF_TIPC`](#pf_tipc)
  - [`PF_BLUETOOTH`](#pf_bluetooth)
  - [`PF_IUCV`](#pf_iucv)
  - [`PF_RXRPC`](#pf_rxrpc)
  - [`PF_ISDN`](#pf_isdn)
  - [`PF_PHONET`](#pf_phonet)
  - [`PF_IEEE802154`](#pf_ieee802154)
  - [`PF_CAIF`](#pf_caif)
  - [`PF_ALG`](#pf_alg)
  - [`MSG_OOB`](#msg_oob)
  - [`MSG_PEEK`](#msg_peek)
  - [`MSG_DONTROUTE`](#msg_dontroute)
  - [`MSG_CTRUNC`](#msg_ctrunc)
  - [`MSG_TRUNC`](#msg_trunc)
  - [`MSG_DONTWAIT`](#msg_dontwait)
  - [`MSG_EOR`](#msg_eor)
  - [`MSG_WAITALL`](#msg_waitall)
  - [`MSG_FIN`](#msg_fin)
  - [`MSG_SYN`](#msg_syn)
  - [`MSG_CONFIRM`](#msg_confirm)
  - [`MSG_RST`](#msg_rst)
  - [`MSG_ERRQUEUE`](#msg_errqueue)
  - [`MSG_NOSIGNAL`](#msg_nosignal)
  - [`MSG_MORE`](#msg_more)
  - [`MSG_WAITFORONE`](#msg_waitforone)
  - [`MSG_FASTOPEN`](#msg_fastopen)
  - [`MSG_CMSG_CLOEXEC`](#msg_cmsg_cloexec)
  - [`SCM_TIMESTAMP`](#scm_timestamp)
  - [`SOCK_RAW`](#sock_raw)
  - [`SOCK_RDM`](#sock_rdm)
  - [`IP_TOS`](#ip_tos)
  - [`IP_TTL`](#ip_ttl)
  - [`IP_HDRINCL`](#ip_hdrincl)
  - [`IP_OPTIONS`](#ip_options)
  - [`IP_ROUTER_ALERT`](#ip_router_alert)
  - [`IP_RECVOPTS`](#ip_recvopts)
  - [`IP_RETOPTS`](#ip_retopts)
  - [`IP_PKTINFO`](#ip_pktinfo)
  - [`IP_PKTOPTIONS`](#ip_pktoptions)
  - [`IP_MTU_DISCOVER`](#ip_mtu_discover)
  - [`IP_RECVERR`](#ip_recverr)
  - [`IP_RECVTTL`](#ip_recvttl)
  - [`IP_RECVTOS`](#ip_recvtos)
  - [`IP_MTU`](#ip_mtu)
  - [`IP_FREEBIND`](#ip_freebind)
  - [`IP_IPSEC_POLICY`](#ip_ipsec_policy)
  - [`IP_XFRM_POLICY`](#ip_xfrm_policy)
  - [`IP_PASSSEC`](#ip_passsec)
  - [`IP_TRANSPARENT`](#ip_transparent)
  - [`IP_ORIGDSTADDR`](#ip_origdstaddr)
  - [`IP_RECVORIGDSTADDR`](#ip_recvorigdstaddr)
  - [`IP_MINTTL`](#ip_minttl)
  - [`IP_NODEFRAG`](#ip_nodefrag)
  - [`IP_CHECKSUM`](#ip_checksum)
  - [`IP_BIND_ADDRESS_NO_PORT`](#ip_bind_address_no_port)
  - [`IP_MULTICAST_IF`](#ip_multicast_if)
  - [`IP_MULTICAST_TTL`](#ip_multicast_ttl)
  - [`IP_MULTICAST_LOOP`](#ip_multicast_loop)
  - [`IP_ADD_MEMBERSHIP`](#ip_add_membership)
  - [`IP_DROP_MEMBERSHIP`](#ip_drop_membership)
  - [`IP_UNBLOCK_SOURCE`](#ip_unblock_source)
  - [`IP_BLOCK_SOURCE`](#ip_block_source)
  - [`IP_ADD_SOURCE_MEMBERSHIP`](#ip_add_source_membership)
  - [`IP_DROP_SOURCE_MEMBERSHIP`](#ip_drop_source_membership)
  - [`IP_MSFILTER`](#ip_msfilter)
  - [`IP_MULTICAST_ALL`](#ip_multicast_all)
  - [`IP_UNICAST_IF`](#ip_unicast_if)
  - [`IP_DEFAULT_MULTICAST_TTL`](#ip_default_multicast_ttl)
  - [`IP_DEFAULT_MULTICAST_LOOP`](#ip_default_multicast_loop)
  - [`IP_PMTUDISC_DONT`](#ip_pmtudisc_dont)
  - [`IP_PMTUDISC_WANT`](#ip_pmtudisc_want)
  - [`IP_PMTUDISC_DO`](#ip_pmtudisc_do)
  - [`IP_PMTUDISC_PROBE`](#ip_pmtudisc_probe)
  - [`IP_PMTUDISC_INTERFACE`](#ip_pmtudisc_interface)
  - [`IP_PMTUDISC_OMIT`](#ip_pmtudisc_omit)
  - [`IPPROTO_HOPOPTS`](#ipproto_hopopts)
  - [`IPPROTO_IGMP`](#ipproto_igmp)
  - [`IPPROTO_IPIP`](#ipproto_ipip)
  - [`IPPROTO_EGP`](#ipproto_egp)
  - [`IPPROTO_PUP`](#ipproto_pup)
  - [`IPPROTO_IDP`](#ipproto_idp)
  - [`IPPROTO_TP`](#ipproto_tp)
  - [`IPPROTO_DCCP`](#ipproto_dccp)
  - [`IPPROTO_ROUTING`](#ipproto_routing)
  - [`IPPROTO_FRAGMENT`](#ipproto_fragment)
  - [`IPPROTO_RSVP`](#ipproto_rsvp)
  - [`IPPROTO_GRE`](#ipproto_gre)
  - [`IPPROTO_ESP`](#ipproto_esp)
  - [`IPPROTO_AH`](#ipproto_ah)
  - [`IPPROTO_NONE`](#ipproto_none)
  - [`IPPROTO_DSTOPTS`](#ipproto_dstopts)
  - [`IPPROTO_MTP`](#ipproto_mtp)
  - [`IPPROTO_ENCAP`](#ipproto_encap)
  - [`IPPROTO_PIM`](#ipproto_pim)
  - [`IPPROTO_COMP`](#ipproto_comp)
  - [`IPPROTO_SCTP`](#ipproto_sctp)
  - [`IPPROTO_MH`](#ipproto_mh)
  - [`IPPROTO_UDPLITE`](#ipproto_udplite)
  - [`IPPROTO_RAW`](#ipproto_raw)
  - [`IPPROTO_BEETPH`](#ipproto_beetph)
  - [`IPPROTO_MPLS`](#ipproto_mpls)
  - [`IPPROTO_MPTCP`](#ipproto_mptcp)
  - [`IPPROTO_ETHERNET`](#ipproto_ethernet)
  - [`MCAST_EXCLUDE`](#mcast_exclude)
  - [`MCAST_INCLUDE`](#mcast_include)
  - [`MCAST_JOIN_GROUP`](#mcast_join_group)
  - [`MCAST_BLOCK_SOURCE`](#mcast_block_source)
  - [`MCAST_UNBLOCK_SOURCE`](#mcast_unblock_source)
  - [`MCAST_LEAVE_GROUP`](#mcast_leave_group)
  - [`MCAST_JOIN_SOURCE_GROUP`](#mcast_join_source_group)
  - [`MCAST_LEAVE_SOURCE_GROUP`](#mcast_leave_source_group)
  - [`MCAST_MSFILTER`](#mcast_msfilter)
  - [`IPV6_ADDRFORM`](#ipv6_addrform)
  - [`IPV6_2292PKTINFO`](#ipv6_2292pktinfo)
  - [`IPV6_2292HOPOPTS`](#ipv6_2292hopopts)
  - [`IPV6_2292DSTOPTS`](#ipv6_2292dstopts)
  - [`IPV6_2292RTHDR`](#ipv6_2292rthdr)
  - [`IPV6_2292PKTOPTIONS`](#ipv6_2292pktoptions)
  - [`IPV6_CHECKSUM`](#ipv6_checksum)
  - [`IPV6_2292HOPLIMIT`](#ipv6_2292hoplimit)
  - [`IPV6_NEXTHOP`](#ipv6_nexthop)
  - [`IPV6_AUTHHDR`](#ipv6_authhdr)
  - [`IPV6_UNICAST_HOPS`](#ipv6_unicast_hops)
  - [`IPV6_MULTICAST_IF`](#ipv6_multicast_if)
  - [`IPV6_MULTICAST_HOPS`](#ipv6_multicast_hops)
  - [`IPV6_MULTICAST_LOOP`](#ipv6_multicast_loop)
  - [`IPV6_ADD_MEMBERSHIP`](#ipv6_add_membership)
  - [`IPV6_DROP_MEMBERSHIP`](#ipv6_drop_membership)
  - [`IPV6_ROUTER_ALERT`](#ipv6_router_alert)
  - [`IPV6_MTU_DISCOVER`](#ipv6_mtu_discover)
  - [`IPV6_MTU`](#ipv6_mtu)
  - [`IPV6_RECVERR`](#ipv6_recverr)
  - [`IPV6_V6ONLY`](#ipv6_v6only)
  - [`IPV6_JOIN_ANYCAST`](#ipv6_join_anycast)
  - [`IPV6_LEAVE_ANYCAST`](#ipv6_leave_anycast)
  - [`IPV6_IPSEC_POLICY`](#ipv6_ipsec_policy)
  - [`IPV6_XFRM_POLICY`](#ipv6_xfrm_policy)
  - [`IPV6_HDRINCL`](#ipv6_hdrincl)
  - [`IPV6_RECVPKTINFO`](#ipv6_recvpktinfo)
  - [`IPV6_PKTINFO`](#ipv6_pktinfo)
  - [`IPV6_RECVHOPLIMIT`](#ipv6_recvhoplimit)
  - [`IPV6_HOPLIMIT`](#ipv6_hoplimit)
  - [`IPV6_RECVHOPOPTS`](#ipv6_recvhopopts)
  - [`IPV6_HOPOPTS`](#ipv6_hopopts)
  - [`IPV6_RTHDRDSTOPTS`](#ipv6_rthdrdstopts)
  - [`IPV6_RECVRTHDR`](#ipv6_recvrthdr)
  - [`IPV6_RTHDR`](#ipv6_rthdr)
  - [`IPV6_RECVDSTOPTS`](#ipv6_recvdstopts)
  - [`IPV6_DSTOPTS`](#ipv6_dstopts)
  - [`IPV6_RECVPATHMTU`](#ipv6_recvpathmtu)
  - [`IPV6_PATHMTU`](#ipv6_pathmtu)
  - [`IPV6_DONTFRAG`](#ipv6_dontfrag)
  - [`IPV6_RECVTCLASS`](#ipv6_recvtclass)
  - [`IPV6_TCLASS`](#ipv6_tclass)
  - [`IPV6_AUTOFLOWLABEL`](#ipv6_autoflowlabel)
  - [`IPV6_ADDR_PREFERENCES`](#ipv6_addr_preferences)
  - [`IPV6_MINHOPCOUNT`](#ipv6_minhopcount)
  - [`IPV6_ORIGDSTADDR`](#ipv6_origdstaddr)
  - [`IPV6_RECVORIGDSTADDR`](#ipv6_recvorigdstaddr)
  - [`IPV6_TRANSPARENT`](#ipv6_transparent)
  - [`IPV6_UNICAST_IF`](#ipv6_unicast_if)
  - [`IPV6_PREFER_SRC_TMP`](#ipv6_prefer_src_tmp)
  - [`IPV6_PREFER_SRC_PUBLIC`](#ipv6_prefer_src_public)
  - [`IPV6_PREFER_SRC_PUBTMP_DEFAULT`](#ipv6_prefer_src_pubtmp_default)
  - [`IPV6_PREFER_SRC_COA`](#ipv6_prefer_src_coa)
  - [`IPV6_PREFER_SRC_HOME`](#ipv6_prefer_src_home)
  - [`IPV6_PREFER_SRC_CGA`](#ipv6_prefer_src_cga)
  - [`IPV6_PREFER_SRC_NONCGA`](#ipv6_prefer_src_noncga)
  - [`IPV6_PMTUDISC_DONT`](#ipv6_pmtudisc_dont)
  - [`IPV6_PMTUDISC_WANT`](#ipv6_pmtudisc_want)
  - [`IPV6_PMTUDISC_DO`](#ipv6_pmtudisc_do)
  - [`IPV6_PMTUDISC_PROBE`](#ipv6_pmtudisc_probe)
  - [`IPV6_PMTUDISC_INTERFACE`](#ipv6_pmtudisc_interface)
  - [`IPV6_PMTUDISC_OMIT`](#ipv6_pmtudisc_omit)
  - [`TCP_NODELAY`](#tcp_nodelay)
  - [`TCP_MAXSEG`](#tcp_maxseg)
  - [`TCP_CORK`](#tcp_cork)
  - [`TCP_KEEPIDLE`](#tcp_keepidle)
  - [`TCP_KEEPINTVL`](#tcp_keepintvl)
  - [`TCP_KEEPCNT`](#tcp_keepcnt)
  - [`TCP_SYNCNT`](#tcp_syncnt)
  - [`TCP_LINGER2`](#tcp_linger2)
  - [`TCP_DEFER_ACCEPT`](#tcp_defer_accept)
  - [`TCP_WINDOW_CLAMP`](#tcp_window_clamp)
  - [`TCP_INFO`](#tcp_info)
  - [`TCP_QUICKACK`](#tcp_quickack)
  - [`TCP_CONGESTION`](#tcp_congestion)
  - [`TCP_MD5SIG`](#tcp_md5sig)
  - [`TCP_COOKIE_TRANSACTIONS`](#tcp_cookie_transactions)
  - [`TCP_THIN_LINEAR_TIMEOUTS`](#tcp_thin_linear_timeouts)
  - [`TCP_THIN_DUPACK`](#tcp_thin_dupack)
  - [`TCP_USER_TIMEOUT`](#tcp_user_timeout)
  - [`TCP_REPAIR`](#tcp_repair)
  - [`TCP_REPAIR_QUEUE`](#tcp_repair_queue)
  - [`TCP_QUEUE_SEQ`](#tcp_queue_seq)
  - [`TCP_REPAIR_OPTIONS`](#tcp_repair_options)
  - [`TCP_FASTOPEN`](#tcp_fastopen)
  - [`TCP_TIMESTAMP`](#tcp_timestamp)
  - [`TCP_NOTSENT_LOWAT`](#tcp_notsent_lowat)
  - [`TCP_CC_INFO`](#tcp_cc_info)
  - [`TCP_SAVE_SYN`](#tcp_save_syn)
  - [`TCP_SAVED_SYN`](#tcp_saved_syn)
  - [`TCP_REPAIR_WINDOW`](#tcp_repair_window)
  - [`TCP_FASTOPEN_CONNECT`](#tcp_fastopen_connect)
  - [`TCP_ULP`](#tcp_ulp)
  - [`TCP_MD5SIG_EXT`](#tcp_md5sig_ext)
  - [`TCP_FASTOPEN_KEY`](#tcp_fastopen_key)
  - [`TCP_FASTOPEN_NO_COOKIE`](#tcp_fastopen_no_cookie)
  - [`TCP_ZEROCOPY_RECEIVE`](#tcp_zerocopy_receive)
  - [`TCP_INQ`](#tcp_inq)
  - [`TCP_CM_INQ`](#tcp_cm_inq)
  - [`TCP_MD5SIG_MAXKEYLEN`](#tcp_md5sig_maxkeylen)
  - [`SO_DEBUG`](#so_debug)
  - [`SHUT_RD`](#shut_rd)
  - [`SHUT_WR`](#shut_wr)
  - [`SHUT_RDWR`](#shut_rdwr)
  - [`LOCK_SH`](#lock_sh)
  - [`LOCK_EX`](#lock_ex)
  - [`LOCK_NB`](#lock_nb)
  - [`LOCK_UN`](#lock_un)
  - [`SS_ONSTACK`](#ss_onstack)
  - [`SS_DISABLE`](#ss_disable)
  - [`PATH_MAX`](#path_max)
  - [`UIO_MAXIOV`](#uio_maxiov)
  - [`FD_SETSIZE`](#fd_setsize)
  - [`EPOLLIN`](#epollin)
  - [`EPOLLPRI`](#epollpri)
  - [`EPOLLOUT`](#epollout)
  - [`EPOLLERR`](#epollerr)
  - [`EPOLLHUP`](#epollhup)
  - [`EPOLLRDNORM`](#epollrdnorm)
  - [`EPOLLRDBAND`](#epollrdband)
  - [`EPOLLWRNORM`](#epollwrnorm)
  - [`EPOLLWRBAND`](#epollwrband)
  - [`EPOLLMSG`](#epollmsg)
  - [`EPOLLRDHUP`](#epollrdhup)
  - [`EPOLLEXCLUSIVE`](#epollexclusive)
  - [`EPOLLWAKEUP`](#epollwakeup)
  - [`EPOLLONESHOT`](#epolloneshot)
  - [`EPOLLET`](#epollet)
  - [`EPOLL_CTL_ADD`](#epoll_ctl_add)
  - [`EPOLL_CTL_MOD`](#epoll_ctl_mod)
  - [`EPOLL_CTL_DEL`](#epoll_ctl_del)
  - [`MNT_FORCE`](#mnt_force)
  - [`MNT_DETACH`](#mnt_detach)
  - [`MNT_EXPIRE`](#mnt_expire)
  - [`UMOUNT_NOFOLLOW`](#umount_nofollow)
  - [`Q_GETFMT`](#q_getfmt)
  - [`Q_GETINFO`](#q_getinfo)
  - [`Q_SETINFO`](#q_setinfo)
  - [`QIF_BLIMITS`](#qif_blimits)
  - [`QIF_SPACE`](#qif_space)
  - [`QIF_ILIMITS`](#qif_ilimits)
  - [`QIF_INODES`](#qif_inodes)
  - [`QIF_BTIME`](#qif_btime)
  - [`QIF_ITIME`](#qif_itime)
  - [`QIF_LIMITS`](#qif_limits)
  - [`QIF_USAGE`](#qif_usage)
  - [`QIF_TIMES`](#qif_times)
  - [`QIF_ALL`](#qif_all)
  - [`Q_SYNC`](#q_sync)
  - [`Q_QUOTAON`](#q_quotaon)
  - [`Q_QUOTAOFF`](#q_quotaoff)
  - [`Q_GETQUOTA`](#q_getquota)
  - [`Q_SETQUOTA`](#q_setquota)
  - [`TCIOFF`](#tcioff)
  - [`TCION`](#tcion)
  - [`TCOOFF`](#tcooff)
  - [`TCOON`](#tcoon)
  - [`TCIFLUSH`](#tciflush)
  - [`TCOFLUSH`](#tcoflush)
  - [`TCIOFLUSH`](#tcioflush)
  - [`NL0`](#nl0)
  - [`NL1`](#nl1)
  - [`TAB0`](#tab0)
  - [`CR0`](#cr0)
  - [`FF0`](#ff0)
  - [`BS0`](#bs0)
  - [`VT0`](#vt0)
  - [`VERASE`](#verase)
  - [`VKILL`](#vkill)
  - [`VINTR`](#vintr)
  - [`VQUIT`](#vquit)
  - [`VLNEXT`](#vlnext)
  - [`IGNBRK`](#ignbrk)
  - [`BRKINT`](#brkint)
  - [`IGNPAR`](#ignpar)
  - [`PARMRK`](#parmrk)
  - [`INPCK`](#inpck)
  - [`ISTRIP`](#istrip)
  - [`INLCR`](#inlcr)
  - [`IGNCR`](#igncr)
  - [`ICRNL`](#icrnl)
  - [`IXANY`](#ixany)
  - [`IMAXBEL`](#imaxbel)
  - [`OPOST`](#opost)
  - [`CS5`](#cs5)
  - [`CRTSCTS`](#crtscts)
  - [`ECHO`](#echo)
  - [`OCRNL`](#ocrnl)
  - [`ONOCR`](#onocr)
  - [`ONLRET`](#onlret)
  - [`OFILL`](#ofill)
  - [`OFDEL`](#ofdel)
  - [`CLONE_VM`](#clone_vm)
  - [`CLONE_FS`](#clone_fs)
  - [`CLONE_FILES`](#clone_files)
  - [`CLONE_SIGHAND`](#clone_sighand)
  - [`CLONE_PTRACE`](#clone_ptrace)
  - [`CLONE_VFORK`](#clone_vfork)
  - [`CLONE_PARENT`](#clone_parent)
  - [`CLONE_THREAD`](#clone_thread)
  - [`CLONE_NEWNS`](#clone_newns)
  - [`CLONE_SYSVSEM`](#clone_sysvsem)
  - [`CLONE_SETTLS`](#clone_settls)
  - [`CLONE_PARENT_SETTID`](#clone_parent_settid)
  - [`CLONE_CHILD_CLEARTID`](#clone_child_cleartid)
  - [`CLONE_DETACHED`](#clone_detached)
  - [`CLONE_UNTRACED`](#clone_untraced)
  - [`CLONE_CHILD_SETTID`](#clone_child_settid)
  - [`CLONE_NEWCGROUP`](#clone_newcgroup)
  - [`CLONE_NEWUTS`](#clone_newuts)
  - [`CLONE_NEWIPC`](#clone_newipc)
  - [`CLONE_NEWUSER`](#clone_newuser)
  - [`CLONE_NEWPID`](#clone_newpid)
  - [`CLONE_NEWNET`](#clone_newnet)
  - [`CLONE_IO`](#clone_io)
  - [`WNOHANG`](#wnohang)
  - [`WUNTRACED`](#wuntraced)
  - [`WSTOPPED`](#wstopped)
  - [`WEXITED`](#wexited)
  - [`WCONTINUED`](#wcontinued)
  - [`WNOWAIT`](#wnowait)
  - [`ADDR_NO_RANDOMIZE`](#addr_no_randomize)
  - [`MMAP_PAGE_ZERO`](#mmap_page_zero)
  - [`ADDR_COMPAT_LAYOUT`](#addr_compat_layout)
  - [`READ_IMPLIES_EXEC`](#read_implies_exec)
  - [`ADDR_LIMIT_32BIT`](#addr_limit_32bit)
  - [`SHORT_INODE`](#short_inode)
  - [`WHOLE_SECONDS`](#whole_seconds)
  - [`STICKY_TIMEOUTS`](#sticky_timeouts)
  - [`ADDR_LIMIT_3GB`](#addr_limit_3gb)
  - [`PTRACE_O_TRACESYSGOOD`](#ptrace_o_tracesysgood)
  - [`PTRACE_O_TRACEFORK`](#ptrace_o_tracefork)
  - [`PTRACE_O_TRACEVFORK`](#ptrace_o_tracevfork)
  - [`PTRACE_O_TRACECLONE`](#ptrace_o_traceclone)
  - [`PTRACE_O_TRACEEXEC`](#ptrace_o_traceexec)
  - [`PTRACE_O_TRACEVFORKDONE`](#ptrace_o_tracevforkdone)
  - [`PTRACE_O_TRACEEXIT`](#ptrace_o_traceexit)
  - [`PTRACE_O_TRACESECCOMP`](#ptrace_o_traceseccomp)
  - [`PTRACE_O_SUSPEND_SECCOMP`](#ptrace_o_suspend_seccomp)
  - [`PTRACE_O_EXITKILL`](#ptrace_o_exitkill)
  - [`PTRACE_O_MASK`](#ptrace_o_mask)
  - [`PTRACE_EVENT_FORK`](#ptrace_event_fork)
  - [`PTRACE_EVENT_VFORK`](#ptrace_event_vfork)
  - [`PTRACE_EVENT_CLONE`](#ptrace_event_clone)
  - [`PTRACE_EVENT_EXEC`](#ptrace_event_exec)
  - [`PTRACE_EVENT_VFORK_DONE`](#ptrace_event_vfork_done)
  - [`PTRACE_EVENT_EXIT`](#ptrace_event_exit)
  - [`PTRACE_EVENT_SECCOMP`](#ptrace_event_seccomp)
  - [`__WNOTHREAD`](#__wnothread)
  - [`__WALL`](#__wall)
  - [`__WCLONE`](#__wclone)
  - [`SPLICE_F_MOVE`](#splice_f_move)
  - [`SPLICE_F_NONBLOCK`](#splice_f_nonblock)
  - [`SPLICE_F_MORE`](#splice_f_more)
  - [`SPLICE_F_GIFT`](#splice_f_gift)
  - [`RTLD_LOCAL`](#rtld_local)
  - [`RTLD_LAZY`](#rtld_lazy)
  - [`POSIX_FADV_NORMAL`](#posix_fadv_normal)
  - [`POSIX_FADV_RANDOM`](#posix_fadv_random)
  - [`POSIX_FADV_SEQUENTIAL`](#posix_fadv_sequential)
  - [`POSIX_FADV_WILLNEED`](#posix_fadv_willneed)
  - [`AT_FDCWD`](#at_fdcwd)
  - [`AT_SYMLINK_NOFOLLOW`](#at_symlink_nofollow)
  - [`AT_REMOVEDIR`](#at_removedir)
  - [`AT_SYMLINK_FOLLOW`](#at_symlink_follow)
  - [`AT_NO_AUTOMOUNT`](#at_no_automount)
  - [`AT_EMPTY_PATH`](#at_empty_path)
  - [`AT_RECURSIVE`](#at_recursive)
  - [`LOG_CRON`](#log_cron)
  - [`LOG_AUTHPRIV`](#log_authpriv)
  - [`LOG_FTP`](#log_ftp)
  - [`LOG_PERROR`](#log_perror)
  - [`PIPE_BUF`](#pipe_buf)
  - [`SI_LOAD_SHIFT`](#si_load_shift)
  - [`SI_USER`](#si_user)
  - [`SI_KERNEL`](#si_kernel)
  - [`SI_QUEUE`](#si_queue)
  - [`SI_TIMER`](#si_timer)
  - [`SI_MESGQ`](#si_mesgq)
  - [`SI_ASYNCIO`](#si_asyncio)
  - [`SI_SIGIO`](#si_sigio)
  - [`SI_TKILL`](#si_tkill)
  - [`SI_ASYNCNL`](#si_asyncnl)
  - [`BUS_ADRALN`](#bus_adraln)
  - [`BUS_ADRERR`](#bus_adrerr)
  - [`BUS_OBJERR`](#bus_objerr)
  - [`BUS_MCEERR_AR`](#bus_mceerr_ar)
  - [`BUS_MCEERR_AO`](#bus_mceerr_ao)
  - [`TRAP_BRKPT`](#trap_brkpt)
  - [`TRAP_TRACE`](#trap_trace)
  - [`TRAP_BRANCH`](#trap_branch)
  - [`TRAP_HWBKPT`](#trap_hwbkpt)
  - [`TRAP_UNK`](#trap_unk)
  - [`CLD_EXITED`](#cld_exited)
  - [`CLD_KILLED`](#cld_killed)
  - [`CLD_DUMPED`](#cld_dumped)
  - [`CLD_TRAPPED`](#cld_trapped)
  - [`CLD_STOPPED`](#cld_stopped)
  - [`CLD_CONTINUED`](#cld_continued)
  - [`SIGEV_SIGNAL`](#sigev_signal)
  - [`SIGEV_NONE`](#sigev_none)
  - [`SIGEV_THREAD`](#sigev_thread)
  - [`P_ALL`](#p_all)
  - [`P_PID`](#p_pid)
  - [`P_PGID`](#p_pgid)
  - [`P_PIDFD`](#p_pidfd)
  - [`UTIME_OMIT`](#utime_omit)
  - [`UTIME_NOW`](#utime_now)
  - [`POLLIN`](#pollin)
  - [`POLLPRI`](#pollpri)
  - [`POLLOUT`](#pollout)
  - [`POLLERR`](#pollerr)
  - [`POLLHUP`](#pollhup)
  - [`POLLNVAL`](#pollnval)
  - [`POLLRDNORM`](#pollrdnorm)
  - [`POLLRDBAND`](#pollrdband)
  - [`POLLRDHUP`](#pollrdhup)
  - [`IPTOS_LOWDELAY`](#iptos_lowdelay)
  - [`IPTOS_THROUGHPUT`](#iptos_throughput)
  - [`IPTOS_RELIABILITY`](#iptos_reliability)
  - [`IPTOS_MINCOST`](#iptos_mincost)
  - [`IPTOS_PREC_NETCONTROL`](#iptos_prec_netcontrol)
  - [`IPTOS_PREC_INTERNETCONTROL`](#iptos_prec_internetcontrol)
  - [`IPTOS_PREC_CRITIC_ECP`](#iptos_prec_critic_ecp)
  - [`IPTOS_PREC_FLASHOVERRIDE`](#iptos_prec_flashoverride)
  - [`IPTOS_PREC_FLASH`](#iptos_prec_flash)
  - [`IPTOS_PREC_IMMEDIATE`](#iptos_prec_immediate)
  - [`IPTOS_PREC_PRIORITY`](#iptos_prec_priority)
  - [`IPTOS_PREC_ROUTINE`](#iptos_prec_routine)
  - [`IPTOS_ECN_MASK`](#iptos_ecn_mask)
  - [`IPTOS_ECN_ECT1`](#iptos_ecn_ect1)
  - [`IPTOS_ECN_ECT0`](#iptos_ecn_ect0)
  - [`IPTOS_ECN_CE`](#iptos_ecn_ce)
  - [`IPOPT_COPY`](#ipopt_copy)
  - [`IPOPT_CLASS_MASK`](#ipopt_class_mask)
  - [`IPOPT_NUMBER_MASK`](#ipopt_number_mask)
  - [`IPOPT_CONTROL`](#ipopt_control)
  - [`IPOPT_RESERVED1`](#ipopt_reserved1)
  - [`IPOPT_MEASUREMENT`](#ipopt_measurement)
  - [`IPOPT_RESERVED2`](#ipopt_reserved2)
  - [`IPOPT_END`](#ipopt_end)
  - [`IPOPT_NOOP`](#ipopt_noop)
  - [`IPOPT_SEC`](#ipopt_sec)
  - [`IPOPT_LSRR`](#ipopt_lsrr)
  - [`IPOPT_TIMESTAMP`](#ipopt_timestamp)
  - [`IPOPT_RR`](#ipopt_rr)
  - [`IPOPT_SID`](#ipopt_sid)
  - [`IPOPT_SSRR`](#ipopt_ssrr)
  - [`IPOPT_RA`](#ipopt_ra)
  - [`IPVERSION`](#ipversion)
  - [`MAXTTL`](#maxttl)
  - [`IPDEFTTL`](#ipdefttl)
  - [`IPOPT_OPTVAL`](#ipopt_optval)
  - [`IPOPT_OLEN`](#ipopt_olen)
  - [`IPOPT_OFFSET`](#ipopt_offset)
  - [`IPOPT_MINOFF`](#ipopt_minoff)
  - [`MAX_IPOPTLEN`](#max_ipoptlen)
  - [`IPOPT_NOP`](#ipopt_nop)
  - [`IPOPT_EOL`](#ipopt_eol)
  - [`IPOPT_TS`](#ipopt_ts)
  - [`IPOPT_TS_TSONLY`](#ipopt_ts_tsonly)
  - [`IPOPT_TS_TSANDADDR`](#ipopt_ts_tsandaddr)
  - [`IPOPT_TS_PRESPEC`](#ipopt_ts_prespec)
  - [`ARPOP_RREQUEST`](#arpop_rrequest)
  - [`ARPOP_RREPLY`](#arpop_rreply)
  - [`ARPOP_InREQUEST`](#arpop_inrequest)
  - [`ARPOP_InREPLY`](#arpop_inreply)
  - [`ARPOP_NAK`](#arpop_nak)
  - [`ATF_NETMASK`](#atf_netmask)
  - [`ATF_DONTPUB`](#atf_dontpub)
  - [`ARPHRD_NETROM`](#arphrd_netrom)
  - [`ARPHRD_ETHER`](#arphrd_ether)
  - [`ARPHRD_EETHER`](#arphrd_eether)
  - [`ARPHRD_AX25`](#arphrd_ax25)
  - [`ARPHRD_PRONET`](#arphrd_pronet)
  - [`ARPHRD_CHAOS`](#arphrd_chaos)
  - [`ARPHRD_IEEE802`](#arphrd_ieee802)
  - [`ARPHRD_ARCNET`](#arphrd_arcnet)
  - [`ARPHRD_APPLETLK`](#arphrd_appletlk)
  - [`ARPHRD_DLCI`](#arphrd_dlci)
  - [`ARPHRD_ATM`](#arphrd_atm)
  - [`ARPHRD_METRICOM`](#arphrd_metricom)
  - [`ARPHRD_IEEE1394`](#arphrd_ieee1394)
  - [`ARPHRD_EUI64`](#arphrd_eui64)
  - [`ARPHRD_INFINIBAND`](#arphrd_infiniband)
  - [`ARPHRD_SLIP`](#arphrd_slip)
  - [`ARPHRD_CSLIP`](#arphrd_cslip)
  - [`ARPHRD_SLIP6`](#arphrd_slip6)
  - [`ARPHRD_CSLIP6`](#arphrd_cslip6)
  - [`ARPHRD_RSRVD`](#arphrd_rsrvd)
  - [`ARPHRD_ADAPT`](#arphrd_adapt)
  - [`ARPHRD_ROSE`](#arphrd_rose)
  - [`ARPHRD_X25`](#arphrd_x25)
  - [`ARPHRD_HWX25`](#arphrd_hwx25)
  - [`ARPHRD_CAN`](#arphrd_can)
  - [`ARPHRD_PPP`](#arphrd_ppp)
  - [`ARPHRD_CISCO`](#arphrd_cisco)
  - [`ARPHRD_HDLC`](#arphrd_hdlc)
  - [`ARPHRD_LAPB`](#arphrd_lapb)
  - [`ARPHRD_DDCMP`](#arphrd_ddcmp)
  - [`ARPHRD_RAWHDLC`](#arphrd_rawhdlc)
  - [`ARPHRD_TUNNEL`](#arphrd_tunnel)
  - [`ARPHRD_TUNNEL6`](#arphrd_tunnel6)
  - [`ARPHRD_FRAD`](#arphrd_frad)
  - [`ARPHRD_SKIP`](#arphrd_skip)
  - [`ARPHRD_LOOPBACK`](#arphrd_loopback)
  - [`ARPHRD_LOCALTLK`](#arphrd_localtlk)
  - [`ARPHRD_FDDI`](#arphrd_fddi)
  - [`ARPHRD_BIF`](#arphrd_bif)
  - [`ARPHRD_SIT`](#arphrd_sit)
  - [`ARPHRD_IPDDP`](#arphrd_ipddp)
  - [`ARPHRD_IPGRE`](#arphrd_ipgre)
  - [`ARPHRD_PIMREG`](#arphrd_pimreg)
  - [`ARPHRD_HIPPI`](#arphrd_hippi)
  - [`ARPHRD_ASH`](#arphrd_ash)
  - [`ARPHRD_ECONET`](#arphrd_econet)
  - [`ARPHRD_IRDA`](#arphrd_irda)
  - [`ARPHRD_FCPP`](#arphrd_fcpp)
  - [`ARPHRD_FCAL`](#arphrd_fcal)
  - [`ARPHRD_FCPL`](#arphrd_fcpl)
  - [`ARPHRD_FCFABRIC`](#arphrd_fcfabric)
  - [`ARPHRD_IEEE802_TR`](#arphrd_ieee802_tr)
  - [`ARPHRD_IEEE80211`](#arphrd_ieee80211)
  - [`ARPHRD_IEEE80211_PRISM`](#arphrd_ieee80211_prism)
  - [`ARPHRD_IEEE80211_RADIOTAP`](#arphrd_ieee80211_radiotap)
  - [`ARPHRD_IEEE802154`](#arphrd_ieee802154)
  - [`ARPHRD_VOID`](#arphrd_void)
  - [`ARPHRD_NONE`](#arphrd_none)
  - [`IFF_TUN`](#iff_tun)
  - [`IFF_TAP`](#iff_tap)
  - [`IFF_NAPI`](#iff_napi)
  - [`IFF_NAPI_FRAGS`](#iff_napi_frags)
  - [`IFF_NO_CARRIER`](#iff_no_carrier)
  - [`IFF_NO_PI`](#iff_no_pi)
  - [`TUN_READQ_SIZE`](#tun_readq_size)
  - [`TUN_TUN_DEV`](#tun_tun_dev)
  - [`TUN_TAP_DEV`](#tun_tap_dev)
  - [`TUN_TYPE_MASK`](#tun_type_mask)
  - [`IFF_ONE_QUEUE`](#iff_one_queue)
  - [`IFF_VNET_HDR`](#iff_vnet_hdr)
  - [`IFF_TUN_EXCL`](#iff_tun_excl)
  - [`IFF_MULTI_QUEUE`](#iff_multi_queue)
  - [`IFF_ATTACH_QUEUE`](#iff_attach_queue)
  - [`IFF_DETACH_QUEUE`](#iff_detach_queue)
  - [`IFF_PERSIST`](#iff_persist)
  - [`IFF_NOFILTER`](#iff_nofilter)
  - [`TUN_TX_TIMESTAMP`](#tun_tx_timestamp)
  - [`TUN_F_CSUM`](#tun_f_csum)
  - [`TUN_F_TSO4`](#tun_f_tso4)
  - [`TUN_F_TSO6`](#tun_f_tso6)
  - [`TUN_F_TSO_ECN`](#tun_f_tso_ecn)
  - [`TUN_F_UFO`](#tun_f_ufo)
  - [`TUN_F_USO4`](#tun_f_uso4)
  - [`TUN_F_USO6`](#tun_f_uso6)
  - [`TUN_PKT_STRIP`](#tun_pkt_strip)
  - [`TUN_FLT_ALLMULTI`](#tun_flt_allmulti)
  - [`T_TYPE`](#t_type)
  - [`TUNSETNOCSUM`](#tunsetnocsum)
  - [`TUNSETDEBUG`](#tunsetdebug)
  - [`TUNSETIFF`](#tunsetiff)
  - [`TUNSETPERSIST`](#tunsetpersist)
  - [`TUNSETOWNER`](#tunsetowner)
  - [`TUNSETLINK`](#tunsetlink)
  - [`TUNSETGROUP`](#tunsetgroup)
  - [`TUNGETFEATURES`](#tungetfeatures)
  - [`TUNSETOFFLOAD`](#tunsetoffload)
  - [`TUNSETTXFILTER`](#tunsettxfilter)
  - [`TUNGETIFF`](#tungetiff)
  - [`TUNGETSNDBUF`](#tungetsndbuf)
  - [`TUNSETSNDBUF`](#tunsetsndbuf)
  - [`TUNATTACHFILTER`](#tunattachfilter)
  - [`TUNDETACHFILTER`](#tundetachfilter)
  - [`TUNGETVNETHDRSZ`](#tungetvnethdrsz)
  - [`TUNSETVNETHDRSZ`](#tunsetvnethdrsz)
  - [`TUNSETQUEUE`](#tunsetqueue)
  - [`TUNSETIFINDEX`](#tunsetifindex)
  - [`TUNGETFILTER`](#tungetfilter)
  - [`TUNSETVNETLE`](#tunsetvnetle)
  - [`TUNGETVNETLE`](#tungetvnetle)
  - [`TUNSETVNETBE`](#tunsetvnetbe)
  - [`TUNGETVNETBE`](#tungetvnetbe)
  - [`TUNSETSTEERINGEBPF`](#tunsetsteeringebpf)
  - [`TUNSETFILTEREBPF`](#tunsetfilterebpf)
  - [`TUNSETCARRIER`](#tunsetcarrier)
  - [`TUNGETDEVNETNS`](#tungetdevnetns)
  - [`FS_IOC_GETFLAGS`](#fs_ioc_getflags)
  - [`FS_IOC_SETFLAGS`](#fs_ioc_setflags)
  - [`FS_IOC_GETVERSION`](#fs_ioc_getversion)
  - [`FS_IOC_SETVERSION`](#fs_ioc_setversion)
  - [`FS_IOC32_GETFLAGS`](#fs_ioc32_getflags)
  - [`FS_IOC32_SETFLAGS`](#fs_ioc32_setflags)
  - [`FS_IOC32_GETVERSION`](#fs_ioc32_getversion)
  - [`FS_IOC32_SETVERSION`](#fs_ioc32_setversion)
  - [`FICLONE`](#ficlone)
  - [`FICLONERANGE`](#ficlonerange)
  - [`ADFS_SUPER_MAGIC`](#adfs_super_magic)
  - [`AFFS_SUPER_MAGIC`](#affs_super_magic)
  - [`AFS_SUPER_MAGIC`](#afs_super_magic)
  - [`AUTOFS_SUPER_MAGIC`](#autofs_super_magic)
  - [`BPF_FS_MAGIC`](#bpf_fs_magic)
  - [`BTRFS_SUPER_MAGIC`](#btrfs_super_magic)
  - [`CGROUP2_SUPER_MAGIC`](#cgroup2_super_magic)
  - [`CGROUP_SUPER_MAGIC`](#cgroup_super_magic)
  - [`CODA_SUPER_MAGIC`](#coda_super_magic)
  - [`CRAMFS_MAGIC`](#cramfs_magic)
  - [`DEBUGFS_MAGIC`](#debugfs_magic)
  - [`DEVPTS_SUPER_MAGIC`](#devpts_super_magic)
  - [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs_super_magic)
  - [`EFS_SUPER_MAGIC`](#efs_super_magic)
  - [`EXT2_SUPER_MAGIC`](#ext2_super_magic)
  - [`EXT3_SUPER_MAGIC`](#ext3_super_magic)
  - [`EXT4_SUPER_MAGIC`](#ext4_super_magic)
  - [`F2FS_SUPER_MAGIC`](#f2fs_super_magic)
  - [`FUSE_SUPER_MAGIC`](#fuse_super_magic)
  - [`FUTEXFS_SUPER_MAGIC`](#futexfs_super_magic)
  - [`HOSTFS_SUPER_MAGIC`](#hostfs_super_magic)
  - [`HPFS_SUPER_MAGIC`](#hpfs_super_magic)
  - [`HUGETLBFS_MAGIC`](#hugetlbfs_magic)
  - [`ISOFS_SUPER_MAGIC`](#isofs_super_magic)
  - [`JFFS2_SUPER_MAGIC`](#jffs2_super_magic)
  - [`MINIX2_SUPER_MAGIC2`](#minix2_super_magic2)
  - [`MINIX2_SUPER_MAGIC`](#minix2_super_magic)
  - [`MINIX3_SUPER_MAGIC`](#minix3_super_magic)
  - [`MINIX_SUPER_MAGIC2`](#minix_super_magic2)
  - [`MINIX_SUPER_MAGIC`](#minix_super_magic)
  - [`MSDOS_SUPER_MAGIC`](#msdos_super_magic)
  - [`NCP_SUPER_MAGIC`](#ncp_super_magic)
  - [`NFS_SUPER_MAGIC`](#nfs_super_magic)
  - [`NILFS_SUPER_MAGIC`](#nilfs_super_magic)
  - [`OCFS2_SUPER_MAGIC`](#ocfs2_super_magic)
  - [`OPENPROM_SUPER_MAGIC`](#openprom_super_magic)
  - [`OVERLAYFS_SUPER_MAGIC`](#overlayfs_super_magic)
  - [`PROC_SUPER_MAGIC`](#proc_super_magic)
  - [`QNX4_SUPER_MAGIC`](#qnx4_super_magic)
  - [`QNX6_SUPER_MAGIC`](#qnx6_super_magic)
  - [`RDTGROUP_SUPER_MAGIC`](#rdtgroup_super_magic)
  - [`REISERFS_SUPER_MAGIC`](#reiserfs_super_magic)
  - [`SECURITYFS_MAGIC`](#securityfs_magic)
  - [`SELINUX_MAGIC`](#selinux_magic)
  - [`SMACK_MAGIC`](#smack_magic)
  - [`SMB_SUPER_MAGIC`](#smb_super_magic)
  - [`SYSFS_MAGIC`](#sysfs_magic)
  - [`TMPFS_MAGIC`](#tmpfs_magic)
  - [`TRACEFS_MAGIC`](#tracefs_magic)
  - [`UDF_SUPER_MAGIC`](#udf_super_magic)
  - [`USBDEVICE_SUPER_MAGIC`](#usbdevice_super_magic)
  - [`XENFS_SUPER_MAGIC`](#xenfs_super_magic)
  - [`NSFS_MAGIC`](#nsfs_magic)
  - [`AT_STATX_SYNC_TYPE`](#at_statx_sync_type)
  - [`AT_STATX_SYNC_AS_STAT`](#at_statx_sync_as_stat)
  - [`AT_STATX_FORCE_SYNC`](#at_statx_force_sync)
  - [`AT_STATX_DONT_SYNC`](#at_statx_dont_sync)
  - [`STATX_TYPE`](#statx_type)
  - [`STATX_MODE`](#statx_mode)
  - [`STATX_NLINK`](#statx_nlink)
  - [`STATX_UID`](#statx_uid)
  - [`STATX_GID`](#statx_gid)
  - [`STATX_ATIME`](#statx_atime)
  - [`STATX_MTIME`](#statx_mtime)
  - [`STATX_CTIME`](#statx_ctime)
  - [`STATX_INO`](#statx_ino)
  - [`STATX_SIZE`](#statx_size)
  - [`STATX_BLOCKS`](#statx_blocks)
  - [`STATX_BASIC_STATS`](#statx_basic_stats)
  - [`STATX_BTIME`](#statx_btime)
  - [`STATX_ALL`](#statx_all)
  - [`STATX_MNT_ID`](#statx_mnt_id)
  - [`STATX_DIOALIGN`](#statx_dioalign)
  - [`STATX__RESERVED`](#statx__reserved)
  - [`STATX_ATTR_COMPRESSED`](#statx_attr_compressed)
  - [`STATX_ATTR_IMMUTABLE`](#statx_attr_immutable)
  - [`STATX_ATTR_APPEND`](#statx_attr_append)
  - [`STATX_ATTR_NODUMP`](#statx_attr_nodump)
  - [`STATX_ATTR_ENCRYPTED`](#statx_attr_encrypted)
  - [`STATX_ATTR_AUTOMOUNT`](#statx_attr_automount)
  - [`STATX_ATTR_MOUNT_ROOT`](#statx_attr_mount_root)
  - [`STATX_ATTR_VERITY`](#statx_attr_verity)
  - [`STATX_ATTR_DAX`](#statx_attr_dax)
  - [`_IOC_NRBITS`](#_ioc_nrbits)
  - [`_IOC_TYPEBITS`](#_ioc_typebits)
  - [`_IOC_SIZEBITS`](#_ioc_sizebits)
  - [`_IOC_DIRBITS`](#_ioc_dirbits)
  - [`_IOC_NONE`](#_ioc_none)
  - [`_IOC_WRITE`](#_ioc_write)
  - [`_IOC_READ`](#_ioc_read)
  - [`_IOC_NRMASK`](#_ioc_nrmask)
  - [`_IOC_TYPEMASK`](#_ioc_typemask)
  - [`_IOC_SIZEMASK`](#_ioc_sizemask)
  - [`_IOC_DIRMASK`](#_ioc_dirmask)
  - [`_IOC_NRSHIFT`](#_ioc_nrshift)
  - [`_IOC_TYPESHIFT`](#_ioc_typeshift)
  - [`_IOC_SIZESHIFT`](#_ioc_sizeshift)
  - [`_IOC_DIRSHIFT`](#_ioc_dirshift)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`linux_like`](#linux_like) | mod |  |
| [`linux`](#linux) | mod | Linux-specific definitions for linux-like values |
| [`group`](#group) | struct |  |
| [`utimbuf`](#utimbuf) | struct |  |
| [`timeval`](#timeval) | struct |  |
| [`rlimit`](#rlimit) | struct |  |
| [`rusage`](#rusage) | struct |  |
| [`ipv6_mreq`](#ipv6_mreq) | struct |  |
| [`hostent`](#hostent) | struct |  |
| [`iovec`](#iovec) | struct |  |
| [`pollfd`](#pollfd) | struct |  |
| [`winsize`](#winsize) | struct |  |
| [`linger`](#linger) | struct |  |
| [`sigval`](#sigval) | struct |  |
| [`itimerval`](#itimerval) | struct |  |
| [`tms`](#tms) | struct |  |
| [`servent`](#servent) | struct |  |
| [`protoent`](#protoent) | struct |  |
| [`in6_addr`](#in6_addr) | struct |  |
| [`in_addr`](#in_addr) | struct |  |
| [`ip_mreq`](#ip_mreq) | struct |  |
| [`ip_mreqn`](#ip_mreqn) | struct |  |
| [`ip_mreq_source`](#ip_mreq_source) | struct |  |
| [`sockaddr`](#sockaddr) | struct |  |
| [`sockaddr_in`](#sockaddr_in) | struct |  |
| [`sockaddr_in6`](#sockaddr_in6) | struct |  |
| [`addrinfo`](#addrinfo) | struct |  |
| [`sockaddr_ll`](#sockaddr_ll) | struct |  |
| [`fd_set`](#fd_set) | struct |  |
| [`tm`](#tm) | struct |  |
| [`sched_param`](#sched_param) | struct |  |
| [`Dl_info`](#dl_info) | struct |  |
| [`lconv`](#lconv) | struct |  |
| [`in_pktinfo`](#in_pktinfo) | struct |  |
| [`ifaddrs`](#ifaddrs) | struct |  |
| [`in6_rtmsg`](#in6_rtmsg) | struct |  |
| [`arpreq`](#arpreq) | struct |  |
| [`arpreq_old`](#arpreq_old) | struct |  |
| [`arphdr`](#arphdr) | struct |  |
| [`mmsghdr`](#mmsghdr) | struct |  |
| [`sockaddr_un`](#sockaddr_un) | struct |  |
| [`sockaddr_storage`](#sockaddr_storage) | struct |  |
| [`utsname`](#utsname) | struct |  |
| [`file_clone_range`](#file_clone_range) | struct |  |
| [`sock_filter`](#sock_filter) | struct |  |
| [`sock_fprog`](#sock_fprog) | struct |  |
| [`statx`](#statx) | struct |  |
| [`statx_timestamp`](#statx_timestamp) | struct |  |
| [`epoll_event`](#epoll_event) | struct |  |
| [`sigevent`](#sigevent) | struct |  |
| [`DIR`](#dir) | enum |  |
| [`FILE`](#file) | enum |  |
| [`timezone`](#timezone) | enum |  |
| [`isalnum`](#isalnum) | fn |  |
| [`isalpha`](#isalpha) | fn |  |
| [`iscntrl`](#iscntrl) | fn |  |
| [`isdigit`](#isdigit) | fn |  |
| [`isgraph`](#isgraph) | fn |  |
| [`islower`](#islower) | fn |  |
| [`isprint`](#isprint) | fn |  |
| [`ispunct`](#ispunct) | fn |  |
| [`isspace`](#isspace) | fn |  |
| [`isupper`](#isupper) | fn |  |
| [`isxdigit`](#isxdigit) | fn |  |
| [`isblank`](#isblank) | fn |  |
| [`tolower`](#tolower) | fn |  |
| [`toupper`](#toupper) | fn |  |
| [`qsort`](#qsort) | fn |  |
| [`bsearch`](#bsearch) | fn |  |
| [`fopen`](#fopen) | fn |  |
| [`freopen`](#freopen) | fn |  |
| [`fflush`](#fflush) | fn |  |
| [`fclose`](#fclose) | fn |  |
| [`remove`](#remove) | fn |  |
| [`rename`](#rename) | fn |  |
| [`tmpfile`](#tmpfile) | fn |  |
| [`setvbuf`](#setvbuf) | fn |  |
| [`setbuf`](#setbuf) | fn |  |
| [`getchar`](#getchar) | fn |  |
| [`putchar`](#putchar) | fn |  |
| [`fgetc`](#fgetc) | fn |  |
| [`fgets`](#fgets) | fn |  |
| [`fputc`](#fputc) | fn |  |
| [`fputs`](#fputs) | fn |  |
| [`puts`](#puts) | fn |  |
| [`ungetc`](#ungetc) | fn |  |
| [`fread`](#fread) | fn |  |
| [`fwrite`](#fwrite) | fn |  |
| [`fseek`](#fseek) | fn |  |
| [`ftell`](#ftell) | fn |  |
| [`rewind`](#rewind) | fn |  |
| [`fgetpos`](#fgetpos) | fn |  |
| [`fsetpos`](#fsetpos) | fn |  |
| [`feof`](#feof) | fn |  |
| [`ferror`](#ferror) | fn |  |
| [`clearerr`](#clearerr) | fn |  |
| [`perror`](#perror) | fn |  |
| [`atof`](#atof) | fn |  |
| [`atoi`](#atoi) | fn |  |
| [`atol`](#atol) | fn |  |
| [`atoll`](#atoll) | fn |  |
| [`strtod`](#strtod) | fn |  |
| [`strtof`](#strtof) | fn |  |
| [`strtol`](#strtol) | fn |  |
| [`strtoll`](#strtoll) | fn |  |
| [`strtoul`](#strtoul) | fn |  |
| [`strtoull`](#strtoull) | fn |  |
| [`calloc`](#calloc) | fn |  |
| [`malloc`](#malloc) | fn |  |
| [`realloc`](#realloc) | fn |  |
| [`free`](#free) | fn |  |
| [`abort`](#abort) | fn |  |
| [`exit`](#exit) | fn |  |
| [`_exit`](#_exit) | fn |  |
| [`system`](#system) | fn |  |
| [`getenv`](#getenv) | fn |  |
| [`strcpy`](#strcpy) | fn |  |
| [`strncpy`](#strncpy) | fn |  |
| [`stpcpy`](#stpcpy) | fn |  |
| [`strcat`](#strcat) | fn |  |
| [`strncat`](#strncat) | fn |  |
| [`strcmp`](#strcmp) | fn |  |
| [`strncmp`](#strncmp) | fn |  |
| [`strcoll`](#strcoll) | fn |  |
| [`strchr`](#strchr) | fn |  |
| [`strrchr`](#strrchr) | fn |  |
| [`strspn`](#strspn) | fn |  |
| [`strcspn`](#strcspn) | fn |  |
| [`strdup`](#strdup) | fn |  |
| [`strndup`](#strndup) | fn |  |
| [`strpbrk`](#strpbrk) | fn |  |
| [`strstr`](#strstr) | fn |  |
| [`strcasecmp`](#strcasecmp) | fn |  |
| [`strncasecmp`](#strncasecmp) | fn |  |
| [`strlen`](#strlen) | fn |  |
| [`strnlen`](#strnlen) | fn |  |
| [`strerror`](#strerror) | fn |  |
| [`strtok`](#strtok) | fn |  |
| [`strtok_r`](#strtok_r) | fn |  |
| [`strxfrm`](#strxfrm) | fn |  |
| [`strsignal`](#strsignal) | fn |  |
| [`wcslen`](#wcslen) | fn |  |
| [`wcstombs`](#wcstombs) | fn |  |
| [`memchr`](#memchr) | fn |  |
| [`wmemchr`](#wmemchr) | fn |  |
| [`memcmp`](#memcmp) | fn |  |
| [`memcpy`](#memcpy) | fn |  |
| [`memmove`](#memmove) | fn |  |
| [`memset`](#memset) | fn |  |
| [`memccpy`](#memccpy) | fn |  |
| [`getpwnam`](#getpwnam) | fn |  |
| [`getpwuid`](#getpwuid) | fn |  |
| [`fprintf`](#fprintf) | fn |  |
| [`printf`](#printf) | fn |  |
| [`snprintf`](#snprintf) | fn |  |
| [`sprintf`](#sprintf) | fn |  |
| [`fscanf`](#fscanf) | fn |  |
| [`scanf`](#scanf) | fn |  |
| [`sscanf`](#sscanf) | fn |  |
| [`getchar_unlocked`](#getchar_unlocked) | fn |  |
| [`putchar_unlocked`](#putchar_unlocked) | fn |  |
| [`socket`](#socket) | fn |  |
| [`connect`](#connect) | fn |  |
| [`listen`](#listen) | fn |  |
| [`accept`](#accept) | fn |  |
| [`getpeername`](#getpeername) | fn |  |
| [`getsockname`](#getsockname) | fn |  |
| [`setsockopt`](#setsockopt) | fn |  |
| [`socketpair`](#socketpair) | fn |  |
| [`sendto`](#sendto) | fn |  |
| [`shutdown`](#shutdown) | fn |  |
| [`chmod`](#chmod) | fn |  |
| [`fchmod`](#fchmod) | fn |  |
| [`fstat`](#fstat) | fn |  |
| [`mkdir`](#mkdir) | fn |  |
| [`stat`](#stat) | fn |  |
| [`pclose`](#pclose) | fn |  |
| [`fdopen`](#fdopen) | fn |  |
| [`fileno`](#fileno) | fn |  |
| [`open`](#open) | fn |  |
| [`creat`](#creat) | fn |  |
| [`fcntl`](#fcntl) | fn |  |
| [`opendir`](#opendir) | fn |  |
| [`readdir`](#readdir) | fn |  |
| [`closedir`](#closedir) | fn |  |
| [`rewinddir`](#rewinddir) | fn |  |
| [`fchmodat`](#fchmodat) | fn |  |
| [`fchown`](#fchown) | fn |  |
| [`fchownat`](#fchownat) | fn |  |
| [`fstatat`](#fstatat) | fn |  |
| [`linkat`](#linkat) | fn |  |
| [`renameat`](#renameat) | fn |  |
| [`symlinkat`](#symlinkat) | fn |  |
| [`unlinkat`](#unlinkat) | fn |  |
| [`access`](#access) | fn |  |
| [`alarm`](#alarm) | fn |  |
| [`chdir`](#chdir) | fn |  |
| [`fchdir`](#fchdir) | fn |  |
| [`chown`](#chown) | fn |  |
| [`lchown`](#lchown) | fn |  |
| [`close`](#close) | fn |  |
| [`dup`](#dup) | fn |  |
| [`dup2`](#dup2) | fn |  |
| [`execl`](#execl) | fn |  |
| [`execle`](#execle) | fn |  |
| [`execlp`](#execlp) | fn |  |
| [`execv`](#execv) | fn |  |
| [`execve`](#execve) | fn |  |
| [`execvp`](#execvp) | fn |  |
| [`fork`](#fork) | fn |  |
| [`fpathconf`](#fpathconf) | fn |  |
| [`getcwd`](#getcwd) | fn |  |
| [`getegid`](#getegid) | fn |  |
| [`geteuid`](#geteuid) | fn |  |
| [`getgid`](#getgid) | fn |  |
| [`getgroups`](#getgroups) | fn |  |
| [`getlogin`](#getlogin) | fn |  |
| [`getopt`](#getopt) | fn |  |
| [`getpgid`](#getpgid) | fn |  |
| [`getpgrp`](#getpgrp) | fn |  |
| [`getpid`](#getpid) | fn |  |
| [`getppid`](#getppid) | fn |  |
| [`getuid`](#getuid) | fn |  |
| [`isatty`](#isatty) | fn |  |
| [`link`](#link) | fn |  |
| [`lseek`](#lseek) | fn |  |
| [`pathconf`](#pathconf) | fn |  |
| [`pipe`](#pipe) | fn |  |
| [`posix_memalign`](#posix_memalign) | fn |  |
| [`aligned_alloc`](#aligned_alloc) | fn |  |
| [`read`](#read) | fn |  |
| [`rmdir`](#rmdir) | fn |  |
| [`seteuid`](#seteuid) | fn |  |
| [`setegid`](#setegid) | fn |  |
| [`setgid`](#setgid) | fn |  |
| [`setpgid`](#setpgid) | fn |  |
| [`setsid`](#setsid) | fn |  |
| [`setuid`](#setuid) | fn |  |
| [`setreuid`](#setreuid) | fn |  |
| [`setregid`](#setregid) | fn |  |
| [`sleep`](#sleep) | fn |  |
| [`nanosleep`](#nanosleep) | fn |  |
| [`tcgetpgrp`](#tcgetpgrp) | fn |  |
| [`tcsetpgrp`](#tcsetpgrp) | fn |  |
| [`ttyname`](#ttyname) | fn |  |
| [`ttyname_r`](#ttyname_r) | fn |  |
| [`unlink`](#unlink) | fn |  |
| [`wait`](#wait) | fn |  |
| [`waitpid`](#waitpid) | fn |  |
| [`write`](#write) | fn |  |
| [`pread`](#pread) | fn |  |
| [`pwrite`](#pwrite) | fn |  |
| [`umask`](#umask) | fn |  |
| [`utime`](#utime) | fn |  |
| [`kill`](#kill) | fn |  |
| [`killpg`](#killpg) | fn |  |
| [`mlock`](#mlock) | fn |  |
| [`munlock`](#munlock) | fn |  |
| [`mlockall`](#mlockall) | fn |  |
| [`munlockall`](#munlockall) | fn |  |
| [`mmap`](#mmap) | fn |  |
| [`munmap`](#munmap) | fn |  |
| [`if_nametoindex`](#if_nametoindex) | fn |  |
| [`if_indextoname`](#if_indextoname) | fn |  |
| [`lstat`](#lstat) | fn |  |
| [`fsync`](#fsync) | fn |  |
| [`setenv`](#setenv) | fn |  |
| [`unsetenv`](#unsetenv) | fn |  |
| [`symlink`](#symlink) | fn |  |
| [`truncate`](#truncate) | fn |  |
| [`ftruncate`](#ftruncate) | fn |  |
| [`signal`](#signal) | fn |  |
| [`getrusage`](#getrusage) | fn |  |
| [`realpath`](#realpath) | fn |  |
| [`times`](#times) | fn |  |
| [`pthread_self`](#pthread_self) | fn |  |
| [`pthread_equal`](#pthread_equal) | fn |  |
| [`pthread_join`](#pthread_join) | fn |  |
| [`pthread_exit`](#pthread_exit) | fn |  |
| [`pthread_attr_init`](#pthread_attr_init) | fn |  |
| [`pthread_attr_destroy`](#pthread_attr_destroy) | fn |  |
| [`pthread_attr_getstacksize`](#pthread_attr_getstacksize) | fn |  |
| [`pthread_attr_setstacksize`](#pthread_attr_setstacksize) | fn |  |
| [`pthread_attr_setdetachstate`](#pthread_attr_setdetachstate) | fn |  |
| [`pthread_detach`](#pthread_detach) | fn |  |
| [`sched_yield`](#sched_yield) | fn |  |
| [`pthread_key_create`](#pthread_key_create) | fn |  |
| [`pthread_key_delete`](#pthread_key_delete) | fn |  |
| [`pthread_getspecific`](#pthread_getspecific) | fn |  |
| [`pthread_setspecific`](#pthread_setspecific) | fn |  |
| [`pthread_mutex_init`](#pthread_mutex_init) | fn |  |
| [`pthread_mutex_destroy`](#pthread_mutex_destroy) | fn |  |
| [`pthread_mutex_lock`](#pthread_mutex_lock) | fn |  |
| [`pthread_mutex_trylock`](#pthread_mutex_trylock) | fn |  |
| [`pthread_mutex_unlock`](#pthread_mutex_unlock) | fn |  |
| [`pthread_mutexattr_init`](#pthread_mutexattr_init) | fn |  |
| [`pthread_mutexattr_destroy`](#pthread_mutexattr_destroy) | fn |  |
| [`pthread_mutexattr_settype`](#pthread_mutexattr_settype) | fn |  |
| [`pthread_cond_init`](#pthread_cond_init) | fn |  |
| [`pthread_cond_wait`](#pthread_cond_wait) | fn |  |
| [`pthread_cond_timedwait`](#pthread_cond_timedwait) | fn |  |
| [`pthread_cond_signal`](#pthread_cond_signal) | fn |  |
| [`pthread_cond_broadcast`](#pthread_cond_broadcast) | fn |  |
| [`pthread_cond_destroy`](#pthread_cond_destroy) | fn |  |
| [`pthread_condattr_init`](#pthread_condattr_init) | fn |  |
| [`pthread_condattr_destroy`](#pthread_condattr_destroy) | fn |  |
| [`pthread_rwlock_init`](#pthread_rwlock_init) | fn |  |
| [`pthread_rwlock_destroy`](#pthread_rwlock_destroy) | fn |  |
| [`pthread_rwlock_rdlock`](#pthread_rwlock_rdlock) | fn |  |
| [`pthread_rwlock_tryrdlock`](#pthread_rwlock_tryrdlock) | fn |  |
| [`pthread_rwlock_wrlock`](#pthread_rwlock_wrlock) | fn |  |
| [`pthread_rwlock_trywrlock`](#pthread_rwlock_trywrlock) | fn |  |
| [`pthread_rwlock_unlock`](#pthread_rwlock_unlock) | fn |  |
| [`pthread_rwlockattr_init`](#pthread_rwlockattr_init) | fn |  |
| [`pthread_rwlockattr_destroy`](#pthread_rwlockattr_destroy) | fn |  |
| [`getsockopt`](#getsockopt) | fn |  |
| [`raise`](#raise) | fn |  |
| [`utimes`](#utimes) | fn |  |
| [`dlopen`](#dlopen) | fn |  |
| [`dlerror`](#dlerror) | fn |  |
| [`dlsym`](#dlsym) | fn |  |
| [`dlclose`](#dlclose) | fn |  |
| [`getaddrinfo`](#getaddrinfo) | fn |  |
| [`freeaddrinfo`](#freeaddrinfo) | fn |  |
| [`hstrerror`](#hstrerror) | fn |  |
| [`gai_strerror`](#gai_strerror) | fn |  |
| [`res_init`](#res_init) | fn |  |
| [`gmtime_r`](#gmtime_r) | fn |  |
| [`localtime_r`](#localtime_r) | fn |  |
| [`mktime`](#mktime) | fn |  |
| [`time`](#time) | fn |  |
| [`gmtime`](#gmtime) | fn |  |
| [`localtime`](#localtime) | fn |  |
| [`difftime`](#difftime) | fn |  |
| [`timegm`](#timegm) | fn |  |
| [`mknod`](#mknod) | fn |  |
| [`gethostname`](#gethostname) | fn |  |
| [`endservent`](#endservent) | fn |  |
| [`getservbyname`](#getservbyname) | fn |  |
| [`getservbyport`](#getservbyport) | fn |  |
| [`getservent`](#getservent) | fn |  |
| [`setservent`](#setservent) | fn |  |
| [`getprotobyname`](#getprotobyname) | fn |  |
| [`getprotobynumber`](#getprotobynumber) | fn |  |
| [`chroot`](#chroot) | fn |  |
| [`usleep`](#usleep) | fn |  |
| [`send`](#send) | fn |  |
| [`recv`](#recv) | fn |  |
| [`putenv`](#putenv) | fn |  |
| [`poll`](#poll) | fn |  |
| [`select`](#select) | fn |  |
| [`setlocale`](#setlocale) | fn |  |
| [`localeconv`](#localeconv) | fn |  |
| [`sem_wait`](#sem_wait) | fn |  |
| [`sem_trywait`](#sem_trywait) | fn |  |
| [`sem_post`](#sem_post) | fn |  |
| [`statvfs`](#statvfs) | fn |  |
| [`fstatvfs`](#fstatvfs) | fn |  |
| [`sigemptyset`](#sigemptyset) | fn |  |
| [`sigaddset`](#sigaddset) | fn |  |
| [`sigfillset`](#sigfillset) | fn |  |
| [`sigdelset`](#sigdelset) | fn |  |
| [`sigismember`](#sigismember) | fn |  |
| [`sigprocmask`](#sigprocmask) | fn |  |
| [`sigpending`](#sigpending) | fn |  |
| [`sysconf`](#sysconf) | fn |  |
| [`mkfifo`](#mkfifo) | fn |  |
| [`fseeko`](#fseeko) | fn |  |
| [`ftello`](#ftello) | fn |  |
| [`tcdrain`](#tcdrain) | fn |  |
| [`cfgetispeed`](#cfgetispeed) | fn |  |
| [`cfgetospeed`](#cfgetospeed) | fn |  |
| [`cfsetispeed`](#cfsetispeed) | fn |  |
| [`cfsetospeed`](#cfsetospeed) | fn |  |
| [`tcgetattr`](#tcgetattr) | fn |  |
| [`tcsetattr`](#tcsetattr) | fn |  |
| [`tcflow`](#tcflow) | fn |  |
| [`tcflush`](#tcflush) | fn |  |
| [`tcgetsid`](#tcgetsid) | fn |  |
| [`tcsendbreak`](#tcsendbreak) | fn |  |
| [`mkstemp`](#mkstemp) | fn |  |
| [`mkdtemp`](#mkdtemp) | fn |  |
| [`tmpnam`](#tmpnam) | fn |  |
| [`openlog`](#openlog) | fn |  |
| [`closelog`](#closelog) | fn |  |
| [`setlogmask`](#setlogmask) | fn |  |
| [`syslog`](#syslog) | fn |  |
| [`nice`](#nice) | fn |  |
| [`grantpt`](#grantpt) | fn |  |
| [`posix_openpt`](#posix_openpt) | fn |  |
| [`ptsname`](#ptsname) | fn |  |
| [`unlockpt`](#unlockpt) | fn |  |
| [`strcasestr`](#strcasestr) | fn |  |
| [`getline`](#getline) | fn |  |
| [`lockf`](#lockf) | fn |  |
| [`adjtime`](#adjtime) | fn |  |
| [`stpncpy`](#stpncpy) | fn |  |
| [`sigqueue`](#sigqueue) | fn |  |
| [`confstr`](#confstr) | fn |  |
| [`dladdr`](#dladdr) | fn |  |
| [`flock`](#flock) | fn |  |
| [`open_wmemstream`](#open_wmemstream) | fn |  |
| [`getsid`](#getsid) | fn |  |
| [`pause`](#pause) | fn |  |
| [`mkdirat`](#mkdirat) | fn |  |
| [`openat`](#openat) | fn |  |
| [`fdopendir`](#fdopendir) | fn |  |
| [`readdir_r`](#readdir_r) | fn | The 64-bit libc on Solaris and illumos only has readdir_r. |
| [`readlinkat`](#readlinkat) | fn |  |
| [`fmemopen`](#fmemopen) | fn |  |
| [`open_memstream`](#open_memstream) | fn |  |
| [`atexit`](#atexit) | fn |  |
| [`sigaction`](#sigaction) | fn |  |
| [`readlink`](#readlink) | fn |  |
| [`pselect`](#pselect) | fn |  |
| [`cfmakeraw`](#cfmakeraw) | fn |  |
| [`cfsetspeed`](#cfsetspeed) | fn |  |
| [`fnmatch`](#fnmatch) | fn |  |
| [`htonl`](#htonl) | fn |  |
| [`htons`](#htons) | fn |  |
| [`ntohl`](#ntohl) | fn |  |
| [`ntohs`](#ntohs) | fn |  |
| [`ioctl`](#ioctl) | fn |  |
| [`sem_destroy`](#sem_destroy) | fn |  |
| [`sem_init`](#sem_init) | fn |  |
| [`fdatasync`](#fdatasync) | fn |  |
| [`mincore`](#mincore) | fn |  |
| [`clock_getres`](#clock_getres) | fn |  |
| [`clock_gettime`](#clock_gettime) | fn |  |
| [`clock_settime`](#clock_settime) | fn |  |
| [`clock_getcpuclockid`](#clock_getcpuclockid) | fn |  |
| [`dirfd`](#dirfd) | fn |  |
| [`memalign`](#memalign) | fn |  |
| [`setgroups`](#setgroups) | fn |  |
| [`pipe2`](#pipe2) | fn |  |
| [`statfs`](#statfs) | fn |  |
| [`fstatfs`](#fstatfs) | fn |  |
| [`memrchr`](#memrchr) | fn |  |
| [`posix_fadvise`](#posix_fadvise) | fn |  |
| [`futimens`](#futimens) | fn |  |
| [`utimensat`](#utimensat) | fn |  |
| [`duplocale`](#duplocale) | fn |  |
| [`freelocale`](#freelocale) | fn |  |
| [`newlocale`](#newlocale) | fn |  |
| [`uselocale`](#uselocale) | fn |  |
| [`mknodat`](#mknodat) | fn |  |
| [`ptsname_r`](#ptsname_r) | fn |  |
| [`clearenv`](#clearenv) | fn |  |
| [`waitid`](#waitid) | fn |  |
| [`getresuid`](#getresuid) | fn |  |
| [`getresgid`](#getresgid) | fn |  |
| [`acct`](#acct) | fn |  |
| [`brk`](#brk) | fn |  |
| [`sbrk`](#sbrk) | fn |  |
| [`vfork`](#vfork) | fn |  |
| [`setresgid`](#setresgid) | fn |  |
| [`setresuid`](#setresuid) | fn |  |
| [`wait4`](#wait4) | fn |  |
| [`login_tty`](#login_tty) | fn |  |
| [`execvpe`](#execvpe) | fn |  |
| [`fexecve`](#fexecve) | fn |  |
| [`getifaddrs`](#getifaddrs) | fn |  |
| [`freeifaddrs`](#freeifaddrs) | fn |  |
| [`bind`](#bind) | fn |  |
| [`writev`](#writev) | fn |  |
| [`readv`](#readv) | fn |  |
| [`sendmsg`](#sendmsg) | fn |  |
| [`recvmsg`](#recvmsg) | fn |  |
| [`uname`](#uname) | fn |  |
| [`strchrnul`](#strchrnul) | fn |  |
| [`strftime`](#strftime) | fn |  |
| [`strftime_l`](#strftime_l) | fn |  |
| [`strptime`](#strptime) | fn |  |
| [`mkostemp`](#mkostemp) | fn |  |
| [`mkostemps`](#mkostemps) | fn |  |
| [`getdomainname`](#getdomainname) | fn |  |
| [`setdomainname`](#setdomainname) | fn |  |
| [`fstatfs64`](#fstatfs64) | fn |  |
| [`statvfs64`](#statvfs64) | fn |  |
| [`fstatvfs64`](#fstatvfs64) | fn |  |
| [`statfs64`](#statfs64) | fn |  |
| [`creat64`](#creat64) | fn |  |
| [`fstat64`](#fstat64) | fn |  |
| [`fstatat64`](#fstatat64) | fn |  |
| [`ftruncate64`](#ftruncate64) | fn |  |
| [`lseek64`](#lseek64) | fn |  |
| [`lstat64`](#lstat64) | fn |  |
| [`mmap64`](#mmap64) | fn |  |
| [`open64`](#open64) | fn |  |
| [`openat64`](#openat64) | fn |  |
| [`posix_fadvise64`](#posix_fadvise64) | fn |  |
| [`pread64`](#pread64) | fn |  |
| [`pwrite64`](#pwrite64) | fn |  |
| [`readdir64`](#readdir64) | fn |  |
| [`readdir64_r`](#readdir64_r) | fn |  |
| [`stat64`](#stat64) | fn |  |
| [`truncate64`](#truncate64) | fn |  |
| [`preadv64`](#preadv64) | fn |  |
| [`pwritev64`](#pwritev64) | fn |  |
| [`forkpty`](#forkpty) | fn |  |
| [`openpty`](#openpty) | fn |  |
| [`statx`](#statx) | fn |  |
| [`_IOC`](#_ioc) | fn | Build an ioctl number, analogous to the C macro of the same name. |
| [`_IO`](#_io) | fn | Build an ioctl number for an argumentless ioctl. |
| [`_IOR`](#_ior) | fn | Build an ioctl number for an read-only ioctl. |
| [`_IOW`](#_iow) | fn | Build an ioctl number for an write-only ioctl. |
| [`_IOWR`](#_iowr) | fn | Build an ioctl number for a read-write ioctl. |
| [`CMSG_ALIGN`](#cmsg_align) | fn |  |
| [`CMSG_FIRSTHDR`](#cmsg_firsthdr) | fn |  |
| [`CMSG_DATA`](#cmsg_data) | fn |  |
| [`CMSG_SPACE`](#cmsg_space) | fn |  |
| [`CMSG_LEN`](#cmsg_len) | fn |  |
| [`FD_CLR`](#fd_clr) | fn |  |
| [`FD_ISSET`](#fd_isset) | fn |  |
| [`FD_SET`](#fd_set) | fn |  |
| [`FD_ZERO`](#fd_zero) | fn |  |
| [`SIGRTMAX`](#sigrtmax) | fn |  |
| [`SIGRTMIN`](#sigrtmin) | fn |  |
| [`WIFSTOPPED`](#wifstopped) | fn |  |
| [`WSTOPSIG`](#wstopsig) | fn |  |
| [`WIFCONTINUED`](#wifcontinued) | fn |  |
| [`WIFSIGNALED`](#wifsignaled) | fn |  |
| [`WTERMSIG`](#wtermsig) | fn |  |
| [`WIFEXITED`](#wifexited) | fn |  |
| [`WEXITSTATUS`](#wexitstatus) | fn |  |
| [`WCOREDUMP`](#wcoredump) | fn |  |
| [`W_EXITCODE`](#w_exitcode) | fn |  |
| [`W_STOPCODE`](#w_stopcode) | fn |  |
| [`QCMD`](#qcmd) | fn |  |
| [`IPOPT_COPIED`](#ipopt_copied) | fn |  |
| [`IPOPT_CLASS`](#ipopt_class) | fn |  |
| [`IPOPT_NUMBER`](#ipopt_number) | fn |  |
| [`IPTOS_ECN`](#iptos_ecn) | fn |  |
| [`KERNEL_VERSION`](#kernel_version) | fn |  |
| [`intmax_t`](#intmax_t) | type |  |
| [`uintmax_t`](#uintmax_t) | type |  |
| [`size_t`](#size_t) | type |  |
| [`ptrdiff_t`](#ptrdiff_t) | type |  |
| [`intptr_t`](#intptr_t) | type |  |
| [`uintptr_t`](#uintptr_t) | type |  |
| [`ssize_t`](#ssize_t) | type |  |
| [`pid_t`](#pid_t) | type |  |
| [`in_addr_t`](#in_addr_t) | type |  |
| [`in_port_t`](#in_port_t) | type |  |
| [`sighandler_t`](#sighandler_t) | type |  |
| [`cc_t`](#cc_t) | type |  |
| [`uid_t`](#uid_t) | type |  |
| [`gid_t`](#gid_t) | type |  |
| [`locale_t`](#locale_t) | type |  |
| [`sa_family_t`](#sa_family_t) | type |  |
| [`speed_t`](#speed_t) | type |  |
| [`tcflag_t`](#tcflag_t) | type |  |
| [`clockid_t`](#clockid_t) | type |  |
| [`timer_t`](#timer_t) | type |  |
| [`key_t`](#key_t) | type |  |
| [`id_t`](#id_t) | type |  |
| [`INT_MIN`](#int_min) | const |  |
| [`INT_MAX`](#int_max) | const |  |
| [`SIG_DFL`](#sig_dfl) | const |  |
| [`SIG_IGN`](#sig_ign) | const |  |
| [`SIG_ERR`](#sig_err) | const |  |
| [`DT_UNKNOWN`](#dt_unknown) | const |  |
| [`DT_FIFO`](#dt_fifo) | const |  |
| [`DT_CHR`](#dt_chr) | const |  |
| [`DT_DIR`](#dt_dir) | const |  |
| [`DT_BLK`](#dt_blk) | const |  |
| [`DT_REG`](#dt_reg) | const |  |
| [`DT_LNK`](#dt_lnk) | const |  |
| [`DT_SOCK`](#dt_sock) | const |  |
| [`FD_CLOEXEC`](#fd_cloexec) | const |  |
| [`USRQUOTA`](#usrquota) | const |  |
| [`GRPQUOTA`](#grpquota) | const |  |
| [`SIGIOT`](#sigiot) | const |  |
| [`S_ISUID`](#s_isuid) | const |  |
| [`S_ISGID`](#s_isgid) | const |  |
| [`S_ISVTX`](#s_isvtx) | const |  |
| [`IF_NAMESIZE`](#if_namesize) | const |  |
| [`IFNAMSIZ`](#ifnamsiz) | const |  |
| [`LOG_EMERG`](#log_emerg) | const |  |
| [`LOG_ALERT`](#log_alert) | const |  |
| [`LOG_CRIT`](#log_crit) | const |  |
| [`LOG_ERR`](#log_err) | const |  |
| [`LOG_WARNING`](#log_warning) | const |  |
| [`LOG_NOTICE`](#log_notice) | const |  |
| [`LOG_INFO`](#log_info) | const |  |
| [`LOG_DEBUG`](#log_debug) | const |  |
| [`LOG_KERN`](#log_kern) | const |  |
| [`LOG_USER`](#log_user) | const |  |
| [`LOG_MAIL`](#log_mail) | const |  |
| [`LOG_DAEMON`](#log_daemon) | const |  |
| [`LOG_AUTH`](#log_auth) | const |  |
| [`LOG_SYSLOG`](#log_syslog) | const |  |
| [`LOG_LPR`](#log_lpr) | const |  |
| [`LOG_NEWS`](#log_news) | const |  |
| [`LOG_UUCP`](#log_uucp) | const |  |
| [`LOG_LOCAL0`](#log_local0) | const |  |
| [`LOG_LOCAL1`](#log_local1) | const |  |
| [`LOG_LOCAL2`](#log_local2) | const |  |
| [`LOG_LOCAL3`](#log_local3) | const |  |
| [`LOG_LOCAL4`](#log_local4) | const |  |
| [`LOG_LOCAL5`](#log_local5) | const |  |
| [`LOG_LOCAL6`](#log_local6) | const |  |
| [`LOG_LOCAL7`](#log_local7) | const |  |
| [`LOG_PID`](#log_pid) | const |  |
| [`LOG_CONS`](#log_cons) | const |  |
| [`LOG_ODELAY`](#log_odelay) | const |  |
| [`LOG_NDELAY`](#log_ndelay) | const |  |
| [`LOG_NOWAIT`](#log_nowait) | const |  |
| [`LOG_PRIMASK`](#log_primask) | const |  |
| [`LOG_FACMASK`](#log_facmask) | const |  |
| [`PRIO_MIN`](#prio_min) | const |  |
| [`PRIO_MAX`](#prio_max) | const |  |
| [`IPPROTO_ICMP`](#ipproto_icmp) | const |  |
| [`IPPROTO_ICMPV6`](#ipproto_icmpv6) | const |  |
| [`IPPROTO_TCP`](#ipproto_tcp) | const |  |
| [`IPPROTO_UDP`](#ipproto_udp) | const |  |
| [`IPPROTO_IP`](#ipproto_ip) | const |  |
| [`IPPROTO_IPV6`](#ipproto_ipv6) | const |  |
| [`INADDR_LOOPBACK`](#inaddr_loopback) | const |  |
| [`INADDR_ANY`](#inaddr_any) | const |  |
| [`INADDR_BROADCAST`](#inaddr_broadcast) | const |  |
| [`INADDR_NONE`](#inaddr_none) | const |  |
| [`IN6ADDR_LOOPBACK_INIT`](#in6addr_loopback_init) | const |  |
| [`IN6ADDR_ANY_INIT`](#in6addr_any_init) | const |  |
| [`ARPOP_REQUEST`](#arpop_request) | const |  |
| [`ARPOP_REPLY`](#arpop_reply) | const |  |
| [`ATF_COM`](#atf_com) | const |  |
| [`ATF_PERM`](#atf_perm) | const |  |
| [`ATF_PUBL`](#atf_publ) | const |  |
| [`ATF_USETRAILERS`](#atf_usetrailers) | const |  |
| [`FNM_PERIOD`](#fnm_period) | const |  |
| [`FNM_NOMATCH`](#fnm_nomatch) | const |  |
| [`FNM_CASEFOLD`](#fnm_casefold) | const |  |
| [`FNM_PATHNAME`](#fnm_pathname) | const |  |
| [`FNM_NOESCAPE`](#fnm_noescape) | const |  |
| [`ULONG_SIZE`](#ulong_size) | const |  |
| [`EXIT_FAILURE`](#exit_failure) | const |  |
| [`EXIT_SUCCESS`](#exit_success) | const |  |
| [`RAND_MAX`](#rand_max) | const |  |
| [`EOF`](#eof) | const |  |
| [`SEEK_SET`](#seek_set) | const |  |
| [`SEEK_CUR`](#seek_cur) | const |  |
| [`SEEK_END`](#seek_end) | const |  |
| [`_IOFBF`](#_iofbf) | const |  |
| [`_IONBF`](#_ionbf) | const |  |
| [`_IOLBF`](#_iolbf) | const |  |
| [`F_DUPFD`](#f_dupfd) | const |  |
| [`F_GETFD`](#f_getfd) | const |  |
| [`F_SETFD`](#f_setfd) | const |  |
| [`F_GETFL`](#f_getfl) | const |  |
| [`F_SETFL`](#f_setfl) | const |  |
| [`F_SETLEASE`](#f_setlease) | const |  |
| [`F_GETLEASE`](#f_getlease) | const |  |
| [`F_NOTIFY`](#f_notify) | const |  |
| [`F_CANCELLK`](#f_cancellk) | const |  |
| [`F_DUPFD_CLOEXEC`](#f_dupfd_cloexec) | const |  |
| [`F_SETPIPE_SZ`](#f_setpipe_sz) | const |  |
| [`F_GETPIPE_SZ`](#f_getpipe_sz) | const |  |
| [`F_ADD_SEALS`](#f_add_seals) | const |  |
| [`F_GET_SEALS`](#f_get_seals) | const |  |
| [`F_SEAL_SEAL`](#f_seal_seal) | const |  |
| [`F_SEAL_SHRINK`](#f_seal_shrink) | const |  |
| [`F_SEAL_GROW`](#f_seal_grow) | const |  |
| [`F_SEAL_WRITE`](#f_seal_write) | const |  |
| [`SIGTRAP`](#sigtrap) | const |  |
| [`PTHREAD_CREATE_JOINABLE`](#pthread_create_joinable) | const |  |
| [`PTHREAD_CREATE_DETACHED`](#pthread_create_detached) | const |  |
| [`CLOCK_REALTIME`](#clock_realtime) | const |  |
| [`CLOCK_MONOTONIC`](#clock_monotonic) | const |  |
| [`CLOCK_PROCESS_CPUTIME_ID`](#clock_process_cputime_id) | const |  |
| [`CLOCK_THREAD_CPUTIME_ID`](#clock_thread_cputime_id) | const |  |
| [`CLOCK_MONOTONIC_RAW`](#clock_monotonic_raw) | const |  |
| [`CLOCK_REALTIME_COARSE`](#clock_realtime_coarse) | const |  |
| [`CLOCK_MONOTONIC_COARSE`](#clock_monotonic_coarse) | const |  |
| [`CLOCK_BOOTTIME`](#clock_boottime) | const |  |
| [`CLOCK_REALTIME_ALARM`](#clock_realtime_alarm) | const |  |
| [`CLOCK_BOOTTIME_ALARM`](#clock_boottime_alarm) | const |  |
| [`CLOCK_TAI`](#clock_tai) | const |  |
| [`TIMER_ABSTIME`](#timer_abstime) | const |  |
| [`RUSAGE_SELF`](#rusage_self) | const |  |
| [`O_RDONLY`](#o_rdonly) | const |  |
| [`O_WRONLY`](#o_wronly) | const |  |
| [`O_RDWR`](#o_rdwr) | const |  |
| [`SOCK_CLOEXEC`](#sock_cloexec) | const |  |
| [`S_IFIFO`](#s_ififo) | const |  |
| [`S_IFCHR`](#s_ifchr) | const |  |
| [`S_IFBLK`](#s_ifblk) | const |  |
| [`S_IFDIR`](#s_ifdir) | const |  |
| [`S_IFREG`](#s_ifreg) | const |  |
| [`S_IFLNK`](#s_iflnk) | const |  |
| [`S_IFSOCK`](#s_ifsock) | const |  |
| [`S_IFMT`](#s_ifmt) | const |  |
| [`S_IRWXU`](#s_irwxu) | const |  |
| [`S_IXUSR`](#s_ixusr) | const |  |
| [`S_IWUSR`](#s_iwusr) | const |  |
| [`S_IRUSR`](#s_irusr) | const |  |
| [`S_IRWXG`](#s_irwxg) | const |  |
| [`S_IXGRP`](#s_ixgrp) | const |  |
| [`S_IWGRP`](#s_iwgrp) | const |  |
| [`S_IRGRP`](#s_irgrp) | const |  |
| [`S_IRWXO`](#s_irwxo) | const |  |
| [`S_IXOTH`](#s_ixoth) | const |  |
| [`S_IWOTH`](#s_iwoth) | const |  |
| [`S_IROTH`](#s_iroth) | const |  |
| [`F_OK`](#f_ok) | const |  |
| [`R_OK`](#r_ok) | const |  |
| [`W_OK`](#w_ok) | const |  |
| [`X_OK`](#x_ok) | const |  |
| [`SIGHUP`](#sighup) | const |  |
| [`SIGINT`](#sigint) | const |  |
| [`SIGQUIT`](#sigquit) | const |  |
| [`SIGILL`](#sigill) | const |  |
| [`SIGABRT`](#sigabrt) | const |  |
| [`SIGFPE`](#sigfpe) | const |  |
| [`SIGKILL`](#sigkill) | const |  |
| [`SIGSEGV`](#sigsegv) | const |  |
| [`SIGPIPE`](#sigpipe) | const |  |
| [`SIGALRM`](#sigalrm) | const |  |
| [`SIGTERM`](#sigterm) | const |  |
| [`PROT_NONE`](#prot_none) | const |  |
| [`PROT_READ`](#prot_read) | const |  |
| [`PROT_WRITE`](#prot_write) | const |  |
| [`PROT_EXEC`](#prot_exec) | const |  |
| [`XATTR_CREATE`](#xattr_create) | const |  |
| [`XATTR_REPLACE`](#xattr_replace) | const |  |
| [`RLIM64_INFINITY`](#rlim64_infinity) | const |  |
| [`LC_CTYPE`](#lc_ctype) | const |  |
| [`LC_NUMERIC`](#lc_numeric) | const |  |
| [`LC_TIME`](#lc_time) | const |  |
| [`LC_COLLATE`](#lc_collate) | const |  |
| [`LC_MONETARY`](#lc_monetary) | const |  |
| [`LC_MESSAGES`](#lc_messages) | const |  |
| [`LC_ALL`](#lc_all) | const |  |
| [`LC_CTYPE_MASK`](#lc_ctype_mask) | const |  |
| [`LC_NUMERIC_MASK`](#lc_numeric_mask) | const |  |
| [`LC_TIME_MASK`](#lc_time_mask) | const |  |
| [`LC_COLLATE_MASK`](#lc_collate_mask) | const |  |
| [`LC_MONETARY_MASK`](#lc_monetary_mask) | const |  |
| [`LC_MESSAGES_MASK`](#lc_messages_mask) | const |  |
| [`MAP_FILE`](#map_file) | const |  |
| [`MAP_SHARED`](#map_shared) | const |  |
| [`MAP_PRIVATE`](#map_private) | const |  |
| [`MAP_FIXED`](#map_fixed) | const |  |
| [`MAP_FAILED`](#map_failed) | const |  |
| [`MS_ASYNC`](#ms_async) | const |  |
| [`MS_INVALIDATE`](#ms_invalidate) | const |  |
| [`MS_SYNC`](#ms_sync) | const |  |
| [`MS_RDONLY`](#ms_rdonly) | const |  |
| [`MS_NOSUID`](#ms_nosuid) | const |  |
| [`MS_NODEV`](#ms_nodev) | const |  |
| [`MS_NOEXEC`](#ms_noexec) | const |  |
| [`MS_SYNCHRONOUS`](#ms_synchronous) | const |  |
| [`MS_REMOUNT`](#ms_remount) | const |  |
| [`MS_MANDLOCK`](#ms_mandlock) | const |  |
| [`MS_DIRSYNC`](#ms_dirsync) | const |  |
| [`MS_NOSYMFOLLOW`](#ms_nosymfollow) | const |  |
| [`MS_NOATIME`](#ms_noatime) | const |  |
| [`MS_NODIRATIME`](#ms_nodiratime) | const |  |
| [`MS_BIND`](#ms_bind) | const |  |
| [`MS_MOVE`](#ms_move) | const |  |
| [`MS_REC`](#ms_rec) | const |  |
| [`MS_SILENT`](#ms_silent) | const |  |
| [`MS_POSIXACL`](#ms_posixacl) | const |  |
| [`MS_UNBINDABLE`](#ms_unbindable) | const |  |
| [`MS_PRIVATE`](#ms_private) | const |  |
| [`MS_SLAVE`](#ms_slave) | const |  |
| [`MS_SHARED`](#ms_shared) | const |  |
| [`MS_RELATIME`](#ms_relatime) | const |  |
| [`MS_KERNMOUNT`](#ms_kernmount) | const |  |
| [`MS_I_VERSION`](#ms_i_version) | const |  |
| [`MS_STRICTATIME`](#ms_strictatime) | const |  |
| [`MS_LAZYTIME`](#ms_lazytime) | const |  |
| [`MS_ACTIVE`](#ms_active) | const |  |
| [`MS_MGC_VAL`](#ms_mgc_val) | const |  |
| [`MS_MGC_MSK`](#ms_mgc_msk) | const |  |
| [`SCM_RIGHTS`](#scm_rights) | const |  |
| [`SCM_CREDENTIALS`](#scm_credentials) | const |  |
| [`PROT_GROWSDOWN`](#prot_growsdown) | const |  |
| [`PROT_GROWSUP`](#prot_growsup) | const |  |
| [`MAP_TYPE`](#map_type) | const |  |
| [`MADV_NORMAL`](#madv_normal) | const |  |
| [`MADV_RANDOM`](#madv_random) | const |  |
| [`MADV_SEQUENTIAL`](#madv_sequential) | const |  |
| [`MADV_WILLNEED`](#madv_willneed) | const |  |
| [`MADV_DONTNEED`](#madv_dontneed) | const |  |
| [`MADV_FREE`](#madv_free) | const |  |
| [`MADV_REMOVE`](#madv_remove) | const |  |
| [`MADV_DONTFORK`](#madv_dontfork) | const |  |
| [`MADV_DOFORK`](#madv_dofork) | const |  |
| [`MADV_MERGEABLE`](#madv_mergeable) | const |  |
| [`MADV_UNMERGEABLE`](#madv_unmergeable) | const |  |
| [`MADV_HUGEPAGE`](#madv_hugepage) | const |  |
| [`MADV_NOHUGEPAGE`](#madv_nohugepage) | const |  |
| [`MADV_DONTDUMP`](#madv_dontdump) | const |  |
| [`MADV_DODUMP`](#madv_dodump) | const |  |
| [`MADV_WIPEONFORK`](#madv_wipeonfork) | const |  |
| [`MADV_KEEPONFORK`](#madv_keeponfork) | const |  |
| [`MADV_COLD`](#madv_cold) | const |  |
| [`MADV_PAGEOUT`](#madv_pageout) | const |  |
| [`MADV_HWPOISON`](#madv_hwpoison) | const |  |
| [`MADV_POPULATE_READ`](#madv_populate_read) | const |  |
| [`MADV_POPULATE_WRITE`](#madv_populate_write) | const |  |
| [`MADV_DONTNEED_LOCKED`](#madv_dontneed_locked) | const |  |
| [`IFF_UP`](#iff_up) | const |  |
| [`IFF_BROADCAST`](#iff_broadcast) | const |  |
| [`IFF_DEBUG`](#iff_debug) | const |  |
| [`IFF_LOOPBACK`](#iff_loopback) | const |  |
| [`IFF_POINTOPOINT`](#iff_pointopoint) | const |  |
| [`IFF_NOTRAILERS`](#iff_notrailers) | const |  |
| [`IFF_RUNNING`](#iff_running) | const |  |
| [`IFF_NOARP`](#iff_noarp) | const |  |
| [`IFF_PROMISC`](#iff_promisc) | const |  |
| [`IFF_ALLMULTI`](#iff_allmulti) | const |  |
| [`IFF_MASTER`](#iff_master) | const |  |
| [`IFF_SLAVE`](#iff_slave) | const |  |
| [`IFF_MULTICAST`](#iff_multicast) | const |  |
| [`IFF_PORTSEL`](#iff_portsel) | const |  |
| [`IFF_AUTOMEDIA`](#iff_automedia) | const |  |
| [`IFF_DYNAMIC`](#iff_dynamic) | const |  |
| [`SOL_IP`](#sol_ip) | const |  |
| [`SOL_TCP`](#sol_tcp) | const |  |
| [`SOL_UDP`](#sol_udp) | const |  |
| [`SOL_IPV6`](#sol_ipv6) | const |  |
| [`SOL_ICMPV6`](#sol_icmpv6) | const |  |
| [`SOL_RAW`](#sol_raw) | const |  |
| [`SOL_DECNET`](#sol_decnet) | const |  |
| [`SOL_X25`](#sol_x25) | const |  |
| [`SOL_PACKET`](#sol_packet) | const |  |
| [`SOL_ATM`](#sol_atm) | const |  |
| [`SOL_AAL`](#sol_aal) | const |  |
| [`SOL_IRDA`](#sol_irda) | const |  |
| [`SOL_NETBEUI`](#sol_netbeui) | const |  |
| [`SOL_LLC`](#sol_llc) | const |  |
| [`SOL_DCCP`](#sol_dccp) | const |  |
| [`SOL_NETLINK`](#sol_netlink) | const |  |
| [`SOL_TIPC`](#sol_tipc) | const |  |
| [`SOL_BLUETOOTH`](#sol_bluetooth) | const |  |
| [`SOL_ALG`](#sol_alg) | const |  |
| [`AF_UNSPEC`](#af_unspec) | const |  |
| [`AF_UNIX`](#af_unix) | const |  |
| [`AF_LOCAL`](#af_local) | const |  |
| [`AF_INET`](#af_inet) | const |  |
| [`AF_AX25`](#af_ax25) | const |  |
| [`AF_IPX`](#af_ipx) | const |  |
| [`AF_APPLETALK`](#af_appletalk) | const |  |
| [`AF_NETROM`](#af_netrom) | const |  |
| [`AF_BRIDGE`](#af_bridge) | const |  |
| [`AF_ATMPVC`](#af_atmpvc) | const |  |
| [`AF_X25`](#af_x25) | const |  |
| [`AF_INET6`](#af_inet6) | const |  |
| [`AF_ROSE`](#af_rose) | const |  |
| [`AF_DECnet`](#af_decnet) | const |  |
| [`AF_NETBEUI`](#af_netbeui) | const |  |
| [`AF_SECURITY`](#af_security) | const |  |
| [`AF_KEY`](#af_key) | const |  |
| [`AF_NETLINK`](#af_netlink) | const |  |
| [`AF_ROUTE`](#af_route) | const |  |
| [`AF_PACKET`](#af_packet) | const |  |
| [`AF_ASH`](#af_ash) | const |  |
| [`AF_ECONET`](#af_econet) | const |  |
| [`AF_ATMSVC`](#af_atmsvc) | const |  |
| [`AF_RDS`](#af_rds) | const |  |
| [`AF_SNA`](#af_sna) | const |  |
| [`AF_IRDA`](#af_irda) | const |  |
| [`AF_PPPOX`](#af_pppox) | const |  |
| [`AF_WANPIPE`](#af_wanpipe) | const |  |
| [`AF_LLC`](#af_llc) | const |  |
| [`AF_CAN`](#af_can) | const |  |
| [`AF_TIPC`](#af_tipc) | const |  |
| [`AF_BLUETOOTH`](#af_bluetooth) | const |  |
| [`AF_IUCV`](#af_iucv) | const |  |
| [`AF_RXRPC`](#af_rxrpc) | const |  |
| [`AF_ISDN`](#af_isdn) | const |  |
| [`AF_PHONET`](#af_phonet) | const |  |
| [`AF_IEEE802154`](#af_ieee802154) | const |  |
| [`AF_CAIF`](#af_caif) | const |  |
| [`AF_ALG`](#af_alg) | const |  |
| [`PF_UNSPEC`](#pf_unspec) | const |  |
| [`PF_UNIX`](#pf_unix) | const |  |
| [`PF_LOCAL`](#pf_local) | const |  |
| [`PF_INET`](#pf_inet) | const |  |
| [`PF_AX25`](#pf_ax25) | const |  |
| [`PF_IPX`](#pf_ipx) | const |  |
| [`PF_APPLETALK`](#pf_appletalk) | const |  |
| [`PF_NETROM`](#pf_netrom) | const |  |
| [`PF_BRIDGE`](#pf_bridge) | const |  |
| [`PF_ATMPVC`](#pf_atmpvc) | const |  |
| [`PF_X25`](#pf_x25) | const |  |
| [`PF_INET6`](#pf_inet6) | const |  |
| [`PF_ROSE`](#pf_rose) | const |  |
| [`PF_DECnet`](#pf_decnet) | const |  |
| [`PF_NETBEUI`](#pf_netbeui) | const |  |
| [`PF_SECURITY`](#pf_security) | const |  |
| [`PF_KEY`](#pf_key) | const |  |
| [`PF_NETLINK`](#pf_netlink) | const |  |
| [`PF_ROUTE`](#pf_route) | const |  |
| [`PF_PACKET`](#pf_packet) | const |  |
| [`PF_ASH`](#pf_ash) | const |  |
| [`PF_ECONET`](#pf_econet) | const |  |
| [`PF_ATMSVC`](#pf_atmsvc) | const |  |
| [`PF_RDS`](#pf_rds) | const |  |
| [`PF_SNA`](#pf_sna) | const |  |
| [`PF_IRDA`](#pf_irda) | const |  |
| [`PF_PPPOX`](#pf_pppox) | const |  |
| [`PF_WANPIPE`](#pf_wanpipe) | const |  |
| [`PF_LLC`](#pf_llc) | const |  |
| [`PF_CAN`](#pf_can) | const |  |
| [`PF_TIPC`](#pf_tipc) | const |  |
| [`PF_BLUETOOTH`](#pf_bluetooth) | const |  |
| [`PF_IUCV`](#pf_iucv) | const |  |
| [`PF_RXRPC`](#pf_rxrpc) | const |  |
| [`PF_ISDN`](#pf_isdn) | const |  |
| [`PF_PHONET`](#pf_phonet) | const |  |
| [`PF_IEEE802154`](#pf_ieee802154) | const |  |
| [`PF_CAIF`](#pf_caif) | const |  |
| [`PF_ALG`](#pf_alg) | const |  |
| [`MSG_OOB`](#msg_oob) | const |  |
| [`MSG_PEEK`](#msg_peek) | const |  |
| [`MSG_DONTROUTE`](#msg_dontroute) | const |  |
| [`MSG_CTRUNC`](#msg_ctrunc) | const |  |
| [`MSG_TRUNC`](#msg_trunc) | const |  |
| [`MSG_DONTWAIT`](#msg_dontwait) | const |  |
| [`MSG_EOR`](#msg_eor) | const |  |
| [`MSG_WAITALL`](#msg_waitall) | const |  |
| [`MSG_FIN`](#msg_fin) | const |  |
| [`MSG_SYN`](#msg_syn) | const |  |
| [`MSG_CONFIRM`](#msg_confirm) | const |  |
| [`MSG_RST`](#msg_rst) | const |  |
| [`MSG_ERRQUEUE`](#msg_errqueue) | const |  |
| [`MSG_NOSIGNAL`](#msg_nosignal) | const |  |
| [`MSG_MORE`](#msg_more) | const |  |
| [`MSG_WAITFORONE`](#msg_waitforone) | const |  |
| [`MSG_FASTOPEN`](#msg_fastopen) | const |  |
| [`MSG_CMSG_CLOEXEC`](#msg_cmsg_cloexec) | const |  |
| [`SCM_TIMESTAMP`](#scm_timestamp) | const |  |
| [`SOCK_RAW`](#sock_raw) | const |  |
| [`SOCK_RDM`](#sock_rdm) | const |  |
| [`IP_TOS`](#ip_tos) | const |  |
| [`IP_TTL`](#ip_ttl) | const |  |
| [`IP_HDRINCL`](#ip_hdrincl) | const |  |
| [`IP_OPTIONS`](#ip_options) | const |  |
| [`IP_ROUTER_ALERT`](#ip_router_alert) | const |  |
| [`IP_RECVOPTS`](#ip_recvopts) | const |  |
| [`IP_RETOPTS`](#ip_retopts) | const |  |
| [`IP_PKTINFO`](#ip_pktinfo) | const |  |
| [`IP_PKTOPTIONS`](#ip_pktoptions) | const |  |
| [`IP_MTU_DISCOVER`](#ip_mtu_discover) | const |  |
| [`IP_RECVERR`](#ip_recverr) | const |  |
| [`IP_RECVTTL`](#ip_recvttl) | const |  |
| [`IP_RECVTOS`](#ip_recvtos) | const |  |
| [`IP_MTU`](#ip_mtu) | const |  |
| [`IP_FREEBIND`](#ip_freebind) | const |  |
| [`IP_IPSEC_POLICY`](#ip_ipsec_policy) | const |  |
| [`IP_XFRM_POLICY`](#ip_xfrm_policy) | const |  |
| [`IP_PASSSEC`](#ip_passsec) | const |  |
| [`IP_TRANSPARENT`](#ip_transparent) | const |  |
| [`IP_ORIGDSTADDR`](#ip_origdstaddr) | const |  |
| [`IP_RECVORIGDSTADDR`](#ip_recvorigdstaddr) | const |  |
| [`IP_MINTTL`](#ip_minttl) | const |  |
| [`IP_NODEFRAG`](#ip_nodefrag) | const |  |
| [`IP_CHECKSUM`](#ip_checksum) | const |  |
| [`IP_BIND_ADDRESS_NO_PORT`](#ip_bind_address_no_port) | const |  |
| [`IP_MULTICAST_IF`](#ip_multicast_if) | const |  |
| [`IP_MULTICAST_TTL`](#ip_multicast_ttl) | const |  |
| [`IP_MULTICAST_LOOP`](#ip_multicast_loop) | const |  |
| [`IP_ADD_MEMBERSHIP`](#ip_add_membership) | const |  |
| [`IP_DROP_MEMBERSHIP`](#ip_drop_membership) | const |  |
| [`IP_UNBLOCK_SOURCE`](#ip_unblock_source) | const |  |
| [`IP_BLOCK_SOURCE`](#ip_block_source) | const |  |
| [`IP_ADD_SOURCE_MEMBERSHIP`](#ip_add_source_membership) | const |  |
| [`IP_DROP_SOURCE_MEMBERSHIP`](#ip_drop_source_membership) | const |  |
| [`IP_MSFILTER`](#ip_msfilter) | const |  |
| [`IP_MULTICAST_ALL`](#ip_multicast_all) | const |  |
| [`IP_UNICAST_IF`](#ip_unicast_if) | const |  |
| [`IP_DEFAULT_MULTICAST_TTL`](#ip_default_multicast_ttl) | const |  |
| [`IP_DEFAULT_MULTICAST_LOOP`](#ip_default_multicast_loop) | const |  |
| [`IP_PMTUDISC_DONT`](#ip_pmtudisc_dont) | const |  |
| [`IP_PMTUDISC_WANT`](#ip_pmtudisc_want) | const |  |
| [`IP_PMTUDISC_DO`](#ip_pmtudisc_do) | const |  |
| [`IP_PMTUDISC_PROBE`](#ip_pmtudisc_probe) | const |  |
| [`IP_PMTUDISC_INTERFACE`](#ip_pmtudisc_interface) | const |  |
| [`IP_PMTUDISC_OMIT`](#ip_pmtudisc_omit) | const |  |
| [`IPPROTO_HOPOPTS`](#ipproto_hopopts) | const | Hop-by-hop option header |
| [`IPPROTO_IGMP`](#ipproto_igmp) | const | group mgmt protocol |
| [`IPPROTO_IPIP`](#ipproto_ipip) | const | for compatibility |
| [`IPPROTO_EGP`](#ipproto_egp) | const | exterior gateway protocol |
| [`IPPROTO_PUP`](#ipproto_pup) | const | pup |
| [`IPPROTO_IDP`](#ipproto_idp) | const | xns idp |
| [`IPPROTO_TP`](#ipproto_tp) | const | tp-4 w/ class negotiation |
| [`IPPROTO_DCCP`](#ipproto_dccp) | const | DCCP |
| [`IPPROTO_ROUTING`](#ipproto_routing) | const | IP6 routing header |
| [`IPPROTO_FRAGMENT`](#ipproto_fragment) | const | IP6 fragmentation header |
| [`IPPROTO_RSVP`](#ipproto_rsvp) | const | resource reservation |
| [`IPPROTO_GRE`](#ipproto_gre) | const | General Routing Encap. |
| [`IPPROTO_ESP`](#ipproto_esp) | const | IP6 Encap Sec. |
| [`IPPROTO_AH`](#ipproto_ah) | const | IP6 Auth Header |
| [`IPPROTO_NONE`](#ipproto_none) | const | IP6 no next header |
| [`IPPROTO_DSTOPTS`](#ipproto_dstopts) | const | IP6 destination option |
| [`IPPROTO_MTP`](#ipproto_mtp) | const |  |
| [`IPPROTO_ENCAP`](#ipproto_encap) | const | encapsulation header |
| [`IPPROTO_PIM`](#ipproto_pim) | const | Protocol indep. |
| [`IPPROTO_COMP`](#ipproto_comp) | const | IP Payload Comp. |
| [`IPPROTO_SCTP`](#ipproto_sctp) | const | SCTP |
| [`IPPROTO_MH`](#ipproto_mh) | const |  |
| [`IPPROTO_UDPLITE`](#ipproto_udplite) | const |  |
| [`IPPROTO_RAW`](#ipproto_raw) | const | raw IP packet |
| [`IPPROTO_BEETPH`](#ipproto_beetph) | const |  |
| [`IPPROTO_MPLS`](#ipproto_mpls) | const |  |
| [`IPPROTO_MPTCP`](#ipproto_mptcp) | const | Multipath TCP |
| [`IPPROTO_ETHERNET`](#ipproto_ethernet) | const | Ethernet-within-IPv6 encapsulation. |
| [`MCAST_EXCLUDE`](#mcast_exclude) | const |  |
| [`MCAST_INCLUDE`](#mcast_include) | const |  |
| [`MCAST_JOIN_GROUP`](#mcast_join_group) | const |  |
| [`MCAST_BLOCK_SOURCE`](#mcast_block_source) | const |  |
| [`MCAST_UNBLOCK_SOURCE`](#mcast_unblock_source) | const |  |
| [`MCAST_LEAVE_GROUP`](#mcast_leave_group) | const |  |
| [`MCAST_JOIN_SOURCE_GROUP`](#mcast_join_source_group) | const |  |
| [`MCAST_LEAVE_SOURCE_GROUP`](#mcast_leave_source_group) | const |  |
| [`MCAST_MSFILTER`](#mcast_msfilter) | const |  |
| [`IPV6_ADDRFORM`](#ipv6_addrform) | const |  |
| [`IPV6_2292PKTINFO`](#ipv6_2292pktinfo) | const |  |
| [`IPV6_2292HOPOPTS`](#ipv6_2292hopopts) | const |  |
| [`IPV6_2292DSTOPTS`](#ipv6_2292dstopts) | const |  |
| [`IPV6_2292RTHDR`](#ipv6_2292rthdr) | const |  |
| [`IPV6_2292PKTOPTIONS`](#ipv6_2292pktoptions) | const |  |
| [`IPV6_CHECKSUM`](#ipv6_checksum) | const |  |
| [`IPV6_2292HOPLIMIT`](#ipv6_2292hoplimit) | const |  |
| [`IPV6_NEXTHOP`](#ipv6_nexthop) | const |  |
| [`IPV6_AUTHHDR`](#ipv6_authhdr) | const |  |
| [`IPV6_UNICAST_HOPS`](#ipv6_unicast_hops) | const |  |
| [`IPV6_MULTICAST_IF`](#ipv6_multicast_if) | const |  |
| [`IPV6_MULTICAST_HOPS`](#ipv6_multicast_hops) | const |  |
| [`IPV6_MULTICAST_LOOP`](#ipv6_multicast_loop) | const |  |
| [`IPV6_ADD_MEMBERSHIP`](#ipv6_add_membership) | const |  |
| [`IPV6_DROP_MEMBERSHIP`](#ipv6_drop_membership) | const |  |
| [`IPV6_ROUTER_ALERT`](#ipv6_router_alert) | const |  |
| [`IPV6_MTU_DISCOVER`](#ipv6_mtu_discover) | const |  |
| [`IPV6_MTU`](#ipv6_mtu) | const |  |
| [`IPV6_RECVERR`](#ipv6_recverr) | const |  |
| [`IPV6_V6ONLY`](#ipv6_v6only) | const |  |
| [`IPV6_JOIN_ANYCAST`](#ipv6_join_anycast) | const |  |
| [`IPV6_LEAVE_ANYCAST`](#ipv6_leave_anycast) | const |  |
| [`IPV6_IPSEC_POLICY`](#ipv6_ipsec_policy) | const |  |
| [`IPV6_XFRM_POLICY`](#ipv6_xfrm_policy) | const |  |
| [`IPV6_HDRINCL`](#ipv6_hdrincl) | const |  |
| [`IPV6_RECVPKTINFO`](#ipv6_recvpktinfo) | const |  |
| [`IPV6_PKTINFO`](#ipv6_pktinfo) | const |  |
| [`IPV6_RECVHOPLIMIT`](#ipv6_recvhoplimit) | const |  |
| [`IPV6_HOPLIMIT`](#ipv6_hoplimit) | const |  |
| [`IPV6_RECVHOPOPTS`](#ipv6_recvhopopts) | const |  |
| [`IPV6_HOPOPTS`](#ipv6_hopopts) | const |  |
| [`IPV6_RTHDRDSTOPTS`](#ipv6_rthdrdstopts) | const |  |
| [`IPV6_RECVRTHDR`](#ipv6_recvrthdr) | const |  |
| [`IPV6_RTHDR`](#ipv6_rthdr) | const |  |
| [`IPV6_RECVDSTOPTS`](#ipv6_recvdstopts) | const |  |
| [`IPV6_DSTOPTS`](#ipv6_dstopts) | const |  |
| [`IPV6_RECVPATHMTU`](#ipv6_recvpathmtu) | const |  |
| [`IPV6_PATHMTU`](#ipv6_pathmtu) | const |  |
| [`IPV6_DONTFRAG`](#ipv6_dontfrag) | const |  |
| [`IPV6_RECVTCLASS`](#ipv6_recvtclass) | const |  |
| [`IPV6_TCLASS`](#ipv6_tclass) | const |  |
| [`IPV6_AUTOFLOWLABEL`](#ipv6_autoflowlabel) | const |  |
| [`IPV6_ADDR_PREFERENCES`](#ipv6_addr_preferences) | const |  |
| [`IPV6_MINHOPCOUNT`](#ipv6_minhopcount) | const |  |
| [`IPV6_ORIGDSTADDR`](#ipv6_origdstaddr) | const |  |
| [`IPV6_RECVORIGDSTADDR`](#ipv6_recvorigdstaddr) | const |  |
| [`IPV6_TRANSPARENT`](#ipv6_transparent) | const |  |
| [`IPV6_UNICAST_IF`](#ipv6_unicast_if) | const |  |
| [`IPV6_PREFER_SRC_TMP`](#ipv6_prefer_src_tmp) | const |  |
| [`IPV6_PREFER_SRC_PUBLIC`](#ipv6_prefer_src_public) | const |  |
| [`IPV6_PREFER_SRC_PUBTMP_DEFAULT`](#ipv6_prefer_src_pubtmp_default) | const |  |
| [`IPV6_PREFER_SRC_COA`](#ipv6_prefer_src_coa) | const |  |
| [`IPV6_PREFER_SRC_HOME`](#ipv6_prefer_src_home) | const |  |
| [`IPV6_PREFER_SRC_CGA`](#ipv6_prefer_src_cga) | const |  |
| [`IPV6_PREFER_SRC_NONCGA`](#ipv6_prefer_src_noncga) | const |  |
| [`IPV6_PMTUDISC_DONT`](#ipv6_pmtudisc_dont) | const |  |
| [`IPV6_PMTUDISC_WANT`](#ipv6_pmtudisc_want) | const |  |
| [`IPV6_PMTUDISC_DO`](#ipv6_pmtudisc_do) | const |  |
| [`IPV6_PMTUDISC_PROBE`](#ipv6_pmtudisc_probe) | const |  |
| [`IPV6_PMTUDISC_INTERFACE`](#ipv6_pmtudisc_interface) | const |  |
| [`IPV6_PMTUDISC_OMIT`](#ipv6_pmtudisc_omit) | const |  |
| [`TCP_NODELAY`](#tcp_nodelay) | const |  |
| [`TCP_MAXSEG`](#tcp_maxseg) | const |  |
| [`TCP_CORK`](#tcp_cork) | const |  |
| [`TCP_KEEPIDLE`](#tcp_keepidle) | const |  |
| [`TCP_KEEPINTVL`](#tcp_keepintvl) | const |  |
| [`TCP_KEEPCNT`](#tcp_keepcnt) | const |  |
| [`TCP_SYNCNT`](#tcp_syncnt) | const |  |
| [`TCP_LINGER2`](#tcp_linger2) | const |  |
| [`TCP_DEFER_ACCEPT`](#tcp_defer_accept) | const |  |
| [`TCP_WINDOW_CLAMP`](#tcp_window_clamp) | const |  |
| [`TCP_INFO`](#tcp_info) | const |  |
| [`TCP_QUICKACK`](#tcp_quickack) | const |  |
| [`TCP_CONGESTION`](#tcp_congestion) | const |  |
| [`TCP_MD5SIG`](#tcp_md5sig) | const |  |
| [`TCP_COOKIE_TRANSACTIONS`](#tcp_cookie_transactions) | const |  |
| [`TCP_THIN_LINEAR_TIMEOUTS`](#tcp_thin_linear_timeouts) | const |  |
| [`TCP_THIN_DUPACK`](#tcp_thin_dupack) | const |  |
| [`TCP_USER_TIMEOUT`](#tcp_user_timeout) | const |  |
| [`TCP_REPAIR`](#tcp_repair) | const |  |
| [`TCP_REPAIR_QUEUE`](#tcp_repair_queue) | const |  |
| [`TCP_QUEUE_SEQ`](#tcp_queue_seq) | const |  |
| [`TCP_REPAIR_OPTIONS`](#tcp_repair_options) | const |  |
| [`TCP_FASTOPEN`](#tcp_fastopen) | const |  |
| [`TCP_TIMESTAMP`](#tcp_timestamp) | const |  |
| [`TCP_NOTSENT_LOWAT`](#tcp_notsent_lowat) | const |  |
| [`TCP_CC_INFO`](#tcp_cc_info) | const |  |
| [`TCP_SAVE_SYN`](#tcp_save_syn) | const |  |
| [`TCP_SAVED_SYN`](#tcp_saved_syn) | const |  |
| [`TCP_REPAIR_WINDOW`](#tcp_repair_window) | const |  |
| [`TCP_FASTOPEN_CONNECT`](#tcp_fastopen_connect) | const |  |
| [`TCP_ULP`](#tcp_ulp) | const |  |
| [`TCP_MD5SIG_EXT`](#tcp_md5sig_ext) | const |  |
| [`TCP_FASTOPEN_KEY`](#tcp_fastopen_key) | const |  |
| [`TCP_FASTOPEN_NO_COOKIE`](#tcp_fastopen_no_cookie) | const |  |
| [`TCP_ZEROCOPY_RECEIVE`](#tcp_zerocopy_receive) | const |  |
| [`TCP_INQ`](#tcp_inq) | const |  |
| [`TCP_CM_INQ`](#tcp_cm_inq) | const |  |
| [`TCP_MD5SIG_MAXKEYLEN`](#tcp_md5sig_maxkeylen) | const |  |
| [`SO_DEBUG`](#so_debug) | const |  |
| [`SHUT_RD`](#shut_rd) | const |  |
| [`SHUT_WR`](#shut_wr) | const |  |
| [`SHUT_RDWR`](#shut_rdwr) | const |  |
| [`LOCK_SH`](#lock_sh) | const |  |
| [`LOCK_EX`](#lock_ex) | const |  |
| [`LOCK_NB`](#lock_nb) | const |  |
| [`LOCK_UN`](#lock_un) | const |  |
| [`SS_ONSTACK`](#ss_onstack) | const |  |
| [`SS_DISABLE`](#ss_disable) | const |  |
| [`PATH_MAX`](#path_max) | const |  |
| [`UIO_MAXIOV`](#uio_maxiov) | const |  |
| [`FD_SETSIZE`](#fd_setsize) | const |  |
| [`EPOLLIN`](#epollin) | const |  |
| [`EPOLLPRI`](#epollpri) | const |  |
| [`EPOLLOUT`](#epollout) | const |  |
| [`EPOLLERR`](#epollerr) | const |  |
| [`EPOLLHUP`](#epollhup) | const |  |
| [`EPOLLRDNORM`](#epollrdnorm) | const |  |
| [`EPOLLRDBAND`](#epollrdband) | const |  |
| [`EPOLLWRNORM`](#epollwrnorm) | const |  |
| [`EPOLLWRBAND`](#epollwrband) | const |  |
| [`EPOLLMSG`](#epollmsg) | const |  |
| [`EPOLLRDHUP`](#epollrdhup) | const |  |
| [`EPOLLEXCLUSIVE`](#epollexclusive) | const |  |
| [`EPOLLWAKEUP`](#epollwakeup) | const |  |
| [`EPOLLONESHOT`](#epolloneshot) | const |  |
| [`EPOLLET`](#epollet) | const |  |
| [`EPOLL_CTL_ADD`](#epoll_ctl_add) | const |  |
| [`EPOLL_CTL_MOD`](#epoll_ctl_mod) | const |  |
| [`EPOLL_CTL_DEL`](#epoll_ctl_del) | const |  |
| [`MNT_FORCE`](#mnt_force) | const |  |
| [`MNT_DETACH`](#mnt_detach) | const |  |
| [`MNT_EXPIRE`](#mnt_expire) | const |  |
| [`UMOUNT_NOFOLLOW`](#umount_nofollow) | const |  |
| [`Q_GETFMT`](#q_getfmt) | const |  |
| [`Q_GETINFO`](#q_getinfo) | const |  |
| [`Q_SETINFO`](#q_setinfo) | const |  |
| [`QIF_BLIMITS`](#qif_blimits) | const |  |
| [`QIF_SPACE`](#qif_space) | const |  |
| [`QIF_ILIMITS`](#qif_ilimits) | const |  |
| [`QIF_INODES`](#qif_inodes) | const |  |
| [`QIF_BTIME`](#qif_btime) | const |  |
| [`QIF_ITIME`](#qif_itime) | const |  |
| [`QIF_LIMITS`](#qif_limits) | const |  |
| [`QIF_USAGE`](#qif_usage) | const |  |
| [`QIF_TIMES`](#qif_times) | const |  |
| [`QIF_ALL`](#qif_all) | const |  |
| [`Q_SYNC`](#q_sync) | const |  |
| [`Q_QUOTAON`](#q_quotaon) | const |  |
| [`Q_QUOTAOFF`](#q_quotaoff) | const |  |
| [`Q_GETQUOTA`](#q_getquota) | const |  |
| [`Q_SETQUOTA`](#q_setquota) | const |  |
| [`TCIOFF`](#tcioff) | const |  |
| [`TCION`](#tcion) | const |  |
| [`TCOOFF`](#tcooff) | const |  |
| [`TCOON`](#tcoon) | const |  |
| [`TCIFLUSH`](#tciflush) | const |  |
| [`TCOFLUSH`](#tcoflush) | const |  |
| [`TCIOFLUSH`](#tcioflush) | const |  |
| [`NL0`](#nl0) | const |  |
| [`NL1`](#nl1) | const |  |
| [`TAB0`](#tab0) | const |  |
| [`CR0`](#cr0) | const |  |
| [`FF0`](#ff0) | const |  |
| [`BS0`](#bs0) | const |  |
| [`VT0`](#vt0) | const |  |
| [`VERASE`](#verase) | const |  |
| [`VKILL`](#vkill) | const |  |
| [`VINTR`](#vintr) | const |  |
| [`VQUIT`](#vquit) | const |  |
| [`VLNEXT`](#vlnext) | const |  |
| [`IGNBRK`](#ignbrk) | const |  |
| [`BRKINT`](#brkint) | const |  |
| [`IGNPAR`](#ignpar) | const |  |
| [`PARMRK`](#parmrk) | const |  |
| [`INPCK`](#inpck) | const |  |
| [`ISTRIP`](#istrip) | const |  |
| [`INLCR`](#inlcr) | const |  |
| [`IGNCR`](#igncr) | const |  |
| [`ICRNL`](#icrnl) | const |  |
| [`IXANY`](#ixany) | const |  |
| [`IMAXBEL`](#imaxbel) | const |  |
| [`OPOST`](#opost) | const |  |
| [`CS5`](#cs5) | const |  |
| [`CRTSCTS`](#crtscts) | const |  |
| [`ECHO`](#echo) | const |  |
| [`OCRNL`](#ocrnl) | const |  |
| [`ONOCR`](#onocr) | const |  |
| [`ONLRET`](#onlret) | const |  |
| [`OFILL`](#ofill) | const |  |
| [`OFDEL`](#ofdel) | const |  |
| [`CLONE_VM`](#clone_vm) | const |  |
| [`CLONE_FS`](#clone_fs) | const |  |
| [`CLONE_FILES`](#clone_files) | const |  |
| [`CLONE_SIGHAND`](#clone_sighand) | const |  |
| [`CLONE_PTRACE`](#clone_ptrace) | const |  |
| [`CLONE_VFORK`](#clone_vfork) | const |  |
| [`CLONE_PARENT`](#clone_parent) | const |  |
| [`CLONE_THREAD`](#clone_thread) | const |  |
| [`CLONE_NEWNS`](#clone_newns) | const |  |
| [`CLONE_SYSVSEM`](#clone_sysvsem) | const |  |
| [`CLONE_SETTLS`](#clone_settls) | const |  |
| [`CLONE_PARENT_SETTID`](#clone_parent_settid) | const |  |
| [`CLONE_CHILD_CLEARTID`](#clone_child_cleartid) | const |  |
| [`CLONE_DETACHED`](#clone_detached) | const |  |
| [`CLONE_UNTRACED`](#clone_untraced) | const |  |
| [`CLONE_CHILD_SETTID`](#clone_child_settid) | const |  |
| [`CLONE_NEWCGROUP`](#clone_newcgroup) | const |  |
| [`CLONE_NEWUTS`](#clone_newuts) | const |  |
| [`CLONE_NEWIPC`](#clone_newipc) | const |  |
| [`CLONE_NEWUSER`](#clone_newuser) | const |  |
| [`CLONE_NEWPID`](#clone_newpid) | const |  |
| [`CLONE_NEWNET`](#clone_newnet) | const |  |
| [`CLONE_IO`](#clone_io) | const |  |
| [`WNOHANG`](#wnohang) | const |  |
| [`WUNTRACED`](#wuntraced) | const |  |
| [`WSTOPPED`](#wstopped) | const |  |
| [`WEXITED`](#wexited) | const |  |
| [`WCONTINUED`](#wcontinued) | const |  |
| [`WNOWAIT`](#wnowait) | const |  |
| [`ADDR_NO_RANDOMIZE`](#addr_no_randomize) | const |  |
| [`MMAP_PAGE_ZERO`](#mmap_page_zero) | const |  |
| [`ADDR_COMPAT_LAYOUT`](#addr_compat_layout) | const |  |
| [`READ_IMPLIES_EXEC`](#read_implies_exec) | const |  |
| [`ADDR_LIMIT_32BIT`](#addr_limit_32bit) | const |  |
| [`SHORT_INODE`](#short_inode) | const |  |
| [`WHOLE_SECONDS`](#whole_seconds) | const |  |
| [`STICKY_TIMEOUTS`](#sticky_timeouts) | const |  |
| [`ADDR_LIMIT_3GB`](#addr_limit_3gb) | const |  |
| [`PTRACE_O_TRACESYSGOOD`](#ptrace_o_tracesysgood) | const |  |
| [`PTRACE_O_TRACEFORK`](#ptrace_o_tracefork) | const |  |
| [`PTRACE_O_TRACEVFORK`](#ptrace_o_tracevfork) | const |  |
| [`PTRACE_O_TRACECLONE`](#ptrace_o_traceclone) | const |  |
| [`PTRACE_O_TRACEEXEC`](#ptrace_o_traceexec) | const |  |
| [`PTRACE_O_TRACEVFORKDONE`](#ptrace_o_tracevforkdone) | const |  |
| [`PTRACE_O_TRACEEXIT`](#ptrace_o_traceexit) | const |  |
| [`PTRACE_O_TRACESECCOMP`](#ptrace_o_traceseccomp) | const |  |
| [`PTRACE_O_SUSPEND_SECCOMP`](#ptrace_o_suspend_seccomp) | const |  |
| [`PTRACE_O_EXITKILL`](#ptrace_o_exitkill) | const |  |
| [`PTRACE_O_MASK`](#ptrace_o_mask) | const |  |
| [`PTRACE_EVENT_FORK`](#ptrace_event_fork) | const |  |
| [`PTRACE_EVENT_VFORK`](#ptrace_event_vfork) | const |  |
| [`PTRACE_EVENT_CLONE`](#ptrace_event_clone) | const |  |
| [`PTRACE_EVENT_EXEC`](#ptrace_event_exec) | const |  |
| [`PTRACE_EVENT_VFORK_DONE`](#ptrace_event_vfork_done) | const |  |
| [`PTRACE_EVENT_EXIT`](#ptrace_event_exit) | const |  |
| [`PTRACE_EVENT_SECCOMP`](#ptrace_event_seccomp) | const |  |
| [`__WNOTHREAD`](#__wnothread) | const |  |
| [`__WALL`](#__wall) | const |  |
| [`__WCLONE`](#__wclone) | const |  |
| [`SPLICE_F_MOVE`](#splice_f_move) | const |  |
| [`SPLICE_F_NONBLOCK`](#splice_f_nonblock) | const |  |
| [`SPLICE_F_MORE`](#splice_f_more) | const |  |
| [`SPLICE_F_GIFT`](#splice_f_gift) | const |  |
| [`RTLD_LOCAL`](#rtld_local) | const |  |
| [`RTLD_LAZY`](#rtld_lazy) | const |  |
| [`POSIX_FADV_NORMAL`](#posix_fadv_normal) | const |  |
| [`POSIX_FADV_RANDOM`](#posix_fadv_random) | const |  |
| [`POSIX_FADV_SEQUENTIAL`](#posix_fadv_sequential) | const |  |
| [`POSIX_FADV_WILLNEED`](#posix_fadv_willneed) | const |  |
| [`AT_FDCWD`](#at_fdcwd) | const |  |
| [`AT_SYMLINK_NOFOLLOW`](#at_symlink_nofollow) | const |  |
| [`AT_REMOVEDIR`](#at_removedir) | const |  |
| [`AT_SYMLINK_FOLLOW`](#at_symlink_follow) | const |  |
| [`AT_NO_AUTOMOUNT`](#at_no_automount) | const |  |
| [`AT_EMPTY_PATH`](#at_empty_path) | const |  |
| [`AT_RECURSIVE`](#at_recursive) | const |  |
| [`LOG_CRON`](#log_cron) | const |  |
| [`LOG_AUTHPRIV`](#log_authpriv) | const |  |
| [`LOG_FTP`](#log_ftp) | const |  |
| [`LOG_PERROR`](#log_perror) | const |  |
| [`PIPE_BUF`](#pipe_buf) | const |  |
| [`SI_LOAD_SHIFT`](#si_load_shift) | const |  |
| [`SI_USER`](#si_user) | const |  |
| [`SI_KERNEL`](#si_kernel) | const |  |
| [`SI_QUEUE`](#si_queue) | const |  |
| [`SI_TIMER`](#si_timer) | const |  |
| [`SI_MESGQ`](#si_mesgq) | const |  |
| [`SI_ASYNCIO`](#si_asyncio) | const |  |
| [`SI_SIGIO`](#si_sigio) | const |  |
| [`SI_TKILL`](#si_tkill) | const |  |
| [`SI_ASYNCNL`](#si_asyncnl) | const |  |
| [`BUS_ADRALN`](#bus_adraln) | const |  |
| [`BUS_ADRERR`](#bus_adrerr) | const |  |
| [`BUS_OBJERR`](#bus_objerr) | const |  |
| [`BUS_MCEERR_AR`](#bus_mceerr_ar) | const |  |
| [`BUS_MCEERR_AO`](#bus_mceerr_ao) | const |  |
| [`TRAP_BRKPT`](#trap_brkpt) | const |  |
| [`TRAP_TRACE`](#trap_trace) | const |  |
| [`TRAP_BRANCH`](#trap_branch) | const |  |
| [`TRAP_HWBKPT`](#trap_hwbkpt) | const |  |
| [`TRAP_UNK`](#trap_unk) | const |  |
| [`CLD_EXITED`](#cld_exited) | const |  |
| [`CLD_KILLED`](#cld_killed) | const |  |
| [`CLD_DUMPED`](#cld_dumped) | const |  |
| [`CLD_TRAPPED`](#cld_trapped) | const |  |
| [`CLD_STOPPED`](#cld_stopped) | const |  |
| [`CLD_CONTINUED`](#cld_continued) | const |  |
| [`SIGEV_SIGNAL`](#sigev_signal) | const |  |
| [`SIGEV_NONE`](#sigev_none) | const |  |
| [`SIGEV_THREAD`](#sigev_thread) | const |  |
| [`P_ALL`](#p_all) | const |  |
| [`P_PID`](#p_pid) | const |  |
| [`P_PGID`](#p_pgid) | const |  |
| [`P_PIDFD`](#p_pidfd) | const |  |
| [`UTIME_OMIT`](#utime_omit) | const |  |
| [`UTIME_NOW`](#utime_now) | const |  |
| [`POLLIN`](#pollin) | const |  |
| [`POLLPRI`](#pollpri) | const |  |
| [`POLLOUT`](#pollout) | const |  |
| [`POLLERR`](#pollerr) | const |  |
| [`POLLHUP`](#pollhup) | const |  |
| [`POLLNVAL`](#pollnval) | const |  |
| [`POLLRDNORM`](#pollrdnorm) | const |  |
| [`POLLRDBAND`](#pollrdband) | const |  |
| [`POLLRDHUP`](#pollrdhup) | const |  |
| [`IPTOS_LOWDELAY`](#iptos_lowdelay) | const |  |
| [`IPTOS_THROUGHPUT`](#iptos_throughput) | const |  |
| [`IPTOS_RELIABILITY`](#iptos_reliability) | const |  |
| [`IPTOS_MINCOST`](#iptos_mincost) | const |  |
| [`IPTOS_PREC_NETCONTROL`](#iptos_prec_netcontrol) | const |  |
| [`IPTOS_PREC_INTERNETCONTROL`](#iptos_prec_internetcontrol) | const |  |
| [`IPTOS_PREC_CRITIC_ECP`](#iptos_prec_critic_ecp) | const |  |
| [`IPTOS_PREC_FLASHOVERRIDE`](#iptos_prec_flashoverride) | const |  |
| [`IPTOS_PREC_FLASH`](#iptos_prec_flash) | const |  |
| [`IPTOS_PREC_IMMEDIATE`](#iptos_prec_immediate) | const |  |
| [`IPTOS_PREC_PRIORITY`](#iptos_prec_priority) | const |  |
| [`IPTOS_PREC_ROUTINE`](#iptos_prec_routine) | const |  |
| [`IPTOS_ECN_MASK`](#iptos_ecn_mask) | const |  |
| [`IPTOS_ECN_ECT1`](#iptos_ecn_ect1) | const |  |
| [`IPTOS_ECN_ECT0`](#iptos_ecn_ect0) | const |  |
| [`IPTOS_ECN_CE`](#iptos_ecn_ce) | const |  |
| [`IPOPT_COPY`](#ipopt_copy) | const |  |
| [`IPOPT_CLASS_MASK`](#ipopt_class_mask) | const |  |
| [`IPOPT_NUMBER_MASK`](#ipopt_number_mask) | const |  |
| [`IPOPT_CONTROL`](#ipopt_control) | const |  |
| [`IPOPT_RESERVED1`](#ipopt_reserved1) | const |  |
| [`IPOPT_MEASUREMENT`](#ipopt_measurement) | const |  |
| [`IPOPT_RESERVED2`](#ipopt_reserved2) | const |  |
| [`IPOPT_END`](#ipopt_end) | const |  |
| [`IPOPT_NOOP`](#ipopt_noop) | const |  |
| [`IPOPT_SEC`](#ipopt_sec) | const |  |
| [`IPOPT_LSRR`](#ipopt_lsrr) | const |  |
| [`IPOPT_TIMESTAMP`](#ipopt_timestamp) | const |  |
| [`IPOPT_RR`](#ipopt_rr) | const |  |
| [`IPOPT_SID`](#ipopt_sid) | const |  |
| [`IPOPT_SSRR`](#ipopt_ssrr) | const |  |
| [`IPOPT_RA`](#ipopt_ra) | const |  |
| [`IPVERSION`](#ipversion) | const |  |
| [`MAXTTL`](#maxttl) | const |  |
| [`IPDEFTTL`](#ipdefttl) | const |  |
| [`IPOPT_OPTVAL`](#ipopt_optval) | const |  |
| [`IPOPT_OLEN`](#ipopt_olen) | const |  |
| [`IPOPT_OFFSET`](#ipopt_offset) | const |  |
| [`IPOPT_MINOFF`](#ipopt_minoff) | const |  |
| [`MAX_IPOPTLEN`](#max_ipoptlen) | const |  |
| [`IPOPT_NOP`](#ipopt_nop) | const |  |
| [`IPOPT_EOL`](#ipopt_eol) | const |  |
| [`IPOPT_TS`](#ipopt_ts) | const |  |
| [`IPOPT_TS_TSONLY`](#ipopt_ts_tsonly) | const |  |
| [`IPOPT_TS_TSANDADDR`](#ipopt_ts_tsandaddr) | const |  |
| [`IPOPT_TS_PRESPEC`](#ipopt_ts_prespec) | const |  |
| [`ARPOP_RREQUEST`](#arpop_rrequest) | const |  |
| [`ARPOP_RREPLY`](#arpop_rreply) | const |  |
| [`ARPOP_InREQUEST`](#arpop_inrequest) | const |  |
| [`ARPOP_InREPLY`](#arpop_inreply) | const |  |
| [`ARPOP_NAK`](#arpop_nak) | const |  |
| [`ATF_NETMASK`](#atf_netmask) | const |  |
| [`ATF_DONTPUB`](#atf_dontpub) | const |  |
| [`ARPHRD_NETROM`](#arphrd_netrom) | const |  |
| [`ARPHRD_ETHER`](#arphrd_ether) | const |  |
| [`ARPHRD_EETHER`](#arphrd_eether) | const |  |
| [`ARPHRD_AX25`](#arphrd_ax25) | const |  |
| [`ARPHRD_PRONET`](#arphrd_pronet) | const |  |
| [`ARPHRD_CHAOS`](#arphrd_chaos) | const |  |
| [`ARPHRD_IEEE802`](#arphrd_ieee802) | const |  |
| [`ARPHRD_ARCNET`](#arphrd_arcnet) | const |  |
| [`ARPHRD_APPLETLK`](#arphrd_appletlk) | const |  |
| [`ARPHRD_DLCI`](#arphrd_dlci) | const |  |
| [`ARPHRD_ATM`](#arphrd_atm) | const |  |
| [`ARPHRD_METRICOM`](#arphrd_metricom) | const |  |
| [`ARPHRD_IEEE1394`](#arphrd_ieee1394) | const |  |
| [`ARPHRD_EUI64`](#arphrd_eui64) | const |  |
| [`ARPHRD_INFINIBAND`](#arphrd_infiniband) | const |  |
| [`ARPHRD_SLIP`](#arphrd_slip) | const |  |
| [`ARPHRD_CSLIP`](#arphrd_cslip) | const |  |
| [`ARPHRD_SLIP6`](#arphrd_slip6) | const |  |
| [`ARPHRD_CSLIP6`](#arphrd_cslip6) | const |  |
| [`ARPHRD_RSRVD`](#arphrd_rsrvd) | const |  |
| [`ARPHRD_ADAPT`](#arphrd_adapt) | const |  |
| [`ARPHRD_ROSE`](#arphrd_rose) | const |  |
| [`ARPHRD_X25`](#arphrd_x25) | const |  |
| [`ARPHRD_HWX25`](#arphrd_hwx25) | const |  |
| [`ARPHRD_CAN`](#arphrd_can) | const |  |
| [`ARPHRD_PPP`](#arphrd_ppp) | const |  |
| [`ARPHRD_CISCO`](#arphrd_cisco) | const |  |
| [`ARPHRD_HDLC`](#arphrd_hdlc) | const |  |
| [`ARPHRD_LAPB`](#arphrd_lapb) | const |  |
| [`ARPHRD_DDCMP`](#arphrd_ddcmp) | const |  |
| [`ARPHRD_RAWHDLC`](#arphrd_rawhdlc) | const |  |
| [`ARPHRD_TUNNEL`](#arphrd_tunnel) | const |  |
| [`ARPHRD_TUNNEL6`](#arphrd_tunnel6) | const |  |
| [`ARPHRD_FRAD`](#arphrd_frad) | const |  |
| [`ARPHRD_SKIP`](#arphrd_skip) | const |  |
| [`ARPHRD_LOOPBACK`](#arphrd_loopback) | const |  |
| [`ARPHRD_LOCALTLK`](#arphrd_localtlk) | const |  |
| [`ARPHRD_FDDI`](#arphrd_fddi) | const |  |
| [`ARPHRD_BIF`](#arphrd_bif) | const |  |
| [`ARPHRD_SIT`](#arphrd_sit) | const |  |
| [`ARPHRD_IPDDP`](#arphrd_ipddp) | const |  |
| [`ARPHRD_IPGRE`](#arphrd_ipgre) | const |  |
| [`ARPHRD_PIMREG`](#arphrd_pimreg) | const |  |
| [`ARPHRD_HIPPI`](#arphrd_hippi) | const |  |
| [`ARPHRD_ASH`](#arphrd_ash) | const |  |
| [`ARPHRD_ECONET`](#arphrd_econet) | const |  |
| [`ARPHRD_IRDA`](#arphrd_irda) | const |  |
| [`ARPHRD_FCPP`](#arphrd_fcpp) | const |  |
| [`ARPHRD_FCAL`](#arphrd_fcal) | const |  |
| [`ARPHRD_FCPL`](#arphrd_fcpl) | const |  |
| [`ARPHRD_FCFABRIC`](#arphrd_fcfabric) | const |  |
| [`ARPHRD_IEEE802_TR`](#arphrd_ieee802_tr) | const |  |
| [`ARPHRD_IEEE80211`](#arphrd_ieee80211) | const |  |
| [`ARPHRD_IEEE80211_PRISM`](#arphrd_ieee80211_prism) | const |  |
| [`ARPHRD_IEEE80211_RADIOTAP`](#arphrd_ieee80211_radiotap) | const |  |
| [`ARPHRD_IEEE802154`](#arphrd_ieee802154) | const |  |
| [`ARPHRD_VOID`](#arphrd_void) | const |  |
| [`ARPHRD_NONE`](#arphrd_none) | const |  |
| [`IFF_TUN`](#iff_tun) | const |  |
| [`IFF_TAP`](#iff_tap) | const |  |
| [`IFF_NAPI`](#iff_napi) | const |  |
| [`IFF_NAPI_FRAGS`](#iff_napi_frags) | const |  |
| [`IFF_NO_CARRIER`](#iff_no_carrier) | const |  |
| [`IFF_NO_PI`](#iff_no_pi) | const |  |
| [`TUN_READQ_SIZE`](#tun_readq_size) | const |  |
| [`TUN_TUN_DEV`](#tun_tun_dev) | const |  |
| [`TUN_TAP_DEV`](#tun_tap_dev) | const |  |
| [`TUN_TYPE_MASK`](#tun_type_mask) | const |  |
| [`IFF_ONE_QUEUE`](#iff_one_queue) | const |  |
| [`IFF_VNET_HDR`](#iff_vnet_hdr) | const |  |
| [`IFF_TUN_EXCL`](#iff_tun_excl) | const |  |
| [`IFF_MULTI_QUEUE`](#iff_multi_queue) | const |  |
| [`IFF_ATTACH_QUEUE`](#iff_attach_queue) | const |  |
| [`IFF_DETACH_QUEUE`](#iff_detach_queue) | const |  |
| [`IFF_PERSIST`](#iff_persist) | const |  |
| [`IFF_NOFILTER`](#iff_nofilter) | const |  |
| [`TUN_TX_TIMESTAMP`](#tun_tx_timestamp) | const |  |
| [`TUN_F_CSUM`](#tun_f_csum) | const |  |
| [`TUN_F_TSO4`](#tun_f_tso4) | const |  |
| [`TUN_F_TSO6`](#tun_f_tso6) | const |  |
| [`TUN_F_TSO_ECN`](#tun_f_tso_ecn) | const |  |
| [`TUN_F_UFO`](#tun_f_ufo) | const |  |
| [`TUN_F_USO4`](#tun_f_uso4) | const |  |
| [`TUN_F_USO6`](#tun_f_uso6) | const |  |
| [`TUN_PKT_STRIP`](#tun_pkt_strip) | const |  |
| [`TUN_FLT_ALLMULTI`](#tun_flt_allmulti) | const |  |
| [`T_TYPE`](#t_type) | const |  |
| [`TUNSETNOCSUM`](#tunsetnocsum) | const |  |
| [`TUNSETDEBUG`](#tunsetdebug) | const |  |
| [`TUNSETIFF`](#tunsetiff) | const |  |
| [`TUNSETPERSIST`](#tunsetpersist) | const |  |
| [`TUNSETOWNER`](#tunsetowner) | const |  |
| [`TUNSETLINK`](#tunsetlink) | const |  |
| [`TUNSETGROUP`](#tunsetgroup) | const |  |
| [`TUNGETFEATURES`](#tungetfeatures) | const |  |
| [`TUNSETOFFLOAD`](#tunsetoffload) | const |  |
| [`TUNSETTXFILTER`](#tunsettxfilter) | const |  |
| [`TUNGETIFF`](#tungetiff) | const |  |
| [`TUNGETSNDBUF`](#tungetsndbuf) | const |  |
| [`TUNSETSNDBUF`](#tunsetsndbuf) | const |  |
| [`TUNATTACHFILTER`](#tunattachfilter) | const |  |
| [`TUNDETACHFILTER`](#tundetachfilter) | const |  |
| [`TUNGETVNETHDRSZ`](#tungetvnethdrsz) | const |  |
| [`TUNSETVNETHDRSZ`](#tunsetvnethdrsz) | const |  |
| [`TUNSETQUEUE`](#tunsetqueue) | const |  |
| [`TUNSETIFINDEX`](#tunsetifindex) | const |  |
| [`TUNGETFILTER`](#tungetfilter) | const |  |
| [`TUNSETVNETLE`](#tunsetvnetle) | const |  |
| [`TUNGETVNETLE`](#tungetvnetle) | const |  |
| [`TUNSETVNETBE`](#tunsetvnetbe) | const |  |
| [`TUNGETVNETBE`](#tungetvnetbe) | const |  |
| [`TUNSETSTEERINGEBPF`](#tunsetsteeringebpf) | const |  |
| [`TUNSETFILTEREBPF`](#tunsetfilterebpf) | const |  |
| [`TUNSETCARRIER`](#tunsetcarrier) | const |  |
| [`TUNGETDEVNETNS`](#tungetdevnetns) | const |  |
| [`FS_IOC_GETFLAGS`](#fs_ioc_getflags) | const |  |
| [`FS_IOC_SETFLAGS`](#fs_ioc_setflags) | const |  |
| [`FS_IOC_GETVERSION`](#fs_ioc_getversion) | const |  |
| [`FS_IOC_SETVERSION`](#fs_ioc_setversion) | const |  |
| [`FS_IOC32_GETFLAGS`](#fs_ioc32_getflags) | const |  |
| [`FS_IOC32_SETFLAGS`](#fs_ioc32_setflags) | const |  |
| [`FS_IOC32_GETVERSION`](#fs_ioc32_getversion) | const |  |
| [`FS_IOC32_SETVERSION`](#fs_ioc32_setversion) | const |  |
| [`FICLONE`](#ficlone) | const |  |
| [`FICLONERANGE`](#ficlonerange) | const |  |
| [`ADFS_SUPER_MAGIC`](#adfs_super_magic) | const |  |
| [`AFFS_SUPER_MAGIC`](#affs_super_magic) | const |  |
| [`AFS_SUPER_MAGIC`](#afs_super_magic) | const |  |
| [`AUTOFS_SUPER_MAGIC`](#autofs_super_magic) | const |  |
| [`BPF_FS_MAGIC`](#bpf_fs_magic) | const |  |
| [`BTRFS_SUPER_MAGIC`](#btrfs_super_magic) | const |  |
| [`CGROUP2_SUPER_MAGIC`](#cgroup2_super_magic) | const |  |
| [`CGROUP_SUPER_MAGIC`](#cgroup_super_magic) | const |  |
| [`CODA_SUPER_MAGIC`](#coda_super_magic) | const |  |
| [`CRAMFS_MAGIC`](#cramfs_magic) | const |  |
| [`DEBUGFS_MAGIC`](#debugfs_magic) | const |  |
| [`DEVPTS_SUPER_MAGIC`](#devpts_super_magic) | const |  |
| [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs_super_magic) | const |  |
| [`EFS_SUPER_MAGIC`](#efs_super_magic) | const |  |
| [`EXT2_SUPER_MAGIC`](#ext2_super_magic) | const |  |
| [`EXT3_SUPER_MAGIC`](#ext3_super_magic) | const |  |
| [`EXT4_SUPER_MAGIC`](#ext4_super_magic) | const |  |
| [`F2FS_SUPER_MAGIC`](#f2fs_super_magic) | const |  |
| [`FUSE_SUPER_MAGIC`](#fuse_super_magic) | const |  |
| [`FUTEXFS_SUPER_MAGIC`](#futexfs_super_magic) | const |  |
| [`HOSTFS_SUPER_MAGIC`](#hostfs_super_magic) | const |  |
| [`HPFS_SUPER_MAGIC`](#hpfs_super_magic) | const |  |
| [`HUGETLBFS_MAGIC`](#hugetlbfs_magic) | const |  |
| [`ISOFS_SUPER_MAGIC`](#isofs_super_magic) | const |  |
| [`JFFS2_SUPER_MAGIC`](#jffs2_super_magic) | const |  |
| [`MINIX2_SUPER_MAGIC2`](#minix2_super_magic2) | const |  |
| [`MINIX2_SUPER_MAGIC`](#minix2_super_magic) | const |  |
| [`MINIX3_SUPER_MAGIC`](#minix3_super_magic) | const |  |
| [`MINIX_SUPER_MAGIC2`](#minix_super_magic2) | const |  |
| [`MINIX_SUPER_MAGIC`](#minix_super_magic) | const |  |
| [`MSDOS_SUPER_MAGIC`](#msdos_super_magic) | const |  |
| [`NCP_SUPER_MAGIC`](#ncp_super_magic) | const |  |
| [`NFS_SUPER_MAGIC`](#nfs_super_magic) | const |  |
| [`NILFS_SUPER_MAGIC`](#nilfs_super_magic) | const |  |
| [`OCFS2_SUPER_MAGIC`](#ocfs2_super_magic) | const |  |
| [`OPENPROM_SUPER_MAGIC`](#openprom_super_magic) | const |  |
| [`OVERLAYFS_SUPER_MAGIC`](#overlayfs_super_magic) | const |  |
| [`PROC_SUPER_MAGIC`](#proc_super_magic) | const |  |
| [`QNX4_SUPER_MAGIC`](#qnx4_super_magic) | const |  |
| [`QNX6_SUPER_MAGIC`](#qnx6_super_magic) | const |  |
| [`RDTGROUP_SUPER_MAGIC`](#rdtgroup_super_magic) | const |  |
| [`REISERFS_SUPER_MAGIC`](#reiserfs_super_magic) | const |  |
| [`SECURITYFS_MAGIC`](#securityfs_magic) | const |  |
| [`SELINUX_MAGIC`](#selinux_magic) | const |  |
| [`SMACK_MAGIC`](#smack_magic) | const |  |
| [`SMB_SUPER_MAGIC`](#smb_super_magic) | const |  |
| [`SYSFS_MAGIC`](#sysfs_magic) | const |  |
| [`TMPFS_MAGIC`](#tmpfs_magic) | const |  |
| [`TRACEFS_MAGIC`](#tracefs_magic) | const |  |
| [`UDF_SUPER_MAGIC`](#udf_super_magic) | const |  |
| [`USBDEVICE_SUPER_MAGIC`](#usbdevice_super_magic) | const |  |
| [`XENFS_SUPER_MAGIC`](#xenfs_super_magic) | const |  |
| [`NSFS_MAGIC`](#nsfs_magic) | const |  |
| [`AT_STATX_SYNC_TYPE`](#at_statx_sync_type) | const |  |
| [`AT_STATX_SYNC_AS_STAT`](#at_statx_sync_as_stat) | const |  |
| [`AT_STATX_FORCE_SYNC`](#at_statx_force_sync) | const |  |
| [`AT_STATX_DONT_SYNC`](#at_statx_dont_sync) | const |  |
| [`STATX_TYPE`](#statx_type) | const |  |
| [`STATX_MODE`](#statx_mode) | const |  |
| [`STATX_NLINK`](#statx_nlink) | const |  |
| [`STATX_UID`](#statx_uid) | const |  |
| [`STATX_GID`](#statx_gid) | const |  |
| [`STATX_ATIME`](#statx_atime) | const |  |
| [`STATX_MTIME`](#statx_mtime) | const |  |
| [`STATX_CTIME`](#statx_ctime) | const |  |
| [`STATX_INO`](#statx_ino) | const |  |
| [`STATX_SIZE`](#statx_size) | const |  |
| [`STATX_BLOCKS`](#statx_blocks) | const |  |
| [`STATX_BASIC_STATS`](#statx_basic_stats) | const |  |
| [`STATX_BTIME`](#statx_btime) | const |  |
| [`STATX_ALL`](#statx_all) | const |  |
| [`STATX_MNT_ID`](#statx_mnt_id) | const |  |
| [`STATX_DIOALIGN`](#statx_dioalign) | const |  |
| [`STATX__RESERVED`](#statx__reserved) | const |  |
| [`STATX_ATTR_COMPRESSED`](#statx_attr_compressed) | const |  |
| [`STATX_ATTR_IMMUTABLE`](#statx_attr_immutable) | const |  |
| [`STATX_ATTR_APPEND`](#statx_attr_append) | const |  |
| [`STATX_ATTR_NODUMP`](#statx_attr_nodump) | const |  |
| [`STATX_ATTR_ENCRYPTED`](#statx_attr_encrypted) | const |  |
| [`STATX_ATTR_AUTOMOUNT`](#statx_attr_automount) | const |  |
| [`STATX_ATTR_MOUNT_ROOT`](#statx_attr_mount_root) | const |  |
| [`STATX_ATTR_VERITY`](#statx_attr_verity) | const |  |
| [`STATX_ATTR_DAX`](#statx_attr_dax) | const |  |
| [`_IOC_NRBITS`](#_ioc_nrbits) | const |  |
| [`_IOC_TYPEBITS`](#_ioc_typebits) | const |  |
| [`_IOC_SIZEBITS`](#_ioc_sizebits) | const |  |
| [`_IOC_DIRBITS`](#_ioc_dirbits) | const |  |
| [`_IOC_NONE`](#_ioc_none) | const |  |
| [`_IOC_WRITE`](#_ioc_write) | const |  |
| [`_IOC_READ`](#_ioc_read) | const |  |
| [`_IOC_NRMASK`](#_ioc_nrmask) | const |  |
| [`_IOC_TYPEMASK`](#_ioc_typemask) | const |  |
| [`_IOC_SIZEMASK`](#_ioc_sizemask) | const |  |
| [`_IOC_DIRMASK`](#_ioc_dirmask) | const |  |
| [`_IOC_NRSHIFT`](#_ioc_nrshift) | const |  |
| [`_IOC_TYPESHIFT`](#_ioc_typeshift) | const |  |
| [`_IOC_SIZESHIFT`](#_ioc_sizeshift) | const |  |
| [`_IOC_DIRSHIFT`](#_ioc_dirshift) | const |  |

## Modules

- [`linux_like`](linux_like/index.md)
- [`linux`](linux/index.md) — Linux-specific definitions for linux-like values

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

- <span id="group-clone"></span>`fn clone(&self) -> group` — [`group`](../index.md)

##### `impl Copy for group`

##### `impl Debug for group`

- <span id="group-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `utimbuf`

```rust
struct utimbuf {
    pub actime: time_t,
    pub modtime: time_t,
}
```

#### Trait Implementations

##### `impl Clone for utimbuf`

- <span id="utimbuf-clone"></span>`fn clone(&self) -> utimbuf` — [`utimbuf`](../index.md)

##### `impl Copy for utimbuf`

##### `impl Debug for utimbuf`

- <span id="utimbuf-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `timeval`

```rust
struct timeval {
    pub tv_sec: time_t,
    pub tv_usec: crate::suseconds_t,
}
```

#### Trait Implementations

##### `impl Clone for timeval`

- <span id="timeval-clone"></span>`fn clone(&self) -> timeval` — [`timeval`](../index.md)

##### `impl Copy for timeval`

##### `impl Debug for timeval`

- <span id="timeval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `rlimit`

```rust
struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
```

#### Trait Implementations

##### `impl Clone for rlimit`

- <span id="rlimit-clone"></span>`fn clone(&self) -> rlimit` — [`rlimit`](../index.md)

##### `impl Copy for rlimit`

##### `impl Debug for rlimit`

- <span id="rlimit-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="rusage-clone"></span>`fn clone(&self) -> rusage` — [`rusage`](../index.md)

##### `impl Copy for rusage`

##### `impl Debug for rusage`

- <span id="rusage-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ipv6_mreq`

```rust
struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for ipv6_mreq`

- <span id="ipv6-mreq-clone"></span>`fn clone(&self) -> ipv6_mreq` — [`ipv6_mreq`](../index.md)

##### `impl Copy for ipv6_mreq`

##### `impl Debug for ipv6_mreq`

- <span id="ipv6-mreq-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="hostent-clone"></span>`fn clone(&self) -> hostent` — [`hostent`](../index.md)

##### `impl Copy for hostent`

##### `impl Debug for hostent`

- <span id="hostent-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `iovec`

```rust
struct iovec {
    pub iov_base: *mut crate::c_void,
    pub iov_len: size_t,
}
```

#### Trait Implementations

##### `impl Clone for iovec`

- <span id="iovec-clone"></span>`fn clone(&self) -> iovec` — [`iovec`](../index.md)

##### `impl Copy for iovec`

##### `impl Debug for iovec`

- <span id="iovec-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="pollfd-clone"></span>`fn clone(&self) -> pollfd` — [`pollfd`](../index.md)

##### `impl Copy for pollfd`

##### `impl Debug for pollfd`

- <span id="pollfd-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="winsize-clone"></span>`fn clone(&self) -> winsize` — [`winsize`](../index.md)

##### `impl Copy for winsize`

##### `impl Debug for winsize`

- <span id="winsize-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `linger`

```rust
struct linger {
    pub l_onoff: crate::c_int,
    pub l_linger: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for linger`

- <span id="linger-clone"></span>`fn clone(&self) -> linger` — [`linger`](../index.md)

##### `impl Copy for linger`

##### `impl Debug for linger`

- <span id="linger-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sigval`

```rust
struct sigval {
    pub sival_ptr: *mut crate::c_void,
}
```

#### Trait Implementations

##### `impl Clone for sigval`

- <span id="sigval-clone"></span>`fn clone(&self) -> sigval` — [`sigval`](../index.md)

##### `impl Copy for sigval`

##### `impl Debug for sigval`

- <span id="sigval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `itimerval`

```rust
struct itimerval {
    pub it_interval: crate::timeval,
    pub it_value: crate::timeval,
}
```

#### Trait Implementations

##### `impl Clone for itimerval`

- <span id="itimerval-clone"></span>`fn clone(&self) -> itimerval` — [`itimerval`](../index.md)

##### `impl Copy for itimerval`

##### `impl Debug for itimerval`

- <span id="itimerval-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="tms-clone"></span>`fn clone(&self) -> tms` — [`tms`](../index.md)

##### `impl Copy for tms`

##### `impl Debug for tms`

- <span id="tms-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="servent-clone"></span>`fn clone(&self) -> servent` — [`servent`](../index.md)

##### `impl Copy for servent`

##### `impl Debug for servent`

- <span id="servent-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="protoent-clone"></span>`fn clone(&self) -> protoent` — [`protoent`](../index.md)

##### `impl Copy for protoent`

##### `impl Debug for protoent`

- <span id="protoent-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in6_addr`

```rust
struct in6_addr {
    pub s6_addr: [u8; 16],
}
```

#### Trait Implementations

##### `impl Clone for in6_addr`

- <span id="in6-addr-clone"></span>`fn clone(&self) -> in6_addr` — [`in6_addr`](../index.md)

##### `impl Copy for in6_addr`

##### `impl Debug for in6_addr`

- <span id="in6-addr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in_addr`

```rust
struct in_addr {
    pub s_addr: crate::in_addr_t,
}
```

#### Trait Implementations

##### `impl Clone for in_addr`

- <span id="in-addr-clone"></span>`fn clone(&self) -> in_addr` — [`in_addr`](#in-addr)

##### `impl Copy for in_addr`

##### `impl Debug for in_addr`

- <span id="in-addr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ip_mreq`

```rust
struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
```

#### Trait Implementations

##### `impl Clone for ip_mreq`

- <span id="ip-mreq-clone"></span>`fn clone(&self) -> ip_mreq` — [`ip_mreq`](#ip-mreq)

##### `impl Copy for ip_mreq`

##### `impl Debug for ip_mreq`

- <span id="ip-mreq-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="ip-mreqn-clone"></span>`fn clone(&self) -> ip_mreqn` — [`ip_mreqn`](#ip-mreqn)

##### `impl Copy for ip_mreqn`

##### `impl Debug for ip_mreqn`

- <span id="ip-mreqn-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="ip-mreq-source-clone"></span>`fn clone(&self) -> ip_mreq_source` — [`ip_mreq_source`](#ip-mreq-source)

##### `impl Copy for ip_mreq_source`

##### `impl Debug for ip_mreq_source`

- <span id="ip-mreq-source-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr`

```rust
struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [crate::c_char; 14],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr`

- <span id="sockaddr-clone"></span>`fn clone(&self) -> sockaddr` — [`sockaddr`](#sockaddr)

##### `impl Copy for sockaddr`

##### `impl Debug for sockaddr`

- <span id="sockaddr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="sockaddr-in-clone"></span>`fn clone(&self) -> sockaddr_in` — [`sockaddr_in`](#sockaddr-in)

##### `impl Copy for sockaddr_in`

##### `impl Debug for sockaddr_in`

- <span id="sockaddr-in-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="sockaddr-in6-clone"></span>`fn clone(&self) -> sockaddr_in6` — [`sockaddr_in6`](#sockaddr-in6)

##### `impl Copy for sockaddr_in6`

##### `impl Debug for sockaddr_in6`

- <span id="sockaddr-in6-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="addrinfo-clone"></span>`fn clone(&self) -> addrinfo` — [`addrinfo`](#addrinfo)

##### `impl Copy for addrinfo`

##### `impl Debug for addrinfo`

- <span id="addrinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="sockaddr-ll-clone"></span>`fn clone(&self) -> sockaddr_ll` — [`sockaddr_ll`](#sockaddr-ll)

##### `impl Copy for sockaddr_ll`

##### `impl Debug for sockaddr_ll`

- <span id="sockaddr-ll-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `fd_set`

```rust
struct fd_set {
    fds_bits: [crate::c_ulong; 16],
}
```

#### Trait Implementations

##### `impl Clone for fd_set`

- <span id="fd-set-clone"></span>`fn clone(&self) -> fd_set` — [`fd_set`](#fd-set)

##### `impl Copy for fd_set`

##### `impl Debug for fd_set`

- <span id="fd-set-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="tm-clone"></span>`fn clone(&self) -> tm` — [`tm`](#tm)

##### `impl Copy for tm`

##### `impl Debug for tm`

- <span id="tm-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sched_param`

```rust
struct sched_param {
    pub sched_priority: crate::c_int,
}
```

#### Trait Implementations

##### `impl Clone for sched_param`

- <span id="sched-param-clone"></span>`fn clone(&self) -> sched_param` — [`sched_param`](#sched-param)

##### `impl Copy for sched_param`

##### `impl Debug for sched_param`

- <span id="sched-param-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="dl-info-clone"></span>`fn clone(&self) -> Dl_info` — [`Dl_info`](#dl-info)

##### `impl Copy for Dl_info`

##### `impl Debug for Dl_info`

- <span id="dl-info-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="lconv-clone"></span>`fn clone(&self) -> lconv` — [`lconv`](#lconv)

##### `impl Copy for lconv`

##### `impl Debug for lconv`

- <span id="lconv-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="in-pktinfo-clone"></span>`fn clone(&self) -> in_pktinfo` — [`in_pktinfo`](#in-pktinfo)

##### `impl Copy for in_pktinfo`

##### `impl Debug for in_pktinfo`

- <span id="in-pktinfo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="ifaddrs-clone"></span>`fn clone(&self) -> ifaddrs` — [`ifaddrs`](#ifaddrs)

##### `impl Copy for ifaddrs`

##### `impl Debug for ifaddrs`

- <span id="ifaddrs-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="in6-rtmsg-clone"></span>`fn clone(&self) -> in6_rtmsg` — [`in6_rtmsg`](#in6-rtmsg)

##### `impl Copy for in6_rtmsg`

##### `impl Debug for in6_rtmsg`

- <span id="in6-rtmsg-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="arpreq-clone"></span>`fn clone(&self) -> arpreq` — [`arpreq`](#arpreq)

##### `impl Copy for arpreq`

##### `impl Debug for arpreq`

- <span id="arpreq-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="arpreq-old-clone"></span>`fn clone(&self) -> arpreq_old` — [`arpreq_old`](#arpreq-old)

##### `impl Copy for arpreq_old`

##### `impl Debug for arpreq_old`

- <span id="arpreq-old-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="arphdr-clone"></span>`fn clone(&self) -> arphdr` — [`arphdr`](#arphdr)

##### `impl Copy for arphdr`

##### `impl Debug for arphdr`

- <span id="arphdr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `mmsghdr`

```rust
struct mmsghdr {
    pub msg_hdr: crate::msghdr,
    pub msg_len: crate::c_uint,
}
```

#### Trait Implementations

##### `impl Clone for mmsghdr`

- <span id="mmsghdr-clone"></span>`fn clone(&self) -> mmsghdr` — [`mmsghdr`](#mmsghdr)

##### `impl Copy for mmsghdr`

##### `impl Debug for mmsghdr`

- <span id="mmsghdr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sockaddr_un`

```rust
struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [crate::c_char; 108],
}
```

#### Trait Implementations

##### `impl Clone for sockaddr_un`

- <span id="sockaddr-un-clone"></span>`fn clone(&self) -> sockaddr_un` — [`sockaddr_un`](#sockaddr-un)

##### `impl Copy for sockaddr_un`

##### `impl Debug for sockaddr_un`

- <span id="sockaddr-un-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="sockaddr-storage-clone"></span>`fn clone(&self) -> sockaddr_storage` — [`sockaddr_storage`](#sockaddr-storage)

##### `impl Copy for sockaddr_storage`

##### `impl Debug for sockaddr_storage`

- <span id="sockaddr-storage-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="utsname-clone"></span>`fn clone(&self) -> utsname` — [`utsname`](#utsname)

##### `impl Copy for utsname`

##### `impl Debug for utsname`

- <span id="utsname-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="file-clone-range-clone"></span>`fn clone(&self) -> file_clone_range` — [`file_clone_range`](#file-clone-range)

##### `impl Copy for file_clone_range`

##### `impl Debug for file_clone_range`

- <span id="file-clone-range-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="sock-filter-clone"></span>`fn clone(&self) -> sock_filter` — [`sock_filter`](#sock-filter)

##### `impl Copy for sock_filter`

##### `impl Debug for sock_filter`

- <span id="sock-filter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sock_fprog`

```rust
struct sock_fprog {
    pub len: crate::c_ushort,
    pub filter: *mut sock_filter,
}
```

#### Trait Implementations

##### `impl Clone for sock_fprog`

- <span id="sock-fprog-clone"></span>`fn clone(&self) -> sock_fprog` — [`sock_fprog`](#sock-fprog)

##### `impl Copy for sock_fprog`

##### `impl Debug for sock_fprog`

- <span id="sock-fprog-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="statx-clone"></span>`fn clone(&self) -> statx` — [`statx`](#statx)

##### `impl Copy for statx`

##### `impl Debug for statx`

- <span id="statx-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="statx-timestamp-clone"></span>`fn clone(&self) -> statx_timestamp` — [`statx_timestamp`](#statx-timestamp)

##### `impl Copy for statx_timestamp`

##### `impl Debug for statx_timestamp`

- <span id="statx-timestamp-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `epoll_event`

```rust
struct epoll_event {
    pub events: u32,
    pub u64: u64,
}
```

#### Trait Implementations

##### `impl Clone for epoll_event`

- <span id="epoll-event-clone"></span>`fn clone(&self) -> epoll_event` — [`epoll_event`](#epoll-event)

##### `impl Copy for epoll_event`

##### `impl Debug for epoll_event`

- <span id="epoll-event-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- <span id="sigevent-clone"></span>`fn clone(&self) -> sigevent` — [`sigevent`](#sigevent)

##### `impl Copy for sigevent`

##### `impl Debug for sigevent`

- <span id="sigevent-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `DIR`

```rust
enum DIR {
}
```

#### Trait Implementations

##### `impl Clone for DIR`

- <span id="dir-clone"></span>`fn clone(&self) -> DIR` — [`DIR`](../index.md)

##### `impl Copy for DIR`

##### `impl Debug for DIR`

- <span id="dir-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FILE`

```rust
enum FILE {
}
```

#### Trait Implementations

##### `impl Clone for FILE`

- <span id="file-clone"></span>`fn clone(&self) -> FILE` — [`FILE`](../index.md)

##### `impl Copy for FILE`

##### `impl Debug for FILE`

- <span id="file-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `timezone`

```rust
enum timezone {
}
```

#### Trait Implementations

##### `impl Clone for timezone`

- <span id="timezone-clone"></span>`fn clone(&self) -> timezone` — [`timezone`](#timezone)

##### `impl Copy for timezone`

##### `impl Debug for timezone`

- <span id="timezone-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

