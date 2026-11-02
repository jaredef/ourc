/* rzcpp: a DERIVED C preprocessor. Authored in ccc's subset, lowered by
 * ccc | rzir-ld | rzir-opt | rzir-ra | cf. Host cpp is the ORACLE (differential), never source.
 * P1: comment strip + line splice; object #define/#undef; #ifdef/#ifndef/#else/#endif; object
 *     macro expansion (rescanned, self-ref guarded).
 * P2: FUNCTION-LIKE macros -- arg collection (paren-nested), whitespace-trimmed args, PRE-EXPANSION
 *     of args (C "expand args first" rule; raw only next to # / ##), # stringize, ## paste, rescan.
 * P3: #if/#elif -- `defined X`/`defined(X)` resolution, macro-expanded condition, full C-precedence
 *     integer const-expr evaluator (|| && | ^ & == != rel << >> + - * / % unary ! ~ - +; ident->0).
 * INCLUDE: #include "local"/<sys> via a work[] high-water (per-file region) + -I search; argv
 *     -D/-I (marshalled argv). TODO: P4 the header set (stdarg.h completes va_arg); P5 tcc gate. */

char inbuf[600000]; int inlen;          /* raw current-file text (reused per file via recursion save) */
char work[900000]; int worklen;         /* comment-stripped + line-spliced text being scanned */
char out[2000000]; int outn;            /* preprocessed output */
char mtxt[800000]; int mtxtn;           /* macro name+body arena */
int mnoff[16384]; int mnlen[16384]; int mboff[16384]; int mblen[16384]; int nmac;  /* macro table */
int mfunc[16384]; int mpoff[16384]; int mplen[16384];  /* function-like flag + param-list text ("a,b,c") in mtxt */
char xb[1600000]; int xbp;              /* expansion output arena (append-only, flushed per line) */
char tb[500000]; int tbp;               /* substituted-body scratch (stack: save/restore tbp per fn-macro) */
char ab[800000]; int abp;               /* pre-expanded macro arguments (stack per fn-macro invocation) */
int cemit[256]; int ctaken[256]; int csp;  /* conditional stack: this-branch-emits? any-branch-taken? */
char idbuf[4096]; int idlen;            /* scratch: current identifier */
char pathbuf[4096]; char pathbuf2[4096]; /* scratch: include name + candidate path */
char incdir[4096]; int incdirlen;       /* one system include dir (argv-provided or default) */
int whi;                                /* work[] free-offset (high-water): each #include gets its own region above the outer */
char incp[16384]; int incoff[64]; int inclen[64]; int nincd; int incpn;  /* -I include search dirs */
int expanding[64]; int nexp;            /* self-reference guard: macro indices currently expanding */
char cbuf[262144]; int cbn;             /* #if condition after `defined` resolution (before macro expansion) */
int ev_pos; int ev_end; int ev_oplen; int ev_opprec;  /* #if expression evaluator cursor (over xb) */

int es(char *s) { int i; i=0; while (s[i]) { out[outn]=s[i]; outn=outn+1; i=i+1; } return 0; }
int oc(int c) { out[outn]=c; outn=outn+1; return 0; }
int isalpha_(int c) { if (c>=97 && c<=122) { return 1; } if (c>=65 && c<=90) { return 1; } if (c==95) { return 1; } return 0; }
int isdigit_(int c) { if (c>=48 && c<=57) { return 1; } return 0; }
int isid_(int c) { if (isalpha_(c)) { return 1; } if (isdigit_(c)) { return 1; } return 0; }
int isws_(int c) { if (c==32 || c==9 || c==12 || c==11) { return 1; } return 0; }  /* space, tab, form-feed \f, vertical-tab \v */

/* --- macro table --- */
int find_macro(int off, int len) { int i; int j; int m;
  i=0; while (i<nmac) {
    if (mnlen[i]==len) { m=1; j=0; while (j<len) { if (mtxt[mnoff[i]+j]!=work[off+j]) { m=0; } j=j+1; } if (m) { return i; } }
    i=i+1; }
  return 0-1; }
int find_macro_s(char *nm, int len) { int i; int j; int m;   /* match against a raw C string (for undef etc.) */
  i=0; while (i<nmac) {
    if (mnlen[i]==len) { m=1; j=0; while (j<len) { if (mtxt[mnoff[i]+j]!=nm[j]) { m=0; } j=j+1; } if (m) { return i; } }
    i=i+1; }
  return 0-1; }
