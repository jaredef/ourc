/* rzrs.c -- Rust-subset -> rzir front-end (rust-to-rzir arc). Reads Rust source on stdin, emits
 * rzir text on stdout, lowered by `rzir-ld | rzir-opt | rzir-ra | cf` to a native ELF -- so Rust
 * reaches codegen entirely through the tower's own backend. Built by the derived tcc (a peer of ccc).
 *
 * R0 pipeline; R1 computational core; R2 memory + aggregates:
 *   struct Name { f: T, ... }; ; struct literal `Name { f: e, .. }` ; field `p.f` r/w ;
 *   arrays `[e, ..]` / `[e; N]`, index `a[i]` r/w ; references `&x` / `&mut x` / `*p` ;
 *   `&Struct` params with auto-deref field access.
 * Backend memory model (mirrors ccc): every vN lives at [rbx + N*8]; an aggregate reserves
 * ceil(size/8) slots and is addressed by `ref BASE`; field/element access = `add addr off` then
 * load/store (off = field byte-offset ; index = i << log2(elemsize)); &x = `ref x_vN`.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static char src[1 << 20];
static int pos, srclen;
static int ncount, czero, lblctr;

/* variables (per function): span -> vN, kind, struct-type */
enum { K_SCALAR, K_STRUCT, K_ARRAY, K_PTRSTRUCT, K_ENUMVAL, K_STR, K_VEC, K_STRING, K_CLOSURE };  /* ... | Closure (vN holds a heap {code@0, env@4} fat pointer; called via calli) */
static int voff[8192], vlen[8192], vvn[8192], vkind[8192], vstyp[8192], vesz[8192], vmoved[8192], nsym;   /* vmoved: R4 use-after-move flag (move types = struct/array/data-enum; scalars/refs are Copy) */
/* functions (module pre-pass) */
static int foff[2048], flen[2048], fret64[2048], fretchar[2048], fretstr[2048], fretenum[2048], fretstring[2048], nfn;   /* R1b/R2/R4/R2b: per-fn return-type tags (i64 / char / &str / data-enum idx / String) */
static int prelude_end;             /* R2-linux: end offset of prepended prelude in src[]; any fn with foff<prelude_end is a prelude fn */
static int fn_needed[2048];         /* R2-linux: 1 iff this fn (indexed as in foff[]) is transitively referenced by user code; non-prelude fns always 1 */
static int opt_en=-1, res_en=-1;   /* R4: built-in Option/Result enum indices (declared in the prelude) */
static int loop_depth=0;           /* R4: inside a while/loop body a trailing expr is DISCARDED (unit), never a fn tail-return */
/* struct types + fields */
static int soff[256], slen[256], ssize[256], nstruct;
static int m_toff[512], m_tlen[512], m_moff[512], m_mlen[512], m_retstyp[512], nmeth;   /* R7: impl-method return-type registry (retstyp = struct idx if the method returns Self/a struct, else -1) */
/* selfhost P0: module-level statics -- `static [mut] NAME: TYPE = INIT;`. g_kind 0=scalar 1=array.
 * Each gets a `gdef NAME <bytes>` (zeroed); non-zero scalar inits are stored at main entry. */
static int g_off[1024], g_len[1024], g_kind[1024], g_esz[1024], g_bytes[1024], g_is64[1024], g_init[1024], nglob;
static int in_main_emit;   /* selfhost P0: set while emitting main's body so global inits are injected */
static int sf_st[4096], sf_off[4096], sf_len[4096], sf_boff[4096], sf_styp[4096], sf_fkind[4096], nsf;   /* P2: sf_fkind = a container field's kind (K_VEC/K_STRING/K_STR), else 0 */
/* enum types + variants. layout: C-like enum = a 4-byte discriminant; data enum = disc@0 + payload@8 (i32 fields). */
static int enoff[256], enlen[256], ensize[256], nenum;
static int ev_en[2048], ev_off[2048], ev_len[2048], ev_disc[2048], ev_nfld[2048], nev;
static int expr_enum;                                         /* enum idx of the just-parsed expr if it's a data-enum value, else -1 */
static int expr_is64;                                         /* rust-closure R1: the just-parsed expr is a genuine 64-bit (i64/u64) value */
static int vis64[8192];                                       /* per-variable i64 flag (parallel to vkind) */
static int expr_ischar; static int vischar[8192];             /* R1b: char values -> Display prints the CHARACTER (via __putc), not the number */
static int expr_isvec;                                        /* R3: this expression produced a Vec descriptor pointer */
static int expr_isstring;                                     /* R2b: this expression produced a heap String descriptor pointer */
static int expr_isclosure;                                    /* R6: this expression produced a closure fat pointer */
/* R6: closure registry -- each `|params| body` is lifted to a top-level fn `__closure_N(env, params...)`,
 * emitted deferred (like generic instantiations). Captures = enclosing locals referenced by name in the body. */
static int clo_boff[512], clo_blen[512], clo_isblock[512], clo_np[512], clo_nc[512];
static int clo_poff[512][8], clo_plen[512][8];
static int clo_caoff[512][16], clo_calen[512][16], clo_cakind[512][16], clo_castyp[512][16];
static int nclo;
static int expr_isstr;                                        /* R2: the just-parsed expr is a &str (a {ptr,len} descriptor pointer) */
/* generic functions (monomorphized): name span, first type-param name, and the source pos of `(` (sig start) */
static int gf_off[256], gf_len[256], gf_tpoff[256], gf_tplen[256], gf_sig[256], gf_tparg[256], ngf;   /* gf_tparg: index of the first param whose type is T */
static int gsub_off, gsub_len, gsub_styp, gsub_active;        /* active T->concrete-struct substitution while emitting an instantiation */
static int inst_gf[1024], inst_styp[1024], ninst;            /* (generic-fn index, concrete struct idx) instantiations to emit */
static int gf_find(int o,int l){ int i; for(i=0;i<ngf;i++) if(gf_len[i]==l&&strncmp(src+gf_off[i],src+o,l)==0) return i; return -1; }

static void die(const char *m){ fprintf(stderr, "rzrs: %s (offset %d)\n", m, pos); exit(1); }

static void skipws(void){
    for(;;){
        while(pos<srclen && (src[pos]==' '||src[pos]=='\t'||src[pos]=='\n'||src[pos]=='\r')) pos++;
        if(pos+1<srclen && src[pos]=='/'&&src[pos+1]=='/'){ while(pos<srclen&&src[pos]!='\n')pos++; continue; }
        if(pos+1<srclen && src[pos]=='/'&&src[pos+1]=='*'){ pos+=2; while(pos+1<srclen&&!(src[pos]=='*'&&src[pos+1]=='/'))pos++; pos+=2; continue; }   /* selfhost P0: block comments */
        break;
    }
}
static int isidst(int c){ return (c>='a'&&c<='z')||(c>='A'&&c<='Z')||c=='_'; }
static int isidc(int c){ return isidst(c)||(c>='0'&&c<='9'); }
static int rd_ident(int *o,int *l){ skipws(); if(pos>=srclen||!isidst(src[pos]))return 0; *o=pos; while(pos<srclen&&isidc(src[pos]))pos++; *l=pos-*o; return 1; }
/* selfhost P0: advance past ws+comments then ONE token (string/char literal, whole ident, or punct).
 * Top-level scan loops must use this instead of `pos++` so keywords INSIDE a comment or string
 * literal aren't matched (e.g. `fn main` written in a doc-comment). */
static void skip_tok(void){ skipws(); if(pos>=srclen)return;
    if(src[pos]=='"'){ pos++; while(pos<srclen&&src[pos]!='"'){ if(src[pos]=='\\')pos++; pos++; } if(pos<srclen)pos++; return; }
    if(src[pos]=='\''){ if(pos+2<srclen && isidst(src[pos+1]) && src[pos+2]!='\''){ pos++; while(pos<srclen&&isidc(src[pos]))pos++; return; }   /* lifetime 'ident (NOT a char literal -- no closing quote) */
        pos++; while(pos<srclen&&src[pos]!='\''){ if(src[pos]=='\\')pos++; pos++; } if(pos<srclen)pos++; return; }
    if(isidst(src[pos])){ while(pos<srclen&&isidc(src[pos]))pos++; return; }
    pos++;
}
static int at_kw(const char *kw){ int o,l,p=pos; if(!rd_ident(&o,&l)){pos=p;return 0;} pos=p; return l==(int)strlen(kw)&&strncmp(src+o,kw,l)==0; }
static int eat_kw(const char *kw){ if(at_kw(kw)){int o,l; rd_ident(&o,&l); return 1;} return 0; }
static int eat(const char *s){ int n,i; skipws(); n=(int)strlen(s); for(i=0;i<n;i++) if(pos+i>=srclen||src[pos+i]!=s[i]) return 0; pos+=n; return 1; }
static int span_eq(int o,int l,const char*s){ return l==(int)strlen(s)&&strncmp(src+o,s,l)==0; }

static void sym_add(int o,int l,int vn,int kind,int styp,int esz){ voff[nsym]=o;vlen[nsym]=l;vvn[nsym]=vn;vkind[nsym]=kind;vstyp[nsym]=styp;vesz[nsym]=esz;vmoved[nsym]=0;vis64[nsym]=0;vischar[nsym]=0;nsym++; }
static void chk_live(int si){ if(vmoved[si]){ fprintf(stderr,"rzrs: error: use of moved value: `"); fwrite(src+voff[si],1,vlen[si],stderr); fprintf(stderr,"`\n"); exit(1); } }
static int sym_find(int o,int l){ int i; for(i=nsym-1;i>=0;i--) if(vlen[i]==l&&strncmp(src+voff[i],src+o,l)==0) return i; return -1; }
static int fn_has(int o,int l){ int i; for(i=0;i<nfn;i++) if(flen[i]==l&&strncmp(src+foff[i],src+o,l)==0) return 1; return 0; }
static int fn_index(int o,int l){ int i; for(i=0;i<nfn;i++) if(flen[i]==l&&strncmp(src+foff[i],src+o,l)==0) return i; return -1; }
static int struct_find(int o,int l){ int i; for(i=0;i<nstruct;i++) if(slen[i]==l&&strncmp(src+soff[i],src+o,l)==0) return i; return -1; }
static int meth_retstyp(int to,int tl,int mo,int ml){ int i; for(i=0;i<nmeth;i++) if(m_tlen[i]==tl&&!strncmp(src+m_toff[i],src+to,tl)&&m_mlen[i]==ml&&!strncmp(src+m_moff[i],src+mo,ml)) return m_retstyp[i]; return -1; }   /* R7: struct idx a method returns, else -1 */
static int g_find(int o,int l){ int i; for(i=0;i<nglob;i++) if(g_len[i]==l&&strncmp(src+g_off[i],src+o,l)==0) return i; return -1; }   /* selfhost P0: module-level static */
static int fld_fkind;   /* P2: kind of the field last resolved by fld_find (K_VEC/K_STRING/K_STR or 0) */
static int fld_find(int st,int o,int l,int *boff,int *fstyp){ int i; for(i=0;i<nsf;i++) if(sf_st[i]==st&&sf_len[i]==l&&strncmp(src+sf_off[i],src+o,l)==0){ *boff=sf_boff[i]; *fstyp=sf_styp[i]; fld_fkind=sf_fkind[i]; return 1;} return 0; }
static int enum_find(int o,int l){ int i; for(i=0;i<nenum;i++) if(enlen[i]==l&&strncmp(src+enoff[i],src+o,l)==0) return i; return -1; }
static int evar_find(int en,int o,int l,int *disc,int *nf){ int i; for(i=0;i<nev;i++) if(ev_en[i]==en&&ev_len[i]==l&&strncmp(src+ev_off[i],src+o,l)==0){ *disc=ev_disc[i]; *nf=ev_nfld[i]; return 1;} return 0; }

/* T48 alphabet purity: cf-darwin's const parser has no '-' arm -- negative i32 consts must
 * reach it FOLDED TO U32 (const 4294967295 = -1; cf's bit31-set arm re-sign-extends, so every
 * consumer sees the same value).  The x86 cf accepts the signed spelling, so the Linux lane's
 * output stays byte-identical (%d as before). */
static int tgt_darwin=0;
static int emit_const(int v){ expr_is64=0; expr_ischar=0;
    if(tgt_darwin&&v<0) printf("  v%d: i32 = const %u\n",ncount,(unsigned)v);
    else printf("  v%d: i32 = const %d\n",ncount,v);
    return ncount++; }
static int emit_binop(const char*op,int a,int b){ expr_enum=-1; expr_is64=0; expr_ischar=0; printf("  v%d: i32 = %s v%d v%d\n",ncount,op,a,b); return ncount++; }
static int emit_conv(int v){ printf("  v%d: i64 = conv v%d\n",ncount,v); return ncount++; }              /* zext i32 -> i64 */
static int emit_binop8(const char*op,int a,int b){ expr_enum=-1; printf("  v%d: i64 = %s v%d v%d\n",ncount,op,a,b); return ncount++; }
/* a 64-bit constant: low32 via const (may wrap negative), zext; high half built with shl8/or8 if needed */
static int emit_i64_const(long v){ unsigned lo=(unsigned)(v&0xFFFFFFFFL), hi=(unsigned)((v>>32)&0xFFFFFFFFL); int vlo;
    vlo=emit_conv(emit_const((int)lo));
    if(hi==0){ expr_is64=1; return vlo; }
    { int vhi=emit_conv(emit_const((int)hi)), v32=emit_conv(emit_const(32)), vsh=emit_binop8("shl8",vhi,v32), r=emit_binop8("or8",vsh,vlo); expr_is64=1; return r; }
}
/* combine two operands with an op that has i32 (op32) and i64 (op8) forms; promotes to i64 if either side is */
static int emit_arith(const char*op32,const char*op8,int a,int ia,int b,int ib,int cmp){
    if(ia||ib){ if(!ia)a=emit_conv(a); if(!ib)b=emit_conv(b); { int r=emit_binop8(op8,a,b); expr_is64=cmp?0:1; return r; } }
    return emit_binop(op32,a,b);   /* i32 (sets expr_is64=0) */
}
static int emit_ref(int vn){ printf("  v%d: i32 = ref v%d\n",ncount,vn); return ncount++; }
static int emit_load(int addr){ printf("  v%d: i32 = load v%d\n",ncount,addr); return ncount++; }
static int emit_load8(int addr){ printf("  v%d: i64 = load8 v%d\n",ncount,addr); return ncount++; }
static int emit_gref(const char*name){ printf("  v%d: i32 = gref %s\n",ncount,name); return ncount++; }
static int emit_gref_sp(int off,int len){ printf("  v%d: i32 = gref ",ncount); fwrite(src+off,1,len,stdout); printf("\n"); return ncount++; }   /* selfhost P0: gref a static by its name span */
static int emit_addr_add(int a,int b);   /* T39 helpers (defined below emit_store8) */
static int emit_pload(int addr);
static void emit_pstore(int addr,int val);
/* build a string literal in the GLOBAL arena (survives function returns, unlike a stack buffer):
   bytes at arena[cursor], then a {ptr@0,len@8} descriptor; bump the cursor; return the descriptor pointer. */
