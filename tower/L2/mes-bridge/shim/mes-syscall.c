/* mes-runs R3: the _sys_call primitives over cf's __syscall -- the trust seam where GNU
 * Mes's syscall abstraction meets the derived substrate. Mes's own lib/linux writes these
 * in inline asm (ccc has none); cf lowers __syscall(num,a,b,c) to `mov rax,num; args in
 * rdi/rsi/rdx; syscall`, so the primitives are a thin C forward. cf carries num + 3 args;
 * _sys_call4+ (4th arg -> r10) awaits a cf widening if the eval path reaches it. */
long _sys_call (long n) { return __syscall (n, 0, 0, 0); }
long _sys_call1 (long n, long a) { return __syscall (n, a, 0, 0); }
long _sys_call2 (long n, long a, long b) { return __syscall (n, a, b, 0); }
long _sys_call3 (long n, long a, long b, long c) { return __syscall (n, a, b, c); }
long _sys_call4 (long n, long a, long b, long c, long d) { return __syscall (n, a, b, c); } /* d dropped -- cf 3-arg */
int _write (int fd, char const *buf, int n) { return __syscall (1, fd, buf, n); }

/* R3: exit + filesystem stubs the eval path never reaches (link-completeness only) */
void _exit (int code) { __syscall (60, code, 0, 0); }
int opendir (char const *name) { return 0-1; }
int pipe (int *fds) { return 0-1; }
int getdents (int fd, char *buf, int n) { return 0-1; }
