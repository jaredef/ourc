/* rzir-opt.c -- M5: the optimizer ported to ccc's C subset (self-hosted optimizer).
 * Port of rzir-opt.py R3: per-function fold (i32 wraparound; unsigned emission; no-fold on
 * out-of-range shifts) + zero-idiom copy-prop (both ends pure single-def) + dead-value
 * cascade + cmp/brz fusion (quiescent rounds only), iterated to a fixpoint. Identity floor:
 * a stream with nothing to do emits byte-identically.
 * Pure = single-def, not address-taken (ref operand), op in {const,arith,cmp,ref}.
 * Mutables are the fold barrier; address-taken values are untouchable (aliasing).
 * Subset notes: all locals at function top; no ternary; subst[] is -1-initialized explicitly
 * (global zero-init would read as "substitute to v0").
 * Build: ccc.bin < rzir-opt.c | rzir-ld | rzir-opt | rzir-ra | cf.
 * Gate: byte-identity vs rzir-opt.py, opt-by-opt fixpoint, the full oracle. */

/* C-OPT-BUF-8MB-OVERFLOW: ld hands opt 8,002,021 bytes on the SEM0 self-build -- 2,021 past the old
   8,000,000 -- and the overrun spliced binary garbage into the emitted text (15 malformed lines at HEAD,
   growing with source size, and a non-ELF image at some sizes). ld and ra were widened for this class
   already; opt was the stage that fix missed. C-TWIN raised to 10,000,000 (2026-09-01): a 9.16MB post-ld stream
   (SEM0 + enum-PartialEq) overflowed the old 9M src[]. The binding constraint is the M0 audit-floor cf (cf-m0), which
   miscompiles this file NON-MONOTONICALLY by total global footprint: cf-m0 builds a WORKING opt at 9.5-11M (footprint
   <=~44MB) but HANGS/segfaults at 11.2/11.4/11.8/12M and runs again at 11.6M -- a whole-program register-allocation
   collision, not a clean ceiling (see sub-arcs/.../opt-twin-mint-above-9mb/). 10M sits mid-safe-band; cf-m0 builds it
   correctly and it holds the 9.16MB stream. NOTE: rzir-opt.rs (the all-Rust twin) is NOT bumped -- its footprint is
   ~56MB at 9M (str_arena + wider line arrays), already at the cf-m0 edge (9.5M segfaults there), so it stays at 9M.
   The twins therefore diverge in buffer size above 9M; they remain behaviorally identical on their shared range
   (<=9M) and on the 6.75MB DDC corpus, which is what the DDC gate checks. Growing opt past ~11M needs the cf-m0
   codegen bug root-fixed first (keeper-gated backend-memory-model). */
char src[50331648]; int slen;
char out[50331648]; int outn; int srcovf; int outovf;
char scr[4000000]; int scrn;      /* scratch: rewritten lines (folds, brc) */

int gloff[2000000]; int gllen[2000000]; int gnl;   /* global line split */
int loff[2000000]; int llen[2000000]; int nl;      /* working window (one function) */
int lsrc[2000000];                 /* 0 = src buffer, 1 = scr buffer */
int dropf[2000000];                /* line dropped */

int defs[40000];    /* first def line, -1 */
int dcount[40000];  /* number of defs */
int okpure[40000];  /* single pure def (const/arith/cmp/ref), not address-taken */
int isconst[40000]; /* pure const def */
int cval[40000];    /* const value (i32 wrapped) */
int usec[40000];    /* use count */
int useln[40000];   /* a use line (valid when usec==1) */
int ataken[40000];  /* address-taken (ref operand) */
int subst[40000];   /* -1 or replacement vN */
int maxv;

