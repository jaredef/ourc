/* ccc.c -- cc-in-C (self-host step 3). v13: LONG=8 -- long (and long typedefs: ssize_t/off_t/...) are 8-byte in struct fields; struct sizes 8-aligned; sizeof(TYPE) operator (struct/long/ptr/int/char). Kernel structs (timespec/tm) now ABI-honest. v12: DECL SURFACE -- void/unsigned/long/short/signed scalar type words (runs), const/extern/
 * static/volatile/register/inline qualifiers (skipquals), multi-level pointers (skipptr), via a whole-token matcher (tok_is); decl_type
 * unified across globals/locals/params/fields/typedefs. (long/unsigned treated int-width for now; extern allocates -- single-file.) v11: TYPEDEF -- typedef alias resolution (typedef struct scm* SCM;), resolved wherever a type
 * is expected (decl_type/at_decl); pointer typedefs carry implicit pointer-ness (dptr). v10: STRUCTS -- struct types (C-ABI field layout), struct ptr/value vars, p->field read/write
 * dispatching to the width-appropriate op (load/load8/loadb, store/store8/loadb; pointer-field store8 wraps conv to zero-extend the
 * narrow pointer). Still self-hosting (ccc.c uses no structs, so ccc(ccc.c)==ccc2(ccc.c) byte-identical fixpoint holds). v9 baseline:
 * SELF-HOSTING -- ccc(ccc.c)==ccc2(ccc.c) byte-identical fixpoint. (int/char, globals, fns, ptrs, arrays, strings, && || !, comments.)
 * if/else/while/nesting; validated vs M0 cc. (The string-literal <=4 split is retired: the root M0 cf address-load
 * bug it dodged is fixed in rzir-to-elf-cf.m0; see FINDINGS-v3.md.) */
char src[2000000];
int slen;
char out[4000000];
int outn;
char nbuf[16];
int cp; int ncount; int op_id; int oplen;
int is64[262144]; int gmax; int gzi;                                             /* R91: per-value CLEAN-WIDE tag (conv/*8/long-field-load) -- an *8 operand that is NOT is64 must be conv'd (zext) first; gmax = running max ncount for per-function clear */
int ishi[262144];                                                                /* R91: per-value HAS-BITS->=32 tag -- set ONLY by shl8(>=32) + *8 ALU results (a genuine 64-bit value like sym<<32); drives ALU promotion. A long-field READ is is64 but NOT ishi (its high half is 0 in the <4GB model), so field arithmetic stays 32-bit -- keeping the promotion to the r_info packing chain alone. */
int glastconst_v; int glastconst_n;                                              /* R91: vN + value of the most recent emit_const -- lets a shift detect a constant count >=32 (the r_info sym<<32 seed) */
int id_off; int id_len;
int var_off[5000]; int var_len[5000]; int var_slot[5000]; int nvar;
int var_bytes[5000]; int dvbytes; int lv_bytes;  /* per var: sizeof-bytes for arrays (0 => use scalar/struct default); one-shot decl input; lvref output */
int emptyarr;                                    /* R125: current local decl was `T name[]` (size inferred from initializer) */
int var_glob[5000]; int var_arr[5000];   /* per var: global? (slot holds addr vs slot#); array? */
int gcount;                              /* global slot counter; addr = 0x700000 + gcount*8 (above code, below the 0x2000000 frame base -- ~25MB arena) */
int lv_glob; int lv_arr; int lv_sa;      /* set by lvref(): kind + slot-or-address */
int nglob; int czero;                    /* #globals (scope reset point); current fn's const-0 slot */
int nfunc; int fname_off[2048]; int fname_len[2048]; int fpos[2048]; int fn_flng[2048]; int fn_rsptr[2048];  /* pre-scanned function table; R138: fn_flng = returns a genuine long; R143: fn_rsptr = struct idx if fn returns `Struct *` (else -1) */
int pnfunc; int pname_off[1024]; int pname_len[1024]; int pn_flng[1024];         /* R160/c99-R1: PROTOTYPE return-type table (name + long-ness) -- separate from fpos (which drives emission), so an EXTERNAL long/double-returning fn (e.g. the bridge's `unsigned long __adddf3(...)` / soft-float) has its CALL result tagged is64 instead of defaulting to i32 (which truncated strtod's double bit pattern). */
int var_char[5000]; int lv_char; int dchar;  /* per var: byte element (char) vs i32; current decl's char flag */
int var_long[5000]; int lv_long;             /* per var: long element (8-byte) -- set from dlong; routes to load8/store8/stride-8 */
int var_unsig[5000]; int lv_unsig;            /* R64: per var unsigned flag -> logical >> (shu) */
int var_ptr[5000]; int lv_ptr; int nstar;  /* per var: pointer depth (#*+typedef-ptr); >=2 => subscript yields an 8-byte pointer */
int snm_off[256]; int snm_len[256]; int sfst[256]; int sfn[256]; int ssz[256]; int nstruct;  /* struct types: name, field range, size */
int fldnm_off[4096]; int fldnm_len[4096]; int fldoff[4096]; int fldw[4096]; int fld_struct[4096]; int ffstruct; int nfld; int fld_pw[4096]; int ffpw;        /* fields: name, byte offset, width 1/4/8 */
int fld_bytes[4096]; int ffbytes;      /* R115: per-field total byte size (scalar width, array=esz*count, struct=ssz) -- for sizeof(struct.field) */
int fld_long[4096]; int fflong;                                                  /* R91: field is a genuine `long` integer (dlong) -- 8-byte ARITHMETIC (*8 ops), vs an 8-byte POINTER (32-bit model) */
int md_active; int md_char; int md_short; int md_long; int md_struct; int md_unsig; int md_stk; int md_soff; int md_slen;  /* R103: base type carried across a multi-declarator `T a, b, c;` continuation (else b/c defaulted to int-4, silently truncating `long v1, v2`) */
int fld_unsig[4096]; int ffunsig; int st_slng;                                   /* R94: field is unsigned -- a SIGNED long field (fflong && !ffunsig) sign-extends on store (r_addend = -4); st_slng captured at a store before the rhs clobbers la_* */
int fld_al[4096]; int salign[256];    /* R71: per-field alignment + per-struct alignment (max member align) */
int fld_soff[4096]; int fld_slen[4096];    /* R72: struct name of a struct/struct-ptr field, for lazy forward-ref resolution */
int fld_arr[4096]; int ffarr;
int td_tagoff[512]; int td_taglen[512];   /* R79c */                                                  /* R28: field is an array (decays to its address, no load) */
int dstruct; int dstk; int dstruct_off; int dstruct_len; int duni; int dunsig; int dshort;   /* current decl: struct idx, is-struct flag, struct-name off/len; R44: decl was `union` */
int var_struct[5000]; int lv_struct; int ffoff; int ffw;   /* per var struct idx; lvref output; find_field outputs */
int tdnm_off[512]; int tdnm_len[512]; int td_struct[512]; int td_char[512]; int td_ptr[512]; int td_long[512]; int td_arr[512]; int ntd; int dtarr; int td_short[512]; int td_unsig[512];  /* typedef aliases (+ array dimension); R137: alias unsignedness */
int enm_off[4096]; int enm_len[4096]; int enm_val[4096]; int nenm; int enm_found;  /* enum constants: name -> int value */
int dptr; int dlong; int dextern;                          /* current decl: implicit pointer; long (8-byte); extern (no gdef) */
int dflt;                                                   /* R160/c99-R3: current decl is a FLOATING type (float/double/long double). ccc has no hardware float -> all floats are the 8-byte double BIT PATTERN and every float op lowers to a soft-float call (__adddf3/__floatdidf/...). Lets ccc compile tcc's pristine gen_opif/gen_cast/gen_cvt const machinery. */
int isflt[262144];                                          /* R160/c99-R3: per-value FLOATING tag (parallel to is64) -- a value holding double bits, so an op on it lowers to soft-float */
int isneg[262144];                                          /* R161/tcc-self-R3: per-value SIGNED-NEGATIVE-CONSTANT tag (-1, -4 via unary minus / emit_int) -- widened by SIGN-extend (not the R91 zero-extend) in a 64-bit COMPARE, else `u64 == -1` zexts -1 to 0x00000000FFFFFFFF and wrongly matches 0xFFFFFFFF (tcc gen_opic's `l2 == -1` fold dropped `& 0xFFFFFFFF`). */
int var_flt[5000]; int lv_flt;                              /* per-var floating flag + lvref output */
int fld_flt[4096]; int fffloat;                             /* per-field floating flag + find_field output */
int cur_nparams; int cur_va_base;                          /* varargs: named-param count + __va_area frame base of the current fn */
int pin_goff[1024]; int pin_glen[1024]; int pin_w[1024]; int pin_vs[1024]; int pin_vc[1024]; int npin;  /* pending global inits: name, elem width, value range */
int pin_vals[65536]; int nvals;                            /* flattened initializer constant values (emitted as init stores at main's entry) */
int pin_hi[65536];                                         /* R160: high 32 bits of each flattened initializer value (0 unless a >=2^32 literal) -- a `static unsigned long A[]={0x1234...}` init would otherwise store only the low 32 bits */
int pin_boff[65536]; int pin_vw[65536]; int pin_kd[65536]; /* R45 hetero inits (pin_w==0 entries): per-value byte offset, width, kind (0=const, 1=string src-pos in pin_vals) */
int ginit_split;                                          /* R121: a LARGE global-init store set was split into __ginitN() functions (called at main's top) instead of inlined -- else main exceeds ra/cf per-function capacity (tcc's 332-entry const opcode table) */
int lv_noff; int lv_nlen; int lv_found;                    /* lvref: the matched var's name off/len; found flag (var vs fn) */
int lbl_ctr; int sw_cval[1024]; int sw_clab[1024]; int sw_valn;  /* switch: monotone label ids; per-case value/label tables (nesting stack) */
int bk_kind[64]; int bk_end[64]; int bk_sp;                /* break-context stack: kind 0=loop (cf `brk`), 1=switch (`goto` end-label bk_end) */

int read_all() { int t; int n; t=0; n=__syscall(0,0,src,65536);
  while (n>0 && t<1900000) { t=t+n; n=__syscall(0,0,src+t,65536); } return t; }  /* cap below src[] capacity */
int es(char *s) { int i; i=0; while (s[i]) { out[outn]=s[i]; outn=outn+1; i=i+1; } return 0; }
int en(int x) { int t; int v; int neg;
  if (x==0) { out[outn]=48; outn=outn+1; return 0; }
  neg=0; v=x; t=0;
  if (v<0) { neg=1;                                                    /* extract digits from the NEGATIVE value -- never negate, so INT_MIN can't overflow (was: `v=0-v` gave `const -` with no digits) */
    while (v<0) { nbuf[t]=48-(v-(v/10)*10); t=t+1; v=v/10; } }         /* v%10 is <=0 (toward-zero div); digit = -(v%10) */
  else { while (v>0) { nbuf[t]=48+(v-(v/10)*10); t=t+1; v=v/10; } }
  if (neg) { out[outn]=45; outn=outn+1; }
  while (t>0) { t=t-1; out[outn]=nbuf[t]; outn=outn+1; } return 0; }
int skipws() { while (cp<slen) {                                                /* whitespace + block comments */
  if (src[cp]==32 || src[cp]==10 || src[cp]==9 || src[cp]==13 || src[cp]==12 || src[cp]==11) { cp=cp+1; }  /* ws incl form-feed \f (12) + vertical-tab \v (11) */
  else if (src[cp]==47 && src[cp+1]==42) { cp=cp+2; while (cp<slen && !(src[cp]==42 && src[cp+1]==47)) { cp=cp+1; } cp=cp+2; }  /* comment */
  else { return 0; } }
  return 0; }
int isalnum_(int c) { if (c>=97 && c<=122) { return 1; } if (c>=65 && c<=90) { return 1; }
  if (c>=48 && c<=57) { return 1; } if (c==95) { return 1; } return 0; }
int read_ident() { skipws(); id_off=cp; while (cp<slen && isalnum_(src[cp])) { cp=cp+1; } id_len=cp-id_off; return 0; }
int id_is(char *s, int n) { int i; if (id_len!=n) { return 0; } i=0; while (i<n) { if (src[id_off+i]!=s[i]) { return 0; } i=i+1; } return 1; }  /* just-read ident == literal? */
int tok_is(char *s) { int i; int p; i=0; p=cp;                                  /* whole-token match at cp -> end pos, else 0 */
  while (s[i]) { if (src[p]!=s[i]) { return 0; } p=p+1; i=i+1; }
  if (isalnum_(src[p])) { return 0; } return p; }
int skipquals() { int e;                                                        /* skip leading storage/cv qualifiers */
  while (cp<slen) { skipws(); e=tok_is("extern"); if (e) { dextern=1; }         /* note extern (global gets no gdef) */
    if (e==0) { e=tok_is("const"); } if (e==0) { e=tok_is("static"); }
    if (e==0) { e=tok_is("volatile"); } if (e==0) { e=tok_is("register"); } if (e==0) { e=tok_is("inline"); }
    if (e==0) { return 0; } cp=e; } return 0; }
int skipptr() { int e;                                                          /* skip a run of '*' and trailing const/volatile */
  nstar=0;
  while (cp<slen) { skipws();
    if (src[cp]==42) { cp=cp+1; nstar=nstar+1; }
    else { e=tok_is("const"); if (e==0) { e=tok_is("volatile"); } if (e==0) { return 0; } cp=e; } }
  return 0; }
int lvref() { int i; int j; int m;                                             /* find var by id_off/id_len; set lv_* */
  lv_glob=0; lv_arr=0; lv_sa=0; lv_char=0; lv_long=0; lv_struct=0-1; lv_ptr=0; lv_found=0; lv_unsig=0; i=0; while (i<nvar) {
    if (var_len[i]==id_len) { m=1; j=0; while (j<id_len) { if (src[var_off[i]+j]!=src[id_off+j]) { m=0; } j=j+1; }
      if (m==1) { lv_glob=var_glob[i]; lv_arr=var_arr[i]; lv_sa=var_slot[i]; lv_char=var_char[i]; lv_long=var_long[i]; lv_struct=var_struct[i]; lv_ptr=var_ptr[i]; lv_noff=var_off[i]; lv_nlen=var_len[i]; lv_bytes=var_bytes[i]; lv_unsig=var_unsig[i]; lv_flt=var_flt[i]; lv_found=1; } }  /* last (innermost) match wins: locals shadow globals */
    i=i+1; }
  return 0; }
int rec_var(int off, int len, int glob, int arr, int sa) {                      /* record a variable (char flag from dchar) */
  var_off[nvar]=off; var_len[nvar]=len; var_glob[nvar]=glob; var_arr[nvar]=arr; var_slot[nvar]=sa; var_char[nvar]=dchar; var_long[nvar]=dlong; var_struct[nvar]=dstruct; var_ptr[nvar]=nstar+dptr; var_bytes[nvar]=dvbytes; var_unsig[nvar]=dunsig; var_flt[nvar]=(dflt && nstar==0 && dptr==0); dvbytes=0; nvar=nvar+1; return 0; }
int decl_type_alias(int save) { int ti;                                         /* typedef alias name, or not-a-type */
  cp=save; skipquals(); skipws();
  if (isalnum_(src[cp])) { if (src[cp]<48 || src[cp]>57) { read_ident();
    if (id_len>0) { ti=find_typedef(id_off, id_len);
      if (ti>=0) { dstruct=td_struct[ti]; dchar=td_char[ti]; dptr=td_ptr[ti]; dlong=td_long[ti]; dtarr=td_arr[ti]; dshort=td_short[ti]; dunsig=td_unsig[ti]; dstk=0; if (dstruct>=0) { dstk=1; } return 1; } } } }
  cp=save; return 0; }
int settype(int sawc, int sawl) { dchar=sawc; dlong=sawl; dstruct=0-1; dstk=0; dptr=0; return 1; }
int decl_type() { int save; int got; int sawc; int sawl; int e;                /* at a type? advance past it, set dchar/dlong/dstruct/dptr */
  save=cp; dstk=0; dlong=0; dextern=0; dtarr=0; duni=0; dunsig=0; dshort=0; dflt=0; skipquals(); skipws();
  e=tok_is("struct"); if (e) { cp=e; skipws(); read_ident(); dstruct_off=id_off; dstruct_len=id_len;
    dstruct=find_struct(id_off, id_len); dstk=1; dchar=0; dptr=0; return 1; }
  e=tok_is("union"); if (e) { cp=e; skipws(); read_ident(); dstruct_off=id_off; dstruct_len=id_len;   /* R44: tagged union (shares the struct namespace; overlay layout via duni) */
    dstruct=find_struct(id_off, id_len); dstk=1; dchar=0; dptr=0; duni=1; return 1; }
  e=tok_is("enum"); if (e) { cp=e; skipws();                                    /* enum [tag] [{..}] -> a plain int */
    if (isalnum_(src[cp]) && src[cp]!=123) { read_ident(); skipws(); }          /* optional tag */
    if (src[cp]==123) { got=1; cp=cp+1; while (cp<slen && got>0) { if (src[cp]==123) { got=got+1; } if (src[cp]==125) { got=got-1; } cp=cp+1; } }  /* skip {..} */
    dstruct=0-1; dstk=0; dchar=0; dptr=0; dlong=0; return 1; }
  got=0; sawc=0; sawl=0;                                                        /* run of scalar type words (unsigned long int ...) */
  while (cp<slen) { skipquals(); skipws();
    e=tok_is("char"); if (e) { cp=e; sawc=1; got=1; }
    else { e=tok_is("int"); if (e) { cp=e; got=1; }
    else { e=tok_is("void"); if (e) { cp=e; got=1; }
    else { e=tok_is("long"); if (e) { cp=e; sawl=1; got=1; }
    else { e=tok_is("short"); if (e) { cp=e; got=1; dshort=1; }
    else { e=tok_is("unsigned"); if (e) { cp=e; got=1; dunsig=1; }
    else { e=tok_is("signed"); if (e) { cp=e; got=1; }
    else { e=tok_is("double"); if (e) { cp=e; sawl=1; got=1; dflt=1; }           /* R160/c99-R3: double = 8-byte FLOATING (double bit pattern; ops -> soft-float) */
    else { e=tok_is("float"); if (e) { cp=e; got=1; dflt=1; }                    /* R160/c99-R3: float = 4-byte FLOATING (stored 32-bit; widened to double bits for arithmetic) */
    else { if (got) { return settype(sawc, sawl); } return decl_type_alias(save); } } } } } } } } } }
  if (got) { return settype(sawc, sawl); }
  return decl_type_alias(save); }
int base_of() { if (lv_glob) { return emit_const(lv_sa); } return lv_sa; }      /* a vN holding the base address */
int emit_const(int n) { es("  v"); en(ncount); es(": i32 = const "); en(n); es("\n"); ncount=ncount+1; glastconst_v=ncount-1; glastconst_n=n; return ncount-1; }
int emit_name(int noff, int nlen) { int i; i=0; while (i<nlen) { out[outn]=src[noff+i]; outn=outn+1; i=i+1; } return 0; }
int emit_gref(int noff, int nlen) { es("  v"); en(ncount); es(": i32 = gref "); emit_name(noff, nlen); es("\n"); ncount=ncount+1; return ncount-1; }  /* address of global NAME (linker resolves) */
int emit_gdef(int noff, int nlen, int size) { es("gdef "); emit_name(noff, nlen); es(" "); en(size); es("\n"); return 0; }  /* one per defined (non-extern) global */
int emit_binop(int oid, int a, int b) {
  es("  v"); en(ncount); es(": i32 = ");
  if (oid==1) { es("mul "); } else if (oid==2) { es("div "); } else if (oid==3) { es("mod "); }
  else if (oid==4) { es("add "); } else if (oid==5) { es("sub "); } else if (oid==6) { es("shl "); }
  else if (oid==7) { es("shr "); } else if (oid==23) { es("shu "); } else if (oid==8) { es("lt "); } else if (oid==9) { es("le "); }
  else if (oid==10) { es("gt "); } else if (oid==11) { es("ge "); } else if (oid==12) { es("eq "); }
  else if (oid==13) { es("ne "); } else if (oid==14) { es("and "); } else if (oid==15) { es("xor "); }
  else if (oid==19) { es("ltu "); } else if (oid==20) { es("leu "); } else if (oid==21) { es("gtu "); } else if (oid==22) { es("geu "); }
  else { es("or "); }
  es("v"); en(a); es(" v"); en(b); es("\n"); ncount=ncount+1; return ncount-1; }
int emit_binop8(int oid, int a, int b) { int scmp;                             /* R91: 8-byte (i64) op family -- REX.W lowering in cf; preserves bits >=32 (reloc r_info sym<<32) */
  scmp = (oid>=8 && oid<=13) || (oid>=19 && oid<=22);                          /* R161/tcc-self-R3: a 64-bit COMPARE with a signed-negative operand must SIGN-extend it (else zext -1 = 0x00000000FFFFFFFF != real -1); bitwise/mask ops keep the R4 zext (and8 -1 = low-32 mask) */
  if (is64[a]==0) { if (scmp && isneg[a]) { a=emit_convs(a); } else { a=emit_conv(a); } }  /* R91: an *8 operand MUST be a clean 64-bit value -- zext any 32-bit operand (a 32-bit const/param/op-result has a STALE high half; conv(-1 mask)=0x00000000FFFFFFFF) */
  if (is64[b]==0) { if (scmp && isneg[b]) { b=emit_convs(b); } else { b=emit_conv(b); } }
  es("  v"); en(ncount); es(": i64 = ");
  if (oid==4) { es("add8 "); } else if (oid==5) { es("sub8 "); } else if (oid==6) { es("shl8 "); }
  else if (oid==7) { es("shr8 "); } else if (oid==23) { es("shu8 "); } else if (oid==14) { es("and8 "); }
  else if (oid==15) { es("xor8 "); } else if (oid==1) { es("mul8 "); }
  else if (oid==2) { es("div8 "); } else if (oid==3) { es("mod8 "); }               /* R138: signed 64-bit division/remainder */
  else if (oid==8) { es("lt8 "); } else if (oid==9) { es("le8 "); } else if (oid==10) { es("gt8 "); }  /* R135: 64-bit compares (result is a 32-bit bool) */
  else if (oid==11) { es("ge8 "); } else if (oid==12) { es("eq8 "); } else if (oid==13) { es("ne8 "); }
  else if (oid==19) { es("ltu8 "); } else if (oid==20) { es("leu8 "); } else if (oid==21) { es("gtu8 "); } else if (oid==22) { es("geu8 "); }
  else { es("or8 "); }
  es("v"); en(a); es(" v"); en(b); es("\n"); ncount=ncount+1;
  if (oid>=8 && oid<=13) { is64[ncount-1]=0; ishi[ncount-1]=0; return ncount-1; }  /* R135: signed compare -> 32-bit bool */
  if (oid>=19 && oid<=22) { is64[ncount-1]=0; ishi[ncount-1]=0; return ncount-1; } /* R135: unsigned compare -> 32-bit bool */
  is64[ncount-1]=1;
  if (oid==7 || oid==23) { ishi[ncount-1]=0; } else { ishi[ncount-1]=1; }        /* R91: shr8/shu8 EXTRACT (result fits 32-bit -> not ishi); shl8 + ALU produce genuine-high values */
  return ncount-1; }
/* ===== R160/c99-R3: soft-float lowering (ccc has no hardware float; a float VALUE is the 8-byte
   double bit pattern, and every float op lowers to a soft-float CALL -- __adddf3/__floatsidf/...
   linked into tcc.elf. Lets ccc compile tcc's pristine gen_opif/gen_cast/gen_cvt const machinery). */
int emit_int_to_flt(int v) { int r;                                             /* int -> double bits: __floatsidf(v) (32-bit int; large-int->double is a later __floatdidf gap) */
  es("  v"); en(ncount); es(": i32 = call __floatsidf v"); en(v); es("\n"); ncount=ncount+1; r=ncount-1;
  is64[r]=1; ishi[r]=1; isflt[r]=1; return r; }
int emit_flt_to_int(int v) { int r;                                             /* double bits -> int: __fixdfsi(v) */
  if (is64[v]==0) { v=emit_conv(v); }
  es("  v"); en(ncount); es(": i32 = call __fixdfsi v"); en(v); es("\n"); ncount=ncount+1; r=ncount-1;
  is64[r]=0; ishi[r]=0; isflt[r]=0; return r; }
int emit_fbinop(int oid, int a, int b) { int r;                                 /* float add sub mul div -> soft-float; a,b are double-bit values (int operands widened) */
  if (isflt[a]==0) { a=emit_int_to_flt(a); }
  if (isflt[b]==0) { b=emit_int_to_flt(b); }
  es("  v"); en(ncount); es(": i32 = call ");
  if (oid==4) { es("__adddf3"); } else if (oid==5) { es("__subdf3"); }
  else if (oid==1) { es("__muldf3"); } else { es("__divdf3"); }                 /* oid 2 = div; only +,-,*,/ reach here */
  es(" v"); en(a); es(" v"); en(b); es("\n"); ncount=ncount+1; r=ncount-1;
  is64[r]=1; ishi[r]=1; isflt[r]=1; return r; }
int emit_fcmp(int oid, int a, int b) { int c; int r;                            /* float compare -> soft-float 2-way call, then the integer test of its result -> 0/1 */
  if (isflt[a]==0) { a=emit_int_to_flt(a); }
  if (isflt[b]==0) { b=emit_int_to_flt(b); }
  es("  v"); en(ncount); es(": i32 = call ");
  if (oid==12) { es("__eqdf2"); } else if (oid==13) { es("__nedf2"); }
  else if (oid==8) { es("__ltdf2"); } else if (oid==9) { es("__ledf2"); }
  else if (oid==10) { es("__gtdf2"); } else { es("__gedf2"); }                  /* oid 11 = >= */
  es(" v"); en(a); es(" v"); en(b); es("\n"); ncount=ncount+1; c=ncount-1;      /* c = the 2-way int result */
  if (oid==12) { r=emit_binop(12, c, czero); } else if (oid==13) { r=emit_binop(13, c, czero); }  /* ==0 / !=0 */
  else if (oid==8) { r=emit_binop(8, c, czero); } else if (oid==9) { r=emit_binop(9, c, czero); } /* <0 / <=0 */
  else if (oid==10) { r=emit_binop(10, c, czero); } else { r=emit_binop(11, c, czero); }          /* >0 / >=0 */
  return r; }
int emit_fld8(int addr) { int t;                                                /* R160/c99-R3: load an 8-byte scalar field; tag is64 (long/double) + isflt (double) so a `v->c.d` read lowers arithmetic to soft-float */
  if (fflong || fffloat) { t=emit_load8L(addr); if (fffloat) { isflt[t]=1; } return t; }
  return emit_load8(addr); }
int emit_lafld8(int addr) { int t;                                              /* R160/c99-R3: same for the lval_addr (la_*) value-struct read path (`a.d` on a struct/union VALUE local) */
  if (la_long || la_flt) { t=emit_load8L(addr); if (la_flt) { isflt[t]=1; } return t; }
  return emit_load8(addr); }
int emit_float_lit(int mant) { int nfrac; int ev; int eneg; int v; int ten; int i;  /* R160/c99-R3: cp at '.'/'e'/'E'; integer part in mant. Emit the double VALUE as a runtime soft-float expr (mant * 10^(exp-nfrac)) -- always bit-correct, needed for tcc source literals (0.0, ...). */
  nfrac=0;
  if (src[cp]==46) { cp=cp+1; while (src[cp]>=48 && src[cp]<=57) { mant=mant*10+(src[cp]-48); nfrac=nfrac+1; cp=cp+1; } }  /* fraction */
  ev=0; eneg=0;
  if (src[cp]==101 || src[cp]==69) { cp=cp+1;                                    /* e/E exponent */
    if (src[cp]==43) { cp=cp+1; } else if (src[cp]==45) { eneg=1; cp=cp+1; }
    while (src[cp]>=48 && src[cp]<=57) { ev=ev*10+(src[cp]-48); cp=cp+1; } }
  while (src[cp]==102 || src[cp]==70 || src[cp]==108 || src[cp]==76) { cp=cp+1; } /* f/F/l/L suffix */
  if (eneg) { ev=0-ev; } ev=ev-nfrac;                                            /* net base-10 exponent */
  v=emit_int_to_flt(emit_const(mant)); ten=emit_int_to_flt(emit_const(10));
  if (ev>0) { i=0; while (i<ev) { v=emit_fbinop(1, v, ten); i=i+1; } }           /* *10 */
  else if (ev<0) { i=0; while (i<(0-ev)) { v=emit_fbinop(2, v, ten); i=i+1; } }  /* /10 */
  return v; }
int emit_copy(int dest, int val) { es("  v"); en(dest); es(": i32 = add v"); en(val); es(" v"); en(czero); es("\n"); return 0; }
int emit_copy8(int dest, int val, int uns) {                                   /* R132(part B): 64-bit copy into a local slot -- 32-bit val widened, 64-bit val preserved (or8 x x = x). R144: widen SIGNED (convs) unless the rhs is unsigned -- a negative int -> long local was zext'd (`long a=-5` held +4294867291; 64-bit div8/mod8 then wrong). Values with bit31 clear are unaffected (convs==conv), so tcc addresses/sizes are unchanged. */
  if (is64[val]) { es("  v"); en(dest); es(": i64 = or8 v"); en(val); es(" v"); en(val); es("\n"); }
  else if (uns) { es("  v"); en(dest); es(": i64 = conv v"); en(val); es("\n"); }
  else { es("  v"); en(dest); es(": i64 = convs v"); en(val); es("\n"); }
  is64[dest]=1; return 0; }
int emit_widen(int val, int uns) { if (is64[val]) { return val; } if (uns) { return emit_conv(val); } return emit_convs(val); }  /* R144: i32->i64 for a long store, signed unless unsigned */
int emit_ref(int base) { es("  v"); en(ncount); es(": i32 = ref v"); en(base); es("\n"); ncount=ncount+1; return ncount-1; }
int emit_get(int base, int idx) { es("  v"); en(ncount); es(": i32 = get v"); en(base); es(" v"); en(idx); es("\n"); ncount=ncount+1; return ncount-1; }
int emit_load(int addr) { es("  v"); en(ncount); es(": i32 = load v"); en(addr); es("\n"); ncount=ncount+1; return ncount-1; }
int emit_getb(int base, int idx) { es("  v"); en(ncount); es(": i32 = getb v"); en(base); es(" v"); en(idx); es("\n"); ncount=ncount+1; return ncount-1; }
int emit_loadb(int addr) { es("  v"); en(ncount); es(": i32 = loadb v"); en(addr); es("\n"); ncount=ncount+1; return ncount-1; }
int emit_loadb_sx(int addr, int uns) { int t; t=emit_loadb(addr);                /* R129: signed char field/deref read -- sign-extend (loadb is zero-extend); a `signed char reg=-1` field read as 255 broke `reg==-1` (tcc's Operand.reg int8_t -> call/jmp OPT_ADDR) */
  if (uns) { return t; } return emit_binop(7, emit_binop(6, t, emit_const(24)), emit_const(24)); }