static int emit_str_literal(int so,int sl){
    int bytes[8192],n=0,k=so,e=so+sl,cur,base,i,doff,desc,co,ad,cc,newcur;
    while(k<e){ int c=(unsigned char)src[k]; if(c=='\\'){ k++; c=(unsigned char)src[k]; if(c=='n')c=10; else if(c=='t')c=9; else if(c=='r')c=13; else if(c=='0')c=0; else if(c=='\\')c=92; else if(c=='"')c=34; } bytes[n++]=c; k++; }
    cur=emit_load(emit_gref("__str_cursor"));                             /* current arena offset */
    base=emit_addr_add(emit_gref("__str_arena"),cur);                 /* &arena[cursor] */
    for(i=0;i<n;i++){ co=emit_const(i); ad=emit_addr_add(base,co); cc=emit_const(bytes[i]); printf("  storeb v%d v%d\n",ad,cc); }
    doff=(n+7)&~7; desc=emit_addr_add(base,emit_const(doff));         /* descriptor after the (aligned) bytes */
    { int d1=desc,b1=base; printf("  store8 v%d v%d\n",d1,b1); }          /* desc.ptr = base */
    { int d2=emit_addr_add(desc,emit_const(8)),ln=emit_const(n); printf("  store8 v%d v%d\n",d2,ln); }   /* desc.len = n */
    newcur=emit_binop("add",cur,emit_const(doff+16)); { int gc=emit_gref("__str_cursor"); printf("  store v%d v%d\n",gc,newcur); }
    expr_isstr=1; return desc;
}
static void emit_store(int addr,int val){ printf("  store v%d v%d\n",addr,val); }
static void emit_store8(int addr,int val){ printf("  store8 v%d v%d\n",addr,val); }
/* T39 (darwin-ptr-width): on the Darwin lane addresses live above 2^32 (image base 0x100400000),
 * so ADDRESS-forming adds must be x-form (add8) and pointer-holding memory cells are 8-byte
 * (load8/store8, 8-byte strides).  On Linux every helper degrades to the exact historic w-form
 * emission, so the default output stays byte-identical.  PW = the pointer-cell stride. */
#define PW (tgt_darwin?8:4)
static int emit_addr_add(int a,int b){ if(tgt_darwin){ expr_enum=-1; expr_is64=0; expr_ischar=0; printf("  v%d: i64 = add8 v%d v%d\n",ncount,a,b); return ncount++; } return emit_binop("add",a,b); }
static int emit_pload(int addr){ if(tgt_darwin){ printf("  v%d: i64 = load8 v%d\n",ncount,addr); return ncount++; } return emit_load(addr); }
static void emit_pstore(int addr,int val){ if(tgt_darwin){ printf("  store8 v%d v%d\n",addr,val); return; } emit_store(addr,val); }
static int emit_loadb(int addr){ int z=emit_const(0); printf("  v%d: i32 = getb v%d v%d\n",ncount,addr,z); return ncount++; }   /* selfhost P0/R2-linux: u8 read via cf-arm64 W16 kw_getb (indexed with offset 0); rewrites loadb -> getb for cross-lane portability */
static void emit_storeb(int addr,int val){ printf("  storeb v%d v%d\n",addr,val); }
static void copy_to(int dest,int val){ printf("  v%d: i32 = add v%d v%d\n",dest,val,czero); }
/* T39: x-form copy for values that may be 64-bit ADDRESSES (match arms carrying enum bases /
 * descriptors).  NOT used for loop slots: rzir-opt keys its multi-def detection on the w-form
 * `add vN czero` idiom, and an add8 loop slot gets its uses hoisted out of the loop (observed:
 * infinite for-loop).  Match bodies have no hoist, and add8 of zext i32 is low32-exact. */
static void copy_to_x(int dest,int val){ if(tgt_darwin){ printf("  v%d: i64 = add8 v%d v%d\n",dest,val,czero); return; } copy_to(dest,val); }
static void copy_to64(int dest,int val){ printf("  v%d: i64 = or8 v%d v%d\n",dest,val,val); }   /* R1: 64-bit copy into a slot (or8 x x = x) */
static int emit_call1(const char*fn,int a){ printf("  v%d: i32 = call %s v%d\n",ncount,fn,a); return ncount++; }
static int as_bool(int v){ return emit_binop("ne",v,czero); }
static int not_bool(int v){ return emit_binop("eq",v,czero); }
static int reserve_slots(int size){ int base=ncount, slots=(size+7)/8, i; printf("  v%d: i32 = const 0\n",base); ncount++; for(i=1;i<slots;i++) ncount++; return base; }
/* R4: bump-allocate `nbytes` from the heap arena -> pointer vN (like __alloc, but callable from C).
 * Option/Result aggregates go here so a `Some(v)`/`Err(e)` RETURNED from a fn survives the callee frame. */
static int emit_heap_alloc(int nbytes){ int cur=emit_load(emit_gref("__heap_cursor")), base=emit_addr_add(emit_gref("__heap"),cur), nc=emit_binop("add",cur,emit_const(nbytes)), gc=emit_gref("__heap_cursor"); printf("  store v%d v%d\n",gc,nc); return base; }
/* R4: `expr?` -- expr is an Option/Result aggregate (base addr). If success (Some=1 / Ok=0), the value
 * is the payload; else RETURN the whole enum from the enclosing fn (None/Err propagates). */
static int emit_try(int base,int en){ int okdisc=(en==res_en)?0:1, disc=emit_load(base), cf=emit_binop("ne",disc,emit_const(okdisc));
    printf("  brz v%d\n  ret v%d\n  pool\n",cf,base); expr_enum=-1; { int co=emit_const(8),ad=emit_addr_add(base,co); return emit_load(ad); } }

static int parse_expr(void);
static int parse_print_macro(void);
static int parse_closure(void);
static int dispatch_vec(int dv);      /* P2: Vec method/index dispatch on a descriptor vN */
static int dispatch_string(int dv);   /* P2: String method dispatch on a descriptor vN */
static void parse_block(void);
static int peek_is(int c);
static int parse_atom(void);
static int parse_match(void);

/* place-expression state: parse_place() returns an ADDRESS vN and sets these to the place's type */
static int place_styp, place_kind, place_esz, place_fkind;   /* place_fkind: P2 container-field kind (K_VEC/K_STRING/K_STR) when a field access stopped at one */

/* parse a place (lvalue) expression -> address vN; sets place_kind/place_styp/place_esz */
static int parse_place(void){
    int o,l,addr,si;
    skipws();
    if (eat("*")) { addr = parse_atom(); place_kind=K_SCALAR; place_styp=-1; place_esz=4; }   /* deref binds tightly: `*p + 20` = `(*p)+20` */
    else {
        if(!rd_ident(&o,&l)) die("expected a place");
        si=sym_find(o,l);
        if(si<0){ int gi=g_find(o,l); if(gi<0) die("undeclared variable in place");   /* selfhost P0: module-level static place -> gref base */
            addr=emit_gref_sp(g_off[gi],g_len[gi]); place_kind=K_SCALAR; place_styp=-1; place_esz=g_esz[gi]; goto place_suffix; }
        chk_live(si);   /* R4: reading/borrowing a moved value is an error */
        if (vkind[si]==K_PTRSTRUCT||vkind[si]==K_STRUCT) { addr=vvn[si]; place_kind=K_STRUCT; place_styp=vstyp[si]; place_esz=4; }  /* &struct param OR owned heap struct: vN IS the base address */
        else if (vkind[si]==K_VEC) { addr=vvn[si]; place_kind=K_VEC; place_styp=-1; place_esz=4; }  /* R3: Vec place = descriptor ptr; `&v` yields it directly, `v[i]` derefs to the element block */
        else if (vkind[si]==K_STRING) { addr=vvn[si]; place_kind=K_STRING; place_styp=-1; place_esz=4; }  /* R2b: String place = descriptor ptr; `&s` yields it */
        else { addr=emit_ref(vvn[si]); place_kind=vkind[si]; place_styp=vstyp[si]; place_esz=vesz[si]; }
    }
    place_suffix:
    place_fkind=0;
    for(;;){ skipws();
        if (eat(".")) { int fo,fl,boff,fstyp; if(!rd_ident(&fo,&fl)) die("expected field name");
            if(place_styp<0 || !fld_find(place_styp,fo,fl,&boff,&fstyp)) die("no such field");
            addr = emit_addr_add(addr, emit_const(boff));
            if(fld_fkind){ place_kind=K_SCALAR; place_styp=-1; place_fkind=fld_fkind; break; }   /* P2: a Vec/String/&str field -- stop; parse_atom loads the descriptor + dispatches its methods */
            if(fstyp>=0){ place_kind=K_STRUCT; place_styp=fstyp; } else { place_kind=K_SCALAR; place_styp=-1; }
        }
        else if (eat("[")) { int isvec=(place_kind==K_VEC),es=place_esz,iv; iv=parse_expr(); if(!eat("]")) die("expected `]`");   /* capture K_VEC/esz BEFORE parse_expr (a var index would reset place state) */
            if(isvec){ addr=emit_pload(addr); es=4; }                               /* Vec: element block lives at desc[0] */
            if(es==1) addr=emit_addr_add(addr, iv);                            /* u8 stride 1 */
            else addr = emit_addr_add(addr, emit_binop("shl", iv, emit_const(es==8?3:2)));   /* i32 *4 / i64 *8 */
            place_kind=K_SCALAR; place_styp=-1; place_esz=es;
        }
        else break;
    }
    return addr;
}

/* is the upcoming token a place followed (eventually) by `.`/`[`/nothing that is an lvalue read?
   We use a heuristic in parse_atom: an identifier followed by `.` or `[` is a place read (load). */

