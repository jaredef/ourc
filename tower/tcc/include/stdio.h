#ifndef _RZ_STDIO_H
#define _RZ_STDIO_H
#include <stddef.h>
#include <stdarg.h>
#include <stdint.h>
typedef int FILE;
#define EOF (0-1)
#define stdin 0
#define stdout 1
#define stderr 2
#define printf printf_
#define fprintf fprintf_
#define sprintf sprintf_
#define fopen fopen_
#define fdopen fdopen_
#define fclose fclose_
#define fread fread_
#define fwrite fwrite_
#define fputc fputc_
#define fgetc fgetc_
#define fflush fflush_
#define open open_
#define read read_
#define write write_
#define close close_
#define lseek lseek_
#endif
