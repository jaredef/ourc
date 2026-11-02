/* rzir-ra.c -- M5: the register allocator ported to ccc's C subset (self-hosted allocator).
 * Byte-identical port of rzir-ra.py R13: per-function linear scan over the two pools
 * (callee-saved r12/r13 for across-call values; caller-saved r10/r11 for the rest), honored
 * class = ARITH + CMP defs/uses + brz/ret consumers, back-edge-aware intervals (loop/pool AND
 * backward goto), R14 mutable-value allocation (every def op honored incl const, every use
 * honored -> one register, mutation = register re-store; no phis needed), FIFO free queues and
 * (end,regname)-ordered expiry to match the Python exactly. Emits `@rNN ` on EVERY def line of
 * an assigned value.
 * Build: ccc.bin < rzir-ra.c | rzir-ld | cf. Gate: byte-identity vs rzir-ra.py on the corpus,
 * then the full oracle through the allocated pipeline. */

/* C-RA-BUF-8MB-OVERFLOW (sibling of C-LD-BUF-8MB-OVERFLOW and cf.rs's C-CF-BUF-8MB-OVERFLOW): `out` had no
   bounds check, and ra's output is LARGER than its input (it annotates every value with a register). At HEAD
   the SEM0 self-emit made ra write 8,640,061 bytes into out[8000000]: from byte 8,000,001 the stream turned
   into the raw contents of the next static, i.e. ~640 KB of binary garbage handed to cf on EVERY self-emit.
   Gates stayed green only because the corrupted tail held cold code. Widened 2026-08-27; the arrays are BSS so
   the emitted binary does not grow. NOTE ra expands its input: size out for the OUTPUT, not the input. */
char src[50331648]; int slen;
char out[50331648]; int outn; int outovf; int srcovf;

int loff[2000000];   /* line start offset */
int llen[2000000];   /* line length (excl newline) */
int nl;

int cumcall[2000000];   /* cumulative count of call-bearing lines, inclusive */

int defs[40000];    /* first def line index, -1 none */
int lu[40000];      /* last use line index, -1 none */
int okdef[40000];   /* first def's op is in the honored class */
int baduse[40000];  /* some use sits in a non-honored consumer */
int areg[40000];    /* assigned register code: 0 none, 1 r12, 2 r13, 3 r10, 4 r11 */
int maxv;

int rgs[256]; int rge[256]; int nrg;             /* back-edge regions (loop/pool + label/goto) */
int lstk[64]; int nlstk;                         /* loop stack */
int lbo[256]; int lbl[256]; int lbline[256]; int nlbl;  /* labels: name off/len, line */

int fcs[8]; int fcsh; int fcst;   /* FIFO free queue, callee-saved (codes 1,2) */
int fcc[8]; int fcch; int fcct;   /* FIFO free queue, safe caller-saved r10/r11 (codes 3,4) */
int fcm[8]; int fcmh; int fcmt;   /* R20 FIFO free queue, marshalling regs (codes 5-8) */
int ae[8]; int av[8]; int ar[8]; int na;   /* active: end line, value, reg code */

int gnum; int gend;

int gdefop;   /* 1 if def op honored */
int gcons;    /* 1 if the line is a consumer-honored def (gdefop, or call/syscall: R20) */
int gcnt;     /* RHS v-ref count (R20: >6-arg calls are not honored) */
int arguse[40000];   /* R20: consumed as a call/syscall arg -> pool restricted to r10-r13 */

int gnoff; int gnlen;

int read_all() { int t; int n; t=0; n=__syscall(0,0,src,65536);
  while (n>0) { t=t+n; if (t+65536>50331648) { srcovf=1; n=0; } else { n=__syscall(0,0,src+t,65536); } } return t; }

int isal(int c) { if (c>=97 && c<=122) { return 1; } if (c>=65 && c<=90) { return 1; }
  if (c>=48 && c<=57) { return 1; } if (c==95) { return 1; } return 0; }

int splitlines() { int i; int st; nl=0; i=0; st=0;
  while (i<slen) {
    if (src[i]==10) { loff[nl]=st; llen[nl]=i-st; nl=nl+1; st=i+1; }
    i=i+1; }
  if (st<slen) { loff[nl]=st; llen[nl]=slen-st; nl=nl+1; }
  return 0; }

int skipsp(int p) { while (src[p]==32 || src[p]==9) { p=p+1; } return p; }

