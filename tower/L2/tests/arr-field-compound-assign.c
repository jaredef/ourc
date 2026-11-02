/* R146: `arr[i].field OP= v` (compound-assign to a field of an ARRAY element) was mis-parsed as
   a plain `=` -> the RHS `OP= v` became garbage and 0 was stored. That is exactly tcc's asm_rex
   `ops[rmi].reg -= 8` (Operand is an array, reg is int8_t) -> the derived tcc mis-encoded r8-r15
   in inline asm (rm bits lost). Fix: the arr[i].field lvalue path now checks is_caop and does
   load-op-store. Both int8_t and int fields, any index. Expect exit 42. */
typedef struct { unsigned int type; signed char reg; int big; } E;
int main(){
  E o[3]; int ok = 1;
  o[1].reg = 10; o[1].reg -= 8;          /* 2 */
  o[2].reg = 3;  o[2].reg += 5;          /* 8 */
  o[0].big = 100; o[0].big -= 58;        /* 42 */
  if (o[1].reg != 2) ok = 0;
  if (o[2].reg != 8) ok = 0;
  if (o[0].big != 42) ok = 0;
  return ok ? 42 : 0;
}
