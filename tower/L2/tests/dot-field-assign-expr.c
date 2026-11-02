/* R147: `IDENT.field = rhs` used as an EXPRESSION (e.g. `if ((o.reg = f()) >= 0)`) was unhandled --
   only IDENT= and IDENT->f= existed -- so it fell to a normal parse that READ o.reg (uninitialized)
   instead of storing. Fix: parse_expr now handles the dot-field assignment-expression. Expect 42. */
typedef struct { unsigned int type; signed char reg; long big; } E;
int f8(){ return 8; }
int main(){
  E o; int ok = 1;
  if ((o.reg = f8()) >= 0) { if (o.reg != 8) ok = 0; } else ok = 0;   /* store + value in cond */
  { int s; s = (o.reg = 3) + 4; if (s != 7 || o.reg != 3) ok = 0; }    /* value of assign-expr */
  o.big = 0; if ((o.big = 9000000000) < 0) ok = 0;                     /* long dot-field assign-expr */
  return ok ? 42 : 0;
}
