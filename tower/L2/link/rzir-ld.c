/* rzir-ld.c -- derived rzir cross-file linker (C port of rzir-ld.py; M6: host-free tcc build).
 * Reads the concatenated rzir units on stdin. Globals: collect `gdef NAME SIZE`, assign 8-aligned
 * addresses from 0x700000, rewrite every `vN: i32 = gref NAME` -> `const <addr>`, drop gdefs.
 * Functions: split into `ir main`/`fn NAME` blocks, DEDUP by name, emit the main block first.
 * ccc subset: globals-first, no ternary, locals at function top. */
/* C-LD-BUF-8MB-OVERFLOW (sibling of cf.rs's C-CF-BUF-8MB-OVERFLOW): read_all() fills src with no bounds check
   and `out` is the next static, so an input past 8MB silently corrupted the linked stream -- characters dropped
   mid-token in the tail. SEM0's self-emit reached 7,998,901 B, i.e. ~1KB of headroom, and every attempt to grow
   the parser past ~40 lines broke the S1/S9 gates for that reason and NOT for any property of the added code.
   Measured: 32 pad fns (rzir 8,002,433) pass, 40 (8,003,321) fail; with src at 12MB both pass. The arrays are
   BSS, so the emitted binary is byte-for-byte the same size (10190 B) -- widening costs no image. */
char src[24000000]; int slen;
char out[24000000]; int outn;
int gloff[2000000]; int gllen[2000000]; int gnl;                  /* all input lines */
int gsoff[40000]; int gslen[40000]; int gsaddr[40000]; int ngs; /* global symtab: name off/len in src, addr */
int bstart[40000]; int bnoff[40000]; int bnlen[40000]; int bmain[40000]; int bdrop[40000]; int nb;  /* function blocks */
int gnum; int gend;

int read_all() { int t; int n; t=0; n=__syscall(0,0,src,65536);
  while (n>0) { t=t+n; n=__syscall(0,0,src+t,65536); } return t; }

int isal(int c) { if (c>=97 && c<=122) { return 1; } if (c>=65 && c<=90) { return 1; }
  if (c>=48 && c<=57) { return 1; } if (c==95) { return 1; } return 0; }

int lc(int i, int p) { if (p<0) { return 10; } if (p>=gllen[i]) { return 10; } return src[gloff[i]+p]; }

int skipsp(int i, int p) { while (lc(i,p)==32 || lc(i,p)==9) { p=p+1; } return p; }

int word_at(int i, int p, char *s) { int k; k=0;
  while (s[k]) { if (lc(i,p+k)!=s[k]) { return 0; } k=k+1; }
  if (isal(lc(i,p+k))) { return 0; }
  return 1; }

int idlen(int i, int p) { int k; k=0; while (isal(lc(i,p))) { k=k+1; p=p+1; } return k; }  /* identifier length at line i pos p */
/* C-LD-QUALIFIED-NAME-COLLISION: a BLOCK name may be qualified (`fn S::a`), and idlen stops at `:` because
   isal is alnum+underscore. So `fn S::a` and `fn S::b` both scanned as the name "S" and the dedup-by-name pass
   below deleted the second as a duplicate of the first -- i.e. every associated fn of a type except the first
   was silently dropped from the program. Block names use this scan instead; idlen is left alone because the
   global symtab and gref paths must keep stopping at `:`. */
int fnlen(int i, int p) { int k; k=0; while (isal(lc(i,p)) || lc(i,p)==58 || lc(i,p)==36) { k=k+1; p=p+1; } return k; }  /* +36=$: mono-mangled names `alloc$T` share a truncated name and were false-deduped -- same class as `::` above */

int parse_num(int i, int p) { gnum=0;
  while (lc(i,p)>=48 && lc(i,p)<=57) { gnum=gnum*10+(lc(i,p)-48); p=p+1; }
  gend=p; return 0; }

int split_lines() { int i; int st; i=0; st=0; gnl=0;
  while (i<slen) { if (src[i]==10) { gloff[gnl]=st; gllen[gnl]=i-st; gnl=gnl+1; st=i+1; } i=i+1; }
  if (st<slen) { gloff[gnl]=st; gllen[gnl]=slen-st; gnl=gnl+1; }
  return 0; }

/* two src substrings equal? */
int sub_eq(int o1, int l1, int o2, int l2) { int k;
  if (l1!=l2) { return 0; }
  k=0; while (k<l1) { if (src[o1+k]!=src[o2+k]) { return 0; } k=k+1; } return 1; }

/* find global symtab index for name at src[off..off+len); -1 none */
int gs_find(int off, int len) { int k; k=0;
  while (k<ngs) { if (sub_eq(gsoff[k], gslen[k], off, len)) { return k; } k=k+1; } return 0-1; }

/* find position of word `s` in line i (word boundary); -1 none */
int find_word(int i, char *s) { int p; p=0;
  while (p<gllen[i]) { if (word_at(i,p,s)) { return p; } p=p+1; } return 0-1; }

