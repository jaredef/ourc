/* R160 (cf-64bit R5b): coercing a genuine 64-bit value to bool via `!x`, `&&`, `||`, or a ternary
   condition tested only the LOW 32 bits, so a value whose set bits are all in the high word (e.g. a
   NaN mantissa `x & D_MANT_MASK`) read as false. Fix: emit_bool/emit_notbool use eq8/ne8 when is64.
   softfloat's d_is_nan (`(x&INF)==INF && (x&MANT)`) depended on this. */
int main(void){
  unsigned long v=0x0008000000000000UL;   /* nonzero only in the high 32 bits */
  int r=0;
  if(v) r+=1; else return 10;              /* if(64bit) */
  if(1 && v) r+=2; else return 11;         /* && right operand */
  if(v || 0) r+=4; else return 12;         /* || left operand */
  if(!v) return 13;                         /* ! */
  r += v ? 8 : 0;                           /* ternary condition */
  if(r!=15) return 20;
  return 0;
}