/* whole-word match of s at p (word chars must end after s) */
int word_at(int p, char *s) { int i; i=0;
  while (s[i]) { if (src[p+i]!=s[i]) { return 0; } i=i+1; }
  if (isal(src[p+i])) { return 0; }
  return 1; }

/* line starts (after indent) with word s */
int line_is(int ln, char *s) { int p; p=skipsp(loff[ln]); return word_at(p, s); }

/* find first '=' in line; -1 if none */
int eq_pos(int ln) { int p; int e; p=loff[ln]; e=p+llen[ln];
  while (p<e) { if (src[p]==61) { return p; } p=p+1; } return 0-1; }

/* line contains "call "/"syscall "/"calli " -- or "param" (R15: param lines read the
 * marshalling regs at fn entry; a value crossing one must be callee-saved) */
int has_call(int ln) { int p; int e; p=loff[ln]; e=p+llen[ln]-4;
  while (p<e) {
    if (src[p]==99 && src[p+1]==97 && src[p+2]==108 && src[p+3]==108) {
      if (src[p+4]==32) { return 1; }
      if (src[p+4]==105 && src[p+5]==32) { return 1; } }   /* calli */
    if (src[p]==112 && src[p+1]==97 && src[p+2]==114 && src[p+3]==97 && src[p+4]==109) { return 1; }
    if (src[p]==106 && src[p+1]==109 && src[p+2]==112) { return 1; }   /* R112: setjmp/longjmp both end in "jmp" -- a longjmp restores rbx/r12/r13/rsp, so any value live across the setjmp must be callee-saved (r12/r13) or slotted, exactly like an across-call value */
    p=p+1; }
  return 0; }

/* parse decimal at p; sets gnum + gend */
int parse_num(int p) { gnum=0;
  while (src[p]>=48 && src[p]<=57) { gnum=gnum*10+(src[p]-48); p=p+1; }
  gend=p; return 0; }

/* def line? after indent: vN : ... returns v or -1; sets gdefop_off (op token after '= ') */
int def_of(int ln) { int p; int v; int e;
  p=skipsp(loff[ln]); e=loff[ln]+llen[ln];
  if (src[p]!=118) { return 0-1; }
  if (src[p+1]<48 || src[p+1]>57) { return 0-1; }
  parse_num(p+1); v=gnum; p=gend;
  if (src[p]!=58) { return 0-1; }
  p=eq_pos(ln); if (p<0) { return 0-1; }
  p=skipsp(p+1);
  gdefop=0;
  if (word_at(p,"add")) { gdefop=1; } if (word_at(p,"sub")) { gdefop=1; }
  if (word_at(p,"mul")) { gdefop=1; } if (word_at(p,"and")) { gdefop=1; }
  if (word_at(p,"or"))  { gdefop=1; } if (word_at(p,"xor")) { gdefop=1; }
  if (word_at(p,"shl")) { gdefop=1; } if (word_at(p,"shr")) { gdefop=1; }
  if (word_at(p,"shu")) { gdefop=1; }
  if (word_at(p,"const")) { gdefop=1; }   /* R14: honored as a DEF op (mov rXd,imm32); const lines have no uses */
  if (word_at(p,"div")) { gdefop=1; } if (word_at(p,"mod")) { gdefop=1; }   /* R19 */
  if (word_at(p,"duv")) { gdefop=1; } if (word_at(p,"udm")) { gdefop=1; }
  if (word_at(p,"load")) { gdefop=1; } if (word_at(p,"get")) { gdefop=1; }   /* R22 */
  if (word_at(p,"getb")) { gdefop=1; }
  if (word_at(p,"lt"))  { gdefop=1; } if (word_at(p,"le"))  { gdefop=1; }
  if (word_at(p,"gt"))  { gdefop=1; } if (word_at(p,"ge"))  { gdefop=1; }
  if (word_at(p,"eq"))  { gdefop=1; } if (word_at(p,"ne"))  { gdefop=1; }
  if (word_at(p,"ltu")) { gdefop=1; } if (word_at(p,"gtu")) { gdefop=1; }
  if (word_at(p,"leu")) { gdefop=1; } if (word_at(p,"geu")) { gdefop=1; }
  gcons=gdefop;                            /* R20: call/syscall args are honored consumers */
  if (word_at(p,"ref")) { gdefop=1; }      /* R23: ref defs honored; gcons stays 0 -- `ref vX`
                                              aliases X, so address-taken values keep their slot */
  if (word_at(p,"call")) { gcons=2; }      /* ("calli" fails the word boundary -- excluded) */
  if (word_at(p,"syscall")) { gcons=2; }
  return v; }

