/* tcc-self-compile R1: struct-VALUED ternary `z = cond ? sA : sB`. tcc's expr_cond does
   `type = bt1==7 ? type1 : type2` (CType structs); ccc's rhs_saddr had no ternary case -> it read
   `cond` as the struct (z = cond) -> the ccc-built derived tcc CRASHED (si_addr=0x7) compiling ANY
   struct-valued ternary, incl. its own expr_cond. Fix: rhs_saddr selects the ADDRESS of the taken
   branch. (Uses struct PARAMS -- multi-initialized struct locals `struct S a={},b={}` are a separate
   pre-existing ccc bug.) */
struct S { int t; long ref; };
int pick(struct S a, struct S b, int c){ struct S z; z = c ? a : b; return z.t; }
int main(void){ struct S a, b; a.t=11; a.ref=0; b.t=22; b.ref=0; return pick(a,b,1)*100 + pick(a,b,0); }
