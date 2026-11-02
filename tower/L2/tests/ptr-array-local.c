/* R150(B): a LOCAL `char *a[N]` is a POINTER array (N*8 bytes), not a byte array.
   The slot-sizing matched dchar (base type char) before the pointer branch, reserving
   ceil(N/8)=1 slot for a 16-byte object -> a[1] overran the neighbor SSA slot (the
   cached &a), nulling it -> reading a[i] later segfaulted. Also fixes e[0][0] double
   subscript (Bug C) and char*[] initializer stride. Guard: dchar && nstar==0 && dptr==0. */
int slen(char*s){int n=0;while(s[n])n++;return n;}
int main(){
  char *a[3]; a[0]="AB"; a[1]="CDE"; a[2]=0;
  char *p; p=a[0]; if (p[0]!='A') return 1;      /* Bug B: ptr into local then deref */
  int i; int tot=0;
  for (i=0; a[i]; i++) tot += slen(a[i]);          /* walk + double-deref via slen */
  if (tot != 5) return 2;
  char b[4]; b[0]=9; b[1]=8; b[2]=7; b[3]=6;       /* real byte array still works */
  if (b[0]+b[3] != 15) return 3;
  return 0;
}
