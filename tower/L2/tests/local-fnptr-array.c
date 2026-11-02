/* R156: a raw LOCAL function-pointer ARRAY declarator `RET (*NAME[N])(params);` inside a
   function body -- the local analogue of R155's global fix. Previously fell into the local
   SCALAR fn-ptr handler, reserved 0 slots, recorded tbl as a scalar with a const-0 base ->
   tbl[i]=f strode from address 0 -> SIGSEGV. Now N 8-byte pointer slots (var_arr=1,var_ptr=1);
   tbl[i] strides by 8 and tbl[i]() routes through R153's calli. */
static int acc = 0;
void a(void){ acc = acc*10 + 1; }
void b(void){ acc = acc*10 + 2; }
void c(void){ acc = acc*10 + 3; }
int main(){
  void (*tbl[4])(void);              /* raw LOCAL fn-ptr array */
  int guard_a = 11; int arr[3]; int guard_b = 22;
  arr[0]=7; arr[2]=9;
  tbl[0]=a; tbl[1]=b; tbl[2]=c;
  int i; for (i=2; i>=0; i--) tbl[i]();        /* c,b,a -> 321 */
  if (guard_a != 11 || guard_b != 22) return 1;   /* no neighbor overrun */
  if (arr[0]+arr[2] != 16) return 2;
  return acc & 255;                            /* 65 */
}
