/* R160 (cf-64bit R5c): the 2nd+ declarator of `unsigned long a=X, b=Y;` lost its 64-bit init.
   The declarator's initializer (right_init -> parse_expr) clobbers the decl-context `dlong`, so the
   continuation declarator saw dlong=0 and copied its is64 init value through a 32-bit `add v,0`,
   dropping the high word. Fix: capture the BASE long-ness after decl_type() and restore it per
   declarator. (softfloat's add_d: `u64 ma=a&MASK, mb=b&MASK` -> mb's high mantissa lost.) */
typedef unsigned long u64;
static u64 f(u64 a, u64 b){
  u64 ma=a&0x000fffffffffffffUL, mb=b&0x000fffffffffffffUL;  /* both must keep 52-bit values */
  if(ma!=0x000fffffffffffffUL) return 1;
  if(mb!=0x000abcdef1234567UL) return 2;
  return 0;
}
int main(void){ return (int)f(0xFFFFFFFFFFFFFFFFUL, 0x7FFabcdef1234567UL); }
