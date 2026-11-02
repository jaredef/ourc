#ifndef _RZ_STDARG_H
#define _RZ_STDARG_H
/* rzcpp/ccc stdarg: va_list is an ARRAY type [base, index] over ccc's __va_area (the 6 arg registers
 * copied to a frame array on entry to a variadic fn). Being an array, it DECAYS to a pointer when
 * passed, so va_arg advances shared state across a call (the vsnprintf pattern). __va_start captures
 * &__va_area + the named-param count; __va_next reads base[index] (8-byte) and advances. */
typedef long va_list[2];
#define va_start(ap, last) __va_start(ap)
#define va_arg(ap, type)   __va_next(ap)
#define va_end(ap)         0
#define va_copy(d, s)      __va_copy(d, s)
#endif
