/* R139 regression: a scalar CAST inside a global struct-array aggregate initializer
   (e.g. the x86 asm `ASMInstr op_type[3]` opcode table `{(uint64_t)0xfbc, ...}`).
   Bug: scalar_cast_at()'s decl_type() call clobbered the decl-context globals
   (dstruct/dchar/dlong/...) mid-declaration, so the array variable `t` lost its
   struct binding -> element stride became the default int width (shl 2) and field
   offsets collapsed to 0. Every `t[i].field` read returned t[0].sym.
   Fix: scalar_cast_at saves/restores the decl-context globals -- a cast is
   side-effect-free on the enclosing declarator.  Expect exit 214 (== gcc). */
typedef unsigned short uint16_t;
typedef unsigned char  uint8_t;
typedef unsigned long  uint64_t;
typedef struct S { uint16_t a; uint16_t b; uint8_t c; } S;
static const S t[] = { { 11, (uint64_t)7, 22 }, { 33, 44, 55 } };
int main() { return t[0].c*100 + t[0].b + t[1].c; }   /* 22*100 + 7 + 55 = 2262; &255 = 214 */
