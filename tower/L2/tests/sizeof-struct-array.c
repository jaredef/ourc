/* R140 regression: sizeof(array-of-struct) reported the SLOT-padded internal size
   (n * ceil(ssz/8) * 8) instead of the true C size (n * ssz). E.g. struct{short,short,
   uchar} is 6 bytes; [2] should be 12, chain gave 16 (2 * ceil(6/8)*8). This broke the
   `sizeof(tbl)/sizeof(tbl[0])` element-count idiom (tcc sizes asm_instrs[] that way).
   Fix: dvbytes = n*ssz for struct-value arrays; allocation stays slot-padded.
   Expect exit 12 + 3 -> combined below == 15. */
typedef struct S { unsigned short a; unsigned short b; unsigned char c; } S;   /* ssz = 6 */
static S t[2];
static S u[3];
int main() {
  int sz;   /* sizeof(t) = 2*6 = 12 */
  int cnt;  /* sizeof(u)/sizeof(u[0]) = 18/6 = 3 */
  sz  = sizeof(t);
  cnt = sizeof(u) / sizeof(u[0]);
  return sz + cnt;   /* 12 + 3 = 15 */
}