int emit_load2(int addr) { es("  v"); en(ncount); es(": i32 = load2 v"); en(addr); es("\n"); ncount=ncount+1; return ncount-1; }  /* R60: 16-bit field load */
int emit_load8(int addr) { es("  v"); en(ncount); es(": i64 = load8 v"); en(addr); es("\n"); ncount=ncount+1; return ncount-1; }  /* pointer/intermediate 8-byte load -- NOT is64 (a <4GB pointer must stay in the 32-bit model) */
int emit_load8L(int addr) { int t; t=emit_load8(addr); is64[t]=1; return t; }    /* R91: load of a GENUINE `long` integer field/var -> is64 (needs *8 arithmetic; e.g. r_info) */
int emit_conv(int a) { if (is64[a]) { return a; } es("  v"); en(ncount); es(": i64 = conv v"); en(a); es("\n"); ncount=ncount+1; is64[ncount-1]=1; return ncount-1; }  /* R160: idempotent -- conv (zext i32->i64) of an ALREADY-64-bit value would zero its high 32 bits (truncate). For tcc's <4GB is64 values conv is already a no-op (high bits 0), so skipping it is identical there; only genuine-64-bit values (long array/deref elements stored back) are fixed. */
int emit_convs(int a) { es("  v"); en(ncount); es(": i64 = convs v"); en(a); es("\n"); ncount=ncount+1; is64[ncount-1]=1; return ncount-1; }  /* R94: SIGN-extend i32->i64 (signed long field store, e.g. r_addend = -4) */
int emit_brz(int v) {                                                            /* R160: `brz v` (if/while/for/&&/||/ternary condition) -- when v is genuinely 64-bit, cf's brz tests only the low 32 bits, so a high-only value (softfloat clz `while(x){x>>=1}`) reads as zero. Reduce to a 64-bit `ne8 v,0` bool first. SAFE for tcc's <4GB is64 values (high bits 0 -> same result). */
  if (is64[v]) { v = emit_binop8(13, v, czero); }
  es("  brz v"); en(v); es("\n"); return 0; }
int emit_bool(int v) { if (is64[v]) { return emit_binop8(13, v, czero); } return emit_binop(13, v, czero); }      /* R160: (v != 0) as 0/1 -- 64-bit-aware (a high-only value must not test just the low 32 bits) */
int emit_notbool(int v) { if (is64[v]) { return emit_binop8(12, v, czero); } return emit_binop(12, v, czero); }   /* R160: (v == 0) as 0/1 -- 64-bit-aware (used by !x / &&/|| / ternary condition coercion) */
int emit_faddr(int noff, int nlen) { es("  v"); en(ncount); es(": i32 = faddr "); emit_name(noff, nlen); es("\n"); ncount=ncount+1; return ncount-1; }  /* function address */
int find_struct(int off, int len) { int i; int j; int m;                       /* struct type by name -> idx, or -1 */
  if (len<=0) { return 0-1; }                                                  /* anonymous struct: never matched by name (bound by index) */
  i=0; while (i<nstruct) {
    if (snm_len[i]==len) { m=1; j=0; while (j<len) { if (src[snm_off[i]+j]!=src[off+j]) { m=0; } j=j+1; }
      if (m==1) { return i; } } i=i+1; }
  return 0-1; }
int find_field(int sidx, int off, int len) { int i; int e; int j; int m;       /* field by name -> sets ffoff, ffw */
  ffoff=0; ffw=8; ffstruct=0-1; ffarr=0; ffpw=8; fflong=0; ffunsig=0; ffbytes=8; if (sidx<0) { return 0; }                             /* opaque/unresolved struct -> safe default (no OOB) */
  i=sfst[sidx]; e=i+sfn[sidx];
  while (i<e) {
    if (fldnm_len[i]==len) { m=1; j=0; while (j<len) { if (src[fldnm_off[i]+j]!=src[off+j]) { m=0; } j=j+1; }
      if (m==1) { ffoff=fldoff[i]; ffw=fldw[i]; ffstruct=fld_struct[i]; ffarr=fld_arr[i]; ffpw=fld_pw[i]; fflong=fld_long[i]; ffunsig=fld_unsig[i]; ffbytes=fld_bytes[i]; fffloat=fld_flt[i]; return 0; } } i=i+1; }
  return 0; }
int find_typedef(int off, int len) { int i; int j; int m;                      /* alias name -> typedef idx, or -1 */
  i=0; while (i<ntd) {
    if (tdnm_len[i]==len) { m=1; j=0; while (j<len) { if (src[tdnm_off[i]+j]!=src[off+j]) { m=0; } j=j+1; }
      if (m==1) { return i; } } i=i+1; }
  return 0-1; }
int find_enum(int off, int len) { int i; int j; int m; enm_found=0;             /* enum-const name -> value; enm_found=1 if present */
  i=0; while (i<nenm) {
    if (enm_len[i]==len) { m=1; j=0; while (j<len) { if (src[enm_off[i]+j]!=src[off+j]) { m=0; } j=j+1; }
      if (m==1) { enm_found=1; return enm_val[i]; } } i=i+1; }
  return 0; }
int skipparens() { int d; d=0;                                                 /* consume a balanced (...) starting at '(' */
  while (cp<slen) { if (src[cp]==40) { d=d+1; cp=cp+1; }
    else if (src[cp]==41) { d=d-1; cp=cp+1; if (d==0) { return 0; } }
    else { cp=cp+1; } }
  return 0; }
int parse_typedef() { int up; int ti; int tgo; int tgl;                                          /* typedef UNDERLYING [*] ALIAS ; (R54: update-or-append -- prescan binds, main pass REBINDS) */
  tgo=0; tgl=0;
  decl_type();                                                                 /* underlying type -> dstruct/dchar/dptr */
  up=dptr; skipws();
  if (src[cp]==40) {                                                           /* typedef RET (*NAME)(params); -- function pointer */
    cp=cp+1; skipws(); skipptr(); read_ident();                                /* '(' '*' NAME */
    ti=find_typedef(id_off, id_len); if (ti<0) { ti=ntd; ntd=ntd+1; tdnm_off[ti]=id_off; tdnm_len[ti]=id_len; }
    td_struct[ti]=0-1; td_char[ti]=0; td_ptr[ti]=1; td_long[ti]=0; td_arr[ti]=0; td_short[ti]=0; td_unsig[ti]=0;
    skipws(); if (src[cp]==41) { cp=cp+1; }                                     /* ')' */
    skipws(); if (src[cp]==40) { skipparens(); }                               /* (params) */
    skipws(); if (src[cp]==59) { cp=cp+1; } return 0; }                        /* ';' */
  if (src[cp]==123) {                                                          /* typedef struct/union { body } Alias -- consume the definition */
    tgo=dstruct_off; tgl=dstruct_len;                                          /* R79c */
    if (dstruct>=0 || dstruct_len>0) { skip_braces(); }                        /* R79b: defer tagged typedef structs */
    else { parse_struct_def(); dstruct=nstruct-1; dchar=0; dlong=0; dunsig=0; }  /* anonymous: register + lay out now, bind by index (R58: body's field parses clobber dchar/dlong -- clear, else the alias inherits the LAST FIELD's scalar class and sizeof() lies) */
    dstk=1; skipws(); }
  if (src[cp]==42) { up=1; } skipptr();                                        /* pointer alias */
  read_ident();                                                                /* the alias name */
  up=up; skipws(); dtarr=0;
  if (src[cp]==91) { cp=cp+1; dtarr=0; while (src[cp]>=48 && src[cp]<=57) { dtarr=dtarr*10+(src[cp]-48); cp=cp+1; } skipws(); if (src[cp]==93) { cp=cp+1; } }  /* ALIAS[N] array typedef */
  ti=find_typedef(id_off, id_len); if (ti<0) { ti=ntd; ntd=ntd+1; tdnm_off[ti]=id_off; tdnm_len[ti]=id_len; }
  td_struct[ti]=dstruct; td_char[ti]=dchar; td_ptr[ti]=up; td_long[ti]=dlong; td_arr[ti]=dtarr; td_short[ti]=dshort; td_unsig[ti]=dunsig;
  td_tagoff[ti]=tgo; td_taglen[ti]=tgl;                                        /* R79c */
  skipws(); cp=cp+1; return 0; }                                               /* ';' */
int peek_brace(int e) { while (e<slen) {                                        /* next non-ws from e is '{' ? */
  if (src[e]==32 || src[e]==10 || src[e]==9 || src[e]==13) { e=e+1; } else { return src[e]==123; } } return 0; }
int peek_tag_brace(int e) { int p;                                              /* R102: `struct TAG {` -- a TAGGED inline aggregate definition (tag sits between the keyword and '{', so peek_brace misses it) */
  p=e; while (p<slen && (src[p]==32 || src[p]==10 || src[p]==9 || src[p]==13)) { p=p+1; }
  if (!(isalnum_(src[p]) && (src[p]<48 || src[p]>57))) { return 0; }             /* need a tag identifier */
  while (p<slen && isalnum_(src[p])) { p=p+1; }
  while (p<slen && (src[p]==32 || src[p]==10 || src[p]==9 || src[p]==13)) { p=p+1; }
  return src[p]==123; }
int parse_tagged_field(int off, int isuni, int isunion);                        /* R102 fwd */
int field_one(int off, int isuni, int base) { int fw; int moff; int got; int isarr; int acount; int fsize; int bwidth; int bitpos; int uoff; int ubytes; int esz; int ealign; int pw;  /* parse one `type [*]name[N]` or bitfield-group field; return new off */
  got=decl_type(); fw=4; if (dchar) { fw=1; } if (dshort && dstk==0) { fw=2; } if (dlong) { fw=8; } skipws();  /* R60: short fields are 2-byte (ELF-exact) */
  if (dptr) { fw=8; } if (src[cp]==42) { fw=8; } skipptr();
  read_ident();
  if (got==0) { skipws(); if (src[cp]==42 || (isalnum_(src[cp]) && (src[cp]<48 || src[cp]>57))) {  /* unknown type (typedef not yet seen, e.g. `uint8_t *p` in a struct laid out before its typedef): the name just read was the TYPE; skip `*` and read the real field name; treat as 8-byte */
    fw=8; dstruct=0-1; skipptr(); read_ident(); }
    else if (md_active) {                                                        /* R103: `T a, b;` continuation -- reuse the base type (else `long v1, v2` truncates v2 to int) */
      dchar=md_char; dshort=md_short; dlong=md_long; dstruct=md_struct; dunsig=md_unsig; dstk=md_stk; dstruct_off=md_soff; dstruct_len=md_slen;
      fw=4; if (dchar) { fw=1; } if (dshort && dstk==0) { fw=2; } if (dlong) { fw=8; } if (dptr) { fw=8; } if (src[cp]==42) { fw=8; } skipptr(); } }
  skipws();
  if (src[cp]==58) {                                                             /* R30: bitfield group `T a:N, b:M, ... ;` -- pack into base-type units */
    ubytes=fw; if (ubytes<1) { ubytes=1; }
    moff=base; if (isuni==0) { off=(off+ubytes)-1; off=off/ubytes; off=off*ubytes; moff=off; }
    uoff=moff; bitpos=0;
    while (1) {
      cp=cp+1; skipws();                                                         /* past ':' */
      bwidth=0; while (src[cp]>=48 && src[cp]<=57) { bwidth=bwidth*10+(src[cp]-48); cp=cp+1; }
      if (bwidth>0) {
        if (bitpos+bwidth > ubytes*8) { uoff=uoff+ubytes; bitpos=0; }            /* doesn't fit current unit -> next unit */
        if (id_len>0) { fldnm_off[nfld]=id_off; fldnm_len[nfld]=id_len; fldoff[nfld]=uoff; fldw[nfld]=ubytes; fld_struct[nfld]=0-1; fld_arr[nfld]=0; fld_pw[nfld]=ubytes; fld_al[nfld]=ubytes; nfld=nfld+1; }  /* named bitfield: byte offset of its unit */
        bitpos=bitpos+bwidth; }
      else { if (bitpos>0) { uoff=uoff+ubytes; bitpos=0; } }                     /* `:0` -> align to next unit */
      skipws();
      if (src[cp]==44) { cp=cp+1; skipws(); skipptr(); read_ident(); skipws(); } /* ',' -> next declarator (shares the type); then loop parses its ':N' */
      else { skipws(); cp=cp+1; skipws();  break; } }                            /* ';' */
    fsize=(uoff+ubytes)-moff;
    md_active=0;                                                                /* R103: bitfield group handles its own commas -- no scalar continuation */
    if (isuni) { return fsize; } return moff+fsize; }
  isarr=0; acount=1; skipws();
  if (src[cp]==91) { isarr=1; cp=cp+1; skipws(); if (src[cp]==40) { cp=cp+1; skipws(); }  /* R75: skip a parenthesized dim `[(N)]` (macro-expanded const, e.g. e_ident[(16)]) */
    acount=0; while (src[cp]>=48 && src[cp]<=57) { acount=acount*10+(src[cp]-48); cp=cp+1; }  /* R28: [N] array field */
    skipws();
    if (src[cp]==47 || src[cp]==42) {                                           /* R44: [A/B] or [A*B] const fold (e.g. tab[16/4]) */
      bwidth=src[cp]; cp=cp+1; skipws(); uoff=0;
      while (src[cp]>=48 && src[cp]<=57) { uoff=uoff*10+(src[cp]-48); cp=cp+1; }
      if (bwidth==47) { if (uoff>0) { acount=acount/uoff; } } else { acount=acount*uoff; }
      skipws(); }
    skipws(); if (src[cp]==41) { cp=cp+1; skipws(); }                           /* R75: close paren of `[(N)]` */
    if (src[cp]==93) { cp=cp+1; } if (acount<1) { acount=1; } }
  pw=4; if (dchar) { pw=1; } else if (dshort && dstk==0) { pw=2; } else if (dlong) { pw=8; } else if (dstk && dstruct>=0) { pw=ssz[dstruct]; }  /* R62: base scalar/struct width */
  if ((nstar+dptr)>=2 || ((nstar+dptr)==1 && isarr)) { pw=8; }                                     /* R63: T** field or array-of-ptr -> subscript element is a POINTER (8), not the base scalar/struct */
  esz=fw; ealign=fw;                                                                              /* element size + alignment */
  if (fw!=8 && dstk && dstruct>=0 && dptr==0) { esz=ssz[dstruct]; if (esz<1) { esz=1; } ealign=salign[dstruct]; if (ealign<1) { ealign=1; } }  /* R30 size; R71 align to the nested struct's OWN alignment, not 8 */
  moff=base; if (isuni==0) { off=(off+ealign)-1; off=off/ealign; off=off*ealign; moff=off; }      /* union: at base; struct: aligned */
  fld_soff[nfld]=dstruct_off; fld_slen[nfld]=0; if (dstk) { fld_slen[nfld]=dstruct_len; }  /* R72: remember the struct name for lazy resolution (forward refs) */
  fld_long[nfld]=0; if (dlong && (nstar+dptr)==0 && isarr==0) { fld_long[nfld]=1; }  /* R91: genuine `long` scalar field (not ptr/array) -> 8-byte arithmetic */
  fld_flt[nfld]=0; if (dflt && (nstar+dptr)==0 && isarr==0) { fld_flt[nfld]=1; }      /* R160/c99-R3: floating scalar field (double/float; e.g. CValue.d) -> reads are isflt */
  fld_unsig[nfld]=dunsig;                                                           /* R94: field unsignedness (for sext-vs-zext on a signed-long store) */
  fldnm_off[nfld]=id_off; fldnm_len[nfld]=id_len; fldoff[nfld]=moff; fldw[nfld]=fw; fld_struct[nfld]=dstruct; fld_arr[nfld]=isarr; fld_pw[nfld]=pw; fld_al[nfld]=ealign;
  fsize=esz; if (isarr) { fsize=esz*acount; }                                                     /* array/struct field occupies element*count bytes */
  fld_bytes[nfld]=fsize; nfld=nfld+1;                                                             /* R115: total byte size for sizeof(struct.field) */
  if (isuni==0) { off=moff+fsize; }
  skipws();
  if (src[cp]==44) { md_active=1; md_char=dchar; md_short=dshort; md_long=dlong; md_struct=dstruct; md_unsig=dunsig; md_stk=dstk; md_soff=dstruct_off; md_slen=dstruct_len; }  /* R103: `,` -> a continuation declarator follows; carry this base type */
  else { md_active=0; }
  cp=cp+1; skipws();                                                           /* ';' or ',' */
  if (isuni) { return fsize; } return off; }                                    /* union: return member size (for max) */
int peek_named_after(int e);   /* forward (R44) */
int parse_member_named(int off, int puni, int inneruni, int mbase);   /* forward (R44/R46) */
int aggr_align_here() { int save; int d; int al; int maxal; int e2;
  save=cp; maxal=1; skipws();
  while (cp<slen && src[cp]!=125) { skipws(); if (src[cp]==125) { break; }
    e2=tok_is("union"); if (e2==0) { e2=tok_is("struct"); }
    if (e2 && peek_brace(e2)) {
      cp=e2; skipws(); cp=cp+1; skipws();
      al=aggr_align_here();
      d=1; while (cp<slen && d>0) { if (src[cp]==123) { d=d+1; } if (src[cp]==125) { d=d-1; } cp=cp+1; }
      skipws(); while (cp<slen && isalnum_(src[cp])) { cp=cp+1; }
      skipws(); if (src[cp]==59) { cp=cp+1; } }
    else { decl_type();
      al=4; if (dchar) { al=1; } else if (dshort && dstk==0) { al=2; } else if (dlong) { al=8; }
      else if (dstk && dstruct>=0) { al=salign[dstruct]; if (al<1) { al=1; } }
      if (dptr) { al=8; }
      skipws(); if (src[cp]==42) { al=8; }
      while (cp<slen && src[cp]!=59 && src[cp]!=125) { cp=cp+1; }
      if (src[cp]==59) { cp=cp+1; } }
    if (al>maxal) { maxal=al; } skipws(); }
  cp=save; return maxal; }
int parse_aggr(int off, int isuni) { int base; int mx; int e; int w;           /* anonymous union {/struct { ... } ; -- R46: RECURSIVE (Sym nests union>struct>union) */
  e=tok_is("union"); if (e==0) { e=tok_is("struct"); } cp=e; skipws(); cp=cp+1; skipws();
  { int aal=aggr_align_here(); off=(off+aal-1)/aal; off=off*aal; base=off; mx=0; }                                     /* align aggregate to 8 */
  while (cp<slen && src[cp]!=125) { skipws();
    e=tok_is("union");
    if (e && peek_brace(e)) {
      if (peek_named_after(e)) {                                                /* named aggregate member */
        if (isuni) { w=parse_member_named(off, 1, 1, base); } else { off=parse_member_named(off, 0, 1, base); w=0; } }
      else { if (isuni) { w=parse_aggr(base, 1); w=w-base; } else { off=parse_aggr(off, 1); w=0; } } }  /* anon: recurse */
    else { e=tok_is("struct");
      if (e && peek_brace(e)) {
        if (peek_named_after(e)) {
          if (isuni) { w=parse_member_named(off, 1, 0, base); } else { off=parse_member_named(off, 0, 0, base); w=0; } }
        else { if (isuni) { w=parse_aggr(base, 0); w=w-base; } else { off=parse_aggr(off, 0); w=0; } } }
      else { w=field_one(off, isuni, base); if (isuni==0) { off=w; w=0; } } }
    if (isuni) { if (w>mx) { mx=w; } } }                                         /* union: max member size */
  cp=cp+1; skipws(); if (src[cp]==59) { cp=cp+1; } skipws();                    /* '}' [';'] then settle on next token */
  if (isuni) { return base+mx; } return off; }
int skip_braces() { int d; d=1; cp=cp+1;                                        /* at '{' -> consume balanced {...} */
  while (cp<slen && d>0) { if (src[cp]==123) { d=d+1; } if (src[cp]==125) { d=d-1; } cp=cp+1; }
  skipws(); if (src[cp]==59) { cp=cp+1; } return 0; }                            /* trailing ';' */
int peek_named_after(int e) { int d;                                            /* R44: from e (at/before '{'), does an IDENT follow the matching '}' ? (named aggregate member) */
  while (e<slen && src[e]!=123) { e=e+1; }
  d=1; e=e+1;
  while (e<slen && d>0) { if (src[e]==123) { d=d+1; } if (src[e]==125) { d=d-1; } e=e+1; }
  while (e<slen && (src[e]==32 || src[e]==10 || src[e]==9 || src[e]==13)) { e=e+1; }
  if (e>=slen) { return 0; }
  if (isalnum_(src[e]) && (src[e]<48 || src[e]>57)) { return 1; } return 0; }
int pmn_sz;                                                                     /* R44: parse_member_named output: the member's size */
int parse_member_named(int off, int puni, int inneruni, int mbase) { int inidx; int ioff; int imx; int w; int moff; int isz;  /* R44: `struct/union { body } NAME;` member -> own anonymous type + one field (R46: mbase = union-parent base) */
  w=tok_is("union"); if (w==0) { w=tok_is("struct"); } cp=w; skipws(); cp=cp+1; skipws();   /* keyword + '{' */
  inidx=nstruct; snm_off[inidx]=0; snm_len[inidx]=0; sfst[inidx]=nfld; nstruct=nstruct+1;   /* inner type: anonymous, bound by index (its fields land inside the parent's window; offsets are member-relative) */
  ioff=0; imx=0;
  while (cp<slen && src[cp]!=125) { skipws();
    if (inneruni) { w=field_one(0, 1, 0); if (w>imx) { imx=w; } }
    else { ioff=field_one(ioff, 0, 0); } }
  cp=cp+1; skipws();                                                            /* '}' */
  isz=ioff; if (inneruni) { isz=imx; }
  isz=(isz+7)/8; isz=isz*8;
  sfn[inidx]=nfld-sfst[inidx]; ssz[inidx]=isz;
  read_ident();                                                                 /* the member name */
  moff=mbase; if (puni==0) { off=(off+7)/8; off=off*8; moff=off; }              /* union parent: at its base; struct parent: 8-aligned */
  fldnm_off[nfld]=id_off; fldnm_len[nfld]=id_len; fldoff[nfld]=moff; fldw[nfld]=8; fld_struct[nfld]=inidx; fld_arr[nfld]=0; fld_al[nfld]=8; nfld=nfld+1;
  skipws(); if (src[cp]==59) { cp=cp+1; } skipws();                             /* ';' */
  pmn_sz=isz;
  if (puni) { return isz; }                                                     /* union parent: member size (for max) */
  return moff+isz; }
int psd_semi;                                                                   /* R26: parse_struct_def consumed a trailing ';' ? (else a var declarator follows) */
int parse_struct_def() { int sidx; int off; int e; int uni; int mx; int w;     /* struct/union NAME { type [*]field | union {..} ; ... } ; */
  uni=duni; duni=0; psd_semi=0;                                                /* R44: union -> overlay layout (duni consumed to a local immediately, then cleared) */
  if (dstruct_len>0) { if (find_struct(dstruct_off, dstruct_len)>=0) {          /* already registered (prepass) -> skip body, but DON'T eat ';' (a var may follow) */
    off=1; cp=cp+1; while (cp<slen && off>0) { if (src[cp]==123) { off=off+1; } if (src[cp]==125) { off=off-1; } cp=cp+1; }
    return 0; } }                                                              /* dstruct already set by decl_type; cp left right after '}' */
  sidx=nstruct; snm_off[sidx]=dstruct_off; snm_len[sidx]=dstruct_len; sfst[sidx]=nfld; nstruct=nstruct+1;
  cp=cp+1; skipws(); off=0; mx=0; md_active=0;                                  /* '{' (R103: no multi-declarator carry into a fresh struct body) */
  while (cp<slen && src[cp]!=125) { skipws();
    e=tok_is("union");
    if (e) { if (peek_tag_brace(e)) { w=parse_tagged_field(off, uni, 1); }       /* R102: union TAG { ... } field */
      else if (peek_brace(e)) {
        if (peek_named_after(e)) { w=parse_member_named(off, uni, 1, 0); }
        else { w=parse_aggr(off, 1); if (uni) { w=w-off; } } }                  /* anon inline; union parent: treat result as size */
      else { w=field_one(off, uni, 0); } }
    else { e=tok_is("struct");
      if (e) { if (peek_tag_brace(e)) { w=parse_tagged_field(off, uni, 0); }      /* R102: struct TAG { ... } field (tcc's switch_t.p) */
        else if (peek_brace(e)) {
          if (peek_named_after(e)) { w=parse_member_named(off, uni, 0, 0); }
          else { w=parse_aggr(off, 0); if (uni) { w=w-off; } } }
        else { w=field_one(off, uni, 0); } }
      else { w=field_one(off, uni, 0); } }
    if (uni) { if (w>mx) { mx=w; } } else { off=w; } }
  cp=cp+1; skipws(); if (src[cp]==59) { cp=cp+1; psd_semi=1; }                 /* '}' then optional ';' (typedef: an alias follows, no ';') */
  if (uni) { off=mx; }                                                          /* union: size = max member size */
  { int fi; int al; al=1; fi=sfst[sidx]; while (fi<nfld) { if (fld_al[fi]>al) { al=fld_al[fi]; } fi=fi+1; }  /* R71: struct alignment = max member alignment */
    salign[sidx]=al; off=(off+al)-1; off=off/al; off=off*al; }                 /* round struct size to its OWN alignment (was always 8 -> SymAttr 2->8) */
  sfn[sidx]=nfld-sfst[sidx]; ssz[sidx]=off;
  if (snm_len[sidx]>0) {                                                       /* R79c: rebind deferred typedef aliases referring to this tag */
    int ti; int j; int m;
    ti=0; while (ti<ntd) {
      if (td_struct[ti]<0 && td_taglen[ti]==snm_len[sidx]) {
        m=1; j=0; while (j<snm_len[sidx]) { if (src[td_tagoff[ti]+j]!=src[snm_off[sidx]+j]) { m=0; } j=j+1; }
        if (m==1) { td_struct[ti]=sidx; } }
      ti=ti+1; } }
  dstruct=sidx; return 0; }
int parse_tagged_field(int off, int isuni, int isunion) { int cidx; int fw; int moff; int pw; int esz; int ealign; int ns;  /* R102: `struct/union TAG { body } [*..]name;` -- register TAG then lay out the field (tcc's switch_t nests `struct case_t {..} **p`) */
  if (isunion) { cp=cp+5; } else { cp=cp+6; }                                    /* consume "union"/"struct" */
  skipws(); read_ident(); dstruct_off=id_off; dstruct_len=id_len;                /* the tag name */
  skipws(); dstk=1; duni=isunion;                                                /* cp at '{' */
  parse_struct_def();                                                            /* register the tag + consume `{ body }`; leaves cp after '}' */
  cidx=find_struct(dstruct_off, dstruct_len); skipws();
  ns=0; while (src[cp]==42) { ns=ns+1; cp=cp+1; skipws(); }                       /* pointer stars */
  read_ident();                                                                  /* field name */
  if (ns>=1) { fw=8; pw=8; esz=8; ealign=8; }                                    /* pointer field -> 8 bytes */
  else { esz=ssz[cidx]; if (esz<1) { esz=1; } fw=esz; pw=esz; ealign=salign[cidx]; if (ealign<1) { ealign=1; } }  /* inline value field */
  moff=off; if (isuni==0) { off=(off+ealign)-1; off=off/ealign; off=off*ealign; moff=off; }  /* align */
  fld_soff[nfld]=dstruct_off; fld_slen[nfld]=dstruct_len; fld_long[nfld]=0; fld_unsig[nfld]=0;
  fldnm_off[nfld]=id_off; fldnm_len[nfld]=id_len; fldoff[nfld]=moff; fldw[nfld]=fw; fld_struct[nfld]=cidx; fld_arr[nfld]=0; fld_pw[nfld]=pw; fld_al[nfld]=ealign; nfld=nfld+1;
  skipws(); cp=cp+1; skipws();                                                   /* ';' */
  if (isuni) { return esz; } return moff+esz; }
int binop_at() { int c0; int c1; skipws(); if (cp>=slen) { return 0; }
  c0=src[cp]; c1=0; if (cp+1<slen) { c1=src[cp+1]; }
  if (c0==42) { op_id=1; oplen=1; return 10; } if (c0==47) { op_id=2; oplen=1; return 10; }
  if (c0==37) { op_id=3; oplen=1; return 10; } if (c0==43) { op_id=4; oplen=1; return 9; }
  if (c0==45) { op_id=5; oplen=1; return 9; }
  if (c0==60) { if (c1==60) { op_id=6; oplen=2; return 8; } if (c1==61) { op_id=9; oplen=2; return 7; } op_id=8; oplen=1; return 7; }
  if (c0==62) { if (c1==62) { op_id=7; oplen=2; return 8; } if (c1==61) { op_id=11; oplen=2; return 7; } op_id=10; oplen=1; return 7; }
  if (c0==61 && c1==61) { op_id=12; oplen=2; return 6; } if (c0==33 && c1==61) { op_id=13; oplen=2; return 6; }
  if (c0==38 && c1==38) { op_id=17; oplen=2; return 2; }                       /* && */
  if (c0==38 && c1!=38) { op_id=14; oplen=1; return 5; } if (c0==94) { op_id=15; oplen=1; return 4; }
  if (c0==124 && c1==124) { op_id=18; oplen=2; return 1; }                      /* || */
  if (c0==124 && c1!=124) { op_id=16; oplen=1; return 3; } return 0; }

int parse_expr(int min_prec);
int fn_returns_long(int noff, int nlen) { int i; int j; int m; i=0;  /* R138: is NAME a known long-returning fn? (fname_* table, filled in the pre-pass) */
  if (nlen==6 && src[noff]==115 && src[noff+1]==116 && src[noff+2]==114 && src[noff+3]==116 && src[noff+4]==111 && src[noff+5]==100) { return 1; }  /* R160/c99-R1: strtod returns a DOUBLE (8-byte bit pattern in rax) but has no in-unit prototype in tcc's fixture (system stdlib.h not inlined) -> the call would default to i32 (read only eax) and `tokc.d = strtod(...)` would drop the high 32 bits. Built-in like __syscall. (strtof returns float=4-byte in eax; strtold=long double, out of scope.) */
  while (i<nfunc) { if (fname_len[i]==nlen && fn_flng[i]) { m=1; j=0; while (j<nlen) { if (src[fname_off[i]+j]!=src[noff+j]) { m=0; } j=j+1; } if (m) { return 1; } } i=i+1; }
  i=0; while (i<pnfunc) { if (pname_len[i]==nlen && pn_flng[i]) { m=1; j=0; while (j<nlen) { if (src[pname_off[i]+j]!=src[noff+j]) { m=0; } j=j+1; } if (m) { return 1; } } i=i+1; }  /* R160/c99-R1: prototype return-types (external long/double-returning fns) */
  return 0; }
int fn_returns_sptr(int noff, int nlen) { int i; int j; int m; i=0;  /* R143: struct idx if NAME returns `Struct *`, else -1 (for f()->field) */
  while (i<nfunc) { if (fname_len[i]==nlen && fn_rsptr[i]>=0) { m=1; j=0; while (j<nlen) { if (src[fname_off[i]+j]!=src[noff+j]) { m=0; } j=j+1; } if (m) { return fn_rsptr[i]; } } i=i+1; }
  return 0-1; }
