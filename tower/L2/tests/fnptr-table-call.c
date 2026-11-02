/* R153: `tbl[i](args)` -- indirect call through a function-pointer ARRAY element
   (a dispatch table / atexit's LIFO handler registry). The scalar global fn-ptr call
   was R152; this adds the indexed form: load8(base + i*8) -> `calli`, in both the
   statement (parse_stmt subscript) and expression (parse_atom) contexts, via emit_icall. */
typedef void (*fn_t)(void);
static int acc = 0;
void a(void){ acc = acc*10 + 1; }
void b(void){ acc = acc*10 + 2; }
void c(void){ acc = acc*10 + 3; }
static fn_t tbl[8]; static int n = 0;
int reg(fn_t f){ tbl[n] = f; n++; return 0; }
int main(){
  reg(a); reg(b); reg(c);
  int i; for (i = n-1; i >= 0; i--) tbl[i]();   /* reverse (LIFO): c,b,a -> 321 */
  return acc & 255;                             /* 321 & 255 = 65 */
}