/* primary/atom (rvalue) */
static int parse_atom(void){
    int o,l,v,p;
    skipws();
    expr_enum=-1; expr_is64=0; expr_ischar=0; expr_isstr=0; expr_isvec=0; expr_isstring=0; expr_isclosure=0;
    if (at_kw("match")) return parse_match();
    if (at_kw("println")||at_kw("print")) return parse_print_macro();   /* R4: println! as a unit-valued expr (e.g. a match arm) */
    if (at_kw("move")){ eat_kw("move"); skipws(); }                     /* R6: `move` closures -- capture is by-value here regardless */
    if (src[pos]=='|') return parse_closure();   /* R6: |params| body (or || body) closure */
    if (src[pos]=='"'){ int so; pos++; so=pos; while(pos<srclen&&src[pos]!='"'){ if(src[pos]=='\\')pos++; pos++; } { int r=emit_str_literal(so,pos-so); pos++; return r; } }   /* R2: &str literal */
    if (src[pos]=='\''){ int cc; pos++; cc=(unsigned char)src[pos];       /* R1b: char literal 'c' -> code point, tagged char */
        if(cc=='\\'){ pos++; cc=(unsigned char)src[pos]; if(cc=='n')cc=10; else if(cc=='t')cc=9; else if(cc=='r')cc=13; else if(cc=='0')cc=0; else if(cc=='\\')cc=92; else if(cc=='\'')cc=39; }
        pos++; if(pos<srclen&&src[pos]=='\'')pos++; { int r=emit_const(cc); expr_ischar=1; return r; } }
    if (eat("(")) { v=parse_expr(); if(!eat(")")) die("expected `)`"); return v; }
    if (eat("&")) { eat_kw("mut"); { int a=parse_place(); return a; } }        /* &place / &mut place -> the address */
    if (peek_is('*')) { int a=parse_place(),lv=emit_load(a); expr_is64=0; expr_ischar=0; return lv; }   /* *place read -> load; a 4-byte load IS i32 (clear is64 leaked by add8 address arithmetic) */
    if (eat("-")) { v=parse_atom(); return emit_binop("sub",czero,v); }
    if (eat("!")) { v=parse_atom(); return not_bool(v); }
    if (pos<srclen && src[pos]>='0'&&src[pos]<='9') { long lv=0; int so,sl,i64suf=0; while(pos<srclen&&src[pos]>='0'&&src[pos]<='9'){lv=lv*10+(src[pos]-'0');pos++;}
        so=pos; while(pos<srclen&&isidc(src[pos]))pos++; sl=pos-so;                                    /* type suffix (i32/i64/u64/...) */
        if((sl==3&&(!strncmp(src+so,"i64",3)||!strncmp(src+so,"u64",3)))) i64suf=1;
        if(i64suf || lv>2147483647L) return emit_i64_const(lv);                                        /* R1: i64 literal (suffix or overflow) */
        return emit_const((int)lv); }
    p=pos;
    if (rd_ident(&o,&l)) {
        if(span_eq(o,l,"true")) return emit_const(1);
        if(span_eq(o,l,"false")) return emit_const(0);
        if(span_eq(o,l,"Vec")){ skipws(); if(src[pos]==':'&&src[pos+1]==':'){ int mo,ml; pos+=2; skipws();   /* R3: Vec::new() / Vec::with_capacity(n) -> empty heap Vec */
            if(src[pos]=='<'){ int d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } skipws(); if(src[pos]==':'&&src[pos+1]==':')pos+=2; skipws(); }   /* turbofish Vec::<T>:: */
            if(rd_ident(&mo,&ml)){ skipws(); if(src[pos]=='('){ pos++; skipws(); if(src[pos]!=')'){ parse_expr(); skipws(); } eat(")"); { int r; printf("  v%d: i32 = call __vec_new\n",ncount); r=ncount++; expr_isvec=1; return r; } } }
            die("R3: only Vec::new / Vec::with_capacity"); } }
        if(span_eq(o,l,"String")){ skipws(); if(src[pos]==':'&&src[pos+1]==':'){ int mo,ml; pos+=2; skipws();   /* R2b: String::new() / String::from(&str) */
            if(rd_ident(&mo,&ml)){ skipws(); if(src[pos]=='('){ pos++; skipws();
                if(span_eq(mo,ml,"new")){ eat(")"); { int r; printf("  v%d: i32 = call __str_new\n",ncount); r=ncount++; expr_isstring=1; return r; } }
                if(span_eq(mo,ml,"from")){ int a=parse_expr(); skipws(); eat(")"); { int r=emit_call1("__str_from",a); expr_isstr=0; expr_isstring=1; return r; } }   /* clear the inner literal's expr_isstr so `let` binds K_STRING */
                die("R2b: only String::new / String::from"); } }
            die("R2b: expected String::new/from"); } }
        if(span_eq(o,l,"vec")){ skipws(); if(src[pos]=='!'){ pos++; skipws(); if(src[pos]=='['){ int dvn; pos++;   /* R3: vec![a, b, c] macro */
            printf("  v%d: i32 = call __vec_new\n",ncount); dvn=ncount++;
            skipws(); if(src[pos]!=']'){ for(;;){ int e=parse_expr(); printf("  v%d: i32 = call __vec_push v%d v%d\n",ncount,dvn,e); ncount++; skipws(); if(eat(","))continue; break; } }
            eat("]"); expr_isvec=1; return dvn; } } }
        if(struct_find(o,l)>=0){ int save=pos; skipws(); if(src[pos]==':'&&src[pos+1]==':'){ int mo,ml; pos+=2; skipws();   /* R7: Type::assoc(args) associated fn -> `call Type_assoc args` */
            if(rd_ident(&mo,&ml)){ skipws(); if(src[pos]=='('){ int args[32],na=0,i,rs; pos++; skipws();
                if(src[pos]!=')'){ for(;;){ args[na++]=parse_expr(); skipws(); if(eat(","))continue; break; } } eat(")");
                printf("  v%d: i32 = call ",ncount); fwrite(src+o,1,l,stdout); printf("_"); fwrite(src+mo,1,ml,stdout); for(i=0;i<na;i++)printf(" v%d",args[i]); printf("\n");
                { int r=ncount++; rs=meth_retstyp(o,l,mo,ml); if(rs>=0){ place_kind=K_PTRSTRUCT; place_styp=rs; } return r; } } }
            die("R7: expected `Type::assoc(...)`"); } pos=save; }
        { int ben=-1; if(span_eq(o,l,"Some")||span_eq(o,l,"None"))ben=opt_en; else if(span_eq(o,l,"Ok")||span_eq(o,l,"Err"))ben=res_en;   /* R4: bare Some/None/Ok/Err (prelude-imported variants) -> heap Option/Result aggregate */
          if(ben>=0){ int disc=(span_eq(o,l,"Some")||span_eq(o,l,"Err"))?1:0, base=emit_heap_alloc(ensize[ben]), dc;   /* None=0 Some=1 ; Ok=0 Err=1 */
              dc=emit_const(disc); emit_store(base,dc);                                        /* disc @0 (base is the address) */
              skipws(); if(src[pos]=='('){ pos++; skipws(); if(src[pos]!=')'){ int av=parse_expr(),co=emit_const(8),ad=emit_addr_add(base,co); emit_store(ad,av); skipws(); } eat(")"); }
              expr_enum=ben; return base; } }
        { int en=enum_find(o,l); if(en>=0){ skipws(); if(src[pos]==':'&&src[pos+1]==':'){   /* E::Variant [(args)] */
            int vo,vl,disc=0,nf=0; pos+=2; if(!rd_ident(&vo,&vl)) die("expected enum variant"); evar_find(en,vo,vl,&disc,&nf); skipws();
            if(ensize[en]>4){                                     /* DATA enum -> always an aggregate (even a nullary variant); value = base address */
                int base=reserve_slots(ensize[en]), i, rb,dc;
                rb=emit_ref(base); dc=emit_const(disc); emit_store(rb,dc);   /* discriminant @0 */
                if(eat("(")){ for(i=0;;i++){ int av,ad,r2,co; skipws(); if(src[pos]==')'){pos++;break;} av=parse_expr(); r2=emit_ref(base); co=emit_const(8+i*4); ad=emit_addr_add(r2,co); emit_store(ad,av); skipws(); if(eat(","))continue; if(src[pos]==')'){pos++;break;} } }
                expr_enum=en; return emit_ref(base);
            }
            expr_enum=-1; return emit_const(disc);                /* C-like enum -> the discriminant directly (a scalar) */
        } } }
        skipws();
        if(src[pos]=='('){ int args[32],argsty[32],na=0,i,gi=gf_find(o,l),tstyp=-1; pos++; skipws();      /* call */
            if(src[pos]!=')'){ for(;;){ place_styp=-1; args[na]=parse_expr(); argsty[na]=place_styp; na++; skipws(); if(eat(","))continue; break; } }
            if(!eat(")")) die("expected `)` in call");
            { int si=sym_find(o,l); if(si>=0 && vkind[si]==K_CLOSURE){ int code=emit_pload(vvn[si]),co=emit_const(PW),ad=emit_addr_add(vvn[si],co),env=emit_pload(ad);   /* R6: call a closure -> calli code env args... */
                printf("  v%d: i32 = calli v%d v%d",ncount,code,env); for(i=0;i<na;i++)printf(" v%d",args[i]); printf("\n"); return ncount++; } }
            if(span_eq(o,l,"__syscall")){ printf("  v%d: i32 = syscall",ncount); for(i=0;i<na;i++)printf(" v%d",args[i]); printf("\n"); return ncount++; }  /* rust-closure R0: raw syscall intrinsic */
            if(span_eq(o,l,"__alloc")){ int cur=emit_load(emit_gref("__heap_cursor")), base=emit_addr_add(emit_gref("__heap"),cur), nc=emit_binop("add",cur,args[0]), gc=emit_gref("__heap_cursor"); printf("  store v%d v%d\n",gc,nc); expr_is64=tgt_darwin; return base; }  /* R3: bump-allocate args[0] bytes from the global heap arena -> pointer */
            if(span_eq(o,l,"__loadb")){ int z=emit_const(0); printf("  v%d: i32 = getb v%d v%d\n",ncount,args[0],z); return ncount++; }   /* R2b/R2-linux: __loadb(a) -> getb a 0 (W16 has indexed getb, no 1-op loadb) */
            if(span_eq(o,l,"__storeb")){ printf("  storeb v%d v%d\n",args[0],args[1]); return emit_const(0); }   /* R2b: write one byte */
            if(span_eq(o,l,"__load8")){ printf("  v%d: i64 = load8 v%d\n",ncount,args[0]); expr_is64=1; return ncount++; }   /* T39: 8-byte cell read (darwin prelude pointer cells) */
            if(span_eq(o,l,"__store8")){ printf("  store8 v%d v%d\n",args[0],args[1]); return emit_const(0); }   /* T39: 8-byte cell write */
            if(gi>=0){                                              /* GENERIC call -> monomorphize by T's concrete type (T = the gf_tparg-th param) */
                int k,found=0; tstyp = gf_tparg[gi]<na ? argsty[gf_tparg[gi]] : -1;
                if(tstyp<0) tstyp=-2;                               /* scalar T (i32) -> single type-erased instance */
                for(k=0;k<ninst;k++) if(inst_gf[k]==gi&&inst_styp[k]==tstyp){found=1;break;}
                if(!found){ inst_gf[ninst]=gi; inst_styp[ninst]=tstyp; ninst++; }
                printf("  v%d: i32 = call ",ncount); fwrite(src+gf_off[gi],1,gf_len[gi],stdout); printf("_");
                if(tstyp==-2) printf("i32"); else fwrite(src+soff[tstyp],1,slen[tstyp],stdout);
                for(i=0;i<na;i++)printf(" v%d",args[i]); printf("\n"); return ncount++;
            }
            if(!fn_has(o,l)){pos=p;die("call to unknown function");}
            { int fi=fn_index(o,l); printf("  v%d: i32 = call ",ncount); fwrite(src+o,1,l,stdout); for(i=0;i<na;i++)printf(" v%d",args[i]); printf("\n");
              i=ncount++; if(fi>=0){ expr_is64=fret64[fi]; expr_ischar=fretchar[fi]; expr_isstr=fretstr[fi]; expr_enum=fretenum[fi]; expr_isstring=fretstring[fi]; } return i; }   /* R1b/R2/R4/R2b: tag the call result with the fn's return type */
        }
        if(struct_find(o,l)>=0 && src[pos]=='{'){                    /* struct literal Name { f: e, .. } */
            int st=struct_find(o,l), base=emit_heap_alloc(ssize[st]); pos++;   /* R7: heap-allocate so a struct RETURNED from a fn (e.g. Type::new) survives the callee frame */
            for(;;){ int fo,fl,boff,fstyp,ev,ad; skipws(); if(src[pos]=='}'){pos++;break;}
                if(!rd_ident(&fo,&fl)) die("expected field in struct literal");
                if(!fld_find(st,fo,fl,&boff,&fstyp)) die("no such field in struct literal");
                if(!eat(":")) die("expected `:` in struct literal"); ev=parse_expr();
                { int co=emit_const(boff); ad=emit_addr_add(base, co); }   /* base is now a heap ADDRESS -> field addr = base+off (no `ref`) */
                if(fld_fkind) emit_pstore(ad,ev); else emit_store(ad,ev);   /* T39: container-desc fields are pointer cells */
                skipws(); if(eat(","))continue; if(src[pos]=='}'){pos++;break;} die("expected `,` or `}`");
            }
            /* heap struct: vN holds the base ADDRESS; kept K_STRUCT (an owned MOVE type) but addressed like a pointer */
            place_kind=K_STRUCT; place_styp=st; return base;
        }
        { int si=sym_find(o,l);
          if(si<0){ int gi=g_find(o,l);                                            /* selfhost P0: module-level static read */
              if(gi>=0){ skipws();
                  if(src[pos]=='['){ pos++; { int iv=parse_expr(); int base,off,ad; eat("]");
                      base=emit_gref_sp(g_off[gi],g_len[gi]);
                      if(g_esz[gi]==1) off=iv; else { int sh=emit_const(g_esz[gi]==8?3:2); off=emit_binop("shl",iv,sh); }
                      ad=emit_addr_add(base,off);
                      if(g_esz[gi]==1) return emit_loadb(ad);
                      if(g_esz[gi]==8){ expr_is64=1; return emit_load8(ad); }
                      return emit_load(ad); } }
                  if(g_kind[gi]==1) return emit_gref_sp(g_off[gi],g_len[gi]);        /* bare global array name -> base pointer (decay) */
                  { int gr=emit_gref_sp(g_off[gi],g_len[gi]);                        /* scalar static read */
                    if(g_is64[gi]){ expr_is64=1; return emit_load8(gr); }
                    return emit_load(gr); } }
              pos=p; die("use of undeclared name"); }
          skipws();
          if(vkind[si]==K_STR && src[pos]=='.'){ int save=pos,mo,ml; pos++; skipws();   /* R2: str.len() -> desc.len */
              if(rd_ident(&mo,&ml)&&span_eq(mo,ml,"len")){ skipws(); if(src[pos]=='('){ pos++; skipws(); eat(")"); { int co=emit_const(8),ad=emit_addr_add(vvn[si],co); return emit_load8(ad); } } }
              pos=save; }
          if(vkind[si]==K_STR){ place_kind=K_STR; expr_isstr=1; return vvn[si]; }         /* R2: &str value = descriptor pointer (Copy) */
          if(vkind[si]==K_VEC) return dispatch_vec(vvn[si]);                              /* R3: Vec methods + v[i] read */
          if(vkind[si]==K_STRING) return dispatch_string(vvn[si]);                        /* R2b: String methods + bare value */
          if(vkind[si]==K_ENUMVAL && (vstyp[si]==opt_en||vstyp[si]==res_en) && src[pos]=='.'){ int save=pos,mo,ml; pos++; skipws();   /* R4: Option/Result methods */
              if(rd_ident(&mo,&ml)){ skipws(); if(src[pos]=='('){ int isopt=(vstyp[si]==opt_en),self=vvn[si],arg=-1,hasarg=0; const char*fn=0; pos++; skipws();
                  if(src[pos]!=')'){ arg=parse_expr(); hasarg=1; skipws(); } eat(")");
                  if(isopt){ if(span_eq(mo,ml,"unwrap"))fn="__opt_unwrap"; else if(span_eq(mo,ml,"unwrap_or"))fn="__opt_unwrap_or"; else if(span_eq(mo,ml,"is_some"))fn="__opt_is_some"; else if(span_eq(mo,ml,"is_none"))fn="__opt_is_none"; }
                  else { if(span_eq(mo,ml,"unwrap"))fn="__res_unwrap"; else if(span_eq(mo,ml,"unwrap_or"))fn="__res_unwrap_or"; else if(span_eq(mo,ml,"unwrap_err"))fn="__res_unwrap_err"; else if(span_eq(mo,ml,"is_ok"))fn="__res_is_ok"; else if(span_eq(mo,ml,"is_err"))fn="__res_is_err"; }
                  if(fn){ printf("  v%d: i32 = call %s v%d",ncount,fn,self); if(hasarg)printf(" v%d",arg); printf("\n"); return ncount++; }
                  die("R4: unknown Option/Result method"); } }
              pos=save; }
          if(src[pos]=='.'){ int save=pos,mo,ml; pos++; skipws();       /* receiver.method(args) -> static dispatch to `Type_method` */
              if(rd_ident(&mo,&ml)){ skipws();
                  if(src[pos]=='('){ int styp=vstyp[si],self,args[32],na=0,i; chk_live(si);
                      if(styp<0) die("method call on a non-struct receiver");
                      self=(vkind[si]==K_PTRSTRUCT||vkind[si]==K_STRUCT||vkind[si]==K_ENUMVAL)?vvn[si]:emit_ref(vvn[si]);   /* self = base address (heap struct/ref/enum: vN already the address) */
                      pos++; skipws(); if(src[pos]!=')'){ for(;;){ args[na++]=parse_expr(); skipws(); if(eat(","))continue; break; } }
                      if(!eat(")")) die("expected `)` in method call");
                      printf("  v%d: i32 = call ",ncount); fwrite(src+soff[styp],1,slen[styp],stdout); printf("_"); fwrite(src+mo,1,ml,stdout);
                      printf(" v%d",self); for(i=0;i<na;i++)printf(" v%d",args[i]); printf("\n");
                      { int r=ncount++, rs=meth_retstyp(soff[styp],slen[styp],mo,ml); if(rs>=0){ place_kind=K_PTRSTRUCT; place_styp=rs; } return r; }   /* R7: tag a method result that returns Self/a struct */
                  } }
              pos=save; }
          if(src[pos]=='.'||src[pos]=='['){ pos=p; { int a=parse_place();
              if(place_fkind==K_VEC){ int dv=emit_pload(a); return dispatch_vec(dv); }        /* P2: self.data.push(x) -- Vec field */
              if(place_fkind==K_STRING){ int dv=emit_pload(a); return dispatch_string(dv); }  /* P2: self.name.push_str(..) -- String field */
              if(place_fkind==K_STR){ int dv=emit_pload(a);                                    /* P2: &str field -> descriptor value */
                  if(src[pos]=='.'){ int sv2=pos,mo,ml; pos++; skipws(); if(rd_ident(&mo,&ml)&&span_eq(mo,ml,"len")){ skipws(); if(src[pos]=='('){ pos++; skipws(); eat(")"); { int co=emit_const(8),ad=emit_addr_add(dv,co); return emit_load8(ad); } } } pos=sv2; }
                  place_kind=K_STR; expr_isstr=1; return dv; }
              if(place_kind==K_SCALAR){ int lv=emit_load(a); expr_is64=0; expr_ischar=0; return lv; } place_kind=vkind[si]; return a; } }  /* field/index: parse_place checks moved */
          chk_live(si);
          if(vkind[si]==K_ENUMVAL){ place_kind=K_SCALAR; place_styp=-1; expr_enum=vstyp[si]; vmoved[si]=1; return vvn[si]; }         /* data-enum by value -> MOVE */
          if(vkind[si]==K_CLOSURE){ place_kind=K_CLOSURE; expr_isclosure=1; return vvn[si]; }  /* R6: closure fat pointer -> Copy */
          if(vkind[si]==K_SCALAR||vkind[si]==K_PTRSTRUCT){ place_kind=vkind[si]; place_styp=vstyp[si]; expr_is64=vis64[si]; expr_ischar=vischar[si]; return vvn[si]; }  /* scalar/reference value -> Copy (no move) */
          if(vkind[si]==K_STRUCT){ place_kind=K_STRUCT; place_styp=vstyp[si]; vmoved[si]=1; return vvn[si]; }             /* owned heap struct by value -> MOVE (vN is the address) */
          place_kind=vkind[si]; place_styp=vstyp[si]; place_esz=vesz[si]; vmoved[si]=1; return emit_ref(vvn[si]);        /* array by value -> MOVE */
        }
    }
    die("expected an expression"); return 0;
}
static int peek_is(int c){ skipws(); return pos<srclen && src[pos]==c; }

