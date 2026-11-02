/* R145: dispatch-first switch (compares emitted BEFORE the bodies, all-forward gotos).
   The old goto-over-bodies + after-bodies dispatch miscompiled at scale in cf (tcc's
   preprocess_skip: a value mis-dispatched -> #if-skip infinite loop). Exercises
   fallthrough, default, char-brace case values, nesting, and an ident containing "case".
   Expect exit 42. */
int f(int c){ switch(c){ case '{': return 1; case '}': return 2; case 0: case 1: return 3; default: return 9; } }
int main(){
  int mycase=2; int r=0; int i;
  for (i=0;i<4;i++){ switch(i){ case 0: case 1: r+=1; break; case 2: r+=10; break; default: r+=100; } }
  /* r = 1+1+10+100 = 112 */
  switch(mycase){ case 2: r+=1000; break; default: r+=0; }   /* r=1112 */
  r = r + f('}')*10 + f(1) + f('x');   /* +20 +3 +9 = 1144 */
  return r & 255;   /* 1144 & 255 = 120... use a clean value */
}
