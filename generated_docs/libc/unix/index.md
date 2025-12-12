*[libc](../index.md) / [unix](index.md)*

---

# Module `unix`

Definitions found commonly among almost all Unix derivatives

More functions and definitions can be found in the more specific modules
according to the platform in question.

## Contents

- [Modules](#modules)
  - [`linux_like`](#linux-like)
  - [`linux`](#linux)
- [Structs](#structs)
  - [`group`](#group)
  - [`utimbuf`](#utimbuf)
  - [`timeval`](#timeval)
  - [`rlimit`](#rlimit)
  - [`rusage`](#rusage)
  - [`ipv6_mreq`](#ipv6-mreq)
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
  - [`in6_addr`](#in6-addr)
  - [`in_addr`](#in-addr)
  - [`ip_mreq`](#ip-mreq)
  - [`ip_mreqn`](#ip-mreqn)
  - [`ip_mreq_source`](#ip-mreq-source)
  - [`sockaddr`](#sockaddr)
  - [`sockaddr_in`](#sockaddr-in)
  - [`sockaddr_in6`](#sockaddr-in6)
  - [`addrinfo`](#addrinfo)
  - [`sockaddr_ll`](#sockaddr-ll)
  - [`fd_set`](#fd-set)
  - [`tm`](#tm)
  - [`sched_param`](#sched-param)
  - [`Dl_info`](#dl-info)
  - [`lconv`](#lconv)
  - [`in_pktinfo`](#in-pktinfo)
  - [`ifaddrs`](#ifaddrs)
  - [`in6_rtmsg`](#in6-rtmsg)
  - [`arpreq`](#arpreq)
  - [`arpreq_old`](#arpreq-old)
  - [`arphdr`](#arphdr)
  - [`mmsghdr`](#mmsghdr)
  - [`sockaddr_un`](#sockaddr-un)
  - [`sockaddr_storage`](#sockaddr-storage)
  - [`utsname`](#utsname)
  - [`file_clone_range`](#file-clone-range)
  - [`sock_filter`](#sock-filter)
  - [`sock_fprog`](#sock-fprog)
  - [`statx`](#statx)
  - [`statx_timestamp`](#statx-timestamp)
  - [`epoll_event`](#epoll-event)
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
  - [`_exit`](#exit)
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
  - [`strtok_r`](#strtok-r)
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
  - [`getchar_unlocked`](#getchar-unlocked)
  - [`putchar_unlocked`](#putchar-unlocked)
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
  - [`posix_memalign`](#posix-memalign)
  - [`aligned_alloc`](#aligned-alloc)
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
  - [`ttyname_r`](#ttyname-r)
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
  - [`if_nametoindex`](#if-nametoindex)
  - [`if_indextoname`](#if-indextoname)
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
  - [`pthread_self`](#pthread-self)
  - [`pthread_equal`](#pthread-equal)
  - [`pthread_join`](#pthread-join)
  - [`pthread_exit`](#pthread-exit)
  - [`pthread_attr_init`](#pthread-attr-init)
  - [`pthread_attr_destroy`](#pthread-attr-destroy)
  - [`pthread_attr_getstacksize`](#pthread-attr-getstacksize)
  - [`pthread_attr_setstacksize`](#pthread-attr-setstacksize)
  - [`pthread_attr_setdetachstate`](#pthread-attr-setdetachstate)
  - [`pthread_detach`](#pthread-detach)
  - [`sched_yield`](#sched-yield)
  - [`pthread_key_create`](#pthread-key-create)
  - [`pthread_key_delete`](#pthread-key-delete)
  - [`pthread_getspecific`](#pthread-getspecific)
  - [`pthread_setspecific`](#pthread-setspecific)
  - [`pthread_mutex_init`](#pthread-mutex-init)
  - [`pthread_mutex_destroy`](#pthread-mutex-destroy)
  - [`pthread_mutex_lock`](#pthread-mutex-lock)
  - [`pthread_mutex_trylock`](#pthread-mutex-trylock)
  - [`pthread_mutex_unlock`](#pthread-mutex-unlock)
  - [`pthread_mutexattr_init`](#pthread-mutexattr-init)
  - [`pthread_mutexattr_destroy`](#pthread-mutexattr-destroy)
  - [`pthread_mutexattr_settype`](#pthread-mutexattr-settype)
  - [`pthread_cond_init`](#pthread-cond-init)
  - [`pthread_cond_wait`](#pthread-cond-wait)
  - [`pthread_cond_timedwait`](#pthread-cond-timedwait)
  - [`pthread_cond_signal`](#pthread-cond-signal)
  - [`pthread_cond_broadcast`](#pthread-cond-broadcast)
  - [`pthread_cond_destroy`](#pthread-cond-destroy)
  - [`pthread_condattr_init`](#pthread-condattr-init)
  - [`pthread_condattr_destroy`](#pthread-condattr-destroy)
  - [`pthread_rwlock_init`](#pthread-rwlock-init)
  - [`pthread_rwlock_destroy`](#pthread-rwlock-destroy)
  - [`pthread_rwlock_rdlock`](#pthread-rwlock-rdlock)
  - [`pthread_rwlock_tryrdlock`](#pthread-rwlock-tryrdlock)
  - [`pthread_rwlock_wrlock`](#pthread-rwlock-wrlock)
  - [`pthread_rwlock_trywrlock`](#pthread-rwlock-trywrlock)
  - [`pthread_rwlock_unlock`](#pthread-rwlock-unlock)
  - [`pthread_rwlockattr_init`](#pthread-rwlockattr-init)
  - [`pthread_rwlockattr_destroy`](#pthread-rwlockattr-destroy)
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
  - [`gai_strerror`](#gai-strerror)
  - [`res_init`](#res-init)
  - [`gmtime_r`](#gmtime-r)
  - [`localtime_r`](#localtime-r)
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
  - [`sem_wait`](#sem-wait)
  - [`sem_trywait`](#sem-trywait)
  - [`sem_post`](#sem-post)
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
  - [`posix_openpt`](#posix-openpt)
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
  - [`open_wmemstream`](#open-wmemstream)
  - [`getsid`](#getsid)
  - [`pause`](#pause)
  - [`mkdirat`](#mkdirat)
  - [`openat`](#openat)
  - [`fdopendir`](#fdopendir)
  - [`readdir_r`](#readdir-r)
  - [`readlinkat`](#readlinkat)
  - [`fmemopen`](#fmemopen)
  - [`open_memstream`](#open-memstream)
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
  - [`sem_destroy`](#sem-destroy)
  - [`sem_init`](#sem-init)
  - [`fdatasync`](#fdatasync)
  - [`mincore`](#mincore)
  - [`clock_getres`](#clock-getres)
  - [`clock_gettime`](#clock-gettime)
  - [`clock_settime`](#clock-settime)
  - [`clock_getcpuclockid`](#clock-getcpuclockid)
  - [`dirfd`](#dirfd)
  - [`memalign`](#memalign)
  - [`setgroups`](#setgroups)
  - [`pipe2`](#pipe2)
  - [`statfs`](#statfs)
  - [`fstatfs`](#fstatfs)
  - [`memrchr`](#memrchr)
  - [`posix_fadvise`](#posix-fadvise)
  - [`futimens`](#futimens)
  - [`utimensat`](#utimensat)
  - [`duplocale`](#duplocale)
  - [`freelocale`](#freelocale)
  - [`newlocale`](#newlocale)
  - [`uselocale`](#uselocale)
  - [`mknodat`](#mknodat)
  - [`ptsname_r`](#ptsname-r)
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
  - [`login_tty`](#login-tty)
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
  - [`strftime_l`](#strftime-l)
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
  - [`posix_fadvise64`](#posix-fadvise64)
  - [`pread64`](#pread64)
  - [`pwrite64`](#pwrite64)
  - [`readdir64`](#readdir64)
  - [`readdir64_r`](#readdir64-r)
  - [`stat64`](#stat64)
  - [`truncate64`](#truncate64)
  - [`preadv64`](#preadv64)
  - [`pwritev64`](#pwritev64)
  - [`forkpty`](#forkpty)
  - [`openpty`](#openpty)
  - [`statx`](#statx)
  - [`_IOC`](#ioc)
  - [`_IO`](#io)
  - [`_IOR`](#ior)
  - [`_IOW`](#iow)
  - [`_IOWR`](#iowr)
  - [`CMSG_ALIGN`](#cmsg-align)
  - [`CMSG_FIRSTHDR`](#cmsg-firsthdr)
  - [`CMSG_DATA`](#cmsg-data)
  - [`CMSG_SPACE`](#cmsg-space)
  - [`CMSG_LEN`](#cmsg-len)
  - [`FD_CLR`](#fd-clr)
  - [`FD_ISSET`](#fd-isset)
  - [`FD_SET`](#fd-set)
  - [`FD_ZERO`](#fd-zero)
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
  - [`W_EXITCODE`](#w-exitcode)
  - [`W_STOPCODE`](#w-stopcode)
  - [`QCMD`](#qcmd)
  - [`IPOPT_COPIED`](#ipopt-copied)
  - [`IPOPT_CLASS`](#ipopt-class)
  - [`IPOPT_NUMBER`](#ipopt-number)
  - [`IPTOS_ECN`](#iptos-ecn)
  - [`KERNEL_VERSION`](#kernel-version)
- [Type Aliases](#type-aliases)
  - [`intmax_t`](#intmax-t)
  - [`uintmax_t`](#uintmax-t)
  - [`size_t`](#size-t)
  - [`ptrdiff_t`](#ptrdiff-t)
  - [`intptr_t`](#intptr-t)
  - [`uintptr_t`](#uintptr-t)
  - [`ssize_t`](#ssize-t)
  - [`pid_t`](#pid-t)
  - [`in_addr_t`](#in-addr-t)
  - [`in_port_t`](#in-port-t)
  - [`sighandler_t`](#sighandler-t)
  - [`cc_t`](#cc-t)
  - [`uid_t`](#uid-t)
  - [`gid_t`](#gid-t)
  - [`locale_t`](#locale-t)
  - [`sa_family_t`](#sa-family-t)
  - [`speed_t`](#speed-t)
  - [`tcflag_t`](#tcflag-t)
  - [`clockid_t`](#clockid-t)
  - [`timer_t`](#timer-t)
  - [`key_t`](#key-t)
  - [`id_t`](#id-t)
- [Constants](#constants)
  - [`INT_MIN`](#int-min)
  - [`INT_MAX`](#int-max)
  - [`SIG_DFL`](#sig-dfl)
  - [`SIG_IGN`](#sig-ign)
  - [`SIG_ERR`](#sig-err)
  - [`DT_UNKNOWN`](#dt-unknown)
  - [`DT_FIFO`](#dt-fifo)
  - [`DT_CHR`](#dt-chr)
  - [`DT_DIR`](#dt-dir)
  - [`DT_BLK`](#dt-blk)
  - [`DT_REG`](#dt-reg)
  - [`DT_LNK`](#dt-lnk)
  - [`DT_SOCK`](#dt-sock)
  - [`FD_CLOEXEC`](#fd-cloexec)
  - [`USRQUOTA`](#usrquota)
  - [`GRPQUOTA`](#grpquota)
  - [`SIGIOT`](#sigiot)
  - [`S_ISUID`](#s-isuid)
  - [`S_ISGID`](#s-isgid)
  - [`S_ISVTX`](#s-isvtx)
  - [`IF_NAMESIZE`](#if-namesize)
  - [`IFNAMSIZ`](#ifnamsiz)
  - [`LOG_EMERG`](#log-emerg)
  - [`LOG_ALERT`](#log-alert)
  - [`LOG_CRIT`](#log-crit)
  - [`LOG_ERR`](#log-err)
  - [`LOG_WARNING`](#log-warning)
  - [`LOG_NOTICE`](#log-notice)
  - [`LOG_INFO`](#log-info)
  - [`LOG_DEBUG`](#log-debug)
  - [`LOG_KERN`](#log-kern)
  - [`LOG_USER`](#log-user)
  - [`LOG_MAIL`](#log-mail)
  - [`LOG_DAEMON`](#log-daemon)
  - [`LOG_AUTH`](#log-auth)
  - [`LOG_SYSLOG`](#log-syslog)
  - [`LOG_LPR`](#log-lpr)
  - [`LOG_NEWS`](#log-news)
  - [`LOG_UUCP`](#log-uucp)
  - [`LOG_LOCAL0`](#log-local0)
  - [`LOG_LOCAL1`](#log-local1)
  - [`LOG_LOCAL2`](#log-local2)
  - [`LOG_LOCAL3`](#log-local3)
  - [`LOG_LOCAL4`](#log-local4)
  - [`LOG_LOCAL5`](#log-local5)
  - [`LOG_LOCAL6`](#log-local6)
  - [`LOG_LOCAL7`](#log-local7)
  - [`LOG_PID`](#log-pid)
  - [`LOG_CONS`](#log-cons)
  - [`LOG_ODELAY`](#log-odelay)
  - [`LOG_NDELAY`](#log-ndelay)
  - [`LOG_NOWAIT`](#log-nowait)
  - [`LOG_PRIMASK`](#log-primask)
  - [`LOG_FACMASK`](#log-facmask)
  - [`PRIO_MIN`](#prio-min)
  - [`PRIO_MAX`](#prio-max)
  - [`IPPROTO_ICMP`](#ipproto-icmp)
  - [`IPPROTO_ICMPV6`](#ipproto-icmpv6)
  - [`IPPROTO_TCP`](#ipproto-tcp)
  - [`IPPROTO_UDP`](#ipproto-udp)
  - [`IPPROTO_IP`](#ipproto-ip)
  - [`IPPROTO_IPV6`](#ipproto-ipv6)
  - [`INADDR_LOOPBACK`](#inaddr-loopback)
  - [`INADDR_ANY`](#inaddr-any)
  - [`INADDR_BROADCAST`](#inaddr-broadcast)
  - [`INADDR_NONE`](#inaddr-none)
  - [`IN6ADDR_LOOPBACK_INIT`](#in6addr-loopback-init)
  - [`IN6ADDR_ANY_INIT`](#in6addr-any-init)
  - [`ARPOP_REQUEST`](#arpop-request)
  - [`ARPOP_REPLY`](#arpop-reply)
  - [`ATF_COM`](#atf-com)
  - [`ATF_PERM`](#atf-perm)
  - [`ATF_PUBL`](#atf-publ)
  - [`ATF_USETRAILERS`](#atf-usetrailers)
  - [`FNM_PERIOD`](#fnm-period)
  - [`FNM_NOMATCH`](#fnm-nomatch)
  - [`FNM_CASEFOLD`](#fnm-casefold)
  - [`FNM_PATHNAME`](#fnm-pathname)
  - [`FNM_NOESCAPE`](#fnm-noescape)
  - [`ULONG_SIZE`](#ulong-size)
  - [`EXIT_FAILURE`](#exit-failure)
  - [`EXIT_SUCCESS`](#exit-success)
  - [`RAND_MAX`](#rand-max)
  - [`EOF`](#eof)
  - [`SEEK_SET`](#seek-set)
  - [`SEEK_CUR`](#seek-cur)
  - [`SEEK_END`](#seek-end)
  - [`_IOFBF`](#iofbf)
  - [`_IONBF`](#ionbf)
  - [`_IOLBF`](#iolbf)
  - [`F_DUPFD`](#f-dupfd)
  - [`F_GETFD`](#f-getfd)
  - [`F_SETFD`](#f-setfd)
  - [`F_GETFL`](#f-getfl)
  - [`F_SETFL`](#f-setfl)
  - [`F_SETLEASE`](#f-setlease)
  - [`F_GETLEASE`](#f-getlease)
  - [`F_NOTIFY`](#f-notify)
  - [`F_CANCELLK`](#f-cancellk)
  - [`F_DUPFD_CLOEXEC`](#f-dupfd-cloexec)
  - [`F_SETPIPE_SZ`](#f-setpipe-sz)
  - [`F_GETPIPE_SZ`](#f-getpipe-sz)
  - [`F_ADD_SEALS`](#f-add-seals)
  - [`F_GET_SEALS`](#f-get-seals)
  - [`F_SEAL_SEAL`](#f-seal-seal)
  - [`F_SEAL_SHRINK`](#f-seal-shrink)
  - [`F_SEAL_GROW`](#f-seal-grow)
  - [`F_SEAL_WRITE`](#f-seal-write)
  - [`SIGTRAP`](#sigtrap)
  - [`PTHREAD_CREATE_JOINABLE`](#pthread-create-joinable)
  - [`PTHREAD_CREATE_DETACHED`](#pthread-create-detached)
  - [`CLOCK_REALTIME`](#clock-realtime)
  - [`CLOCK_MONOTONIC`](#clock-monotonic)
  - [`CLOCK_PROCESS_CPUTIME_ID`](#clock-process-cputime-id)
  - [`CLOCK_THREAD_CPUTIME_ID`](#clock-thread-cputime-id)
  - [`CLOCK_MONOTONIC_RAW`](#clock-monotonic-raw)
  - [`CLOCK_REALTIME_COARSE`](#clock-realtime-coarse)
  - [`CLOCK_MONOTONIC_COARSE`](#clock-monotonic-coarse)
  - [`CLOCK_BOOTTIME`](#clock-boottime)
  - [`CLOCK_REALTIME_ALARM`](#clock-realtime-alarm)
  - [`CLOCK_BOOTTIME_ALARM`](#clock-boottime-alarm)
  - [`CLOCK_TAI`](#clock-tai)
  - [`TIMER_ABSTIME`](#timer-abstime)
  - [`RUSAGE_SELF`](#rusage-self)
  - [`O_RDONLY`](#o-rdonly)
  - [`O_WRONLY`](#o-wronly)
  - [`O_RDWR`](#o-rdwr)
  - [`SOCK_CLOEXEC`](#sock-cloexec)
  - [`S_IFIFO`](#s-ififo)
  - [`S_IFCHR`](#s-ifchr)
  - [`S_IFBLK`](#s-ifblk)
  - [`S_IFDIR`](#s-ifdir)
  - [`S_IFREG`](#s-ifreg)
  - [`S_IFLNK`](#s-iflnk)
  - [`S_IFSOCK`](#s-ifsock)
  - [`S_IFMT`](#s-ifmt)
  - [`S_IRWXU`](#s-irwxu)
  - [`S_IXUSR`](#s-ixusr)
  - [`S_IWUSR`](#s-iwusr)
  - [`S_IRUSR`](#s-irusr)
  - [`S_IRWXG`](#s-irwxg)
  - [`S_IXGRP`](#s-ixgrp)
  - [`S_IWGRP`](#s-iwgrp)
  - [`S_IRGRP`](#s-irgrp)
  - [`S_IRWXO`](#s-irwxo)
  - [`S_IXOTH`](#s-ixoth)
  - [`S_IWOTH`](#s-iwoth)
  - [`S_IROTH`](#s-iroth)
  - [`F_OK`](#f-ok)
  - [`R_OK`](#r-ok)
  - [`W_OK`](#w-ok)
  - [`X_OK`](#x-ok)
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
  - [`PROT_NONE`](#prot-none)
  - [`PROT_READ`](#prot-read)
  - [`PROT_WRITE`](#prot-write)
  - [`PROT_EXEC`](#prot-exec)
  - [`XATTR_CREATE`](#xattr-create)
  - [`XATTR_REPLACE`](#xattr-replace)
  - [`RLIM64_INFINITY`](#rlim64-infinity)
  - [`LC_CTYPE`](#lc-ctype)
  - [`LC_NUMERIC`](#lc-numeric)
  - [`LC_TIME`](#lc-time)
  - [`LC_COLLATE`](#lc-collate)
  - [`LC_MONETARY`](#lc-monetary)
  - [`LC_MESSAGES`](#lc-messages)
  - [`LC_ALL`](#lc-all)
  - [`LC_CTYPE_MASK`](#lc-ctype-mask)
  - [`LC_NUMERIC_MASK`](#lc-numeric-mask)
  - [`LC_TIME_MASK`](#lc-time-mask)
  - [`LC_COLLATE_MASK`](#lc-collate-mask)
  - [`LC_MONETARY_MASK`](#lc-monetary-mask)
  - [`LC_MESSAGES_MASK`](#lc-messages-mask)
  - [`MAP_FILE`](#map-file)
  - [`MAP_SHARED`](#map-shared)
  - [`MAP_PRIVATE`](#map-private)
  - [`MAP_FIXED`](#map-fixed)
  - [`MAP_FAILED`](#map-failed)
  - [`MS_ASYNC`](#ms-async)
  - [`MS_INVALIDATE`](#ms-invalidate)
  - [`MS_SYNC`](#ms-sync)
  - [`MS_RDONLY`](#ms-rdonly)
  - [`MS_NOSUID`](#ms-nosuid)
  - [`MS_NODEV`](#ms-nodev)
  - [`MS_NOEXEC`](#ms-noexec)
  - [`MS_SYNCHRONOUS`](#ms-synchronous)
  - [`MS_REMOUNT`](#ms-remount)
  - [`MS_MANDLOCK`](#ms-mandlock)
  - [`MS_DIRSYNC`](#ms-dirsync)
  - [`MS_NOSYMFOLLOW`](#ms-nosymfollow)
  - [`MS_NOATIME`](#ms-noatime)
  - [`MS_NODIRATIME`](#ms-nodiratime)
  - [`MS_BIND`](#ms-bind)
  - [`MS_MOVE`](#ms-move)
  - [`MS_REC`](#ms-rec)
  - [`MS_SILENT`](#ms-silent)
  - [`MS_POSIXACL`](#ms-posixacl)
  - [`MS_UNBINDABLE`](#ms-unbindable)
  - [`MS_PRIVATE`](#ms-private)
  - [`MS_SLAVE`](#ms-slave)
  - [`MS_SHARED`](#ms-shared)
  - [`MS_RELATIME`](#ms-relatime)
  - [`MS_KERNMOUNT`](#ms-kernmount)
  - [`MS_I_VERSION`](#ms-i-version)
  - [`MS_STRICTATIME`](#ms-strictatime)
  - [`MS_LAZYTIME`](#ms-lazytime)
  - [`MS_ACTIVE`](#ms-active)
  - [`MS_MGC_VAL`](#ms-mgc-val)
  - [`MS_MGC_MSK`](#ms-mgc-msk)
  - [`SCM_RIGHTS`](#scm-rights)
  - [`SCM_CREDENTIALS`](#scm-credentials)
  - [`PROT_GROWSDOWN`](#prot-growsdown)
  - [`PROT_GROWSUP`](#prot-growsup)
  - [`MAP_TYPE`](#map-type)
  - [`MADV_NORMAL`](#madv-normal)
  - [`MADV_RANDOM`](#madv-random)
  - [`MADV_SEQUENTIAL`](#madv-sequential)
  - [`MADV_WILLNEED`](#madv-willneed)
  - [`MADV_DONTNEED`](#madv-dontneed)
  - [`MADV_FREE`](#madv-free)
  - [`MADV_REMOVE`](#madv-remove)
  - [`MADV_DONTFORK`](#madv-dontfork)
  - [`MADV_DOFORK`](#madv-dofork)
  - [`MADV_MERGEABLE`](#madv-mergeable)
  - [`MADV_UNMERGEABLE`](#madv-unmergeable)
  - [`MADV_HUGEPAGE`](#madv-hugepage)
  - [`MADV_NOHUGEPAGE`](#madv-nohugepage)
  - [`MADV_DONTDUMP`](#madv-dontdump)
  - [`MADV_DODUMP`](#madv-dodump)
  - [`MADV_WIPEONFORK`](#madv-wipeonfork)
  - [`MADV_KEEPONFORK`](#madv-keeponfork)
  - [`MADV_COLD`](#madv-cold)
  - [`MADV_PAGEOUT`](#madv-pageout)
  - [`MADV_HWPOISON`](#madv-hwpoison)
  - [`MADV_POPULATE_READ`](#madv-populate-read)
  - [`MADV_POPULATE_WRITE`](#madv-populate-write)
  - [`MADV_DONTNEED_LOCKED`](#madv-dontneed-locked)
  - [`IFF_UP`](#iff-up)
  - [`IFF_BROADCAST`](#iff-broadcast)
  - [`IFF_DEBUG`](#iff-debug)
  - [`IFF_LOOPBACK`](#iff-loopback)
  - [`IFF_POINTOPOINT`](#iff-pointopoint)
  - [`IFF_NOTRAILERS`](#iff-notrailers)
  - [`IFF_RUNNING`](#iff-running)
  - [`IFF_NOARP`](#iff-noarp)
  - [`IFF_PROMISC`](#iff-promisc)
  - [`IFF_ALLMULTI`](#iff-allmulti)
  - [`IFF_MASTER`](#iff-master)
  - [`IFF_SLAVE`](#iff-slave)
  - [`IFF_MULTICAST`](#iff-multicast)
  - [`IFF_PORTSEL`](#iff-portsel)
  - [`IFF_AUTOMEDIA`](#iff-automedia)
  - [`IFF_DYNAMIC`](#iff-dynamic)
  - [`SOL_IP`](#sol-ip)
  - [`SOL_TCP`](#sol-tcp)
  - [`SOL_UDP`](#sol-udp)
  - [`SOL_IPV6`](#sol-ipv6)
  - [`SOL_ICMPV6`](#sol-icmpv6)
  - [`SOL_RAW`](#sol-raw)
  - [`SOL_DECNET`](#sol-decnet)
  - [`SOL_X25`](#sol-x25)
  - [`SOL_PACKET`](#sol-packet)
  - [`SOL_ATM`](#sol-atm)
  - [`SOL_AAL`](#sol-aal)
  - [`SOL_IRDA`](#sol-irda)
  - [`SOL_NETBEUI`](#sol-netbeui)
  - [`SOL_LLC`](#sol-llc)
  - [`SOL_DCCP`](#sol-dccp)
  - [`SOL_NETLINK`](#sol-netlink)
  - [`SOL_TIPC`](#sol-tipc)
  - [`SOL_BLUETOOTH`](#sol-bluetooth)
  - [`SOL_ALG`](#sol-alg)
  - [`AF_UNSPEC`](#af-unspec)
  - [`AF_UNIX`](#af-unix)
  - [`AF_LOCAL`](#af-local)
  - [`AF_INET`](#af-inet)
  - [`AF_AX25`](#af-ax25)
  - [`AF_IPX`](#af-ipx)
  - [`AF_APPLETALK`](#af-appletalk)
  - [`AF_NETROM`](#af-netrom)
  - [`AF_BRIDGE`](#af-bridge)
  - [`AF_ATMPVC`](#af-atmpvc)
  - [`AF_X25`](#af-x25)
  - [`AF_INET6`](#af-inet6)
  - [`AF_ROSE`](#af-rose)
  - [`AF_DECnet`](#af-decnet)
  - [`AF_NETBEUI`](#af-netbeui)
  - [`AF_SECURITY`](#af-security)
  - [`AF_KEY`](#af-key)
  - [`AF_NETLINK`](#af-netlink)
  - [`AF_ROUTE`](#af-route)
  - [`AF_PACKET`](#af-packet)
  - [`AF_ASH`](#af-ash)
  - [`AF_ECONET`](#af-econet)
  - [`AF_ATMSVC`](#af-atmsvc)
  - [`AF_RDS`](#af-rds)
  - [`AF_SNA`](#af-sna)
  - [`AF_IRDA`](#af-irda)
  - [`AF_PPPOX`](#af-pppox)
  - [`AF_WANPIPE`](#af-wanpipe)
  - [`AF_LLC`](#af-llc)
  - [`AF_CAN`](#af-can)
  - [`AF_TIPC`](#af-tipc)
  - [`AF_BLUETOOTH`](#af-bluetooth)
  - [`AF_IUCV`](#af-iucv)
  - [`AF_RXRPC`](#af-rxrpc)
  - [`AF_ISDN`](#af-isdn)
  - [`AF_PHONET`](#af-phonet)
  - [`AF_IEEE802154`](#af-ieee802154)
  - [`AF_CAIF`](#af-caif)
  - [`AF_ALG`](#af-alg)
  - [`PF_UNSPEC`](#pf-unspec)
  - [`PF_UNIX`](#pf-unix)
  - [`PF_LOCAL`](#pf-local)
  - [`PF_INET`](#pf-inet)
  - [`PF_AX25`](#pf-ax25)
  - [`PF_IPX`](#pf-ipx)
  - [`PF_APPLETALK`](#pf-appletalk)
  - [`PF_NETROM`](#pf-netrom)
  - [`PF_BRIDGE`](#pf-bridge)
  - [`PF_ATMPVC`](#pf-atmpvc)
  - [`PF_X25`](#pf-x25)
  - [`PF_INET6`](#pf-inet6)
  - [`PF_ROSE`](#pf-rose)
  - [`PF_DECnet`](#pf-decnet)
  - [`PF_NETBEUI`](#pf-netbeui)
  - [`PF_SECURITY`](#pf-security)
  - [`PF_KEY`](#pf-key)
  - [`PF_NETLINK`](#pf-netlink)
  - [`PF_ROUTE`](#pf-route)
  - [`PF_PACKET`](#pf-packet)
  - [`PF_ASH`](#pf-ash)
  - [`PF_ECONET`](#pf-econet)
  - [`PF_ATMSVC`](#pf-atmsvc)
  - [`PF_RDS`](#pf-rds)
  - [`PF_SNA`](#pf-sna)
  - [`PF_IRDA`](#pf-irda)
  - [`PF_PPPOX`](#pf-pppox)
  - [`PF_WANPIPE`](#pf-wanpipe)
  - [`PF_LLC`](#pf-llc)
  - [`PF_CAN`](#pf-can)
  - [`PF_TIPC`](#pf-tipc)
  - [`PF_BLUETOOTH`](#pf-bluetooth)
  - [`PF_IUCV`](#pf-iucv)
  - [`PF_RXRPC`](#pf-rxrpc)
  - [`PF_ISDN`](#pf-isdn)
  - [`PF_PHONET`](#pf-phonet)
  - [`PF_IEEE802154`](#pf-ieee802154)
  - [`PF_CAIF`](#pf-caif)
  - [`PF_ALG`](#pf-alg)
  - [`MSG_OOB`](#msg-oob)
  - [`MSG_PEEK`](#msg-peek)
  - [`MSG_DONTROUTE`](#msg-dontroute)
  - [`MSG_CTRUNC`](#msg-ctrunc)
  - [`MSG_TRUNC`](#msg-trunc)
  - [`MSG_DONTWAIT`](#msg-dontwait)
  - [`MSG_EOR`](#msg-eor)
  - [`MSG_WAITALL`](#msg-waitall)
  - [`MSG_FIN`](#msg-fin)
  - [`MSG_SYN`](#msg-syn)
  - [`MSG_CONFIRM`](#msg-confirm)
  - [`MSG_RST`](#msg-rst)
  - [`MSG_ERRQUEUE`](#msg-errqueue)
  - [`MSG_NOSIGNAL`](#msg-nosignal)
  - [`MSG_MORE`](#msg-more)
  - [`MSG_WAITFORONE`](#msg-waitforone)
  - [`MSG_FASTOPEN`](#msg-fastopen)
  - [`MSG_CMSG_CLOEXEC`](#msg-cmsg-cloexec)
  - [`SCM_TIMESTAMP`](#scm-timestamp)
  - [`SOCK_RAW`](#sock-raw)
  - [`SOCK_RDM`](#sock-rdm)
  - [`IP_TOS`](#ip-tos)
  - [`IP_TTL`](#ip-ttl)
  - [`IP_HDRINCL`](#ip-hdrincl)
  - [`IP_OPTIONS`](#ip-options)
  - [`IP_ROUTER_ALERT`](#ip-router-alert)
  - [`IP_RECVOPTS`](#ip-recvopts)
  - [`IP_RETOPTS`](#ip-retopts)
  - [`IP_PKTINFO`](#ip-pktinfo)
  - [`IP_PKTOPTIONS`](#ip-pktoptions)
  - [`IP_MTU_DISCOVER`](#ip-mtu-discover)
  - [`IP_RECVERR`](#ip-recverr)
  - [`IP_RECVTTL`](#ip-recvttl)
  - [`IP_RECVTOS`](#ip-recvtos)
  - [`IP_MTU`](#ip-mtu)
  - [`IP_FREEBIND`](#ip-freebind)
  - [`IP_IPSEC_POLICY`](#ip-ipsec-policy)
  - [`IP_XFRM_POLICY`](#ip-xfrm-policy)
  - [`IP_PASSSEC`](#ip-passsec)
  - [`IP_TRANSPARENT`](#ip-transparent)
  - [`IP_ORIGDSTADDR`](#ip-origdstaddr)
  - [`IP_RECVORIGDSTADDR`](#ip-recvorigdstaddr)
  - [`IP_MINTTL`](#ip-minttl)
  - [`IP_NODEFRAG`](#ip-nodefrag)
  - [`IP_CHECKSUM`](#ip-checksum)
  - [`IP_BIND_ADDRESS_NO_PORT`](#ip-bind-address-no-port)
  - [`IP_MULTICAST_IF`](#ip-multicast-if)
  - [`IP_MULTICAST_TTL`](#ip-multicast-ttl)
  - [`IP_MULTICAST_LOOP`](#ip-multicast-loop)
  - [`IP_ADD_MEMBERSHIP`](#ip-add-membership)
  - [`IP_DROP_MEMBERSHIP`](#ip-drop-membership)
  - [`IP_UNBLOCK_SOURCE`](#ip-unblock-source)
  - [`IP_BLOCK_SOURCE`](#ip-block-source)
  - [`IP_ADD_SOURCE_MEMBERSHIP`](#ip-add-source-membership)
  - [`IP_DROP_SOURCE_MEMBERSHIP`](#ip-drop-source-membership)
  - [`IP_MSFILTER`](#ip-msfilter)
  - [`IP_MULTICAST_ALL`](#ip-multicast-all)
  - [`IP_UNICAST_IF`](#ip-unicast-if)
  - [`IP_DEFAULT_MULTICAST_TTL`](#ip-default-multicast-ttl)
  - [`IP_DEFAULT_MULTICAST_LOOP`](#ip-default-multicast-loop)
  - [`IP_PMTUDISC_DONT`](#ip-pmtudisc-dont)
  - [`IP_PMTUDISC_WANT`](#ip-pmtudisc-want)
  - [`IP_PMTUDISC_DO`](#ip-pmtudisc-do)
  - [`IP_PMTUDISC_PROBE`](#ip-pmtudisc-probe)
  - [`IP_PMTUDISC_INTERFACE`](#ip-pmtudisc-interface)
  - [`IP_PMTUDISC_OMIT`](#ip-pmtudisc-omit)
  - [`IPPROTO_HOPOPTS`](#ipproto-hopopts)
  - [`IPPROTO_IGMP`](#ipproto-igmp)
  - [`IPPROTO_IPIP`](#ipproto-ipip)
  - [`IPPROTO_EGP`](#ipproto-egp)
  - [`IPPROTO_PUP`](#ipproto-pup)
  - [`IPPROTO_IDP`](#ipproto-idp)
  - [`IPPROTO_TP`](#ipproto-tp)
  - [`IPPROTO_DCCP`](#ipproto-dccp)
  - [`IPPROTO_ROUTING`](#ipproto-routing)
  - [`IPPROTO_FRAGMENT`](#ipproto-fragment)
  - [`IPPROTO_RSVP`](#ipproto-rsvp)
  - [`IPPROTO_GRE`](#ipproto-gre)
  - [`IPPROTO_ESP`](#ipproto-esp)
  - [`IPPROTO_AH`](#ipproto-ah)
  - [`IPPROTO_NONE`](#ipproto-none)
  - [`IPPROTO_DSTOPTS`](#ipproto-dstopts)
  - [`IPPROTO_MTP`](#ipproto-mtp)
  - [`IPPROTO_ENCAP`](#ipproto-encap)
  - [`IPPROTO_PIM`](#ipproto-pim)
  - [`IPPROTO_COMP`](#ipproto-comp)
  - [`IPPROTO_SCTP`](#ipproto-sctp)
  - [`IPPROTO_MH`](#ipproto-mh)
  - [`IPPROTO_UDPLITE`](#ipproto-udplite)
  - [`IPPROTO_RAW`](#ipproto-raw)
  - [`IPPROTO_BEETPH`](#ipproto-beetph)
  - [`IPPROTO_MPLS`](#ipproto-mpls)
  - [`IPPROTO_MPTCP`](#ipproto-mptcp)
  - [`IPPROTO_ETHERNET`](#ipproto-ethernet)
  - [`MCAST_EXCLUDE`](#mcast-exclude)
  - [`MCAST_INCLUDE`](#mcast-include)
  - [`MCAST_JOIN_GROUP`](#mcast-join-group)
  - [`MCAST_BLOCK_SOURCE`](#mcast-block-source)
  - [`MCAST_UNBLOCK_SOURCE`](#mcast-unblock-source)
  - [`MCAST_LEAVE_GROUP`](#mcast-leave-group)
  - [`MCAST_JOIN_SOURCE_GROUP`](#mcast-join-source-group)
  - [`MCAST_LEAVE_SOURCE_GROUP`](#mcast-leave-source-group)
  - [`MCAST_MSFILTER`](#mcast-msfilter)
  - [`IPV6_ADDRFORM`](#ipv6-addrform)
  - [`IPV6_2292PKTINFO`](#ipv6-2292pktinfo)
  - [`IPV6_2292HOPOPTS`](#ipv6-2292hopopts)
  - [`IPV6_2292DSTOPTS`](#ipv6-2292dstopts)
  - [`IPV6_2292RTHDR`](#ipv6-2292rthdr)
  - [`IPV6_2292PKTOPTIONS`](#ipv6-2292pktoptions)
  - [`IPV6_CHECKSUM`](#ipv6-checksum)
  - [`IPV6_2292HOPLIMIT`](#ipv6-2292hoplimit)
  - [`IPV6_NEXTHOP`](#ipv6-nexthop)
  - [`IPV6_AUTHHDR`](#ipv6-authhdr)
  - [`IPV6_UNICAST_HOPS`](#ipv6-unicast-hops)
  - [`IPV6_MULTICAST_IF`](#ipv6-multicast-if)
  - [`IPV6_MULTICAST_HOPS`](#ipv6-multicast-hops)
  - [`IPV6_MULTICAST_LOOP`](#ipv6-multicast-loop)
  - [`IPV6_ADD_MEMBERSHIP`](#ipv6-add-membership)
  - [`IPV6_DROP_MEMBERSHIP`](#ipv6-drop-membership)
  - [`IPV6_ROUTER_ALERT`](#ipv6-router-alert)
  - [`IPV6_MTU_DISCOVER`](#ipv6-mtu-discover)
  - [`IPV6_MTU`](#ipv6-mtu)
  - [`IPV6_RECVERR`](#ipv6-recverr)
  - [`IPV6_V6ONLY`](#ipv6-v6only)
  - [`IPV6_JOIN_ANYCAST`](#ipv6-join-anycast)
  - [`IPV6_LEAVE_ANYCAST`](#ipv6-leave-anycast)
  - [`IPV6_IPSEC_POLICY`](#ipv6-ipsec-policy)
  - [`IPV6_XFRM_POLICY`](#ipv6-xfrm-policy)
  - [`IPV6_HDRINCL`](#ipv6-hdrincl)
  - [`IPV6_RECVPKTINFO`](#ipv6-recvpktinfo)
  - [`IPV6_PKTINFO`](#ipv6-pktinfo)
  - [`IPV6_RECVHOPLIMIT`](#ipv6-recvhoplimit)
  - [`IPV6_HOPLIMIT`](#ipv6-hoplimit)
  - [`IPV6_RECVHOPOPTS`](#ipv6-recvhopopts)
  - [`IPV6_HOPOPTS`](#ipv6-hopopts)
  - [`IPV6_RTHDRDSTOPTS`](#ipv6-rthdrdstopts)
  - [`IPV6_RECVRTHDR`](#ipv6-recvrthdr)
  - [`IPV6_RTHDR`](#ipv6-rthdr)
  - [`IPV6_RECVDSTOPTS`](#ipv6-recvdstopts)
  - [`IPV6_DSTOPTS`](#ipv6-dstopts)
  - [`IPV6_RECVPATHMTU`](#ipv6-recvpathmtu)
  - [`IPV6_PATHMTU`](#ipv6-pathmtu)
  - [`IPV6_DONTFRAG`](#ipv6-dontfrag)
  - [`IPV6_RECVTCLASS`](#ipv6-recvtclass)
  - [`IPV6_TCLASS`](#ipv6-tclass)
  - [`IPV6_AUTOFLOWLABEL`](#ipv6-autoflowlabel)
  - [`IPV6_ADDR_PREFERENCES`](#ipv6-addr-preferences)
  - [`IPV6_MINHOPCOUNT`](#ipv6-minhopcount)
  - [`IPV6_ORIGDSTADDR`](#ipv6-origdstaddr)
  - [`IPV6_RECVORIGDSTADDR`](#ipv6-recvorigdstaddr)
  - [`IPV6_TRANSPARENT`](#ipv6-transparent)
  - [`IPV6_UNICAST_IF`](#ipv6-unicast-if)
  - [`IPV6_PREFER_SRC_TMP`](#ipv6-prefer-src-tmp)
  - [`IPV6_PREFER_SRC_PUBLIC`](#ipv6-prefer-src-public)
  - [`IPV6_PREFER_SRC_PUBTMP_DEFAULT`](#ipv6-prefer-src-pubtmp-default)
  - [`IPV6_PREFER_SRC_COA`](#ipv6-prefer-src-coa)
  - [`IPV6_PREFER_SRC_HOME`](#ipv6-prefer-src-home)
  - [`IPV6_PREFER_SRC_CGA`](#ipv6-prefer-src-cga)
  - [`IPV6_PREFER_SRC_NONCGA`](#ipv6-prefer-src-noncga)
  - [`IPV6_PMTUDISC_DONT`](#ipv6-pmtudisc-dont)
  - [`IPV6_PMTUDISC_WANT`](#ipv6-pmtudisc-want)
  - [`IPV6_PMTUDISC_DO`](#ipv6-pmtudisc-do)
  - [`IPV6_PMTUDISC_PROBE`](#ipv6-pmtudisc-probe)
  - [`IPV6_PMTUDISC_INTERFACE`](#ipv6-pmtudisc-interface)
  - [`IPV6_PMTUDISC_OMIT`](#ipv6-pmtudisc-omit)
  - [`TCP_NODELAY`](#tcp-nodelay)
  - [`TCP_MAXSEG`](#tcp-maxseg)
  - [`TCP_CORK`](#tcp-cork)
  - [`TCP_KEEPIDLE`](#tcp-keepidle)
  - [`TCP_KEEPINTVL`](#tcp-keepintvl)
  - [`TCP_KEEPCNT`](#tcp-keepcnt)
  - [`TCP_SYNCNT`](#tcp-syncnt)
  - [`TCP_LINGER2`](#tcp-linger2)
  - [`TCP_DEFER_ACCEPT`](#tcp-defer-accept)
  - [`TCP_WINDOW_CLAMP`](#tcp-window-clamp)
  - [`TCP_INFO`](#tcp-info)
  - [`TCP_QUICKACK`](#tcp-quickack)
  - [`TCP_CONGESTION`](#tcp-congestion)
  - [`TCP_MD5SIG`](#tcp-md5sig)
  - [`TCP_COOKIE_TRANSACTIONS`](#tcp-cookie-transactions)
  - [`TCP_THIN_LINEAR_TIMEOUTS`](#tcp-thin-linear-timeouts)
  - [`TCP_THIN_DUPACK`](#tcp-thin-dupack)
  - [`TCP_USER_TIMEOUT`](#tcp-user-timeout)
  - [`TCP_REPAIR`](#tcp-repair)
  - [`TCP_REPAIR_QUEUE`](#tcp-repair-queue)
  - [`TCP_QUEUE_SEQ`](#tcp-queue-seq)
  - [`TCP_REPAIR_OPTIONS`](#tcp-repair-options)
  - [`TCP_FASTOPEN`](#tcp-fastopen)
  - [`TCP_TIMESTAMP`](#tcp-timestamp)
  - [`TCP_NOTSENT_LOWAT`](#tcp-notsent-lowat)
  - [`TCP_CC_INFO`](#tcp-cc-info)
  - [`TCP_SAVE_SYN`](#tcp-save-syn)
  - [`TCP_SAVED_SYN`](#tcp-saved-syn)
  - [`TCP_REPAIR_WINDOW`](#tcp-repair-window)
  - [`TCP_FASTOPEN_CONNECT`](#tcp-fastopen-connect)
  - [`TCP_ULP`](#tcp-ulp)
  - [`TCP_MD5SIG_EXT`](#tcp-md5sig-ext)
  - [`TCP_FASTOPEN_KEY`](#tcp-fastopen-key)
  - [`TCP_FASTOPEN_NO_COOKIE`](#tcp-fastopen-no-cookie)
  - [`TCP_ZEROCOPY_RECEIVE`](#tcp-zerocopy-receive)
  - [`TCP_INQ`](#tcp-inq)
  - [`TCP_CM_INQ`](#tcp-cm-inq)
  - [`TCP_MD5SIG_MAXKEYLEN`](#tcp-md5sig-maxkeylen)
  - [`SO_DEBUG`](#so-debug)
  - [`SHUT_RD`](#shut-rd)
  - [`SHUT_WR`](#shut-wr)
  - [`SHUT_RDWR`](#shut-rdwr)
  - [`LOCK_SH`](#lock-sh)
  - [`LOCK_EX`](#lock-ex)
  - [`LOCK_NB`](#lock-nb)
  - [`LOCK_UN`](#lock-un)
  - [`SS_ONSTACK`](#ss-onstack)
  - [`SS_DISABLE`](#ss-disable)
  - [`PATH_MAX`](#path-max)
  - [`UIO_MAXIOV`](#uio-maxiov)
  - [`FD_SETSIZE`](#fd-setsize)
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
  - [`EPOLL_CTL_ADD`](#epoll-ctl-add)
  - [`EPOLL_CTL_MOD`](#epoll-ctl-mod)
  - [`EPOLL_CTL_DEL`](#epoll-ctl-del)
  - [`MNT_FORCE`](#mnt-force)
  - [`MNT_DETACH`](#mnt-detach)
  - [`MNT_EXPIRE`](#mnt-expire)
  - [`UMOUNT_NOFOLLOW`](#umount-nofollow)
  - [`Q_GETFMT`](#q-getfmt)
  - [`Q_GETINFO`](#q-getinfo)
  - [`Q_SETINFO`](#q-setinfo)
  - [`QIF_BLIMITS`](#qif-blimits)
  - [`QIF_SPACE`](#qif-space)
  - [`QIF_ILIMITS`](#qif-ilimits)
  - [`QIF_INODES`](#qif-inodes)
  - [`QIF_BTIME`](#qif-btime)
  - [`QIF_ITIME`](#qif-itime)
  - [`QIF_LIMITS`](#qif-limits)
  - [`QIF_USAGE`](#qif-usage)
  - [`QIF_TIMES`](#qif-times)
  - [`QIF_ALL`](#qif-all)
  - [`Q_SYNC`](#q-sync)
  - [`Q_QUOTAON`](#q-quotaon)
  - [`Q_QUOTAOFF`](#q-quotaoff)
  - [`Q_GETQUOTA`](#q-getquota)
  - [`Q_SETQUOTA`](#q-setquota)
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
  - [`CLONE_VM`](#clone-vm)
  - [`CLONE_FS`](#clone-fs)
  - [`CLONE_FILES`](#clone-files)
  - [`CLONE_SIGHAND`](#clone-sighand)
  - [`CLONE_PTRACE`](#clone-ptrace)
  - [`CLONE_VFORK`](#clone-vfork)
  - [`CLONE_PARENT`](#clone-parent)
  - [`CLONE_THREAD`](#clone-thread)
  - [`CLONE_NEWNS`](#clone-newns)
  - [`CLONE_SYSVSEM`](#clone-sysvsem)
  - [`CLONE_SETTLS`](#clone-settls)
  - [`CLONE_PARENT_SETTID`](#clone-parent-settid)
  - [`CLONE_CHILD_CLEARTID`](#clone-child-cleartid)
  - [`CLONE_DETACHED`](#clone-detached)
  - [`CLONE_UNTRACED`](#clone-untraced)
  - [`CLONE_CHILD_SETTID`](#clone-child-settid)
  - [`CLONE_NEWCGROUP`](#clone-newcgroup)
  - [`CLONE_NEWUTS`](#clone-newuts)
  - [`CLONE_NEWIPC`](#clone-newipc)
  - [`CLONE_NEWUSER`](#clone-newuser)
  - [`CLONE_NEWPID`](#clone-newpid)
  - [`CLONE_NEWNET`](#clone-newnet)
  - [`CLONE_IO`](#clone-io)
  - [`WNOHANG`](#wnohang)
  - [`WUNTRACED`](#wuntraced)
  - [`WSTOPPED`](#wstopped)
  - [`WEXITED`](#wexited)
  - [`WCONTINUED`](#wcontinued)
  - [`WNOWAIT`](#wnowait)
  - [`ADDR_NO_RANDOMIZE`](#addr-no-randomize)
  - [`MMAP_PAGE_ZERO`](#mmap-page-zero)
  - [`ADDR_COMPAT_LAYOUT`](#addr-compat-layout)
  - [`READ_IMPLIES_EXEC`](#read-implies-exec)
  - [`ADDR_LIMIT_32BIT`](#addr-limit-32bit)
  - [`SHORT_INODE`](#short-inode)
  - [`WHOLE_SECONDS`](#whole-seconds)
  - [`STICKY_TIMEOUTS`](#sticky-timeouts)
  - [`ADDR_LIMIT_3GB`](#addr-limit-3gb)
  - [`PTRACE_O_TRACESYSGOOD`](#ptrace-o-tracesysgood)
  - [`PTRACE_O_TRACEFORK`](#ptrace-o-tracefork)
  - [`PTRACE_O_TRACEVFORK`](#ptrace-o-tracevfork)
  - [`PTRACE_O_TRACECLONE`](#ptrace-o-traceclone)
  - [`PTRACE_O_TRACEEXEC`](#ptrace-o-traceexec)
  - [`PTRACE_O_TRACEVFORKDONE`](#ptrace-o-tracevforkdone)
  - [`PTRACE_O_TRACEEXIT`](#ptrace-o-traceexit)
  - [`PTRACE_O_TRACESECCOMP`](#ptrace-o-traceseccomp)
  - [`PTRACE_O_SUSPEND_SECCOMP`](#ptrace-o-suspend-seccomp)
  - [`PTRACE_O_EXITKILL`](#ptrace-o-exitkill)
  - [`PTRACE_O_MASK`](#ptrace-o-mask)
  - [`PTRACE_EVENT_FORK`](#ptrace-event-fork)
  - [`PTRACE_EVENT_VFORK`](#ptrace-event-vfork)
  - [`PTRACE_EVENT_CLONE`](#ptrace-event-clone)
  - [`PTRACE_EVENT_EXEC`](#ptrace-event-exec)
  - [`PTRACE_EVENT_VFORK_DONE`](#ptrace-event-vfork-done)
  - [`PTRACE_EVENT_EXIT`](#ptrace-event-exit)
  - [`PTRACE_EVENT_SECCOMP`](#ptrace-event-seccomp)
  - [`__WNOTHREAD`](#wnothread)
  - [`__WALL`](#wall)
  - [`__WCLONE`](#wclone)
  - [`SPLICE_F_MOVE`](#splice-f-move)
  - [`SPLICE_F_NONBLOCK`](#splice-f-nonblock)
  - [`SPLICE_F_MORE`](#splice-f-more)
  - [`SPLICE_F_GIFT`](#splice-f-gift)
  - [`RTLD_LOCAL`](#rtld-local)
  - [`RTLD_LAZY`](#rtld-lazy)
  - [`POSIX_FADV_NORMAL`](#posix-fadv-normal)
  - [`POSIX_FADV_RANDOM`](#posix-fadv-random)
  - [`POSIX_FADV_SEQUENTIAL`](#posix-fadv-sequential)
  - [`POSIX_FADV_WILLNEED`](#posix-fadv-willneed)
  - [`AT_FDCWD`](#at-fdcwd)
  - [`AT_SYMLINK_NOFOLLOW`](#at-symlink-nofollow)
  - [`AT_REMOVEDIR`](#at-removedir)
  - [`AT_SYMLINK_FOLLOW`](#at-symlink-follow)
  - [`AT_NO_AUTOMOUNT`](#at-no-automount)
  - [`AT_EMPTY_PATH`](#at-empty-path)
  - [`AT_RECURSIVE`](#at-recursive)
  - [`LOG_CRON`](#log-cron)
  - [`LOG_AUTHPRIV`](#log-authpriv)
  - [`LOG_FTP`](#log-ftp)
  - [`LOG_PERROR`](#log-perror)
  - [`PIPE_BUF`](#pipe-buf)
  - [`SI_LOAD_SHIFT`](#si-load-shift)
  - [`SI_USER`](#si-user)
  - [`SI_KERNEL`](#si-kernel)
  - [`SI_QUEUE`](#si-queue)
  - [`SI_TIMER`](#si-timer)
  - [`SI_MESGQ`](#si-mesgq)
  - [`SI_ASYNCIO`](#si-asyncio)
  - [`SI_SIGIO`](#si-sigio)
  - [`SI_TKILL`](#si-tkill)
  - [`SI_ASYNCNL`](#si-asyncnl)
  - [`BUS_ADRALN`](#bus-adraln)
  - [`BUS_ADRERR`](#bus-adrerr)
  - [`BUS_OBJERR`](#bus-objerr)
  - [`BUS_MCEERR_AR`](#bus-mceerr-ar)
  - [`BUS_MCEERR_AO`](#bus-mceerr-ao)
  - [`TRAP_BRKPT`](#trap-brkpt)
  - [`TRAP_TRACE`](#trap-trace)
  - [`TRAP_BRANCH`](#trap-branch)
  - [`TRAP_HWBKPT`](#trap-hwbkpt)
  - [`TRAP_UNK`](#trap-unk)
  - [`CLD_EXITED`](#cld-exited)
  - [`CLD_KILLED`](#cld-killed)
  - [`CLD_DUMPED`](#cld-dumped)
  - [`CLD_TRAPPED`](#cld-trapped)
  - [`CLD_STOPPED`](#cld-stopped)
  - [`CLD_CONTINUED`](#cld-continued)
  - [`SIGEV_SIGNAL`](#sigev-signal)
  - [`SIGEV_NONE`](#sigev-none)
  - [`SIGEV_THREAD`](#sigev-thread)
  - [`P_ALL`](#p-all)
  - [`P_PID`](#p-pid)
  - [`P_PGID`](#p-pgid)
  - [`P_PIDFD`](#p-pidfd)
  - [`UTIME_OMIT`](#utime-omit)
  - [`UTIME_NOW`](#utime-now)
  - [`POLLIN`](#pollin)
  - [`POLLPRI`](#pollpri)
  - [`POLLOUT`](#pollout)
  - [`POLLERR`](#pollerr)
  - [`POLLHUP`](#pollhup)
  - [`POLLNVAL`](#pollnval)
  - [`POLLRDNORM`](#pollrdnorm)
  - [`POLLRDBAND`](#pollrdband)
  - [`POLLRDHUP`](#pollrdhup)
  - [`IPTOS_LOWDELAY`](#iptos-lowdelay)
  - [`IPTOS_THROUGHPUT`](#iptos-throughput)
  - [`IPTOS_RELIABILITY`](#iptos-reliability)
  - [`IPTOS_MINCOST`](#iptos-mincost)
  - [`IPTOS_PREC_NETCONTROL`](#iptos-prec-netcontrol)
  - [`IPTOS_PREC_INTERNETCONTROL`](#iptos-prec-internetcontrol)
  - [`IPTOS_PREC_CRITIC_ECP`](#iptos-prec-critic-ecp)
  - [`IPTOS_PREC_FLASHOVERRIDE`](#iptos-prec-flashoverride)
  - [`IPTOS_PREC_FLASH`](#iptos-prec-flash)
  - [`IPTOS_PREC_IMMEDIATE`](#iptos-prec-immediate)
  - [`IPTOS_PREC_PRIORITY`](#iptos-prec-priority)
  - [`IPTOS_PREC_ROUTINE`](#iptos-prec-routine)
  - [`IPTOS_ECN_MASK`](#iptos-ecn-mask)
  - [`IPTOS_ECN_ECT1`](#iptos-ecn-ect1)
  - [`IPTOS_ECN_ECT0`](#iptos-ecn-ect0)
  - [`IPTOS_ECN_CE`](#iptos-ecn-ce)
  - [`IPOPT_COPY`](#ipopt-copy)
  - [`IPOPT_CLASS_MASK`](#ipopt-class-mask)
  - [`IPOPT_NUMBER_MASK`](#ipopt-number-mask)
  - [`IPOPT_CONTROL`](#ipopt-control)
  - [`IPOPT_RESERVED1`](#ipopt-reserved1)
  - [`IPOPT_MEASUREMENT`](#ipopt-measurement)
  - [`IPOPT_RESERVED2`](#ipopt-reserved2)
  - [`IPOPT_END`](#ipopt-end)
  - [`IPOPT_NOOP`](#ipopt-noop)
  - [`IPOPT_SEC`](#ipopt-sec)
  - [`IPOPT_LSRR`](#ipopt-lsrr)
  - [`IPOPT_TIMESTAMP`](#ipopt-timestamp)
  - [`IPOPT_RR`](#ipopt-rr)
  - [`IPOPT_SID`](#ipopt-sid)
  - [`IPOPT_SSRR`](#ipopt-ssrr)
  - [`IPOPT_RA`](#ipopt-ra)
  - [`IPVERSION`](#ipversion)
  - [`MAXTTL`](#maxttl)
  - [`IPDEFTTL`](#ipdefttl)
  - [`IPOPT_OPTVAL`](#ipopt-optval)
  - [`IPOPT_OLEN`](#ipopt-olen)
  - [`IPOPT_OFFSET`](#ipopt-offset)
  - [`IPOPT_MINOFF`](#ipopt-minoff)
  - [`MAX_IPOPTLEN`](#max-ipoptlen)
  - [`IPOPT_NOP`](#ipopt-nop)
  - [`IPOPT_EOL`](#ipopt-eol)
  - [`IPOPT_TS`](#ipopt-ts)
  - [`IPOPT_TS_TSONLY`](#ipopt-ts-tsonly)
  - [`IPOPT_TS_TSANDADDR`](#ipopt-ts-tsandaddr)
  - [`IPOPT_TS_PRESPEC`](#ipopt-ts-prespec)
  - [`ARPOP_RREQUEST`](#arpop-rrequest)
  - [`ARPOP_RREPLY`](#arpop-rreply)
  - [`ARPOP_InREQUEST`](#arpop-inrequest)
  - [`ARPOP_InREPLY`](#arpop-inreply)
  - [`ARPOP_NAK`](#arpop-nak)
  - [`ATF_NETMASK`](#atf-netmask)
  - [`ATF_DONTPUB`](#atf-dontpub)
  - [`ARPHRD_NETROM`](#arphrd-netrom)
  - [`ARPHRD_ETHER`](#arphrd-ether)
  - [`ARPHRD_EETHER`](#arphrd-eether)
  - [`ARPHRD_AX25`](#arphrd-ax25)
  - [`ARPHRD_PRONET`](#arphrd-pronet)
  - [`ARPHRD_CHAOS`](#arphrd-chaos)
  - [`ARPHRD_IEEE802`](#arphrd-ieee802)
  - [`ARPHRD_ARCNET`](#arphrd-arcnet)
  - [`ARPHRD_APPLETLK`](#arphrd-appletlk)
  - [`ARPHRD_DLCI`](#arphrd-dlci)
  - [`ARPHRD_ATM`](#arphrd-atm)
  - [`ARPHRD_METRICOM`](#arphrd-metricom)
  - [`ARPHRD_IEEE1394`](#arphrd-ieee1394)
  - [`ARPHRD_EUI64`](#arphrd-eui64)
  - [`ARPHRD_INFINIBAND`](#arphrd-infiniband)
  - [`ARPHRD_SLIP`](#arphrd-slip)
  - [`ARPHRD_CSLIP`](#arphrd-cslip)
  - [`ARPHRD_SLIP6`](#arphrd-slip6)
  - [`ARPHRD_CSLIP6`](#arphrd-cslip6)
  - [`ARPHRD_RSRVD`](#arphrd-rsrvd)
  - [`ARPHRD_ADAPT`](#arphrd-adapt)
  - [`ARPHRD_ROSE`](#arphrd-rose)
  - [`ARPHRD_X25`](#arphrd-x25)
  - [`ARPHRD_HWX25`](#arphrd-hwx25)
  - [`ARPHRD_CAN`](#arphrd-can)
  - [`ARPHRD_PPP`](#arphrd-ppp)
  - [`ARPHRD_CISCO`](#arphrd-cisco)
  - [`ARPHRD_HDLC`](#arphrd-hdlc)
  - [`ARPHRD_LAPB`](#arphrd-lapb)
  - [`ARPHRD_DDCMP`](#arphrd-ddcmp)
  - [`ARPHRD_RAWHDLC`](#arphrd-rawhdlc)
  - [`ARPHRD_TUNNEL`](#arphrd-tunnel)
  - [`ARPHRD_TUNNEL6`](#arphrd-tunnel6)
  - [`ARPHRD_FRAD`](#arphrd-frad)
  - [`ARPHRD_SKIP`](#arphrd-skip)
  - [`ARPHRD_LOOPBACK`](#arphrd-loopback)
  - [`ARPHRD_LOCALTLK`](#arphrd-localtlk)
  - [`ARPHRD_FDDI`](#arphrd-fddi)
  - [`ARPHRD_BIF`](#arphrd-bif)
  - [`ARPHRD_SIT`](#arphrd-sit)
  - [`ARPHRD_IPDDP`](#arphrd-ipddp)
  - [`ARPHRD_IPGRE`](#arphrd-ipgre)
  - [`ARPHRD_PIMREG`](#arphrd-pimreg)
  - [`ARPHRD_HIPPI`](#arphrd-hippi)
  - [`ARPHRD_ASH`](#arphrd-ash)
  - [`ARPHRD_ECONET`](#arphrd-econet)
  - [`ARPHRD_IRDA`](#arphrd-irda)
  - [`ARPHRD_FCPP`](#arphrd-fcpp)
  - [`ARPHRD_FCAL`](#arphrd-fcal)
  - [`ARPHRD_FCPL`](#arphrd-fcpl)
  - [`ARPHRD_FCFABRIC`](#arphrd-fcfabric)
  - [`ARPHRD_IEEE802_TR`](#arphrd-ieee802-tr)
  - [`ARPHRD_IEEE80211`](#arphrd-ieee80211)
  - [`ARPHRD_IEEE80211_PRISM`](#arphrd-ieee80211-prism)
  - [`ARPHRD_IEEE80211_RADIOTAP`](#arphrd-ieee80211-radiotap)
  - [`ARPHRD_IEEE802154`](#arphrd-ieee802154)
  - [`ARPHRD_VOID`](#arphrd-void)
  - [`ARPHRD_NONE`](#arphrd-none)
  - [`IFF_TUN`](#iff-tun)
  - [`IFF_TAP`](#iff-tap)
  - [`IFF_NAPI`](#iff-napi)
  - [`IFF_NAPI_FRAGS`](#iff-napi-frags)
  - [`IFF_NO_CARRIER`](#iff-no-carrier)
  - [`IFF_NO_PI`](#iff-no-pi)
  - [`TUN_READQ_SIZE`](#tun-readq-size)
  - [`TUN_TUN_DEV`](#tun-tun-dev)
  - [`TUN_TAP_DEV`](#tun-tap-dev)
  - [`TUN_TYPE_MASK`](#tun-type-mask)
  - [`IFF_ONE_QUEUE`](#iff-one-queue)
  - [`IFF_VNET_HDR`](#iff-vnet-hdr)
  - [`IFF_TUN_EXCL`](#iff-tun-excl)
  - [`IFF_MULTI_QUEUE`](#iff-multi-queue)
  - [`IFF_ATTACH_QUEUE`](#iff-attach-queue)
  - [`IFF_DETACH_QUEUE`](#iff-detach-queue)
  - [`IFF_PERSIST`](#iff-persist)
  - [`IFF_NOFILTER`](#iff-nofilter)
  - [`TUN_TX_TIMESTAMP`](#tun-tx-timestamp)
  - [`TUN_F_CSUM`](#tun-f-csum)
  - [`TUN_F_TSO4`](#tun-f-tso4)
  - [`TUN_F_TSO6`](#tun-f-tso6)
  - [`TUN_F_TSO_ECN`](#tun-f-tso-ecn)
  - [`TUN_F_UFO`](#tun-f-ufo)
  - [`TUN_F_USO4`](#tun-f-uso4)
  - [`TUN_F_USO6`](#tun-f-uso6)
  - [`TUN_PKT_STRIP`](#tun-pkt-strip)
  - [`TUN_FLT_ALLMULTI`](#tun-flt-allmulti)
  - [`T_TYPE`](#t-type)
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
  - [`FS_IOC_GETFLAGS`](#fs-ioc-getflags)
  - [`FS_IOC_SETFLAGS`](#fs-ioc-setflags)
  - [`FS_IOC_GETVERSION`](#fs-ioc-getversion)
  - [`FS_IOC_SETVERSION`](#fs-ioc-setversion)
  - [`FS_IOC32_GETFLAGS`](#fs-ioc32-getflags)
  - [`FS_IOC32_SETFLAGS`](#fs-ioc32-setflags)
  - [`FS_IOC32_GETVERSION`](#fs-ioc32-getversion)
  - [`FS_IOC32_SETVERSION`](#fs-ioc32-setversion)
  - [`FICLONE`](#ficlone)
  - [`FICLONERANGE`](#ficlonerange)
  - [`ADFS_SUPER_MAGIC`](#adfs-super-magic)
  - [`AFFS_SUPER_MAGIC`](#affs-super-magic)
  - [`AFS_SUPER_MAGIC`](#afs-super-magic)
  - [`AUTOFS_SUPER_MAGIC`](#autofs-super-magic)
  - [`BPF_FS_MAGIC`](#bpf-fs-magic)
  - [`BTRFS_SUPER_MAGIC`](#btrfs-super-magic)
  - [`CGROUP2_SUPER_MAGIC`](#cgroup2-super-magic)
  - [`CGROUP_SUPER_MAGIC`](#cgroup-super-magic)
  - [`CODA_SUPER_MAGIC`](#coda-super-magic)
  - [`CRAMFS_MAGIC`](#cramfs-magic)
  - [`DEBUGFS_MAGIC`](#debugfs-magic)
  - [`DEVPTS_SUPER_MAGIC`](#devpts-super-magic)
  - [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs-super-magic)
  - [`EFS_SUPER_MAGIC`](#efs-super-magic)
  - [`EXT2_SUPER_MAGIC`](#ext2-super-magic)
  - [`EXT3_SUPER_MAGIC`](#ext3-super-magic)
  - [`EXT4_SUPER_MAGIC`](#ext4-super-magic)
  - [`F2FS_SUPER_MAGIC`](#f2fs-super-magic)
  - [`FUSE_SUPER_MAGIC`](#fuse-super-magic)
  - [`FUTEXFS_SUPER_MAGIC`](#futexfs-super-magic)
  - [`HOSTFS_SUPER_MAGIC`](#hostfs-super-magic)
  - [`HPFS_SUPER_MAGIC`](#hpfs-super-magic)
  - [`HUGETLBFS_MAGIC`](#hugetlbfs-magic)
  - [`ISOFS_SUPER_MAGIC`](#isofs-super-magic)
  - [`JFFS2_SUPER_MAGIC`](#jffs2-super-magic)
  - [`MINIX2_SUPER_MAGIC2`](#minix2-super-magic2)
  - [`MINIX2_SUPER_MAGIC`](#minix2-super-magic)
  - [`MINIX3_SUPER_MAGIC`](#minix3-super-magic)
  - [`MINIX_SUPER_MAGIC2`](#minix-super-magic2)
  - [`MINIX_SUPER_MAGIC`](#minix-super-magic)
  - [`MSDOS_SUPER_MAGIC`](#msdos-super-magic)
  - [`NCP_SUPER_MAGIC`](#ncp-super-magic)
  - [`NFS_SUPER_MAGIC`](#nfs-super-magic)
  - [`NILFS_SUPER_MAGIC`](#nilfs-super-magic)
  - [`OCFS2_SUPER_MAGIC`](#ocfs2-super-magic)
  - [`OPENPROM_SUPER_MAGIC`](#openprom-super-magic)
  - [`OVERLAYFS_SUPER_MAGIC`](#overlayfs-super-magic)
  - [`PROC_SUPER_MAGIC`](#proc-super-magic)
  - [`QNX4_SUPER_MAGIC`](#qnx4-super-magic)
  - [`QNX6_SUPER_MAGIC`](#qnx6-super-magic)
  - [`RDTGROUP_SUPER_MAGIC`](#rdtgroup-super-magic)
  - [`REISERFS_SUPER_MAGIC`](#reiserfs-super-magic)
  - [`SECURITYFS_MAGIC`](#securityfs-magic)
  - [`SELINUX_MAGIC`](#selinux-magic)
  - [`SMACK_MAGIC`](#smack-magic)
  - [`SMB_SUPER_MAGIC`](#smb-super-magic)
  - [`SYSFS_MAGIC`](#sysfs-magic)
  - [`TMPFS_MAGIC`](#tmpfs-magic)
  - [`TRACEFS_MAGIC`](#tracefs-magic)
  - [`UDF_SUPER_MAGIC`](#udf-super-magic)
  - [`USBDEVICE_SUPER_MAGIC`](#usbdevice-super-magic)
  - [`XENFS_SUPER_MAGIC`](#xenfs-super-magic)
  - [`NSFS_MAGIC`](#nsfs-magic)
  - [`AT_STATX_SYNC_TYPE`](#at-statx-sync-type)
  - [`AT_STATX_SYNC_AS_STAT`](#at-statx-sync-as-stat)
  - [`AT_STATX_FORCE_SYNC`](#at-statx-force-sync)
  - [`AT_STATX_DONT_SYNC`](#at-statx-dont-sync)
  - [`STATX_TYPE`](#statx-type)
  - [`STATX_MODE`](#statx-mode)
  - [`STATX_NLINK`](#statx-nlink)
  - [`STATX_UID`](#statx-uid)
  - [`STATX_GID`](#statx-gid)
  - [`STATX_ATIME`](#statx-atime)
  - [`STATX_MTIME`](#statx-mtime)
  - [`STATX_CTIME`](#statx-ctime)
  - [`STATX_INO`](#statx-ino)
  - [`STATX_SIZE`](#statx-size)
  - [`STATX_BLOCKS`](#statx-blocks)
  - [`STATX_BASIC_STATS`](#statx-basic-stats)
  - [`STATX_BTIME`](#statx-btime)
  - [`STATX_ALL`](#statx-all)
  - [`STATX_MNT_ID`](#statx-mnt-id)
  - [`STATX_DIOALIGN`](#statx-dioalign)
  - [`STATX__RESERVED`](#statx-reserved)
  - [`STATX_ATTR_COMPRESSED`](#statx-attr-compressed)
  - [`STATX_ATTR_IMMUTABLE`](#statx-attr-immutable)
  - [`STATX_ATTR_APPEND`](#statx-attr-append)
  - [`STATX_ATTR_NODUMP`](#statx-attr-nodump)
  - [`STATX_ATTR_ENCRYPTED`](#statx-attr-encrypted)
  - [`STATX_ATTR_AUTOMOUNT`](#statx-attr-automount)
  - [`STATX_ATTR_MOUNT_ROOT`](#statx-attr-mount-root)
  - [`STATX_ATTR_VERITY`](#statx-attr-verity)
  - [`STATX_ATTR_DAX`](#statx-attr-dax)
  - [`_IOC_NRBITS`](#ioc-nrbits)
  - [`_IOC_TYPEBITS`](#ioc-typebits)
  - [`_IOC_SIZEBITS`](#ioc-sizebits)
  - [`_IOC_DIRBITS`](#ioc-dirbits)
  - [`_IOC_NONE`](#ioc-none)
  - [`_IOC_WRITE`](#ioc-write)
  - [`_IOC_READ`](#ioc-read)
  - [`_IOC_NRMASK`](#ioc-nrmask)
  - [`_IOC_TYPEMASK`](#ioc-typemask)
  - [`_IOC_SIZEMASK`](#ioc-sizemask)
  - [`_IOC_DIRMASK`](#ioc-dirmask)
  - [`_IOC_NRSHIFT`](#ioc-nrshift)
  - [`_IOC_TYPESHIFT`](#ioc-typeshift)
  - [`_IOC_SIZESHIFT`](#ioc-sizeshift)
  - [`_IOC_DIRSHIFT`](#ioc-dirshift)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`linux_like`](#linux-like) | mod |  |
| [`linux`](#linux) | mod | Linux-specific definitions for linux-like values |
| [`group`](#group) | struct |  |
| [`utimbuf`](#utimbuf) | struct |  |
| [`timeval`](#timeval) | struct |  |
| [`rlimit`](#rlimit) | struct |  |
| [`rusage`](#rusage) | struct |  |
| [`ipv6_mreq`](#ipv6-mreq) | struct |  |
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
| [`in6_addr`](#in6-addr) | struct |  |
| [`in_addr`](#in-addr) | struct |  |
| [`ip_mreq`](#ip-mreq) | struct |  |
| [`ip_mreqn`](#ip-mreqn) | struct |  |
| [`ip_mreq_source`](#ip-mreq-source) | struct |  |
| [`sockaddr`](#sockaddr) | struct |  |
| [`sockaddr_in`](#sockaddr-in) | struct |  |
| [`sockaddr_in6`](#sockaddr-in6) | struct |  |
| [`addrinfo`](#addrinfo) | struct |  |
| [`sockaddr_ll`](#sockaddr-ll) | struct |  |
| [`fd_set`](#fd-set) | struct |  |
| [`tm`](#tm) | struct |  |
| [`sched_param`](#sched-param) | struct |  |
| [`Dl_info`](#dl-info) | struct |  |
| [`lconv`](#lconv) | struct |  |
| [`in_pktinfo`](#in-pktinfo) | struct |  |
| [`ifaddrs`](#ifaddrs) | struct |  |
| [`in6_rtmsg`](#in6-rtmsg) | struct |  |
| [`arpreq`](#arpreq) | struct |  |
| [`arpreq_old`](#arpreq-old) | struct |  |
| [`arphdr`](#arphdr) | struct |  |
| [`mmsghdr`](#mmsghdr) | struct |  |
| [`sockaddr_un`](#sockaddr-un) | struct |  |
| [`sockaddr_storage`](#sockaddr-storage) | struct |  |
| [`utsname`](#utsname) | struct |  |
| [`file_clone_range`](#file-clone-range) | struct |  |
| [`sock_filter`](#sock-filter) | struct |  |
| [`sock_fprog`](#sock-fprog) | struct |  |
| [`statx`](#statx) | struct |  |
| [`statx_timestamp`](#statx-timestamp) | struct |  |
| [`epoll_event`](#epoll-event) | struct |  |
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
| [`_exit`](#exit) | fn |  |
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
| [`strtok_r`](#strtok-r) | fn |  |
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
| [`getchar_unlocked`](#getchar-unlocked) | fn |  |
| [`putchar_unlocked`](#putchar-unlocked) | fn |  |
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
| [`posix_memalign`](#posix-memalign) | fn |  |
| [`aligned_alloc`](#aligned-alloc) | fn |  |
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
| [`ttyname_r`](#ttyname-r) | fn |  |
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
| [`if_nametoindex`](#if-nametoindex) | fn |  |
| [`if_indextoname`](#if-indextoname) | fn |  |
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
| [`pthread_self`](#pthread-self) | fn |  |
| [`pthread_equal`](#pthread-equal) | fn |  |
| [`pthread_join`](#pthread-join) | fn |  |
| [`pthread_exit`](#pthread-exit) | fn |  |
| [`pthread_attr_init`](#pthread-attr-init) | fn |  |
| [`pthread_attr_destroy`](#pthread-attr-destroy) | fn |  |
| [`pthread_attr_getstacksize`](#pthread-attr-getstacksize) | fn |  |
| [`pthread_attr_setstacksize`](#pthread-attr-setstacksize) | fn |  |
| [`pthread_attr_setdetachstate`](#pthread-attr-setdetachstate) | fn |  |
| [`pthread_detach`](#pthread-detach) | fn |  |
| [`sched_yield`](#sched-yield) | fn |  |
| [`pthread_key_create`](#pthread-key-create) | fn |  |
| [`pthread_key_delete`](#pthread-key-delete) | fn |  |
| [`pthread_getspecific`](#pthread-getspecific) | fn |  |
| [`pthread_setspecific`](#pthread-setspecific) | fn |  |
| [`pthread_mutex_init`](#pthread-mutex-init) | fn |  |
| [`pthread_mutex_destroy`](#pthread-mutex-destroy) | fn |  |
| [`pthread_mutex_lock`](#pthread-mutex-lock) | fn |  |
| [`pthread_mutex_trylock`](#pthread-mutex-trylock) | fn |  |
| [`pthread_mutex_unlock`](#pthread-mutex-unlock) | fn |  |
| [`pthread_mutexattr_init`](#pthread-mutexattr-init) | fn |  |
| [`pthread_mutexattr_destroy`](#pthread-mutexattr-destroy) | fn |  |
| [`pthread_mutexattr_settype`](#pthread-mutexattr-settype) | fn |  |
| [`pthread_cond_init`](#pthread-cond-init) | fn |  |
| [`pthread_cond_wait`](#pthread-cond-wait) | fn |  |
| [`pthread_cond_timedwait`](#pthread-cond-timedwait) | fn |  |
| [`pthread_cond_signal`](#pthread-cond-signal) | fn |  |
| [`pthread_cond_broadcast`](#pthread-cond-broadcast) | fn |  |
| [`pthread_cond_destroy`](#pthread-cond-destroy) | fn |  |
| [`pthread_condattr_init`](#pthread-condattr-init) | fn |  |
| [`pthread_condattr_destroy`](#pthread-condattr-destroy) | fn |  |
| [`pthread_rwlock_init`](#pthread-rwlock-init) | fn |  |
| [`pthread_rwlock_destroy`](#pthread-rwlock-destroy) | fn |  |
| [`pthread_rwlock_rdlock`](#pthread-rwlock-rdlock) | fn |  |
| [`pthread_rwlock_tryrdlock`](#pthread-rwlock-tryrdlock) | fn |  |
| [`pthread_rwlock_wrlock`](#pthread-rwlock-wrlock) | fn |  |
| [`pthread_rwlock_trywrlock`](#pthread-rwlock-trywrlock) | fn |  |
| [`pthread_rwlock_unlock`](#pthread-rwlock-unlock) | fn |  |
| [`pthread_rwlockattr_init`](#pthread-rwlockattr-init) | fn |  |
| [`pthread_rwlockattr_destroy`](#pthread-rwlockattr-destroy) | fn |  |
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
| [`gai_strerror`](#gai-strerror) | fn |  |
| [`res_init`](#res-init) | fn |  |
| [`gmtime_r`](#gmtime-r) | fn |  |
| [`localtime_r`](#localtime-r) | fn |  |
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
| [`sem_wait`](#sem-wait) | fn |  |
| [`sem_trywait`](#sem-trywait) | fn |  |
| [`sem_post`](#sem-post) | fn |  |
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
| [`posix_openpt`](#posix-openpt) | fn |  |
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
| [`open_wmemstream`](#open-wmemstream) | fn |  |
| [`getsid`](#getsid) | fn |  |
| [`pause`](#pause) | fn |  |
| [`mkdirat`](#mkdirat) | fn |  |
| [`openat`](#openat) | fn |  |
| [`fdopendir`](#fdopendir) | fn |  |
| [`readdir_r`](#readdir-r) | fn | The 64-bit libc on Solaris and illumos only has readdir_r. |
| [`readlinkat`](#readlinkat) | fn |  |
| [`fmemopen`](#fmemopen) | fn |  |
| [`open_memstream`](#open-memstream) | fn |  |
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
| [`sem_destroy`](#sem-destroy) | fn |  |
| [`sem_init`](#sem-init) | fn |  |
| [`fdatasync`](#fdatasync) | fn |  |
| [`mincore`](#mincore) | fn |  |
| [`clock_getres`](#clock-getres) | fn |  |
| [`clock_gettime`](#clock-gettime) | fn |  |
| [`clock_settime`](#clock-settime) | fn |  |
| [`clock_getcpuclockid`](#clock-getcpuclockid) | fn |  |
| [`dirfd`](#dirfd) | fn |  |
| [`memalign`](#memalign) | fn |  |
| [`setgroups`](#setgroups) | fn |  |
| [`pipe2`](#pipe2) | fn |  |
| [`statfs`](#statfs) | fn |  |
| [`fstatfs`](#fstatfs) | fn |  |
| [`memrchr`](#memrchr) | fn |  |
| [`posix_fadvise`](#posix-fadvise) | fn |  |
| [`futimens`](#futimens) | fn |  |
| [`utimensat`](#utimensat) | fn |  |
| [`duplocale`](#duplocale) | fn |  |
| [`freelocale`](#freelocale) | fn |  |
| [`newlocale`](#newlocale) | fn |  |
| [`uselocale`](#uselocale) | fn |  |
| [`mknodat`](#mknodat) | fn |  |
| [`ptsname_r`](#ptsname-r) | fn |  |
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
| [`login_tty`](#login-tty) | fn |  |
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
| [`strftime_l`](#strftime-l) | fn |  |
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
| [`posix_fadvise64`](#posix-fadvise64) | fn |  |
| [`pread64`](#pread64) | fn |  |
| [`pwrite64`](#pwrite64) | fn |  |
| [`readdir64`](#readdir64) | fn |  |
| [`readdir64_r`](#readdir64-r) | fn |  |
| [`stat64`](#stat64) | fn |  |
| [`truncate64`](#truncate64) | fn |  |
| [`preadv64`](#preadv64) | fn |  |
| [`pwritev64`](#pwritev64) | fn |  |
| [`forkpty`](#forkpty) | fn |  |
| [`openpty`](#openpty) | fn |  |
| [`statx`](#statx) | fn |  |
| [`_IOC`](#ioc) | fn | Build an ioctl number, analogous to the C macro of the same name. |
| [`_IO`](#io) | fn | Build an ioctl number for an argumentless ioctl. |
| [`_IOR`](#ior) | fn | Build an ioctl number for an read-only ioctl. |
| [`_IOW`](#iow) | fn | Build an ioctl number for an write-only ioctl. |
| [`_IOWR`](#iowr) | fn | Build an ioctl number for a read-write ioctl. |
| [`CMSG_ALIGN`](#cmsg-align) | fn |  |
| [`CMSG_FIRSTHDR`](#cmsg-firsthdr) | fn |  |
| [`CMSG_DATA`](#cmsg-data) | fn |  |
| [`CMSG_SPACE`](#cmsg-space) | fn |  |
| [`CMSG_LEN`](#cmsg-len) | fn |  |
| [`FD_CLR`](#fd-clr) | fn |  |
| [`FD_ISSET`](#fd-isset) | fn |  |
| [`FD_SET`](#fd-set) | fn |  |
| [`FD_ZERO`](#fd-zero) | fn |  |
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
| [`W_EXITCODE`](#w-exitcode) | fn |  |
| [`W_STOPCODE`](#w-stopcode) | fn |  |
| [`QCMD`](#qcmd) | fn |  |
| [`IPOPT_COPIED`](#ipopt-copied) | fn |  |
| [`IPOPT_CLASS`](#ipopt-class) | fn |  |
| [`IPOPT_NUMBER`](#ipopt-number) | fn |  |
| [`IPTOS_ECN`](#iptos-ecn) | fn |  |
| [`KERNEL_VERSION`](#kernel-version) | fn |  |
| [`intmax_t`](#intmax-t) | type |  |
| [`uintmax_t`](#uintmax-t) | type |  |
| [`size_t`](#size-t) | type |  |
| [`ptrdiff_t`](#ptrdiff-t) | type |  |
| [`intptr_t`](#intptr-t) | type |  |
| [`uintptr_t`](#uintptr-t) | type |  |
| [`ssize_t`](#ssize-t) | type |  |
| [`pid_t`](#pid-t) | type |  |
| [`in_addr_t`](#in-addr-t) | type |  |
| [`in_port_t`](#in-port-t) | type |  |
| [`sighandler_t`](#sighandler-t) | type |  |
| [`cc_t`](#cc-t) | type |  |
| [`uid_t`](#uid-t) | type |  |
| [`gid_t`](#gid-t) | type |  |
| [`locale_t`](#locale-t) | type |  |
| [`sa_family_t`](#sa-family-t) | type |  |
| [`speed_t`](#speed-t) | type |  |
| [`tcflag_t`](#tcflag-t) | type |  |
| [`clockid_t`](#clockid-t) | type |  |
| [`timer_t`](#timer-t) | type |  |
| [`key_t`](#key-t) | type |  |
| [`id_t`](#id-t) | type |  |
| [`INT_MIN`](#int-min) | const |  |
| [`INT_MAX`](#int-max) | const |  |
| [`SIG_DFL`](#sig-dfl) | const |  |
| [`SIG_IGN`](#sig-ign) | const |  |
| [`SIG_ERR`](#sig-err) | const |  |
| [`DT_UNKNOWN`](#dt-unknown) | const |  |
| [`DT_FIFO`](#dt-fifo) | const |  |
| [`DT_CHR`](#dt-chr) | const |  |
| [`DT_DIR`](#dt-dir) | const |  |
| [`DT_BLK`](#dt-blk) | const |  |
| [`DT_REG`](#dt-reg) | const |  |
| [`DT_LNK`](#dt-lnk) | const |  |
| [`DT_SOCK`](#dt-sock) | const |  |
| [`FD_CLOEXEC`](#fd-cloexec) | const |  |
| [`USRQUOTA`](#usrquota) | const |  |
| [`GRPQUOTA`](#grpquota) | const |  |
| [`SIGIOT`](#sigiot) | const |  |
| [`S_ISUID`](#s-isuid) | const |  |
| [`S_ISGID`](#s-isgid) | const |  |
| [`S_ISVTX`](#s-isvtx) | const |  |
| [`IF_NAMESIZE`](#if-namesize) | const |  |
| [`IFNAMSIZ`](#ifnamsiz) | const |  |
| [`LOG_EMERG`](#log-emerg) | const |  |
| [`LOG_ALERT`](#log-alert) | const |  |
| [`LOG_CRIT`](#log-crit) | const |  |
| [`LOG_ERR`](#log-err) | const |  |
| [`LOG_WARNING`](#log-warning) | const |  |
| [`LOG_NOTICE`](#log-notice) | const |  |
| [`LOG_INFO`](#log-info) | const |  |
| [`LOG_DEBUG`](#log-debug) | const |  |
| [`LOG_KERN`](#log-kern) | const |  |
| [`LOG_USER`](#log-user) | const |  |
| [`LOG_MAIL`](#log-mail) | const |  |
| [`LOG_DAEMON`](#log-daemon) | const |  |
| [`LOG_AUTH`](#log-auth) | const |  |
| [`LOG_SYSLOG`](#log-syslog) | const |  |
| [`LOG_LPR`](#log-lpr) | const |  |
| [`LOG_NEWS`](#log-news) | const |  |
| [`LOG_UUCP`](#log-uucp) | const |  |
| [`LOG_LOCAL0`](#log-local0) | const |  |
| [`LOG_LOCAL1`](#log-local1) | const |  |
| [`LOG_LOCAL2`](#log-local2) | const |  |
| [`LOG_LOCAL3`](#log-local3) | const |  |
| [`LOG_LOCAL4`](#log-local4) | const |  |
| [`LOG_LOCAL5`](#log-local5) | const |  |
| [`LOG_LOCAL6`](#log-local6) | const |  |
| [`LOG_LOCAL7`](#log-local7) | const |  |
| [`LOG_PID`](#log-pid) | const |  |
| [`LOG_CONS`](#log-cons) | const |  |
| [`LOG_ODELAY`](#log-odelay) | const |  |
| [`LOG_NDELAY`](#log-ndelay) | const |  |
| [`LOG_NOWAIT`](#log-nowait) | const |  |
| [`LOG_PRIMASK`](#log-primask) | const |  |
| [`LOG_FACMASK`](#log-facmask) | const |  |
| [`PRIO_MIN`](#prio-min) | const |  |
| [`PRIO_MAX`](#prio-max) | const |  |
| [`IPPROTO_ICMP`](#ipproto-icmp) | const |  |
| [`IPPROTO_ICMPV6`](#ipproto-icmpv6) | const |  |
| [`IPPROTO_TCP`](#ipproto-tcp) | const |  |
| [`IPPROTO_UDP`](#ipproto-udp) | const |  |
| [`IPPROTO_IP`](#ipproto-ip) | const |  |
| [`IPPROTO_IPV6`](#ipproto-ipv6) | const |  |
| [`INADDR_LOOPBACK`](#inaddr-loopback) | const |  |
| [`INADDR_ANY`](#inaddr-any) | const |  |
| [`INADDR_BROADCAST`](#inaddr-broadcast) | const |  |
| [`INADDR_NONE`](#inaddr-none) | const |  |
| [`IN6ADDR_LOOPBACK_INIT`](#in6addr-loopback-init) | const |  |
| [`IN6ADDR_ANY_INIT`](#in6addr-any-init) | const |  |
| [`ARPOP_REQUEST`](#arpop-request) | const |  |
| [`ARPOP_REPLY`](#arpop-reply) | const |  |
| [`ATF_COM`](#atf-com) | const |  |
| [`ATF_PERM`](#atf-perm) | const |  |
| [`ATF_PUBL`](#atf-publ) | const |  |
| [`ATF_USETRAILERS`](#atf-usetrailers) | const |  |
| [`FNM_PERIOD`](#fnm-period) | const |  |
| [`FNM_NOMATCH`](#fnm-nomatch) | const |  |
| [`FNM_CASEFOLD`](#fnm-casefold) | const |  |
| [`FNM_PATHNAME`](#fnm-pathname) | const |  |
| [`FNM_NOESCAPE`](#fnm-noescape) | const |  |
| [`ULONG_SIZE`](#ulong-size) | const |  |
| [`EXIT_FAILURE`](#exit-failure) | const |  |
| [`EXIT_SUCCESS`](#exit-success) | const |  |
| [`RAND_MAX`](#rand-max) | const |  |
| [`EOF`](#eof) | const |  |
| [`SEEK_SET`](#seek-set) | const |  |
| [`SEEK_CUR`](#seek-cur) | const |  |
| [`SEEK_END`](#seek-end) | const |  |
| [`_IOFBF`](#iofbf) | const |  |
| [`_IONBF`](#ionbf) | const |  |
| [`_IOLBF`](#iolbf) | const |  |
| [`F_DUPFD`](#f-dupfd) | const |  |
| [`F_GETFD`](#f-getfd) | const |  |
| [`F_SETFD`](#f-setfd) | const |  |
| [`F_GETFL`](#f-getfl) | const |  |
| [`F_SETFL`](#f-setfl) | const |  |
| [`F_SETLEASE`](#f-setlease) | const |  |
| [`F_GETLEASE`](#f-getlease) | const |  |
| [`F_NOTIFY`](#f-notify) | const |  |
| [`F_CANCELLK`](#f-cancellk) | const |  |
| [`F_DUPFD_CLOEXEC`](#f-dupfd-cloexec) | const |  |
| [`F_SETPIPE_SZ`](#f-setpipe-sz) | const |  |
| [`F_GETPIPE_SZ`](#f-getpipe-sz) | const |  |
| [`F_ADD_SEALS`](#f-add-seals) | const |  |
| [`F_GET_SEALS`](#f-get-seals) | const |  |
| [`F_SEAL_SEAL`](#f-seal-seal) | const |  |
| [`F_SEAL_SHRINK`](#f-seal-shrink) | const |  |
| [`F_SEAL_GROW`](#f-seal-grow) | const |  |
| [`F_SEAL_WRITE`](#f-seal-write) | const |  |
| [`SIGTRAP`](#sigtrap) | const |  |
| [`PTHREAD_CREATE_JOINABLE`](#pthread-create-joinable) | const |  |
| [`PTHREAD_CREATE_DETACHED`](#pthread-create-detached) | const |  |
| [`CLOCK_REALTIME`](#clock-realtime) | const |  |
| [`CLOCK_MONOTONIC`](#clock-monotonic) | const |  |
| [`CLOCK_PROCESS_CPUTIME_ID`](#clock-process-cputime-id) | const |  |
| [`CLOCK_THREAD_CPUTIME_ID`](#clock-thread-cputime-id) | const |  |
| [`CLOCK_MONOTONIC_RAW`](#clock-monotonic-raw) | const |  |
| [`CLOCK_REALTIME_COARSE`](#clock-realtime-coarse) | const |  |
| [`CLOCK_MONOTONIC_COARSE`](#clock-monotonic-coarse) | const |  |
| [`CLOCK_BOOTTIME`](#clock-boottime) | const |  |
| [`CLOCK_REALTIME_ALARM`](#clock-realtime-alarm) | const |  |
| [`CLOCK_BOOTTIME_ALARM`](#clock-boottime-alarm) | const |  |
| [`CLOCK_TAI`](#clock-tai) | const |  |
| [`TIMER_ABSTIME`](#timer-abstime) | const |  |
| [`RUSAGE_SELF`](#rusage-self) | const |  |
| [`O_RDONLY`](#o-rdonly) | const |  |
| [`O_WRONLY`](#o-wronly) | const |  |
| [`O_RDWR`](#o-rdwr) | const |  |
| [`SOCK_CLOEXEC`](#sock-cloexec) | const |  |
| [`S_IFIFO`](#s-ififo) | const |  |
| [`S_IFCHR`](#s-ifchr) | const |  |
| [`S_IFBLK`](#s-ifblk) | const |  |
| [`S_IFDIR`](#s-ifdir) | const |  |
| [`S_IFREG`](#s-ifreg) | const |  |
| [`S_IFLNK`](#s-iflnk) | const |  |
| [`S_IFSOCK`](#s-ifsock) | const |  |
| [`S_IFMT`](#s-ifmt) | const |  |
| [`S_IRWXU`](#s-irwxu) | const |  |
| [`S_IXUSR`](#s-ixusr) | const |  |
| [`S_IWUSR`](#s-iwusr) | const |  |
| [`S_IRUSR`](#s-irusr) | const |  |
| [`S_IRWXG`](#s-irwxg) | const |  |
| [`S_IXGRP`](#s-ixgrp) | const |  |
| [`S_IWGRP`](#s-iwgrp) | const |  |
| [`S_IRGRP`](#s-irgrp) | const |  |
| [`S_IRWXO`](#s-irwxo) | const |  |
| [`S_IXOTH`](#s-ixoth) | const |  |
| [`S_IWOTH`](#s-iwoth) | const |  |
| [`S_IROTH`](#s-iroth) | const |  |
| [`F_OK`](#f-ok) | const |  |
| [`R_OK`](#r-ok) | const |  |
| [`W_OK`](#w-ok) | const |  |
| [`X_OK`](#x-ok) | const |  |
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
| [`PROT_NONE`](#prot-none) | const |  |
| [`PROT_READ`](#prot-read) | const |  |
| [`PROT_WRITE`](#prot-write) | const |  |
| [`PROT_EXEC`](#prot-exec) | const |  |
| [`XATTR_CREATE`](#xattr-create) | const |  |
| [`XATTR_REPLACE`](#xattr-replace) | const |  |
| [`RLIM64_INFINITY`](#rlim64-infinity) | const |  |
| [`LC_CTYPE`](#lc-ctype) | const |  |
| [`LC_NUMERIC`](#lc-numeric) | const |  |
| [`LC_TIME`](#lc-time) | const |  |
| [`LC_COLLATE`](#lc-collate) | const |  |
| [`LC_MONETARY`](#lc-monetary) | const |  |
| [`LC_MESSAGES`](#lc-messages) | const |  |
| [`LC_ALL`](#lc-all) | const |  |
| [`LC_CTYPE_MASK`](#lc-ctype-mask) | const |  |
| [`LC_NUMERIC_MASK`](#lc-numeric-mask) | const |  |
| [`LC_TIME_MASK`](#lc-time-mask) | const |  |
| [`LC_COLLATE_MASK`](#lc-collate-mask) | const |  |
| [`LC_MONETARY_MASK`](#lc-monetary-mask) | const |  |
| [`LC_MESSAGES_MASK`](#lc-messages-mask) | const |  |
| [`MAP_FILE`](#map-file) | const |  |
| [`MAP_SHARED`](#map-shared) | const |  |
| [`MAP_PRIVATE`](#map-private) | const |  |
| [`MAP_FIXED`](#map-fixed) | const |  |
| [`MAP_FAILED`](#map-failed) | const |  |
| [`MS_ASYNC`](#ms-async) | const |  |
| [`MS_INVALIDATE`](#ms-invalidate) | const |  |
| [`MS_SYNC`](#ms-sync) | const |  |
| [`MS_RDONLY`](#ms-rdonly) | const |  |
| [`MS_NOSUID`](#ms-nosuid) | const |  |
| [`MS_NODEV`](#ms-nodev) | const |  |
| [`MS_NOEXEC`](#ms-noexec) | const |  |
| [`MS_SYNCHRONOUS`](#ms-synchronous) | const |  |
| [`MS_REMOUNT`](#ms-remount) | const |  |
| [`MS_MANDLOCK`](#ms-mandlock) | const |  |
| [`MS_DIRSYNC`](#ms-dirsync) | const |  |
| [`MS_NOSYMFOLLOW`](#ms-nosymfollow) | const |  |
| [`MS_NOATIME`](#ms-noatime) | const |  |
| [`MS_NODIRATIME`](#ms-nodiratime) | const |  |
| [`MS_BIND`](#ms-bind) | const |  |
| [`MS_MOVE`](#ms-move) | const |  |
| [`MS_REC`](#ms-rec) | const |  |
| [`MS_SILENT`](#ms-silent) | const |  |
| [`MS_POSIXACL`](#ms-posixacl) | const |  |
| [`MS_UNBINDABLE`](#ms-unbindable) | const |  |
| [`MS_PRIVATE`](#ms-private) | const |  |
| [`MS_SLAVE`](#ms-slave) | const |  |
| [`MS_SHARED`](#ms-shared) | const |  |
| [`MS_RELATIME`](#ms-relatime) | const |  |
| [`MS_KERNMOUNT`](#ms-kernmount) | const |  |
| [`MS_I_VERSION`](#ms-i-version) | const |  |
| [`MS_STRICTATIME`](#ms-strictatime) | const |  |
| [`MS_LAZYTIME`](#ms-lazytime) | const |  |
| [`MS_ACTIVE`](#ms-active) | const |  |
| [`MS_MGC_VAL`](#ms-mgc-val) | const |  |
| [`MS_MGC_MSK`](#ms-mgc-msk) | const |  |
| [`SCM_RIGHTS`](#scm-rights) | const |  |
| [`SCM_CREDENTIALS`](#scm-credentials) | const |  |
| [`PROT_GROWSDOWN`](#prot-growsdown) | const |  |
| [`PROT_GROWSUP`](#prot-growsup) | const |  |
| [`MAP_TYPE`](#map-type) | const |  |
| [`MADV_NORMAL`](#madv-normal) | const |  |
| [`MADV_RANDOM`](#madv-random) | const |  |
| [`MADV_SEQUENTIAL`](#madv-sequential) | const |  |
| [`MADV_WILLNEED`](#madv-willneed) | const |  |
| [`MADV_DONTNEED`](#madv-dontneed) | const |  |
| [`MADV_FREE`](#madv-free) | const |  |
| [`MADV_REMOVE`](#madv-remove) | const |  |
| [`MADV_DONTFORK`](#madv-dontfork) | const |  |
| [`MADV_DOFORK`](#madv-dofork) | const |  |
| [`MADV_MERGEABLE`](#madv-mergeable) | const |  |
| [`MADV_UNMERGEABLE`](#madv-unmergeable) | const |  |
| [`MADV_HUGEPAGE`](#madv-hugepage) | const |  |
| [`MADV_NOHUGEPAGE`](#madv-nohugepage) | const |  |
| [`MADV_DONTDUMP`](#madv-dontdump) | const |  |
| [`MADV_DODUMP`](#madv-dodump) | const |  |
| [`MADV_WIPEONFORK`](#madv-wipeonfork) | const |  |
| [`MADV_KEEPONFORK`](#madv-keeponfork) | const |  |
| [`MADV_COLD`](#madv-cold) | const |  |
| [`MADV_PAGEOUT`](#madv-pageout) | const |  |
| [`MADV_HWPOISON`](#madv-hwpoison) | const |  |
| [`MADV_POPULATE_READ`](#madv-populate-read) | const |  |
| [`MADV_POPULATE_WRITE`](#madv-populate-write) | const |  |
| [`MADV_DONTNEED_LOCKED`](#madv-dontneed-locked) | const |  |
| [`IFF_UP`](#iff-up) | const |  |
| [`IFF_BROADCAST`](#iff-broadcast) | const |  |
| [`IFF_DEBUG`](#iff-debug) | const |  |
| [`IFF_LOOPBACK`](#iff-loopback) | const |  |
| [`IFF_POINTOPOINT`](#iff-pointopoint) | const |  |
| [`IFF_NOTRAILERS`](#iff-notrailers) | const |  |
| [`IFF_RUNNING`](#iff-running) | const |  |
| [`IFF_NOARP`](#iff-noarp) | const |  |
| [`IFF_PROMISC`](#iff-promisc) | const |  |
| [`IFF_ALLMULTI`](#iff-allmulti) | const |  |
| [`IFF_MASTER`](#iff-master) | const |  |
| [`IFF_SLAVE`](#iff-slave) | const |  |
| [`IFF_MULTICAST`](#iff-multicast) | const |  |
| [`IFF_PORTSEL`](#iff-portsel) | const |  |
| [`IFF_AUTOMEDIA`](#iff-automedia) | const |  |
| [`IFF_DYNAMIC`](#iff-dynamic) | const |  |
| [`SOL_IP`](#sol-ip) | const |  |
| [`SOL_TCP`](#sol-tcp) | const |  |
| [`SOL_UDP`](#sol-udp) | const |  |
| [`SOL_IPV6`](#sol-ipv6) | const |  |
| [`SOL_ICMPV6`](#sol-icmpv6) | const |  |
| [`SOL_RAW`](#sol-raw) | const |  |
| [`SOL_DECNET`](#sol-decnet) | const |  |
| [`SOL_X25`](#sol-x25) | const |  |
| [`SOL_PACKET`](#sol-packet) | const |  |
| [`SOL_ATM`](#sol-atm) | const |  |
| [`SOL_AAL`](#sol-aal) | const |  |
| [`SOL_IRDA`](#sol-irda) | const |  |
| [`SOL_NETBEUI`](#sol-netbeui) | const |  |
| [`SOL_LLC`](#sol-llc) | const |  |
| [`SOL_DCCP`](#sol-dccp) | const |  |
| [`SOL_NETLINK`](#sol-netlink) | const |  |
| [`SOL_TIPC`](#sol-tipc) | const |  |
| [`SOL_BLUETOOTH`](#sol-bluetooth) | const |  |
| [`SOL_ALG`](#sol-alg) | const |  |
| [`AF_UNSPEC`](#af-unspec) | const |  |
| [`AF_UNIX`](#af-unix) | const |  |
| [`AF_LOCAL`](#af-local) | const |  |
| [`AF_INET`](#af-inet) | const |  |
| [`AF_AX25`](#af-ax25) | const |  |
| [`AF_IPX`](#af-ipx) | const |  |
| [`AF_APPLETALK`](#af-appletalk) | const |  |
| [`AF_NETROM`](#af-netrom) | const |  |
| [`AF_BRIDGE`](#af-bridge) | const |  |
| [`AF_ATMPVC`](#af-atmpvc) | const |  |
| [`AF_X25`](#af-x25) | const |  |
| [`AF_INET6`](#af-inet6) | const |  |
| [`AF_ROSE`](#af-rose) | const |  |
| [`AF_DECnet`](#af-decnet) | const |  |
| [`AF_NETBEUI`](#af-netbeui) | const |  |
| [`AF_SECURITY`](#af-security) | const |  |
| [`AF_KEY`](#af-key) | const |  |
| [`AF_NETLINK`](#af-netlink) | const |  |
| [`AF_ROUTE`](#af-route) | const |  |
| [`AF_PACKET`](#af-packet) | const |  |
| [`AF_ASH`](#af-ash) | const |  |
| [`AF_ECONET`](#af-econet) | const |  |
| [`AF_ATMSVC`](#af-atmsvc) | const |  |
| [`AF_RDS`](#af-rds) | const |  |
| [`AF_SNA`](#af-sna) | const |  |
| [`AF_IRDA`](#af-irda) | const |  |
| [`AF_PPPOX`](#af-pppox) | const |  |
| [`AF_WANPIPE`](#af-wanpipe) | const |  |
| [`AF_LLC`](#af-llc) | const |  |
| [`AF_CAN`](#af-can) | const |  |
| [`AF_TIPC`](#af-tipc) | const |  |
| [`AF_BLUETOOTH`](#af-bluetooth) | const |  |
| [`AF_IUCV`](#af-iucv) | const |  |
| [`AF_RXRPC`](#af-rxrpc) | const |  |
| [`AF_ISDN`](#af-isdn) | const |  |
| [`AF_PHONET`](#af-phonet) | const |  |
| [`AF_IEEE802154`](#af-ieee802154) | const |  |
| [`AF_CAIF`](#af-caif) | const |  |
| [`AF_ALG`](#af-alg) | const |  |
| [`PF_UNSPEC`](#pf-unspec) | const |  |
| [`PF_UNIX`](#pf-unix) | const |  |
| [`PF_LOCAL`](#pf-local) | const |  |
| [`PF_INET`](#pf-inet) | const |  |
| [`PF_AX25`](#pf-ax25) | const |  |
| [`PF_IPX`](#pf-ipx) | const |  |
| [`PF_APPLETALK`](#pf-appletalk) | const |  |
| [`PF_NETROM`](#pf-netrom) | const |  |
| [`PF_BRIDGE`](#pf-bridge) | const |  |
| [`PF_ATMPVC`](#pf-atmpvc) | const |  |
| [`PF_X25`](#pf-x25) | const |  |
| [`PF_INET6`](#pf-inet6) | const |  |
| [`PF_ROSE`](#pf-rose) | const |  |
| [`PF_DECnet`](#pf-decnet) | const |  |
| [`PF_NETBEUI`](#pf-netbeui) | const |  |
| [`PF_SECURITY`](#pf-security) | const |  |
| [`PF_KEY`](#pf-key) | const |  |
| [`PF_NETLINK`](#pf-netlink) | const |  |
| [`PF_ROUTE`](#pf-route) | const |  |
| [`PF_PACKET`](#pf-packet) | const |  |
| [`PF_ASH`](#pf-ash) | const |  |
| [`PF_ECONET`](#pf-econet) | const |  |
| [`PF_ATMSVC`](#pf-atmsvc) | const |  |
| [`PF_RDS`](#pf-rds) | const |  |
| [`PF_SNA`](#pf-sna) | const |  |
| [`PF_IRDA`](#pf-irda) | const |  |
| [`PF_PPPOX`](#pf-pppox) | const |  |
| [`PF_WANPIPE`](#pf-wanpipe) | const |  |
| [`PF_LLC`](#pf-llc) | const |  |
| [`PF_CAN`](#pf-can) | const |  |
| [`PF_TIPC`](#pf-tipc) | const |  |
| [`PF_BLUETOOTH`](#pf-bluetooth) | const |  |
| [`PF_IUCV`](#pf-iucv) | const |  |
| [`PF_RXRPC`](#pf-rxrpc) | const |  |
| [`PF_ISDN`](#pf-isdn) | const |  |
| [`PF_PHONET`](#pf-phonet) | const |  |
| [`PF_IEEE802154`](#pf-ieee802154) | const |  |
| [`PF_CAIF`](#pf-caif) | const |  |
| [`PF_ALG`](#pf-alg) | const |  |
| [`MSG_OOB`](#msg-oob) | const |  |
| [`MSG_PEEK`](#msg-peek) | const |  |
| [`MSG_DONTROUTE`](#msg-dontroute) | const |  |
| [`MSG_CTRUNC`](#msg-ctrunc) | const |  |
| [`MSG_TRUNC`](#msg-trunc) | const |  |
| [`MSG_DONTWAIT`](#msg-dontwait) | const |  |
| [`MSG_EOR`](#msg-eor) | const |  |
| [`MSG_WAITALL`](#msg-waitall) | const |  |
| [`MSG_FIN`](#msg-fin) | const |  |
| [`MSG_SYN`](#msg-syn) | const |  |
| [`MSG_CONFIRM`](#msg-confirm) | const |  |
| [`MSG_RST`](#msg-rst) | const |  |
| [`MSG_ERRQUEUE`](#msg-errqueue) | const |  |
| [`MSG_NOSIGNAL`](#msg-nosignal) | const |  |
| [`MSG_MORE`](#msg-more) | const |  |
| [`MSG_WAITFORONE`](#msg-waitforone) | const |  |
| [`MSG_FASTOPEN`](#msg-fastopen) | const |  |
| [`MSG_CMSG_CLOEXEC`](#msg-cmsg-cloexec) | const |  |
| [`SCM_TIMESTAMP`](#scm-timestamp) | const |  |
| [`SOCK_RAW`](#sock-raw) | const |  |
| [`SOCK_RDM`](#sock-rdm) | const |  |
| [`IP_TOS`](#ip-tos) | const |  |
| [`IP_TTL`](#ip-ttl) | const |  |
| [`IP_HDRINCL`](#ip-hdrincl) | const |  |
| [`IP_OPTIONS`](#ip-options) | const |  |
| [`IP_ROUTER_ALERT`](#ip-router-alert) | const |  |
| [`IP_RECVOPTS`](#ip-recvopts) | const |  |
| [`IP_RETOPTS`](#ip-retopts) | const |  |
| [`IP_PKTINFO`](#ip-pktinfo) | const |  |
| [`IP_PKTOPTIONS`](#ip-pktoptions) | const |  |
| [`IP_MTU_DISCOVER`](#ip-mtu-discover) | const |  |
| [`IP_RECVERR`](#ip-recverr) | const |  |
| [`IP_RECVTTL`](#ip-recvttl) | const |  |
| [`IP_RECVTOS`](#ip-recvtos) | const |  |
| [`IP_MTU`](#ip-mtu) | const |  |
| [`IP_FREEBIND`](#ip-freebind) | const |  |
| [`IP_IPSEC_POLICY`](#ip-ipsec-policy) | const |  |
| [`IP_XFRM_POLICY`](#ip-xfrm-policy) | const |  |
| [`IP_PASSSEC`](#ip-passsec) | const |  |
| [`IP_TRANSPARENT`](#ip-transparent) | const |  |
| [`IP_ORIGDSTADDR`](#ip-origdstaddr) | const |  |
| [`IP_RECVORIGDSTADDR`](#ip-recvorigdstaddr) | const |  |
| [`IP_MINTTL`](#ip-minttl) | const |  |
| [`IP_NODEFRAG`](#ip-nodefrag) | const |  |
| [`IP_CHECKSUM`](#ip-checksum) | const |  |
| [`IP_BIND_ADDRESS_NO_PORT`](#ip-bind-address-no-port) | const |  |
| [`IP_MULTICAST_IF`](#ip-multicast-if) | const |  |
| [`IP_MULTICAST_TTL`](#ip-multicast-ttl) | const |  |
| [`IP_MULTICAST_LOOP`](#ip-multicast-loop) | const |  |
| [`IP_ADD_MEMBERSHIP`](#ip-add-membership) | const |  |
| [`IP_DROP_MEMBERSHIP`](#ip-drop-membership) | const |  |
| [`IP_UNBLOCK_SOURCE`](#ip-unblock-source) | const |  |
| [`IP_BLOCK_SOURCE`](#ip-block-source) | const |  |
| [`IP_ADD_SOURCE_MEMBERSHIP`](#ip-add-source-membership) | const |  |
| [`IP_DROP_SOURCE_MEMBERSHIP`](#ip-drop-source-membership) | const |  |
| [`IP_MSFILTER`](#ip-msfilter) | const |  |
| [`IP_MULTICAST_ALL`](#ip-multicast-all) | const |  |
| [`IP_UNICAST_IF`](#ip-unicast-if) | const |  |
| [`IP_DEFAULT_MULTICAST_TTL`](#ip-default-multicast-ttl) | const |  |
| [`IP_DEFAULT_MULTICAST_LOOP`](#ip-default-multicast-loop) | const |  |
| [`IP_PMTUDISC_DONT`](#ip-pmtudisc-dont) | const |  |
| [`IP_PMTUDISC_WANT`](#ip-pmtudisc-want) | const |  |
| [`IP_PMTUDISC_DO`](#ip-pmtudisc-do) | const |  |
| [`IP_PMTUDISC_PROBE`](#ip-pmtudisc-probe) | const |  |
| [`IP_PMTUDISC_INTERFACE`](#ip-pmtudisc-interface) | const |  |
| [`IP_PMTUDISC_OMIT`](#ip-pmtudisc-omit) | const |  |
| [`IPPROTO_HOPOPTS`](#ipproto-hopopts) | const | Hop-by-hop option header |
| [`IPPROTO_IGMP`](#ipproto-igmp) | const | group mgmt protocol |
| [`IPPROTO_IPIP`](#ipproto-ipip) | const | for compatibility |
| [`IPPROTO_EGP`](#ipproto-egp) | const | exterior gateway protocol |
| [`IPPROTO_PUP`](#ipproto-pup) | const | pup |
| [`IPPROTO_IDP`](#ipproto-idp) | const | xns idp |
| [`IPPROTO_TP`](#ipproto-tp) | const | tp-4 w/ class negotiation |
| [`IPPROTO_DCCP`](#ipproto-dccp) | const | DCCP |
| [`IPPROTO_ROUTING`](#ipproto-routing) | const | IP6 routing header |
| [`IPPROTO_FRAGMENT`](#ipproto-fragment) | const | IP6 fragmentation header |
| [`IPPROTO_RSVP`](#ipproto-rsvp) | const | resource reservation |
| [`IPPROTO_GRE`](#ipproto-gre) | const | General Routing Encap. |
| [`IPPROTO_ESP`](#ipproto-esp) | const | IP6 Encap Sec. |
| [`IPPROTO_AH`](#ipproto-ah) | const | IP6 Auth Header |
| [`IPPROTO_NONE`](#ipproto-none) | const | IP6 no next header |
| [`IPPROTO_DSTOPTS`](#ipproto-dstopts) | const | IP6 destination option |
| [`IPPROTO_MTP`](#ipproto-mtp) | const |  |
| [`IPPROTO_ENCAP`](#ipproto-encap) | const | encapsulation header |
| [`IPPROTO_PIM`](#ipproto-pim) | const | Protocol indep. |
| [`IPPROTO_COMP`](#ipproto-comp) | const | IP Payload Comp. |
| [`IPPROTO_SCTP`](#ipproto-sctp) | const | SCTP |
| [`IPPROTO_MH`](#ipproto-mh) | const |  |
| [`IPPROTO_UDPLITE`](#ipproto-udplite) | const |  |
| [`IPPROTO_RAW`](#ipproto-raw) | const | raw IP packet |
| [`IPPROTO_BEETPH`](#ipproto-beetph) | const |  |
| [`IPPROTO_MPLS`](#ipproto-mpls) | const |  |
| [`IPPROTO_MPTCP`](#ipproto-mptcp) | const | Multipath TCP |
| [`IPPROTO_ETHERNET`](#ipproto-ethernet) | const | Ethernet-within-IPv6 encapsulation. |
| [`MCAST_EXCLUDE`](#mcast-exclude) | const |  |
| [`MCAST_INCLUDE`](#mcast-include) | const |  |
| [`MCAST_JOIN_GROUP`](#mcast-join-group) | const |  |
| [`MCAST_BLOCK_SOURCE`](#mcast-block-source) | const |  |
| [`MCAST_UNBLOCK_SOURCE`](#mcast-unblock-source) | const |  |
| [`MCAST_LEAVE_GROUP`](#mcast-leave-group) | const |  |
| [`MCAST_JOIN_SOURCE_GROUP`](#mcast-join-source-group) | const |  |
| [`MCAST_LEAVE_SOURCE_GROUP`](#mcast-leave-source-group) | const |  |
| [`MCAST_MSFILTER`](#mcast-msfilter) | const |  |
| [`IPV6_ADDRFORM`](#ipv6-addrform) | const |  |
| [`IPV6_2292PKTINFO`](#ipv6-2292pktinfo) | const |  |
| [`IPV6_2292HOPOPTS`](#ipv6-2292hopopts) | const |  |
| [`IPV6_2292DSTOPTS`](#ipv6-2292dstopts) | const |  |
| [`IPV6_2292RTHDR`](#ipv6-2292rthdr) | const |  |
| [`IPV6_2292PKTOPTIONS`](#ipv6-2292pktoptions) | const |  |
| [`IPV6_CHECKSUM`](#ipv6-checksum) | const |  |
| [`IPV6_2292HOPLIMIT`](#ipv6-2292hoplimit) | const |  |
| [`IPV6_NEXTHOP`](#ipv6-nexthop) | const |  |
| [`IPV6_AUTHHDR`](#ipv6-authhdr) | const |  |
| [`IPV6_UNICAST_HOPS`](#ipv6-unicast-hops) | const |  |
| [`IPV6_MULTICAST_IF`](#ipv6-multicast-if) | const |  |
| [`IPV6_MULTICAST_HOPS`](#ipv6-multicast-hops) | const |  |
| [`IPV6_MULTICAST_LOOP`](#ipv6-multicast-loop) | const |  |
| [`IPV6_ADD_MEMBERSHIP`](#ipv6-add-membership) | const |  |
| [`IPV6_DROP_MEMBERSHIP`](#ipv6-drop-membership) | const |  |
| [`IPV6_ROUTER_ALERT`](#ipv6-router-alert) | const |  |
| [`IPV6_MTU_DISCOVER`](#ipv6-mtu-discover) | const |  |
| [`IPV6_MTU`](#ipv6-mtu) | const |  |
| [`IPV6_RECVERR`](#ipv6-recverr) | const |  |
| [`IPV6_V6ONLY`](#ipv6-v6only) | const |  |
| [`IPV6_JOIN_ANYCAST`](#ipv6-join-anycast) | const |  |
| [`IPV6_LEAVE_ANYCAST`](#ipv6-leave-anycast) | const |  |
| [`IPV6_IPSEC_POLICY`](#ipv6-ipsec-policy) | const |  |
| [`IPV6_XFRM_POLICY`](#ipv6-xfrm-policy) | const |  |
| [`IPV6_HDRINCL`](#ipv6-hdrincl) | const |  |
| [`IPV6_RECVPKTINFO`](#ipv6-recvpktinfo) | const |  |
| [`IPV6_PKTINFO`](#ipv6-pktinfo) | const |  |
| [`IPV6_RECVHOPLIMIT`](#ipv6-recvhoplimit) | const |  |
| [`IPV6_HOPLIMIT`](#ipv6-hoplimit) | const |  |
| [`IPV6_RECVHOPOPTS`](#ipv6-recvhopopts) | const |  |
| [`IPV6_HOPOPTS`](#ipv6-hopopts) | const |  |
| [`IPV6_RTHDRDSTOPTS`](#ipv6-rthdrdstopts) | const |  |
| [`IPV6_RECVRTHDR`](#ipv6-recvrthdr) | const |  |
| [`IPV6_RTHDR`](#ipv6-rthdr) | const |  |
| [`IPV6_RECVDSTOPTS`](#ipv6-recvdstopts) | const |  |
| [`IPV6_DSTOPTS`](#ipv6-dstopts) | const |  |
| [`IPV6_RECVPATHMTU`](#ipv6-recvpathmtu) | const |  |
| [`IPV6_PATHMTU`](#ipv6-pathmtu) | const |  |
| [`IPV6_DONTFRAG`](#ipv6-dontfrag) | const |  |
| [`IPV6_RECVTCLASS`](#ipv6-recvtclass) | const |  |
| [`IPV6_TCLASS`](#ipv6-tclass) | const |  |
| [`IPV6_AUTOFLOWLABEL`](#ipv6-autoflowlabel) | const |  |
| [`IPV6_ADDR_PREFERENCES`](#ipv6-addr-preferences) | const |  |
| [`IPV6_MINHOPCOUNT`](#ipv6-minhopcount) | const |  |
| [`IPV6_ORIGDSTADDR`](#ipv6-origdstaddr) | const |  |
| [`IPV6_RECVORIGDSTADDR`](#ipv6-recvorigdstaddr) | const |  |
| [`IPV6_TRANSPARENT`](#ipv6-transparent) | const |  |
| [`IPV6_UNICAST_IF`](#ipv6-unicast-if) | const |  |
| [`IPV6_PREFER_SRC_TMP`](#ipv6-prefer-src-tmp) | const |  |
| [`IPV6_PREFER_SRC_PUBLIC`](#ipv6-prefer-src-public) | const |  |
| [`IPV6_PREFER_SRC_PUBTMP_DEFAULT`](#ipv6-prefer-src-pubtmp-default) | const |  |
| [`IPV6_PREFER_SRC_COA`](#ipv6-prefer-src-coa) | const |  |
| [`IPV6_PREFER_SRC_HOME`](#ipv6-prefer-src-home) | const |  |
| [`IPV6_PREFER_SRC_CGA`](#ipv6-prefer-src-cga) | const |  |
| [`IPV6_PREFER_SRC_NONCGA`](#ipv6-prefer-src-noncga) | const |  |
| [`IPV6_PMTUDISC_DONT`](#ipv6-pmtudisc-dont) | const |  |
| [`IPV6_PMTUDISC_WANT`](#ipv6-pmtudisc-want) | const |  |
| [`IPV6_PMTUDISC_DO`](#ipv6-pmtudisc-do) | const |  |
| [`IPV6_PMTUDISC_PROBE`](#ipv6-pmtudisc-probe) | const |  |
| [`IPV6_PMTUDISC_INTERFACE`](#ipv6-pmtudisc-interface) | const |  |
| [`IPV6_PMTUDISC_OMIT`](#ipv6-pmtudisc-omit) | const |  |
| [`TCP_NODELAY`](#tcp-nodelay) | const |  |
| [`TCP_MAXSEG`](#tcp-maxseg) | const |  |
| [`TCP_CORK`](#tcp-cork) | const |  |
| [`TCP_KEEPIDLE`](#tcp-keepidle) | const |  |
| [`TCP_KEEPINTVL`](#tcp-keepintvl) | const |  |
| [`TCP_KEEPCNT`](#tcp-keepcnt) | const |  |
| [`TCP_SYNCNT`](#tcp-syncnt) | const |  |
| [`TCP_LINGER2`](#tcp-linger2) | const |  |
| [`TCP_DEFER_ACCEPT`](#tcp-defer-accept) | const |  |
| [`TCP_WINDOW_CLAMP`](#tcp-window-clamp) | const |  |
| [`TCP_INFO`](#tcp-info) | const |  |
| [`TCP_QUICKACK`](#tcp-quickack) | const |  |
| [`TCP_CONGESTION`](#tcp-congestion) | const |  |
| [`TCP_MD5SIG`](#tcp-md5sig) | const |  |
| [`TCP_COOKIE_TRANSACTIONS`](#tcp-cookie-transactions) | const |  |
| [`TCP_THIN_LINEAR_TIMEOUTS`](#tcp-thin-linear-timeouts) | const |  |
| [`TCP_THIN_DUPACK`](#tcp-thin-dupack) | const |  |
| [`TCP_USER_TIMEOUT`](#tcp-user-timeout) | const |  |
| [`TCP_REPAIR`](#tcp-repair) | const |  |
| [`TCP_REPAIR_QUEUE`](#tcp-repair-queue) | const |  |
| [`TCP_QUEUE_SEQ`](#tcp-queue-seq) | const |  |
| [`TCP_REPAIR_OPTIONS`](#tcp-repair-options) | const |  |
| [`TCP_FASTOPEN`](#tcp-fastopen) | const |  |
| [`TCP_TIMESTAMP`](#tcp-timestamp) | const |  |
| [`TCP_NOTSENT_LOWAT`](#tcp-notsent-lowat) | const |  |
| [`TCP_CC_INFO`](#tcp-cc-info) | const |  |
| [`TCP_SAVE_SYN`](#tcp-save-syn) | const |  |
| [`TCP_SAVED_SYN`](#tcp-saved-syn) | const |  |
| [`TCP_REPAIR_WINDOW`](#tcp-repair-window) | const |  |
| [`TCP_FASTOPEN_CONNECT`](#tcp-fastopen-connect) | const |  |
| [`TCP_ULP`](#tcp-ulp) | const |  |
| [`TCP_MD5SIG_EXT`](#tcp-md5sig-ext) | const |  |
| [`TCP_FASTOPEN_KEY`](#tcp-fastopen-key) | const |  |
| [`TCP_FASTOPEN_NO_COOKIE`](#tcp-fastopen-no-cookie) | const |  |
| [`TCP_ZEROCOPY_RECEIVE`](#tcp-zerocopy-receive) | const |  |
| [`TCP_INQ`](#tcp-inq) | const |  |
| [`TCP_CM_INQ`](#tcp-cm-inq) | const |  |
| [`TCP_MD5SIG_MAXKEYLEN`](#tcp-md5sig-maxkeylen) | const |  |
| [`SO_DEBUG`](#so-debug) | const |  |
| [`SHUT_RD`](#shut-rd) | const |  |
| [`SHUT_WR`](#shut-wr) | const |  |
| [`SHUT_RDWR`](#shut-rdwr) | const |  |
| [`LOCK_SH`](#lock-sh) | const |  |
| [`LOCK_EX`](#lock-ex) | const |  |
| [`LOCK_NB`](#lock-nb) | const |  |
| [`LOCK_UN`](#lock-un) | const |  |
| [`SS_ONSTACK`](#ss-onstack) | const |  |
| [`SS_DISABLE`](#ss-disable) | const |  |
| [`PATH_MAX`](#path-max) | const |  |
| [`UIO_MAXIOV`](#uio-maxiov) | const |  |
| [`FD_SETSIZE`](#fd-setsize) | const |  |
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
| [`EPOLL_CTL_ADD`](#epoll-ctl-add) | const |  |
| [`EPOLL_CTL_MOD`](#epoll-ctl-mod) | const |  |
| [`EPOLL_CTL_DEL`](#epoll-ctl-del) | const |  |
| [`MNT_FORCE`](#mnt-force) | const |  |
| [`MNT_DETACH`](#mnt-detach) | const |  |
| [`MNT_EXPIRE`](#mnt-expire) | const |  |
| [`UMOUNT_NOFOLLOW`](#umount-nofollow) | const |  |
| [`Q_GETFMT`](#q-getfmt) | const |  |
| [`Q_GETINFO`](#q-getinfo) | const |  |
| [`Q_SETINFO`](#q-setinfo) | const |  |
| [`QIF_BLIMITS`](#qif-blimits) | const |  |
| [`QIF_SPACE`](#qif-space) | const |  |
| [`QIF_ILIMITS`](#qif-ilimits) | const |  |
| [`QIF_INODES`](#qif-inodes) | const |  |
| [`QIF_BTIME`](#qif-btime) | const |  |
| [`QIF_ITIME`](#qif-itime) | const |  |
| [`QIF_LIMITS`](#qif-limits) | const |  |
| [`QIF_USAGE`](#qif-usage) | const |  |
| [`QIF_TIMES`](#qif-times) | const |  |
| [`QIF_ALL`](#qif-all) | const |  |
| [`Q_SYNC`](#q-sync) | const |  |
| [`Q_QUOTAON`](#q-quotaon) | const |  |
| [`Q_QUOTAOFF`](#q-quotaoff) | const |  |
| [`Q_GETQUOTA`](#q-getquota) | const |  |
| [`Q_SETQUOTA`](#q-setquota) | const |  |
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
| [`CLONE_VM`](#clone-vm) | const |  |
| [`CLONE_FS`](#clone-fs) | const |  |
| [`CLONE_FILES`](#clone-files) | const |  |
| [`CLONE_SIGHAND`](#clone-sighand) | const |  |
| [`CLONE_PTRACE`](#clone-ptrace) | const |  |
| [`CLONE_VFORK`](#clone-vfork) | const |  |
| [`CLONE_PARENT`](#clone-parent) | const |  |
| [`CLONE_THREAD`](#clone-thread) | const |  |
| [`CLONE_NEWNS`](#clone-newns) | const |  |
| [`CLONE_SYSVSEM`](#clone-sysvsem) | const |  |
| [`CLONE_SETTLS`](#clone-settls) | const |  |
| [`CLONE_PARENT_SETTID`](#clone-parent-settid) | const |  |
| [`CLONE_CHILD_CLEARTID`](#clone-child-cleartid) | const |  |
| [`CLONE_DETACHED`](#clone-detached) | const |  |
| [`CLONE_UNTRACED`](#clone-untraced) | const |  |
| [`CLONE_CHILD_SETTID`](#clone-child-settid) | const |  |
| [`CLONE_NEWCGROUP`](#clone-newcgroup) | const |  |
| [`CLONE_NEWUTS`](#clone-newuts) | const |  |
| [`CLONE_NEWIPC`](#clone-newipc) | const |  |
| [`CLONE_NEWUSER`](#clone-newuser) | const |  |
| [`CLONE_NEWPID`](#clone-newpid) | const |  |
| [`CLONE_NEWNET`](#clone-newnet) | const |  |
| [`CLONE_IO`](#clone-io) | const |  |
| [`WNOHANG`](#wnohang) | const |  |
| [`WUNTRACED`](#wuntraced) | const |  |
| [`WSTOPPED`](#wstopped) | const |  |
| [`WEXITED`](#wexited) | const |  |
| [`WCONTINUED`](#wcontinued) | const |  |
| [`WNOWAIT`](#wnowait) | const |  |
| [`ADDR_NO_RANDOMIZE`](#addr-no-randomize) | const |  |
| [`MMAP_PAGE_ZERO`](#mmap-page-zero) | const |  |
| [`ADDR_COMPAT_LAYOUT`](#addr-compat-layout) | const |  |
| [`READ_IMPLIES_EXEC`](#read-implies-exec) | const |  |
| [`ADDR_LIMIT_32BIT`](#addr-limit-32bit) | const |  |
| [`SHORT_INODE`](#short-inode) | const |  |
| [`WHOLE_SECONDS`](#whole-seconds) | const |  |
| [`STICKY_TIMEOUTS`](#sticky-timeouts) | const |  |
| [`ADDR_LIMIT_3GB`](#addr-limit-3gb) | const |  |
| [`PTRACE_O_TRACESYSGOOD`](#ptrace-o-tracesysgood) | const |  |
| [`PTRACE_O_TRACEFORK`](#ptrace-o-tracefork) | const |  |
| [`PTRACE_O_TRACEVFORK`](#ptrace-o-tracevfork) | const |  |
| [`PTRACE_O_TRACECLONE`](#ptrace-o-traceclone) | const |  |
| [`PTRACE_O_TRACEEXEC`](#ptrace-o-traceexec) | const |  |
| [`PTRACE_O_TRACEVFORKDONE`](#ptrace-o-tracevforkdone) | const |  |
| [`PTRACE_O_TRACEEXIT`](#ptrace-o-traceexit) | const |  |
| [`PTRACE_O_TRACESECCOMP`](#ptrace-o-traceseccomp) | const |  |
| [`PTRACE_O_SUSPEND_SECCOMP`](#ptrace-o-suspend-seccomp) | const |  |
| [`PTRACE_O_EXITKILL`](#ptrace-o-exitkill) | const |  |
| [`PTRACE_O_MASK`](#ptrace-o-mask) | const |  |
| [`PTRACE_EVENT_FORK`](#ptrace-event-fork) | const |  |
| [`PTRACE_EVENT_VFORK`](#ptrace-event-vfork) | const |  |
| [`PTRACE_EVENT_CLONE`](#ptrace-event-clone) | const |  |
| [`PTRACE_EVENT_EXEC`](#ptrace-event-exec) | const |  |
| [`PTRACE_EVENT_VFORK_DONE`](#ptrace-event-vfork-done) | const |  |
| [`PTRACE_EVENT_EXIT`](#ptrace-event-exit) | const |  |
| [`PTRACE_EVENT_SECCOMP`](#ptrace-event-seccomp) | const |  |
| [`__WNOTHREAD`](#wnothread) | const |  |
| [`__WALL`](#wall) | const |  |
| [`__WCLONE`](#wclone) | const |  |
| [`SPLICE_F_MOVE`](#splice-f-move) | const |  |
| [`SPLICE_F_NONBLOCK`](#splice-f-nonblock) | const |  |
| [`SPLICE_F_MORE`](#splice-f-more) | const |  |
| [`SPLICE_F_GIFT`](#splice-f-gift) | const |  |
| [`RTLD_LOCAL`](#rtld-local) | const |  |
| [`RTLD_LAZY`](#rtld-lazy) | const |  |
| [`POSIX_FADV_NORMAL`](#posix-fadv-normal) | const |  |
| [`POSIX_FADV_RANDOM`](#posix-fadv-random) | const |  |
| [`POSIX_FADV_SEQUENTIAL`](#posix-fadv-sequential) | const |  |
| [`POSIX_FADV_WILLNEED`](#posix-fadv-willneed) | const |  |
| [`AT_FDCWD`](#at-fdcwd) | const |  |
| [`AT_SYMLINK_NOFOLLOW`](#at-symlink-nofollow) | const |  |
| [`AT_REMOVEDIR`](#at-removedir) | const |  |
| [`AT_SYMLINK_FOLLOW`](#at-symlink-follow) | const |  |
| [`AT_NO_AUTOMOUNT`](#at-no-automount) | const |  |
| [`AT_EMPTY_PATH`](#at-empty-path) | const |  |
| [`AT_RECURSIVE`](#at-recursive) | const |  |
| [`LOG_CRON`](#log-cron) | const |  |
| [`LOG_AUTHPRIV`](#log-authpriv) | const |  |
| [`LOG_FTP`](#log-ftp) | const |  |
| [`LOG_PERROR`](#log-perror) | const |  |
| [`PIPE_BUF`](#pipe-buf) | const |  |
| [`SI_LOAD_SHIFT`](#si-load-shift) | const |  |
| [`SI_USER`](#si-user) | const |  |
| [`SI_KERNEL`](#si-kernel) | const |  |
| [`SI_QUEUE`](#si-queue) | const |  |
| [`SI_TIMER`](#si-timer) | const |  |
| [`SI_MESGQ`](#si-mesgq) | const |  |
| [`SI_ASYNCIO`](#si-asyncio) | const |  |
| [`SI_SIGIO`](#si-sigio) | const |  |
| [`SI_TKILL`](#si-tkill) | const |  |
| [`SI_ASYNCNL`](#si-asyncnl) | const |  |
| [`BUS_ADRALN`](#bus-adraln) | const |  |
| [`BUS_ADRERR`](#bus-adrerr) | const |  |
| [`BUS_OBJERR`](#bus-objerr) | const |  |
| [`BUS_MCEERR_AR`](#bus-mceerr-ar) | const |  |
| [`BUS_MCEERR_AO`](#bus-mceerr-ao) | const |  |
| [`TRAP_BRKPT`](#trap-brkpt) | const |  |
| [`TRAP_TRACE`](#trap-trace) | const |  |
| [`TRAP_BRANCH`](#trap-branch) | const |  |
| [`TRAP_HWBKPT`](#trap-hwbkpt) | const |  |
| [`TRAP_UNK`](#trap-unk) | const |  |
| [`CLD_EXITED`](#cld-exited) | const |  |
| [`CLD_KILLED`](#cld-killed) | const |  |
| [`CLD_DUMPED`](#cld-dumped) | const |  |
| [`CLD_TRAPPED`](#cld-trapped) | const |  |
| [`CLD_STOPPED`](#cld-stopped) | const |  |
| [`CLD_CONTINUED`](#cld-continued) | const |  |
| [`SIGEV_SIGNAL`](#sigev-signal) | const |  |
| [`SIGEV_NONE`](#sigev-none) | const |  |
| [`SIGEV_THREAD`](#sigev-thread) | const |  |
| [`P_ALL`](#p-all) | const |  |
| [`P_PID`](#p-pid) | const |  |
| [`P_PGID`](#p-pgid) | const |  |
| [`P_PIDFD`](#p-pidfd) | const |  |
| [`UTIME_OMIT`](#utime-omit) | const |  |
| [`UTIME_NOW`](#utime-now) | const |  |
| [`POLLIN`](#pollin) | const |  |
| [`POLLPRI`](#pollpri) | const |  |
| [`POLLOUT`](#pollout) | const |  |
| [`POLLERR`](#pollerr) | const |  |
| [`POLLHUP`](#pollhup) | const |  |
| [`POLLNVAL`](#pollnval) | const |  |
| [`POLLRDNORM`](#pollrdnorm) | const |  |
| [`POLLRDBAND`](#pollrdband) | const |  |
| [`POLLRDHUP`](#pollrdhup) | const |  |
| [`IPTOS_LOWDELAY`](#iptos-lowdelay) | const |  |
| [`IPTOS_THROUGHPUT`](#iptos-throughput) | const |  |
| [`IPTOS_RELIABILITY`](#iptos-reliability) | const |  |
| [`IPTOS_MINCOST`](#iptos-mincost) | const |  |
| [`IPTOS_PREC_NETCONTROL`](#iptos-prec-netcontrol) | const |  |
| [`IPTOS_PREC_INTERNETCONTROL`](#iptos-prec-internetcontrol) | const |  |
| [`IPTOS_PREC_CRITIC_ECP`](#iptos-prec-critic-ecp) | const |  |
| [`IPTOS_PREC_FLASHOVERRIDE`](#iptos-prec-flashoverride) | const |  |
| [`IPTOS_PREC_FLASH`](#iptos-prec-flash) | const |  |
| [`IPTOS_PREC_IMMEDIATE`](#iptos-prec-immediate) | const |  |
| [`IPTOS_PREC_PRIORITY`](#iptos-prec-priority) | const |  |
| [`IPTOS_PREC_ROUTINE`](#iptos-prec-routine) | const |  |
| [`IPTOS_ECN_MASK`](#iptos-ecn-mask) | const |  |
| [`IPTOS_ECN_ECT1`](#iptos-ecn-ect1) | const |  |
| [`IPTOS_ECN_ECT0`](#iptos-ecn-ect0) | const |  |
| [`IPTOS_ECN_CE`](#iptos-ecn-ce) | const |  |
| [`IPOPT_COPY`](#ipopt-copy) | const |  |
| [`IPOPT_CLASS_MASK`](#ipopt-class-mask) | const |  |
| [`IPOPT_NUMBER_MASK`](#ipopt-number-mask) | const |  |
| [`IPOPT_CONTROL`](#ipopt-control) | const |  |
| [`IPOPT_RESERVED1`](#ipopt-reserved1) | const |  |
| [`IPOPT_MEASUREMENT`](#ipopt-measurement) | const |  |
| [`IPOPT_RESERVED2`](#ipopt-reserved2) | const |  |
| [`IPOPT_END`](#ipopt-end) | const |  |
| [`IPOPT_NOOP`](#ipopt-noop) | const |  |
| [`IPOPT_SEC`](#ipopt-sec) | const |  |
| [`IPOPT_LSRR`](#ipopt-lsrr) | const |  |
| [`IPOPT_TIMESTAMP`](#ipopt-timestamp) | const |  |
| [`IPOPT_RR`](#ipopt-rr) | const |  |
| [`IPOPT_SID`](#ipopt-sid) | const |  |
| [`IPOPT_SSRR`](#ipopt-ssrr) | const |  |
| [`IPOPT_RA`](#ipopt-ra) | const |  |
| [`IPVERSION`](#ipversion) | const |  |
| [`MAXTTL`](#maxttl) | const |  |
| [`IPDEFTTL`](#ipdefttl) | const |  |
| [`IPOPT_OPTVAL`](#ipopt-optval) | const |  |
| [`IPOPT_OLEN`](#ipopt-olen) | const |  |
| [`IPOPT_OFFSET`](#ipopt-offset) | const |  |
| [`IPOPT_MINOFF`](#ipopt-minoff) | const |  |
| [`MAX_IPOPTLEN`](#max-ipoptlen) | const |  |
| [`IPOPT_NOP`](#ipopt-nop) | const |  |
| [`IPOPT_EOL`](#ipopt-eol) | const |  |
| [`IPOPT_TS`](#ipopt-ts) | const |  |
| [`IPOPT_TS_TSONLY`](#ipopt-ts-tsonly) | const |  |
| [`IPOPT_TS_TSANDADDR`](#ipopt-ts-tsandaddr) | const |  |
| [`IPOPT_TS_PRESPEC`](#ipopt-ts-prespec) | const |  |
| [`ARPOP_RREQUEST`](#arpop-rrequest) | const |  |
| [`ARPOP_RREPLY`](#arpop-rreply) | const |  |
| [`ARPOP_InREQUEST`](#arpop-inrequest) | const |  |
| [`ARPOP_InREPLY`](#arpop-inreply) | const |  |
| [`ARPOP_NAK`](#arpop-nak) | const |  |
| [`ATF_NETMASK`](#atf-netmask) | const |  |
| [`ATF_DONTPUB`](#atf-dontpub) | const |  |
| [`ARPHRD_NETROM`](#arphrd-netrom) | const |  |
| [`ARPHRD_ETHER`](#arphrd-ether) | const |  |
| [`ARPHRD_EETHER`](#arphrd-eether) | const |  |
| [`ARPHRD_AX25`](#arphrd-ax25) | const |  |
| [`ARPHRD_PRONET`](#arphrd-pronet) | const |  |
| [`ARPHRD_CHAOS`](#arphrd-chaos) | const |  |
| [`ARPHRD_IEEE802`](#arphrd-ieee802) | const |  |
| [`ARPHRD_ARCNET`](#arphrd-arcnet) | const |  |
| [`ARPHRD_APPLETLK`](#arphrd-appletlk) | const |  |
| [`ARPHRD_DLCI`](#arphrd-dlci) | const |  |
| [`ARPHRD_ATM`](#arphrd-atm) | const |  |
| [`ARPHRD_METRICOM`](#arphrd-metricom) | const |  |
| [`ARPHRD_IEEE1394`](#arphrd-ieee1394) | const |  |
| [`ARPHRD_EUI64`](#arphrd-eui64) | const |  |
| [`ARPHRD_INFINIBAND`](#arphrd-infiniband) | const |  |
| [`ARPHRD_SLIP`](#arphrd-slip) | const |  |
| [`ARPHRD_CSLIP`](#arphrd-cslip) | const |  |
| [`ARPHRD_SLIP6`](#arphrd-slip6) | const |  |
| [`ARPHRD_CSLIP6`](#arphrd-cslip6) | const |  |
| [`ARPHRD_RSRVD`](#arphrd-rsrvd) | const |  |
| [`ARPHRD_ADAPT`](#arphrd-adapt) | const |  |
| [`ARPHRD_ROSE`](#arphrd-rose) | const |  |
| [`ARPHRD_X25`](#arphrd-x25) | const |  |
| [`ARPHRD_HWX25`](#arphrd-hwx25) | const |  |
| [`ARPHRD_CAN`](#arphrd-can) | const |  |
| [`ARPHRD_PPP`](#arphrd-ppp) | const |  |
| [`ARPHRD_CISCO`](#arphrd-cisco) | const |  |
| [`ARPHRD_HDLC`](#arphrd-hdlc) | const |  |
| [`ARPHRD_LAPB`](#arphrd-lapb) | const |  |
| [`ARPHRD_DDCMP`](#arphrd-ddcmp) | const |  |
| [`ARPHRD_RAWHDLC`](#arphrd-rawhdlc) | const |  |
| [`ARPHRD_TUNNEL`](#arphrd-tunnel) | const |  |
| [`ARPHRD_TUNNEL6`](#arphrd-tunnel6) | const |  |
| [`ARPHRD_FRAD`](#arphrd-frad) | const |  |
| [`ARPHRD_SKIP`](#arphrd-skip) | const |  |
| [`ARPHRD_LOOPBACK`](#arphrd-loopback) | const |  |
| [`ARPHRD_LOCALTLK`](#arphrd-localtlk) | const |  |
| [`ARPHRD_FDDI`](#arphrd-fddi) | const |  |
| [`ARPHRD_BIF`](#arphrd-bif) | const |  |
| [`ARPHRD_SIT`](#arphrd-sit) | const |  |
| [`ARPHRD_IPDDP`](#arphrd-ipddp) | const |  |
| [`ARPHRD_IPGRE`](#arphrd-ipgre) | const |  |
| [`ARPHRD_PIMREG`](#arphrd-pimreg) | const |  |
| [`ARPHRD_HIPPI`](#arphrd-hippi) | const |  |
| [`ARPHRD_ASH`](#arphrd-ash) | const |  |
| [`ARPHRD_ECONET`](#arphrd-econet) | const |  |
| [`ARPHRD_IRDA`](#arphrd-irda) | const |  |
| [`ARPHRD_FCPP`](#arphrd-fcpp) | const |  |
| [`ARPHRD_FCAL`](#arphrd-fcal) | const |  |
| [`ARPHRD_FCPL`](#arphrd-fcpl) | const |  |
| [`ARPHRD_FCFABRIC`](#arphrd-fcfabric) | const |  |
| [`ARPHRD_IEEE802_TR`](#arphrd-ieee802-tr) | const |  |
| [`ARPHRD_IEEE80211`](#arphrd-ieee80211) | const |  |
| [`ARPHRD_IEEE80211_PRISM`](#arphrd-ieee80211-prism) | const |  |
| [`ARPHRD_IEEE80211_RADIOTAP`](#arphrd-ieee80211-radiotap) | const |  |
| [`ARPHRD_IEEE802154`](#arphrd-ieee802154) | const |  |
| [`ARPHRD_VOID`](#arphrd-void) | const |  |
| [`ARPHRD_NONE`](#arphrd-none) | const |  |
| [`IFF_TUN`](#iff-tun) | const |  |
| [`IFF_TAP`](#iff-tap) | const |  |
| [`IFF_NAPI`](#iff-napi) | const |  |
| [`IFF_NAPI_FRAGS`](#iff-napi-frags) | const |  |
| [`IFF_NO_CARRIER`](#iff-no-carrier) | const |  |
| [`IFF_NO_PI`](#iff-no-pi) | const |  |
| [`TUN_READQ_SIZE`](#tun-readq-size) | const |  |
| [`TUN_TUN_DEV`](#tun-tun-dev) | const |  |
| [`TUN_TAP_DEV`](#tun-tap-dev) | const |  |
| [`TUN_TYPE_MASK`](#tun-type-mask) | const |  |
| [`IFF_ONE_QUEUE`](#iff-one-queue) | const |  |
| [`IFF_VNET_HDR`](#iff-vnet-hdr) | const |  |
| [`IFF_TUN_EXCL`](#iff-tun-excl) | const |  |
| [`IFF_MULTI_QUEUE`](#iff-multi-queue) | const |  |
| [`IFF_ATTACH_QUEUE`](#iff-attach-queue) | const |  |
| [`IFF_DETACH_QUEUE`](#iff-detach-queue) | const |  |
| [`IFF_PERSIST`](#iff-persist) | const |  |
| [`IFF_NOFILTER`](#iff-nofilter) | const |  |
| [`TUN_TX_TIMESTAMP`](#tun-tx-timestamp) | const |  |
| [`TUN_F_CSUM`](#tun-f-csum) | const |  |
| [`TUN_F_TSO4`](#tun-f-tso4) | const |  |
| [`TUN_F_TSO6`](#tun-f-tso6) | const |  |
| [`TUN_F_TSO_ECN`](#tun-f-tso-ecn) | const |  |
| [`TUN_F_UFO`](#tun-f-ufo) | const |  |
| [`TUN_F_USO4`](#tun-f-uso4) | const |  |
| [`TUN_F_USO6`](#tun-f-uso6) | const |  |
| [`TUN_PKT_STRIP`](#tun-pkt-strip) | const |  |
| [`TUN_FLT_ALLMULTI`](#tun-flt-allmulti) | const |  |
| [`T_TYPE`](#t-type) | const |  |
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
| [`FS_IOC_GETFLAGS`](#fs-ioc-getflags) | const |  |
| [`FS_IOC_SETFLAGS`](#fs-ioc-setflags) | const |  |
| [`FS_IOC_GETVERSION`](#fs-ioc-getversion) | const |  |
| [`FS_IOC_SETVERSION`](#fs-ioc-setversion) | const |  |
| [`FS_IOC32_GETFLAGS`](#fs-ioc32-getflags) | const |  |
| [`FS_IOC32_SETFLAGS`](#fs-ioc32-setflags) | const |  |
| [`FS_IOC32_GETVERSION`](#fs-ioc32-getversion) | const |  |
| [`FS_IOC32_SETVERSION`](#fs-ioc32-setversion) | const |  |
| [`FICLONE`](#ficlone) | const |  |
| [`FICLONERANGE`](#ficlonerange) | const |  |
| [`ADFS_SUPER_MAGIC`](#adfs-super-magic) | const |  |
| [`AFFS_SUPER_MAGIC`](#affs-super-magic) | const |  |
| [`AFS_SUPER_MAGIC`](#afs-super-magic) | const |  |
| [`AUTOFS_SUPER_MAGIC`](#autofs-super-magic) | const |  |
| [`BPF_FS_MAGIC`](#bpf-fs-magic) | const |  |
| [`BTRFS_SUPER_MAGIC`](#btrfs-super-magic) | const |  |
| [`CGROUP2_SUPER_MAGIC`](#cgroup2-super-magic) | const |  |
| [`CGROUP_SUPER_MAGIC`](#cgroup-super-magic) | const |  |
| [`CODA_SUPER_MAGIC`](#coda-super-magic) | const |  |
| [`CRAMFS_MAGIC`](#cramfs-magic) | const |  |
| [`DEBUGFS_MAGIC`](#debugfs-magic) | const |  |
| [`DEVPTS_SUPER_MAGIC`](#devpts-super-magic) | const |  |
| [`ECRYPTFS_SUPER_MAGIC`](#ecryptfs-super-magic) | const |  |
| [`EFS_SUPER_MAGIC`](#efs-super-magic) | const |  |
| [`EXT2_SUPER_MAGIC`](#ext2-super-magic) | const |  |
| [`EXT3_SUPER_MAGIC`](#ext3-super-magic) | const |  |
| [`EXT4_SUPER_MAGIC`](#ext4-super-magic) | const |  |
| [`F2FS_SUPER_MAGIC`](#f2fs-super-magic) | const |  |
| [`FUSE_SUPER_MAGIC`](#fuse-super-magic) | const |  |
| [`FUTEXFS_SUPER_MAGIC`](#futexfs-super-magic) | const |  |
| [`HOSTFS_SUPER_MAGIC`](#hostfs-super-magic) | const |  |
| [`HPFS_SUPER_MAGIC`](#hpfs-super-magic) | const |  |
| [`HUGETLBFS_MAGIC`](#hugetlbfs-magic) | const |  |
| [`ISOFS_SUPER_MAGIC`](#isofs-super-magic) | const |  |
| [`JFFS2_SUPER_MAGIC`](#jffs2-super-magic) | const |  |
| [`MINIX2_SUPER_MAGIC2`](#minix2-super-magic2) | const |  |
| [`MINIX2_SUPER_MAGIC`](#minix2-super-magic) | const |  |
| [`MINIX3_SUPER_MAGIC`](#minix3-super-magic) | const |  |
| [`MINIX_SUPER_MAGIC2`](#minix-super-magic2) | const |  |
| [`MINIX_SUPER_MAGIC`](#minix-super-magic) | const |  |
| [`MSDOS_SUPER_MAGIC`](#msdos-super-magic) | const |  |
| [`NCP_SUPER_MAGIC`](#ncp-super-magic) | const |  |
| [`NFS_SUPER_MAGIC`](#nfs-super-magic) | const |  |
| [`NILFS_SUPER_MAGIC`](#nilfs-super-magic) | const |  |
| [`OCFS2_SUPER_MAGIC`](#ocfs2-super-magic) | const |  |
| [`OPENPROM_SUPER_MAGIC`](#openprom-super-magic) | const |  |
| [`OVERLAYFS_SUPER_MAGIC`](#overlayfs-super-magic) | const |  |
| [`PROC_SUPER_MAGIC`](#proc-super-magic) | const |  |
| [`QNX4_SUPER_MAGIC`](#qnx4-super-magic) | const |  |
| [`QNX6_SUPER_MAGIC`](#qnx6-super-magic) | const |  |
| [`RDTGROUP_SUPER_MAGIC`](#rdtgroup-super-magic) | const |  |
| [`REISERFS_SUPER_MAGIC`](#reiserfs-super-magic) | const |  |
| [`SECURITYFS_MAGIC`](#securityfs-magic) | const |  |
| [`SELINUX_MAGIC`](#selinux-magic) | const |  |
| [`SMACK_MAGIC`](#smack-magic) | const |  |
| [`SMB_SUPER_MAGIC`](#smb-super-magic) | const |  |
| [`SYSFS_MAGIC`](#sysfs-magic) | const |  |
| [`TMPFS_MAGIC`](#tmpfs-magic) | const |  |
| [`TRACEFS_MAGIC`](#tracefs-magic) | const |  |
| [`UDF_SUPER_MAGIC`](#udf-super-magic) | const |  |
| [`USBDEVICE_SUPER_MAGIC`](#usbdevice-super-magic) | const |  |
| [`XENFS_SUPER_MAGIC`](#xenfs-super-magic) | const |  |
| [`NSFS_MAGIC`](#nsfs-magic) | const |  |
| [`AT_STATX_SYNC_TYPE`](#at-statx-sync-type) | const |  |
| [`AT_STATX_SYNC_AS_STAT`](#at-statx-sync-as-stat) | const |  |
| [`AT_STATX_FORCE_SYNC`](#at-statx-force-sync) | const |  |
| [`AT_STATX_DONT_SYNC`](#at-statx-dont-sync) | const |  |
| [`STATX_TYPE`](#statx-type) | const |  |
| [`STATX_MODE`](#statx-mode) | const |  |
| [`STATX_NLINK`](#statx-nlink) | const |  |
| [`STATX_UID`](#statx-uid) | const |  |
| [`STATX_GID`](#statx-gid) | const |  |
| [`STATX_ATIME`](#statx-atime) | const |  |
| [`STATX_MTIME`](#statx-mtime) | const |  |
| [`STATX_CTIME`](#statx-ctime) | const |  |
| [`STATX_INO`](#statx-ino) | const |  |
| [`STATX_SIZE`](#statx-size) | const |  |
| [`STATX_BLOCKS`](#statx-blocks) | const |  |
| [`STATX_BASIC_STATS`](#statx-basic-stats) | const |  |
| [`STATX_BTIME`](#statx-btime) | const |  |
| [`STATX_ALL`](#statx-all) | const |  |
| [`STATX_MNT_ID`](#statx-mnt-id) | const |  |
| [`STATX_DIOALIGN`](#statx-dioalign) | const |  |
| [`STATX__RESERVED`](#statx-reserved) | const |  |
| [`STATX_ATTR_COMPRESSED`](#statx-attr-compressed) | const |  |
| [`STATX_ATTR_IMMUTABLE`](#statx-attr-immutable) | const |  |
| [`STATX_ATTR_APPEND`](#statx-attr-append) | const |  |
| [`STATX_ATTR_NODUMP`](#statx-attr-nodump) | const |  |
| [`STATX_ATTR_ENCRYPTED`](#statx-attr-encrypted) | const |  |
| [`STATX_ATTR_AUTOMOUNT`](#statx-attr-automount) | const |  |
| [`STATX_ATTR_MOUNT_ROOT`](#statx-attr-mount-root) | const |  |
| [`STATX_ATTR_VERITY`](#statx-attr-verity) | const |  |
| [`STATX_ATTR_DAX`](#statx-attr-dax) | const |  |
| [`_IOC_NRBITS`](#ioc-nrbits) | const |  |
| [`_IOC_TYPEBITS`](#ioc-typebits) | const |  |
| [`_IOC_SIZEBITS`](#ioc-sizebits) | const |  |
| [`_IOC_DIRBITS`](#ioc-dirbits) | const |  |
| [`_IOC_NONE`](#ioc-none) | const |  |
| [`_IOC_WRITE`](#ioc-write) | const |  |
| [`_IOC_READ`](#ioc-read) | const |  |
| [`_IOC_NRMASK`](#ioc-nrmask) | const |  |
| [`_IOC_TYPEMASK`](#ioc-typemask) | const |  |
| [`_IOC_SIZEMASK`](#ioc-sizemask) | const |  |
| [`_IOC_DIRMASK`](#ioc-dirmask) | const |  |
| [`_IOC_NRSHIFT`](#ioc-nrshift) | const |  |
| [`_IOC_TYPESHIFT`](#ioc-typeshift) | const |  |
| [`_IOC_SIZESHIFT`](#ioc-sizeshift) | const |  |
| [`_IOC_DIRSHIFT`](#ioc-dirshift) | const |  |

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for group`

- <span id="group-clone"></span>`fn clone(&self) -> group` — [`group`](../index.md#group)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for utimbuf`

- <span id="utimbuf-clone"></span>`fn clone(&self) -> utimbuf` — [`utimbuf`](../index.md#utimbuf)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for timeval`

- <span id="timeval-clone"></span>`fn clone(&self) -> timeval` — [`timeval`](../index.md#timeval)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for rlimit`

- <span id="rlimit-clone"></span>`fn clone(&self) -> rlimit` — [`rlimit`](../index.md#rlimit)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for rusage`

- <span id="rusage-clone"></span>`fn clone(&self) -> rusage` — [`rusage`](../index.md#rusage)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for ipv6_mreq`

- <span id="ipv6-mreq-clone"></span>`fn clone(&self) -> ipv6_mreq` — [`ipv6_mreq`](../index.md#ipv6-mreq)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for hostent`

- <span id="hostent-clone"></span>`fn clone(&self) -> hostent` — [`hostent`](../index.md#hostent)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for iovec`

- <span id="iovec-clone"></span>`fn clone(&self) -> iovec` — [`iovec`](../index.md#iovec)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for pollfd`

- <span id="pollfd-clone"></span>`fn clone(&self) -> pollfd` — [`pollfd`](../index.md#pollfd)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for winsize`

- <span id="winsize-clone"></span>`fn clone(&self) -> winsize` — [`winsize`](../index.md#winsize)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for linger`

- <span id="linger-clone"></span>`fn clone(&self) -> linger` — [`linger`](../index.md#linger)

##### `impl Copy for linger`

##### `impl Debug for linger`

- <span id="linger-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `sigval`

```rust
struct sigval {
    pub sival_ptr: *mut crate::c_void,
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for sigval`

- <span id="sigval-clone"></span>`fn clone(&self) -> sigval` — [`sigval`](../index.md#sigval)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for itimerval`

- <span id="itimerval-clone"></span>`fn clone(&self) -> itimerval` — [`itimerval`](../index.md#itimerval)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for tms`

- <span id="tms-clone"></span>`fn clone(&self) -> tms` — [`tms`](../index.md#tms)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for servent`

- <span id="servent-clone"></span>`fn clone(&self) -> servent` — [`servent`](../index.md#servent)

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for protoent`

- <span id="protoent-clone"></span>`fn clone(&self) -> protoent` — [`protoent`](../index.md#protoent)

##### `impl Copy for protoent`

##### `impl Debug for protoent`

- <span id="protoent-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in6_addr`

```rust
struct in6_addr {
    pub s6_addr: [u8; 16],
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:47-220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L47-L220)*

#### Trait Implementations

##### `impl Clone for in6_addr`

- <span id="in6-addr-clone"></span>`fn clone(&self) -> in6_addr` — [`in6_addr`](../index.md#in6-addr)

##### `impl Copy for in6_addr`

##### `impl Debug for in6_addr`

- <span id="in6-addr-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `in_addr`

```rust
struct in_addr {
    pub s_addr: crate::in_addr_t,
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:15-230`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L15-L230)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:234-254`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L234-L254)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:234-254`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L234-L254)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:234-254`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L234-L254)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:264-297`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L264-L297)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:264-297`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L264-L297)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:301-323`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L301-L323)*

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

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:301-323`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L301-L323)*

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:40-42`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L40-L42)*

#### Trait Implementations

##### `impl Clone for DIR`

- <span id="dir-clone"></span>`fn clone(&self) -> DIR` — [`DIR`](../index.md#dir)

##### `impl Copy for DIR`

##### `impl Debug for DIR`

- <span id="dir-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FILE`

```rust
enum FILE {
}
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:585-587`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L585-L587)*

#### Trait Implementations

##### `impl Clone for FILE`

- <span id="file-clone"></span>`fn clone(&self) -> FILE` — [`FILE`](../index.md#file)

##### `impl Copy for FILE`

##### `impl Debug for FILE`

- <span id="file-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `timezone`

```rust
enum timezone {
}
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:11-13`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L11-L13)*

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:590`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L590)*

### `isalpha`

```rust
unsafe fn isalpha(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:591`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L591)*

### `iscntrl`

```rust
unsafe fn iscntrl(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:592`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L592)*

### `isdigit`

```rust
unsafe fn isdigit(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:593`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L593)*

### `isgraph`

```rust
unsafe fn isgraph(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:594`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L594)*

### `islower`

```rust
unsafe fn islower(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:595`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L595)*

### `isprint`

```rust
unsafe fn isprint(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:596`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L596)*

### `ispunct`

```rust
unsafe fn ispunct(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:597`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L597)*

### `isspace`

```rust
unsafe fn isspace(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:598`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L598)*

### `isupper`

```rust
unsafe fn isupper(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:599`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L599)*

### `isxdigit`

```rust
unsafe fn isxdigit(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:600`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L600)*

### `isblank`

```rust
unsafe fn isblank(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:601`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L601)*

### `tolower`

```rust
unsafe fn tolower(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:602`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L602)*

### `toupper`

```rust
unsafe fn toupper(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:603`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L603)*

### `qsort`

```rust
unsafe fn qsort(base: *mut c_void, num: size_t, size: size_t, compar: Option<fn(*const c_void, *const c_void) -> c_int>)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:604-609`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L604-L609)*

### `bsearch`

```rust
unsafe fn bsearch(key: *const c_void, base: *const c_void, num: size_t, size: size_t, compar: Option<fn(*const c_void, *const c_void) -> c_int>) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:610-616`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L610-L616)*

### `fopen`

```rust
unsafe fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:622`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L622)*

### `freopen`

```rust
unsafe fn freopen(filename: *const c_char, mode: *const c_char, file: *mut FILE) -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:628`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L628)*

### `fflush`

```rust
unsafe fn fflush(file: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:630`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L630)*

### `fclose`

```rust
unsafe fn fclose(file: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:631`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L631)*

### `remove`

```rust
unsafe fn remove(filename: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:632`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L632)*

### `rename`

```rust
unsafe fn rename(oldname: *const c_char, newname: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:633`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L633)*

### `tmpfile`

```rust
unsafe fn tmpfile() -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:635`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L635)*

### `setvbuf`

```rust
unsafe fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int, size: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:636`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L636)*

### `setbuf`

```rust
unsafe fn setbuf(stream: *mut FILE, buf: *mut c_char)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:637`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L637)*

### `getchar`

```rust
unsafe fn getchar() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:638`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L638)*

### `putchar`

```rust
unsafe fn putchar(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:639`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L639)*

### `fgetc`

```rust
unsafe fn fgetc(stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:640`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L640)*

### `fgets`

```rust
unsafe fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:641`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L641)*

### `fputc`

```rust
unsafe fn fputc(c: c_int, stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:642`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L642)*

### `fputs`

```rust
unsafe fn fputs(s: *const c_char, stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:647`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L647)*

### `puts`

```rust
unsafe fn puts(s: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:648`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L648)*

### `ungetc`

```rust
unsafe fn ungetc(c: c_int, stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:649`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L649)*

### `fread`

```rust
unsafe fn fread(ptr: *mut c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:650`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L650)*

### `fwrite`

```rust
unsafe fn fwrite(ptr: *const c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:655`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L655)*

### `fseek`

```rust
unsafe fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:656`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L656)*

### `ftell`

```rust
unsafe fn ftell(stream: *mut FILE) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:657`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L657)*

### `rewind`

```rust
unsafe fn rewind(stream: *mut FILE)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:658`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L658)*

### `fgetpos`

```rust
unsafe fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:661`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L661)*

### `fsetpos`

```rust
unsafe fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:664`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L664)*

### `feof`

```rust
unsafe fn feof(stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:665`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L665)*

### `ferror`

```rust
unsafe fn ferror(stream: *mut FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:666`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L666)*

### `clearerr`

```rust
unsafe fn clearerr(stream: *mut FILE)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:667`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L667)*

### `perror`

```rust
unsafe fn perror(s: *const c_char)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:668`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L668)*

### `atof`

```rust
unsafe fn atof(s: *const c_char) -> c_double
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:669`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L669)*

### `atoi`

```rust
unsafe fn atoi(s: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:670`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L670)*

### `atol`

```rust
unsafe fn atol(s: *const c_char) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:671`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L671)*

### `atoll`

```rust
unsafe fn atoll(s: *const c_char) -> c_longlong
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:672`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L672)*

### `strtod`

```rust
unsafe fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:677`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L677)*

### `strtof`

```rust
unsafe fn strtof(s: *const c_char, endp: *mut *mut c_char) -> c_float
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:678`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L678)*

### `strtol`

```rust
unsafe fn strtol(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:679`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L679)*

### `strtoll`

```rust
unsafe fn strtoll(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_longlong
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:680`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L680)*

### `strtoul`

```rust
unsafe fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:681`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L681)*

### `strtoull`

```rust
unsafe fn strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulonglong
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:682`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L682)*

### `calloc`

```rust
unsafe fn calloc(nobj: size_t, size: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:684`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L684)*

### `malloc`

```rust
unsafe fn malloc(size: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:686`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L686)*

### `realloc`

```rust
unsafe fn realloc(p: *mut c_void, size: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:687`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L687)*

### `free`

```rust
unsafe fn free(p: *mut c_void)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:688`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L688)*

### `abort`

```rust
unsafe fn abort() -> never
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:689`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L689)*

### `exit`

```rust
unsafe fn exit(status: c_int) -> never
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:690`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L690)*

### `_exit`

```rust
unsafe fn _exit(status: c_int) -> never
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:691`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L691)*

### `system`

```rust
unsafe fn system(s: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:696`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L696)*

### `getenv`

```rust
unsafe fn getenv(s: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:697`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L697)*

### `strcpy`

```rust
unsafe fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:699`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L699)*

### `strncpy`

```rust
unsafe fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:700`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L700)*

### `stpcpy`

```rust
unsafe fn stpcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:701`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L701)*

### `strcat`

```rust
unsafe fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:702`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L702)*

### `strncat`

```rust
unsafe fn strncat(s: *mut c_char, ct: *const c_char, n: size_t) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:703`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L703)*

### `strcmp`

```rust
unsafe fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:704`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L704)*

### `strncmp`

```rust
unsafe fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:705`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L705)*

### `strcoll`

```rust
unsafe fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:706`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L706)*

### `strchr`

```rust
unsafe fn strchr(cs: *const c_char, c: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:707`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L707)*

### `strrchr`

```rust
unsafe fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:708`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L708)*

### `strspn`

```rust
unsafe fn strspn(cs: *const c_char, ct: *const c_char) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:709`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L709)*

### `strcspn`

```rust
unsafe fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:710`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L710)*

### `strdup`

```rust
unsafe fn strdup(cs: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:711`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L711)*

### `strndup`

```rust
unsafe fn strndup(cs: *const c_char, n: size_t) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:712`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L712)*

### `strpbrk`

```rust
unsafe fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:713`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L713)*

### `strstr`

```rust
unsafe fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:714`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L714)*

### `strcasecmp`

```rust
unsafe fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:715`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L715)*

### `strncasecmp`

```rust
unsafe fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:716`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L716)*

### `strlen`

```rust
unsafe fn strlen(cs: *const c_char) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:717`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L717)*

### `strnlen`

```rust
unsafe fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:718`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L718)*

### `strerror`

```rust
unsafe fn strerror(n: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:723`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L723)*

### `strtok`

```rust
unsafe fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:724`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L724)*

### `strtok_r`

```rust
unsafe fn strtok_r(s: *mut c_char, t: *const c_char, p: *mut *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:725`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L725)*

### `strxfrm`

```rust
unsafe fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:726`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L726)*

### `strsignal`

```rust
unsafe fn strsignal(sig: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:727`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L727)*

### `wcslen`

```rust
unsafe fn wcslen(buf: *const wchar_t) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:728`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L728)*

### `wcstombs`

```rust
unsafe fn wcstombs(dest: *mut c_char, src: *const wchar_t, n: size_t) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:729`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L729)*

### `memchr`

```rust
unsafe fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:731`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L731)*

### `wmemchr`

```rust
unsafe fn wmemchr(cx: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:732`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L732)*

### `memcmp`

```rust
unsafe fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:733`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L733)*

### `memcpy`

```rust
unsafe fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:734`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L734)*

### `memmove`

```rust
unsafe fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:735`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L735)*

### `memset`

```rust
unsafe fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:736`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L736)*

### `memccpy`

```rust
unsafe fn memccpy(dest: *mut c_void, src: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:737`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L737)*

### `getpwnam`

```rust
unsafe fn getpwnam(name: *const c_char) -> *mut passwd
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:742`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L742)*

### `getpwuid`

```rust
unsafe fn getpwuid(uid: crate::uid_t) -> *mut passwd
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:744`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L744)*

### `fprintf`

```rust
unsafe fn fprintf(stream: *mut crate::FILE, format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:746`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L746)*

### `printf`

```rust
unsafe fn printf(format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:747`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L747)*

### `snprintf`

```rust
unsafe fn snprintf(s: *mut c_char, n: size_t, format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:748`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L748)*

### `sprintf`

```rust
unsafe fn sprintf(s: *mut c_char, format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:749`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L749)*

### `fscanf`

```rust
unsafe fn fscanf(stream: *mut crate::FILE, format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:754`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L754)*

### `scanf`

```rust
unsafe fn scanf(format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:759`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L759)*

### `sscanf`

```rust
unsafe fn sscanf(s: *const c_char, format: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:764`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L764)*

### `getchar_unlocked`

```rust
unsafe fn getchar_unlocked() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:765`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L765)*

### `putchar_unlocked`

```rust
unsafe fn putchar_unlocked(c: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:766`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L766)*

### `socket`

```rust
unsafe fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:773`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L773)*

### `connect`

```rust
unsafe fn connect(socket: c_int, address: *const sockaddr, len: socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:784`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L784)*

### `listen`

```rust
unsafe fn listen(socket: c_int, backlog: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:790`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L790)*

### `accept`

```rust
unsafe fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:798`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L798)*

### `getpeername`

```rust
unsafe fn getpeername(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:806-807`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L806-L807)*

### `getsockname`

```rust
unsafe fn getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:815-816`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L815-L816)*

### `setsockopt`

```rust
unsafe fn setsockopt(socket: c_int, level: c_int, name: c_int, value: *const c_void, option_len: socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:819-825`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L819-L825)*

### `socketpair`

```rust
unsafe fn socketpair(domain: c_int, type_: c_int, protocol: c_int, socket_vector: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:834-839`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L834-L839)*

### `sendto`

```rust
unsafe fn sendto(socket: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, addrlen: socklen_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:850-857`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L850-L857)*

### `shutdown`

```rust
unsafe fn shutdown(socket: c_int, how: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:859`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L859)*

### `chmod`

```rust
unsafe fn chmod(path: *const c_char, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:865`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L865)*

### `fchmod`

```rust
unsafe fn fchmod(fd: c_int, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:870`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L870)*

### `fstat`

```rust
unsafe fn fstat(fildes: c_int, buf: *mut stat) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:886`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L886)*

### `mkdir`

```rust
unsafe fn mkdir(path: *const c_char, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:888`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L888)*

### `stat`

```rust
unsafe fn stat(path: *const c_char, buf: *mut stat) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:904`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L904)*

### `pclose`

```rust
unsafe fn pclose(stream: *mut crate::FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:906`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L906)*

### `fdopen`

```rust
unsafe fn fdopen(fd: c_int, mode: *const c_char) -> *mut crate::FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:911`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L911)*

### `fileno`

```rust
unsafe fn fileno(stream: *mut crate::FILE) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:912`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L912)*

### `open`

```rust
unsafe fn open(path: *const c_char, oflag: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:919`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L919)*

### `creat`

```rust
unsafe fn creat(path: *const c_char, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:925`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L925)*

### `fcntl`

```rust
unsafe fn fcntl(fd: c_int, cmd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:935`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L935)*

### `opendir`

```rust
unsafe fn opendir(dirname: *const c_char) -> *mut crate::DIR
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:946`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L946)*

### `readdir`

```rust
unsafe fn readdir(dirp: *mut crate::DIR) -> *mut crate::dirent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:958`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L958)*

### `closedir`

```rust
unsafe fn closedir(dirp: *mut crate::DIR) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:963`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L963)*

### `rewinddir`

```rust
unsafe fn rewinddir(dirp: *mut crate::DIR)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:972`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L972)*

### `fchmodat`

```rust
unsafe fn fchmodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:974`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L974)*

### `fchown`

```rust
unsafe fn fchown(fd: c_int, owner: crate::uid_t, group: crate::gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:975`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L975)*

### `fchownat`

```rust
unsafe fn fchownat(dirfd: c_int, pathname: *const c_char, owner: crate::uid_t, group: crate::gid_t, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:976-982`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L976-L982)*

### `fstatat`

```rust
unsafe fn fstatat(dirfd: c_int, pathname: *const c_char, buf: *mut stat, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:996`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L996)*

### `linkat`

```rust
unsafe fn linkat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:997-1003`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L997-L1003)*

### `renameat`

```rust
unsafe fn renameat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1004-1009`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1004-L1009)*

### `symlinkat`

```rust
unsafe fn symlinkat(target: *const c_char, newdirfd: c_int, linkpath: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1010`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1010)*

### `unlinkat`

```rust
unsafe fn unlinkat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1011`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1011)*

### `access`

```rust
unsafe fn access(path: *const c_char, amode: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1013`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1013)*

### `alarm`

```rust
unsafe fn alarm(seconds: c_uint) -> c_uint
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1014`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1014)*

### `chdir`

```rust
unsafe fn chdir(dir: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1015`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1015)*

### `fchdir`

```rust
unsafe fn fchdir(dirfd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1016`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1016)*

### `chown`

```rust
unsafe fn chown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1017`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1017)*

### `lchown`

```rust
unsafe fn lchown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1022`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1022)*

### `close`

```rust
unsafe fn close(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1031`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1031)*

### `dup`

```rust
unsafe fn dup(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1032`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1032)*

### `dup2`

```rust
unsafe fn dup2(src: c_int, dst: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1033`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1033)*

### `execl`

```rust
unsafe fn execl(path: *const c_char, arg0: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1035`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1035)*

### `execle`

```rust
unsafe fn execle(path: *const c_char, arg0: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1036`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1036)*

### `execlp`

```rust
unsafe fn execlp(file: *const c_char, arg0: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1037`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1037)*

### `execv`

```rust
unsafe fn execv(prog: *const c_char, argv: *const *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1040`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1040)*

### `execve`

```rust
unsafe fn execve(prog: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1041-1045`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1041-L1045)*

### `execvp`

```rust
unsafe fn execvp(c: *const c_char, argv: *const *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1046`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1046)*

### `fork`

```rust
unsafe fn fork() -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1048`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1048)*

### `fpathconf`

```rust
unsafe fn fpathconf(filedes: c_int, name: c_int) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1049`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1049)*

### `getcwd`

```rust
unsafe fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1050`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1050)*

### `getegid`

```rust
unsafe fn getegid() -> gid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1051`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1051)*

### `geteuid`

```rust
unsafe fn geteuid() -> uid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1052`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1052)*

### `getgid`

```rust
unsafe fn getgid() -> gid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1053`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1053)*

### `getgroups`

```rust
unsafe fn getgroups(ngroups_max: c_int, groups: *mut gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1054`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1054)*

### `getlogin`

```rust
unsafe fn getlogin() -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1056`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1056)*

### `getopt`

```rust
unsafe fn getopt(argc: c_int, argv: *const *mut c_char, optstr: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1061`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1061)*

### `getpgid`

```rust
unsafe fn getpgid(pid: pid_t) -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1062`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1062)*

### `getpgrp`

```rust
unsafe fn getpgrp() -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1063`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1063)*

### `getpid`

```rust
unsafe fn getpid() -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1064`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1064)*

### `getppid`

```rust
unsafe fn getppid() -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1065`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1065)*

### `getuid`

```rust
unsafe fn getuid() -> uid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1066`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1066)*

### `isatty`

```rust
unsafe fn isatty(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1067`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1067)*

### `link`

```rust
unsafe fn link(src: *const c_char, dst: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1069`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1069)*

### `lseek`

```rust
unsafe fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1071`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1071)*

### `pathconf`

```rust
unsafe fn pathconf(path: *const c_char, name: c_int) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1072`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1072)*

### `pipe`

```rust
unsafe fn pipe(fds: *mut c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1073`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1073)*

### `posix_memalign`

```rust
unsafe fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1074`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1074)*

### `aligned_alloc`

```rust
unsafe fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1075`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1075)*

### `read`

```rust
unsafe fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1080`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1080)*

### `rmdir`

```rust
unsafe fn rmdir(path: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1081`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1081)*

### `seteuid`

```rust
unsafe fn seteuid(uid: uid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1082`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1082)*

### `setegid`

```rust
unsafe fn setegid(gid: gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1083`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1083)*

### `setgid`

```rust
unsafe fn setgid(gid: gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1084`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1084)*

### `setpgid`

```rust
unsafe fn setpgid(pid: pid_t, pgid: pid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1085`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1085)*

### `setsid`

```rust
unsafe fn setsid() -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1086`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1086)*

### `setuid`

```rust
unsafe fn setuid(uid: uid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1087`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1087)*

### `setreuid`

```rust
unsafe fn setreuid(ruid: uid_t, euid: uid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1088`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1088)*

### `setregid`

```rust
unsafe fn setregid(rgid: gid_t, egid: gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1089`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1089)*

### `sleep`

```rust
unsafe fn sleep(secs: c_uint) -> c_uint
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1094`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1094)*

### `nanosleep`

```rust
unsafe fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1101`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1101)*

### `tcgetpgrp`

```rust
unsafe fn tcgetpgrp(fd: c_int) -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1102`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1102)*

### `tcsetpgrp`

```rust
unsafe fn tcsetpgrp(fd: c_int, pgrp: crate::pid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1103`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1103)*

### `ttyname`

```rust
unsafe fn ttyname(fd: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1104`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1104)*

### `ttyname_r`

```rust
unsafe fn ttyname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1113`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1113)*

### `unlink`

```rust
unsafe fn unlink(c: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1114`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1114)*

### `wait`

```rust
unsafe fn wait(status: *mut c_int) -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1119`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1119)*

### `waitpid`

```rust
unsafe fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1124`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1124)*

### `write`

```rust
unsafe fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1129`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1129)*

### `pread`

```rust
unsafe fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1135`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1135)*

### `pwrite`

```rust
unsafe fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1141`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1141)*

### `umask`

```rust
unsafe fn umask(mask: mode_t) -> mode_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1142`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1142)*

### `utime`

```rust
unsafe fn utime(file: *const c_char, buf: *const utimbuf) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1146`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1146)*

### `kill`

```rust
unsafe fn kill(pid: pid_t, sig: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1152`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1152)*

### `killpg`

```rust
unsafe fn killpg(pgrp: pid_t, sig: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1157`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1157)*

### `mlock`

```rust
unsafe fn mlock(addr: *const c_void, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1159`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1159)*

### `munlock`

```rust
unsafe fn munlock(addr: *const c_void, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1160`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1160)*

### `mlockall`

```rust
unsafe fn mlockall(flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1161`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1161)*

### `munlockall`

```rust
unsafe fn munlockall() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1162`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1162)*

### `mmap`

```rust
unsafe fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1169-1176`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1169-L1176)*

### `munmap`

```rust
unsafe fn munmap(addr: *mut c_void, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1181`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1181)*

### `if_nametoindex`

```rust
unsafe fn if_nametoindex(ifname: *const c_char) -> c_uint
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1183`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1183)*

### `if_indextoname`

```rust
unsafe fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1184`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1184)*

### `lstat`

```rust
unsafe fn lstat(path: *const c_char, buf: *mut stat) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1200`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1200)*

### `fsync`

```rust
unsafe fn fsync(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1206`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1206)*

### `setenv`

```rust
unsafe fn setenv(name: *const c_char, val: *const c_char, overwrite: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1212`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1212)*

### `unsetenv`

```rust
unsafe fn unsetenv(name: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1218`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1218)*

### `symlink`

```rust
unsafe fn symlink(path1: *const c_char, path2: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1220`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1220)*

### `truncate`

```rust
unsafe fn truncate(path: *const c_char, length: off_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1223`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1223)*

### `ftruncate`

```rust
unsafe fn ftruncate(fd: c_int, length: off_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1225`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1225)*

### `signal`

```rust
unsafe fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1227`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1227)*

### `getrusage`

```rust
unsafe fn getrusage(resource: c_int, usage: *mut rusage) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1231`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1231)*

### `realpath`

```rust
unsafe fn realpath(pathname: *const c_char, resolved: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1243`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1243)*

### `times`

```rust
unsafe fn times(buf: *mut crate::tms) -> crate::clock_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1246`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1246)*

### `pthread_self`

```rust
unsafe fn pthread_self() -> crate::pthread_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1248`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1248)*

### `pthread_equal`

```rust
unsafe fn pthread_equal(t1: crate::pthread_t, t2: crate::pthread_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1249`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1249)*

### `pthread_join`

```rust
unsafe fn pthread_join(native: crate::pthread_t, value: *mut *mut c_void) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1254`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1254)*

### `pthread_exit`

```rust
unsafe fn pthread_exit(value: *mut c_void) -> never
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1255`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1255)*

### `pthread_attr_init`

```rust
unsafe fn pthread_attr_init(attr: *mut crate::pthread_attr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1256`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1256)*

### `pthread_attr_destroy`

```rust
unsafe fn pthread_attr_destroy(attr: *mut crate::pthread_attr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1257`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1257)*

### `pthread_attr_getstacksize`

```rust
unsafe fn pthread_attr_getstacksize(attr: *const crate::pthread_attr_t, stacksize: *mut size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1258-1261`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1258-L1261)*

### `pthread_attr_setstacksize`

```rust
unsafe fn pthread_attr_setstacksize(attr: *mut crate::pthread_attr_t, stack_size: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1262-1263`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1262-L1263)*

### `pthread_attr_setdetachstate`

```rust
unsafe fn pthread_attr_setdetachstate(attr: *mut crate::pthread_attr_t, state: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1264`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1264)*

### `pthread_detach`

```rust
unsafe fn pthread_detach(thread: crate::pthread_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1265`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1265)*

### `sched_yield`

```rust
unsafe fn sched_yield() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1267`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1267)*

### `pthread_key_create`

```rust
unsafe fn pthread_key_create(key: *mut crate::pthread_key_t, dtor: Option<fn(*mut c_void)>) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1268-1271`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1268-L1271)*

### `pthread_key_delete`

```rust
unsafe fn pthread_key_delete(key: crate::pthread_key_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1272`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1272)*

### `pthread_getspecific`

```rust
unsafe fn pthread_getspecific(key: crate::pthread_key_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1273`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1273)*

### `pthread_setspecific`

```rust
unsafe fn pthread_setspecific(key: crate::pthread_key_t, value: *const c_void) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1274`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1274)*

### `pthread_mutex_init`

```rust
unsafe fn pthread_mutex_init(lock: *mut crate::pthread_mutex_t, attr: *const crate::pthread_mutexattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1275-1278`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1275-L1278)*

### `pthread_mutex_destroy`

```rust
unsafe fn pthread_mutex_destroy(lock: *mut crate::pthread_mutex_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1279`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1279)*

### `pthread_mutex_lock`

```rust
unsafe fn pthread_mutex_lock(lock: *mut crate::pthread_mutex_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1280`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1280)*

### `pthread_mutex_trylock`

```rust
unsafe fn pthread_mutex_trylock(lock: *mut crate::pthread_mutex_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1281`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1281)*

### `pthread_mutex_unlock`

```rust
unsafe fn pthread_mutex_unlock(lock: *mut crate::pthread_mutex_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1282`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1282)*

### `pthread_mutexattr_init`

```rust
unsafe fn pthread_mutexattr_init(attr: *mut crate::pthread_mutexattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1284`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1284)*

### `pthread_mutexattr_destroy`

```rust
unsafe fn pthread_mutexattr_destroy(attr: *mut crate::pthread_mutexattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1289`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1289)*

### `pthread_mutexattr_settype`

```rust
unsafe fn pthread_mutexattr_settype(attr: *mut crate::pthread_mutexattr_t, _type: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1290`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1290)*

### `pthread_cond_init`

```rust
unsafe fn pthread_cond_init(cond: *mut crate::pthread_cond_t, attr: *const crate::pthread_condattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1296-1299`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1296-L1299)*

### `pthread_cond_wait`

```rust
unsafe fn pthread_cond_wait(cond: *mut crate::pthread_cond_t, lock: *mut crate::pthread_mutex_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1304-1307`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1304-L1307)*

### `pthread_cond_timedwait`

```rust
unsafe fn pthread_cond_timedwait(cond: *mut crate::pthread_cond_t, lock: *mut crate::pthread_mutex_t, abstime: *const crate::timespec) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1313-1317`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1313-L1317)*

### `pthread_cond_signal`

```rust
unsafe fn pthread_cond_signal(cond: *mut crate::pthread_cond_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1318`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1318)*

### `pthread_cond_broadcast`

```rust
unsafe fn pthread_cond_broadcast(cond: *mut crate::pthread_cond_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1319`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1319)*

### `pthread_cond_destroy`

```rust
unsafe fn pthread_cond_destroy(cond: *mut crate::pthread_cond_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1320`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1320)*

### `pthread_condattr_init`

```rust
unsafe fn pthread_condattr_init(attr: *mut crate::pthread_condattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1321`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1321)*

### `pthread_condattr_destroy`

```rust
unsafe fn pthread_condattr_destroy(attr: *mut crate::pthread_condattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1322`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1322)*

### `pthread_rwlock_init`

```rust
unsafe fn pthread_rwlock_init(lock: *mut crate::pthread_rwlock_t, attr: *const crate::pthread_rwlockattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1327-1330`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1327-L1330)*

### `pthread_rwlock_destroy`

```rust
unsafe fn pthread_rwlock_destroy(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1335`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1335)*

### `pthread_rwlock_rdlock`

```rust
unsafe fn pthread_rwlock_rdlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1340`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1340)*

### `pthread_rwlock_tryrdlock`

```rust
unsafe fn pthread_rwlock_tryrdlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1345`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1345)*

### `pthread_rwlock_wrlock`

```rust
unsafe fn pthread_rwlock_wrlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1350`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1350)*

### `pthread_rwlock_trywrlock`

```rust
unsafe fn pthread_rwlock_trywrlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1355`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1355)*

### `pthread_rwlock_unlock`

```rust
unsafe fn pthread_rwlock_unlock(lock: *mut crate::pthread_rwlock_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1360`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1360)*

### `pthread_rwlockattr_init`

```rust
unsafe fn pthread_rwlockattr_init(attr: *mut crate::pthread_rwlockattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1361`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1361)*

### `pthread_rwlockattr_destroy`

```rust
unsafe fn pthread_rwlockattr_destroy(attr: *mut crate::pthread_rwlockattr_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1362`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1362)*

### `getsockopt`

```rust
unsafe fn getsockopt(sockfd: c_int, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut crate::socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1370-1376`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1370-L1376)*

### `raise`

```rust
unsafe fn raise(signum: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1377`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1377)*

### `utimes`

```rust
unsafe fn utimes(filename: *const c_char, times: *const crate::timeval) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1381`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1381)*

### `dlopen`

```rust
unsafe fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1382`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1382)*

### `dlerror`

```rust
unsafe fn dlerror() -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1383`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1383)*

### `dlsym`

```rust
unsafe fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1384`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1384)*

### `dlclose`

```rust
unsafe fn dlclose(handle: *mut c_void) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1385`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1385)*

### `getaddrinfo`

```rust
unsafe fn getaddrinfo(node: *const c_char, service: *const c_char, hints: *const addrinfo, res: *mut *mut addrinfo) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1393-1398`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1393-L1398)*

### `freeaddrinfo`

```rust
unsafe fn freeaddrinfo(res: *mut addrinfo)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1401`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1401)*

### `hstrerror`

```rust
unsafe fn hstrerror(errcode: c_int) -> *const c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1402`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1402)*

### `gai_strerror`

```rust
unsafe fn gai_strerror(errcode: c_int) -> *const c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1403`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1403)*

### `res_init`

```rust
unsafe fn res_init() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1428`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1428)*

### `gmtime_r`

```rust
unsafe fn gmtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1434`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1434)*

### `localtime_r`

```rust
unsafe fn localtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1439`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1439)*

### `mktime`

```rust
unsafe fn mktime(tm: *mut tm) -> time_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1448`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1448)*

### `time`

```rust
unsafe fn time(time: *mut time_t) -> time_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1453`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1453)*

### `gmtime`

```rust
unsafe fn gmtime(time_p: *const time_t) -> *mut tm
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1458`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1458)*

### `localtime`

```rust
unsafe fn localtime(time_p: *const time_t) -> *mut tm
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1463`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1463)*

### `difftime`

```rust
unsafe fn difftime(time1: time_t, time0: time_t) -> c_double
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1468`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1468)*

### `timegm`

```rust
unsafe fn timegm(tm: *mut crate::tm) -> time_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1474`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1474)*

### `mknod`

```rust
unsafe fn mknod(pathname: *const c_char, mode: mode_t, dev: crate::dev_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1481`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1481)*

### `gethostname`

```rust
unsafe fn gethostname(name: *mut c_char, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1483`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1483)*

### `endservent`

```rust
unsafe fn endservent()
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1484`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1484)*

### `getservbyname`

```rust
unsafe fn getservbyname(name: *const c_char, proto: *const c_char) -> *mut servent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1485`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1485)*

### `getservbyport`

```rust
unsafe fn getservbyport(port: c_int, proto: *const c_char) -> *mut servent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1486`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1486)*

### `getservent`

```rust
unsafe fn getservent() -> *mut servent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1487`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1487)*

### `setservent`

```rust
unsafe fn setservent(stayopen: c_int)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1488`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1488)*

### `getprotobyname`

```rust
unsafe fn getprotobyname(name: *const c_char) -> *mut protoent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1489`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1489)*

### `getprotobynumber`

```rust
unsafe fn getprotobynumber(proto: c_int) -> *mut protoent
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1490`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1490)*

### `chroot`

```rust
unsafe fn chroot(name: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1491`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1491)*

### `usleep`

```rust
unsafe fn usleep(secs: c_uint) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1499`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1499)*

### `send`

```rust
unsafe fn send(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1505`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1505)*

### `recv`

```rust
unsafe fn recv(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1511`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1511)*

### `putenv`

```rust
unsafe fn putenv(string: *mut c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1517`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1517)*

### `poll`

```rust
unsafe fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1522`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1522)*

### `select`

```rust
unsafe fn select(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *mut timeval) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1534-1540`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1534-L1540)*

### `setlocale`

```rust
unsafe fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1542`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1542)*

### `localeconv`

```rust
unsafe fn localeconv() -> *mut lconv
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1543`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1543)*

### `sem_wait`

```rust
unsafe fn sem_wait(sem: *mut sem_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1549`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1549)*

### `sem_trywait`

```rust
unsafe fn sem_trywait(sem: *mut sem_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1550`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1550)*

### `sem_post`

```rust
unsafe fn sem_post(sem: *mut sem_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1551`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1551)*

### `statvfs`

```rust
unsafe fn statvfs(path: *const c_char, buf: *mut crate::statvfs) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1553`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1553)*

### `fstatvfs`

```rust
unsafe fn fstatvfs(fd: c_int, buf: *mut crate::statvfs) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1555`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1555)*

### `sigemptyset`

```rust
unsafe fn sigemptyset(set: *mut sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1558`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1558)*

### `sigaddset`

```rust
unsafe fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1560`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1560)*

### `sigfillset`

```rust
unsafe fn sigfillset(set: *mut sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1562`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1562)*

### `sigdelset`

```rust
unsafe fn sigdelset(set: *mut sigset_t, signum: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1564`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1564)*

### `sigismember`

```rust
unsafe fn sigismember(set: *const sigset_t, signum: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1566`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1566)*

### `sigprocmask`

```rust
unsafe fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1569`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1569)*

### `sigpending`

```rust
unsafe fn sigpending(set: *mut sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1571`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1571)*

### `sysconf`

```rust
unsafe fn sysconf(name: c_int) -> c_long
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1574`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1574)*

### `mkfifo`

```rust
unsafe fn mkfifo(path: *const c_char, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1576`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1576)*

### `fseeko`

```rust
unsafe fn fseeko(stream: *mut crate::FILE, offset: off_t, whence: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1579`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1579)*

### `ftello`

```rust
unsafe fn ftello(stream: *mut crate::FILE) -> off_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1581`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1581)*

### `tcdrain`

```rust
unsafe fn tcdrain(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1586`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1586)*

### `cfgetispeed`

```rust
unsafe fn cfgetispeed(termios: *const crate::termios) -> crate::speed_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1587`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1587)*

### `cfgetospeed`

```rust
unsafe fn cfgetospeed(termios: *const crate::termios) -> crate::speed_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1588`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1588)*

### `cfsetispeed`

```rust
unsafe fn cfsetispeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1589`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1589)*

### `cfsetospeed`

```rust
unsafe fn cfsetospeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1590`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1590)*

### `tcgetattr`

```rust
unsafe fn tcgetattr(fd: c_int, termios: *mut crate::termios) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1591`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1591)*

### `tcsetattr`

```rust
unsafe fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const crate::termios) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1592`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1592)*

### `tcflow`

```rust
unsafe fn tcflow(fd: c_int, action: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1593`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1593)*

### `tcflush`

```rust
unsafe fn tcflush(fd: c_int, action: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1594`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1594)*

### `tcgetsid`

```rust
unsafe fn tcgetsid(fd: c_int) -> crate::pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1595`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1595)*

### `tcsendbreak`

```rust
unsafe fn tcsendbreak(fd: c_int, duration: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1596`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1596)*

### `mkstemp`

```rust
unsafe fn mkstemp(template: *mut c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1598`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1598)*

### `mkdtemp`

```rust
unsafe fn mkdtemp(template: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1599`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1599)*

### `tmpnam`

```rust
unsafe fn tmpnam(ptr: *mut c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1601`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1601)*

### `openlog`

```rust
unsafe fn openlog(ident: *const c_char, logopt: c_int, facility: c_int)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1603`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1603)*

### `closelog`

```rust
unsafe fn closelog()
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1604`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1604)*

### `setlogmask`

```rust
unsafe fn setlogmask(maskpri: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1605`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1605)*

### `syslog`

```rust
unsafe fn syslog(priority: c_int, message: *const c_char)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1607`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1607)*

### `nice`

```rust
unsafe fn nice(incr: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1612`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1612)*

### `grantpt`

```rust
unsafe fn grantpt(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1614`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1614)*

### `posix_openpt`

```rust
unsafe fn posix_openpt(flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1615`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1615)*

### `ptsname`

```rust
unsafe fn ptsname(fd: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1616`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1616)*

### `unlockpt`

```rust
unsafe fn unlockpt(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1617`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1617)*

### `strcasestr`

```rust
unsafe fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1620`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1620)*

### `getline`

```rust
unsafe fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1621`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1621)*

### `lockf`

```rust
unsafe fn lockf(fd: c_int, cmd: c_int, len: off_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1624`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1624)*

### `adjtime`

```rust
unsafe fn adjtime(delta: *const timeval, olddelta: *mut timeval) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1658`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1658)*

### `stpncpy`

```rust
unsafe fn stpncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1674`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1674)*

### `sigqueue`

```rust
unsafe fn sigqueue(pid: pid_t, sig: c_int, value: crate::sigval) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1688`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1688)*

### `confstr`

```rust
unsafe fn confstr(name: c_int, buf: *mut c_char, len: size_t) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1701`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1701)*

### `dladdr`

```rust
unsafe fn dladdr(addr: *const c_void, info: *mut Dl_info) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1709`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1709)*

### `flock`

```rust
unsafe fn flock(fd: c_int, operation: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1717`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1717)*

### `open_wmemstream`

```rust
unsafe fn open_wmemstream(ptr: *mut *mut wchar_t, sizeloc: *mut size_t) -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1725`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1725)*

### `getsid`

```rust
unsafe fn getsid(pid: pid_t) -> pid_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1733`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1733)*

### `pause`

```rust
unsafe fn pause() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1738`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1738)*

### `mkdirat`

```rust
unsafe fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1740`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1740)*

### `openat`

```rust
unsafe fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1742`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1742)*

### `fdopendir`

```rust
unsafe fn fdopendir(fd: c_int) -> *mut crate::DIR
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1752`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1752)*

### `readdir_r`

```rust
unsafe fn readdir_r(dirp: *mut crate::DIR, entry: *mut crate::dirent, result: *mut *mut crate::dirent) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1775-1779`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1775-L1779)*

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

*Defined in [`libc-0.2.178/src/unix/mod.rs:1807-1812`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1807-L1812)*

### `fmemopen`

```rust
unsafe fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1813`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1813)*

### `open_memstream`

```rust
unsafe fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1814`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1814)*

### `atexit`

```rust
unsafe fn atexit(cb: fn()) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1815`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1815)*

### `sigaction`

```rust
unsafe fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1817-1818`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1817-L1818)*

### `readlink`

```rust
unsafe fn readlink(path: *const c_char, buf: *mut c_char, bufsz: size_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1819`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1819)*

### `pselect`

```rust
unsafe fn pselect(nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set, errorfds: *mut fd_set, timeout: *const timespec, sigmask: *const sigset_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1830-1837`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1830-L1837)*

### `cfmakeraw`

```rust
unsafe fn cfmakeraw(termios: *mut crate::termios)
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1849`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1849)*

### `cfsetspeed`

```rust
unsafe fn cfsetspeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1868`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1868)*

### `fnmatch`

```rust
unsafe fn fnmatch(pattern: *const c_char, name: *const c_char, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1874`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1874)*

### `htonl`

```rust
const fn htonl(hostlong: u32) -> u32
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1628-1643`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1628-L1643)*

### `htons`

```rust
const fn htons(hostshort: u16) -> u16
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1628-1643`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1628-L1643)*

### `ntohl`

```rust
const fn ntohl(netlong: u32) -> u32
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1628-1643`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1628-L1643)*

### `ntohs`

```rust
const fn ntohs(netshort: u16) -> u16
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:1628-1643`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L1628-L1643)*

### `ioctl`

```rust
unsafe fn ioctl(fd: c_int, request: crate::c_ulong) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1693`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1693)*

### `sem_destroy`

```rust
unsafe fn sem_destroy(sem: *mut sem_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1831`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1831)*

### `sem_init`

```rust
unsafe fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1832`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1832)*

### `fdatasync`

```rust
unsafe fn fdatasync(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1833`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1833)*

### `mincore`

```rust
unsafe fn mincore(addr: *mut c_void, len: size_t, vec: *mut c_uchar) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1834`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1834)*

### `clock_getres`

```rust
unsafe fn clock_getres(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1837`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1837)*

### `clock_gettime`

```rust
unsafe fn clock_gettime(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1839`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1839)*

### `clock_settime`

```rust
unsafe fn clock_settime(clk_id: crate::clockid_t, tp: *const crate::timespec) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1841`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1841)*

### `clock_getcpuclockid`

```rust
unsafe fn clock_getcpuclockid(pid: crate::pid_t, clk_id: *mut crate::clockid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1842`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1842)*

### `dirfd`

```rust
unsafe fn dirfd(dirp: *mut crate::DIR) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1844`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1844)*

### `memalign`

```rust
unsafe fn memalign(align: size_t, size: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1846`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1846)*

### `setgroups`

```rust
unsafe fn setgroups(ngroups: size_t, ptr: *const crate::gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1847`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1847)*

### `pipe2`

```rust
unsafe fn pipe2(fds: *mut c_int, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1848`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1848)*

### `statfs`

```rust
unsafe fn statfs(path: *const c_char, buf: *mut statfs) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1850`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1850)*

### `fstatfs`

```rust
unsafe fn fstatfs(fd: c_int, buf: *mut statfs) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1852`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1852)*

### `memrchr`

```rust
unsafe fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1853`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1853)*

### `posix_fadvise`

```rust
unsafe fn posix_fadvise(fd: c_int, offset: off_t, len: off_t, advise: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1855`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1855)*

### `futimens`

```rust
unsafe fn futimens(fd: c_int, times: *const crate::timespec) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1857`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1857)*

### `utimensat`

```rust
unsafe fn utimensat(dirfd: c_int, path: *const c_char, times: *const crate::timespec, flag: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1859-1864`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1859-L1864)*

### `duplocale`

```rust
unsafe fn duplocale(base: crate::locale_t) -> crate::locale_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1865`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1865)*

### `freelocale`

```rust
unsafe fn freelocale(loc: crate::locale_t)
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1866`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1866)*

### `newlocale`

```rust
unsafe fn newlocale(mask: c_int, locale: *const c_char, base: crate::locale_t) -> crate::locale_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1867`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1867)*

### `uselocale`

```rust
unsafe fn uselocale(loc: crate::locale_t) -> crate::locale_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1868`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1868)*

### `mknodat`

```rust
unsafe fn mknodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, dev: dev_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1869`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1869)*

### `ptsname_r`

```rust
unsafe fn ptsname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1871`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1871)*

### `clearenv`

```rust
unsafe fn clearenv() -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1872`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1872)*

### `waitid`

```rust
unsafe fn waitid(idtype: idtype_t, id: id_t, infop: *mut crate::siginfo_t, options: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1873-1878`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1873-L1878)*

### `getresuid`

```rust
unsafe fn getresuid(ruid: *mut crate::uid_t, euid: *mut crate::uid_t, suid: *mut crate::uid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1879-1883`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1879-L1883)*

### `getresgid`

```rust
unsafe fn getresgid(rgid: *mut crate::gid_t, egid: *mut crate::gid_t, sgid: *mut crate::gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1884-1888`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1884-L1888)*

### `acct`

```rust
unsafe fn acct(filename: *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1889`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1889)*

### `brk`

```rust
unsafe fn brk(addr: *mut c_void) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1890`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1890)*

### `sbrk`

```rust
unsafe fn sbrk(increment: intptr_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1891`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1891)*

### `vfork`

```rust
unsafe fn vfork() -> crate::pid_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1896`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1896)*

### `setresgid`

```rust
unsafe fn setresgid(rgid: crate::gid_t, egid: crate::gid_t, sgid: crate::gid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1897`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1897)*

### `setresuid`

```rust
unsafe fn setresuid(ruid: crate::uid_t, euid: crate::uid_t, suid: crate::uid_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1898`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1898)*

### `wait4`

```rust
unsafe fn wait4(pid: crate::pid_t, status: *mut c_int, options: c_int, rusage: *mut crate::rusage) -> crate::pid_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1900-1905`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1900-L1905)*

### `login_tty`

```rust
unsafe fn login_tty(fd: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1906`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1906)*

### `execvpe`

```rust
unsafe fn execvpe(file: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1909-1913`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1909-L1913)*

### `fexecve`

```rust
unsafe fn fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1914`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1914)*

### `getifaddrs`

```rust
unsafe fn getifaddrs(ifap: *mut *mut crate::ifaddrs) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1916`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1916)*

### `freeifaddrs`

```rust
unsafe fn freeifaddrs(ifa: *mut crate::ifaddrs)
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1917`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1917)*

### `bind`

```rust
unsafe fn bind(socket: c_int, address: *const crate::sockaddr, address_len: crate::socklen_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1918-1922`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1918-L1922)*

### `writev`

```rust
unsafe fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1924`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1924)*

### `readv`

```rust
unsafe fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1925`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1925)*

### `sendmsg`

```rust
unsafe fn sendmsg(fd: c_int, msg: *const crate::msghdr, flags: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1928`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1928)*

### `recvmsg`

```rust
unsafe fn recvmsg(fd: c_int, msg: *mut crate::msghdr, flags: c_int) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1930`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1930)*

### `uname`

```rust
unsafe fn uname(buf: *mut crate::utsname) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1931`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1931)*

### `strchrnul`

```rust
unsafe fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1933`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1933)*

### `strftime`

```rust
unsafe fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const crate::tm) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1935-1940`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1935-L1940)*

### `strftime_l`

```rust
unsafe fn strftime_l(s: *mut c_char, max: size_t, format: *const c_char, tm: *const crate::tm, locale: crate::locale_t) -> size_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1941-1947`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1941-L1947)*

### `strptime`

```rust
unsafe fn strptime(s: *const c_char, format: *const c_char, tm: *mut crate::tm) -> *mut c_char
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1948`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1948)*

### `mkostemp`

```rust
unsafe fn mkostemp(template: *mut c_char, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1951`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1951)*

### `mkostemps`

```rust
unsafe fn mkostemps(template: *mut c_char, suffixlen: c_int, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1953`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1953)*

### `getdomainname`

```rust
unsafe fn getdomainname(name: *mut c_char, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1955`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1955)*

### `setdomainname`

```rust
unsafe fn setdomainname(name: *const c_char, len: size_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1956`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1956)*

### `fstatfs64`

```rust
unsafe fn fstatfs64(fd: c_int, buf: *mut statfs64) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1970`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1970)*

### `statvfs64`

```rust
unsafe fn statvfs64(path: *const c_char, buf: *mut statvfs64) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1971`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1971)*

### `fstatvfs64`

```rust
unsafe fn fstatvfs64(fd: c_int, buf: *mut statvfs64) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1972`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1972)*

### `statfs64`

```rust
unsafe fn statfs64(path: *const c_char, buf: *mut statfs64) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1973`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1973)*

### `creat64`

```rust
unsafe fn creat64(path: *const c_char, mode: mode_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1974`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1974)*

### `fstat64`

```rust
unsafe fn fstat64(fildes: c_int, buf: *mut stat64) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1976`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1976)*

### `fstatat64`

```rust
unsafe fn fstatat64(dirfd: c_int, pathname: *const c_char, buf: *mut stat64, flags: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1978-1983`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1978-L1983)*

### `ftruncate64`

```rust
unsafe fn ftruncate64(fd: c_int, length: off64_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1984`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1984)*

### `lseek64`

```rust
unsafe fn lseek64(fd: c_int, offset: off64_t, whence: c_int) -> off64_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1985`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1985)*

### `lstat64`

```rust
unsafe fn lstat64(path: *const c_char, buf: *mut stat64) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1987`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1987)*

### `mmap64`

```rust
unsafe fn mmap64(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off64_t) -> *mut c_void
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1988-1995`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1988-L1995)*

### `open64`

```rust
unsafe fn open64(path: *const c_char, oflag: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1996`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1996)*

### `openat64`

```rust
unsafe fn openat64(fd: c_int, path: *const c_char, oflag: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1997`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1997)*

### `posix_fadvise64`

```rust
unsafe fn posix_fadvise64(fd: c_int, offset: off64_t, len: off64_t, advise: c_int) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1998-2003`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1998-L2003)*

### `pread64`

```rust
unsafe fn pread64(fd: c_int, buf: *mut c_void, count: size_t, offset: off64_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:2004`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L2004)*

### `pwrite64`

```rust
unsafe fn pwrite64(fd: c_int, buf: *const c_void, count: size_t, offset: off64_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:2005-2010`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L2005-L2010)*

### `readdir64`

```rust
unsafe fn readdir64(dirp: *mut crate::DIR) -> *mut crate::dirent64
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:2011`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L2011)*

### `readdir64_r`

```rust
unsafe fn readdir64_r(dirp: *mut crate::DIR, entry: *mut crate::dirent64, result: *mut *mut crate::dirent64) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:2012-2016`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L2012-L2016)*

### `stat64`

```rust
unsafe fn stat64(path: *const c_char, buf: *mut stat64) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:2018`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L2018)*

### `truncate64`

```rust
unsafe fn truncate64(path: *const c_char, length: off64_t) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:2019`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L2019)*

### `preadv64`

```rust
unsafe fn preadv64(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:2032-2037`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L2032-L2037)*

### `pwritev64`

```rust
unsafe fn pwritev64(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off64_t) -> ssize_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:2038-2043`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L2038-L2043)*

### `forkpty`

```rust
unsafe fn forkpty(amaster: *mut c_int, name: *mut c_char, termp: *const termios, winp: *const crate::winsize) -> crate::pid_t
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:2052-2057`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L2052-L2057)*

### `openpty`

```rust
unsafe fn openpty(amaster: *mut c_int, aslave: *mut c_int, name: *mut c_char, termp: *const termios, winp: *const crate::winsize) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:2059-2065`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L2059-L2065)*

### `statx`

```rust
unsafe fn statx(dirfd: c_int, pathname: *const c_char, flags: c_int, mask: c_uint, statxbuf: *mut statx) -> c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:2078-2084`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L2078-L2084)*

### `_IOC`

```rust
const fn _IOC(dir: u32, ty: u32, nr: u32, size: usize) -> crate::c_ulong
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1659-1669`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1659-L1669)*

Build an ioctl number, analogous to the C macro of the same name.

### `_IO`

```rust
const fn _IO(ty: u32, nr: u32) -> crate::c_ulong
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1672-1674`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1672-L1674)*

Build an ioctl number for an argumentless ioctl.

### `_IOR`

```rust
const fn _IOR<T>(ty: u32, nr: u32) -> crate::c_ulong
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1677-1679`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1677-L1679)*

Build an ioctl number for an read-only ioctl.

### `_IOW`

```rust
const fn _IOW<T>(ty: u32, nr: u32) -> crate::c_ulong
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1682-1684`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1682-L1684)*

Build an ioctl number for an write-only ioctl.

### `_IOWR`

```rust
const fn _IOWR<T>(ty: u32, nr: u32) -> crate::c_ulong
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1687-1689`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1687-L1689)*

Build an ioctl number for a read-write ioctl.

### `CMSG_ALIGN`

```rust
const fn CMSG_ALIGN(len: usize) -> usize
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1698-1700`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1698-L1700)*

### `CMSG_FIRSTHDR`

```rust
unsafe fn CMSG_FIRSTHDR(mhdr: *const crate::msghdr) -> *mut crate::cmsghdr
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1702-1748`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1702-L1748)*

### `CMSG_DATA`

```rust
unsafe fn CMSG_DATA(cmsg: *const crate::cmsghdr) -> *mut crate::c_uchar
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1702-1748`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1702-L1748)*

### `CMSG_SPACE`

```rust
const unsafe fn CMSG_SPACE(length: crate::c_uint) -> crate::c_uint
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1702-1748`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1702-L1748)*

### `CMSG_LEN`

```rust
const unsafe fn CMSG_LEN(length: crate::c_uint) -> crate::c_uint
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1702-1748`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1702-L1748)*

### `FD_CLR`

```rust
unsafe fn FD_CLR(fd: crate::c_int, set: *mut fd_set)
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1702-1748`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1702-L1748)*

### `FD_ISSET`

```rust
unsafe fn FD_ISSET(fd: crate::c_int, set: *const fd_set) -> bool
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1702-1748`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1702-L1748)*

### `FD_SET`

```rust
unsafe fn FD_SET(fd: crate::c_int, set: *mut fd_set)
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1702-1748`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1702-L1748)*

### `FD_ZERO`

```rust
unsafe fn FD_ZERO(set: *mut fd_set)
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1702-1748`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1702-L1748)*

### `SIGRTMAX`

```rust
fn SIGRTMAX() -> crate::c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `SIGRTMIN`

```rust
fn SIGRTMIN() -> crate::c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `WIFSTOPPED`

```rust
const fn WIFSTOPPED(status: crate::c_int) -> bool
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `WSTOPSIG`

```rust
const fn WSTOPSIG(status: crate::c_int) -> crate::c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `WIFCONTINUED`

```rust
const fn WIFCONTINUED(status: crate::c_int) -> bool
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `WIFSIGNALED`

```rust
const fn WIFSIGNALED(status: crate::c_int) -> bool
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `WTERMSIG`

```rust
const fn WTERMSIG(status: crate::c_int) -> crate::c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `WIFEXITED`

```rust
const fn WIFEXITED(status: crate::c_int) -> bool
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `WEXITSTATUS`

```rust
const fn WEXITSTATUS(status: crate::c_int) -> crate::c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `WCOREDUMP`

```rust
const fn WCOREDUMP(status: crate::c_int) -> bool
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `W_EXITCODE`

```rust
const fn W_EXITCODE(ret: crate::c_int, sig: crate::c_int) -> crate::c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `W_STOPCODE`

```rust
const fn W_STOPCODE(sig: crate::c_int) -> crate::c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `QCMD`

```rust
const fn QCMD(cmd: crate::c_int, type_: crate::c_int) -> crate::c_int
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `IPOPT_COPIED`

```rust
const fn IPOPT_COPIED(o: u8) -> u8
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `IPOPT_CLASS`

```rust
const fn IPOPT_CLASS(o: u8) -> u8
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `IPOPT_NUMBER`

```rust
const fn IPOPT_NUMBER(o: u8) -> u8
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `IPTOS_ECN`

```rust
const fn IPTOS_ECN(x: u8) -> u8
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

### `KERNEL_VERSION`

```rust
const fn KERNEL_VERSION(a: u32, b: u32, c: u32) -> u32
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1750-1823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1750-L1823)*

## Type Aliases

### `intmax_t`

```rust
type intmax_t = i64;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:8`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L8)*

### `uintmax_t`

```rust
type uintmax_t = u64;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:9`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L9)*

### `size_t`

```rust
type size_t = usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:11`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L11)*

### `ptrdiff_t`

```rust
type ptrdiff_t = isize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:12`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L12)*

### `intptr_t`

```rust
type intptr_t = isize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:13`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L13)*

### `uintptr_t`

```rust
type uintptr_t = usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:14`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L14)*

### `ssize_t`

```rust
type ssize_t = isize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:15`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L15)*

### `pid_t`

```rust
type pid_t = i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:17`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L17)*

### `in_addr_t`

```rust
type in_addr_t = u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:18`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L18)*

### `in_port_t`

```rust
type in_port_t = u16;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:19`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L19)*

### `sighandler_t`

```rust
type sighandler_t = size_t;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:20`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L20)*

### `cc_t`

```rust
type cc_t = crate::c_uchar;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:21`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L21)*

### `uid_t`

```rust
type uid_t = u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:35`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L35)*

### `gid_t`

```rust
type gid_t = u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:36`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L36)*

### `locale_t`

```rust
type locale_t = *mut crate::c_void;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:45`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L45)*

### `sa_family_t`

```rust
type sa_family_t = u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:3`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L3)*

### `speed_t`

```rust
type speed_t = crate::c_uint;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:4`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L4)*

### `tcflag_t`

```rust
type tcflag_t = crate::c_uint;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:5`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L5)*

### `clockid_t`

```rust
type clockid_t = crate::c_int;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:6`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L6)*

### `timer_t`

```rust
type timer_t = *mut crate::c_void;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:7`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L7)*

### `key_t`

```rust
type key_t = crate::c_int;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:8`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L8)*

### `id_t`

```rust
type id_t = crate::c_uint;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:9`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L9)*

## Constants

### `INT_MIN`
```rust
const INT_MIN: crate::c_int = -2_147_483_648i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:222`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L222)*

### `INT_MAX`
```rust
const INT_MAX: crate::c_int = 2_147_483_647i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:223`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L223)*

### `SIG_DFL`
```rust
const SIG_DFL: sighandler_t = 0usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:225`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L225)*

### `SIG_IGN`
```rust
const SIG_IGN: sighandler_t = 1usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:226`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L226)*

### `SIG_ERR`
```rust
const SIG_ERR: sighandler_t = 18_446_744_073_709_551_615usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:227`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L227)*

### `DT_UNKNOWN`
```rust
const DT_UNKNOWN: u8 = 0u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:231`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L231)*

### `DT_FIFO`
```rust
const DT_FIFO: u8 = 1u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:232`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L232)*

### `DT_CHR`
```rust
const DT_CHR: u8 = 2u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:233`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L233)*

### `DT_DIR`
```rust
const DT_DIR: u8 = 4u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:234`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L234)*

### `DT_BLK`
```rust
const DT_BLK: u8 = 6u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:235`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L235)*

### `DT_REG`
```rust
const DT_REG: u8 = 8u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:236`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L236)*

### `DT_LNK`
```rust
const DT_LNK: u8 = 10u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:237`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L237)*

### `DT_SOCK`
```rust
const DT_SOCK: u8 = 12u8;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:238`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L238)*

### `FD_CLOEXEC`
```rust
const FD_CLOEXEC: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:243`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L243)*

### `USRQUOTA`
```rust
const USRQUOTA: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:249`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L249)*

### `GRPQUOTA`
```rust
const GRPQUOTA: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:250`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L250)*

### `SIGIOT`
```rust
const SIGIOT: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:253`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L253)*

### `S_ISUID`
```rust
const S_ISUID: mode_t = 2_048u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:255`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L255)*

### `S_ISGID`
```rust
const S_ISGID: mode_t = 1_024u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:256`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L256)*

### `S_ISVTX`
```rust
const S_ISVTX: mode_t = 512u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:257`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L257)*

### `IF_NAMESIZE`
```rust
const IF_NAMESIZE: size_t = 16usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:266`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L266)*

### `IFNAMSIZ`
```rust
const IFNAMSIZ: size_t = 16usize;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:267`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L267)*

### `LOG_EMERG`
```rust
const LOG_EMERG: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:271`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L271)*

### `LOG_ALERT`
```rust
const LOG_ALERT: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:272`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L272)*

### `LOG_CRIT`
```rust
const LOG_CRIT: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:273`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L273)*

### `LOG_ERR`
```rust
const LOG_ERR: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:274`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L274)*

### `LOG_WARNING`
```rust
const LOG_WARNING: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:275`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L275)*

### `LOG_NOTICE`
```rust
const LOG_NOTICE: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:276`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L276)*

### `LOG_INFO`
```rust
const LOG_INFO: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:277`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L277)*

### `LOG_DEBUG`
```rust
const LOG_DEBUG: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:278`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L278)*

### `LOG_KERN`
```rust
const LOG_KERN: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:280`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L280)*

### `LOG_USER`
```rust
const LOG_USER: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:281`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L281)*

### `LOG_MAIL`
```rust
const LOG_MAIL: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:282`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L282)*

### `LOG_DAEMON`
```rust
const LOG_DAEMON: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:283`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L283)*

### `LOG_AUTH`
```rust
const LOG_AUTH: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:284`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L284)*

### `LOG_SYSLOG`
```rust
const LOG_SYSLOG: crate::c_int = 40i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:285`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L285)*

### `LOG_LPR`
```rust
const LOG_LPR: crate::c_int = 48i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:286`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L286)*

### `LOG_NEWS`
```rust
const LOG_NEWS: crate::c_int = 56i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:287`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L287)*

### `LOG_UUCP`
```rust
const LOG_UUCP: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:288`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L288)*

### `LOG_LOCAL0`
```rust
const LOG_LOCAL0: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:289`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L289)*

### `LOG_LOCAL1`
```rust
const LOG_LOCAL1: crate::c_int = 136i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:290`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L290)*

### `LOG_LOCAL2`
```rust
const LOG_LOCAL2: crate::c_int = 144i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:291`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L291)*

### `LOG_LOCAL3`
```rust
const LOG_LOCAL3: crate::c_int = 152i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:292`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L292)*

### `LOG_LOCAL4`
```rust
const LOG_LOCAL4: crate::c_int = 160i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:293`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L293)*

### `LOG_LOCAL5`
```rust
const LOG_LOCAL5: crate::c_int = 168i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:294`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L294)*

### `LOG_LOCAL6`
```rust
const LOG_LOCAL6: crate::c_int = 176i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:295`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L295)*

### `LOG_LOCAL7`
```rust
const LOG_LOCAL7: crate::c_int = 184i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:296`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L296)*

### `LOG_PID`
```rust
const LOG_PID: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:300`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L300)*

### `LOG_CONS`
```rust
const LOG_CONS: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:301`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L301)*

### `LOG_ODELAY`
```rust
const LOG_ODELAY: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:302`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L302)*

### `LOG_NDELAY`
```rust
const LOG_NDELAY: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:303`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L303)*

### `LOG_NOWAIT`
```rust
const LOG_NOWAIT: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:304`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L304)*

### `LOG_PRIMASK`
```rust
const LOG_PRIMASK: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:307`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L307)*

### `LOG_FACMASK`
```rust
const LOG_FACMASK: crate::c_int = 1_016i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:308`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L308)*

### `PRIO_MIN`
```rust
const PRIO_MIN: crate::c_int = -20i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:312`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L312)*

### `PRIO_MAX`
```rust
const PRIO_MAX: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:313`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L313)*

### `IPPROTO_ICMP`
```rust
const IPPROTO_ICMP: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:316`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L316)*

### `IPPROTO_ICMPV6`
```rust
const IPPROTO_ICMPV6: crate::c_int = 58i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:317`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L317)*

### `IPPROTO_TCP`
```rust
const IPPROTO_TCP: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:318`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L318)*

### `IPPROTO_UDP`
```rust
const IPPROTO_UDP: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:319`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L319)*

### `IPPROTO_IP`
```rust
const IPPROTO_IP: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:320`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L320)*

### `IPPROTO_IPV6`
```rust
const IPPROTO_IPV6: crate::c_int = 41i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:321`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L321)*

### `INADDR_LOOPBACK`
```rust
const INADDR_LOOPBACK: in_addr_t = 2_130_706_433u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:323`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L323)*

### `INADDR_ANY`
```rust
const INADDR_ANY: in_addr_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:324`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L324)*

### `INADDR_BROADCAST`
```rust
const INADDR_BROADCAST: in_addr_t = 4_294_967_295u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:325`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L325)*

### `INADDR_NONE`
```rust
const INADDR_NONE: in_addr_t = 4_294_967_295u32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:326`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L326)*

### `IN6ADDR_LOOPBACK_INIT`
```rust
const IN6ADDR_LOOPBACK_INIT: in6_addr;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:328-330`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L328-L330)*

### `IN6ADDR_ANY_INIT`
```rust
const IN6ADDR_ANY_INIT: in6_addr;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:332-334`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L332-L334)*

### `ARPOP_REQUEST`
```rust
const ARPOP_REQUEST: u16 = 1u16;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:336`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L336)*

### `ARPOP_REPLY`
```rust
const ARPOP_REPLY: u16 = 2u16;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:337`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L337)*

### `ATF_COM`
```rust
const ATF_COM: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:339`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L339)*

### `ATF_PERM`
```rust
const ATF_PERM: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:340`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L340)*

### `ATF_PUBL`
```rust
const ATF_PUBL: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:341`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L341)*

### `ATF_USETRAILERS`
```rust
const ATF_USETRAILERS: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:342`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L342)*

### `FNM_PERIOD`
```rust
const FNM_PERIOD: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:348`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L348)*

### `FNM_NOMATCH`
```rust
const FNM_NOMATCH: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:351`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L351)*

### `FNM_CASEFOLD`
```rust
const FNM_CASEFOLD: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:361`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L361)*

### `FNM_PATHNAME`
```rust
const FNM_PATHNAME: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:376`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L376)*

### `FNM_NOESCAPE`
```rust
const FNM_NOESCAPE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/mod.rs:395`](../../../.source_1765210505/libc-0.2.178/src/unix/mod.rs#L395)*

### `ULONG_SIZE`
```rust
const ULONG_SIZE: usize = 64usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:367`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L367)*

### `EXIT_FAILURE`
```rust
const EXIT_FAILURE: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:373`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L373)*

### `EXIT_SUCCESS`
```rust
const EXIT_SUCCESS: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:374`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L374)*

### `RAND_MAX`
```rust
const RAND_MAX: crate::c_int = 2_147_483_647i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:375`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L375)*

### `EOF`
```rust
const EOF: crate::c_int = -1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:376`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L376)*

### `SEEK_SET`
```rust
const SEEK_SET: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:377`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L377)*

### `SEEK_CUR`
```rust
const SEEK_CUR: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:378`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L378)*

### `SEEK_END`
```rust
const SEEK_END: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:379`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L379)*

### `_IOFBF`
```rust
const _IOFBF: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:380`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L380)*

### `_IONBF`
```rust
const _IONBF: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:381`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L381)*

### `_IOLBF`
```rust
const _IOLBF: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:382`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L382)*

### `F_DUPFD`
```rust
const F_DUPFD: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:384`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L384)*

### `F_GETFD`
```rust
const F_GETFD: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:385`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L385)*

### `F_SETFD`
```rust
const F_SETFD: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:386`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L386)*

### `F_GETFL`
```rust
const F_GETFL: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:387`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L387)*

### `F_SETFL`
```rust
const F_SETFL: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:388`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L388)*

### `F_SETLEASE`
```rust
const F_SETLEASE: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:391`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L391)*

### `F_GETLEASE`
```rust
const F_GETLEASE: crate::c_int = 1_025i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:392`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L392)*

### `F_NOTIFY`
```rust
const F_NOTIFY: crate::c_int = 1_026i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:393`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L393)*

### `F_CANCELLK`
```rust
const F_CANCELLK: crate::c_int = 1_029i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:394`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L394)*

### `F_DUPFD_CLOEXEC`
```rust
const F_DUPFD_CLOEXEC: crate::c_int = 1_030i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:395`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L395)*

### `F_SETPIPE_SZ`
```rust
const F_SETPIPE_SZ: crate::c_int = 1_031i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:396`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L396)*

### `F_GETPIPE_SZ`
```rust
const F_GETPIPE_SZ: crate::c_int = 1_032i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:397`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L397)*

### `F_ADD_SEALS`
```rust
const F_ADD_SEALS: crate::c_int = 1_033i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:398`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L398)*

### `F_GET_SEALS`
```rust
const F_GET_SEALS: crate::c_int = 1_034i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:399`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L399)*

### `F_SEAL_SEAL`
```rust
const F_SEAL_SEAL: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:401`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L401)*

### `F_SEAL_SHRINK`
```rust
const F_SEAL_SHRINK: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:402`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L402)*

### `F_SEAL_GROW`
```rust
const F_SEAL_GROW: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:403`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L403)*

### `F_SEAL_WRITE`
```rust
const F_SEAL_WRITE: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:404`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L404)*

### `SIGTRAP`
```rust
const SIGTRAP: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:408`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L408)*

### `PTHREAD_CREATE_JOINABLE`
```rust
const PTHREAD_CREATE_JOINABLE: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:410`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L410)*

### `PTHREAD_CREATE_DETACHED`
```rust
const PTHREAD_CREATE_DETACHED: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:411`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L411)*

### `CLOCK_REALTIME`
```rust
const CLOCK_REALTIME: crate::clockid_t = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:413`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L413)*

### `CLOCK_MONOTONIC`
```rust
const CLOCK_MONOTONIC: crate::clockid_t = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:414`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L414)*

### `CLOCK_PROCESS_CPUTIME_ID`
```rust
const CLOCK_PROCESS_CPUTIME_ID: crate::clockid_t = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:415`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L415)*

### `CLOCK_THREAD_CPUTIME_ID`
```rust
const CLOCK_THREAD_CPUTIME_ID: crate::clockid_t = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:416`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L416)*

### `CLOCK_MONOTONIC_RAW`
```rust
const CLOCK_MONOTONIC_RAW: crate::clockid_t = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:417`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L417)*

### `CLOCK_REALTIME_COARSE`
```rust
const CLOCK_REALTIME_COARSE: crate::clockid_t = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:418`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L418)*

### `CLOCK_MONOTONIC_COARSE`
```rust
const CLOCK_MONOTONIC_COARSE: crate::clockid_t = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:419`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L419)*

### `CLOCK_BOOTTIME`
```rust
const CLOCK_BOOTTIME: crate::clockid_t = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:420`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L420)*

### `CLOCK_REALTIME_ALARM`
```rust
const CLOCK_REALTIME_ALARM: crate::clockid_t = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:421`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L421)*

### `CLOCK_BOOTTIME_ALARM`
```rust
const CLOCK_BOOTTIME_ALARM: crate::clockid_t = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:422`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L422)*

### `CLOCK_TAI`
```rust
const CLOCK_TAI: crate::clockid_t = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:423`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L423)*

### `TIMER_ABSTIME`
```rust
const TIMER_ABSTIME: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:424`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L424)*

### `RUSAGE_SELF`
```rust
const RUSAGE_SELF: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:426`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L426)*

### `O_RDONLY`
```rust
const O_RDONLY: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:428`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L428)*

### `O_WRONLY`
```rust
const O_WRONLY: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:429`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L429)*

### `O_RDWR`
```rust
const O_RDWR: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:430`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L430)*

### `SOCK_CLOEXEC`
```rust
const SOCK_CLOEXEC: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:432`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L432)*

### `S_IFIFO`
```rust
const S_IFIFO: mode_t = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:434`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L434)*

### `S_IFCHR`
```rust
const S_IFCHR: mode_t = 8_192u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:435`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L435)*

### `S_IFBLK`
```rust
const S_IFBLK: mode_t = 24_576u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:436`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L436)*

### `S_IFDIR`
```rust
const S_IFDIR: mode_t = 16_384u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:437`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L437)*

### `S_IFREG`
```rust
const S_IFREG: mode_t = 32_768u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:438`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L438)*

### `S_IFLNK`
```rust
const S_IFLNK: mode_t = 40_960u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:439`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L439)*

### `S_IFSOCK`
```rust
const S_IFSOCK: mode_t = 49_152u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:440`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L440)*

### `S_IFMT`
```rust
const S_IFMT: mode_t = 61_440u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:441`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L441)*

### `S_IRWXU`
```rust
const S_IRWXU: mode_t = 448u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:442`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L442)*

### `S_IXUSR`
```rust
const S_IXUSR: mode_t = 64u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:443`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L443)*

### `S_IWUSR`
```rust
const S_IWUSR: mode_t = 128u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:444`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L444)*

### `S_IRUSR`
```rust
const S_IRUSR: mode_t = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:445`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L445)*

### `S_IRWXG`
```rust
const S_IRWXG: mode_t = 56u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:446`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L446)*

### `S_IXGRP`
```rust
const S_IXGRP: mode_t = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:447`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L447)*

### `S_IWGRP`
```rust
const S_IWGRP: mode_t = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:448`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L448)*

### `S_IRGRP`
```rust
const S_IRGRP: mode_t = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:449`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L449)*

### `S_IRWXO`
```rust
const S_IRWXO: mode_t = 7u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:450`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L450)*

### `S_IXOTH`
```rust
const S_IXOTH: mode_t = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:451`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L451)*

### `S_IWOTH`
```rust
const S_IWOTH: mode_t = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:452`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L452)*

### `S_IROTH`
```rust
const S_IROTH: mode_t = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:453`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L453)*

### `F_OK`
```rust
const F_OK: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:454`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L454)*

### `R_OK`
```rust
const R_OK: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:455`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L455)*

### `W_OK`
```rust
const W_OK: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:456`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L456)*

### `X_OK`
```rust
const X_OK: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:457`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L457)*

### `SIGHUP`
```rust
const SIGHUP: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:458`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L458)*

### `SIGINT`
```rust
const SIGINT: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:459`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L459)*

### `SIGQUIT`
```rust
const SIGQUIT: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:460`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L460)*

### `SIGILL`
```rust
const SIGILL: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:461`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L461)*

### `SIGABRT`
```rust
const SIGABRT: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:462`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L462)*

### `SIGFPE`
```rust
const SIGFPE: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:463`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L463)*

### `SIGKILL`
```rust
const SIGKILL: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:464`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L464)*

### `SIGSEGV`
```rust
const SIGSEGV: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:465`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L465)*

### `SIGPIPE`
```rust
const SIGPIPE: crate::c_int = 13i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:466`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L466)*

### `SIGALRM`
```rust
const SIGALRM: crate::c_int = 14i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:467`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L467)*

### `SIGTERM`
```rust
const SIGTERM: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:468`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L468)*

### `PROT_NONE`
```rust
const PROT_NONE: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:470`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L470)*

### `PROT_READ`
```rust
const PROT_READ: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:471`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L471)*

### `PROT_WRITE`
```rust
const PROT_WRITE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:472`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L472)*

### `PROT_EXEC`
```rust
const PROT_EXEC: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:473`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L473)*

### `XATTR_CREATE`
```rust
const XATTR_CREATE: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:475`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L475)*

### `XATTR_REPLACE`
```rust
const XATTR_REPLACE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:476`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L476)*

### `RLIM64_INFINITY`
```rust
const RLIM64_INFINITY: crate::rlim64_t = 18_446_744_073_709_551_615u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:482`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L482)*

### `LC_CTYPE`
```rust
const LC_CTYPE: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:502`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L502)*

### `LC_NUMERIC`
```rust
const LC_NUMERIC: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:503`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L503)*

### `LC_TIME`
```rust
const LC_TIME: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:504`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L504)*

### `LC_COLLATE`
```rust
const LC_COLLATE: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:505`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L505)*

### `LC_MONETARY`
```rust
const LC_MONETARY: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:506`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L506)*

### `LC_MESSAGES`
```rust
const LC_MESSAGES: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:507`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L507)*

### `LC_ALL`
```rust
const LC_ALL: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:508`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L508)*

### `LC_CTYPE_MASK`
```rust
const LC_CTYPE_MASK: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:512`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L512)*

### `LC_NUMERIC_MASK`
```rust
const LC_NUMERIC_MASK: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:513`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L513)*

### `LC_TIME_MASK`
```rust
const LC_TIME_MASK: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:514`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L514)*

### `LC_COLLATE_MASK`
```rust
const LC_COLLATE_MASK: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:515`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L515)*

### `LC_MONETARY_MASK`
```rust
const LC_MONETARY_MASK: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:516`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L516)*

### `LC_MESSAGES_MASK`
```rust
const LC_MESSAGES_MASK: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:517`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L517)*

### `MAP_FILE`
```rust
const MAP_FILE: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:520`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L520)*

### `MAP_SHARED`
```rust
const MAP_SHARED: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:521`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L521)*

### `MAP_PRIVATE`
```rust
const MAP_PRIVATE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:522`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L522)*

### `MAP_FIXED`
```rust
const MAP_FIXED: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:523`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L523)*

### `MAP_FAILED`
```rust
const MAP_FAILED: *mut crate::c_void = {0xffffffffffffffff as *mut core::ffi::c_void};
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:525`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L525)*

### `MS_ASYNC`
```rust
const MS_ASYNC: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:528`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L528)*

### `MS_INVALIDATE`
```rust
const MS_INVALIDATE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:529`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L529)*

### `MS_SYNC`
```rust
const MS_SYNC: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:530`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L530)*

### `MS_RDONLY`
```rust
const MS_RDONLY: crate::c_ulong = 1u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:533`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L533)*

### `MS_NOSUID`
```rust
const MS_NOSUID: crate::c_ulong = 2u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:534`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L534)*

### `MS_NODEV`
```rust
const MS_NODEV: crate::c_ulong = 4u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:535`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L535)*

### `MS_NOEXEC`
```rust
const MS_NOEXEC: crate::c_ulong = 8u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:536`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L536)*

### `MS_SYNCHRONOUS`
```rust
const MS_SYNCHRONOUS: crate::c_ulong = 16u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:537`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L537)*

### `MS_REMOUNT`
```rust
const MS_REMOUNT: crate::c_ulong = 32u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:538`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L538)*

### `MS_MANDLOCK`
```rust
const MS_MANDLOCK: crate::c_ulong = 64u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:539`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L539)*

### `MS_DIRSYNC`
```rust
const MS_DIRSYNC: crate::c_ulong = 128u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:540`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L540)*

### `MS_NOSYMFOLLOW`
```rust
const MS_NOSYMFOLLOW: crate::c_ulong = 256u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:541`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L541)*

### `MS_NOATIME`
```rust
const MS_NOATIME: crate::c_ulong = 1_024u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:542`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L542)*

### `MS_NODIRATIME`
```rust
const MS_NODIRATIME: crate::c_ulong = 2_048u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:543`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L543)*

### `MS_BIND`
```rust
const MS_BIND: crate::c_ulong = 4_096u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:544`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L544)*

### `MS_MOVE`
```rust
const MS_MOVE: crate::c_ulong = 8_192u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:545`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L545)*

### `MS_REC`
```rust
const MS_REC: crate::c_ulong = 16_384u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:546`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L546)*

### `MS_SILENT`
```rust
const MS_SILENT: crate::c_ulong = 32_768u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:547`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L547)*

### `MS_POSIXACL`
```rust
const MS_POSIXACL: crate::c_ulong = 65_536u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:548`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L548)*

### `MS_UNBINDABLE`
```rust
const MS_UNBINDABLE: crate::c_ulong = 131_072u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:549`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L549)*

### `MS_PRIVATE`
```rust
const MS_PRIVATE: crate::c_ulong = 262_144u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:550`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L550)*

### `MS_SLAVE`
```rust
const MS_SLAVE: crate::c_ulong = 524_288u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:551`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L551)*

### `MS_SHARED`
```rust
const MS_SHARED: crate::c_ulong = 1_048_576u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:552`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L552)*

### `MS_RELATIME`
```rust
const MS_RELATIME: crate::c_ulong = 2_097_152u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:553`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L553)*

### `MS_KERNMOUNT`
```rust
const MS_KERNMOUNT: crate::c_ulong = 4_194_304u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:554`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L554)*

### `MS_I_VERSION`
```rust
const MS_I_VERSION: crate::c_ulong = 8_388_608u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:555`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L555)*

### `MS_STRICTATIME`
```rust
const MS_STRICTATIME: crate::c_ulong = 16_777_216u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:556`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L556)*

### `MS_LAZYTIME`
```rust
const MS_LAZYTIME: crate::c_ulong = 33_554_432u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:557`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L557)*

### `MS_ACTIVE`
```rust
const MS_ACTIVE: crate::c_ulong = 1_073_741_824u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:558`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L558)*

### `MS_MGC_VAL`
```rust
const MS_MGC_VAL: crate::c_ulong = 3_236_757_504u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:559`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L559)*

### `MS_MGC_MSK`
```rust
const MS_MGC_MSK: crate::c_ulong = 4_294_901_760u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:560`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L560)*

### `SCM_RIGHTS`
```rust
const SCM_RIGHTS: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:562`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L562)*

### `SCM_CREDENTIALS`
```rust
const SCM_CREDENTIALS: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:563`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L563)*

### `PROT_GROWSDOWN`
```rust
const PROT_GROWSDOWN: crate::c_int = 16_777_216i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:565`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L565)*

### `PROT_GROWSUP`
```rust
const PROT_GROWSUP: crate::c_int = 33_554_432i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:566`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L566)*

### `MAP_TYPE`
```rust
const MAP_TYPE: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:568`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L568)*

### `MADV_NORMAL`
```rust
const MADV_NORMAL: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:570`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L570)*

### `MADV_RANDOM`
```rust
const MADV_RANDOM: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:571`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L571)*

### `MADV_SEQUENTIAL`
```rust
const MADV_SEQUENTIAL: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:572`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L572)*

### `MADV_WILLNEED`
```rust
const MADV_WILLNEED: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:573`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L573)*

### `MADV_DONTNEED`
```rust
const MADV_DONTNEED: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:574`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L574)*

### `MADV_FREE`
```rust
const MADV_FREE: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:575`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L575)*

### `MADV_REMOVE`
```rust
const MADV_REMOVE: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:576`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L576)*

### `MADV_DONTFORK`
```rust
const MADV_DONTFORK: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:577`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L577)*

### `MADV_DOFORK`
```rust
const MADV_DOFORK: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:578`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L578)*

### `MADV_MERGEABLE`
```rust
const MADV_MERGEABLE: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:579`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L579)*

### `MADV_UNMERGEABLE`
```rust
const MADV_UNMERGEABLE: crate::c_int = 13i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:580`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L580)*

### `MADV_HUGEPAGE`
```rust
const MADV_HUGEPAGE: crate::c_int = 14i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:581`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L581)*

### `MADV_NOHUGEPAGE`
```rust
const MADV_NOHUGEPAGE: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:582`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L582)*

### `MADV_DONTDUMP`
```rust
const MADV_DONTDUMP: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:583`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L583)*

### `MADV_DODUMP`
```rust
const MADV_DODUMP: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:584`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L584)*

### `MADV_WIPEONFORK`
```rust
const MADV_WIPEONFORK: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:585`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L585)*

### `MADV_KEEPONFORK`
```rust
const MADV_KEEPONFORK: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:586`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L586)*

### `MADV_COLD`
```rust
const MADV_COLD: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:587`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L587)*

### `MADV_PAGEOUT`
```rust
const MADV_PAGEOUT: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:588`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L588)*

### `MADV_HWPOISON`
```rust
const MADV_HWPOISON: crate::c_int = 100i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:589`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L589)*

### `MADV_POPULATE_READ`
```rust
const MADV_POPULATE_READ: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:592`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L592)*

### `MADV_POPULATE_WRITE`
```rust
const MADV_POPULATE_WRITE: crate::c_int = 23i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:593`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L593)*

### `MADV_DONTNEED_LOCKED`
```rust
const MADV_DONTNEED_LOCKED: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:594`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L594)*

### `IFF_UP`
```rust
const IFF_UP: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:598`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L598)*

### `IFF_BROADCAST`
```rust
const IFF_BROADCAST: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:599`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L599)*

### `IFF_DEBUG`
```rust
const IFF_DEBUG: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:600`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L600)*

### `IFF_LOOPBACK`
```rust
const IFF_LOOPBACK: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:601`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L601)*

### `IFF_POINTOPOINT`
```rust
const IFF_POINTOPOINT: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:602`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L602)*

### `IFF_NOTRAILERS`
```rust
const IFF_NOTRAILERS: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:603`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L603)*

### `IFF_RUNNING`
```rust
const IFF_RUNNING: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:604`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L604)*

### `IFF_NOARP`
```rust
const IFF_NOARP: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:605`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L605)*

### `IFF_PROMISC`
```rust
const IFF_PROMISC: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:606`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L606)*

### `IFF_ALLMULTI`
```rust
const IFF_ALLMULTI: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:607`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L607)*

### `IFF_MASTER`
```rust
const IFF_MASTER: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:608`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L608)*

### `IFF_SLAVE`
```rust
const IFF_SLAVE: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:609`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L609)*

### `IFF_MULTICAST`
```rust
const IFF_MULTICAST: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:610`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L610)*

### `IFF_PORTSEL`
```rust
const IFF_PORTSEL: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:611`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L611)*

### `IFF_AUTOMEDIA`
```rust
const IFF_AUTOMEDIA: crate::c_int = 16_384i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:612`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L612)*

### `IFF_DYNAMIC`
```rust
const IFF_DYNAMIC: crate::c_int = 32_768i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:613`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L613)*

### `SOL_IP`
```rust
const SOL_IP: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:615`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L615)*

### `SOL_TCP`
```rust
const SOL_TCP: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:616`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L616)*

### `SOL_UDP`
```rust
const SOL_UDP: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:617`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L617)*

### `SOL_IPV6`
```rust
const SOL_IPV6: crate::c_int = 41i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:618`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L618)*

### `SOL_ICMPV6`
```rust
const SOL_ICMPV6: crate::c_int = 58i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:619`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L619)*

### `SOL_RAW`
```rust
const SOL_RAW: crate::c_int = 255i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:620`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L620)*

### `SOL_DECNET`
```rust
const SOL_DECNET: crate::c_int = 261i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:621`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L621)*

### `SOL_X25`
```rust
const SOL_X25: crate::c_int = 262i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:622`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L622)*

### `SOL_PACKET`
```rust
const SOL_PACKET: crate::c_int = 263i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:623`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L623)*

### `SOL_ATM`
```rust
const SOL_ATM: crate::c_int = 264i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:624`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L624)*

### `SOL_AAL`
```rust
const SOL_AAL: crate::c_int = 265i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:625`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L625)*

### `SOL_IRDA`
```rust
const SOL_IRDA: crate::c_int = 266i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:626`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L626)*

### `SOL_NETBEUI`
```rust
const SOL_NETBEUI: crate::c_int = 267i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:627`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L627)*

### `SOL_LLC`
```rust
const SOL_LLC: crate::c_int = 268i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:628`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L628)*

### `SOL_DCCP`
```rust
const SOL_DCCP: crate::c_int = 269i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:629`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L629)*

### `SOL_NETLINK`
```rust
const SOL_NETLINK: crate::c_int = 270i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:630`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L630)*

### `SOL_TIPC`
```rust
const SOL_TIPC: crate::c_int = 271i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:631`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L631)*

### `SOL_BLUETOOTH`
```rust
const SOL_BLUETOOTH: crate::c_int = 274i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:632`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L632)*

### `SOL_ALG`
```rust
const SOL_ALG: crate::c_int = 279i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:633`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L633)*

### `AF_UNSPEC`
```rust
const AF_UNSPEC: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:635`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L635)*

### `AF_UNIX`
```rust
const AF_UNIX: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:636`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L636)*

### `AF_LOCAL`
```rust
const AF_LOCAL: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:637`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L637)*

### `AF_INET`
```rust
const AF_INET: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:638`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L638)*

### `AF_AX25`
```rust
const AF_AX25: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:639`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L639)*

### `AF_IPX`
```rust
const AF_IPX: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:640`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L640)*

### `AF_APPLETALK`
```rust
const AF_APPLETALK: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:641`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L641)*

### `AF_NETROM`
```rust
const AF_NETROM: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:642`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L642)*

### `AF_BRIDGE`
```rust
const AF_BRIDGE: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:643`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L643)*

### `AF_ATMPVC`
```rust
const AF_ATMPVC: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:644`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L644)*

### `AF_X25`
```rust
const AF_X25: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:645`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L645)*

### `AF_INET6`
```rust
const AF_INET6: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:646`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L646)*

### `AF_ROSE`
```rust
const AF_ROSE: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:647`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L647)*

### `AF_DECnet`
```rust
const AF_DECnet: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:648`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L648)*

### `AF_NETBEUI`
```rust
const AF_NETBEUI: crate::c_int = 13i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:649`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L649)*

### `AF_SECURITY`
```rust
const AF_SECURITY: crate::c_int = 14i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:650`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L650)*

### `AF_KEY`
```rust
const AF_KEY: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:651`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L651)*

### `AF_NETLINK`
```rust
const AF_NETLINK: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:652`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L652)*

### `AF_ROUTE`
```rust
const AF_ROUTE: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:653`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L653)*

### `AF_PACKET`
```rust
const AF_PACKET: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:654`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L654)*

### `AF_ASH`
```rust
const AF_ASH: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:655`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L655)*

### `AF_ECONET`
```rust
const AF_ECONET: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:656`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L656)*

### `AF_ATMSVC`
```rust
const AF_ATMSVC: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:657`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L657)*

### `AF_RDS`
```rust
const AF_RDS: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:658`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L658)*

### `AF_SNA`
```rust
const AF_SNA: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:659`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L659)*

### `AF_IRDA`
```rust
const AF_IRDA: crate::c_int = 23i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:660`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L660)*

### `AF_PPPOX`
```rust
const AF_PPPOX: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:661`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L661)*

### `AF_WANPIPE`
```rust
const AF_WANPIPE: crate::c_int = 25i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:662`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L662)*

### `AF_LLC`
```rust
const AF_LLC: crate::c_int = 26i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:663`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L663)*

### `AF_CAN`
```rust
const AF_CAN: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:664`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L664)*

### `AF_TIPC`
```rust
const AF_TIPC: crate::c_int = 30i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:665`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L665)*

### `AF_BLUETOOTH`
```rust
const AF_BLUETOOTH: crate::c_int = 31i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:666`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L666)*

### `AF_IUCV`
```rust
const AF_IUCV: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:667`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L667)*

### `AF_RXRPC`
```rust
const AF_RXRPC: crate::c_int = 33i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:668`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L668)*

### `AF_ISDN`
```rust
const AF_ISDN: crate::c_int = 34i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:669`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L669)*

### `AF_PHONET`
```rust
const AF_PHONET: crate::c_int = 35i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:670`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L670)*

### `AF_IEEE802154`
```rust
const AF_IEEE802154: crate::c_int = 36i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:671`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L671)*

### `AF_CAIF`
```rust
const AF_CAIF: crate::c_int = 37i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:672`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L672)*

### `AF_ALG`
```rust
const AF_ALG: crate::c_int = 38i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:673`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L673)*

### `PF_UNSPEC`
```rust
const PF_UNSPEC: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:675`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L675)*

### `PF_UNIX`
```rust
const PF_UNIX: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:676`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L676)*

### `PF_LOCAL`
```rust
const PF_LOCAL: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:677`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L677)*

### `PF_INET`
```rust
const PF_INET: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:678`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L678)*

### `PF_AX25`
```rust
const PF_AX25: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:679`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L679)*

### `PF_IPX`
```rust
const PF_IPX: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:680`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L680)*

### `PF_APPLETALK`
```rust
const PF_APPLETALK: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:681`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L681)*

### `PF_NETROM`
```rust
const PF_NETROM: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:682`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L682)*

### `PF_BRIDGE`
```rust
const PF_BRIDGE: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:683`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L683)*

### `PF_ATMPVC`
```rust
const PF_ATMPVC: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:684`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L684)*

### `PF_X25`
```rust
const PF_X25: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:685`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L685)*

### `PF_INET6`
```rust
const PF_INET6: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:686`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L686)*

### `PF_ROSE`
```rust
const PF_ROSE: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:687`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L687)*

### `PF_DECnet`
```rust
const PF_DECnet: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:688`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L688)*

### `PF_NETBEUI`
```rust
const PF_NETBEUI: crate::c_int = 13i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:689`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L689)*

### `PF_SECURITY`
```rust
const PF_SECURITY: crate::c_int = 14i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:690`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L690)*

### `PF_KEY`
```rust
const PF_KEY: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:691`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L691)*

### `PF_NETLINK`
```rust
const PF_NETLINK: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:692`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L692)*

### `PF_ROUTE`
```rust
const PF_ROUTE: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:693`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L693)*

### `PF_PACKET`
```rust
const PF_PACKET: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:694`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L694)*

### `PF_ASH`
```rust
const PF_ASH: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:695`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L695)*

### `PF_ECONET`
```rust
const PF_ECONET: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:696`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L696)*

### `PF_ATMSVC`
```rust
const PF_ATMSVC: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:697`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L697)*

### `PF_RDS`
```rust
const PF_RDS: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:698`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L698)*

### `PF_SNA`
```rust
const PF_SNA: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:699`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L699)*

### `PF_IRDA`
```rust
const PF_IRDA: crate::c_int = 23i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:700`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L700)*

### `PF_PPPOX`
```rust
const PF_PPPOX: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:701`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L701)*

### `PF_WANPIPE`
```rust
const PF_WANPIPE: crate::c_int = 25i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:702`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L702)*

### `PF_LLC`
```rust
const PF_LLC: crate::c_int = 26i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:703`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L703)*

### `PF_CAN`
```rust
const PF_CAN: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:704`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L704)*

### `PF_TIPC`
```rust
const PF_TIPC: crate::c_int = 30i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:705`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L705)*

### `PF_BLUETOOTH`
```rust
const PF_BLUETOOTH: crate::c_int = 31i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:706`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L706)*

### `PF_IUCV`
```rust
const PF_IUCV: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:707`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L707)*

### `PF_RXRPC`
```rust
const PF_RXRPC: crate::c_int = 33i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:708`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L708)*

### `PF_ISDN`
```rust
const PF_ISDN: crate::c_int = 34i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:709`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L709)*

### `PF_PHONET`
```rust
const PF_PHONET: crate::c_int = 35i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:710`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L710)*

### `PF_IEEE802154`
```rust
const PF_IEEE802154: crate::c_int = 36i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:711`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L711)*

### `PF_CAIF`
```rust
const PF_CAIF: crate::c_int = 37i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:712`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L712)*

### `PF_ALG`
```rust
const PF_ALG: crate::c_int = 38i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:713`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L713)*

### `MSG_OOB`
```rust
const MSG_OOB: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:715`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L715)*

### `MSG_PEEK`
```rust
const MSG_PEEK: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:716`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L716)*

### `MSG_DONTROUTE`
```rust
const MSG_DONTROUTE: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:717`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L717)*

### `MSG_CTRUNC`
```rust
const MSG_CTRUNC: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:718`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L718)*

### `MSG_TRUNC`
```rust
const MSG_TRUNC: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:719`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L719)*

### `MSG_DONTWAIT`
```rust
const MSG_DONTWAIT: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:720`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L720)*

### `MSG_EOR`
```rust
const MSG_EOR: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:721`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L721)*

### `MSG_WAITALL`
```rust
const MSG_WAITALL: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:722`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L722)*

### `MSG_FIN`
```rust
const MSG_FIN: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:723`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L723)*

### `MSG_SYN`
```rust
const MSG_SYN: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:724`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L724)*

### `MSG_CONFIRM`
```rust
const MSG_CONFIRM: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:725`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L725)*

### `MSG_RST`
```rust
const MSG_RST: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:726`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L726)*

### `MSG_ERRQUEUE`
```rust
const MSG_ERRQUEUE: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:727`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L727)*

### `MSG_NOSIGNAL`
```rust
const MSG_NOSIGNAL: crate::c_int = 16_384i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:728`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L728)*

### `MSG_MORE`
```rust
const MSG_MORE: crate::c_int = 32_768i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:729`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L729)*

### `MSG_WAITFORONE`
```rust
const MSG_WAITFORONE: crate::c_int = 65_536i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:730`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L730)*

### `MSG_FASTOPEN`
```rust
const MSG_FASTOPEN: crate::c_int = 536_870_912i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:731`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L731)*

### `MSG_CMSG_CLOEXEC`
```rust
const MSG_CMSG_CLOEXEC: crate::c_int = 1_073_741_824i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:732`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L732)*

### `SCM_TIMESTAMP`
```rust
const SCM_TIMESTAMP: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:734`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L734)*

### `SOCK_RAW`
```rust
const SOCK_RAW: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:736`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L736)*

### `SOCK_RDM`
```rust
const SOCK_RDM: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:737`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L737)*

### `IP_TOS`
```rust
const IP_TOS: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:738`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L738)*

### `IP_TTL`
```rust
const IP_TTL: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:739`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L739)*

### `IP_HDRINCL`
```rust
const IP_HDRINCL: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:740`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L740)*

### `IP_OPTIONS`
```rust
const IP_OPTIONS: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:741`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L741)*

### `IP_ROUTER_ALERT`
```rust
const IP_ROUTER_ALERT: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:742`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L742)*

### `IP_RECVOPTS`
```rust
const IP_RECVOPTS: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:743`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L743)*

### `IP_RETOPTS`
```rust
const IP_RETOPTS: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:744`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L744)*

### `IP_PKTINFO`
```rust
const IP_PKTINFO: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:745`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L745)*

### `IP_PKTOPTIONS`
```rust
const IP_PKTOPTIONS: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:746`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L746)*

### `IP_MTU_DISCOVER`
```rust
const IP_MTU_DISCOVER: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:747`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L747)*

### `IP_RECVERR`
```rust
const IP_RECVERR: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:748`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L748)*

### `IP_RECVTTL`
```rust
const IP_RECVTTL: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:749`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L749)*

### `IP_RECVTOS`
```rust
const IP_RECVTOS: crate::c_int = 13i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:750`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L750)*

### `IP_MTU`
```rust
const IP_MTU: crate::c_int = 14i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:751`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L751)*

### `IP_FREEBIND`
```rust
const IP_FREEBIND: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:752`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L752)*

### `IP_IPSEC_POLICY`
```rust
const IP_IPSEC_POLICY: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:753`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L753)*

### `IP_XFRM_POLICY`
```rust
const IP_XFRM_POLICY: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:754`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L754)*

### `IP_PASSSEC`
```rust
const IP_PASSSEC: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:755`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L755)*

### `IP_TRANSPARENT`
```rust
const IP_TRANSPARENT: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:756`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L756)*

### `IP_ORIGDSTADDR`
```rust
const IP_ORIGDSTADDR: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:757`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L757)*

### `IP_RECVORIGDSTADDR`
```rust
const IP_RECVORIGDSTADDR: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:758`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L758)*

### `IP_MINTTL`
```rust
const IP_MINTTL: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:759`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L759)*

### `IP_NODEFRAG`
```rust
const IP_NODEFRAG: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:760`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L760)*

### `IP_CHECKSUM`
```rust
const IP_CHECKSUM: crate::c_int = 23i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:761`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L761)*

### `IP_BIND_ADDRESS_NO_PORT`
```rust
const IP_BIND_ADDRESS_NO_PORT: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:762`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L762)*

### `IP_MULTICAST_IF`
```rust
const IP_MULTICAST_IF: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:763`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L763)*

### `IP_MULTICAST_TTL`
```rust
const IP_MULTICAST_TTL: crate::c_int = 33i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:764`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L764)*

### `IP_MULTICAST_LOOP`
```rust
const IP_MULTICAST_LOOP: crate::c_int = 34i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:765`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L765)*

### `IP_ADD_MEMBERSHIP`
```rust
const IP_ADD_MEMBERSHIP: crate::c_int = 35i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:766`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L766)*

### `IP_DROP_MEMBERSHIP`
```rust
const IP_DROP_MEMBERSHIP: crate::c_int = 36i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:767`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L767)*

### `IP_UNBLOCK_SOURCE`
```rust
const IP_UNBLOCK_SOURCE: crate::c_int = 37i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:768`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L768)*

### `IP_BLOCK_SOURCE`
```rust
const IP_BLOCK_SOURCE: crate::c_int = 38i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:769`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L769)*

### `IP_ADD_SOURCE_MEMBERSHIP`
```rust
const IP_ADD_SOURCE_MEMBERSHIP: crate::c_int = 39i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:770`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L770)*

### `IP_DROP_SOURCE_MEMBERSHIP`
```rust
const IP_DROP_SOURCE_MEMBERSHIP: crate::c_int = 40i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:771`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L771)*

### `IP_MSFILTER`
```rust
const IP_MSFILTER: crate::c_int = 41i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:772`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L772)*

### `IP_MULTICAST_ALL`
```rust
const IP_MULTICAST_ALL: crate::c_int = 49i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:773`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L773)*

### `IP_UNICAST_IF`
```rust
const IP_UNICAST_IF: crate::c_int = 50i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:774`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L774)*

### `IP_DEFAULT_MULTICAST_TTL`
```rust
const IP_DEFAULT_MULTICAST_TTL: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:776`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L776)*

### `IP_DEFAULT_MULTICAST_LOOP`
```rust
const IP_DEFAULT_MULTICAST_LOOP: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:777`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L777)*

### `IP_PMTUDISC_DONT`
```rust
const IP_PMTUDISC_DONT: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:779`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L779)*

### `IP_PMTUDISC_WANT`
```rust
const IP_PMTUDISC_WANT: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:780`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L780)*

### `IP_PMTUDISC_DO`
```rust
const IP_PMTUDISC_DO: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:781`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L781)*

### `IP_PMTUDISC_PROBE`
```rust
const IP_PMTUDISC_PROBE: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:782`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L782)*

### `IP_PMTUDISC_INTERFACE`
```rust
const IP_PMTUDISC_INTERFACE: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:783`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L783)*

### `IP_PMTUDISC_OMIT`
```rust
const IP_PMTUDISC_OMIT: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:784`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L784)*

### `IPPROTO_HOPOPTS`
```rust
const IPPROTO_HOPOPTS: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:788`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L788)*

Hop-by-hop option header

### `IPPROTO_IGMP`
```rust
const IPPROTO_IGMP: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:791`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L791)*

group mgmt protocol

### `IPPROTO_IPIP`
```rust
const IPPROTO_IPIP: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:793`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L793)*

for compatibility

### `IPPROTO_EGP`
```rust
const IPPROTO_EGP: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:796`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L796)*

exterior gateway protocol

### `IPPROTO_PUP`
```rust
const IPPROTO_PUP: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:798`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L798)*

pup

### `IPPROTO_IDP`
```rust
const IPPROTO_IDP: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:801`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L801)*

xns idp

### `IPPROTO_TP`
```rust
const IPPROTO_TP: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:803`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L803)*

tp-4 w/ class negotiation

### `IPPROTO_DCCP`
```rust
const IPPROTO_DCCP: crate::c_int = 33i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:805`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L805)*

DCCP

### `IPPROTO_ROUTING`
```rust
const IPPROTO_ROUTING: crate::c_int = 43i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:808`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L808)*

IP6 routing header

### `IPPROTO_FRAGMENT`
```rust
const IPPROTO_FRAGMENT: crate::c_int = 44i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:810`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L810)*

IP6 fragmentation header

### `IPPROTO_RSVP`
```rust
const IPPROTO_RSVP: crate::c_int = 46i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:812`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L812)*

resource reservation

### `IPPROTO_GRE`
```rust
const IPPROTO_GRE: crate::c_int = 47i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:814`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L814)*

General Routing Encap.

### `IPPROTO_ESP`
```rust
const IPPROTO_ESP: crate::c_int = 50i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:816`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L816)*

IP6 Encap Sec. Payload

### `IPPROTO_AH`
```rust
const IPPROTO_AH: crate::c_int = 51i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:818`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L818)*

IP6 Auth Header

### `IPPROTO_NONE`
```rust
const IPPROTO_NONE: crate::c_int = 59i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:821`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L821)*

IP6 no next header

### `IPPROTO_DSTOPTS`
```rust
const IPPROTO_DSTOPTS: crate::c_int = 60i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:823`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L823)*

IP6 destination option

### `IPPROTO_MTP`
```rust
const IPPROTO_MTP: crate::c_int = 92i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:824`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L824)*

### `IPPROTO_ENCAP`
```rust
const IPPROTO_ENCAP: crate::c_int = 98i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:826`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L826)*

encapsulation header

### `IPPROTO_PIM`
```rust
const IPPROTO_PIM: crate::c_int = 103i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:828`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L828)*

Protocol indep. multicast

### `IPPROTO_COMP`
```rust
const IPPROTO_COMP: crate::c_int = 108i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:830`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L830)*

IP Payload Comp. Protocol

### `IPPROTO_SCTP`
```rust
const IPPROTO_SCTP: crate::c_int = 132i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:832`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L832)*

SCTP

### `IPPROTO_MH`
```rust
const IPPROTO_MH: crate::c_int = 135i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:833`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L833)*

### `IPPROTO_UDPLITE`
```rust
const IPPROTO_UDPLITE: crate::c_int = 136i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:834`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L834)*

### `IPPROTO_RAW`
```rust
const IPPROTO_RAW: crate::c_int = 255i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:836`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L836)*

raw IP packet

### `IPPROTO_BEETPH`
```rust
const IPPROTO_BEETPH: crate::c_int = 94i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:837`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L837)*

### `IPPROTO_MPLS`
```rust
const IPPROTO_MPLS: crate::c_int = 137i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:838`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L838)*

### `IPPROTO_MPTCP`
```rust
const IPPROTO_MPTCP: crate::c_int = 262i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:840`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L840)*

Multipath TCP

### `IPPROTO_ETHERNET`
```rust
const IPPROTO_ETHERNET: crate::c_int = 143i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:842`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L842)*

Ethernet-within-IPv6 encapsulation.

### `MCAST_EXCLUDE`
```rust
const MCAST_EXCLUDE: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:844`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L844)*

### `MCAST_INCLUDE`
```rust
const MCAST_INCLUDE: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:845`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L845)*

### `MCAST_JOIN_GROUP`
```rust
const MCAST_JOIN_GROUP: crate::c_int = 42i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:846`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L846)*

### `MCAST_BLOCK_SOURCE`
```rust
const MCAST_BLOCK_SOURCE: crate::c_int = 43i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:847`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L847)*

### `MCAST_UNBLOCK_SOURCE`
```rust
const MCAST_UNBLOCK_SOURCE: crate::c_int = 44i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:848`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L848)*

### `MCAST_LEAVE_GROUP`
```rust
const MCAST_LEAVE_GROUP: crate::c_int = 45i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:849`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L849)*

### `MCAST_JOIN_SOURCE_GROUP`
```rust
const MCAST_JOIN_SOURCE_GROUP: crate::c_int = 46i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:850`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L850)*

### `MCAST_LEAVE_SOURCE_GROUP`
```rust
const MCAST_LEAVE_SOURCE_GROUP: crate::c_int = 47i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:851`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L851)*

### `MCAST_MSFILTER`
```rust
const MCAST_MSFILTER: crate::c_int = 48i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:852`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L852)*

### `IPV6_ADDRFORM`
```rust
const IPV6_ADDRFORM: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:854`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L854)*

### `IPV6_2292PKTINFO`
```rust
const IPV6_2292PKTINFO: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:855`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L855)*

### `IPV6_2292HOPOPTS`
```rust
const IPV6_2292HOPOPTS: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:856`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L856)*

### `IPV6_2292DSTOPTS`
```rust
const IPV6_2292DSTOPTS: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:857`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L857)*

### `IPV6_2292RTHDR`
```rust
const IPV6_2292RTHDR: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:858`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L858)*

### `IPV6_2292PKTOPTIONS`
```rust
const IPV6_2292PKTOPTIONS: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:859`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L859)*

### `IPV6_CHECKSUM`
```rust
const IPV6_CHECKSUM: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:860`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L860)*

### `IPV6_2292HOPLIMIT`
```rust
const IPV6_2292HOPLIMIT: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:861`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L861)*

### `IPV6_NEXTHOP`
```rust
const IPV6_NEXTHOP: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:862`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L862)*

### `IPV6_AUTHHDR`
```rust
const IPV6_AUTHHDR: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:863`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L863)*

### `IPV6_UNICAST_HOPS`
```rust
const IPV6_UNICAST_HOPS: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:864`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L864)*

### `IPV6_MULTICAST_IF`
```rust
const IPV6_MULTICAST_IF: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:865`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L865)*

### `IPV6_MULTICAST_HOPS`
```rust
const IPV6_MULTICAST_HOPS: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:866`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L866)*

### `IPV6_MULTICAST_LOOP`
```rust
const IPV6_MULTICAST_LOOP: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:867`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L867)*

### `IPV6_ADD_MEMBERSHIP`
```rust
const IPV6_ADD_MEMBERSHIP: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:868`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L868)*

### `IPV6_DROP_MEMBERSHIP`
```rust
const IPV6_DROP_MEMBERSHIP: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:869`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L869)*

### `IPV6_ROUTER_ALERT`
```rust
const IPV6_ROUTER_ALERT: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:870`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L870)*

### `IPV6_MTU_DISCOVER`
```rust
const IPV6_MTU_DISCOVER: crate::c_int = 23i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:871`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L871)*

### `IPV6_MTU`
```rust
const IPV6_MTU: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:872`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L872)*

### `IPV6_RECVERR`
```rust
const IPV6_RECVERR: crate::c_int = 25i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:873`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L873)*

### `IPV6_V6ONLY`
```rust
const IPV6_V6ONLY: crate::c_int = 26i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:874`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L874)*

### `IPV6_JOIN_ANYCAST`
```rust
const IPV6_JOIN_ANYCAST: crate::c_int = 27i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:875`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L875)*

### `IPV6_LEAVE_ANYCAST`
```rust
const IPV6_LEAVE_ANYCAST: crate::c_int = 28i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:876`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L876)*

### `IPV6_IPSEC_POLICY`
```rust
const IPV6_IPSEC_POLICY: crate::c_int = 34i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:877`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L877)*

### `IPV6_XFRM_POLICY`
```rust
const IPV6_XFRM_POLICY: crate::c_int = 35i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:878`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L878)*

### `IPV6_HDRINCL`
```rust
const IPV6_HDRINCL: crate::c_int = 36i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:879`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L879)*

### `IPV6_RECVPKTINFO`
```rust
const IPV6_RECVPKTINFO: crate::c_int = 49i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:880`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L880)*

### `IPV6_PKTINFO`
```rust
const IPV6_PKTINFO: crate::c_int = 50i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:881`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L881)*

### `IPV6_RECVHOPLIMIT`
```rust
const IPV6_RECVHOPLIMIT: crate::c_int = 51i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:882`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L882)*

### `IPV6_HOPLIMIT`
```rust
const IPV6_HOPLIMIT: crate::c_int = 52i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:883`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L883)*

### `IPV6_RECVHOPOPTS`
```rust
const IPV6_RECVHOPOPTS: crate::c_int = 53i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:884`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L884)*

### `IPV6_HOPOPTS`
```rust
const IPV6_HOPOPTS: crate::c_int = 54i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:885`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L885)*

### `IPV6_RTHDRDSTOPTS`
```rust
const IPV6_RTHDRDSTOPTS: crate::c_int = 55i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:886`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L886)*

### `IPV6_RECVRTHDR`
```rust
const IPV6_RECVRTHDR: crate::c_int = 56i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:887`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L887)*

### `IPV6_RTHDR`
```rust
const IPV6_RTHDR: crate::c_int = 57i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:888`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L888)*

### `IPV6_RECVDSTOPTS`
```rust
const IPV6_RECVDSTOPTS: crate::c_int = 58i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:889`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L889)*

### `IPV6_DSTOPTS`
```rust
const IPV6_DSTOPTS: crate::c_int = 59i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:890`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L890)*

### `IPV6_RECVPATHMTU`
```rust
const IPV6_RECVPATHMTU: crate::c_int = 60i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:891`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L891)*

### `IPV6_PATHMTU`
```rust
const IPV6_PATHMTU: crate::c_int = 61i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:892`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L892)*

### `IPV6_DONTFRAG`
```rust
const IPV6_DONTFRAG: crate::c_int = 62i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:893`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L893)*

### `IPV6_RECVTCLASS`
```rust
const IPV6_RECVTCLASS: crate::c_int = 66i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:894`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L894)*

### `IPV6_TCLASS`
```rust
const IPV6_TCLASS: crate::c_int = 67i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:895`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L895)*

### `IPV6_AUTOFLOWLABEL`
```rust
const IPV6_AUTOFLOWLABEL: crate::c_int = 70i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:896`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L896)*

### `IPV6_ADDR_PREFERENCES`
```rust
const IPV6_ADDR_PREFERENCES: crate::c_int = 72i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:897`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L897)*

### `IPV6_MINHOPCOUNT`
```rust
const IPV6_MINHOPCOUNT: crate::c_int = 73i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:898`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L898)*

### `IPV6_ORIGDSTADDR`
```rust
const IPV6_ORIGDSTADDR: crate::c_int = 74i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:899`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L899)*

### `IPV6_RECVORIGDSTADDR`
```rust
const IPV6_RECVORIGDSTADDR: crate::c_int = 74i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:900`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L900)*

### `IPV6_TRANSPARENT`
```rust
const IPV6_TRANSPARENT: crate::c_int = 75i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:901`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L901)*

### `IPV6_UNICAST_IF`
```rust
const IPV6_UNICAST_IF: crate::c_int = 76i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:902`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L902)*

### `IPV6_PREFER_SRC_TMP`
```rust
const IPV6_PREFER_SRC_TMP: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:903`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L903)*

### `IPV6_PREFER_SRC_PUBLIC`
```rust
const IPV6_PREFER_SRC_PUBLIC: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:904`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L904)*

### `IPV6_PREFER_SRC_PUBTMP_DEFAULT`
```rust
const IPV6_PREFER_SRC_PUBTMP_DEFAULT: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:905`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L905)*

### `IPV6_PREFER_SRC_COA`
```rust
const IPV6_PREFER_SRC_COA: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:906`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L906)*

### `IPV6_PREFER_SRC_HOME`
```rust
const IPV6_PREFER_SRC_HOME: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:907`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L907)*

### `IPV6_PREFER_SRC_CGA`
```rust
const IPV6_PREFER_SRC_CGA: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:908`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L908)*

### `IPV6_PREFER_SRC_NONCGA`
```rust
const IPV6_PREFER_SRC_NONCGA: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:909`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L909)*

### `IPV6_PMTUDISC_DONT`
```rust
const IPV6_PMTUDISC_DONT: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:911`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L911)*

### `IPV6_PMTUDISC_WANT`
```rust
const IPV6_PMTUDISC_WANT: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:912`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L912)*

### `IPV6_PMTUDISC_DO`
```rust
const IPV6_PMTUDISC_DO: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:913`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L913)*

### `IPV6_PMTUDISC_PROBE`
```rust
const IPV6_PMTUDISC_PROBE: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:914`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L914)*

### `IPV6_PMTUDISC_INTERFACE`
```rust
const IPV6_PMTUDISC_INTERFACE: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:915`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L915)*

### `IPV6_PMTUDISC_OMIT`
```rust
const IPV6_PMTUDISC_OMIT: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:916`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L916)*

### `TCP_NODELAY`
```rust
const TCP_NODELAY: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:918`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L918)*

### `TCP_MAXSEG`
```rust
const TCP_MAXSEG: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:919`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L919)*

### `TCP_CORK`
```rust
const TCP_CORK: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:920`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L920)*

### `TCP_KEEPIDLE`
```rust
const TCP_KEEPIDLE: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:921`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L921)*

### `TCP_KEEPINTVL`
```rust
const TCP_KEEPINTVL: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:922`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L922)*

### `TCP_KEEPCNT`
```rust
const TCP_KEEPCNT: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:923`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L923)*

### `TCP_SYNCNT`
```rust
const TCP_SYNCNT: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:924`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L924)*

### `TCP_LINGER2`
```rust
const TCP_LINGER2: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:925`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L925)*

### `TCP_DEFER_ACCEPT`
```rust
const TCP_DEFER_ACCEPT: crate::c_int = 9i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:926`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L926)*

### `TCP_WINDOW_CLAMP`
```rust
const TCP_WINDOW_CLAMP: crate::c_int = 10i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:927`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L927)*

### `TCP_INFO`
```rust
const TCP_INFO: crate::c_int = 11i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:928`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L928)*

### `TCP_QUICKACK`
```rust
const TCP_QUICKACK: crate::c_int = 12i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:929`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L929)*

### `TCP_CONGESTION`
```rust
const TCP_CONGESTION: crate::c_int = 13i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:930`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L930)*

### `TCP_MD5SIG`
```rust
const TCP_MD5SIG: crate::c_int = 14i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:931`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L931)*

### `TCP_COOKIE_TRANSACTIONS`
```rust
const TCP_COOKIE_TRANSACTIONS: crate::c_int = 15i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:938`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L938)*

### `TCP_THIN_LINEAR_TIMEOUTS`
```rust
const TCP_THIN_LINEAR_TIMEOUTS: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:941`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L941)*

### `TCP_THIN_DUPACK`
```rust
const TCP_THIN_DUPACK: crate::c_int = 17i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:942`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L942)*

### `TCP_USER_TIMEOUT`
```rust
const TCP_USER_TIMEOUT: crate::c_int = 18i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:943`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L943)*

### `TCP_REPAIR`
```rust
const TCP_REPAIR: crate::c_int = 19i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:944`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L944)*

### `TCP_REPAIR_QUEUE`
```rust
const TCP_REPAIR_QUEUE: crate::c_int = 20i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:945`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L945)*

### `TCP_QUEUE_SEQ`
```rust
const TCP_QUEUE_SEQ: crate::c_int = 21i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:946`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L946)*

### `TCP_REPAIR_OPTIONS`
```rust
const TCP_REPAIR_OPTIONS: crate::c_int = 22i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:947`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L947)*

### `TCP_FASTOPEN`
```rust
const TCP_FASTOPEN: crate::c_int = 23i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:948`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L948)*

### `TCP_TIMESTAMP`
```rust
const TCP_TIMESTAMP: crate::c_int = 24i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:949`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L949)*

### `TCP_NOTSENT_LOWAT`
```rust
const TCP_NOTSENT_LOWAT: crate::c_int = 25i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:950`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L950)*

### `TCP_CC_INFO`
```rust
const TCP_CC_INFO: crate::c_int = 26i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:951`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L951)*

### `TCP_SAVE_SYN`
```rust
const TCP_SAVE_SYN: crate::c_int = 27i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:952`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L952)*

### `TCP_SAVED_SYN`
```rust
const TCP_SAVED_SYN: crate::c_int = 28i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:953`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L953)*

### `TCP_REPAIR_WINDOW`
```rust
const TCP_REPAIR_WINDOW: crate::c_int = 29i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:958`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L958)*

### `TCP_FASTOPEN_CONNECT`
```rust
const TCP_FASTOPEN_CONNECT: crate::c_int = 30i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:959`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L959)*

### `TCP_ULP`
```rust
const TCP_ULP: crate::c_int = 31i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:960`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L960)*

### `TCP_MD5SIG_EXT`
```rust
const TCP_MD5SIG_EXT: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:961`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L961)*

### `TCP_FASTOPEN_KEY`
```rust
const TCP_FASTOPEN_KEY: crate::c_int = 33i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:962`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L962)*

### `TCP_FASTOPEN_NO_COOKIE`
```rust
const TCP_FASTOPEN_NO_COOKIE: crate::c_int = 34i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:963`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L963)*

### `TCP_ZEROCOPY_RECEIVE`
```rust
const TCP_ZEROCOPY_RECEIVE: crate::c_int = 35i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:964`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L964)*

### `TCP_INQ`
```rust
const TCP_INQ: crate::c_int = 36i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:965`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L965)*

### `TCP_CM_INQ`
```rust
const TCP_CM_INQ: crate::c_int = 36i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:966`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L966)*

### `TCP_MD5SIG_MAXKEYLEN`
```rust
const TCP_MD5SIG_MAXKEYLEN: usize = 80usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:969`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L969)*

### `SO_DEBUG`
```rust
const SO_DEBUG: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:973`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L973)*

### `SHUT_RD`
```rust
const SHUT_RD: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:975`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L975)*

### `SHUT_WR`
```rust
const SHUT_WR: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:976`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L976)*

### `SHUT_RDWR`
```rust
const SHUT_RDWR: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:977`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L977)*

### `LOCK_SH`
```rust
const LOCK_SH: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:979`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L979)*

### `LOCK_EX`
```rust
const LOCK_EX: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:980`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L980)*

### `LOCK_NB`
```rust
const LOCK_NB: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:981`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L981)*

### `LOCK_UN`
```rust
const LOCK_UN: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:982`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L982)*

### `SS_ONSTACK`
```rust
const SS_ONSTACK: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:984`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L984)*

### `SS_DISABLE`
```rust
const SS_DISABLE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:985`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L985)*

### `PATH_MAX`
```rust
const PATH_MAX: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:987`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L987)*

### `UIO_MAXIOV`
```rust
const UIO_MAXIOV: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:989`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L989)*

### `FD_SETSIZE`
```rust
const FD_SETSIZE: usize = 1_024usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:991`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L991)*

### `EPOLLIN`
```rust
const EPOLLIN: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:993`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L993)*

### `EPOLLPRI`
```rust
const EPOLLPRI: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:994`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L994)*

### `EPOLLOUT`
```rust
const EPOLLOUT: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:995`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L995)*

### `EPOLLERR`
```rust
const EPOLLERR: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:996`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L996)*

### `EPOLLHUP`
```rust
const EPOLLHUP: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:997`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L997)*

### `EPOLLRDNORM`
```rust
const EPOLLRDNORM: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:998`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L998)*

### `EPOLLRDBAND`
```rust
const EPOLLRDBAND: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:999`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L999)*

### `EPOLLWRNORM`
```rust
const EPOLLWRNORM: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1000`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1000)*

### `EPOLLWRBAND`
```rust
const EPOLLWRBAND: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1001`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1001)*

### `EPOLLMSG`
```rust
const EPOLLMSG: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1002`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1002)*

### `EPOLLRDHUP`
```rust
const EPOLLRDHUP: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1003`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1003)*

### `EPOLLEXCLUSIVE`
```rust
const EPOLLEXCLUSIVE: crate::c_int = 268_435_456i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1004`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1004)*

### `EPOLLWAKEUP`
```rust
const EPOLLWAKEUP: crate::c_int = 536_870_912i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1005`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1005)*

### `EPOLLONESHOT`
```rust
const EPOLLONESHOT: crate::c_int = 1_073_741_824i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1006`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1006)*

### `EPOLLET`
```rust
const EPOLLET: crate::c_int = -2_147_483_648i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1007`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1007)*

### `EPOLL_CTL_ADD`
```rust
const EPOLL_CTL_ADD: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1009`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1009)*

### `EPOLL_CTL_MOD`
```rust
const EPOLL_CTL_MOD: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1010`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1010)*

### `EPOLL_CTL_DEL`
```rust
const EPOLL_CTL_DEL: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1011`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1011)*

### `MNT_FORCE`
```rust
const MNT_FORCE: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1013`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1013)*

### `MNT_DETACH`
```rust
const MNT_DETACH: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1014`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1014)*

### `MNT_EXPIRE`
```rust
const MNT_EXPIRE: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1015`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1015)*

### `UMOUNT_NOFOLLOW`
```rust
const UMOUNT_NOFOLLOW: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1016`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1016)*

### `Q_GETFMT`
```rust
const Q_GETFMT: crate::c_int = 8_388_612i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1018`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1018)*

### `Q_GETINFO`
```rust
const Q_GETINFO: crate::c_int = 8_388_613i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1019`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1019)*

### `Q_SETINFO`
```rust
const Q_SETINFO: crate::c_int = 8_388_614i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1020`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1020)*

### `QIF_BLIMITS`
```rust
const QIF_BLIMITS: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1021`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1021)*

### `QIF_SPACE`
```rust
const QIF_SPACE: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1022`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1022)*

### `QIF_ILIMITS`
```rust
const QIF_ILIMITS: u32 = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1023`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1023)*

### `QIF_INODES`
```rust
const QIF_INODES: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1024`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1024)*

### `QIF_BTIME`
```rust
const QIF_BTIME: u32 = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1025`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1025)*

### `QIF_ITIME`
```rust
const QIF_ITIME: u32 = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1026`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1026)*

### `QIF_LIMITS`
```rust
const QIF_LIMITS: u32 = 5u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1027`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1027)*

### `QIF_USAGE`
```rust
const QIF_USAGE: u32 = 10u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1028`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1028)*

### `QIF_TIMES`
```rust
const QIF_TIMES: u32 = 48u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1029`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1029)*

### `QIF_ALL`
```rust
const QIF_ALL: u32 = 63u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1030`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1030)*

### `Q_SYNC`
```rust
const Q_SYNC: crate::c_int = 8_388_609i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1032`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1032)*

### `Q_QUOTAON`
```rust
const Q_QUOTAON: crate::c_int = 8_388_610i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1033`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1033)*

### `Q_QUOTAOFF`
```rust
const Q_QUOTAOFF: crate::c_int = 8_388_611i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1034`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1034)*

### `Q_GETQUOTA`
```rust
const Q_GETQUOTA: crate::c_int = 8_388_615i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1035`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1035)*

### `Q_SETQUOTA`
```rust
const Q_SETQUOTA: crate::c_int = 8_388_616i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1036`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1036)*

### `TCIOFF`
```rust
const TCIOFF: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1038`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1038)*

### `TCION`
```rust
const TCION: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1039`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1039)*

### `TCOOFF`
```rust
const TCOOFF: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1040`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1040)*

### `TCOON`
```rust
const TCOON: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1041`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1041)*

### `TCIFLUSH`
```rust
const TCIFLUSH: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1042`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1042)*

### `TCOFLUSH`
```rust
const TCOFLUSH: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1043`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1043)*

### `TCIOFLUSH`
```rust
const TCIOFLUSH: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1044`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1044)*

### `NL0`
```rust
const NL0: crate::tcflag_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1045`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1045)*

### `NL1`
```rust
const NL1: crate::tcflag_t = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1046`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1046)*

### `TAB0`
```rust
const TAB0: crate::tcflag_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1047`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1047)*

### `CR0`
```rust
const CR0: crate::tcflag_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1048`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1048)*

### `FF0`
```rust
const FF0: crate::tcflag_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1049`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1049)*

### `BS0`
```rust
const BS0: crate::tcflag_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1050`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1050)*

### `VT0`
```rust
const VT0: crate::tcflag_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1051`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1051)*

### `VERASE`
```rust
const VERASE: usize = 2usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1052`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1052)*

### `VKILL`
```rust
const VKILL: usize = 3usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1053`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1053)*

### `VINTR`
```rust
const VINTR: usize = 0usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1054`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1054)*

### `VQUIT`
```rust
const VQUIT: usize = 1usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1055`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1055)*

### `VLNEXT`
```rust
const VLNEXT: usize = 15usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1056`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1056)*

### `IGNBRK`
```rust
const IGNBRK: crate::tcflag_t = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1057`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1057)*

### `BRKINT`
```rust
const BRKINT: crate::tcflag_t = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1058`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1058)*

### `IGNPAR`
```rust
const IGNPAR: crate::tcflag_t = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1059`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1059)*

### `PARMRK`
```rust
const PARMRK: crate::tcflag_t = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1060`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1060)*

### `INPCK`
```rust
const INPCK: crate::tcflag_t = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1061`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1061)*

### `ISTRIP`
```rust
const ISTRIP: crate::tcflag_t = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1062`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1062)*

### `INLCR`
```rust
const INLCR: crate::tcflag_t = 64u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1063`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1063)*

### `IGNCR`
```rust
const IGNCR: crate::tcflag_t = 128u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1064`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1064)*

### `ICRNL`
```rust
const ICRNL: crate::tcflag_t = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1065`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1065)*

### `IXANY`
```rust
const IXANY: crate::tcflag_t = 2_048u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1066`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1066)*

### `IMAXBEL`
```rust
const IMAXBEL: crate::tcflag_t = 8_192u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1067`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1067)*

### `OPOST`
```rust
const OPOST: crate::tcflag_t = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1068`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1068)*

### `CS5`
```rust
const CS5: crate::tcflag_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1069`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1069)*

### `CRTSCTS`
```rust
const CRTSCTS: crate::tcflag_t = 2_147_483_648u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1070`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1070)*

### `ECHO`
```rust
const ECHO: crate::tcflag_t = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1071`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1071)*

### `OCRNL`
```rust
const OCRNL: crate::tcflag_t = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1072`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1072)*

### `ONOCR`
```rust
const ONOCR: crate::tcflag_t = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1073`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1073)*

### `ONLRET`
```rust
const ONLRET: crate::tcflag_t = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1074`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1074)*

### `OFILL`
```rust
const OFILL: crate::tcflag_t = 64u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1075`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1075)*

### `OFDEL`
```rust
const OFDEL: crate::tcflag_t = 128u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1076`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1076)*

### `CLONE_VM`
```rust
const CLONE_VM: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1078`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1078)*

### `CLONE_FS`
```rust
const CLONE_FS: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1079`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1079)*

### `CLONE_FILES`
```rust
const CLONE_FILES: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1080`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1080)*

### `CLONE_SIGHAND`
```rust
const CLONE_SIGHAND: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1081`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1081)*

### `CLONE_PTRACE`
```rust
const CLONE_PTRACE: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1082`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1082)*

### `CLONE_VFORK`
```rust
const CLONE_VFORK: crate::c_int = 16_384i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1083`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1083)*

### `CLONE_PARENT`
```rust
const CLONE_PARENT: crate::c_int = 32_768i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1084`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1084)*

### `CLONE_THREAD`
```rust
const CLONE_THREAD: crate::c_int = 65_536i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1085`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1085)*

### `CLONE_NEWNS`
```rust
const CLONE_NEWNS: crate::c_int = 131_072i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1086`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1086)*

### `CLONE_SYSVSEM`
```rust
const CLONE_SYSVSEM: crate::c_int = 262_144i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1087`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1087)*

### `CLONE_SETTLS`
```rust
const CLONE_SETTLS: crate::c_int = 524_288i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1088`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1088)*

### `CLONE_PARENT_SETTID`
```rust
const CLONE_PARENT_SETTID: crate::c_int = 1_048_576i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1089`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1089)*

### `CLONE_CHILD_CLEARTID`
```rust
const CLONE_CHILD_CLEARTID: crate::c_int = 2_097_152i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1090`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1090)*

### `CLONE_DETACHED`
```rust
const CLONE_DETACHED: crate::c_int = 4_194_304i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1091`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1091)*

### `CLONE_UNTRACED`
```rust
const CLONE_UNTRACED: crate::c_int = 8_388_608i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1092`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1092)*

### `CLONE_CHILD_SETTID`
```rust
const CLONE_CHILD_SETTID: crate::c_int = 16_777_216i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1093`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1093)*

### `CLONE_NEWCGROUP`
```rust
const CLONE_NEWCGROUP: crate::c_int = 33_554_432i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1094`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1094)*

### `CLONE_NEWUTS`
```rust
const CLONE_NEWUTS: crate::c_int = 67_108_864i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1095`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1095)*

### `CLONE_NEWIPC`
```rust
const CLONE_NEWIPC: crate::c_int = 134_217_728i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1096`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1096)*

### `CLONE_NEWUSER`
```rust
const CLONE_NEWUSER: crate::c_int = 268_435_456i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1097`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1097)*

### `CLONE_NEWPID`
```rust
const CLONE_NEWPID: crate::c_int = 536_870_912i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1098`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1098)*

### `CLONE_NEWNET`
```rust
const CLONE_NEWNET: crate::c_int = 1_073_741_824i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1099`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1099)*

### `CLONE_IO`
```rust
const CLONE_IO: crate::c_int = -2_147_483_648i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1100`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1100)*

### `WNOHANG`
```rust
const WNOHANG: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1102`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1102)*

### `WUNTRACED`
```rust
const WUNTRACED: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1103`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1103)*

### `WSTOPPED`
```rust
const WSTOPPED: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1104`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1104)*

### `WEXITED`
```rust
const WEXITED: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1105`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1105)*

### `WCONTINUED`
```rust
const WCONTINUED: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1106`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1106)*

### `WNOWAIT`
```rust
const WNOWAIT: crate::c_int = 16_777_216i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1107`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1107)*

### `ADDR_NO_RANDOMIZE`
```rust
const ADDR_NO_RANDOMIZE: crate::c_int = 262_144i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1110`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1110)*

### `MMAP_PAGE_ZERO`
```rust
const MMAP_PAGE_ZERO: crate::c_int = 1_048_576i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1111`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1111)*

### `ADDR_COMPAT_LAYOUT`
```rust
const ADDR_COMPAT_LAYOUT: crate::c_int = 2_097_152i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1112`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1112)*

### `READ_IMPLIES_EXEC`
```rust
const READ_IMPLIES_EXEC: crate::c_int = 4_194_304i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1113`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1113)*

### `ADDR_LIMIT_32BIT`
```rust
const ADDR_LIMIT_32BIT: crate::c_int = 8_388_608i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1114`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1114)*

### `SHORT_INODE`
```rust
const SHORT_INODE: crate::c_int = 16_777_216i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1115`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1115)*

### `WHOLE_SECONDS`
```rust
const WHOLE_SECONDS: crate::c_int = 33_554_432i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1116`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1116)*

### `STICKY_TIMEOUTS`
```rust
const STICKY_TIMEOUTS: crate::c_int = 67_108_864i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1117`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1117)*

### `ADDR_LIMIT_3GB`
```rust
const ADDR_LIMIT_3GB: crate::c_int = 134_217_728i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1118`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1118)*

### `PTRACE_O_TRACESYSGOOD`
```rust
const PTRACE_O_TRACESYSGOOD: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1121`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1121)*

### `PTRACE_O_TRACEFORK`
```rust
const PTRACE_O_TRACEFORK: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1122`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1122)*

### `PTRACE_O_TRACEVFORK`
```rust
const PTRACE_O_TRACEVFORK: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1123`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1123)*

### `PTRACE_O_TRACECLONE`
```rust
const PTRACE_O_TRACECLONE: crate::c_int = 8i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1124`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1124)*

### `PTRACE_O_TRACEEXEC`
```rust
const PTRACE_O_TRACEEXEC: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1125`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1125)*

### `PTRACE_O_TRACEVFORKDONE`
```rust
const PTRACE_O_TRACEVFORKDONE: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1126`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1126)*

### `PTRACE_O_TRACEEXIT`
```rust
const PTRACE_O_TRACEEXIT: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1127`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1127)*

### `PTRACE_O_TRACESECCOMP`
```rust
const PTRACE_O_TRACESECCOMP: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1128`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1128)*

### `PTRACE_O_SUSPEND_SECCOMP`
```rust
const PTRACE_O_SUSPEND_SECCOMP: crate::c_int = 2_097_152i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1129`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1129)*

### `PTRACE_O_EXITKILL`
```rust
const PTRACE_O_EXITKILL: crate::c_int = 1_048_576i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1130`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1130)*

### `PTRACE_O_MASK`
```rust
const PTRACE_O_MASK: crate::c_int = 3_145_983i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1131`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1131)*

### `PTRACE_EVENT_FORK`
```rust
const PTRACE_EVENT_FORK: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1134`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1134)*

### `PTRACE_EVENT_VFORK`
```rust
const PTRACE_EVENT_VFORK: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1135`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1135)*

### `PTRACE_EVENT_CLONE`
```rust
const PTRACE_EVENT_CLONE: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1136`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1136)*

### `PTRACE_EVENT_EXEC`
```rust
const PTRACE_EVENT_EXEC: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1137`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1137)*

### `PTRACE_EVENT_VFORK_DONE`
```rust
const PTRACE_EVENT_VFORK_DONE: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1138`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1138)*

### `PTRACE_EVENT_EXIT`
```rust
const PTRACE_EVENT_EXIT: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1139`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1139)*

### `PTRACE_EVENT_SECCOMP`
```rust
const PTRACE_EVENT_SECCOMP: crate::c_int = 7i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1140`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1140)*

### `__WNOTHREAD`
```rust
const __WNOTHREAD: crate::c_int = 536_870_912i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1142`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1142)*

### `__WALL`
```rust
const __WALL: crate::c_int = 1_073_741_824i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1143`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1143)*

### `__WCLONE`
```rust
const __WCLONE: crate::c_int = -2_147_483_648i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1144`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1144)*

### `SPLICE_F_MOVE`
```rust
const SPLICE_F_MOVE: crate::c_uint = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1146`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1146)*

### `SPLICE_F_NONBLOCK`
```rust
const SPLICE_F_NONBLOCK: crate::c_uint = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1147`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1147)*

### `SPLICE_F_MORE`
```rust
const SPLICE_F_MORE: crate::c_uint = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1148`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1148)*

### `SPLICE_F_GIFT`
```rust
const SPLICE_F_GIFT: crate::c_uint = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1149`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1149)*

### `RTLD_LOCAL`
```rust
const RTLD_LOCAL: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1151`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1151)*

### `RTLD_LAZY`
```rust
const RTLD_LAZY: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1152`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1152)*

### `POSIX_FADV_NORMAL`
```rust
const POSIX_FADV_NORMAL: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1154`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1154)*

### `POSIX_FADV_RANDOM`
```rust
const POSIX_FADV_RANDOM: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1155`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1155)*

### `POSIX_FADV_SEQUENTIAL`
```rust
const POSIX_FADV_SEQUENTIAL: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1156`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1156)*

### `POSIX_FADV_WILLNEED`
```rust
const POSIX_FADV_WILLNEED: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1157`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1157)*

### `AT_FDCWD`
```rust
const AT_FDCWD: crate::c_int = -100i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1159`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1159)*

### `AT_SYMLINK_NOFOLLOW`
```rust
const AT_SYMLINK_NOFOLLOW: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1160`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1160)*

### `AT_REMOVEDIR`
```rust
const AT_REMOVEDIR: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1161`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1161)*

### `AT_SYMLINK_FOLLOW`
```rust
const AT_SYMLINK_FOLLOW: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1162`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1162)*

### `AT_NO_AUTOMOUNT`
```rust
const AT_NO_AUTOMOUNT: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1163`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1163)*

### `AT_EMPTY_PATH`
```rust
const AT_EMPTY_PATH: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1164`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1164)*

### `AT_RECURSIVE`
```rust
const AT_RECURSIVE: crate::c_int = 32_768i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1165`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1165)*

### `LOG_CRON`
```rust
const LOG_CRON: crate::c_int = 72i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1167`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1167)*

### `LOG_AUTHPRIV`
```rust
const LOG_AUTHPRIV: crate::c_int = 80i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1168`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1168)*

### `LOG_FTP`
```rust
const LOG_FTP: crate::c_int = 88i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1169`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1169)*

### `LOG_PERROR`
```rust
const LOG_PERROR: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1170`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1170)*

### `PIPE_BUF`
```rust
const PIPE_BUF: usize = 4_096usize;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1172`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1172)*

### `SI_LOAD_SHIFT`
```rust
const SI_LOAD_SHIFT: crate::c_uint = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1174`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1174)*

### `SI_USER`
```rust
const SI_USER: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1177`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1177)*

### `SI_KERNEL`
```rust
const SI_KERNEL: crate::c_int = 128i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1178`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1178)*

### `SI_QUEUE`
```rust
const SI_QUEUE: crate::c_int = -1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1179`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1179)*

### `SI_TIMER`
```rust
const SI_TIMER: crate::c_int = -2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1186`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1186)*

### `SI_MESGQ`
```rust
const SI_MESGQ: crate::c_int = -3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1187`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1187)*

### `SI_ASYNCIO`
```rust
const SI_ASYNCIO: crate::c_int = -4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1188`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1188)*

### `SI_SIGIO`
```rust
const SI_SIGIO: crate::c_int = -5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1195`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1195)*

### `SI_TKILL`
```rust
const SI_TKILL: crate::c_int = -6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1196`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1196)*

### `SI_ASYNCNL`
```rust
const SI_ASYNCNL: crate::c_int = -60i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1197`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1197)*

### `BUS_ADRALN`
```rust
const BUS_ADRALN: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1200`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1200)*

### `BUS_ADRERR`
```rust
const BUS_ADRERR: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1201`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1201)*

### `BUS_OBJERR`
```rust
const BUS_OBJERR: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1202`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1202)*

### `BUS_MCEERR_AR`
```rust
const BUS_MCEERR_AR: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1204`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1204)*

### `BUS_MCEERR_AO`
```rust
const BUS_MCEERR_AO: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1205`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1205)*

### `TRAP_BRKPT`
```rust
const TRAP_BRKPT: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1208`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1208)*

### `TRAP_TRACE`
```rust
const TRAP_TRACE: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1209`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1209)*

### `TRAP_BRANCH`
```rust
const TRAP_BRANCH: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1210`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1210)*

### `TRAP_HWBKPT`
```rust
const TRAP_HWBKPT: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1211`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1211)*

### `TRAP_UNK`
```rust
const TRAP_UNK: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1212`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1212)*

### `CLD_EXITED`
```rust
const CLD_EXITED: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1215`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1215)*

### `CLD_KILLED`
```rust
const CLD_KILLED: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1216`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1216)*

### `CLD_DUMPED`
```rust
const CLD_DUMPED: crate::c_int = 3i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1217`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1217)*

### `CLD_TRAPPED`
```rust
const CLD_TRAPPED: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1218`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1218)*

### `CLD_STOPPED`
```rust
const CLD_STOPPED: crate::c_int = 5i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1219`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1219)*

### `CLD_CONTINUED`
```rust
const CLD_CONTINUED: crate::c_int = 6i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1220`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1220)*

### `SIGEV_SIGNAL`
```rust
const SIGEV_SIGNAL: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1222`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1222)*

### `SIGEV_NONE`
```rust
const SIGEV_NONE: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1223`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1223)*

### `SIGEV_THREAD`
```rust
const SIGEV_THREAD: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1224`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1224)*

### `P_ALL`
```rust
const P_ALL: idtype_t = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1226`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1226)*

### `P_PID`
```rust
const P_PID: idtype_t = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1227`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1227)*

### `P_PGID`
```rust
const P_PGID: idtype_t = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1228`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1228)*

### `P_PIDFD`
```rust
const P_PIDFD: idtype_t = 3u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1231`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1231)*

### `UTIME_OMIT`
```rust
const UTIME_OMIT: crate::c_long = 1_073_741_822i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1235`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1235)*

### `UTIME_NOW`
```rust
const UTIME_NOW: crate::c_long = 1_073_741_823i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1236`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1236)*

### `POLLIN`
```rust
const POLLIN: crate::c_short = 1i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1238`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1238)*

### `POLLPRI`
```rust
const POLLPRI: crate::c_short = 2i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1239`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1239)*

### `POLLOUT`
```rust
const POLLOUT: crate::c_short = 4i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1240`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1240)*

### `POLLERR`
```rust
const POLLERR: crate::c_short = 8i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1241`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1241)*

### `POLLHUP`
```rust
const POLLHUP: crate::c_short = 16i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1242`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1242)*

### `POLLNVAL`
```rust
const POLLNVAL: crate::c_short = 32i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1243`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1243)*

### `POLLRDNORM`
```rust
const POLLRDNORM: crate::c_short = 64i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1244`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1244)*

### `POLLRDBAND`
```rust
const POLLRDBAND: crate::c_short = 128i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1245`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1245)*

### `POLLRDHUP`
```rust
const POLLRDHUP: crate::c_short = 8_192i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1247`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1247)*

### `IPTOS_LOWDELAY`
```rust
const IPTOS_LOWDELAY: u8 = 16u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1251`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1251)*

### `IPTOS_THROUGHPUT`
```rust
const IPTOS_THROUGHPUT: u8 = 8u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1252`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1252)*

### `IPTOS_RELIABILITY`
```rust
const IPTOS_RELIABILITY: u8 = 4u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1253`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1253)*

### `IPTOS_MINCOST`
```rust
const IPTOS_MINCOST: u8 = 2u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1254`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1254)*

### `IPTOS_PREC_NETCONTROL`
```rust
const IPTOS_PREC_NETCONTROL: u8 = 224u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1256`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1256)*

### `IPTOS_PREC_INTERNETCONTROL`
```rust
const IPTOS_PREC_INTERNETCONTROL: u8 = 192u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1257`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1257)*

### `IPTOS_PREC_CRITIC_ECP`
```rust
const IPTOS_PREC_CRITIC_ECP: u8 = 160u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1258`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1258)*

### `IPTOS_PREC_FLASHOVERRIDE`
```rust
const IPTOS_PREC_FLASHOVERRIDE: u8 = 128u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1259`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1259)*

### `IPTOS_PREC_FLASH`
```rust
const IPTOS_PREC_FLASH: u8 = 96u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1260`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1260)*

### `IPTOS_PREC_IMMEDIATE`
```rust
const IPTOS_PREC_IMMEDIATE: u8 = 64u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1261`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1261)*

### `IPTOS_PREC_PRIORITY`
```rust
const IPTOS_PREC_PRIORITY: u8 = 32u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1262`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1262)*

### `IPTOS_PREC_ROUTINE`
```rust
const IPTOS_PREC_ROUTINE: u8 = 0u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1263`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1263)*

### `IPTOS_ECN_MASK`
```rust
const IPTOS_ECN_MASK: u8 = 3u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1265`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1265)*

### `IPTOS_ECN_ECT1`
```rust
const IPTOS_ECN_ECT1: u8 = 1u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1266`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1266)*

### `IPTOS_ECN_ECT0`
```rust
const IPTOS_ECN_ECT0: u8 = 2u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1267`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1267)*

### `IPTOS_ECN_CE`
```rust
const IPTOS_ECN_CE: u8 = 3u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1268`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1268)*

### `IPOPT_COPY`
```rust
const IPOPT_COPY: u8 = 128u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1270`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1270)*

### `IPOPT_CLASS_MASK`
```rust
const IPOPT_CLASS_MASK: u8 = 96u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1271`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1271)*

### `IPOPT_NUMBER_MASK`
```rust
const IPOPT_NUMBER_MASK: u8 = 31u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1272`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1272)*

### `IPOPT_CONTROL`
```rust
const IPOPT_CONTROL: u8 = 0u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1274`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1274)*

### `IPOPT_RESERVED1`
```rust
const IPOPT_RESERVED1: u8 = 32u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1275`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1275)*

### `IPOPT_MEASUREMENT`
```rust
const IPOPT_MEASUREMENT: u8 = 64u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1276`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1276)*

### `IPOPT_RESERVED2`
```rust
const IPOPT_RESERVED2: u8 = 96u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1277`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1277)*

### `IPOPT_END`
```rust
const IPOPT_END: u8 = 0u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1278`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1278)*

### `IPOPT_NOOP`
```rust
const IPOPT_NOOP: u8 = 1u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1279`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1279)*

### `IPOPT_SEC`
```rust
const IPOPT_SEC: u8 = 130u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1280`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1280)*

### `IPOPT_LSRR`
```rust
const IPOPT_LSRR: u8 = 131u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1281`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1281)*

### `IPOPT_TIMESTAMP`
```rust
const IPOPT_TIMESTAMP: u8 = 68u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1282`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1282)*

### `IPOPT_RR`
```rust
const IPOPT_RR: u8 = 7u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1283`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1283)*

### `IPOPT_SID`
```rust
const IPOPT_SID: u8 = 136u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1284`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1284)*

### `IPOPT_SSRR`
```rust
const IPOPT_SSRR: u8 = 137u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1285`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1285)*

### `IPOPT_RA`
```rust
const IPOPT_RA: u8 = 148u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1286`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1286)*

### `IPVERSION`
```rust
const IPVERSION: u8 = 4u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1287`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1287)*

### `MAXTTL`
```rust
const MAXTTL: u8 = 255u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1288`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1288)*

### `IPDEFTTL`
```rust
const IPDEFTTL: u8 = 64u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1289`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1289)*

### `IPOPT_OPTVAL`
```rust
const IPOPT_OPTVAL: u8 = 0u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1290`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1290)*

### `IPOPT_OLEN`
```rust
const IPOPT_OLEN: u8 = 1u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1291`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1291)*

### `IPOPT_OFFSET`
```rust
const IPOPT_OFFSET: u8 = 2u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1292`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1292)*

### `IPOPT_MINOFF`
```rust
const IPOPT_MINOFF: u8 = 4u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1293`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1293)*

### `MAX_IPOPTLEN`
```rust
const MAX_IPOPTLEN: u8 = 40u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1294`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1294)*

### `IPOPT_NOP`
```rust
const IPOPT_NOP: u8 = 1u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1295`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1295)*

### `IPOPT_EOL`
```rust
const IPOPT_EOL: u8 = 0u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1296`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1296)*

### `IPOPT_TS`
```rust
const IPOPT_TS: u8 = 68u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1297`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1297)*

### `IPOPT_TS_TSONLY`
```rust
const IPOPT_TS_TSONLY: u8 = 0u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1298`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1298)*

### `IPOPT_TS_TSANDADDR`
```rust
const IPOPT_TS_TSANDADDR: u8 = 1u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1299`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1299)*

### `IPOPT_TS_PRESPEC`
```rust
const IPOPT_TS_PRESPEC: u8 = 3u8;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1300`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1300)*

### `ARPOP_RREQUEST`
```rust
const ARPOP_RREQUEST: u16 = 3u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1302`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1302)*

### `ARPOP_RREPLY`
```rust
const ARPOP_RREPLY: u16 = 4u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1303`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1303)*

### `ARPOP_InREQUEST`
```rust
const ARPOP_InREQUEST: u16 = 8u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1304`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1304)*

### `ARPOP_InREPLY`
```rust
const ARPOP_InREPLY: u16 = 9u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1305`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1305)*

### `ARPOP_NAK`
```rust
const ARPOP_NAK: u16 = 10u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1306`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1306)*

### `ATF_NETMASK`
```rust
const ATF_NETMASK: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1308`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1308)*

### `ATF_DONTPUB`
```rust
const ATF_DONTPUB: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1309`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1309)*

### `ARPHRD_NETROM`
```rust
const ARPHRD_NETROM: u16 = 0u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1311`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1311)*

### `ARPHRD_ETHER`
```rust
const ARPHRD_ETHER: u16 = 1u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1312`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1312)*

### `ARPHRD_EETHER`
```rust
const ARPHRD_EETHER: u16 = 2u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1313`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1313)*

### `ARPHRD_AX25`
```rust
const ARPHRD_AX25: u16 = 3u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1314`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1314)*

### `ARPHRD_PRONET`
```rust
const ARPHRD_PRONET: u16 = 4u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1315`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1315)*

### `ARPHRD_CHAOS`
```rust
const ARPHRD_CHAOS: u16 = 5u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1316`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1316)*

### `ARPHRD_IEEE802`
```rust
const ARPHRD_IEEE802: u16 = 6u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1317`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1317)*

### `ARPHRD_ARCNET`
```rust
const ARPHRD_ARCNET: u16 = 7u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1318`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1318)*

### `ARPHRD_APPLETLK`
```rust
const ARPHRD_APPLETLK: u16 = 8u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1319`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1319)*

### `ARPHRD_DLCI`
```rust
const ARPHRD_DLCI: u16 = 15u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1320`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1320)*

### `ARPHRD_ATM`
```rust
const ARPHRD_ATM: u16 = 19u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1321`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1321)*

### `ARPHRD_METRICOM`
```rust
const ARPHRD_METRICOM: u16 = 23u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1322`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1322)*

### `ARPHRD_IEEE1394`
```rust
const ARPHRD_IEEE1394: u16 = 24u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1323`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1323)*

### `ARPHRD_EUI64`
```rust
const ARPHRD_EUI64: u16 = 27u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1324`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1324)*

### `ARPHRD_INFINIBAND`
```rust
const ARPHRD_INFINIBAND: u16 = 32u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1325`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1325)*

### `ARPHRD_SLIP`
```rust
const ARPHRD_SLIP: u16 = 256u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1327`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1327)*

### `ARPHRD_CSLIP`
```rust
const ARPHRD_CSLIP: u16 = 257u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1328`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1328)*

### `ARPHRD_SLIP6`
```rust
const ARPHRD_SLIP6: u16 = 258u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1329`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1329)*

### `ARPHRD_CSLIP6`
```rust
const ARPHRD_CSLIP6: u16 = 259u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1330`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1330)*

### `ARPHRD_RSRVD`
```rust
const ARPHRD_RSRVD: u16 = 260u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1331`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1331)*

### `ARPHRD_ADAPT`
```rust
const ARPHRD_ADAPT: u16 = 264u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1332`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1332)*

### `ARPHRD_ROSE`
```rust
const ARPHRD_ROSE: u16 = 270u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1333`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1333)*

### `ARPHRD_X25`
```rust
const ARPHRD_X25: u16 = 271u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1334`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1334)*

### `ARPHRD_HWX25`
```rust
const ARPHRD_HWX25: u16 = 272u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1335`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1335)*

### `ARPHRD_CAN`
```rust
const ARPHRD_CAN: u16 = 280u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1336`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1336)*

### `ARPHRD_PPP`
```rust
const ARPHRD_PPP: u16 = 512u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1337`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1337)*

### `ARPHRD_CISCO`
```rust
const ARPHRD_CISCO: u16 = 513u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1338`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1338)*

### `ARPHRD_HDLC`
```rust
const ARPHRD_HDLC: u16 = 513u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1339`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1339)*

### `ARPHRD_LAPB`
```rust
const ARPHRD_LAPB: u16 = 516u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1340`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1340)*

### `ARPHRD_DDCMP`
```rust
const ARPHRD_DDCMP: u16 = 517u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1341`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1341)*

### `ARPHRD_RAWHDLC`
```rust
const ARPHRD_RAWHDLC: u16 = 518u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1342`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1342)*

### `ARPHRD_TUNNEL`
```rust
const ARPHRD_TUNNEL: u16 = 768u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1344`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1344)*

### `ARPHRD_TUNNEL6`
```rust
const ARPHRD_TUNNEL6: u16 = 769u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1345`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1345)*

### `ARPHRD_FRAD`
```rust
const ARPHRD_FRAD: u16 = 770u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1346`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1346)*

### `ARPHRD_SKIP`
```rust
const ARPHRD_SKIP: u16 = 771u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1347`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1347)*

### `ARPHRD_LOOPBACK`
```rust
const ARPHRD_LOOPBACK: u16 = 772u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1348`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1348)*

### `ARPHRD_LOCALTLK`
```rust
const ARPHRD_LOCALTLK: u16 = 773u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1349`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1349)*

### `ARPHRD_FDDI`
```rust
const ARPHRD_FDDI: u16 = 774u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1350`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1350)*

### `ARPHRD_BIF`
```rust
const ARPHRD_BIF: u16 = 775u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1351`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1351)*

### `ARPHRD_SIT`
```rust
const ARPHRD_SIT: u16 = 776u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1352`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1352)*

### `ARPHRD_IPDDP`
```rust
const ARPHRD_IPDDP: u16 = 777u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1353`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1353)*

### `ARPHRD_IPGRE`
```rust
const ARPHRD_IPGRE: u16 = 778u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1354`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1354)*

### `ARPHRD_PIMREG`
```rust
const ARPHRD_PIMREG: u16 = 779u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1355`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1355)*

### `ARPHRD_HIPPI`
```rust
const ARPHRD_HIPPI: u16 = 780u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1356`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1356)*

### `ARPHRD_ASH`
```rust
const ARPHRD_ASH: u16 = 781u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1357`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1357)*

### `ARPHRD_ECONET`
```rust
const ARPHRD_ECONET: u16 = 782u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1358`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1358)*

### `ARPHRD_IRDA`
```rust
const ARPHRD_IRDA: u16 = 783u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1359`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1359)*

### `ARPHRD_FCPP`
```rust
const ARPHRD_FCPP: u16 = 784u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1360`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1360)*

### `ARPHRD_FCAL`
```rust
const ARPHRD_FCAL: u16 = 785u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1361`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1361)*

### `ARPHRD_FCPL`
```rust
const ARPHRD_FCPL: u16 = 786u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1362`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1362)*

### `ARPHRD_FCFABRIC`
```rust
const ARPHRD_FCFABRIC: u16 = 787u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1363`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1363)*

### `ARPHRD_IEEE802_TR`
```rust
const ARPHRD_IEEE802_TR: u16 = 800u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1364`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1364)*

### `ARPHRD_IEEE80211`
```rust
const ARPHRD_IEEE80211: u16 = 801u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1365`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1365)*

### `ARPHRD_IEEE80211_PRISM`
```rust
const ARPHRD_IEEE80211_PRISM: u16 = 802u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1366`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1366)*

### `ARPHRD_IEEE80211_RADIOTAP`
```rust
const ARPHRD_IEEE80211_RADIOTAP: u16 = 803u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1367`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1367)*

### `ARPHRD_IEEE802154`
```rust
const ARPHRD_IEEE802154: u16 = 804u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1368`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1368)*

### `ARPHRD_VOID`
```rust
const ARPHRD_VOID: u16 = 65_535u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1370`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1370)*

### `ARPHRD_NONE`
```rust
const ARPHRD_NONE: u16 = 65_534u16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1371`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1371)*

### `IFF_TUN`
```rust
const IFF_TUN: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1377`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1377)*

### `IFF_TAP`
```rust
const IFF_TAP: crate::c_int = 2i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1378`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1378)*

### `IFF_NAPI`
```rust
const IFF_NAPI: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1379`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1379)*

### `IFF_NAPI_FRAGS`
```rust
const IFF_NAPI_FRAGS: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1380`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1380)*

### `IFF_NO_CARRIER`
```rust
const IFF_NO_CARRIER: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1382`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1382)*

### `IFF_NO_PI`
```rust
const IFF_NO_PI: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1383`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1383)*

### `TUN_READQ_SIZE`
```rust
const TUN_READQ_SIZE: crate::c_short = 500i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1385`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1385)*

### `TUN_TUN_DEV`
```rust
const TUN_TUN_DEV: crate::c_short = 1i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1387`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1387)*

### `TUN_TAP_DEV`
```rust
const TUN_TAP_DEV: crate::c_short = 2i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1388`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1388)*

### `TUN_TYPE_MASK`
```rust
const TUN_TYPE_MASK: crate::c_short = 15i16;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1389`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1389)*

### `IFF_ONE_QUEUE`
```rust
const IFF_ONE_QUEUE: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1391`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1391)*

### `IFF_VNET_HDR`
```rust
const IFF_VNET_HDR: crate::c_int = 16_384i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1392`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1392)*

### `IFF_TUN_EXCL`
```rust
const IFF_TUN_EXCL: crate::c_int = 32_768i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1393`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1393)*

### `IFF_MULTI_QUEUE`
```rust
const IFF_MULTI_QUEUE: crate::c_int = 256i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1394`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1394)*

### `IFF_ATTACH_QUEUE`
```rust
const IFF_ATTACH_QUEUE: crate::c_int = 512i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1395`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1395)*

### `IFF_DETACH_QUEUE`
```rust
const IFF_DETACH_QUEUE: crate::c_int = 1_024i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1396`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1396)*

### `IFF_PERSIST`
```rust
const IFF_PERSIST: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1398`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1398)*

### `IFF_NOFILTER`
```rust
const IFF_NOFILTER: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1399`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1399)*

### `TUN_TX_TIMESTAMP`
```rust
const TUN_TX_TIMESTAMP: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1401`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1401)*

### `TUN_F_CSUM`
```rust
const TUN_F_CSUM: crate::c_uint = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1403`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1403)*

### `TUN_F_TSO4`
```rust
const TUN_F_TSO4: crate::c_uint = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1404`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1404)*

### `TUN_F_TSO6`
```rust
const TUN_F_TSO6: crate::c_uint = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1405`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1405)*

### `TUN_F_TSO_ECN`
```rust
const TUN_F_TSO_ECN: crate::c_uint = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1406`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1406)*

### `TUN_F_UFO`
```rust
const TUN_F_UFO: crate::c_uint = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1407`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1407)*

### `TUN_F_USO4`
```rust
const TUN_F_USO4: crate::c_uint = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1408`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1408)*

### `TUN_F_USO6`
```rust
const TUN_F_USO6: crate::c_uint = 64u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1409`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1409)*

### `TUN_PKT_STRIP`
```rust
const TUN_PKT_STRIP: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1411`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1411)*

### `TUN_FLT_ALLMULTI`
```rust
const TUN_FLT_ALLMULTI: crate::c_int = 1i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1413`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1413)*

### `T_TYPE`
```rust
const T_TYPE: u32 = 84u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1415`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1415)*

### `TUNSETNOCSUM`
```rust
const TUNSETNOCSUM: crate::c_ulong = 1_074_025_672u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1416`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1416)*

### `TUNSETDEBUG`
```rust
const TUNSETDEBUG: crate::c_ulong = 1_074_025_673u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1417`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1417)*

### `TUNSETIFF`
```rust
const TUNSETIFF: crate::c_ulong = 1_074_025_674u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1418`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1418)*

### `TUNSETPERSIST`
```rust
const TUNSETPERSIST: crate::c_ulong = 1_074_025_675u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1419`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1419)*

### `TUNSETOWNER`
```rust
const TUNSETOWNER: crate::c_ulong = 1_074_025_676u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1420`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1420)*

### `TUNSETLINK`
```rust
const TUNSETLINK: crate::c_ulong = 1_074_025_677u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1421`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1421)*

### `TUNSETGROUP`
```rust
const TUNSETGROUP: crate::c_ulong = 1_074_025_678u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1422`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1422)*

### `TUNGETFEATURES`
```rust
const TUNGETFEATURES: crate::c_ulong = 2_147_767_503u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1423`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1423)*

### `TUNSETOFFLOAD`
```rust
const TUNSETOFFLOAD: crate::c_ulong = 1_074_025_680u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1424`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1424)*

### `TUNSETTXFILTER`
```rust
const TUNSETTXFILTER: crate::c_ulong = 1_074_025_681u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1425`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1425)*

### `TUNGETIFF`
```rust
const TUNGETIFF: crate::c_ulong = 2_147_767_506u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1426`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1426)*

### `TUNGETSNDBUF`
```rust
const TUNGETSNDBUF: crate::c_ulong = 2_147_767_507u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1427`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1427)*

### `TUNSETSNDBUF`
```rust
const TUNSETSNDBUF: crate::c_ulong = 1_074_025_684u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1428`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1428)*

### `TUNATTACHFILTER`
```rust
const TUNATTACHFILTER: crate::c_ulong = 1_074_812_117u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1429`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1429)*

### `TUNDETACHFILTER`
```rust
const TUNDETACHFILTER: crate::c_ulong = 1_074_812_118u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1430`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1430)*

### `TUNGETVNETHDRSZ`
```rust
const TUNGETVNETHDRSZ: crate::c_ulong = 2_147_767_511u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1431`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1431)*

### `TUNSETVNETHDRSZ`
```rust
const TUNSETVNETHDRSZ: crate::c_ulong = 1_074_025_688u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1432`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1432)*

### `TUNSETQUEUE`
```rust
const TUNSETQUEUE: crate::c_ulong = 1_074_025_689u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1433`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1433)*

### `TUNSETIFINDEX`
```rust
const TUNSETIFINDEX: crate::c_ulong = 1_074_025_690u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1434`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1434)*

### `TUNGETFILTER`
```rust
const TUNGETFILTER: crate::c_ulong = 2_148_553_947u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1435`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1435)*

### `TUNSETVNETLE`
```rust
const TUNSETVNETLE: crate::c_ulong = 1_074_025_692u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1436`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1436)*

### `TUNGETVNETLE`
```rust
const TUNGETVNETLE: crate::c_ulong = 2_147_767_517u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1437`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1437)*

### `TUNSETVNETBE`
```rust
const TUNSETVNETBE: crate::c_ulong = 1_074_025_694u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1438`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1438)*

### `TUNGETVNETBE`
```rust
const TUNGETVNETBE: crate::c_ulong = 2_147_767_519u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1439`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1439)*

### `TUNSETSTEERINGEBPF`
```rust
const TUNSETSTEERINGEBPF: crate::c_ulong = 2_147_767_520u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1440`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1440)*

### `TUNSETFILTEREBPF`
```rust
const TUNSETFILTEREBPF: crate::c_ulong = 2_147_767_521u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1441`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1441)*

### `TUNSETCARRIER`
```rust
const TUNSETCARRIER: crate::c_ulong = 1_074_025_698u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1442`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1442)*

### `TUNGETDEVNETNS`
```rust
const TUNGETDEVNETNS: crate::c_ulong = 21_731u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1443`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1443)*

### `FS_IOC_GETFLAGS`
```rust
const FS_IOC_GETFLAGS: crate::c_ulong = 2_148_034_049u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1446`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1446)*

### `FS_IOC_SETFLAGS`
```rust
const FS_IOC_SETFLAGS: crate::c_ulong = 1_074_292_226u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1447`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1447)*

### `FS_IOC_GETVERSION`
```rust
const FS_IOC_GETVERSION: crate::c_ulong = 2_148_038_145u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1448`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1448)*

### `FS_IOC_SETVERSION`
```rust
const FS_IOC_SETVERSION: crate::c_ulong = 1_074_296_322u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1449`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1449)*

### `FS_IOC32_GETFLAGS`
```rust
const FS_IOC32_GETFLAGS: crate::c_ulong = 2_147_771_905u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1450`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1450)*

### `FS_IOC32_SETFLAGS`
```rust
const FS_IOC32_SETFLAGS: crate::c_ulong = 1_074_030_082u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1451`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1451)*

### `FS_IOC32_GETVERSION`
```rust
const FS_IOC32_GETVERSION: crate::c_ulong = 2_147_776_001u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1452`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1452)*

### `FS_IOC32_SETVERSION`
```rust
const FS_IOC32_SETVERSION: crate::c_ulong = 1_074_034_178u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1453`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1453)*

### `FICLONE`
```rust
const FICLONE: crate::c_ulong = 1_074_041_865u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1455`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1455)*

### `FICLONERANGE`
```rust
const FICLONERANGE: crate::c_ulong = 1_075_876_877u64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1456`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1456)*

### `ADFS_SUPER_MAGIC`
```rust
const ADFS_SUPER_MAGIC: crate::c_long = 44_533i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1464`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1464)*

### `AFFS_SUPER_MAGIC`
```rust
const AFFS_SUPER_MAGIC: crate::c_long = 44_543i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1465`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1465)*

### `AFS_SUPER_MAGIC`
```rust
const AFS_SUPER_MAGIC: crate::c_long = 1_397_113_167i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1466`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1466)*

### `AUTOFS_SUPER_MAGIC`
```rust
const AUTOFS_SUPER_MAGIC: crate::c_long = 391i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1467`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1467)*

### `BPF_FS_MAGIC`
```rust
const BPF_FS_MAGIC: crate::c_long = 3_405_662_737i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1468`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1468)*

### `BTRFS_SUPER_MAGIC`
```rust
const BTRFS_SUPER_MAGIC: crate::c_long = 2_435_016_766i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1469`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1469)*

### `CGROUP2_SUPER_MAGIC`
```rust
const CGROUP2_SUPER_MAGIC: crate::c_long = 1_667_723_888i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1470`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1470)*

### `CGROUP_SUPER_MAGIC`
```rust
const CGROUP_SUPER_MAGIC: crate::c_long = 2_613_483i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1471`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1471)*

### `CODA_SUPER_MAGIC`
```rust
const CODA_SUPER_MAGIC: crate::c_long = 1_937_076_805i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1472`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1472)*

### `CRAMFS_MAGIC`
```rust
const CRAMFS_MAGIC: crate::c_long = 684_539_205i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1473`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1473)*

### `DEBUGFS_MAGIC`
```rust
const DEBUGFS_MAGIC: crate::c_long = 1_684_170_528i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1474`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1474)*

### `DEVPTS_SUPER_MAGIC`
```rust
const DEVPTS_SUPER_MAGIC: crate::c_long = 7_377i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1475`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1475)*

### `ECRYPTFS_SUPER_MAGIC`
```rust
const ECRYPTFS_SUPER_MAGIC: crate::c_long = 61_791i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1476`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1476)*

### `EFS_SUPER_MAGIC`
```rust
const EFS_SUPER_MAGIC: crate::c_long = 4_278_867i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1477`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1477)*

### `EXT2_SUPER_MAGIC`
```rust
const EXT2_SUPER_MAGIC: crate::c_long = 61_267i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1478`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1478)*

### `EXT3_SUPER_MAGIC`
```rust
const EXT3_SUPER_MAGIC: crate::c_long = 61_267i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1479`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1479)*

### `EXT4_SUPER_MAGIC`
```rust
const EXT4_SUPER_MAGIC: crate::c_long = 61_267i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1480`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1480)*

### `F2FS_SUPER_MAGIC`
```rust
const F2FS_SUPER_MAGIC: crate::c_long = 4_076_150_800i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1481`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1481)*

### `FUSE_SUPER_MAGIC`
```rust
const FUSE_SUPER_MAGIC: crate::c_long = 1_702_057_286i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1482`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1482)*

### `FUTEXFS_SUPER_MAGIC`
```rust
const FUTEXFS_SUPER_MAGIC: crate::c_long = 195_894_762i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1483`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1483)*

### `HOSTFS_SUPER_MAGIC`
```rust
const HOSTFS_SUPER_MAGIC: crate::c_long = 12_648_430i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1484`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1484)*

### `HPFS_SUPER_MAGIC`
```rust
const HPFS_SUPER_MAGIC: crate::c_long = 4_187_351_113i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1485`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1485)*

### `HUGETLBFS_MAGIC`
```rust
const HUGETLBFS_MAGIC: crate::c_long = 2_508_478_710i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1486`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1486)*

### `ISOFS_SUPER_MAGIC`
```rust
const ISOFS_SUPER_MAGIC: crate::c_long = 38_496i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1487`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1487)*

### `JFFS2_SUPER_MAGIC`
```rust
const JFFS2_SUPER_MAGIC: crate::c_long = 29_366i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1488`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1488)*

### `MINIX2_SUPER_MAGIC2`
```rust
const MINIX2_SUPER_MAGIC2: crate::c_long = 9_336i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1489`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1489)*

### `MINIX2_SUPER_MAGIC`
```rust
const MINIX2_SUPER_MAGIC: crate::c_long = 9_320i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1490`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1490)*

### `MINIX3_SUPER_MAGIC`
```rust
const MINIX3_SUPER_MAGIC: crate::c_long = 19_802i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1491`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1491)*

### `MINIX_SUPER_MAGIC2`
```rust
const MINIX_SUPER_MAGIC2: crate::c_long = 5_007i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1492`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1492)*

### `MINIX_SUPER_MAGIC`
```rust
const MINIX_SUPER_MAGIC: crate::c_long = 4_991i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1493`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1493)*

### `MSDOS_SUPER_MAGIC`
```rust
const MSDOS_SUPER_MAGIC: crate::c_long = 19_780i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1494`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1494)*

### `NCP_SUPER_MAGIC`
```rust
const NCP_SUPER_MAGIC: crate::c_long = 22_092i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1495`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1495)*

### `NFS_SUPER_MAGIC`
```rust
const NFS_SUPER_MAGIC: crate::c_long = 26_985i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1496`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1496)*

### `NILFS_SUPER_MAGIC`
```rust
const NILFS_SUPER_MAGIC: crate::c_long = 13_364i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1497`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1497)*

### `OCFS2_SUPER_MAGIC`
```rust
const OCFS2_SUPER_MAGIC: crate::c_long = 1_952_539_503i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1498`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1498)*

### `OPENPROM_SUPER_MAGIC`
```rust
const OPENPROM_SUPER_MAGIC: crate::c_long = 40_865i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1499`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1499)*

### `OVERLAYFS_SUPER_MAGIC`
```rust
const OVERLAYFS_SUPER_MAGIC: crate::c_long = 2_035_054_128i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1500`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1500)*

### `PROC_SUPER_MAGIC`
```rust
const PROC_SUPER_MAGIC: crate::c_long = 40_864i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1501`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1501)*

### `QNX4_SUPER_MAGIC`
```rust
const QNX4_SUPER_MAGIC: crate::c_long = 47i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1502`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1502)*

### `QNX6_SUPER_MAGIC`
```rust
const QNX6_SUPER_MAGIC: crate::c_long = 1_746_473_250i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1503`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1503)*

### `RDTGROUP_SUPER_MAGIC`
```rust
const RDTGROUP_SUPER_MAGIC: crate::c_long = 124_082_209i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1504`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1504)*

### `REISERFS_SUPER_MAGIC`
```rust
const REISERFS_SUPER_MAGIC: crate::c_long = 1_382_369_651i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1505`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1505)*

### `SECURITYFS_MAGIC`
```rust
const SECURITYFS_MAGIC: crate::c_long = 1_935_894_131i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1506`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1506)*

### `SELINUX_MAGIC`
```rust
const SELINUX_MAGIC: crate::c_long = 4_185_718_668i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1507`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1507)*

### `SMACK_MAGIC`
```rust
const SMACK_MAGIC: crate::c_long = 1_128_357_203i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1508`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1508)*

### `SMB_SUPER_MAGIC`
```rust
const SMB_SUPER_MAGIC: crate::c_long = 20_859i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1509`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1509)*

### `SYSFS_MAGIC`
```rust
const SYSFS_MAGIC: crate::c_long = 1_650_812_274i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1510`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1510)*

### `TMPFS_MAGIC`
```rust
const TMPFS_MAGIC: crate::c_long = 16_914_836i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1511`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1511)*

### `TRACEFS_MAGIC`
```rust
const TRACEFS_MAGIC: crate::c_long = 1_953_653_091i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1512`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1512)*

### `UDF_SUPER_MAGIC`
```rust
const UDF_SUPER_MAGIC: crate::c_long = 352_400_198i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1513`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1513)*

### `USBDEVICE_SUPER_MAGIC`
```rust
const USBDEVICE_SUPER_MAGIC: crate::c_long = 40_866i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1514`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1514)*

### `XENFS_SUPER_MAGIC`
```rust
const XENFS_SUPER_MAGIC: crate::c_long = 2_881_100_148i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1515`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1515)*

### `NSFS_MAGIC`
```rust
const NSFS_MAGIC: crate::c_long = 1_853_056_627i64;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1516`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1516)*

### `AT_STATX_SYNC_TYPE`
```rust
const AT_STATX_SYNC_TYPE: crate::c_int = 24_576i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1580`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1580)*

### `AT_STATX_SYNC_AS_STAT`
```rust
const AT_STATX_SYNC_AS_STAT: crate::c_int = 0i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1581`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1581)*

### `AT_STATX_FORCE_SYNC`
```rust
const AT_STATX_FORCE_SYNC: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1582`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1582)*

### `AT_STATX_DONT_SYNC`
```rust
const AT_STATX_DONT_SYNC: crate::c_int = 16_384i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1583`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1583)*

### `STATX_TYPE`
```rust
const STATX_TYPE: crate::c_uint = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1584`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1584)*

### `STATX_MODE`
```rust
const STATX_MODE: crate::c_uint = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1585`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1585)*

### `STATX_NLINK`
```rust
const STATX_NLINK: crate::c_uint = 4u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1586`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1586)*

### `STATX_UID`
```rust
const STATX_UID: crate::c_uint = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1587`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1587)*

### `STATX_GID`
```rust
const STATX_GID: crate::c_uint = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1588`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1588)*

### `STATX_ATIME`
```rust
const STATX_ATIME: crate::c_uint = 32u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1589`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1589)*

### `STATX_MTIME`
```rust
const STATX_MTIME: crate::c_uint = 64u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1590`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1590)*

### `STATX_CTIME`
```rust
const STATX_CTIME: crate::c_uint = 128u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1591`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1591)*

### `STATX_INO`
```rust
const STATX_INO: crate::c_uint = 256u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1592`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1592)*

### `STATX_SIZE`
```rust
const STATX_SIZE: crate::c_uint = 512u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1593`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1593)*

### `STATX_BLOCKS`
```rust
const STATX_BLOCKS: crate::c_uint = 1_024u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1594`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1594)*

### `STATX_BASIC_STATS`
```rust
const STATX_BASIC_STATS: crate::c_uint = 2_047u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1595`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1595)*

### `STATX_BTIME`
```rust
const STATX_BTIME: crate::c_uint = 2_048u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1596`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1596)*

### `STATX_ALL`
```rust
const STATX_ALL: crate::c_uint = 4_095u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1597`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1597)*

### `STATX_MNT_ID`
```rust
const STATX_MNT_ID: crate::c_uint = 4_096u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1598`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1598)*

### `STATX_DIOALIGN`
```rust
const STATX_DIOALIGN: crate::c_uint = 8_192u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1599`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1599)*

### `STATX__RESERVED`
```rust
const STATX__RESERVED: crate::c_int = -2_147_483_648i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1600`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1600)*

### `STATX_ATTR_COMPRESSED`
```rust
const STATX_ATTR_COMPRESSED: crate::c_int = 4i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1601`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1601)*

### `STATX_ATTR_IMMUTABLE`
```rust
const STATX_ATTR_IMMUTABLE: crate::c_int = 16i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1602`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1602)*

### `STATX_ATTR_APPEND`
```rust
const STATX_ATTR_APPEND: crate::c_int = 32i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1603`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1603)*

### `STATX_ATTR_NODUMP`
```rust
const STATX_ATTR_NODUMP: crate::c_int = 64i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1604`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1604)*

### `STATX_ATTR_ENCRYPTED`
```rust
const STATX_ATTR_ENCRYPTED: crate::c_int = 2_048i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1605`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1605)*

### `STATX_ATTR_AUTOMOUNT`
```rust
const STATX_ATTR_AUTOMOUNT: crate::c_int = 4_096i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1606`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1606)*

### `STATX_ATTR_MOUNT_ROOT`
```rust
const STATX_ATTR_MOUNT_ROOT: crate::c_int = 8_192i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1607`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1607)*

### `STATX_ATTR_VERITY`
```rust
const STATX_ATTR_VERITY: crate::c_int = 1_048_576i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1608`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1608)*

### `STATX_ATTR_DAX`
```rust
const STATX_ATTR_DAX: crate::c_int = 2_097_152i32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1609`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1609)*

### `_IOC_NRBITS`
```rust
const _IOC_NRBITS: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1616`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1616)*

### `_IOC_TYPEBITS`
```rust
const _IOC_TYPEBITS: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1617`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1617)*

### `_IOC_SIZEBITS`
```rust
const _IOC_SIZEBITS: u32 = 14u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1638`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1638)*

### `_IOC_DIRBITS`
```rust
const _IOC_DIRBITS: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1639`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1639)*

### `_IOC_NONE`
```rust
const _IOC_NONE: u32 = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1641`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1641)*

### `_IOC_WRITE`
```rust
const _IOC_WRITE: u32 = 1u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1642`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1642)*

### `_IOC_READ`
```rust
const _IOC_READ: u32 = 2u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1643`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1643)*

### `_IOC_NRMASK`
```rust
const _IOC_NRMASK: u32 = 255u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1646`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1646)*

### `_IOC_TYPEMASK`
```rust
const _IOC_TYPEMASK: u32 = 255u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1647`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1647)*

### `_IOC_SIZEMASK`
```rust
const _IOC_SIZEMASK: u32 = 16_383u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1648`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1648)*

### `_IOC_DIRMASK`
```rust
const _IOC_DIRMASK: u32 = 3u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1649`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1649)*

### `_IOC_NRSHIFT`
```rust
const _IOC_NRSHIFT: u32 = 0u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1651`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1651)*

### `_IOC_TYPESHIFT`
```rust
const _IOC_TYPESHIFT: u32 = 8u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1652`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1652)*

### `_IOC_SIZESHIFT`
```rust
const _IOC_SIZESHIFT: u32 = 16u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1653`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1653)*

### `_IOC_DIRSHIFT`
```rust
const _IOC_DIRSHIFT: u32 = 30u32;
```

*Defined in [`libc-0.2.178/src/unix/linux_like/mod.rs:1654`](../../../.source_1765210505/libc-0.2.178/src/unix/linux_like/mod.rs#L1654)*