int undef_macro(int idx) { if (idx>=0) { mnlen[idx]=0; } return 0; }   /* tombstone: len 0 never matches */

/* add macro: name/params/body are spans in work[]. isfn=function-like; params "a,b,c" (spaces pre-trimmed) */
int add_macro(int noff, int nlen, int isfn, int poff, int plen, int boff, int blen) { int idx; int i;
  idx=find_macro(noff, nlen); if (idx>=0) { undef_macro(idx); }         /* redefine: drop old */
  mnoff[nmac]=mtxtn; i=0; while (i<nlen) { mtxt[mtxtn]=work[noff+i]; mtxtn=mtxtn+1; i=i+1; } mnlen[nmac]=nlen;
  mfunc[nmac]=isfn;
  mpoff[nmac]=mtxtn; i=0; while (i<plen) { mtxt[mtxtn]=work[poff+i]; mtxtn=mtxtn+1; i=i+1; } mplen[nmac]=plen;
  mboff[nmac]=mtxtn; i=0; while (i<blen) { mtxt[mtxtn]=work[boff+i]; mtxtn=mtxtn+1; i=i+1; } mblen[nmac]=blen;
  nmac=nmac+1; return 0; }
/* param name (buf[off,len)) -> its index in macro m's param list "a, b, c", or -1 (spaces tolerated) */
int param_index(int m, char *buf, int off, int len) { int po; int pl; int i; int s; int e; int j; int k; int mt;
  if (len<=0) { return 0-1; } po=mpoff[m]; pl=mplen[m]; i=po; k=0;
  while (i<po+pl) {
    while (i<po+pl && isws_(mtxt[i])) { i=i+1; }
    s=i; while (i<po+pl && mtxt[i]!=44) { i=i+1; }                       /* param name up to ',' */
    e=i; while (e>s && isws_(mtxt[e-1])) { e=e-1; }                      /* trim trailing ws */
    if (e-s==len) { mt=1; j=0; while (j<len) { if (mtxt[s+j]!=buf[off+j]) { mt=0; } j=j+1; } if (mt) { return k; } }
    k=k+1; if (i<po+pl) { i=i+1; } }
  return 0-1; }

/* --- pass 1: strip comments + splice backslash-newlines, inbuf[0..inlen) -> work[base..]; returns length --- */
int strip_into(int base) { int i; int w; int c; w=base; i=0;
  while (i<inlen) { c=inbuf[i];
    if (c==92 && i+1<inlen && inbuf[i+1]==10) { i=i+2; }                /* '\' newline -> splice */
    else if (c==34) { work[w]=c; w=w+1; i=i+1;                          /* "string" (copy verbatim) */
      while (i<inlen && inbuf[i]!=34) { if (inbuf[i]==92) { work[w]=inbuf[i]; w=w+1; i=i+1; } work[w]=inbuf[i]; w=w+1; i=i+1; }
      if (i<inlen) { work[w]=inbuf[i]; w=w+1; i=i+1; } }
    else if (c==39) { work[w]=c; w=w+1; i=i+1;                          /* 'char' */
      while (i<inlen && inbuf[i]!=39) { if (inbuf[i]==92) { work[w]=inbuf[i]; w=w+1; i=i+1; } work[w]=inbuf[i]; w=w+1; i=i+1; }
      if (i<inlen) { work[w]=inbuf[i]; w=w+1; i=i+1; } }
    else if (c==47 && i+1<inlen && inbuf[i+1]==42) { i=i+2;             /* block comment -> space */
      while (i+1<inlen && !(inbuf[i]==42 && inbuf[i+1]==47)) { i=i+1; } i=i+2; work[w]=32; w=w+1; }
    else if (c==47 && i+1<inlen && inbuf[i+1]==47) { i=i+2;             /* line comment -> drop to newline */
      while (i<inlen && inbuf[i]!=10) { i=i+1; } }
    else { work[w]=c; w=w+1; i=i+1; } }
  return w-base; }

int process(int wbase, int wlen);   /* forward (recursion via #include) */

/* --- file reading (syscall) --- */
int read_fd(int fd) { int t; int n; t=0; n=__syscall(0, fd, inbuf, 65536);
  while (n>0) { t=t+n; n=__syscall(0, fd, inbuf+t, 65536); } inlen=t; return t; }
int open_read(char *path) { int fd; fd=__syscall(2, path, 0, 0);              /* open(path, O_RDONLY) */
  if (fd<0) { return 0-1; } read_fd(fd); __syscall(3, fd, 0, 0); return 0; }  /* read all; close */
