/* R157: a 64-bit HEX literal (>8 hex digits) must be a genuine 64-bit const, not truncated to
   the low 32 bits. The hex lexer read into a 32-bit accumulator and returned emit_const(low32),
   so 0x4018000000000000UL became 0 (its low word). The DECIMAL path already used emit_lit64;
   this mirrors it (accumulate the high 32 bits, route to emit_lit64 when >8 digits). Foundational
   for the float arc (IEEE bit patterns) and any 64-bit mask/reloc literal. */
int main(void){
  unsigned long a = 0x4018000000000000UL;   /* 6.0's IEEE bits */
  unsigned long b = 0xDEADBEEFCAFEBABEUL;
  if ((a >> 52) != 0x401) return 1;
  if ((a & 0xFFFFFFFFUL) != 0) return 2;     /* low word is 0 -- must survive */
  if ((b >> 32) != 0xDEADBEEFUL) return 3;   /* high word preserved */
  if ((b & 0xFFFFFFFFUL) != 0xCAFEBABEUL) return 4;
  if (0xFF != 255) return 5;                 /* 32-bit hex still fine */
  return 0;
}