/* match EXPR { pat => e, ... } as an expression -> if-else-if decision tree over a result slot.
   pat: `_` | INT-literal | E::Variant | E::Variant(binds).  Scrutinee: int/C-enum (scalar) or data enum (base addr). */
static int parse_match(void){
    int scrutv, base=-1, en, discv, resVN, mend;
    eat_kw("match");
    scrutv=parse_expr(); en=expr_enum;
    if(!eat("{")) die("expected `{` after match scrutinee");
    if(en>=0){ base=scrutv; discv=emit_load(base); } else discv=scrutv;   /* data enum: disc @0 */
    resVN=emit_const(0); mend=lblctr++;
    for(;;){ int po,pl,disc=0,nf=0,c,savesym=nsym; skipws();
        if(src[pos]=='}'){ pos++; break; }
        if(src[pos]=='_' && !isidc(src[pos+1])){                 /* wildcard: unconditional */
            pos++; if(!eat("=>")) die("expected `=>`");
            { int av=parse_expr(); copy_to_x(resVN,av); }
            printf("  goto __Lmend%d\n",mend);
            skipws(); if(eat(","))continue; if(src[pos]=='}'){pos++;break;} continue;
        }
        if(src[pos]>='0'&&src[pos]<='9'){ int v=0; while(pos<srclen&&src[pos]>='0'&&src[pos]<='9'){v=v*10+(src[pos]-'0');pos++;} disc=v; nf=0; }
        else { if(!rd_ident(&po,&pl)) die("expected pattern");
            if(src[pos]==':'&&src[pos+1]==':'){ int ve=enum_find(po,pl); pos+=2; if(!rd_ident(&po,&pl)) die("expected variant"); evar_find(ve,po,pl,&disc,&nf); }
            else if((span_eq(po,pl,"Some")||span_eq(po,pl,"None"))&&opt_en>=0){ evar_find(opt_en,po,pl,&disc,&nf); }   /* R4: bare Option pattern */
            else if((span_eq(po,pl,"Ok")||span_eq(po,pl,"Err"))&&res_en>=0){ evar_find(res_en,po,pl,&disc,&nf); }       /* R4: bare Result pattern */
            else { die("R3 pattern: literal, `_`, or E::Variant"); }
        }
        c=emit_binop("eq", discv, emit_const(disc));
        printf("  brz v%d\n",c);
        if(eat("(")){ int bi=0; for(;;){ int bo,bl; skipws(); if(src[pos]==')'){pos++;break;}   /* bind payload fields */
            if(src[pos]=='_'&&!isidc(src[pos+1])){ pos++; } else { if(!rd_ident(&bo,&bl)) die("expected binding");
                { int ad,co,lv; co=emit_const(8+bi*4); ad=emit_addr_add(base,co); lv=emit_load(ad); sym_add(bo,bl,lv,K_SCALAR,-1,4); } }
            bi++; skipws(); if(eat(","))continue; if(src[pos]==')'){pos++;break;} } }
        if(!eat("=>")) die("expected `=>`");
        { int av=parse_expr(); copy_to_x(resVN,av); }
        nsym=savesym;
        printf("  goto __Lmend%d\n  pool\n",mend);
        skipws(); if(eat(","))continue; if(src[pos]=='}'){pos++;break;}
    }
    printf("  label __Lmend%d\n",mend);
    expr_enum=-1; return resVN;
}

static int parse_cast(void){ int a=parse_atom(),ia=expr_is64,ic=expr_ischar,aen=expr_enum;
    skipws();
    for(;;){    /* R4: postfix `?` and Option/Result `.method()` on ANY receiver (var OR call result) */
        if(src[pos]=='?'){ pos++; a=emit_try(a,aen); ia=0; ic=0; aen=-1; skipws(); continue; }
        if(src[pos]=='.' && (aen==opt_en||aen==res_en) && aen>=0){ int save=pos,mo,ml; pos++; skipws();
            if(rd_ident(&mo,&ml)){ skipws(); if(src[pos]=='('){ int isopt=(aen==opt_en),arg=-1,hasarg=0; const char*fn=0; pos++; skipws();
                if(src[pos]!=')'){ arg=parse_expr(); hasarg=1; skipws(); } eat(")");
                if(isopt){ if(span_eq(mo,ml,"unwrap"))fn="__opt_unwrap"; else if(span_eq(mo,ml,"unwrap_or"))fn="__opt_unwrap_or"; else if(span_eq(mo,ml,"is_some"))fn="__opt_is_some"; else if(span_eq(mo,ml,"is_none"))fn="__opt_is_none"; }
                else { if(span_eq(mo,ml,"unwrap"))fn="__res_unwrap"; else if(span_eq(mo,ml,"unwrap_or"))fn="__res_unwrap_or"; else if(span_eq(mo,ml,"unwrap_err"))fn="__res_unwrap_err"; else if(span_eq(mo,ml,"is_ok"))fn="__res_is_ok"; else if(span_eq(mo,ml,"is_err"))fn="__res_is_err"; }
                if(!fn) die("R4: unknown Option/Result method");
                printf("  v%d: i32 = call %s v%d",ncount,fn,a); if(hasarg)printf(" v%d",arg); printf("\n"); a=ncount++; ia=0; ic=0; aen=-1; skipws(); continue; } }
            pos=save; break; }
        break;
    }
    while(at_kw("as")){ eat_kw("as"); { int to,tl; eat("&"); rd_ident(&to,&tl);
        if(tl==4&&!strncmp(src+to,"char",4)){ ic=1; }                                              /* `e as char` -> Display as a character */
        else if(tl==3&&(!strncmp(src+to,"i64",3)||!strncmp(src+to,"u64",3))){ if(!ia){a=emit_conv(a);} ia=1; ic=0; }   /* `as i64` -> zext, numeric */
        else { ic=0; } } }                                                                          /* as i32/u8/u32/usize -> numeric */
    expr_is64=ia; expr_ischar=ic; return a; }
static int parse_mul(void){ int a=parse_cast(),ia=expr_is64; for(;;){ skipws();
    if(eat("*")){ int b=parse_cast(),ib=expr_is64; a=emit_arith("mul","mul8",a,ia,b,ib,0); ia=expr_is64; }
    else if(eat("/")){ int b=parse_cast(),ib=expr_is64; a=emit_arith("div","div8",a,ia,b,ib,0); ia=expr_is64; }
    else if(eat("%")){ int b=parse_cast(),ib=expr_is64; a=emit_arith("mod","mod8",a,ia,b,ib,0); ia=expr_is64; }
    else break; } expr_is64=ia; return a; }
static int parse_add(void){ int a=parse_mul(),ia=expr_is64; for(;;){ skipws();
    if(eat("+")){ int b=parse_mul(),ib=expr_is64; a=emit_arith("add","add8",a,ia,b,ib,0); ia=expr_is64; }
    else if(pos<srclen&&src[pos]=='-'&&src[pos+1]!='>'){ int b; pos++; b=parse_mul(); { int ib=expr_is64; a=emit_arith("sub","sub8",a,ia,b,ib,0); ia=expr_is64; } }
    else break; } expr_is64=ia; return a; }
static int parse_cmp(void){ int a=parse_add(),ia=expr_is64,b,ib;
    #define CMP(o32,o8) { b=parse_add(); ib=expr_is64; return emit_arith(o32,o8,a,ia,b,ib,1); }
    skipws();
    if(eat("==")) CMP("eq","eq8") if(eat("!=")) CMP("ne","ne8")
    if(eat("<=")) CMP("le","le8") if(eat(">=")) CMP("ge","ge8")
    if(pos<srclen&&src[pos]=='<'){pos++; CMP("lt","lt8")} if(pos<srclen&&src[pos]=='>'){pos++; CMP("gt","gt8")}
    #undef CMP
    return a; }
static int parse_and(void){ int a=parse_cmp(); for(;;){ skipws(); if(!eat("&&"))break;
    { int res=emit_const(0); printf("  brz v%d\n",a); {int b=parse_cmp(); copy_to(res,as_bool(b));} printf("  pool\n"); a=res; } } return a; }
static int parse_expr(void){ int a=parse_and(); for(;;){ skipws(); if(!eat("||"))break;
    { int res=emit_const(1); printf("  brz v%d\n",not_bool(a)); {int b=parse_and(); copy_to(res,as_bool(b));} printf("  pool\n"); a=res; } } return a; }

/* rust-closure R0/R4: println!/print! as a UNIT-valued expression (returns 0). Does NOT consume a
 * trailing `;` -- callable both as a statement (parse_stmt) and as a match-arm/expr (parse_atom). */
/* P2: Vec/String method+index dispatch on a descriptor vN `dv` -- shared by local vars AND struct
 * fields (self.data.push(x) loads the field then calls dispatch_vec on the loaded descriptor). */
static int dispatch_vec(int dv){
    if(src[pos]=='.'){ int save=pos,mo,ml; pos++; skipws();
        if(rd_ident(&mo,&ml)){ skipws();
            if(span_eq(mo,ml,"len")&&src[pos]=='('){ pos++; skipws(); eat(")"); return emit_call1("__vec_len",dv); }
            if(span_eq(mo,ml,"push")&&src[pos]=='('){ pos++; skipws(); { int e=parse_expr(); skipws(); eat(")"); printf("  v%d: i32 = call __vec_push v%d v%d\n",ncount,dv,e); return ncount++; } }
            if(span_eq(mo,ml,"pop")&&src[pos]=='('){ pos++; skipws(); eat(")"); { int r=emit_call1("__vec_pop",dv); expr_enum=opt_en; return r; } } }
        pos=save; }
    if(src[pos]=='['){ pos++; { int iv=parse_expr(); eat("]"); { int pp=emit_pload(dv),off=emit_binop("shl",iv,emit_const(2)),ad=emit_addr_add(pp,off),lv=emit_load(ad); expr_is64=0; return lv; } } }
    place_kind=K_VEC; expr_isvec=1; return dv;
}
static int dispatch_string(int dv){
    if(src[pos]=='.'){ int save=pos,mo,ml; pos++; skipws();
        if(rd_ident(&mo,&ml)){ skipws();
            if(span_eq(mo,ml,"len")&&src[pos]=='('){ pos++; skipws(); eat(")"); return emit_call1("__str_len",dv); }
            if(span_eq(mo,ml,"push")&&src[pos]=='('){ pos++; skipws(); { int e=parse_expr(); skipws(); eat(")"); printf("  v%d: i32 = call __str_push_byte v%d v%d\n",ncount,dv,e); return ncount++; } }
            if(span_eq(mo,ml,"push_str")&&src[pos]=='('){ pos++; skipws(); { int e=parse_expr(); skipws(); eat(")"); printf("  v%d: i32 = call __str_push_str v%d v%d\n",ncount,dv,e); return ncount++; } }
            if((span_eq(mo,ml,"as_str")||span_eq(mo,ml,"as_bytes"))&&src[pos]=='('){ pos++; skipws(); eat(")"); place_kind=K_STR; expr_isstr=1; return dv; } }
        pos=save; }
    place_kind=K_STRING; expr_isstring=1; return dv;
}
static int parse_print_macro(void){
    int nl=at_kw("println"), fo,fl,holeidx=0,args[32],args64[32],argsch[32],argsstr[32],argsstring[32],na=0,k,end,c;
    eat_kw(nl?"println":"print"); if(!eat("!")) die("expected `!` after print macro"); if(!eat("(")) die("expected `(`");
    skipws(); if(src[pos]!='"') die("R0 println! expects a string-literal format"); pos++;
    fo=pos; while(pos<srclen&&src[pos]!='"'){ if(src[pos]=='\\')pos++; pos++; } fl=pos-fo; pos++;
    skipws(); while(eat(",")){ skipws(); if(src[pos]==')')break; args[na]=parse_expr(); args64[na]=expr_is64; argsch[na]=expr_ischar; argsstr[na]=expr_isstr; argsstring[na]=expr_isstring; na++; skipws(); }
    if(!eat(")")) die("expected `)`");
    k=fo; end=fo+fl;
    while(k<end){ c=src[k];
        if(c=='\\'){ k++; c=src[k]; if(c=='n')c=10; else if(c=='t')c=9; else if(c=='r')c=13; else if(c=='0')c=0; emit_call1("__putc",emit_const(c)); k++; continue; }
        if(c=='{'){ if(k+1<end&&src[k+1]=='{'){ emit_call1("__putc",emit_const('{')); k+=2; continue; }
            while(k<end&&src[k]!='}')k++; k++;
            if(argsstring[holeidx]){ int d=args[holeidx],p=emit_pload(d),co=emit_const(8),ad=emit_addr_add(d,co),ln=emit_load(ad); printf("  v%d: i32 = call __print_str v%d v%d\n",ncount,p,ln); ncount++; }   /* R2b: String Display (4-byte ptr/len fields) */
            else if(argsstr[holeidx]){ int d=args[holeidx],p=emit_load8(d),co=emit_const(8),ad=emit_addr_add(d,co),nl2=emit_load8(ad); printf("  v%d: i32 = call __print_str v%d v%d\n",ncount,p,nl2); ncount++; }
            else if(argsch[holeidx]) emit_call1("__putc",args[holeidx]);
            else emit_call1(args64[holeidx]?"__print_int64":"__print_int",args[holeidx]);
            holeidx++; continue; }
        if(c=='}'){ if(k+1<end&&src[k+1]=='}'){ emit_call1("__putc",emit_const('}')); k+=2; continue; } k++; continue; }
        emit_call1("__putc",emit_const(c)); k++;
    }
    if(nl) emit_call1("__putc",emit_const(10));
    return emit_const(0);
}
/* R6: parse `|params| body` -> record it for deferred lifting, build its heap env from captured locals,
 * and return a heap {code@0=faddr(__closure_N), env@4} fat pointer. */
