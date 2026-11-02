/* R160 (cf-64bit R5b): a `static unsigned long A[]={0x..64bit hex..}` GLOBAL initializer stored only
   the low 32 bits -- parse_const_val's `int v` truncated the literal and the pin table (pin_vals) is
   int. Fix: capture the high word (ce_hi -> pin_hi) and emit a genuine 64-bit init store (emit_lit64)
   for width-8 elements. */
static unsigned long A[4]={0x123456789ABCDEF0UL,0x7A3B1C9D5E6F8A2BUL,0xFEDCBA9876543210UL,0xFFFFFFFF00000001UL};
int main(void){
  if(A[0]!=0x123456789ABCDEF0UL) return 1;
  if(A[1]!=0x7A3B1C9D5E6F8A2BUL) return 2;
  if(A[2]!=0xFEDCBA9876543210UL) return 3;
  if(A[3]!=0xFFFFFFFF00000001UL) return 4;
  if(A[0]>=A[1]) return 5;               /* unsigned 64-bit order preserved */
  return 0;
}