/* resolve+read an include name: as-is (local), then each -I dir. Returns 0 (inbuf filled) or -1. */
int open_path(char *name) { int i; int j; int k; int nl;
  if (open_read(name)==0) { return 0; }
  nl=0; while (name[nl]) { nl=nl+1; }
  i=0; while (i<nincd) { j=0; k=incoff[i]; while (j<inclen[i]) { pathbuf2[j]=incp[k]; j=j+1; k=k+1; }
    pathbuf2[j]=47; j=j+1; k=0; while (k<nl) { pathbuf2[j]=name[k]; j=j+1; k=k+1; } pathbuf2[j]=0;   /* dir/name */
    if (open_read(pathbuf2)==0) { return 0; } i=i+1; }
  return 0-1; }

/* am I currently emitting? (all conditional levels active) */
int emitting() { int i; i=0; while (i<csp) { if (cemit[i]==0) { return 0; } i=i+1; } return 1; }

int ocx(int c) { xb[xbp]=c; xbp=xbp+1; return 0; }                       /* append to expansion arena */
int oct(int c) { tb[tbp]=c; tbp=tbp+1; return 0; }                       /* append to substituted-body scratch */
int guarded(int m) { int k; k=0; while (k<nexp) { if (expanding[k]==m) { return 1; } k=k+1; } return 0; }
int cpx(char *buf, int off, int len) { int i; i=0; while (i<len) { ocx(buf[off+i]); i=i+1; } return 0; }  /* copy span -> xb */
/* stringize arg src[off,len) into tb: wrap in quotes, escape " and \, collapse ws runs, trim ends */
int strize(char *src, int off, int len) { int i; int c; int sp; oct(34); i=0; sp=0;
  while (i<len && isws_(src[off+i])) { i=i+1; }                          /* trim leading */
  while (i<len) { c=src[off+i];
    if (isws_(c)) { sp=1; i=i+1; }
    else { if (sp) { oct(32); sp=0; } if (c==34 || c==92) { oct(92); } oct(c); i=i+1; } }
  oct(34); return 0; }

int expand(char *src, int soff, int slen);   /* forward (recursive rescanning) */
/* build macro m's substituted body into tb. aoff/alen = RAW args (for #, ## operands);
   exoff/exlen = PRE-EXPANDED args in ab[] (for ordinary substitution -- the C "expand args first" rule). */
int subst(int m, char *src, int *aoff, int *alen, int *exoff, int *exlen, int na) {
  int i; int j; int k; int c; int pi; int fl; int bo; int bl; int pp; int fol;
  bo=mboff[m]; bl=mblen[m]; fl=tbp; i=0; pp=0;                           /* pp = previous op was ## */
  while (i<bl) { c=mtxt[bo+i];
    if (c==35 && i+1<bl && mtxt[bo+i+1]==35) {                           /* ## paste: abut prev and next */
      while (tbp>fl && tb[tbp-1]==32) { tbp=tbp-1; } i=i+2;
      while (i<bl && isws_(mtxt[bo+i])) { i=i+1; } pp=1; }
    else if (c==35) {                                                    /* # stringize next param (RAW arg) */
      i=i+1; while (i<bl && isws_(mtxt[bo+i])) { i=i+1; }
      j=i; while (i<bl && isid_(mtxt[bo+i])) { i=i+1; }
      pi=param_index(m, mtxt, bo+j, i-j);
      if (pi>=0 && pi<na) { strize(src, aoff[pi], alen[pi]); } else { oct(35); while (j<i) { oct(mtxt[bo+j]); j=j+1; } } pp=0; }
    else if (isalpha_(c)) { j=i; while (i<bl && isid_(mtxt[bo+i])) { i=i+1; }
      pi=param_index(m, mtxt, bo+j, i-j);
      if (pi>=0 && pi<na) {
        k=i; while (k<bl && isws_(mtxt[bo+k])) { k=k+1; } fol=0;         /* param followed by ## ? */
        if (k+1<bl && mtxt[bo+k]==35 && mtxt[bo+k+1]==35) { fol=1; }
        if (pp || fol) { k=0; while (k<alen[pi]) { oct(src[aoff[pi]+k]); k=k+1; } }   /* paste operand: RAW */
        else { k=0; while (k<exlen[pi]) { oct(ab[exoff[pi]+k]); k=k+1; } } }          /* ordinary: EXPANDED */
      else { while (j<i) { oct(mtxt[bo+j]); j=j+1; } } pp=0; }
    else if (c==34 || c==39) { oct(c); i=i+1; while (i<bl && mtxt[bo+i]!=c) { if (mtxt[bo+i]==92) { oct(mtxt[bo+i]); i=i+1; } oct(mtxt[bo+i]); i=i+1; } if (i<bl) { oct(mtxt[bo+i]); i=i+1; } pp=0; }
    else { oct(c); if (isws_(c)==0) { pp=0; } i=i+1; } }
  return 0; }
