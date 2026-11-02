/* R1 (c99-closure): the fix that made `tokc.d = strtod(...)` work -- a long/double-returning call's
   8-byte result stored into a SIGNED-long-shaped field must NOT be convs-truncated (convs reads only
   the low 32 bits). Covers both the strtod built-in tag and prototype-registered externals. */
typedef unsigned long u64;
u64 mk(int x);                    /* prototype: external-style long-returning call */
struct C { long tag; long v; };   /* v is a signed-long field (the convs path) */
static struct C c;
int main(void){
  c.v = mk(0);                    /* 8-byte call result -> signed-long field; must keep all 64 bits */
  if (c.v != 0x400921f9f01b866eUL) return 1;
  c.v = mk(1);
  if (c.v != 0x400921f9f01b866fUL) return 2;
  return 0;
}
u64 mk(int x){ return 0x400921f9f01b866eUL + (u64)x; }