/* count \bvN\b refs in [p,e) -> gcnt (R20: >6-arg call lines are not honored) */
int scan_count(int p, int e) { int prev; gcnt=0;
  while (p<e) {
    if (src[p]==118 && src[p+1]>=48 && src[p+1]<=57) {
      prev=0; if (p>0) { prev=isal(src[p-1]); }
      if (prev==0) {
        parse_num(p+1);
        if (isal(src[gend])==0) { gcnt=gcnt+1; p=gend; }
        else { p=p+1; } }
      else { p=p+1; } }
    else { p=p+1; } }
  return 0; }

/* scan uses (\bvN\b) in [p,e); consumer honored -> hon (2 = call/syscall arg position).
 * dv = the line's def v (not a use) */
int scan_uses(int p, int e, int ln, int hon, int dv) { int v; int prev;
  while (p<e) {
    if (src[p]==118 && src[p+1]>=48 && src[p+1]<=57) {
      prev=0; if (p>0) { prev=isal(src[p-1]); }
      if (prev==0) {
        parse_num(p+1);
        if (isal(src[gend])==0) {
          v=gnum;
          if (v<40000) {
            lu[v]=ln;
            if (v!=dv) { if (hon==0) { baduse[v]=1; }
                         if (hon==2) { arguse[v]=1; } }
            if (v>maxv) { maxv=v; } }
          p=gend; }
        else { p=p+1; } }
      else { p=p+1; } }
    else { p=p+1; } }
  return 0; }

/* label name at line (after "label "/"goto "): sets gnoff/gnlen */
int name_after(int ln, int skip) { int p;
  p=skipsp(loff[ln]); p=p+skip; p=skipsp(p);
  gnoff=p; while (isal(src[p])) { p=p+1; } gnlen=p-gnoff; return 0; }

int names_eq(int o1, int l1, int o2, int l2) { int i;
  if (l1!=l2) { return 0; } i=0;
  while (i<l1) { if (src[o1+i]!=src[o2+i]) { return 0; } i=i+1; }
  return 1; }

/* register code -> name rank for expiry ordering ('r10'<'r11'<'r12'<'r13') */
/* python sorts by NAME string: r10<r11<r12<r13<r8<r9<rdi<rsi */
int rrank(int c) { if (c==3) { return 0; } if (c==4) { return 1; }
  if (c==1) { return 2; } if (c==2) { return 3; }
  if (c==7) { return 4; } if (c==8) { return 5; }
  if (c==6) { return 6; } return 7; }

int emit_line(int ln) { int p; int e; p=loff[ln]; e=p+llen[ln];
  if (outn+llen[ln]+2>=50331648) { outovf=1; return 0; }
  while (p<e) { out[outn]=src[p]; outn=outn+1; p=p+1; }
  out[outn]=10; outn=outn+1; return 0; }

/* emit def line with @rNN inserted before the first '=' */
int emit_ann(int ln, int code) { int p; int e; int q;
  p=loff[ln]; e=p+llen[ln]; q=eq_pos(ln);
  if (outn+llen[ln]+16>=50331648) { outovf=1; return 0; }
  while (p<q) { out[outn]=src[p]; outn=outn+1; p=p+1; }
  out[outn]=64; outn=outn+1; out[outn]=114; outn=outn+1;    /* @r */
  if (code==1) { out[outn]=49; outn=outn+1; out[outn]=50; outn=outn+1; }   /* 12 */
  if (code==2) { out[outn]=49; outn=outn+1; out[outn]=51; outn=outn+1; }   /* 13 */
  if (code==3) { out[outn]=49; outn=outn+1; out[outn]=48; outn=outn+1; }   /* 10 */
  if (code==4) { out[outn]=49; outn=outn+1; out[outn]=49; outn=outn+1; }   /* 11 */
  if (code==5) { out[outn]=115; outn=outn+1; out[outn]=105; outn=outn+1; } /* si */
  if (code==6) { out[outn]=100; outn=outn+1; out[outn]=105; outn=outn+1; } /* di */
  if (code==7) { out[outn]=56; outn=outn+1; }                              /* 8 */
  if (code==8) { out[outn]=57; outn=outn+1; }                              /* 9 */
  out[outn]=32; outn=outn+1;
  while (p<e) { out[outn]=src[p]; outn=outn+1; p=p+1; }
  out[outn]=10; outn=outn+1; return 0; }