int parse_call(int noff, int nlen) { int args[16]; int na; int i; int isvar; int isglob; int tslot;  /* NAME(a,b,...) */
  id_off=noff; id_len=nlen; lvref(); isvar=lv_found; isglob=lv_glob; tslot=lv_sa;  /* callee a variable (fn ptr)? -> calli */
  cp=cp+1;                                                                      /* consume '(' */
  na=0; skipws();
  while (cp<slen && src[cp]!=41 && na<16) {                                     /* until ')' (na bound: no smash) */
    args[na]=parse_expr(0); na=na+1; skipws();
    if (src[cp]==44) { cp=cp+1; skipws(); } }                                  /* ',' */
  cp=cp+1;                                                                      /* ')' */
  if (isvar) {                                                                 /* function-pointer call -> calli vTARGET args */
    if (isglob) { tslot=emit_load(emit_gref(noff,nlen)); }                     /* global holds the fn address */
    es("  v"); en(ncount); es(": i32 = calli v"); en(tslot);
    i=0; while (i<na) { es(" v"); en(args[i]); i=i+1; } es("\n"); ncount=ncount+1; if (fn_returns_long(noff, nlen)) { is64[ncount-1]=1; } lit_huge=0; cast_un=0; cast_ps=0; return ncount-1; }
  es("  v"); en(ncount); es(": i32 = ");
  if (nlen==9 && src[noff]==95 && src[noff+1]==95 && src[noff+2]==115) { es("syscall "); }  /* __syscall -> syscall */
  else if (nlen==6 && src[noff]==115 && src[noff+1]==101 && src[noff+2]==116 && src[noff+3]==106 && src[noff+4]==109 && src[noff+5]==112) { es("setjmp "); }   /* R112: setjmp -> cf intrinsic (saves rbx/r12/r13/rsp/rip to a fixed buf, returns 0; longjmp restores). Operand kept for liveness; cf ignores it (tcc has one active jmp_buf). */
  else if (nlen==7 && src[noff]==108 && src[noff+1]==111 && src[noff+2]==110 && src[noff+3]==103 && src[noff+4]==106 && src[noff+5]==109 && src[noff+6]==112) { es("longjmp "); }  /* R112: longjmp -> cf intrinsic (restores + jmp, always returns 1; tcc_error always longjmp(buf,1)) */
  else { es("call "); i=0; while (i<nlen) { out[outn]=src[noff+i]; outn=outn+1; i=i+1; } }   /* call NAME */
  i=0; while (i<na) { es(" v"); en(args[i]); i=i+1; }                           /* arg slots */
  es("\n"); ncount=ncount+1; if (fn_returns_long(noff, nlen)) { is64[ncount-1]=1; ishi[ncount-1]=1; }  /* R138: long/double-returning call -> is64. R160/c99-R1: ALSO ishi -- the result genuinely spans 64 bits (e.g. `tokc.d = strtod(...)` double bits), so a store into a SIGNED-long-shaped field must NOT sign-extend (convs reads only the low 32 bits -> drops the high word). Safe vs r_addend=-4 (that rhs is a 32-bit CONST, not a call). */
  if (nlen==6 && src[noff]==115 && src[noff+1]==116 && src[noff+2]==114 && src[noff+3]==116 && src[noff+4]==111 && src[noff+5]==100) { isflt[ncount-1]=1; }  /* R160/c99-R3: strtod returns a DOUBLE (bit pattern) -> isflt, so `tokc.d = strtod(...)` stores the bits directly (NOT re-converted via __floatsidf -- treating the bits as an int gave 0.0 for every float literal). */
  lit_huge=0; cast_un=0; cast_ps=0;                                              /* R160: a call RESULT's hugeness/signedness/pointee-width come from its RETURN type, never from an ARGUMENT. Clear flags a huge/unsigned/cast arg set, else `f(0x..HUGE)==1` mis-folds (stale lit_huge) and `cmp(u64,u64)<0` becomes an unsigned `ltu` (stale cast_un) -> a negative result reads as >=0 */
  return ncount-1; }
int parse_string() { int base; int bref; int len; int p; int off; int ch; int va; int vc; int vo;  /* "..." -> local buffer */
  p=cp+1; len=0;                                                                /* count chars (escapes), across ADJACENT literals ("a" "b") */
  while (1) { while (src[p]!=34) { if (src[p]==92) { p=p+1; } p=p+1; len=len+1; }
    p=p+1; while (src[p]==32 || src[p]==9 || src[p]==10 || src[p]==13) { p=p+1; } if (src[p]!=34) { break; } p=p+1; }
  base=ncount; ncount=ncount+(len+1+7)/8;                                       /* reserve ceil((len+1)/8) byte slots */
  bref=emit_ref(base);
  cp=cp+1; off=0;                                                               /* storeb each char */
  while (1) { while (src[cp]!=34) {
    ch=src[cp]; if (ch==92) { cp=cp+1; ch=src[cp]; if (ch==110) { ch=10; } else if (ch==116) { ch=9; } else if (ch==48) { ch=0; } else if (ch==114) { ch=13; } else if (ch==97) { ch=7; } else if (ch==98) { ch=8; } else if (ch==118) { ch=11; } else if (ch==102) { ch=12; } else if (ch==101) { ch=27; } }
    vc=emit_const(ch); vo=emit_const(off); va=emit_binop(4, bref, vo);
    es("  storeb v"); en(va); es(" v"); en(vc); es("\n"); cp=cp+1; off=off+1; }
    cp=cp+1; while (src[cp]==32 || src[cp]==9 || src[cp]==10 || src[cp]==13) { cp=cp+1; } if (src[cp]!=34) { break; } cp=cp+1; }  /* past ", into next adjacent literal or done */
  vc=emit_const(0); vo=emit_const(off); va=emit_binop(4, bref, vo);            /* null terminator */
  es("  storeb v"); en(va); es(" v"); en(vc); es("\n"); return bref; }
int hexdig (int c) { if (c>=48 && c<=57) { return c-48; }   /* hex digit value, or -1 */
  if (c>=97 && c<=102) { return c-87; } if (c>=65 && c<=70) { return c-55; } return 0-1; }
int la_w; int la_struct; int la_arr; int la_long; int la_unsig; int la_flt;      /* lval_addr outputs: width (1/4/8) + residual struct idx + is-array (decays to address); R91: la_long = genuine long field; R94: la_unsig = field unsignedness; R160/c99-R3: la_flt = floating field */
int r130_pw;                                                                     /* R130: pointee width of a `++(field)` result, so an enclosing `*(++(field))` deref picks the right load width (parse_atom locals can't cross the recursive call) */
int cast_pw;                                                                     /* pointee width (1/4/8) of the most recent (TYPE*) cast -- for an enclosing ((TYPE*)e)[i] */
int cast_ps; int cast_un; int lit_huge;                                        /* R55: literal exceeded 32 bits (folds compares); unsigned literal rides cast_un */                                                                     /* R45: pointee SIZE (ssz for struct*) of the most recent pointer cast -- scales an immediately-following +/- (((T*)p)-1) */
int lit_lo; int lit_hi;                                                        /* R132: a >=2^32 integer literal decomposed into low/high 32-bit halves (16-bit-limb decimal accumulation -- no 64-bit arithmetic in ccc, so bootstrap-safe) */
int ce_hi;                                                                     /* R160: high 32 bits of the last const_term()/const_expr() literal (0 if it fits 32 bits) -- lets a 64-bit hex/decimal INITIALIZER keep its high word (parse_const_val's `int v` truncates) */
int chain_pure() { int p; int d; p=cp;                                         /* peek: is the accessor chain from cp a . / -> field chain (subscripts on array/ptr fields OK)? */
  while (1) {
    if (src[p]==45 && src[p+1]==62) { p=p+2; } else if (src[p]==46) { p=p+1; } else { return 1; }   /* chain ended at a non-accessor -> handled by lval_addr */
    while (p<slen && (src[p]==32 || src[p]==9)) { p=p+1; }
    while (p<slen && isalnum_(src[p])) { p=p+1; }                              /* field name */
    while (p<slen && (src[p]==32 || src[p]==9)) { p=p+1; }
    while (src[p]==91) {                                                        /* R32: subscript(s) on this field -- skip balanced [...], lval_cont handles them */
      d=1; p=p+1; while (p<slen && d>0) { if (src[p]==91) { d=d+1; } if (src[p]==93) { d=d-1; } p=p+1; }
      while (p<slen && (src[p]==32 || src[p]==9)) { p=p+1; } } } }
/* Typed-value lvalue resolver (R21). Pure . / -> field chains only. All chain type-state in LOCALS
   (a=address of current field, st=its struct idx, w=its width); find_field outputs consumed to locals
   at each boundary; outputs written ONLY to la_w/la_struct at return. No sub-parse, and no read of the
   lv_ or ff globals after the primary -- so the R20 accumulation-desync is structurally unrepresentable. */
int lval_cont(int a, int st, int w) { int base; int wasarr; int idx; int pw; int lo; int lu; int lfl;   /* continue a . / -> chain from address a (current field: struct st, width w); handles [i] on array/ptr fields */
  wasarr=ffarr; pw=ffpw; lo=fflong; lu=ffunsig; lfl=fffloat;                    /* the incoming field's array flag + element/pointee width (R62) + long-ness (R91) + unsignedness (R94) + floatness (R160/c99-R3) */
  while (1) {
    if (src[cp]==45 && src[cp+1]==62) { cp=cp+2; base=emit_load8(a); read_ident(); find_field(st, id_off, id_len);
      a=emit_binop(4, base, emit_const(ffoff)); w=ffw; st=ffstruct; wasarr=ffarr; pw=ffpw; lo=fflong; lu=ffunsig; lfl=fffloat; }  /* -> : field is a pointer; deref then +off */
    else if (src[cp]==46) { cp=cp+1; read_ident(); find_field(st, id_off, id_len);
      a=emit_binop(4, a, emit_const(ffoff)); w=ffw; st=ffstruct; wasarr=ffarr; pw=ffpw; lo=fflong; lu=ffunsig; lfl=fffloat; }     /* .  : inline struct; +off */
    else if (src[cp]==91) { cp=cp+1; idx=parse_expr(0); skipws(); cp=cp+1;    /* R32: [i] -- subscript the current field */
      if (wasarr==0) { a=emit_load8(a); }                                      /* pointer field: deref to the array base (array field: a is already the base) */
      if (pw==1) { a=emit_binop(4, a, idx); } else if (pw==2) { a=emit_binop(4, a, emit_binop(6, idx, emit_const(1))); }  /* R62: stride = POINTEE width (char* -> 1, short* -> 2) */
      else if (pw==8) { a=emit_binop(4, a, emit_binop(6, idx, emit_const(3))); } else { a=emit_binop(4, a, emit_binop(6, idx, emit_const(2))); }
      w=pw; wasarr=0; lo=0; lu=0; lfl=0; }                                     /* after subscript: element width = pw; a -> element, not an array (R91: element long-ness unknown -> conservative 0) */
    else { la_w=w; la_struct=st; la_arr=wasarr; la_long=lo; la_unsig=lu; la_flt=lfl; return a; } } }
int lval_addr() { int a; int st; int w; int pg; int psa; int poff; int plen; int base;
  read_ident(); lvref(); pg=lv_glob; psa=lv_sa; poff=lv_noff; plen=lv_nlen; st=lv_struct;
  if (src[cp]==45 && src[cp+1]==62) { if (pg) { base=emit_load(emit_gref(poff,plen)); } else { base=psa; } }  /* primary -> : base = pointer value (struct addr) */
  else { if (pg) { base=emit_gref(poff,plen); } else { base=emit_ref(psa); } }                                /* primary .  : base = struct address */
  if (src[cp]==45 && src[cp+1]==62) { cp=cp+2; } else { cp=cp+1; }             /* consume first accessor */
  read_ident(); find_field(st, id_off, id_len);
  a=emit_binop(4, base, emit_const(ffoff)); w=ffw; st=ffstruct;               /* a = address of first field */
  return lval_cont(a, st, w); }
int parse_expr(int mp);
int emit_icall(int target) { int a[16]; int n; int i; int r;                    /* R153: cp at '(' -- parse (args) and emit `calli vTARGET args...` through an already-loaded fn-pointer VALUE (e.g. a dispatch-table element tbl[i]()). Own frame -> a[] is recursion-safe. Mirrors parse_call's calli path (ccc.c:446). */
  cp=cp+1; skipws(); n=0;
  while (cp<slen && src[cp]!=41 && n<16) { a[n]=parse_expr(0); n=n+1; skipws(); if (src[cp]==44) { cp=cp+1; skipws(); } }
  cp=cp+1;                                                                       /* ')' */
  r=ncount; es("  v"); en(ncount); es(": i32 = calli v"); en(target);
  i=0; while (i<n) { es(" v"); en(a[i]); i=i+1; } es("\n"); ncount=ncount+1; lit_huge=0; cast_un=0; cast_ps=0; return r; }  /* R160: clear stale lit_huge/cast_un/cast_ps from arguments (see parse_call) */