/* expand src[soff,soff+slen), appending the fully macro-expanded result to xb */
int expand(char *src, int soff, int slen) { int i; int j; int k; int m; int c; int na; int depth; int t0; int end; int ab0; int x0; int te; int ts; int lm; int dd; int pl;
  int aoff[64]; int alen[64]; int exoff[64]; int exlen[64];
  i=soff; end=soff+slen;
  while (i<end) { c=src[i];
    if (c==34 || c==39) { ocx(c); i=i+1; while (i<end && src[i]!=c) { if (src[i]==92) { ocx(src[i]); i=i+1; } ocx(src[i]); i=i+1; } if (i<end) { ocx(src[i]); i=i+1; } }
    else if (isalpha_(c)) { j=i; while (i<end && isid_(src[i])) { i=i+1; }
      m=find_macro_s(src+j, i-j);
      if (m<0 || guarded(m)) { cpx(src, j, i-j); }
      else if (mfunc[m]==0) { t0=tbp;                                          /* object macro: copy body to tb, then rescan-across-boundary -- an object macro whose body ENDS in a fn-like macro name, followed by (args) in the source, must expand the call (e.g. `#define DEF_BWLX DEF_BWLQ` then `DEF_BWLX(cmps)`). Previously the body was expanded in isolation, leaving `DEF_BWLQ(cmps)` unexpanded -> the b/w/l/q asm tokens never entered the enum. */
        k=mboff[m]; while (k<mboff[m]+mblen[m]) { tb[tbp]=mtxt[k]; tbp=tbp+1; k=k+1; }
        te=tbp; while (te>t0 && isws_(tb[te-1])) { te=te-1; }
        ts=te; while (ts>t0 && isid_(tb[ts-1])) { ts=ts-1; }
        pl=0; lm=0-1;
        if (te>ts) { lm=find_macro_s(tb+ts, te-ts);
          if (lm>=0 && mfunc[lm]) {
            k=i; while (k<end && (isws_(src[k]) || src[k]==10 || src[k]==13)) { k=k+1; }
            if (k<end && src[k]==40) { dd=0; pl=1;
              while (k<end) { c=src[k];
                if (c==34 || c==39) { tb[tbp]=c; tbp=tbp+1; k=k+1; while (k<end && src[k]!=c) { if (src[k]==92) { tb[tbp]=src[k]; tbp=tbp+1; k=k+1; } tb[tbp]=src[k]; tbp=tbp+1; k=k+1; } if (k<end) { tb[tbp]=src[k]; tbp=tbp+1; k=k+1; } }
                else if (c==40) { tb[tbp]=c; tbp=tbp+1; dd=dd+1; k=k+1; }
                else if (c==41) { tb[tbp]=c; tbp=tbp+1; dd=dd-1; k=k+1; if (dd==0) { i=k; k=end; } }
                else { tb[tbp]=c; tbp=tbp+1; k=k+1; } } } } }
        if (pl && lm!=m) { expand(tb, t0, tbp-t0); tbp=t0; }
        else { expanding[nexp]=m; nexp=nexp+1; expand(tb, t0, tbp-t0); nexp=nexp-1; tbp=t0; } }
      else { k=i; while (k<end && isws_(src[k])) { k=k+1; }              /* fn macro: needs '(' */
        if (k>=end || src[k]!=40) { cpx(src, j, i-j); }
        else { k=k+1; na=0; depth=0; aoff[0]=k;                          /* collect args (top-level commas, paren-nested) */
          while (k<end && depth>=0) { c=src[k];
            if (c==34 || c==39) { k=k+1; while (k<end && src[k]!=c) { if (src[k]==92) { k=k+1; } k=k+1; } k=k+1; }
            else if (c==40) { depth=depth+1; k=k+1; }
            else if (c==41) { if (depth==0) { alen[na]=k-aoff[na]; na=na+1; depth=0-1; } else { depth=depth-1; } k=k+1; }
            else if (c==44 && depth==0) { alen[na]=k-aoff[na]; na=na+1; aoff[na]=k+1; k=k+1; }
            else { k=k+1; } }
          i=k;                                                           /* past ')' */
          j=0; while (j<na) { while (alen[j]>0 && isws_(src[aoff[j]])) { aoff[j]=aoff[j]+1; alen[j]=alen[j]-1; }   /* trim each arg */
            while (alen[j]>0 && isws_(src[aoff[j]+alen[j]-1])) { alen[j]=alen[j]-1; } j=j+1; }
          if (na==1 && alen[0]<=0) { na=0; }                             /* NAME() -> zero args */
          ab0=abp; j=0; while (j<na) { exoff[j]=abp; x0=xbp; expand(src, aoff[j], alen[j]);  /* pre-expand each arg into ab */
            k=x0; while (k<xbp) { ab[abp]=xb[k]; abp=abp+1; k=k+1; } exlen[j]=xbp-x0; xbp=x0; j=j+1; }
          t0=tbp; subst(m, src, aoff, alen, exoff, exlen, na);           /* substituted body -> tb[t0,tbp) */
          abp=ab0;                                                       /* free arg-expansion scratch */
          te=tbp; while (te>t0 && isws_(tb[te-1])) { te=te-1; }          /* rescan-across-boundary: does the body END in a fn-like macro name? */
          ts=te; while (ts>t0 && isid_(tb[ts-1])) { ts=ts-1; }
          pl=0; lm=0-1;
          if (te>ts) { lm=find_macro_s(tb+ts, te-ts);
            if (lm>=0 && mfunc[lm]) {                                     /* yes -> if source continues with (args), pull them into tb so the call expands (e.g. ELFW(R_INFO)(sym,type)) */
              k=i; while (k<end && (isws_(src[k]) || src[k]==10 || src[k]==13)) { k=k+1; }
              if (k<end && src[k]==40) { dd=0; pl=1;
                while (k<end) { c=src[k];
                  if (c==34 || c==39) { tb[tbp]=c; tbp=tbp+1; k=k+1; while (k<end && src[k]!=c) { if (src[k]==92) { tb[tbp]=src[k]; tbp=tbp+1; k=k+1; } tb[tbp]=src[k]; tbp=tbp+1; k=k+1; } if (k<end) { tb[tbp]=src[k]; tbp=tbp+1; k=k+1; } }
                  else if (c==40) { tb[tbp]=c; tbp=tbp+1; dd=dd+1; k=k+1; }
                  else if (c==41) { tb[tbp]=c; tbp=tbp+1; dd=dd-1; k=k+1; if (dd==0) { i=k; k=end; } }
                  else { tb[tbp]=c; tbp=tbp+1; k=k+1; } } } } }
          if (pl && lm!=m) { expand(tb, t0, tbp-t0); tbp=t0; }            /* R93: pulled args are SOURCE tokens (not m's replacement) -- expand WITHOUT m's blue-paint so a nested fn-macro in an arg expands (ELFW(R_INFO)(a, ELFW(R_TYPE)(b))) */
          else { expanding[nexp]=m; nexp=nexp+1; expand(tb, t0, tbp-t0); nexp=nexp-1; tbp=t0; } } } }
    else { ocx(c); i=i+1; } }
  return 0; }

