/* R155: a raw GLOBAL function-pointer ARRAY declarator `RET (*NAME[N])(params);`.
   R152 handled only the scalar `(*g)()`; the `[N]` tail was left unconsumed, so the
   array was mis-sized (1 slot) and mis-typed (scalar) -> tbl[i]=/tbl[i]() took
   pointer-deref branches -> SEGV. Now sized as N 8-byte pointer slots (var_arr=1,
   var_ptr=1), so tbl[i] strides by 8 and tbl[i]() routes through R153's calli. */
static int acc = 0;
void a(void){ acc = acc*10 + 1; }
void b(void){ acc = acc*10 + 2; }
void c(void){ acc = acc*10 + 3; }
static void (*tbl[4])(void);
int main(){
  tbl[0]=a; tbl[1]=b; tbl[2]=c;
  int i; for (i=2; i>=0; i--) tbl[i]();   /* c,b,a -> 321 */
  return acc & 255;                       /* 65 */
}
