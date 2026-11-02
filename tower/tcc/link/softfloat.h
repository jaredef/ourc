/* softfloat.h -- freestanding IEEE-754 soft-float runtime (bit-pattern ABI).
 *
 * All doubles are the 64-bit IEEE binary64 bit pattern carried in an
 * `unsigned long` (assumed 64-bit). All floats are the 32-bit IEEE binary32
 * bit pattern carried in an `unsigned`. NO float/double types appear here:
 * every routine is pure integer arithmetic on the IEEE bit patterns.
 *
 * Names & semantics follow libgcc so a compiler can lower float ops to them.
 */
#ifndef SOFTFLOAT_H
#define SOFTFLOAT_H

/* ---- double (binary64) arithmetic ---- */
unsigned long __adddf3(unsigned long a, unsigned long b);
unsigned long __subdf3(unsigned long a, unsigned long b);
unsigned long __muldf3(unsigned long a, unsigned long b);
unsigned long __divdf3(unsigned long a, unsigned long b);
unsigned long __negdf2(unsigned long a);

/* ---- double compares (libgcc convention) ----
 * __ltdf2 <0 iff a<b, __gtdf2 >0 iff a>b, __eqdf2 ==0 iff a==b, etc.
 * Any NaN operand => "unordered": eq/ne report not-equal, all ordered
 * relations (lt/le/gt/ge) report false via the sign convention below.
 * __unorddf2 returns 1 if either operand is NaN, else 0. */
int __eqdf2(unsigned long a, unsigned long b);
int __nedf2(unsigned long a, unsigned long b);
int __ltdf2(unsigned long a, unsigned long b);
int __ledf2(unsigned long a, unsigned long b);
int __gtdf2(unsigned long a, unsigned long b);
int __gedf2(unsigned long a, unsigned long b);
int __unorddf2(unsigned long a, unsigned long b);

/* ---- double <-> int conversions ---- */
unsigned long __floatsidf(int a);         /* signed int   -> double bits */
unsigned long __floatunsidf(unsigned a);  /* unsigned int -> double bits */
int          __fixdfsi(unsigned long a);  /* double bits  -> signed int (trunc) */
unsigned     __fixunsdfsi(unsigned long a);/* double bits -> unsigned int (trunc) */

/* ---- float (binary32) arithmetic ---- */
unsigned __addsf3(unsigned a, unsigned b);
unsigned __subsf3(unsigned a, unsigned b);
unsigned __mulsf3(unsigned a, unsigned b);
unsigned __divsf3(unsigned a, unsigned b);
unsigned __negsf2(unsigned a);

/* ---- float compares ---- */
int __eqsf2(unsigned a, unsigned b);
int __nesf2(unsigned a, unsigned b);
int __ltsf2(unsigned a, unsigned b);
int __lesf2(unsigned a, unsigned b);
int __gtsf2(unsigned a, unsigned b);
int __gesf2(unsigned a, unsigned b);
int __unordsf2(unsigned a, unsigned b);

/* ---- float <-> int conversions ---- */
unsigned __floatsisf(int a);
unsigned __floatunsisf(unsigned a);
int      __fixsfsi(unsigned a);
unsigned __fixunssfsi(unsigned a);

/* ---- float <-> double ---- */
unsigned long __extendsfdf2(unsigned a);  /* float bits  -> double bits */
unsigned      __truncdfsf2(unsigned long a);/* double bits -> float bits (round nearest even) */

#endif /* SOFTFLOAT_H */