static int parse_closure(void){
    int cid=nclo++, np=0, nc=0, bstart, bend, isblock=0, env, clo, i;
    pos++;                                                     /* consume first `|` */
    skipws();
    if(src[pos]=='|'){ pos++; }                               /* `||` -- no params */
    else { for(;;){ int po,pl; skipws(); if(src[pos]=='|'){ pos++; break; }
            if(!rd_ident(&po,&pl)) die("expected closure param"); clo_poff[cid][np]=po; clo_plen[cid][np]=pl; np++;
            skipws(); if(eat(":")){ int to,tl; if(eat("&")){ eat_kw("mut"); } rd_ident(&to,&tl); skipws(); if(src[pos]=='<'){ int d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } } }
            skipws(); if(eat(","))continue; if(src[pos]=='|'){ pos++; break; } } }
    clo_np[cid]=np;
    skipws();
    if(src[pos]=='{'){ int d=0; isblock=1; bstart=pos; while(pos<srclen){ if(src[pos]=='{')d++; else if(src[pos]=='}'){ d--; pos++; if(d==0)break; continue; } pos++; } bend=pos; }
    else { int d=0; bstart=pos; while(pos<srclen){ int c=src[pos]; if(c=='('||c=='['||c=='{')d++; else if(c==')'||c==']'||c=='}'){ if(d==0)break; d--; } else if(d==0&&(c==','||c==';'))break; pos++; } bend=pos; }
    clo_boff[cid]=bstart; clo_blen[cid]=bend-bstart; clo_isblock[cid]=isblock;
    /* capture scan: identifiers in the body that resolve to an enclosing local (not a param, not after `.`) */
    { int q=bstart; while(q<bend){ if(isidst(src[q])){ int s=q,len,isp=0,isdot,pp,si; while(q<bend&&isidc(src[q]))q++; len=q-s;
          for(i=0;i<np;i++) if(clo_plen[cid][i]==len&&!strncmp(src+clo_poff[cid][i],src+s,len)) isp=1;
          pp=s-1; while(pp>=bstart&&(src[pp]==' '||src[pp]=='\t'))pp--; isdot=(pp>=bstart&&src[pp]=='.');
          if(!isp&&!isdot&&(si=sym_find(s,len))>=0){ int dup=0; for(i=0;i<nc;i++) if(clo_calen[cid][i]==len&&!strncmp(src+clo_caoff[cid][i],src+s,len))dup=1;
              if(!dup&&nc<16){ clo_caoff[cid][nc]=s; clo_calen[cid][nc]=len; clo_cakind[cid][nc]=vkind[si]; clo_castyp[cid][nc]=vstyp[si]; nc++; } } }
        else q++; } }
    clo_nc[cid]=nc;
    env=emit_heap_alloc(nc>0?nc*PW:PW);                          /* env holds the captured values */
    for(i=0;i<nc;i++){ int si=sym_find(clo_caoff[cid][i],clo_calen[cid][i]), off=emit_const(i*PW), ad=emit_addr_add(env,off); emit_pstore(ad,vvn[si]); }   /* T39: captures may be descriptors/addresses -> pointer cells */
    clo=emit_heap_alloc(2*PW);
    { int fa; printf("  v%d: i32 = faddr __closure_%d\n",ncount,cid); fa=ncount++; emit_pstore(clo,fa); { int co=emit_const(PW),ad=emit_addr_add(clo,co); emit_pstore(ad,env); } }   /* T39: code is a >2^32 address on darwin */
    expr_isclosure=1; place_kind=K_CLOSURE; return clo;
}
/* R6: emit the lifted body of closure cid as a top-level fn `__closure_N(env, params...)` (deferred). */
static void emit_closure_body(int cid){
    int i, envvn, pvn[8], savedpos=pos;
    ncount=0; nsym=0;
    printf("fn __closure_%d\n",cid);
    envvn=ncount++; printf("  v%d: i32 = param\n",envvn);        /* ALL params must be contiguous at the top (backend maps them positionally) */
    for(i=0;i<clo_np[cid];i++){ pvn[i]=ncount++; printf("  v%d: i32 = param\n",pvn[i]); }
    for(i=0;i<clo_nc[cid];i++){ int off=emit_const(i*PW),ad=emit_addr_add(envvn,off),lv=emit_pload(ad); sym_add(clo_caoff[cid][i],clo_calen[cid][i],lv,clo_cakind[cid][i],clo_castyp[cid][i],4); }   /* captures loaded from env AFTER the params */
    for(i=0;i<clo_np[cid];i++) sym_add(clo_poff[cid][i],clo_plen[cid][i],pvn[i],K_SCALAR,-1,4);
    czero=emit_const(0);
    pos=clo_boff[cid];
    if(clo_isblock[cid]){ parse_block(); printf("  ret v%d\n",czero); }
    else { int r=parse_expr(); printf("  ret v%d\n",r); }
    pos=savedpos;
}
static void parse_stmt(void){
    int o,l;
    skipws();
    if (eat_kw("let")) {
        int vn,ev,letis64=0,letischar=0; eat_kw("mut");
        if(!rd_ident(&o,&l)) die("expected name after let");
        if(eat(":")){                                             /* type annotation */
            int to,tl; if(eat("&")){ eat_kw("mut"); }
            if(eat("[")){ rd_ident(&to,&tl); eat(";"); {int co,cl; while(pos<srclen&&src[pos]!=']')pos++;} eat("]"); }
            else { rd_ident(&to,&tl); if(tl==3&&(!strncmp(src+to,"i64",3)||!strncmp(src+to,"u64",3))) letis64=1; if(tl==4&&!strncmp(src+to,"char",4)) letischar=1;
                skipws(); if(src[pos]=='<'){ int d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } } }   /* R3: skip generic args e.g. Vec<i32> */
        }
        if(!eat("=")) die("expected `=` in let");
        skipws();
        if (src[pos]=='[') {                                      /* array literal */
            int base,i,cnt=0,vals[256],rep=0,repv=-1;
            pos++;
            if(src[pos]!=']'){ vals[cnt++]=parse_expr(); skipws();
                if(eat(";")){ rep=1; { int n=0,any=0; skipws(); while(pos<srclen&&src[pos]>='0'&&src[pos]<='9'){n=n*10+(src[pos]-'0');pos++;any=1;} (void)any; cnt=n; repv=vals[0]; } }
                else { for(;;){ skipws(); if(src[pos]==']')break; if(!eat(","))die("expected `,`"); skipws(); if(src[pos]==']')break; vals[cnt++]=parse_expr(); } } }
            if(!eat("]")) die("expected `]`"); eat(";");
            base=reserve_slots(cnt*4);
            for(i=0;i<cnt;i++){ int rb=emit_ref(base), co=emit_const(i*4), ad=emit_addr_add(rb, co); emit_store(ad, rep?repv:vals[i]); }
            sym_add(o,l,base,K_ARRAY,-1,4); return;
        }
        { int po=pos, so,sl; skipws();                            /* struct literal? Name { ... } */
          if(rd_ident(&so,&sl) && struct_find(so,sl)>=0){ skipws(); if(src[pos]=='{'){ pos=po; place_kind=K_SCALAR;
              ev=parse_atom();  /* parses the struct literal -> heap base address, place_kind=K_STRUCT */
              if(place_kind==K_STRUCT){ eat(";"); sym_add(o,l,ev,K_STRUCT,place_styp,4); return; } } }
          pos=po;
        }
        place_kind=K_SCALAR; ev=parse_expr(); if(!eat(";")) die("expected `;` after let");
        if (expr_enum>=0) { sym_add(o,l,ev,K_ENUMVAL,expr_enum,4); return; }                  /* let x = E::Variant(..) (data enum value) */
        if (place_kind==K_PTRSTRUCT) { sym_add(o,l,ev,K_PTRSTRUCT,place_styp,4); return; }   /* let r = &struct-place */
        if (expr_isstr) { sym_add(o,l,ev,K_STR,-1,4); return; }                               /* R2: `let s = "..."` -- ev is the descriptor pointer */
        if (expr_isvec) { sym_add(o,l,ev,K_VEC,-1,4); return; }                                /* R3: `let v = Vec::new()`/vec![..] -- ev is the descriptor pointer */
        if (expr_isstring) { sym_add(o,l,ev,K_STRING,-1,4); return; }                           /* R2b: `let s = String::new()`/from -- ev is the descriptor pointer */
        if (expr_isclosure) { sym_add(o,l,ev,K_CLOSURE,-1,4); return; }                          /* R6: `let f = |x| ..` -- ev is the {code,env} fat pointer */
        if (letis64 || expr_is64) { if(!expr_is64){ ev=emit_conv(ev); } vn=ncount++; copy_to64(vn,ev); sym_add(o,l,vn,K_SCALAR,-1,4); vis64[nsym-1]=1; return; }   /* R1: i64 local */
        { int ischar=letischar||expr_ischar; vn=ncount++; copy_to(vn,ev); sym_add(o,l,vn,K_SCALAR,-1,4); if(ischar) vischar[nsym-1]=1; return; }   /* R1b: char local */
    }
    if (eat_kw("return")) { skipws(); if(pos<srclen&&src[pos]==';'){ pos++; printf("  ret v%d\n",czero); } else { int ev=parse_expr(); eat(";"); printf("  ret v%d\n",ev); } return; }   /* selfhost P0: bare `return;` in a unit fn -> ret 0 */
    if (eat_kw("break")) { eat(";"); printf("  brk\n"); return; }
    if (eat_kw("continue")) { eat(";"); printf("  cnt\n"); return; }   /* the continue STATEMENT is `cnt` (a jump); the loop's continue-TARGET is `cont` (cf distinguishes them) */
    if (eat_kw("if")) { int c=parse_expr(),lbl; printf("  brz v%d\n",c); parse_block(); skipws();
        if(at_kw("else")){ eat_kw("else"); lbl=lblctr++; printf("  goto __Lelse%d\n  pool\n",lbl); if(at_kw("if"))parse_stmt(); else parse_block(); printf("  label __Lelse%d\n",lbl); }
        else printf("  pool\n"); return; }
    if (at_kw("unsafe")) { eat_kw("unsafe"); parse_block(); return; }   /* selfhost P0: `unsafe { .. }` -- transparent block (static mut access) */
    if (eat_kw("while")) { int c; printf("  loop\n"); c=parse_expr(); printf("  brz v%d\n",c); loop_depth++; parse_block(); loop_depth--; printf("  cont\n  jmp\n  endloop\n"); return; }
    if (eat_kw("loop")) { int one; printf("  loop\n"); one=emit_const(1); printf("  brz v%d\n",one); loop_depth++; parse_block(); loop_depth--; printf("  cont\n  jmp\n  endloop\n"); return; }
    if (eat_kw("for")) {   /* R5: for PAT in ITER { body } -- range (a..b / a..=b) or Vec (&v / v.iter() [.enumerate()]) */
        int istuple=0,io=0,il=0,xo,xl,is_coll=0,coll_si=-1,enumerate=0,savesym;
        skipws();
        if(eat("(")){ istuple=1; skipws(); rd_ident(&io,&il); skipws(); eat(","); skipws(); rd_ident(&xo,&xl); skipws(); eat(")"); }
        else rd_ident(&xo,&xl);
        skipws(); if(!eat_kw("in")) die("expected `in` in for");
        skipws();
        { int save=pos, io2,il2; if(src[pos]=='&'){ pos++; skipws(); eat_kw("mut"); skipws(); }
          if(rd_ident(&io2,&il2)){ int si=sym_find(io2,il2);
              if(si>=0 && vkind[si]==K_VEC){ is_coll=1; coll_si=si; skipws();
                  while(src[pos]=='.'){ int mo,ml; pos++; skipws(); if(rd_ident(&mo,&ml)){ skipws(); if(src[pos]=='('){ pos++; skipws(); eat(")"); } if(span_eq(mo,ml,"enumerate")) enumerate=1; } skipws(); } } }
          if(!is_coll) pos=save; }
        if(is_coll){
            int idxsl=ncount++, lensl=ncount++, cond;
            copy_to(idxsl,emit_const(0)); { int lv=emit_call1("__vec_len",vvn[coll_si]); copy_to(lensl,lv); }
            printf("  loop\n"); cond=emit_binop("lt",idxsl,lensl); printf("  brz v%d\n",cond);
            { int p2=emit_pload(vvn[coll_si]), off=emit_binop("shl",idxsl,emit_const(2)), ea=emit_addr_add(p2,off), elem=emit_load(ea);
              savesym=nsym; if(enumerate||istuple){ sym_add(io,il,idxsl,K_SCALAR,-1,4); sym_add(xo,xl,elem,K_SCALAR,-1,4); } else sym_add(xo,xl,elem,K_SCALAR,-1,4); }
            loop_depth++; parse_block(); loop_depth--; nsym=savesym;
            printf("  cont\n"); { int one=emit_const(1),nv=emit_binop("add",idxsl,one),rf=emit_ref(idxsl); emit_store(rf,nv); } printf("  jmp\n  endloop\n"); return;   /* sequenced: no arg-eval-order dependence across host compilers */
        } else {
            int startv=parse_expr(), incl, endv, itv=ncount++, endsl=ncount++, cond;
            skipws(); if(!eat("..")) die("expected `..` in for range"); incl=eat("=");
            endv=parse_expr(); copy_to(itv,startv); copy_to(endsl,endv);
            printf("  loop\n"); cond=emit_binop(incl?"le":"lt",itv,endsl); printf("  brz v%d\n",cond);
            savesym=nsym; sym_add(xo,xl,itv,K_SCALAR,-1,4);
            loop_depth++; parse_block(); loop_depth--; nsym=savesym;
            printf("  cont\n"); { int one=emit_const(1),nv=emit_binop("add",itv,one),rf=emit_ref(itv); emit_store(rf,nv); } printf("  jmp\n  endloop\n"); return;   /* sequenced: no arg-eval-order dependence across host compilers */
        }
    }
    if (at_kw("std")) { if(eat_kw("std")&&eat("::")&&eat_kw("process")&&eat("::")&&eat_kw("exit")&&eat("(")){ int ev=parse_expr(); eat(")"); eat(";"); printf("  ret v%d\n",ev); return; } die("R2 std path: only std::process::exit"); }
    if (at_kw("println")||at_kw("print")) { parse_print_macro(); eat(";"); return; }   /* rust-closure R0: println!/print! statement */
    if (at_kw("match")) { int v=parse_match(); skipws(); if(pos<srclen&&src[pos]=='}'&&loop_depth==0){ printf("  ret v%d\n",v); } else eat(";"); return; }   /* R4: match statement (optional `;`) or fn tail-expr (not inside a loop) */

    /* place assignment (x/x.f/a[i]/*p = e) OR expr-stmt OR tail-expr.
       decide by a lookahead for a top-level `=` (not ==/<=/>=/!=) before the statement end. */
    {
        int q=pos, depth=0, isasg=0;
        while(q<srclen){ int c=src[q];
            if(c=='('||c=='['||c=='{') depth++;
            else if(c==')'||c==']'||c=='}'){ if(depth==0)break; depth--; }
            else if(depth==0 && c==';') break;
            else if(depth==0 && c=='=' && src[q+1]!='=' && src[q-1]!='=' && src[q-1]!='<' && src[q-1]!='>' && src[q-1]!='!'){ isasg=1; break; }
            q++;
        }
        if (isasg) { int addr=parse_place(),pe=place_esz; if(!eat("=")) die("expected `=`"); { int ev=parse_expr(); if(!eat(";")) die("expected `;` after assignment");
            if(pe==1) emit_storeb(addr,ev);                                        /* selfhost P0: u8 place -> storeb */
            else if(pe==8||expr_is64) emit_store8(addr,ev);                        /* i64 place OR i64 rhs -> store8 */
            else emit_store(addr,ev); } return; }
        { int ev=parse_expr(); skipws();
          if(eat(";")){ (void)ev; return; }
          if(pos<srclen&&src[pos]=='}'){ if(loop_depth==0) printf("  ret v%d\n",ev); return; }
          die("expected `;` or `}` after expression"); }
    }
}

