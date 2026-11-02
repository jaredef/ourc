#ifndef _FCNTL_H
#define _FCNTL_H
/* Linux x86-64 open(2) flag values -- tcc uses these for object-file output.
   Absent this header, rzcpp leaves O_* unexpanded and ccc bakes them to 0,
   so open() degrades to O_RDONLY and every -c output fails with ENOENT. */
#define O_RDONLY 0
#define O_WRONLY 1
#define O_RDWR   2
#define O_CREAT  64
#define O_TRUNC  512
#define O_APPEND 1024
#define O_BINARY 0
#endif
