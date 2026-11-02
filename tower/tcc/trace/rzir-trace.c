/* rzir-trace -- a composable rzir instrumentation PASS. Injects a shadow-stack hook into every
 * function: `__tenter(&name)` right after the params (the name is built into a per-frame local
 * buffer, so it travels IN the binary -- no external map), and `__texit()` before each `ret`.
 * Fresh vN are allocated above the function's max; the name cost lands in each function itself,
 * never concentrated in main.
 *
 *   ccc < tcc.pp.c | rzir-trace | rzir-ld | rzir-opt | rzir-ra | cf > tcc.trace.elf
 *
 * The trace RUNTIME (__tenter/__texit/__tbacktrace, trace-runtime.c) must NOT be run through this
 * pass, or the hooks would recurse; functions whose name starts with "__t" are passed through
 * unchanged as a belt-and-suspenders guard. ccc subset; host-free (built by the derived chain). */

char src[4200000]; int slen;
char out[5600000]; int outn;
int loff[200000]; int llen[200000]; int nl;   /* line start + length (length INCLUDES the '\n') */

int read_all() { int t; int n; t=0; n=__syscall(0,0,src,65536);
  while (n>0 && t<4190000) { t=t+n; n=__syscall(0,0,src+t,65536); } return t; }

int emit(char *s, int n) { int i; i=0; while (i<n) { out[outn]=s[i]; outn=outn+1; i=i+1; } return 0; }
int emitline(int i) { emit(src+loff[i], llen[i]); return 0; }

/* emit a base-10 integer into out */
int emitnum(int v) { char tmp[12]; int t; int q; q=v; t=0;
  if (q==0) { out[outn]=48; outn=outn+1; return 0; }
  while (q>0) { tmp[t]=48+q%10; t=t+1; q=q/10; }
  while (t>0) { t=t-1; out[outn]=tmp[t]; outn=outn+1; } return 0; }

int isaln(int c) { if (c>=97 && c<=122) { return 1; } if (c>=65 && c<=90) { return 1; }
  if (c>=48 && c<=57) { return 1; } if (c==95) { return 1; } return 0; }

/* split src into lines (each length includes the trailing '\n') */
int split() { int i; int s; nl=0; i=0; s=0;
  while (i<slen) { if (src[i]==10) { loff[nl]=s; llen[nl]=i-s+1; nl=nl+1; s=i+1; } i=i+1; }
  if (s<slen) { loff[nl]=s; llen[nl]=slen-s; nl=nl+1; } return 0; }

/* is line i a function header? "ir NAME ..." or "fn NAME" at column 0 */
int is_hdr(int i) { int p; p=loff[i];
  if (src[p]==105 && src[p+1]==114 && src[p+2]==32) { return 1; }   /* "ir " */
  if (src[p]==102 && src[p+1]==110 && src[p+2]==32) { return 1; }   /* "fn " */
  return 0; }

/* line i is a `param` def? (indented `  vN: i32 = param`) */
int is_param(int i) { int p; int e; p=loff[i]; e=p+llen[i]-5;
  while (p<e) { if (src[p]==61 && src[p+1]==32 && src[p+2]==112 && src[p+3]==97 && src[p+4]==114 && src[p+5]==97 && src[p+6]==109) { return 1; } p=p+1; }  /* "= param" */
  return 0; }

/* line i is a `  ret vN` / `  ret` ? */
int is_ret(int i) { int p; p=loff[i]; while (src[p]==32) { p=p+1; }
  if (src[p]==114 && src[p+1]==101 && src[p+2]==116 && (src[p+3]==32 || src[p+3]==10)) { return 1; } return 0; }

/* max vN referenced in line i (word-boundary 'v'+digits); returns -1 if none */
int line_maxv(int i) { int p; int e; int mx; int prev; int v; mx=0-1; p=loff[i]; e=p+llen[i]; prev=32;
  while (p<e) {
    if (src[p]==118 && isaln(prev)==0 && src[p+1]>=48 && src[p+1]<=57) {   /* 'v' at boundary, then digit */
      v=0; p=p+1; while (src[p]>=48 && src[p]<=57) { v=v*10+(src[p]-48); p=p+1; }
      if (v>mx) { mx=v; } prev=48; }
    else { prev=src[p]; p=p+1; } }
  return mx; }