static void parse_block(void){
    int save=nsym;
    if(!eat("{")) die("expected `{`");
    for(;;){ skipws(); if(pos<srclen&&src[pos]=='}'){pos++;break;} if(pos>=srclen)die("unterminated block"); parse_stmt(); }
    nsym=save;
}

/* params + body; pos is at `(`. imploff/impllen = method receiver type (else -1/0). gsub may be active (T->concrete). */
static void parse_fn_rest(int imploff, int impllen){
    int po,pl;
    if(!eat("(")) die("expected `(`");
    if(impllen>0){                                            /* method: inject `&self`/`self`/`&mut self` -- but NOT for an associated fn like `Type::new` (no self) */
        int save=pos; skipws(); if(eat("&")){ eat_kw("mut"); } skipws();
        { int so,sl,pp=pos; if(rd_ident(&so,&sl) && span_eq(so,sl,"self")){ int styp=struct_find(imploff,impllen),vn=ncount++; printf("  v%d: i32 = param\n",vn);
              if(styp>=0) sym_add(so,sl,vn,K_PTRSTRUCT,styp,4); else sym_add(so,sl,vn,K_SCALAR,-1,4);
              skipws(); if(src[pos]==','){pos++;} }
          else { pos=save; } }                                /* associated fn -- rewind, parse params normally */
    }
    skipws();
    if(src[pos]!=')') for(;;){
        int vn,isref=0,styp=-1,isenum=0,een=-1,pis64=0,pischar=0,pisstr=0,pisvec=0,pisstring=0,pisclosure=0;
        if(!rd_ident(&po,&pl)) die("expected param name");
        if(eat(":")){ int to,tl; if(eat("&")){ isref=1; eat_kw("mut"); if(src[pos]=='\''){ pos++; { int lo,ll; rd_ident(&lo,&ll); } } } if(rd_ident(&to,&tl)){ styp=struct_find(to,tl); if(styp<0){ een=enum_find(to,tl); if(een>=0) isenum=1; }
            if(tl==4&&!strncmp(src+to,"impl",4)){ pisclosure=1; styp=-1; skipws(); { int fo,fl; rd_ident(&fo,&fl); } skipws();   /* R6: `impl Fn(..)->..` param -> closure fat pointer */
                if(src[pos]=='('){ int d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='(')d++; else if(src[pos]==')')d--; pos++; } } skipws();
                if(src[pos]=='-'&&src[pos+1]=='>'){ int ro,rl; pos+=2; skipws(); if(eat("&")){} rd_ident(&ro,&rl); skipws(); if(src[pos]=='<'){ int d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } } } }
            if(tl==3&&(!strncmp(src+to,"i64",3)||!strncmp(src+to,"u64",3))) pis64=1;
            if(tl==4&&!strncmp(src+to,"char",4)) pischar=1;
            if(isref&&tl==3&&!strncmp(src+to,"str",3)){ pisstr=1; styp=-1; }
            if(tl==3&&!strncmp(src+to,"Vec",3)){ pisvec=1; styp=-1; }          /* R3: Vec / &Vec / &mut Vec param -> descriptor pointer */
            if(tl==6&&!strncmp(src+to,"String",6)){ pisstring=1; styp=-1; }    /* R2b: String / &String / &mut String param -> descriptor pointer */
            if(styp<0 && !isenum && gsub_active && tl==gsub_len && strncmp(src+to,src+gsub_off,tl)==0) styp=gsub_styp;
            skipws(); if(src[pos]=='<'){ int d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } } } }  /* generic param T -> the concrete struct; skip <..> args */
        vn=ncount++; printf("  v%d: i32 = param\n",vn);
        if(pisclosure) sym_add(po,pl,vn,K_CLOSURE,-1,4);                    /* R6: impl Fn param -> closure fat pointer */
        else if(pisvec) sym_add(po,pl,vn,K_VEC,-1,4);                       /* R3: Vec param -> descriptor pointer */
        else if(pisstring) sym_add(po,pl,vn,K_STRING,-1,4);                 /* R2b: String param -> descriptor pointer */
        else if(pisstr) sym_add(po,pl,vn,K_STR,-1,4);                       /* R2: &str param -> descriptor pointer */
        else if(styp>=0) sym_add(po,pl,vn,K_PTRSTRUCT,styp,4);              /* &Struct OR Struct-by-value: vN holds the base address (aggregates pass by address; field access auto-derefs) */
        else if(isenum&&ensize[een]>4) sym_add(po,pl,vn,K_ENUMVAL,een,4);   /* data-enum by value: vN holds the base address */
        else if(isenum) sym_add(po,pl,vn,K_SCALAR,-1,4);                    /* C-like enum by value: vN holds the discriminant */
        else { sym_add(po,pl,vn,K_SCALAR,-1,4); if(pis64) vis64[nsym-1]=1; if(pischar) vischar[nsym-1]=1; }   /* R1: i64/u64 param; R1b: char param */
        (void)isref;
        skipws(); if(eat(","))continue; break;
    }
    if(!eat(")")) die("expected `)`");
    if(eat("->")){ int to,tl; if(eat("&")){ if(src[pos]=='\''){ pos++; rd_ident(&to,&tl); skipws(); } } rd_ident(&to,&tl);
        skipws(); if(src[pos]=='<'){ int d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } } }   /* return type (skip `&'lifetime` + generic args e.g. Option<i32>) */
    czero=emit_const(0);
    if(in_main_emit){ int i; in_main_emit=0; for(i=0;i<nglob;i++) if(g_kind[i]==0&&g_init[i]!=0){ int gr=emit_gref_sp(g_off[i],g_len[i]),cv=emit_const(g_init[i]); if(g_is64[i]){ int c8=emit_conv(cv); printf("  store8 v%d v%d\n",gr,c8); } else printf("  store v%d v%d\n",gr,cv); } }   /* selfhost P0: non-zero scalar static inits at main entry */
    parse_block();
    printf("  ret v%d\n",czero);
}

static void parse_fn(int is_main, int imploff, int impllen){
    int o,l;
    ncount=0; nsym=0; in_main_emit=is_main;
    if(!rd_ident(&o,&l)) die("expected fn name");
    if(peek_is('<')){ int d=0; while(pos<srclen){ if(src[pos]=='<')d++; else if(src[pos]=='>'){d--;pos++;if(d==0)break;continue;} pos++; } }  /* skip generic params */
    if(is_main) printf("ir main -> i32\n");
    else { printf("fn "); if(impllen>0){ fwrite(src+imploff,1,impllen,stdout); printf("_"); } fwrite(src+o,1,l,stdout); printf("\n"); }  /* method -> `Type_method` */
    parse_fn_rest(imploff, impllen);
}

/* emit a monomorphized instantiation of generic fn gi with T = struct styp: name `gfname_TypeName` */
static void emit_generic_inst(int gi, int styp){
    ncount=0; nsym=0;
    gsub_off=gf_tpoff[gi]; gsub_len=gf_tplen[gi]; gsub_styp=styp; gsub_active=1;
    printf("fn "); fwrite(src+gf_off[gi],1,gf_len[gi],stdout); printf("_");
    if(styp==-2) printf("i32"); else fwrite(src+soff[styp],1,slen[styp],stdout); printf("\n");   /* styp==-2: scalar T instance */
    pos=gf_sig[gi];
    parse_fn_rest(-1,0);
    gsub_active=0;
}

/* pre-pass: parse a `struct Name { f: T, ... }` at pos (pos is just past `struct`) */
static void parse_struct_decl(void){
    int o,l,st,off=0;
    if(!rd_ident(&o,&l)) die("expected struct name");
    st=nstruct; soff[st]=o; slen[st]=l; nstruct++;
    skipws(); if(src[pos]=='<'){ int d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } }   /* R7: generic struct `Name<T>` -- T-fields are i32-sized in this value model, so strip the params */
    if(!eat("{")) die("expected `{` in struct");
    for(;;){ int fo,fl,to,tl,fstyp,fkind=0,isref=0; skipws(); if(src[pos]=='}'){pos++;break;}
        if(!rd_ident(&fo,&fl)) die("expected field name"); if(!eat(":")) die("expected `:`");
        fstyp=-1; if(eat("&")){ isref=1; if(src[pos]=='\''){ pos++; { int lo,ll; rd_ident(&lo,&ll); } } }   /* &T / &'a T reference field -> 4-byte pointer */
        if(rd_ident(&to,&tl)){ fstyp=struct_find(to,tl);
            if(tl==3&&!strncmp(src+to,"Vec",3)) fkind=K_VEC;                            /* P2: Vec<T> field */
            else if(tl==6&&!strncmp(src+to,"String",6)) fkind=K_STRING;                 /* P2: String field */
            else if(isref&&tl==3&&!strncmp(src+to,"str",3)) fkind=K_STR; }              /* P2: &str field */
        skipws(); if(src[pos]=='<'){ int d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } }   /* R7: field type generic args e.g. Vec<T> -> a 4-byte descriptor pointer */
        sf_st[nsf]=st; sf_off[nsf]=fo; sf_len[nsf]=fl; sf_boff[nsf]=off; sf_styp[nsf]=fstyp; sf_fkind[nsf]=fkind; nsf++;
        off += (fstyp>=0)? ((ssize[fstyp]+3)/4*4) : ((tgt_darwin&&fkind)?8:4);   /* T39: Vec/String/&str desc fields widen to 8B */
        skipws(); if(eat(","))continue; if(src[pos]=='}'){pos++;break;} die("expected `,` or `}` in struct");
    }
    ssize[st]=off;
}

/* pre-pass: `enum Name { A, B(i32), C(i32,i32) }` -> variants with discriminants + payload field counts */
static void parse_enum_decl(void){
    int o,l,en,disc=0,maxf=0;
    if(!rd_ident(&o,&l)) die("expected enum name");
    en=nenum; enoff[en]=o; enlen[en]=l; nenum++;
    if(!eat("{")) die("expected `{` in enum");
    for(;;){ int vo,vl,nf=0; skipws(); if(src[pos]=='}'){pos++;break;}
        if(!rd_ident(&vo,&vl)) die("expected variant");
        if(eat("(")){ for(;;){ int to,tl; skipws(); if(src[pos]==')'){pos++;break;} rd_ident(&to,&tl); nf++; skipws(); if(eat(","))continue; if(src[pos]==')'){pos++;break;} } }
        ev_en[nev]=en; ev_off[nev]=vo; ev_len[nev]=vl; ev_disc[nev]=disc; ev_nfld[nev]=nf; nev++;
        if(nf>maxf) maxf=nf; disc++;
        skipws(); if(eat(","))continue; if(src[pos]=='}'){pos++;break;}
    }
    ensize[en] = maxf>0 ? (8+maxf*4) : 4;
}

/* rust-closure R0: std-subset prelude (compiled by rzrs itself) -- print helpers over the write
 * syscall. __putc writes the low byte of a scalar via `&x` (no byte array needed).
 * darwin lane: the syscall-carrying prelude lines exist in two target numberings (Linux x86-64
 * write=1/exit=60; Darwin arm64 write=4/exit=1 -- the *-darwin.c convention, T23). The prelude is
 * assembled in main() in the SAME line order either way, so the default (Linux) output stays
 * byte-identical. Select Darwin with argv[1] == --target=darwin. */
static const char *PRELUDE_IO_LINUX =
"fn __putc(c: i32) { let x = c; __syscall(1, 1, &x, 1); }\n"
"fn __print_str(p: i32, n: i32) { __syscall(1, 1, p, n); }\n";
static const char *PRELUDE_IO_DARWIN =
"fn __putc(c: i32) { let x = c; __syscall(4, 1, &x, 1); }\n"
"fn __print_str(p: i32, n: i32) { __syscall(4, 1, p, n); }\n";
static const char *PRELUDE_PANIC_LINUX =
"fn __panic() -> i32 { __syscall(60, 101, 0, 0); 0 }\n";
static const char *PRELUDE_PANIC_DARWIN =
"fn __panic() -> i32 { __syscall(1, 101, 0, 0); 0 }\n";
static const char *PRELUDE_A_LINUX =
"fn __print_uint(m: i32) { if m >= 10 { __print_uint(m / 10); } __putc(48 + m % 10); }\n"
"fn __print_int(n: i32) { let mut m = n; if m < 0 { __putc(45); m = 0 - m; } __print_uint(m); }\n"
"fn __print_uint64(m: i64) { if m >= 10 { __print_uint64(m / 10); } __putc(48 + (m % 10) as i32); }\n"
"fn __print_int64(n: i64) { let mut m: i64 = n; if m < 0 { __putc(45); m = 0 - m; } __print_uint64(m); }\n"
/* R3: Vec<i32> over the heap arena. A Vec = a {ptr@0, len@4, cap@8} descriptor; elements are a
 * separately-allocated i32 block that doubles on overflow (bump-allocated, never freed). */
"fn __vec_new() -> i32 { let d = __alloc(12); *(d + 0) = 0; *(d + 4) = 0; *(d + 8) = 0; d }\n"
"fn __vec_len(d: i32) -> i32 { *(d + 4) }\n"
"fn __vec_get(d: i32, i: i32) -> i32 { let p = *(d + 0); *(p + i * 4) }\n"
"fn __vec_set(d: i32, i: i32, x: i32) { let p = *(d + 0); *(p + i * 4) = x; }\n"
"fn __vec_push(d: i32, x: i32) { let len = *(d + 4); let cap = *(d + 8); if len == cap { let mut nc = cap * 2; if nc == 0 { nc = 4; } let np = __alloc(nc * 4); let op = *(d + 0); let mut i = 0; while i < len { *(np + i * 4) = *(op + i * 4); i = i + 1; } *(d + 0) = np; *(d + 8) = nc; } let p = *(d + 0); *(p + len * 4) = x; *(d + 4) = len + 1; }\n"
/* R4: built-in Option/Result as data enums + method helpers, all dogfooded in the Rust subset.
 * (disc: Option None=0 Some=1 ; Result Ok=0 Err=1 -- matched by emit_try / bare-constructor code.) */