int parse_atom() { int t; int s; int c0; int b8; int ag; int asa; int aa; int ac; int aptr; int astruct; int aoff; int alen; int along; int cwuns; int cflt;
  skipws(); c0=src[cp];
  if ((c0==43 && src[cp+1]==43) || (c0==45 && src[cp+1]==45)) {                  /* ++x / --x (pre): write then return new (simple ident) */
    if (c0==43) { s=4; } else { s=5; }                                          /* add / sub */
    cp=cp+2; skipws();
    if (src[cp]==40) {                                                           /* R130: ++(lvalue) as a VALUE -- parenthesized pre-inc/dec of a (possibly ->/. chained) lvalue; return the NEW value (tcc's inp(): `ch = *(++(file->buf_ptr))`). Was: read_ident on `(` -> garbage -> SEGV */
      cp=cp+1; skipws(); aa=cp; read_ident(); skipws();
      cp=aa; b8=lval_addr(); skipws(); t=la_w; r130_pw=ffpw;                    /* ffpw = pointee width for the enclosing `*(...)` */
      ac=1; if (t==8 && la_long==0) { ac=ffpw; }                                /* pointer field: increment stride = pointee width (char*->1, int*->4); long/scalar field -> 1 */
      if (t==1) { aa=emit_binop(s, emit_loadb(b8), emit_const(ac)); es("  storeb v"); en(b8); es(" v"); en(aa); es("\n"); }
      else if (t==2) { aa=emit_binop(s, emit_load2(b8), emit_const(ac)); es("  store2 v"); en(b8); es(" v"); en(aa); es("\n"); }
      else if (t==8) { aa=emit_binop8(s, emit_load8(b8), emit_const(ac)); es("  store8 v"); en(b8); es(" v"); en(aa); es("\n"); }  /* 64-bit ptr arith (don't truncate the pointer) */
      else { aa=emit_binop(s, emit_load(b8), emit_const(ac)); es("  store v"); en(b8); es(" v"); en(aa); es("\n"); }
      skipws(); if (src[cp]==41) { cp=cp+1; } return aa; }
    read_ident(); lvref();
    ag=lv_glob; asa=lv_sa; ac=lv_char; aptr=lv_ptr; along=lv_long; aoff=lv_noff; alen=lv_nlen;
    t=1; if (aptr>=1) { if (aptr>=2 || along) { t=8; } else if (ac) { t=1; }     /* R149: ptr-depth>=2 BEFORE char -- `char **e; ++e` strides by 8 (pointee is a pointer), not 1 */
      else if (aptr==1 && lv_struct>=0) { t=ssz[lv_struct]; }                    /* R45: struct* stride = ssz (e.g. ++popt over an option table) */
      else { t=4; } }
    if (ag) { b8=emit_gref(aoff,alen); asa=emit_load(b8);
      aa=emit_binop(s, asa, emit_const(t)); es("  store v"); en(b8); es(" v"); en(aa); es("\n"); return aa; }
    aa=emit_binop(s, asa, emit_const(t)); emit_copy(asa, aa); return aa; }
  if (c0==115 && src[cp+1]==105 && src[cp+2]==122 && src[cp+3]==101 && src[cp+4]==111 && src[cp+5]==102
      && (src[cp+6]==40 || src[cp+6]==32 || src[cp+6]==9 || src[cp+6]==10)) {     /* sizeof -> constant (TYPE, (EXPR), or bare EXPR) */
    cp=cp+6; skipws(); aptr=0;
    if (src[cp]==40) { aptr=1; s=cp+1; cp=cp+1; skipws();                        /* '(' */
      if (decl_type()) {                                                         /* sizeof(TYPE) */
        t=dptr; skipws(); if (src[cp]==42) { t=1; } skipptr();                    /* R29: skipws so `sizeof(T *)` (space before *) sees the pointer -> 8, not the struct size */
        b8=4; if (t) { b8=8; } else { if (dchar) { b8=1; } else { if (dshort && dstk==0) { b8=2; } else { if (dlong) { b8=8; }
          else { if (dstk) { b8=4; if (dstruct>=0) { b8=ssz[dstruct]; } } } } } }
        skipws(); cp=cp+1; return emit_const(b8); }                              /* ')' */
      cp=s; }                                                                    /* not a type -> operand is an EXPR */
    skipws(); aa=0;                                                              /* sizeof EXPR / sizeof(EXPR): operand is a value */
    while (src[cp]==42) { aa=aa+1; cp=cp+1; skipws(); }                          /* leading derefs */
    b8=8;                                                                        /* default (complex expr) */
    if (isalnum_(src[cp]) && (src[cp]<48 || src[cp]>57)) {
      read_ident(); lvref(); ag=lv_ptr; ac=lv_char; along=lv_long; astruct=lv_struct; skipws();
      if (astruct>=0 && aa==0 && ((src[cp]==45 && src[cp+1]==62) || src[cp]==46)) {  /* R115: sizeof(p->field / s.field ...) -> the FINAL field's byte size (ffbytes: scalar width, array = esz*count, struct = ssz). The handler ignored the chain -> returned sizeof(ptr)=8 or the whole-struct size -> tcc's pstrcpy(bf->filename, sizeof(bf->filename), ..) copied only 7 chars, truncating error-message filenames. */
        while ((src[cp]==45 && src[cp+1]==62) || src[cp]==46) {
          if (src[cp]==46) { cp=cp+1; } else { cp=cp+2; }
          read_ident(); find_field(astruct, id_off, id_len); astruct=ffstruct; }
        b8=ffbytes; }
      else {
      if (src[cp]==91) { aa=aa+1; }                                              /* ident[..] -> element */
      if (aa>0) { if (ac) { b8=1; } else if (ag>=2 || along) { b8=8; } else if (astruct>=0) { b8=ssz[astruct]; } else { b8=4; } }  /* element size (R80: *structptr -> struct size, not 4 -- else memset(s,0,sizeof *s) under-clears a Sym) */
      else if (astruct>=0 && ag<1 && lv_bytes==0) { b8=ssz[astruct]; }           /* struct value */
      else if (lv_bytes>0) { b8=lv_bytes; }                                      /* whole array */
      else if (ac) { b8=1; } else if (ag>=1 || along) { b8=8; } else { b8=4; } } } /* scalar */
    if (aptr) { t=1; while (cp<slen && t>0) { if (src[cp]==40) { t=t+1; } if (src[cp]==41) { t=t-1; } cp=cp+1; } }  /* (EXPR): skip to ')' */
    else { if (src[cp]==91) { t=1; cp=cp+1; while (cp<slen && t>0) { if (src[cp]==91) { t=t+1; } if (src[cp]==93) { t=t-1; } cp=cp+1; } } }  /* bare: consume operand's [..] */
    return emit_const(b8); }
  if (c0==34) { t=parse_string(); skipws();                                     /* "string" literal */
    while (src[cp]==91) { cp=cp+1; b8=parse_expr(0); skipws(); cp=cp+1; t=emit_getb(t, b8); skipws(); }  /* R45: "lit"[i] subscript (e.g. ":"[0]) */
    return t; }
  if (c0==33) { cp=cp+1; t=parse_atom(); return emit_notbool(t); }              /* !atom == (atom == 0); R160: 64-bit-aware */
  if (c0==126) { cp=cp+1; t=parse_atom(); t=emit_binop(5, czero, t); return emit_binop(5, t, emit_const(1)); }  /* ~atom == (0-atom)-1 (cf has no neg const) */
  if (c0==45) { cp=cp+1; t=parse_atom(); if (is64[t]) { return emit_binop8(5, czero, t); } s=(glastconst_v==t); t=emit_binop(5, czero, t); if (s) { isneg[t]=1; } return t; }  /* -atom (R160: 64-bit operand -> sub8, else `-4294967296L` truncated to 32-bit -> 0). R161/tcc-self-R3: `-CONST` (e.g. -1) tagged isneg so a 64-bit compare sign-extends it */
  if (c0==39) { cp=cp+1; s=src[cp];                                              /* 'c' char literal */
    if (s==92) { cp=cp+1; s=src[cp];                                             /* \escape */
      if (s==110) { s=10; } else if (s==116) { s=9; } else if (s==48) { s=0; } else if (s==114) { s=13; }
      else if (s==97) { s=7; } else if (s==98) { s=8; } else if (s==118) { s=11; } else if (s==102) { s=12; } else if (s==101) { s=27; } }  /* \a \b \v \f \e */
    cp=cp+1; cp=cp+1; return emit_const(s); }                                    /* char + closing ' */
  if (c0==38 && src[cp+1]==40) {                                                /* &( lvalue ) -- parenthesized address-of (e.g. &(p->hash_next)) */
    cp=cp+2; skipws();
    if (src[cp]==42) {                                                           /* R51: &(*ps)->f...->g = field's ADDRESS through a deref'd ptr-to-ptr */
      cp=cp+1; skipws(); read_ident(); lvref();
      if (lv_glob) { asa=emit_load(emit_gref(lv_noff, lv_nlen)); } else { asa=lv_sa; }
      asa=emit_load8(asa); b8=lv_struct;                                         /* (*ps): pointed-to pointer value; its struct type */
      skipws(); cp=cp+1; skipws();                                               /* ')' */
      while (src[cp]==45 && src[cp+1]==62) {
        cp=cp+2; read_ident(); find_field(b8, id_off, id_len);
        asa=emit_binop(4, asa, emit_const(ffoff)); b8=ffstruct;
        if (src[cp]==45 && src[cp+1]==62) { asa=emit_load8(asa); } }
      return asa; }
    if (src[cp]==40) {                                                          /* R89: &((TYPE*)base)[i] -- address of a cast-subscript element (find_elf_sym: &((Elf64_Sym*)s->data)[idx]) */
      s=cp; cp=cp+1;
      if (decl_type()) {                                                        /* confirmed a cast (TYPE*) */
        cp=s; asa=parse_atom(); b8=cast_ps;                                     /* rewind; parse_atom yields the pointer value + records cast_ps stride */
        skipws(); if (src[cp]==41) { cp=cp+1; }                                 /* consume the &(...) closing ')' */
        skipws();
        if (src[cp]==91) { cp=cp+1; t=parse_expr(0); skipws(); cp=cp+1;         /* [idx] : base + idx*stride */
          asa=emit_binop(4, asa, emit_binop(1, t, emit_const(b8))); }
        return asa; }
      cp=s; }                                                                   /* not a cast: fall through to the chain handling below */
    s=cp; read_ident(); skipws();
    if ((src[cp]==45 && src[cp+1]==62) || src[cp]==46) {                        /* &(chain) -> field's address */
      cp=s; asa=lval_addr(); skipws(); cp=cp+1; return asa; }                   /* consume ')' */
    lvref(); ag=lv_glob; asa=lv_sa; aa=lv_arr; aoff=lv_noff; alen=lv_nlen; skipws(); cp=cp+1;  /* &(plain ident) ; consume ')' */
    if (ag) { return emit_gref(aoff, alen); }
    if (aa) { return asa; }
    return emit_ref(asa); }
  if (c0==38) { cp=cp+1; read_ident(); s=id_off; b8=id_len; lvref();            /* &lvalue */
    if (lv_found==0) { return emit_faddr(s, b8); }                             /* &function -> its address (faddr) */
    ag=lv_glob; asa=lv_sa; aa=lv_arr; ac=lv_char; aptr=lv_ptr; along=lv_long; aoff=lv_noff; alen=lv_nlen; astruct=lv_struct; skipws();
    if (((src[cp]==45 && src[cp+1]==62) || src[cp]==46) && chain_pure()) { cp=s; return lval_addr(); }  /* R21: &<pure . / -> chain> = the field's address */
    if (src[cp]==45 && src[cp+1]==62) {                                         /* &p->f...->g = the field's address (arrow chain, no final load; impure fallback) */
      if (ag) { asa=emit_load(emit_gref(aoff,alen)); }                          /* global ptr: load its value; local: asa is already the value */
      while (src[cp]==45 && src[cp+1]==62) {
        cp=cp+2; read_ident(); find_field(astruct, id_off, id_len);
        asa=emit_binop(4, asa, emit_const(ffoff)); astruct=ffstruct;
        if (src[cp]==45 && src[cp+1]==62) { asa=emit_load8(asa); } }            /* intermediate: load ptr, continue */
      return asa; }
    if (src[cp]==91) { cp=cp+1; t=parse_expr(0); skipws(); cp=cp+1;             /* &a[i] = base + i*elemsize (char:1 int:4 ptr:8 struct:ssz) */
      if (ac) { b8=t; } else if (aptr>=2 || along || (aa && aptr>=1)) { b8=emit_binop(6, t, emit_const(3)); }
      else if (astruct>=0 && (aptr==1 || (aptr==0 && aa))) { b8=emit_binop(1, t, emit_const(ssz[astruct])); }  /* R48: &structarr[i]; R98: &structPTR[i] (aptr==1, e.g. tcc's &vtop[-1].type) -- both stride by the element struct size, not a bare 4 */
      else { b8=emit_binop(6, t, emit_const(2)); }
      if (ag) { if (aa) { asa=emit_gref(aoff,alen); } else { asa=emit_load(emit_gref(aoff,alen)); }  /* R51: global ARRAY = address; global POINTER (TokenSym **tab) = its value */
        asa=emit_binop(4, asa, b8); }
      else { asa=emit_binop(4, asa, b8); }
      skipws();
      while ((src[cp]==46) || (src[cp]==45 && src[cp+1]==62)) {                   /* R84: trailing . / -> chain on &arr[i]; the DOT case was unhandled -> `&arr[i].field` left `.field` unparsed and desynced the whole function */
        if (src[cp]==45) { asa=emit_load8(asa); cp=cp+2; } else { cp=cp+1; }      /* -> : load the element's ptr first; . : struct-value element, no load */
        read_ident(); find_field(astruct, id_off, id_len);
        asa=emit_binop(4, asa, emit_const(ffoff)); astruct=ffstruct; }
      return asa; }
    if (ag) { return emit_gref(aoff,alen); }                                    /* &global = its address */
    if (aa) { return asa; }                                                    /* &local_array = base */
    return emit_ref(asa); }                                                    /* &local_scalar */
  if (c0==42) { cp=cp+1; skipws(); s=cp; aptr=0; along=0; astruct=0; r130_pw=0;   /* *atom (deref) -- honor pointee width */
    if ((src[cp]==43 && src[cp+1]==43) || (src[cp]==45 && src[cp+1]==45)) { cp=cp+2; skipws(); }  /* *++p / *--p : look past the pre-inc/dec to the ident for the type */
    if (isalnum_(src[cp]) && (src[cp]<48 || src[cp]>57)) {                        /* *IDENT: look up pointee type */
      read_ident(); lvref(); skipws();
      if (src[cp]!=46 && src[cp]!=91 && src[cp]!=40 && !(src[cp]==45 && src[cp+1]==62)) { aptr=lv_ptr; along=lv_char; astruct=lv_long; }  /* plain deref only (skip . [ ( arrow accessors) */
      }
    else if (src[cp]==40) { cp=cp+1; skipws();                                    /* R110: *(TYPE**)e -- the CAST's pointee width drives the deref load width (was defaulting to 4-byte -> `(*(case_t**)pa)->v1` read a truncated pointer -> case_cmp always 0 -> switch cases unsorted) */
      if (decl_type()) { skipptr(); if (nstar>=2 || dptr) { aptr=2; } else if (dlong) { astruct=1; } else if (dchar) { along=1; } } }
    cp=s;
    t=parse_atom();
    if (along==0 && aptr==0 && astruct==0 && r130_pw>0) {                         /* R130: pointee width came from a `*(++(field))` result -- apply it */
      if (r130_pw==1) { along=1; } else if (r130_pw==8) { astruct=1; } r130_pw=0; }
    if (along==0 && aptr==0 && astruct==0 && cast_ps==1) { along=1; }             /* R160/c99-R2: `*(EXPR)` where EXPR is a char* POINTER EXPRESSION (not *IDENT, not a cast) -- the deref'd expression's pointee width (cast_ps) drives the load. Was defaulting to `load` (4-byte) -> tcc's `*(p - 1) != ch` in the LL/ULL suffix lexer read 4 bytes -> mismatch -> "incorrect integer suffix". */
    if (aptr<2) { cast_ps=0; }                                                   /* R97: a scalar deref result (int/char/long pointee) is NOT a pointer -- clear the stride parse_atom left from reading `p` so a following `*p - n` doesn't pointer-scale n by the pointee size */
    if (astruct) { return emit_load8L(t); }                                      /* R160: *(long*) pointee -> is64 (was emit_load8 -> NOT is64 -> a `unsigned long x=*p` then emit_widen'd it with convs, truncating the high 32 bits) */
    if (aptr>=2) { return emit_load8(t); }                                       /* ptr-to-ptr: pointee is a POINTER (not is64-arithmetic) */
    if (along) { return emit_loadb(t); }                                         /* char pointee: 1-byte */
    return emit_load(t); }                                                       /* int pointee: 4-byte */
  if (c0==40) { s=cp; cp=cp+1;                                                  /* '(' -- cast (TYPE) or (expr) */
    if (decl_type()) { skipptr(); ac=4; if (nstar>=2) { ac=8; } else if (dchar) { ac=1; } else if (dlong) { ac=8; } skipws();  /* pointee width of the cast */
      if (src[cp]==41) { cp=cp+1; b8=0;                                          /* R45: pointee SIZE for cast-ptr +/- arithmetic (((T*)p)-1) */
        if (nstar>=1 || dptr) { b8=ac; if (nstar==1 && dchar==0 && dlong==0 && dstruct>=0) { b8=ssz[dstruct]; } }
        cwuns=dunsig; cflt=dflt;                                                 /* R160: capture cast unsignedness + floatness BEFORE parse_atom clobbers the decl globals (a `(u64)(expr)` operand recurses through decl_type) */
        t=parse_atom(); cast_pw=ac; cast_ps=b8; cast_un=cwuns;
        if (b8==0 && nstar==0 && dptr==0 && dstruct<0) {                          /* R99: a SCALAR narrowing cast must actually truncate (tcc's `r == (char)r` short/near-jump test relied on it; was identity -> always-short jump -> nested-loop displacement overflowed rel8 -> segfault) */
          if (cflt) { if (isflt[t]==0) { t=emit_int_to_flt(t); } }                /* R160/c99-R3: (double)/(float) of an INT -> __floatsidf; float->float is identity (double bits) */
          else if (isflt[t]) { t=emit_flt_to_int(t);                             /* R160/c99-R3: (int)/(char)... of a FLOAT -> __fixdfsi, then the normal int narrowing below on the int result */
            if (dchar) { if (dunsig) { t=emit_binop(14, t, emit_const(255)); } else { t=emit_binop(7, emit_binop(6, t, emit_const(24)), emit_const(24)); } }
            else if (dshort && dlong==0) { if (dunsig) { t=emit_binop(14, t, emit_const(65535)); } else { t=emit_binop(7, emit_binop(6, t, emit_const(16)), emit_const(16)); } } }
          else if (dchar) { if (dunsig) { t=emit_binop(14, t, emit_const(255)); }      /* (unsigned char): & 0xFF */
                       else { t=emit_binop(7, emit_binop(6, t, emit_const(24)), emit_const(24)); } }  /* (char): sign-extend low 8 bits via <<24 >>24 (>> is arithmetic) */
          else if (dshort && dlong==0) { if (dunsig) { t=emit_binop(14, t, emit_const(65535)); }      /* (unsigned short): & 0xFFFF */
                       else { t=emit_binop(7, emit_binop(6, t, emit_const(16)), emit_const(16)); } }   /* (short): <<16 >>16 */
          else if (ac==8 && is64[t]==0) { t=emit_widen(t, cwuns); }              /* R160: a WIDENING (long)/(unsigned long)/(uint64_t) cast -> genuine 64-bit value (ac==8 captured pre-parse_atom; was identity -> `(u64)(mm&MASK)<<29` shifted 32-bit -> high bits lost, softfloat __extendsfdf2 subnormal) */
          else if (ac==4 && dlong==0 && is64[t]) { if (cwuns) { t=emit_binop8(14, t, emit_int(0-1)); } else { t=emit_convs(t); } } }  /* R160/c99-R4: a NARROWING (int)/(unsigned) cast of a genuine 64-bit value MUST drop the high 32 bits (signed -> sext low32 via convs; unsigned -> mask 0xFFFFFFFF). Was identity -> `(int)c.i == c.i` (tcc's fits-in-imm32 check) compared c to itself = always true -> every 64-bit const truncated to imm32 in ALU ops (`x | 1<<52`). */
        return t; } }                    /* (TYPE)atom -> value; record cast_pw for an enclosing (cast)[i] */
    cp=s+1; skipws();
    if (isalnum_(src[cp]) && (src[cp]<48 || src[cp]>57)) {                        /* R111: (p--)->f / (p++)->f -- post-inc/dec of a pointer VAR then an arrow/dot field read. The generic (expr) path parsed `p--` (correct OLD-p value) but DROPPED the trailing ->field, so the enclosing parse_expr re-scanned `->f` as `- > f` -> emitted `gt` instead of load (tcc's gtst_addr `inv ^= (vtop--)->c.i` read garbage -> wrong jump opcode -> 9+ case switch crash). Field read must use the PRE-inc/dec pointer while p still gets updated. */
      b8=cp; read_ident(); skipws();
      if ((src[cp]==43 && src[cp+1]==43) || (src[cp]==45 && src[cp+1]==45)) {
        if (src[cp]==43) { aa=4; } else { aa=5; }                                /* add / sub */
        cp=cp+2; skipws();
        if (src[cp]==41) {
          ac=cp+1;
          while (ac<slen && (src[ac]==32 || src[ac]==10 || src[ac]==9 || src[ac]==13)) { ac=ac+1; }
          if ((src[ac]==45 && src[ac+1]==62) || src[ac]==46) {                   /* ')' then '->' or '.' : take this path */
            lvref(); astruct=lv_struct;
            if (lv_glob) { asa=emit_gref(lv_noff,lv_nlen); t=emit_load(asa); }    /* global ptr: slot addr + its value */
            else { asa=lv_sa; t=emit_binop(4, asa, emit_const(0)); }             /* local ptr: SNAPSHOT the OLD value -- emit_copy below mutates asa's SSA number in place, and the field walk must read the pre-dec pointer */
            s=emit_binop(aa, t, emit_const(ssz[astruct]));                        /* p +/- sizeof(*p); compute BEFORE the field walk (t stays OLD p) */
            if (lv_glob) { es("  store v"); en(asa); es(" v"); en(s); es("\n"); } /* write back the post-inc/dec */
            else { emit_copy(asa, s); }
            cp=ac;
            while ((src[cp]==45 && src[cp+1]==62) || src[cp]==46) {
              if (src[cp]==46) { cp=cp+1; } else { cp=cp+2; }
              read_ident(); find_field(astruct, id_off, id_len);
              t=emit_binop(4, t, emit_const(ffoff)); astruct=ffstruct;
              if (src[cp]==45 && src[cp+1]==62) { t=emit_load8(t); }             /* next arrow: field is a pointer, load it */
              else if (src[cp]==46) { }                                          /* next dot: inline sub-struct, keep address */
              else { if (ffw==1) { return emit_loadb_sx(t, ffunsig); } if (ffw==2) { return emit_load2(t); } if (ffw==8) { return emit_fld8(t); } return emit_load(t); } }
            return t; } }
      }
      cp=s+1; }                                                                  /* probe failed: restore for R51 / generic (expr) */
    if (src[cp]==42) {                                                           /* R51: (*ps)->f...->g READ (deref'd ptr-to-ptr, arrow chain after the parens) */
      b8=cp; cp=cp+1; skipws();
      if (isalnum_(src[cp]) && (src[cp]<48 || src[cp]>57)) {
        read_ident(); skipws();
        if (src[cp]==41) { t=cp+1;
          while (t<slen && (src[t]==32 || src[t]==10 || src[t]==9 || src[t]==13)) { t=t+1; }
          if (src[t]==45 && src[t+1]==62) {                                      /* ')' then '->' : take this path */
            lvref();
            if (lv_glob) { asa=emit_load(emit_gref(lv_noff, lv_nlen)); } else { asa=lv_sa; }
            asa=emit_load8(asa); ac=lv_struct;                                   /* (*ps) value + its struct type */
            cp=t;
            while (src[cp]==45 && src[cp+1]==62) {
              cp=cp+2; read_ident(); find_field(ac, id_off, id_len);
              asa=emit_binop(4, asa, emit_const(ffoff)); ac=ffstruct;
              if (src[cp]==45 && src[cp+1]==62) { asa=emit_load8(asa); }
              else { if (ffw==1) { return emit_loadb_sx(asa, ffunsig); } if (ffw==2) { return emit_load2(asa); } if (ffw==8) { return emit_fld8(asa); } return emit_load(asa); } }
            return asa; } } }
      if (src[cp]==40) {                                                         /* R110: (*(TYPE**)e)->f...->g READ -- deref a CAST'd ptr-to-ptr, then arrow chain (tcc's case_cmp: `(*(struct case_t**)pa)->v1`) */
        t=cp; cp=cp+1; skipws();
        if (decl_type()) { ac=dstruct; skipptr(); skipws(); if (src[cp]==41) { cp=cp+1; }  /* past the cast '(TYPE**)' -> ac = pointee struct */
          asa=parse_atom(); asa=emit_load8(asa); skipws();                       /* *e : the pointed-to pointer VALUE (8-byte) */
          if (src[cp]==41) { cp=cp+1; skipws();                                  /* ')' of the outer parens */
            if (src[cp]==45 && src[cp+1]==62) {
              while (src[cp]==45 && src[cp+1]==62) {
                cp=cp+2; read_ident(); find_field(ac, id_off, id_len);
                asa=emit_binop(4, asa, emit_const(ffoff)); ac=ffstruct;
                if (src[cp]==45 && src[cp+1]==62) { asa=emit_load8(asa); }
                else { if (ffw==1) { return emit_loadb_sx(asa, ffunsig); } if (ffw==2) { return emit_load2(asa); } if (ffw==8) { return emit_fld8(asa); } return emit_load(asa); } }
              return asa; } } }
        cp=t; }
      cp=b8; }                                                                   /* not (*ID)->... -> restore, generic (expr) */
    cast_pw=0; cp=s+1; t=parse_expr(0); skipws(); cp=cp+1;                       /* (expr) */
    while (src[cp]==91) { cp=cp+1; b8=parse_expr(0); skipws(); cp=cp+1;          /* (expr)[i] postfix subscript (e.g. ((unsigned char*)str)[i]) */
      if (cast_pw==1) { t=emit_loadb(emit_binop(4, t, b8)); }
      else if (cast_pw==8) { t=emit_load8(emit_binop(4, t, emit_binop(6, b8, emit_const(3)))); }
      else { t=emit_load(emit_binop(4, t, emit_binop(6, b8, emit_const(2)))); } cast_pw=0; }
    return t; }                                                                 /* (expr) */
  if (isalnum_(c0) && (c0<48 || c0>57)) { read_ident(); skipws();
    if (src[cp]==40) {                                                          /* f(...) call, or a varargs builtin */
      if (id_is("__va_start", 10)) { cp=cp+1; t=parse_expr(0); skipws(); cp=cp+1;   /* t = &va_list (long[2]) */
        s=emit_conv(emit_ref(cur_va_base));                                        /* this fn's __va_area address */
        es("  store8 v"); en(t); es(" v"); en(s); es("\n");                        /* va[0] = base */
        b8=emit_binop(4, t, emit_const(8)); s=emit_conv(emit_const(cur_nparams));  /* &va[1]; named count */
        es("  store8 v"); en(b8); es(" v"); en(s); es("\n"); return t; }           /* va[1] = index (= count) */
      if (id_is("__va_next", 9)) { cp=cp+1; t=parse_expr(0); skipws(); cp=cp+1;    /* t = va_list; read+advance */
        s=emit_load8(t); b8=emit_binop(4, t, emit_const(8)); asa=emit_load8(b8);   /* base; &va[1]; idx */
        aa=emit_load8(emit_binop(4, s, emit_binop(6, asa, emit_const(3))));        /* val = base[idx] (8-byte) */
        ac=emit_conv(emit_binop(4, asa, emit_const(1)));                           /* idx+1 */
        es("  store8 v"); en(b8); es(" v"); en(ac); es("\n"); return aa; }         /* va[1] = idx+1 */
      if (id_is("__va_copy", 9)) { cp=cp+1; t=parse_expr(0); skipws(); cp=cp+1;    /* __va_copy(d, s): copy 2 longs */
        s=parse_expr(0); skipws(); cp=cp+1;
        b8=emit_load8(s); es("  store8 v"); en(t); es(" v"); en(b8); es("\n");      /* d[0] = s[0] */
        asa=emit_binop(4, t, emit_const(8)); aa=emit_load8(emit_binop(4, s, emit_const(8)));
        es("  store8 v"); en(asa); es(" v"); en(aa); es("\n"); return t; }          /* d[1] = s[1] */
      s=id_off; b8=id_len; astruct=fn_returns_sptr(s, b8);                        /* R143: `f()->field` -- capture the return struct BEFORE parse_call clobbers id_* via arg parsing */
      asa=parse_call(s, b8); skipws();
      if (astruct>=0 && src[cp]==45 && src[cp+1]==62) {                            /* call result is a struct ptr, followed by -> : chain the field read (mirrors the p->field->.. path) */
        while (src[cp]==45 && src[cp+1]==62) {
          cp=cp+2; read_ident(); find_field(astruct, id_off, id_len);
          asa=emit_binop(4, asa, emit_const(ffoff)); astruct=ffstruct;
          if (src[cp]==45 && src[cp+1]==62) { asa=emit_load8(asa); }
          else { if (ffarr) { return asa; } if (ffw==1) { return emit_loadb_sx(asa, ffunsig); } if (ffw==2) { return emit_load2(asa); } if (ffw==8) { return emit_fld8(asa); } return emit_load(asa); } }
        return asa; }
      return asa; }
    s=id_off; b8=id_len; lvref(); ag=lv_glob; asa=lv_sa; aa=lv_arr; ac=lv_char; aptr=lv_ptr; along=lv_long; astruct=lv_struct; aoff=lv_noff; alen=lv_nlen;
    cast_un=lv_unsig;                                                          /* R64: unsigned scalar var -> value is unsigned (logical >>); subscript/postfix sub-parses reset it */
    cast_ps=0;                                                                /* R66: pointer VARIABLE + n scales by pointee size (like R45's cast_ps) -- sym++ in a Sym pool must stride ssz */
    if (lv_ptr>=1) { if (lv_char) { cast_ps=1; } else if (lv_ptr>=2 || lv_long) { cast_ps=8; } else if (lv_struct>=0) { cast_ps=ssz[lv_struct]; } else { cast_ps=4; } }
    else if (lv_arr) { if (lv_char) { cast_ps=1; } else if (lv_long) { cast_ps=8; } else if (lv_struct>=0) { cast_ps=ssz[lv_struct]; } else { cast_ps=4; } }  /* R95: array name `a` decays to &a[0] -- a bare `a + n` must scale by ELEMENT size (tcc's `(__vstack + 1)`; unscaled -> misaligned value stack -> get_reg register collisions) */
    if (lv_found==0) { t=find_enum(s, b8); if (enm_found) { return emit_const(t); }   /* enum constant -> its value */
      return emit_faddr(s, b8); }                                             /* else bare function name -> faddr */
    if (is_caop() && aa==0 && aptr==0 && astruct<0) {                         /* R101: scalar `x OP= v` used as an EXPRESSION -> perform the store AND yield the new value (tcc's `if ((r &= 0x3f) >= 0x30)`); statement-level OP= is handled in parse_stmt and never reaches here */
      cp=cp+ca_len; b8=parse_expr(0);
      if (ag) { asa=emit_gref(aoff,alen); t=emit_binop(ca_op, emit_load(asa), b8); es("  store v"); en(asa); es(" v"); en(t); es("\n"); return t; }
      t=emit_binop(ca_op, asa, b8); emit_copy(asa, t); return t; }
    if (((src[cp]==45 && src[cp+1]==62) || src[cp]==46) && chain_pure()) {      /* R21: pure . / -> chain -> typed-value resolver */
      cp=s; asa=lval_addr();
      if (la_arr) { cast_ps=la_w; return asa; }                                /* R28: array field decays to its address; R69: element stride for a following `+ n` (was leaking the BASE ptr's ssz) */
      cast_ps=0;                                                               /* R69: a loaded scalar field value is not the base pointer -- clear the R66 leak */
      if (la_w==1) { return emit_loadb_sx(asa, la_unsig); } if (la_w==2) { return emit_load2(asa); } if (la_w==8) { return emit_lafld8(asa); } return emit_load(asa); }
    if (src[cp]==45 && src[cp+1]==62) {                                         /* p->field->... (chained read; impure/subscript fallback) */
      if (ag) { asa=emit_load(emit_gref(aoff,alen)); }                         /* global ptr: load its value */
      while (src[cp]==45 && src[cp+1]==62) {
        cp=cp+2; read_ident(); find_field(astruct, id_off, id_len);
        asa=emit_binop(4, asa, emit_const(ffoff));                             /* address = base + field offset */
        astruct=ffstruct;                                                      /* field's struct type -> next -> in chain */
        if (src[cp]==45 && src[cp+1]==62) { asa=emit_load8(asa); }             /* intermediate: load the ptr, continue */
        else { if (ffw==1) { return emit_loadb_sx(asa, ffunsig); } if (ffw==2) { return emit_load2(asa); } if (ffw==8) { return emit_fld8(asa); } return emit_load(asa); } }
      return asa; }
    if (src[cp]==46) {                                                          /* x.field.field... (read; nested sub-structs are inline -> accumulate offsets) */
      if (ag) { asa=emit_gref(aoff,alen); } else { asa=emit_ref(asa); }         /* &struct base */
      while (src[cp]==46) {
        cp=cp+1; read_ident(); find_field(astruct, id_off, id_len);
        asa=emit_binop(4, asa, emit_const(ffoff)); astruct=ffstruct;           /* addr += offset; descend */
        if (src[cp]==45 && src[cp+1]==62) {                                     /* x.field->... : field is a pointer -> load, continue as arrow */
          asa=emit_load8(asa);
          while (src[cp]==45 && src[cp+1]==62) {
            cp=cp+2; read_ident(); find_field(astruct, id_off, id_len);
            asa=emit_binop(4, asa, emit_const(ffoff)); astruct=ffstruct;
            if (src[cp]==45 && src[cp+1]==62) { asa=emit_load8(asa); }
            else { if (ffw==1) { return emit_loadb_sx(asa, ffunsig); } if (ffw==2) { return emit_load2(asa); } if (ffw==8) { return emit_fld8(asa); } return emit_load(asa); } }
          return asa; }
        if (src[cp]!=46) {                                                      /* last field: load width-appropriate */
          if (ffw==1) { return emit_loadb_sx(asa, ffunsig); } if (ffw==2) { return emit_load2(asa); }
          if (ffw==8) { return emit_fld8(asa); }
          return emit_load(asa); } }
      return asa; }
    if (src[cp]==91) { cp=cp+1; t=parse_expr(0); skipws(); cp=cp+1;             /* x[i] */
      if (ag) { if (aa) { asa=emit_gref(aoff,alen); } else { asa=emit_load(emit_gref(aoff,alen)); } }  /* global array: address; global pointer: its value */
      if (src[cp]==46 && astruct>=0 && (aptr==1 || (aptr==0 && aa))) {           /* R21 P4 + R48: p[i].field / arr[i].field (read; element addr = base + i*ssz) */
        asa=emit_binop(4, asa, emit_binop(1, t, emit_const(ssz[astruct])));      /* addr of element = base + i*ssz */
        asa=lval_cont(asa, astruct, 8);                                          /* run trailing . / -> chain */
        if (la_arr) { return asa; }                                              /* R28: array field decays to its address */
        if (la_w==1) { return emit_loadb_sx(asa, la_unsig); } if (la_w==2) { return emit_load2(asa); } if (la_w==8) { return emit_lafld8(asa); } return emit_load(asa); }
      if (src[cp]==45 && src[cp+1]==62) {                                        /* x[i]->field... (read; element is a struct pointer) */
        if (aptr>=2 || along || (aa && aptr>=1)) { asa=emit_load8(emit_binop(4, asa, emit_binop(6, t, emit_const(3)))); }  /* R36: pointer-element array (T*[] / T**) -> 8-byte element, stride 8 */
        else { asa=emit_load(emit_binop(4, asa, emit_binop(6, t, emit_const(2)))); }                    /* int-element array -> 4-byte, stride 4 */
        while (src[cp]==45 && src[cp+1]==62) {
          cp=cp+2; read_ident(); find_field(astruct, id_off, id_len);
          asa=emit_binop(4, asa, emit_const(ffoff)); astruct=ffstruct;           /* addr = base + field offset */
          if (src[cp]==45 && src[cp+1]==62) { asa=emit_load8(asa); }             /* intermediate: struct ptr field is 8-byte */
          else { if (ffarr) { return asa; }                                      /* R57: array field decays to its address (table_ident[v]->str) */
            if (ffw==1) { return emit_loadb_sx(asa, ffunsig); } if (ffw==2) { return emit_load2(asa); } if (ffw==8) { return emit_fld8(asa); } return emit_load(asa); } }
        return asa; }
      if (aptr>=2 || along || (aa && aptr>=1)) {                                /* long/ptr element (incl T*[] array): 8-byte load, stride 8 */
        asa=emit_load8(emit_binop(4, asa, emit_binop(6, t, emit_const(3))));
        if (aptr>=2) { s=aptr-1; } else if (aa && aptr>=1) { s=aptr; } else { s=0; }  /* R116: remaining pointer depth of the element */
        while (src[cp]==91) {                                                   /* R116: p[i][j] chained subscript through pointer levels (ptr-to-ptr / arrays of pointers were DROPPING the 2nd subscript, returning the element pointer) */
          cp=cp+1; t=parse_expr(0); skipws(); cp=cp+1;
          if (s>=2) { asa=emit_load8(emit_binop(4, asa, emit_binop(6, t, emit_const(3)))); s=s-1; }
          else if (s==1) { if (ac) { asa=emit_getb(asa, t); } else if (along) { asa=emit_load8(emit_binop(4, asa, emit_binop(6, t, emit_const(3)))); } else { asa=emit_load(emit_binop(4, asa, emit_binop(6, t, emit_const(2)))); } s=0; }
          else { asa=emit_load(emit_binop(4, asa, emit_binop(6, t, emit_const(2)))); } }
        if (src[cp]==40) { return emit_icall(asa); }                            /* R153: tbl[i](args) -- indirect call through a fn-pointer array element (EXPRESSION context) */
        if (s==0 && along) { is64[asa]=1; }                                     /* R160: a genuine LONG array/deref element (unsigned long a[i] / *p) is is64 -- was NOT set, so a consuming `unsigned long x=a[i]` emit_widen'd it with convs, truncating the high 32 bits */
        return asa; }
      if (ac) { return emit_getb(asa, t); }                                     /* char: 1-byte element */
      return emit_load(emit_binop(4, asa, emit_binop(6, t, emit_const(2)))); }  /* int element: load(base + i*4) -- stride 4 == sizeof(int) (LP64) */
    if ((src[cp]==43 && src[cp+1]==43) || (src[cp]==45 && src[cp+1]==45)) {     /* x++ / x-- (post): return old, write new */
      if (src[cp]==43) { s=4; } else { s=5; }                                   /* add / sub */
      cp=cp+2; t=1;                                                             /* step: 1 for scalars, elem-size for pointers */
      if (aptr>=1) { if (aptr>=2 || along) { t=8; } else if (ac) { t=1; }        /* R149: ptr-depth>=2 BEFORE char -- `char **e; e++` strides by 8 (pointee is a pointer), not 1 */
        else if (aptr==1 && astruct>=0) { t=ssz[astruct]; }                      /* R45: struct* stride = ssz */
        else { t=4; } }
      if (ag) { b8=emit_gref(aoff,alen); asa=emit_load(b8);                      /* global: old = load */
        aa=emit_binop(s, asa, emit_const(t)); es("  store v"); en(b8); es(" v"); en(aa); es("\n"); return asa; }
      b8=emit_binop(4, asa, czero);                                             /* local: fresh temp = old value */
      aa=emit_binop(s, asa, emit_const(t)); emit_copy(asa, aa); return b8; }     /* x = new; return old */
    if (ag) { if (aa) { return emit_gref(aoff,alen); } if (along) { return emit_load8L(emit_gref(aoff,alen)); } return emit_load(emit_gref(aoff,alen)); }  /* global name (long -> 8-byte load, is64) */
    if (along && aa==0) { is64[asa]=1; }                                        /* R132(part B): a `long` LOCAL's slot holds a clean 64-bit value (assignment uses emit_copy8) -> mark is64 so consumers use the *8 op family */
    if (lv_flt && aa==0) { is64[asa]=1; isflt[asa]=1; }                          /* R160/c99-R3: a `double` LOCAL's slot holds the 8-byte double bit pattern -> is64 + isflt (arithmetic lowers to soft-float) */
    return asa; }                                                              /* local scalar/array name */
  s=0; b8=0; t=0; ac=0;                                                          /* b8 = significant digits; t = leading-zero skip done; R157: ac = HIGH 32 bits */
  if (src[cp]==48 && cp+1<slen && (src[cp+1]==120 || src[cp+1]==88)) {          /* 0x.. hex literal (R55: suffixes, 64-bit magnitude, unsignedness) */
    cp=cp+2; while (cp<slen && src[cp]==48) { cp=cp+1; }                         /* leading zeros */
    while (cp<slen && hexdig(src[cp])>=0) { if (b8==0 && hexdig(src[cp])>=8) { t=1; }
      ac=ac*16 + ((s>>28)&15);                                                   /* R157: accumulate the high 32 bits (carry out of s's top nibble) BEFORE s<<4 */
      s=s*16+hexdig(src[cp]); b8=b8+1; cp=cp+1; }
    if (b8>8) { lit_huge=1; }                                                    /* > 32 bits: any i32-substrate value compares below it */
    else if (b8==8 && t) { cast_un=1; }                                          /* bit31 set: unsigned literal (0x80000000U etc) */
    while (src[cp]==117 || src[cp]==85 || src[cp]==108 || src[cp]==76) { if (src[cp]==117 || src[cp]==85) { cast_un=1; } cp=cp+1; }  /* u/U/l/L suffixes */
    if (b8>8) { lit_lo=s; lit_hi=ac; return emit_lit64(); }                       /* R157: 64-bit hex literal -> genuine 64-bit const (was: emit_const(low32) -> truncated to 0x...) */
    if (s<0) { t=emit_binop(6, emit_const(1), emit_const(31));                    /* R55: bit31-set const = (1<<31)|low31 (en() cannot print INT_MIN) */
      if ((s & 2147483647) != 0) { t=emit_binop(16, t, emit_const(s & 2147483647)); }
      return t; }
    return emit_const(s); }
  b8=read_dec_hilo(); s=lit_lo;                                                  /* R132: 16-bit-limb read -> lit_lo:lit_hi (s = low 32 bits) */
  if (src[cp]==46 || src[cp]==101 || src[cp]==69) { return emit_float_lit(s); }  /* R160/c99-R3: `.`/`e`/`E` after digits -> a FLOAT literal (0.0, 1.5, 6.022e23) */
  if (b8>10) { lit_huge=1; }                                                     /* decimal > 10 digits: beyond u32 */
  while (src[cp]==117 || src[cp]==85 || src[cp]==108 || src[cp]==76) { if (src[cp]==117 || src[cp]==85) { cast_un=1; } cp=cp+1; }
  if (lit_hi != 0) { return emit_lit64(); }                                      /* R132: value >= 2^32 -> genuine 64-bit const */
  if (s<0) { cast_un=1; }                                                        /* R144: a decimal in [2^31,2^32) is a POSITIVE long (bit31 set) -- mark unsigned so a widen to long ZERO-extends (not sext, which would make it negative) */
  return emit_const(s); }