/* copy function NAME (from header line i) into nm[]; returns length. ir NAME -> / fn NAME */
char nm[256];
int getname(int i) { int p; int k; p=loff[i]+3; k=0;   /* past "ir "/"fn " */
  while (p<loff[i]+llen[i] && src[p]!=32 && src[p]!=10 && k<255) { nm[k]=src[p]; k=k+1; p=p+1; }
  nm[k]=0; return k; }

/* is nm[] a trace-runtime name (starts with "__t")? -> pass through, no injection */
int is_tracefn(int nlen) { if (nlen>=3 && nm[0]==95 && nm[1]==95 && nm[2]==116) { return 1; } return 0; }

int nv;   /* next free vN in the current function */

/* inject at entry: build nm[] into a local buffer, then `__tenter(&buf)` -- the name travels IN the
 * binary (no external map). Cost lands in each function itself (its name only), never concentrated in
 * main, so ra's per-function vN budget is safe. The buffer lives in the frame; ancestor buffers stay
 * valid while their frames are on the stack, which is exactly when __tbacktrace reads them. */
int inject_enter(int nlen) { int slots; int bufslot; int vref; int k;
  slots=(nlen+1+7)/8; bufslot=nv; nv=nv+slots;                 /* reserve the buffer slots */
  vref=nv; nv=nv+1;
  emit("  v", 3); emitnum(vref); emit(": i32 = ref v", 13); emitnum(bufslot); emit("\n", 1);
  k=0;
  while (k<=nlen) {                                            /* nlen chars + NUL */
    emit("  v", 3); emitnum(nv); emit(": i32 = const ", 14); if (k<nlen) { emitnum(nm[k]); } else { emitnum(0); } emit("\n", 1);
    emit("  v", 3); emitnum(nv+1); emit(": i32 = const ", 14); emitnum(k); emit("\n", 1);
    emit("  v", 3); emitnum(nv+2); emit(": i32 = add v", 13); emitnum(vref); emit(" v", 2); emitnum(nv+1); emit("\n", 1);
    emit("  storeb v", 10); emitnum(nv+2); emit(" v", 2); emitnum(nv); emit("\n", 1);
    nv=nv+3; k=k+1; }
  emit("  v", 3); emitnum(nv); emit(": i32 = call __tenter v", 23); emitnum(vref); emit("\n", 1);
  nv=nv+1; return 0; }

int main() { int i; int j; int fid; int maxv; int nlen; int injected; int skipfn; int hend; int m;
  slen=read_all(); split(); outn=0; fid=0;
  i=0;
  while (i<nl) {
    if (is_hdr(i)) {
      nlen=getname(i); fid=fid+1; skipfn=is_tracefn(nlen);
      /* scan [i .. next header) for maxv */
      hend=nl; m=i+1; while (m<nl) { if (is_hdr(m)) { hend=m; m=nl; } else { m=m+1; } }
      maxv=0-1; j=i; while (j<hend) { m=line_maxv(j); if (m>maxv) { maxv=m; } j=j+1; }
      nv=maxv+1; injected=0;
      emitline(i);                       /* the header */
      i=i+1;
      while (i<hend) {                    /* emit the rest of the function with injections */
        if (skipfn==0 && injected==0 && is_param(i)==0) { inject_enter(nlen); injected=1; }   /* first body line: __tenter(&name) */
        if (skipfn==0 && is_ret(i)) {                        /* pop before returning */
          emit("  v", 3); emitnum(nv); emit(": i32 = call __texit\n", 21); nv=nv+1; }
        emitline(i); i=i+1; }
    } else { emitline(i); i=i+1; } }
  __syscall(1, 1, out, outn); return 0; }