int streq(char *a, int aoff, char *b, int blen) { int j;   /* work[aoff..] starts with the blen-char string b? */
  j=0; while (j<blen) { if (a[aoff+j]!=b[j]) { return 0; } j=j+1; } return 1; }

/* --- #if constant-expression evaluator (over the macro-expanded, defined-resolved condition in xb) --- */
int ev_bin(int minprec);   /* forward */
int ev_skip() { while (ev_pos<ev_end && isws_(xb[ev_pos])) { ev_pos=ev_pos+1; } return 0; }
int ev_num() { int v; int c; v=0;
  if (xb[ev_pos]==48 && ev_pos+1<ev_end && (xb[ev_pos+1]==120 || xb[ev_pos+1]==88)) { ev_pos=ev_pos+2;   /* 0x hex */
    while (ev_pos<ev_end) { c=xb[ev_pos];
      if (c>=48 && c<=57) { v=v*16+(c-48); } else if (c>=97 && c<=102) { v=v*16+(c-87); } else if (c>=65 && c<=70) { v=v*16+(c-55); } else { break; }
      ev_pos=ev_pos+1; } }
  else { while (ev_pos<ev_end && xb[ev_pos]>=48 && xb[ev_pos]<=57) { v=v*10+(xb[ev_pos]-48); ev_pos=ev_pos+1; } }
  while (ev_pos<ev_end && (xb[ev_pos]==108 || xb[ev_pos]==76 || xb[ev_pos]==117 || xb[ev_pos]==85)) { ev_pos=ev_pos+1; }  /* L/U suffix */
  return v; }
