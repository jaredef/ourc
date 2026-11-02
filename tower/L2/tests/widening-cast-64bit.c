/* R160 (cf-64bit R5c): a WIDENING scalar cast `(unsigned long)x` / `(u64)(expr)` was an identity, so
   `(u64)m << 29` (m a 32-bit value) shifted 32-bit and lost the high bits. Fix: parse_atom's scalar
   cast widens (emit_widen) when the cast type is 8-byte; the cast's long-ness/signedness are captured
   BEFORE the operand's parse_atom (which clobbers the decl globals for `(u64)(expr)`).
   softfloat's __extendsfdf2: `((u64)m)<<29` and `((u64)(mm & MASK))<<29`. */
typedef unsigned long u64; typedef unsigned u32;
int main(void){
  u32 m=0x7a1b4dU;
  u64 a=((u64)m)<<29;                 /* cast of an atom */
  if(a!=0xf4369a0000000UL) return 1;
  u64 b=((u64)(m & 0x7fffffU))<<29;   /* cast of a parenthesized expr */
  if(b!=0xf4369a0000000UL) return 2;
  u64 c=(u64)m + 0x100000000UL;       /* widened value in 64-bit arithmetic */
  if(c!=0x100000000UL+0x7a1b4dUL) return 3;
  return 0;
}
