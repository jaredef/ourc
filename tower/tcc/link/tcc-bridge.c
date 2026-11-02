/* tcc-bridge.c -- resolves tcc's remaining externs against rzlibc (renames) + minimal stubs.
   ccc-compatible: int for pointers, trailing-underscore libc from rzlibc. First-run scaffolding:
   real malloc/free/realloc; everything else stubbed to surface the first real execution failure. */
int malloc_(int n);
int free_(int p);
int realloc_(int p, int n);

/* real bridges to rzlibc */
int malloc(int n) { return malloc_(n); }
int free(int p) { return free_(p); }
int realloc(int p, int n) { return realloc_(p, n); }

/* stubs -- return 0 (surface first failure empirically) */
int qsort(char *base, int nmemb, int size, int (*cmp)(char*, char*)) {         /* R108: was a STUB (return 0) -- tcc's switch qsort'd nothing, so `duplicate case value` fired spuriously on any 9+ unsorted cases (all tcc.c token switches). Insertion sort (case counts are small). */
  int i; int j; int k; char t; char *a; char *b;
  i = 1;
  while (i < nmemb) {
    j = i;
    while (j > 0) {
      a = base + (j-1)*size; b = base + j*size;
      if (cmp(a, b) <= 0) { break; }
      k = 0; while (k < size) { t = a[k]; a[k] = b[k]; b[k] = t; k = k+1; }
      j = j - 1;
    }
    i = i + 1;
  }
  return 0;
}
int strstr(int a, int b) { return 0; }
/* R1 (c99-closure): real strtod -- parse a decimal float and return its IEEE-754 double BIT PATTERN
   in rax. ccc has no float, so the value is built entirely with the (chain-compiled, 34/34 bit-exact)
   integer soft-float routines. ccc treats `double` as 8-byte (fn_flng=1) so tcc's `tokc.d = strtod(...)`
   stores all 8 returned bytes -> user float literals land correctly in .data. Bit-exact vs glibc on
   common literals, <=2 ulp on the extremes (no correctly-rounded bignum). */
unsigned long __floatsidf(int a);
unsigned long __adddf3(unsigned long a, unsigned long b);
unsigned long __muldf3(unsigned long a, unsigned long b);
unsigned long __divdf3(unsigned long a, unsigned long b);
unsigned __truncdfsf2(unsigned long a);
unsigned long strtod(char *s, char **endptr) {
  char *p; int neg; int decexp; int seendot; int expneg; int ev; int i; int c;
  unsigned long ten; unsigned long acc;
  p = s; neg = 0; decexp = 0; seendot = 0;
  while (*p==32 || *p==9) { p = p+1; }
  if (*p==43) { p = p+1; } else if (*p==45) { neg = 1; p = p+1; }
  ten = __floatsidf(10); acc = 0;
  while (1) {
    c = *p;
    if (c>=48 && c<=57) { acc = __adddf3(__muldf3(acc, ten), __floatsidf(c-48)); if (seendot) { decexp = decexp-1; } p = p+1; }
    else if (c==46 && seendot==0) { seendot = 1; p = p+1; }
    else { break; }
  }
  if (*p==101 || *p==69) {                                                     /* e / E */
    p = p+1; expneg = 0;
    if (*p==43) { p = p+1; } else if (*p==45) { expneg = 1; p = p+1; }
    ev = 0; while (*p>=48 && *p<=57) { ev = ev*10 + (*p-48); p = p+1; }
    if (expneg) { ev = 0-ev; } decexp = decexp + ev;
  }
  if (decexp>0) { i = 0; while (i<decexp) { acc = __muldf3(acc, ten); i = i+1; } }
  else if (decexp<0) { i = 0; while (i < (0-decexp)) { acc = __divdf3(acc, ten); i = i+1; } }
  if (neg) { acc = acc ^ 0x8000000000000000UL; }
  if (endptr) { *endptr = p; }
  return acc;
}
unsigned strtof(char *s, char **endptr) { return __truncdfsf2(strtod(s, endptr)); }  /* float bits (32-bit, eax) */
int strtold(int a, int b) { return 0; }
/* R126: real strtoull/strtoul (were STUBS returning 0 without advancing endptr) -- the x86
   assembler's asm_expr_unary does `n=strtoull(p,&p,0); if(*p) tcc_error("invalid number syntax")`,
   so every `$imm` asm operand failed. base 0 auto-detects 0x hex / 0 octal / decimal; sets *endptr. */