int gnum; int gend;
int gop;           /* def op: 0 none, 1 const, 2 arith, 3 cmp, 4 ref, 5 other */
int garith;        /* 1 add 2 sub 3 mul 4 and 5 or 6 xor 7 shl 8 shr 9 shu */
int gcmp;          /* 1 lt 2 le 3 gt 4 ge 5 eq 6 ne 7 ltu 8 gtu 9 leu 10 geu */
int gfold; int gfoldok;
int gA; int gB;
int nbuf[16];

int read_all() { int t; int n; t=0; n=__syscall(0,0,src,65536);
  while (n>0) { t=t+n; if (t+65536>50331648) { srcovf=1; n=0; } else { n=__syscall(0,0,src+t,65536); } } return t; }

int isal(int c) { if (c>=97 && c<=122) { return 1; } if (c>=65 && c<=90) { return 1; }
  if (c>=48 && c<=57) { return 1; } if (c==95) { return 1; } return 0; }

int lchar(int i, int p) {
  if (p>=llen[i]) { return 10; }   /* virtual newline: src lines carry a real one, scratch
                                      lines would otherwise expose STALE scratch from a prior
                                      function (isal(stale) broke token boundaries) */
  if (lsrc[i]) { return scr[loff[i]+p]; } return src[loff[i]+p]; }

int skipsp(int i, int p) { while (lchar(i,p)==32 || lchar(i,p)==9) { p=p+1; } return p; }

int word_at(int i, int p, char *s) { int k; k=0;
  while (s[k]) { if (lchar(i,p+k)!=s[k]) { return 0; } k=k+1; }
  if (isal(lchar(i,p+k))) { return 0; }
  return 1; }

int parse_num(int i, int p) { gnum=0;
  while (lchar(i,p)>=48 && lchar(i,p)<=57) { gnum=gnum*10+(lchar(i,p)-48); p=p+1; }
  gend=p; return 0; }

int eq_pos(int i) { int p; p=0;
  while (p<llen[i]) { if (lchar(i,p)==61) { return p; } p=p+1; }
  return 0-1; }

int line_is(int i, char *s) { int p; p=skipsp(i,0); return word_at(i,p,s); }

/* def line: vN : ... = OP; returns vN or -1; sets gop/garith/gcmp */
int def_of(int i) { int p; int v;
  p=skipsp(i,0);
  if (lchar(i,p)!=118) { return 0-1; }
  if (lchar(i,p+1)<48 || lchar(i,p+1)>57) { return 0-1; }
  parse_num(i,p+1); v=gnum; p=gend;
  if (lchar(i,p)!=58) { return 0-1; }
  p=eq_pos(i); if (p<0) { return 0-1; }
  p=skipsp(i,p+1);
  gop=5; garith=0; gcmp=0;
  if (word_at(i,p,"const")) { gop=1; }
  if (word_at(i,p,"ref"))   { gop=4; }
  if (word_at(i,p,"add")) { gop=2; garith=1; } if (word_at(i,p,"sub")) { gop=2; garith=2; }
  if (word_at(i,p,"mul")) { gop=2; garith=3; } if (word_at(i,p,"and")) { gop=2; garith=4; }
  if (word_at(i,p,"or"))  { gop=2; garith=5; } if (word_at(i,p,"xor")) { gop=2; garith=6; }
  if (word_at(i,p,"shl")) { gop=2; garith=7; } if (word_at(i,p,"shr")) { gop=2; garith=8; }
  if (word_at(i,p,"shu")) { gop=2; garith=9; }
  if (word_at(i,p,"lt"))  { gop=3; gcmp=1; }  if (word_at(i,p,"le"))  { gop=3; gcmp=2; }
  if (word_at(i,p,"gt"))  { gop=3; gcmp=3; }  if (word_at(i,p,"ge"))  { gop=3; gcmp=4; }
  if (word_at(i,p,"eq"))  { gop=3; gcmp=5; }  if (word_at(i,p,"ne"))  { gop=3; gcmp=6; }
  if (word_at(i,p,"ltu")) { gop=3; gcmp=7; }  if (word_at(i,p,"gtu")) { gop=3; gcmp=8; }
  if (word_at(i,p,"leu")) { gop=3; gcmp=9; }  if (word_at(i,p,"geu")) { gop=3; gcmp=10; }
  return v; }

