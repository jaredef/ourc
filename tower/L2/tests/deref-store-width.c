/* R150: `*p = v;` as a STATEMENT must store at the pointee width. Was an unconditional
   4-byte store -> `*charptr = 0` zeroed 4 bytes, clobbering the neighbor (tcc-style
   `*s = 0; s++` in strtok/tokenizers left a wrong byte). Every other deref-store path
   (expression, *p++, cast, chain) already branched on width; this one didn't. */
char *pick(char *s){ *s = 0; s++; return s; }
int main(){
  char b[4]; b[0]=97; b[1]=98; b[2]=99; b[3]=0;
  char *r = pick(b);
  if (*r != 98) return 1;          /* b[1] must survive the *s=0 (which only zeroes b[0]) */
  if (b[0] != 0) return 2;
  if (b[2] != 99) return 3;        /* the over-wide store used to clobber b[1..3] */
  return 0;
}
