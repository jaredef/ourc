/* BREAK #1 (DOMINANT): ==/!= (and while(x), if(x), x==0) on a 64-bit value compare only the LOW 32
   bits (ccc emits eq/ne, not eq8/ne8). while(x){x>>=1;} on a high-only value exits immediately. */
int main(void){ unsigned long a=0x100000005UL; unsigned long b=0x200000005UL;
  if (a==b) return 1;              /* low words equal, high differ -> NOT equal; ccc wrongly says == */
  { unsigned long x=0x100000000UL; int n=0; while(x){ n++; x=x>>1; } if (n!=33) return 2; }
  if ((0x500000000UL != 0UL) != 1) return 3;
  return 0; }