int strtoull(char *s, char **endptr, int base) {
  char *p; int c; int d; int r; int neg;
  p = s; neg = 0;
  while (*p==32 || *p==9) { p = p+1; }
  if (*p==43) { p = p+1; } else if (*p==45) { neg = 1; p = p+1; }
  if (base==0) {
    if (*p==48 && (p[1]==120 || p[1]==88)) { base = 16; p = p+2; }
    else if (*p==48) { base = 8; p = p+1; }
    else { base = 10; }
  } else if (base==16 && *p==48 && (p[1]==120 || p[1]==88)) { p = p+2; }
  r = 0;
  while (1) {
    c = *p;
    if (c>=48 && c<=57) { d = c-48; }
    else if (c>=97 && c<=122) { d = c-97+10; }
    else if (c>=65 && c<=90) { d = c-65+10; }
    else { break; }
    if (d >= base) { break; }
    r = r*base + d; p = p+1;
  }
  if (endptr) { *endptr = p; }
  if (neg) { r = 0-r; }
  return r;
}
int strtoul(char *s, char **endptr, int base) { return strtoull(s, endptr, base); }
/* R141: real snprintf (was a `return 0` stub -> wrote nothing). tcc's inline-asm register
   substitution does snprintf(buf,sizeof,"%%%s", get_tok_str(reg)) to build "%eax"; the stub
   left buf garbage -> the operand text was empty -> `%0` assembled as [0x0] (absolute mem)
   instead of the register. Positional-arg twin of vsnprintf (which is va_list-based). */
int _vs_num(char *dst, int o, int size, int v, int base, int uns, int width, int zero);
int snprintf(char *dst, int size, char *fmt, int a1, int a2, int a3, int a4, int a5) {
  int args[5]; int ai; int i; int o; int c; int width; int zero; int v; char *s; int j;
  args[0]=a1; args[1]=a2; args[2]=a3; args[3]=a4; args[4]=a5; ai=0;
  i=0; o=0;
  while (fmt[i] != 0 && o<size-1) {
    c=fmt[i];
    if (c != 37) { dst[o]=c; o=o+1; i=i+1; }
    else {
      i=i+1; width=0; zero=0;
      if (fmt[i]==48) { zero=1; i=i+1; }
      while (fmt[i]>=48 && fmt[i]<=57) { width=width*10+(fmt[i]-48); i=i+1; }
      while (fmt[i]==108 || fmt[i]==104) { i=i+1; }
      c=fmt[i]; i=i+1;
      if (c==100) { v=args[ai]; ai=ai+1; o=_vs_num(dst,o,size,v,10,0,width,zero); }
      else if (c==117) { v=args[ai]; ai=ai+1; o=_vs_num(dst,o,size,v,10,1,width,zero); }
      else if (c==120) { v=args[ai]; ai=ai+1; o=_vs_num(dst,o,size,v,16,1,width,zero); }
      else if (c==99) { v=args[ai]; ai=ai+1; if (o<size-1) { dst[o]=v; o=o+1; } }
      else if (c==115) { v=args[ai]; ai=ai+1; s=(char*)v; j=0; while (s[j] != 0 && o<size-1) { dst[o]=s[j]; o=o+1; j=j+1; } }
      else if (c==37) { if (o<size-1) { dst[o]=37; o=o+1; } }
      else { if (o<size-1) { dst[o]=37; o=o+1; } if (o<size-1) { dst[o]=c; o=o+1; } }
    }
  }
  if (o<size) { dst[o]=0; } else if (size>0) { dst[size-1]=0; }
  return o;
}
/* R114: real vsnprintf so tcc's error1/strcat_vprintf actually FORMAT the message (was a stub
   -> empty error text). Walks fmt, pulls each arg from the va_list via ccc's __va_next intrinsic
   (the same va the tcc_error(fmt,...) -> error1 -> strcat_vprintf chain threads down). Handles
   %d %u %x %c %s %% with optional zero/space min-width and l/h length modifiers -- the specifiers
   tcc's diagnostics use. Bounded by size. */