int parse_expr(int min_prec) { int left; int prec; int oid; int ol; int right; int res; int c; int tn; int save; int alo; int aln; int sc; int base; int stx; int w; int rc; int pw;
  if (min_prec==0) {                                                            /* assignment expr: IDENT = rhs (right-assoc, lowest prec) */
    save=cp; skipws();
    if (src[cp]==42 && src[cp+1]!=61 && src[cp+1]!=42) {                        /* *IDENT = rhs : deref-store expr (e.g. switch(*t = *p)) */
      cp=cp+1; skipws();
      if (isalnum_(src[cp]) && (src[cp]<48 || src[cp]>57)) {
        read_ident(); lvref(); res=lv_glob; c=lv_sa; alo=lv_noff; aln=lv_nlen; w=4;
        if (lv_char) { w=1; } else if (lv_ptr>=2 || lv_long) { w=8; }
        skipws();
        if (src[cp]==61 && src[cp+1]!=61) {                                     /* it IS *IDENT = rhs */
          cp=cp+1; right=parse_expr(0);
          if (res) { base=emit_load(emit_gref(alo,aln)); } else { base=c; }     /* p's value = the address */
          if (w==1) { es("  storeb v"); en(base); es(" v"); en(right); es("\n"); } else if (w==2) { es("  store2 v"); en(base); es(" v"); en(right); es("\n"); }
          else if (w==8) { if (ishi[right]) { tn=right; } else { tn=emit_conv(right); } es("  store8 v"); en(base); es(" v"); en(tn); es("\n"); }  /* R91: rhs already i64 (add8/shl8/...) -> no conv (would truncate) */
          else { es("  store v"); en(base); es(" v"); en(right); es("\n"); }
          return right; } }                                                    /* value of the assignment is the assigned value */
      cp=save; skipws(); }                                                     /* not *ident= -> restore, normal parse */
    if (isalnum_(src[cp]) && (src[cp]<48 || src[cp]>57)) {
      read_ident(); alo=id_off; aln=id_len; skipws();
      if (src[cp]==61 && src[cp+1]!=61) {                                       /* '=' but not '==' */
        lvref(); res=lv_glob; c=lv_sa; w=lv_long; stx=lv_flt;                   /* capture lvalue + long-ness + floatness (R160/c99-R3) BEFORE rhs (rhs clobbers lv_*) */
        cp=cp+1; right=parse_expr(0);                                           /* consume '='; rhs */
        if (stx && isflt[right]==0) { right=emit_int_to_flt(right); }           /* R160/c99-R3: `d = <int>` -> int->float */
        else if (stx==0 && isflt[right]) { right=emit_flt_to_int(right); }       /* R160/c99-R3: `intvar = <float>` -> float->int */
        if (res) { tn=emit_gref(alo,aln);                                       /* gref line BEFORE the store line */
          if (w || stx) { rc=emit_widen(right, cast_un); es("  store8 v"); en(tn); es(" v"); en(rc); es("\n"); }  /* R132(part B): long/double GLOBAL -> 8-byte store */
          else { es("  store v"); en(tn); es(" v"); en(right); es("\n"); } }
        else { if (w || stx) { emit_copy8(c, right, cast_un); if (stx) { isflt[c]=1; } } else { emit_copy(c, right); } }  /* R132(part B): long/double LOCAL -> 64-bit copy */
        return right; }                                                        /* value of an assignment is the assigned value */
      if (src[cp]==45 && src[cp+1]==62) {                                       /* IDENT->f...->g = rhs : arrow-field assignment-expr */
        sc=cp;                                                                  /* scan-ahead: skip the arrow chain, is there a trailing single '='? */
        while (src[sc]==45 && src[sc+1]==62) { sc=sc+2;
          while (sc<slen && (src[sc]==32 || src[sc]==9 || src[sc]==10 || src[sc]==13)) { sc=sc+1; }
          while (sc<slen && isalnum_(src[sc])) { sc=sc+1; }
          while (sc<slen && (src[sc]==32 || src[sc]==9 || src[sc]==10 || src[sc]==13)) { sc=sc+1; } }
        if (src[sc]==61 && src[sc+1]!=61) {                                     /* it IS an arrow-field assignment */
          lvref(); if (lv_glob) { base=emit_load(emit_gref(alo,aln)); } else { base=lv_sa; }  /* IDENT's pointer value */
          stx=lv_struct; w=8;
          while (src[cp]==45 && src[cp+1]==62) {
            cp=cp+2; read_ident(); find_field(stx, id_off, id_len);
            base=emit_binop(4, base, emit_const(ffoff)); stx=ffstruct;
            if (src[cp]==45 && src[cp+1]==62) { base=emit_load8(base); } else { w=ffw; skipws(); } }  /* last field: width */
          cp=cp+1; right=parse_expr(0);                                         /* '='; rhs */
          if (w==1) { es("  storeb v"); en(base); es(" v"); en(right); es("\n"); } else if (w==2) { es("  store2 v"); en(base); es(" v"); en(right); es("\n"); }
          else if (w==8) { if (ishi[right]) { c=right; } else { c=emit_conv(right); } es("  store8 v"); en(base); es(" v"); en(c); es("\n"); }  /* R91: rhs already i64 -> no conv (would truncate) */
          else { es("  store v"); en(base); es(" v"); en(right); es("\n"); }
          return right; } }                                                    /* assignment yields the assigned value */
      if (src[cp]==46) {                                                        /* R147: IDENT.f...= rhs -- DOT-field assignment-EXPR (was unhandled: only IDENT= and IDENT->f= existed, so `(o.reg = f()) >= 0` fell to a normal parse that READ o.reg instead of storing) */
        sc=cp;                                                                  /* scan-ahead: skip the . / -> chain, is there a trailing single '='? */
        while (src[sc]==46 || (src[sc]==45 && src[sc+1]==62)) {
          if (src[sc]==46) { sc=sc+1; } else { sc=sc+2; }
          while (sc<slen && (src[sc]==32 || src[sc]==9 || src[sc]==10 || src[sc]==13)) { sc=sc+1; }
          while (sc<slen && isalnum_(src[sc])) { sc=sc+1; }
          while (sc<slen && (src[sc]==32 || src[sc]==9 || src[sc]==10 || src[sc]==13)) { sc=sc+1; } }
        if (src[sc]==61 && src[sc+1]!=61) {                                     /* it IS a dot-field assignment */
          cp=save; base=lval_addr(); w=la_w; skipws(); cp=cp+1; right=parse_expr(0);  /* &field ; '=' ; rhs */
          if (w==1) { es("  storeb v"); en(base); es(" v"); en(right); es("\n"); } else if (w==2) { es("  store2 v"); en(base); es(" v"); en(right); es("\n"); }
          else if (w==8) { if (ishi[right]) { c=right; } else { c=emit_conv(right); } es("  store8 v"); en(base); es(" v"); en(c); es("\n"); }
          else { es("  store v"); en(base); es(" v"); en(right); es("\n"); }
          return right; } }                                                    /* assignment yields the assigned value */
      if (src[cp]==91) {                                                        /* R154: IDENT[i] = rhs -- SUBSCRIPT assignment-EXPRESSION. Was unhandled (only *IDENT=, IDENT=, IDENT->f=, IDENT.f= existed): `(a[i]=v)` fell to parse_atom which READ a[i] and DROPPED `=v`, so `while((buf[i]=v))` looped once and `return (a[i]=5)` returned garbage. Mirrors the statement store (ccc.c ~1671) but RETURNS the assigned value. */
        sc=cp; stx=0;                                                          /* scan-ahead over ONE balanced [...] to a trailing single '=' */
        while (sc<slen) { if (src[sc]==91) { stx=stx+1; } else if (src[sc]==93) { stx=stx-1; if (stx==0) { sc=sc+1; } } if (stx==0) { break; } sc=sc+1; }
        while (sc<slen && (src[sc]==32 || src[sc]==9 || src[sc]==10 || src[sc]==13)) { sc=sc+1; }
        if (src[sc]==61 && src[sc+1]!=61) {                                     /* it IS a subscript assignment (single [..], not [i][j] or ==) */
          lvref();                                                             /* id_off/id_len still = the ident from read_ident */
          if (lv_glob) { if (lv_arr) { base=emit_gref(lv_noff,lv_nlen); } else { base=emit_load(emit_gref(lv_noff,lv_nlen)); } } else { base=lv_sa; }
          w=4; if (lv_ptr>=2 || lv_long || (lv_arr && lv_ptr>=1)) { w=8; } else if (lv_char) { w=1; }  /* element width (matches the R47 store path) */
          cp=cp+1; rc=parse_expr(0); skipws(); cp=cp+1;                        /* '[' idx ']' */
          cp=cp+1; right=parse_expr(0);                                        /* '=' rhs */
          if (w==8) { tn=emit_binop(4, base, emit_binop(6, rc, emit_const(3))); right=emit_conv(right); es("  store8 v"); en(tn); es(" v"); en(right); es("\n"); }
          else if (w==1) { es("  putb v"); en(base); es(" v"); en(rc); es(" v"); en(right); es("\n"); }
          else { tn=emit_binop(4, base, emit_binop(6, rc, emit_const(2))); es("  store v"); en(tn); es(" v"); en(right); es("\n"); }
          return right; } }                                                    /* the subscript assignment yields the assigned value */
      cp=save; } }                                                             /* not an assignment -> normal parse */
  cast_ps=0; cast_un=0; lit_huge=0; left=parse_atom(); c=cast_ps; w=cast_un; sc=lit_huge; pw=0; cast_ps=0; cast_un=0; lit_huge=0; prec=binop_at();  /* R45/R50/R55/R91: capture cast pointee-size + unsignedness + literal hugeness + operand width */
  while (prec>0 && prec>=min_prec) { oid=op_id; ol=oplen; cp=cp+ol;
    if (oid==17) {                                                             /* a && b -- short-circuit */
      res=emit_const(0); emit_brz(left);
      right=parse_expr(prec+1); emit_copy(res, emit_bool(right)); es("  pool\n"); left=res; }  /* R160: 64-bit-aware right coercion (1 && v with a high-only v) */
    else if (oid==18) {                                                        /* a || b -- short-circuit */
      res=emit_const(1); c=emit_notbool(left); emit_brz(c);                    /* R160: 64-bit-aware left short-circuit test (v || 0) */
      right=parse_expr(prec+1); emit_copy(res, emit_bool(right)); es("  pool\n"); left=res; }
    else { right=parse_expr(prec+1); rc=cast_ps;                                 /* R76: capture RHS pointee width (re-exported by parse_expr) */
      if (oid==5 && c>1 && rc>=1) {                                             /* R76: ptr - ptr -> (a - b)/sizeof(elem) (both operands pointers) */
        left=emit_binop(2, emit_binop(5, left, right), emit_const(c)); cast_un=0; }
      else {
      if (c>1 && (oid==4 || oid==5)) { right=emit_binop(1, right, emit_const(c)); }  /* R45: ((T*)p) +/- n -> n*sizeof(T) */
      if (((lit_huge && is64[left]==0) || (sc && is64[right]==0)) && oid>=8 && oid<=13) {  /* R55: compare vs a >32-bit literal folds (every 32-bit substrate value is below it). R160: only when the OTHER operand is a plain 32-bit substrate -- a genuine 64-bit (is64) operand needs a REAL compare (below). */
        if (lit_huge) { if (oid==8 || oid==9 || oid==13) { left=emit_const(1); } else { left=emit_const(0); } }  /* x OP HUGE: lt/le/ne true */
        else { if (oid==10 || oid==11 || oid==13) { left=emit_const(1); } else { left=emit_const(0); } }         /* HUGE OP x: gt/ge/ne true */
        lit_huge=0; cast_un=0; }
      else {
      if ((w || cast_un) && oid>=8 && oid<=11) { oid=oid+11; }                   /* R50: (unsigned) cast on either side -> ltu/leu/gtu/geu */
      if (oid==7 && w) { oid=23; }                                              /* R64: unsigned >> -> shu (LOGICAL shift; else `unsigned c >> 8` loops in tcc's o()) */
      cast_un=0;
      pw=0;                                                                      /* R91 (NARROW): promote to *8 ONLY where genuine 64-bit is needed in the 32-bit value model: */
      if (oid==6 || oid==7 || oid==23) {                                         /* shift: SEED on a constant count >=32 (the sole op moving bits into the high half; r_info sym<<32 / >>32) */
        if (right==glastconst_v && glastconst_n>=32) { pw=1; }
        if (is64[left]) { pw=1; } }                                             /* R132(part B): shifting a genuine 64-bit value (long local / >=2^32 literal) -> 64-bit shift */
      else if (oid==4 || oid==5 || oid==14 || oid==15 || oid==16) {              /* add/sub/and/xor/or: PROPAGATE only from a genuine-high operand (shl8/*8 chain), NOT from a plain long-field read */
        if (ishi[left] || ishi[right]) { pw=1; }
        if (is64[left] || is64[right]) { pw=1; } }                              /* R132(part B): any genuine 64-bit operand -> 64-bit ALU */
      else if (oid==1) {                                                        /* R133(part C): multiply of a genuine 64-bit operand -> mul8 (imul r64) */
        if (is64[left] || is64[right]) { pw=1; } }
      else if (oid==2 || oid==3) {                                             /* R138: div/mod of a genuine 64-bit operand -> div8/mod8 (idiv r64). NARROW: only when an operand is truly is64 (long local / >=2^32 literal / long-returning call); 32-bit int div stays 32-bit */
        if (is64[left] || is64[right]) { pw=1; } }
      else if (oid>=19 && oid<=22) {                                            /* R135: UNSIGNED compare of a genuine 64-bit operand -> the *8 compare. NARROW to unsigned only: tcc's codegen compares is64 values (offsets, sv->c.i) 32-bit ON PURPOSE (`c.i == (int32_t)c.i` fits-checks, signed <), and promoting those breaks the asm operand path (the R91 lesson). The 64-bit need is the parse_number classification `n >= 0x80000000U` (unsigned). */
        if (is64[left] || is64[right]) { pw=1; } }
      else if (oid==12 || oid==13) {                                            /* R160: EQUALITY ==/!= of a genuine 64-bit operand -> eq8/ne8 (was 32-bit: compared low word only -> `a==b` differing in high, `x!=0` on a high-only value, `x==HUGE` all wrong). SAFE for tcc's <4GB is64 values: high bits are 0 both sides, so eq8==eq there; only genuine-64-bit results change. (Also fixes `c.i==(int32_t)c.i` fits-checks -- eq8 IS the fit test.) */
        if (is64[left] || is64[right]) { pw=1; } }
      else if (oid>=8 && oid<=11) {                                             /* R162: SIGNED relational < <= > >= of a genuine 64-bit operand -> lt8/le8/gt8/ge8. Was 32-bit: compared low word only -> `long v>2147483647L` false (v low=0), `v<HUGE` right only by high-word coincidence. SAFE for tcc's <4GB is64 values: high bits 0 both sides, so *8 == the 32-bit compare there; only genuine-64-bit results change. Mirrors the R160 equality promotion. */
        if (is64[left] || is64[right]) { pw=1; } }
      if (isflt[left] || isflt[right]) {                                        /* R160/c99-R3: a FLOATING operand -> lower to a soft-float call */
        if (oid==1 || oid==2 || oid==4 || oid==5) { left=emit_fbinop(oid, left, right); }  /* * / + - */
        else if (oid>=8 && oid<=13) { left=emit_fcmp(oid, left, right); }        /* < <= > >= == != */
        else { left=emit_binop(oid, left, right); } }
      else if (pw) { left=emit_binop8(oid, left, right); }
      else { left=emit_binop(oid, left, right); } } } }
    prec=binop_at(); if (!((oid==4 || oid==5) && c>=1)) { c=0; } w=0; sc=0; lit_huge=0; }  /* R73: keep cast_ps across consecutive ptr +/- (ptr+a+b: each add scales). R160/c99-R2: c>=1 (not >1) -- a CHAR pointer (c==1) also keeps its pointee width so an enclosing `*(p - 1)` derefs 1-byte (scaling at :950 is still guarded by c>1, so no spurious char scaling). */
  if (min_prec==0) { skipws();                                                  /* ternary cond ? a : b (lowest prec, right-assoc) */
    if (cp<slen && src[cp]==63) { cp=cp+1;
      res=emit_const(0); tn=emit_notbool(left);                                 /* result slot; tn = !cond (R160: 64-bit-aware) */
      emit_brz(left); right=parse_expr(0);                                      /* cond true: res = a */
      if (is64[right]) { emit_copy8(res, right, 1); } else { emit_copy(res, right); }  /* R160: preserve a 64-bit branch (was emit_copy -> 32-bit -> high bits lost) */
      es("  pool\n");
      skipws(); cp=cp+1;                                                        /* ':' */
      emit_brz(tn); rc=parse_expr(0);                                           /* cond false: res = b */
      if (is64[rc] || is64[res]) { emit_copy8(res, rc, 1); } else { emit_copy(res, rc); }  /* R160: if either arm is 64-bit, both write 64-bit (consistent slot) */
      es("  pool\n");
      left=res; c=0; } }
  cast_ps=c;                                                                    /* R45: re-export an unconsumed cast pointee-size (a paren level up may own the +/-, e.g. ((T*)p) - 1) */
  cast_un=w;                                                                    /* R50: re-export unsignedness ((unsigned)v as a paren/operand level up) */
  lit_huge=sc;                                                                  /* R55: re-export literal hugeness (the compare may live a level up) */
  return left; }

int parse_stmt();
int parse_block() { int sv; skipws(); cp=cp+1;                                  /* '{' */
  sv=nvar;                                                                       /* R100: C block scope -- pop names declared in this block on '}' so a nested `int v` doesn't leak out and shadow an outer v for the rest of the function (var_slot holds a monotonic rzir id, so popping nvar never reuses a live slot) */
  skipws(); while (cp<slen && src[cp]!=125) { parse_stmt(); skipws(); }
  cp=cp+1; nvar=sv; return 0; }                                                 /* '}' -- restore scope */
int parse_body() { skipws(); if (src[cp]==123) { parse_block(); } else { parse_stmt(); } return 0; }
int parse_comma() { int v; v=parse_expr(0); skipws();                          /* comma operator: eval each (side effects); value is the LAST */
  while (src[cp]==44) { cp=cp+1; v=parse_expr(0); skipws(); }
  return v; }
int parse_if() { int vC; int eid;                                              /* if (cond) body [else body] */
  cp=cp+2; skipws(); cp=cp+1;                                                   /* 'if' '(' */
  vC=parse_comma(); skipws(); cp=cp+1;                                          /* ')' */
  emit_brz(vC); parse_body();                               /* if !cond -> skip body (to matching pool) */
  skipws();
  if (src[cp]==101 && src[cp+1]==108 && src[cp+2]==115 && src[cp+3]==101 && !isalnum_(src[cp+4])) {  /* else */
    cp=cp+4; eid=lbl_ctr; lbl_ctr=lbl_ctr+1;                                    /* R104: after the if-body, UNCONDITIONALLY skip the else. A `goto` landing inside the if-body (e.g. tcc's `goto block_after_label`) still hits this skip -- the old `brz (cond==0)` re-test wrongly ran the else because the goto bypassed the condition. */
    es("  goto __Lelse"); en(eid); es("\n"); es("  pool\n");                   /* close the if-body block; brz lands here (cond false) -> falls into the else */
    parse_body();
    es("  label __Lelse"); en(eid); es("\n"); }
  else { es("  pool\n"); }
  return 0; }
int parse_while() { int vC;                                                     /* while (cond) body */
  cp=cp+5; skipws(); cp=cp+1;                                                   /* 'while' '(' */
  es("  loop\n"); bk_kind[bk_sp]=0; bk_sp=bk_sp+1;                              /* push loop break-context */
  vC=parse_comma(); skipws(); cp=cp+1;                                         /* ')' */
  emit_brz(vC); parse_body();
  es("  cont\n"); es("  jmp\n"); es("  endloop\n"); bk_sp=bk_sp-1; return 0; }
int parse_for() { int vC; int incr; int after; int d;                          /* for (init; cond; incr) body */
  cp=cp+3; skipws(); cp=cp+1; skipws();                                        /* 'for' '(' */
  if (src[cp]==59) { cp=cp+1; }                                                 /* init (R48: comma list `A = x, B = 0` -- emit EACH; scan to the top-level ';' first) */
  else { incr=cp; d=0;
    while (cp<slen) { if (src[cp]==40) { d=d+1; } if (src[cp]==41) { d=d-1; } if (src[cp]==59 && d==0) { break; } cp=cp+1; }
    d=cp; cp=incr;
    while (cp<d) { parse_stmt(); skipws(); if (src[cp]==44) { cp=cp+1; skipws(); } }
    if (cp==d) { cp=cp+1; } }                                                   /* last piece may leave the ';' unconsumed */
  skipws(); es("  loop\n"); bk_kind[bk_sp]=0; bk_sp=bk_sp+1;                     /* push loop break-context */
  if (src[cp]==59) { vC=emit_const(1); } else { vC=parse_comma(); }             /* cond ('' -> always true) */
  skipws(); cp=cp+1;                                                            /* ';' */
  emit_brz(vC);
  incr=cp; d=1;                                                                 /* remember incr; skip to matching ')' */
  while (cp<slen && d>0) { if (src[cp]==40) { d=d+1; } if (src[cp]==41) { d=d-1; } if (d>0) { cp=cp+1; } }
  d=cp; cp=cp+1;                                                                /* d = the ')' position; past ')' */
  parse_body(); after=cp;
  es("  cont\n");                                                               /* R45: continue-target BEFORE incr (cf backpatches cnt here; `for(..;..;++p){..continue;..}` must run incr) */
  if (src[incr]!=41) { cp=incr;                                                 /* R47: comma-separated updates `++p, --*n` -- emit EACH */
    while (cp<d) { parse_stmt(); skipws(); if (src[cp]==44) { cp=cp+1; skipws(); } } }
  cp=after;
  es("  jmp\n"); es("  endloop\n"); bk_sp=bk_sp-1; return 0; }
int parse_do() { int vC;                                                        /* do body while (cond) ; */
  cp=cp+2; skipws(); es("  loop\n"); bk_kind[bk_sp]=0; bk_sp=bk_sp+1; parse_body();  /* 'do'; body runs first */
  skipws(); cp=cp+5; skipws(); cp=cp+1;                                         /* 'while' '(' */
  vC=parse_comma(); skipws(); cp=cp+1;                                         /* cond ')' */
  emit_brz(vC);                                             /* exit if cond false */
  es("  cont\n"); es("  jmp\n"); es("  endloop\n"); bk_sp=bk_sp-1; skipws(); cp=cp+1; return 0; }  /* ';' */

int parse_const_val() { int neg; int s; int v; skipws();                        /* compile-time int/char/hex const (case labels, post-cpp) */
  neg=0; if (src[cp]==45) { neg=1; cp=cp+1; skipws(); }                          /* -N */
  if (src[cp]==39) { cp=cp+1; s=src[cp];                                         /* 'c' char literal */
    if (s==92) { cp=cp+1; s=src[cp];
      if (s==110) { s=10; } else if (s==116) { s=9; } else if (s==48) { s=0; } else if (s==114) { s=13; }
      else if (s==97) { s=7; } else if (s==98) { s=8; } else if (s==118) { s=11; } else if (s==102) { s=12; }
      else if (s==101) { s=27; } else if (s==92) { s=92; } else if (s==39) { s=39; } }
    cp=cp+1; cp=cp+1; if (neg) { s=0-s; } return s; }                            /* char + closing ' */
  v=0; ce_hi=0;                                                                   /* R160: default -- fits 32 bits */
  if (src[cp]==48 && (src[cp+1]==120 || src[cp+1]==88)) {                         /* 0x hex */
    cp=cp+2; while (hexdig(src[cp])>=0) { ce_hi=(ce_hi*16)+((v>>28)&15); v=(v*16)+hexdig(src[cp]); cp=cp+1; } }  /* R160: track high word too (v keeps low 32; (v>>28)&15 is the nibble carried out of bit31) */
  else { read_dec_hilo(); v=lit_lo; ce_hi=lit_hi; }                              /* R160: decimal via 16-bit-limb reader -> lit_lo:lit_hi (was `v*10+d` truncating a >=2^32 literal) */
  while (src[cp]==117 || src[cp]==85 || src[cp]==108 || src[cp]==76) { cp=cp+1; }  /* R55: u/U/l/L suffixes */
  if (neg) { v=0-v; ce_hi=0; } return v; }                                        /* R160: negative huge literals in const-exprs are unsupported -- drop hi (rare; scalar path handles -N) */
int emit_int(int v) { int r; if (v>=0) { return emit_const(v); } r=emit_binop(5, czero, emit_const(0-v)); isneg[r]=1; return r; }  /* signed const (cf has no neg const). R161/tcc-self-R3: tag isneg so a 64-bit compare sign-extends (the R4 and8 mask ignores isneg -- bitwise keeps zext) */
int emit_u32(int v) { int t;                                                    /* R132: a 32-bit const honoring bit31 (en() prints signed) -- (1<<31)|low31 */
  if (v>=0) { return emit_const(v); }
  t=emit_binop(6, emit_const(1), emit_const(31));
  if ((v & 2147483647) != 0) { t=emit_binop(16, t, emit_const(v & 2147483647)); }
  return t; }
int emit_lit64() { int lo; int hi; int sh;                                      /* R132: emit lit_hi:lit_lo as a genuine 64-bit const -- or8(shl8(conv(hi),32), conv(lo)); result is64+ishi (cf's REX.W op family, no new cf ops) */
  lo=emit_conv(emit_u32(lit_lo));
  hi=emit_conv(emit_u32(lit_hi));
  sh=emit_binop8(6, hi, emit_const(32));
  return emit_binop8(16, sh, lo); }
int read_dec_hilo() { int l0; int l1; int l2; int l3; int t; int n; int d;      /* R132: read decimal digits at cp into lit_lo:lit_hi via 4x16-bit limbs (each limb*10+carry fits a 32-bit int); returns digit count */
  l0=0; l1=0; l2=0; l3=0; n=0;
  while (cp<slen && src[cp]>=48 && src[cp]<=57) {
    d=src[cp]-48;
    t=l0*10+d; l0=t & 65535; d=t/65536;
    t=l1*10+d; l1=t & 65535; d=t/65536;
    t=l2*10+d; l2=t & 65535; d=t/65536;
    t=l3*10+d; l3=t & 65535;
    n=n+1; cp=cp+1; }
  lit_lo=(l1*65536)+l0; lit_hi=(l3*65536)+l2;
  return n; }
int const_expr();   /* forward */
int cast_paren_at(int p);   /* forward (R45) */
int scalar_cast_at(int p) { int save; int r;                                    /* R119: `( TYPE )` scalar cast (int/long/unsigned/uint64_t...) -> pos past ')', or 0. cast_paren_at only catches POINTER casts (needs '*'); const_expr treated `(unsigned long)166` as `(expr)`=0 and dropped the operand -> const inits with `(uint64_t)..` (the x86 opcode table) evaluated to 0. */
  int s0; int s1; int s2; int s3; int s4; int s5; int s6; int s7; int s8; int s9; int s10; int s11; int s12;
  r=0;
  if (src[p]!=40) { return 0; }
  s0=dstk; s1=dlong; s2=dextern; s3=dtarr; s4=duni; s5=dunsig; s6=dshort; s7=dchar; s8=dptr; s9=dstruct; s10=dstruct_off; s11=dstruct_len; s12=nstar;  /* R139: decl_type() below mutates the decl-context globals; a cast must be side-effect-free so an enclosing declarator (e.g. `T arr[]={ {..,(uint64_t)x,..} }`) keeps its dstruct/width -- else t loses its struct binding (stride/field-offset corrupt, the x86 op_type[3] table). */
  save=cp; cp=p+1; skipws();
  if (decl_type()) { skipptr(); skipws(); if (src[cp]==41) { r=cp+1; } }
  cp=save;
  dstk=s0; dlong=s1; dextern=s2; dtarr=s3; duni=s4; dunsig=s5; dshort=s6; dchar=s7; dptr=s8; dstruct=s9; dstruct_off=s10; dstruct_len=s11; nstar=s12;  /* R139: restore decl-context */
  return r; }
int const_term() { int v; skipws();                                             /* one term: (expr), enum-const, or int/hex/char literal */
  if (src[cp]==126) { cp=cp+1; return ~const_term(); }                           /* R119: ~ (bitwise not) */
  if (src[cp]==33 && src[cp+1]!=61) { cp=cp+1; v=const_term(); if (v==0) { return 1; } return 0; }  /* R119: ! (logical not) */
  if (src[cp]==45 && src[cp+1]!=45) { cp=cp+1; return 0-const_term(); }          /* R119: unary - */
  if (src[cp]==40) { v=cast_paren_at(cp); if (v) { cp=v; return const_term(); }  /* R45: (T*) cast prefix (e.g. ((void*)0)) -- skip, take the operand */
    v=scalar_cast_at(cp); if (v) { cp=v; return const_term(); }                  /* R119: (int)/(long)/(uint64_t)... scalar cast -- skip, take the operand */
    cp=cp+1; v=const_expr(); skipws(); cp=cp+1; return v; }                      /* ( expr ) */
  if (isalnum_(src[cp]) && (src[cp]<48 || src[cp]>57)) { read_ident(); return find_enum(id_off, id_len); }  /* enum const (absent -> 0) */
  return parse_const_val(); }
int cast_paren_at(int p) { int seen;                                            /* R45: is (IDENT... *...) at p a pointer-cast? (idents/ws/* only, >=1 '*') */
  if (src[p]!=40) { return 0; } p=p+1; seen=0;
  while (p<slen && src[p]!=41) {
    if (src[p]==42) { seen=1; p=p+1; }
    else if (src[p]==32 || src[p]==10 || src[p]==9 || src[p]==13) { p=p+1; }
    else if (isalnum_(src[p]) && (src[p]<48 || src[p]>57)) { p=p+1; }
    else { return 0; } }
  if (p>=slen) { return 0; } if (seen==0) { return 0; } return p+1; }           /* position past ')' */
int const_expr() { int v; int r; int c; int c1;                                 /* compile-time int expr (enum inits, case labels); left-to-right, tcc's are simple */
  v=const_term();
  while (1) { skipws(); c=src[cp]; c1=0; if (cp+1<slen) { c1=src[cp+1]; }
    if (c==43 || (c==45 && c1!=45) || c==42 || c==47 || c==37 || c==60 || c==62 || (c==124 && c1!=124) || (c==38 && c1!=38) || c==94 || c==33 || c==61 || c==63) { ce_hi=0; }  /* R160: any const-expr operator invalidates the captured 64-bit hi (const arith is 32-bit) -> fall back to a 32-bit init value */
    if (c==43) { cp=cp+1; v=v+const_term(); }                                    /* + */
    else if (c==45 && c1!=45) { cp=cp+1; v=v-const_term(); }                     /* - */
    else if (c==42) { cp=cp+1; v=v*const_term(); }                              /* * */
    else if (c==47) { cp=cp+1; r=const_term(); if (r!=0) { v=v/r; } }            /* R119: / */
    else if (c==37) { cp=cp+1; r=const_term(); if (r!=0) { v=v%r; } }            /* R119: % */
    else if (c==60 && c1==60) { cp=cp+2; r=const_term(); v=v<<r; }              /* << */
    else if (c==62 && c1==62) { cp=cp+2; r=const_term(); v=v>>r; }              /* R119: >> */
    else if (c==124 && c1!=124) { cp=cp+1; v=v|const_term(); }                  /* | */
    else if (c==38 && c1!=38) { cp=cp+1; v=v&const_term(); }                    /* & */
    else if (c==94) { cp=cp+1; v=v^const_term(); }                              /* R119: ^ */
    else if (c==61 && c1==61) { cp=cp+2; r=const_term(); if (v==r) { v=1; } else { v=0; } }  /* R119: == */
    else if (c==33 && c1==61) { cp=cp+2; r=const_term(); if (v!=r) { v=1; } else { v=0; } }  /* R119: != */
    else if (c==60 && c1==61) { cp=cp+2; r=const_term(); if (v<=r) { v=1; } else { v=0; } }  /* R119: <= */
    else if (c==62 && c1==61) { cp=cp+2; r=const_term(); if (v>=r) { v=1; } else { v=0; } }  /* R119: >= */
    else if (c==60) { cp=cp+1; r=const_term(); if (v<r) { v=1; } else { v=0; } }              /* R119: < */
    else if (c==62) { cp=cp+1; r=const_term(); if (v>r) { v=1; } else { v=0; } }              /* R119: > */
    else if (c==63) { cp=cp+1; r=const_expr(); skipws(); cp=cp+1; c=const_expr(); if (v!=0) { return r; } return c; }  /* R119: ?: (cond in v) */
    else { return v; } }
  return v; }
/* parse a global initializer at cp (scalar `expr` or `{ v0, v1, ... }`); record const values in the
   pending-init table for `goff/glen` (element width w). Returns the element count. String-literal
   elements store 0 (char*[] string tables are a later rung). */
int parse_init(int goff, int glen, int w) { int vs; int c;
  vs=nvals; skipws();
  if (src[cp]==123) { cp=cp+1; skipws();                                         /* { ... } */
    while (cp<slen && src[cp]!=125) { skipws(); if (src[cp]==125) { c=0; }
      else if (src[cp]==34) { c=34; cp=cp+1; while (cp<slen && src[cp]!=34) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; pin_vals[nvals]=0; pin_hi[nvals]=0; nvals=nvals+1; }
      else { pin_vals[nvals]=const_expr(); pin_hi[nvals]=ce_hi; nvals=nvals+1; }  /* R160: keep the 64-bit hi word */
      skipws(); if (src[cp]==44) { cp=cp+1; } skipws(); }
    if (src[cp]==125) { cp=cp+1; } }
  else if (src[cp]==34) {                                                        /* char *X = "..." : store the string's ADDRESS (built at emit via parse_string); w=0 -> hetero pin */
    pin_boff[nvals]=0; pin_vw[nvals]=8; pin_kd[nvals]=1; pin_vals[nvals]=cp; nvals=nvals+1; w=0;
    while (src[cp]==34) { cp=cp+1; while (src[cp]!=34) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; skipws(); } }
  else { pin_vals[nvals]=const_expr(); pin_hi[nvals]=ce_hi; nvals=nvals+1; }      /* scalar */
  pin_goff[npin]=goff; pin_glen[npin]=glen; pin_w[npin]=w; pin_vs[npin]=vs; pin_vc[npin]=nvals-vs; npin=npin+1;
  return nvals-vs; }
/* R118: parse ONE aggregate initializer `{ f0, f1, ... }` at cp (the '{' is NOT yet consumed),
   recording hetero pin entries (pin_w=0) at eboff+fldoff for each field. Handles a scalar field,
   a string field, AND a nested ARRAY-FIELD `{...}` (flattened to per-element pins at fldoff+j*elemw
   -- was DROPPED, e.g. the x86 assembler's `ASMInstr op_type[3]` opcode table -> whole table zeroed). */
int parse_aggr_fields(int sidx, int eboff) { int fi; int k; int j; int ew; int nc; int p;
  fi=sfst[sidx]; k=0; cp=cp+1; skipws();                                         /* consume '{' */
  while (cp<slen && src[cp]!=125) { skipws(); if (src[cp]==125) { break; }
    if (k<sfn[sidx]) {
      if (fld_arr[fi+k] && src[cp]==123) {                                       /* nested array-field { ... } */
        ew=fldw[fi+k]; nc=1; if (ew>0) { nc=fld_bytes[fi+k]/ew; } cp=cp+1; skipws(); j=0;
        while (cp<slen && src[cp]!=125) { skipws(); if (src[cp]==125) { break; }
          if (j<nc) { pin_boff[nvals]=eboff+fldoff[fi+k]+j*ew; pin_vw[nvals]=ew; pin_kd[nvals]=0; pin_vals[nvals]=const_expr(); nvals=nvals+1; }
          else { const_expr(); }
          j=j+1; skipws(); if (src[cp]==44) { cp=cp+1; } skipws(); }
        if (src[cp]==125) { cp=cp+1; } }
      else if (src[cp]==34) {                                                    /* string field -> store its ADDRESS */
        pin_boff[nvals]=eboff+fldoff[fi+k]; pin_vw[nvals]=fldw[fi+k]; pin_kd[nvals]=1; pin_vals[nvals]=cp; nvals=nvals+1;
        cp=cp+1; while (cp<slen && src[cp]!=34) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }
      else {                                                                     /* scalar field */
        pin_boff[nvals]=eboff+fldoff[fi+k]; pin_vw[nvals]=fldw[fi+k]; pin_kd[nvals]=0; pin_vals[nvals]=const_expr(); nvals=nvals+1; }
      k=k+1; }
    else { p=cp; if (src[cp]==34) { cp=cp+1; while (cp<slen && src[cp]!=34) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; } else { const_expr(); } if (cp<=p) { cp=cp+1; } }  /* excess: consume+drop */
    skipws(); if (src[cp]==44) { cp=cp+1; } skipws(); }
  if (src[cp]==125) { cp=cp+1; } return 0; }
/* R45: struct-array global init `T arr[] = { {..}, ... }` -- hetero pending entry (pin_w=0). Returns count. */
int parse_sarr_init(int goff, int glen, int sidx) { int vs; int ne;
  vs=nvals; ne=0; skipws();
  if (src[cp]!=123) { return 0; } cp=cp+1; skipws();                             /* outer { */
  while (cp<slen && src[cp]!=125) { skipws();
    if (src[cp]==123) { parse_aggr_fields(sidx, ne*ssz[sidx]); ne=ne+1; }        /* one element */
    else if (src[cp]==125) { break; }
    else { cp=cp+1; }                                                            /* stray (progress) */
    skipws(); if (src[cp]==44) { cp=cp+1; } skipws(); }
  if (src[cp]==125) { cp=cp+1; }
  pin_goff[npin]=goff; pin_glen[npin]=glen; pin_w[npin]=0; pin_vs[npin]=vs; pin_vc[npin]=nvals-vs; npin=npin+1;
  return ne; }
/* R118: single struct-VALUE global `T s = { f0, f1, ... }` -- one aggregate, base offset 0. */
int parse_struct_val_init(int goff, int glen, int sidx) { int vs;
  vs=nvals; skipws(); if (src[cp]!=123) { return 0; }
  parse_aggr_fields(sidx, 0);
  pin_goff[npin]=goff; pin_glen[npin]=glen; pin_w[npin]=0; pin_vs[npin]=vs; pin_vc[npin]=nvals-vs; npin=npin+1;
  return 1; }
