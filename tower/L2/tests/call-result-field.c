/* R143: postfix `->field` after a function CALL -- `f(args)->field` as both an rvalue
   (read) and an lvalue (write). ccc's parse_call returned the pointer with no postfix
   chaining, so `f()->m` mis-tokenized `->` and both read and store went nowhere. This
   blocked include guards in the derived tcc: the guard is recorded via
   `search_cached_include(s1, file->filename, 1)->ifndef_macro = file->ifndef_macro_saved;`
   (store through a call result) -> the store was dropped -> every guarded header re-opened
   on a second #include (diamond include hung/failed).
   Fix: a fn-return-struct table (fn_rsptr) recorded in the pre-pass; parse_atom chains
   `->field` reads after a struct-ptr-returning call, and the call-statement path handles
   `call()->field = v` stores.  Expect exit: 77 + 55 = 132. */
typedef struct E { int a; int m; } E;
E pool[8];
E *at(int i) { return &pool[i]; }
int main() {
  at(3)->m = 77;          /* store through a call result */
  at(5)->m = 55;
  return at(3)->m + at(5)->m - 0;   /* read through a call result: 77 + 55 = 132 */
}
