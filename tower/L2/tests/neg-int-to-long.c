/* R144: negative int -> long widening must SIGN-extend (was conv/zext -> `long a=-5`
   held +4294967291; 64-bit div8/mod8 then wrong). A positive decimal in [2^31,2^32)
   must still ZERO-extend (it is a positive long). Both checked. Expect exit 42. */
int main(){
  long a = -5;                     /* sext -> -5 */
  int x = -7; long b = x;          /* signed int -> long: sext */
  unsigned long u = 4000000000;    /* positive long, zext */
  int ok = 1;
  if (a % 3 != -2) ok = 0;         /* -5 % 3 == -2 */
  if (b * 1000 % 7 != 0) ok = 0;   /* -7000 % 7 == 0 */
  if (u % 1000 != 0) ok = 0;       /* 4000000000 % 1000 == 0 */
  return ok ? 42 : 0;
}
