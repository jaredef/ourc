/* R158: `long`/`unsigned long` scalar function PARAMS preserve the full 64-bit argument.
   They were declared i32 -> 64-bit args truncated to 32 bits (softfloat's bit-pattern ABI is
   all `unsigned long` params). Now tagged is64 (after the per-fn is64 clear), so cf loads the
   full arg register and consumers use the *8 op family. Pointers stay 32-bit (the value model);
   only genuine long/ulong scalars promote. */
unsigned long himul(unsigned long a, unsigned long b){ return (a*b) >> 32; }
unsigned long hishift(unsigned long x){ return x >> 40; }
int slt(long a, long b){ return a < b; }
int main(void){
  if (himul(0x100000000UL, 3) != 3) return 1;              /* 64-bit multiply, high word */
  if (hishift(0xABCDEF0000000000UL) != 0xABCDEFUL) return 2; /* high bits survive */
  if (slt(-5L, 3L) != 1) return 3;                          /* signed long param compare */
  if (slt(9L, 2L) != 0) return 4;
  return 0;
}