int ev_primary() { int v; int c;
  ev_skip(); if (ev_pos>=ev_end) { return 0; } c=xb[ev_pos];
  if (c==40) { ev_pos=ev_pos+1; v=ev_bin(0); ev_skip(); if (ev_pos<ev_end && xb[ev_pos]==41) { ev_pos=ev_pos+1; } return v; }
  if (c==33) { ev_pos=ev_pos+1; v=ev_primary(); if (v==0) { return 1; } return 0; }        /* !x */
  if (c==126) { ev_pos=ev_pos+1; v=ev_primary(); return 0-v-1; }                            /* ~x = -x-1 */
  if (c==45) { ev_pos=ev_pos+1; return 0-ev_primary(); }                                    /* -x */
  if (c==43) { ev_pos=ev_pos+1; return ev_primary(); }                                      /* +x */
  if (c==39) { ev_pos=ev_pos+1; c=xb[ev_pos];                                               /* 'c' */
    if (c==92) { ev_pos=ev_pos+1; c=xb[ev_pos]; if (c==110) { c=10; } else if (c==116) { c=9; } else if (c==48) { c=0; } else if (c==114) { c=13; } }
    ev_pos=ev_pos+1; if (ev_pos<ev_end && xb[ev_pos]==39) { ev_pos=ev_pos+1; } return c; }
  if (c>=48 && c<=57) { return ev_num(); }
  if (isalpha_(c)) { while (ev_pos<ev_end && isid_(xb[ev_pos])) { ev_pos=ev_pos+1; } return 0; }   /* undefined ident -> 0 */
  ev_pos=ev_pos+1; return 0; }
int ev_peekop() { int c; int c1; ev_skip(); ev_oplen=1; if (ev_pos>=ev_end) { return 0; }
  c=xb[ev_pos]; c1=0; if (ev_pos+1<ev_end) { c1=xb[ev_pos+1]; }
  if (c==124 && c1==124) { ev_oplen=2; ev_opprec=1; return 1; }
  if (c==38 && c1==38) { ev_oplen=2; ev_opprec=2; return 2; }
  if (c==124) { ev_opprec=3; return 3; } if (c==94) { ev_opprec=4; return 4; } if (c==38) { ev_opprec=5; return 5; }
  if (c==61 && c1==61) { ev_oplen=2; ev_opprec=6; return 6; } if (c==33 && c1==61) { ev_oplen=2; ev_opprec=6; return 7; }
  if (c==60 && c1==60) { ev_oplen=2; ev_opprec=8; return 12; } if (c==62 && c1==62) { ev_oplen=2; ev_opprec=8; return 13; }
  if (c==60 && c1==61) { ev_oplen=2; ev_opprec=7; return 10; } if (c==62 && c1==61) { ev_oplen=2; ev_opprec=7; return 11; }
  if (c==60) { ev_opprec=7; return 8; } if (c==62) { ev_opprec=7; return 9; }
  if (c==43) { ev_opprec=9; return 14; } if (c==45) { ev_opprec=9; return 15; }
  if (c==42) { ev_opprec=10; return 16; } if (c==47) { ev_opprec=10; return 17; } if (c==37) { ev_opprec=10; return 18; }
  return 0; }
int ev_apply(int op, int a, int b) {
  if (op==1) { if (a || b) { return 1; } return 0; } if (op==2) { if (a && b) { return 1; } return 0; }
  if (op==3) { return a|b; } if (op==4) { return a^b; } if (op==5) { return a&b; }
  if (op==6) { if (a==b) { return 1; } return 0; } if (op==7) { if (a!=b) { return 1; } return 0; }
  if (op==8) { if (a<b) { return 1; } return 0; } if (op==9) { if (a>b) { return 1; } return 0; }
  if (op==10) { if (a<=b) { return 1; } return 0; } if (op==11) { if (a>=b) { return 1; } return 0; }
  if (op==12) { return a<<b; } if (op==13) { return a>>b; }
  if (op==14) { return a+b; } if (op==15) { return a-b; } if (op==16) { return a*b; }
  if (op==17) { if (b==0) { return 0; } return a/b; } if (op==18) { if (b==0) { return 0; } return a-(a/b)*b; }
  return 0; }