/* first two vN operands after '='; returns count */
int rhs_ops(int i, int dv) { int p; int n; int prev;
  p=eq_pos(i); if (p<0) { return 0; }
  p=p+1; n=0;
  while (p<llen[i]) {
    if (lchar(i,p)==118 && lchar(i,p+1)>=48 && lchar(i,p+1)<=57) {
      prev=0; if (p>0) { prev=isal(lchar(i,p-1)); }
      if (prev==0) {
        parse_num(i,p+1);
        if (isal(lchar(i,gend))==0) {
          if (n==0) { gA=gnum; } if (n==1) { gB=gnum; }
          n=n+1; p=gend; }
        else { p=p+1; } }
      else { p=p+1; } }
    else { p=p+1; } }
  return n; }

/* unsigned < via sign flip; logical shifts (ccc >> is arithmetic) */
int ult(int a, int b) { int x; int y; x=a+(0-2147483647-1); y=b+(0-2147483647-1);
  if (x<y) { return 1; } return 0; }
int lshr1(int v) { return (v >> 1) & 2147483647; }
int lshr(int v, int n) { int m; if (n==0) { return v; }
  m=2147483647 >> (n-1);
  return (lshr1(v) >> (n-1)) & m; }

/* fold garith(isar=1)/gcmp(isar=0) on a,b -> gfold; gfoldok=0 when unsafe */
/* NOTE (2026-07-09, darwin-chain-self-host): the .py oracle now refuses to fold
 * when an operand exceeds 32 bits (Darwin GBASE globals are 64-bit consts; the
 * i32 fold truncates them -- see substrate-traps T22). This C port's int-width
 * cval[] cannot even represent such consts; widening it is an obligation of the
 * rzir-opt-darwin native rebuild (M-rung there), not patched here.
 * UPDATE (2026-07-09, darwin-chain-self-host O1): the Darwin port LANDED the width
 * machinery -- see its rzir-opt-darwin.c (cvlo/cvhf/cvng split const tables + pre-fold1
 * guard mirroring the py refusal, negatives included).  OBLIGATION for this file's next
 * M-rung: adopt the same guard + a 64-bit-capable cval (behavior-neutral on Linux
 * corpora) so the two ports do not fork semantically.  Also witnessed there: this
 * port's cmp/brz FUSION adjacency (`useln[dv]==i+1`) diverges from the oracle when a
 * DROPPED def sits between the cmp and the brz -- py compacts dropped lines before
 * fusing, so the check must target the next NON-dropped line (latent here; real
 * mismatches on the Darwin corpus: en/strtest/ctest/ccc-scale units). */
int fold1(int isar, int op, int a, int b) {
  gfoldok=1;
  if (isar) {
    if (op==1) { gfold=a+b; return 0; }
    if (op==2) { gfold=a-b; return 0; }
    if (op==3) { gfold=a*b; return 0; }
    if (op==4) { gfold=a&b; return 0; }
    if (op==5) { gfold=a|b; return 0; }
    if (op==6) { gfold=a^b; return 0; }
    if (b<0 || b>31) { gfoldok=0; return 0; }
    if (op==7) { gfold=a<<b; return 0; }
    if (op==8) { gfold=a>>b; return 0; }
    if (op==9) { gfold=lshr(a,b); return 0; }
    gfoldok=0; return 0; }
  if (op==1) { if (a<b) { gfold=1; } else { gfold=0; } return 0; }
  if (op==2) { if (a<=b) { gfold=1; } else { gfold=0; } return 0; }
  if (op==3) { if (a>b) { gfold=1; } else { gfold=0; } return 0; }
  if (op==4) { if (a>=b) { gfold=1; } else { gfold=0; } return 0; }
  if (op==5) { if (a==b) { gfold=1; } else { gfold=0; } return 0; }
  if (op==6) { if (a!=b) { gfold=1; } else { gfold=0; } return 0; }
  if (op==7) { gfold=ult(a,b); return 0; }
  if (op==8) { gfold=ult(b,a); return 0; }
  if (op==9) { if (ult(b,a)) { gfold=0; } else { gfold=1; } return 0; }
  if (op==10) { if (ult(a,b)) { gfold=0; } else { gfold=1; } return 0; }
  gfoldok=0; return 0; }