int fcc_push(int c) { fcc[fcct]=c; fcct=fcct+1; if (fcct==8) { fcct=0; } return 0; }
int fcm_push(int c) { fcm[fcmt]=c; fcmt=fcmt+1; if (fcmt==8) { fcmt=0; } return 0; }
int fcm_pop() { int c; if (fcmh==fcmt) { return 0; } c=fcm[fcmh]; fcmh=fcmh+1; if (fcmh==8) { fcmh=0; } return c; }
int fcs_push(int c) { fcs[fcst]=c; fcst=fcst+1; if (fcst==8) { fcst=0; } return 0; }
int fcc_pop() { int c; if (fcch==fcct) { return 0; } c=fcc[fcch]; fcch=fcch+1; if (fcch==8) { fcch=0; } return c; }
int fcs_pop() { int c; if (fcsh==fcst) { return 0; } c=fcs[fcsh]; fcsh=fcsh+1; if (fcsh==8) { fcsh=0; } return c; }
int release(int c) { if (c==1 || c==2) { fcs_push(c); return 0; }
  if (c==3 || c==4) { fcc_push(c); return 0; }
  fcm_push(c); return 0; }

/* expire actives with end < d. Python parity: releases happen in CURRENT list order (the
 * previous iteration's sorted order + the appended tail), then the survivors are sorted by
 * (end, regname, value) -- active.sort() runs after the filter in the Python. */
int expire(int d) { int i; int j; int k; int t;
  i=0; j=0;
  while (i<na) {
    if (ae[i]<d) { release(ar[i]); }
    else { ae[j]=ae[i]; av[j]=av[i]; ar[j]=ar[i]; j=j+1; }
    i=i+1; }
  na=j;
  /* insertion sort survivors by (end, rrank, value) */
  i=1;
  while (i<na) { j=i;
    while (j>0) {
      k=0;
      if (ae[j-1]>ae[j]) { k=1; }
      else { if (ae[j-1]==ae[j]) {
               if (rrank(ar[j-1])>rrank(ar[j])) { k=1; }
               else { if (rrank(ar[j-1])==rrank(ar[j]) && av[j-1]>av[j]) { k=1; } } } }
      if (k) { t=ae[j-1]; ae[j-1]=ae[j]; ae[j]=t;
               t=av[j-1]; av[j-1]=av[j]; av[j]=t;
               t=ar[j-1]; ar[j-1]=ar[j]; ar[j]=t;
               j=j-1; }
      else { j=0; } }
    i=i+1; }
  return 0; }

