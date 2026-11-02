/* R6 (c99-closure): store through a deref'd ptr-to-ptr `(*q)->field = v` -- tcc's decl_designator
   does `(*cur_field)->c = index_last`, which crashed the derived tcc, making ARRAY designated
   initializers `int a[4]={[2]=9}` unusable. */
struct S { int c; int v; struct S* next; };
struct S g; struct S* p;
void set(struct S** cur_field, int x){ if (cur_field) (*cur_field)->c = x; }
int main(void){
  struct S* lp = &g;
  struct S** q = &lp;
  (*q)->v = 11;                 /* local ptr-to-ptr, nonzero offset */
  p = &g; set(&p, 7);           /* param ptr-to-ptr (tcc's exact shape) */
  return g.c + g.v;             /* 7 + 11 = 18 */
}
