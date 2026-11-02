/* R4 (c99-closure): the RUNTIME fix that unblocked dtoa's %f. A genuine 64-bit value OR'd/AND'd with a
   >32-bit constant must keep all 64 bits (the const must NOT be truncated to imm32), and a NARROWING
   (int)/(unsigned) cast of a 64-bit VALUE must drop the high 32 bits. Root cause was that `(int)<64-bit>`
   was an identity, so tcc's `(int)c.i == c.i` fits-in-imm32 check compared c to itself = always true ->
   every 64-bit const truncated to imm32 in ALU ops (frac | 0x10000000000000). Tested through variables
   (the runtime path); compile-time const-folding of `(unsigned)<huge lit> != k` is a separate ccc
   const-folder gap, tracked in trajectory. */
typedef unsigned long u64;
u64 mkc(void){ return 0x123456789AUL; }
u64 hibit(void){ return 0x10000000000000UL; }
int main(void){
  u64 c = mkc();
  if ((int)c != 0x3456789A) return 1;              /* signed narrow: low 32, sext */
  if ((unsigned)c != 0x3456789AU) return 2;        /* unsigned narrow: low 32 */
  if (((int)c == c)) return 3;                      /* the fits-check itself: must be FALSE */
  { u64 a = 0; if (((a | hibit()) >> 48) != 16) return 4; }  /* the unblocked ALU-const OR */
  { u64 b = hibit() + 1; if ((b & hibit()) == 0) return 5; } /* AND keeps the high bit */
  return 0;
}
