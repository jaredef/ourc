/* R3 (c99-closure): ccc lowers native float ops to soft-float calls (keeps tcc pristine). This KAT
   needs soft-float linked, so it is exercised via the chain+softfloat harness + the c99 battery, not
   run standalone. Kept as documentation of the covered surface. */
typedef unsigned long u64;
union U { double d; u64 i; long double ld; };
int main(void){ union U a,b; a.i=0x3FF8000000000000UL; b.i=0x4000000000000000UL;  /* 1.5, 2.0 */
  double x = a.d * b.d;       /* soft-float mul -> 3.0 */
  int i = 5; double y = (double)i;   /* int->double */
  double z = 1.5; y += z;            /* compound + literal */
  return (a.d < b.d) ? 0 : 1; }      /* soft-float compare */