int ev_bin(int minprec) { int left; int op; int prec; int right;
  left=ev_primary();
  while (1) { op=ev_peekop(); if (op==0) { return left; } prec=ev_opprec; if (prec<minprec) { return left; }
    ev_pos=ev_pos+ev_oplen; right=ev_bin(prec+1); left=ev_apply(op, left, right); }
  return left; }
/* resolve `defined X` / `defined(X)` in work[off,len) -> cbuf (1/0), copy the rest verbatim */
int resolve_defined(int off, int len) { int i; int j; int nj; int end; int paren; int m; cbn=0;
  i=off; end=off+len;
  while (i<end) {
    if (isalpha_(work[i])) { j=i; while (i<end && isid_(work[i])) { i=i+1; }
      if (i-j==7 && streq(work, j, "defined", 7)) {
        while (i<end && isws_(work[i])) { i=i+1; } paren=0;
        if (i<end && work[i]==40) { paren=1; i=i+1; while (i<end && isws_(work[i])) { i=i+1; } }
        nj=i; while (i<end && isid_(work[i])) { i=i+1; } m=find_macro(nj, i-nj);
        if (m>=0) { cbuf[cbn]=49; } else { cbuf[cbn]=48; } cbn=cbn+1;
        if (paren) { while (i<end && isws_(work[i])) { i=i+1; } if (i<end && work[i]==41) { i=i+1; } } }
      else { while (j<i) { cbuf[cbn]=work[j]; cbn=cbn+1; j=j+1; } } }
    else { cbuf[cbn]=work[i]; cbn=cbn+1; i=i+1; } }
  return 0; }
int eval_cond(int off, int len) { resolve_defined(off, len);
  xbp=0; expand(cbuf, 0, cbn); ev_pos=0; ev_end=xbp;
  if (ev_bin(0)!=0) { return 1; } return 0; }
int emit_upto(int n) { int i; i=0; while (i<n) { if (cemit[i]==0) { return 0; } i=i+1; } return 1; }

/* handle a directive line: work[off..off+len) begins at '#' */
int do_directive(int off, int len) { int p; int e; int noff; int nlen; int boff; int idx; int c; int j; int save_inlen;
  p=off+1; while (p<off+len && isws_(work[p])) { p=p+1; }                 /* after '#' */
  e=p; while (e<off+len && isalpha_(work[e])) { e=e+1; }                  /* directive keyword [p,e) */
  /* ---- conditionals (evaluated even while skipping, to track nesting) ---- */
  if (streq(work, p, "ifdef", 5)) { p=e; while (p<off+len && isws_(work[p])) { p=p+1; } noff=p; while (p<off+len && isid_(work[p])) { p=p+1; }
    idx=find_macro(noff, p-noff); if (idx>=0) { c=1; } else { c=0; } cemit[csp]=c; ctaken[csp]=c; csp=csp+1; return 0; }
  if (streq(work, p, "ifndef", 6)) { p=e; while (p<off+len && isws_(work[p])) { p=p+1; } noff=p; while (p<off+len && isid_(work[p])) { p=p+1; }
    idx=find_macro(noff, p-noff); if (idx>=0) { c=0; } else { c=1; } cemit[csp]=c; ctaken[csp]=c; csp=csp+1; return 0; }
  if (streq(work, p, "if", 2)) { if (emit_upto(csp)) { c=eval_cond(e, off+len-e); } else { c=0; }   /* #if EXPR */
    cemit[csp]=c; ctaken[csp]=c; csp=csp+1; return 0; }
  if (streq(work, p, "elif", 4)) { if (csp>0) {                                         /* #elif EXPR */
    if (emit_upto(csp-1) && ctaken[csp-1]==0) { if (eval_cond(e, off+len-e)) { cemit[csp-1]=1; ctaken[csp-1]=1; } else { cemit[csp-1]=0; } }
    else { cemit[csp-1]=0; } } return 0; }
  if (streq(work, p, "else", 4)) { if (csp>0) { if (ctaken[csp-1]) { cemit[csp-1]=0; } else { cemit[csp-1]=1; ctaken[csp-1]=1; } } return 0; }
  if (streq(work, p, "endif", 5)) { if (csp>0) { csp=csp-1; } return 0; }
  if (emitting()==0) { return 0; }                                       /* below: only when active */
  /* ---- #define (object-like only in P1) ---- */
  if (streq(work, p, "define", 6)) { p=e; while (p<off+len && isws_(work[p])) { p=p+1; }
    noff=p; while (p<off+len && isid_(work[p])) { p=p+1; } nlen=p-noff;
    if (p<off+len && work[p]==40) {                                      /* function-like: NAME(params) body */
      p=p+1; noff=noff; j=p;                                             /* param list [j, close) */
      while (p<off+len && work[p]!=41) { p=p+1; } idx=j;                 /* idx reused: param span start */
      c=p; p=p+1;                                                        /* c = param end; past ')' */
      while (p<off+len && isws_(work[p])) { p=p+1; }                     /* body after ws */
      add_macro(noff, nlen, 1, idx, c-idx, p, off+len-p); return 0; }
    while (p<off+len && isws_(work[p])) { p=p+1; }                        /* object-like: body after ws */
    boff=p; add_macro(noff, nlen, 0, 0, 0, boff, off+len-boff); return 0; }
  if (streq(work, p, "undef", 5)) { p=e; while (p<off+len && isws_(work[p])) { p=p+1; }
    noff=p; while (p<off+len && isid_(work[p])) { p=p+1; } undef_macro(find_macro(noff, p-noff)); return 0; }
  if (streq(work, p, "include", 7)) { p=e; while (p<off+len && isws_(work[p])) { p=p+1; }
    if (p>=off+len) { return 0; } c=work[p];                              /* '"'..'"' local, or '<'..'>' system */
    if (c==60) { c=62; }
    p=p+1; j=0; while (p<off+len && work[p]!=c) { pathbuf[j]=work[p]; j=j+1; p=p+1; } pathbuf[j]=0;
    idx=open_path(pathbuf);                                               /* try local then -I dirs; reads into inbuf */
    if (idx<0) { return 0; }                                             /* not found: skip (headers land in P4) */
    save_inlen=whi; boff=strip_into(whi); whi=whi+boff;                   /* strip included file into a fresh region */
    process(save_inlen, boff); whi=save_inlen; return 0; }                /* process it (recurse), then reclaim */
  /* #error halts; #pragma / #line ignored */
  if (streq(work, p, "error", 5)) { es("#error\n"); __syscall(1, 2, "rzcpp: #error\n", 14); return 0; }
  return 0; }

