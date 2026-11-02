/* R154 (Bug E): `arr[i] = v` used as an EXPRESSION must return the assigned value.
   parse_expr(0) had assignment-expr branches for *IDENT=, IDENT=, IDENT->f=, IDENT.f=
   but not IDENT[i]= -- so `(a[i]=v)` fell to parse_atom which READ a[i] and dropped `=v`
   (`while((buf[i]=c))` looped once; `return (a[i]=5)` returned garbage). */
int main(void){
  int a[4]; char b[8]; char *p[3];
  if ((a[1] = 5) != 5) return 1;                 /* int element */
  if ((b[2] = 7) != 7) return 2;                 /* char element */
  if ((p[0] = "x") == 0) return 3;               /* pointer element */
  const char *s = "hello"; int i = 0;
  while ((b[i] = s[i])) i++;                      /* copy loop via assign-as-condition */
  if (i != 5) return 4;
  if (b[0] != 'h' || b[4] != 'o') return 5;
  a[2] = 1; a[2] += 9;                           /* compound-assign statement still works */
  if (a[2] != 10) return 6;
  return 0;
}