int sc(int c) { if (scrn+1>=4000000) { outovf=1; return 0; } scr[scrn]=c; scrn=scrn+1; return 0; }
int scs(char *s) { int k; k=0; while (s[k]) { sc(s[k]); k=k+1; } return 0; }
int scu(int v) { int q; int r; int t;   /* UNSIGNED decimal of an i32 bit pattern */
  t=0;
  while (1) {
    q=lshr1(v)/5;
    r=v-q*10;
    if (r>=10) { q=q+1; r=r-10; }
    if (r<0) { q=q-1; r=r+10; }
    nbuf[t]=r; t=t+1; v=q;
    if (v==0) { break; } }
  while (t>0) { t=t-1; sc(48+nbuf[t]); }
  return 0; }

int rw_const(int i, int dv, int v) { int st;
  st=scrn;
  scs("  v"); scu(dv); scs(": i32 = const "); scu(v);
  loff[i]=st; llen[i]=scrn-st; lsrc[i]=1; return 0; }

int canon(int v) { while (v<40000 && subst[v]>=0) { v=subst[v]; } return v; }

int emit_line(int i) { int p; int c; int prev; int t; int q; int r;
  if (outn+llen[i]+64>=50331648) { outovf=1; return 0; }
  p=0;
  while (p<llen[i]) {
    c=lchar(i,p);
    if (c==118 && lchar(i,p+1)>=48 && lchar(i,p+1)<=57) {
      prev=0; if (p>0) { prev=isal(lchar(i,p-1)); }
      if (prev==0) {
        parse_num(i,p+1);
        if (isal(lchar(i,gend))==0) {
          out[outn]=118; outn=outn+1;
          q=canon(gnum); t=0;
          while (1) { r=q-(q/10)*10; nbuf[t]=r; t=t+1; q=q/10; if (q==0) { break; } }
          while (t>0) { t=t-1; out[outn]=48+nbuf[t]; outn=outn+1; }
          p=gend; }
        else { out[outn]=c; outn=outn+1; p=p+1; } }
      else { out[outn]=c; outn=outn+1; p=p+1; } }
    else { out[outn]=c; outn=outn+1; p=p+1; } }
  out[outn]=10; outn=outn+1; return 0; }