/* main line loop over work[0..worklen) */
int process(int wbase, int wlen) { int i; int ls; int p; int k; int wend;
  i=wbase; wend=wbase+wlen;
  while (i<wend) { ls=i;                                                 /* line start */
    while (i<wend && work[i]!=10) { i=i+1; }                             /* to end of line [ls,i) */
    p=ls; while (p<i && isws_(work[p])) { p=p+1; }
    if (p<i && work[p]==35) { do_directive(p, i-p); }                    /* '#' directive (may #include -> recurse) */
    else { if (emitting()) { xbp=0; expand(work, ls, i-ls); k=0; while (k<xbp) { oc(xb[k]); k=k+1; } oc(10); } }
    i=i+1; }                                                             /* past newline */
  return 0; }

/* argv (marshalled at 0x10000000): -Idir add a search dir; -Dname[=val] predefine a macro */
int parse_args() { char **a; char *s; int k; int j; int no; int bo; a=268435456; incpn=0; k=1;
  while (a[k]) { s=a[k];
    if (s[0]==45 && s[1]==73) { incoff[nincd]=incpn; j=2; while (s[j]) { incp[incpn]=s[j]; incpn=incpn+1; j=j+1; }
      inclen[nincd]=incpn-incoff[nincd]; nincd=nincd+1; }
    else if (s[0]==45 && s[1]==68) { j=2; no=0; while (s[j] && s[j]!=61) { work[no]=s[j]; no=no+1; j=j+1; }   /* -Dname[=val] */
      if (s[j]==61) { j=j+1; bo=no; while (s[j]) { work[bo]=s[j]; bo=bo+1; j=j+1; } } else { work[no]=49; bo=no+1; }
      add_macro(0, no, 0, 0, 0, no, bo-no); }
    k=k+1; }
  return 0; }
int main() { int L;
  nmac=0; mtxtn=0; csp=0; nexp=0; outn=0; incdirlen=0; xbp=0; tbp=0; abp=0; nincd=0;
  parse_args();                                                          /* argv: -Idir, -Dname[=val] */
  read_fd(0);                                                            /* main file from stdin */
  L=strip_into(0); whi=L;                                               /* strip main; includes go above whi */
  process(0, L);
  __syscall(1, 1, out, outn);
  return 0; }
