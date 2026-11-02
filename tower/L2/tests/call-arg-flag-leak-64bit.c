/* R160 (cf-64bit R5): a function call left lit_huge / cast_un / cast_ps set from parsing a HUGE or
   UNSIGNED argument, and the enclosing expression wrongly consumed those flags:
     - `f(0x..HUGE) == small`  -> the R55 lit_huge fold fired (stale lit_huge) and folded the
       compare to a constant, even though the call RESULT is a plain 32-bit int.
     - `cmp(u64,u64) < 0`      -> the `<` became an UNSIGNED `ltu` (stale cast_un), so a negative
       result (-1) read as >= 0.  This is exactly softfloat's __ltdf2(a,b)<0.
   A call result's hugeness/signedness/pointee-width come from its RETURN type, never an argument. */
typedef unsigned long u64;
int ident1(u64 x){ return 1; }
int cmp(u64 a, u64 b){ if (a<b) return -1; if (a>b) return 1; return 0; }
int main(void){
  /* lit_huge leak: call whose arg is a huge literal, result compared to a small literal */
  if (ident1(0x4018000000000000UL) == 1) { /* must really evaluate the call, not const-fold */ } else return 1;
  /* cast_un leak: cmp with huge unsigned args, negative result compared < 0 (inline) */
  u64 x=0x3FF8000000000000UL, y=0x4010000000000000UL;
  if (!(cmp(x,y) < 0))  return 2;   /* 1.5 < 4.0 -> -1 -> must be < 0 */
  if (!(cmp(y,x) >= 0)) return 3;   /* 4.0 < 1.5 false -> 1 -> >= 0 */
  if (!(cmp(x,x) == 0)) return 4;
  return 0;
}