/* process one function spanning lines [fs, fe) */
int do_func(int fs, int fe) { int i; int v; int e; int p; int q; int hon; int dv;
  int nc; int s; int en2; int chg; int c; int isacross;
  /* reset per-function state */
  v=0; while (v<=maxv) { defs[v]=0-1; lu[v]=0-1; okdef[v]=0; baduse[v]=0; areg[v]=0; arguse[v]=0; v=v+1; }
  maxv=0; nrg=0; nlstk=0; nlbl=0;
  /* pass 1: defs, uses, regions, call lines */
  nc=0;
  i=fs;
  while (i<fe) {
    if (has_call(i)) { nc=nc+1; }
    cumcall[i]=nc;
    dv=def_of(i);
    hon=0;
    if (dv>=0) { hon=gcons; }
    if (line_is(i,"brz")) { hon=1; }
    if (line_is(i,"brc")) { hon=1; }      /* R3(opt): fused compare-branch, both operands */
    if (line_is(i,"ret")) { hon=1; }
    if (line_is(i,"store")) { hon=1; }    /* R22: store/storeb/put/putb -- all positions honored */
    if (line_is(i,"storeb")) { hon=1; }
    if (line_is(i,"put")) { hon=1; }
    if (line_is(i,"putb")) { hon=1; }
    if (dv>=0 && dv<40000) {                       /* R14: mutable values allowed -- okdef = AND over ALL defs */
      if (defs[dv]<0) { defs[dv]=i; okdef[dv]=1; if (dv>maxv) { maxv=dv; } }
      else { if (i>lu[dv]) { lu[dv]=i; } }        /* R22 fix: trailing dead defs extend the interval */
      if (gdefop==0) { okdef[dv]=0; } }
    /* use scan over RHS (after first '='), or whole line when no '=' */
    q=eq_pos(i); p=loff[i]; e=p+llen[i];
    if (q>=0) { p=q+1; }
    if (hon==2) { scan_count(p, e); if (gcnt>6) { hon=0; } }   /* R20: spill-path args stay slotted */
    scan_uses(p, e, i, hon, dv);
    /* regions */
    if (line_is(i,"loop")) { lstk[nlstk]=i; nlstk=nlstk+1; }
    /* R14 fix: jmp (the back-jump) pops the loop region, not pool -- ccc's pool closes brz-IFs */
    if (line_is(i,"jmp")) { if (nlstk>0) { nlstk=nlstk-1; rgs[nrg]=lstk[nlstk]; rge[nrg]=i; nrg=nrg+1; } }
    if (line_is(i,"label")) { name_after(i,5); lbo[nlbl]=gnoff; lbl[nlbl]=gnlen; lbline[nlbl]=i; nlbl=nlbl+1; }
    if (line_is(i,"goto")) { name_after(i,4);
      v=0; while (v<nlbl) {
        if (names_eq(gnoff,gnlen,lbo[v],lbl[v])) { rgs[nrg]=lbline[v]; rge[nrg]=i; nrg=nrg+1; v=nlbl; }
        else { v=v+1; } } }
    i=i+1; }
  /* pass 2: back-edge interval extension to fixpoint */
  chg=1;
  while (chg) { chg=0;
    v=0; while (v<=maxv) {
      if (defs[v]>=0) { e=lu[v]; if (e<defs[v]) { e=defs[v]; }
        i=0; while (i<nrg) {
          if (defs[v]<rgs[i] && rgs[i]<=e && e<rge[i]) { lu[v]=rge[i]; e=rge[i]; chg=1; }
          i=i+1; } }
      v=v+1; } }
  /* pass 3: linear scan in line order; emit */
  fcsh=0; fcst=0; fcch=0; fcct=0; na=0;
  fcsh=0; fcst=0; fcch=0; fcct=0; fcmh=0; fcmt=0;
  fcs_push(1); fcs_push(2); fcc_push(3); fcc_push(4);
  fcm_push(5); fcm_push(6); fcm_push(7); fcm_push(8);   /* R15/R20 */
  i=fs;
  while (i<fe) {
    dv=def_of(i);
    if (dv>=0 && dv<40000) {
      if (defs[dv]==i && okdef[dv]==1 && baduse[dv]==0 && lu[dv]>defs[dv]) {
        expire(i);
        s=defs[dv]; en2=lu[dv];
        isacross=0;   /* R21: the lu-line call is an arg read (before the clobber) -- excluded */
        if (en2>s+1) { if (cumcall[en2-1]-cumcall[s]>0) { isacross=1; } }
        c=0;
        if (isacross) { c=fcs_pop(); }
        else { if (arguse[dv]) { c=fcc_pop(); if (c==0) { c=fcs_pop(); } }
               else { c=fcm_pop(); if (c==0) { c=fcc_pop(); } if (c==0) { c=fcs_pop(); } } }
        if (c>0) { areg[dv]=c; ae[na]=en2; av[na]=dv; ar[na]=c; na=na+1; } } }
    if (dv>=0 && dv<40000 && areg[dv]>0) { emit_ann(i, areg[dv]); }   /* R14: every def line */
    else { emit_line(i); }
    i=i+1; }
  return 0; }

int main() { int i; int fs; int infunc;
  slen=read_all(); outn=0; maxv=39999;
  splitlines();
  /* pre lines pass through; functions begin at "ir main" or "fn " */
  infunc=0; fs=0; i=0;
  while (i<nl) {
    if (line_is(i,"ir") || line_is(i,"fn")) {
      if (infunc) { do_func(fs, i); }
      else { fs=0; while (fs<i) { emit_line(fs); fs=fs+1; } }
      infunc=1; fs=i; }
    i=i+1; }
  if (infunc) { do_func(fs, nl); }
  else { fs=0; while (fs<nl) { emit_line(fs); fs=fs+1; } }
  if (outovf==0 && srcovf==0) { __syscall(1, 1, out, outn); }
  return 0; }
