/* Era-appropriate (C89) stdio declarations -- no modern-glibc typedefs (__gnuc_va_list,
   _Float32) that tcc-0.9.27 predates. Enough to compile+link real stdio programs. */
#ifndef _STDIO_H
#define _STDIO_H
int printf(const char *fmt, ...);
int fprintf(void *stream, const char *fmt, ...);
int putchar(int c);
int puts(const char *s);
#endif
