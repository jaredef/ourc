/* R2 (c99-closure): `*(EXPR)` where EXPR is a char* POINTER EXPRESSION (p-1, p+n) must deref 1 byte,
   not 4. Was defaulting to a 4-byte `load` (char pointee width lost across pointer arithmetic) ->
   tcc's LL/ULL suffix lexer `*(p - 1) != ch` read 4 bytes -> "incorrect integer suffix". */
char buf[8];
int main(void){
  buf[0]='L'; buf[1]='L'; buf[2]=';'; buf[3]='X';
  char *p = buf + 2;
  if (*(p - 1) != 'L') return 1;        /* p-1 -> buf[1]='L', 1-byte */
  if (*(p - 2) != 'L') return 2;
  if (*(p + 1) != 'X') return 3;
  { char *q = buf; int ch = *q++; ch = *q++; if (*(q - 1) != ch) return 4; }  /* the exact suffix-loop shape */
  return 0;
}