/* char arr[] = "..." ["..."]* : record the (escape-decoded, concatenated) bytes + trailing NUL as w=1
   init values; emit_init lays them down as storeb at gref+offset at the entry. Returns byte count. */
int parse_str_init(int goff, int glen) { int vs; int ch;
  vs=nvals; skipws();
  while (src[cp]==34) {                                                            /* one or more adjacent string literals */
    cp=cp+1;
    while (src[cp]!=34) {
      ch=src[cp];
      if (ch==92) { cp=cp+1; ch=src[cp]; if (ch==110) { ch=10; } else if (ch==116) { ch=9; } else if (ch==48) { ch=0; } else if (ch==114) { ch=13; } else if (ch==97) { ch=7; } else if (ch==98) { ch=8; } else if (ch==118) { ch=11; } else if (ch==102) { ch=12; } else if (ch==101) { ch=27; } }
      pin_vals[nvals]=ch; nvals=nvals+1; cp=cp+1; }
    cp=cp+1; skipws(); }                                                           /* past closing " ; re-check adjacency */
  pin_vals[nvals]=0; nvals=nvals+1;                                                /* implicit trailing NUL */
  pin_goff[npin]=goff; pin_glen[npin]=glen; pin_w[npin]=1; pin_vs[npin]=vs; pin_vc[npin]=nvals-vs; npin=npin+1;
  return nvals-vs; }
/* emit the recorded global-init stores (at the top of main -- the entry runs first) */
int emit_pin_stores(int i) { int j; int gr; int addr; int w; int vi; int vv; int sv;  /* emit the init stores for pin i */
  gr=emit_gref(pin_goff[i], pin_glen[i]); w=pin_w[i]; vi=pin_vs[i];
  j=0; while (j<pin_vc[i]) {
    if (w==0) {                                                               /* R45 hetero entry: per-value byte offset/width/kind */
      addr=emit_binop(4, gr, emit_const(pin_boff[vi+j]));
      if (pin_kd[vi+j]==1) { sv=cp; cp=pin_vals[vi+j]; vv=parse_string(); cp=sv;  /* build the string buffer; store its ADDRESS */
        vv=emit_conv(vv); es("  store8 v"); en(addr); es(" v"); en(vv); es("\n"); }
      else { vv=emit_int(pin_vals[vi+j]);
        if (pin_vw[vi+j]==1) { es("  storeb v"); en(addr); es(" v"); en(vv); es("\n"); }
        else if (pin_vw[vi+j]==2) { es("  store2 v"); en(addr); es(" v"); en(vv); es("\n"); }
        else if (pin_vw[vi+j]==8) { vv=emit_conv(vv); es("  store8 v"); en(addr); es(" v"); en(vv); es("\n"); }
        else { es("  store v"); en(addr); es(" v"); en(vv); es("\n"); } }
      j=j+1; }
    else {
    addr=emit_binop(4, gr, emit_const(j*w));
    if (w==8 && pin_hi[vi+j]!=0) { lit_lo=pin_vals[vi+j]; lit_hi=pin_hi[vi+j]; vv=emit_lit64(); es("  store8 v"); en(addr); es(" v"); en(vv); es("\n"); }  /* R160: 64-bit initializer -> genuine 64-bit value (was store8 of a 32-bit-truncated const) */
    else {
    vv=emit_int(pin_vals[vi+j]);
    if (w==1) { es("  storeb v"); en(addr); es(" v"); en(vv); es("\n"); } else if (w==2) { es("  store2 v"); en(addr); es(" v"); en(vv); es("\n"); }
    else if (w==8) { vv=emit_conv(vv); es("  store8 v"); en(addr); es(" v"); en(vv); es("\n"); }
    else { es("  store v"); en(addr); es(" v"); en(vv); es("\n"); }
    }
    j=j+1; } }
  return 0; }
int pin_split_ok(int i) { int vi; int j;                                      /* R121: split pin i into its own __ginitN? Only LARGE + STRING-FREE pins -- a string init builds a LOCAL buffer whose ADDRESS is stored; in a separate fn that buffer dies on return (would leave dangling global char* -- e.g. tcc's option table). Inline (in main) keeps the buffer alive. */
  if (pin_vc[i]<500) { return 0; }                                            /* small: inline (keeps main byte-identical for small programs) */
  if (pin_w[i]!=0) { return 1; }                                              /* homogeneous array (no string entries possible) */
  vi=pin_vs[i]; j=0; while (j<pin_vc[i]) { if (pin_kd[vi+j]==1) { return 0; } j=j+1; }  /* hetero with a string -> inline */
  return 1; }
int emit_init() { int i;                                                      /* at main's top: inline each pin, or call __ginitN for large string-free pins */
  i=0; while (i<npin) {
    if (pin_split_ok(i)) { ginit_split=1; es("  v"); en(ncount); es(": i32 = call __ginit"); en(i); es("\n"); ncount=ncount+1; }
    else { emit_pin_stores(i); }
    i=i+1; }
  return 0; }
int emit_ginit_bodies() { int i; int gzi2;                                    /* R121: `fn __ginitN` bodies for the split (large string-free) pins, after main */
  if (ginit_split==0) { return 0; }
  i=0; while (i<npin) {
    if (pin_split_ok(i)) {
      es("fn __ginit"); en(i); es("\n");
      gzi2=0; while (gzi2<40000) { is64[gzi2]=0; ishi[gzi2]=0; isflt[gzi2]=0; isneg[gzi2]=0; gzi2=gzi2+1; }  /* clear per-function value tags */
      czero=0; es("  v0: i32 = const 0\n"); ncount=1;
      emit_pin_stores(i);
      es("  ret v0\n"); es("end\n"); }
    i=i+1; }
  return 0; }
int register_enum() { int val; int pv; cp=cp+1; val=0; skipws();                 /* at '{': add each A[=expr] as name->value */
  while (cp<slen && src[cp]!=125) { pv=cp;
    read_ident(); enm_off[nenm]=id_off; enm_len[nenm]=id_len; skipws();
    if (src[cp]==61) { cp=cp+1; val=const_expr(); skipws(); }                    /* = expr */
    enm_val[nenm]=val; nenm=nenm+1; val=val+1;
    if (src[cp]==44) { cp=cp+1; skipws(); }                                      /* , */
    if (cp<=pv) { cp=cp+1; } }                                                   /* never stall (progress guard) */
  cp=cp+1; return 0; }                                                           /* '}' */
int prescan_enums() {                                                           /* pass: register ALL enum constants (source-wide) */
  cp=0;
  while (cp<slen) {
    if (src[cp]==34) { cp=cp+1; while (cp<slen && src[cp]!=34) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }
    else if (src[cp]==39) { cp=cp+1; while (cp<slen && src[cp]!=39) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }
    else if (src[cp]==47 && src[cp+1]==42) { cp=cp+2; while (cp<slen && !(src[cp]==42 && src[cp+1]==47)) { cp=cp+1; } cp=cp+2; }
    else if (src[cp]==101 && src[cp+1]==110 && src[cp+2]==117 && src[cp+3]==109 && isalnum_(src[cp+4])==0
             && (cp==0 || isalnum_(src[cp-1])==0)) {                            /* 'enum' at a token boundary */
      cp=cp+4; skipws();
      if (isalnum_(src[cp]) && src[cp]!=123) { read_ident(); skipws(); }        /* optional tag name */
      if (src[cp]==123) { register_enum(); } }                                  /* def -> register; bare use -> continue */
    else { cp=cp+1; } }
  return 0; }
/* switch (E) { case Ci: Si  default: Sd } -- single-pass desugar:
   eval E to a live temp; `goto` the dispatch chain (emitted after the bodies); each `case`/`default`
   drops a label inline (fallthrough is natural); `break` inside emits `goto <end>` (the break stack).
   Case values are captured at parse time (parse_const_val) and re-emitted as t==Ci compares at dispatch. */
int parse_switch() { int t; int swid; int base; int hasdef; int deflab; int i; int lid; int vcmp; int save; int depth; int ci; int c0;
  cp=cp+6; skipws(); cp=cp+1;                                                    /* 'switch' '(' */
  t=parse_expr(0); skipws(); cp=cp+1;                                            /* ')' */
  swid=lbl_ctr; lbl_ctr=lbl_ctr+1;                                              /* this switch's dispatch/end id */
  bk_kind[bk_sp]=1; bk_end[bk_sp]=swid; bk_sp=bk_sp+1;                          /* push switch break-context */
  base=sw_valn; hasdef=0; deflab=0;
  skipws(); cp=cp+1; skipws();                                                  /* '{' */
  save=cp;                                                                       /* R145: body start -- pre-scan cases, then re-parse */
  /* R145: DISPATCH-FIRST. The old desugar emitted `goto __Lswd` over the bodies and put the
     compare chain AFTER them; that forward-jump-over-bodies + after-bodies dispatch miscompiled
     at scale in cf (tcc's preprocess_skip: a value mis-dispatched -> #if-skip infinite loop).
     Now: PRE-SCAN the top-level case/default labels (capturing values + label ids), emit the
     dispatch INLINE (all-forward gotos to the case labels below), THEN parse the bodies. */
  depth=0;                                                                       /* pre-scan: skip strings/chars/nested braces; skipws eats block comments */
  while (cp<slen) {
    skipws(); c0=src[cp];
    if (c0==125) { if (depth==0) { i=0; cp=cp; break; } depth=depth-1; cp=cp+1; }  /* '}' */
    else if (c0==123) { depth=depth+1; cp=cp+1; }                                  /* '{' */
    else if (c0==34) { cp=cp+1; while (cp<slen && src[cp]!=34) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }  /* "str" */
    else if (c0==39) { cp=cp+1; while (cp<slen && src[cp]!=39) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }  /* 'c' */
    else if (depth==0 && tok_is("case")) { cp=cp+4; lid=lbl_ctr; lbl_ctr=lbl_ctr+1;
      sw_cval[sw_valn]=const_expr(); sw_clab[sw_valn]=lid; sw_valn=sw_valn+1; skipws(); cp=cp+1; }  /* ':' */
    else if (depth==0 && tok_is("default")) { cp=cp+7; skipws(); cp=cp+1; hasdef=1; deflab=lbl_ctr; lbl_ctr=lbl_ctr+1; }
    else if (isalnum_(c0)) { while (cp<slen && isalnum_(src[cp])) { cp=cp+1; } }   /* consume a whole ident/number token so `case`/`default` are never matched mid-identifier (tok_is checks only the TRAILING boundary) */
    else { cp=cp+1; } }
  i=base;                                                                        /* emit the dispatch (before bodies) */
  while (i<sw_valn) {
    vcmp=emit_binop(12, t, emit_int(sw_cval[i]));                               /* t == Ci */
    emit_brz(vcmp);
    es("  goto __Lswc"); en(sw_clab[i]); es("\n"); es("  pool\n");
    i=i+1; }
  if (hasdef) { es("  goto __Lswc"); en(deflab); es("\n"); }
  else { es("  goto __Lswe"); en(swid); es("\n"); }
  cp=save; ci=base;                                                             /* re-parse the body, emitting the pre-assigned labels + statements */
  skipws(); while (cp<slen && src[cp]!=125) {
    if (tok_is("case")) { cp=cp+4; const_expr(); skipws(); cp=cp+1;            /* consume `case CONST :` (value already captured) */
      es("  label __Lswc"); en(sw_clab[ci]); es("\n"); ci=ci+1; }
    else if (tok_is("default")) { cp=cp+7; skipws(); cp=cp+1;
      es("  label __Lswc"); en(deflab); es("\n"); }
    else if (src[cp]==123) { parse_block(); }
    else { parse_stmt(); }
    skipws(); }
  cp=cp+1;                                                                       /* '}' */
  es("  label __Lswe"); en(swid); es("\n");
  sw_valn=base; bk_sp=bk_sp-1; return 0; }                                       /* pop cases + break-context */
int ca_op; int ca_len;                                                          /* compound-assign: emit_binop id + operator length */
int is_caop() { int a; int b;                                                   /* at cp: compound-assign op (OP=)? set ca_op/ca_len; else 0 */
  a=src[cp]; b=src[cp+1];
  if (a==60 && b==60 && src[cp+2]==61) { ca_op=6; ca_len=3; return 1; }          /* <<= */
  if (a==62 && b==62 && src[cp+2]==61) { ca_op=7; ca_len=3; return 1; }          /* >>= */
  if (b!=61) { return 0; }                                                       /* remaining are single-char OP followed by '=' */
  if (a==43) { ca_op=4; ca_len=2; return 1; }                                    /* += */
  if (a==45) { ca_op=5; ca_len=2; return 1; }                                    /* -= */
  if (a==42) { ca_op=1; ca_len=2; return 1; }                                    /* *= */
  if (a==47) { ca_op=2; ca_len=2; return 1; }                                    /* /= */
  if (a==37) { ca_op=3; ca_len=2; return 1; }                                    /* %= */
  if (a==38) { ca_op=14; ca_len=2; return 1; }                                   /* &= */
  if (a==124) { ca_op=16; ca_len=2; return 1; }                                  /* |= */
  if (a==94) { ca_op=15; ca_len=2; return 1; }                                   /* ^= */
  return 0; }