"enum Option { None, Some(i32) }\n"
"enum Result { Ok(i32), Err(i32) }\n";
static const char *PRELUDE_A_DARWIN =
"fn __print_uint(m: i32) { if m >= 10 { __print_uint(m / 10); } __putc(48 + m % 10); }\n"
"fn __print_int(n: i32) { let mut m = n; if m < 0 { __putc(45); m = 0 - m; } __print_uint(m); }\n"
"fn __print_uint64(m: i64) { if m >= 10 { __print_uint64(m / 10); } __putc(48 + (m % 10) as i32); }\n"
"fn __print_int64(n: i64) { let mut m: i64 = n; if m < 0 { __putc(45); m = 0 - m; } __print_uint64(m); }\n"
/* T39 darwin vec: descriptor {ptr@0 (8-byte cell), len@8, cap@12}; ptr cells via __load8/__store8;
 * i64-typed ptr params so address arithmetic promotes to add8. */
"fn __vec_new() -> i32 { let d = __alloc(16); __store8(d, 0); *(d + 8) = 0; *(d + 12) = 0; d }\n"
"fn __vec_len(d: i64) -> i32 { *(d + 8) }\n"
"fn __vec_get(d: i64, i: i32) -> i32 { let p = __load8(d); *(p + i * 4) }\n"
"fn __vec_set(d: i64, i: i32, x: i32) { let p = __load8(d); *(p + i * 4) = x; }\n"
"fn __vec_push(d: i64, x: i32) { let len = *(d + 8); let cap = *(d + 12); if len == cap { let mut nc = cap * 2; if nc == 0 { nc = 4; } let np = __alloc(nc * 4); let op = __load8(d); let mut i = 0; while i < len { *(np + i * 4) = *(op + i * 4); i = i + 1; } __store8(d, np); *(d + 12) = nc; } let p = __load8(d); *(p + len * 4) = x; *(d + 8) = len + 1; }\n"
"enum Option { None, Some(i32) }\n"
"enum Result { Ok(i32), Err(i32) }\n";
static const char *PRELUDE_B_LINUX =
"fn __opt_is_some(o: Option) -> bool { match o { Some(_) => true, None => false } }\n"
"fn __opt_is_none(o: Option) -> bool { match o { Some(_) => false, None => true } }\n"
"fn __opt_unwrap(o: Option) -> i32 { match o { Some(x) => x, None => __panic() } }\n"
"fn __opt_unwrap_or(o: Option, d: i32) -> i32 { match o { Some(x) => x, None => d } }\n"
"fn __res_is_ok(r: Result) -> bool { match r { Ok(_) => true, Err(_) => false } }\n"
"fn __res_is_err(r: Result) -> bool { match r { Ok(_) => false, Err(_) => true } }\n"
"fn __res_unwrap(r: Result) -> i32 { match r { Ok(x) => x, Err(_) => __panic() } }\n"
"fn __res_unwrap_or(r: Result, d: i32) -> i32 { match r { Ok(x) => x, Err(_) => d } }\n"
"fn __res_unwrap_err(r: Result) -> i32 { match r { Ok(_) => __panic(), Err(e) => e } }\n"
"fn __vec_pop(d: i32) -> Option { let len = *(d + 4); if len == 0 { return None; } let p = *(d + 0); let v = *(p + (len - 1) * 4); *(d + 4) = len - 1; Some(v) }\n"
/* R2b: heap-growable String -- a {ptr@0, len@8, cap@16} byte buffer. ptr/len are laid out like a &str
 * descriptor, so a String's descriptor can be READ as a &str (Display, push_str source). */
"fn __str_new() -> i32 { let d = __alloc(24); *(d + 0) = 0; *(d + 8) = 0; *(d + 16) = 0; d }\n"
"fn __str_len(d: i32) -> i32 { *(d + 8) }\n"
"fn __str_reserve(d: i32, need: i32) { let cap = *(d + 16); if need > cap { let mut nc = cap * 2; if nc < need { nc = need; } if nc < 8 { nc = 8; } let np = __alloc(nc); let op = *(d + 0); let len = *(d + 8); let mut i = 0; while i < len { __storeb(np + i, __loadb(op + i)); i = i + 1; } *(d + 0) = np; *(d + 16) = nc; } }\n"
"fn __str_push_byte(d: i32, b: i32) { let len = *(d + 8); __str_reserve(d, len + 1); let p = *(d + 0); __storeb(p + len, b); *(d + 8) = len + 1; }\n"
"fn __str_push_str(d: i32, s: i32) { let sp = *(s + 0); let sl = *(s + 8); let mut i = 0; while i < sl { __str_push_byte(d, __loadb(sp + i)); i = i + 1; } }\n"
"fn __str_from(s: i32) -> i32 { let d = __str_new(); __str_push_str(d, s); d }\n";
static const char *PRELUDE_B_DARWIN =
"fn __opt_is_some(o: Option) -> bool { match o { Some(_) => true, None => false } }\n"
"fn __opt_is_none(o: Option) -> bool { match o { Some(_) => false, None => true } }\n"
"fn __opt_unwrap(o: Option) -> i32 { match o { Some(x) => x, None => __panic() } }\n"
"fn __opt_unwrap_or(o: Option, d: i32) -> i32 { match o { Some(x) => x, None => d } }\n"
"fn __res_is_ok(r: Result) -> bool { match r { Ok(_) => true, Err(_) => false } }\n"
"fn __res_is_err(r: Result) -> bool { match r { Ok(_) => false, Err(_) => true } }\n"
"fn __res_unwrap(r: Result) -> i32 { match r { Ok(x) => x, Err(_) => __panic() } }\n"
"fn __res_unwrap_or(r: Result, d: i32) -> i32 { match r { Ok(x) => x, Err(_) => d } }\n"
"fn __res_unwrap_err(r: Result) -> i32 { match r { Ok(_) => __panic(), Err(e) => e } }\n"
"fn __vec_pop(d: i64) -> Option { let len = *(d + 8); if len == 0 { return None; } let p = __load8(d); let v = *(p + (len - 1) * 4); *(d + 8) = len - 1; Some(v) }\n"
"fn __str_new() -> i32 { let d = __alloc(24); __store8(d, 0); *(d + 8) = 0; *(d + 16) = 0; d }\n"
"fn __str_len(d: i64) -> i32 { *(d + 8) }\n"
"fn __str_reserve(d: i64, need: i32) { let cap = *(d + 16); if need > cap { let mut nc = cap * 2; if nc < need { nc = need; } if nc < 8 { nc = 8; } let np = __alloc(nc); let op = __load8(d); let len = *(d + 8); let mut i = 0; while i < len { __storeb(np + i, __loadb(op + i)); i = i + 1; } __store8(d, np); *(d + 16) = nc; } }\n"
"fn __str_push_byte(d: i64, b: i32) { let len = *(d + 8); __str_reserve(d, len + 1); let p = __load8(d); __storeb(p + len, b); *(d + 8) = len + 1; }\n"
"fn __str_push_str(d: i64, s: i64) { let sp = __load8(s); let sl = *(s + 8); let mut i = 0; while i < sl { __str_push_byte(d, __loadb(sp + i)); i = i + 1; } }\n"
"fn __str_from(s: i64) -> i32 { let d = __str_new(); __str_push_str(d, s); d }\n";

/* R2-linux: conditional-prelude use analysis. Scan src[lo..hi] for identifier tokens
 * matching prelude fn names and mark them needed. Also handle Rust surface features
 * that desugar (at parse/emit time) into prelude calls -- these must be detected at
 * the SOURCE level here because desugaring happens later, after this analysis.
 * Transitive-closure runs by iterating over prelude fns and rescanning their bodies. */
static void mark_needed_by_name(int o,int l){
    int i; for(i=0;i<nfn;i++) if(foff[i]<prelude_end && flen[i]==l && strncmp(src+foff[i],src+o,l)==0){ fn_needed[i]=1; return; } }
static void mark_needed_by_cstr(const char *nm){
    int l=(int)strlen(nm),i;
    for(i=0;i<nfn;i++) if(foff[i]<prelude_end && flen[i]==l && strncmp(src+foff[i],nm,l)==0){ fn_needed[i]=1; return; } }
static void scan_range_for_prelude_refs(int lo,int hi){
    int q=lo;
    while(q<hi){
        if(src[q]=='/' && q+1<hi && src[q+1]=='/'){ while(q<hi&&src[q]!='\n')q++; continue; }
        if(src[q]=='"'){ q++; while(q<hi && src[q]!='"'){ if(src[q]=='\\'&&q+1<hi)q++; q++; } if(q<hi)q++; continue; }
        if(isidst(src[q])){
            int s=q; while(q<hi&&isidc(src[q]))q++;
            { int len=q-s;
              /* direct prelude fn reference */
              mark_needed_by_name(s,len);
              /* surface -> prelude implications */
              if(len==7 && !strncmp(src+s,"println",7)){ mark_needed_by_cstr("__putc"); mark_needed_by_cstr("__print_str"); mark_needed_by_cstr("__print_uint"); mark_needed_by_cstr("__print_int"); mark_needed_by_cstr("__print_uint64"); mark_needed_by_cstr("__print_int64"); }
              else if(len==5 && !strncmp(src+s,"print",5)){ mark_needed_by_cstr("__putc"); mark_needed_by_cstr("__print_str"); mark_needed_by_cstr("__print_uint"); mark_needed_by_cstr("__print_int"); mark_needed_by_cstr("__print_uint64"); mark_needed_by_cstr("__print_int64"); }
              else if(len==3 && !strncmp(src+s,"Vec",3)){ mark_needed_by_cstr("__vec_new"); mark_needed_by_cstr("__vec_len"); mark_needed_by_cstr("__vec_get"); mark_needed_by_cstr("__vec_set"); mark_needed_by_cstr("__vec_push"); mark_needed_by_cstr("__vec_pop"); }
              else if(len==3 && !strncmp(src+s,"vec",3)){ mark_needed_by_cstr("__vec_new"); mark_needed_by_cstr("__vec_push"); }
              else if(len==6 && !strncmp(src+s,"String",6)){ mark_needed_by_cstr("__str_new"); mark_needed_by_cstr("__str_len"); mark_needed_by_cstr("__str_reserve"); mark_needed_by_cstr("__str_push_byte"); mark_needed_by_cstr("__str_push_str"); mark_needed_by_cstr("__str_from"); }
              else if(len==6 && !strncmp(src+s,"Option",6)){ mark_needed_by_cstr("__opt_is_some"); mark_needed_by_cstr("__opt_is_none"); mark_needed_by_cstr("__opt_unwrap"); mark_needed_by_cstr("__opt_unwrap_or"); mark_needed_by_cstr("__panic"); }
              else if(len==6 && !strncmp(src+s,"Result",6)){ mark_needed_by_cstr("__res_is_ok"); mark_needed_by_cstr("__res_is_err"); mark_needed_by_cstr("__res_unwrap"); mark_needed_by_cstr("__res_unwrap_or"); mark_needed_by_cstr("__res_unwrap_err"); mark_needed_by_cstr("__panic"); }
              else if((len==4&&!strncmp(src+s,"Some",4))||(len==4&&!strncmp(src+s,"None",4))||(len==2&&!strncmp(src+s,"Ok",2))||(len==3&&!strncmp(src+s,"Err",3))){
                  mark_needed_by_cstr("__opt_is_some"); mark_needed_by_cstr("__opt_is_none"); mark_needed_by_cstr("__opt_unwrap"); mark_needed_by_cstr("__opt_unwrap_or");
                  mark_needed_by_cstr("__res_is_ok"); mark_needed_by_cstr("__res_is_err"); mark_needed_by_cstr("__res_unwrap"); mark_needed_by_cstr("__res_unwrap_or"); mark_needed_by_cstr("__res_unwrap_err");
                  mark_needed_by_cstr("__panic"); }
              /* method-call desugarings: peek for `.push(` etc. before this ident is orthogonal --
               * we conservatively fire on the method-NAME appearing anywhere. */
              else if(len==4 && !strncmp(src+s,"push",4)){ mark_needed_by_cstr("__vec_push"); mark_needed_by_cstr("__str_push_byte"); mark_needed_by_cstr("__vec_new"); }
              else if(len==8 && !strncmp(src+s,"push_str",8)){ mark_needed_by_cstr("__str_push_str"); mark_needed_by_cstr("__str_push_byte"); mark_needed_by_cstr("__str_reserve"); }
              else if(len==3 && !strncmp(src+s,"pop",3)){ mark_needed_by_cstr("__vec_pop"); }
              else if(len==3 && !strncmp(src+s,"len",3)){ mark_needed_by_cstr("__vec_len"); mark_needed_by_cstr("__str_len"); }
              else if(len==6 && !strncmp(src+s,"unwrap",6)){ mark_needed_by_cstr("__opt_unwrap"); mark_needed_by_cstr("__res_unwrap"); mark_needed_by_cstr("__panic"); }
              else if(len==9 && !strncmp(src+s,"unwrap_or",9)){ mark_needed_by_cstr("__opt_unwrap_or"); mark_needed_by_cstr("__res_unwrap_or"); }
              else if(len==10 && !strncmp(src+s,"unwrap_err",10)){ mark_needed_by_cstr("__res_unwrap_err"); mark_needed_by_cstr("__panic"); }
              else if(len==7 && !strncmp(src+s,"is_some",7)){ mark_needed_by_cstr("__opt_is_some"); }
              else if(len==7 && !strncmp(src+s,"is_none",7)){ mark_needed_by_cstr("__opt_is_none"); }
              else if(len==5 && !strncmp(src+s,"is_ok",5)){ mark_needed_by_cstr("__res_is_ok"); }
              else if(len==6 && !strncmp(src+s,"is_err",6)){ mark_needed_by_cstr("__res_is_err"); }
              else if(len==4 && !strncmp(src+s,"from",4)){ mark_needed_by_cstr("__str_from"); mark_needed_by_cstr("__str_new"); mark_needed_by_cstr("__str_push_str"); mark_needed_by_cstr("__str_push_byte"); mark_needed_by_cstr("__str_reserve"); }
              else if(len==7 && !strncmp(src+s,"reserve",7)){ mark_needed_by_cstr("__str_reserve"); }
              else if(len==6 && !strncmp(src+s,"panic!",6)){ mark_needed_by_cstr("__panic"); }
            }
            continue;
        }
        /* postfix `?` -> Option/Result plumbing */
        if(src[q]=='?'){ mark_needed_by_cstr("__panic"); q++; continue; }
        q++;
    }
}
static void compute_prelude_needs(void){
    int i,changed;
    /* seed: scan user source (past prelude) */
    scan_range_for_prelude_refs(prelude_end, srclen);
    /* transitive close over prelude fn bodies */
    do{ changed=0;
        for(i=0;i<nfn;i++) if(foff[i]<prelude_end && fn_needed[i]){
            /* find this fn's body { ... } and rescan */
            int p2=foff[i]+flen[i], d;
            while(p2<prelude_end && src[p2]!='{') p2++;
            if(p2>=prelude_end) continue;
            { int bs=p2+1, be, dd=1; be=bs;
              while(be<prelude_end && dd){ if(src[be]=='{')dd++; else if(src[be]=='}'){dd--; if(!dd)break;} be++; }
              (void)d;
              /* snapshot & rescan */
              { int before=0,after=0,j; for(j=0;j<nfn;j++) if(foff[j]<prelude_end) before+=fn_needed[j];
                scan_range_for_prelude_refs(bs,be);
                for(j=0;j<nfn;j++) if(foff[j]<prelude_end) after+=fn_needed[j];
                if(after>before) changed=1; }
            }
        }
    } while(changed);
}

