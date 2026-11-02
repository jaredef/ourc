/* R148: `if (cond) s++, reg = v;` -- comma-operator if-body without braces, where the
   first operand is a bare expression (postfix s++). The bare-expression-statement path
   must treat the comma tail as PART of the guarded statement; otherwise `reg = v` escapes
   the if and runs unconditionally (tcc's asm_parse_numeric_reg mis-parsed %r8/%r9). */
int reg_from(char *s){ int reg; int c;
  if (*s != 'r') return -1; s++; reg = 0;
  if ((c = *s) >= '0' && c <= '9') s++, reg = c - '0'; else return -1;
  if ((c = *s) >= '0' && c <= '5') s++, reg = reg * 10 + c - '0';
  if (reg > 15) return -1; if (*s != 0) return -1; return reg; }
int main(){
  int bad; bad=0;
  if (reg_from("r8")  != 8)  bad=1;
  if (reg_from("r9")  != 9)  bad=2;
  if (reg_from("r10") != 10) bad=3;
  if (reg_from("r15") != 15) bad=4;
  if (reg_from("r16") != -1) bad=5;
  if (reg_from("r3")  != 3)  bad=6;
  return bad; }
