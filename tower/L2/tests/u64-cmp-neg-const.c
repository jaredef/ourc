/* tcc-self-compile R3: a 64-bit value compared to a SIGNED-NEGATIVE constant must sign-extend the
   constant. ccc zero-extended it (R91), so `u64(0xFFFFFFFF) == -1` wrongly matched (0x00000000FFFFFFFF
   == the zext'd -1). This made tcc's gen_opic `l2 == -1` fold-term fire, DROPPING `x & 0xFFFFFFFF` ->
   the self-compiled tcc emitted corrupt relocation symbol indices. Fix: isneg tag, sign-extend in a
   64-bit compare only (bitwise and8 keeps R4's zext mask). */
typedef unsigned long u64;
int main(void){
  u64 a = 0xFFFFFFFF;
  if (a == -1) return 1;                 /* must be FALSE: 0xFFFFFFFF != 0xFFFF..FFFF */
  if (a != 0xFFFFFFFF) return 2;         /* unsigned const still zext -> equal */
  { u64 b = 0x1FFFFFFFF; if (b == -1) return 3; if (b == 0xFFFFFFFF) return 4; }
  if ((a & 0xFFFFFFFF) != 0xFFFFFFFF) return 5;  /* the and8 mask (R4) must still work */
  { long c = -1; if (c != -1) return 6; if ((u64)c != a) { if (a == (u64)c) return 7; } }
  return 0;
}