/* analyze the working window: fill per-value tables (subst reset to -1) */
int analyze() { int i; int v; int p; int dv; int prev; int isref; int sg;
  v=0; while (v<=maxv) { defs[v]=0-1; dcount[v]=0; okpure[v]=0; isconst[v]=0;
    usec[v]=0; useln[v]=0-1; ataken[v]=0; v=v+1; }
  maxv=0;   /* subst is PERSISTENT across rounds (reset once per function in do_func):
               copy-prop's mappings must survive to emission -- drops persist in dropf,
               and a reset here orphans every dropped def's remaining uses */
  i=0;
  while (i<nl) {
    if (dropf[i]) { i=i+1; continue; }
    dv=def_of(i);
    isref=0;
    if (dv>=0 && dv<40000) {
      if (dv>maxv) { maxv=dv; }
      if (defs[dv]<0) { defs[dv]=i; if (gop<5) { okpure[dv]=1; } }
      dcount[dv]=dcount[dv]+1;
      if (gop==4) { isref=1; }
      if (gop==1 && dcount[dv]==1) {
        p=eq_pos(i); p=skipsp(i,p+1); p=p+5; p=skipsp(i,p);
        sg=1; if (lchar(i,p)==45) { sg=0-1; p=p+1; }               /* negative const (e.g. -2147483648); INT_MIN round-trips via double i32 overflow */
        parse_num(i,p); isconst[dv]=1; cval[dv]=sg*gnum; } }
    p=eq_pos(i); if (p<0) { p=0; } else { p=p+1; }
    while (p<llen[i]) {
      if (lchar(i,p)==118 && lchar(i,p+1)>=48 && lchar(i,p+1)<=57) {
        prev=0; if (p>0) { prev=isal(lchar(i,p-1)); }
        if (prev==0) {
          parse_num(i,p+1);
          if (isal(lchar(i,gend))==0) {
            v=gnum;
            if (v<40000 && v!=dv) {
              usec[v]=usec[v]+1; useln[v]=i;
              if (v>maxv) { maxv=v; }
              if (isref) { ataken[v]=1; } }
            p=gend; }
          else { p=p+1; } }
        else { p=p+1; } }
      else { p=p+1; } }
    i=i+1; }
  v=0; while (v<=maxv) {
    if (dcount[v]!=1 || ataken[v]) { okpure[v]=0; isconst[v]=0; }
    v=v+1; }
  return 0; }

/* recount uses over live lines with canon applied (for DVE) */
int recount() { int i; int v; int p; int dv; int prev;
  v=0; while (v<=maxv) { usec[v]=0; v=v+1; }
  i=0;
  while (i<nl) {
    if (dropf[i]==0) {
      dv=def_of(i);
      p=eq_pos(i); if (p<0) { p=0; } else { p=p+1; }
      while (p<llen[i]) {
        if (lchar(i,p)==118 && lchar(i,p+1)>=48 && lchar(i,p+1)<=57) {
          prev=0; if (p>0) { prev=isal(lchar(i,p-1)); }
          if (prev==0) {
            parse_num(i,p+1);
            if (isal(lchar(i,gend))==0) {
              v=canon(gnum);
              if (v<40000 && v!=dv) { usec[v]=usec[v]+1; }
              p=gend; }
            else { p=p+1; } }
          else { p=p+1; } }
        else { p=p+1; } } }
    i=i+1; }
  return 0; }

/* one round; returns 1 if anything changed */
int opt_round() { int i; int dv; int n; int changed; int did; int sar; int sop;
  int z; int x; int grew; int v; int st;
  analyze();
  changed=0;
  i=0;                                   /* FOLD */
  while (i<nl) {
    if (dropf[i]==0) {
      dv=def_of(i);
      if (dv>=0 && dv<40000 && okpure[dv] && isconst[dv]==0 && (gop==2 || gop==3)) {
        sar=0; sop=gcmp; if (gop==2) { sar=1; sop=garith; }
        n=rhs_ops(i,dv);
        if (n==2 && gA<40000 && gB<40000 && isconst[gA] && isconst[gB]) {
          fold1(sar,sop,cval[gA],cval[gB]);
          if (gfoldok) { rw_const(i,dv,gfold); changed=1; } } } }
    i=i+1; }
  if (changed) { return 1; }
  did=0;
  i=0;                                   /* COPY-PROP */
  while (i<nl) {
    if (dropf[i]==0) {
      dv=def_of(i);
      if (dv>=0 && dv<40000 && okpure[dv] && gop==2 && garith==1 && subst[dv]<0) {
        n=rhs_ops(i,dv);
        if (n==2 && gA<40000 && gB<40000) {
          z=0; x=0-1;                     /* EXACTLY one zero operand (python parity: a  */
          if (isconst[gA] && cval[gA]==0) { z=z+1; x=gB; }   /* both-zero add is NOT a copy) */
          if (isconst[gB] && cval[gB]==0) { z=z+1; x=gA; }
          if (z==1 && x>=0) {
            if (okpure[x] || isconst[x]) { subst[dv]=x; dropf[i]=1; did=1; } } } } }
    i=i+1; }
  grew=1;                                /* DVE cascade */
  while (grew) { grew=0;
    recount();
    v=0;
    while (v<=maxv) {
      if (okpure[v] && subst[v]<0 && usec[v]==0 && defs[v]>=0) {
        if (dropf[defs[v]]==0) { dropf[defs[v]]=1; grew=1; did=1; } }
      v=v+1; } }
  if (did) { return 1; }
  analyze();                             /* FUSION (quiescent round) */
  i=0;
  while (i<nl-1) {
    if (dropf[i]==0) {
      dv=def_of(i);
      if (dv>=0 && dv<40000 && okpure[dv] && gop==3 && usec[dv]==1 && useln[dv]==i+1) {
        if (dropf[i+1]==0 && line_is(i+1,"brz")) {
          sop=gcmp;
          n=rhs_ops(i,dv);
          if (n==2) {
            st=scrn;
            scs("  brc ");
            if (sop==1) { scs("lt"); } if (sop==2) { scs("le"); }
            if (sop==3) { scs("gt"); } if (sop==4) { scs("ge"); }
            if (sop==5) { scs("eq"); } if (sop==6) { scs("ne"); }
            if (sop==7) { scs("ltu"); } if (sop==8) { scs("gtu"); }
            if (sop==9) { scs("leu"); } if (sop==10) { scs("geu"); }
            sc(32); sc(118); scu(gA); sc(32); sc(118); scu(gB);
            loff[i]=st; llen[i]=scrn-st; lsrc[i]=1;
            dropf[i+1]=1; changed=1; } } } }
    i=i+1; }
  return changed; }