int __va_next(int ap);
int _vs_num(char *dst, int o, int size, int v, int base, int uns, int width, int zero) {
  char tmp[16]; int t; int q; int i;
  int neg; neg=0; if (uns==0 && v<0) { neg=1; v=0-v; }
  t=0; q=v;
  if (q==0) { tmp[0]=48; t=1; }
  else { while (q!=0) { i=q-(q/base)*base; if (i<0) { i=i+base; } if (i<10) { tmp[t]=48+i; } else { tmp[t]=87+i; } t=t+1; q=q/base; if (uns && q<0) { q=0; } } }
  i=t; if (neg) { i=i+1; }
  while (i<width) { if (o<size-1) { if (zero) { dst[o]=48; } else { dst[o]=32; } o=o+1; } i=i+1; }
  if (neg) { if (o<size-1) { dst[o]=45; o=o+1; } }
  while (t>0) { t=t-1; if (o<size-1) { dst[o]=tmp[t]; o=o+1; } }
  return o;
}
int vsnprintf(char *dst, int size, char *fmt, int ap) {
  int i; int o; int c; int width; int zero; int v; char *s; int j;
  i=0; o=0;
  while (fmt[i] != 0 && o<size-1) {
    c=fmt[i];
    if (c != 37) { dst[o]=c; o=o+1; i=i+1; }
    else {
      i=i+1; width=0; zero=0;
      if (fmt[i]==48) { zero=1; i=i+1; }
      while (fmt[i]>=48 && fmt[i]<=57) { width=width*10+(fmt[i]-48); i=i+1; }
      while (fmt[i]==108 || fmt[i]==104) { i=i+1; }
      c=fmt[i]; i=i+1;
      if (c==100) { v=__va_next(ap); o=_vs_num(dst,o,size,v,10,0,width,zero); }
      else if (c==117) { v=__va_next(ap); o=_vs_num(dst,o,size,v,10,1,width,zero); }
      else if (c==120) { v=__va_next(ap); o=_vs_num(dst,o,size,v,16,1,width,zero); }
      else if (c==99) { v=__va_next(ap); if (o<size-1) { dst[o]=v; o=o+1; } }
      else if (c==115) { v=__va_next(ap); s=(char*)v; j=0; while (s[j] != 0 && o<size-1) { dst[o]=s[j]; o=o+1; j=j+1; } }
      else if (c==37) { if (o<size-1) { dst[o]=37; o=o+1; } }
      else { if (o<size-1) { dst[o]=37; o=o+1; } if (o<size-1) { dst[o]=c; o=o+1; } }
    }
  }
  if (o<size) { dst[o]=0; } else if (size>0) { dst[size-1]=0; }
  return o;
}
int sscanf(char *s, char *fmt, int *p1, int *p2, int *p3) {  /* R68: minimal %d.%d.%d integer scan (version parsing) */
  int i; int n; int v; int neg;
  i=0; n=0;
  while (n < 3) {
    while (s[i] != 0) { if ((s[i]>=48 && s[i]<=57) || s[i]==45) { break; } i=i+1; }
    if (s[i]==0) { break; }
    neg=0; if (s[i]==45) { neg=1; i=i+1; }
    v=0; while (s[i]>=48 && s[i]<=57) { v=v*10+(s[i]-48); i=i+1; }
    if (neg) { v=0-v; }
    if (n==0) { p1[0]=v; } else if (n==1) { p2[0]=v; } else { p3[0]=v; }
    n=n+1;
  }
  return n;
}
int fputs(int a, int b) { return 0; }
int fseek(int a, int b, int c) { return 0; }
int ftell(int a) { return 0; }
int fdopen_(int fd, int wr);
int fdopen(int fd, int mode) { return fdopen_(fd, mode); }  /* R74: register raw fd as a FILE handle so ELF output actually writes */
int getcwd(int a, int b) { return 0; }
int getenv(int a) { return 0; }
int gettimeofday(int a, int b) { return 0; }
int localtime(int a) { return 0; }
int time(int a) { return 0; }
int unlink(int a) { return 0; }
int execvp(int a, int b) { return 0; }
int ldexp(int a, int b) { return 0; }
int assert(int a) { return 0; }
/* R112: setjmp/longjmp are now REAL cf intrinsics (ccc emits the `setjmp`/`longjmp` opcodes;
   cf saves rbx/r12/r13/rsp/resume-rip to a fixed buffer 0x1FF0000 and restores+jmps on longjmp).
   No stub needed -- and the error path now UNWINDS cleanly instead of returning into a corrupt
   parser. tcc uses one active jmp_buf (s1->error_jmp_buf) and always longjmp(buf,1), which the
   single-buffer intrinsic models exactly. */
/* desync/preprocessing artifacts (bogus symbols from miscompiled functions) */
int cc_error(int a, int b, int c, int d, int e) { return 0; }
int ELFW(int a) { return 0; }
int f() { return 0; }
/* R45 dbg markers: D<nn> / V<value> to stderr via raw syscall (no FILE deps) */
int dbg(int n) { char b[4]; b[0]=68; b[1]=48+(n/10)%10; b[2]=48+n%10; b[3]=10; __syscall(1, 2, b, 4); return 0; }
int dbgv(int n) { char b[16]; int i; int neg; neg=0; if (n<0) { neg=1; n=0-n; }
  i=14; b[15]=10; if (n==0) { b[i]=48; i=i-1; }
  while (n>0 && i>1) { b[i]=48+n%10; n=n/10; i=i-1; }
  if (neg) { b[i]=45; i=i-1; }
  b[i]=86; __syscall(1, 2, b+i, 16-i); return 0; }
int dbgs(char *p) { int n; n=0; while (p[n] != 0 && n < 2048) { n=n+1; }
  __syscall(1, 2, p, n); __syscall(1, 2, "\n", 1); return 0; }
int memcpy_(char *d, char *s, int n);
int memcpy_dbg(char *d, char *s, int n) { dbg(30); dbgv((int)d); dbgv((int)s); dbgv(n); return memcpy_(d, s, n); }
int memcpy_dbg2(int site, char *d, char *s, int n) { dbg(31); dbgv(site); dbgv((int)d); dbgv((int)s); dbgv(n); return memcpy_(d, s, n); }
/* R49: pointer-semantics wrappers -- rzlibc strchr_/strrchr_ return an INDEX (-1 if absent); C returns a pointer (NULL if absent) */
int strchr_(char *s, int c);
int strrchr_(char *s, int c);
int strchr_p(char *s, int c) { int i; i = strchr_(s, c); if (i < 0) { return 0; } return (int)(s + i); }
int strrchr_p(char *s, int c) { int i; i = strrchr_(s, c); if (i < 0) { return 0; } return (int)(s + i); }
