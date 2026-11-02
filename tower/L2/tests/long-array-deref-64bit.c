/* R160 R2: `unsigned long a[i]` and `*p` (long pointer) preserve the full 64 bits.
   The long-element read used emit_load8 (no is64 tag) -> a consuming `unsigned long x=a[i]`
   emit_widen'd it with convs (sign-extend), truncating the high 32. And the store convs'd an
   already-64-bit value. Fixed: emit_load8L for long elements + is64 tag; emit_conv idempotent on is64. */
int main(void){
  unsigned long a[2]; a[0]=0xAB00000000UL; a[1]=0xCD00000012UL;
  unsigned long x=a[0]; if (((x>>32)&255)!=0xAB) return 1;
  unsigned long *p=&a[1]; unsigned long y=*p; if (((y>>32)&255)!=0xCD) return 2;
  if ((y&255)!=0x12) return 3;
  int b[3]; b[0]=7; b[2]=9; if (b[0]+b[2]!=16) return 4;    /* int arrays unchanged */
  return 0;
}