int parse_stmt() { int vE; int slot; int idx; int base2; int lglob; int lsa; int lchar; int lstruct; int loff; int llen; int st; int larr; int lptr; int llong; int ac; int lflt; int p0;
  skipws(); st=cp;
  if (src[cp]==123) { parse_block(); return 0; }                                /* nested { } block statement */
  if (src[cp]==59) { cp=cp+1; return 0; }                                       /* empty statement ; */
  if ((src[cp]==43 && src[cp+1]==43) || (src[cp]==45 && src[cp+1]==45)) {        /* R45: ++x / --x statement (e.g. a for-update `++p`) -> parse_atom's pre-inc */
    idx=4; if (src[cp]==45) { idx=5; }                                           /* add / sub */
    cp=cp+2; skipws();
    if (src[cp]==40) {                                                           /* R130: ++(lvalue) / --(lvalue) -- parenthesized pre-inc/dec (e.g. tcc's inp() `ch = *(++(file->buf_ptr))`); as a STATEMENT it's identical to a post-inc, so route the (possibly ->/. chained) lvalue through lval_addr like R113. Was: read_ident on `(` -> garbage lvalue -> wild store -> SEGV */
      cp=cp+1; skipws(); slot=cp; read_ident(); skipws();
      cp=slot; base2=lval_addr(); skipws(); st=la_w;
      if (st==1) { vE=emit_binop(idx, emit_loadb(base2), emit_const(1)); es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); }
      else if (st==2) { vE=emit_binop(idx, emit_load2(base2), emit_const(1)); es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
      else if (st==8) { vE=emit_binop(idx, emit_load8(base2), emit_const(1)); vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
      else { vE=emit_binop(idx, emit_load(base2), emit_const(1)); es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
      skipws(); if (src[cp]==41) { cp=cp+1; } skipws(); if (src[cp]==59) { cp=cp+1; } return 0; }
    if (src[cp]==42) {                                                           /* R47: ++*p / --*n -- pre-inc/dec THROUGH a pointer */
      cp=cp+1; skipws(); read_ident(); lvref();
      if (lv_glob) { base2=emit_load(emit_gref(lv_noff, lv_nlen)); } else { base2=lv_sa; }  /* the pointer VALUE */
      vE=4; if (lv_char) { vE=1; } else if (lv_ptr>=2 || lv_long) { vE=8; }      /* pointee width */
      if (vE==1) { slot=emit_loadb(base2); slot=emit_binop(idx, slot, emit_const(1)); es("  storeb v"); en(base2); es(" v"); en(slot); es("\n"); } else if (vE==2) { slot=emit_loadb(base2); slot=emit_binop(idx, slot, emit_const(1)); es("  store2 v"); en(base2); es(" v"); en(slot); es("\n"); }
      else if (vE==8) { slot=emit_load8(base2); slot=emit_binop(idx, slot, emit_const(1)); slot=emit_conv(slot); es("  store8 v"); en(base2); es(" v"); en(slot); es("\n"); }
      else { slot=emit_load(base2); slot=emit_binop(idx, slot, emit_const(1)); es("  store v"); en(base2); es(" v"); en(slot); es("\n"); }
      skipws(); if (src[cp]==59 || src[cp]==41) { cp=cp+1; } return 0; }
    slot=cp; read_ident(); skipws();
    if (((src[cp]==45 && src[cp+1]==62) || src[cp]==46) && chain_pure()) {       /* R46: ++/-- of a FIELD CHAIN (--tokstr_buf.len) -> typed resolver */
      cp=slot; base2=lval_addr();
      ac=1; if (la_w==8 && la_long==0) { ac=ffpw; }                              /* R142: ++/-- of a POINTER field (8-byte, non-long) strides by the pointee width, not 1 (tcc's `++s1->include_stack_ptr` on a BufferedFile** -> stride 8; long field stays 1) */
      if (la_w==1) { vE=emit_loadb(base2); vE=emit_binop(idx, vE, emit_const(1)); es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); } else if (la_w==2) { vE=emit_loadb(base2); vE=emit_binop(idx, vE, emit_const(1)); es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
      else if (la_w==8) { vE=emit_load8(base2); vE=emit_binop(idx, vE, emit_const(ac)); vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
      else { vE=emit_load(base2); vE=emit_binop(idx, vE, emit_const(1)); es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
      skipws(); if (src[cp]==59 || src[cp]==41) { cp=cp+1; } return 0; }
    cp=st;                                                                       /* simple ident -> parse_atom's pre-inc (stride-aware) */
    parse_expr(0); skipws();
    if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; }                        /* R105: `++s, --len;` -- comma tail is PART of this statement (else it escapes an unbraced if-body guard -- tcc's parse_string `if((is_long=*s=='L'))++s,--len;` dropped a char from every string literal) */
    if (src[cp]==59 || src[cp]==41) { cp=cp+1; } return 0; }
  if (src[cp]==105 && src[cp+1]==102 && (src[cp+2]==40 || src[cp+2]==32)) { parse_if(); return 0; }   /* if */
  if (src[cp]==119 && src[cp+1]==104 && src[cp+2]==105 && src[cp+3]==108 && src[cp+4]==101            /* while */
      && (src[cp+5]==40 || src[cp+5]==32)) { parse_while(); return 0; }
  if (src[cp]==102 && src[cp+1]==111 && src[cp+2]==114 && (src[cp+3]==40 || src[cp+3]==32)) { parse_for(); return 0; }  /* for */
  if (src[cp]==103 && src[cp+1]==111 && src[cp+2]==116 && src[cp+3]==111 && (src[cp+4]==32 || src[cp+4]==9)) {  /* goto NAME ; */
    cp=cp+4; read_ident(); es("  goto "); emit_name(id_off, id_len); es("\n"); skipws(); cp=cp+1; return 0; }
  if (src[cp]==100 && src[cp+1]==111 && (src[cp+2]==32 || src[cp+2]==9 || src[cp+2]==10 || src[cp+2]==123)) { parse_do(); return 0; }  /* do */
  if (src[cp]==115 && src[cp+1]==119 && src[cp+2]==105 && src[cp+3]==116 && src[cp+4]==99 && src[cp+5]==104   /* switch */
      && (src[cp+6]==40 || src[cp+6]==32)) { parse_switch(); return 0; }
  idx=tok_is("break"); if (idx) {                                               /* break ; -> switch: goto end; loop: cf brk */
    if (bk_sp>0 && bk_kind[bk_sp-1]==1) { es("  goto __Lswe"); en(bk_end[bk_sp-1]); es("\n"); }
    else { es("  brk\n"); }
    cp=idx; skipws(); cp=cp+1; return 0; }                                      /* cp=end of "break", then past ';' */
  idx=tok_is("continue"); if (idx) { es("  cnt\n"); cp=idx; skipws(); cp=cp+1; return 0; }  /* continue ; -> cnt; cp=end of "continue", past ';' */
  if (decl_type()) {                                                            /* int/char [*]NAME[[N]] [= expr] [, ...] ; -- base type persists across declarators */
    llong=dlong; lflt=dflt;                                                     /* R160: capture BASE long-ness (+ R160/c99-R3 floatness) -- a declarator's initializer (right_init->parse_expr) can clobber dlong/dflt, so the 2nd+ declarator of `unsigned long a=x, b=y` (or `long double f1, f2`) saw them cleared */
    while (1) {                                                                 /* R22: per-declarator loop (comma-separated) */
    dlong=llong; dflt=lflt;                                                     /* R160: restore base long-ness + floatness for THIS declarator */
    skipws();
    skipptr();                                                                  /* *p (any depth; skipptr resets nstar per declarator) */
    if (src[cp]==40 && src[cp+1]==42) {                                          /* RET (*NAME)(params) -- fn pointer local -> scalar */
      cp=cp+2; read_ident(); slot=id_off; vE=id_len; skipws();
      if (src[cp]==91) {                                                         /* R156: RET (*NAME[N])(params) -- fn-ptr ARRAY local (mirror of the R155 global form; N 8-byte pointer elements, ONE slot each). tbl[i] strides by 8; tbl[i]() routes through the R153 calli path. */
        cp=cp+1; idx=const_expr(); skipws(); cp=cp+1;                            /* [ N ] */
        if (src[cp]==41) { cp=cp+1; } skipws(); if (src[cp]==40) { skipparens(); } skipws();  /* ) (params) */
        dstruct=0-1; nstar=1; dvbytes=idx*8;                                     /* element = 8-byte fn pointer; sizeof = N*8 */
        base2=ncount; ncount=ncount+idx;                                         /* reserve N slots (one per 8-byte pointer, NOT ceil(N/8)) */
        rec_var(slot, vE, 0, 1, emit_ref(base2)); }                             /* var_arr=1, var_ptr=nstar=1 */
      else {
      if (src[cp]==41) { cp=cp+1; } skipws(); if (src[cp]==40) { skipparens(); } skipws();  /* ) (params) */
      dstruct=0-1;
      if (src[cp]==61) { cp=cp+1; right_init(slot, vE); } else { rec_var(slot, vE, 0, 0, emit_const(0)); } } }
    else {
    read_ident(); slot=id_off; vE=id_len;                                       /* stash name off/len */
    skipws();
    if (dtarr>0 && src[cp]!=91) {                                               /* array-typedef local (e.g. va_list ap) -> reserve dtarr slots */
      base2=ncount; ncount=ncount+dtarr; rec_var(slot, vE, 0, 1, emit_ref(base2)); }
    else if (src[cp]==91) {                                                     /* a[N] -- reserve slots, ref base */
      cp=cp+1; skipws(); idx=0; emptyarr=0;
      if (src[cp]==93) { emptyarr=1; idx=128; }                                  /* R125: `T name[]` -- size inferred from initializer; over-reserve 128, fix var_bytes after the init */
      else if (src[cp]>=48 && src[cp]<=57) { while (src[cp]>=48 && src[cp]<=57) { idx=idx*10+(src[cp]-48); cp=cp+1; } }
      else { parse_expr(0); idx=128; }                                          /* R90: VLA a[expr] -- consume the size expr, reserve a fixed 128-elem buffer (gfunc_call's char _onstack[nb_args]) */
      skipws(); cp=cp+1;                                                        /* ] */
      if (dchar && nstar==0 && dptr==0) { dvbytes=idx; idx=(idx+7)/8; }        /* char[N]: N bytes; ceil(N/8) slots. R150: guard nstar/dptr -- `char *a[N]` is a POINTER array (N*8), NOT a byte array; matched dchar before the ptr branch -> 1 slot for a 16-byte object -> a[1] overran the neighbor SSA slot */
      else if (dstruct>=0 && nstar==0 && dptr==0) { dvbytes=idx*ssz[dstruct]; idx=(dvbytes+7)/8; }  /* R124: struct-VALUE[N]: N*ssz bytes, ceil/8 slots -- was reserving only N slots (1/elem) -> a >8B struct array overflowed adjacent locals (asm_instr operands[30] clobbered nb_outputs) */
      else if (dlong || dptr || nstar>=1) { dvbytes=idx*8; }                    /* long/ptr[N]: N*8 bytes */
      else { dvbytes=idx*4; }                                                   /* int[N]: N*4 bytes (LP64) */
      base2=ncount; ncount=ncount+idx;
      rec_var(slot, vE, 0, 1, emit_ref(base2));                                 /* local array (var_char from dchar; var_bytes from dvbytes) */
      skipws();
      if (src[cp]==61) { cp=cp+1; skipws();                                     /* R117: local array initializer -- was DROPPED; the `= {..}` / `= "str"` got mis-parsed as stray statements (values computed, never stored) */
        st=4; if (dchar && nstar==0 && dptr==0) { st=1; } else if (dlong || dptr || nstar>=1) { st=8; }  /* element stride (R150: char *a[]={...} strides by 8, not 1) */
        lsa=emit_ref(base2); lstruct=0;                                         /* buffer base addr; lstruct = element index */
        if (src[cp]==123) { cp=cp+1; skipws();                                  /* { v0, v1, ... } */
          while (src[cp]!=125 && cp<slen) {
            vE=parse_expr(0); larr=emit_binop(4, lsa, emit_const(lstruct*st));
            if (st==1) { es("  storeb v"); en(larr); es(" v"); en(vE); es("\n"); }
            else if (st==8) { vE=emit_conv(vE); es("  store8 v"); en(larr); es(" v"); en(vE); es("\n"); }
            else { es("  store v"); en(larr); es(" v"); en(vE); es("\n"); }
            lstruct=lstruct+1; skipws(); if (src[cp]==44) { cp=cp+1; skipws(); } }
          cp=cp+1;                                                             /* } */
          if (emptyarr && nvar>0) { var_bytes[nvar-1]=lstruct*st; } }          /* R125: `T name[]={..}` -- sizeof = count*stride (was 128 from the VLA fallback) */
        else if (src[cp]==34) { cp=cp+1;                                       /* "string" -> storeb each char + NUL */
          while (src[cp]!=34 && cp<slen) {
            idx=src[cp]; if (idx==92) { cp=cp+1; idx=src[cp]; if (idx==110) { idx=10; } else if (idx==116) { idx=9; } else if (idx==48) { idx=0; } else if (idx==114) { idx=13; } }
            larr=emit_binop(4, lsa, emit_const(lstruct)); vE=emit_const(idx);   /* compute addr + char BEFORE the storeb line (else emit_const splits it) */
            es("  storeb v"); en(larr); es(" v"); en(vE); es("\n");
            lstruct=lstruct+1; cp=cp+1; }
          larr=emit_binop(4, lsa, emit_const(lstruct)); vE=emit_const(0);       /* NUL terminator */
          es("  storeb v"); en(larr); es(" v"); en(vE); es("\n");
          cp=cp+1;                                                             /* closing " */
          if (emptyarr && nvar>0) { var_bytes[nvar-1]=lstruct+1; } } }         /* R125: `char name[]="str"` -- sizeof = len+NUL */
      }
    else if (src[cp]==61) { cp=cp+1; right_init(slot, vE); }
    else { rec_var(slot, vE, 0, 0, emit_const(0));                              /* local scalar */
      if (dstk && dstruct>=0 && nstar==0 && dptr==0) { ncount=ncount+((ssz[dstruct]+7)/8)-1; } } }  /* struct VALUE local: reserve remaining slots */
    skipws();
    if (src[cp]==44) { cp=cp+1; }                                               /* ',' -> next declarator (same base type) */
    else { cp=cp+1; return 0; } }                                              /* ';' -> done */
  }
  if (src[cp]==114 && src[cp+1]==101 && src[cp+2]==116 && src[cp+3]==117 && src[cp+4]==114 && src[cp+5]==110) {  /* return */
    cp=cp+6; vE=parse_expr(0); es("  ret v"); en(vE); es("\n"); skipws(); cp=cp+1; return 0;
  }
  if (src[cp]==40) {                                                            /* (expr)[i] = v  -- cast-subscript store, e.g. ((unsigned char*)p)[i] = v */
    base2=cp; larr=1; slot=cp+1;                                               /* scan to the matching ')' */
    while (slot<slen && larr>0) { if (src[slot]==40) { larr=larr+1; } if (src[slot]==41) { larr=larr-1; } slot=slot+1; }
    st=slot; while (st<slen && (src[st]==32 || src[st]==9)) { st=st+1; }
    if (src[st]==91) {                                                         /* ')' followed by '[' -> subscript on the paren value */
      cast_pw=0; cp=cp+1; slot=parse_expr(0); skipws(); cp=cp+1;               /* (expr) -> base pointer; cast_pw = pointee width */
      idx=cast_pw; if (idx==0) { idx=4; }
      skipws(); cp=cp+1; vE=parse_expr(0); skipws(); cp=cp+1;                  /* '[' index ']' */
      if (idx==1) { slot=emit_binop(4, slot, vE); }
      else if (idx==8) { slot=emit_binop(4, slot, emit_binop(6, vE, emit_const(3))); }
      else { slot=emit_binop(4, slot, emit_binop(6, vE, emit_const(2))); }
      skipws();
      if (src[cp]==61 && src[cp+1]!=61) { cp=cp+1; vE=parse_expr(0);           /* '=' ; rhs */
        if (idx==1) { es("  storeb v"); en(slot); es(" v"); en(vE); es("\n"); } else if (idx==2) { es("  store2 v"); en(slot); es(" v"); en(vE); es("\n"); }
        else if (idx==8) { vE=emit_conv(vE); es("  store8 v"); en(slot); es(" v"); en(vE); es("\n"); }
        else { es("  store v"); en(slot); es(" v"); en(vE); es("\n"); }
        skipws(); cp=cp+1; return 0; }
      skipws(); cp=cp+1; return 0; }                                          /* (expr)[i]; rvalue stmt -> discard, eat ';' */
    cp=base2; }                                                               /* not (expr)[..] -> restore, fall through */
  if (src[cp]==40 && src[cp+1]==42) {                                          /* R161/c99-R6: (*q)->field... = v -- STORE through a deref'd ptr-to-ptr (tcc's decl_designator `(*cur_field)->c = index_last`, which crashed the derived tcc -> ARRAY designated initializers `int a[4]={[2]=9}` unusable). Peek-then-commit; falls through untouched if not this exact form. */
    p0=cp; slot=cp+2; while (src[slot]==32 || src[slot]==9) { slot=slot+1; }    /* past '(' '*' ws */
    if (isalnum_(src[slot])) {
      cp=slot; read_ident(); skipws();
      if (src[cp]==41 && src[cp+1]==45 && src[cp+2]==62) {                      /* ') ->' -> commit to the deref-store form */
        lvref(); lglob=lv_glob; if (lglob) { base2=emit_load8(emit_gref(lv_noff,lv_nlen)); } else { base2=lv_sa; }  /* q's value (a T**) */
        base2=emit_load8(base2);                                              /* *q  (the T* base) */
        lstruct=lv_struct; cp=cp+1;                                           /* consume ')' */
        while (src[cp]==45 && src[cp+1]==62) {                                /* walk the -> chain */
          cp=cp+2; read_ident(); find_field(lstruct, id_off, id_len);
          base2=emit_binop(4, base2, emit_const(ffoff)); lstruct=ffstruct; st=ffw;
          if (src[cp]==45 && src[cp+1]==62) { base2=emit_load8(base2); }       /* intermediate pointer field -> load, continue */
          else { skipws(); cp=cp+1; vE=parse_expr(0);                          /* last field: '=' then store at width st */
            if (st==1) { es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); }
            else if (st==2) { es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
            else if (st==8) { vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
            else { es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
            skipws(); if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; } cp=cp+1; return 0; } } }
      cp=p0; }                                                                 /* not the (*q)-> store form -> restore, fall through */
  }
  if (src[cp]==40) { st=cp; cp=cp+1; skipws();                                  /* (TYPE)e; -- a cast statement (e.g. `(void)ev;`); eval via parse_expr + discard */
    if (at_decl()) { cp=st; vE=parse_expr(0); skipws(); if (src[cp]==59) { cp=cp+1; } return 0; }
    cp=st; }                                                                    /* not a cast -> restore, fall through */
  if (src[cp]==42) {                                                            /* *p = v -- store through ptr */
    cp=cp+1;
    if ((src[cp]==45 && src[cp+1]==45) || (src[cp]==43 && src[cp+1]==43)) {     /* R128: *--p = v / *++p = v -- PRE inc/dec p, then store (was unhandled -> read_ident on `--` -> wild store -> SEGV; tcc tokenizer `*--p = c = '.'`) */
      st=src[cp]; cp=cp+2; skipws();
      read_ident(); lvref(); lglob=lv_glob; lsa=lv_sa; loff=lv_noff; llen=lv_nlen; lchar=lv_char; lptr=lv_ptr; llong=lv_long; skipws();
      idx=4; if (lchar) { idx=1; } else if (lptr>=2 || llong) { idx=8; }        /* pointee stride */
      if (lglob) { slot=emit_load(emit_gref(loff,llen)); } else { slot=lsa; }   /* p's current value */
      if (st==43) { slot=emit_binop(4, slot, emit_const(idx)); } else { slot=emit_binop(5, slot, emit_const(idx)); }  /* new p = p +/- stride */
      if (lglob) { es("  store v"); en(emit_gref(loff,llen)); es(" v"); en(slot); es("\n"); } else { emit_copy(lsa, slot); }  /* write p back FIRST (pre) */
      skipws(); cp=cp+1; vE=parse_expr(0);                                      /* '=' ; rhs */
      if (idx==1) { es("  storeb v"); en(slot); es(" v"); en(vE); es("\n"); }
      else if (idx==8) { vE=emit_conv(vE); es("  store8 v"); en(slot); es(" v"); en(vE); es("\n"); }
      else { es("  store v"); en(slot); es(" v"); en(vE); es("\n"); }
      skipws(); cp=cp+1; return 0; }                                           /* ';' */
    if (src[cp]==40) {                                                          /* R27: *(TYPE)expr = v -- store through a cast-deref */
      base2=cp; cp=cp+1;
      if (decl_type()) { skipptr();                                             /* (TYPE *..) -> nstar = the cast's pointer depth */
        if (src[cp]==41) { cp=cp+1;                                             /* ')' */
          idx=4; if (nstar>=2) { idx=8; } else if (lchar) { idx=1; } else if (dchar) { idx=1; } else if (dlong) { idx=8; }  /* width of *(TYPE): deref drops one '*' */
          slot=parse_expr(1);                                                   /* address expr (min_prec 1: no assignment) */
          skipws(); cp=cp+1; vE=parse_expr(0);                                  /* '=' ; rhs */
          if (idx==1) { es("  storeb v"); en(slot); es(" v"); en(vE); es("\n"); } else if (idx==2) { es("  store2 v"); en(slot); es(" v"); en(vE); es("\n"); }
          else if (idx==8) { vE=emit_conv(vE); es("  store8 v"); en(slot); es(" v"); en(vE); es("\n"); }
          else { es("  store v"); en(slot); es(" v"); en(vE); es("\n"); }
          skipws(); cp=cp+1; return 0; } }                                     /* ';' */
      cp=base2; }                                                              /* not a cast -> restore, fall through */
    base2=cp; read_ident(); skipws();                                          /* peek: is the deref target a . / -> chain? (e.g. *bf->buf_end = c) */
    if ((src[cp]==45 && src[cp+1]==62) || src[cp]==46) {                        /* *CHAIN = v / *CHAIN++ = v : store through the field's pointer VALUE */
      cp=base2; slot=lval_addr(); skipws(); st=ffpw; if (st==0) { st=1; }       /* slot = &field (addr of the pointer field); st = pointee width */
      if ((src[cp]==43 && src[cp+1]==43) || (src[cp]==45 && src[cp+1]==45)) {   /* R130: `*p->field++ = v` -- store then post-inc the field pointer by its pointee width (tcc's `*s1->ifdef_stack_ptr++ = c`). Was unhandled. */
        idx=4; if (src[cp]==45) { idx=5; } cp=cp+2; skipws(); cp=cp+1;          /* skip ++/-- then '=' */
        base2=emit_load8(slot); vE=parse_expr(0);                              /* current pointer value; rhs */
        if (st==1) { es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); }
        else if (st==2) { es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
        else if (st==8) { vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
        else { es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
        base2=emit_binop8(idx, base2, emit_const(st)); es("  store8 v"); en(slot); es(" v"); en(base2); es("\n");  /* advance the field ptr */
        skipws(); cp=cp+1; return 0; }
      base2=emit_load8(slot); skipws(); cp=cp+1; vE=parse_expr(0);              /* plain *CHAIN = v */
      if (st==1) { es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); }
      else if (st==2) { es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
      else if (st==8) { vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
      else { es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
      skipws(); cp=cp+1; return 0; }
    cp=base2;
    read_ident(); lvref(); lglob=lv_glob; lsa=lv_sa; loff=lv_noff; llen=lv_nlen; lchar=lv_char; lptr=lv_ptr; llong=lv_long; lstruct=lv_struct; lflt=lv_flt; skipws();
    if (src[cp]==43 || src[cp]==45) {                                           /* *p++ = v / *p-- = v -- store then post inc/dec p */
      idx=4; if (lchar) { idx=1; } else if (lptr>=2 || llong) { idx=8; }        /* element width = pointer stride */
      st=src[cp]; cp=cp+2; skipws(); cp=cp+1;                                    /* skip '++'/'--' then '=' */
      if (lstruct>=0 && lptr==1 && lchar==0 && llong==0) {                       /* R77: *q++ = STRUCT value (e.g. sort_syms `*q++ = *p`) -- copy ssz bytes, advance by ssz */
        idx=ssz[lstruct];
        if (lglob) { slot=emit_load(emit_gref(loff,llen)); } else { slot=lsa; }  /* q's current value (address) */
        vE=rhs_saddr(); emit_scopy(slot, vE, idx);                               /* copy the struct */
        if (st==43) { base2=emit_binop(4, slot, emit_const(idx)); } else { base2=emit_binop(5, slot, emit_const(idx)); }
        if (lglob) { es("  store v"); en(emit_gref(loff,llen)); es(" v"); en(base2); es("\n"); } else { emit_copy(lsa, base2); }
        skipws(); cp=cp+1; return 0; }
      vE=parse_expr(0);
      if (lglob) { slot=emit_load(emit_gref(loff,llen)); } else { slot=lsa; }    /* p's current value (address) */
      if (idx==1) { es("  storeb v"); en(slot); es(" v"); en(vE); es("\n"); } else if (idx==2) { es("  store2 v"); en(slot); es(" v"); en(vE); es("\n"); }
      else if (idx==8) { vE=emit_conv(vE); es("  store8 v"); en(slot); es(" v"); en(vE); es("\n"); }
      else { es("  store v"); en(slot); es(" v"); en(vE); es("\n"); }
      if (st==43) { base2=emit_binop(4, slot, emit_const(idx)); } else { base2=emit_binop(5, slot, emit_const(idx)); }
      if (lglob) { es("  store v"); en(emit_gref(loff,llen)); es(" v"); en(base2); es("\n"); } else { emit_copy(lsa, base2); }
      skipws(); cp=cp+1; return 0; }                                            /* ';' */
    cp=cp+1;                                                                     /* = */
    if (lstruct>=0 && lptr==1 && lchar==0 && llong==0) {                          /* R95: *p = STRUCT value (whole-struct copy; tcc's `*vtop = *v`) -- a scalar store copied only 4 bytes */
      if (lglob) { slot=emit_load(emit_gref(loff,llen)); } else { slot=lsa; }
      vE=rhs_saddr(); emit_scopy(slot, vE, ssz[lstruct]);
      skipws(); cp=cp+1; return 0; }
    vE=parse_expr(0);
    if (lglob) { slot=emit_load(emit_gref(loff,llen)); } else { slot=lsa; }     /* p's value = the address */
    idx=4; if (lchar) { idx=1; } else if (lptr>=2 || llong) { idx=8; }          /* R150: `*p = v;` STATEMENT store honors pointee width (was an unconditional 4-byte store -> `*charptr = 0` zeroed 4 bytes, clobbering the neighbor byte; every OTHER deref-store path branched on width, this one didn't) */
    if (idx==1) { es("  storeb v"); en(slot); es(" v"); en(vE); es("\n"); }
    else if (idx==8) { vE=emit_conv(vE); es("  store8 v"); en(slot); es(" v"); en(vE); es("\n"); }
    else { es("  store v"); en(slot); es(" v"); en(vE); es("\n"); }
    skipws(); cp=cp+1; return 0; }
  read_ident(); slot=id_off; base2=id_len; skipws();                            /* NAME ... (stash for label) */
  if (src[cp]==58) { es("  label "); emit_name(slot, base2); es("\n"); cp=cp+1;  /* LABEL: stmt -- the labeled stmt is PART of this statement (R45: else `if (c) lab: s;` guards only the label and s runs unconditionally) */
    skipws(); if (src[cp]==125) { return 0; }                                   /* label at block end (no stmt) */
    parse_stmt(); return 0; }
  if (src[cp]==40) {                                                            /* f(...); call statement (or a varargs builtin stmt) */
    if (id_is("__va_start", 10) || id_is("__va_next", 9) || id_is("__va_copy", 9)) { cp=st; parse_expr(0); skipws(); cp=cp+1; return 0; }  /* route builtins through parse_atom */
    lstruct=fn_returns_sptr(id_off, id_len);                                    /* R143: `f()->field = v` -- capture the return struct BEFORE parse_call clobbers id_* */
    base2=parse_call(id_off, id_len); skipws();
    if (lstruct>=0 && src[cp]==45 && src[cp+1]==62) {                           /* call result is a struct ptr, followed by -> : walk to the assigned field's ADDRESS, then store */
      st=4;
      while (src[cp]==45 && src[cp+1]==62) {
        cp=cp+2; read_ident(); find_field(lstruct, id_off, id_len);
        base2=emit_binop(4, base2, emit_const(ffoff)); lstruct=ffstruct; st=ffw;
        if (src[cp]==45 && src[cp+1]==62) { base2=emit_load8(base2); }          /* intermediate pointer field -> load, continue */
        else { skipws(); if (src[cp]==61) { cp=cp+1; vE=parse_expr(0);          /* last field: base2 = &field ; `= rhs` */
            if (st==1) { es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); }
            else if (st==2) { es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
            else if (st==8) { vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
            else { es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
            skipws(); if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; } cp=cp+1; return 0; }
          break; } }
    }
    if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; }                       /* R127: `f(), tail;` -- comma tail is PART of this stmt (else it escapes an if/loop-body guard: tcc's gen_addr32 `if(r&VT_SYM) greloca(..), c=0;` ran c=0 unconditionally -> every asm `$imm` via gen_le32 emitted 0) */
    cp=cp+1; return 0; }
  lvref(); lglob=lv_glob; lsa=lv_sa; lchar=lv_char; lstruct=lv_struct; larr=lv_arr; lptr=lv_ptr; llong=lv_long; loff=lv_noff; llen=lv_nlen; lflt=lv_flt;  /* capture before sub-parse */
  if (((src[cp]==45 && src[cp+1]==62) || src[cp]==46) && chain_pure()) {        /* R21: pure . / -> chain write -> typed-value resolver */
    cp=slot; base2=lval_addr(); skipws();
    if ((src[cp]==43 && src[cp+1]==43) || (src[cp]==45 && src[cp+1]==45)) {     /* R113: p->field++ / p->field-- / s.field++ as a STATEMENT -- load, +/-1, store back. Was mis-parsed as `= (unary +)` -> stored 0 (tcc's `s1->nb_errors++` never incremented -> error path never propagated a nonzero exit). */
      if (src[cp]==43) { idx=4; } else { idx=5; }                              /* add / sub */
      st=la_w; slot=1; if (st==8 && la_long==0) { slot=ffpw; }                  /* R130: pointer field ++/-- advances by the POINTEE width (int*->4), not 1 */
      if (st==1) { vE=emit_binop(idx, emit_loadb(base2), emit_const(slot)); es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); }
      else if (st==2) { vE=emit_binop(idx, emit_load2(base2), emit_const(slot)); es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
      else if (st==8) { vE=emit_binop8(idx, emit_load8(base2), emit_const(slot)); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
      else { vE=emit_binop(idx, emit_load(base2), emit_const(slot)); es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
      cp=cp+2; skipws();
      if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; }                     /* comma tail (guards an unbraced if-body) */
      cp=cp+1; return 0; }                                                      /* ';' */
    if (is_caop()) { cp=cp+ca_len;                                             /* L OP= v : v = load[la_w](L) OP rhs */
      if (la_w==1) { vE=emit_binop(ca_op, emit_loadb(base2), parse_expr(0)); es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); } else if (la_w==2) { vE=emit_binop(ca_op, emit_load2(base2), parse_expr(0)); es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }  /* R92: 2-byte field OP= must READ via load2 (was loadb -> truncated `unsigned short r &= ..`, losing VT_SYM 0x200 -> reloc symbol 0) */
      else if (la_w==8) { vE=emit_binop(ca_op, emit_load8(base2), parse_expr(0)); vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
      else { vE=emit_binop(ca_op, emit_load(base2), parse_expr(0)); es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
      skipws();
      if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; }                     /* R74: `p->f |= v, y = w;` comma tail is PART of this statement (else it escapes an if-body guard) */
      cp=cp+1; return 0; }
    cp=cp+1; st=la_w; st_slng=(la_long && la_unsig==0); lflt=la_flt;            /* '=' ; capture LHS width + SIGNED-long-ness (R94) + floatness (R160/c99-R3) BEFORE the rhs clobbers la_* */
    if (st==4 && la_struct>=0 && la_arr==0) {                                   /* R53: struct-VALUE field target (vtop->type = *type / = s->type / = type1) */
      idx=la_struct; vE=rhs_saddr(); emit_scopy(base2, vE, ssz[idx]);
      skipws(); cp=cp+1; return 0; }
    vE=parse_expr(0);
    if (lflt && isflt[vE]==0) { vE=emit_int_to_flt(vE); }                       /* R160/c99-R3: `c.ld = c.i` -- int rhs -> float before the field store (gen_cvt_itof's (double)int) */
    else if (lflt==0 && isflt[vE]) { vE=emit_flt_to_int(vE); }                  /* R160/c99-R3: `c.i = c.ld` -- FLOAT rhs -> INT field (gen_cvt_ftoi's (int)double const path) */
    if (st==1) { es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); } else if (st==2) { es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
    else if (st==8) {
      if (lflt) { if (is64[vE]==0) { vE=emit_conv(vE); } }                      /* R160/c99-R3: a float field store -> the 8-byte double bits, no sign-extend */
      else if (st_slng) { if (ishi[vE]==0) { vE=emit_convs(vE); } }             /* R94: SIGNED long field -> sext unless genuine high bits (ishi). NOT is64 (a long local is zero-extended, high=0 not sign) -> keeps r_addend=-4 correct */
      else { if (ishi[vE]==0 && is64[vE]==0) { vE=emit_conv(vE); } }             /* R137: UNSIGNED long field (uint64_t, via typedef unsignedness) -> zext; skip if already 64-bit so `tokc.i = n` keeps n's high bits */
      es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
    else { es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
    skipws();
    if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; }                       /* R74: `p->f = v, y = w;` comma tail is PART of this statement (else it escapes an if-body guard) */
    cp=cp+1; return 0; }                                                        /* ';' */
  if (src[cp]==45 && src[cp+1]==62) {                                           /* p->field->... = v  (write, chained; impure/subscript fallback) */
    if (lglob) { base2=emit_load(emit_gref(loff,llen)); } else { base2=lsa; }   /* base = ptr value */
    while (src[cp]==45 && src[cp+1]==62) {
      cp=cp+2; read_ident(); find_field(lstruct, id_off, id_len);
      base2=emit_binop(4, base2, emit_const(ffoff));                           /* addr = base + field offset */
      lstruct=ffstruct;                                                        /* field's struct type -> next -> */
      if (src[cp]==45 && src[cp+1]==62) { base2=emit_load8(base2); }           /* intermediate: load ptr, continue */
      else {                                                                    /* last field: '=' then store */
        skipws(); cp=cp+1; st=ffw; vE=parse_expr(0);                            /* capture LHS field width BEFORE the rhs clobbers ffw */
        if (st==1) { es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); } else if (st==2) { es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
        else if (st==8) { vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
        else { es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
        skipws(); cp=cp+1; return 0; } }
    return 0; }
  if (src[cp]==46) {                                                           /* x.field.field... = v (nested sub-structs inline) */
    if (lglob) { base2=emit_gref(loff,llen); } else { base2=emit_ref(lsa); }    /* &struct base */
    while (src[cp]==46) {
      cp=cp+1; read_ident(); find_field(lstruct, id_off, id_len);
      base2=emit_binop(4, base2, emit_const(ffoff)); lstruct=ffstruct;          /* addr += offset; descend */
      if (src[cp]==45 && src[cp+1]==62) {                                       /* x.field->... = v : field is a pointer -> load, continue as arrow */
        base2=emit_load8(base2);
        while (src[cp]==45 && src[cp+1]==62) {
          cp=cp+2; read_ident(); find_field(lstruct, id_off, id_len);
          base2=emit_binop(4, base2, emit_const(ffoff)); lstruct=ffstruct;
          if (src[cp]==45 && src[cp+1]==62) { base2=emit_load8(base2); }
          else { skipws(); cp=cp+1; st=ffw; vE=parse_expr(0);                    /* capture LHS width before rhs */
            if (st==1) { es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); } else if (st==2) { es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
            else if (st==8) { vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
            else { es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
            skipws(); cp=cp+1; return 0; } }
        return 0; }
      if (src[cp]!=46) {                                                        /* last field: '=' then store */
        skipws(); cp=cp+1; st=ffw; vE=parse_expr(0);                            /* capture LHS width before rhs */
        if (st==1) { es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); } else if (st==2) { es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
        else if (st==8) { vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
        else { es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
        skipws(); cp=cp+1; return 0; } }
    return 0; }
  if (src[cp]==91) {                                                            /* x[i] = v -- put/putb */
    cp=cp+1; idx=parse_expr(0); skipws(); cp=cp+1; skipws();                    /* past ] */
    if (src[cp]==40 && (lptr>=1 || larr)) {                                     /* R153: tbl[i](args); STATEMENT -- indirect call through a fn-pointer array element (atexit's handler loop). Element = load8(base + i*8). */
      if (lglob) { if (larr) { base2=emit_gref(loff,llen); } else { base2=emit_load(emit_gref(loff,llen)); } } else { base2=lsa; }
      base2=emit_load8(emit_binop(4, base2, emit_binop(6, idx, emit_const(3))));  /* fn-ptr element */
      emit_icall(base2); skipws(); if (src[cp]==59) { cp=cp+1; } return 0; }
    if (src[cp]==46 && lstruct>=0 && (lptr==1 || (lptr==0 && larr))) {          /* R21 P4 + R48: p[i].field / arr[i].field = v (element addr = base + i*ssz) */
      if (lglob) { if (larr) { base2=emit_gref(loff,llen); } else { base2=emit_load(emit_gref(loff,llen)); } } else { base2=lsa; }
      base2=emit_binop(4, base2, emit_binop(1, idx, emit_const(ssz[lstruct]))); /* element addr = base + i*ssz */
      base2=lval_cont(base2, lstruct, 8);                                       /* trailing . / -> chain */
      skipws(); st=la_w;
      if (is_caop()) { cp=cp+ca_len; vE=parse_expr(0);                          /* R146: arr[i].field OP= v -- was mis-parsed as `=` (garbage stored, 0) because this path didn't check for a compound op; tcc's asm_rex `ops[rmi].reg -= 8` -> r8-r15 mis-encoded */
        if (st==1) { vE=emit_binop(ca_op, emit_loadb_sx(base2, la_unsig), vE); es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); }
        else if (st==2) { vE=emit_binop(ca_op, emit_load2(base2), vE); es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
        else if (st==8) { vE=emit_binop(ca_op, emit_load8(base2), vE); vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
        else { vE=emit_binop(ca_op, emit_load(base2), vE); es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
        skipws(); if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; } cp=cp+1; return 0; }
      cp=cp+1; vE=parse_expr(0);                                                /* '=' ; capture LHS width before rhs */
      if (st==1) { es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); } else if (st==2) { es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
      else if (st==8) { vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
      else { es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
      skipws(); cp=cp+1; return 0; }
    if (src[cp]==45 && src[cp+1]==62) {                                         /* x[i]->field... = v (element is a struct pointer) */
      if (lglob) { if (larr) { base2=emit_gref(loff,llen); } else { base2=emit_load(emit_gref(loff,llen)); } } else { base2=lsa; }
      if (lptr>=2 || llong || (larr && lptr>=1)) { base2=emit_load8(emit_binop(4, base2, emit_binop(6, idx, emit_const(3)))); }  /* R36: pointer-element array -> 8-byte element */
      else { base2=emit_load(emit_binop(4, base2, emit_binop(6, idx, emit_const(2)))); }                    /* int-element array -> 4-byte element */
      while (src[cp]==45 && src[cp+1]==62) {
        cp=cp+2; read_ident(); find_field(lstruct, id_off, id_len);
        base2=emit_binop(4, base2, emit_const(ffoff)); lstruct=ffstruct;        /* addr = base + field offset */
        if (src[cp]==45 && src[cp+1]==62) { base2=emit_load8(base2); }          /* intermediate: struct ptr field is 8-byte */
        else if (src[cp]==91) {                                                 /* R57: x[i]->arr[j] = v (subscript on an array/ptr field) */
          if (ffarr==0) { base2=emit_load8(base2); }                            /* pointer field: elements behind the pointer */
          cp=cp+1; slot=parse_expr(0); skipws(); cp=cp+1;                       /* [j] */
          st=ffw;
          if (st==8) { base2=emit_binop(4, base2, emit_binop(6, slot, emit_const(3))); }
          else if (st==1) { base2=emit_binop(4, base2, slot); }
          else { base2=emit_binop(4, base2, emit_binop(6, slot, emit_const(2))); }
          skipws(); cp=cp+1; vE=parse_expr(0);                                  /* '=' rhs */
          if (st==1) { es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); } else if (st==2) { es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
          else if (st==8) { vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
          else { es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
          skipws(); cp=cp+1; return 0; }
        else { skipws(); cp=cp+1; st=ffw; vE=parse_expr(0);                     /* last field: capture LHS width before rhs */
          if (st==1) { es("  storeb v"); en(base2); es(" v"); en(vE); es("\n"); } else if (st==2) { es("  store2 v"); en(base2); es(" v"); en(vE); es("\n"); }
          else if (st==8) { vE=emit_conv(vE); es("  store8 v"); en(base2); es(" v"); en(vE); es("\n"); }
          else { es("  store v"); en(base2); es(" v"); en(vE); es("\n"); }
          skipws(); cp=cp+1; return 0; } }
      return 0; }
    if (src[cp]==61 && src[cp+1]!=61 && lstruct>=0 && ((lptr==1 && larr==0) || (lptr==0 && larr))) {  /* R85: x[i] = whole STRUCT value -- element is a struct: struct-ptr `C* p` (p[i], lptr==1 larr==0) or struct-array `C arr[]` (lptr==0 larr). R109: NOT an array of struct POINTERS `C* bas[]` (lptr==1 larr==1) whose element is an 8-byte pointer -> falls through to store8 (tcc's `struct case_t **p`) */
      if (lglob) { if (larr) { base2=emit_gref(loff,llen); } else { base2=emit_load(emit_gref(loff,llen)); } } else { base2=lsa; }
      base2=emit_binop(4, base2, emit_binop(1, idx, emit_const(ssz[lstruct])));  /* element addr = base + i*ssz */
      cp=cp+1; vE=rhs_saddr(); emit_scopy(base2, vE, ssz[lstruct]);              /* '=' ; struct copy */
      skipws(); cp=cp+1; return 0; }
    if (is_caop()) { cp=cp+ca_len;                                              /* R122: x[i] OP= v -> x[i] = x[i] OP v (load elem, apply, store back). Was: fell to bare-expr `if(src[cp]!=61)` and emitted NO store -> `regs_allocated[r]|=m` silently dropped / corrupted a live slot */
      if (lglob) { if (larr) { base2=emit_gref(loff,llen); } else { base2=emit_load(emit_gref(loff,llen)); } } else { base2=lsa; }
      if (lptr>=2 || llong || (larr && lptr>=1)) { slot=emit_binop(4, base2, emit_binop(6, idx, emit_const(3))); st=8; }
      else if (lchar) { slot=emit_binop(4, base2, idx); st=1; }
      else { slot=emit_binop(4, base2, emit_binop(6, idx, emit_const(2))); st=4; }
      vE=parse_expr(0);
      if (st==1) { vE=emit_binop(ca_op, emit_loadb(slot), vE); es("  storeb v"); en(slot); es(" v"); en(vE); es("\n"); }
      else if (st==8) { vE=emit_binop(ca_op, emit_load8(slot), vE); vE=emit_conv(vE); es("  store8 v"); en(slot); es(" v"); en(vE); es("\n"); }
      else { vE=emit_binop(ca_op, emit_load(slot), vE); es("  store v"); en(slot); es(" v"); en(vE); es("\n"); }
      skipws(); if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; } cp=cp+1; return 0; }
    if (src[cp]!=61) { cp=cp+1; return 0; }                                     /* bare `x[i];` expr stmt -> discard, skip ';' */
    cp=cp+1;                                                                     /* '=' */
    vE=parse_expr(0);
    if (lglob) { if (larr) { base2=emit_gref(loff,llen); } else { base2=emit_load(emit_gref(loff,llen)); } } else { base2=lsa; }  /* global array: address; global pointer: value */
    if (lptr>=2 || llong || (larr && lptr>=1)) { slot=emit_binop(4, base2, emit_binop(6, idx, emit_const(3))); vE=emit_conv(vE);  /* long/ptr element (incl T*[] and char*[] arrays): store8(base + i*8) -- R47: BEFORE the char test (matches the R36 read path) */
      es("  store8 v"); en(slot); es(" v"); en(vE); es("\n"); }
    else if (lchar) { es("  putb v"); en(base2); es(" v"); en(idx); es(" v"); en(vE); es("\n"); }   /* char: 1-byte element */
    else { slot=emit_binop(4, base2, emit_binop(6, idx, emit_const(2)));         /* int element: store(base + i*4) -- stride 4 (LP64) */
      es("  store v"); en(slot); es(" v"); en(vE); es("\n"); }
    skipws(); cp=cp+1; return 0; }
  if (is_caop()) { cp=cp+ca_len; vE=parse_expr(0);                              /* x OP= v -> x = x OP v (scalar) */
    if ((ca_op==4 || ca_op==5) && lptr>=1) {                                    /* R106: `p += n` / `p -= n` on a POINTER -- scale n by the pointee size (the non-compound `p = p + n` already scales via cast_ps; tcc's `macro_ptr += (sz+3)/4` on an int* advanced 1 byte/word) */
      idx=4; if (lchar) { idx=1; } else if (lptr>=2 || llong) { idx=8; } else if (lstruct>=0) { idx=ssz[lstruct]; }
      if (idx!=1) { vE=emit_binop(1, vE, emit_const(idx)); } }
    if (lglob) { slot=emit_gref(loff,llen);
      if (lflt) { idx=emit_load8L(slot); isflt[idx]=1; vE=emit_fbinop(ca_op, idx, vE); es("  store8 v"); en(slot); es(" v"); en(vE); es("\n"); }  /* R160/c99-R3: `double g OP= v` -> soft-float */
      else if (llong) { vE=emit_binop8(ca_op, emit_load8L(slot), vE); es("  store8 v"); en(slot); es(" v"); en(vE); es("\n"); }  /* R160: `long g OP= v` -> 64-bit */
      else { vE=emit_binop(ca_op, emit_load(slot), vE); es("  store v"); en(slot); es(" v"); en(vE); es("\n"); } }
    else { if (lflt) { is64[lsa]=1; isflt[lsa]=1; emit_copy8(lsa, emit_fbinop(ca_op, lsa, vE), 1); isflt[lsa]=1; }  /* R160/c99-R3: `double x OP= v` local -> soft-float */
      else if (llong) { is64[lsa]=1; emit_copy8(lsa, emit_binop8(ca_op, lsa, vE), 1); }  /* R160: `long x OP= v` local -> 64-bit (was 32-bit -> high bits lost) */
      else { emit_copy(lsa, emit_binop(ca_op, lsa, vE)); } }
    skipws();
    if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; }                        /* R74: `x |= v, y = w;` -- comma tail is PART of this statement (else it escapes an if-body guard) */
    cp=cp+1; return 0; }
  if (src[cp]==61) { cp=cp+1;                                                   /* x = v */
    if (lstruct>=0 && lptr==0 && lchar==0 && llong==0 && larr==0 && lglob==0) {  /* R53: LOCAL struct VALUE = rhs (dtype = *type) */
      base2=emit_ref(lsa); vE=rhs_saddr(); emit_scopy(base2, vE, ssz[lstruct]);
      skipws(); cp=cp+1; return 0; }
    if (lstruct>=0 && lptr==0 && lchar==0 && llong==0 && larr==1 && lglob==1) {  /* R53: GLOBAL struct VALUE = rhs */
      base2=emit_gref(loff,llen); vE=rhs_saddr(); emit_scopy(base2, vE, ssz[lstruct]);
      skipws(); cp=cp+1; return 0; }
    vE=parse_expr(0);
    if (lflt && isflt[vE]==0) { vE=emit_int_to_flt(vE); }                        /* R160/c99-R3: `d = <int>` -> int->float before the store */
    else if (lflt==0 && isflt[vE]) { vE=emit_flt_to_int(vE); }                   /* R160/c99-R3: `intvar = <float>` -> float->int */
    if (lglob) { slot=emit_gref(loff,llen);
      if (llong || lflt) { vE=emit_widen(vE, cast_un); es("  store8 v"); en(slot); es(" v"); en(vE); es("\n"); }  /* R132/R133: long/double GLOBAL scalar = rhs -> 8-byte store */
      else { es("  store v"); en(slot); es(" v"); en(vE); es("\n"); } }
    else { if (llong || lflt) { emit_copy8(lsa, vE, cast_un); if (lflt) { isflt[lsa]=1; } } else { emit_copy(lsa, vE); } }   /* R132/R133: long/double LOCAL scalar = rhs -> 64-bit copy */
    skipws();
    if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; }                        /* R49: `a = 0, b = x;` -- the comma tail is PART of this statement (if-body scope) */
    cp=cp+1; return 0; }
  cp=st; parse_expr(0); skipws();                                              /* bare expression statement (e.g. 0; f()+g();) */
  if (src[cp]==44) { cp=cp+1; parse_stmt(); return 0; }                         /* R148: `s++, reg=v;` -- comma tail is PART of this statement (else it escapes an unbraced if-body guard -> tcc's asm_parse_numeric_reg `if(...)s++,reg=reg*10+c-'0';` ran the reg= unconditionally, mis-parsing r8/r9) */
  cp=cp+1; return 0; }
int right_init(int noff, int nlen) { int vE; int vi; int islong; int isf;  /* R2(nontrivial): record the var (type + placeholder slot) BEFORE parsing the init, so a self-referential `sizeof *p` in p's own initializer resolves p's type (subsumes the R1 save/restore -- rec_var now runs with the decl-type globals still intact, before parse_expr clobbers them) */
  islong=dlong; isf=dflt;                                                      /* R132(part B): capture long-ness (+ R160/c99-R3 floatness) before parse_expr clobbers the decl globals */
  rec_var(noff, nlen, 0, 0, czero);                                            /* register the declared type now; slot patched after */
  vi=nvar-1;
  vE=parse_expr(0);                                                            /* init expr; `sizeof *p` etc. now sees p */
  if (isf) { if (isflt[vE]==0) { vE=emit_int_to_flt(vE); } var_slot[vi]=emit_binop8(16, vE, vE); isflt[var_slot[vi]]=1; return 0; }  /* R160/c99-R3: `double d = <int>` -> int->float; float value -> 64-bit slot copy */
  if (islong) { if (is64[vE]) { var_slot[vi]=emit_binop8(16, vE, vE); } else { var_slot[vi]=emit_widen(vE, cast_un); } return 0; }  /* R132(part B): long LOCAL init -> 64-bit slot (or8 x x = x, or R144 signed-aware widen: `long a=-5` must SEXT, not zext) */
  var_slot[vi]=emit_binop(4, vE, czero); return 0; }                          /* patch the slot to the init value */
int emit_scopy(int dst, int srca, int nb) { int k; int va; int vb; int vv;      /* R53: struct-value copy -- EXACTLY nb bytes (R81: 8/4/2/1 tails; store8-only over-copied and clobbered neighbors) */
  k=0;
  while (k+8<=nb) { va=emit_binop(4, dst, emit_const(k)); vb=emit_binop(4, srca, emit_const(k));
    vv=emit_load8(vb); es("  store8 v"); en(va); es(" v"); en(vv); es("\n"); k=k+8; }
  while (k+4<=nb) { va=emit_binop(4, dst, emit_const(k)); vb=emit_binop(4, srca, emit_const(k));
    vv=emit_load(vb); es("  store v"); en(va); es(" v"); en(vv); es("\n"); k=k+4; }
  while (k+2<=nb) { va=emit_binop(4, dst, emit_const(k)); vb=emit_binop(4, srca, emit_const(k));
    vv=emit_load2(vb); es("  store2 v"); en(va); es(" v"); en(vv); es("\n"); k=k+2; }
  while (k+1<=nb) { va=emit_binop(4, dst, emit_const(k)); vb=emit_binop(4, srca, emit_const(k));
    vv=emit_loadb(vb); es("  storeb v"); en(va); es(" v"); en(vv); es("\n"); k=k+1; }
  return 0; }
int rhs_saddr() { int sv; int base; int idx; int st; int glob; int sa; int off; int len; int larr; int qq; int qd; int qf; int cnd; int tn2;  /* R53: ADDRESS of a struct-valued RHS: *IDENT | IDENT.chain | IDENT | R85: IDENT[i] */
  skipws();
  qq=cp; qd=0; qf=0;                                                            /* R161/tcc-self-R1: detect a top-level '?' -> struct-valued ternary RHS `cond ? sA : sB` (tcc's expr_cond `type = bt1==7 ? type1 : type2`). Without this rhs_saddr read `cond` as the struct -> `z = cond` (crashed the derived tcc's own expr_cond on any struct-ternary). */
  while (qq<slen) {
    if (src[qq]==40 || src[qq]==91) { qd=qd+1; }
    else if (src[qq]==41 || src[qq]==93) { if (qd==0) { qq=slen; } else { qd=qd-1; } }
    else if (qd==0 && src[qq]==63) { qf=1; qq=slen; }
    else if (qd==0 && (src[qq]==59 || src[qq]==44)) { qq=slen; }
    qq=qq+1; }
  if (qf) {                                                                     /* select the ADDRESS of the taken struct branch (each branch resolved by a recursive rhs_saddr) */
    cnd=parse_expr(1); skipws(); cp=cp+1;                                       /* condition (stops before '?'), then consume '?' */
    base=emit_const(0); tn2=emit_notbool(cnd);
    emit_brz(cnd); sa=rhs_saddr(); emit_copy(base, sa); es("  pool\n");         /* cond true: base = &sA */
    skipws(); cp=cp+1;                                                          /* ':' */
    emit_brz(tn2); sa=rhs_saddr(); emit_copy(base, sa); es("  pool\n");         /* cond false: base = &sB */
    return base; }
  if (src[cp]==42) { cp=cp+1; skipws();                                          /* *EXPR -> the pointer VALUE is the struct's address */
    sv=cp;                                                                       /* R96: *f() (deref of a CALL) -- was read as a bare var -> call never consumed -> crash */
    if (isalnum_(src[cp])) { read_ident(); skipws(); if (src[cp]==40) { cp=sv; return parse_expr(0); } }
    cp=sv; read_ident(); lvref();                                                /* *p -> p's value */
    glob=lv_glob; sa=lv_sa; off=lv_noff; len=lv_nlen; st=lv_struct;
    if (glob) { base=emit_load(emit_gref(off, len)); } else { base=sa; }         /* pointer value = the struct's address */
    skipws();
    if ((src[cp]==45 && src[cp+1]==45) || (src[cp]==43 && src[cp+1]==43)) {       /* R107: `*p--` / `*p++` -- use the CURRENT pointer value, then post-inc/dec p (tcc's `switchval = *vtop--`; the `--` was dropped for a STRUCT copy -> the switch value was never popped -> +1 vstack leak) */
      idx=4; if (lv_char) { idx=1; } else if (st>=0) { idx=ssz[st]; } else if (lv_ptr>=2 || lv_long) { idx=8; }  /* pointee stride */
      larr=5; if (src[cp]==43) { larr=4; } cp=cp+2; skipws();                     /* -- => sub(5); ++ => add(4) */
      idx=emit_binop(larr, base, emit_const(idx));                               /* new pointer value -- compute BEFORE the store line (emit_binop emits its own rzir) */
      if (glob) { sa=emit_gref(off, len); es("  store v"); en(sa); es(" v"); en(idx); es("\n"); }
      else { emit_copy(lv_sa, idx); } }
    return base; }
  sv=cp; read_ident(); skipws();
  if (((src[cp]==45 && src[cp+1]==62) || src[cp]==46) && chain_pure()) { cp=sv; return lval_addr(); }  /* q->f.g chain -> field address */
  lvref(); glob=lv_glob; sa=lv_sa; st=lv_struct; off=lv_noff; len=lv_nlen; larr=lv_arr;
  if (src[cp]==91) {                                                            /* R85: ptr[i] / arr[i] struct-value element -> address = base + i*ssz */
    cp=cp+1; idx=parse_expr(0); skipws(); cp=cp+1;
    if (glob) { if (larr) { base=emit_gref(off,len); } else { base=emit_load(emit_gref(off,len)); } } else { base=sa; }  /* array: base addr; pointer: its value */
    return emit_binop(4, base, emit_binop(1, idx, emit_const(ssz[st]))); }
  if (glob) { return emit_gref(off, len); }                                     /* global struct value */
  if (larr) { return sa; }                                                      /* array-ish base */
  return emit_ref(sa); }                                                        /* local struct value: its slots' address */

int at_func() { int s; int r;                                                  /* lookahead: is '[quals] TYPE [*..]NAME (' a function/prototype? */
  s=cp; r=0;
  if (tok_is("typedef")) { cp=s; return 0; }                                  /* typedef is never a function */
  if (decl_type()) { skipptr(); skipws();                                     /* TYPE [*..] (multi-word + quals via decl_type) */
    if (isalnum_(src[cp])) { read_ident(); skipws(); if (src[cp]==40) { r=1; } } }  /* NAME ( */
  cp=s; return r; }
int at_funcdef() { int s; int r;                                              /* like at_func but only a DEFINITION (body '{'), not a prototype */
  s=cp; r=0;
  if (tok_is("typedef")) { cp=s; return 0; }
  if (decl_type()) { skipptr(); skipws();
    if (isalnum_(src[cp])) { read_ident(); skipws();
      if (src[cp]==40) { skipparens(); skipws(); if (src[cp]==123) { r=1; } } } }   /* NAME (params) { */
  cp=s; return r; }
int at_decl() { int p; int l; int s; int r;                                   /* at a type/qualifier keyword or a typedef alias name? */
  s=cp; r=0; skipquals(); skipws();
  if (tok_is("typedef")) { r=1; }
  else if (tok_is("struct")) { r=1; }
  else if (tok_is("enum")) { r=1; }
  else if (tok_is("void")) { r=1; } else if (tok_is("char")) { r=1; } else if (tok_is("int")) { r=1; }
  else if (tok_is("long")) { r=1; } else if (tok_is("short")) { r=1; }
  else if (tok_is("unsigned")) { r=1; } else if (tok_is("signed")) { r=1; }
  else if (tok_is("double")) { r=1; } else if (tok_is("float")) { r=1; }        /* R56: float/double decls (R44 added them to decl_type only -- at_decl walked INTO `extern float strtof(...)` registering phantom globals) */
  else if (tok_is("union")) { r=1; }
  else { p=cp; while (p<slen && isalnum_(src[p])) { p=p+1; } l=p-cp;            /* lookahead identifier -> known typedef? */
    if (l>0) { if (find_typedef(cp, l)>=0) { r=1; } } }
  cp=s; return r; }
int parse_global() { int noff; int nlen; int addr; int n; int isp; int w; int cnt;  /* top-level int/char/struct/typedef ... ; */
  if (src[cp]==116 && src[cp+1]==121 && src[cp+2]==112 && src[cp+3]==101 && src[cp+4]==100 && src[cp+5]==101 && src[cp+6]==102 && (src[cp+7]==32 || src[cp+7]==9 || src[cp+7]==10)) {  /* typedef (R56: tab/newline after keyword too) */
    cp=cp+7; skipws(); parse_typedef(); return 0; }
  if (at_func()) {                                                             /* prototype (fn DEFs excluded by the loop) -> skip to ';' */
    while (cp<slen && src[cp]!=59) { cp=cp+1; } if (cp<slen) { cp=cp+1; } return 0; }
  decl_type(); skipws();
  if (src[cp]==59) { cp=cp+1; return 0; }                                       /* bare type decl (e.g. `enum {..};`) -- no variable */
  if (dstk) {                                                                   /* struct type (named or via typedef alias) */
    if (src[cp]==123) { parse_struct_def(); skipws();                           /* struct NAME { ... } [decl] ; */
      if (src[cp]==59) { cp=cp+1; return 0; }                                   /* trailing ';' not yet consumed (prepass/redef) -> type def only */
      if (psd_semi) { return 0; } }                                            /* parse_struct_def ate the ';' -> type def only; else a var declarator follows */
    while (1) {                                                                 /* R24: per-declarator loop (struct globals) */
    isp=dptr; skipws(); if (src[cp]==42) { isp=1; } skipptr(); if (nstar>0) { isp=1; }  /* pointer: * (any depth) or ptr typedef */
    read_ident(); noff=id_off; nlen=id_len; skipws();
    if (src[cp]==91) {                                                          /* X arr[expr] : array of struct-value or struct-ptr */
      cp=cp+1; n=const_expr(); skipws(); cp=cp+1;                               /* [ const-size ] */
      skipws();
      if (src[cp]==61 && isp==0 && dstruct>=0) { cp=cp+1; skipws();             /* R45: = { {..}, ... } struct-array initializer */
        cnt=parse_sarr_init(noff, nlen, dstruct); if (n<1) { n=cnt; } }
      if (isp) { w=1; } else { w=1; if (dstruct>=0) { w=(ssz[dstruct]+7)/8; } }  /* slots per element (ptr:1 slot; value:ceil(ssz/8)) */
      if (n<1) { n=1; }
      if (isp==0 && dstruct>=0) { dvbytes=n*ssz[dstruct]; } else { dvbytes=n*w*8; }  /* R140: sizeof(arr) = count * TRUE element size; struct-value elements are ssz bytes (NOT the slot-padded ceil(ssz/8)*8 -- e.g. {short,short,uchar}=6, [2]=12 not 16). Allocation below stays slot-padded (gcount/emit_gdef). */
      addr=7340032+gcount*8; gcount=gcount+n*w; rec_var(noff,nlen,1,1,addr);
      if (dextern==0) { emit_gdef(noff, nlen, n*w*8); } }
    else if (isp) { addr=7340032+gcount*8; gcount=gcount+1; rec_var(noff,nlen,1,0,addr);         /* pointer scalar */
      if (dextern==0) { emit_gdef(noff, nlen, 8); } }
    else { n=1; if (dstruct>=0) { n=(ssz[dstruct]+7)/8; }                       /* value (allocate size; opaque struct -> 1 slot) */
      addr=7340032+gcount*8; gcount=gcount+n; rec_var(noff,nlen,1,1,addr);
      if (dextern==0) { emit_gdef(noff, nlen, n*8); }
      skipws();
      if (src[cp]==61 && dstruct>=0) { cp=cp+1; parse_struct_val_init(noff, nlen, dstruct); } }  /* R118: struct-value initializer `T s = {..}` -- was DROPPED */
    skipws();
    if (src[cp]==44) { cp=cp+1; }                                              /* ',' -> next declarator */
    else { cp=cp+1; return 0; } } }                                           /* ';' -> done */
  while (1) {                                                                  /* R22: per-declarator loop (globals; base type persists) */
  skipptr();                                                                  /* *g (any depth; skipptr resets nstar) */
  if (src[cp]==40 && src[cp+1]==42) {                                          /* R152: RET (*NAME)(params) GLOBAL fn pointer -> scalar 8-byte. Was: read_ident on '(' -> empty name -> `g` never recorded as a var -> `g=h; g()` had no lvalue (mis-stored) and emitted a bogus direct `call g` (no symbol). The local (1350) + typedef (214) forms were handled; parse_global was not. The calli indirect-call path already exists -- this just makes `g` a known var. */
    cp=cp+2; read_ident(); noff=id_off; nlen=id_len; skipws();
    if (src[cp]==91) {                                                          /* R155: RET (*NAME[N])(params) -- GLOBAL fn-pointer ARRAY. R152 only did the scalar `(*g)()`; the `[N]` tail was left unconsumed -> tbl mis-sized (1 slot) + mis-typed (scalar) -> tbl[i]=/tbl[i]() took ptr-deref branches -> SEGV. Mirror the g[N] sizing (ccc.c ~1837). */
      cp=cp+1; skipws(); n=0; if (src[cp]!=93) { n=const_expr(); }
      skipws(); cp=cp+1; skipws();                                             /* ] */
      if (src[cp]==41) { cp=cp+1; } skipws(); if (src[cp]==40) { skipparens(); }  /* ) (params) */
      if (n==0) { n=1; }
      dvbytes=n*8; nstar=1;                                                    /* N 8-byte ptr elements; nstar->var_ptr=1 so tbl[i] strides by 8 and tbl[i]() routes to R153 calli */
      addr=7340032+gcount*8; gcount=gcount+n; rec_var(noff, nlen, 1, 1, addr);  /* array-of-pointer global (arr=1) */
      if (dextern==0) { emit_gdef(noff, nlen, n*8); } }
    else {
      if (src[cp]==41) { cp=cp+1; } skipws(); if (src[cp]==40) { skipparens(); }  /* ) (params) */
      addr=7340032+gcount*8; gcount=gcount+1; rec_var(noff, nlen, 1, 0, addr);   /* 8-byte pointer scalar global (R152) */
      if (dextern==0) { emit_gdef(noff, nlen, 8); } } }
  else {
  w=4; if (dptr || nstar>0) { w=8; } else if (dchar) { w=1; } else if (dlong) { w=8; }  /* element width (R47: pointer BEFORE char -- char *x[] is 8-byte elements) */
  read_ident(); noff=id_off; nlen=id_len; skipws();
  if (src[cp]==91) {                                                            /* g[N] */
    cp=cp+1; skipws(); n=0;
    if (src[cp]!=93) { n=const_expr(); }                                        /* R45: const-expr size (e.g. isidnum_table[256 - (-1)]); empty [] -> 0 (init-sized) */
    skipws(); cp=cp+1; skipws();                                               /* ] */
    if (src[cp]==61) { cp=cp+1; skipws();
      if (dchar && src[cp]==34) { cnt=parse_str_init(noff, nlen); if (n==0) { n=cnt; } }  /* = "..." (string-literal char[]) */
      else { cnt=parse_init(noff, nlen, w); if (n==0) { n=cnt; } } }               /* = { ... } */
    if (n==0) { n=1; }
    dvbytes=n*w;                                                               /* sizeof(g) = element count * element width */
    if (w==1) { n=(n+7)/8; }                                                   /* byte elements: ceil(N/8) byte-packed slots (R47: keyed on w, not dchar -- char *x[] has w=8) */
    addr=7340032+gcount*8; gcount=gcount+n; rec_var(noff, nlen, 1, 1, addr);
    if (dextern==0) { emit_gdef(noff, nlen, n*8); } }
  else {
    addr=7340032+gcount*8; gcount=gcount+1; rec_var(noff, nlen, 1, 0, addr);    /* scalar global */
    if (dextern==0) { emit_gdef(noff, nlen, 8); }
    skipws();
    if (src[cp]==61) { cp=cp+1; parse_init(noff, nlen, w); } } }                /* scalar = expr (extra } closes the R152 fn-ptr else) */
  skipws();
  if (src[cp]==44) { cp=cp+1; }                                               /* ',' -> next declarator */
  else { cp=cp+1; return 0; } } }                                            /* ';' -> done */

int isentry(int i) {                                                           /* fname[i] == the entry ? */
  if (fname_len[i]!=4) { return 0; }
  if (src[fname_off[i]]==109 && src[fname_off[i]+1]==97 && src[fname_off[i]+2]==105 && src[fname_off[i]+3]==110) { return 1; }
  return 0; }
int skip_funcbody() { int depth;                                               /* cp at a fn DEF: advance past its body */
  while (cp<slen && src[cp]!=123) { cp=cp+1; }                                 /* to '{' (params have no '{') */
  if (cp>=slen) { return 0; }
  depth=1; cp=cp+1;
  while (cp<slen && depth>0) {
    if (src[cp]==47 && src[cp+1]==42) { cp=cp+2; while (cp<slen && !(src[cp]==42 && src[cp+1]==47)) { cp=cp+1; } cp=cp+2; }
    else if (src[cp]==34) { cp=cp+1; while (cp<slen && src[cp]!=34) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }
    else if (src[cp]==39) { cp=cp+1; while (cp<slen && src[cp]!=39) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }
    else { if (src[cp]==123) { depth=depth+1; } if (src[cp]==125) { depth=depth-1; } cp=cp+1; } }
  return 0; }
int prescan() { int depth; int start; int noff; int nlen; int pv;                /* record each fn def's name + start */
  skipws();
  while (cp<slen) {
    skipws(); if (cp>=slen) { return 0; }
    pv=cp;                                                                       /* progress guard */
    if (tok_is("typedef")) { cp=tok_is("typedef");                              /* skip a typedef decl (brace-aware): not a fn */
      while (cp<slen && src[cp]!=59) { if (src[cp]==123) { skip_braces(); } else { cp=cp+1; } }
      if (cp<slen && src[cp]==59) { cp=cp+1; } if (cp<=pv) { cp=cp+1; } skipws(); }
    else {
    start=cp; decl_type(); skipptr();                                           /* return type [*] */
    read_ident(); noff=id_off; nlen=id_len;
    while (cp<slen && src[cp]!=123 && src[cp]!=59 && src[cp]!=61) { cp=cp+1; }  /* to '{' (def), ';' (proto), or '=' (init global) */
    if (src[cp]==61) {                                                         /* initialized global (maybe brace-list) -- not a fn */
      while (cp<slen && src[cp]!=59 && src[cp]!=123) {                          /* string/char-aware: a ';' or '{' inside "..."/'.' is not the init end */
        if (src[cp]==34) { cp=cp+1; while (cp<slen && src[cp]!=34) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } }
        else if (src[cp]==39) { cp=cp+1; while (cp<slen && src[cp]!=39) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } }
        cp=cp+1; }
      if (cp<slen && src[cp]==123) { skip_braces(); }                          /* {..} (skip_braces also eats trailing ';') */
      else if (cp<slen && src[cp]==59) { cp=cp+1; } }                          /* scalar init ';' */
    else if (src[cp]==59) { cp=cp+1;                                            /* prototype -- don't emit (no body), but record its return-type for call-result width */
      if (nlen>0 && pnfunc<1024) { pname_off[pnfunc]=noff; pname_len[pnfunc]=nlen; pn_flng[pnfunc]=(dlong && nstar==0); pnfunc=pnfunc+1; } }
    else if (dstk && nlen==0 && src[cp]==123) { parse_struct_def(); }          /* struct def among fns -> register, not a phantom fn */
    else {
    fpos[nfunc]=start; fname_off[nfunc]=noff; fname_len[nfunc]=nlen; fn_flng[nfunc]=(dlong && nstar==0); fn_rsptr[nfunc]=0-1; if (nstar==1 && dstruct>=0) { fn_rsptr[nfunc]=dstruct; }  /* R143: `Struct *f()` -> record the return struct so `f()->field` can resolve */ nfunc=nfunc+1;
    depth=1; cp=cp+1;                                                          /* past '{', match (skipping comments/strings/chars) */
    while (cp<slen && depth>0) {
      if (src[cp]==47 && src[cp+1]==42) { cp=cp+2; while (cp<slen && !(src[cp]==42 && src[cp+1]==47)) { cp=cp+1; } cp=cp+2; }  /* comment */
      else if (src[cp]==34) { cp=cp+1; while (cp<slen && src[cp]!=34) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }    /* string */
      else if (src[cp]==39) { cp=cp+1; while (cp<slen && src[cp]!=39) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }    /* char */
      else { if (src[cp]==123) { depth=depth+1; } if (src[cp]==125) { depth=depth-1; } cp=cp+1; } } }
    if (cp<=pv) { cp=cp+1; } } }                                                /* never stall (close the non-typedef branch) */
  return 0; }
int emit_func(int ism) { int pc; int i; int variadic; int slot; int cv; int d;  /* emit one function body */
  decl_type(); skipptr();                                                      /* return type [*] */
  read_ident();                                                                /* the function name */
  if (ism) { es("ir ma"); es("in -> i32\n"); }
  else { es("fn "); i=0; while (i<id_len) { out[outn]=src[id_off+i]; outn=outn+1; i=i+1; } es("\n"); }
  nvar=nglob;                                                                  /* scope = globals only */
  skipws(); cp=cp+1; pc=0; variadic=0; cur_nparams=0; cur_va_base=0; slot=0; skipws(); /* '(' then params; R158: slot = long-param bitmask */
  while (cp<slen && src[cp]!=41) {
    if (src[cp]==46 && src[cp+1]==46 && src[cp+2]==46) {                        /* '...' variadic: spill remaining arg regs */
      cp=cp+3; variadic=1; cur_nparams=pc; while (pc<6) { es("  v"); en(pc); es(": i32 = param\n"); pc=pc+1; }
      skipws(); }
    else {
      decl_type(); skipptr(); skipws();                                        /* param type [*]; dchar/dstruct via decl_type */
      if (src[cp]==41) { } else {                                             /* (void) / no name -> loop exits */
        if (src[cp]==40) {                                                    /* RET (*name)(args) function-pointer param -> a pointer */
          cp=cp+1; skipws(); skipptr();                                       /* '(' then '*' (sets nstar) */
          read_ident(); skipws();
          if (src[cp]==41) { cp=cp+1; } skipws();                             /* ')' */
          if (src[cp]==40) { d=1; cp=cp+1; while (cp<slen && d>0) { if (src[cp]==40) { d=d+1; } if (src[cp]==41) { d=d-1; } cp=cp+1; } } }  /* skip (args) */
        else {
          read_ident(); skipws();                                            /* param name (struct type from decl_type) */
          while (src[cp]==91) { while (cp<slen && src[cp]!=93) { cp=cp+1; }   /* T name[..] param decays to pointer */
            if (src[cp]==93) { cp=cp+1; } nstar=nstar+1; skipws(); } }
        rec_var(id_off, id_len, 0, 0, pc);
        es("  v"); en(pc); es(": i32 = param\n");
        if (dlong && nstar==0 && dptr==0) { slot=slot + (1<<pc); }               /* R158: genuine `long`/`unsigned long` scalar param -> mark for is64 (preserve the full 64-bit arg; NOT pointers -- those stay 32-bit per the value model) */
        pc=pc+1; }
      skipws(); if (src[cp]==44) { cp=cp+1; skipws(); } } }                    /* ',' */
  cp=cp+1;                                                                      /* ')' */
  if (ncount>gmax) { gmax=ncount; } gzi=0; while (gzi<gmax) { is64[gzi]=0; ishi[gzi]=0; isflt[gzi]=0; isneg[gzi]=0; gzi=gzi+1; }  /* R91: clear is64/ishi for the fresh function's value range (vN resets per function). R160/c99-R3: ALSO isflt -- else a float value's v-number in one fn (e.g. main's `a*b`) mis-tags the SAME v-number (a u64 param `sig`) in the next fn (softfloat's round_pack_d) -> a spurious `__eqdf2`. */
  i=0; while (i<pc) { if ((slot>>i)&1) { is64[i]=1; } i=i+1; }                    /* R158: re-mark long params is64 AFTER the clear -- cf loads the full 64-bit arg register, and consumers (emit_binop8) skip the truncating conv */
  czero=pc; es("  v"); en(pc); es(": i32 = const 0\n"); ncount=pc+1;
  if (variadic) {                                                              /* copy the 6 arg regs into __va_area[6] (8-byte slots) */
    cur_va_base=ncount; ncount=ncount+6; i=0;
    while (i<6) { slot=emit_binop(4, emit_ref(cur_va_base), emit_const(i*8));   /* &__va_area[i] */
      cv=emit_conv(i);                                                          /* (i64)param_i -- emit BEFORE the store8 line */
      es("  store8 v"); en(slot); es(" v"); en(cv); es("\n"); i=i+1; } }        /* __va_area[i] = (i64)param_i */
  if (ism) { emit_init(); }                                                     /* run recorded global initializers at entry */
  skipws(); cp=cp+1; skipws();                                                  /* '{' */
  while (cp<slen && src[cp]!=125) { parse_stmt(); skipws(); }                   /* body until '}' */
  cp=cp+1;
  es("  ret v"); en(czero); es("\n");
  if (ism==0) { es("end\n"); }
  return 0; }
int prescan_typedefs() {                                                        /* R54: register typedef ALIASES before struct layout, so `CType type;` fields resolve fld_struct */
  cp=0;
  while (cp<slen) {
    if (src[cp]==34) { cp=cp+1; while (cp<slen && src[cp]!=34) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }
    else if (src[cp]==39) { cp=cp+1; while (cp<slen && src[cp]!=39) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }
    else if (src[cp]==47 && src[cp+1]==42) { cp=cp+2; while (cp<slen && !(src[cp]==42 && src[cp+1]==47)) { cp=cp+1; } cp=cp+2; }
    else if (src[cp]==116 && src[cp+1]==121 && src[cp+2]==112 && src[cp+3]==101 && src[cp+4]==100 && src[cp+5]==101 && src[cp+6]==102
             && (src[cp+7]==32 || src[cp+7]==10 || src[cp+7]==9) && (cp==0 || isalnum_(src[cp-1])==0)) {   /* 'typedef' at a token boundary */
      cp=cp+7; skipws(); parse_typedef(); }
    else { cp=cp+1; } }
  return 0; }
int resolve_fwd_fields() { int i; int si;                                       /* R72: resolve struct/struct-ptr fields whose struct was a FORWARD ref at layout time */
  i=0; while (i<nfld) {
    if (fld_struct[i]<0 && fld_slen[i]>0) { si=find_struct(fld_soff[i], fld_slen[i]); if (si>=0) { fld_struct[i]=si; } }
    i=i+1; }
  return 0; }
int prescan_structs() { int save;                                              /* register ALL struct defs first, so forward refs resolve */
  cp=0;
  while (cp<slen) {
    if (src[cp]==34) { cp=cp+1; while (cp<slen && src[cp]!=34) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }        /* string */
    else if (src[cp]==39) { cp=cp+1; while (cp<slen && src[cp]!=39) { if (src[cp]==92) { cp=cp+1; } cp=cp+1; } cp=cp+1; }   /* char */
    else if (src[cp]==47 && src[cp+1]==42) { cp=cp+2; while (cp<slen && !(src[cp]==42 && src[cp+1]==47)) { cp=cp+1; } cp=cp+2; }  /* comment */
    else if (src[cp]==115 && src[cp+1]==116 && src[cp+2]==114 && src[cp+3]==117 && src[cp+4]==99 && src[cp+5]==116 && src[cp+6]==32
             && (cp==0 || isalnum_(src[cp-1])==0)) {                            /* 'struct ' at a token boundary */
      save=cp; cp=cp+6; skipws(); read_ident(); dstruct_off=id_off; dstruct_len=id_len; skipws();
      if (src[cp]==123) { if (dstruct_len>0) { dstk=1; duni=0; parse_struct_def(); } else { skip_braces(); } } else { cp=save+6; } }  /* tagged def -> register; anon -> skip (registered at its typedef); use -> continue */
    else if (src[cp]==117 && src[cp+1]==110 && src[cp+2]==105 && src[cp+3]==111 && src[cp+4]==110 && src[cp+5]==32
             && (cp==0 || isalnum_(src[cp-1])==0)) {                            /* R44: 'union ' at a token boundary */
      save=cp; cp=cp+5; skipws(); read_ident(); dstruct_off=id_off; dstruct_len=id_len; skipws();
      if (src[cp]==123) { if (dstruct_len>0) { dstk=1; duni=1; parse_struct_def(); } else { skip_braces(); } } else { cp=save+5; } }
    else { cp=cp+1; } }
  return 0; }
int main() { int i;
  slen=read_all(); cp=0; ncount=1; nvar=0; gcount=0; nfunc=0; nstruct=0; nfld=0; dstruct=0-1; dstk=0; ntd=0; dptr=0; dlong=0;
  prescan_typedefs();                                                          /* R54: pass 0a -- typedef aliases (so struct fields typed by alias resolve) */
  prescan_structs();                                                           /* pass 0: all struct layouts (forward-ref safe) */
  resolve_fwd_fields();                                                        /* R72: fix up forward-ref struct-ptr fields (CType.ref -> Sym etc.) */
  nenm=0; prescan_enums();                                                     /* pass 0b: all enum constants (forward-ref safe) */
  cp=0; skipws();
  while (cp<slen) {                                                            /* register ALL top-level globals (before AND after fns) */
    skipws(); if (cp>=slen) { i=cp; }                                           /* (i reused as progress guard below) */
    if (cp>=slen) { cp=slen; }
    else if (at_funcdef()) { skip_funcbody(); }                                 /* skip fn bodies -- their locals are not globals */
    else if (at_decl()) { i=cp; parse_global(); skipws(); if (cp<=i) { cp=cp+1; } }
    else { cp=cp+1; } }                                                          /* stray token -> advance */
  nglob=nvar;
  cp=0; skipws();                                                              /* rewind: prescan/emit re-scan from the top */
  prescan();
  i=0; while (i<nfunc) { if (isentry(i)) { cp=fpos[i]; emit_func(1); } i=i+1; }      /* entry first */
  i=0; while (i<nfunc) { if (isentry(i)==0) { cp=fpos[i]; emit_func(0); } i=i+1; }   /* then the rest */
  emit_ginit_bodies();                                                              /* R121: __ginitN() bodies (only if the init was split) */
  __syscall(1, 1, out, outn); return 0; }
