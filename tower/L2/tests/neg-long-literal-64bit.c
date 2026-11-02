/* BREAK #7: a large NEGATIVE long literal (-4294967296L == -2^32 == 0xFFFFFFFF00000000) lost its
   high bits. Root: unary minus emitted `sub(0, v)` as a 32-bit `sub` even when v is is64, so the
   low words (0-0) gave 0. R4: unary minus uses emit_binop8(5,...) when the operand is is64. */
int main(void){
  long x = -4294967296L;
  if (((x >> 32) & 255) != 255) return 1;   /* high dword is 0xFFFFFFFF -> low byte 0xFF == 255 */
  if (x + 4294967296L != 0) return 2;        /* exact negation: x == -2^32 */
  return 0;
}
