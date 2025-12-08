# Crate `libc`

libc - Raw FFI bindings to platforms' system libraries

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`new`](#new)
  - [`primitives`](#primitives)
  - [`unix`](#unix)
  - [`types`](#types)
  - [`prelude`](#prelude)
  - [`common`](#common)
  - [`linux_uapi`](#linux_uapi)
  - [`glibc`](#glibc)
  - [`linux_like`](#linux_like)
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
- [Enums](#enums)
  - [`DIR`](#dir)
  - [`FILE`](#file)
- [Functions](#functions)
  - [`unnamed`](#unnamed)
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
- [Type Aliases](#type-aliases)
  - [`c_schar`](#c_schar)
  - [`c_uchar`](#c_uchar)
  - [`c_short`](#c_short)
  - [`c_ushort`](#c_ushort)
  - [`c_longlong`](#c_longlong)
  - [`c_ulonglong`](#c_ulonglong)
  - [`c_float`](#c_float)
  - [`c_double`](#c_double)
  - [`c_char`](#c_char)
  - [`c_int`](#c_int)
  - [`c_uint`](#c_uint)
  - [`c_long`](#c_long)
  - [`c_ulong`](#c_ulong)
  - [`int8_t`](#int8_t)
  - [`int16_t`](#int16_t)
  - [`int32_t`](#int32_t)
  - [`int64_t`](#int64_t)
  - [`uint8_t`](#uint8_t)
  - [`uint16_t`](#uint16_t)
  - [`uint32_t`](#uint32_t)
  - [`uint64_t`](#uint64_t)
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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`new`](#new) | mod | This module contains the future directory structure. |
| [`primitives`](#primitives) | mod | This module contains type aliases for C's platform-specific types |
| [`unix`](#unix) | mod | Definitions found commonly among almost all Unix derivatives |
| [`types`](#types) | mod | Platform-agnostic support types. |
| [`prelude`](#prelude) | mod | Frequently-used types that are available on all platforms |
| [`common`](#common) | mod | Interfaces that are common across multiple platforms |
| [`linux_uapi`](#linux_uapi) | mod | This directory maps to `include/uapi` in the Linux source tree. |
| [`glibc`](#glibc) | mod | GNU libc. |
| [`linux_like`](#linux_like) | mod |  |
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
| [`DIR`](#dir) | enum |  |
| [`FILE`](#file) | enum |  |
| [`unnamed`](#unnamed) | fn |  |
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
| [`c_schar`](#c_schar) | type |  |
| [`c_uchar`](#c_uchar) | type |  |
| [`c_short`](#c_short) | type |  |
| [`c_ushort`](#c_ushort) | type |  |
| [`c_longlong`](#c_longlong) | type |  |
| [`c_ulonglong`](#c_ulonglong) | type |  |
| [`c_float`](#c_float) | type |  |
| [`c_double`](#c_double) | type |  |
| [`c_char`](#c_char) | type |  |
| [`c_int`](#c_int) | type |  |
| [`c_uint`](#c_uint) | type |  |
| [`c_long`](#c_long) | type |  |
| [`c_ulong`](#c_ulong) | type |  |
| [`int8_t`](#int8_t) | type |  |
| [`int16_t`](#int16_t) | type |  |
| [`int32_t`](#int32_t) | type |  |
| [`int64_t`](#int64_t) | type |  |
| [`uint8_t`](#uint8_t) | type |  |
| [`uint16_t`](#uint16_t) | type |  |
| [`uint32_t`](#uint32_t) | type |  |
| [`uint64_t`](#uint64_t) | type |  |
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

## Modules

- [`macros`](macros/index.md) - 
- [`new`](new/index.md) - This module contains the future directory structure. If possible, new definitions should
- [`primitives`](primitives/index.md) - This module contains type aliases for C's platform-specific types
- [`unix`](unix/index.md) - Definitions found commonly among almost all Unix derivatives
- [`types`](types/index.md) - Platform-agnostic support types.
- [`prelude`](prelude/index.md) - Frequently-used types that are available on all platforms
- [`common`](common/index.md) - Interfaces that are common across multiple platforms
- [`linux_uapi`](linux_uapi/index.md) - This directory maps to `include/uapi` in the Linux source tree.
- [`glibc`](glibc/index.md) - GNU libc.
- [`linux_like`](linux_like/index.md) - 

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

- <span id="group-clone"></span>`fn clone(&self) -> group` — [`group`](#group)

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

- <span id="utimbuf-clone"></span>`fn clone(&self) -> utimbuf` — [`utimbuf`](#utimbuf)

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

- <span id="timeval-clone"></span>`fn clone(&self) -> timeval` — [`timeval`](#timeval)

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

- <span id="rlimit-clone"></span>`fn clone(&self) -> rlimit` — [`rlimit`](#rlimit)

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

- <span id="rusage-clone"></span>`fn clone(&self) -> rusage` — [`rusage`](#rusage)

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

- <span id="ipv6-mreq-clone"></span>`fn clone(&self) -> ipv6_mreq` — [`ipv6_mreq`](#ipv6-mreq)

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

- <span id="hostent-clone"></span>`fn clone(&self) -> hostent` — [`hostent`](#hostent)

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

- <span id="iovec-clone"></span>`fn clone(&self) -> iovec` — [`iovec`](#iovec)

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

- <span id="pollfd-clone"></span>`fn clone(&self) -> pollfd` — [`pollfd`](#pollfd)

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

- <span id="winsize-clone"></span>`fn clone(&self) -> winsize` — [`winsize`](#winsize)

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

- <span id="linger-clone"></span>`fn clone(&self) -> linger` — [`linger`](#linger)

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

- <span id="sigval-clone"></span>`fn clone(&self) -> sigval` — [`sigval`](#sigval)

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

- <span id="itimerval-clone"></span>`fn clone(&self) -> itimerval` — [`itimerval`](#itimerval)

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

- <span id="tms-clone"></span>`fn clone(&self) -> tms` — [`tms`](#tms)

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

- <span id="servent-clone"></span>`fn clone(&self) -> servent` — [`servent`](#servent)

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

- <span id="protoent-clone"></span>`fn clone(&self) -> protoent` — [`protoent`](#protoent)

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

- <span id="in6-addr-clone"></span>`fn clone(&self) -> in6_addr` — [`in6_addr`](#in6-addr)

##### `impl Copy for in6_addr`

##### `impl Debug for in6_addr`

- <span id="in6-addr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `DIR`

```rust
enum DIR {
}
```

#### Trait Implementations

##### `impl Clone for DIR`

- <span id="dir-clone"></span>`fn clone(&self) -> DIR` — [`DIR`](#dir)

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

- <span id="file-clone"></span>`fn clone(&self) -> FILE` — [`FILE`](#file)

##### `impl Copy for FILE`

##### `impl Debug for FILE`

- <span id="file-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

