/* R152: a GLOBAL scalar function pointer `T (*g)(params);` -- parse_global didn't
   recognize the `(*NAME)(params)` declarator (read_ident hit '(' -> empty name), so g
   was never a known var: `g=h` mis-stored and `g()` emitted a bogus direct `call g`.
   The calli indirect-call path already existed (used by param/local/typedef fn-ptrs);
   this makes a global fn-ptr a known var so it routes through calli. */
static int acc = 0;
void a(void){ acc += 1; }
void b(void){ acc += 20; }
static void (*g)(void);
int main(){ g = a; g(); g = b; g(); return acc; }   /* 21 */
