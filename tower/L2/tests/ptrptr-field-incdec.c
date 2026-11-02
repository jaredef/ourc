/* R142 regression: ++/-- of a POINTER-to-pointer struct FIELD used stride 1 instead of
   the pointee width. `S{ BF **sp; }` with `++s->sp` (pre-inc field statement) emitted
   `field + 1` instead of `field + 8` (sizeof(BF*)). Since `s->sp--` (post) correctly used
   stride 8, a push(++)/pop(--) pair did NOT cancel -> tcc's `++s1->include_stack_ptr` /
   `s1->include_stack_ptr--` desynced the include stack (`== include_stack` never held at
   include EOF) -> the derived tcc SIGSEGV on ANY `#include` that opens a file.
   Fix (ccc.c parse_stmt field-chain ++/--): a non-long 8-byte field strides by ffpw.
   A genuine long field keeps stride 1 (checked below).  Expect exit 1. */
typedef struct BF { int x; } BF;
typedef struct S { BF **sp; long n; } S;
int main() {
  S st; S *s; BF *arr[8];
  s = &st;
  s->sp = arr;      /* depth 0 */
  ++s->sp;          /* push: +8 (pointee = BF*, 8 bytes) */
  s->sp--;          /* pop:  -8 */
  s->n = 5; --s->n; /* long field: stride 1 -> n = 4, unaffected by the fix */
  if (s->sp == arr && s->n == 4) return 1;
  return 0;
}
