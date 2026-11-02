/* BREAKS #5 + #6: a ternary producing a long, and a compound-assign (+=) on a long, used to copy /
   op through a 32-bit path -> the high bits were dropped. R3 (emit_copy8 for is64 ternary arms;
   emit_binop8/emit_load8L/emit_copy8 for `op=` on an llong lvalue) fixes both. */
int main(void){
  unsigned long a=0x300000007UL, b=0x800000002UL;
  /* #5 ternary result long */
  unsigned long t = (a<b) ? a : b;          /* a<b true -> t=a=0x300000007 */
  if (t != 0x300000007UL) return 1;
  /* #6 compound-assign on a long local */
  unsigned long s = 0x100000000UL;
  s += 0x000000005UL;                         /* -> 0x100000005 (low += without clobbering high) */
  if (s != 0x100000005UL) return 2;
  s += a;                                     /* 0x100000005 + 0x300000007 = 0x40000000C */
  if (s != 0x40000000CUL) return 3;
  return 0;
}