int do_func(int fs, int fe) { int i; int r;
  nl=0; scrn=0; maxv=39999;   /* clear the FULL table range: a small function after a large one
                                must not inherit stale subst/dcount above its own max vN */
  i=0; while (i<40000) { subst[i]=0-1; i=i+1; }
  i=fs;
  while (i<fe) { loff[nl]=gloff[i]; llen[nl]=gllen[i]; lsrc[nl]=0; dropf[nl]=0; nl=nl+1; i=i+1; }
  r=1;
  while (r) { r=opt_round(); }
  analyze();
  i=0;
  while (i<nl) { if (dropf[i]==0) { emit_line(i); } i=i+1; }
  return 0; }

int emit_global(int j) {
  nl=1; loff[0]=gloff[j]; llen[0]=gllen[j]; lsrc[0]=0; dropf[0]=0;
  emit_line(0); return 0; }

int main() { int i; int st; int fs; int infunc; int j; int hdr;
  slen=read_all(); outn=0; maxv=39999;
  i=0; while (i<40000) { subst[i]=0-1; i=i+1; }   /* zero-init would read as "-> v0" */
  gnl=0; i=0; st=0;
  while (i<slen) {
    if (src[i]==10) { gloff[gnl]=st; gllen[gnl]=i-st; gnl=gnl+1; st=i+1; }
    i=i+1; }
  if (st<slen) { gloff[gnl]=st; gllen[gnl]=slen-st; gnl=gnl+1; }
  infunc=0; fs=0; i=0;
  while (i<gnl) {
    nl=1; loff[0]=gloff[i]; llen[0]=gllen[i]; lsrc[0]=0; dropf[0]=0;
    hdr=0;
    if (line_is(0,"ir")) { hdr=1; }
    if (line_is(0,"fn")) { hdr=1; }
    if (hdr) {
      if (infunc) { do_func(fs,i); }
      else { j=0; while (j<i) { emit_global(j); j=j+1; } }
      infunc=1; fs=i; }
    i=i+1; }
  if (infunc) { do_func(fs,gnl); }
  else { j=0; while (j<gnl) { emit_global(j); j=j+1; } }
  if (srcovf==0 && outovf==0) { __syscall(1,1,out,outn); }
  return 0; }