int ec(int c) { out[outn]=c; outn=outn+1; return 0; }
int es(char *s) { int k; k=0; while (s[k]) { out[outn]=s[k]; outn=outn+1; k=k+1; } return 0; }
int en(int n) { char b[16]; int i; if (n==0) { ec(48); return 0; }
  i=0; while (n>0) { b[i]=48+n%10; n=n/10; i=i+1; }
  while (i>0) { i=i-1; ec(b[i]); } return 0; }

/* emit line i verbatim (no trailing newline) */
int emit_raw(int i) { int k; k=0; while (k<gllen[i]) { out[outn]=src[gloff[i]+k]; outn=outn+1; k=k+1; } return 0; }

/* emit one line of a block, transforming gref -> const, dropping gdef; adds a newline */
int emit_line(int i) { int p; int gp; int noff; int nlen; int gi; int k;
  p=skipsp(i,0);
  if (word_at(i,p,"gdef")) { return 0; }                       /* safety: drop stray gdef */
  gp=find_word(i,"gref");
  if (gp>=0) {
    noff=gloff[i]+skipsp(i,gp+4);                              /* name after `gref ` */
    nlen=idlen(i,skipsp(i,gp+4));
    gi=gs_find(noff, nlen);
    if (gi>=0) {
      k=0; while (k<gp) { out[outn]=src[gloff[i]+k]; outn=outn+1; k=k+1; }  /* prefix `  vN: i32 = ` */
      es("const "); en(gsaddr[gi]); ec(10); return 0; }
  }
  emit_raw(i); ec(10); return 0; }

/* emit block b: lines [bstart[b], next) */
int emit_block(int b) { int i; int e;
  if (b+1<nb) { e=bstart[b+1]; } else { e=gnl; }
  i=bstart[b]; while (i<e) { emit_line(i); i=i+1; } return 0; }

int main() { int i; int p; int cur; int arena_ovf; int noff; int nlen; int nameoff; int namelen; int isir; int prevname; int j; int mb;
  slen=read_all(); outn=0; ngs=0; nb=0;
  split_lines();

  /* pass 1: global symtab (dedup by name) */
  cur=7340032; arena_ovf=0;                                                 /* 0x700000 */
  i=0; while (i<gnl) {
    p=skipsp(i,0);
    if (word_at(i,p,"gdef")) {
      p=skipsp(i,p+4);
      noff=gloff[i]+p; nlen=idlen(i,p);
      if (gs_find(noff,nlen)<0) {
        p=skipsp(i,p+nlen); parse_num(i,p);
        gsoff[ngs]=noff; gslen[ngs]=nlen; gsaddr[ngs]=cur; ngs=ngs+1;
        cur=cur+(gnum+7)/8*8;
        if (cur<0) arena_ovf=1;                 /* C-ARENA-LD-I32: wrapped past INT_MAX */
        if (cur>2113929216) arena_ovf=1; } }    /* fbase would pass cf's 0x7F000000 */
    i=i+1; }

  /* pass 2: identify blocks (ir main / fn NAME), dedup by name */
  i=0; while (i<gnl) {
    p=skipsp(i,0);
    isir=0; nameoff=0-1; namelen=0;
    if (word_at(i,p,"ir")) { p=skipsp(i,p+2); if (word_at(i,p,"main")) { isir=1; nameoff=gloff[i]+p; namelen=fnlen(i,p); } }
    else if (word_at(i,p,"fn")) { p=skipsp(i,p+2); nameoff=gloff[i]+p; namelen=fnlen(i,p); }
    if (nameoff>=0) {
      bstart[nb]=i; bnoff[nb]=nameoff; bnlen[nb]=namelen; bmain[nb]=isir; bdrop[nb]=0;
      j=0; while (j<nb) { if (sub_eq(bnoff[j],bnlen[j],nameoff,namelen)) { bdrop[nb]=1; } j=j+1; }
      nb=nb+1; }
    i=i+1; }

  /* CFML per-program frame base: when the arenas exceed the default 128MB frame base, hand the arena top
   * (cur, after pass 1) to cf via a `heaptop` directive so cf places the frame arena / sp-save / memsz ABOVE
   * the arenas (else the fixed frame region collides with a large program's heap). Below 128MB the default cf
   * layout is already above the arenas, so NO directive is emitted -> byte-identical output (every provenance
   * and golden input has arena_top < 128MB). opt/ra forward the top-level pre-block directive unchanged; cf
   * consumes it; the M0 cf never sees it (it only lowers small programs). */
  /* C-ARENA-LD-I32: poison heaptop on overflow so cf's 0x7F000000 guard refuses (mirrors the Rust twin; DDC pair). */
  if (cur>134217728 || arena_ovf) { es("heaptop "); if (arena_ovf) en(2147483647); else en(cur); ec(10); }

  /* emit: main block first, then the rest in order (non-dropped, non-main) */
  mb=0-1; j=0; while (j<nb) { if (bmain[j] && bdrop[j]==0 && mb<0) { mb=j; } j=j+1; }
  if (mb>=0) { emit_block(mb); }
  j=0; while (j<nb) { if (bmain[j]==0 && bdrop[j]==0) { emit_block(j); } j=j+1; }

  __syscall(1,1,out,outn);
  return 0; }