int main(int argc, char **argv){
    int p,o,l,seen_main=0,plen;
    const char *pio, *ppanic, *pa, *pb;
    int lio, la, lpanic, lb;
    pio=PRELUDE_IO_LINUX; ppanic=PRELUDE_PANIC_LINUX; pa=PRELUDE_A_LINUX; pb=PRELUDE_B_LINUX;
    if(argc>1 && strcmp(argv[1],"--target=darwin")==0){ pio=PRELUDE_IO_DARWIN; ppanic=PRELUDE_PANIC_DARWIN; pa=PRELUDE_A_DARWIN; pb=PRELUDE_B_DARWIN; tgt_darwin=1; }
    lio=(int)strlen(pio); la=(int)strlen(pa); lpanic=(int)strlen(ppanic); lb=(int)strlen(pb);
    plen=lio+la+lpanic+lb;
    srclen=(int)fread(src+plen,1,sizeof(src)-1-plen,stdin);
    memcpy(src,pio,lio); memcpy(src+lio,pa,la);
    memcpy(src+lio+la,ppanic,lpanic); memcpy(src+lio+la+lpanic,pb,lb);
    srclen+=plen; src[srclen]=0; pos=0; prelude_end=plen;
    if(eat_kw("module")){ int mo,ml; rd_ident(&mo,&ml); }

    /* pre-pass 1: struct + enum decls (in order, so later types can nest earlier ones); skip impl/trait blocks */
    p=pos;
    while(pos<srclen){ if(eat_kw("struct")) parse_struct_decl(); else if(eat_kw("enum")) parse_enum_decl();
        else if(eat_kw("impl")||eat_kw("trait")){ int d; while(pos<srclen&&src[pos]!='{')pos++; d=0; while(pos<srclen){ if(src[pos]=='{')d++; else if(src[pos]=='}'){d--;pos++;if(d==0)break;continue;} pos++; } }
        else if(eat_kw("fn")){ int d; rd_ident(&o,&l); while(pos<srclen&&src[pos]!='{')pos++; d=0; while(pos<srclen){ if(src[pos]=='{')d++; else if(src[pos]=='}'){d--;pos++;if(d==0)break;continue;} pos++; } } else skip_tok(); }
    pos=p;
    { int i; for(i=0;i<nenum;i++){ if(enlen[i]==6&&!strncmp(src+enoff[i],"Option",6))opt_en=i; if(enlen[i]==6&&!strncmp(src+enoff[i],"Result",6))res_en=i; } }   /* R4: resolve built-in enum indices */
    /* pre-pass 2: FREE fn names + GENERIC fn registry (skip impl/trait method bodies) */
    while(pos<srclen){ if(eat_kw("impl")||eat_kw("trait")){ int d; while(pos<srclen&&src[pos]!='{')pos++; d=0; while(pos<srclen){ if(src[pos]=='{')d++; else if(src[pos]=='}'){d--;pos++;if(d==0)break;continue;} pos++; } }
        else if(eat_kw("fn")){ if(rd_ident(&o,&l)){ skipws();
            if(src[pos]=='<'){ int gi=ngf,d; gf_off[gi]=o; gf_len[gi]=l; pos++; skipws(); rd_ident(&gf_tpoff[gi],&gf_tplen[gi]);   /* generic fn: record name + first type param + sig pos */
                d=1; while(pos<srclen&&d>0){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } skipws(); gf_sig[gi]=pos;
                { int sp=pos+1,pidx=0; gf_tparg[gi]=0;              /* scan params: find the first param whose type is T */
                  while(sp<srclen && src[sp]!=')'){
                      while(sp<srclen&&(src[sp]==' '||src[sp]=='\t'||src[sp]=='\n'||src[sp]==',')){ if(src[sp]==',')pidx++; sp++; }
                      while(sp<srclen&&isidc(src[sp]))sp++;         /* param name */
                      while(sp<srclen&&(src[sp]==' '||src[sp]==':'||src[sp]=='\t'))sp++;
                      if(src[sp]=='&'){ sp++; while(sp<srclen&&src[sp]==' ')sp++; if(!strncmp(src+sp,"mut",3)&&!isidc(src[sp+3])){sp+=3;while(sp<srclen&&src[sp]==' ')sp++;} }
                      { int to=sp; while(sp<srclen&&isidc(src[sp]))sp++;
                        if(sp-to==gf_tplen[gi]&&!strncmp(src+to,src+gf_tpoff[gi],sp-to)){ gf_tparg[gi]=pidx; break; } }
                      while(sp<srclen&&src[sp]!=','&&src[sp]!=')')sp++;
                  } }
                ngf++; }
            else { int fi=nfn; foff[fi]=o;flen[fi]=l;fret64[fi]=0;fretchar[fi]=0;fretstr[fi]=0;fretenum[fi]=-1;fretstring[fi]=0;nfn++;   /* free fn: also capture the return type */
                while(pos<srclen&&src[pos]!='(')pos++; { int d=0; while(pos<srclen){ if(src[pos]=='(')d++; else if(src[pos]==')'){d--;pos++;if(d==0)break;continue;} pos++; } }
                skipws(); if(pos+1<srclen&&src[pos]=='-'&&src[pos+1]=='>'){ int isref2=0; pos+=2; skipws(); if(eat("&")){ isref2=1; if(src[pos]=='\''){ pos++; { int lo,ll; rd_ident(&lo,&ll); } skipws(); } }
                    { int to,tl; if(rd_ident(&to,&tl)){ if(tl==3&&(!strncmp(src+to,"i64",3)||!strncmp(src+to,"u64",3)))fret64[fi]=1; if(tl==4&&!strncmp(src+to,"char",4))fretchar[fi]=1; if(isref2&&tl==3&&!strncmp(src+to,"str",3))fretstr[fi]=1;
                        if(tl==6&&!strncmp(src+to,"Option",6))fretenum[fi]=opt_en; if(tl==6&&!strncmp(src+to,"Result",6))fretenum[fi]=res_en;
                        if(tl==6&&!strncmp(src+to,"String",6))fretstring[fi]=1; } } } } } }   /* R4: Option/Result ; R2b: String return type */
        else skip_tok(); }
    pos=p;
    /* R7 pre-pass: impl-method return types -> registry (for `Type::assoc()` result typing + method-call result typing) */
    while(pos<srclen){ if(eat_kw("impl")){ int a,al,b,bl,tyo,tyl,d;
            skipws(); if(src[pos]=='<'){ d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } }   /* `impl<T>` -- skip leading generic params */
            if(rd_ident(&a,&al)){ skipws(); if(src[pos]=='<'){ d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } }
                skipws(); if(at_kw("for")){ eat_kw("for"); rd_ident(&b,&bl); tyo=b; tyl=bl; } else { tyo=a; tyl=al; }
                skipws(); if(src[pos]=='<'){ d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } }
                if(eat("{")){ for(;;){ skipws(); if(src[pos]=='}'){pos++;break;} if(pos>=srclen)break;
                    if(eat_kw("fn")){ int mo,ml,rs=-1; if(rd_ident(&mo,&ml)){ if(peek_is('<')){ d=0; while(pos<srclen){ if(src[pos]=='<')d++; else if(src[pos]=='>'){d--;pos++;if(d==0)break;continue;} pos++; } }
                        while(pos<srclen&&src[pos]!='(')pos++; { d=0; while(pos<srclen){ if(src[pos]=='(')d++; else if(src[pos]==')'){d--;pos++;if(d==0)break;continue;} pos++; } }   /* skip params */
                        skipws(); if(src[pos]=='-'&&src[pos+1]=='>'){ int ro,rl; pos+=2; skipws(); if(eat("&")){ if(src[pos]=='\''){ pos++; { int lo,ll; rd_ident(&lo,&ll); } skipws(); } }
                            if(rd_ident(&ro,&rl)){ if(rl==4&&!strncmp(src+ro,"Self",4)) rs=struct_find(tyo,tyl); else rs=struct_find(ro,rl); } }
                        m_toff[nmeth]=tyo; m_tlen[nmeth]=tyl; m_moff[nmeth]=mo; m_mlen[nmeth]=ml; m_retstyp[nmeth]=rs; nmeth++;
                        { d=0; while(pos<srclen){ if(src[pos]=='{')d++; else if(src[pos]=='}'){d--;pos++;if(d==0)break;continue;} pos++; } } } }   /* skip body */
                    else skip_tok(); } } } }
        else skip_tok(); }
    pos=p;

    /* selfhost P0 pre-pass: register module-level `static [mut] NAME: TYPE = INIT;` */
    while(pos<srclen){ if(eat_kw("static")){ int no,nl,esz=4,is64=0,kind=0,bytes=4,initv=0;
            eat_kw("mut"); if(!rd_ident(&no,&nl)) die("expected static name"); skipws(); eat(":"); skipws();
            if(src[pos]=='['){ int to,tl,n=0; pos++; skipws(); rd_ident(&to,&tl);
                if(tl==3&&(!strncmp(src+to,"i64",3)||!strncmp(src+to,"u64",3))){ esz=8; is64=1; } else if(tl==2&&(!strncmp(src+to,"u8",2)||!strncmp(src+to,"i8",2))){ esz=1; } else esz=4;
                skipws(); eat(";"); skipws(); while(src[pos]>='0'&&src[pos]<='9'){ n=n*10+(src[pos]-'0'); pos++; } skipws(); eat("]");
                kind=1; bytes=n*esz; }
            else { int to,tl; rd_ident(&to,&tl); if(tl==3&&(!strncmp(src+to,"i64",3)||!strncmp(src+to,"u64",3))){ esz=8; is64=1; bytes=8; } else { esz=4; bytes=4; } }
            skipws(); if(eat("=")){ skipws();                                     /* const init (Rust requires const); only non-zero SCALAR init emits a store */
                if(src[pos]=='['){ int d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='[')d++; else if(src[pos]==']')d--; pos++; } }   /* [0; N] etc -> zeroed by gdef */
                else { int neg=0,v=0; if(src[pos]=='-'){ neg=1; pos++; } while(src[pos]>='0'&&src[pos]<='9'){ v=v*10+(src[pos]-'0'); pos++; } while(pos<srclen&&isidc(src[pos]))pos++; initv=neg?-v:v; } }
            skipws(); eat(";");
            g_off[nglob]=no; g_len[nglob]=nl; g_kind[nglob]=kind; g_esz[nglob]=esz; g_bytes[nglob]=bytes<4?4:bytes; g_is64[nglob]=is64; g_init[nglob]=initv; nglob++; }
        else skip_tok(); }
    pos=p;
    printf("gdef __str_arena 16777216\ngdef __str_cursor 8\n");   /* R2: global arena for string literals (survives fn returns) */
    printf("gdef __heap 1048576\ngdef __heap_cursor 8\n");      /* R3: global heap arena for Vec descriptors + element blocks (__alloc bump) */
    { int i; for(i=0;i<nglob;i++){ printf("gdef "); fwrite(src+g_off[i],1,g_len[i],stdout); printf(" %d\n",g_bytes[i]); } }   /* selfhost P0: static storage (zeroed) */
    /* emit main first */
    while(pos<srclen){ if(eat_kw("fn")){ int fo,fl,q=pos; if(rd_ident(&fo,&fl)&&span_eq(fo,fl,"main")){ pos=q; parse_fn(1,-1,0); seen_main=1; break; } } else skip_tok(); }
    if(!seen_main) die("no `fn main`");
    pos=p;
    compute_prelude_needs();   /* R2-linux: seed fn_needed[] from user source + close transitively over prelude bodies */
    /* emit helpers (skip main, skip struct/enum decls, skip UNUSED prelude fns), then impl methods */
    while(pos<srclen){
        if(eat_kw("fn")){ int fo,fl,q=pos; if(rd_ident(&fo,&fl)){ int isgen=gf_find(fo,fl)>=0;
            /* R2-linux: prelude fn with foff<prelude_end AND not marked needed -> skip body */
            int is_prelude=(fo<prelude_end);
            int fi_dbg=fn_index(fo,fl);
            int skip_unused_prelude = is_prelude && fi_dbg>=0 && !fn_needed[fi_dbg];
            if(!span_eq(fo,fl,"main") && !isgen && !skip_unused_prelude){ pos=q; parse_fn(0,-1,0); }
            else { pos=q; rd_ident(&fo,&fl); if(peek_is('<')){ int d=0; while(pos<srclen){ if(src[pos]=='<')d++; else if(src[pos]=='>'){d--;pos++;if(d==0)break;continue;} pos++; } }  /* skip main OR generic fn body (generics emit as instantiations) OR unused prelude */
                { int d=0; while(pos<srclen){ if(src[pos]=='{')d++; else if(src[pos]=='}'){d--;pos++;if(d==0)break;continue;} pos++; } } } } }
        else if(eat_kw("impl")){                              /* impl [Trait for] Type { fn method... } -> emit each method as `Type_method` */
            int a,al,b,bl,to,tl; skipws(); if(src[pos]=='<'){ int d=1; pos++; while(pos<srclen&&d){ if(src[pos]=='<')d++; else if(src[pos]=='>')d--; pos++; } }   /* R7: `impl<T>` -- skip the leading generic params */
            if(!rd_ident(&a,&al)) die("expected impl type");
            if(at_kw("for")){ eat_kw("for"); if(!rd_ident(&b,&bl)) die("expected type after `for`"); to=b; tl=bl; } else { to=a; tl=al; }
            if(peek_is('<')){ int d=0; while(pos<srclen){ if(src[pos]=='<')d++; else if(src[pos]=='>'){d--;pos++;if(d==0)break;continue;} pos++; } }
            if(!eat("{")) die("expected `{` in impl");
            for(;;){ skipws(); if(src[pos]=='}'){pos++;break;} if(pos>=srclen)die("unterminated impl");
                if(eat_kw("fn")) parse_fn(0,to,tl); else pos++;
            }
        }
        else if(eat_kw("struct")||eat_kw("enum")||eat_kw("trait")){ int d=0; while(pos<srclen){ if(src[pos]=='{')d++; else if(src[pos]=='}'){d--;pos++;if(d==0)break;continue;} pos++; } }
        else skip_tok();
    }
    /* emit each generic-fn instantiation (monomorphization); the loop covers instantiations spawned by earlier ones */
    { int k=0; while(k<ninst){ emit_generic_inst(inst_gf[k], inst_styp[k]); k++; } }
    /* R6: emit each lifted closure body (may itself register more closures -> drain the growing list) */
    { int k=0; while(k<nclo){ emit_closure_body(k); k++; } }
    printf("end\n");
    return 0;
}
