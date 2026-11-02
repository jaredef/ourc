#ifndef _RZ_STDDEF_H
#define _RZ_STDDEF_H
typedef unsigned long size_t;
typedef long ptrdiff_t;
typedef long ssize_t;
typedef unsigned int wchar_t;
#define NULL ((void*)0)
#define offsetof(t, m) ((size_t)(&((t*)0)->m))
#endif
