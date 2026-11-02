// S1 parser (re-architecture stage S1 — lex + parse + real AST). Grows pipeline by pipeline.
// A REAL AST data model (not flag banks); dump() emits the canonical AST observable diffed against
// rustc's -Zunpretty=ast-tree (transcribed to canonical in goldens/s1-ast.txt via s1-ast-diff.sh).
// Authored in the rzrs dialect the frozen stage0 (L5) compiles; modeled on Layer-7 expr_parser.rs.
//
// Coverage: AST-0 const item; AST-1 Path; AST-2 R1 Ref/Ptr/Array/Slice; AST-2 R2 Tup + generic args;
// AST-4 R2 exprs (binops/unary/paren/path/cmp); AST-6 R1 items-as-enum + struct item with named fields.
// NOTE: braces { } in comments are now fine -- the L6-hardening arc resolved that defect (a). Example: { }.

use std::io::Read;   // AST-10: read_to_string on stdin (rustc-native; rzrs lowers to syscall(0) via L6 STDIN-READ)

type P<T> = Box<T>;

#[derive(Clone)]
enum Tok { Pub, Const, Struct, Enum, Fn, Let, If, Else, Match, Type, Use, Mod, Impl, Trait, Where, As, Mut, Unsafe, While, Loop, For, In, Hash, Colon, PathSep, Eq, EqEq, Ne, FatArrow, Pipe, OrOr, AndAnd, Semi, Amp, Star, Plus, PlusEq, Minus, Arrow, Slash, Percent, Caret, Dot, DotDot, DotDotEq, LBrack, RBrack, LParen, RParen, LBrace, RBrace, Comma, Lt, Gt, Le, Ge, Shl, Shr, Question, At, Bang, Ident(i64), Int(i64), Float(String), Str(String), Char(String), Lifetime(String), End }

struct PathSeg { ident: String, args: Vec<Ty>, paren: bool }   // paren = `Fn(A, B) -> R` parenthesized trait-arg form
struct Path { segments: Vec<PathSeg> }
// A `use` tree: a prefix path + one of Simple(rename or "") / Glob (`*`) / Nested (`{ .. }`), mirroring rustc's UseTreeKind.
struct UseTree { prefix: Path, kind: UseKind }
enum UseKind { Simple(String), Glob, Nested(Vec<UseTree>) }
// AST-2 R3: Ref carries an optional lifetime (`&'a T`) as a String ("" = elided). Ty::Lt is a lifetime in
// type-arg position (`Chars<'a>`). Lifetimes are pervasive in the real front-end crates.
enum Ty { Path(P<Path>), Ref(String, bool, P<Ty>), Ptr(bool, P<Ty>), Array(P<Ty>, P<ExprK>), Slice(P<Ty>), Tup(Vec<Ty>), Lt(String), ImplTrait(Vec<Path>), DynTrait(Vec<Path>), Assoc(String, P<Ty>), AssocBound(String, P<Ty>), FnPtr(Vec<Ty>, Option<P<Ty>>), Never, QPath(P<Ty>, P<Path>, Vec<PathSeg>), FnPtrExt(String, P<Ty>), CVarArgs, ConstArg(bool, Vec<String>) }   // FnPtrExt = unsafe/extern-abi prefix on a wrapped FnPtr; CVarArgs = `...` terminal fn-ptr param; ConstArg = `Foo<3>`/`Foo<{..}>` const generic arg (small variants: no existing variant enlarged)
// If's else is a Vec<ExprK> of 0 or 1 (the else expr: a Block-expr or an else-if). Vec-of-0-or-1 models the
// optional else WITHOUT an Option field matched by-ref (which trips the filed by-ref-Option L6 defect) -- the
// same faithful trick as Block.stmts. Block(P<Block>) is a block used as an expr.
enum ExprK { LitInt(i64), LitFloat(String), Path(P<Path>), Bin(String, P<ExprK>, P<ExprK>), Unary(String, P<ExprK>), Paren(P<ExprK>), Field(P<ExprK>, String), Method(P<ExprK>, String, Vec<Ty>, Vec<ExprK>), Call(P<ExprK>, Vec<ExprK>), Index(P<ExprK>, P<ExprK>), If(P<ExprK>, P<Block>, Vec<ExprK>), Block(P<Block>), Unsafe(P<Block>), Match(P<ExprK>, Vec<Arm>), Str(String), Char(String), Ref(bool, P<ExprK>), Deref(P<ExprK>), Cast(P<ExprK>, P<Ty>), Closure(bool, Vec<CParam>, Vec<Ty>, P<ExprK>), Array(Vec<ExprK>), Tuple(Vec<ExprK>), Range(Option<P<ExprK>>, Option<P<ExprK>>, bool), While(String, P<ExprK>, P<Block>), Loop(String, P<Block>), ForLoop(String, P<Pat>, P<ExprK>, P<Block>), Assign(P<ExprK>, P<ExprK>), AssignOp(String, P<ExprK>, P<ExprK>), Struct(P<Path>, Vec<EField>, Option<P<ExprK>>), Try(P<ExprK>), Await(P<ExprK>), Bool(bool), Let(P<Pat>, P<ExprK>), Ret(Vec<ExprK>), Break(String, Vec<ExprK>), Continue(String), MacCall(P<Path>, String, Vec<String>), QPath(P<Ty>, P<Path>, Vec<PathSeg>), Repeat(P<ExprK>, P<ExprK>), Async(bool, P<Block>), ConstBlock(P<Block>), RawRef(bool, P<ExprK>), TryBlock(P<Block>) }   // `<T as Trait>::method`; Repeat `[v; n]`; Async `async [move] {..}`; ConstBlock `const {..}`; RawRef `&raw const/mut e`

fn binop_bp(t: &Tok) -> i64 { match t { Tok::OrOr => 1, Tok::AndAnd => 2, Tok::EqEq => 3, Tok::Ne => 3, Tok::Lt => 3, Tok::Gt => 3, Tok::Le => 3, Tok::Ge => 3, Tok::Pipe => 4, Tok::Caret => 5, Tok::Amp => 6, Tok::Shl => 7, Tok::Shr => 7, Tok::Plus => 8, Tok::Minus => 8, Tok::Star => 9, Tok::Slash => 9, Tok::Percent => 9, _ => 0 } }   // rustc precedence: || < && < cmp < | < ^ < & < <<>> < +/- < */%
fn binop_name(t: &Tok) -> String { match t { Tok::OrOr => String::from("Or"), Tok::AndAnd => String::from("And"), Tok::EqEq => String::from("Eq"), Tok::Ne => String::from("Ne"), Tok::Lt => String::from("Lt"), Tok::Gt => String::from("Gt"), Tok::Le => String::from("Le"), Tok::Ge => String::from("Ge"), Tok::Pipe => String::from("BitOr"), Tok::Caret => String::from("BitXor"), Tok::Amp => String::from("BitAnd"), Tok::Shl => String::from("Shl"), Tok::Shr => String::from("Shr"), Tok::Plus => String::from("Add"), Tok::Minus => String::from("Sub"), Tok::Star => String::from("Mul"), Tok::Slash => String::from("Div"), Tok::Percent => String::from("Rem"), _ => String::from("?") } }

struct Field { is_pub: bool, ident: String, ty: P<Ty> }
enum VariantKind { Unit, Tuple(Vec<Ty>), Struct(Vec<Field>) }
struct Variant { ident: String, kind: VariantKind, disr: Option<P<ExprK>> }   // disr = explicit discriminant `= <const-expr>` (rustc Variant.disr_expr)
struct Param { name: String, ty: P<Ty> }
// AST-4 R4f: closure param. A tiny struct-wrapper (not a bare `Vec<..>`) because L6 mis-displays a bare
// element of a match-payload Vec as a garbage pointer (filed defect g); a FIELD of a struct element displays
// correctly. R42: the param is a real Pat (rustc closure params ARE patterns), so `|&n|`/`|(a,b)|`/`|_|`/
// `|mut x|` parse via parse_pat_single and render via dump_pat (== rustc), not a bare ident string.
struct CParam { pat: Pat, ty: Vec<Ty> }   // AST-4: closure param pattern, with an OPTIONAL type annotation (`|x: u32|`) as a 0-or-1 Vec
struct EField { name: String, expr: P<ExprK> }
// AST-3 R1: irrefutable let patterns (Wild/Ident/Tuple). Refutable pats (Lit/Or) + Ref/TupleStruct/Struct
// arrive with match arms (AST-4 R3+). No Option/Box-struct-field here, so the filed L6 defects are not tripped.
// AST-3 R2: refutable pats (Lit/TupleStruct/Or) join the irrefutable ones for match arms. Single-segment
// tuple-struct/variant paths for R2; multi-segment path patterns (`E::V`) + struct pats (`S { f }`) deferred.
enum Pat { Wild, Ident(String), Tuple(Vec<Pat>), Lit(P<ExprK>), TupleStruct(P<Path>, Vec<Pat>), Or(Vec<Pat>), Path(P<Path>), Struct(P<Path>, Vec<FieldPat>, bool), Range(P<ExprK>, P<ExprK>, bool), Ref(P<Pat>), Bind(String, String), At(String, P<Pat>), Slice(Vec<Pat>), Rest }
// AST-3 R4: struct pattern field. shorthand `x` -> pat = Ident(x) + shorthand=true (== rustc PatField is_shorthand);
// explicit `x: subpat` -> shorthand=false. The bool on Pat::Struct is the `..` rest.
struct FieldPat { name: String, pat: P<Pat>, shorthand: bool }
struct Arm { pat: P<Pat>, guard: Option<P<ExprK>>, body: P<ExprK> }
// A block is a stmt sequence (rustc Block.stmts). A trailing no-semi expr is a Stmt::Expr (the block value);
// a semi'd expr is Stmt::Semi. Let carries a REQUIRED type (optional-type let deferred to the by-ref-Option-match
// L6 fix rung, since an `Option<P<Ty>>` matched by-ref trips the filed disc-misread defect).
enum Stmt { Let(P<Pat>, Option<P<Ty>>, Option<P<ExprK>>, Option<P<Block>>), Expr(P<ExprK>), Semi(P<ExprK>), Item(P<Item>), Empty }   // Item = a nested item in a block (rustc StmtKind::Item); Empty = a bare `;`
struct Block { stmts: Vec<Stmt> }
// AST-6 R2: fn ret is now Option<P<Ty>> matched by-ref -- no-ret fns (`fn f() {}`) render Ret Default
// (== rustc output: Default), typed rets render Ret <ty> (== output: Ty). Unblocked by the L6-hardening
// arc (defect b, by-ref Option<...> match discriminant, now fixed); the required-ret R1 workaround is retired.
// AST-7 R1: generics threaded through the ItemKind PAYLOAD (a Vec<GParam> leading each of Struct/Enum/Fn),
// NOT as a top-level Item field -- so the dump iterates a bound match-payload Vec (the proven dump_arms shape),
// sidestepping filed defect (d). GParam = a type param with trait bounds (`<T: A + B>`).
struct GParam { name: String, bounds: Vec<Path>, is_lt: bool, is_const: bool, deflt: String }   // is_const: `<const N: T>` param (bounds[0] holds T). deflt: `<T = u8>` default type param, "" if none. Lifetime/type outlives bounds ride in `bounds` as a single-seg Path whose ident carries the leading tick (`'a`).
// AST-8 R1: outer attributes retained on items, threaded through the ItemKind PAYLOAD (a leading Vec<Attr> on
// Struct/Fn) -- defect-d-safe. args = a raw comma-joined ident string (string-literal args deferred; the lexer
// has no string literals yet). Attrs on other item kinds are an R1 gap.
struct Attr { path: P<Path>, args: String }
// AST-6 R4: TypeAlias/Use/Mod. Mod's nested items are a Vec<Item> PAYLOAD (iterated + dump_item recursively --
// payload-threaded, defect-d-safe). Impl (inherent) is a sub-rung.
enum ItemKind { Const(P<Ty>, Option<P<ExprK>>), Struct(Vec<GParam>, Vec<Field>), TupleStruct(Vec<GParam>, Vec<Ty>), UnitStruct(Vec<GParam>), Enum(Vec<GParam>, Vec<Variant>), Fn(Vec<GParam>, Vec<Param>, Option<P<Ty>>, Option<P<Block>>, bool), TypeAlias(Vec<GParam>, P<Ty>), AssocTyDecl(Vec<GParam>), Use(P<UseTree>), Mod(Vec<Item>), Impl(Vec<GParam>, Option<P<Ty>>, P<Ty>, Vec<Item>), Trait(Vec<GParam>, Vec<Item>), TraitAlias(Vec<GParam>, Vec<Path>), AutoTrait(Vec<GParam>, Vec<Item>), ForeignMod(Vec<Item>), ExternCrate, Static(P<Ty>, Option<P<ExprK>>), Union(Vec<GParam>, Vec<Field>), MacCall(P<ExprK>) }   // item-position `name! { .. }` -> rustc ItemKind::MacCall; the P<ExprK> is the reused ExprK::MacCall token carrier
// AST-8 R2: attrs are a SHARED Item field (lifted off the Struct/Fn payloads) so EVERY item kind carries outer
// attributes uniformly. dump_item emits them right after the item header, before the kind's fields (== rustc, attrs
// as an Item field before the kind body). The Vec<Attr> moves into the Item built by whichever parse_item arm runs.
// AST-7 R2: a where-predicate `Ty: bound + bound` (rustc WhereBoundPredicate). Attached to the Item (rustc puts the
// WhereClause in Generics); rendered after the generics on the header.
struct WherePred { ty: P<Ty>, bounds: Vec<Path> }
struct Item { vis: String, ident: String, attrs: Vec<Attr>, wheres: Vec<WherePred>, bounds: Vec<Path>, lo: i64, hi: i64, kind: ItemKind }
struct Krate { items: Vec<Item> }

fn is_idc(c: u8) -> bool { (c >= 97 && c <= 122) || (c >= 65 && c <= 90) || (c >= 48 && c <= 57) || c == 95 || c >= 128 }   // reduced XID: any UTF-8 non-ASCII byte (>=128) extends an ident (café/naïve/Ωmega/姓名)

// Byte-wise equality of two DYNAMIC strings. The rzrs subset's `==` on String only lowers against a
// compile-time `&'static str` literal (every existing use is `w == "kw"`); comparing two runtime String
// values (the MAC-0 registry lookup) must go through as_bytes(), the lexer's known-good idiom.
// Element-wise copy of a Vec<String>. Whole-Vec `.clone()` is NOT a lowered subset idiom (the only such use
// miscompiled to an empty vec); copying element String clones -- which ARE lowered -- is the safe form.
fn veccopy(v: &Vec<String>) -> Vec<String> {
    let mut o: Vec<String> = Vec::new();
    let mut i = 0;
    while i < v.len() { o.push(v[i].clone()); i = i + 1; }
    o
}
// Split a space-joined token string into its tokens. as_bytes + slice-to_string (the lexer's proven idiom).
fn split_ws(s: &str) -> Vec<String> {
    let bb = s.as_bytes();
    let n = bb.len();
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < n {
        while i < n && bb[i] == 32 { i = i + 1; }
        if i >= n { break; }
        let st = i;
        while i < n && bb[i] != 32 { i = i + 1; }
        let sl = &s[st..i];
        out.push(sl.to_string());
    }
    out
}
// Split a space-joined token string on TOP-LEVEL `,` tokens (commas inside ()[]{} do not split), returning the
// balanced parts as re-joined token strings. The MAC-3 multi-metavar separator model.
fn split_commas(s: &str) -> Vec<String> { split_bysep(s, ",") }
// Split a space-joined token string on the TOP-LEVEL `sep` token (balanced over ()[]{}), returning the parts as
// re-joined token strings. Empty sep -> one part per top-level token (the empty-separator repetition case).
fn split_bysep(s: &str, sep: &str) -> Vec<String> {
    let toks = split_ws(s);
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut d = 0;
    let mut i = 0;
    let n = toks.len();
    let emptysep = sep.len() == 0;
    while i < n {
        let t = &toks[i];
        let opens = streq(t, "(") || streq(t, "[") || streq(t, "{");
        let closes = streq(t, ")") || streq(t, "]") || streq(t, "}");
        if !emptysep && streq(t, sep) && d == 0 { out.push(cur); cur = String::new(); i = i + 1; continue; }
        if opens { d = d + 1; }
        if closes { d = d - 1; }
        if emptysep && d == 0 && !opens && !closes { out.push(t.to_string()); i = i + 1; continue; }
        if cur.len() > 0 { cur.push(' '); } cur.push_str(t); i = i + 1;
    }
    if !emptysep { out.push(cur); }
    out
}
fn streq(a: &str, b: &str) -> bool {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    if ab.len() != bb.len() { return false; }
    let mut i = 0;
    while i < ab.len() { if ab[i] != bb[i] { return false; } i = i + 1; }
    true
}
fn lex(src: &str) -> (Vec<Tok>, Vec<i64>, Vec<String>) {
    let b = src.as_bytes(); let mut i = 0; let mut out: Vec<Tok> = Vec::new(); let mut offs: Vec<i64> = Vec::new(); let mut idn: Vec<String> = Vec::new();
    if b.len() > 2 && b[0] == 35 && b[1] == 33 && b[2] != 91 { let mut sb = 2; while sb < b.len() && b[sb] != 10 { sb += 1; } i = sb; }   // shebang `#!/…` at BOF: rustc skips the line. An inner attr `#![…]` (b[2]==`[`=91) is NOT a shebang -- left for the lexer/parse_attrs.
    while i < b.len() { let c = b[i]; let ts = i as i64;
        if c == 32 || c == 10 || c == 9 { i += 1; continue; }
        if c == 58 { if i + 1 < b.len() && b[i + 1] == 58 { out.push(Tok::PathSep); offs.push(ts); i += 2; } else { out.push(Tok::Colon); offs.push(ts); i += 1; } continue; }
        if c == 61 { if i + 1 < b.len() && b[i + 1] == 61 { out.push(Tok::EqEq); offs.push(ts); i += 2; } else if i + 1 < b.len() && b[i + 1] == 62 { out.push(Tok::FatArrow); offs.push(ts); i += 2; } else { out.push(Tok::Eq); offs.push(ts); i += 1; } continue; }
        if c == 124 { if i + 1 < b.len() && b[i + 1] == 124 { out.push(Tok::OrOr); offs.push(ts); i += 2; } else { out.push(Tok::Pipe); offs.push(ts); i += 1; } continue; }
        if c == 33 { if i + 1 < b.len() && b[i + 1] == 61 { out.push(Tok::Ne); offs.push(ts); i += 2; } else { out.push(Tok::Bang); offs.push(ts); i += 1; } continue; }
        if c == 59 { out.push(Tok::Semi); offs.push(ts); i += 1; continue; }
        if c == 38 { if i + 1 < b.len() && b[i + 1] == 38 { out.push(Tok::AndAnd); offs.push(ts); i += 2; } else { out.push(Tok::Amp); offs.push(ts); i += 1; } continue; }
        if c == 42 { out.push(Tok::Star); offs.push(ts); i += 1; continue; }
        if c == 43 { if i + 1 < b.len() && b[i + 1] == 61 { out.push(Tok::PlusEq); offs.push(ts); i += 2; } else { out.push(Tok::Plus); offs.push(ts); i += 1; } continue; }
        if c == 45 { if i + 1 < b.len() && b[i + 1] == 62 { out.push(Tok::Arrow); offs.push(ts); i += 2; } else { out.push(Tok::Minus); offs.push(ts); i += 1; } continue; }
        if c == 47 && i + 1 < b.len() && b[i + 1] == 47 { i += 2; while i < b.len() && b[i] != 10 { i += 1; } continue; }   // `//` line comment (incl `///` doc for now) -- skip to EOL
        if c == 47 && i + 1 < b.len() && b[i + 1] == 42 { i += 2; while i + 1 < b.len() && !(b[i] == 42 && b[i + 1] == 47) { i += 1; } i += 2; continue; }   // `/* .. */` block comment
        if c == 47 { out.push(Tok::Slash); offs.push(ts); i += 1; continue; }
        if c == 37 { out.push(Tok::Percent); offs.push(ts); i += 1; continue; }
        if c == 94 { out.push(Tok::Caret); offs.push(ts); i += 1; continue; }
        if c == 63 { out.push(Tok::Question); offs.push(ts); i += 1; continue; }   /* ? try operator */
        if c == 64 { out.push(Tok::At); offs.push(ts); i += 1; continue; }   /* @ pattern binding */
        if c == 35 { out.push(Tok::Hash); offs.push(ts); i += 1; continue; }
        if c == 34 { let start = i + 1; i += 1; while i < b.len() && b[i] != 34 { if b[i] == 92 { i += 2; } else { i += 1; } } let content = &src[start..i]; i += 1; out.push(Tok::Str(content.to_string())); offs.push(ts); continue; }
        // char literal `'x'` / `'\x'` vs lifetime `'a` -- only lex a char when a closing `'` follows the (escaped) char
        if c == 39 { if i + 1 < b.len() && b[i + 1] == 92 {
            if i + 3 < b.len() && b[i + 2] == 117 && b[i + 3] == 123 {   // `'\u{ hex }'` unicode escape -- scan to `}` (the 2-char-escape path below assumes `\x`, dropping `{..}`)
                let start = i + 1; let mut j = i + 4; while j < b.len() && b[j] != 125 { j += 1; } j += 1;   // past `}`
                let content = &src[start..j]; i = j + 1;   // past the closing `'`
                out.push(Tok::Char(content.to_string())); offs.push(ts); continue; }
            if i + 5 < b.len() && b[i + 2] == 120 {   // `'\x41'` hex escape -- always `\x` + 2 hex digits (the 2-char-escape path below assumes `\` + 1 char, truncating `\x` and leaking the digits)
                let start = i + 1; i += 5; let content = &src[start..i]; i += 1;   // content `\x41`, then past the closing `'`
                out.push(Tok::Char(content.to_string())); offs.push(ts); continue; }
            let start = i + 1; i += 3; let content = &src[start..i]; i += 1; out.push(Tok::Char(content.to_string())); offs.push(ts); continue; }
            if i + 2 < b.len() && b[i + 2] == 39 { let content = &src[(i + 1)..(i + 2)]; out.push(Tok::Char(content.to_string())); offs.push(ts); i += 3; continue; }
            if i + 1 < b.len() && ((b[i + 1] >= 97 && b[i + 1] <= 122) || (b[i + 1] >= 65 && b[i + 1] <= 90) || b[i + 1] == 95) {   /* lifetime `'name` (tick + ident, no closing tick) -- distinct from a `'x'` char literal */
                i += 1; let lo = i; while i < b.len() && is_idc(b[i]) { i += 1; } let nm = &src[lo..i]; out.push(Tok::Lifetime(nm.to_string())); offs.push(ts); continue; }
            i += 1; continue; }
        if c == 46 { if i + 1 < b.len() && b[i + 1] == 46 { if i + 2 < b.len() && b[i + 2] == 61 { out.push(Tok::DotDotEq); offs.push(ts); i += 3; } else { out.push(Tok::DotDot); offs.push(ts); i += 2; } } else { out.push(Tok::Dot); offs.push(ts); i += 1; } continue; }
        if c == 91 { out.push(Tok::LBrack); offs.push(ts); i += 1; continue; }
        if c == 93 { out.push(Tok::RBrack); offs.push(ts); i += 1; continue; }
        if c == 40 { out.push(Tok::LParen); offs.push(ts); i += 1; continue; }
        if c == 41 { out.push(Tok::RParen); offs.push(ts); i += 1; continue; }
        if c == 123 { out.push(Tok::LBrace); offs.push(ts); i += 1; continue; }
        if c == 125 { out.push(Tok::RBrace); offs.push(ts); i += 1; continue; }
        if c == 44 { out.push(Tok::Comma); offs.push(ts); i += 1; continue; }
        if c == 60 && i + 1 < b.len() && b[i + 1] == 60 { out.push(Tok::Shl); offs.push(ts); i += 2; continue; }   // `<<` shift
        if c == 62 && i + 1 < b.len() && b[i + 1] == 62 { out.push(Tok::Shr); offs.push(ts); i += 2; continue; }   // `>>` shift OR two generic closes (split at parse time)
        if c == 60 { if i + 1 < b.len() && b[i + 1] == 61 { out.push(Tok::Le); offs.push(ts); i += 2; } else { out.push(Tok::Lt); offs.push(ts); i += 1; } continue; }
        if c == 62 { if i + 1 < b.len() && b[i + 1] == 61 { out.push(Tok::Ge); offs.push(ts); i += 2; } else { out.push(Tok::Gt); offs.push(ts); i += 1; } continue; }
        if c >= 48 && c <= 57 { let start = i;
            let mut radix: i64 = 10;
            if b[i] == 48 && i + 1 < b.len() {   // `0x`/`0o`/`0b` base prefix
                let p = b[i + 1];
                if p == 120 || p == 88 { radix = 16; i += 2; }
                else if p == 111 || p == 79 { radix = 8; i += 2; }
                else if p == 98 || p == 66 { radix = 2; i += 2; } }
            let mut is_float = false;
            if radix == 16 { while i < b.len() && ((b[i] >= 48 && b[i] <= 57) || (b[i] >= 97 && b[i] <= 102) || (b[i] >= 65 && b[i] <= 70) || b[i] == 95) { i += 1; } }
            else { while i < b.len() && ((b[i] >= 48 && b[i] <= 57) || b[i] == 95) { i += 1; } }   // digits + `_` separators
            if radix == 10 {
                if i + 1 < b.len() && b[i] == 46 && b[i + 1] >= 48 && b[i + 1] <= 57 {   // `.` DIGIT -> float frac (NOT `t.0`/`1..2`/`1.method`)
                    is_float = true; i += 1; while i < b.len() && ((b[i] >= 48 && b[i] <= 57) || b[i] == 95) { i += 1; } }
                else if i < b.len() && b[i] == 46 && !(i + 1 < b.len() && (b[i + 1] == 46 || (b[i + 1] >= 48 && b[i + 1] <= 57) || (b[i + 1] >= 97 && b[i + 1] <= 122) || (b[i + 1] >= 65 && b[i + 1] <= 90) || b[i + 1] == 95 || b[i + 1] >= 128)) {
                    // digitless trailing-dot float `1.` -- a `.` NOT followed by digit/`.`/ident-start (i.e. `}`/`)`/`]`/`;`/`,`/op/EOF). Consume the single `.` (advances i, no non-progress). `1..2`=range, `1.0`=frac, `1.foo`=field all excluded above.
                    is_float = true; i += 1; }
                if i < b.len() && (b[i] == 101 || b[i] == 69) {   // e/E exponent
                    is_float = true; i += 1; if i < b.len() && (b[i] == 43 || b[i] == 45) { i += 1; } while i < b.len() && b[i] >= 48 && b[i] <= 57 { i += 1; } } }
            let numend = i;
            while i < b.len() && is_idc(b[i]) { i += 1; }   // optional type suffix `u8`/`i32`/`usize`/`f64` (elided in the reduced dump)
            if is_float { let ftext = &src[start..numend]; out.push(Tok::Float(ftext.to_string())); offs.push(ts); }
            else { let mut n: i64 = 0; let mut k = start; if radix != 10 { k += 2; }
                let cap = lit_mul();
                let mut over = false;
                while k < numend { let ch = b[k]; if ch != 95 { let d = if ch >= 48 && ch <= 57 { (ch - 48) as i64 } else if ch >= 97 && ch <= 102 { (ch - 97 + 10) as i64 } else if ch >= 65 && ch <= 70 { (ch - 65 + 10) as i64 } else { 0 }; if n >= cap { over = true; } else { n = n * radix + d; if n >= cap { over = true; } } } k += 1; }
                // C-S1-LIT-OVERFLOW-REFUSE (locales/s1-literal-representation CL-1 residual, closed 2026-08-26).
                // A literal too large for the packing used to WRAP, silently and DIFFERENTLY in each build of this
                // parser: measured on `9223372036854775808`, an L6-built s1_parser produced -123372036854775808
                // and a SEM0-built one produced an empty dump; on a real 39-digit `_i128` literal from rustc's
                // own test suite they produced -396250801992368000 and 0. That is the worst shape a limit can
                // take -- two tiers of the same compiler silently disagreeing on a VALUE. rustc rejects the
                // unsuffixed form outright ("literal out of range"), so refusing is also the closer behaviour.
                // The bound is lit_mul() because a literal at or above it aliases into the suffix band, which is
                // the same representational limit CL-1 records; refusing converts that residual from a silent
                // wrong answer into a clean one.
                if over { eprintln!("error: integer literal out of range for this tier (>= {}) @{}", cap, ts); std::process::exit(1); }
                let mut packed: i64 = n; if i > numend { packed = n + suffix_offset_of(&src[numend..i]); } out.push(Tok::Int(packed)); offs.push(ts); }
            continue; }
        // byte literal prefix `b'x'` / `b"..."` / `br"..."` / `br#"..."#` -- consume the `b` and delegate to the
        // char/string/raw-string lexer (byteness elided; the Char/Str node shape is preserved). Precedes the ident lexer.
        if c == 98 && i + 1 < b.len() && (b[i + 1] == 39 || b[i + 1] == 34 || (b[i + 1] == 114 && i + 2 < b.len() && (b[i + 2] == 34 || b[i + 2] == 35))) {
            i += 1; continue;
        }
        // C-string literal prefix `c"..."` / `cr"..."` / `cr#"..."#` -- same reduced projection as the `b` prefix above:
        // consume the `c` and delegate to the string/raw-string lexer (CStr-ness elided; the Str node shape is preserved).
        // Without this, `c` lexes as an ident (`c` is idc) and the string mis-parses as a separate token. No `c'..'` form.
        if c == 99 && i + 1 < b.len() && (b[i + 1] == 34 || (b[i + 1] == 114 && i + 2 < b.len() && (b[i + 2] == 34 || b[i + 2] == 35))) {
            i += 1; continue;
        }
        // raw string `r"..."` / `r#"..."#` (verbatim content, no escapes) -- must precede the ident lexer since `r` is idc.
        if c == 114 && i + 1 < b.len() && (b[i + 1] == 34 || b[i + 1] == 35) {
            let mut k = i + 1; while k < b.len() && b[k] == 35 { k += 1; }   // lookahead over the `#` run
            if k < b.len() && b[k] == 34 {
                let hashes = k - (i + 1);
                let cstart = k + 1; let mut j = cstart;
                loop {   // scan for a closing `"` followed by `hashes` `#`
                    if j >= b.len() { break; }
                    if b[j] == 34 { let mut m = j + 1; let mut h = 0; while m < b.len() && h < hashes && b[m] == 35 { h += 1; m += 1; } if h == hashes { break; } }
                    j += 1;
                }
                let content = &src[cstart..j];
                out.push(Tok::Str(content.to_string())); offs.push(ts);
                i = j + 1 + hashes; continue;
            }
        }
        // raw identifier `r#type` / `r#match` -- `r#` + an ident-start char (NOT a quote, which is a raw string handled above).
        // rustc: a plain Ident with keyword-ness escaped, so emit Tok::Ident directly (never a keyword token). Precedes the ident lexer.
        if c == 114 && i + 2 < b.len() && b[i + 1] == 35 && ((b[i + 2] >= 97 && b[i + 2] <= 122) || (b[i + 2] >= 65 && b[i + 2] <= 90) || b[i + 2] == 95) {
            let lo = i + 2; let mut j = lo; while j < b.len() && is_idc(b[j]) { j += 1; }
            let word = &src[lo..j];
            let ix = idn.len() as i64; idn.push(word.to_string()); out.push(Tok::Ident(ix)); offs.push(ts);
            i = j; continue;
        }
        if (c >= 97 && c <= 122) || (c >= 65 && c <= 90) || c == 95 || c >= 128 {   // reduced XID ident-start: ASCII letter/underscore OR any UTF-8 non-ASCII lead byte (>=128)
            let lo = i; while i < b.len() && is_idc(b[i]) { i += 1; }
            let word = &src[lo..i];
            if word == "pub" { out.push(Tok::Pub); offs.push(ts); }
            else if word == "const" { out.push(Tok::Const); offs.push(ts); }
            else if word == "struct" { out.push(Tok::Struct); offs.push(ts); }
            else if word == "enum" { out.push(Tok::Enum); offs.push(ts); }
            else if word == "fn" { out.push(Tok::Fn); offs.push(ts); }
            else if word == "let" { out.push(Tok::Let); offs.push(ts); }
            else if word == "if" { out.push(Tok::If); offs.push(ts); }
            else if word == "else" { out.push(Tok::Else); offs.push(ts); }
            else if word == "match" { out.push(Tok::Match); offs.push(ts); }
            else if word == "type" { out.push(Tok::Type); offs.push(ts); }
            else if word == "use" { out.push(Tok::Use); offs.push(ts); }
            else if word == "mod" { out.push(Tok::Mod); offs.push(ts); }
            else if word == "impl" { out.push(Tok::Impl); offs.push(ts); }
            else if word == "trait" { out.push(Tok::Trait); offs.push(ts); }
            else if word == "where" { out.push(Tok::Where); offs.push(ts); }
            else if word == "as" { out.push(Tok::As); offs.push(ts); }
            else if word == "unsafe" { out.push(Tok::Unsafe); offs.push(ts); }
            else if word == "mut" { out.push(Tok::Mut); offs.push(ts); }
            else if word == "while" { out.push(Tok::While); offs.push(ts); }
            else if word == "loop" { out.push(Tok::Loop); offs.push(ts); }
            else if word == "for" { out.push(Tok::For); offs.push(ts); }
            else if word == "in" { out.push(Tok::In); offs.push(ts); }
            else { let ix = idn.len() as i64; idn.push(word.to_string()); out.push(Tok::Ident(ix)); offs.push(ts); }
            continue;
        }
        if c == 36 { let ix = idn.len() as i64; idn.push(String::from("$")); out.push(Tok::Ident(ix)); offs.push(ts); i += 1; continue; }
        i += 1;
    }
    out.push(Tok::End); offs.push(b.len() as i64); (out, offs, idn)
}

fn id_str_raw(idn: &Vec<String>, ix: i64) -> String { idn[ix as usize].clone() }
fn id_eq_raw(idn: &Vec<String>, ix: i64, s: &str) -> bool { streq(&idn[ix as usize], s) }
// CT-13: the parser answered input it cannot handle with UNBOUNDED ALLOCATION rather than a diagnostic --
// 28.5 GB peak RSS and an OOM kill on a 252-line file. There are 58 `loop` constructs on the parse side, so a
// per-loop guard is the wrong shape; the invariant they all share is that a parse loop must ADVANCE. This is
// one stall detector at the single point every loop passes through. The threshold is far above any legitimate
// re-peek at one position (hundreds) and is reached in well under a second when a loop is genuinely stuck.
static mut PK_POS: i64 = 0 - 1;
static mut PK_N: i64 = 0;
fn pk_guard(pos: i64) {
    unsafe {
        if pos == PK_POS {
            PK_N = PK_N + 1;
            if PK_N > 2000000 { eprintln!("error: parser made no progress at token {} -- construct not handled by this tier", pos); std::process::exit(1); }
        } else { PK_POS = pos; PK_N = 0; }
    }
}
struct Parser { idents: Vec<String>, toks: Vec<Tok>, offs: Vec<i64>, pos: usize, no_struct: bool, expr_path: bool, pending_gt: i32, in_cond: bool, expand: bool, mac_names: Vec<String>, mac_matchers: Vec<String>, mac_bodies: Vec<String>, mac_metavars: Vec<String>, mac_reps: Vec<String>, mac_depth: i32, rule_toks: Vec<String>, rep_els: Vec<String>, drop_stmt: bool, stmt_attrs: Vec<Attr>, resolve: bool, moddir: String, pend_move: bool, pend_async: bool }   // pend_move: a `move` awaits its closure (S1-amendment). in_cond: inside an if/while condition, where `let PAT = EXPR` is a valid let-chain operand. expand: S2 macro-expansion mode (off by default so the S1 unexpanded goldens hold; on via the `//@S2EXPAND` stdin marker). mac_names/mac_matchers/mac_bodies: the macro_rules registry, ONE ROW PER ARM (MAC-1 multi-arm; matcher is the arm's joined matcher tokens, "" = empty matcher). mac_depth: recursion guard. rule_toks: scratch holding a def's raw rule tokens for mac_register_arms (a struct field, since a &Vec param is not a lowered subset access shape).
impl Parser {
    fn peek(&self) -> Tok { pk_guard(self.pos as i64); if self.pos < self.toks.len() { self.toks[self.pos].clone() } else { Tok::End } }   // clamp past-end to End (mirrors peek2) so an over-bump on an unhandled construct yields a graceful `?` not a panic
    fn peek2(&self) -> Tok { if self.pos + 1 < self.toks.len() { self.toks[self.pos + 1].clone() } else { Tok::End } }
    // Read-only lookahead: is the item a (possibly multi-segment) macro CALL `Ident (:: Ident)* !`? The single-segment arm
    // (peek2 == Bang) missed `bitflags::bitflags! { .. }` (a multi-seg path macro), so the parser fell through and PARSED the
    // macro's non-Rust DSL body (`impl X: u16 { .. }`), desyncing everything after it. Scan the path shape without bumping.
    fn peek_item_macro(&self) -> bool {
        let mut p = self.pos;
        match &self.toks[p] { Tok::Ident(_) => {}, _ => return false }
        p += 1;
        loop {
            let t = if p < self.toks.len() { &self.toks[p] } else { return false };
            match t { Tok::Bang => return true, Tok::PathSep => {
                p += 1;
                match if p < self.toks.len() { &self.toks[p] } else { return false } { Tok::Ident(_) => { p += 1; }, _ => return false }
            }, _ => return false }
        }
    }
    // Read-only lookahead: skip a run of balanced `#[..]` outer-attr groups, then report whether the first non-attr token is an
    // item keyword. Distinguishes `#[cfg] let x=1;` (a STMT with attrs) from `#[derive] struct L;` (an ITEM in a block). Bounds-clamped.
    // True when the attribute group at the cursor is followed by NOTHING -- a crate whose entire content is
    // inner attributes. Same scan as attr_leads_item, opposite answer at end-of-tokens.
    fn attrs_run_to_end(&self) -> bool {
        let mut p = self.pos;
        if p >= self.toks.len() { return false; }
        match &self.toks[p] { Tok::Hash => {}, _ => return false }
        loop {
            if p >= self.toks.len() { return true; }
            // Tok::End is a REAL element of the token vector, not just the past-the-end sentinel peek() returns,
            // so a length test alone walks straight past it and the scan answered false for `#![no_std]`.
            match &self.toks[p] { Tok::End => return true, _ => {} }
            match &self.toks[p] { Tok::Hash => { p += 1; }, _ => return false }
            if p >= self.toks.len() { return true; }
            match &self.toks[p] { Tok::Bang => { p += 1; }, _ => {} }
            if p >= self.toks.len() { return true; }
            match &self.toks[p] { Tok::End => return true, _ => {} }
            match &self.toks[p] { Tok::LBrack => {
                let mut depth = 0;
                loop {
                    if p >= self.toks.len() { return true; }
                    match &self.toks[p] {
                        Tok::LBrack => { depth += 1; p += 1; }
                        Tok::RBrack => { depth -= 1; p += 1; if depth == 0 { break } }
                        _ => { p += 1; }
                    }
                } }, _ => return false }
            if p >= self.toks.len() { return true; }
        }
    }
    fn attr_leads_item(&self) -> bool {
        let mut p = self.pos;
        loop {
            if p >= self.toks.len() { return false; }
            match &self.toks[p] { Tok::Hash => { p += 1; }, _ => break }
            if p >= self.toks.len() { return false; }
            match &self.toks[p] { Tok::LBrack => {
                let mut depth = 0;
                loop {
                    if p >= self.toks.len() { return false; }
                    match &self.toks[p] {
                        Tok::LBrack => { depth += 1; p += 1; }
                        Tok::RBrack => { depth -= 1; p += 1; if depth == 0 { break } }
                        _ => { p += 1; }
                    }
                } }, _ => return false }
        }
        if p >= self.toks.len() { return false; }
        match &self.toks[p] {
            Tok::Struct => true, Tok::Enum => true, Tok::Fn => true, Tok::Impl => true,
            Tok::Trait => true, Tok::Const => true, Tok::Use => true, Tok::Mod => true,
            Tok::Type => true, Tok::Pub => true,
            // CT-14: was `Tok::Unsafe => true`, unconditionally. A bare `unsafe { .. }` is a BLOCK, not an item,
            // and starts_item gets this right one screen away (Unsafe only when peek2 is Fn/Impl/Trait). Past a
            // leading attribute the restriction was dropped, so `#[inline] unsafe { .. }` routed to parse_item,
            // which then consumed to end-of-input looking for an item that is not there. `extern` is included
            // for the in-body `unsafe extern "C" fn` item CRASH-3 already routes.
            Tok::Unsafe => { if p + 1 >= self.toks.len() { false } else { match &self.toks[p + 1] { Tok::Fn => true, Tok::Impl => true, Tok::Trait => true, Tok::Ident(ix4) => { let w4 = self.id_str(*ix4); streq(&w4, "extern") }, _ => false } } },
            // The reserved-word Ident item-starters that starts_item detects inline (macro_rules / static / decl_macro)
            // must ALSO be recognized here, past a leading attr. WITHOUT this, `#[allow(unused)] macro_rules! m {..}` in a
            // fn body returns false -> parse_stmt treats it as an EXPR-stmt -> the malformed `macro_rules ! m {..}` macro
            // -call parse spins (HANG, e.g. core/ptr swap_nonoverlapping's attributed inner macro_rules).
            // rzrs #6/7: pass an OWNED clone of the ident (self.toks[p].clone() -- the element-clone idiom peek() uses),
            // never a Vec-index-reached `&String` to streq (that mis-lowers to garbage under newstage0 -> empty rz output).
            _ => {
                let tk = self.toks[p].clone();
                match tk {
                    Tok::Ident(ix) => { let w = self.id_str(ix);
                        if p + 1 >= self.toks.len() { false }
                        else { let nx = self.toks[p+1].clone();
                            if streq(&w, "macro_rules") { match nx { Tok::Bang => true, _ => false } }
                            else if streq(&w, "static") { match nx { Tok::Ident(_) => true, Tok::Mut => true, _ => false } }
                            else if streq(&w, "macro") { match nx { Tok::Ident(_) => true, _ => false } }
                            else { false } }
                    }
                    _ => false
                }
            } }
    }
    // close ONE generic `>`: a lexed `>>` (Shr) closes two levels, so split it via pending_gt. Returns true if a `>` was eaten.
    fn eat_close_gt(&mut self) -> bool {
        if self.pending_gt > 0 { self.pending_gt = self.pending_gt - 1; return true; }
        match self.peek() { Tok::Gt => { self.bump(); true }, Tok::Shr => { self.bump(); self.pending_gt = 1; true }, Tok::Ge => { self.toks[self.pos] = Tok::Eq; true }, _ => false }   // `>=` in generic-close position (`Vec<u8>=b`): consume the `>`, rewrite the token to the trailing `=` (Eq); Shr's counter mechanism can't apply since the 2nd half is `=` not `>`
    }
    fn cur_off(&self) -> i64 { if self.pos < self.offs.len() { self.offs[self.pos] } else { let n = self.offs.len(); if n > 0 { self.offs[n - 1] } else { 0 } } }   // clamp to the last offset past-end
    fn bump(&mut self) -> Tok { if self.pos < self.toks.len() { let t = self.toks[self.pos].clone(); self.pos += 1; t } else { Tok::End } }   // clamp: an over-bump past End stays at End instead of panicking
    fn id_str(&self, ix: i64) -> String { id_str_raw(&self.idents, ix) }
    fn id_eq(&self, ix: i64, s: &str) -> bool { id_eq_raw(&self.idents, ix, s) }
    fn ident(&mut self) -> String { match self.bump() { Tok::Ident(ix) => self.id_str(ix), _ => String::from("?") } }
    // path := seg (:: seg)* ;  seg := IDENT (< ty (, ty)* >)?
    // HRTB binder `for<'a, 'b>` prefix on a trait bound / fn-trait / dyn / impl bound (rustc PolyTraitRef.bound_generic_params).
    // Only a binder in TYPE/BOUND position: `for` immediately followed by `<`. The for-LOOP keyword is consumed earlier
    // (parse_expr / impl-for) before any parse_path, so seeing `for <` here is unambiguously an HRTB binder. Elide it
    // (the following bound parses normally); the reduced dump renders just the trait, matching rustc's PolyTraitRef trait.
    fn skip_hrtb_binder(&mut self) {
        match self.peek() { Tok::For => { match self.peek2() { Tok::Lt => {
            self.bump(); self.bump();   // `for` `<`
            loop { if self.eat_close_gt() { break } self.bump(); }   // discard the lifetime binder list
        }, _ => {} } }, _ => {} }
    }
    fn parse_path(&mut self) -> Path {
        self.skip_hrtb_binder();
        match self.peek() { Tok::PathSep => { self.bump(); }, _ => {} }   // optional leading `::` crate-root (`::core::A`); root segment elided, real segments read normally
        let mut segments: Vec<PathSeg> = Vec::new();
        segments.push(self.parse_seg());
        // Stop the segment loop when a generic-close `>` is PENDING (self.pending_gt>0 -- a glued `>>` was split and one
        // `>` still belongs to an ENCLOSING context). Otherwise the trait path of a qself `<T as Tr<U>>::m` greedily eats
        // the `::m` (and the following call args) because the qself-closing `>` is hidden in pending_gt, not a stream token
        // -> `<T as Tr<U>::m(*x)>` mis-parse whose `*`-arg reparse STACK-OVERFLOWS (core/convert AsRef `<T as AsRef<U>>::
        // as_ref(*self)`). In every normal path the enclosing generic's arg loop consumes pending_gt before returning here,
        // so pending_gt is 0 and this is a no-op; it only fires for the glued-`>>` qself/generic-close boundary.
        loop { if self.pending_gt > 0 { break; } match self.peek() { Tok::PathSep => { self.bump(); segments.push(self.parse_seg()); }, _ => break } }
        Path { segments: segments }
    }
    // use-tree := prefix? (`::` `{` tree (`,` tree)* `}` | `::` `*` | `as` IDENT | (empty))
    // Reads plain-ident prefix segments separated by `::`; at each `::` a `{` opens a Nested group and `*` a Glob;
    // a trailing `as IDENT` is a Simple rename; otherwise Simple with an empty rename. Prefix segs carry no generics.
    fn parse_use_tree(&mut self) -> UseTree {
        let mut segs: Vec<PathSeg> = Vec::new();
        loop {
            match self.peek() {
                Tok::LBrace => { self.bump();
                    let mut items: Vec<UseTree> = Vec::new();
                    loop { match self.peek() { Tok::RBrace => { self.bump(); break; } Tok::Comma => { self.bump(); } _ => { items.push(self.parse_use_tree()); } } }
                    return UseTree { prefix: Path { segments: segs }, kind: UseKind::Nested(items) }; }
                Tok::Star => { self.bump(); return UseTree { prefix: Path { segments: segs }, kind: UseKind::Glob }; }
                _ => { segs.push(PathSeg { ident: self.ident(), args: Vec::new(), paren: false });
                    match self.peek() {
                        Tok::PathSep => { self.bump(); }
                        Tok::As => { self.bump(); let r = self.ident(); return UseTree { prefix: Path { segments: segs }, kind: UseKind::Simple(r) }; }
                        _ => { return UseTree { prefix: Path { segments: segs }, kind: UseKind::Simple(String::from("")) }; }
                    } }
            }
        }
    }
    // qself := `<` ty `as` path `>` ( `::` seg )*  -- qualified path `<Ty as Trait>::Assoc`. qty+trait parse as TYPES.
    fn parse_qself(&mut self) -> (P<Ty>, P<Path>, Vec<PathSeg>) {
        self.bump();                         // `<`
        self.parse_qself_body()
    }
    // CT-19, second attempt. `A<<u8 as Tr>::D>` lexes `<<` as ONE Shl, so the generic-arg opener never saw a
    // `Lt`, never routed to parse_qself, and the item loop recovered by INVENTING an `Item Const` named `?` --
    // 34 invented items tree-wide, the whole C-S1-PHANTOM-CONST-ITEM class.
    // The FIRST attempt rewrote the token in place (`self.toks[self.pos] = Tok::Lt`). That is correct Rust and
    // passed the differential 5/5, but writing into a Vec element held in a struct field is not self-hostable
    // at this tier: gates went 14/18 and reverting restored 18/18. Note `probe49`'s selfemit_rc=0 did NOT
    // catch it -- emission succeeding is not gen2 BEHAVING.
    // This form needs neither a mutation nor a pending_lt counter: a Shl IS both `<` of `A<` and the qself's
    // own `<`, so consuming it whole lands exactly at the qself TYPE, which is where the body starts.
    fn parse_qself_body(&mut self) -> (P<Ty>, P<Path>, Vec<PathSeg>) {
        let qty = Box::new(self.parse_ty()); // qself type (parse_ty forces expr_path=false)
        self.bump();                         // `as`
        let sv = self.expr_path; self.expr_path = false;   // trait path takes bare `<` generics
        let tr = Box::new(self.parse_path());
        self.expr_path = sv;
        self.eat_close_gt();                 // closing `>` (or split `>>`)
        let mut proj: Vec<PathSeg> = Vec::new();
        loop { match self.peek() { Tok::PathSep => { self.bump(); proj.push(self.parse_seg()); }, _ => break } }
        (qty, tr, proj)
    }
    // CT-9: the EXPRESSION mouth only. `as Trait` is optional here -- `<_>::default()` and
    // `<dyn TraitEngine<..>>::new(..)` are the only qself shapes the rustc tree writes in expression position,
    // and every one of the 19 omits `as`. The TYPE mouth above keeps the unconditional bump: relaxing it there
    // regressed 3 files, two of them to a stack overflow, because a `<` that is NOT a qself (the `Result<()>`
    // in `|file: &mut dyn Write| -> io::Result<()>`) reaches it, and the unconditional bump was what forced
    // progress out of that state. Type-position qselfs in this tree always write `as`.
    fn parse_qself_expr(&mut self) -> (P<Ty>, P<Path>, Vec<PathSeg>) {
        self.bump();                         // `<`
        let qty = Box::new(self.parse_ty());
        let mut tr: P<Path> = Box::new(Path { segments: Vec::new() });
        match self.peek() { Tok::As => { self.bump();
                let sv = self.expr_path; self.expr_path = false;
                tr = Box::new(self.parse_path());
                self.expr_path = sv; }, _ => {} }
        self.eat_close_gt();
        let mut proj: Vec<PathSeg> = Vec::new();
        loop { match self.peek() { Tok::PathSep => { self.bump(); proj.push(self.parse_seg()); }, _ => break } }
        (qty, tr, proj)
    }
    fn parse_seg(&mut self) -> PathSeg {
        let ident = self.ident();
        let mut args: Vec<Ty> = Vec::new();
        let paren_ok = match self.peek() { Tok::LParen => { self.expr_path == false }, _ => false };   // `Fn(A)->R` only in TYPE paths; in expr paths `f(..)` is a Call (postfix)
        if paren_ok {   // parenthesized trait-arg `Fn(A, B) -> R` (rustc GenericArgs::Parenthesized)
            self.bump();
            loop { match self.peek() { Tok::RParen => { self.bump(); break }, _ => { args.push(self.parse_ty()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }
            match self.peek() { Tok::Arrow => { self.bump(); args.push(Ty::Assoc(String::from("Output"), Box::new(self.parse_ty()))); }, _ => {} }
            return PathSeg { ident: ident, args: args, paren: true };
        }
        let mut turbofish = false;
        match self.peek() { Tok::PathSep => { match self.peek2() { Tok::Lt => { self.bump(); turbofish = true; }, _ => {} } }, _ => {} }   // turbofish `Ident::<T>`
        // In EXPRESSION paths a bare `<` is the less-than operator, NOT generics (only `::<` turbofish opens generics);
        // in TYPE paths (expr_path=false) a bare `<` opens generic args (`Vec<T>`). Resolves the classic `<` ambiguity.
        let take_generics = turbofish || (self.expr_path == false);
        // A leading Shl here is `A<` immediately followed by a qself `<`: consume it whole and parse the qself
        // body, which is the one generic argument.
        if take_generics { match self.peek() { Tok::Shl => { self.bump(); let r = self.parse_qself_body(); args.push(Ty::QPath(r.0, r.1, r.2));
                // The qself is the FIRST argument, not necessarily the only one: 4 of the 48 sites in the tree
                // carry a following arg (`A<<u8 as Tr>::D, u32>`). Returning here would have silently dropped
                // them -- precisely the invented/missing-item class this rung exists to close.
                loop { if self.eat_close_gt() { break }
                    match self.peek() { Tok::Comma => { self.bump(); }, _ => { args.push(self.parse_ty()); } } }
                return PathSeg { ident: ident, args: args, paren: false }; }, _ => {} } }
        if take_generics { match self.peek() { Tok::Lt => { self.bump();
            loop { if self.eat_close_gt() { break }   // `>` or split `>>` closes the generic-arg list
                let is_assoc = match self.peek() { Tok::Ident(_) => { match self.peek2() { Tok::Eq => true, _ => false } }, _ => false };   // `Item = ty` assoc-type binding
                let is_assoc_bound = match self.peek() { Tok::Ident(_) => { match self.peek2() { Tok::Colon => true, _ => false } }, _ => false };   // `Item: Clone` assoc-type BOUND (rustc AssocItemConstraint kind Bound; distinct from the `=` equality form)
                let is_const = match self.peek() { Tok::Int(_) => true, Tok::Float(_) => true, Tok::Char(_) => true, Tok::LBrace => true, _ => false };   // `Foo<3>` literal / `Foo<{ .. }>` block -> a single const generic arg (rustc GenericArg::Const(AnonConst)); bare ident stays a type path
                if is_const { match self.peek() { Tok::LBrace => { let cv = self.capture_group(); args.push(Ty::ConstArg(true, cv)); }, _ => { let mut cv: Vec<String> = Vec::new(); cv.push(tok_str(&self.peek(), &self.idents)); self.bump(); args.push(Ty::ConstArg(false, cv)); } } }
                else if is_assoc { let nm = self.ident(); self.bump(); match self.peek() { Tok::Int(_) => { let mut cv: Vec<String> = Vec::new(); cv.push(tok_str(&self.peek(), &self.idents)); self.bump(); args.push(Ty::Assoc(nm, Box::new(Ty::ConstArg(false, cv)))); }, _ => { args.push(Ty::Assoc(nm, Box::new(self.parse_ty()))); } } }
                else if is_assoc_bound { let nm = self.ident(); self.bump(); let b = self.parse_ty();   // `Item: Clone`; consume+elide any extra `+ Bound` (reduced dump keeps the first bound)
                    loop { match self.peek() { Tok::Plus => { self.bump(); let _ = self.parse_ty(); }, _ => break } }
                    args.push(Ty::AssocBound(nm, Box::new(b))); }
                else { args.push(self.parse_ty()); }
                if self.pending_gt == 0 { match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } }   // only eat an arg-separator comma when NOT mid-split-`>>`: after `Foo<Bar<A,B>>` the inner arg left pending_gt=1 (a virtual `>` still to close this level), so a following `,` belongs to the ENCLOSING list (e.g. the next where-predicate), NOT this arg list -- eating it desynced `where R: AsMut<X<'a,'b>>, F: ..`
        }, _ => {} } }
        PathSeg { ident: ident, args: args, paren: false }
    }
    // ty := & [mut] ty | * (const|mut) ty | [ ty (; INT)? ] | ( ty (, ty)* ) | path
    fn parse_ty(&mut self) -> Ty {
        let sv = self.expr_path; self.expr_path = false;   // types always take bare `<` generics, even inside an expr path (turbofish args / cast)
        let t = self.parse_ty_inner();
        self.expr_path = sv;
        t
    }
    fn parse_ty_inner(&mut self) -> Ty {
        match self.peek() {
            Tok::Amp => { self.bump();
                let mut lt = String::new(); match self.peek() { Tok::Lifetime(nm) => { self.bump(); lt = nm; }, _ => {} }   // `&'a T`
                let mut m = false; match self.peek() { Tok::Mut => { self.bump(); m = true; }, _ => {} } Ty::Ref(lt, m, Box::new(self.parse_ty())) }
            Tok::AndAnd => { self.toks[self.pos] = Tok::Amp; Ty::Ref(String::from(""), false, Box::new(self.parse_ty_inner())) }   // `&&T` -- the lexer glued `&&` into AndAnd; split it (rewrite in place to a single `&`, R67-style) so the outer ref (no lifetime/mut on a glued `&&`) wraps the inner `&T`; was falling to `?`/hang
            Tok::Lifetime(nm) => { self.bump(); Ty::Lt(nm) }   // lifetime in type-arg position `<'a>`
            Tok::Star => { self.bump(); let mut m = false; match self.bump() { Tok::Mut => { m = true; }, _ => {} } Ty::Ptr(m, Box::new(self.parse_ty())) }
            Tok::LBrack => { self.bump(); let elem = Box::new(self.parse_ty());
                match self.peek() { Tok::Semi => { self.bump(); let n = Box::new(self.parse_expr(0)); self.bump(); Ty::Array(elem, n) }, _ => { self.bump(); Ty::Slice(elem) } } }
            Tok::Bang => { self.bump(); Ty::Never }   // never type `!` (rustc TyKind::Never)
            Tok::Lt => { let r = self.parse_qself(); Ty::QPath(r.0, r.1, r.2) }   // `<Ty as Trait>::Assoc` qualified path type
            Tok::LParen => { self.bump(); let mut items: Vec<Ty> = Vec::new(); let mut saw_comma = false;
                loop { match self.peek() { Tok::RParen => { self.bump(); break }, _ => { items.push(self.parse_ty()); match self.peek() { Tok::Comma => { self.bump(); saw_comma = true; }, _ => {} } } } }
                // C-SEM0-GEN2-PAREN-TY (locales/rustc-crate-targeting): this was `items.remove(0)`, and `remove`
                // is NOT in s9_method_kind's dispatch -- SEM0 lowered it to `call remove`, a call to a function
                // that does not exist. So a SEM0-BUILT s1_parser carried a dangling call on exactly this line,
                // and died the moment it parsed a one-element parenthesized type `(T)`. An L6-built one did not,
                // because L6 lowers `.remove` correctly. Measured: `fn s() -> (E) { }` -- 18 bytes -- gen1 exits
                // 0, gen2 SIGSEGVs; `()`, `(A,B)` and `(A,B,C)` are all fine, since only the ONE-element case
                // reaches this branch. `pop()` is the same operation here (index 0 IS the last element when the
                // length is 1) and IS lowered.
                if items.len() == 1 && !saw_comma { let mut only: Vec<Ty> = items; let taken = only.pop(); match taken { Some(t0) => t0, None => Ty::Tup(only) } } else { Ty::Tup(items) } }
            Tok::Fn => self.parse_fnptr_after_fn()   // `fn(A, B) -> R` bare fn-ptr type (rustc TyKind::FnPtr; incl. variadic `...`)
                ,
            Tok::Unsafe => self.parse_bare_fn_ty(),   // `unsafe [extern [abi]] fn(..)` -> FnPtrExt-wrapped
            Tok::For => { self.skip_hrtb_binder(); self.parse_ty_inner() }   // HRTB `for<'a> fn(&'a u8)` / `for<'a> Trait` -- skip the binder (reduced dump elides the `for<>` quantifier, like other modifiers), parse the wrapped fn-ptr/trait; was falling to parse_path -> `?`
            Tok::Impl => { self.bump(); Ty::ImplTrait(self.parse_bounds()) }   // `impl Trait` -> rustc TyKind::ImplTrait
            Tok::Ident(ix) => { let nm = self.id_str(ix); if nm == "dyn" { self.bump(); Ty::DynTrait(self.parse_bounds()) } else if nm == "extern" { self.parse_bare_fn_ty() } else { let p = self.parse_path(); Ty::Path(Box::new(p)) } }   // `dyn Trait` -> TraitObject; `extern [abi] fn(..)` -> FnPtrExt; else a path type
            _ => { let p = self.parse_path(); Ty::Path(Box::new(p)) }
        }
    }
    // trait-bound list `Path (+ Path)*` (for impl/dyn Trait + generic-param bounds)
    fn parse_bounds(&mut self) -> Vec<Path> {
        let mut bounds: Vec<Path> = Vec::new();
        bounds.push(self.parse_bound_one());
        loop { match self.peek() { Tok::Plus => { self.bump(); bounds.push(self.parse_bound_one()); }, _ => break } }
        bounds
    }
    fn parse_bound_one(&mut self) -> Path {   // a single bound: a trait path OR a lifetime `'a` (dyn/impl `+ 'a` outlives -> tick-Path, like dm10)
        match self.peek() { Tok::Lifetime(l) => { self.bump(); let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: format!("'{}", l), args: Vec::new(), paren: false }); Path { segments: segs } }, _ => self.parse_bound_path() }
    }
    // a trait bound path with an optional leading relaxed `?` modifier (`?Sized`) -> rustc TraitBoundModifier::Maybe.
    // The `?` is a MODIFIER on the following poly-trait-ref, NOT a standalone bound: consume it and ELIDE it from the
    // reduced dump (like the unsafe/const/mut/async modifiers we already drop). Eliding -- rather than rendering `?Sized`
    // -- is load-bearing for self-host: the L6 tower drops a literal `?` inside a `format!("?{}")`, so a rendered `?Sized`
    // would diverge rc(`?Sized`) vs rz(`Sized`) and fail the witness bootstrap; eliding keeps rc==rz. Node shape is correct
    // (one bound `Sized`, no comma-split, no `?` sentinel). `~const` (MaybeConst) needs a `~` lexer token first -- deferred.
    fn parse_bound_path(&mut self) -> Path {
        match self.peek() { Tok::Question => { self.bump(); }, _ => {} }   // consume + elide the relaxed `?` modifier
        match self.peek() { Tok::Bang => { self.bump(); }, _ => {} }   // consume + elide a negative `!Send` bound polarity (rustc polarity Negative); was leaking `!` -> `?` sentinel / comma-split
        self.parse_path()
    }
    // capture a const-generic param's `= <default>` tokens (nesting-aware) into a rendered String (rustc AnonConst),
    // so `<const N: usize = 8>` renders the default like a type-param default instead of dropping it. Keeps the EXACT
    // original skip-loop shape (proven L6-safe) + one accumulate line per arm; extracted as a helper so parse_generics
    // does not grow (R51). Caller has already consumed the `=`. `dv` starts empty; each kept token appends via tok_str.
    fn capture_const_default(&mut self) -> String {
        let mut dv = String::new(); let mut d: i64 = 0;
        loop { match self.peek() {
            Tok::Lt => { let t = tok_str(&self.peek(), &self.idents); if dv.len() == 0 { dv = t; } else { dv = format!("{} {}", dv, t); } d += 1; self.bump(); }
            Tok::Gt => { if d == 0 { break } let t = tok_str(&self.peek(), &self.idents); if dv.len() == 0 { dv = t; } else { dv = format!("{} {}", dv, t); } d -= 1; self.bump(); }
            Tok::Shr => { if d == 0 { break } let t = tok_str(&self.peek(), &self.idents); if dv.len() == 0 { dv = t; } else { dv = format!("{} {}", dv, t); } d -= 2; self.bump(); }
            Tok::Comma => { if d == 0 { break } let t = tok_str(&self.peek(), &self.idents); if dv.len() == 0 { dv = t; } else { dv = format!("{} {}", dv, t); } self.bump(); }
            Tok::End => break,
            _ => { let t = tok_str(&self.peek(), &self.idents); if dv.len() == 0 { dv = t; } else { dv = format!("{} {}", dv, t); } self.bump(); }
        } }
        dv
    }
    // `fn ( ins ) -> out` with current token at `fn`; a terminal `...` (lexed DotDot+Dot) is a C-variadic param (rustc CVarArgs).
    fn parse_fnptr_after_fn(&mut self) -> Ty {
        self.bump(); self.bump();   // `fn` + `(`
        let mut ins: Vec<Ty> = Vec::new();
        loop {
            match self.peek() { Tok::RParen => { self.bump(); break }, _ => {} }
            let is_var = match self.peek() { Tok::DotDot => { match self.peek2() { Tok::Dot => true, _ => false } }, _ => false };
            if is_var { self.bump(); self.bump(); ins.push(Ty::CVarArgs); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } }
            else {
                match self.peek2() { Tok::Colon => { self.bump(); self.bump(); }, _ => {} }   // optional `name:` on a param
                ins.push(self.parse_ty()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} }
            }
        }
        let out = match self.peek() { Tok::Arrow => { self.bump(); Some(Box::new(self.parse_ty())) }, _ => None };
        Ty::FnPtr(ins, out)
    }
    // `[unsafe] [extern [abi-str]] fn(..)` -> the prefix is captured as a string, the inner FnPtr is wrapped (rustc TyKind::FnPtr safety/ext).
    fn parse_bare_fn_ty(&mut self) -> Ty {
        let mut pfx = String::new();
        match self.peek() { Tok::Unsafe => { self.bump(); pfx = String::from("unsafe"); }, _ => {} }
        match self.peek() { Tok::Ident(ix) => { let w = self.id_str(ix); if w == "extern" { self.bump();
            if pfx.len() > 0 { pfx = format!("{} extern", pfx); } else { pfx = String::from("extern"); }
            match self.peek() { Tok::Str(s) => { self.bump(); pfx = format!("{} \"{}\"", pfx, s); }, _ => {} } } }, _ => {} }
        let inner = self.parse_fnptr_after_fn();
        if pfx.len() > 0 { Ty::FnPtrExt(pfx, Box::new(inner)) } else { inner }
    }
    // closure body: `|params| BODY`. With an explicit `-> RetTy` the body is REQUIRED to be a block (rustc: FnDecl.output=Ty, body=Block);
    // the ret-type is elided in the reduced dump. Without `->`, the body is any expr. Fixes L4-m1 (a `-> u8 { .. }` closure dropped the
    // body to a bogus LitInt 0 and leaked the real `{ .. }` as a detached stmt because parse_expr(0) can't start at `->`).
    fn parse_closure_body(&mut self) -> P<ExprK> {
        match self.peek() { Tok::Arrow => { self.bump(); let _ = self.parse_ty(); Box::new(ExprK::Block(Box::new(self.parse_block()))) }, _ => Box::new(self.parse_expr(0)) }
    }
    // S1-amendment: consume an explicit closure `-> RetTy` into a 0-or-1 Vec (like CParam.ty); empty = no ret.
    fn parse_closure_ret(&mut self) -> Vec<Ty> {
        let mut r: Vec<Ty> = Vec::new();
        match self.peek() { Tok::Arrow => { self.bump(); r.push(self.parse_ty()); }, _ => {} }
        r
    }
    // The closure body after the (already-consumed) ret: a block if `-> RetTy` was present (rustc requires it), else any expr.
    fn parse_closure_body_after(&mut self, has_ret: bool) -> P<ExprK> {
        if has_ret { Box::new(ExprK::Block(Box::new(self.parse_block()))) } else { Box::new(self.parse_expr(0)) }
    }
    // if := if EXPR BLOCK ( else ( if | BLOCK ) )?  (else stored as a Vec of 0 or 1)
    fn parse_if(&mut self) -> ExprK {
        self.bump();                         // if
        let sv = self.no_struct; self.no_struct = true;
        let svc = self.in_cond; self.in_cond = true;   // enable let-EXPR + let-chain operands in the condition (`if let A=x && let B=y`)
        let cond = Box::new(self.parse_expr(0));
        self.in_cond = svc;
        self.no_struct = sv;
        let then = Box::new(self.parse_block());
        let mut els: Vec<ExprK> = Vec::new();
        match self.peek() { Tok::Else => { self.bump();
            match self.peek() { Tok::If => { els.push(self.parse_if()); }, _ => { els.push(ExprK::Block(Box::new(self.parse_block()))); } } }, _ => {} }
        ExprK::If(cond, then, els)
    }
    // atom := INT | path | ( expr ) | if-expr | BLOCK (block-expr)
    // after a `..`/`..=`, parse an end expr unless a closer/terminator follows (open-ended range `a..` / `..`).
    // CT-17: `{` was missing from this stop set. In a HEADER context -- `for i in 0.. {`, `if a.. {`,
    // `while x.. {` -- a `{` opens the loop/if BODY, and taking it as the range's end expression made the loop
    // then consume the NEXT `{`, so the enclosing `impl` never closed and swallowed the rest of the file:
    // 71 KB and 20 top-level items nested one level too deep, at rc=0 with zero UNLOWERED. Gated on no_struct
    // (the flag that already marks header position) so an ordinary `0..{ .. }` outside a header is untouched.
    fn range_end_opt(&mut self) -> Option<P<ExprK>> {
        match self.peek() { Tok::Semi => None, Tok::RBrack => None, Tok::RParen => None, Tok::RBrace => None, Tok::Comma => None, Tok::End => None,
            Tok::LBrace => { if self.no_struct { None } else { Some(Box::new(self.parse_expr(0))) } },
            _ => Some(Box::new(self.parse_expr(0))) }
    }
    fn parse_atom(&mut self) -> ExprK {
        // optional loop label `'a:` before loop/while/for (rustc opt_label); consumed here, threaded into the loop node.
        let mut label = String::new();
        match self.peek() { Tok::Lifetime(l) => { match self.peek2() { Tok::Colon => { self.bump(); self.bump(); label = l; }, _ => {} } }, _ => {} }
        // CT-20: an attribute in EXPRESSION position -- `let h = #[inline(always)] |c: C<u32>| { };` -- was
        // never consumed. There is no Tok::Hash arm anywhere in expression parsing; the three Hash sites are
        // the lexer, the item-lookahead, and parse_stmt. The parser is total, so with a simple closure param
        // it produced something and moved on; with a GENERIC one it stalled, and with trailing input it
        // instead mis-nested -- swallowing the next six fns of region_infer/mod.rs into one that never closed.
        // rustc allows attributes on expressions, so the fix is to consume and continue.
        match self.peek() { Tok::Hash => { let _ea = self.parse_attrs(); return self.parse_atom(); }, _ => {} }
        let atk = tok_str(&self.peek(), &self.idents);
        if streq(&atk, "try") { match self.peek2() { Tok::LBrace => { self.bump(); let tb = self.parse_block(); return ExprK::TryBlock(Box::new(tb)); }, _ => {} } }
        match self.peek() {
            Tok::Lt => { let svep = self.expr_path; self.expr_path = true;   // leading `<` in expr position is a qself path `<T as Trait>::method` (never less-than: that needs a left operand)
                let r = self.parse_qself_expr(); self.expr_path = svep; return ExprK::QPath(r.0, r.1, r.2); }
            Tok::DotDot => { self.bump(); let end = self.range_end_opt(); return ExprK::Range(None, end, false); }   // `..b` / `..` (no start)
            Tok::DotDotEq => { self.bump(); let end = self.range_end_opt(); return ExprK::Range(None, end, true); }   // `..=b` (no start)
            Tok::LParen => { self.bump();
                match self.peek() { Tok::RParen => { self.bump(); return ExprK::Tuple(Vec::new()); }, _ => {} }   // `()` unit expr (rustc ExprKind::Tup [])
                let svp = self.no_struct; self.no_struct = false; let e = self.parse_expr(0); self.no_struct = svp;
                match self.peek() {
                    Tok::Comma => { let mut items: Vec<ExprK> = Vec::new(); items.push(e);
                        loop { match self.peek() { Tok::RParen => { self.bump(); break }, Tok::Comma => { self.bump(); match self.peek() { Tok::RParen => { self.bump(); break }, _ => { items.push(self.parse_expr(0)); } } }, _ => { self.bump(); } } }
                        ExprK::Tuple(items) }
                    _ => { self.bump(); ExprK::Paren(Box::new(e)) }
                } }
            Tok::LBrack => { self.bump();
                match self.peek() { Tok::RBrack => { self.bump(); return ExprK::Array(Vec::new()); }, _ => {} }   // `[]` empty
                let first = self.parse_expr(0);
                match self.peek() {
                    Tok::Semi => { self.bump(); let count = self.parse_expr(0); self.bump(); ExprK::Repeat(Box::new(first), Box::new(count)) }   // `[v; n]` array-repeat -> rustc ExprKind::Repeat
                    _ => { let mut items: Vec<ExprK> = Vec::new(); items.push(first);
                        match self.peek() { Tok::Comma => { self.bump(); }, _ => {} }
                        loop { match self.peek() { Tok::RBrack => { self.bump(); break }, _ => { items.push(self.parse_expr(0)); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }
                        ExprK::Array(items) } } }
            Tok::If => self.parse_if(),
            Tok::While => { self.bump(); let sv = self.no_struct; self.no_struct = true;
                let svc = self.in_cond; self.in_cond = true; let c = Box::new(self.parse_expr(0)); self.in_cond = svc;   // `while let PAT = EXPR` + let-chains
                self.no_struct = sv; let b = Box::new(self.parse_block()); ExprK::While(label, c, b) }
            Tok::Loop => { self.bump(); let b = Box::new(self.parse_block()); ExprK::Loop(label, b) }
            Tok::For => { self.bump(); let pat = Box::new(self.parse_pat()); self.bump(); let sv = self.no_struct; self.no_struct = true; let it = Box::new(self.parse_expr(0)); self.no_struct = sv; let b = Box::new(self.parse_block()); ExprK::ForLoop(label, pat, it, b) }
            Tok::Match => self.parse_match_expr(),
            Tok::Pipe => { self.bump(); let mut params: Vec<CParam> = Vec::new();
                loop { match self.peek() { Tok::Pipe => { self.bump(); break }, _ => { let pat = self.parse_pat_single();   // closure param is a Pat (`&n`/`(a,b)`/`_`/`mut x`); parse_pat_single stops at the closing `|`
                    let mut ty: Vec<Ty> = Vec::new(); match self.peek() { Tok::Colon => { self.bump(); ty.push(self.parse_ty()); }, _ => {} }   // optional `: ty`
                    params.push(CParam { pat: pat, ty: ty }); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }
                let mv = self.pend_move; let ret = self.parse_closure_ret(); let body = self.parse_closure_body_after(ret.len() > 0); ExprK::Closure(mv, params, ret, body) }
            Tok::OrOr => { self.bump(); let params: Vec<CParam> = Vec::new();   // `||`-> zero-param closure (`||` lexes as OrOr, not two Pipe); body is any expr incl. a block
                let mv = self.pend_move; let ret = self.parse_closure_ret(); let body = self.parse_closure_body_after(ret.len() > 0); ExprK::Closure(mv, params, ret, body) }
            Tok::LBrace => { let b = self.parse_block(); ExprK::Block(Box::new(b)) }
            Tok::Unsafe => { self.bump(); let b = self.parse_block(); ExprK::Unsafe(Box::new(b)) }   // `unsafe { .. }` block expr (rustc: Block w/ rules:Unsafe)
            Tok::Const => { self.bump();   // `const { .. }` inline-const block -> ExprK::ConstBlock; `const [move] |..|` const closure -> ExprK::Closure (constness elided, like move/unsafe/async)
                match self.peek() { Tok::LBrace => { let b = self.parse_block(); ExprK::ConstBlock(Box::new(b)) }, _ => self.parse_atom() } }
            Tok::Int(n) => { self.bump(); ExprK::LitInt(n) }
            Tok::Float(s) => { self.bump(); ExprK::LitFloat(s) }
            Tok::Str(sv) => { self.bump(); ExprK::Str(format!("{}", sv)) }
            Tok::Char(cv) => { self.bump(); ExprK::Char(format!("{}", cv)) }
            Tok::PathSep => { let svep = self.expr_path; self.expr_path = true; let p = self.parse_path(); self.expr_path = svep; return ExprK::Path(Box::new(p)); }   // leading `::` crate-root expr path (`::std::u8::MAX`)
            Tok::Ident(ix) => return self.parse_ident_expr(self.id_str(ix)),
            _ => { self.bump(); ExprK::LitInt(0) }
        }
    }
    // Ident-headed atom: bool literals, move/async/return/break/continue keyword exprs, path / macro-call / struct-literal.
    // Extracted from parse_atom (S9-F7 lean discipline: the struct-field `Vec<EField>` was held live across many bump()
    // 3-word Token temps inside parse_atom's extreme register pressure, which L6 miscompiled at self-emit scale).
    fn parse_ident_expr(&mut self, nm: String) -> ExprK {
                if nm == "true" { self.bump(); return ExprK::Bool(true); } if nm == "false" { self.bump(); return ExprK::Bool(false); }   // bool literals (== rustc Lit(Bool)), not path exprs
                if nm == "move" { self.bump(); self.pend_move = true; let e = self.parse_atom(); self.pend_move = false; return e; }   // `move` closure capture (S1-amendment: read+cleared at Closure construction)
                if nm == "async" { self.bump(); let mut mv = false;   // `async [move] { .. }` block -> ExprK::Async; `async [move] |..|` closure stays a Closure (asyncness elided there, as before)
                    match self.peek() { Tok::Ident(ix) => { let m = self.id_str(ix); if m == "move" { self.bump(); mv = true; } }, _ => {} }
                    match self.peek() { Tok::LBrace => { let b = self.parse_block(); return ExprK::Async(mv, Box::new(b)); }, _ => {} }
                    return self.parse_atom(); }
                if nm == "return" { self.bump(); let mut v: Vec<ExprK> = Vec::new();   // `return [expr]` -> rustc ExprKind::Ret(Some/None)
                    let stop = match self.peek() { Tok::Semi => true, Tok::RBrace => true, Tok::Comma => true, Tok::RParen => true, _ => false };
                    if stop == false { v.push(self.parse_expr(0)); } return ExprK::Ret(v); }
                if nm == "break" { self.bump(); let mut lbl = String::new();   // `break ['label] [expr]` -> rustc ExprKind::Break(label?, value?)
                    match self.peek() { Tok::Lifetime(l) => { self.bump(); lbl = l; }, _ => {} }
                    let mut v: Vec<ExprK> = Vec::new();
                    let stop = match self.peek() { Tok::Semi => true, Tok::RBrace => true, Tok::Comma => true, Tok::RParen => true, _ => false };
                    if stop == false { v.push(self.parse_expr(0)); } return ExprK::Break(lbl, v); }
                if nm == "continue" { self.bump(); let mut lbl = String::new();   // `continue ['label]` -> rustc ExprKind::Continue(label?)
                    match self.peek() { Tok::Lifetime(l) => { self.bump(); lbl = l; }, _ => {} }
                    return ExprK::Continue(lbl); }
                let svep = self.expr_path; self.expr_path = true;   // expr-path: bare `<` is less-than, not generics
                let p = self.parse_path();
                self.expr_path = svep;
                // p is consumed once per mutually-exclusive arm (no early-return move -- the rzrs move-checker models
                // branch moves, not early-return-in-a-match-arm).
                match self.peek() {
                    Tok::Bang => self.parse_mac_tail(p),   // `path!(..)`/`![..]`/`!{..}` macro call
                    _ => {
                        let is_lit = match self.peek() { Tok::LBrace => { if self.no_struct { false } else { true } }, _ => false };
                        if is_lit {
                            self.parse_struct_lit_tail(p)
                        } else { ExprK::Path(Box::new(p)) }
                    }
                }
    }
    // struct-literal tail `Path { field: expr, .., ..base }` after the leading `{` is confirmed. Extracted so the
    // `Vec<EField>` accumulator is not held live across bump() 3-word temps in parse_ident_expr (S9-F7 lean discipline).
    fn parse_struct_lit_tail(&mut self, p: Path) -> ExprK {
                            self.bump();             // LBrace
                            let sv = self.no_struct; self.no_struct = false;
                            let mut fs: Vec<EField> = Vec::new();
                            let mut base: Option<P<ExprK>> = None;   // struct-update `..base` (rustc StructRest::Base)
                            loop { match self.peek() {
                                Tok::RBrace => { self.bump(); break }
                                Tok::DotDot => { self.bump();
                                    let has_base = match self.peek() { Tok::RBrace => false, Tok::Comma => false, _ => true };
                                    if has_base { base = Some(Box::new(self.parse_expr(0))); }
                                    match self.peek() { Tok::RBrace => { self.bump(); }, _ => {} }
                                    break }   // rest is terminal: eat closer and break
                                _ => { let nm = self.ident();
                                    match self.peek() {
                                        Tok::Colon => { self.bump(); let ex = Box::new(self.parse_expr(0)); fs.push(EField { name: nm, expr: ex }); }
                                        // field shorthand `S { x }` -> x: x. Built with Vec::new()+push, NOT vec![..].
                                        // C-G2D-VECMAC-UNLOWERED: RESOLVED 2026-08-25 -- emit_s9 now lowers vec! (see s9_vec_macro).
                                        // The original note said lowering it "is macro EXPANSION, S2's job, not the lowerer's",
                                        // because a MacCall payload is raw token STRINGS. That was true when written and became
                                        // FALSE when println! gained s9_reparse_expr: the lowerer had the token-soup machinery
                                        // all along. A recorded BLOCKER goes stale exactly like a recorded LAYOUT does -- re-check
                                        // whether the missing capability has since arrived before believing one.
                                        // The Vec::new()+push here is left as-is: it is what the desugar emits anyway.
                                        _ => { let mut segs1: Vec<PathSeg> = Vec::new(); segs1.push(PathSeg { ident: nm.clone(), args: Vec::new(), paren: false }); let sp = Box::new(ExprK::Path(Box::new(Path { segments: segs1 }))); fs.push(EField { name: nm, expr: sp }); }
                                    }
                                    match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }
                            self.no_struct = sv;
                            ExprK::Struct(Box::new(p), fs, base)
    }
    // args := ( expr (, expr)* )
    fn parse_args(&mut self) -> Vec<ExprK> {
        let mut args: Vec<ExprK> = Vec::new();
        self.bump();                         // LParen
        loop { match self.peek() { Tok::RParen => { self.bump(); break }, _ => { args.push(self.parse_expr(0)); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }
        args
    }
    // postfix := atom ( .field | .method(args) | (call-args) | [index] )*  (left-assoc, tighter than unary)
    fn parse_postfix(&mut self) -> ExprK {
        let mut e = self.parse_atom();
        loop {
            match self.peek() {
                Tok::Dot => { self.bump();
                    match self.peek() {
                        Tok::Int(n) => { self.bump(); e = ExprK::Field(Box::new(e), format!("{}", n)); }   // tuple index `t.0` (rustc: Field w/ a numeric symbol); chains via the loop for `t.0.1`
                        Tok::Float(s) => { self.bump();   // `x.0.1` -- the float lexer ate `0.1`; split "A.B" into two tuple-index Fields
                            let fb = s.as_bytes(); let mut lo: i64 = 0; let mut hi: i64 = 0; let mut seen = false; let mut k = 0;
                            while k < fb.len() { let c = fb[k]; if c == 46 { seen = true; } else { if seen { hi = hi * 10 + (c - 48) as i64; } else { lo = lo * 10 + (c - 48) as i64; } } k += 1; }
                            e = ExprK::Field(Box::new(ExprK::Field(Box::new(e), format!("{}", lo))), format!("{}", hi)); }
                        _ => { let name = self.ident();
                            if name == "await" { e = ExprK::Await(Box::new(e)); }   // `.await` postfix -> rustc ExprKind::Await (not a field named await)
                            else {
                            let mut tys: Vec<Ty> = Vec::new();   // method turbofish `.name::<T>(..)`
                            match self.peek() { Tok::PathSep => { match self.peek2() { Tok::Lt => { self.bump(); self.bump();   // consume `::` `<`
                                loop { if self.eat_close_gt() { break }   // close on `>` OR a split glued `>>` (e.g. `.collect::<Vec<u8>>()`) via eat_close_gt/pending_gt
                                    let is_const = match self.peek() { Tok::Int(_) => true, Tok::Float(_) => true, Tok::Char(_) => true, Tok::LBrace => true, _ => false };   // `.m::<T, 3>()` const-generic arg (mirrors the type-path arg loop ~L328); a bare int/char/block is a const arg, not a type path -> was rendering `?`
                                    if is_const { match self.peek() { Tok::LBrace => { let cv = self.capture_group(); tys.push(Ty::ConstArg(true, cv)); }, _ => { let mut cv: Vec<String> = Vec::new(); cv.push(tok_str(&self.peek(), &self.idents)); self.bump(); tys.push(Ty::ConstArg(false, cv)); } } }
                                    else { tys.push(self.parse_ty()); }
                                    match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } }, _ => {} } }, _ => {} }
                            match self.peek() { Tok::LParen => { let args = self.parse_args(); e = ExprK::Method(Box::new(e), name, tys, args) }, _ => { e = ExprK::Field(Box::new(e), name) } } } }
                    } }
                Tok::LParen => { let args = self.parse_args(); e = ExprK::Call(Box::new(e), args) }
                Tok::LBrack => { self.bump(); let idx = Box::new(self.parse_expr(0)); self.bump(); e = ExprK::Index(Box::new(e), idx) }
                Tok::Question => { self.bump(); e = ExprK::Try(Box::new(e)) }   // e? -> Try (may repeat: e??)
                _ => break
            }
        }
        e
    }
    // unary := - unary | & [mut] unary | * unary | postfix
    // `path ! <delim> tokens <delim>` -> MacCall. Captures the delimiter kind + a flat raw-token list WITHOUT re-parsing
    // as exprs (rustc keeps the pre-expansion tokenstream raw). Assumes peek()==`!`.
    // Captures a balanced delimiter group as a flat raw-token list. Assumes peek() is the OPEN delimiter; consumes through the matching close.
    // Extracted so callers (parse_mac_tail + the item-position macro_rules path) share it WITHOUT inlining the loop -- inlining the
    // loop into parse_item pushed that fn past a size/complexity limit in the rzrs hand-rolled codegen and MISCOMPILED it (segfault),
    // even though the new path never ran. A shared helper keeps each caller small: a THIRD L6 hazard class -- function SIZE.
    fn capture_group(&mut self) -> Vec<String> {
        self.bump();                         // open delimiter
        let mut toks: Vec<String> = Vec::new();
        let mut depth = 0;
        loop {
            let closing = match self.peek() { Tok::RParen => true, Tok::RBrack => true, Tok::RBrace => true, _ => false };
            if depth == 0 { if closing { self.bump(); break; } }
            match self.peek() { Tok::LParen => { depth = depth + 1; } Tok::LBrack => { depth = depth + 1; } Tok::LBrace => { depth = depth + 1; } Tok::RParen => { depth = depth - 1; } Tok::RBrack => { depth = depth - 1; } Tok::RBrace => { depth = depth - 1; } _ => {} }
            toks.push(tok_str(&self.peek(), &self.idents)); self.bump();
        }
        toks
    }
    // Does arm i match the call (name + key)? A LITERAL arm (no metavar) matches when its matcher string == key;
    // a single-metavar arm `$x:frag` (MAC-2) matches any non-empty key (the whole arg span binds to the metavar).
    fn arm_ok(&self, i: usize, name: &str, key: &str) -> bool {
        if !streq(&self.mac_names[i], name) { return false; }
        if self.mac_reps[i].len() > 0 { return true; }   // repetition arm: matches any 0+ element sequence (MAC-3)
        let mv = &self.mac_metavars[i];
        if mv.len() == 0 { return streq(&self.mac_matchers[i], key); }   // literal arm
        let names = split_ws(mv);
        if names.len() == 1 { return key.len() > 0; }   // single metavar binds the whole arg span (MAC-2)
        let vals = split_commas(key);   // multi-metavar: as many comma-parts as metavars
        names.len() == vals.len()
    }
    // MAC-3: separator of a repetition matcher `$( GROUP )sep* / +` ("~" = empty sep), or "" if not a repetition.
    fn rep_of(&self, m: &str) -> String {
        let mt = split_ws(m);
        let n = mt.len();
        if n >= 4 && streq(&mt[0], "$") && streq(&mt[1], "(") {
            let last = &mt[n - 1];
            if streq(last, "*") || streq(last, "+") {
                let mut d = 0;
                let mut ci = 1;
                let mut k = 1;
                while k < n { if streq(&mt[k], "(") { d = d + 1; } if streq(&mt[k], ")") { d = d - 1; if d == 0 { ci = k; k = n; } } k = k + 1; }
                let lasti = n - 1;
                if ci + 1 < lasti { return mt[ci + 1].to_string(); }
                return String::from("~");
            }
        }
        String::from("")
    }
    // MAC-3: the inner GROUP of a `$( GROUP )sep*` matcher, joined (assumes rep_of != "").
    fn rep_inner(&self, m: &str) -> String {
        let mt = split_ws(m);
        let n = mt.len();
        let mut d = 0;
        let mut ci = 1;
        let mut k = 1;
        while k < n { if streq(&mt[k], "(") { d = d + 1; } if streq(&mt[k], ")") { d = d - 1; if d == 0 { ci = k; k = n; } } k = k + 1; }
        let mut inner = String::new();
        let mut j = 2;
        while j < ci { if inner.len() > 0 { inner.push(' '); } inner.push_str(&mt[j]); j = j + 1; }
        inner
    }
    // MAC-3: expand the body's `$( GROUP )bsep*` repetition group(s): for each captured element in self.rep_els,
    // substitute the repetition metavar `rep_meta` in GROUP, joined by the body-group separator bsep. Non-repetition
    // tokens pass through. Reads self.rep_els (a FIELD) -- els cannot be a &Vec param.
    fn expand_rep_body(&self, body: &str, rep_meta: &str) -> String {
        let bt = split_ws(body);
        let n = bt.len();
        let mut out = String::new();
        let mut i = 0;
        while i < n {
            if streq(&bt[i], "$") && i + 1 < n && streq(&bt[i + 1], "(") {
                let mut d = 0;
                let mut ci = i + 1;
                let mut k = i + 1;
                while k < n { if streq(&bt[k], "(") { d = d + 1; } if streq(&bt[k], ")") { d = d - 1; if d == 0 { ci = k; k = n; } } k = k + 1; }
                let mut inner = String::new();
                let mut j = i + 2;
                while j < ci { if inner.len() > 0 { inner.push(' '); } inner.push_str(&bt[j]); j = j + 1; }
                let mut bsep = String::new();
                let mut after = ci + 1;
                if after < n && !streq(&bt[after], "*") && !streq(&bt[after], "+") { bsep = bt[after].to_string(); after = after + 1; }
                if after < n && (streq(&bt[after], "*") || streq(&bt[after], "+")) { after = after + 1; }
                let mut e = 0;
                let ne = self.rep_els.len();
                while e < ne {
                    let el = &self.rep_els[e];
                    let piece = self.subst_body(&inner, rep_meta, el);
                    if out.len() > 0 { out.push(' '); }
                    if e > 0 && bsep.len() > 0 { out.push_str(&bsep); out.push(' '); }
                    out.push_str(&piece);
                    e = e + 1;
                }
                i = after;
                continue;
            }
            if out.len() > 0 { out.push(' '); } out.push_str(&bt[i]); i = i + 1;
        }
        out
    }
    // Is there an arm we may expand now? Fail-closed at the recursion limit.
    fn mac_has(&self, name: &str, key: &str) -> bool {
        if self.mac_depth >= 128 { return false; }
        let mut i = 0;
        let n = self.mac_names.len();
        while i < n { if self.arm_ok(i, name, key) { return true; } i = i + 1; }
        false
    }
    // MAC-2: if a matcher PART is exactly `$ NAME : FRAG`, return NAME (the single bound metavariable); else "".
    fn metavar_of(&self, m: &str) -> String {
        // Byte-scan the joined matcher part for the exact shape `$ NAME : FRAG` (bytes: '$' ' ' NAME ' ' ':' ' ' FRAG).
        // Pure as_bytes + slice-to_string (the lexer's proven idiom) -- re-lexing inside this method mis-lowered.
        let mb = m.as_bytes();
        if mb.len() < 3 { return String::from(""); }
        if mb[0] != 36 || mb[1] != 32 { return String::from(""); }
        let mut j = 2;
        while j < mb.len() && mb[j] != 32 { j = j + 1; }
        if j + 2 < mb.len() && mb[j] == 32 && mb[j + 1] == 58 && mb[j + 2] == 32 { let sl = &m[2..j]; return sl.to_string(); }
        String::from("")
    }
    // MAC-3: for a comma-separated matcher of single-fragment metavars `$a:expr , $b:expr , ...`, return the metavar
    // NAMES space-joined ("a b ..."); a single metavar returns just its name (MAC-2). Returns "" if ANY top-level
    // comma-part is not a single `$NAME:FRAG` (i.e. a literal/structural matcher -- handled by the literal path).
    fn matcher_metavars(&self, m: &str) -> String {
        let parts = split_commas(m);
        let mut names = String::new();
        let mut i = 0;
        let n = parts.len();
        while i < n {
            let nm = self.metavar_of(&parts[i]);
            if nm.len() == 0 { return String::from(""); }
            if names.len() > 0 { names.push(' '); } names.push_str(&nm);
            i = i + 1;
        }
        names
    }
    // MAC-2: substitute each `$ NAME` (NAME==mv) in the body token stream with the captured value tokens `val`.
    // Substitute each `$ NAME` (NAME==mv) in the space-joined body with the captured value tokens `val`.
    // Token walk over the joined string via as_bytes + slice-to_string (no lex-in-method, which mis-lowers).
    fn subst_body(&self, body: &str, mv: &str, val: &str) -> String {
        let bb = body.as_bytes();
        let n = bb.len();
        let mut out = String::new();
        let mut i = 0;
        while i < n {
            let s = i;
            while i < n && bb[i] != 32 { i = i + 1; }
            let tok = &body[s..i];
            if i < n && bb[i] == 32 { i = i + 1; }
            if streq(tok, "$") {
                let s2 = i;
                while i < n && bb[i] != 32 { i = i + 1; }
                let nxt = &body[s2..i];
                if i < n && bb[i] == 32 { i = i + 1; }
                if streq(nxt, mv) { if out.len() > 0 { out.push(' '); } out.push_str(val); continue; }
                if out.len() > 0 { out.push(' '); } out.push_str(tok);
                if out.len() > 0 { out.push(' '); } out.push_str(nxt);
                continue;
            }
            if out.len() > 0 { out.push(' '); } out.push_str(tok);
        }
        out
    }
    // MAC-1/2: parse the def's raw rule tokens from self.rule_toks into ONE ROW PER ARM. An arm is
    // `( MATCHER ) => { BODY } [;]`; matcher/body captured as balanced-delimiter joined-token strings
    // (matcher "" = empty). A `$ NAME : FRAG` matcher records its metavar. Reads a struct FIELD (not a &Vec param).
    fn mac_register_arms(&mut self, name: String) {
        let rn = self.rule_toks.len();
        let mut i = 0;
        while i < rn {
            if !streq(&self.rule_toks[i], "(") { i = i + 1; continue; }   // resync to the next arm opener
            i = i + 1;
            let mut m = String::new();
            let mut d = 0;
            while i < rn { if streq(&self.rule_toks[i], ")") && d == 0 { i = i + 1; break; }
                if streq(&self.rule_toks[i], "(") || streq(&self.rule_toks[i], "[") || streq(&self.rule_toks[i], "{") { d = d + 1; }
                if streq(&self.rule_toks[i], ")") || streq(&self.rule_toks[i], "]") || streq(&self.rule_toks[i], "}") { d = d - 1; }
                if m.len() > 0 { m.push(' '); } m.push_str(&self.rule_toks[i]); i = i + 1; }
            if i < rn && streq(&self.rule_toks[i], "=>") { i = i + 1; }
            // CD-19: MAC body delimiter can be { }, ( ), or [ ] (serialize tuple!/peel! use ( )-bodied arms `=> ( .. )`;
            // the prior code only consumed `{` + terminated on `}`, so a ()-bodied arm left `(` in the body and the loop
            // swallowed all following arms -- losing the real matcher of a multi-arm macro with an empty first arm).
            let mut bo: i64 = 0;
            if i < rn { if streq(&self.rule_toks[i], "{") { bo = 0; i = i + 1; } else { if streq(&self.rule_toks[i], "(") { bo = 1; i = i + 1; } else { if streq(&self.rule_toks[i], "[") { bo = 2; i = i + 1; } } } }
            let mut b = String::new();
            let mut d2 = 0;
            while i < rn {
                let mut isclose = false;
                if bo == 0 { if streq(&self.rule_toks[i], "}") { isclose = true; } } else { if bo == 1 { if streq(&self.rule_toks[i], ")") { isclose = true; } } else { if streq(&self.rule_toks[i], "]") { isclose = true; } } }
                if isclose && d2 == 0 { i = i + 1; break; }
                if streq(&self.rule_toks[i], "(") || streq(&self.rule_toks[i], "[") || streq(&self.rule_toks[i], "{") { d2 = d2 + 1; }
                if streq(&self.rule_toks[i], ")") || streq(&self.rule_toks[i], "]") || streq(&self.rule_toks[i], "}") { d2 = d2 - 1; }
                if b.len() > 0 { b.push(' '); } b.push_str(&self.rule_toks[i]); i = i + 1; }
            if i < rn && streq(&self.rule_toks[i], ";") { i = i + 1; }
            let rep = self.rep_of(&m);
            let mv;
            if rep.len() > 0 { let inner = self.rep_inner(&m); mv = self.matcher_metavars(&inner); }
            else { mv = self.matcher_metavars(&m); }
            self.mac_names.push(name.clone());
            self.mac_matchers.push(m);
            self.mac_bodies.push(b);
            self.mac_metavars.push(mv);
            self.mac_reps.push(rep);
        }
    }
    // S2-MACRO first slice: an ITEM-position macro CALL of an `$( impl PATH for $T { members } )+` macro expands to one impl
    // per $T in the invocation, each with the arm-body impl's fn/const members. Emits `{base}::{impl#K}` + `::member` per
    // generated impl, threading impl_k (macro impls are hand-written-equivalent -> PASS A position). Skips macro DEFS and any
    // macro whose registered body is not the impl-rep shape (-> no false positive -> no numbering regression). Needs self
    // for the mac_* registry (populated in this same crate driver by mac_register_arms).
    fn emit_mac_impls(&self, cratename: &str, base: &str, e: &ExprK, impl_k: i64) -> i64 {
        match e {
            ExprK::MacCall(pp, delim, toks) => {
                if streq(delim, "MacroDefBrace") { return impl_k; }   // a macro_rules DEF, not a call
                let name = dump_path(pp);
                let mut bi: i64 = -1;
                let mut i: usize = 0; let nn = self.mac_names.len();
                while i < nn { if streq(&self.mac_names[i], &name) { if mac_body_is_impl_rep(&self.mac_bodies[i]) { bi = i as i64; } } i = i + 1; }
                if bi < 0 {
                    // impl_pos!-shape ($(struct+derive+impl)+): mac_body_is_impl_rep is false, so bi<0. Emit the ADDITIVE
                    // struct-name owners INLINE here (a new Parser method is NOT dispatched under newstage0 -- B1). Copy
                    // each name-matching arm body to a LOCAL (word-iter detector), then scan the invocation for names.
                    let mut rk = impl_k;
                    let mut i2: usize = 0;
                    while i2 < nn {
                        if streq(&self.mac_names[i2], &name) {
                            let mut bl = String::new(); bl.push_str(&self.mac_bodies[i2]);
                            if mac_body_gen_struct(&bl) {
                                let mut inv2 = String::new();
                                for t in toks.iter() { if inv2.len() > 0 { inv2.push(' '); } inv2.push_str(t); }
                                emit_mac_struct_names2(cratename, base, &inv2);
                                let mut mat2 = String::new(); mat2.push_str(&self.mac_matchers[i2]);
                                emit_positional_struct_names(cratename, base, &inv2, &mat2, &bl);   // body `struct $META` + non-rep matcher -> positional invocation ident (typed_def_id!)
                                emit_last_metavar_struct(cratename, base, &inv2, &mat2, &bl);   // struct named by the matcher's LAST metavar -> invocation's last ident (atomic_int!, 128 skipped)
                                emit_mac_mod(cratename, base, &inv2, &bl);   // body `mod NAME { .. $(struct $META)+ }` -> NAME + fixed items + repetition struct names (nonzero impl_zeroable_primitive!)
                                // S8 impl_pos! (2026-08-14): body carries `impl A/B/C for $ident { .. }` blocks. TIER = GROUP 1
                                // at the macro source position: PHASE A per invocation struct emit Pos/Add/Sub {impl#k}+members;
                                // PHASE B per struct emit its derives (list order). Verified vs span.seq.txt #93-130.
                                if str_contains(&bl, "impl ") {
                                    let cntg = count_inv_structs(&inv2);
                                    let mut gi: i64 = 0;
                                    while gi < cntg { rk = emit_body_impl_blocks(cratename, base, &bl, rk); gi = gi + 1; }   // PHASE A
                                    rk = emit_invocation_struct_derives(cratename, base, &inv2, rk);                        // PHASE B
                                }
                            }
                            if mac_body_gen_enum(&bl) { emit_mac_enum_names(cratename, base, &bl); }
                            if mac_body_gen_fn(&bl) {   // CD-16: module-level fn-gen macro (leb128 impl_*_leb128!) -> <base>::<fnname>
                                let mut invf = String::new(); for t in toks.iter() { if invf.len() > 0 { invf.push(' '); } invf.push_str(t); }
                                let mut matf = String::new(); matf.push_str(&self.mac_matchers[i2]);
                                emit_positional_fn_names(cratename, base, &invf, &matf, &bl);
                            }
                            if mac_is_recursive_impl(&self.mac_matchers[i2], &bl) {   // CD-18/19: peano-recursive tuple-impl macro -> count_impl_blocks x ident-depth impls
                                let mut invr = String::new(); for t in toks.iter() { if invr.len() > 0 { invr.push(' '); } invr.push_str(t); }
                                let depth = count_type_groups(&invr);
                                let mut d: i64 = 0;
                                while d < depth { rk = emit_body_impl_blocks(cratename, base, &bl, rk); d = d + 1; }
                            }
                            let meta = mac_typealias_meta(&bl);
                            if meta.len() > 0 {
                                let mut mat = String::new(); mat.push_str(&self.mac_matchers[i2]);
                                let pre = metavar_prefix(&mat, &meta);
                                if pre.len() > 0 {
                                    let mut inv3 = String::new();
                                    for t in toks.iter() { if inv3.len() > 0 { inv3.push(' '); } inv3.push_str(t); }
                                    emit_typealias_names(cratename, base, &inv3, &pre);
                                }
                            }
                        }
                        i2 = i2 + 1;
                    }
                    return rk;
                }
                let mut bodyloc = String::new(); bodyloc.push_str(&self.mac_bodies[bi as usize]);   // COPY out of the Vec-index (rzrs #6/7: Vec-index-derived byte-scan mis-lowers; scan a LOCAL)
                let mut inv = String::new();
                for t in toks.iter() { if inv.len() > 0 { inv.push(' '); } inv.push_str(t); }
                let cnt = count_type_groups(&inv);
                let nimpl = count_impl_blocks(&bodyloc);   // CD-17: 2+ impl blocks per repetition (direct_serialize_impls!)
                let mut k = impl_k; let mut g: i64 = 0;
                while g < cnt {
                    if nimpl >= 2 { k = emit_body_impl_blocks(cratename, base, &bodyloc, k); }   // one {impl#N} per block, members per block
                    else {
                        let ip = if base.len() == 0 { format!("{{impl#{}}}", k) } else { format!("{}::{{impl#{}}}", base, k) };
                        println!("CRATERES {}|{}", cratename, ip);
                        emit_impl_members(cratename, &ip, &bodyloc);
                        k = k + 1;
                    }
                    g = g + 1;
                }
                k
            }
            _ => impl_k
        }
    }
    // S2-MACRO slice 2 detect: the macro at index i is a METHOD-generating repetition -- its body generates `fn $<metavar>`
    // (a fn NAMED by a metavar; distinct from impl-rep's concrete `fn tokens`). Copy the body local (Vec-index byte-scan
    // mis-lowers) and test for `fn $ `.
    fn mac_is_method_gen(&self, i: usize) -> bool {
        let mut bl = String::new(); bl.push_str(&self.mac_bodies[i]);
        str_contains(&bl, "fn $ ")
    }
    // Emit the methods a method-gen macro CALL generates into the enclosing impl (ipath): one `{ipath}::name` per
    // `;`-separated invocation group, named by the group's first ident. Skips defs + non-method-gen macros.
    fn emit_mac_methods(&self, cratename: &str, ipath: &str, e: &ExprK) {
        match e {
            ExprK::MacCall(pp, delim, toks) => {
                if streq(delim, "MacroDefBrace") { return; }
                let name = dump_path(pp);
                let mut bi: i64 = -1; let mut i: usize = 0; let nn = self.mac_names.len();
                while i < nn { if streq(&self.mac_names[i], &name) { if self.mac_is_method_gen(i) { bi = i as i64; } } i = i + 1; }
                if bi < 0 {
                    // FIXED-name method-gen: an EMPTY-matcher macro whose body has concrete `fn NAME` members (e.g.
                    // bit_relations_inherent_impls! -> union/subtract/intersect on the enclosing inherent impl).
                    let mut j2: usize = 0;
                    while j2 < nn {
                        if streq(&self.mac_names[j2], &name) {
                            let mut mm = String::new(); mm.push_str(&self.mac_matchers[j2]);
                            if mm.len() == 0 { let mut fb = String::new(); fb.push_str(&self.mac_bodies[j2]); emit_fixed_impl_methods(cratename, ipath, &fb); }
                        }
                        j2 = j2 + 1;
                    }
                    return;
                }
                let mut inv = String::new();
                for t in toks.iter() { if inv.len() > 0 { inv.push(' '); } inv.push_str(t); }
                // The comma-group splitter applies ONLY to a REPETITION body (`$( .. )*`, one method per group). A
                // NON-repetition method-gen macro (e.g. `diagnostic_method!(name, Level::X)` -- matcher `$name, $level`)
                // generates ONE method per invocation named by the FIRST arg; its comma separates ARGS not method-groups,
                // so it must stay on the single-group semi path (emit only the first ident). Within a repetition, pick
                // semi (`;`-terminated, hir expect_methods) vs comma (`,`-terminated, proc_macro suffixed_int_literals).
                let mut bl = String::new(); bl.push_str(&self.mac_bodies[bi as usize]);
                if str_contains(&bl, "$ (") { if str_contains(&inv, ";") { emit_semi_group_first_idents(cratename, ipath, &inv); } else { emit_comma_group_first_idents(cratename, ipath, &inv); } }
                else { emit_semi_group_first_idents(cratename, ipath, &inv); }
            }
            _ => {}
        }
    }
    // For a top-level impl, expand any method-gen macro CALLS among its inner items into `{ipath}::name` (additive methods).
    fn emit_impl_macro_methods(&self, cratename: &str, ipath: &str, it: &Item) {
        match &it.kind {
            ItemKind::Impl(_g, _tr, _ty, inner) => { for m in inner.iter() { match &m.kind { ItemKind::MacCall(e) => { self.emit_mac_methods(cratename, ipath, e); }, ItemKind::Fn(_gf, _pf, _rf, _bf, _af) => { emit_method_mac_consts(cratename, ipath, m, &self.mac_names, &self.mac_bodies); }, _ => {} } } }
            _ => {}
        }
    }
    // Expand a macro call in EXPRESSION position: find the first matching arm, substitute a bound metavar (if any)
    // into its body, re-lex, sub-parse as an expr, and return that AST in place of the MacCall.
    fn mac_expand_expr(&mut self, name: &str, key: &str) -> ExprK {
        let mut idx = 0;
        let mut found = false;
        let mut i = 0;
        let n = self.mac_names.len();
        while i < n { if !found && self.arm_ok(i, name, key) { idx = i; found = true; } i = i + 1; }
        let body = self.mac_bodies[idx].clone();
        let mv = self.mac_metavars[idx].clone();
        let rp = self.mac_reps[idx].clone();
        // Bind metavars to the call args and substitute. Repetition arm -> split the call on the sep into elements and
        // expand the body's `$(...)*` group per element. Single metavar -> the whole key. Multiple -> the key split on
        // top-level commas, bound positionally. A literal arm (mv=="") passes the body through unchanged.
        let mut sbody = body;
        if rp.len() > 0 {
            // Split the call on the repetition separator into elements. Dispatch to LITERAL-arg split calls: passing a
            // &String sep (from a local) to split_bysep mis-lowered under rzrs, where a &'static str literal works.
            if streq(&rp, "~") { self.rep_els = split_bysep(key, ""); }
            else if streq(&rp, ",") { self.rep_els = split_commas(key); }
            else if streq(&rp, ";") { self.rep_els = split_bysep(key, ";"); }
            else { self.rep_els = split_bysep(key, ","); }
            let bd = self.mac_bodies[idx].clone();
            sbody = self.expand_rep_body(&bd, &mv);
        } else if mv.len() > 0 {
            let names = split_ws(&mv);
            if names.len() == 1 { sbody = self.subst_body(&sbody, &mv, key); }
            else {
                let vals = split_commas(key);
                let mut k = 0;
                while k < names.len() { let nm = &names[k]; let vv = &vals[k]; sbody = self.subst_body(&sbody, nm, vv); k = k + 1; }
            }
        }
        let (bt, bo, idn) = lex(&sbody);
        let mut sub = Parser { idents: idn, toks: bt, offs: bo, pos: 0, no_struct: false, expr_path: false, pending_gt: 0, in_cond: false, expand: true, mac_names: veccopy(&self.mac_names), mac_matchers: veccopy(&self.mac_matchers), mac_bodies: veccopy(&self.mac_bodies), mac_metavars: veccopy(&self.mac_metavars), mac_reps: veccopy(&self.mac_reps), mac_depth: self.mac_depth + 1, rule_toks: Vec::new(), rep_els: Vec::new(), drop_stmt: false, stmt_attrs: Vec::new(), resolve: false, moddir: String::from(""), pend_move: false, pend_async: false };
        sub.parse_expr(0)
    }
    // MAC-7: `concat!(lit, lit, ...)` -> a single string literal joining the parts (string literals de-quoted,
    // other literal tokens verbatim). Its OWN helper -- inlining this into parse_mac_tail (near the fn-size envelope)
    // globally miscompiled the rzrs build.
    fn concat_lits(&self, key: &str) -> String {
        // Byte-walk the joined arg string; emit the contents of each `"..."` string literal. Slices the &str PARAM
        // `key` (the subst_body idiom that lowers), NOT a Vec-indexed &String slice (that mis-lowered).
        let kb = key.as_bytes();
        let n = kb.len();
        let mut out = String::new();
        let mut i = 0;
        while i < n {
            if kb[i] == 34 {
                let st = i + 1;
                let mut j = st;
                while j < n && kb[j] != 34 { j = j + 1; }
                let inner = &key[st..j];
                out.push_str(inner);
                i = j + 1;
                continue;
            }
            i = i + 1;
        }
        out
    }
    // MAC-8: `cfg!(PRED)` -> a bool constant. 1=true, 0=false, -1=unknown (host-dependent predicate -> leave as
    // MacCall, no witness exercises it). Only the host-INDEPENDENT empties are handled: `all()` is vacuously true,
    // `any()` vacuously false. Its own helper (parse_mac_tail is near the fn-size envelope).
    fn cfg_bool(&self, key: &str) -> i32 {
        if streq(key, "all ( )") { return 1; }
        if streq(key, "any ( )") { return 0; }
        -1
    }
    fn parse_mac_tail(&mut self, path: Path) -> ExprK {
        self.bump();                         // `!`
        let delim = match self.peek() { Tok::LParen => String::from("Paren"), Tok::LBrack => String::from("Bracket"), Tok::LBrace => String::from("Brace"), _ => String::from("None") };
        let toks = self.capture_group();
        // MAC-1: in expand mode, dispatch a call to a registered macro by matching the joined call-arg tokens
        // against each arm's matcher; splice the winning arm's expanded body.
        if self.expand && path.segments.len() == 1 {
            let name = mac_first_seg(&path);
            let mut key = String::new();
            let mut ki = 0;
            while ki < toks.len() { if key.len() > 0 { key.push(' '); } key.push_str(&toks[ki]); ki = ki + 1; }
            // MAC-7: built-in fn-like macros (NOT macro_rules-defined) -- name-keyed direct expansion, checked before
            // the registry. `stringify!(TOKENS)` -> a string literal of the space-joined token text (rustc spacing).
            if streq(&name, "stringify") { return ExprK::Str(key); }
            if streq(&name, "concat") { return ExprK::Str(self.concat_lits(&key)); }
            if streq(&name, "cfg") { let c = self.cfg_bool(&key); if c == 1 { return ExprK::Bool(true); } if c == 0 { return ExprK::Bool(false); } }
            if self.mac_has(&name, &key) { return self.mac_expand_expr(&name, &key); }
        }
        ExprK::MacCall(Box::new(path), delim, toks)
    }
    fn parse_unary(&mut self) -> ExprK {
        match self.peek() {
            Tok::Minus => { self.bump(); ExprK::Unary(String::from("Neg"), Box::new(self.parse_unary())) }
            Tok::Bang => { self.bump(); ExprK::Unary(String::from("Not"), Box::new(self.parse_unary())) }   // `!x` logical/bitwise not (rustc UnOp::Not)
            Tok::Amp => { self.bump();
                match self.peek() { Tok::Ident(ix) => { let r = self.id_str(ix); if r == "raw" { match self.peek2() {   // `&raw const e` / `&raw mut e` -> rustc ExprKind::AddrOf(Raw, ..); `raw` is contextual only before const/mut
                    Tok::Const => { self.bump(); self.bump(); return ExprK::RawRef(false, Box::new(self.parse_unary())); }
                    Tok::Mut => { self.bump(); self.bump(); return ExprK::RawRef(true, Box::new(self.parse_unary())); }
                    _ => {} } } }, _ => {} }
                let mut m = false; match self.peek() { Tok::Mut => { self.bump(); m = true; }, _ => {} } ExprK::Ref(m, Box::new(self.parse_unary())) }
            Tok::AndAnd => { self.bump(); let mut m = false; match self.peek() { Tok::Mut => { self.bump(); m = true; }, _ => {} } ExprK::Ref(false, Box::new(ExprK::Ref(m, Box::new(self.parse_unary())))) }   // prefix `&&x` double-ref (rustc AddrOf(AddrOf(..))); `a && b` binop is unaffected (handled in the climb)
            Tok::Star => { self.bump(); ExprK::Deref(Box::new(self.parse_unary())) }
            _ => self.parse_postfix()
        }
    }
    // cast := unary ( as ty )*
    fn parse_cast(&mut self) -> ExprK {
        let mut e = self.parse_unary();
        loop { match self.peek() { Tok::As => { self.bump(); let ty = Box::new(self.parse_ty()); e = ExprK::Cast(Box::new(e), ty); }, _ => break } }
        e
    }
    // expr := precedence-climbing over binops (+ - = bp 1; * / = bp 2)
    // for `OP=` compound assigns (`-=`,`*=`,`/=`,`%=`,`&=`,`|=`,`^=`,`<<=`,`>>=`): the assign-op name, else "".
    // Detected as an assignable binop token immediately followed by `=` (peek2). `+=` is a distinct PlusEq token.
    fn compound_assign_op(&self) -> String {
        let is_eq2 = match self.peek2() { Tok::Eq => true, _ => false };
        if is_eq2 == false { return String::new(); }
        match self.peek() { Tok::Minus => String::from("SubAssign"), Tok::Star => String::from("MulAssign"), Tok::Slash => String::from("DivAssign"), Tok::Percent => String::from("RemAssign"), Tok::Amp => String::from("BitAndAssign"), Tok::Pipe => String::from("BitOrAssign"), Tok::Caret => String::from("BitXorAssign"), Tok::Shl => String::from("ShlAssign"), Tok::Shr => String::from("ShrAssign"), _ => String::new() }
    }
    fn parse_expr(&mut self, min_bp: i64) -> ExprK {
        // `let PAT = EXPR` is a valid operand only inside an if/while condition (self.in_cond).
        // The scrutinee EXPR binds at a restricted floor (bp 3) that STOPS before `&&`(2)/`||`(1),
        // so `let P = a && b` parses as `(let P = a) && b` and the outer climb joins the chain
        // (rustc renders each let as ExprKind::Let joined by Binary(And)). Cleared inside the scrutinee.
        let is_let = match self.peek() { Tok::Let => self.in_cond, _ => false };
        let mut lhs = if is_let {
            self.bump();                         // let
            let pat = Box::new(self.parse_pat());
            self.bump();                         // =
            let svc = self.in_cond; self.in_cond = false;
            let scrut = Box::new(self.parse_expr(3));
            self.in_cond = svc;
            ExprK::Let(pat, scrut)
        } else { self.parse_cast() };
        loop {
            let t = self.peek();
            let b = binop_bp(&t);
            if b == 0 || b < min_bp { break; }
            if self.compound_assign_op().len() > 0 { break; }   // `OP=` is a compound assign, not a binop -- leave it for the tail
            self.bump();
            let rhs = self.parse_expr(b + 1);
            lhs = ExprK::Bin(binop_name(&t), Box::new(lhs), Box::new(rhs));
        }
        if min_bp == 0 {
            let ca = self.compound_assign_op();
            if ca.len() > 0 { self.bump(); self.bump(); let rhs = self.parse_expr(0); lhs = ExprK::AssignOp(ca, Box::new(lhs), Box::new(rhs)); }
            else { match self.peek() {
                Tok::DotDot => { self.bump(); let end = self.range_end_opt(); lhs = ExprK::Range(Some(Box::new(lhs)), end, false); }   // `a..` / `a..b`
                Tok::DotDotEq => { self.bump(); let end = self.range_end_opt(); lhs = ExprK::Range(Some(Box::new(lhs)), end, true); }   // `a..=b`
                Tok::Eq => { self.bump(); let rhs = self.parse_expr(0); lhs = ExprK::Assign(Box::new(lhs), Box::new(rhs)); }
                Tok::PlusEq => { self.bump(); let rhs = self.parse_expr(0); lhs = ExprK::AssignOp(String::from("AddAssign"), Box::new(lhs), Box::new(rhs)); }
                _ => {}
            } }
        }
        lhs
    }
    // field := [pub] IDENT : TYPE
    fn parse_field(&mut self) -> Field {
        let _ = self.parse_attrs();          // field-position outer attrs `struct S { #[cfg(a)] b: u8 }` -- consumed (Field has no attr slot in the reduced dump); without this the `#[..]` desynced into a spin (ledger #4 L4-h1 HANG)
        let mut is_pub = false;
        match self.peek() { Tok::Pub => { self.bump(); is_pub = true;
            match self.peek() { Tok::LParen => { loop { match self.peek() { Tok::RParen => { self.bump(); break }, Tok::End => break, _ => { self.bump(); } } } }, _ => {} } }, _ => {} }   // `pub(crate)`/`pub(in path)` field restriction -- consume the `(..)` (reduced dump keeps just vis=pub); was leaking `(crate)` as a bogus `?` field
        let ident = self.ident();
        self.bump();                         // colon
        let ty = Box::new(self.parse_ty());
        Field { is_pub: is_pub, ident: ident, ty: ty }
    }
    // struct-fields := LBRACE (field (, field)* ,?)? RBRACE
    fn parse_struct_fields(&mut self) -> Vec<Field> {
        let mut fields: Vec<Field> = Vec::new();
        self.bump();                         // LBrace
        loop {
            match self.peek() {
                Tok::RBrace => { self.bump(); break }
                _ => { fields.push(self.parse_field()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } }
            }
        }
        fields
    }
    // variant := IDENT ( ( ty (, ty)* ) | struct-fields )? ;  kind = Unit | Tuple | Struct
    fn parse_variant(&mut self) -> Variant {
        let _ = self.parse_attrs();          // variant-position outer attrs `enum E { #[cfg(a)] V, W }` -- consumed (Variant has no attr slot in the reduced dump)
        let ident = self.ident();
        let kind = match self.peek() {
            Tok::LParen => { self.bump(); let mut tys: Vec<Ty> = Vec::new();
                loop { match self.peek() { Tok::RParen => { self.bump(); break }, _ => { tys.push(self.parse_ty()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }
                VariantKind::Tuple(tys) }
            Tok::LBrace => { let fields = self.parse_struct_fields(); VariantKind::Struct(fields) }
            _ => VariantKind::Unit,
        };
        let mut disr: Option<P<ExprK>> = None;   // `A = <const-expr>` explicit discriminant (rustc disr_expr)
        match self.peek() { Tok::Eq => { self.bump(); disr = Some(Box::new(self.parse_expr(0))); }, _ => {} }
        Variant { ident: ident, kind: kind, disr: disr }
    }
    // enum-variants := LBRACE (variant (, variant)* ,?)? RBRACE
    fn parse_enum_variants(&mut self) -> Vec<Variant> {
        let mut variants: Vec<Variant> = Vec::new();
        self.bump();                         // LBrace
        loop {
            match self.peek() {
                Tok::RBrace => { self.bump(); break }
                _ => { variants.push(self.parse_variant()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } }
            }
        }
        variants
    }
    // param := IDENT : TYPE
    fn parse_param(&mut self) -> Param {
        let _ = self.parse_attrs();          // param-position attrs `fn f(#[allow(unused)] x: u8)` -- consumed (Param has no attr slot in the reduced dump)
        let name = self.parse_param_binding_name();
        self.bump();                         // colon
        let ty = Box::new(self.parse_ty());
        Param { name: name, ty: ty }
    }
    // Binding position is a full pattern, not just mut-ident. Fast path keeps the plain mut-ident
    // binding unchanged. A binding that opens a real pattern parses a full Pat via parse_pat_single
    // and folds it to a String via dump_pat, so Param keeps its String name field with no variant
    // enlargement. Note: inline comments here avoid unbalanced delimiter glyphs; the newstage0
    // lexer brace-tracks through line comments, so an unbalanced open glyph would desync it.
    fn parse_param_binding_name(&mut self) -> String {
        let is_pat = match self.peek() {
            Tok::LParen => true,
            Tok::Amp => true,
            Tok::LBrack => true,
            Tok::Ident(_) => { match self.peek2() { Tok::LParen => true, Tok::LBrace => true, Tok::PathSep => true, _ => false } }
            _ => false,
        };
        if is_pat {
            let p = self.parse_pat_single();
            dump_pat(&p)
        } else {
            let is_mut = match self.peek() { Tok::Mut => { self.bump(); true }, _ => false };
            let raw = self.ident();
            if is_mut { format!("mut {}", raw) } else { raw }
        }
    }
    // fn-params := LPAREN (param (, param)* ,?)? RPAREN
    fn parse_fn_params(&mut self) -> Vec<Param> {
        let mut params: Vec<Param> = Vec::new();
        self.bump();                         // LParen
        // optional leading self-receiver (`self` / `&self` / `&mut self`) -> a Param name=self
        let mut got_self = false;
        match self.peek() {
            Tok::Amp => { got_self = true; }
            Tok::Ident(ix) => { let nm = self.id_str(ix); if nm == String::from("self") { got_self = true; } }
            Tok::Mut => { match self.peek2() { Tok::Ident(ix) => { let nm = self.id_str(ix); if nm == String::from("self") { got_self = true; } }, _ => {} } }   // by-value `mut self` receiver: leads with `mut` (not `&`); guard on peek2==self so `fn f(mut x: u8)` is NOT mistaken for a receiver. The got_self branch's line-734 `mut`-consume already handles it once detected.
            _ => {}
        }
        if got_self {
            // S1-amendment (receiver form, 2026-08-10): capture `&` and `mut` so the receiver form self / &self /
            // &mut self / mut self is recorded in Param.name (NO AST enum/struct change -- finding-#4-safe encoding).
            let mut has_amp = false; let mut has_mut = false;
            match self.peek() { Tok::Amp => { self.bump(); has_amp = true; }, _ => {} }
            match self.peek() { Tok::Lifetime(_) => { self.bump(); }, _ => {} }   // `&'a self` / `&'a mut self` -- consume the explicit lifetime after `&` before `self`
            match self.peek() { Tok::Mut => { self.bump(); has_mut = true; }, _ => {} }
            self.bump();                     // self
            let recv = if has_amp { if has_mut { String::from("&mut self") } else { String::from("&self") } } else { if has_mut { String::from("mut self") } else { String::from("self") } };
            match self.peek() {
                Tok::Colon => { self.bump(); let ty = self.parse_ty(); params.push(Param { name: String::from("self"), ty: Box::new(ty) }); }   // explicit-typed receiver `self: Box<Self>` -> keep name=self; the ty carries the type
                _ => { let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: String::from("Self"), args: Vec::new(), paren: false }); params.push(Param { name: recv, ty: Box::new(Ty::Path(Box::new(Path { segments: segs }))) }); }   // bare/ref receiver -> the receiver form in name, synthesized `Self`-path ty
            }
            match self.peek() { Tok::Comma => { self.bump(); }, _ => {} }
        }
        loop {
            match self.peek() {
                Tok::RParen => { self.bump(); break }
                Tok::DotDot => { match self.peek2() { Tok::Dot => { self.bump(); self.bump(); params.push(Param { name: String::from("..."), ty: Box::new(Ty::CVarArgs) }); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } }, _ => { params.push(self.parse_param()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }   // terminal `...` (lexed DotDot+Dot) C-variadic param (mirrors parse_fnptr_after_fn ~L414); covers foreign fns `fn f(x: u8, ...)` and regular fns
                _ => { params.push(self.parse_param()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } }
            }
        }
        params
    }
    // pat := pat-single ( | pat-single )*  (Or)
    fn parse_pat(&mut self) -> Pat {
        match self.peek() { Tok::Pipe => { self.bump(); }, _ => {} }   // tolerate an optional leading `|` (match-arm or-pattern `| A | B =>`)
        let first = self.parse_pat_single();
        match self.peek() {
            Tok::Pipe => { let mut alts: Vec<Pat> = Vec::new(); alts.push(first);
                loop { match self.peek() { Tok::Pipe => { self.bump(); alts.push(self.parse_pat_single()); }, _ => break } }
                Pat::Or(alts) }
            _ => first
        }
    }
    // struct-pattern body `{ f, g: subpat, .. }` -> (fields, has_rest). Called with peek == LBrace.
    fn parse_pat_struct_body(&mut self) -> (Vec<FieldPat>, bool) {
        self.bump();                         // LBrace
        let mut fields: Vec<FieldPat> = Vec::new(); let mut rest = false;
        loop {
            match self.peek() {
                Tok::RBrace => { self.bump(); break }
                Tok::DotDot => { self.bump(); rest = true; }
                _ => { let nm = self.ident();
                    match self.peek() {
                        Tok::Colon => { self.bump(); let sp = Box::new(self.parse_pat()); fields.push(FieldPat { name: nm, pat: sp, shorthand: false }); }
                        _ => { let b = Box::new(Pat::Ident(nm.clone())); fields.push(FieldPat { name: nm, pat: b, shorthand: true }); }
                    }
                    match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } }
            }
        }
        (fields, rest)
    }
    fn read_path_pat(&mut self) -> ExprK {
        let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: self.ident(), args: Vec::new(), paren: false });
        loop { match self.peek() { Tok::PathSep => { self.bump(); segs.push(PathSeg { ident: self.ident(), args: Vec::new(), paren: false }); }, _ => break } }
        ExprK::Path(Box::new(Path { segments: segs }))
    }
    fn read_lit_pat(&mut self) -> ExprK {
        match self.peek() { Tok::Ident(_) => { return self.read_path_pat(); }, _ => {} }   // path endpoint e.g. i64::MAX
        let neg = match self.peek() { Tok::Minus => { self.bump(); true }, _ => false };
        let base = match self.bump() { Tok::Int(n) => ExprK::LitInt(n), Tok::Char(cv) => ExprK::Char(cv), Tok::Float(fv) => ExprK::LitFloat(fv), _ => ExprK::LitInt(0) };
        if neg { ExprK::Unary(String::from("Neg"), Box::new(base)) } else { base }   // rustc PatKind::Range bound -> Unary(Neg, Lit)
    }
    // after a pattern `..`/`..=`, read an endpoint ONLY when a real endpoint-start token follows (literal/char/float/`-`/path-ident); a terminator (`=>`/`,`/`}`/`)`/`]`/`|`/`if`/End/`=`) means an ABSENT endpoint (half-open range). never bumps a non-endpoint token -> no fabrication, no `=> body` theft, no desync.
    fn pat_range_end_opt(&mut self) -> Option<ExprK> {
        match self.peek() {
            Tok::Int(_) => Some(self.read_lit_pat()),
            Tok::Char(_) => Some(self.read_lit_pat()),
            Tok::Float(_) => Some(self.read_lit_pat()),
            Tok::Minus => Some(self.read_lit_pat()),
            Tok::Ident(_) => Some(self.read_lit_pat()),
            _ => None,
        }
    }
    // a literal pattern low bound; if `..`/`..=` follows -> a range pattern `lo..hi` / `lo..=hi` (fully-bounded == rustc PatKind::Range, unchanged shape) or `lo..` (RangeFrom, open end -> Pat::Lit(ExprK::Range(Some,None,inc)) reusing the proven expr-range representation).
    fn maybe_range_pat(&mut self, lo: ExprK) -> Pat {
        match self.peek() {
            Tok::DotDot => { self.bump();
                match self.pat_range_end_opt() {
                    Some(hi) => Pat::Range(Box::new(lo), Box::new(hi), false),
                    None => Pat::Lit(Box::new(ExprK::Range(Some(Box::new(lo)), None, false))),
                } }
            Tok::DotDotEq => { self.bump();
                match self.pat_range_end_opt() {
                    Some(hi) => Pat::Range(Box::new(lo), Box::new(hi), true),
                    None => Pat::Lit(Box::new(ExprK::Range(Some(Box::new(lo)), None, true))),
                } }
            _ => Pat::Lit(Box::new(lo))
        }
    }
    // a single-ident tail: `X..`/`X..=Y` is a path-low-bound range pat; else `_` -> Wild, any other ident -> a binding. flat (early return then terminal if-else) so no nested-if lands in a hot match arm.
    fn ident_or_range_pat(&mut self, n0: String) -> Pat {
        let is_range = match self.peek() { Tok::DotDot => true, Tok::DotDotEq => true, _ => false };
        if is_range {
            let mut segs: Vec<PathSeg> = Vec::new();
            segs.push(PathSeg { ident: n0, args: Vec::new(), paren: false });
            return self.maybe_range_pat(ExprK::Path(Box::new(Path { segments: segs })));
        }
        if n0 == "_" { Pat::Wild } else { Pat::Ident(n0) }
    }
    // pat-single := ( pat (, pat)* ) | INT (Lit) | IDENT ( ( pat (, pat)* ) )? (TupleStruct) | _ (Wild) | IDENT (binding)
    fn parse_pat_single(&mut self) -> Pat {
        match self.peek() {
            Tok::LParen => { self.bump(); let mut pats: Vec<Pat> = Vec::new();
                loop { match self.peek() { Tok::RParen => { self.bump(); break }, _ => { pats.push(self.parse_pat()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }
                Pat::Tuple(pats) }
            Tok::LBrack => { self.bump(); let mut pats: Vec<Pat> = Vec::new();   // slice-pattern `[a, .., z]` (rustc PatKind::Slice; `..` -> Rest)
                loop { match self.peek() { Tok::RBrack => { self.bump(); break }, _ => { pats.push(self.parse_pat()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }
                Pat::Slice(pats) }
            Tok::DotDot => { self.bump();   // `..N` RangeTo (open low end) if a real endpoint follows; bare `..` -> rustc PatKind::Rest (slice / `x @ ..`)
                match self.pat_range_end_opt() {
                    Some(hi) => Pat::Lit(Box::new(ExprK::Range(None, Some(Box::new(hi)), false))),
                    None => Pat::Rest,
                } }
            Tok::DotDotEq => { self.bump();   // `..=N` RangeTo inclusive (open low end); `..=` with no endpoint is invalid rust -> bail to Rest for forward-progress
                match self.pat_range_end_opt() {
                    Some(hi) => Pat::Lit(Box::new(ExprK::Range(None, Some(Box::new(hi)), true))),
                    None => Pat::Rest,
                } }
            Tok::Minus => { self.bump(); let base = match self.bump() { Tok::Int(n) => ExprK::LitInt(n), Tok::Float(fv) => ExprK::LitFloat(fv), _ => ExprK::LitInt(0) };   // negative-literal pattern `-1`/`-2.5` (+ range `-5..=-1`) -> rustc PatKind::Lit(Unary(Neg, Lit))
                self.maybe_range_pat(ExprK::Unary(String::from("Neg"), Box::new(base))) }
            Tok::Int(n) => { self.bump(); self.maybe_range_pat(ExprK::LitInt(n)) }
            Tok::Char(cv) => { self.bump(); self.maybe_range_pat(ExprK::Char(cv)) }   // char-lit pattern `'a'` / range `'0'..='9'`
            Tok::Str(sv) => { self.bump(); Pat::Lit(Box::new(ExprK::Str(sv))) }   // string-lit pattern `"s"` -> rustc PatKind::Lit(Str)
            Tok::Amp => { self.bump(); match self.peek() { Tok::Mut => { self.bump(); }, _ => {} } Pat::Ref(Box::new(self.parse_pat_single())) }   // `&pat` / `&mut pat` -> rustc PatKind::Ref (ref-mut consumed here; the `mut` is reference-mutability, NOT a `mut x` binding mode -- else `&mut S{..}` mis-parses as Bind(mut,S) and leaves `{..}` unconsumed -> stmt desync/hang)
            Tok::AndAnd => { self.toks[self.pos] = Tok::Amp; Pat::Ref(Box::new(self.parse_pat_single())) }   // `&&pat` (e.g. `|&&x|`) -- split the glued AndAnd (R67-style in-place rewrite) into two Ref layers; was falling to `?`/hang
            Tok::Mut => { self.bump(); let nm = self.ident();   // `mut x` binding (+ optional `@ subpat`)
                match self.peek() { Tok::At => { self.bump(); let sub = Box::new(self.parse_pat_single()); Pat::At(format!("mut {}", nm), sub) }, _ => Pat::Bind(String::from("mut"), nm) } }
            _ => { let n0 = self.ident();   // first path segment (kept as a plain String so a bare binding needs no Vec-index clone)
                if n0 == "true" { return Pat::Lit(Box::new(ExprK::Bool(true))); }   // bool-lit pattern -> rustc PatKind::Lit(Bool), not an ident binding
                if n0 == "false" { return Pat::Lit(Box::new(ExprK::Bool(false))); }
                if n0 == "box" { let sub = Box::new(self.parse_pat_single()); return Pat::At(String::from("box"), sub); }   // box pat: consume inner (forward-progress)
                if n0 == "ref" { let mode = match self.peek() { Tok::Mut => { self.bump(); String::from("ref mut") }, _ => String::from("ref") }; let nm = self.ident();   // `ref x` / `ref mut x` binding-mode (+ optional `@ subpat`)
                    match self.peek() { Tok::At => { self.bump(); let sub = Box::new(self.parse_pat_single()); return Pat::At(format!("{} {}", mode, nm), sub); }, _ => { return Pat::Bind(mode, nm); } } }
                match self.peek() {
                    Tok::PathSep => {   // multi-segment path `E::V` (+ optional tuple / struct payload) -- rustc Path / TupleStruct / Struct
                        let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: n0, args: Vec::new(), paren: false });
                        loop { match self.peek() { Tok::PathSep => { self.bump(); segs.push(PathSeg { ident: self.ident(), args: Vec::new(), paren: false }); }, _ => break } }
                        match self.peek() {
                            Tok::LParen => { self.bump(); let mut pats: Vec<Pat> = Vec::new();
                                loop { match self.peek() { Tok::RParen => { self.bump(); break }, _ => { pats.push(self.parse_pat()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }
                                Pat::TupleStruct(Box::new(Path { segments: segs }), pats) }
                            Tok::LBrace => { let fr = self.parse_pat_struct_body(); Pat::Struct(Box::new(Path { segments: segs }), fr.0, fr.1) }
                            _ => { let is_range = match self.peek() { Tok::DotDot => true, Tok::DotDotEq => true, _ => false };   // path low bound of a range pat
                                if is_range { self.maybe_range_pat(ExprK::Path(Box::new(Path { segments: segs }))) } else { Pat::Path(Box::new(Path { segments: segs })) } }
                        } }
                    Tok::LParen => { self.bump(); let mut pats: Vec<Pat> = Vec::new();   // single-segment tuple-struct `E(x)`
                        loop { match self.peek() { Tok::RParen => { self.bump(); break }, _ => { pats.push(self.parse_pat()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }
                        let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: n0, args: Vec::new(), paren: false });
                        Pat::TupleStruct(Box::new(Path { segments: segs }), pats) }
                    Tok::LBrace => { let fr = self.parse_pat_struct_body();   // single-segment struct pattern `S { f, .. }`
                        let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: n0, args: Vec::new(), paren: false });
                        Pat::Struct(Box::new(Path { segments: segs }), fr.0, fr.1) }
                    Tok::At => { self.bump(); let sub = Box::new(self.parse_pat_single()); Pat::At(n0, sub) }   // `x @ subpat` -- rustc PatKind::Ident with a subpattern
                    Tok::Bang => { let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: n0, args: Vec::new(), paren: false }); let mac = self.parse_mac_tail(Path { segments: segs }); Pat::Lit(Box::new(mac)) }   // `foo!(..)` macro in pattern position (rustc PatKind::MacCall) -- consume the macro tail via parse_mac_tail, carry in Pat::Lit (no new Pat variant, R40); was falling to a binding + leaking `!()` -> hang
                    _ => self.ident_or_range_pat(n0)   // `_`/binding, or `X..`/`X..=Y` single-ident path low bound of a range pat (flattened into a helper to avoid a nested-if-in-match-arm R50 miscompile)
                }
            }
        }
    }
    // match := match EXPR { ( pat => EXPR ,? )* }
    fn parse_match_expr(&mut self) -> ExprK {
        self.bump();                         // match
        let sv = self.no_struct; self.no_struct = true;
        let scrut = Box::new(self.parse_expr(0));
        self.no_struct = sv;
        self.bump();                         // LBrace
        let mut arms: Vec<Arm> = Vec::new();
        loop {
            match self.peek() {
                Tok::RBrace => { self.bump(); break }
                _ => { let _ = self.parse_attrs();   // `#[cfg(a)] Pat => ..` outer attr on a match arm -- consume+elide (reduced dump has no arm-attr slot); was leaking `#[..]` into bogus arms
                    let pat = Box::new(self.parse_pat());
                    let mut guard: Option<P<ExprK>> = None;   // arm guard `pat if <cond> =>` (rustc Arm.guard)
                    match self.peek() { Tok::If => { self.bump(); let sv2 = self.no_struct; self.no_struct = true; let svc2 = self.in_cond; self.in_cond = true; guard = Some(Box::new(self.parse_expr(0))); self.in_cond = svc2; self.no_struct = sv2; }, _ => {} }   // a match-arm guard IS a condition context: `if let PAT = e` / `if let a=x && let b=y` (let-chains) are valid here -- set in_cond so parse_expr admits ExprK::Let (else `let` desyncs the arm + swallows following items into the impl)
                    self.bump();             // FatArrow
                    // A `{ .. }` arm body is a COMPLETE arm (rustc: a block-like arm body needs no comma and does NOT
                    // continue into postfix). parse_expr(0) would greedily read `{ .. } (next_pat)` as a call `{block}(..)`,
                    // swallowing the FOLLOWING arm's pattern + guard -> desync. Parse the brace-block directly so the arm ends.
                    let body = match self.peek() { Tok::LBrace => Box::new(ExprK::Block(Box::new(self.parse_block()))), _ => Box::new(self.parse_expr(0)) };
                    match self.peek() { Tok::Comma => { self.bump(); }, _ => {} }
                    arms.push(Arm { pat: pat, guard: guard, body: body }); }
            }
        }
        ExprK::Match(scrut, arms)
    }
    // `const` at statement start is an ITEM (`const X: T = e;` / `const fn`) UNLESS it heads an EXPRESSION:
    // `const { .. }` inline-const block, or `const |..|` / `const move |..|` const closure. Routing those to the
    // expr path (the ConstBlock/Closure atom arms) keeps a bare-tail `const {..}`/`const ||..` from wedging the
    // item parser (peek2-only; no bump, so no forward-progress hazard).
    fn const_leads_item(&self) -> bool {
        match self.peek2() {
            Tok::LBrace => false,   // `const { .. }` block expr
            Tok::Pipe => false,     // `const |a| ..` closure
            Tok::OrOr => false,     // `const || ..` closure
            Tok::Ident(ix) => { let m = self.id_str(ix); if m == "move" { false } else { true } }   // `const move |..|` closure vs `const NAME: T` item
            _ => true,
        }
    }
    // stmt := let PAT : TYPE = EXPR ; | EXPR ; (Semi) | EXPR (Expr, trailing no-semi block value)
    // MAC-8: does the just-parsed stmt's attr set (self.stmt_attrs) carry a vacuously-false `#[cfg(any())]`?
    // Extracted from parse_stmt to keep that fn under the rzrs codegen size envelope (R51); reads a FIELD.
    fn attrs_cfg_drop(&self) -> bool {
        let mut ai = 0;
        let n = self.stmt_attrs.len();
        while ai < n {
            let pn = dump_path(&self.stmt_attrs[ai].path);
            if streq(&pn, "cfg") { let ar = &self.stmt_attrs[ai].args; if streq(ar, "any()") { return true; } }
            ai = ai + 1;
        }
        false
    }
    fn parse_stmt(&mut self) -> Stmt {
        // CRASH-1: macros-2.0 `macro NAME(..) {..}` inside a fn body. Checked in its OWN statement, deliberately
        // NOT as another arm of the starts_item chain below: L6 miscompiles that chain by dropping exactly one
        // arm BY POSITION (append -> `macro` ok/`macro_rules` lost; hoist -> the same swap; fold with `||` of
        // streq CALLS -> both lost, the ||-chain-of-fn-calls class). Measured four ways 2026-08-25; see
        // locales/sem0-crash-class C-L6-ELSEIF-CHAIN-ARM-DROP. parse_item ("MacroDefBrace") already handles
        // the form at top level, so this is routing only.
        let decl_macro_item = match self.peek() {
            Tok::Ident(ix2) => { let w2 = self.id_str(ix2);
                if streq(&w2, "macro") { match self.peek2() { Tok::Ident(_) => true, _ => false } } else { false } }
            _ => false };
        if decl_macro_item { let it = Box::new(self.parse_item()); self.drop_stmt = false; return Stmt::Item(it); }
        // CRASH-3: an in-body `unsafe extern "C" fn` item. starts_item accepts Tok::Unsafe only when peek2 is
        // Fn/Impl/Trait; `extern` lexes as an Ident, so `unsafe extern` fell through to EXPRESSION parsing and
        // SIGSEGVd -- the same routing-gap shape as CRASH-1. Its own statement, NOT a new case in the
        // starts_item chain (C-L6-ELSEIF-CHAIN-ARM-DROP). In-body `unsafe fn` and `extern "C" fn` already
        // worked; only the pair was unrouted. rustc_codegen_llvm/src/llvm_util.rs.
        let unsafe_extern_item = match self.peek() {
            Tok::Unsafe => { match self.peek2() { Tok::Ident(ix3) => { let w3 = self.id_str(ix3); streq(&w3, "extern") }, _ => false } }
            _ => false };
        if unsafe_extern_item { let it = Box::new(self.parse_item()); self.drop_stmt = false; return Stmt::Item(it); }
        let starts_item = match self.peek() {   // a nested ITEM in a block (rustc StmtKind::Item), incl. a leading attr/vis
            Tok::Struct => true, Tok::Enum => true, Tok::Fn => true, Tok::Impl => true,
            Tok::Trait => true, Tok::Const => self.const_leads_item(), Tok::Use => true, Tok::Mod => true,
            Tok::Type => true, Tok::Hash => self.attr_leads_item(), Tok::Pub => true,
            Tok::Unsafe => { match self.peek2() { Tok::Fn => true, Tok::Impl => true, Tok::Trait => true, _ => false } },

            Tok::Ident(ix) => { let w = self.id_str(ix); if streq(&w, "macro_rules") { match self.peek2() { Tok::Bang => true, _ => false } } else if streq(&w, "macro") { match self.peek2() { Tok::Ident(_) => true, _ => false } }   /* CRASH-1: a macros-2.0 `macro NAME(..) {..}` item declared INSIDE a fn body. attr_leads_item already recognised this form; starts_item did not, so a bare in-body decl_macro fell through to EXPRESSION parsing and SIGSEGVd on the matcher `$(`. parse_item ("MacroDefBrace") handles the form correctly at top level -- a ROUTING gap, not a missing mechanism. rustc uses it (rustc_driver_impl print_crate_info, rustc_builtin_macros); crate census 2026-08-25 graded 6/72 crates CRASH. */ else if streq(&w, "static") { match self.peek2() { Tok::Ident(_) => true, Tok::Mut => true, _ => false } } else if streq(&w, "extern") { match self.peek2() { Tok::Str(_) => true, Tok::LBrace => true, _ => false } } else { false } },   // `macro_rules! NAME { .. }` in a fn body IS a nested ITEM (rustc StmtKind::Item / MacroDef); `macro_rules` is a plain Ident so it was NOT detected -> parsed as an expr-stmt macro-call, desyncing the block + swallowing the following items. `static [mut] NAME: T = e;` in a fn body is ALSO a nested ITEM (rustc StmtKind::Item / StaticItem, an owner -> `<fn>::NAME` def-path); `static` is a reserved-word Ident so it needs the same explicit detection (peek2 = NAME or `mut`)
            _ => false };
        if starts_item { let it = Box::new(self.parse_item()); self.drop_stmt = false; return Stmt::Item(it); }
        // MAC-8: capture the stmt's outer attrs; in expand mode a vacuously-false `#[cfg(any())]` DROPS the stmt.
        // dropit is computed from the attrs (owned local Vec, safe), and self.drop_stmt is set as the LAST action
        // (after the body parse, which may recurse into parse_block and clobber the field) so THIS frame's value wins.
        let mut dropit = false;
        match self.peek() { Tok::Hash => { self.stmt_attrs = self.parse_attrs(); if self.expand { dropit = self.attrs_cfg_drop(); } }, _ => {} }
        let st = match self.peek() {
            Tok::Semi => { self.bump(); Stmt::Empty }   // bare `;` -> rustc StmtKind::Empty
            Tok::Let => { self.bump(); let pat = Box::new(self.parse_pat());
                let mut ty: Option<P<Ty>> = None;
                match self.peek() { Tok::Colon => { self.bump(); ty = Some(Box::new(self.parse_ty())); }, _ => {} }
                let mut init: Option<P<ExprK>> = None;   // `let x;` / `let x: T;` -> rustc LocalKind::Decl (no `= init`)
                let mut els: Option<P<Block>> = None;    // `let PAT = e else { .. };` -> rustc LocalKind::InitElse
                match self.peek() {
                    Tok::Eq => { self.bump(); init = Some(Box::new(self.parse_expr(0)));
                        match self.peek() { Tok::Else => { self.bump(); els = Some(Box::new(self.parse_block())); }, _ => {} } }
                    _ => {}
                }
                self.bump();                 // semi
                Stmt::Let(pat, ty, init, els) }
            _ => {
                // A block-like expr (if/match/loop/while/for/unsafe/{}) at STATEMENT position is a COMPLETE statement (rustc):
                // a following `(`/`[`/`.`/`?` starts a NEW statement, it does NOT postfix-apply to the block. Parse just the
                // atom (no postfix/binary) for these -- else `match x {..} (a, b)` mis-parses as a CALL `match(a,b)` (empty
                // callee -> `call  ` -> crash). C-G2D-BLOCK-STMT-CALL: the gen2 crash, since item_keyword_intercept ends
                // `match self.peek() {..} (None, attrs)`. Classification (Semi vs tail Expr) is UNCHANGED, so goldens hold.
                let bl = match self.peek() { Tok::If => true, Tok::Match => true, Tok::Loop => true, Tok::While => true, Tok::For => true, Tok::Unsafe => true, Tok::LBrace => true, _ => false };
                let e = if bl { Box::new(self.parse_atom()) } else { Box::new(self.parse_expr(0)) };
                match self.peek() { Tok::Semi => { self.bump(); Stmt::Semi(e) }, _ => Stmt::Expr(e) } }
        };
        self.drop_stmt = dropit;
        st
    }
    // block := LBRACE stmt* RBRACE
    fn parse_block(&mut self) -> Block {
        let mut stmts: Vec<Stmt> = Vec::new();
        let svb = self.no_struct; self.no_struct = false;
        self.bump();                         // LBrace
        // CT-13: this loop had NO progress guard and NO end-of-token guard. A statement parse that consumes
        // nothing -- reachable on input this tier does not handle, e.g. `#[attr] unsafe { loop { if (*p) .. } }`
        // -- spun here pushing Stmt values forever: 28.5 GB peak RSS and an OOM kill on a 252-line file, with
        // no diagnostic. The two guards below bound the loop unconditionally, so unhandled input costs a
        // malformed parse rather than the machine. This is the class fix; the individual triggers are not.
        loop {
            match self.peek() {
                Tok::RBrace => { self.bump(); break }
                _ => { let before = self.pos; let st = self.parse_stmt(); if !self.drop_stmt { stmts.push(st); } if self.pos == before { self.bump(); } }   // MAC-8: skip a `#[cfg(any())]`-gated stmt in expand mode
            }
            if self.pos >= self.toks.len() { break; }
        }
        self.no_struct = svb;
        Block { stmts: stmts }
    }
    // item := [pub] ( const IDENT : TYPE = EXPR ; | struct IDENT struct-fields | enum IDENT enum-variants | fn IDENT fn-params ( -> TYPE )? block )
    // generics := ( < IDENT ( : path (+ path)* )? (, ...)* > )?
    fn parse_generics(&mut self) -> Vec<GParam> {
        let mut params: Vec<GParam> = Vec::new();
        match self.peek() {
            Tok::Lt => { self.bump();
                loop { if self.eat_close_gt() { break }   // `>` or split `>>` closes the generic-param list (e.g. `<T: Foo<Bar>>`)
                    let _ = self.parse_attrs();   // attr on a generic param `<#[may_dangle] T>` -- consume+elide the leading `#[..]` so one attributed param stays one param (was shredded into `<?, ?, may_dangle, ?, T>`)
                    match self.peek() {
                    Tok::Gt => { self.bump(); break }
                    Tok::Lifetime(nm) => { self.bump();   // lifetime param `'a` (with optional `'a: 'b` outlives bound, captured into `bounds` as tick-carrying single-seg Paths)
                        let mut bounds: Vec<Path> = Vec::new();
                        match self.peek() { Tok::Colon => { self.bump(); loop { match self.peek() { Tok::Lifetime(l) => { self.bump(); let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: format!("'{}", l), args: Vec::new(), paren: false }); bounds.push(Path { segments: segs }); }, _ => break } match self.peek() { Tok::Plus => { self.bump(); }, _ => break } } }, _ => {} }
                        params.push(GParam { name: nm, bounds: bounds, is_lt: true, is_const: false, deflt: String::from("") });
                        match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } }
                    Tok::Const => { self.bump(); let name = self.ident(); self.bump();   // `const N: T` const generic param (bump the colon)
                        let mut bounds: Vec<Path> = Vec::new(); bounds.push(self.parse_bound_path());
                        // optional `= <const-default>` (rustc AnonConst): CAPTURE the default tokens (nesting-aware) into deflt so it
                        // renders `= 8` like a type-param default, instead of silently dropping it. tok_str(&self.peek(), &self.idents) is L6-proven.
                        let mut cdeflt = String::new();
                        match self.peek() { Tok::Eq => { self.bump(); cdeflt = self.capture_const_default(); }, _ => {} }
                        params.push(GParam { name: name, bounds: bounds, is_lt: false, is_const: true, deflt: cdeflt });
                        match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } }
                    _ => { let name = self.ident(); let mut bounds: Vec<Path> = Vec::new();
                        match self.peek() { Tok::Colon => { self.bump();
                            loop { match self.peek() { Tok::Lifetime(l) => { self.bump(); let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: format!("'{}", l), args: Vec::new(), paren: false }); bounds.push(Path { segments: segs }); }, _ => { bounds.push(self.parse_bound_path()); } } match self.peek() { Tok::Plus => { self.bump(); }, _ => break } } }, _ => {} }
                        let mut deflt = String::new();   // `<T = u8>` default type param (rustc GenericParam kind Type{default})
                        match self.peek() { Tok::Eq => { self.bump(); let t = self.parse_ty(); deflt = match &t { Ty::Path(p) => dump_path(p), _ => dump_ty(&t) }; }, _ => {} }
                        params.push(GParam { name: name, bounds: bounds, is_lt: false, is_const: false, deflt: deflt });
                        match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } }
                } } }
            _ => {}
        }
        params
    }
    // attrs := ( # [ path ( ( ident (, ident)* ) )? ] )*   (outer attributes)
    fn parse_attrs(&mut self) -> Vec<Attr> {
        let mut attrs: Vec<Attr> = Vec::new();
        loop {
            match self.peek() {
                Tok::Hash => { self.bump();   // #
                    let is_inner = match self.peek() { Tok::Bang => { self.bump(); true }, _ => false };   // inner attr `#![..]` (rustc style Inner): consume the `!`; elided below so it does not mis-attach to the next item as an outer attr
                    self.bump();               // [
                    let svep = self.expr_path; self.expr_path = true;   // attr path stops before `(`; parse_attrs owns the arg list (not a paren Fn-trait seg)
                    let path = Box::new(self.parse_path());
                    self.expr_path = svep;
                    let mut args = String::new();
                    match self.peek() {
                        Tok::LParen => { self.bump();          // list form -- capture a BALANCED delimiter group as raw tokens (nested `#[repr(align(8))]`/`#[cfg_attr(..)]` supported)
                            let mut depth = 0;
                            loop {
                                let closing = match self.peek() { Tok::RParen => true, Tok::RBrack => true, Tok::RBrace => true, _ => false };
                                if depth == 0 { if closing { self.bump(); break; } }
                                match self.peek() {
                                    Tok::LParen => { args = format!("{}(", args); depth = depth + 1; self.bump(); }
                                    Tok::LBrack => { args = format!("{}[", args); depth = depth + 1; self.bump(); }
                                    Tok::LBrace => { args = format!("{}{{", args); depth = depth + 1; self.bump(); }
                                    Tok::RParen => { args = format!("{})", args); depth = depth - 1; self.bump(); }
                                    Tok::RBrack => { args = format!("{}]", args); depth = depth - 1; self.bump(); }
                                    Tok::RBrace => { args = format!("{}}}", args); depth = depth - 1; self.bump(); }
                                    Tok::Comma => { self.bump(); args = format!("{}, ", args); }
                                    Tok::Eq => { self.bump(); args = format!("{} = ", args); }
                                    _ => { let s = tok_str(&self.peek(), &self.idents); self.bump(); args = format!("{}{}", args, s); }
                                }
                            } }
                        Tok::Eq => { self.bump(); args = String::from("= ");   // name-value form `#[doc = "x"]` -- the following item is NOT swallowed
                            loop { match self.peek() { Tok::RBrack => break, _ => {} } let s = tok_str(&self.peek(), &self.idents); self.bump(); args = format!("{}{}", args, s); } }
                        _ => {}
                    }
                    self.bump();               // ]
                    if is_inner == false { attrs.push(Attr { path: path, args: args }); } }   // outer attrs attach to the item; inner (crate/mod-level) attrs are consumed + elided
                _ => break
            }
        }
        attrs
    }
    // where-clause := ( where ( Ty ( : bound (+ bound)* )? ),* )?  -- parsed just before the item body; empty if no `where`
    fn parse_where(&mut self) -> Vec<WherePred> {
        let mut preds: Vec<WherePred> = Vec::new();
        match self.peek() {
            Tok::Where => { self.bump();
                loop {
                    match self.peek() { Tok::LBrace => break, Tok::Semi => break, Tok::Eq => break, _ => {} }
                    let _ = self.parse_attrs();   // `where #[cfg(a)] T: Clone` -- consume+elide an outer attr on a where-predicate (reduced dump has no pred-attr slot); was leaking `#[..]` into bogus items
                    let ty = Box::new(self.parse_ty());
                    let mut bounds: Vec<Path> = Vec::new();
                    match self.peek() { Tok::Colon => { self.bump();
                        loop { match self.peek() { Tok::Lifetime(l) => { self.bump(); let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: format!("'{}", l), args: Vec::new(), paren: false }); bounds.push(Path { segments: segs }); }, _ => { bounds.push(self.parse_bound_path()); } } match self.peek() { Tok::Plus => { self.bump(); }, _ => break } } }, _ => {} }
                    preds.push(WherePred { ty: ty, bounds: bounds });
                    match self.peek() { Tok::Comma => { self.bump(); }, _ => break }
                } }
            _ => {}
        }
        preds
    }
    // vis := ( pub [ ( crate | super | self | in path ) ] )?   -- AST-6: restricted visibility (rustc Visibility)
    fn parse_vis(&mut self) -> String {
        match self.peek() {
            Tok::Pub => { self.bump();
                match self.peek() {
                    Tok::LParen => { self.bump(); let mut inner = String::new();
                        loop { match self.peek() {
                            Tok::RParen => { self.bump(); break }
                            Tok::Ident(ix) => { let nm = self.id_str(ix); self.bump(); inner = format!("{}{}", inner, nm); }
                            Tok::In => { self.bump(); inner = format!("{}in ", inner); }
                            Tok::PathSep => { self.bump(); inner = format!("{}::", inner); }
                            _ => { self.bump(); }
                        } }
                        format!("pub({})", inner) }
                    _ => String::from("pub")
                } }
            _ => String::from("priv")
        }
    }
    // Parse a `fn` item body from (and including) the `fn` keyword. Shared by a plain `fn` and a `const fn`
    // (constness is elided in the reduced canonical dump, like the other elided Fn modifiers). Assumes peek()==`fn`.
    fn parse_fn_item(&mut self, lo: i64, attrs: Vec<Attr>, vis: String) -> Item {
        self.bump();                         // `fn`
        let is_async = self.pend_async; self.pend_async = false;   // S1-amendment: capture+clear the pending `async` BEFORE the body parses (so a nested fn does not inherit it)
        let ident = self.ident(); let gp = self.parse_generics();
        let params = self.parse_fn_params();
        let ret = match self.peek() { Tok::Arrow => { self.bump(); Some(Box::new(self.parse_ty())) }, _ => None };
        let wheres = self.parse_where();
        let body = match self.peek() { Tok::LBrace => Some(Box::new(self.parse_block())), _ => { self.bump(); None } };
        Item { vis: vis, ident: ident, attrs: attrs, wheres: wheres, bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::Fn(gp, params, ret, body, is_async) }
    }
    fn parse_item(&mut self) -> Item {
        let lo = self.cur_off();             // AST-9: item span lo (includes leading attrs, like rustc)
        let attrs = self.parse_attrs();
        let vis = self.parse_vis();
        match self.peek() { Tok::Ident(ix) => { let w = self.id_str(ix); if w == "macro_rules" { match self.peek2() { Tok::Bang => {   // `macro_rules ! NAME { rules }` DEFINITION -> rustc ItemKind::MacroDef. Reuse ItemKind::MacCall carrier (no enum growth): path=NAME, sentinel delim "MacroDefBrace", raw rule tokens.
            self.bump(); self.bump();        // `macro_rules` `!`
            let name = self.ident();         // NAME (item ident, like rustc)
            let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: name.clone(), args: Vec::new(), paren: false });
            let toks = self.capture_group();
            // MAC-1: register every arm of the def for later call-site dispatch (the def itself still dumps as
            // MacCall, matching rustc's retained macro_rules). Stash the raw rule tokens in a field (a &Vec param
            // is not a lowered subset access shape) and let mac_register_arms walk them into one row per arm.
            self.rule_toks = veccopy(&toks);
            self.mac_register_arms(name.clone());
            let mac = ExprK::MacCall(Box::new(Path { segments: segs }), String::from("MacroDefBrace"), toks);
            return Item { vis: vis, ident: String::from(""), attrs: attrs, wheres: Vec::new(), bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::MacCall(Box::new(mac)) };
        }, _ => {} } } }, _ => {} }
        // `macro NAME(params) { body }` / `macro NAME { rules }` -- decl_macro 2.0 DEFINITION -> rustc ItemKind::MacroDef,
        // one owner `NAME`. `macro` is a reserved-word Ident so `macro` + Ident is unambiguous. WITHOUT this, `macro m()`
        // falls through to the expr/`?` arms -> 2 garbage `?` owners + swallows the following item (a mini-desync). Reuse
        // the macro_rules MacroDef carrier (path=NAME, delim "MacroDefBrace") so emit_crate_defpath's MacCall arm emits NAME.
        // Consume the optional `(params)` then the `{ body }` (both balanced) so the parser resyncs. NOT registered for S2
        // expansion (the engine targets macro_rules; decl_macro is a separate surface).
        match self.peek() { Tok::Ident(ix) => { let w = self.id_str(ix); if streq(&w, "macro") { match self.peek2() { Tok::Ident(_) => {
            self.bump();                     // `macro`
            let name = self.ident();         // NAME
            let mut segs: Vec<PathSeg> = Vec::new(); segs.push(PathSeg { ident: name.clone(), args: Vec::new(), paren: false });
            match self.peek() { Tok::LParen => { let _ = self.capture_group(); }, _ => {} }   // optional (params)
            let toks = match self.peek() { Tok::LBrace => self.capture_group(), _ => Vec::new() };   // { body / rules }
            let mac = ExprK::MacCall(Box::new(Path { segments: segs }), String::from("MacroDefBrace"), toks);
            return Item { vis: vis, ident: String::from(""), attrs: attrs, wheres: Vec::new(), bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::MacCall(Box::new(mac)) };
        }, _ => {} } } }, _ => {} }
        if self.peek_item_macro() {   // item-position macro call `path! { .. }` (thread_local!/bitflags!/foo::bar!) -> rustc ItemKind::MacCall
            let p = self.parse_path();   // (possibly multi-segment) macro path; parse_mac_tail then captures the delimited body opaquely
            let mac = self.parse_mac_tail(p);
            match self.peek() { Tok::Semi => { self.bump(); }, _ => {} }   // optional trailing `;` on a paren/bracket item macro
            return Item { vis: vis, ident: String::from(""), attrs: attrs, wheres: Vec::new(), bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::MacCall(Box::new(mac)) };
        }
        match self.peek() { Tok::Ident(ix) => { let w = self.id_str(ix); if w == "async" { self.bump(); self.pend_async = true; } }, _ => {} }   // leading `async` on fn -> set pend_async (S1-amendment; captured+cleared in parse_fn_item)
        match self.peek() { Tok::Unsafe => { self.bump(); }, _ => {} }   // leading `unsafe` on impl/trait/fn -> dispatch normally (unsafety elided, like the other reduced-dump modifiers)
        match self.peek() { Tok::Ident(ix) => { let w = self.id_str(ix); if w == "default" { match self.peek2() { Tok::Impl => { self.bump(); }, _ => {} } } }, _ => {} }   // `default impl ..` -> consume+elide `default` so the main match dispatches to impl parsing (defaultness elided, like unsafe/async). WITHOUT this, `default` falls to the const `_` arm and parse_ty hits the `for` of `impl Tr for S` as a mis-detected HRTB -> unbounded parse_ty_inner recursion (stack overflow). `default fn` stays on the const arm (peek2 != Impl).
        let (hit, attrs) = self.item_keyword_intercept(lo, attrs, vis.clone());   // `auto trait`/`extern crate`/`extern "ABI" {}` -- extracted HELPER (R51: keeps parse_item under the rzrs codegen envelope; the inline form miscompiled cursor.rs)
        match hit { Some(it) => return it, _ => {} }
        match self.peek() {
            Tok::Struct => { self.bump(); let ident = self.ident(); let gp = self.parse_generics();
                let wheres = self.parse_where();
                match self.peek() {
                    Tok::LParen => { self.bump(); let mut tys: Vec<Ty> = Vec::new();   // tuple struct `S(T, U);`
                        loop { match self.peek() { Tok::RParen => { self.bump(); break }, _ => { let _ = self.parse_attrs(); self.parse_vis(); tys.push(self.parse_ty()); match self.peek() { Tok::Comma => { self.bump(); }, _ => {} } } } }
                        self.bump();   // `;`
                        Item { vis: vis.clone(), ident: ident, attrs: attrs, wheres: wheres, bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::TupleStruct(gp, tys) } }
                    Tok::Semi => { self.bump();   // unit struct `S;`
                        Item { vis: vis.clone(), ident: ident, attrs: attrs, wheres: wheres, bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::UnitStruct(gp) } }
                    _ => { let fields = self.parse_struct_fields();   // named struct `S { .. }`
                        Item { vis: vis.clone(), ident: ident, attrs: attrs, wheres: wheres, bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::Struct(gp, fields) } }
                } }
            Tok::Enum => { self.bump(); let ident = self.ident(); let gp = self.parse_generics();
                let wheres = self.parse_where();
                let variants = self.parse_enum_variants();
                Item { vis: vis.clone(), ident: ident, attrs: attrs, wheres: wheres, bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::Enum(gp, variants) } }
            Tok::Fn => self.parse_fn_item(lo, attrs, vis.clone()),
            Tok::Type => { self.bump(); let ident = self.ident(); let gp = self.parse_generics();
                // optional `: bound + bound` (assoc-type / trait-item bounds; read-only reuse of parse_bounds)
                let bounds = match self.peek() { Tok::Colon => { self.bump(); self.parse_bounds() }, _ => Vec::new() };
                let leadw = self.parse_where();   // leading where (rare on a type alias; before `=`)
                // optional `= ty` -- bare `type Item;` / bounded `type Out: Clone;` have NO `=` (rustc AssocItem TyAlias, ty None)
                let opt_ty = match self.peek() { Tok::Eq => { self.bump(); Some(Box::new(self.parse_ty())) }, _ => None };
                let trailw = self.parse_where();   // trailing where AFTER `= ty` (rustc TyAlias after_where_clause): `type A = u8 where T: Copy;` -- was bumped as if the semi, leaking `T: Copy;` as a bogus Const item
                let wheres = if leadw.len() > 0 { leadw } else { trailw };   // immutable select (a type alias has where in one position); no reassign/move-loop -> L6-safe
                self.bump();                 // semi
                let kind = match opt_ty { Some(ty) => ItemKind::TypeAlias(gp, ty), None => ItemKind::AssocTyDecl(gp) };
                Item { vis: vis.clone(), ident: ident, attrs: attrs, wheres: wheres, bounds: bounds, lo: lo, hi: self.cur_off(), kind: kind } }
            Tok::Use => { self.bump();
                let tree = Box::new(self.parse_use_tree());
                self.bump();                 // semi
                Item { vis: vis.clone(), ident: String::from(""), attrs: attrs, wheres: Vec::new(), bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::Use(tree) } }
            Tok::Mod => { self.bump(); let ident = self.ident();
                let mut items: Vec<Item> = Vec::new();
                match self.peek() {
                    Tok::LBrace => { self.bump();
                        loop { match self.peek() { Tok::RBrace => { self.bump(); break }, Tok::End => break, _ => { let bp = self.pos; items.push(self.parse_item()); if self.pos == bp { break } } } } }
                    _ => { self.bump();    // `mod NAME;` external module declaration
                        // S3 CRATE DRIVER (in-parser): in resolve mode, READ the sibling file NAME.rs (relative to the
                        // process CWD -- the crate root's directory) via the portable std::fs::read_to_string that the
                        // stage0 fs-read idiom lowers, lex + parse its items as the module body. Gated on self.resolve
                        // so S1/S2 (which do not read files) keep the empty-mod behavior. Retires the external driver.
                        if self.resolve {
                            let path = format!("{}{}.rs", self.moddir, ident);   // moddir prefix -> NAME's submodules resolve under NAME/ (a/b.rs). format! is a fresh String (ident.clone() aliases in rzrs).
                            let body = std::fs::read_to_string(&path).unwrap();
                            let (bt, bo, idn) = lex(&body);
                            let mut sub = Parser { idents: idn, toks: bt, offs: bo, pos: 0, no_struct: false, expr_path: false, pending_gt: 0, in_cond: false, expand: self.expand, mac_names: Vec::new(), mac_matchers: Vec::new(), mac_bodies: Vec::new(), mac_metavars: Vec::new(), mac_reps: Vec::new(), mac_depth: 0, rule_toks: Vec::new(), rep_els: Vec::new(), drop_stmt: false, stmt_attrs: Vec::new(), resolve: self.resolve, moddir: format!("{}{}/", self.moddir, ident), pend_move: false, pend_async: false };
                            loop { match sub.peek() { Tok::End => break, _ => { let bp = sub.pos; items.push(sub.parse_item()); if sub.pos == bp { break } } } }
                        }
                    }
                }
                Item { vis: vis.clone(), ident: ident, attrs: attrs, wheres: Vec::new(), bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::Mod(items) } }
            Tok::Impl => { self.bump(); let gp = self.parse_generics();
                let pol = match self.peek() { Tok::Bang => { self.bump(); String::from("!") }, _ => String::from("") };   // `impl !Send for A {}` negative-impl polarity; `!` marker stored in ident, "" positive
                let t1 = Box::new(self.parse_ty());
                let mut trait_ref: Option<P<Ty>> = None;
                let self_ty = match self.peek() { Tok::For => { self.bump(); trait_ref = Some(t1); Box::new(self.parse_ty()) }, _ => t1 };   // `impl Tr for S {..}` -> t1 was the trait ref; `impl S {..}` -> t1 is the self type
                let wheres = self.parse_where();
                let mut items: Vec<Item> = Vec::new();
                self.bump();                 // LBrace
                loop { match self.peek() { Tok::RBrace => { self.bump(); break }, Tok::End => break, _ => { let bp = self.pos; items.push(self.parse_item()); if self.pos == bp { break } } } }
                Item { vis: vis.clone(), ident: pol, attrs: attrs, wheres: wheres, bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::Impl(gp, trait_ref, self_ty, items) } }
            Tok::Trait => { self.bump(); let ident = self.ident(); let gp = self.parse_generics();
                // trait ALIAS `trait Foo = Bar + Baz;` / `trait Foo<T> = Bar<T> where T: Clone;` -> TraitAlias (rustc ItemKind::TraitAlias); the `=` RHS is a bound list, NOT a brace body. Without this the `=` falls into brace-body parsing and each bound mis-parses as a bogus const item.
                match self.peek() { Tok::Eq => { self.bump(); let ab = self.parse_bounds(); let aw = self.parse_where(); self.bump();   // consume `;`
                        return Item { vis: vis.clone(), ident: ident, attrs: attrs, wheres: aw, bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::TraitAlias(gp, ab) }; }, _ => {} }
                // optional supertrait `: B + C` after the trait header (rustc Trait bounds)
                let bounds = match self.peek() { Tok::Colon => { self.bump(); self.parse_bounds() }, _ => Vec::new() };
                let wheres = self.parse_where();
                let mut items: Vec<Item> = Vec::new();
                self.bump();                 // LBrace
                loop { match self.peek() { Tok::RBrace => { self.bump(); break }, Tok::End => break, _ => { let bp = self.pos; items.push(self.parse_item()); if self.pos == bp { break } } } }
                Item { vis: vis.clone(), ident: ident, attrs: attrs, wheres: wheres, bounds: bounds, lo: lo, hi: self.cur_off(), kind: ItemKind::Trait(gp, items) } }
            _ => { self.bump();              // const
                match self.peek() { Tok::Unsafe => { self.bump(); }, _ => {} }   // `const unsafe fn` -- elide `unsafe` (constness + unsafety both elided); `const unsafe` only ever qualifies a fn
                match self.peek() { Tok::Ident(ix) => { let w = self.id_str(ix); if w == "extern" { self.bump(); match self.peek() { Tok::Str(_) => { self.bump(); }, _ => {} } } }, _ => {} }   // `const [unsafe] extern "C" fn` -- elide `extern` + optional ABI string
                match self.peek() {
                    Tok::Fn => self.parse_fn_item(lo, attrs, vis.clone()),   // `const [unsafe] [extern] fn` -- constness/unsafety/abi elided in the reduced dump
                    _ => { let ident = self.ident();
                        self.bump();                 // colon
                        let ty = Box::new(self.parse_ty());
                        let mut rhs: Option<P<ExprK>> = None;   // trait assoc const `const Z: usize;` has no `= expr`; impl/free const does
                        match self.peek() { Tok::Eq => { self.bump(); rhs = Some(Box::new(self.parse_expr(0))); }, _ => {} }
                        self.bump();                 // semi
                        Item { vis: vis.clone(), ident: ident, attrs: attrs, wheres: Vec::new(), bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::Const(ty, rhs) } }
                } }
        }
    }
    // Small keyword-intercept helper (extracted from parse_item to keep it under the rzrs codegen envelope; R51 -- the inline form miscompiled cursor.rs).
    fn item_keyword_intercept(&mut self, lo: i64, attrs: Vec<Attr>, vis: String) -> (Option<Item>, Vec<Attr>) {
        match self.peek() { Tok::Ident(ix) => { let w = self.id_str(ix);
            if w == "auto" { self.bump(); self.bump(); return (Some(self.parse_auto_trait_item(lo, attrs, vis)), Vec::new()); }   // `auto trait Foo {}` -> AutoTrait; consume `auto` `trait`
            if w == "extern" { return self.parse_extern_dispatch(lo, attrs, vis); }   // extracted to a helper to keep this fn small (R51 fn-size hazard)
            if w == "static" { return (Some(self.parse_static_item(lo, attrs, vis)), Vec::new()); }   // `static [mut] X: T = e;` -> ItemKind::Static (helper keeps this fn small, R51)
            if w == "union" { return (Some(self.parse_union_item(lo, attrs, vis)), Vec::new()); }   // `union U { .. }` -> ItemKind::Union (contextual keyword; helper keeps this fn small, R51)
        }, _ => {} }
        (None, attrs)
    }
    // `extern crate NAME;` -> ExternCrate; `extern "ABI" { .. }` -> ForeignMod; `extern "ABI" fn ..` -> a normal Fn ITEM (extern header
    // elided) -- consume `extern`+ABI then fall through so the main dispatch parses the `fn`. Its own small helper per R51 (cursor.rs canary).
    fn parse_extern_dispatch(&mut self, lo: i64, attrs: Vec<Attr>, vis: String) -> (Option<Item>, Vec<Attr>) {
        match self.peek2() {
            Tok::Ident(ix) => { let c = self.id_str(ix); if c == "crate" { self.bump(); self.bump(); return (Some(self.parse_extern_crate_item(lo, attrs, vis)), Vec::new()); } },
            Tok::Str(_) => { self.bump(); self.bump();   // consume `extern` + "ABI" (elided)
                match self.peek() { Tok::LBrace => { return (Some(self.parse_foreign_mod_item(lo, attrs, vis)), Vec::new()); }, _ => { return (None, attrs); } } },
            // CT-21: the ABI string is OPTIONAL. A bare `extern { type VaListImpl; }` had no arm here, so the
            // `extern` was never recognised as an item head, its block was parsed as expression text, and the
            // item loop recovered by inventing a `Const` it could not name -- the `?`-ident phantom class.
            // 7 items across 3 files, and the residue of C-S1-PHANTOM-CONST-ITEM after CT-19 closed its qself
            // cause. Only `extern` is consumed; there is no ABI token to elide.
            Tok::LBrace => { self.bump(); return (Some(self.parse_foreign_mod_item(lo, attrs, vis)), Vec::new()); },
            _ => {} }
        (None, attrs)
    }
    fn parse_auto_trait_item(&mut self, lo: i64, attrs: Vec<Attr>, vis: String) -> Item {   // `auto trait NAME [: bounds] { .. }` -> AutoTrait (mirrors the Trait arm)
        let ident = self.ident(); let gp = self.parse_generics();
        let bounds = match self.peek() { Tok::Colon => { self.bump(); self.parse_bounds() }, _ => Vec::new() };
        let wheres = self.parse_where();
        let mut items: Vec<Item> = Vec::new();
        self.bump();                 // LBrace
        loop { match self.peek() { Tok::RBrace => { self.bump(); break }, Tok::End => break, _ => { let bp = self.pos; items.push(self.parse_item()); if self.pos == bp { break } } } }
        Item { vis: vis, ident: ident, attrs: attrs, wheres: wheres, bounds: bounds, lo: lo, hi: self.cur_off(), kind: ItemKind::AutoTrait(gp, items) }
    }
    fn parse_union_item(&mut self, lo: i64, attrs: Vec<Attr>, vis: String) -> Item {   // `union NAME<gp> { fields }` -> ItemKind::Union (rustc; a contextual keyword, `union` is an Ident). Body reuses parse_struct_fields.
        self.bump();                 // `union`
        let ident = self.ident(); let gp = self.parse_generics();
        let wheres = self.parse_where();
        let fields = self.parse_struct_fields();
        Item { vis: vis, ident: ident, attrs: attrs, wheres: wheres, bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::Union(gp, fields) }
    }
    fn parse_static_item(&mut self, lo: i64, attrs: Vec<Attr>, vis: String) -> Item {   // `static [mut] NAME: TY = EXPR;` -> ItemKind::Static (rustc StaticItem; mutability elided). `static` is an Ident, not a keyword token.
        self.bump();                 // `static`
        match self.peek() { Tok::Mut => { self.bump(); }, _ => {} }   // `static mut` -- mutability elided in the reduced dump
        let ident = self.ident();
        self.bump();                 // colon
        let ty = Box::new(self.parse_ty());
        let mut rhs: Option<P<ExprK>> = None;
        match self.peek() { Tok::Eq => { self.bump(); rhs = Some(Box::new(self.parse_expr(0))); }, _ => {} }   // `= EXPR` (a `static X: T;` in a foreign block has none)
        self.bump();                 // semi
        Item { vis: vis, ident: ident, attrs: attrs, wheres: Vec::new(), bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::Static(ty, rhs) }
    }
    fn parse_extern_crate_item(&mut self, lo: i64, attrs: Vec<Attr>, vis: String) -> Item {   // `extern crate NAME [as RENAME];` -> ExternCrate (name in ident; rename elided)
        let name = self.ident();
        let mut ident = String::new(); ident.push_str(&name);   // the BINDING name: the `as RENAME` alias when present, else the crate name. rustc's ExternCrate def-path is the alias (`extern crate self as rustc_type_ir` -> owner `rustc_type_ir`, not `self`); `name` stays the original for the resolve-mode dep file read.
        match self.peek() { Tok::As => { self.bump(); ident = self.ident(); }, _ => {} }
        self.bump();                 // semi
        // S3 CROSS-CRATE (f): in resolve mode, read the dependency crate's root <moddir>NAME.rs and embed its items as a
        // synthetic module named NAME with vis "extern" -- so `use NAME::PATH` resolves into the dep crate (check_from
        // enforces cross-crate privacy: only pub items are visible), while emit_item_defpath skips it (the dep's defs
        // are in the dep crate's DefId space, not this crate's def-paths). S1/S2 (resolve off) keep plain ExternCrate.
        if self.resolve {
            let path = format!("{}{}.rs", self.moddir, name);
            let body = std::fs::read_to_string(&path).unwrap();
            let (bt, bo, idn) = lex(&body);
            let mut sub = Parser { idents: idn, toks: bt, offs: bo, pos: 0, no_struct: false, expr_path: false, pending_gt: 0, in_cond: false, expand: self.expand, mac_names: Vec::new(), mac_matchers: Vec::new(), mac_bodies: Vec::new(), mac_metavars: Vec::new(), mac_reps: Vec::new(), mac_depth: 0, rule_toks: Vec::new(), rep_els: Vec::new(), drop_stmt: false, stmt_attrs: Vec::new(), resolve: self.resolve, moddir: format!("{}{}/", self.moddir, name), pend_move: false, pend_async: false };
            let mut items: Vec<Item> = Vec::new();
            loop { match sub.peek() { Tok::End => break, _ => { let bp = sub.pos; items.push(sub.parse_item()); if sub.pos == bp { break } } } }
            return Item { vis: String::from("extern"), ident: ident, attrs: attrs, wheres: Vec::new(), bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::Mod(items) };
        }
        Item { vis: vis, ident: ident, attrs: attrs, wheres: Vec::new(), bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::ExternCrate }
    }
    fn parse_foreign_mod_item(&mut self, lo: i64, attrs: Vec<Attr>, vis: String) -> Item {   // `extern "ABI" { <foreign items> }` -> ForeignMod; caller (parse_extern_dispatch) already consumed `extern` + ABI, peek()==LBrace
        let mut items: Vec<Item> = Vec::new();
        self.bump();                 // LBrace
        loop { match self.peek() { Tok::RBrace => { self.bump(); break }, Tok::End => break, _ => { let bp = self.pos; items.push(self.parse_item()); if self.pos == bp { break } } } }
        Item { vis: vis, ident: String::from(""), attrs: attrs, wheres: Vec::new(), bounds: Vec::new(), lo: lo, hi: self.cur_off(), kind: ItemKind::ForeignMod(items) }
    }
    fn parse_krate(&mut self) -> Krate {
        let mut items: Vec<Item> = Vec::new();
        loop { match self.peek() { Tok::End => break, _ => {
            // CT-15: the parser is TOTAL, so a file that is an `include!` FRAGMENT rather than a crate -- a bare
            // top-level `match`, which rustc rejects with "expected item, found keyword `match`" -- was parsed
            // into invented items (7,624 of them in codegen_gcc/src/intrinsic_archs.rs) and reported rc=0 with
            // zero UNLOWERED. Three such files were being COUNTED CLEAN by the tree metric. The list is
            // deliberately only tokens that can NEVER begin an item, so no valid crate can be refused by it.
            let bad = match self.peek() { Tok::Match => true, Tok::If => true, Tok::While => true, Tok::Loop => true, Tok::For => true, Tok::Let => true, _ => false };
            if bad { eprintln!("error: expected item at top level -- this file is not a crate (an include! fragment?)"); std::process::exit(1); }
            // CT-22: `#![..]` is a CRATE attribute, not an item. A crate whose entire content is inner
            // attributes -- `codegen_gcc/build_system/build_sysroot/lib.rs` is 11 bytes of `#![no_std]` --
            // reached parse_item with nothing to parse, and the loop recovered by inventing an `Item Const` it
            // could not name. Consumed here so the loop reaches End and stops.
            // Narrowed to attributes that run to END OF FILE. Consuming every crate-level inner attr changed
            // one S1 witness span (`Item Fn@0..36` -> `@26..36`, the attrs no longer folded into the item) and
            // failed FIDELITY (witnesses) against the golden. That span change is arguably more faithful, but
            // it is not what this rung set out to do, and editing the golden that guards S1 to make a fix pass
            // is the wrong direction. This form touches only the case with no item to attach to.
            if self.attrs_run_to_end() { break; }
            let it = self.parse_item();
            // MAC-8 attribute cfg-gating: in expand mode, an item carrying a vacuously-false `#[cfg(any())]` is
            // DROPPED from the crate (== rustc `-Zunpretty=expanded`). Only the host-independent `any()` drops here;
            // `all()`/host-dependent predicates keep the item. Gated on self.expand so the S1 unexpanded goldens hold.
            let mut drop = false;
            if self.expand {
                let mut ai = 0;
                while ai < it.attrs.len() {
                    let pn = dump_path(&it.attrs[ai].path);
                    if streq(&pn, "cfg") { let ar = &it.attrs[ai].args; if streq(ar, "any()") { drop = true; } }
                    ai = ai + 1;
                }
            }
            if !drop { items.push(it); }
        } } }
        Krate { items: items }
    }
}

fn vis_str(is_pub: bool) -> String { if is_pub { String::from("pub") } else { String::from("priv") } }

fn dump_seg(seg: &PathSeg) -> String {
    let mut s = format!("{}", seg.ident);
    if seg.paren {   // `Fn(A, B) -> R`: inputs are the plain args, the return rides as an Assoc("Output", R)
        let mut ret = String::new(); let mut ins = String::new(); let mut first = true;
        for a in seg.args.iter() {
            match a { Ty::Assoc(_nm, ty) => { ret = format!(" -> {}", dump_ty(ty)); }, _ => { if first { ins = dump_ty(a); first = false; } else { ins = format!("{}, {}", ins, dump_ty(a)); } } }
        }
        if seg.args.len() > 0 { s = format!("{}({}){}", s, ins, ret); } else { s = format!("{}(){}", s, ret); }
    } else if seg.args.len() > 0 {
        s = format!("{}<", s); let mut first = true;
        for a in seg.args.iter() { if first { s = format!("{}{}", s, dump_ty(a)); first = false; } else { s = format!("{}, {}", s, dump_ty(a)); } }
        s = format!("{}>", s);
    }
    s
}
// Flat raw-token rendering for a macro tokenstream (pre-expansion; not re-parsed). Covers the common tokens; the exact
// rendering is a deterministic reduced projection (the golden is self-referential), the point is a MacCall carrying its raw tokens.
fn tok_str(t: &Tok, idn: &Vec<String>) -> String {
    match t {
        Tok::Ident(ix) => id_str_raw(idn, *ix), Tok::Int(n) => format!("{}{}", lit_value(*n), suffix_name(lit_suffix(*n))), Tok::Float(s) => format!("{}", s), Tok::Str(s) => format!("\"{}\"", s), Tok::Char(c) => format!("'{}'", c), Tok::Lifetime(l) => format!("'{}", l),
        Tok::Comma => String::from(","), Tok::Semi => String::from(";"), Tok::Colon => String::from(":"), Tok::PathSep => String::from("::"), Tok::Dot => String::from("."),
        Tok::Eq => String::from("="), Tok::EqEq => String::from("=="), Tok::Ne => String::from("!="), Tok::FatArrow => String::from("=>"), Tok::Arrow => String::from("->"),
        Tok::Plus => String::from("+"), Tok::Minus => String::from("-"), Tok::Star => String::from("*"), Tok::Slash => String::from("/"), Tok::Percent => String::from("%"),
        Tok::Amp => String::from("&"), Tok::Pipe => String::from("|"), Tok::Caret => String::from("^"), Tok::Bang => String::from("!"), Tok::Question => String::from("?"), Tok::At => String::from("@"),
        Tok::Lt => String::from("<"), Tok::Gt => String::from(">"), Tok::Le => String::from("<="), Tok::Ge => String::from(">="), Tok::AndAnd => String::from("&&"), Tok::OrOr => String::from("||"),
        Tok::DotDot => String::from(".."), Tok::DotDotEq => String::from("..="), Tok::Shl => String::from("<<"), Tok::Shr => String::from(">>"), Tok::PlusEq => String::from("+="),   // range/shift/compound tokens in a macro token-tree (rustc keeps them verbatim); were falling to the `?` fallback
        Tok::LParen => String::from("("), Tok::RParen => String::from(")"), Tok::LBrack => String::from("["), Tok::RBrack => String::from("]"), Tok::LBrace => String::from("{"), Tok::RBrace => String::from("}"),
        Tok::Pub => String::from("pub"), Tok::Const => String::from("const"), Tok::Struct => String::from("struct"), Tok::Enum => String::from("enum"), Tok::Fn => String::from("fn"), Tok::Let => String::from("let"), Tok::If => String::from("if"), Tok::Else => String::from("else"), Tok::Match => String::from("match"), Tok::Type => String::from("type"), Tok::Use => String::from("use"), Tok::Mod => String::from("mod"), Tok::Impl => String::from("impl"), Tok::Trait => String::from("trait"), Tok::Where => String::from("where"), Tok::As => String::from("as"), Tok::Mut => String::from("mut"), Tok::Unsafe => String::from("unsafe"), Tok::While => String::from("while"), Tok::Loop => String::from("loop"), Tok::For => String::from("for"), Tok::In => String::from("in"),   // keyword tokens render verbatim inside macro token-trees (rustc keeps DelimArgs tokens)
        _ => String::from("?")
    }
}
fn dump_use_tree(t: &UseTree) -> String {
    let pre = dump_path(&t.prefix);
    match &t.kind {
        UseKind::Simple(r) => { if r.len() > 0 { format!("Simple {} as {}", pre, r) } else { format!("Simple {}", pre) } }
        UseKind::Glob => format!("Glob {}", pre),
        UseKind::Nested(items) => {
            let mut s = String::new(); let mut first = true;
            for u in items.iter() { if first { s = dump_use_tree(u); first = false; } else { s = format!("{}, {}", s, dump_use_tree(u)); } }
            format!("Nested {} {{ {} }}", pre, s)
        }
    }
}
// C-S2-SEGIDX-OWNED: `path.segments[0].ident.clone()` -- INDEXING a Vec field of an OWNED struct param -- reads
// wrong under a SEM0-built SEM0. Every macro name therefore compared unequal, so stringify!/concat!/cfg! and the
// whole macro_rules registry all fell through and NOTHING expanded -- silently, with a well-formed unexpanded
// MacCall rather than a crash. Extracted to a free fn taking &Path and walking with .iter(), which is the idiom
// this file already uses everywhere else (finding #8: never inline a loop through a destructured field).
// Measured: s2 bootstrap DIFFs 11/13 -> 6/13. The residual 6 all take macro ARGUMENTS; rebuilding the arg key
// with .iter() instead of `toks[ki]` was tried and is INERT, so the residual is NOT Vec-index reads.
fn mac_first_seg(p: &Path) -> String { let mut out = String::new(); let mut i = 0; for sg in p.segments.iter() { if i == 0 { out = sg.ident.clone(); } i = i + 1; } out }
fn dump_path(p: &Path) -> String {
    let mut s = String::new(); let mut first = true;
    for seg in p.segments.iter() { if first { s = dump_seg(seg); first = false; } else { s = format!("{}::{}", s, dump_seg(seg)); } }
    s
}
// C-G2D-FNPTREXT-BOX-DEREF (P25j): the FnPtrExt arm's inline `match &**inner` over-derefs the box exactly as
// P25c (refmatch) and P25g (item-MacCall) did -- emit_s9 hands the arm the ALREADY-resolved inner address, so
// the source-level `&**` loads through it a second time and the dump SEGVs. Same fix as its two precedents:
// route the inner match through a helper taking `&Ty`, which rustc reaches by Deref coercion and emit_s9 by
// passing the address it already has. The trigger is the ABI PREFIX, not the variadic tail: `fn(u8, ...)`
// dumps correctly (plain FnPtr, no wrapper) while `extern "C" fn()` crashes -- an FnPtrExt exists only when a
// prefix was captured, so every extern/unsafe fn-pointer type went through the over-deref and no plain one did.
fn dump_fnptr_ext(pfx: &str, inner: &Ty) -> String {
    match inner { Ty::FnPtr(ins, out) => format!("Ty FnPtr {}{}", pfx, fnptr_body(ins, out)), _ => dump_ty(inner) }
}
fn fnptr_body(ins: &Vec<Ty>, out: &Option<P<Ty>>) -> String {   // the `(ins) -> out` tail shared by FnPtr and FnPtrExt
    let mut s = String::new(); let mut first = true;
    for t in ins.iter() { if first { s = dump_ty(t); first = false; } else { s = format!("{}, {}", s, dump_ty(t)); } }
    let os = match out { Some(t) => format!(" -> {}", dump_ty(t)), None => String::new() };
    format!("({}){}", s, os)
}
fn dump_ty(t: &Ty) -> String {
    match t {
        Ty::Path(p) => format!("Ty Path {}", dump_path(p)),
        Ty::Ref(lt, m, inner) => { let ls = if lt.len() > 0 { format!("'{} ", lt) } else { String::from("") }; let mm = if *m { "mut " } else { "" }; format!("Ty Ref({}{}{})", ls, mm, dump_ty(inner)) }
        Ty::Lt(nm) => format!("Ty Lifetime '{}", nm),
        Ty::ImplTrait(bounds) => { let mut b = String::new(); let mut first = true; for bp in bounds.iter() { if first { b = dump_path(bp); first = false; } else { b = format!("{} + {}", b, dump_path(bp)); } } format!("Ty Impl({})", b) }
        Ty::DynTrait(bounds) => { let mut b = String::new(); let mut first = true; for bp in bounds.iter() { if first { b = dump_path(bp); first = false; } else { b = format!("{} + {}", b, dump_path(bp)); } } format!("Ty Dyn({})", b) }
        Ty::Assoc(nm, ty) => format!("Assoc {} = {}", nm, dump_ty(ty)),
        Ty::AssocBound(nm, ty) => format!("Assoc {} : {}", nm, dump_ty(ty)),   // `Item: Clone` bound-form assoc constraint (vs the `=` equality form)
        Ty::FnPtr(ins, out) => format!("Ty FnPtr{}", fnptr_body(ins, out)),
        Ty::FnPtrExt(pfx, inner) => dump_fnptr_ext(pfx, inner),   // C-G2D-FNPTREXT-BOX-DEREF: helper takes &Ty, avoiding the `match &**inner` over-deref
        Ty::CVarArgs => String::from("..."),
        Ty::ConstArg(is_block, toks) => { let mut inner = String::new(); let mut first = true; for t in toks.iter() { if first { inner = format!("{}", t); first = false; } else { inner = format!("{} {}", inner, t); } } if *is_block { format!("Const {{ {} }}", inner) } else { format!("Const {}", inner) } }
        Ty::Ptr(m, inner) => { let mm = if *m { "mut " } else { "const " }; format!("Ty Ptr({}{})", mm, dump_ty(inner)) }
        Ty::Array(elem, n) => format!("Ty Array({}; {})", dump_ty(elem), dump_expr(n)),
        Ty::Never => String::from("Ty Never"),
        Ty::QPath(qty, tr, proj) => { let mut ps = String::new(); for seg in proj.iter() { ps = format!("{}::{}", ps, dump_seg(seg)); } format!("Ty QPath(<{} as {}>{})", dump_ty(qty), dump_path(tr), ps) }
        Ty::Slice(elem) => format!("Ty Slice({})", dump_ty(elem)),
        Ty::Tup(items) => { let mut s = String::from("Ty Tup("); let mut first = true;
            for it in items.iter() { if first { s = format!("{}{}", s, dump_ty(it)); first = false; } else { s = format!("{}, {}", s, dump_ty(it)); } }
            format!("{})", s) }
    }
}
fn dump_args(args: &Vec<ExprK>) -> String {
    let mut s = String::new(); let mut first = true;
    for a in args.iter() { if first { s = dump_expr(a); first = false; } else { s = format!("{}, {}", s, dump_expr(a)); } }
    s
}
fn dump_expr(e: &ExprK) -> String {
    match e {
        ExprK::LitInt(n) => format!("Expr LitInt {}", lit_value(*n)),
        ExprK::LitFloat(s) => format!("Expr LitFloat {}", s),
        ExprK::Path(p) => format!("Expr Path {}", dump_path(p)),
        ExprK::Bin(op, a, b) => format!("Expr Bin {}({}, {})", op, dump_expr(a), dump_expr(b)),
        ExprK::Unary(op, a) => format!("Expr Unary {}({})", op, dump_expr(a)),
        ExprK::Paren(a) => format!("Expr Paren({})", dump_expr(a)),
        ExprK::Field(recv, name) => format!("Expr Field({}, {})", dump_expr(recv), name),
        ExprK::Method(recv, name, tys, args) => { let mut tf = String::new(); if tys.len() > 0 { let mut ts = String::new(); let mut first = true; for t in tys.iter() { if first { ts = dump_ty(t); first = false; } else { ts = format!("{}, {}", ts, dump_ty(t)); } } tf = format!("::<{}>", ts); } format!("Expr Method({}, {}{}, [{}])", dump_expr(recv), name, tf, dump_args(args)) }
        ExprK::Call(callee, args) => format!("Expr Call({}, [{}])", dump_expr(callee), dump_args(args)),
        ExprK::Index(recv, idx) => format!("Expr Index({}, {})", dump_expr(recv), dump_expr(idx)),
        ExprK::If(c, t, els) => { let mut e = String::new(); for x in els.iter() { e = dump_expr(x); }
            if els.len() > 0 { format!("Expr If({}, {}, {})", dump_expr(c), dump_block(t), e) } else { format!("Expr If({}, {})", dump_expr(c), dump_block(t)) } }
        ExprK::Block(b) => format!("Expr Block({})", dump_block(b)),
        ExprK::Unsafe(b) => format!("Expr Unsafe({})", dump_block(b)),
        ExprK::Match(scrut, arms) => format!("Expr Match({}, [{}])", dump_expr(scrut), dump_arms(arms)),
        ExprK::Str(sv) => format!("Expr Str \"{}\"", sv),
        ExprK::Char(cv) => format!("Expr Char '{}'", cv),
        ExprK::Ref(m, e) => { let mm = if *m { "mut " } else { "" }; format!("Expr AddrOf({}{})", mm, dump_expr(e)) }
        ExprK::Repeat(v, n) => format!("Expr Repeat({}; {})", dump_expr(v), dump_expr(n)),
        ExprK::Async(mv, b) => { let cap = if *mv { "Value" } else { "Ref" }; format!("Expr Async({}, {})", cap, dump_block(b)) }
        ExprK::ConstBlock(b) => format!("Expr ConstBlock({})", dump_block(b)),
        ExprK::TryBlock(b) => format!("Expr TryBlock({})", dump_block(b)),
        ExprK::RawRef(m, e) => { let mm = if *m { "raw mut " } else { "raw const " }; format!("Expr AddrOf({}{})", mm, dump_expr(e)) }
        ExprK::Deref(e) => format!("Expr Deref({})", dump_expr(e)),
        ExprK::Cast(e, ty) => format!("Expr Cast({}, {})", dump_expr(e), dump_ty(ty)),
        ExprK::Closure(is_move, params, ret, body) => { let mut ps = String::new(); let mut first = true;
            for p in params.iter() { let mut one = dump_pat(&p.pat); for t in p.ty.iter() { one = format!("{}: {}", dump_pat(&p.pat), dump_ty(t)); } if first { ps = one; first = false; } else { ps = format!("{}, {}", ps, one); } }
            let mv = if *is_move { "move " } else { "" }; let mut rs = String::new(); for t in ret.iter() { rs = format!(" -> {}", dump_ty(t)); }
            format!("Expr Closure({}[{}]{}, {})", mv, ps, rs, dump_expr(body)) }
        ExprK::Array(items) => format!("Expr Array([{}])", dump_args(items)),
        ExprK::Tuple(items) => format!("Expr Tuple([{}])", dump_args(items)),
        ExprK::Range(a, b, inc) => { let op = if *inc { "..=" } else { ".." }; let ls = match a { Some(e) => dump_expr(e), None => String::from("_") }; let rs = match b { Some(e) => dump_expr(e), None => String::from("_") }; format!("Expr Range({} {} {})", ls, op, rs) }
        ExprK::While(lbl, c, b) => { let ls = if lbl.len() > 0 { format!("'{}: ", lbl) } else { String::new() }; format!("Expr While({}{}, {})", ls, dump_expr(c), dump_block(b)) },
        ExprK::Loop(lbl, b) => { let ls = if lbl.len() > 0 { format!("'{}: ", lbl) } else { String::new() }; format!("Expr Loop({}{})", ls, dump_block(b)) },
        ExprK::ForLoop(lbl, pat, it, b) => { let ls = if lbl.len() > 0 { format!("'{}: ", lbl) } else { String::new() }; format!("Expr ForLoop({}{} in {}, {})", ls, dump_pat(pat), dump_expr(it), dump_block(b)) },
        ExprK::Assign(l, r) => format!("Expr Assign({} = {})", dump_expr(l), dump_expr(r)),
        ExprK::AssignOp(op, l, r) => format!("Expr AssignOp({} {} {})", op, dump_expr(l), dump_expr(r)),
        ExprK::Try(inner) => format!("Expr Try({})", dump_expr(inner)),
        ExprK::Await(inner) => format!("Expr Await({})", dump_expr(inner)),
        ExprK::Bool(b) => { let bs = if *b { "true" } else { "false" }; format!("Expr LitBool {}", bs) }
        ExprK::Let(pat, scrut) => format!("Expr Let({} = {})", dump_pat(pat), dump_expr(scrut)),
        ExprK::Ret(v) => { let mut inner = String::new(); for e in v.iter() { inner = dump_expr(e); } format!("Expr Ret({})", inner) }
        ExprK::Break(lbl, v) => { let mut inner = String::new(); for e in v.iter() { inner = dump_expr(e); } let ls = if lbl.len() > 0 { format!("'{} ", lbl) } else { String::new() }; format!("Expr Break({}{})", ls, inner) }
        ExprK::Continue(lbl) => { let ls = if lbl.len() > 0 { format!("'{}", lbl) } else { String::new() }; format!("Expr Continue({})", ls) }
        ExprK::MacCall(path, delim, toks) => { let mut inner = String::new(); let mut first = true; for t in toks.iter() { if first { inner = format!("{}", t); first = false; } else { inner = format!("{} {}", inner, t); } } format!("Expr MacCall({} ! {}[{}])", dump_path(path), delim, inner) }
        ExprK::QPath(qty, tr, proj) => { let mut ps = String::new(); for seg in proj.iter() { ps = format!("{}::{}", ps, dump_seg(seg)); } format!("Expr QPath(<{} as {}>{})", dump_ty(qty), dump_path(tr), ps) }
        ExprK::Struct(path, fields, base) => { let mut fs = String::new(); let mut first = true; for f in fields.iter() { if first { fs = format!("{}: {}", f.name, dump_expr(&f.expr)); first = false; } else { fs = format!("{}, {}: {}", fs, f.name, dump_expr(&f.expr)); } } let bs = match base { Some(e) => { if first { format!("..{}", dump_expr(e)) } else { format!("{}, ..{}", fs, dump_expr(e)) } }, None => fs }; format!("Expr Struct({}, [{}])", dump_path(path), bs) }
    }
}
fn dump_variant(v: &Variant) -> String {
    let base = match &v.kind {
        VariantKind::Unit => format!("    Variant ident={} Unit", v.ident),
        VariantKind::Tuple(tys) => { let mut s = format!("    Variant ident={} Tuple(", v.ident); let mut first = true;
            for t in tys.iter() { if first { s = format!("{}{}", s, dump_ty(t)); first = false; } else { s = format!("{}, {}", s, dump_ty(t)); } }
            format!("{})", s) }
        VariantKind::Struct(fields) => { let mut s = format!("    Variant ident={} Struct(", v.ident); let mut first = true;
            for f in fields.iter() { if first { s = format!("{}{} {}", s, f.ident, dump_ty(&f.ty)); first = false; } else { s = format!("{}, {} {}", s, f.ident, dump_ty(&f.ty)); } }
            format!("{})", s) }
    };
    match &v.disr { Some(e) => format!("{} disr={}", base, dump_expr(e)), None => base }
}
fn dump_pat(p: &Pat) -> String {
    match p {
        Pat::Wild => String::from("Pat Wild"),
        Pat::Ident(name) => format!("Pat Ident {}", name),
        Pat::Tuple(pats) => format!("Pat Tuple({})", dump_pats(pats)),
        Pat::Lit(e) => format!("Pat Lit({})", dump_expr(e)),
        Pat::TupleStruct(path, pats) => format!("Pat TupleStruct({}, [{}])", dump_path(path), dump_pats(pats)),
        Pat::Or(alts) => format!("Pat Or([{}])", dump_pats(alts)),
        Pat::Path(path) => format!("Pat Path {}", dump_path(path)),
        Pat::Range(lo, hi, inc) => { let op = if *inc { "..=" } else { ".." }; format!("Pat Range({} {} {})", dump_expr(lo), op, dump_expr(hi)) }
        Pat::Ref(inner) => format!("Pat Ref({})", dump_pat(inner)),
        Pat::Bind(mode, name) => format!("Pat Ident {} {}", mode, name),
        Pat::At(name, sub) => format!("Pat Ident {} @ {}", name, dump_pat(sub)),
        Pat::Slice(pats) => format!("Pat Slice([{}])", dump_pats(pats)),
        Pat::Rest => String::from("Pat Rest"),
        Pat::Struct(path, fields, rest) => { let mut s = String::new(); let mut first = true;
            for f in fields.iter() { let fstr = if f.shorthand { format!("{}", f.name) } else { format!("{}: {}", f.name, dump_pat(&f.pat)) };
                if first { s = fstr; first = false; } else { s = format!("{}, {}", s, fstr); } }
            if *rest { if first { s = String::from(".."); } else { s = format!("{}, ..", s); } }
            format!("Pat Struct({}, [{}])", dump_path(path), s) }
    }
}
fn dump_pats(pats: &Vec<Pat>) -> String {
    let mut s = String::new(); let mut first = true;
    for q in pats.iter() { if first { s = dump_pat(q); first = false; } else { s = format!("{}, {}", s, dump_pat(q)); } }
    s
}
fn dump_arm(a: &Arm) -> String { let g = match &a.guard { Some(e) => format!(" if {}", dump_expr(e)), None => String::new() }; format!("Arm({}{} => {})", dump_pat(&a.pat), g, dump_expr(&a.body)) }
fn dump_arms(arms: &Vec<Arm>) -> String {
    let mut s = String::new(); let mut first = true;
    for a in arms.iter() { if first { s = dump_arm(a); first = false; } else { s = format!("{}, {}", s, dump_arm(a)); } }
    s
}
fn dump_stmt(s: &Stmt) -> String {
    match s {
        Stmt::Let(pat, ty, init, els) => { let tstr = match ty { Some(t) => format!(" {}", dump_ty(t)), None => String::from("") }; let istr = match init { Some(e) => format!(" = {}", dump_expr(e)), None => String::from("") }; let estr = match els { Some(b) => format!(" else {}", dump_block(b)), None => String::from("") }; format!("Let {}{}{}{}", dump_pat(pat), tstr, istr, estr) },
        Stmt::Expr(e) => format!("Expr {}", dump_expr(e)),
        Stmt::Semi(e) => format!("Semi {}", dump_expr(e)),
        Stmt::Item(it) => format!("Item {}", dump_item(it)),
        Stmt::Empty => String::from("Empty"),
    }
}
// Inline block dump `Block(stmt; stmt; ...)` so blocks nest cleanly inside if/match/block exprs (the prior
// fixed-indent format did not nest). The Fn arm passes its `body: &P<Block>` payload here as a whole
// (fn(&Block)) rather than accessing fields inline -- L6 mis-reads an inline field access on a Box<Struct>
// by-ref match payload, but the call-boundary deref is correct.
fn dump_block(b: &Block) -> String {
    let mut s = String::from("Block("); let mut first = true;
    for st in b.stmts.iter() { if first { s.push_str(&dump_stmt(st)); first = false; } else { s.push_str("; "); s.push_str(&dump_stmt(st)); } }   // push_str (not format!-accum): O(stmts) not O(stmts^2)
    s.push_str(")"); s
}
fn dump_gparam(p: &GParam) -> String {
    if p.is_const { let mut t = String::new(); for b in p.bounds.iter() { t = dump_path(b); }
        let suf = if p.deflt.len() > 0 { format!(" = {}", p.deflt) } else { String::new() };   // `<const N: T = 8>` default (rustc AnonConst); immutable suffix + single format! (NO mutate-then-return, which L6 miscompiles)
        format!("const {}: {}{}", p.name, t, suf) }
    else if p.is_lt { let mut s = format!("'{}", p.name); let mut first = true;   // `'a` or `'b: 'a` (outlives bound rides in bounds as tick-carrying Paths); FLAT loop, NO nested-if -- the L6-safe shape (nested-if here segfaults stage0)
        for b in p.bounds.iter() { if first { s = format!("{}: {}", s, dump_path(b)); first = false; } else { s = format!("{} + {}", s, dump_path(b)); } }
        s }
    else { let mut s = if p.bounds.len() == 0 { format!("{}", p.name) }
            else { let mut x = format!("{}: ", p.name); let mut first = true;
                for b in p.bounds.iter() { if first { x = format!("{}{}", x, dump_path(b)); first = false; } else { x = format!("{} + {}", x, dump_path(b)); } }
                x };
        if p.deflt.len() > 0 { s = format!("{} = {}", s, p.deflt); }   // `<T = u8>` default type param
        s }
}
// Mirrors dump_arms: the gparams Vec is a bound MATCH PAYLOAD passed to this fn (the proven-safe shape),
// NOT a top-level it.<field> read (which trips filed defect d). Returns "" or "<...>".
fn dump_gparams(gparams: &Vec<GParam>) -> String {
    let mut g = String::new(); let mut any = false; let mut first = true;
    for p in gparams.iter() { any = true; if first { g = format!("<{}", dump_gparam(p)); first = false; } else { g = format!("{}, {}", g, dump_gparam(p)); } }
    if any { format!("{}>", g) } else { String::from("") }
}
fn dump_bound_list(bounds: &Vec<Path>) -> String {   // `: B + C` supertrait / assoc-type bounds; empty -> ""
    if bounds.len() == 0 { return String::from(""); }
    let mut b = String::new(); let mut first = true;
    for bp in bounds.iter() { if first { b = dump_path(bp); first = false; } else { b = format!("{} + {}", b, dump_path(bp)); } }
    format!(": {}", b)
}
fn dump_attr(a: &Attr) -> String {
    if a.args.len() == 0 { format!("Attr {}", dump_path(&a.path)) } else { format!("Attr {}({})", dump_path(&a.path), a.args) }
}
fn dump_attrs(attrs: &Vec<Attr>) -> String {
    let mut s = String::new();
    for a in attrs.iter() { s = format!("{}\n    {}", s, dump_attr(a)); }
    s
}
fn dump_wheres(wheres: &Vec<WherePred>) -> String {
    if wheres.len() == 0 { return String::from(""); }
    let mut s = String::from(" where "); let mut first = true;
    for w in wheres.iter() { let mut b = String::new(); let mut fb = true;
        for bp in w.bounds.iter() { if fb { b = dump_path(bp); fb = false; } else { b = format!("{} + {}", b, dump_path(bp)); } }
        let one = format!("{}: {}", dump_ty(&w.ty), b);
        if first { s = format!("{}{}", s, one); first = false; } else { s = format!("{}, {}", s, one); } }
    s
}
// DRIFT #4 L4-m3 fix: nested items must indent by DEPTH. Each recursive descent (Mod/Impl/Trait/
// AutoTrait/ForeignMod child) wraps the child's WHOLE rendered subtree in reindent1, which prepends
// one 2-space level to EVERY line. Because a grandchild is already reindented by its parent's arm,
// the parent's reindent stacks on top -> indent accumulates with depth (deeper = more indent, == rustc).
// Byte-scan over as_bytes()+slice+format! only (no split/chars); flat while+single-if -> L6-safe (R50).
fn reindent1(s: String) -> String {
    let b = s.as_bytes();
    let mut out = String::from("  ");
    let mut start = 0;
    let mut i = 0;
    while i < b.len() {
        if b[i] == 10 { let line = &s[start..i]; out = format!("{}{}\n  ", out, line); start = i + 1; }
        i += 1;
    }
    format!("{}{}", out, &s[start..])
}
// C-G2D-ITEMMAC-BOX-DEREF: an item-position ItemKind::MacCall carries a P<ExprK> box. emit_s9 binds a box match-payload
// with kind=2 (double-load) -> `mac` is ALREADY the inner ExprK address; a source `match &**mac {..}` then over-derefs it
// as the inner-match scrutinee and faults (like P25c). Route the inner match through a helper taking `&ExprK` and hand it
// `mac` -- rustc Deref-coerces (`&**mac`), emit_s9 passes the already-resolved inner address (which IS &ExprK). No over-deref.
fn dump_item_maccall(mac: &ExprK, sp: &str, vis: &str, a: &str) -> String {
    match mac {
        ExprK::MacCall(path, delim, toks) => { if delim == "MacroDefBrace" {
                let mut inner = String::new(); let mut first = true; for t in toks.iter() { if first { inner = format!("{}", t); first = false; } else { inner = format!("{} {}", inner, t); } }
                format!("  Item MacroDef{} vis={} ident={}{}\n    MacroDef Brace[{}]", sp, vis, dump_path(path), a, inner)
            } else { format!("  Item MacCall{} vis={}{}\n    {}", sp, vis, a, dump_expr(mac)) } }
        _ => format!("  Item MacCall{} vis={}{}\n    {}", sp, vis, a, dump_expr(mac)) }
}
fn dump_item(it: &Item) -> String {
    let a = format!("{}{}", dump_attrs(&it.attrs), dump_wheres(&it.wheres));   // AST-8 attrs + AST-7 R2 where-clause, after the header on EVERY item kind
    let sp = format!("@{}..{}", it.lo, it.hi);   // AST-9: byte-offset span on the item header (narrow first cut; per-expr/ty spans + line:col = later)
    match &it.kind {
        ItemKind::Const(ty, rhs) => { let vstr = match rhs { Some(e) => format!("\n    {}", dump_expr(e)), None => String::from("\n    None") }; format!("  Item Const{} vis={} ident={}{}\n    {}{}", sp, it.vis, it.ident, a, dump_ty(ty), vstr) },
        ItemKind::Static(ty, rhs) => { let vstr = match rhs { Some(e) => format!("\n    {}", dump_expr(e)), None => String::from("\n    None") }; format!("  Item Static{} vis={} ident={}{}\n    {}{}", sp, it.vis, it.ident, a, dump_ty(ty), vstr) },
        ItemKind::Struct(gparams, fields) => {
            let mut s = format!("  Item Struct{} vis={} ident={}{}{}", sp, it.vis, it.ident, dump_gparams(gparams), a);
            for f in fields.iter() { s = format!("{}\n    Field vis={} ident={} {}", s, vis_str(f.is_pub), f.ident, dump_ty(&f.ty)); }
            s
        }
        ItemKind::Union(gparams, fields) => {
            let mut s = format!("  Item Union{} vis={} ident={}{}{}", sp, it.vis, it.ident, dump_gparams(gparams), a);
            for f in fields.iter() { s = format!("{}\n    Field vis={} ident={} {}", s, vis_str(f.is_pub), f.ident, dump_ty(&f.ty)); }
            s
        }
        ItemKind::TupleStruct(gparams, tys) => {
            let mut t = String::new(); let mut first = true; for ty in tys.iter() { if first { t = dump_ty(ty); first = false; } else { t = format!("{}, {}", t, dump_ty(ty)); } }
            format!("  Item Struct{} vis={} ident={}{}{}\n    Tuple({})", sp, it.vis, it.ident, dump_gparams(gparams), a, t)
        }
        ItemKind::UnitStruct(gparams) => format!("  Item Struct{} vis={} ident={}{}{}\n    Unit", sp, it.vis, it.ident, dump_gparams(gparams), a),
        ItemKind::Enum(gparams, variants) => {
            let mut s = format!("  Item Enum{} vis={} ident={}{}{}", sp, it.vis, it.ident, dump_gparams(gparams), a);
            for v in variants.iter() { s = format!("{}\n{}", s, dump_variant(v)); }
            s
        }
        ItemKind::Fn(gparams, params, ret, body, _async) => {
            let mut s = format!("  Item Fn{} vis={} ident={}{}{}", sp, it.vis, it.ident, dump_gparams(gparams), a);
            for p in params.iter() { let tstr = dump_ty(&p.ty); if is_self_param(&p.name) && tstr == String::from("Ty Path Self") { s = format!("{}\n    Param name={} ImplicitSelf", s, p.name); } else { s = format!("{}\n    Param name={} {}", s, p.name, tstr); } }
            let rstr = match ret { Some(t) => format!("\n    Ret {}", dump_ty(t)), None => String::from("\n    Ret Default") };
            s = format!("{}{}", s, rstr);
            match body { Some(b) => format!("{}\n    {}", s, dump_block(b)), None => format!("{}\n    Body None", s) }
        }
        ItemKind::TypeAlias(gparams, ty) => format!("  Item TypeAlias{} vis={} ident={}{}{}{}\n    = {}", sp, it.vis, it.ident, dump_gparams(gparams), dump_bound_list(&it.bounds), a, dump_ty(ty)),
        ItemKind::AssocTyDecl(gparams) => format!("  Item AssocTyDecl{} vis={} ident={}{}{}{}\n    ty None", sp, it.vis, it.ident, dump_gparams(gparams), dump_bound_list(&it.bounds), a),
        ItemKind::MacCall(mac) => dump_item_maccall(mac, &sp, &it.vis, &a),   // C-G2D-ITEMMAC-BOX-DEREF: helper takes &ExprK (Deref-coerced), avoiding `match &**mac` over-deref of the box payload
        ItemKind::Use(tree) => format!("  Item Use{} vis={}{}\n    Use {}", sp, it.vis, a, dump_use_tree(tree)),
        ItemKind::Mod(items) => {
            let mut s = format!("  Item Mod{} vis={} ident={}{}", sp, it.vis, it.ident, a);
            for i in items.iter() { s = format!("{}\n{}", s, reindent1(dump_item(i))); }
            s
        }
        ItemKind::Impl(gparams, trait_ref, self_ty, items) => {
            let tstr = match trait_ref { Some(t) => format!("{} for ", dump_ty(t)), None => String::from("") };
            let pol = if it.ident.len() > 0 { format!("{}", it.ident) } else { String::from("") };   // "!" for a negative impl (polarity), "" positive
            let mut s = format!("  Item Impl{}{} {}{}{}{}", sp, dump_gparams(gparams), pol, tstr, dump_ty(self_ty), a);
            for i in items.iter() { s = format!("{}\n{}", s, reindent1(dump_item(i))); }
            s
        }
        ItemKind::AutoTrait(gparams, items) => {
            let mut s = format!("  Item Trait{} is_auto=Yes vis={} ident={}{}{}{}", sp, it.vis, it.ident, dump_gparams(gparams), dump_bound_list(&it.bounds), a);
            for i in items.iter() { s = format!("{}\n{}", s, reindent1(dump_item(i))); }
            s
        }
        ItemKind::ForeignMod(items) => {
            let mut s = format!("  Item ForeignMod{}{}", sp, a);
            for i in items.iter() { s = format!("{}\n{}", s, reindent1(dump_item(i))); }
            s
        }
        ItemKind::ExternCrate => format!("  Item ExternCrate{} vis={} ident={}{}", sp, it.vis, it.ident, a),
        ItemKind::TraitAlias(gparams, bounds) => format!("  Item TraitAlias{} vis={} ident={}{}{}{}", sp, it.vis, it.ident, dump_gparams(gparams), dump_bound_list(bounds), a),
        ItemKind::Trait(gparams, items) => {
            let mut s = format!("  Item Trait{} vis={} ident={}{}{}{}", sp, it.vis, it.ident, dump_gparams(gparams), dump_bound_list(&it.bounds), a);
            for i in items.iter() { s = format!("{}\n{}", s, reindent1(dump_item(i))); }
            s
        }
    }
}
fn dump(c: &Krate) -> String {
    let mut s = String::from("Crate");
    for it in c.items.iter() { s.push_str("\n"); s.push_str(&dump_item(it)); }   // push_str (not format!-accum): O(N) total, avoids the O(N^2) realloc+copy over all items
    s
}

// ---- S4 HIR lowering (AST -> HIR; `//@S4HIR` mode). Emits rustc `-Zunpretty=hir` SOURCE form (not the S-expr dump
// form): the desugared front-end. HIR-1 lands the `?` Try desugar. Free fns over &Ty/&ExprK/&Pat/&Block/&Stmt/&Item/
// &Path (the proven dump idiom; a &P<X> match payload coerces to &X on the call boundary, per the dump family). Both
// EXPECTED (rustc) and ACTUAL are `tr -s ' \n\t' ' '` whitespace-collapsed, so token ADJACENCY matters (Ok(1),
// Result<i32, ()>, g()) but exact spacing does not -- a doubled space collapses.
fn hir_seg(seg: &PathSeg) -> String {
    if seg.args.len() > 0 {
        let mut a = String::new(); let mut first = true;
        for t in seg.args.iter() { if first { a = hir_ty(t); first = false; } else { a = format!("{}, {}", a, hir_ty(t)); } }
        format!("{}<{}>", seg.ident, a)
    } else { format!("{}", seg.ident) }
}
fn hir_path(p: &Path) -> String {
    let mut s = String::new(); let mut first = true;
    for seg in p.segments.iter() { if first { s = hir_seg(seg); first = false; } else { s = format!("{}::{}", s, hir_seg(seg)); } }
    s
}
fn hir_ty(t: &Ty) -> String {
    match t {
        Ty::Path(p) => hir_path(p),
        Ty::Tup(items) => { let mut s = String::new(); let mut first = true;
            for it in items.iter() { if first { s = hir_ty(it); first = false; } else { s = format!("{}, {}", s, hir_ty(it)); } }
            format!("({})", s) }
        Ty::Ref(lt, m, inner) => { let ls = if lt.len() > 0 { format!("'{} ", lt) } else { String::from("'_ ") }; let mm = if *m { "mut " } else { "" }; format!("&{}{}{}", ls, mm, hir_ty(inner)) }   // rustc HIR names an elided ref lifetime `'_`
        Ty::Slice(elem) => format!("[{}]", hir_ty(elem)),
        _ => String::from("_")
    }
}
fn hir_args(args: &Vec<ExprK>) -> String {
    let mut s = String::new(); let mut first = true;
    for a in args.iter() { if first { s = hir_expr(a); first = false; } else { s = format!("{}, {}", s, hir_expr(a)); } }
    s
}
fn hir_expr(e: &ExprK) -> String {
    match e {
        ExprK::LitInt(n) => format!("{}{}", lit_value(*n), suffix_name(lit_suffix(*n))),
        ExprK::Path(p) => hir_path(p),
        ExprK::Call(callee, args) => format!("{}({})", hir_expr(callee), hir_args(args)),
        ExprK::Assign(l, r) => format!("{} = {}", hir_expr(l), hir_expr(r)),
        // HIR-1: the `?` Try-trait desugar == rustc: `EXPR?` -> `match branch(EXPR) { Break { 0: residual } =>
        // return from_residual(residual), Continue { 0: val } => val, }`. Struct-variant patterns synthesized as
        // literal text (they are not parsed from source). rustc's `#[allow(unreachable_code)]` on the arm bodies is
        // stripped by the instrument, so we omit it.
        ExprK::Try(inner) => format!("match branch({}) {{ Break {{ 0: residual }} => return from_residual(residual), Continue {{ 0: val }} => val, }}", hir_expr(inner)),
        // HIR-2: the `for` desugar == rustc: `for PAT in ITER { BODY }` -> `{ let _t = match into_iter(ITER) {
        // mut iter => loop { match next(&mut iter) { None {} => break, Some { 0: PAT } => BODY } }, }; _t }`. The
        // `mut iter`/`&mut iter`/`_t` scaffolding is synthesized literal text; ITER/PAT/BODY splice from the AST.
        ExprK::ForLoop(_lbl, pat, iter, body) => format!("{{ let _t = match into_iter({}) {{ mut iter => loop {{ match next(&mut iter) {{ None {{}} => break, Some {{ 0: {} }} => {} }} }}, }}; _t }}", hir_expr(iter), hir_pat(pat), hir_block(body)),
        // HIR-3: the `while [let]` desugar == rustc: both `while COND { B }` and `while let PAT = E { B }` lower to
        // `loop { if COND { B } else { break; } }` (rustc's HIR pretty-print RETAINS `if let`, so a let-cond renders as
        // `if let PAT = E`). hir_cond renders the condition (`let PAT = E` for a let-chain operand, else the bare expr).
        ExprK::While(_lbl, cond, body) => format!("loop {{ if {} {} else {{ break; }} }}", hir_cond(cond), hir_block(body)),
        // HIR-4: `if [let]` == rustc, which RETAINS the sugar: `if COND { A } [else B]` (a let-cond renders `if let PAT
        // = E`). The else branch is a single ExprK -- a Block (`else { .. }`) or another If (`else if ..`), so
        // `else ` + hir_expr(x) reconstructs the else-if chain with no special case.
        ExprK::If(cond, then, els) => { let mut e = String::new(); for x in els.iter() { e = format!(" else {}", hir_expr(x)); } format!("if {} {}{}", hir_cond(cond), hir_block(then), e) }
        ExprK::Block(b) => hir_block(b),
        ExprK::Ret(v) => { let mut inner = String::new(); for e in v.iter() { inner = format!(" {}", hir_expr(e)); } format!("return{}", inner) }
        // HIR-7 expr-normalize: rustc's HIR keeps binops FLAT (no auto-precedence parens; explicit source `Paren`
        // nodes are preserved) and renders operators as symbols. op strings are word-form (Add/Sub/..) -> binop_sym.
        ExprK::Bin(op, a, b) => format!("{} {} {}", hir_expr(a), binop_sym(op), hir_expr(b)),
        ExprK::Unary(op, a) => format!("{}{}", unop_sym(op), hir_expr(a)),
        ExprK::Paren(a) => format!("({})", hir_expr(a)),
        ExprK::Field(recv, name) => format!("{}.{}", hir_expr(recv), name),
        ExprK::Method(recv, name, tys, args) => { let mut tf = String::new(); if tys.len() > 0 { let mut ts = String::new(); let mut first = true; for t in tys.iter() { if first { ts = hir_ty(t); first = false; } else { ts = format!("{}, {}", ts, hir_ty(t)); } } tf = format!("::<{}>", ts); } format!("{}.{}{}({})", hir_expr(recv), name, tf, hir_args(args)) }
        ExprK::Index(recv, idx) => format!("{}[{}]", hir_expr(recv), hir_expr(idx)),
        ExprK::Match(scrut, arms) => format!("match {} {{ {}}}", hir_expr(scrut), hir_arms(arms)),
        ExprK::Str(sv) => format!("\"{}\"", sv),
        ExprK::Char(cv) => format!("'{}'", cv),
        ExprK::Bool(bv) => { if *bv { String::from("true") } else { String::from("false") } }
        ExprK::Ref(m, e) => { let mm = if *m { "mut " } else { "" }; format!("&{}{}", mm, hir_expr(e)) }
        ExprK::Deref(e) => format!("*{}", hir_expr(e)),
        ExprK::Cast(e, ty) => format!("{} as {}", hir_expr(e), hir_ty(ty)),
        ExprK::Tuple(items) => format!("({})", hir_args(items)),
        ExprK::Array(items) => format!("[{}]", hir_args(items)),
        ExprK::LitFloat(sv) => format!("{}", sv),
        // HIR-8 (+ S1-amendment 2026-08-10): closures render `[move ]|PARAMS|[ -> RET] BODY` -- `move` + the return
        // type are now stored (Closure(is_move, params, ret, body)). DEFERRED (S1 gap S4-c): `async {..}` (rustc lowers
        // it to a `|mut _task_context: ResumeTy|` generator). ConstBlock `const {..}` is retained.
        ExprK::Closure(is_move, params, ret, body) => { let mut ps = String::new(); let mut first = true;
            for p in params.iter() { let mut one = hir_pat(&p.pat); for t in p.ty.iter() { one = format!("{}: {}", hir_pat(&p.pat), hir_ty(t)); } if first { ps = one; first = false; } else { ps = format!("{}, {}", ps, one); } }
            let mv = if *is_move { "move " } else { "" }; let mut rs = String::new(); for t in ret.iter() { rs = format!(" -> {}", hir_ty(t)); }
            format!("{}|{}|{} {}", mv, ps, rs, hir_expr(body)) }
        ExprK::ConstBlock(b) => format!("const {}", hir_block(b)),
        ExprK::TryBlock(b) => format!("try {}", hir_block(b)),
        // rustc LOWERS range EXPRs to the lang-item struct literal (`a..b` -> `Range { start: a, end: b }`); note this
        // is a real lowering, UNLIKE range PATTERNS (HIR-9) which retain `lo...hi`. Inclusive/open forms deferred
        // (no witness yet); exclusive both-bounded is the common front-end shape.
        ExprK::Range(a, b, inc) => { let ls = match a { Some(e) => hir_expr(e), None => String::new() }; let rs = match b { Some(e) => hir_expr(e), None => String::new() }; if *inc { format!("RangeInclusive::new({}, {})", ls, rs) } else { format!("Range {{ start: {}, end: {} }}", ls, rs) } }
        ExprK::Loop(lbl, body) => { let ls = if lbl.len() > 0 { format!("'{}: ", lbl) } else { String::new() }; format!("{}loop {}", ls, hir_block(body)) }
        ExprK::Break(lbl, v) => { let ls = if lbl.len() > 0 { format!(" '{}", lbl) } else { String::new() }; let mut vs = String::new(); for e in v.iter() { vs = format!(" {}", hir_expr(e)); } format!("break{}{}", ls, vs) }
        ExprK::Continue(lbl) => { if lbl.len() > 0 { format!("continue '{}", lbl) } else { String::from("continue") } }
        ExprK::Unsafe(b) => format!("unsafe {}", hir_block(b)),
        // S4-c-1 (S1-amendment 2026-08-10): an async BLOCK lowers to a `[move ]|mut _task_context: ResumeTy| BODY`
        // generator closure == rustc HIR. (async FN -- header `async fn .. -> /*impl Trait*/` + a generator-wrapped
        // body -- is DEFERRED to S4-c-2; it needs an async flag on the Fn item, which the parser currently elides.)
        ExprK::Async(is_move, b) => { let mv = if *is_move { "move " } else { "" }; format!("{}|mut _task_context: ResumeTy| {}", mv, hir_block(b)) }
        ExprK::Await(e) => format!("{}.await", hir_expr(e)),
        ExprK::Struct(path, fields, base) => { let mut fs = String::new(); let mut first = true;
            for f in fields.iter() { let one = format!("{}: {}", f.name, hir_expr(&f.expr)); if first { fs = one; first = false; } else { fs = format!("{}, {}", fs, one); } }
            let bs = match base { Some(e) => { if first { format!("..{}", hir_expr(e)) } else { format!("{}, ..{}", fs, hir_expr(e)) } }, None => fs };
            format!("{} {{ {} }}", hir_path(path), bs) }
        _ => String::from("_")
    }
}
// rustc HIR renders binops as symbols; the parser stores word-form op strings (Add/Sub/Mul/..). Flat if-else chain
// over streq (no loops; the proven flat-dispatch shape).
fn binop_sym(op: &str) -> String {
    if streq(op, "Add") { String::from("+") }
    else if streq(op, "Sub") { String::from("-") }
    else if streq(op, "Mul") { String::from("*") }
    else if streq(op, "Div") { String::from("/") }
    else if streq(op, "Rem") { String::from("%") }
    else if streq(op, "And") { String::from("&&") }
    else if streq(op, "Or") { String::from("||") }
    else if streq(op, "Lt") { String::from("<") }
    else if streq(op, "Gt") { String::from(">") }
    else if streq(op, "Le") { String::from("<=") }
    else if streq(op, "Ge") { String::from(">=") }
    else if streq(op, "Eq") { String::from("==") }
    else if streq(op, "Ne") { String::from("!=") }
    else if streq(op, "BitAnd") { String::from("&") }
    else if streq(op, "BitOr") { String::from("|") }
    else if streq(op, "BitXor") { String::from("^") }
    else if streq(op, "Shl") { String::from("<<") }
    else if streq(op, "Shr") { String::from(">>") }
    else { format!("{}", op) }
}
fn unop_sym(op: &str) -> String {
    if streq(op, "Neg") { String::from("-") } else if streq(op, "Not") { String::from("!") } else { String::from("*") }
}
fn hir_arms(arms: &Vec<Arm>) -> String {
    let mut s = String::new();
    for a in arms.iter() { let g = match &a.guard { Some(e) => format!(" if {}", hir_expr(e)), None => String::new() }; s = format!("{}{}{} => {}, ", s, hir_pat(&a.pat), g, hir_expr(&a.body)); }
    s
}
// A while/if condition. A `let PAT = E` operand (ExprK::Let) renders as `let PAT = E` so the enclosing `if` reads
// `if let PAT = E`; any other expr renders bare.
fn hir_cond(e: &ExprK) -> String {
    match e {
        ExprK::Let(pat, scrut) => format!("let {} = {}", hir_pat(pat), hir_expr(scrut)),
        _ => hir_expr(e)
    }
}
fn is_for(e: &ExprK) -> bool { match e { ExprK::ForLoop(_l, _p, _i, _b) => true, _ => false } }
fn hir_pats(pats: &Vec<Pat>) -> String {
    let mut s = String::new(); let mut first = true;
    for q in pats.iter() { if first { s = hir_pat(q); first = false; } else { s = format!("{}, {}", s, hir_pat(q)); } }
    s
}
fn hir_pat(p: &Pat) -> String {
    match p {
        Pat::Ident(name) => format!("{}", name),
        Pat::Bind(mode, name) => { if mode.len() > 0 { format!("{} {}", mode, name) } else { format!("{}", name) } }
        Pat::TupleStruct(path, pats) => format!("{}({})", hir_path(path), hir_pats(pats)),
        Pat::Path(path) => hir_path(path),
        Pat::Lit(e) => hir_expr(e),
        // HIR-9 pattern-lower. rustc HIR quirks (baseline-inspected): inclusive range pats render `lo...hi` (three
        // dots, old syntax); `@` binds with NO surrounding spaces (`x@10`).
        Pat::Tuple(pats) => format!("({})", hir_pats(pats)),
        Pat::Or(alts) => hir_or_pats(alts),
        Pat::Range(lo, hi, inc) => { let op = if *inc { "..." } else { ".." }; format!("{}{}{}", hir_expr(lo), op, hir_expr(hi)) }
        Pat::At(name, sub) => format!("{}@{}", name, hir_pat(sub)),
        Pat::Ref(inner) => format!("&{}", hir_pat(inner)),
        Pat::Slice(pats) => format!("[{}]", hir_pats(pats)),
        Pat::Rest => String::from(".."),
        Pat::Struct(path, fields, rest) => { let mut s = String::new(); let mut first = true;
            for f in fields.iter() { let one = if f.shorthand { format!("{}", f.name) } else { format!("{}: {}", f.name, hir_pat(&f.pat)) };
                if first { s = one; first = false; } else { s = format!("{}, {}", s, one); } }
            if *rest { if first { s = String::from(".."); } else { s = format!("{}, ..", s); } }
            format!("{} {{ {} }}", hir_path(path), s) }
        Pat::Wild => String::from("_"),
        _ => String::from("_")
    }
}
fn hir_or_pats(pats: &Vec<Pat>) -> String {
    let mut s = String::new(); let mut first = true;
    for q in pats.iter() { if first { s = hir_pat(q); first = false; } else { s = format!("{} | {}", s, hir_pat(q)); } }
    s
}
// `last` = this stmt is the block's final stmt (its tail). Only the desugared `for` (a block-expr that rustc's
// for-lowering emits as a semi-terminated statement) gets a trailing `;` in non-tail position; every other block-like
// expr statement (loop/while/if/match/plain-block) renders bare, matching rustc. The tail expr never gets a `;`.
fn hir_stmt(s: &Stmt, last: bool) -> String {
    match s {
        Stmt::Let(pat, _ty, init, els) => { let istr = match init { Some(e) => format!(" = {}", hir_expr(e)), None => String::new() }; let estr = match els { Some(b) => format!(" else {}", hir_block(b)), None => String::new() }; format!("let {}{}{};", hir_pat(pat), istr, estr) }
        Stmt::Expr(e) => { if last { hir_expr(e) } else if is_for(e) { format!("{};", hir_expr(e)) } else { hir_expr(e) } }
        Stmt::Semi(e) => format!("{};", hir_expr(e)),
        _ => String::new()
    }
}
fn hir_block(b: &Block) -> String {
    let mut s = String::from("{ "); let mut first = true;
    let n = b.stmts.len(); let mut i = 0;
    for st in b.stmts.iter() { let last = i + 1 == n; let one = hir_stmt(st, last); if first { s = format!("{}{}", s, one); first = false; } else { s = format!("{} {}", s, one); } i += 1; }
    format!("{} }}", s)
}
fn hir_item(it: &Item) -> String {
    match &it.kind {
        ItemKind::Fn(_g, params, ret, body, is_async) => {
            let mut ps = String::new(); let mut first = true;
            for p in params.iter() { let one = format!("{}: {}", p.name, hir_ty(&p.ty)); if first { ps = one; first = false; } else { ps = format!("{}, {}", ps, one); } }
            let rstr = match ret { Some(t) => format!(" -> {}", hir_ty(t)), None => String::new() };
            let bstr = match body { Some(b) => hir_block(b), None => String::from("{ }") };
            // S4-c-2 (S1-amendment): an async fn lowers to `async fn NAME(P) -> /*impl Trait*/ |mut _task_context:
            // ResumeTy| { { let _t = BODY; _t } }` == rustc HIR (the opaque return + the generator-wrapped body).
            if *is_async { format!("async fn {}({}) -> /*impl Trait*/ |mut _task_context: ResumeTy| {{ {{ let _t = {}; _t }} }}", it.ident, ps, bstr) }
            else { format!("fn {}({}){} {}", it.ident, ps, rstr, bstr) }
        }
        _ => String::new()
    }
}
fn emit_hir(c: &Krate) {
    for it in c.items.iter() { let line = hir_item(it); if line.len() > 0 { println!("HIR {}", line); } }
}

// ---- S4 HIR-6 (P-HIR-RES-DEFID, the S3->S4 bridge; `//@S4HIRRES` mode). The minimal Res/DefId slice: rustc's hir-tree
// emits `res: Def(Fn, DefId(.. :: NAME))` at each fn-call PATH site. Emit `HIRRES Fn NAME` for every call-expr whose
// callee is a single-segment path resolving to a top-level fn/const/static of this crate (collect_fn_names). A free-fn
// recursive expr walk (the emit_scope_uses idiom); dump_path on the coerced &ExprK; EMIT-path, no Parser field -> rz-safe.
fn hirres_callee_name(e: &ExprK) -> String {
    match e { ExprK::Path(pp) => dump_path(pp), _ => String::new() }
}
// CD/HIR-6 cross-crate: collect the crate's `use` imports as `name|fullpath` (name = rename or last segment). A bare
// call to a use-imported name resolves to that full (cross-crate) path -- rustc reports Def(Fn, DefId(N:.. crate::path)).
// Mirrors emit_privuse_tree's proven &UseTree recursion (Simple leaf + Nested join).
fn collect_use_tree(base: &str, ut: &UseTree, out: &mut Vec<String>) {
    let pp = dump_path(&ut.prefix);
    let full = if base.len() == 0 { pp } else { format!("{}::{}", base, pp) };
    match &ut.kind {
        UseKind::Simple(rename) => { let nm = if rename.len() > 0 { format!("{}", rename) } else { last_seg(&full) }; out.push(format!("{}|{}", nm, full)); }
        UseKind::Nested(kids) => { for k in kids.iter() { collect_use_tree(&full, k, out); } }
        _ => {}
    }
}
fn collect_uses(c: &Krate) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for it in c.items.iter() { match &it.kind { ItemKind::Use(ut) => { collect_use_tree("", ut, &mut out); }, _ => {} } }
    out
}
// normalize an extern-crate-rooted path to the STABLE canonical form rustc reports: core::/alloc:: as-is; std re-exports
// of core modules (cmp/mem/option/result/fmt/iter/ops) -> rewrite std->core; other std:: as-is; a local (non-extern) path
// -> "" (skip). &str-param slice (path[5..] strips the "std::" prefix -- a constant-offset param slice, not a Vec-index).
fn xcrate_target(path: &str) -> String {
    if str_match_at(path, 0, "core::") { return format!("{}", path); }
    if str_match_at(path, 0, "alloc::") { return format!("{}", path); }
    if str_match_at(path, 0, "std::cmp::") { return format!("core::{}", &path[5..]); }
    if str_match_at(path, 0, "std::mem::") { return format!("core::{}", &path[5..]); }
    if str_match_at(path, 0, "std::option::") { return format!("core::{}", &path[5..]); }
    if str_match_at(path, 0, "std::result::") { return format!("core::{}", &path[5..]); }
    if str_match_at(path, 0, "std::fmt::") { return format!("core::{}", &path[5..]); }
    if str_match_at(path, 0, "std::iter::") { return format!("core::{}", &path[5..]); }
    if str_match_at(path, 0, "std::ops::") { return format!("core::{}", &path[5..]); }
    if str_match_at(path, 0, "std::vec::") { return format!("alloc::{}", &path[5..]); }
    if str_match_at(path, 0, "std::boxed::") { return format!("alloc::{}", &path[5..]); }
    if str_match_at(path, 0, "std::string::") { return format!("alloc::{}", &path[5..]); }
    if str_match_at(path, 0, "std::rc::") { return format!("alloc::{}", &path[5..]); }
    if str_match_at(path, 0, "std::") { return format!("{}", path); }
    String::new()
}
// small std/core TYPE-KIND stub table: a cross-crate TYPE path's kind (Struct/Enum/Trait) -- cannot be position-determined
// (unlike a Fn call / a trait bound), so a table is required. Unknown extern types -> "" (skipped, not guessed). Extend as
// witnesses need. Desugar-implicit types (core::ops::range::Range etc.) are deliberately ABSENT -> stay out of scope.
// strip generic args from a type path: `std::vec::Vec<i32>` -> `std::vec::Vec` (cut at the first `<`; trim trailing space).
// &str-param byte-scan (no Vec-index clone). dump_path renders a generic type path with its `< .. >` args attached.
fn path_no_args(s: &str) -> String {
    let by = s.as_bytes(); let n = by.len(); let mut i: usize = 0;
    while i < n { if by[i] == 60 { break; } i = i + 1; }
    let mut e = i; while e > 0 { if by[e-1] == 32 { e = e - 1; } else { break; } }
    let mut out = String::new(); let mut j: usize = 0; while j < e { out.push(by[j] as char); j = j + 1; } out
}
fn xtype_kind(path: &str) -> String {
    if streq(path, "alloc::vec::Vec") { return format!("Struct"); }
    if streq(path, "alloc::string::String") { return format!("Struct"); }
    if streq(path, "alloc::boxed::Box") { return format!("Struct"); }
    if streq(path, "core::option::Option") { return format!("Enum"); }
    if streq(path, "core::result::Result") { return format!("Enum"); }
    if streq(path, "core::fmt::Debug") { return format!("Trait"); }
    if streq(path, "core::cmp::Ordering") { return format!("Enum"); }
    String::new()
}
fn use_lookup(uses: &Vec<String>, name: &str) -> String {
    for u in uses.iter() { if streq(&nth_field(u, 0), name) { return nth_field(u, 1); } }
    String::new()
}
// unified def table: "Kind|Name" for every top-level nameable item -> the Res target set. def_lookup returns a name's
// Kind ("" if none). TupleStruct/UnitStruct render as Struct in hir-tree; PrimTy (i32/..) is NOT a def -> not in table.
fn collect_defs(c: &Krate) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for it in c.items.iter() {
        let nm = format!("{}", it.ident);
        match &it.kind {
            ItemKind::Fn(_a, _b, _r, _bd, _z) => { out.push(format!("Fn|{}", nm)); }
            ItemKind::Const(_t, _e) => { out.push(format!("Const|{}", nm)); }
            ItemKind::Static(_t, _e) => { out.push(format!("Static|{}", nm)); }
            ItemKind::Struct(_g, _f) => { out.push(format!("Struct|{}", nm)); }
            ItemKind::TupleStruct(_g, _t) => { out.push(format!("Struct|{}", nm)); }
            ItemKind::UnitStruct(_g) => { out.push(format!("Struct|{}", nm)); }
            ItemKind::Enum(_g, vs) => { out.push(format!("Enum|{}", nm)); for v in vs.iter() { out.push(format!("Variant|{}::{}", nm, v.ident)); } }
            ItemKind::Union(_g, _f) => { out.push(format!("Union|{}", nm)); }
            ItemKind::TypeAlias(_g, _t) => { out.push(format!("TyAlias|{}", nm)); }
            _ => {}
        }
    }
    out
}
fn def_lookup(defs: &Vec<String>, name: &str) -> String {
    for d in defs.iter() { if streq(&nth_field(d, 1), name) { return nth_field(d, 0); } }
    String::new()
}
// the binding NAMES a pattern introduces -- a bare path to one of these is a Res::Local (NOT a Def).
fn pat_binds(p: &Pat, out: &mut Vec<String>) {
    match p {
        Pat::Ident(n) => { out.push(format!("{}", n)); }
        Pat::Bind(_m, n) => { out.push(format!("{}", n)); }
        Pat::At(n, sub) => { out.push(last_seg(n)); pat_binds(sub, out); }
        Pat::Tuple(ps) => { for x in ps.iter() { pat_binds(x, out); } }
        Pat::TupleStruct(_pp, ps) => { for x in ps.iter() { pat_binds(x, out); } }
        Pat::Or(ps) => { for x in ps.iter() { pat_binds(x, out); } }
        Pat::Ref(sub) => { pat_binds(sub, out); }
        Pat::Slice(ps) => { for x in ps.iter() { pat_binds(x, out); } }
        Pat::Struct(_pp, fps, _r) => { for f in fps.iter() { pat_binds(&f.pat, out); } }
        _ => {}
    }
}
// type-position Res: a Ty::Path head matching a Struct/Enum/Union/TyAlias -> `HIRRES <Kind> <name>`. PrimTy skipped.
fn hirres_ty(t: &Ty, defs: &Vec<String>, uses: &Vec<String>) {
    match t {
        Ty::Path(p) => { let nm0 = dump_path(p); let nm = path_no_args(&nm0); if !str_contains(&nm, "::") { let k = def_lookup(defs, &nm); if streq(&k, "Struct") || streq(&k, "Enum") || streq(&k, "Union") || streq(&k, "TyAlias") { println!("HIRRES {} {}", k, nm); } else { let up = use_lookup(uses, &nm); let xp = xcrate_target(&up); if xp.len() > 0 { let tk = xtype_kind(&xp); if tk.len() > 0 { println!("HIRRES {} {}", tk, xp); } } } } else { let xp = xcrate_target(&nm); if xp.len() > 0 { let tk = xtype_kind(&xp); if tk.len() > 0 { println!("HIRRES {} {}", tk, xp); } } } }
        Ty::Ref(_l, _m, x) => { hirres_ty(x, defs, uses); }
        Ty::Ptr(_m, x) => { hirres_ty(x, defs, uses); }
        Ty::Array(x, _n) => { hirres_ty(x, defs, uses); }
        Ty::Slice(x) => { hirres_ty(x, defs, uses); }
        Ty::Tup(xs) => { for x in xs.iter() { hirres_ty(x, defs, uses); } }
        _ => {}
    }
}
// value-position Res: a bare single-seg path = Res::Local (in scope) else Const/Static Def; a Call callee = Fn Def; a
// struct-literal path = Struct/Enum Def; a Cast carries a type. `locals` threaded by value + veccopy snapshots (S5 idiom).
fn hirres_expr(e: &ExprK, defs: &Vec<String>, uses: &Vec<String>, locals: &Vec<String>) {
    match e {
        ExprK::Path(pp) => { let nm = dump_path(pp); if !str_contains(&nm, "::") { if vec_contains(locals, &nm) { } else { let k = def_lookup(defs, &nm); if streq(&k, "Const") || streq(&k, "Static") { println!("HIRRES {} {}", k, nm); } } } else { let k = def_lookup(defs, &nm); if streq(&k, "Variant") { println!("HIRRES Variant {}", nm); } } }
        ExprK::Call(callee, args) => { let nm = hirres_callee_name(callee); if nm.len() > 0 { let k = def_lookup(defs, &nm); if streq(&k, "Fn") { println!("HIRRES Fn {}", nm); } else { let mut xp = xcrate_target(&nm); if xp.len() == 0 { let up = use_lookup(uses, &nm); if up.len() > 0 { xp = xcrate_target(&up); } } if xp.len() > 0 { println!("HIRRES Fn {}", xp); } } } hirres_expr(callee, defs, uses, locals); for a in args.iter() { hirres_expr(a, defs, uses, locals); } }
        ExprK::Struct(path, fields, base) => { let nm = dump_path(path); if !str_contains(&nm, "::") { let k = def_lookup(defs, &nm); if streq(&k, "Struct") || streq(&k, "Enum") { println!("HIRRES {} {}", k, nm); } } for f in fields.iter() { hirres_expr(&f.expr, defs, uses, locals); } match base { Some(x) => { hirres_expr(x, defs, uses, locals); }, _ => {} } }
        ExprK::Cast(x, t) => { hirres_expr(x, defs, uses, locals); hirres_ty(t, defs, uses); }
        ExprK::Bin(_op, l, r) => { hirres_expr(l, defs, uses, locals); hirres_expr(r, defs, uses, locals); }
        ExprK::Unary(_op, x) => { hirres_expr(x, defs, uses, locals); }
        ExprK::Paren(x) => { hirres_expr(x, defs, uses, locals); }
        ExprK::Field(x, _f) => { hirres_expr(x, defs, uses, locals); }
        ExprK::Method(recv, _m, _ts, args) => { hirres_expr(recv, defs, uses, locals); for a in args.iter() { hirres_expr(a, defs, uses, locals); } }
        ExprK::Index(x, y) => { hirres_expr(x, defs, uses, locals); hirres_expr(y, defs, uses, locals); }
        ExprK::If(c, b, els) => { hirres_expr(c, defs, uses, locals); let mut l2: Vec<String> = veccopy(locals); hirres_cond_binds(c, &mut l2); hirres_block(b, defs, uses, &l2); for x in els.iter() { hirres_expr(x, defs, uses, locals); } }
        ExprK::Block(b) => { hirres_block(b, defs, uses, locals); }
        ExprK::Unsafe(b) => { hirres_block(b, defs, uses, locals); }
        ExprK::Match(s, arms) => { hirres_expr(s, defs, uses, locals); for a in arms.iter() { let mut l2: Vec<String> = veccopy(locals); pat_binds(&a.pat, &mut l2); hirres_expr(&a.body, defs, uses, &l2); } }
        ExprK::Ref(_m, x) => { hirres_expr(x, defs, uses, locals); }
        ExprK::RawRef(_m, x) => { hirres_expr(x, defs, uses, locals); }
        ExprK::Deref(x) => { hirres_expr(x, defs, uses, locals); }
        ExprK::Tuple(xs) => { for x in xs.iter() { hirres_expr(x, defs, uses, locals); } }
        ExprK::Array(xs) => { for x in xs.iter() { hirres_expr(x, defs, uses, locals); } }
        ExprK::While(_l, c, b) => { hirres_expr(c, defs, uses, locals); let mut l2: Vec<String> = veccopy(locals); hirres_cond_binds(c, &mut l2); hirres_block(b, defs, uses, &l2); }
        ExprK::Loop(_l, b) => { hirres_block(b, defs, uses, locals); }
        ExprK::ForLoop(_l, p, it, b) => { hirres_expr(it, defs, uses, locals); let mut l2: Vec<String> = veccopy(locals); pat_binds(p, &mut l2); hirres_block(b, defs, uses, &l2); }
        ExprK::Assign(l, r) => { hirres_expr(l, defs, uses, locals); hirres_expr(r, defs, uses, locals); }
        ExprK::AssignOp(_op, l, r) => { hirres_expr(l, defs, uses, locals); hirres_expr(r, defs, uses, locals); }
        ExprK::Try(x) => { hirres_expr(x, defs, uses, locals); }
        ExprK::Await(x) => { hirres_expr(x, defs, uses, locals); }
        ExprK::Ret(xs) => { for x in xs.iter() { hirres_expr(x, defs, uses, locals); } }
        ExprK::Break(_l, xs) => { for x in xs.iter() { hirres_expr(x, defs, uses, locals); } }
        ExprK::Repeat(v, n) => { hirres_expr(v, defs, uses, locals); hirres_expr(n, defs, uses, locals); }
        ExprK::Async(_m, b) => { hirres_block(b, defs, uses, locals); }
        ExprK::ConstBlock(b) => { hirres_block(b, defs, uses, locals); }
        ExprK::TryBlock(b) => { hirres_block(b, defs, uses, locals); }
        _ => {}
    }
}
fn hirres_cond_binds(c: &ExprK, out: &mut Vec<String>) {
    match c { ExprK::Let(p, _e) => { pat_binds(p, out); }, _ => {} }
}
fn hirres_block(b: &Block, defs: &Vec<String>, uses: &Vec<String>, locals: &Vec<String>) {
    let mut cur: Vec<String> = veccopy(locals);
    for st in b.stmts.iter() {
        match st {
            Stmt::Let(p, ty, init, els) => {
                match ty { Some(t) => { hirres_ty(t, defs, uses); }, _ => {} }
                match init { Some(e) => { hirres_expr(e, defs, uses, &cur); }, _ => {} }
                match els { Some(bl) => { hirres_block(bl, defs, uses, &cur); }, _ => {} }
                pat_binds(p, &mut cur);
            }
            Stmt::Expr(e) => { hirres_expr(e, defs, uses, &cur); }
            Stmt::Semi(e) => { hirres_expr(e, defs, uses, &cur); }
            _ => {}
        }
    }
}
// cross-crate TRAIT BOUNDS: `<T: Debug>` / where T: core::fmt::Debug -> rustc Def(Trait, DefId(N>0: core::fmt::Debug)).
// A bound is POSITION-determined to be a Trait (no type table needed). Resolve each bound path (use-imported or direct
// extern) via xcrate_target; lifetime/`?Sized`/local bounds -> "" (skip).
fn emit_hirres_bounds(g: &Vec<GParam>, uses: &Vec<String>) {
    for gp in g.iter() {
        for b in gp.bounds.iter() {
            let nm = dump_path(b);
            let mut xp = xcrate_target(&nm);
            if xp.len() == 0 { let up = use_lookup(uses, &nm); if up.len() > 0 { xp = xcrate_target(&up); } }
            if xp.len() > 0 { println!("HIRRES Trait {}", xp); }
        }
    }
}
fn emit_hirres(c: &Krate) {
    let defs = collect_defs(c);
    let uses = collect_uses(c);
    for it in c.items.iter() {
        match &it.kind {
            ItemKind::Fn(g, p, r, b, _a) => {
                emit_hirres_bounds(g, &uses);
                for pm in p.iter() { hirres_ty(&pm.ty, &defs, &uses); }
                match r { Some(t) => { hirres_ty(t, &defs, &uses); }, _ => {} }
                let mut locals: Vec<String> = Vec::new();
                for pm in p.iter() { locals.push(format!("{}", pm.name)); }
                match b { Some(bl) => { hirres_block(bl, &defs, &uses, &locals); }, _ => {} }
            }
            ItemKind::Struct(_g, fs) => { for f in fs.iter() { hirres_ty(&f.ty, &defs, &uses); } }
            ItemKind::Union(_g, fs) => { for f in fs.iter() { hirres_ty(&f.ty, &defs, &uses); } }
            ItemKind::TupleStruct(_g, ts) => { for t in ts.iter() { hirres_ty(t, &defs, &uses); } }
            _ => {}
        }
    }
}

// ---- S5 type inference (AST -> typed HIR; `//@S5TYPED` mode). Emits rustc `-Zunpretty=hir,typed`: the HIR with EVERY
// expr wrapped `(EXPR as TYPE)`. TYCK-1 scope = scalar inference (literal-default i32 + let-propagation + binop result
// typing + block-tail/fn-ret typing + unit). The var->type ENV is an &TyEnv ctx param (keeper-decided 2026-08-10);
// the emit reuses the S4 hir_* skeleton (hir_pat/hir_path/hir_ty/binop_sym) + adds type synthesis. Free fns over
// &ExprK/&Stmt/&Block/&Item/&Pat/&TyEnv (the proven dump/hir idiom + the prototyped &TyEnv access). Per-stmt env
// snapshot via veccopy (finding #2: never whole-Vec.clone). Type text is used via .iter()+index-match + format!, NEVER
// a direct env.types[i] slice/clone (finding #12).
struct TyEnv { names: Vec<String>, types: Vec<String> }
// The type bound to `name` in env: parallel-iterate names/types, match the index (NOT env.types[i] direct -- that
// mis-lowers to garbage). Keep the LAST match so a local `let` shadows an earlier binding (fn-items seeded first).
// Prototyped rz==rc before landing.
fn env_type(env: &TyEnv, name: &str) -> String {
    let mut i = 0; let mut hit = -1;
    for n in env.names.iter() { if streq(n, name) { hit = i; } i += 1; }
    if hit < 0 { return String::from("{unknown}"); }
    let mut j = 0; let mut out = String::from("{unknown}");
    for t in env.types.iter() { if j == hit { out = format!("{}", t); } j += 1; }
    out
}
// TYCK-3: extract the return type from a fn-item type string `fn(P1, P2) -> R {name}` (byte-scan a &str PARAM --
// finding 6/7 subset-safe). No `-> ` => unit ret `()`.
fn sig_ret(sig: &str) -> String {
    let b = sig.as_bytes();
    let mut i = 0; let mut arrow = 0; let mut found = false;
    while i + 2 < b.len() { if b[i] == 45 && b[i+1] == 62 && b[i+2] == 32 { arrow = i + 3; found = true; } i += 1; }
    if !found { return String::from("()"); }
    let mut j = arrow; let mut end = b.len();
    while j + 1 < b.len() { if b[j] == 32 && b[j+1] == 123 { end = j; j = b.len(); } else { j += 1; } }
    format!("{}", &sig[arrow..end])
}
// The param-type list `P1, P2` between the first `(` and its matching `)` of a fn-item sig (byte-scan a &str PARAM).
fn sig_params(sig: &str) -> String {
    let b = sig.as_bytes();
    let mut lp = 0; let mut found = false; let mut i = 0;
    while i < b.len() { if b[i] == 40 { lp = i + 1; found = true; i = b.len(); } else { i += 1; } }
    if !found { return String::new(); }
    let mut j = lp; let mut rp = lp;
    while j < b.len() { if b[j] == 41 { rp = j; j = b.len(); } else { j += 1; } }
    format!("{}", &sig[lp..rp])
}
// The k-th (0-based) `, `-separated segment of a param-type list (byte-scan a &str PARAM); "" if out of range.
fn nth_param(params: &str, k: i32) -> String {
    let b = params.as_bytes();
    let mut seg = 0; let mut start = 0; let mut i = 0;
    while i < b.len() {
        if b[i] == 44 { if seg == k { return format!("{}", &params[start..i]); } seg += 1; start = i + 1; if start < b.len() { if b[start] == 32 { start += 1; } } }
        i += 1;
    }
    if seg == k { return format!("{}", &params[start..]); }
    String::new()
}
// A fn item's fn-item type string `fn(P1, P2) -> R {name}` (rustc's ZST fn-item type). Unit ret omits ` -> ()`.
// A GENERIC fn is stored with a `<G1, G2> ` prefix (TYCK-5b reads it to monomorphize at the call site).
fn fn_sig(it: &Item) -> String {
    match &it.kind {
        ItemKind::Fn(gp, params, ret, _body, _async) => {
            let mut ps = String::new(); let mut first = true;
            for p in params.iter() { let t = hir_ty(&p.ty); if first { ps = t; first = false; } else { ps = format!("{}, {}", ps, t); } }
            let rstr = match ret { Some(t) => format!(" -> {}", hir_ty(t)), None => String::new() };
            let gpre = if gp.len() > 0 { format!("{} ", gparams_names(gp)) } else { String::new() };   // names only -- gp_prefix reads bare type-var names for mono
            format!("{}fn({}){} {{{}}}", gpre, ps, rstr, it.ident)
        }
        _ => String::new()
    }
}
// ---- TYCK-5b generic monomorphization helpers (byte-scan over &str PARAMs -- finding 6/7 subset-safe; subst_var
// prototyped rz==rc before landing). A generic registry sig is `<G1, G2> fn(P) -> R {name}`.
fn starts_generic(csig: &str) -> bool { let b = csig.as_bytes(); if b.len() == 0 { return false; } b[0] == 60 }
fn is_ident_byte(c: u8) -> bool { (c >= 65 && c <= 90) || (c >= 97 && c <= 122) || (c >= 48 && c <= 57) || c == 95 }
// Whole-word replace of type-var `g` by `b` in `s` (a `TT`/`Tx` containing T is NOT touched).
fn subst_var(s: &str, g: &str, b: &str) -> String {
    let sb = s.as_bytes(); let gb = g.as_bytes(); let gl = gb.len();
    let mut out = String::new(); let mut i = 0;
    while i < sb.len() {
        let mut m = false;
        if i + gl <= sb.len() {
            let bok = if i == 0 { true } else { if is_ident_byte(sb[i-1]) { false } else { true } };
            let aok = if i + gl == sb.len() { true } else { if is_ident_byte(sb[i+gl]) { false } else { true } };
            if bok && aok { let mut k = 0; let mut eq = true; while k < gl { if sb[i+k] != gb[k] { eq = false; } k += 1; } m = eq; }
        }
        if m { out = format!("{}{}", out, b); i += gl; } else { out = format!("{}{}", out, &s[i..i+1]); i += 1; }
    }
    out
}
// The `<...>` gparam list content at the start of a generic sig (no brackets), or "".
fn gp_prefix(csig: &str) -> String {
    let b = csig.as_bytes();
    if b.len() == 0 { return String::new(); }
    if b[0] != 60 { return String::new(); }
    let mut i = 1; let mut end = 1;
    while i < b.len() { if b[i] == 62 { end = i; i = b.len(); } else { i += 1; } }
    format!("{}", &csig[1..end])
}
// The sig after the `<...> ` prefix (or the whole sig if non-generic).
fn after_gp(csig: &str) -> String {
    let b = csig.as_bytes();
    if b.len() == 0 { return String::new(); }
    if b[0] != 60 { return format!("{}", csig); }
    let mut i = 1; let mut start = 0;
    while i + 1 < b.len() { if b[i] == 62 && b[i+1] == 32 { start = i + 2; i = b.len(); } else { i += 1; } }
    format!("{}", &csig[start..])
}
// The `{name}` base of a fn-item sig.
fn base_of_sig(sig: &str) -> String {
    let b = sig.as_bytes();
    let mut lb = 0; let mut rb = 0; let mut i = 0;
    while i < b.len() { if b[i] == 123 { lb = i + 1; } if b[i] == 125 { rb = i; } i += 1; }
    if rb > lb { return format!("{}", &sig[lb..rb]); }
    String::new()
}
// A fn-item sig without its ` {name}` suffix (`fn(T) -> T {id}` -> `fn(T) -> T`).
fn strip_name(sig: &str) -> String {
    let b = sig.as_bytes();
    let mut cut = b.len(); let mut i = 0;
    while i < b.len() { if b[i] == 123 { if i > 0 { cut = i - 1; } else { cut = 0; } } i += 1; }
    format!("{}", &sig[0..cut])
}
// Infer a type-param `g`'s binding: the type of the first arg whose (inner) param type == g. Iterates args (no
// Vec-index). "_" if not directly inferable (nested/return-position inference deferred).
fn infer_gparam(iparams: &str, args: &Vec<ExprK>, g: &str, env: &TyEnv) -> String {
    let mut k = 0;
    for a in args.iter() { let pt = nth_param(iparams, k); if streq(&pt, g) { return ty_of(a, env, ""); } k += 1; }
    String::from("_")
}
// Monomorphize a generic call: from `<G..> fn(P) -> R {name}` + the args, infer each G, substitute, and build the
// substituted fn-item type `fn(i32) -> i32 {name::<i32>}`.
fn mono_type(csig: &str, args: &Vec<ExprK>, env: &TyEnv) -> String {
    let gplist = gp_prefix(csig);
    let inner = after_gp(csig);
    let base = base_of_sig(&inner);
    let iparams = sig_params(&inner);
    let mut sub = strip_name(&inner);
    let mut monoargs = String::new(); let mut gfirst = true; let mut gi = 0;
    while gi < 16 {
        let g = nth_param(&gplist, gi);
        if g.len() == 0 { gi = 16; }
        else { let binding = infer_gparam(&iparams, args, &g, env); sub = subst_var(&sub, &g, &binding); if gfirst { monoargs = binding; gfirst = false; } else { monoargs = format!("{}, {}", monoargs, binding); } gi += 1; }
    }
    format!("{} {{{}::<{}>}}", sub, base, monoargs)
}
// TYCK-6a: a single generic param NAME (no bounds -- bounds move to a where clause in rustc's typed header).
fn gparam_name_only(p: &GParam) -> String {
    if p.is_lt { format!("'{}", p.name) }
    else if p.is_const { let mut t = String::new(); for b in p.bounds.iter() { t = dump_path(b); } format!("const {}: {}", p.name, t) }
    else { format!("{}", p.name) }
}
// The `<X, Y>` bound-STRIPPED generic-param list (or "").
fn gparams_names(gp: &Vec<GParam>) -> String {
    if gp.len() == 0 { return String::new(); }
    let mut s = String::new(); let mut first = true;
    for p in gp.iter() { let nm = gparam_name_only(p); if first { s = nm; first = false; } else { s = format!("{}, {}", s, nm); } }
    format!("<{}>", s)
}
// The `B1 + B2` bound list of a path-bound Vec.
fn join_bounds(bounds: &Vec<Path>) -> String {
    let mut b = String::new(); let mut fb = true;
    for bp in bounds.iter() { if fb { b = dump_path(bp); fb = false; } else { b = format!("{} + {}", b, dump_path(bp)); } }
    b
}
// The k-th (0-based) ` + `-separated trait in a bound list ("P + Q"); "" if out of range.
fn nth_bound(s: &str, k: i32) -> String {
    let b = s.as_bytes();
    let mut seg = 0; let mut start = 0; let mut i = 0;
    while i + 2 < b.len() { if b[i] == 32 && b[i+1] == 43 && b[i+2] == 32 { if seg == k { return format!("{}", &s[start..i]); } seg += 1; start = i + 3; i += 3; } else { i += 1; } }
    if seg == k { return format!("{}", &s[start..]); }
    String::new()
}
// TYCK-7: the combined ` where ..` clause -- inline gparam bounds hoisted + explicit where-predicates (rustc's typed
// header merges both here). "" if none.
fn all_wheres(gp: &Vec<GParam>, wheres: &Vec<WherePred>) -> String {
    let mut s = String::new(); let mut first = true;
    for p in gp.iter() { if p.is_lt { } else if p.is_const { } else { if p.bounds.len() > 0 { let one = format!("{}: {}", p.name, join_bounds(&p.bounds)); if first { s = one; first = false; } else { s = format!("{}, {}", s, one); } } } }
    for w in wheres.iter() { let one = format!("{}: {}", hir_ty(&w.ty), join_bounds(&w.bounds)); if first { s = one; first = false; } else { s = format!("{}, {}", s, one); } }
    if first { String::new() } else { format!(" where {}", s) }
}
// The ` where X: T, ..` clause collected from type-param bounds (rustc's typed header hoists inline bounds here); "".
fn gparams_where(gp: &Vec<GParam>) -> String {
    let mut s = String::new(); let mut first = true;
    for p in gp.iter() {
        if p.is_lt { } else if p.is_const { } else {
            if p.bounds.len() > 0 {
                let mut b = String::new(); let mut fb = true;
                for bp in p.bounds.iter() { if fb { b = dump_path(bp); fb = false; } else { b = format!("{} + {}", b, dump_path(bp)); } }
                let one = format!("{}: {}", p.name, b);
                if first { s = one; first = false; } else { s = format!("{}, {}", s, one); }
            }
        }
    }
    if first { String::new() } else { format!(" where {}", s) }
}
// A trait's decl-only method `fn m(&self) -> R;` (no body).
fn ty_trait_method(it: &Item) -> String {
    match &it.kind {
        ItemKind::Fn(gp, params, ret, _body, _async) => {
            let mut ps = String::new(); let mut first = true;
            for p in params.iter() { if is_self_param(&p.name) { if first { ps = format!("{}", p.name); first = false; } else { ps = format!("{}, {}", ps, p.name); } } else { let one = format!("{}: {}", p.name, hir_ty(&p.ty)); if first { ps = one; first = false; } else { ps = format!("{}, {}", ps, one); } } }
            let rstr = match ret { Some(t) => format!(" -> {}", hir_ty(t)), None => String::new() };
            format!("fn {}{}({}){};", it.ident, gparams_names(gp), ps, rstr)
        }
        _ => String::new()
    }
}
// TYCK-4: a method's non-self signature `fn(P) -> R` (self excluded; sig_ret/sig_params read it).
fn method_sig(it: &Item) -> String {
    match &it.kind {
        ItemKind::Fn(_g, params, ret, _body, _async) => {
            let mut ps = String::new(); let mut first = true;
            for p in params.iter() { if is_self_param(&p.name) { } else { let t = hir_ty(&p.ty); if first { ps = t; first = false; } else { ps = format!("{}, {}", ps, t); } } }
            let rstr = match ret { Some(t) => format!(" -> {}", hir_ty(t)), None => String::new() };
            format!("fn({}){}", ps, rstr)
        }
        _ => String::new()
    }
}
fn is_known(s: &str) -> bool { if s.len() == 0 { return false; } if streq(s, "{unknown}") { return false; } true }
fn suffix_name(c: i64) -> String { if c==1{return String::from("i8");} if c==2{return String::from("i16");} if c==3{return String::from("i32");} if c==4{return String::from("i64");} if c==5{return String::from("i128");} if c==6{return String::from("isize");} if c==7{return String::from("u8");} if c==8{return String::from("u16");} if c==9{return String::from("u32");} if c==10{return String::from("u64");} if c==11{return String::from("u128");} if c==12{return String::from("usize");} String::new() }
// C-S1-LITERAL-PACK-AMBIGUOUS: the multiplier is COMPUTED, not written. An integer literal is packed as
// `value + code * lit_mul()`, so any literal >= lit_mul() in the SOURCE aliases into the suffix band -- and the
// twelve constants of this very table used to be exactly such literals, which is why lowering `lit_value(*n)`
// once took the whole-file board from 35 ok to 11 ok. Building the multiplier from two factors that are each
// far below it removes every >= 1e15 literal from this file, so the lowering can now unpack unconditionally.
//
// THE MULTIPLIER IS DERIVED, NOT CHOSEN (locales/s1-literal-representation/constraints.md CL-2). Suffix codes
// run 1..12, so the largest packed value is 13*M - 1 and must fit in i64:
//     M <= (2^63 - 1) / 13 = 709,490,156,681,136,600
// M was 1e15 -- 709x below the bound, for no recorded reason. It is now 7e17, which is the largest round value
// under it, so an unsuffixed literal is carried EXACTLY up to 7e17 instead of 1e15. The loss is not removed:
// a single i64 cannot carry an arbitrary i64 value AND a suffix code, so CL-1 is irreducible in this
// representation and can only be MOVED. Removing it needs the suffix carried out of band (CL-1, forward).
// The factors are BOUND TO TYPED i64 LOCALS first: written inline as `1000000 * 1000000000` the product is
// evaluated at 32-bit width and wraps, which is the difference between a literal (carried at its own width) and
// a COMPUTED constant (subject to the tier's arithmetic width). Measured: the inline form gave -1530494975.
fn lit_mul() -> i64 { let a: i64 = 700000000; let b: i64 = 1000000000; a * b }
fn suffix_offset_of(s: &str) -> i64 { let m = lit_mul(); if streq(s,"i8"){return m;} if streq(s,"i16"){return 2*m;} if streq(s,"i32"){return 3*m;} if streq(s,"i64"){return 4*m;} if streq(s,"i128"){return 5*m;} if streq(s,"isize"){return 6*m;} if streq(s,"u8"){return 7*m;} if streq(s,"u16"){return 8*m;} if streq(s,"u32"){return 9*m;} if streq(s,"u64"){return 10*m;} if streq(s,"u128"){return 11*m;} if streq(s,"usize"){return 12*m;} 0 }
// C-S1-CARRIER-TEXT-LEAK (2026-08-26): token-to-text rendering used to print the PACKED CARRIER -- a `1i64`
// token came back as the text "2800000000000000001" (1 + 4*lit_mul()). That text is re-lexed by the macro
// payload path, and it round-tripped only by accident: re-packing an unsuffixed 2.8e18 happens to reproduce the
// same carrier. It stopped round-tripping the moment out-of-range literals began to REFUSE, because the carrier
// is by construction out of range. Rendering the SOURCE form (value + suffix name, as dump_expr already does)
// removes the leak instead of exempting the re-lex from the check.
fn lit_value(p: i64) -> i64 { p % lit_mul() }
fn lit_suffix(p: i64) -> i64 { p / lit_mul() }
// S1-amendment: a method-receiver param (name carries the receiver form, per the 2026-08-10 receiver-form amendment).
fn is_self_param(name: &str) -> bool { streq(name, "self") || streq(name, "&self") || streq(name, "&mut self") || streq(name, "mut self") }
// The type `self` binds in a method body, from the receiver form: &self -> &SelfTy, &mut self -> &mut SelfTy, else SelfTy.
fn self_env_type(recv: &str, self_ty: &str) -> String {
    if streq(recv, "&self") { format!("&{}", self_ty) }
    else if streq(recv, "&mut self") { format!("&mut {}", self_ty) }
    else { format!("{}", self_ty) }
}
// TYCK-6b (the trait obligation): resolve a method sig for receiver-base `base`. First the inherent/impl key
// `Base::method`; on a miss, if `base` is a type-param with a trait bound (`bound:Base` -> Trait), discharge the
// obligation via `Trait::method`. "" if unresolved.
// B3 (GAT projection-under-inference): the content of the FIRST `<..>` group of a rendered type (`Self::Item<X>` -> "X");
// "" if none. &str byte-scan.
fn ret_gat_arg(ret: &str) -> String {
    let b = ret.as_bytes(); let n = b.len();
    let mut lt = n; let mut i = 0; while i < n { if b[i] == 60 { lt = i; i = n; } i = i + 1; }
    if lt == n { return String::new(); }
    let mut gt = n; let mut j = lt + 1; while j < n { if b[j] == 62 { gt = j; j = n; } else { j = j + 1; } }
    if gt == n { return String::new(); }
    format!("{}", &ret[lt + 1..gt])
}
// The 0-based position of the formal param (in a `,`-joined `sig_params` list) whose type renders exactly `ty`; -1 if
// none. Used to find which call argument binds a method type-param that surfaces as a GAT arg in the return type.
fn param_pos_of(params: &str, ty: &str) -> i32 {
    let mut k = 0; while k < 8 { let pe = nth_param(params, k); if pe.len() == 0 { return -1; } if streq(&pe, ty) { return k; } k = k + 1; } -1
}
// Replace the content of the FIRST `<..>` group of `ret` with `sub` (`Self::Item<X>` + "i32" -> "Self::Item<i32>").
fn splice_first_gat(ret: &str, sub: &str) -> String {
    let b = ret.as_bytes(); let n = b.len();
    let mut lt = n; let mut i = 0; while i < n { if b[i] == 60 { lt = i; i = n; } i = i + 1; }
    if lt == n { return format!("{}", ret); }
    let mut gt = n; let mut j = lt + 1; while j < n { if b[j] == 62 { gt = j; j = n; } else { j = j + 1; } }
    if gt == n { return format!("{}", ret); }
    format!("{}{}{}", &ret[0..lt + 1], sub, &ret[gt..])
}
// TYCK-8 projection: a method ret of `Self::Assoc` (an assoc-type projection), given a CONCRETE receiver `base`, resolves
// to the impl's `type Assoc = <ty>` binding (stored `assoc:<base>::<Assoc>` by build_global_env). &str-param slice (ret[6..]
// strips "Self::"). Non-projection rets pass through unchanged.
fn project_assoc(ret: &str, base: &str, env: &TyEnv) -> String {
    if str_match_at(ret, 0, "Self::") {
        let item = format!("{}", &ret[6..]);
        // concrete receiver: the impl's `type Assoc = <ty>` binding.
        let key = format!("assoc:{}::{}", base, item); let b = env_type(env, &key); if is_known(&b) { return b; }
        // generic receiver (a type-param T with a `bound:T` trait): rustc keeps the qualified projection `<T as Trait>::Assoc`.
        let bkey = format!("bound:{}", base); let blist = env_type(env, &bkey);
        if is_known(&blist) { let tr = nth_bound(&blist, 0); if tr.len() > 0 { return format!("<{} as {}>::{}", base, tr, item); } }
    }
    format!("{}", ret)
}
fn resolve_method_sig(env: &TyEnv, base: &str, name: &str) -> String {
    let key = format!("{}::{}", base, name);
    let msig = env_type(env, &key);
    if is_known(&msig) { return msig; }
    let bkey = format!("bound:{}", base);
    let blist = env_type(env, &bkey);
    if is_known(&blist) {
        let mut i = 0;
        while i < 16 { let tr = nth_bound(&blist, i); if tr.len() == 0 { i = 16; } else { let key2 = format!("{}::{}", tr, name); let s2 = env_type(env, &key2); if is_known(&s2) { return s2; } let skey = format!("super:{}", tr); let slist = env_type(env, &skey); if is_known(&slist) { let mut j = 0; while j < 16 { let st = nth_bound(&slist, j); if st.len() == 0 { j = 16; } else { let key3 = format!("{}::{}", st, name); let s3 = env_type(env, &key3); if is_known(&s3) { return s3; } j += 1; } } } i += 1; } }
    }
    String::new()
}
// Strip a leading reference so `&A`/`&mut A` -> `A` for a field/method receiver-type lookup (byte-scan &str PARAM).
fn strip_ref(t: &str) -> String {
    let b = t.as_bytes();
    if b.len() >= 5 { if b[0] == 38 && b[1] == 109 && b[2] == 117 && b[3] == 116 && b[4] == 32 { return format!("{}", &t[5..]); } }
    if b.len() >= 1 { if b[0] == 38 { return format!("{}", &t[1..]); } }
    format!("{}", t)
}
// The global type env, seeded into every fn/method block env: top-level fns (name -> fn-item type), struct fields
// (`Type.field` -> field type), and inherent/trait methods (`Type::method` -> non-self sig). Keys with `.`/`::` never
// collide with local var/param names. The S5 global-table idiom (env-folding) -- no extra ctx param.
fn build_global_env(c: &Krate) -> TyEnv {
    let mut names: Vec<String> = Vec::new(); let mut types: Vec<String> = Vec::new();
    for it in c.items.iter() {
        match &it.kind {
            ItemKind::Fn(_g, _p, _r, _b, _async) => { names.push(format!("{}", it.ident)); types.push(fn_sig(it)); }
            ItemKind::Struct(_gp, fields) => { for f in fields.iter() { names.push(format!("{}.{}", it.ident, f.ident)); types.push(hir_ty(&f.ty)); } }
            // a unit struct's value path `A` has type `A` (TYCK-6a); trait methods keyed `Trait::method` (TYCK-6b bound res).
            ItemKind::UnitStruct(_gp) => { names.push(format!("{}", it.ident)); types.push(format!("{}", it.ident)); }
            ItemKind::Trait(_gp, items) => { for m in items.iter() { match &m.kind { ItemKind::Fn(_g2, _p2, _r2, _b2, _async) => { names.push(format!("{}::{}", it.ident, m.ident)); types.push(method_sig(m)); }, _ => {} } } if it.bounds.len() > 0 { names.push(format!("super:{}", it.ident)); types.push(join_bounds(&it.bounds)); } }
            ItemKind::Impl(_gp, _tr, self_ty, items) => { let sty = hir_ty(self_ty); for m in items.iter() { match &m.kind { ItemKind::Fn(_g2, _p2, _r2, _b2, _async) => { names.push(format!("{}::{}", sty, m.ident)); types.push(method_sig(m)); }, ItemKind::TypeAlias(_g3, ty3) => { names.push(format!("assoc:{}::{}", sty, m.ident)); types.push(hir_ty(ty3)); }, _ => {} } } }
            _ => {}
        }
    }
    TyEnv { names: names, types: types }
}
fn is_cmp_op(op: &str) -> bool {
    streq(op, "Eq") || streq(op, "Ne") || streq(op, "Lt") || streq(op, "Gt") || streq(op, "Le") || streq(op, "Ge") || streq(op, "And") || streq(op, "Or")
}
fn pat_name(p: &Pat) -> String {
    match p { Pat::Ident(name) => format!("{}", name), Pat::Bind(_mode, name) => format!("{}", name), _ => String::from("") }
}
// The inferred TYPE of an expr: int literal -> the EXPECTED type if one is pushed down (TYCK-2 annotation/param
// context), else i32 (rustc's default); bool -> bool; a comparison/logical binop -> bool; an arithmetic binop -> the
// expected type if pushed, else its operands' type; a var (Path) -> its env binding. `exp` = "" means infer.
// TYCK-residual: is a field name a NUMERIC tuple index (`t.0`)? + the N-th element type of a tuple type `(T0, T1, ..)`
// (depth-aware comma split, trimmed). &str-param byte-scans (rz-safe).
fn is_num_field(s: &str) -> bool { let b = s.as_bytes(); if b.len() == 0 { return false; } if b[0] >= 48 { if b[0] <= 57 { return true; } } false }
fn trim_seg(s: &str, a: usize, b: usize) -> String {
    let by = s.as_bytes(); let mut i = a; let mut e = b;
    while i < e { if by[i] == 32 { i = i + 1; } else { break; } }
    while e > i { if by[e-1] == 32 { e = e - 1; } else { break; } }
    let mut out = String::new(); let mut j = i; while j < e { out.push(by[j] as char); j = j + 1; } out
}
fn tuple_nth(tup: &str, idx: &str) -> String {
    let n = parse_i64(idx); let by = tup.as_bytes(); let m = by.len();
    if m < 2 { return String::new(); }
    let mut cur: i64 = 0; let mut depth: i64 = 0; let mut start: usize = 1; let mut i: usize = 1;
    while i + 1 < m {
        let c = by[i];
        if c == 40 { depth = depth + 1; } if c == 91 { depth = depth + 1; } if c == 60 { depth = depth + 1; }
        if c == 41 { depth = depth - 1; } if c == 93 { depth = depth - 1; } if c == 62 { depth = depth - 1; }
        if c == 44 { if depth == 0 { if cur == n { return trim_seg(tup, start, i); } cur = cur + 1; start = i + 1; } }
        i = i + 1;
    }
    if cur == n { return trim_seg(tup, start, m - 1); }
    String::new()
}
// TYCK-residual: the ELEMENT type of an array/slice type `[T; N]` / `[T]` / `&[T]` -> T (depth-0 `;` split; strips a
// leading `&`). &str-param byte-scan.
fn elem_of(t: &str) -> String {
    let base = strip_ref(t); let by = base.as_bytes(); let m = by.len();
    if m < 2 { return String::new(); }
    if by[0] != 91 { return String::new(); }
    let mut depth: i64 = 0; let mut i: usize = 1; let mut semi: usize = 0; let mut found = false;
    while i + 1 < m {
        let c = by[i];
        if c == 91 { depth = depth + 1; } if c == 40 { depth = depth + 1; } if c == 60 { depth = depth + 1; }
        if c == 93 { depth = depth - 1; } if c == 41 { depth = depth - 1; } if c == 62 { depth = depth - 1; }
        if c == 59 { if depth == 0 { semi = i; found = true; i = m; } }
        if i < m { i = i + 1; }
    }
    let e = if found { semi } else { m - 1 };
    trim_seg(&base, 1, e)
}
fn ty_of(e: &ExprK, env: &TyEnv, exp: &str) -> String {
    match e {
        ExprK::LitInt(n) => { let c = lit_suffix(*n); if c != 0 { suffix_name(c) } else if exp.len() > 0 { format!("{}", exp) } else { String::from("i32") } }
        ExprK::Bool(_b) => String::from("bool"),
        ExprK::Path(p) => env_type(env, &hir_path(p)),
        ExprK::Bin(op, a, _b) => { if is_cmp_op(op) { String::from("bool") } else if exp.len() > 0 { format!("{}", exp) } else { ty_of(a, env, "") } }
        ExprK::Paren(a) => ty_of(a, env, exp),
        // TYCK-3/5b: a call's type = the callee fn's RETURN type. For a GENERIC callee, monomorphize first (infer +
        // substitute the type args) then read the substituted ret.
        ExprK::Call(callee, args) => { let csig = ty_of(callee, env, ""); if starts_generic(&csig) { let mono = mono_type(&csig, args, env); sig_ret(&mono) } else { sig_ret(&csig) } }
        // TYCK-4/6b: a method call's type = the resolved method's return type (inherent `RecvBase::method`, or via a
        // type-param's trait bound -- the trait obligation).
        ExprK::Method(recv, name, _tys, args) => {
            let rt = ty_of(recv, env, ""); let base = strip_ref(&rt);
            let msig = resolve_method_sig(env, &base, name); let r0 = sig_ret(&msig);
            // B3: if the ret carries a GAT arg that is a method type-param bound by a call argument (`Self::Item<X>`,
            // X = the k-th formal's type), infer X from that argument's actual type and substitute (`Self::Item<i32>`).
            let ga = ret_gat_arg(&r0); let mut r = format!("{}", r0);
            if ga.len() > 0 { let params = sig_params(&msig); let pos = param_pos_of(&params, &ga);
                if pos >= 0 { let mut idx = 0; let mut inferred = String::new();
                    for a in args.iter() { if idx == pos { inferred = ty_of(a, env, ""); } idx = idx + 1; }
                    if is_known(&inferred) { r = splice_first_gat(&r0, &inferred); } } }
            project_assoc(&r, &base, env)
        }
        // a field's type = the struct field's declared type (`RecvBase.field`).
        ExprK::Field(recv, name) => { let rt = ty_of(recv, env, ""); let base = strip_ref(&rt); if is_num_field(name) { if str_match_at(&base, 0, "(") { tuple_nth(&base, name) } else { let key = format!("{}.{}", base, name); env_type(env, &key) } } else { let key = format!("{}.{}", base, name); env_type(env, &key) } }
        ExprK::Tuple(xs) => { let mut inner = String::new(); let mut first = true; for x in xs.iter() { let one = ty_of(x, env, ""); if first { inner = one; first = false; } else { inner = format!("{}, {}", inner, one); } } format!("({})", inner) }
        ExprK::Array(xs) => { let mut et = String::from("_"); for x in xs.iter() { et = ty_of(x, env, ""); break; } format!("[{}; {}]", et, xs.len()) }
        ExprK::Index(a, _i) => { let at = ty_of(a, env, ""); elem_of(&at) }
        ExprK::If(_c, tb, ev) => { if ev.len() == 0 { String::from("()") } else { ty_block_tail(tb, env) } }
        ExprK::Cast(_x, ty) => hir_ty(ty),
        ExprK::Ref(m, x) => { let it = ty_of(x, env, ""); if *m { format!("&mut {}", it) } else { format!("&{}", it) } }
        ExprK::Deref(x) => { let it = ty_of(x, env, ""); strip_ref(&it) }
        ExprK::Str(_s) => String::from("&str"),
        ExprK::Char(_c) => String::from("char"),
        ExprK::Struct(path, _f, _b) => dump_path(path),
        _ => String::from("{?}")
    }
}
// The typed rendering of an expr's CHILDREN (leaf = the hir form; compound = recurse via ty_expr on sub-exprs). For an
// arithmetic binop the operands inherit the binop's result type as their expected type (bidirectional propagation);
// a comparison binop pushes nothing (each operand infers its own type).
fn ty_inner(e: &ExprK, env: &TyEnv, exp: &str) -> String {
    match e {
        ExprK::LitInt(n) => format!("{}{}", lit_value(*n), suffix_name(lit_suffix(*n))),
        ExprK::Path(p) => hir_path(p),
        ExprK::Bin(op, a, b) => { let sub = if is_cmp_op(op) { String::new() } else { ty_of(e, env, exp) }; format!("{} {} {}", ty_expr(a, env, &sub), binop_sym(op), ty_expr(b, env, &sub)) }
        ExprK::Paren(a) => ty_inner(a, env, exp),
        ExprK::Bool(bv) => { if *bv { String::from("true") } else { String::from("false") } }
        // TYCK-3/5b: `CALLEE(ARGS)`. Monomorphic -> callee typed as its fn-item type. GENERIC -> the callee renders
        // `(name as fn(SUBST) -> R {name::<ARGS>})` (mono type), args typed against the substituted params.
        ExprK::Call(callee, args) => {
            let csig = ty_of(callee, env, "");
            if starts_generic(&csig) {
                let mono = mono_type(&csig, args, env); let params = sig_params(&mono);
                let mut argstr = String::new(); let mut first = true; let mut k = 0;
                for a in args.iter() { let pe = nth_param(&params, k); let one = ty_expr(a, env, &pe); if first { argstr = one; first = false; } else { argstr = format!("{}, {}", argstr, one); } k += 1; }
                format!("({} as {})({})", ty_inner(callee, env, ""), mono, argstr)
            } else {
                let params = sig_params(&csig);
                let mut argstr = String::new(); let mut first = true; let mut k = 0;
                for a in args.iter() { let pe = nth_param(&params, k); let one = ty_expr(a, env, &pe); if first { argstr = one; first = false; } else { argstr = format!("{}, {}", argstr, one); } k += 1; }
                format!("{}({})", ty_expr(callee, env, ""), argstr)
            }
        }
        // TYCK-4: `RECV.method(ARGS)` -- receiver typed, args against the resolved method's params.
        ExprK::Method(recv, name, _tys, args) => { let rt = ty_of(recv, env, ""); let base = strip_ref(&rt); let msig = resolve_method_sig(env, &base, name); let params = sig_params(&msig); let mut argstr = String::new(); let mut first = true; let mut k = 0; for a in args.iter() { let pe = nth_param(&params, k); let one = ty_expr(a, env, &pe); if first { argstr = one; first = false; } else { argstr = format!("{}, {}", argstr, one); } k += 1; } format!("{}.{}({})", ty_expr(recv, env, ""), name, argstr) }
        ExprK::Field(recv, name) => format!("{}.{}", ty_expr(recv, env, ""), name),
        ExprK::Tuple(xs) => { let mut inner = String::new(); let mut first = true; for x in xs.iter() { let one = ty_expr(x, env, ""); if first { inner = one; first = false; } else { inner = format!("{}, {}", inner, one); } } format!("({})", inner) }
        ExprK::Array(xs) => { let mut inner = String::new(); let mut first = true; for x in xs.iter() { let one = ty_expr(x, env, ""); if first { inner = one; first = false; } else { inner = format!("{}, {}", inner, one); } } format!("[{}]", inner) }
        ExprK::Index(a, i) => format!("{}[{}]", ty_expr(a, env, ""), ty_expr(i, env, "usize")),
        ExprK::If(c, tb, ev) => { let mut out = format!("if {} {}", ty_expr(c, env, ""), ty_arm_block(tb, env)); for e2 in ev.iter() { match e2 { ExprK::Block(bl) => { out = format!("{} else {}", out, ty_arm_block(bl, env)); } _ => { out = format!("{} else {}", out, ty_inner(e2, env, "")); } } } out }
        ExprK::Cast(x, ty) => format!("{} as {}", ty_expr(x, env, ""), hir_ty(ty)),
        ExprK::Ref(m, x) => { if *m { format!("&mut {}", ty_expr(x, env, "")) } else { format!("&{}", ty_expr(x, env, "")) } }
        ExprK::Deref(x) => format!("*{}", ty_expr(x, env, "")),
        ExprK::Str(sv) => format!("\"{}\"", sv),
        ExprK::Char(cv) => format!("'{}'", cv),
        ExprK::Struct(path, fields, base) => { let sn = dump_path(path); let mut fs = String::new(); let mut first = true; for f in fields.iter() { let ft = env_type(env, &format!("{}.{}", sn, f.name)); let one = format!("{}: {}", f.name, ty_expr(&f.expr, env, &ft)); if first { fs = one; first = false; } else { fs = format!("{}, {}", fs, one); } } match base { Some(bx) => { if first { fs = format!("..{}", ty_expr(bx, env, "")); } else { fs = format!("{}, ..{}", fs, ty_expr(bx, env, "")); } }, None => {} } format!("{} {{ {} }}", sn, fs) }
        _ => String::from("?")
    }
}
// One expr, typed: `(INNER as TYPE)` == rustc hir,typed.
fn ty_expr(e: &ExprK, env: &TyEnv, exp: &str) -> String { format!("({} as {})", ty_inner(e, env, exp), ty_of(e, env, exp)) }
// The type this stmt contributes as a block tail (an Expr stmt -> its type; otherwise unit).
fn ty_stmt_tailty(s: &Stmt, env: &TyEnv) -> String { match s { Stmt::Expr(e) => ty_of(e, env, ""), _ => String::from("()") } }
fn ty_stmt(s: &Stmt, env: &TyEnv, last: bool) -> String {
    match s {
        Stmt::Let(pat, ty, init, _els) => { let tstr = match ty { Some(t) => format!(": {}", hir_ty(t)), None => String::new() }; let exp = match ty { Some(t) => hir_ty(t), None => String::new() }; let istr = match init { Some(e) => format!(" = {}", ty_expr(e, env, &exp)), None => String::new() }; format!("let {}{}{};", hir_pat(pat), tstr, istr) }
        Stmt::Expr(e) => { if last { ty_expr(e, env, "") } else { format!("{};", ty_expr(e, env, "")) } }
        Stmt::Semi(e) => format!("{};", ty_expr(e, env, "")),
        _ => String::new()
    }
}
// A block, typed + WRAPPED `({ .. } as TAILTYPE)`. Seeds from the parent env (the fn's params, via veccopy) then
// threads a growing env: each `let name = init` (or `let name: T`) extends names/types AFTER typing that stmt. An
// annotation `let x: T = ..` binds x:T (and pushes T as the init's expected type); else x:ty_of(init).
fn ty_arm_block(b: &Block, penv: &TyEnv) -> String {
    let mut names: Vec<String> = veccopy(&penv.names);
    let mut types: Vec<String> = veccopy(&penv.types);
    let mut s = String::from("{ ");
    let n = b.stmts.len(); let mut i = 0; let mut first = true;
    for st in b.stmts.iter() {
        let last = i + 1 == n;
        let env = TyEnv { names: veccopy(&names), types: veccopy(&types) };
        let one = ty_stmt(st, &env, last);
        if first { s = format!("{}{}", s, one); first = false; } else { s = format!("{} {}", s, one); }
        match st { Stmt::Let(pat, ty, init, _els) => { let nm = pat_name(pat); if nm.len() > 0 { let exp = match ty { Some(t) => hir_ty(t), None => String::new() }; match init { Some(e) => { names.push(nm); types.push(ty_of(e, &env, &exp)); }, None => { match ty { Some(t) => { names.push(nm); types.push(hir_ty(t)); }, None => {} } } } } }, _ => {} }
        i += 1;
    }
    format!("{} }}", s)
}
fn ty_block_tail(b: &Block, penv: &TyEnv) -> String {
    let mut names: Vec<String> = veccopy(&penv.names);
    let mut types: Vec<String> = veccopy(&penv.types);
    let mut tail_ty = String::from("()");
    let n = b.stmts.len(); let mut i = 0;
    for st in b.stmts.iter() {
        let last = i + 1 == n;
        let env = TyEnv { names: veccopy(&names), types: veccopy(&types) };
        if last { tail_ty = ty_stmt_tailty(st, &env); }
        match st { Stmt::Let(pat, ty, init, _els) => { let nm = pat_name(pat); if nm.len() > 0 { let exp = match ty { Some(t) => hir_ty(t), None => String::new() }; match init { Some(e) => { names.push(nm); types.push(ty_of(e, &env, &exp)); }, None => { match ty { Some(t) => { names.push(nm); types.push(hir_ty(t)); }, None => {} } } } } }, _ => {} }
        i += 1;
    }
    tail_ty
}
fn ty_block(b: &Block, penv: &TyEnv) -> String {
    let mut names: Vec<String> = veccopy(&penv.names);
    let mut types: Vec<String> = veccopy(&penv.types);
    let mut s = String::from("{ ");
    let mut tail_ty = String::from("()");
    let n = b.stmts.len(); let mut i = 0; let mut first = true;
    for st in b.stmts.iter() {
        let last = i + 1 == n;
        let env = TyEnv { names: veccopy(&names), types: veccopy(&types) };
        if last { tail_ty = ty_stmt_tailty(st, &env); }
        let one = ty_stmt(st, &env, last);
        if first { s = format!("{}{}", s, one); first = false; } else { s = format!("{} {}", s, one); }
        match st { Stmt::Let(pat, ty, init, _els) => { let nm = pat_name(pat); if nm.len() > 0 { let exp = match ty { Some(t) => hir_ty(t), None => String::new() }; match init { Some(e) => { names.push(nm); types.push(ty_of(e, &env, &exp)); }, None => { match ty { Some(t) => { names.push(nm); types.push(hir_ty(t)); }, None => {} } } } } }, _ => {} }
        i += 1;
    }
    format!("({} }} as {})", s, tail_ty)
}
// A fn/method, typed. `self_ty` non-empty => this is an impl method: the receiver renders `&self` (S1-AST gap: the
// receiver form self/&self/&mut self is not stored -- assume &self) and `self` binds `&SelfTy` in the body env.
fn ty_fn(it: &Item, genv: &TyEnv, self_ty: &str) -> String {
    match &it.kind {
        ItemKind::Fn(gp, params, ret, body, _async) => {
            let mut ps = String::new(); let mut first = true;
            let mut pn: Vec<String> = veccopy(&genv.names); let mut pt: Vec<String> = veccopy(&genv.types);
            // TYCK-6b/7: register each type-param's trait bound(s) (`bound:X` -> `P + Q`, from inline gparam bounds AND
            // explicit where-preds) so a method call on `x: X` discharges the obligation via one of the bound traits.
            for g in gp.iter() { if g.is_lt { } else if g.is_const { } else { if g.bounds.len() > 0 { pn.push(format!("bound:{}", g.name)); pt.push(join_bounds(&g.bounds)); } } }
            // Merge repeated predicates for the same type-param (`where T: Ct, T: Mk` AND an inline `<T: A>`) into one
            // ` + `-joined `bound:T` so method-res + projection see the UNION, not just the last predicate (which clobbers).
            for w in it.wheres.iter() {
                let k = format!("bound:{}", hir_ty(&w.ty)); let nb = join_bounds(&w.bounds);
                // last-match scan of the LOCAL pn/pt (a &Vec param would not lower in rz -- finding 3; local Vec .iter() does).
                let mut hit = -1; let mut i = 0; for n in pn.iter() { if streq(n, &k) { hit = i; } i += 1; }
                let mut prev = String::from("{unknown}"); if hit >= 0 { let mut j = 0; for t in pt.iter() { if j == hit { prev = format!("{}", t); } j += 1; } }
                let mut merged = format!("{}", nb); if is_known(&prev) { merged = format!("{} + {}", prev, nb); }
                pn.push(k); pt.push(merged);
            }
            for p in params.iter() {
                // S1-amendment: render the receiver form (name) verbatim; seed `self` -> the receiver-form type.
                if is_self_param(&p.name) { if first { ps = format!("{}", p.name); first = false; } else { ps = format!("{}, {}", ps, p.name); } pn.push(String::from("self")); pt.push(self_env_type(&p.name, self_ty)); }
                else { let one = format!("{}: {}", p.name, hir_ty(&p.ty)); if first { ps = one; first = false; } else { ps = format!("{}, {}", ps, one); } pn.push(format!("{}", p.name)); pt.push(hir_ty(&p.ty)); }
            }
            let penv = TyEnv { names: pn, types: pt };
            let rstr = match ret { Some(t) => format!(" -> {}", hir_ty(t)), None => String::new() };
            let bstr = match body { Some(b) => ty_block(b, &penv), None => String::from("({ } as ())") };
            // TYCK-5a/6a: the bound-stripped generic list `<X>` after the name + a hoisted ` where X: T` clause,
            // matching rustc's typed header. In a generic body a type-param stays uninstantiated (x: T -> T).
            format!("fn {}{}({}){}{} {}", it.ident, gparams_names(gp), ps, rstr, all_wheres(gp, &it.wheres), bstr)
        }
        _ => String::new()
    }
}
fn ty_item(it: &Item, genv: &TyEnv) -> String {
    match &it.kind {
        ItemKind::Fn(_g, _p, _r, _b, _async) => ty_fn(it, genv, ""),
        ItemKind::Struct(_gp, fields) => { let mut fs = String::new(); for f in fields.iter() { fs = format!("{}{}: {}, ", fs, f.ident, hir_ty(&f.ty)); } format!("struct {} {{ {}}}", it.ident, fs) }
        ItemKind::UnitStruct(_gp) => format!("struct {};", it.ident),
        ItemKind::Trait(gp, items) => {
            let sup = if it.bounds.len() > 0 { format!(": {}", join_bounds(&it.bounds)) } else { String::new() };
            let mut s = format!("trait {}{}{} {{", it.ident, gparams_names(gp), sup);
            // TYCK-8: an associated-type DECL `type Item;` alongside decl-only methods.
            for m in items.iter() { let one = match &m.kind { ItemKind::AssocTyDecl(g) => format!("type {}{};", m.ident, gparams_names(g)), _ => ty_trait_method(m) }; if one.len() > 0 { s = format!("{} {}", s, one); } }
            format!("{} }}", s)
        }
        ItemKind::Impl(_gp, trait_ref, self_ty, items) => {
            let sty = hir_ty(self_ty);
            let tstr = match trait_ref { Some(t) => format!("{} for ", hir_ty(t)), None => String::new() };
            let mut s = format!("impl {}{} {{", tstr, sty);
            // TYCK-8: an associated-type BINDING `type Item = i32;` alongside methods.
            for m in items.iter() { let one = match &m.kind { ItemKind::TypeAlias(g, ty) => format!("type {}{} = {};", m.ident, gparams_names(g), hir_ty(ty)), _ => ty_fn(m, genv, &sty) }; if one.len() > 0 { s = format!("{} {}", s, one); } }
            format!("{} }}", s)
        }
        _ => String::new()
    }
}
// ---- S9 lower-codegen (`//@S9` mode). emit_s9(&ast) lowers the AST to EXECUTABLE rzir into the proven cc-in-c backend
// (ld|opt|ra|cf), replacing L6's byte-walk. A0 = the scalar spine: a `fn main` doing integer let/arith + `println!("{}",x)`
// RUNS == rustc. rzir SSA `vN` per fn (params first, then s9fresh()); `fn main` is the entry (backend turns ret -> exit).
static mut S9NC: i32 = 0;
fn s9fresh() -> i32 { unsafe { let r = S9NC; S9NC = S9NC + 1; r } }
// GLOBAL merge-label counter (C-G2D-LABEL-COLLISION). vreg ids (S9NC) reset per fn, so labels keyed on a value-slot `rv`
// (__Li{rv}/__Lm{rv}) REPEAT across fns; the backend resolves labels GLOBALLY (L6 emits 1048 unique labels, emit_s9 emitted
// 676 duplicates), so a `goto __Li2` jumped to another fn's `label __Li2` -> cross-fn jump -> crash whenever two fns each held
// a merge label. S9LBL is NEVER reset (s9_userfn/s9_main reset S9NC only), so every merge point gets a globally-unique id.
static mut S9LBL: i32 = 0;
fn s9_lbl() -> i32 { unsafe { let r = S9LBL; S9LBL = S9LBL + 1; r } }
// C-S9-I64-LITERAL: a value outside i32 range cannot ride a single `const`. cf lowers `const` as
// `mov qword [slot], imm32` (store_imm), which SIGN-EXTENDS a 32-bit immediate -- so `1000000000000000`
// landed as -1530494976, exactly mod 2^32. Every suffixed literal is such a value, because the lexer packs
// suffix and value into one i64 (`suffix_offset_of` is 1e15..12e15), which is why `1u32`/`1u64`/`1_000u32`
// dumped as garbage while a bare `1` was fine. Mirror L6's materialisation exactly (see its output for
// `let a: i64 = 1000000000000000`): split lo/hi, ZERO-extend each with `conv`, reassemble with shl8/or8.
// Only non-negative k takes this path -- every packed literal is non-negative, and a negative 64-bit
// constant would need the sign handled at the split rather than by conv.
// C-S9-CONST-DUP: the self-image is 294,790 lines of a 300,000-line twin ceiling, and the EMISSION-AUDIT
// prices 36% of the emission as intra-function redundancy. Constants are the bulk of it.
// MEASURED before building, because L6's equivalent had two traps this one does not:
//   (a) L6's `ec` doubled as a fresh MUTABLE SLOT. Here only 2 of 23 `let X = s9const(..)` names are ever a
//       write destination (`nc`, `one`); those two sites keep the uncached emitter.
//   (b) L6's callers read its vreg counter directly around the call. Here S9NC is touched ONLY by s9fresh and
//       the per-function resets, so no counter-burning is needed.
// The cache is STRAIGHT-LINE: reset after every control-flow emission. All control flow in the generated code
// goes through the ten s9emit_* helpers -- the nine inline control-flow println!s are the hand-written runtime
// preamble, which never calls s9const. An earlier attempt reset at only TWO of the THREE function-entry sites,
// so the cache leaked across functions and eliminated 99.6% of constants instead of the safe share.
// SHAPE: an exact structural clone of the WITNESSED memo cache S9CLH/S9CLN/s9clh_has/s9clh_add below (the closure
// dedup table), which self-hosts today. The first attempt invented its own shape -- a [i32;64] value table adjacent
// to an [i64;64] key table, and a negative sentinel returned as the payload -- and BOTH of those are shapes this
// file's corpus never witnessed. Emission came out as `v-1` (the sentinel, as a vreg) and gen2 hung. Statics here
// are uniformly 8-BYTE STRIDED with the declared element type inert (read s9_static_bytes: `n * 8` ignoring the
// element; s9_static_elem_addr: `gref A + i*8`; s9_global_store: store8), so [i32;N] is an untested width. Both
// tables are [i64;N] and no sentinel crosses a return boundary.
static mut S9CK: [i64; 64] = [0; 64];
static mut S9CV: [i64; 64] = [0; 64];
static mut S9CN: i32 = 0;
fn s9ck_reset() { unsafe { S9CN = 0; } }
fn s9ck_has(k: i64) -> bool { unsafe { let mut i = 0; while i < S9CN { if S9CK[i as usize] == k { return true; } i = i + 1; } false } }
fn s9ck_get(k: i64) -> i64 { unsafe { let mut i = 0; while i < S9CN { if S9CK[i as usize] == k { return S9CV[i as usize]; } i = i + 1; } 0 } }
fn s9ck_put(k: i64, v: i64) { unsafe { if S9CN < 64 { S9CK[S9CN as usize] = k; S9CV[S9CN as usize] = v; S9CN = S9CN + 1; } } }
fn s9const(k: i64) -> i32 {
    if s9ck_has(k) { return s9ck_get(k) as i32; }
    let made = s9const_fresh(k);
    s9ck_put(k, made as i64);
    made
}
fn s9const_fresh(k: i64) -> i32 {
    if k > 2147483647 {
        let hi = k / 4294967296;
        let mut lo = k - hi * 4294967296;
        if lo > 2147483647 { lo = lo - 4294967296; }   // low 32 bits as a SIGNED i32 (cf's const takes i32)
        let vlo = s9fresh(); println!("  v{}: i32 = const {}", vlo, lo);
        let vlo8 = s9fresh(); println!("  v{}: i64 = conv v{}", vlo8, vlo);     // conv = zero-extend low 32
        let vhi = s9fresh(); println!("  v{}: i32 = const {}", vhi, hi);
        let vhi8 = s9fresh(); println!("  v{}: i64 = conv v{}", vhi8, vhi);
        let v32 = s9fresh(); println!("  v{}: i32 = const 32", v32);
        let v328 = s9fresh(); println!("  v{}: i64 = conv v{}", v328, v32);
        let vsh = s9fresh(); println!("  v{}: i64 = shl8 v{} v{}", vsh, vhi8, v328);
        let vor = s9fresh(); println!("  v{}: i64 = or8 v{} v{}", vor, vsh, vlo8);
        return vor;
    }
    let v = s9fresh(); println!("  v{}: i32 = const {}", v, k); v
}
// C-S9-I64-LITERAL: an op whose mnemonic ends in `8` is cf's 64-bit variant (cf keys on the trailing byte,
// not on the type annotation -- see its `is64` test -- but emit the honest annotation anyway).
fn s9_op_is64(op: &str) -> bool { let b = op.as_bytes(); if b.len() == 0 { return false; } b[b.len() - 1] == 56 }
fn s9op(op: &str, a: i32, b: i32) -> i32 {
    let v = s9fresh();
    if s9_op_is64(op) { println!("  v{}: i64 = {} v{} v{}", v, op, a, b); } else { println!("  v{}: i32 = {} v{} v{}", v, op, a, b); }
    v
}
fn s9call1(f: &str, a: i32) -> i32 { let v = s9fresh(); println!("  v{}: i32 = call {} v{}", v, f, a); v }
fn s9call2(f: &str, a: i32, b: i32) -> i32 { let v = s9fresh(); println!("  v{}: i32 = call {} v{} v{}", v, f, a, b); v }
// is expr `e` string-typed? (a Str literal, or a Path/Field/init whose inferred type mentions String/str). Mirrors the format
// placeholder dispatch's string check; pure (no emit). Used to route `==`/`!=` to __str_eq (content compare) vs int `eq`/`ne`.
fn s9_is_str_expr(env: &S9Env, e: &ExprK) -> bool {
    match e { ExprK::Str(_s) => true, _ => { let ty = s9_lookup_type(env, &s9_bare_name(e)); let fty = s9_field_type(env, &s9_field_name(e)); let ity = s9_init_ty(env, e); str_contains(&ty, "String") || str_contains(&ty, "str") || str_contains(&fty, "String") || str_contains(&fty, "str") || str_contains(&ity, "String") || str_contains(&ity, "str") } }
}
// B1 control-flow emit leaves (kept OUT of the recursion-cycle fns per S9-F7: the cycle fn holds <=1 value across calls).
fn s9emit_brz(c: i32) { println!("  brz v{}", c); s9ck_reset(); }
fn s9emit_pool() { println!("  pool"); s9ck_reset(); }
// then-arm tail: write the then value into the reserved result slot rv (slot-reuse merge, per the L6 value-if reference),
// skip the else via goto, close the brz block. The merge label id IS rv (fresh per if -> unique), so no lbl is held.
fn s9emit_then_tail(rv: i32, lbl: i32, tv: i32) { s9ck_reset(); println!("  v{}: i64 = or8 v{} v{}", rv, tv, tv); println!("  goto __Li{}", lbl); println!("  pool"); }
// else-arm tail: write the else value into rv, land the merge label (globally-unique `lbl`, not `rv`).
fn s9emit_else_tail(rv: i32, lbl: i32, ev: i32) { s9ck_reset(); println!("  v{}: i64 = or8 v{} v{}", rv, ev, ev); println!("  label __Li{}", lbl); }
// block value (B7 P-LC-BLOCK-TAIL): walk stmts accumulating intra-block `let` binds into a local env, dispatch Semi
// (statements incl assign/break/continue/nested control flow), and return the TAIL Expr's slot (unit -> const 0). Shared
// by if-arms / match-arms / block-exprs. Mirrors s9_main's accumulation but returns the tail. In the recursion cycle,
// but the B2 wholesale-segfault was the Break move-error, not accumulation -- gated to confirm accumulation is sound here.
fn s9_bv(b: &Block, env: &S9Env) -> i32 {
    let mut names: Vec<String> = veccopy(&env.names); let mut slots: Vec<i32> = veccopy_i32(&env.slots); let mut types: Vec<String> = veccopy(&env.types);
    let n = b.stmts.len(); let mut idx = 0;
    for st in b.stmts.iter() {
        match st {
            Stmt::Let(pat, ty, init, _els) => { match init { Some(e) => { let e2 = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: veccopy(&types) }; let v = s9expr(e, &e2); let ar = s9_pat_tuple_arity(pat); if ar >= 0 { let tup = s9_tup4(v); let mut i = 0; while i < ar { let snm = s9_pat_nth_bind(pat, i); let fsi = s9_tup_nth(&tup, i); if snm.len() > 0 { names.push(snm); slots.push(fsi); types.push(String::from("")); } i += 1; } } else { let nm = s9_patname(pat); let tt = s9_pick_ty(&e2, s9_let_ty(ty), e); if nm.len() > 0 { names.push(nm); slots.push(s9_let_copy(e, v)); types.push(tt); } } } None => { let nm = s9_patname(pat); if nm.len() > 0 { let e3 = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: veccopy(&types) }; let tt = s9_defer_pick(s9_let_ty(ty), b, &nm, &e3); names.push(nm); slots.push(s9_defer_slot()); types.push(tt); } } } }
            Stmt::Expr(e) => { let e2 = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: veccopy(&types) }; if idx + 1 == n { return s9expr(e, &e2); } else { s9_stmt(e, &e2); } }
            Stmt::Semi(e) => { let e2 = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: veccopy(&types) }; s9_stmt(e, &e2); }
            _ => {}
        }
        idx = idx + 1;
    }
    s9const(0)
}
// value-if lowering (P-LC-IF). rv reserved FIRST (held across the whole fn, 1 value); cv/tv/ev each short-lived; all emit
// pushed into leaf helpers -> the S9-F7 lean shape (<=1 held value, 3 leaf + 3 cycle calls). els is a single &ExprK
// (a Block for `else {..}` or an If for `else if`); s9expr routes both.
fn s9_if(cond: &ExprK, then: &Block, els: &ExprK, env: &S9Env) -> i32 {
    let rv = s9fresh();
    let lbl = s9_lbl();
    let cv = s9expr(cond, env);
    s9emit_brz(cv);
    let tv = s9_bv(then, env);
    s9emit_then_tail(rv, lbl, tv);
    let ev = s9expr(els, env);
    s9emit_else_tail(rv, lbl, ev);
    rv
}
// B2 loop substrate. S9BV holds the CURRENT loop's break-value slot (saved/restored across nesting); leaf emit helpers.
static mut S9BV: i32 = 0;
// accessor fns for S9BV (the current loop's break-value slot). L6 mis-compiles an INLINE `unsafe { S9BV }` read in a
// distant fn (returns garbage); wrapping the access in a fn -- like s9fresh does for S9NC -- reads it correctly.
fn s9bv_get() -> i32 { unsafe { S9BV } }
fn s9bv_set(v: i32) { unsafe { S9BV = v; } }
fn s9emit_loop_open() { println!("  loop"); s9ck_reset(); }
fn s9emit_loop_close() { println!("  cont"); println!("  jmp"); println!("  endloop"); s9ck_reset(); }
fn s9emit_brk() { println!("  brk"); }
fn s9emit_ret(r: i32) { println!("  ret v{}", r); }
// the callee fn name of a Call (a bare path). Classified through a &ExprK param (match the param directly -- avoid an
// inline `&**callee` deref-match in the s9expr cycle fn, S9-F12 class).
fn s9_callee_name(callee: &ExprK) -> String { match callee { ExprK::Path(p) => dump_path(p), ExprK::QPath(qty, _tr, proj) => s9_qpath_name(qty, proj), _ => String::new() } }
// CT-9: `<S as Tr>::m` and `<S>::m` both denote the inherent `S::m` path that s9_call already resolves. The
// qself TYPE supplies the head; the trait, when written, is resolution information this tier does not need.
// Returns "" when the head is not nameable (`_`, `dyn ..`) -- the caller REFUSES rather than lowering a guess,
// because the silent alternative is what CT-9 caught: a QPath callee scored "" and lowered to a constant 0.
fn s9_qpath_head(qty: &Ty) -> String { match qty { Ty::Path(p) => dump_path(p), _ => String::new() } }
// The CALL mouth needs the same guard as the bare-path mouth: `<_>::default()` reaches s9expr as a Call whose
// callee is a QPath, so without this it re-enters exactly the silent-zero CT-9 caught (s9_call("") -> 0).
fn s9_qpath_guard(callee: &ExprK, cn: &str) {
    if cn.len() > 0 { return; }
    match callee { ExprK::QPath(_q, _t, _p) => { eprintln!("emit_s9: qself call with an uninferable head (`<_>::` / `<dyn ..>::`) is unlowered (CT-9)"); std::process::exit(1); }, _ => {} }
}
fn s9_qpath_name(qty: &Ty, proj: &Vec<PathSeg>) -> String {
    let head = s9_qpath_head(qty);
    if head.len() == 0 { return String::new(); }
    if streq(&head, "_") { return String::new(); }
    let mut out = head;
    // The segment ident goes through a helper rather than being read inline in the format!. Read inline, L6
    // emitted `call S::<garbage bytes>` -- the rustc-built parser was correct, so this is a gen1-only
    // divergence of the same family as the L6 struct-literal field-read defect, not a lowering mistake.
    for seg in proj.iter() { let sn = s9_seg_ident(seg); out = format!("{}::{}", out, sn); }
    out
}
// `format!("{}", ..)` is the copy shape dump_seg already uses and gen1 already compiles. `String::from(&seg.ident[..])`
// segfaulted gen1 outright -- str-slicing a borrowed field is not a lowered shape at this tier.
fn s9_seg_ident(seg: &PathSeg) -> String { format!("{}", seg.ident) }
// ---- SR layout foothold (aggregate layout via the rzir heap; no L6 runtime change). L6 lays an aggregate out as a
// heap-cursor-bump allocation with each field stored at its byte offset (i32 field = 4 bytes); the value is the base
// address; field access = load at base+offset. These are the layout PRIMITIVES the D tier consumes.
// heap-alloc `size` bytes, return the base-address slot (bump __heap_cursor, per the L6 reference).
fn s9_exit(args: &Vec<ExprK>, env: &S9Env) -> i32 {
    // C-SEM0-EXIT-NOOP: std::process::exit(N). emit_s9 had NO abnormal-termination mechanism -- exit, abort,
    // panic!, assert!, unwrap-on-None all silently continued (exit 0), so a SEM0-compiled program could not fail.
    // rzir has a `syscall` OP (how __flush reaches the kernel); `call __syscall` is an rzrs intrinsic L6 lowers
    // and is NOT callable from emitted rzir -- probed, it segfaults. Flush first so buffered stdout is not lost.
    // This cost 559 rzir lines in the self-image and could not land until the gref CSE bought headroom.
    let code = if args.len() > 0 { s9expr(&args[0], env) } else { s9const(0) };
    let fv = s9fresh(); println!("  v{}: i32 = call __flush", fv);
    let nv = s9fresh(); println!("  v{}: i32 = const 60", nv);
    let zv = s9fresh(); println!("  v{}: i32 = const 0", zv);
    let zw = s9fresh(); println!("  v{}: i32 = const 0", zw);
    let rv = s9fresh(); println!("  v{}: i32 = syscall v{} v{} v{} v{}", rv, nv, code, zv, zw);
    rv
}
static mut S9_GHEAP: i32 = 0 - 1;   /* EMISSION-AUDIT: per-fn cached `gref __heap` vreg (-1 = not yet emitted) */
static mut S9_GCUR: i32 = 0 - 1;    /* per-fn cached `gref __heap_cursor` vreg */
// EMISSION-AUDIT-2026-08-25: 95% of all `gref` emissions were intra-function duplicates -- __heap_cursor alone
// appeared 10,746 times (exactly 2 per s9_alloc call, one of them redundant WITHIN the call) and __heap 5,373.
// Globals are address-stable, so one materialization per function suffices. Hoisted to the function ENTRY (after
// the params, which must stay v0..vn-1 in order) so the definition dominates every use -- a lazily-cached vreg
// first defined inside a loop body would not. Two long-lived vregs per function: they are loop-invariant
// addresses a register allocator wants held anyway. See locales/sem0-refuse-cleanly/EMISSION-AUDIT-2026-08-25.md.
fn s9_heap_globals() {
    unsafe {
        let a = s9fresh(); println!("  v{}: i32 = gref __heap", a); S9_GHEAP = a;
        let b = s9fresh(); println!("  v{}: i32 = gref __heap_cursor", b); S9_GCUR = b;
    }
}
fn s9_gheap() -> i32 { unsafe { S9_GHEAP } }
fn s9_gcur() -> i32 { unsafe { S9_GCUR } }
fn s9_alloc(size: i32) -> i32 {
    let cur = s9_gcur();
    let off = s9fresh(); println!("  v{}: i32 = load v{}", off, cur);
    let hb = s9_gheap();
    let base = s9fresh(); println!("  v{}: i32 = add v{} v{}", base, hb, off);
    let sz = s9const(size as i64);
    let nc = s9fresh(); println!("  v{}: i32 = add v{} v{}", nc, off, sz);
    println!("  store v{} v{}", cur, nc);
    base
}
fn s9_store_at(base: i32, offset: i32, val: i32) { let o = s9const(offset as i64); let addr = s9fresh(); println!("  v{}: i32 = add v{} v{}", addr, base, o); println!("  store v{} v{}", addr, val); }
// C-S9AW-AGG-32: s9_store_at/s9_load_at are the 32-bit HEADER accessors -- [len@0, cap@4, dataptr@8] holds only a
// length, a capacity and a sub-4GB pointer, so it stays 4-byte-fielded. s9_slot_* are the 64-bit ELEMENT/PAYLOAD
// accessors: struct fields, tuple elements, enum payloads, Vec/array elements and Box cells, all at an 8-byte stride
// via store8/load8. Both plain `store` and plain `load` are 32-bit, so a slot must use the *8 form on BOTH sides --
// converting one alone silently keeps the truncation. Keeping the two classes as distinct FUNCTIONS (rather than a
// width argument) is deliberate: it makes a header/slot mix-up a name error rather than a silent corruption.
fn s9_slot_store(base: i32, offset: i32, val: i32) { let o = s9const(offset as i64); let addr = s9fresh(); println!("  v{}: i32 = add v{} v{}", addr, base, o); println!("  store8 v{} v{}", addr, val); }
fn s9_slot_load(base: i32, offset: i32) -> i32 { let o = s9const(offset as i64); let addr = s9fresh(); println!("  v{}: i32 = add v{} v{}", addr, base, o); let r = s9fresh(); println!("  v{}: i64 = load8 v{}", r, addr); r }
fn s9_load_at(base: i32, offset: i32) -> i32 { let o = s9const(offset as i64); let addr = s9fresh(); println!("  v{}: i32 = add v{} v{}", addr, base, o); let r = s9fresh(); println!("  v{}: i32 = load v{}", r, addr); r }
// tuple construction (D-tier, SR layout): lower each elem, alloc arity*4 bytes, store elem i at offset i*4, value = base.
// (All-i32 elements: 4-byte stride. Mixed-width/struct-field layout is the next SR rung.)
fn s9_tuple(elems: &Vec<ExprK>, env: &S9Env) -> i32 {
    let mut slots: Vec<i32> = Vec::new(); let mut n = 0;
    for e in elems.iter() { let s = s9expr(e, env); slots.push(s); n += 1; }
    let base = s9_alloc(n * 8);
    let mut off = 0;
    for s in slots.iter() { s9_slot_store(base, off, *s); off += 8; }
    base
}
// tuple-destructure support (C-G2D-TUPLE-DESTRUCTURE): a tuple value is an aggregate block (field i @ i*4, s9_tuple), so
// `let (a,b,c) = t` binds each sub-pattern i to load(tupbase + i*4) -- uniform for scalar (the value) AND aggregate (the
// stored address). `base` is a PARAM (Copy) so the tuple base (a fn-return i32) is read ONCE at the call site: the 4 field
// loads happen inside this fn's budget, carried out as struct FIELDS (Copy on read, move-check exempt). Arity <= 4 covers
// every s1_parser tuple; a load past a shorter tuple's alloc reads within the arena (harmless -- the field is never bound).
struct S9Tup4 { f0: i32, f1: i32, f2: i32, f3: i32 }
fn s9_tup4(base: i32) -> S9Tup4 { let a = s9_slot_load(base, 0); let b = s9_slot_load(base, 8); let c = s9_slot_load(base, 16); let d = s9_slot_load(base, 24); S9Tup4 { f0: a, f1: b, f2: c, f3: d } }
// the i-th field slot of a read tuple (struct-field reads are Copy, so `t` may be read per sub-pattern without a move).
fn s9_tup_nth(t: &S9Tup4, i: i32) -> i32 { if i == 0 { t.f0 } else if i == 1 { t.f1 } else if i == 2 { t.f2 } else { t.f3 } }
// tuple-pattern arity (>=0) or -1 if `pat` is not a Pat::Tuple. Matched through a &Pat PARAM (deref-coercion from the
// Stmt::Let &Box<Pat>) so the classification never uses an inline `match &**pat` (L6 mis-compiles that box-deref, S9-F12).
fn s9_pat_tuple_arity(pat: &Pat) -> i32 { let mut r = -1; match pat { Pat::Tuple(pats) => { r = pats.len() as i32; }, _ => {} } r }
// struct construction (D2, SR layout): alloc nfields*4; each named field stored at its DEF offset (s9_field_off, so
// construction-order != def-order is handled); value = base. `..rest` struct-update is deferred.
fn s9_struct(sname: &str, sfields: &Vec<EField>, env: &S9Env) -> i32 {
    let mut n = 0; for _f in sfields.iter() { n += 1; }
    let base = s9_alloc(n * 8);
    for f in sfields.iter() { let off = s9_field_off_q(env, sname, &f.name); let v = s9expr(&f.expr, env); s9_slot_store(base, off, v); }   // SR-2: place each field at its Struct#field offset (C-G2D-FIELD-COLLISION), not the bare last-writer offset
    base
}
// method call `recv.foo(args)` (C5-1): desugar to a direct call with the receiver as arg 0, per the L6 reference. The
// method fn is emitted as `fn <foo>` (unmangled -- C5-1 assumes UNIQUE method names across impls, like SR-2's unique
// field names; Type-mangling + binding-type resolution is C5-1b when a collision appears). recv + explicit args each
// lowered once (S9-F13/F15); fixed-arity suffix (S9-F14); recv is arg 0 so <=5 explicit args before the cf 6-arg cap.
// C-S9-ITER-SUM: `v.iter().sum()` is the most common adapter terminal in the corpus (24 occurrences, 19 files;
// 12 probes use it as their ONLY adapter and are OK under L6 but MISCOMP under SEM0 -- hoist regressions).
// Lowered directly as a pointer walk over the [len@0, cap@4, dataptr@8] layout, modelled on s9_for_iter's loop
// rather than by building an iterator object: there is no iterator representation in this tier, and the
// terminal is what programs actually use.
// Only the un-adapted form is recognised. `v.iter().map(..).sum()` is NOT this shape and must keep its current
// behaviour rather than silently summing the wrong sequence.
fn s9_iter_sum(recv: &ExprK, env: &S9Env) -> i32 {
    let vbase = s9expr(recv, env);
    let it = s9_vec_iter_bounds(vbase);
    let esl = s9fresh(); s9emit_setslot(esl, it.dptr);
    let zero = s9const(0);
    let acc = s9fresh(); s9emit_setslot(acc, zero);
    s9emit_loop_open();
    let g = s9op("lt", esl, it.end);
    s9emit_brz(g);
    let ev = s9_slot_load(esl, 0);
    let nv = s9op("add", acc, ev);
    s9emit_setslot(acc, nv);
    s9emit_cont();
    s9emit_incr4(esl);
    s9emit_jmpend();
    acc
}
// is this method call exactly `<expr>.iter()`? (the receiver of a terminal we lower directly)
// unwrap `<v>.iter()` to <v> and sum it
fn s9_iter_sum_of(itercall: &ExprK, env: &S9Env) -> i32 { match itercall { ExprK::Method(r, _n, _t, _a) => s9_iter_sum(r, env), _ => s9const(0) } }
fn s9_is_iter_call(e: &ExprK) -> bool { match e { ExprK::Method(_r, n, _t, a) => { if a.len() > 0 { return false; } streq(n, "iter") } _ => false } }
// C-S9-VEC-POP: `v.pop()` returned nothing and removed nothing -- `match v.pop()` gave 0 with the length
// unchanged, while L6 is correct. It is the dominant `while let` scrutinee in the corpus (10 of 14 files use
// the pop family), so `while let` cannot pay off until pop works; this rung does pop, which is independently
// verifiable through `match v.pop()`.
// Emits the value-if shape: when len != 0, decrement the length in place, read the last element at
// dataptr + (len-1)*8, and wrap it in a Some cell; otherwise yield a None cell.
fn s9_some_of(env: &S9Env, v: i32) -> i32 {
    let tag = s9_enum_tag(env, "Some");
    let base = s9_alloc(16);
    let tv = s9const(tag as i64);
    s9_store_at(base, 0, tv);
    s9_slot_store(base, 8, v);
    base
}
fn s9_vec_pop(base: i32, env: &S9Env) -> i32 {
    let rv = s9fresh();
    let lbl = s9_lbl();
    let len = s9_load_at(base, 0);
    let zero = s9const(0);
    let nz = s9op("ne", len, zero);
    s9emit_brz(nz);
    let one = s9const(1);
    let nl = s9op("sub", len, one);
    s9_store_at(base, 0, nl);
    let dp = s9_load_at(base, 8);
    let eight = s9const(8);
    let off = s9op("mul", nl, eight);
    let addr = s9op("add", dp, off);
    let val = s9_slot_load(addr, 0);
    let some = s9_some_of(env, val);
    s9emit_then_tail(rv, lbl, some);
    let none = s9_enum_unit(env, "None");
    s9emit_else_tail(rv, lbl, none);
    rv
}
// L6 LANDMINE, hit on this rung and worth the three builds it cost: a COMMENT INTERPOSED IN THE ELSE-IF CHAIN
// below -- four comment lines between the `pop` arm and `else { 0 }` -- makes an L6-built s1_parser stop
// recognising `pop`. It broke `g2 != g3` inside `parse_ty_inner`, at a `.pop()` call the change never touched,
// and it survived three wrong hypotheses (chain length, an early return, a value-if at the call site), each of
// which cost a full build. The file already records "comments go on their own line"; this sharpens it to: a
// comment must not sit INSIDE an expression chain. Keep annotation above the function.
//
// P-TRY fallout, which is what the new kinds are for: closing `?` made two previously-unreached methods
// reachable, and an unrecognised method lowers to `call <name>`, a call to a function that does not exist.
// `ctrlflow_qmark_none` had been scored OK only because `?` was unlowered, so `v.first()?` never ran; the
// moment `?` worked it segfaulted. Both new kinds are the same value-if shape as pop.
fn s9_method_kind(name: &str) -> i32 {
    if streq(name, "push") { 1 } else if streq(name, "push_str") { 2 } else if streq(name, "len") { 3 }
    else if streq(name, "clone") { 4 } else if streq(name, "is_some") { 5 } else if streq(name, "is_none") { 6 }
    else if streq(name, "unwrap") { 7 } else if streq(name, "as_bytes") { 8 } else if streq(name, "pop") { 9 } else { 0 }
}
// L6 LANDMINE, hit and measured on this rung: the two new kinds were first appended to the else-if chain above,
// taking it from 9 arms to 11. L6 DROPS ARMS past ~10, so an L6-built s1_parser stopped recognising `pop` --
// and since a SEM0-built one has no such defect, `g2 != g3` broke on the tower's own source, at a call site
// nothing about this change should have touched. The chain stays at 9; new kinds go in this separate function.
fn s9_method_kind2(name: &str) -> i32 {
    if streq(name, "first") { return 10; }
    if streq(name, "unwrap_or") { return 11; }
    0
}
// `v.first()` -> Some(v[0]) when the length is non-zero, else None.
fn s9_vec_first(base: i32, env: &S9Env) -> i32 {
    let rv = s9fresh();
    let lbl = s9_lbl();
    let len = s9_load_at(base, 0);
    let zero = s9const(0);
    let nz = s9op("ne", len, zero);
    s9emit_brz(nz);
    let dp = s9_load_at(base, 8);
    let val = s9_slot_load(dp, 0);
    let some = s9_some_of(env, val);
    s9emit_then_tail(rv, lbl, some);
    let none = s9_enum_unit(env, "None");
    s9emit_else_tail(rv, lbl, none);
    rv
}
// `o.unwrap_or(d)` -> the payload when the tag is the Some tag, else `d`. Option only (Some=1); a Result
// receiver would need the Ok=0 tag, and `unwrap_or` on Result is not yet distinguished -- recorded, not built.
// The tag is a PARAMETER now, not the constant 1: Some=1, Ok=0. Everything else about the lowering is
// identical, which is why this was one constant rather than a second builtin.
fn s9_unwrap_tag(env: &S9Env, recv: &ExprK) -> i32 {
    let rty = s9_lookup_type(env, &s9_bare_name(recv));
    if str_contains(&rty, "Result") { return 0; }
    let rit = s9_init_ty(env, recv);
    if str_contains(&rit, "Result") { return 0; }
    1
}
fn s9_opt_unwrap_or(base: i32, dflt: i32) -> i32 { s9_opt_unwrap_or_t(base, dflt, 1) }
fn s9_opt_unwrap_or_t(base: i32, dflt: i32, want: i32) -> i32 {
    let rv = s9fresh();
    let lbl = s9_lbl();
    let tag = s9_load_at(base, 0);
    let one = s9const(want as i64);
    let is_some = s9op("eq", tag, one);
    s9emit_brz(is_some);
    let pay = s9_slot_load(base, 8);
    s9emit_then_tail(rv, lbl, pay);
    s9emit_else_tail(rv, lbl, dflt);
    rv
}
// IDENTITY string methods in the flat model (a &str IS the String inline [len@0,bytes@4+] layout): `.to_string()`/`.to_owned()`
// /`.as_str()`/`.as_ref()` return the receiver value unchanged. Kept as a SEPARATE small predicate + an early return in
// s9_method (NOT added to s9_method_kind's if-else chain, which L6 mis-compiles once it grows past ~10 arms -- that lengthening
// made as_bytes fall through to a generic call in the L6 build only, breaking self-emit byte-identity). C-G2D-NONMAIN-AGGREGATE-METHOD.
fn s9_str_ident_m(name: &str) -> bool { streq(name, "to_string") || streq(name, "to_owned") || streq(name, "as_str") || streq(name, "as_ref") }
// The variable name behind `&mut X` / `&X` / `X` (the lvalue target of read_to_string).
fn s9_ref_target_name(e: &ExprK) -> String {
    match e {
        ExprK::Ref(_m, inner) => s9_bare_name(inner),
        ExprK::RawRef(_m, inner) => s9_bare_name(inner),
        ExprK::Path(p) => dump_path(p),
        _ => String::from(""),
    }
}
fn s9_method(recv: &ExprK, name: &str, args: &Vec<ExprK>, env: &S9Env) -> i32 {
    // C-SCI-READ-TO-STRING-INERT fix: s1_parser's fixed stdin idiom `std::io::stdin().read_to_string(&mut X)` must lower to
    // a syscall(0) read into a large inline String buffer ([len@0, bytes@4+]), rebinding X to it (mirrors L6 STDIN-READ).
    // Without this, emit_s9 emitted inert `call std::io::stdin` / `call read_to_string` (no backend intrinsic, no emitted fn)
    // so X stayed empty and the self-compiled SEM0 was INERT (the whole reason gen2 produced no output). recv (stdin()) has
    // no side effect and is intentionally NOT lowered here (lowering it would emit a dead call). The 52 sem witnesses never
    // read stdin, so this path is exercised only by the self-image + any //@S9 stdin-reading program.
    if streq(name, "read_to_string") {
        let mut tgt = String::from("");
        for a in args.iter() { tgt = s9_ref_target_name(a); break; }
        // C-G2D-STR-GROW: header-model String [len@0, cap@4, dataptr@8]; read the 1.5MB source directly into dataptr's buffer.
        let hdr = s9_alloc(3 * 4);
        let buf = s9_alloc(375000 * 4);                        // ~1.5MB (matches L6's SRC buffer ceiling)
        let capc = s9const(1500000); s9_store_at(hdr, 4, capc);
        s9_store_at(hdr, 8, buf);
        let nr = s9const(0); let fd = s9const(0); let cnt = s9const(1500000);
        let n = s9fresh(); println!("  v{}: i32 = syscall v{} v{} v{} v{}", n, nr, fd, buf, cnt);   // read(0, buf, 1.5M)
        s9_store_at(hdr, 0, n);                                // String length = bytes read
        if tgt.len() > 0 { let slot = s9lookup(env, &tgt); let ra = s9fresh(); println!("  v{}: i32 = ref v{}", ra, slot); println!("  store v{} v{}", ra, hdr); }
        return hdr;                                            // the Result; a trailing .unwrap() is identity on the header
    }
    if s9_str_ident_m(name) { return s9expr(recv, env); }   // to_string/to_owned/as_str/as_ref: identity (avoids an unresolved call)
    // L6 LANDMINE, measured twice on this rung. The new kinds were first APPENDED to s9_method_kind's else-if
    // chain (9 arms -> 11); L6 drops arms past ~10, so an L6-built parser stopped recognising `pop`. Moving them
    // into a helper called from the TOP of s9_method_kind did not fix it either -- an early `return` in front of
    // that expression-position chain is mis-compiled the same way. Both broke `g2 != g3` in `parse_ty_inner`, at
    // a `.pop()` call this change should never have touched. s9_method_kind is therefore left EXACTLY as it was
    // and the new kinds are consulted here, at the call site, where no chain is extended and no return is added.
    let mut kind = s9_method_kind(name);
    // A BUILTIN METHOD NAME MUST NOT SHADOW A USER-DEFINED ONE, and the two names added this rung both did.
    // Measured: `impl Pair<T> { fn first(&self) -> T }` had the builtin Vec-first lowering applied to a struct
    // receiver (segfault), and `m.get(&99).copied().unwrap_or(-1)` had the Option lowering applied to a receiver
    // that is not an Option (segfault). Both probes had been OK. The crate's own fn registry decides: if the
    // program defines a method with this name, the program wins. `unwrap_or` additionally requires the receiver
    // to be a known Option, since its lowering reads a tag and a payload slot that only an Option has.
    if kind == 0 {
        let k2 = s9_method_kind2(name);
        let defined = s9_fn_ret_type(env, name);
        if defined.len() == 0 {
            if k2 == 11 {
                let rty = s9_lookup_type(env, &s9_bare_name(recv));
                let rit = s9_init_ty(env, recv);
                if str_contains(&rty, "Option") || str_contains(&rit, "Option") || str_contains(&rty, "Result") || str_contains(&rit, "Result") { kind = k2; }
            } else { kind = k2; }
        }
    }
    let rslot = s9expr(recv, env);
    match kind {
        1 => s9_push_dispatch(rslot, args, env),
        2 => s9_str_push(rslot, args, env),
        3 => s9_vec_len(rslot),
        4 => rslot,
        5 => { let t = s9_load_at(rslot, 0); let z = s9const(0); s9op("ne", t, z) }
        6 => { let t = s9_load_at(rslot, 0); let z = s9const(0); s9op("eq", t, z) }
        7 => s9_slot_load(rslot, 8),   // C-S9AW-AGG-32: Option payload moved 4 -> 8 with the widened enum slot
        8 => rslot,
        9 => s9_vec_pop(rslot, env),
        10 => s9_vec_first(rslot, env),
        11 => { let mut d = 0; for a in args.iter() { d = s9expr(a, env); } let wt = s9_unwrap_tag(env, recv); s9_opt_unwrap_or_t(rslot, d, wt) },
        _ => { let tgt = s9_method_target(recv, name, env); s9_method_generic(rslot, &tgt, args, env) },
    }
}
// generic method call: emit `call <name> v{recv} v{args}`. CL-A8 (locales/call-abi-arity): the arity was FIXED at
// five arguments plus the receiver, and the comment here said why -- "cf 6-arg cap". That cap is gone: cf now
// passes arguments 7..N on the stack (CL-A1), so the front end no longer has to truncate. Arguments past the
// fifth were still EVALUATED for their side effects and then silently dropped from the call, which is the worst
// shape a limit can take.
// The two-pass form is s9_array's, which already holds a Vec<i32> across s9expr in a loop and self-hosts: pass
// one evaluates every argument, pass two builds the operand text. Nothing but the Vec is carried across the
// recursive call, which is what S9-F7 asks of a function inside the cycle.
fn s9_method_generic(rslot: i32, name: &str, args: &Vec<ExprK>, env: &S9Env) -> i32 {
    let mut slots: Vec<i32> = Vec::new();
    for a in args.iter() { let s = s9expr(a, env); slots.push(s); }
    let r = s9fresh();
    let mut suf = format!(" v{}", rslot);
    for s in slots.iter() { suf.push_str(&format!(" v{}", *s)); }
    println!("  v{}: i32 = call {}{}", r, name, suf);
    r
}
// emit a call. Args lowered to slots; the call line is emitted at FIXED arity (<=6, the cf register cap) via a plain
// println! -- NOT a format!/push_str loop over the slots, which L6 mis-compiles to a runtime crash (S9-F13, S5-F/G
// String-in-loop class). >6 args is C2 CHUNK (deferred; the 6-arg form is used as a placeholder cap here).
// String::from(x) is IDENTITY in the flat model (a &str/String share the inline [len@0, bytes@4+] layout), so it returns the
// arg's value directly rather than an unresolved `call String::from` (no backend fn -> corrupts a real fn frame -> segv,
// C-G2D-NONMAIN-AGGREGATE-METHOD). Single arg; the loop mirrors the ExprK::Ret evaluate-args idiom (L6-safe).
fn s9_str_ident(args: &Vec<ExprK>, env: &S9Env) -> i32 { let mut r = 0; for a in args.iter() { r = s9expr(a, env); } r }
// CL-A8: same six-argument truncation as s9_method_generic above, same removal, same two-pass shape.
// C-S9-UFCS-SELF-ARG: `S::m(&s)` for a `&self` method emitted `call S::m` while the definition was `fn m`.
// The resolved name is chosen BEFORE the args are lowered so that the emitted call and the emitted definition
// cannot disagree.
// `Self::a()` inside `impl S` means `S::a()`. The owning type rides in the env as `$Self` rather than a global,
// because every static at this tier is a scalar or an int array -- there is nowhere to hold a name. Per-impl,
// so two impls that both define `a` each resolve to their own (probed: `S::b()`/`T::b()` gave `00` before).
fn s9_self_head(nm: &str) -> bool { let b = nm.as_bytes(); if b.len() < 6 { return false; } b[0] == 83 && b[1] == 101 && b[2] == 108 && b[3] == 102 && b[4] == 58 && b[5] == 58 }
fn s9_self_ty(env: &S9Env) -> String { let key = String::from("$Self"); if s9_field_has(env, &key) { return s9_field_type(env, &key); } String::new() }
fn s9_env_with_self(env: &S9Env, tn: &str) -> S9Env {
    let mut e = s9_env_copy(env);
    e.fnames.push(String::from("$Self")); e.foffs.push(0); e.ftypes.push(String::from(tn));
    e
}
fn s9_desugar_self(nm: &str, env: &S9Env) -> String {
    if s9_self_head(nm) == false { return String::from(nm); }
    let ty = s9_self_ty(env);
    if ty.len() == 0 { return String::from(nm); }
    let ls = s9_last_seg_str(nm);
    format!("{}::{}", ty, ls)
}
// Split in two: the desugared name is a CALL RESULT, and binding it here and then using it three times (the
// qualified test, the last-segment scan, and the return) is the C-L6-CALLRESULT-3USE landmine -- L6 rejected
// the file with `use of moved value` and stopped emitting inside this very function. As a PARAMETER the same
// value is unrestricted, so the three uses live one frame down.
fn s9_call_bare(nm: &str, env: &S9Env) -> String {
    if s9_qualified(nm) == false { return String::from(nm); }
    let ls = s9_last_seg_str(nm);
    let key = format!("$m#{}", ls);
    if s9_method_dup(env, &ls) { return String::from(nm); }
    if s9_reg_known(env, &key) { return ls; }
    String::from(nm)
}
fn s9_call_target(nm0: &str, env: &S9Env) -> String {
    let nm = s9_desugar_self(nm0, env);
    s9_call_bare(&nm, env)
}
fn s9_call(nm: &str, args: &Vec<ExprK>, env: &S9Env) -> i32 {
    let tgt = s9_call_target(nm, env);
    let mut slots: Vec<i32> = Vec::new();
    for a in args.iter() { let s = s9expr(a, env); slots.push(s); }
    let r = s9fresh();
    let mut suf = String::new();
    for s in slots.iter() { suf.push_str(&format!(" v{}", *s)); }
    println!("  v{}: i32 = call {}{}", r, &tgt, suf);
    r
}
// break-with-value: write the value into the enclosing loop's break-value slot (S9BV). Leaf helper so the Break arm's
// loop body is just two calls (L6's move-checker rejects an unsafe-read + call-result folded into one loop body).
fn s9emit_break_val(bv: i32, rv: i32) { println!("  v{}: i64 = or8 v{} v{}", bv, rv, rv); }
fn s9emit_cnt() { println!("  cnt"); }
// mutable local write: `x = rhs` -> materialize the address of x's slot (ref) then store in place (the value slot is a
// mutable cell, per the L6 reference). Kept lean: rv + slot short-lived.
// store `val` into a FIELD lvalue `fe.fname` (mirrors the ExprK::Field READ path): base = the aggregate address, off = the
// named-field byte offset (or `n*4` for a numeric tuple field), then store. Without this, `self.field = x` / `self.field OP= x`
// fell to the `_ => {}` default and were NO-OPS -- so the parser's cursor `self.pos += 1` (bump) never advanced -> parse_krate
// looped forever (C-G2D-FIELD-ASSIGN, the gen2 parse HANG). 43 `self.field OP=` sites depend on this.
// C-G2D-INDEX-ASSIGN-NOOP: s9_assign matched Path and Field and dropped EVERYTHING else via `_ => {}`, so an
// INDEX lhs (`v[i] = e`) lowered to nothing at all -- the rhs was evaluated and the store silently discarded.
// Same shape as C-SCI-ASSIGNOP-NOOP (`+=` fell to s9expr's `_ => s9const(0)`): a missing arm reads as a no-op,
// not as an error, so it surfaces far away as corrupt state. Mirrors s9_index's READ path exactly -- byte vs
// word stride chosen by the same element-type test -- storing instead of loading.
fn s9_index_store(arr: &ExprK, idx: &ExprK, val: i32, env: &S9Env) {
    let base = s9expr(arr, env);
    let iv = s9expr(idx, env);
    let aty = s9_lookup_type(env, &s9_bare_name(arr));
    let dp = s9_load_at(base, 8);   // dataptr: bytes/words live at base+8 (C-G2D-STR-GROW header)
    if str_contains(&aty, "bytes") || str_contains(&aty, "u8") {
        let addr = s9op("add", dp, iv);
        println!("  storeb v{} v{}", addr, val);
    } else {
        let f1 = s9const(8); let ioff = s9op("mul", iv, f1);
        let addr = s9op("add", dp, ioff);
        println!("  store8 v{} v{}", addr, val);
    }
}
fn s9_field_store(fe: &ExprK, fname: &str, val: i32, env: &S9Env) {
    if is_all_digits(fname) { let base = s9expr(fe, env); s9_slot_store(base, (str_to_i64(fname) as i32) * 8, val); }
    else { let off = s9_field_off_r(env, fe, fname); let base = s9expr(fe, env); s9_slot_store(base, off, val); }   // SR-2: store at the receiver-qualified offset (C-G2D-FIELD-COLLISION)
}
fn s9_is_deref(e: &ExprK) -> bool { match e { ExprK::Deref(_d) => true, _ => false } }
fn s9_assign_deref(lhs: &ExprK, rhs: &ExprK, env: &S9Env) {
    match lhs { ExprK::Deref(dinner) => { let da = s9expr(dinner, env); let dv = s9expr(rhs, env); println!("  store8 v{} v{}", da, dv); } _ => {} }
}
fn s9_assignop_deref(op: &str, lhs: &ExprK, rhs: &ExprK, env: &S9Env) {
    match lhs { ExprK::Deref(dinner) => { let da = s9expr(dinner, env); s9_assignop_at(op, da, rhs, env); } _ => {} }
}
fn s9_assignop_at(op: &str, da: i32, rhs: &ExprK, env: &S9Env) {
    let cur = s9_slot_load(da, 0);
    let rv2 = s9_arith_val(rhs, env);
    let base = s9_assignop_base(op);
    let nv2 = s9op(&base, cur, rv2);
    println!("  store8 v{} v{}", da, nv2);
}
fn s9_assign(lhs: &ExprK, rhs: &ExprK, env: &S9Env) {
    if s9_is_deref(lhs) { s9_assign_deref(lhs, rhs, env); return; }
    let rv = s9expr(rhs, env);
    match lhs { ExprK::Path(p) => { let nm = dump_path(p); if s9_is_static(env, &nm) { s9_global_store(&nm, rv); } else { let slot = s9lookup(env, &nm); let a = s9fresh(); println!("  v{}: i32 = ref v{}", a, slot); println!("  store8 v{} v{}", a, rv); } } ExprK::Field(fe, fname) => { s9_field_store(fe, fname, rv, env); } ExprK::Index(arr, idx) => { let an = s9_bare_name(arr); if s9_static_arr(env, &an) { let iv = s9expr(idx, env); let ad = s9_static_elem_addr(&an, iv); s9_slot_store(ad, 0, rv); } else { s9_index_store(arr, idx, rv, env); } } _ => {} }
}
// the base binop for a compound-assign op name (mirrors s9_binop's Bin mapping: Rem->mod, Shr->shu).
fn s9_assignop_base(op: &str) -> String {
    // must mirror s9_binop's 64-bit forms exactly -- `x OP= y` is `x = x OP y`, so a 32-bit mnemonic here truncates
    // the same value the binary form preserves. (They were out of step for one build; the += witness caught it.)
    if streq(op, "AddAssign") { return String::from("add8"); } if streq(op, "SubAssign") { return String::from("sub8"); }
    if streq(op, "MulAssign") { return String::from("mul8"); } if streq(op, "DivAssign") { return String::from("div8"); }
    if streq(op, "RemAssign") { return String::from("mod8"); }
    if streq(op, "BitAndAssign") { return String::from("and8"); } if streq(op, "BitOrAssign") { return String::from("or8"); }
    if streq(op, "BitXorAssign") { return String::from("xor8"); }
    if streq(op, "ShlAssign") { return String::from("shl8"); } if streq(op, "ShrAssign") { return String::from("shu8"); }
    String::from("add8")
}
// `x OP= y` -> `x = x OP y`. emit_s9 had NO AssignOp arm (it fell to s9expr's `_ => s9const(0)` default), so every `+=`
// was a silent no-op -> any while-loop that advances via `+=` never terminated. s1_parser uses `+=` 151x incl the
// tokenizer's core `i += 1` advance, so the self-compiled SEM0 hung in its lexer (C-SCI-GEN2-PROCESS-CRASH). Path lhs only
// (the dominant case incl the lexer/parser cursors); a non-Path compound target would surface as the next threshold.
// C-S9-ITER-SCALAR-NO-AUTODEREF: `for x in a.iter()` binds a SCALAR element to the slot ADDRESS (by design -- see
// s9_iter_bind), so the body must write `*x`. rustc accepts BOTH `s += *x` and `s += x`, the latter through
// AddAssign<&i32>. SEM0 had no such coercion, so `s += x` added the ADDRESS: measured 72351828 instead of 6, on
// arrays AND Vecs, while `s += *x` and index-based loops were correct and L6 got all of them right.
// Auto-deref is applied ONLY in arithmetic operand position. The Path arm still yields the address, so an
// explicit `*x` keeps working through the ordinary Deref path -- both spellings now agree with rustc.
fn s9_arith_val(e: &ExprK, env: &S9Env) -> i32 {
    // L6 LANDMINE: an i32 must not be used on two paths ("use of moved value" -- it refused the whole file).
    // The slot is therefore lowered separately in each branch; s9expr's emission side effect is fine because
    // exactly one branch runs.
    // Only a bare Path can be an iter binding, so match the VARIANT first: this predicate runs on every
    // arithmetic operand, and doing the name/type lookup unconditionally allocates a String per operand. That
    // tripped the arena gate -- the third time a per-call-site predicate has done so (bool Display, the closure
    // liftability check, and now this). Literals and compound expressions never reach the lookup.
    match e {
        ExprK::Path(_p) => {
            let nm = s9_bare_name(e);
            let ty = s9_lookup_type(env, &nm);
            if streq(&ty, "iterscalar") { let v = s9expr(e, env); return s9_slot_load(v, 0); }
            s9expr(e, env)
        }
        _ => s9expr(e, env)
    }
}
fn s9_assignop(op: &str, lhs: &ExprK, rhs: &ExprK, env: &S9Env) {
    if s9_is_deref(lhs) { s9_assignop_deref(op, lhs, rhs, env); return; }
    let lv = s9expr(lhs, env);
    let rv = s9_arith_val(rhs, env);
    let base = s9_assignop_base(op);
    let nv = s9op(&base, lv, rv);
    match lhs { ExprK::Path(p) => { let nm = dump_path(p); let slot = s9lookup(env, &nm); let a = s9fresh(); println!("  v{}: i32 = ref v{}", a, slot); println!("  store8 v{} v{}", a, nv); } ExprK::Field(fe, fname) => { s9_field_store(fe, fname, nv, env); } _ => {} }
}
// walk a statement-context block (loop body / stmt-if then), dispatching each stmt through s9_stmt with the ENCLOSING
// env. Kept lean per S9-F7: this fn is IN the recursion cycle, so it holds NOTHING across the call (no per-stmt env
// rebuild). Intra-block `let` binds are deferred to B7 (block-tail); B2 loop/if bodies reference outer locals only.
fn s9_walk(b: &Block, env: &S9Env) {
    // loop/if body walker: accumulate intra-body `let` binds into a local env (like s9_bv/s9_main) so a `let r = ..` in a
    // loop body is bound for the statements that follow it (was dropped -> the name resolved to slot 0). env-accumulation
    // via veccopy+push is sound in the recursion cycle (S9-F7). Dispatch-only: no tail value returned.
    let mut names: Vec<String> = veccopy(&env.names); let mut slots: Vec<i32> = veccopy_i32(&env.slots); let mut types: Vec<String> = veccopy(&env.types);
    for st in b.stmts.iter() {
        match st {
            Stmt::Let(pat, ty, init, _els) => { match init { Some(e) => { let e2 = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: veccopy(&types) }; let v = s9expr(e, &e2); let ar = s9_pat_tuple_arity(pat); if ar >= 0 { let tup = s9_tup4(v); let mut i = 0; while i < ar { let snm = s9_pat_nth_bind(pat, i); let fsi = s9_tup_nth(&tup, i); if snm.len() > 0 { names.push(snm); slots.push(fsi); types.push(String::from("")); } i += 1; } } else { let nm = s9_patname(pat); let tt = s9_pick_ty(&e2, s9_let_ty(ty), e); if nm.len() > 0 { names.push(nm); slots.push(s9_let_copy(e, v)); types.push(tt); } } } None => { let nm = s9_patname(pat); if nm.len() > 0 { let e3 = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: veccopy(&types) }; let tt = s9_defer_pick(s9_let_ty(ty), b, &nm, &e3); names.push(nm); slots.push(s9_defer_slot()); types.push(tt); } } } }
            Stmt::Semi(e) => { let e2 = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: veccopy(&types) }; s9_stmt(e, &e2); }
            Stmt::Expr(e) => { let e2 = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: veccopy(&types) }; s9_stmt(e, &e2); }
            _ => {}
        }
    }
}
// `loop { body }` value: reserve break-value slot bv (the ONLY value held across s9_walk, per S9-F7); loop skeleton
// (loop / const-1 guard / brz / body / cont / jmp / endloop) matching the L6 reference; break writes bv, read bv after.
fn s9_loop(b: &Block, env: &S9Env) -> i32 {
    let bv = s9fresh();
    s9bv_set(bv);
    s9emit_loop_open();
    let c = s9const(1); s9emit_brz(c);
    s9_walk(b, env);
    s9emit_loop_close();
    bv
}
// `while C { B }` = the loop skeleton with the REAL condition C in the guard slot (loop / <C> / brz C / <B> /
// cont/jmp/endloop). Unit value (no break-value slot -- Rust forbids `break EXPR` in a while). Lean: nothing held
// across s9_walk; cv short-lived.
fn s9_while(cond: &ExprK, b: &Block, env: &S9Env) -> i32 {
    s9emit_loop_open();
    let cv = s9expr(cond, env);
    s9emit_brz(cv);
    s9_walk(b, env);
    s9emit_loop_close();
    s9const(0)
}
// for-range emit helpers (leaves). The increment sits BETWEEN cont and jmp (so `continue` still increments), and reuses
// the induction slot via `or` (per the L6 reference), not ref+store.
fn s9emit_cont() { println!("  cont"); s9ck_reset(); }
fn s9emit_jmpend() { println!("  jmp"); println!("  endloop"); s9ck_reset(); }
fn s9emit_incr(islot: i32) { let one = s9fresh(); println!("  v{}: i32 = const 1", one); let sum = s9fresh(); println!("  v{}: i64 = add8 v{} v{}", sum, islot, one); println!("  v{}: i64 = or8 v{} v{}", islot, sum, sum); }
fn s9emit_incr4(islot: i32) { let four = s9fresh(); println!("  v{}: i32 = const 8", four); let sum = s9fresh(); println!("  v{}: i64 = add8 v{} v{}", sum, islot, four); println!("  v{}: i64 = or8 v{} v{}", islot, sum, sum); }
fn s9emit_setslot(slot: i32, val: i32) { println!("  v{}: i64 = or8 v{} v{}", slot, val, val); }
// `let x = INIT`: if INIT is a bare lvalue (a Path var or a `x.f` field), s9expr returns the EXISTING slot, so `let lo = i`
// would ALIAS i's vreg -- a later `i += 1` then also changes lo (C-G2D-LET-ALIAS: `let lo=i; while ..i+=1; &src[lo..i]` ->
// empty slice -> lex keyword recognition failed in the self-image). Copy the value into a FRESH vreg so the binding is
// independent. For an aggregate lvalue the or-copy duplicates the ADDRESS (still the same aggregate -- correct: the source is
// moved). A computed init (call/binop/index/..) already yields a fresh vreg, so no copy is needed there.
// or-copy a bare-lvalue let-init into a FRESH slot so a later mutation of the source var does not also mutate the binding
// (C-G2D-LET-ALIAS). A Cast is identity-lowered (`i as i64` returns i's slot), so `let ts = i as i64` ALIASED i -- the lexer's
// `let ts = i as i64; ...advance i...; offs.push(ts)` then pushed i's ADVANCED value (offs[0]=6 for "struct", not 0). Recurse
// through the Cast so its inner lvalue is copied (call-boundary box deref is the sound idiom, not an inline `match &**box`).
fn s9_let_copy(e: &ExprK, v: i32) -> i32 { match e { ExprK::Path(_p) => { let f = s9fresh(); s9emit_setslot(f, v); f } ExprK::Field(_o, _n) => { let f = s9fresh(); s9emit_setslot(f, v); f } ExprK::Cast(inner, _t) => s9_let_copy(inner, v), _ => v } }
// the .iter() bounds of a Vec, carried as FIELDS (dptr, end) so s9_for_iter reads fields not locals (C-ESC-FIELD-ROUTE): end
// = dataptr + len*4. Computed in this small fn's budget.
struct S9VecIter { dptr: i32, end: i32 }
// `vbase` is a PARAM (Copy) so it can be read 3x: an i32 from a fn RETURN (s9expr/s9_load_at/s9op) is mis-treated by L6's
// move-checker as NON-Copy -> usable ONCE (the "i32 not Copy after return" quirk); passing it as a param restores Copy.
// Each fn-return local (len/dp/ed/bytes/end) is used exactly once for the same reason. C-ESC-FIELD-ROUTE, refined.
fn s9_vec_iter_bounds(vbase: i32) -> S9VecIter { let len = s9_load_at(vbase, 0); let dp = s9_load_at(vbase, 8); let ed = s9_load_at(vbase, 8); let f4 = s9const(8); let bytes = s9op("mul", len, f4); let end = s9op("add", ed, bytes); S9VecIter { dptr: dp, end: end } }
// `for i in a..b { B }` desugar (P-LC-FOR-RANGE): hoist a,b; materialize induction slot i=a (or-copy); loop / i<b (le for
// a..=b) guard / brz / body(i bound) / cont / i=i+1 (slot-reuse) / jmp / endloop. Unit value. Non-range iterators (real
// Iterator) are E-tier (S8), deferred. Lean per S9-F9: only isl held across s9_walk (benv is the call arg).
// P-RANGE-EXPR (substrate-pipeline-oracle.md rank 3, 304 occurrences / 118 files). THE ONLY MECHANISM PIPELINE
// on the board, and the probe that classified it is one line: `for i in 0..4` sums to 6, but
// `let r = 0..4; for i in r` gives 0 -- a range existed ONLY as an inline form at the for-loop mouth and had no
// representation as a VALUE.
//
// The dominant real shape is a range PASSED AS AN ARGUMENT -- `set.insert_range(9831..=9837)`,
// `insert_range(0..10)` -- so a value is exactly what is needed, not more loop sugar.
//
// Representation: a 3-slot cell [lo@0, hi@8, incl@16], the same shape every other aggregate here uses. The
// inclusive flag is carried rather than baked in, because the for-loop consumer below has to reconstruct the
// comparison from a value it did not see written.
fn s9_range_new(lo: &Option<P<ExprK>>, hi: &Option<P<ExprK>>, incl: bool, env: &S9Env) -> i32 {
    let losl = match lo { Some(e) => s9expr(e, env), None => s9const(0) };
    let hisl = match hi { Some(e) => s9expr(e, env), None => s9const(0) };
    let base = s9_alloc(24);
    s9_slot_store(base, 0, losl);
    s9_slot_store(base, 8, hisl);
    let iv = if incl { s9const(1) } else { s9const(0) };
    s9_slot_store(base, 16, iv);
    base
}
// L6 LANDMINE, SHARPENED here. The recorded rule is "an i32 must not be used twice in a match arm". The real
// boundary is narrower and it is not about match arms: an i32 that is a CALL RESULT may be passed to at most ONE
// subsequent call. Measured, bisected against L6 directly: one `s9_slot_load(cell, ..)` compiles (298,219 rzir
// lines), TWO give `error: use of moved value` and truncate the emission to 159,307 -- which yielded a 1.49 MB
// gen1 that segfaulted instantly. `s9_load_at` does the same, so it is the value and not the callee.
// The SAME value as a PARAMETER is unrestricted, which is why `s9_vec_iter_bounds(vbase)` reads its base three
// times and always has. So the fix is a parameter boundary: evaluate here, read there.
// (Arena refuted first, per the hazard list: 14.9 MB peak.)
fn s9_for_rangeval(pat: &Pat, iter: &ExprK, body: &Block, env: &S9Env) -> i32 {
    let cell = s9expr(iter, env);
    s9_for_rangecell(pat, cell, body, env)
}
fn s9_for_rangecell(pat: &Pat, cell: i32, body: &Block, env: &S9Env) -> i32 {
    let losl = s9_slot_load(cell, 0);
    let hisl = s9_slot_load(cell, 8);
    let incl = s9_slot_load(cell, 16);
    let hi2 = s9op("add8", hisl, incl);
    let isl = s9fresh(); println!("  v{}: i64 = or8 v{} v{}", isl, losl, losl);
    let nm = s9_patname(pat);
    let mut bn: Vec<String> = veccopy(&env.names); bn.push(nm);
    let mut bs: Vec<i32> = veccopy_i32(&env.slots); bs.push(isl);
    let mut bt: Vec<String> = veccopy(&env.types); bt.push(String::from(""));
    let benv = S9Env { names: bn, slots: bs, fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: bt };
    s9emit_loop_open();
    let g = s9op("lt", isl, hi2); s9emit_brz(g);
    s9_walk(body, &benv);
    s9emit_cont();
    s9emit_incr(isl);
    s9emit_jmpend();
    s9const(0)
}
fn s9_for_is_rangeval(iter: &ExprK, env: &S9Env) -> bool {
    let ity = s9_init_ty(env, iter);
    if str_contains(&ity, "Range") { return true; }
    let nm = s9_bare_name(iter);
    let lty = s9_lookup_type(env, &nm);
    str_contains(&lty, "Range")
}
fn s9_for(pat: &Pat, iter: &ExprK, body: &Block, env: &S9Env) -> i32 {
    let isrv = s9_for_is_rangeval(iter, env);
    match iter {
        ExprK::Range(lo, hi, incl) => {
            let losl = match lo { Some(e) => s9expr(e, env), None => s9const(0) };
            let hisl = match hi { Some(e) => s9expr(e, env), None => s9const(0) };
            let isl = s9fresh(); println!("  v{}: i64 = or8 v{} v{}", isl, losl, losl);
            let nm = s9_patname(pat);
            let mut bn: Vec<String> = veccopy(&env.names); bn.push(nm);
            let mut bs: Vec<i32> = veccopy_i32(&env.slots); bs.push(isl);
            let mut bt: Vec<String> = veccopy(&env.types); bt.push(String::from(""));
            let benv = S9Env { names: bn, slots: bs, fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: bt };
            s9emit_loop_open();
            let cmp = if *incl { "le" } else { "lt" };
            let g = s9op(cmp, isl, hisl); s9emit_brz(g);
            s9_walk(body, &benv);
            s9emit_cont();
            s9emit_incr(isl);
            s9emit_jmpend();
            s9const(0)
        }
        ExprK::Method(recv, mname, _mtys, _margs) => { if streq(mname, "iter") { s9_for_iter(pat, recv, body, env) } else { s9const(0) } }
        // P-RANGE-EXPR, the second consumer of the cell: `let r = 0..4; for i in r`. The arm above is the INLINE
        // form and stays as it is -- it needs no allocation and is the common case. This one loads lo/hi/incl
        // back out of a range VALUE, which is why the flag is stored rather than baked in. Inclusive is folded
        // into the bound (`hi + incl`) so the comparison stays a static `lt` instead of becoming dynamic.
        _ => { if isrv { s9_for_rangeval(pat, iter, body, env) } else { s9const(0) } }
    }
}
// the receiver Vec's declared type text (for element-kind classification): a Path local's type, or a `x.f` field's type.
fn s9_recv_vty(env: &S9Env, recv: &ExprK) -> String { match recv { ExprK::Path(p) => { let nm = dump_path(p); s9_lookup_type(env, &nm) } ExprK::Field(o, f) => { let sty = s9_recv_struct(o, env); s9_field_type_q(env, &sty, f) } _ => String::from("") } }   // SR-2: a `x.f.iter()` receiver's Vec type by the qualified field (C-G2D-FIELD-COLLISION: PathSeg.args:Vec<Ty> vs Attr.args:String)
// the element binding-kind of a `Vec<INNER>` type text: extract INNER (after the FIRST `Vec<`) and classify it (s9_ty_kind:
// 1 SCALAR i*/u*/bool, 0 aggregate String/Vec/user-struct, 2 Box). Default 1 (scalar) when no `Vec<` is found -> unchanged
// behavior for an unknown receiver. Classifying the WHOLE `Vec<..>` text would wrongly hit "Vec" -> 0; we skip past `Vec<`.
fn s9_vec_elem_kind(vty: &str) -> i32 { let b = vty.as_bytes(); let mut i = 0; let mut cut = -1; while i + 3 < b.len() { if cut < 0 { if b[i] == 86 { if b[i+1] == 101 { if b[i+2] == 99 { if b[i+3] == 60 { cut = (i as i32) + 4; } } } } } i += 1; } if cut < 0 { return 1; } let inner = &vty[(cut as usize)..]; s9_ty_kind(inner) }
// pick the loop-var binding + its by-ref flag: a SCALAR element (ek==1) binds `e` to the slot ADDRESS (esl) so a body `*e`
// loads it; an AGGREGATE element (String/Vec/struct/enum, ek 0/2) binds `e` to the slot VALUE (evl = load(esl)) = the
// element's stored address, so bare `e` (dump(e), e.len(), match e) is the aggregate. `.iter()` yields `&T`, so an aggregate
// binding IS a reference -- refty=1 records "Ty Ref" (via s9_iter_refty) so a `match e` on an enum element binds its payload
// BY-REF (`*x` loads), not by-value (which would deref the payload VALUE -> segv). `ek` is read exactly ONCE (a fn param).
struct S9IterB { slot: i32, refty: i32 }
fn s9_iter_bind(ek: i32, esl: i32, evl: i32) -> S9IterB { if ek == 1 { S9IterB { slot: esl, refty: 0 } } else { S9IterB { slot: evl, refty: 1 } } }
// the recorded type text of the loop binding: "Ty Ref" for an aggregate element (so s9_scrut_by_ref sees a reference), else "".
fn s9_iter_refty(refty: i32) -> String { if refty != 0 { String::from("Ty Ref") } else { String::from("") } }
// `for e in v.iter()` -> pointer walk: esl (the element slot ADDRESS) steps by 4 bytes/iter while esl < end. ONE mutable slot
// (esl, like Range's isl); the bounds are FIELD reads (it.dptr/it.end) so this stays within L6's move-check budget. Each
// iteration ALSO refreshes evl = load(esl) (the element VALUE) -- always emitted (harmless dead vreg for scalars) so the only
// type-dependent branch is the ONE `ek` read in s9_pick_iter_slot, never a per-iteration conditional emit. Scalar element:
// bind esl, body `*e` loads. Aggregate element: bind evl (the stored aggregate address) so bare `e` is the aggregate. This is
// the type-aware dual of the s9_bind_slot enum-payload binding (C-G2D-VECITER-ELEMENT-KIND -- the gen2 dump-path fix).
fn s9_for_iter(pat: &Pat, recv: &ExprK, body: &Block, env: &S9Env) -> i32 {
    let vty = s9_recv_vty(env, recv);
    let ek = s9_vec_elem_kind(&vty);
    let vbase = s9expr(recv, env);
    let it = s9_vec_iter_bounds(vbase);
    let esl = s9fresh(); s9emit_setslot(esl, it.dptr);
    let evl = s9fresh(); s9emit_setslot(evl, esl);
    let ib = s9_iter_bind(ek, esl, evl);
    let nm = s9_patname(pat);
    let mut bn: Vec<String> = veccopy(&env.names); bn.push(nm);
    let mut bs: Vec<i32> = veccopy_i32(&env.slots); bs.push(ib.slot);
    // SR-2: for a USER-struct element, record the FULL element type "Ty Ref(Ty Path Struct)" so `f.field` in the body qualifies
    // by the element struct (C-G2D-FIELD-COLLISION); String/Vec/str/scalar elements keep the plain "Ty Ref"/"" (unchanged dispatch).
    let es = s9_struct_of_ty(&vty);
    let etystr = if ib.refty != 0 { if streq(&es, "Vec") || es.len() == 0 { String::from("Ty Ref") } else { format!("Ty Ref(Ty Path {})", es) } } else { String::from("iterscalar") };   // C-S9-ITER-SCALAR-NO-AUTODEREF: mark it so arithmetic can load through it   // C-G2D-STR-VEC-ELEM-TY: a String/str element now records "Ty Ref(Ty Path String)"/"...str" (was bare "Ty Ref", which the format!/{} classifier missed -> a Vec<String> element like a MacCall tok printed as its ADDRESS via __pi). Vec/empty stay bare "Ty Ref" (unchanged dispatch).
    let mut bt: Vec<String> = veccopy(&env.types); bt.push(etystr);
    let benv = S9Env { names: bn, slots: bs, fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: bt };
    s9emit_loop_open();
    let g = s9op("lt", esl, it.end); s9emit_brz(g);
    let ev = s9_slot_load(esl, 0); s9emit_setslot(evl, ev);
    s9_walk(body, &benv);
    s9emit_cont();
    s9emit_incr4(esl);
    s9emit_jmpend();
    s9const(0)
}
// match arm tail: write the arm body value into the reserved result slot rv (or-merge), jump to the shared merge label.
fn s9emit_arm_tail(rv: i32, lbl: i32, bodyval: i32) { s9ck_reset(); println!("  v{}: i64 = or8 v{} v{}", rv, bodyval, bodyval); println!("  goto __Lm{}", lbl); }
fn s9emit_match_label(lbl: i32) { println!("  label __Lm{}", lbl); s9ck_reset(); }
// `match X { pat => body, .. }` (P-LC-MATCH B5): lower the scrutinee to slot sv; per arm a guard test (integer literal
// pat -> eq sv lit + brz; wildcard/binding -> irrefutable, no test) then body -> or into the reserved result slot rv +
// goto the shared merge label; a refutable arm closes with pool. rv doubles as the merge label id (per B1). Pattern is
// classified INLINE (S9-F11: never pass a matched pat/field to a helper). Payload/tuple/struct destructure + ranges +
// ident-binding-into-body-env are B6/D-tier, deferred.
// is this arm's pattern an integer LITERAL (a refutable eq-test)? classified through a &Pat param (matched directly --
// NOT `&*box_field`, which L6 mis-compiles to a runtime crash, S9-F12). `&**litbox` on a &Box IS the sound codebase idiom.
// the last `::`-segment of a path string (E::A -> A); and whether a path is qualified (contains `::`).
fn s9_last_seg_str(s: &str) -> String { let b = s.as_bytes(); let mut cut = 0; let mut i = 0; while i + 1 < b.len() { if b[i] == 58 { if b[i+1] == 58 { cut = i + 2; } } i += 1; } format!("{}", &s[cut..]) }
fn s9_qualified(s: &str) -> bool { let b = s.as_bytes(); let mut i = 0; let mut q = false; while i + 1 < b.len() { if b[i] == 58 { if b[i+1] == 58 { q = true; } } i += 1; } q }
// pattern classification (via &Pat params, S9-F12): kind 0 = irrefutable (ident/wild), 1 = int literal, 2 = variant
// tuple-struct (E::A(v), has payload binding), 3 = variant path (E::B, unit). test-val = the lit (kind 1) or the variant
// TAG (kinds 2/3, via the dual registry). bind-name = the single payload binding of a tuple-struct pattern ("" if none).
// a bool-lit pattern (`true`/`false`) is a REFUTABLE literal test (kind 1), value 1/0 on the i32 bool model -- was falling to
// the inner `_ => 0` (irrefutable), so `match b { true => A, false => B }` never emitted the test and ALWAYS took the first arm
// (C-G2D-BOOL-PAT: e.g. any s1_parser `match somebool { .. }` mis-selected). (`match &**litbox` is the proven-safe box idiom, S9-F12 exception.)
// C-G2D-REFMATCH-BOX-DEREF: the Pat::Lit payload is a P<ExprK> box. emit_s9 binds a box match-payload with kind=2
// (double-load) -> the binding is ALREADY the inner ExprK address; a source `&**litbox` then over-derefs it and, used as
// the inner-match scrutinee, faults (s9_read_scrut loads the tag through garbage). rustc has no such issue (litbox is
// &Box<ExprK>). The fix keeps both semantics in agreement: hand `litbox` to a helper taking `&ExprK` -- rustc Deref-coerces
// (`&**litbox`), emit_s9 passes the already-resolved inner address (which IS &ExprK). No explicit deref -> no over-deref.
fn s9_litexpr_kind(e: &ExprK) -> i32 { match e { ExprK::LitInt(_n) => 1, ExprK::Bool(_b) => 1, _ => 0 } }
// The byte value of a char-literal's SOURCE TEXT as the lexer stores it (the content between the quotes, escapes
// unexpanded): "a" -> 97, " " -> 32, "\\n" -> 10, "\\x41" -> 65. Written as independent `if .. { return }` statements
// rather than an else-if chain, per the arm-drop landmine recorded in this file.
fn s9_hexval(c: u8) -> i64 {
    if c >= 48 { if c <= 57 { return (c - 48) as i64; } }
    if c >= 97 { if c <= 102 { return (c - 97) as i64 + 10; } }
    if c >= 65 { if c <= 70 { return (c - 65) as i64 + 10; } }
    0
}
fn s9_char_val(cv: &str) -> i64 {
    let b = cv.as_bytes();
    if b.len() == 0 { return 0; }
    if b[0] != 92 { return b[0] as i64; }
    if b.len() < 2 { return 92; }
    let e = b[1];
    if e == 110 { return 10; }
    if e == 116 { return 9; }
    if e == 114 { return 13; }
    if e == 48 { return 0; }
    if e == 92 { return 92; }
    if e == 39 { return 39; }
    if e == 34 { return 34; }
    if e == 120 { if b.len() >= 4 { return s9_hexval(b[2]) * 16 + s9_hexval(b[3]); } return 0; }
    e as i64
}
fn s9_litexpr_val(e: &ExprK) -> i32 { match e { ExprK::LitInt(n) => lit_value(*n) as i32, ExprK::Bool(b) => { if *b { 1 } else { 0 } }, _ => 0 } }
fn s9_pat_kind(pat: &Pat) -> i32 { match pat { Pat::Lit(litbox) => s9_litexpr_kind(litbox), Pat::TupleStruct(_p, _pats) => 2, Pat::Path(_p) => 3, _ => 0 } }
fn s9_pat_test_val(env: &S9Env, pat: &Pat) -> i32 { match pat { Pat::Lit(litbox) => s9_litexpr_val(litbox), Pat::TupleStruct(pbox, _pats) => { let f = dump_path(pbox); s9_enum_tag(env, &f) }, Pat::Path(pbox) => { let f = dump_path(pbox); s9_enum_tag(env, &f) }, _ => 0 } }
// the i-th TupleStruct sub-pattern's bind name ("" if none / out of range). `E::Bin(op,a,b)` field 1 -> "a".
fn s9_pat_nth_bind(pat: &Pat, i: i32) -> String { let mut r = String::new(); match pat { Pat::TupleStruct(_p, pats) => { let mut k = 0; for sp in pats.iter() { if k == i { r = s9_patname(sp); } k = k + 1; } }, Pat::Tuple(pats) => { let mut k = 0; for sp in pats.iter() { if k == i { r = s9_patname(sp); } k = k + 1; } }, _ => {} } r }
// classify an arm's pattern in ONE &Pat access (L6 moves a loop-var FIELD reused across >2 calls, S9-F13 class).
// MULTI-FIELD: carry each field's bind name (n0..n3) as a FIELD so s9_match reads fields, not loop-locals (C-ESC-FIELD-ROUTE).
// the binding KIND of a dumped field type: 1 SCALAR (i*/u*/bool; by-ref -> ADDRESS so `*n` loads), 2 BOX (celled; -> DOUBLE
// load: field->cell ptr, cell->inner addr), 0 OTHER-AGGREGATE (String/Vec/user; address-valued -> SINGLE load = the address).
fn s9_ty_kind(ty: &str) -> i32 {
    if str_contains(ty, "Box") { return 2; }
    if str_contains(ty, "String") { return 0; } if str_contains(ty, "Vec") { return 0; }
    if str_contains(ty, " i8") || str_contains(ty, " i16") || str_contains(ty, " i32") || str_contains(ty, " i64") { return 1; }
    if str_contains(ty, " u8") || str_contains(ty, " u16") || str_contains(ty, " u32") || str_contains(ty, " u64") { return 1; }
    if str_contains(ty, "usize") || str_contains(ty, "isize") || str_contains(ty, "bool") { return 1; }
    0
}
// per-field binding-kind digits "1"/"2"/"0".. of a variant (Tuple fields), stored in the ftypes registry. "" for Unit/Struct.
fn s9_variant_flags(v: &Variant) -> String { let mut r = String::new(); match &v.kind { VariantKind::Tuple(tys) => { for t in tys.iter() { let ts = dump_ty(t); if s9_is_opt_box(&ts) { r.push_str("3"); } else if s9_bare_box_ty(&ts) { r.push_str("2"); } else { let k = s9_ty_kind(&ts); if k == 1 { r.push_str("1"); } else { r.push_str("0"); } } } }, _ => {} } r }   // SR: a bare Box<T>/P<T> payload -> '2' (DOUBLE load); s9_ty_kind sees only literal "Box", missing the P<T> alias -> a P<Path>/P<Ty> variant payload was single-loaded (box cell) -> dump_ty(Ty::Path(p)) crashed (C-G2D-P-ALIAS-VARIANT-BOX)
// an `Option<Box<X>>` / `Option<P<X>>` field (P = Box alias). The FIELD binds by SINGLE load (the Option block is an aggregate),
// but its inner `Some(payload)` is a Box that needs the DOUBLE load, so it is flagged '3' and the binding records a box-marker
// type text (s9_bind_ty) that the NESTED `match body { Some(b) => .. }` reads (s9_scrut_opt_box) to bind `b` with kind 2.
// P< keys the alias segment ("Ty Path P<"); "Param"/"Path" don't contain "P<". (C-G2D-NESTED-OPTION-PAYLOAD.)
fn s9_is_opt_box(ts: &str) -> bool { if str_contains(ts, "Option") == false { return false; } if str_contains(ts, "Box") { return true; } str_contains(ts, "P<") }
// the recorded binding TYPE text for a payload flagged f: a box-marker for an Option<Box> field ('3'), else none. The marker
// only needs to satisfy s9_scrut_opt_box (contains "Option" + "Box") when this binding is later a nested-match scrutinee.
fn s9_bind_ty(f: i32) -> String { if f == 3 { String::from("Option<Box>") } else { String::from("") } }
// the i-th binding kind (1 scalar / 2 box / 0 other-aggregate; default 1 if absent).
fn s9_nth_flag(flags: &str, i: i32) -> i32 { let b = flags.as_bytes(); if i >= 0 && (i as usize) < b.len() { if b[i as usize] == 49 { return 1; } if b[i as usize] == 50 { return 2; } if b[i as usize] == 51 { return 3; } return 0; } 1 }
// the scalar-flag string registered (in ftypes) for a TupleStruct pattern's variant.
fn s9_pat_variant_flags(env: &S9Env, pat: &Pat) -> String { match pat { Pat::TupleStruct(pbox, _pats) => { let f = dump_path(pbox); if s9_qualified(&f) { let e = s9_enum_seg(&f); let v = s9_last_seg_str(&f); let qk = format!("{}#{}", e, v); if s9_field_has(env, &qk) { return s9_field_type(env, &qk); } } s9_field_type(env, &s9_last_seg_str(&f)) }, _ => String::new() } }   // SR-2: per-enum variant flags (C-G2D-VARIANT-FLAGS-COLLISION)
// the VARIANT name of a tuple-struct pattern (`E::V(..)` -> "V"), "" otherwise. (SR-2: to look up the payload's declared type.)
fn s9_pat_vname(pat: &Pat) -> String { match pat { Pat::TupleStruct(pbox, _pats) => { let f = dump_path(pbox); s9_last_seg_str(&f) }, _ => String::new() } }
// C-G2D-VARIANT-FIELD-TY-COLLISION: the ENUM-qualified variant key ("VariantKind#Struct") from a qualified pattern path,
// else the bare variant name. Used to look up the payload field TYPE registry `Enum#Variant#i` so a variant NAME shared
// across enums (ItemKind::Struct vs VariantKind::Struct) does not last-writer-collide on the bare `Struct#i` key.
fn s9_pat_vqual(pat: &Pat) -> String { match pat { Pat::TupleStruct(pbox, _pats) => { let f = dump_path(pbox); if s9_qualified(&f) { let e = s9_enum_seg(&f); let v = s9_last_seg_str(&f); format!("{}#{}", e, v) } else { s9_last_seg_str(&f) } }, _ => String::new() } }
// the declared TYPE TEXT of variant `vname`'s i-th tuple field ("" if unknown) -- keyed `vname#i` in the field registry.
// A match payload binding pushes this as its btype so `for f in payload.iter()` / `payload.field` type-resolve (SR-2).
fn s9_variant_field_ty(env: &S9Env, vname: &str, i: i32) -> String { if vname.len() == 0 { return String::from(""); } let qk = format!("{}#{}", vname, i); if s9_field_has(env, &qk) { s9_field_type(env, &qk) } else { String::from("") } }
struct S9ArmInfo { kind: i32, testval: i32, n0: String, n1: String, n2: String, n3: String, n4: String, n5: String, s0: i32, s1: i32, s2: i32, s3: i32, s4: i32, s5: i32 }
// C-S9-VARIANT-DISC1-ARITY5: carried only n0..n3, so a tuple variant with FIVE payload fields silently lost its 5th
// binding and the name read as garbage. ItemKind::Fn(Vec<GParam>, Vec<Param>, Option<P<Ty>>, Option<P<Block>>, bool)
// is exactly that shape -- why a SEM0-built SEM0 reported EVERY fn `async`. Single-variant enums escaped it: an
// irrefutable match never builds an S9ArmInfo, which is why the reproducer also needed a second variant.
fn s9_arm_info(pat: &Pat, env: &S9Env) -> S9ArmInfo { let k = s9_pat_kind(pat); let tv = s9_pat_test_val(env, pat); let fl = s9_pat_variant_flags(env, pat); S9ArmInfo { kind: k, testval: tv, n0: s9_pat_nth_bind(pat, 0), n1: s9_pat_nth_bind(pat, 1), n2: s9_pat_nth_bind(pat, 2), n3: s9_pat_nth_bind(pat, 3), n4: s9_pat_nth_bind(pat, 4), n5: s9_pat_nth_bind(pat, 5), s0: s9_nth_flag(&fl, 0), s1: s9_nth_flag(&fl, 1), s2: s9_nth_flag(&fl, 2), s3: s9_nth_flag(&fl, 3), s4: s9_nth_flag(&fl, 4), s5: s9_nth_flag(&fl, 5) } }
// match lowering (B5 int + D3 enum + B6 payload bind). For an enum match (any arm is a variant pattern) the scrutinee
// operand is the TAG (load at base+0); for an int match it is the value. Per arm: eq(sv, lit-or-tag) + brz (refutable),
// or body (irrefutable); a tuple-struct arm binds its single payload (load at base+4) into the arm env; body -> or into
// the reserved result slot rv + goto the shared merge label; refutable arms close with pool.
// MULTI-FIELD: carry each field's binding SLOT (b0..b3) as a FIELD (C-ESC-FIELD-ROUTE). by-ref -> the field ADDRESS
// (base+off) so `*n` loads; by-value -> the field VALUE (load base+off). Only computed when is_enum (base is a block ptr).
// carry the scrutinee `base` + by-ref mode `refm` as FIELDS so s9_match can bind each field per-arm (type-aware) without
// reusing the fn-return `base` local (C-ESC-FIELD-ROUTE). tag/pay as before.
struct S9Sc { tag: i32, pay: i32, refm: i32, base: i32 }
// is the match scrutinee a REFERENCE? (`match &x`, or `match e` where e: &T). Under a by-ref match rustc binds each payload
// BY REFERENCE (a &Field) so `*n` derefs; a by-value match binds the field VALUE. L6-safe: a mut bool + STATEMENT arms.
fn s9_scrut_by_ref(scrut: &ExprK, env: &S9Env) -> bool { let mut r = false; match scrut { ExprK::Ref(_m, _inner) => { r = true; } ExprK::Path(p) => { let nm = dump_path(p); let ty = s9_lookup_type(env, &nm); if str_contains(&ty, "Ty Ref") { r = true; } } _ => {} } r }
// is the match scrutinee an `Option<Box<X>>` / `Option<P<X>>`? (its declared type carries the box-marker or the real
// Option<Box/P> text.) When so, its `Some(payload)` binds the payload with kind 2 (DOUBLE load: Some@4 -> box cell -> inner
// addr), since the builtin Option variant carries no field flags. Returns 1/0 (i32, Copy -> loop-exempt). (C-G2D-NESTED-OPTION-PAYLOAD.)
// C-G2D-DISR-OPTBOX: also detect an Option<Box> scrutinee reached via `&x` / a FIELD (e.g. `match &v.disr { Some(e) => .. }`
// where v.disr: Option<P<ExprK>>). Was Path-only, so a Ref/Field scrutinee fell to 0 -> the Some payload was single-loaded
// (bound the box cell, not the inner) -> dump_expr(e) crashed. s9_recv_vty types a Path OR a Field; see through Ref/Paren first.
fn s9_scrut_opt_box(scrut: &ExprK, env: &S9Env) -> i32 {
    let ty = match scrut { ExprK::Ref(_m, inner) => s9_recv_vty(env, inner), ExprK::Paren(inner) => s9_recv_vty(env, inner), _ => s9_recv_vty(env, scrut) };
    if s9_is_opt_box(&ty) { 1 } else { 0 }
}
// read the scrutinee: tag@0, payload VALUE @4, and the binding slot `bind` -- by-ref match -> the payload's ADDRESS (base+4)
// so a `*n` in the arm loads it, by-value -> the payload VALUE. `bind` is carried as a FIELD so the arm loop reads it without
// a move: L6's move-checker rejects a REUSED LOCAL across loop iterations (a spurious move that ABORTS L6's compilation --
// C-L6H-MOVECHECK-LOOPLOCAL), but a struct-FIELD access is re-read each iteration and exempt (the same reason the old
// binding read `sc.pay` directly in the loop). The value-selection lives here in a SMALL fn, not the high-pressure loop.
// the binding slot for a payload field at byte offset `off`: by-ref -> ADDRESS (base+off); by-value -> VALUE (load base+off).
// the binding slot for a payload field at `off`. by-ref SCALAR (refm!=0 && scalar!=0) -> the field ADDRESS (base+off) so `*n`
// loads it; otherwise (aggregate, or by-value) -> the field VALUE (load base+off) = the scalar value / the inner address (with
// the transparent Box). All params are Copy. (P-SCI-REFMATCH-BOX type-aware binding.)
fn s9_bind_slot(base: i32, refm: i32, off: i32, kind: i32) -> i32 { let mut r = 0; if kind == 1 && refm != 0 { let o = s9const(off as i64); r = s9op("add", base, o); } else { let f = s9_slot_load(base, off); if kind == 2 { r = s9_slot_load(f, 0); } else { r = f; } } r }
fn s9_read_scrut(base: i32, is_enum: bool, by_ref: bool) -> S9Sc { let mut tag = base; let mut pay = base; if is_enum { tag = s9_load_at(base, 0); pay = s9_slot_load(base, 8); } let mut refm = 0; if by_ref { refm = 1; } S9Sc { tag: tag, pay: pay, refm: refm, base: base } }
// C-S9-IF-LET-UNLOWERED: `ExprK::Let` (the condition of `if let` / `while let`) had no arm, so it fell to the
// silent-zero catch-all and every `if let` took its ELSE branch. Measured: `if let Some(x) = Some(5)` printed 0,
// `while let Some(x) = v.pop()` never ran; L6 is correct on both. After the closure work, `Let` is the LARGEST
// remaining unlowered ExprK (63 occurrences on the corpus).
// `if let PAT = E { A } else { B }` is `match E { PAT => A, _ => B }`, but synthetic arms cannot be built: Arm
// owns P<Pat>/P<ExprK> and the AST derives no Clone (only Tok does). So the single-arm shape of s9_match is
// mirrored here instead -- the same s9_arm_info / s9_read_scrut / s9_bind_slot helpers, with s9_if's
// brz/then-tail/else-tail control flow.
fn s9_if_let_binds(info: &S9ArmInfo, sc: &S9Sc, obk: i32, vn: &str, env: &S9Env) -> S9Env {
    let mut names = veccopy(&env.names); let mut slots = veccopy_i32(&env.slots); let mut btypes = veccopy(&env.types);
    let k0 = if obk == 1 { 2 } else { info.s0 };
    if info.n0.len() > 0 { names.push(format!("{}", info.n0)); slots.push(s9_bind_slot(sc.base, sc.refm, 8, k0)); btypes.push(s9_variant_field_ty(env, vn, 0)); }
    if info.n1.len() > 0 { names.push(format!("{}", info.n1)); slots.push(s9_bind_slot(sc.base, sc.refm, 16, info.s1)); btypes.push(s9_variant_field_ty(env, vn, 1)); }
    if info.n2.len() > 0 { names.push(format!("{}", info.n2)); slots.push(s9_bind_slot(sc.base, sc.refm, 24, info.s2)); btypes.push(s9_variant_field_ty(env, vn, 2)); }
    if info.n3.len() > 0 { names.push(format!("{}", info.n3)); slots.push(s9_bind_slot(sc.base, sc.refm, 32, info.s3)); btypes.push(s9_variant_field_ty(env, vn, 3)); }
    S9Env { names: names, slots: slots, fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: btypes }
}
// P-TRY, step 2: the `?` mouth. THIS IS ROUTING, NOT MECHANISM -- every piece below already existed and is
// exercised by `if let`: s9_read_scrut reads the tag and payload, the value-if shape (brz / then_tail /
// else_tail) is s9_if_let_val's, and s9emit_ret is the early-exit edge. `?` is that shape with the failure arm
// replaced by a return of the whole value. Measured before building: the hand-desugared forms
// (`match o { S(v) => v, N => return }`, `if let .. else { return }`, and two in sequence) were ALL already
// correct, which is what classified this pipeline as routing.
//
// THE SUCCESS TAG DIFFERS BY TYPE AND THE TAGS COLLIDE: Result is Ok=0/Err=1, Option is None=0/Some=1, so tag
// 0 means SUCCESS for Result and FAILURE for Option. The tag alone cannot decide it -- the scrutinee's declared
// or inferred type must. Defaulting to Option would be wrong for rustc, where `?` is overwhelmingly on Result.
fn s9_try_ok_tag(inner: &ExprK, env: &S9Env) -> i64 {
    let ty = s9_lookup_type(env, &s9_bare_name(inner));
    if str_contains(&ty, "Result") { return 0; }
    let ity = s9_init_ty(env, inner);
    if str_contains(&ity, "Result") { return 0; }
    let fty = s9_field_type(env, &s9_field_name(inner));
    if str_contains(&fty, "Result") { return 0; }
    1
}
fn s9_try(inner: &ExprK, env: &S9Env) -> i32 {
    let rv = s9fresh();
    let lbl = s9_lbl();
    let base = s9expr(inner, env);
    let sc = s9_read_scrut(base, true, false);
    let okt = s9_try_ok_tag(inner, env);
    let tvl = s9const(okt);
    let c = s9op("eq", sc.tag, tvl);
    s9emit_brz(c);
    let tv = s9_slot_load(sc.base, 8);
    s9emit_then_tail(rv, lbl, tv);
    s9emit_ret(sc.base);
    let z = s9const(0);
    s9emit_else_tail(rv, lbl, z);
    rv
}
fn s9_if_let_val(pat: &Pat, scrut: &ExprK, then: &Block, els: &ExprK, env: &S9Env) -> i32 {
    let rv = s9fresh();
    let lbl = s9_lbl();
    let base = s9expr(scrut, env);
    let k = s9_pat_kind(pat);
    let is_enum = k >= 2;
    let by_ref = s9_scrut_by_ref(scrut, env);
    let obk = s9_scrut_opt_box(scrut, env);
    let sc = s9_read_scrut(base, is_enum, by_ref);
    let info = s9_arm_info(pat, env);
    let tvl = s9const(info.testval as i64);
    let c = s9op("eq", sc.tag, tvl);
    s9emit_brz(c);
    let vn = s9_pat_vqual(pat);
    let benv = s9_if_let_binds(&info, &sc, obk, &vn, env);
    let tv = s9_bv(then, &benv);
    s9emit_then_tail(rv, lbl, tv);
    let ev = s9expr(els, env);
    s9emit_else_tail(rv, lbl, ev);
    rv
}
// `while let P = S { B }` = the loop skeleton with the if-let GUARD in the condition slot: loop / eval S / test the
// tag against P's discriminant / brz (which exits the loop, exactly as s9_while's `brz cv` does) / bind / <B> /
// cont-jmp-endloop. No `pool`: inside a loop the brz scope is closed by endloop, which is why s9_while emits no pool
// either. Scrutinee is re-evaluated per iteration, which is what Rust does and what `while let Some(x) = v.pop()`
// requires. Unit value -- Rust forbids `break EXPR` from a while.
// P-LETCHAIN, THIRD MOUTH: `while let P = e && cond`. The remaining 23 `Let` occurrences after the `if` chain
// landed were all this shape -- `while let hir::ExprKind::Path(ref qpath) = expr.kind && ..` in rustc_lint and
// rustc_mir_transform. Same interior again (s9_chain_emit), so nothing new is built; only the scope arithmetic
// differs. Inside a loop the LAST `brz` scope is closed by `endloop`, which is why the plain while-let emits no
// `pool` at all -- so a chain of n tests closes n-1 of them itself and leaves the last to loop_close.
// In a LOOP the tests cannot nest the way they do in an `if`. Nesting was the first attempt and it is wrong in
// general: only the OUTERMOST `brz` is closed by `endloop` and therefore only that one exits the loop; a failing
// inner test lands after its `pool`, falls into `cont`/`jmp`, and iterates again. With a `let` first and a
// constant guard second that accidentally behaves; with the guard FIRST it segfaulted. Measured, and the shape
// census is 1 occurrence in the whole tree -- which is a reason to get it right cheaply, not a reason to ship a
// segfault.
// Break-on-failure instead: each test emits `if !t { break }` and closes its own scope immediately, so every
// leaf exits the loop on failure regardless of order, and there is no scope arithmetic at all.
fn s9_chain_emit_loop(cond: &ExprK, env: &S9Env) -> S9Env {
    match s9f12_deref(cond) {
        ExprK::Bin(op, a, b) => {
            if streq(op, "And") { let e1 = s9_chain_emit_loop(a, env); return s9_chain_emit_loop(b, &e1); }
            s9_chain_test_brk(cond, env)
        }
        ExprK::Let(lp, ls) => s9_chain_let_brk(lp, ls, env),
        _ => s9_chain_test_brk(cond, env)
    }
}
fn s9_chain_test_brk(cond: &ExprK, env: &S9Env) -> S9Env {
    let v = s9expr(cond, env);
    s9_brk_unless(v);
    s9_env_copy(env)
}
fn s9_chain_let_brk(lp: &Pat, ls: &ExprK, env: &S9Env) -> S9Env {
    let base = s9expr(ls, env);
    let k = s9_pat_kind(lp);
    let is_enum = k >= 2;
    let by_ref = s9_scrut_by_ref(ls, env);
    let obk = s9_scrut_opt_box(ls, env);
    let sc = s9_read_scrut(base, is_enum, by_ref);
    let info = s9_arm_info(lp, env);
    let tvl = s9const(info.testval as i64);
    let c = s9op("eq", sc.tag, tvl);
    s9_brk_unless(c);
    let vn = s9_pat_vqual(lp);
    s9_if_let_binds(&info, &sc, obk, &vn, env)
}
fn s9_brk_unless(v: i32) {
    let z = s9const(0);
    let nv = s9op("eq", v, z);
    s9emit_brz(nv);
    s9emit_brk();
    s9emit_pool();
}
fn s9_while_chain(cond: &ExprK, b: &Block, env: &S9Env) -> i32 {
    s9emit_loop_open();
    let benv = s9_chain_emit_loop(cond, env);
    s9_walk(b, &benv);
    s9emit_loop_close();
    s9const(0)
}
fn s9_while_let(pat: &Pat, scrut: &ExprK, b: &Block, env: &S9Env) -> i32 {
    s9emit_loop_open();
    let base = s9expr(scrut, env);
    let k = s9_pat_kind(pat);
    let is_enum = k >= 2;
    let by_ref = s9_scrut_by_ref(scrut, env);
    let obk = s9_scrut_opt_box(scrut, env);
    let sc = s9_read_scrut(base, is_enum, by_ref);
    let info = s9_arm_info(pat, env);
    let tvl = s9const(info.testval as i64);
    let c = s9op("eq", sc.tag, tvl);
    s9emit_brz(c);
    let vn = s9_pat_vqual(pat);
    let benv = s9_if_let_binds(&info, &sc, obk, &vn, env);
    s9_walk(b, &benv);
    s9emit_loop_close();
    s9const(0)
}
// P-LETCHAIN (substrate-pipeline-oracle.md rank 2, 1,565 occurrences / 30.1% of the gap). `if a && let P = b`
// parses as `If(Bin(And, a, Let(P, b)), then)`, and `ExprK::Let` inside that `And` had no arm, so the whole
// condition fell to the silent-zero catch-all.
//
// THIS IS THE SECOND MOUTH ONTO P-TRY's INTERIOR, exactly as the oracle's meet predicted: the body below is
// s9_if_let_stmt's -- s9_read_scrut, s9_arm_info, s9_if_let_binds -- with one extra guard branch in front.
// Nothing new is built.
//
// It cannot live in s9_and: that returns a VALUE, and a let-chain has to return a BINDING ENVIRONMENT to the
// then-block. The dispatch therefore happens at the `if`, where the then-block is in scope.
//
// Census over the whole rustc tree bounds the shape: 1,482 single `&& let`, TWO with a second `let` in the same
// condition, and none deeper. Both operand ORDERS are handled (`a && let P = b` and `let P = b && a`); Rust
// evaluates the guard of the second form INSIDE the binding's scope, which is why `benv` is threaded there.
fn s9_letchain_stmt(guard: &ExprK, lp: &Pat, ls: &ExprK, then: &Block, gfirst: bool, env: &S9Env) -> i32 {
    if gfirst { let gv = s9expr(guard, env); s9emit_brz(gv); }
    let base = s9expr(ls, env);
    let k = s9_pat_kind(lp);
    let is_enum = k >= 2;
    let by_ref = s9_scrut_by_ref(ls, env);
    let obk = s9_scrut_opt_box(ls, env);
    let sc = s9_read_scrut(base, is_enum, by_ref);
    let info = s9_arm_info(lp, env);
    let tvl = s9const(info.testval as i64);
    let c = s9op("eq", sc.tag, tvl);
    s9emit_brz(c);
    let vn = s9_pat_vqual(lp);
    let benv = s9_if_let_binds(&info, &sc, obk, &vn, env);
    if gfirst == false { let gv2 = s9expr(guard, &benv); s9emit_brz(gv2); }
    s9_walk(then, &benv);
    s9emit_pool();
    if gfirst == false { s9emit_pool(); }
    if gfirst { s9emit_pool(); }
    s9const(0)
}
// P-LETCHAIN, GENERALISED. The first cut handled a two-operand chain by inspecting the top `Bin`'s two sides,
// and it closed only 570 of 1,565 occurrences. Real rustc chains MORE THAN ONE let, across lines:
//     if ... && let Some(base) = self.locals[place.local]
//            && let Some(new_local) = self.try_as_local(base, location)
// which nests as Bin(And, Bin(And, a, Let(P)), Let(Q)). The outer handler saw the RHS `Let(Q)`, treated the
// whole left side as a plain guard, and handed it to s9expr -- where the inner `Let` hit the catch-all again.
// The line-oriented census missed this shape entirely because the operands sit on separate lines.
//
// So the chain is walked RECURSIVELY instead. Each leaf emits exactly one `brz`, in source order, and a `let`
// leaf extends the environment that later leaves and the body see -- which is Rust's scoping rule and also what
// makes `&& x > 3` work after `let Some(x) = ..`. The count of leaves is the count of `pool`s to close.
fn s9_chain_count(cond: &ExprK) -> i32 {
    match s9f12_deref(cond) {
        ExprK::Bin(op, a, b) => { if streq(op, "And") { let l = s9_chain_count(a); let r = s9_chain_count(b); return l + r; } 1 }
        _ => 1
    }
}
fn s9_chain_emit(cond: &ExprK, env: &S9Env) -> S9Env {
    match s9f12_deref(cond) {
        ExprK::Bin(op, a, b) => {
            if streq(op, "And") { let e1 = s9_chain_emit(a, env); return s9_chain_emit(b, &e1); }
            let v = s9expr(cond, env); s9emit_brz(v); s9_env_copy(env)
        }
        ExprK::Let(lp, ls) => {
            let base = s9expr(ls, env);
            let k = s9_pat_kind(lp);
            let is_enum = k >= 2;
            let by_ref = s9_scrut_by_ref(ls, env);
            let obk = s9_scrut_opt_box(ls, env);
            let sc = s9_read_scrut(base, is_enum, by_ref);
            let info = s9_arm_info(lp, env);
            let tvl = s9const(info.testval as i64);
            let c = s9op("eq", sc.tag, tvl);
            s9emit_brz(c);
            let vn = s9_pat_vqual(lp);
            s9_if_let_binds(&info, &sc, obk, &vn, env)
        }
        _ => { let v = s9expr(cond, env); s9emit_brz(v); s9_env_copy(env) }
    }
}
fn s9_env_copy(env: &S9Env) -> S9Env {
    S9Env { names: veccopy(&env.names), slots: veccopy_i32(&env.slots), fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: veccopy(&env.types) }
}
fn s9_chain_stmt(cond: &ExprK, then: &Block, env: &S9Env) -> i32 {
    let n = s9_chain_count(cond);
    let benv = s9_chain_emit(cond, env);
    s9_walk(then, &benv);
    let mut i = 0;
    while i < n { s9emit_pool(); i = i + 1; }
    s9const(0)
}
fn s9_chain_val(cond: &ExprK, then: &Block, els: &ExprK, env: &S9Env) -> i32 {
    let rv = s9fresh();
    let lbl = s9_lbl();
    let n = s9_chain_count(cond);
    let benv = s9_chain_emit(cond, env);
    let tv = s9_bv(then, &benv);
    s9emit_arm_tail(rv, lbl, tv);
    let mut i = 0;
    while i < n { s9emit_pool(); i = i + 1; }
    let ev = s9expr(els, env);
    s9emit_setslot(rv, ev);
    s9emit_match_label(lbl);
    rv
}
// P-LETCHAIN in VALUE position (`if a && let P = b { .. } else { .. }`). The statement form above short-circuits
// with nested `brz`, but that leaves the two failure paths -- guard false, wrong variant -- with no way to reach
// a shared else. This uses the match machinery instead: both failures fall THROUGH to the else, the success arm
// jumps over it with `goto`, and the label lands after. Evaluation order is still Rust's: the scrutinee is only
// reached when the guard held, and the guard of the `let P = b && a` order is evaluated inside the binding.
// Measured before this existed: `if c && let Some(x) = b {..} else {..}` with c true and b Some(3) printed the
// ELSE branch. Not a regression -- the old path was a silent zero -- but a live wrong answer either way.
fn s9_letchain_val(guard: &ExprK, lp: &Pat, ls: &ExprK, then: &Block, els: &ExprK, gfirst: bool, env: &S9Env) -> i32 {
    let rv = s9fresh();
    let lbl = s9_lbl();
    if gfirst { let gv = s9expr(guard, env); s9emit_brz(gv); }
    let base = s9expr(ls, env);
    let k = s9_pat_kind(lp);
    let is_enum = k >= 2;
    let by_ref = s9_scrut_by_ref(ls, env);
    let obk = s9_scrut_opt_box(ls, env);
    let sc = s9_read_scrut(base, is_enum, by_ref);
    let info = s9_arm_info(lp, env);
    let tvl = s9const(info.testval as i64);
    let c = s9op("eq", sc.tag, tvl);
    s9emit_brz(c);
    let vn = s9_pat_vqual(lp);
    let benv = s9_if_let_binds(&info, &sc, obk, &vn, env);
    if gfirst == false { let gv2 = s9expr(guard, &benv); s9emit_brz(gv2); }
    let tv = s9_bv(then, &benv);
    s9emit_arm_tail(rv, lbl, tv);
    s9emit_pool();
    if gfirst == false { s9emit_pool(); }
    if gfirst { s9emit_pool(); }
    let ev = s9expr(els, env);
    s9emit_setslot(rv, ev);
    s9emit_match_label(lbl);
    rv
}
// the `Let` operand of an `&&` condition, and the other operand, when this is a let-chain. Returns -1 in `which`
// when it is not one. Kept as three small accessors because this tier mis-compiles an inline `match &**e`.
// P-LETCHAIN dispatch for a `Bin` condition. A let-chain in STATEMENT position takes the path above; anything
// else -- including a let-chain with an `else` arm -- falls through to the pre-existing behaviour, which is
// recorded as a residual rather than guessed at.
fn s9_if_bin(bop: &str, ba: &ExprK, bb: &ExprK, cond: &ExprK, then: &Block, els: &Vec<ExprK>, env: &S9Env) -> i32 {
    let _ = ba; let _ = bb; let _ = bop;
    if s9_chain_has_let(cond) {
        if els.len() == 0 { return s9_chain_stmt(cond, then, env); }
        return s9_chain_val(cond, then, &els[0], env);
    }
    if els.len() == 0 { let cv = s9expr(cond, env); s9emit_brz(cv); s9_walk(then, env); s9emit_pool(); return s9const(0); }
    s9_if(cond, then, &els[0], env)
}
// does this condition contain a `let` anywhere in its `&&` chain?
fn s9_chain_has_let(cond: &ExprK) -> bool {
    match s9f12_deref(cond) {
        ExprK::Bin(op, a, b) => { if streq(op, "And") { if s9_chain_has_let(a) { return true; } return s9_chain_has_let(b); } false }
        ExprK::Let(_p, _s) => true,
        _ => false
    }
}
fn s9_lc_which(cond: &ExprK) -> i32 {
    match s9f12_deref(cond) {
        ExprK::Bin(op, a, b) => {
            if streq(op, "And") == false { return 0 - 1; }
            match s9f12_deref(b) { ExprK::Let(_p, _s) => { return 1; } _ => {} }
            match s9f12_deref(a) { ExprK::Let(_p, _s) => { return 0; } _ => {} }
            0 - 1
        }
        _ => 0 - 1
    }
}
fn s9_if_let_stmt(pat: &Pat, scrut: &ExprK, then: &Block, env: &S9Env) -> i32 {
    let base = s9expr(scrut, env);
    let k = s9_pat_kind(pat);
    let is_enum = k >= 2;
    let by_ref = s9_scrut_by_ref(scrut, env);
    let obk = s9_scrut_opt_box(scrut, env);
    let sc = s9_read_scrut(base, is_enum, by_ref);
    let info = s9_arm_info(pat, env);
    let tvl = s9const(info.testval as i64);
    let c = s9op("eq", sc.tag, tvl);
    s9emit_brz(c);
    let vn = s9_pat_vqual(pat);
    let benv = s9_if_let_binds(&info, &sc, obk, &vn, env);
    s9_walk(then, &benv);
    s9emit_pool();
    s9const(0)
}
fn s9_match(scrut: &ExprK, arms: &Vec<Arm>, env: &S9Env) -> i32 {
    let rv = s9fresh();
    let mlbl = s9_lbl();
    let base = s9expr(scrut, env);
    let mut is_enum = false;
    for arm in arms.iter() { let k = s9_pat_kind(&arm.pat); if k >= 2 { is_enum = true; } }
    let by_ref = s9_scrut_by_ref(scrut, env);
    let obk = s9_scrut_opt_box(scrut, env);   // scrutinee is Option<Box/P> -> its Some payload@4 is a box cell (kind 2, double load)
    let sc = s9_read_scrut(base, is_enum, by_ref);
    for arm in arms.iter() {
        let info = s9_arm_info(&arm.pat, env);
        let refutable = info.kind > 0;
        if refutable { let t = s9const(info.testval as i64); let c = s9op("eq", sc.tag, t); s9emit_brz(c); }
        let mut names = veccopy(&env.names); let mut slots = veccopy_i32(&env.slots); let mut btypes = veccopy(&env.types);
        let k0 = if obk == 1 { 2 } else { info.s0 };   // override the builtin-Option Some payload to the box double-load
        let vn = s9_pat_vqual(&arm.pat);   // SR-2: ENUM-qualified variant key, to record each payload binding's DECLARED type via Enum#Variant#i (C-G2D-VARIANT-FIELD-TY-COLLISION); falls back to bare for unqualified patterns
        // MULTI-FIELD binding: 4 field-read blocks (info.nX names + sc.bX slots -- all FIELDS, exempt from L6's move-checker).
        // Each binding records its DECLARED type (s9_variant_field_ty) so a nested loop/field access on the payload resolves
        // its element/field struct (SR-2, C-G2D-FIELD-COLLISION). The real Option<Box> type subsumes s9_bind_ty's marker.
        if info.n0.len() > 0 { names.push(format!("{}", info.n0)); slots.push(s9_bind_slot(sc.base, sc.refm, 8, k0)); btypes.push(s9_variant_field_ty(env, &vn, 0)); }
        if info.n1.len() > 0 { names.push(format!("{}", info.n1)); slots.push(s9_bind_slot(sc.base, sc.refm, 16, info.s1)); btypes.push(s9_variant_field_ty(env, &vn, 1)); }
        if info.n2.len() > 0 { names.push(format!("{}", info.n2)); slots.push(s9_bind_slot(sc.base, sc.refm, 24, info.s2)); btypes.push(s9_variant_field_ty(env, &vn, 2)); }
        if info.n3.len() > 0 { names.push(format!("{}", info.n3)); slots.push(s9_bind_slot(sc.base, sc.refm, 32, info.s3)); btypes.push(s9_variant_field_ty(env, &vn, 3)); }
        if info.n4.len() > 0 { names.push(format!("{}", info.n4)); slots.push(s9_bind_slot(sc.base, sc.refm, 40, info.s4)); btypes.push(s9_variant_field_ty(env, &vn, 4)); }
        if info.n5.len() > 0 { names.push(format!("{}", info.n5)); slots.push(s9_bind_slot(sc.base, sc.refm, 48, info.s5)); btypes.push(s9_variant_field_ty(env, &vn, 5)); }
        let benv = S9Env { names: names, slots: slots, fnames: veccopy(&env.fnames), foffs: veccopy_i32(&env.foffs), ftypes: veccopy(&env.ftypes), frets_name: veccopy(&env.frets_name), frets_ty: veccopy(&env.frets_ty), types: btypes };
        let bodyval = s9expr(&arm.body, &benv);
        s9emit_arm_tail(rv, mlbl, bodyval);
        if refutable { s9emit_pool(); }
    }
    s9emit_match_label(mlbl);
    rv
}
// the value env for a fn body: name -> rzir slot (a struct so `&S9Env` lowers -- a &Vec<String> param does not, S5-G).
// name->slot value env, PLUS the program-global struct field-layout registry (fnames[i] -> foffs[i], the byte offset of
// that field name in its struct). The registry is immutable, built once in emit_s9, threaded via every env copy. SR-2
// LIMITATION: field names are assumed UNIQUE across structs (no per-binding type -> no S5 dependency); a shared field
// name across two structs would collide. Resolving that needs binding-type tracking (syntactic) or wiring S5.
struct S9Env { names: Vec<String>, slots: Vec<i32>, fnames: Vec<String>, foffs: Vec<i32>, types: Vec<String>, ftypes: Vec<String>, frets_name: Vec<String>, frets_ty: Vec<String> }
// C5-1b R4: the declared RETURN type of fn `name` ("" unknown/unit); a SEPARATE registry from fnames (a fn name must NOT
// enter fnames or s9_reg_known would route its call to enum construction).
fn s9_fn_ret_type(env: &S9Env, name: &str) -> String {
    let mut i = 0; let mut hit = -1; for n in env.frets_name.iter() { if streq(n, name) { hit = i; } i += 1; }
    if hit < 0 { return String::from(""); }
    let mut j = 0; let mut out = String::from(""); for t in env.frets_ty.iter() { if j == hit { out = format!("{}", t); } j += 1; } out
}
// C5-1b R3: the declared TYPE text of struct field `name` ("" if unknown/variant); parallel to fnames like foffs.
fn s9_field_type(env: &S9Env, name: &str) -> String {
    let mut i = 0; let mut hit = -1; for n in env.fnames.iter() { if streq(n, name) { hit = i; } i += 1; }
    if hit < 0 { return String::from(""); }
    let mut j = 0; let mut out = String::from(""); for t in env.ftypes.iter() { if j == hit { out = format!("{}", t); } j += 1; } out
}
// the field ident of a Field expr `x.f` (for print dispatch), "" for any non-Field.
fn s9_field_name(e: &ExprK) -> String { match e { ExprK::Field(_x, f) => format!("{}", f), _ => String::from("") } }
// is the dumped field type a BARE Box<T>/P<T> (the OUTERMOST constructor), not Option/Vec-wrapped? dump_ty renders
// "Ty Path Box<..>" / "Ty Path P<..>" (P = the Box alias) vs "Ty Path Option<..>". Check the byte right after "Ty Path " (idx 8)
// so an Option<Box>/Vec<P> field (matched via its Some/iter path, not `&field`) is NOT treated as a bare box. (C-G2D-BOX-FIELD-REF.)
fn s9_bare_box_ty(ft: &str) -> bool { let b = ft.as_bytes(); if b.len() >= 12 && b[8] == 66 && b[9] == 111 && b[10] == 120 && b[11] == 60 { return true; } if b.len() >= 10 && b[8] == 80 && b[9] == 60 { return true; } false }
// lower `&e`: normally identity (aggregates are address-valued), BUT if `e` is a `.field` access whose field is a bare Box<T>/P<T>
// (celled), deref the cell so `dump(&f.ty)` gets the inner &T (Box deref-coercion), not the cell pointer (C-G2D-BOX-FIELD-REF).
// `e` is used ONCE (a single &ExprK param match) to keep L6 move-check happy. (S9-F12-safe: param match, not an inline `&**box`.)
// `*e` must LOAD when e is a reference to a SCALAR: taken inline (`*&a`) or held in a binding the existing
// s9_init_ty Ref arm recorded as `Ty Ref scalar`. Everything else keeps the pre-existing s9_scrut_by_ref rule.
fn s9_deref_scalar(inner: &ExprK, env: &S9Env) -> bool {
    match inner {
        ExprK::Ref(_m, rin) => { let rn = s9_bare_name(rin); return s9_is_scalar_local(env, &rn); }
        ExprK::Path(p) => { let nm = dump_path(p); let t = s9_lookup_type(env, &nm); return str_contains(&t, "Ty Ref scalar"); }
        _ => {}
    }
    false
}
fn s9_is_scalar_local(env: &S9Env, nm: &str) -> bool {
    let t = s9_lookup_type(env, nm);
    if t.len() == 0 { return false; }
    if str_contains(&t, "scalarlit") { return true; }
    if str_contains(&t, "String") || str_contains(&t, "str") || str_contains(&t, "Vec") { return false; }
    if str_contains(&t, "i8") || str_contains(&t, "i16") || str_contains(&t, "i32") || str_contains(&t, "i64") { return true; }
    if str_contains(&t, "u8") || str_contains(&t, "u16") || str_contains(&t, "u32") || str_contains(&t, "u64") { return true; }
    if str_contains(&t, "usize") || str_contains(&t, "isize") || str_contains(&t, "bool") || str_contains(&t, "char") { return true; }
    false
}
fn s9_addr_of_local(env: &S9Env, nm: &str) -> i32 {
    let slot = s9lookup(env, nm);
    let a = s9fresh(); println!("  v{}: i32 = ref v{}", a, slot); a
}
fn s9_ref_of(e: &ExprK, env: &S9Env) -> i32 {
    match e { ExprK::Path(rp) => { let rn = dump_path(rp); if s9_is_scalar_local(env, &rn) { return s9_addr_of_local(env, &rn); } } _ => {} }
    let base = s9expr(e, env); match e { ExprK::Field(fe, fname) => { let sty = s9_recv_struct(fe, env); let ft = s9_field_type_q(env, &sty, fname); if s9_bare_box_ty(&ft) { s9_load_at(base, 0) } else { base } } _ => base } }   // SR-2: box-ness by the receiver struct's field type (C-G2D-FIELD-COLLISION)
// [emit-s9-vecstring-elem locale R3] element display-kind of `w[i]` where `w: Vec<String>/Vec<&str>` -> "String"/"str" so
// println!/format! of an indexed Vec element dispatch __print_str/__scat_s (not __pi on the descriptor pointer).
fn s9_elem_ty(env: &S9Env, e: &ExprK) -> String {
    match e {
        ExprK::Index(arr, _idx) => { let an = s9_bare_name(arr); let at = s9_lookup_type(env, &an); if str_contains(&at, "String") { String::from("String") } else if str_contains(&at, "str") { String::from("str") } else { String::from("") } }
        _ => String::from("")
    }
}
// C5-1b: the declared type text of binding `name` ("" if unknown); last-writer-wins over the parallel names/types vecs.
fn s9_lookup_type(env: &S9Env, name: &str) -> String {
    let mut i = 0; let mut hit = -1; for n in env.names.iter() { if streq(n, name) { hit = i; } i += 1; }
    if hit < 0 { return String::from(""); }
    let mut j = 0; let mut out = String::from(""); for t in env.types.iter() { if j == hit { out = format!("{}", t); } j += 1; } out
}
// the type text of a `let PAT: TY = ..` annotation (dump_ty), "" if unannotated. Init-inference is a later slice.
fn s9_let_ty(ty: &Option<P<Ty>>) -> String { match ty { Some(t) => dump_ty(t), None => String::from("") } }
// C5-1b R2 init-inference: an UNANNOTATED `let x = ..` infers its type from the init expr -- String::new()/format!/a string
// literal -> "String", Vec::new() -> "Vec". Takes &ExprK (S9-F12-safe: never `match &**box`; the caller passes the init).
// the value-type of a block: the type of its LAST tail expr-statement (last-writer-wins over the stmt walk). Used to infer a
// `let x = match/if { .. }` binding type from the block arms (C-G2D-MATCH-INIT-TY).
fn s9_block_tail_ty(env: &S9Env, b: &Block) -> String { let mut r = String::from(""); for s in b.stmts.iter() { match s { Stmt::Expr(e) => { r = s9_init_ty(env, e); } _ => {} } } r }
// C-G2D-DEFERRED-LET-UNBOUND: a DEFERRED binding `let x;` (declaration with NO initializer, assigned later in
// if/match arms) registered NO name, NO slot and NO type -- the three Stmt::Let binder sites were `None => {}`.
// Every later `x = e` and every read of `x` therefore resolved against an unbound name (slot 0 / a bare address),
// so an aggregate deferred binding rendered as its POINTER (`let mv; if .. { mv = String::from("A") } ..` printed
// 24117256, not "A"); in mac_register_arms that garbage rode into the mac_metavars registry and SEGV'd the crate
// dump. Fix: DECLARE the binding at the `let` -- a fresh zero-initialized slot, typed from the annotation when
// present, else from the FIRST assignment to `x` in this block. The type search recurses into if/else and match
// arms because that IS the deferred-let idiom (the value is produced in the arms, never at the declaration).
// One level up from C-G2D-MATCH-INIT-TY: that infers from an initializer EXPR, this from a later STATEMENT.
fn s9_defer_slot() -> i32 { let f = s9fresh(); let z = s9const(0); s9emit_setslot(f, z); f }
// annotation wins; else infer from the first assignment. A HELPER, not an inline value-if at the three binder
// sites: those walkers are the high-pressure fns of C-SCI-REFMATCH-L6-WALL, where a value-selection is what
// makes L6 truncate its own compilation.
fn s9_defer_pick(ann: String, b: &Block, nm: &str, env: &S9Env) -> String { if ann.len() > 0 { return ann; } s9_defer_ty(b, nm, env) }
fn s9_defer_ty(b: &Block, nm: &str, env: &S9Env) -> String {
    for st in b.stmts.iter() { let t = s9_defer_ty_stmt(st, nm, env); if t.len() > 0 { return t; } }
    String::from("")
}
fn s9_defer_ty_stmt(st: &Stmt, nm: &str, env: &S9Env) -> String {
    match st { Stmt::Semi(e) => s9_defer_ty_expr(e, nm, env), Stmt::Expr(e) => s9_defer_ty_expr(e, nm, env), _ => String::from("") }
}
fn s9_defer_ty_expr(e: &ExprK, nm: &str, env: &S9Env) -> String {
    match e {
        ExprK::Assign(l, r) => { let ln = s9_bare_name(l); if streq(&ln, nm) { s9_init_ty(env, r) } else { String::from("") } }
        ExprK::If(_c, t, els) => { let a = s9_defer_ty(t, nm, env); if a.len() > 0 { return a; } let mut r = String::from(""); for x in els.iter() { let u = s9_defer_ty_expr(x, nm, env); if u.len() > 0 { r = u; } } r }
        ExprK::Block(bk) => s9_defer_ty(bk, nm, env),
        ExprK::Match(_s, arms) => { let mut r = String::from(""); for a in arms.iter() { let t = s9_defer_ty_expr(&a.body, nm, env); if t.len() > 0 { r = t; } } r }
        _ => String::from("") }
}
fn s9_init_ty(env: &S9Env, e: &ExprK) -> String {
    match e {
        ExprK::Call(callee, _a) => { let cn = s9_callee_name(callee); let cls = s9_last_seg_str(&cn); if streq(&cls, "Some") { String::from("Option") } else if streq(&cls, "Ok") || streq(&cls, "Err") { String::from("Result") } else if streq(&cn, "String::new") { String::from("String") } else if streq(&cn, "String::from") { String::from("String") } else if streq(&cn, "Vec::new") { String::from("Vec") } else { s9_fn_ret_type(env, &cn) } }
        // The "later slice" the s9_let_ty comment defers: an unannotated `let a = A { v: 10 }` recorded NO type, so
        // a method call on `a` could not name its receiver. Recorded in the `Ty Path NAME` shape s9_struct_of_ty
        // parses, which is what the annotated path already produces.
        ExprK::Struct(sp, _fs, _base) => { let pn = path_base(&dump_path(sp)); if pn.len() > 0 { format!("Ty Path {}", pn) } else { String::from("") } }
        ExprK::Path(up) => { let un = dump_path(up); let key = format!("$ust#{}", un); if s9_reg_known(env, &key) { format!("Ty Path {}", un) } else { String::from("") } }
        ExprK::Str(_s) => String::from("String"),
        // C-S9-BOOL-DISPLAY: `let b = true;` recorded type "" so a later `println!("{}", b)` could not be seen
        // as a bool and printed 1/0. A comparison init (`let c = x > 1;`) has the same problem.
        ExprK::Bool(_b) => String::from("bool"),
        ExprK::Closure(_mv, _ps, _rt, _body) => { let h = s9_clos_hash(e); if s9clh_has(h) { String::from("closure") } else { String::from("") } },
        // C-S9-INT-LITERAL-UNTYPED: an unannotated `let x = 4;` recorded "" -- SEM0 could not tell a scalar local
        // from anything else, which blocks BOTH the `&mut`-on-a-scalar refusal and the celling fix
        // (C-S9-REF-AS-VALUE). The marker is deliberately NOT "i64": returning a real width steered the
        // type-directed op selection and made `let mut s = 0; for x in a.iter() { s += x; }` print an address
        // instead of 6 (measured). "scalarlit" matches none of "String"/"str"/"Vec"/"bool"/"i32"/"i64", so every
        // existing contains-based consumer behaves exactly as it did for "".
        ExprK::LitInt(_n) => String::from("scalarlit"),
        ExprK::LitFloat(_t) => String::from("f64"),   // CL-6: `let x = 1.5f64;` recorded "" -> printed 1500000000
        ExprK::Range(_l, _h, _i) => String::from("Range"),   // P-RANGE-EXPR: so `let r = 0..4; for i in r` can find the cell
        // CL-6: an arithmetic Bin over f64 operands is f64. `a + b` on two scaled locals is a scaled local, and
        // without this it printed 3750000000 instead of 3.75 -- the ADDITION was already right, only the type was
        // lost. Only the LEFT operand is consulted, and only when the op is arithmetic, to keep this off the
        // per-placeholder allocation path the bool-Display note warns about.
        ExprK::Bin(op, l, _r) => { if s9_is_bool_op(op) { String::from("bool") } else { let ln = s9_bare_name(l); let lt = s9_lookup_type(env, &ln); if str_contains(&lt, "f64") || str_contains(&lt, "f32") { String::from("f64") } else { let li = s9_init_ty(env, l); if str_contains(&li, "f64") { String::from("f64") } else { String::from("") } } } },

        ExprK::MacCall(p, _b, _a) => { let nm = dump_path(p); if streq(&nm, "format") { String::from("String") } else { String::from("") } }
        ExprK::Method(recv, name, _tys, _a) => { if streq(name, "as_bytes") { String::from("bytes") } else if streq(name, "clone") { let rn = s9_bare_name(recv); s9_lookup_type(env, &rn) } else { s9_fn_ret_type(env, name) } },   // clone inherits the receiver's type (C-G2D-MATCH-INIT-TY: else `let c = v.clone()` typed "" -> printed as int-address). to_string left as-is (blanket String would break int.to_string()).
        // `&EXPR` -- see through the ref (slice `&s[a..b]`, `&t`) WHEN the inner type is known. When it is NOT,
        // record `Ty Ref` instead of "": that is what makes `let r = &n; *r` deref correctly (C-S9-REF-LOCAL-DEREF).
        // See-through is preserved for every case that relied on it, since all of those have a known inner type.
        ExprK::Ref(_m, inner) => { let it = s9_init_ty(env, inner); if it.len() > 0 { if streq(&it, "scalarlit") { String::from("Ty Ref scalar") } else { it } } else { let rn = s9_bare_name(inner); if s9_is_scalar_local(env, &rn) { String::from("Ty Ref scalar") } else { String::from("Ty Ref") } } },   // C-S9-MUTREF-SCALAR: `scalar` distinguishes a reference to a SCALAR (which must LOAD on deref, now that `&a` is a real address) from a reference to an aggregate (already address-valued). str_contains("Ty Ref") consumers are unaffected.   // a ref to a LITERAL stays a ref
        ExprK::Index(_arr, idx) => { if is_range_expr(idx) { String::from("String") } else { String::from("") } },   // P-S8T-SLICE: `s[a..b]` materializes a String
        ExprK::Array(elems) => s9_arr_vty(env, elems),   // an array literal `[..]` is Vec-shaped (s9_array); record the element kind so `.iter()` binds by-value for aggregate elements
        // a `let x = match/if { .. }` (or a bare block) inherits its arms' value type (C-G2D-MATCH-INIT-TY): without this the
        // binding typed "" and format!/println! dispatched the String result as an INT -> printed its ADDRESS (e.g. the fn
        // dump's `rstr = match ret { .. None => String::from("\n Ret Default") }` rendered a heap address, not the text).
        ExprK::Match(_scrut, arms) => { let mut r = String::from(""); for a in arms.iter() { let t = s9_init_ty(env, &a.body); if t.len() > 0 { r = t; } } r }
        ExprK::If(_c, then, els) => { let t = s9_block_tail_ty(env, then); if t.len() > 0 { t } else { let mut r = String::from(""); for e0 in els.iter() { r = s9_init_ty(env, e0); } r } }
        ExprK::Block(b) => s9_block_tail_ty(env, b),
        _ => String::from("")
    }
}
// the recorded type text of an array literal, in the `Vec<Ty Path INNER>` shape s9_vec_elem_kind classifies (s9_array
// materializes an array as a Vec-like header, so `.iter()` over it walks the same slots). The element kind comes from the
// FIRST element: a Str/format!/String-init element -> aggregate (address-valued slots, bind by VALUE); otherwise scalar
// (the conservative default -- an aggregate array of non-Str elements is a latent gap, recorded C-G2D-ARRAY-ELEM-KIND).
fn s9_arr_vty(env: &S9Env, elems: &Vec<ExprK>) -> String { let mut et = String::from(""); for x in elems.iter() { et = s9_init_ty(env, x); break; } if str_contains(&et, "String") || str_contains(&et, "Vec") { String::from("Vec<Ty Path String>") } else { String::from("Vec<Ty Path i32>") } }
// is this expr a range `a..b` (for slice-index detection)? &ExprK param so a &Box<ExprK> idx auto-derefs (like s9_index).
fn is_range_expr(e: &ExprK) -> bool { match e { ExprK::Range(_a, _b, _i) => true, _ => false } }
// annotation wins; else infer from the init (leaf helper so the branch move stays out of the stmt-accumulation loop).
fn s9_pick_ty(env: &S9Env, ann: String, e: &ExprK) -> String { if ann.len() > 0 { ann } else { s9_init_ty(env, e) } }
// the bare single-ident name of a Path expr (for print dispatch), "" for any non-Path.
fn s9_bare_name(e: &ExprK) -> String { match e { ExprK::Path(p) => dump_path(p), ExprK::Ref(_m, inner) => s9_bare_name(inner), ExprK::Paren(inner) => s9_bare_name(inner), _ => String::from("") } }   // C-G2D-STR-REF-CLASSIFY: see through `&x`/`(x)` to x's name so a str-typed `&x` classifies as a string. Was opaque -> emit_s9 lowered `println!("call {}", &nm)` via __pi (print ADDRESS) not __print_str -> gen2 printed the callee name as a pointer
// the byte offset of struct field `name` (SR field-layout lookup; 0 if unknown -- last-writer-wins over the registry).
fn s9_field_off(env: &S9Env, name: &str) -> i32 {
    let mut i = 0; let mut hit = -1; for n in env.fnames.iter() { if streq(n, name) { hit = i; } i += 1; }
    if hit < 0 { return 0; }
    let mut j = 0; let mut out = 0; for o in env.foffs.iter() { if j == hit { out = *o; } j += 1; } out
}
// SR-2 field disambiguation (C-G2D-FIELD-COLLISION): a field name recurs across structs (`ty`/`ident`/..) at DIFFERENT
// offsets, and the bare last-writer registry mis-resolves it -> wrong offset -> SIGSEGV in the full dump. These helpers
// resolve a field by the RECEIVER's struct type when it is known.
// is `key` registered (exact)? (offset 0 is a valid field offset, so s9_field_off's 0-return is ambiguous with not-found.)
fn s9_field_has(env: &S9Env, key: &str) -> bool { let mut r = false; for n in env.fnames.iter() { if streq(n, key) { r = true; } } r }
// the bare struct-name of a type text ("Ty Path A" / "Ty Ref(Ty Path A)" -> "A"): the ident after the LAST "Ty Path ".
fn s9_struct_of_ty(t: &str) -> String { let b = t.as_bytes(); let mut cut = -1; let mut i = 0; while i + 8 <= b.len() { if b[i]==84 && b[i+1]==121 && b[i+2]==32 && b[i+3]==80 && b[i+4]==97 && b[i+5]==116 && b[i+6]==104 && b[i+7]==32 { cut = (i as i32) + 8; } i += 1; } if cut < 0 { return String::from(""); } let s = cut as usize; let mut j = s; while j < b.len() { let c = b[j]; if (c>=65 && c<=90)||(c>=97 && c<=122)||(c>=48 && c<=57)||c==95 { j += 1; } else { break; } } format!("{}", &t[s..j]) }
// the struct-type name of a field-access RECEIVER expr ("" if unknown): a Path var/param via its recorded type; a nested
// `o.f` via o's struct then f's field type (the qualified ftype). Kept to Path + one Field level (the dump receivers).
fn s9_chain_ret_struct(env: &S9Env, inner_ty: &str, mname: &str) -> String {
    if inner_ty.len() == 0 { return String::new(); }
    let key = format!("{}::{}", inner_ty, mname);
    let rt = s9_fn_ret_type(env, &key);
    s9_struct_of_ty(&rt)
}
fn s9_recv_struct(fe: &ExprK, env: &S9Env) -> String {
    match fe {
        ExprK::Struct(sp, _fs, _b) => path_base(&dump_path(sp)),   // `A { v: 10 }.get()` names its own type
        // `a.to_b().m()` -- a method-CHAIN receiver. Fully concrete: the inner receiver names its type, and the
        // method's return type is in the frets registry under `Ty::name` (registered qualified in rung 7). With
        // no arm here this returned "unknown" and a colliding name fell back to the bare form, which dangles.
        ExprK::Method(mr, mn, _mt, _ma) => { let it = s9_recv_struct(mr, env); s9_chain_ret_struct(env, &it, mn) }
        ExprK::Path(p) => { let nm = dump_path(p); let t = s9_lookup_type(env, &nm); s9_struct_of_ty(&t) }
        ExprK::Field(o, f2) => { let ost = s9_recv_struct(o, env); if ost.len() == 0 { String::from("") } else { let qk = format!("{}#{}", ost, f2); let ft = s9_field_type(env, &qk); s9_struct_of_ty(&ft) } }
        _ => String::from("")
    }
}
// the offset of field `fname` on struct `sname` (qualified `sname#fname` when registered, else the bare fallback).
fn s9_field_off_q(env: &S9Env, sname: &str, fname: &str) -> i32 { if sname.len() > 0 { let qk = format!("{}#{}", sname, fname); if s9_field_has(env, &qk) { return s9_field_off(env, &qk); } } s9_field_off(env, fname) }
// the ENUM segment of a `Enum::Variant` path (the segment before the last `::`), "" if unqualified.
fn s9_enum_seg(s: &str) -> String { let b = s.as_bytes(); let mut last = 0; let mut prev = 0; let mut i = 0; while i + 1 < b.len() { if b[i] == 58 && b[i+1] == 58 { prev = last; last = i + 2; } i += 1; } if last == 0 { return String::from(""); } format!("{}", &s[prev..(last - 2)]) }
// the TAG (variant index) of a variant path, qualified by its ENUM (`Enum#Variant`) when known, else the bare fallback.
// C-G2D-VARIANT-TAG-COLLISION: a variant name shared across enums (ExprK::Path idx 2 vs Ty::Path idx 0) got the bare
// last-writer index -> a value constructed/matched with the wrong tag mis-dispatched (ExprK::Path read as ExprK::LitInt).
fn s9_enum_tag(env: &S9Env, fullpath: &str) -> i32 { let v = s9_last_seg_str(fullpath); if s9_qualified(fullpath) { let e = s9_enum_seg(fullpath); let qk = format!("{}#{}", e, v); if s9_field_has(env, &qk) { return s9_field_off(env, &qk); } } s9_field_off(env, &v) }
// the declared TYPE of field `fname` on struct `sname` (qualified when registered, else the bare fallback). SR-2 dual of
// s9_field_off_q: `ty` recurs across structs at different box-ness, so the bare last-writer mis-detects a box (C-G2D-FIELD-COLLISION).
fn s9_field_type_q(env: &S9Env, sname: &str, fname: &str) -> String { if sname.len() > 0 { let qk = format!("{}#{}", sname, fname); if s9_field_has(env, &qk) { return s9_field_type(env, &qk); } } s9_field_type(env, fname) }
// the offset of field `fname` on receiver `fe`: qualified by the receiver struct when known, else the bare fallback.
fn s9_field_off_r(env: &S9Env, fe: &ExprK, fname: &str) -> i32 { let sty = s9_recv_struct(fe, env); s9_field_off_q(env, &sty, fname) }
// build the base env carrying the struct field registry (each struct's fields at byte offset i*4, all-i32 stride).
fn s9_build_reg(c: &Krate) -> S9Env {
    let mut fnames: Vec<String> = Vec::new(); let mut foffs: Vec<i32> = Vec::new(); let mut ftypes: Vec<String> = Vec::new();
    let mut frets_name: Vec<String> = Vec::new(); let mut frets_ty: Vec<String> = Vec::new();
    // fn-return-type registry (C5-1b R4): top-level fns + impl-method fns -> ident -> dump_ty(ret). Separate from fnames.
    for it in c.items.iter() {
        match &it.kind {
            ItemKind::Fn(_g, _p, ret, _b, _a) => { frets_name.push(format!("{}", it.ident)); frets_ty.push(s9_let_ty(ret)); }
            // Impl fn return types were registered under the BARE name only, so `let a = A::new()` (whose callee
            // name is qualified) missed, and two types both defining `new` overwrote each other last-writer-wins.
            // Registered under BOTH names; the qualified one is what the Call init-inference arm looks up.
            ItemKind::Impl(_g2, _tr, ty2, items) => { let tn2 = path_base(&hir_ty(ty2)); for m in items.iter() { match &m.kind { ItemKind::Fn(_gf, _pf, mret, _bf, _af) => { frets_name.push(format!("{}", m.ident)); frets_ty.push(s9_let_ty(mret)); frets_name.push(format!("{}::{}", tn2, m.ident)); frets_ty.push(s9_let_ty(mret)); } _ => {} } } }
            _ => {}
        }
    }
    // builtin prelude enum Option { None, Some(T) } -- None=tag 0, Some=tag 1 (rustc declaration order). Registered so
    // Some(x)/None construct + match reuse the SR-3 enum layout directly (E4). Result rides here too when needed.
    fnames.push(format!("None")); foffs.push(0); ftypes.push(String::from(""));
    fnames.push(format!("Some")); foffs.push(1); ftypes.push(String::from(""));
    // P-TRY, step 1 (locales/rustc-crate-targeting/substrate-pipeline-oracle.md). `Result { Ok(T), Err(E) }` --
    // Ok=0, Err=1 in rustc declaration order. The note above said "Result rides here too when needed"; it is
    // needed now, and it was NOT registered: measured, `if let Ok(v) = r` produced NO OUTPUT while the identical
    // shape on Option printed correctly.
    // This REFUTES part of the oracle's own classification. P-TRY was probed as pure ROUTING using a
    // hand-declared `enum O { S(i64), N }`, which cannot see whether the BUILTIN registry covers Result -- and
    // `?` in rustc is overwhelmingly on Result, not Option. The probe established something true and not the
    // thing needed, which is the CL-A5 hazard at a new coordinate.
    fnames.push(format!("Ok")); foffs.push(0); ftypes.push(String::from(""));
    fnames.push(format!("Err")); foffs.push(1); ftypes.push(String::from(""));
    // P-REPEAT: top-level integer `const NAME: T = <int literal>` values, keyed `$c#NAME`. There was no const
    // VALUE registry at all -- consts were parsed and dumped but never available to the lowering -- and the
    // dominant real array-repeat shape is `[0; BYTE_LEN]`, a repeat whose count is a const NAME. Registering the
    // value is what turns that from a refusal into an unroll.
    for it in c.items.iter() {
        match &it.kind {
            ItemKind::Const(_t, rhs) => { match rhs { Some(e) => { let k = s9_litexpr_kind(e); if k == 1 { fnames.push(format!("$c#{}", it.ident)); foffs.push(s9_litexpr_val(e)); ftypes.push(String::from("")); } } None => {} } }
            _ => {}
        }
    }
    for it in c.items.iter() {
        match &it.kind {
            ItemKind::Struct(_g, fields) => { let mut nf = 0; for _q in fields.iter() { nf += 1; } fnames.push(format!("$nflds#{}", it.ident)); foffs.push(nf); ftypes.push(String::from("")); let mut i = 0; for f in fields.iter() { let ft = dump_ty(&f.ty); fnames.push(format!("{}#{}", it.ident, f.ident)); foffs.push(i * 8); ftypes.push(format!("{}", ft)); fnames.push(format!("{}", f.ident)); foffs.push(i * 8); ftypes.push(ft); i += 1; } }   // SR-2: a QUALIFIED `Struct#field` entry (disambiguates a field-name shared across structs at different offsets) PLUS the bare `field` fallback (last-writer, for an unknown receiver type). C-G2D-FIELD-COLLISION.
            ItemKind::Enum(_g, variants) => { let mut i = 0; for v in variants.iter() { let vf = s9_variant_flags(v); fnames.push(format!("{}", v.ident)); foffs.push(i); ftypes.push(format!("{}", vf)); fnames.push(format!("{}#{}", it.ident, v.ident)); foffs.push(i); ftypes.push(format!("{}", vf)); match &v.kind { VariantKind::Tuple(tys) => { let mut j = 0; for t in tys.iter() { let ft = dump_ty(t); fnames.push(format!("{}#{}", v.ident, j)); foffs.push(j); ftypes.push(format!("{}", ft)); fnames.push(format!("{}#{}#{}", it.ident, v.ident, j)); foffs.push(j); ftypes.push(ft); j += 1; } }, _ => {} } i += 1; } }   // SR-2: `Enum#Variant` -> the variant's TAG (index), disambiguating a variant name shared across enums (C-G2D-VARIANT-TAG-COLLISION)   // SR-2: register each tuple-variant field's TYPE TEXT keyed BOTH `Variant#i` (bare, last-writer) AND `Enum#Variant#i` (qualified) so a match payload binding records its real type even when a variant NAME is shared across enums (C-G2D-VARIANT-FIELD-TY-COLLISION: ItemKind::Struct#0=Vec<GParam> vs VariantKind::Struct#0=Vec<Field>)
            // A method emitted BARE (it has `&self`) is registered `$m#NAME`, so a UFCS call site that wrote the
            // QUALIFIED path can find it. This is the exact inverse of C-S9-ASSOC-FN-NAME-MISMATCH recorded in
            // emit_s9: self-LESS impl fns are emitted qualified because the call site qualifies them; a `&self`
            // method is emitted bare, and `S::m(&s)` then emitted `call S::m` against a definition named `m`.
            // The backend links a dangling call without erroring, so it printed garbage instead of failing.
            ItemKind::Impl(_gi, _tri, _tyi, mitems) => { for mm in mitems.iter() { match &mm.kind { ItemKind::Fn(_gm, mparams, _rm, _bm, _am) => { let mut hs = false; for pp in mparams.iter() { if is_self_param(&pp.name) { hs = true; } } if hs { let n = s9_method_defs(c, &mm.ident); fnames.push(format!("$m#{}", mm.ident)); foffs.push(n); ftypes.push(String::from("")); if n > 1 { fnames.push(format!("$mdup#{}", mm.ident)); foffs.push(n); ftypes.push(String::from("")); } } } _ => {} } } }
            ItemKind::UnitStruct(_gu) => { fnames.push(format!("$ust#{}", it.ident)); foffs.push(0); ftypes.push(String::from("")); }
            ItemKind::Static(ty, _init) => { fnames.push(format!("$s#{}", it.ident)); foffs.push(0); ftypes.push(String::from("")); if s9_static_bytes(ty) > 4 { fnames.push(format!("$sa#{}", it.ident)); foffs.push(0); ftypes.push(String::from("")); } }   // $sa# marks an ARRAY-typed static (flat block, 8-byte stride)   // C-G2D-STATIC-MUT: register each top-level `static [mut]` NAME so a bare Path read/write routes to the GLOBAL, not a local slot
            _ => {}
        }
    }
    S9Env { names: Vec::new(), slots: Vec::new(), fnames: fnames, foffs: foffs, types: Vec::new(), ftypes: ftypes, frets_name: frets_name, frets_ty: frets_ty }
}
// is `name` a registered field/variant? (used to route a qualified path to enum construction, not a fn call / var lookup).
fn s9_reg_known(env: &S9Env, name: &str) -> bool { let mut hit = false; for n in env.fnames.iter() { if streq(n, name) { hit = true; } } hit }
// C-G2D-STATIC-MUT: is `name` a top-level `static [mut]`? (registered `$s#NAME` in s9_build_reg). A bare Path that is a
// static reads/writes the GLOBAL slot; without this it fell to s9lookup (returned 0) and s9_assign wrote through a null
// `ref v0` -> gen2's own S9NC/S9LBL/S9BV counters never persisted (every SSA temp v0 -> collisions -> SIGSEGV at scale).
// Bytes to reserve for a `static`: an array reserves 8 per element (the stride the global accessors use), a
// scalar reserves 4 as before. A non-literal length falls back to 4 -- conservative, and it keeps the previous
// behaviour rather than guessing a size.
fn s9_static_bytes(t: &Ty) -> i64 {
    match t {
        Ty::Array(_e, n) => { let k = s9_litexpr_val(n); if k > 0 { (k as i64) * 8 } else { 4 } }
        _ => 4
    }
}
// is `nm` a static whose declared type is an array? (indexed statics address a FLAT block, not the
// [len@0,cap@4,dataptr@8] header a Vec/array LOCAL uses)
fn s9_static_arr(env: &S9Env, nm: &str) -> bool { let key = format!("$sa#{}", nm); s9_reg_known(env, &key) }
// address of `A[i]` for a flat static array: gref A + i*8
fn s9_static_elem_addr(nm: &str, iv: i32) -> i32 {
    let g = s9fresh(); println!("  v{}: i32 = gref {}", g, nm);
    let e = s9const(8); let off = s9op("mul", iv, e);
    s9op("add", g, off)
}
// C-S9-TOPLEVEL-CONST-READ: a top-level `const K: i32 = 70` was REGISTERED (`$c#K`, for the [x; n] repeat
// count) but never READ back in expression position, so `println!("{}", K)` fell through to s9lookup, found no
// local slot, and printed a raw heap pointer. Two predicates rather than one lookup returning a sentinel: a
// vreg id is a non-negative i32 with no spare value to mean "absent".
fn s9_is_constitem(env: &S9Env, name: &str) -> bool { let key = format!("$c#{}", name); s9_field_has(env, &key) }
fn s9_constitem_val(env: &S9Env, name: &str) -> i32 { let key = format!("$c#{}", name); let v = s9_field_off(env, &key); s9const(v as i64) }
// C-S9-STATIC-NO-INIT: `gdef NAME n` reserves a ZEROED slot, which was right for every static s1_parser itself
// writes (all `= 0`) and wrong for every other program -- `static K: i32 = 69` read back 0. The initializer is
// stored at entry to `main`, which is the first point at which stores can be emitted at all.
fn s9_static_inits(c: &Krate) {
    for it in c.items.iter() {
        match &it.kind {
            ItemKind::Static(_ty, init) => { match init { Some(e) => { let k = s9_litexpr_kind(e); if k == 1 { let v = s9_litexpr_val(e); let cv = s9const(v as i64); s9_global_store(&it.ident, cv); } } None => {} } }
            _ => {}
        }
    }
}
// C-S9-BARE-METHOD-COLLISION: a `&self` method is emitted BARE, so two types implementing the same trait
// method both emit `fn m`, ld dedups by name (correct for genuine duplicates), and EVERY call reaches the
// first. Measured: `a.m()` then `b.m()` on two impls returning 8 and 6 printed `88`, silently.
// A name defined by more than one impl is emitted QUALIFIED instead and dispatched by receiver type. Names
// defined once keep the bare form unchanged, which is the overwhelming majority and keeps this rung's blast
// radius off every program that never collides.
fn s9_method_defs(c: &Krate, name: &str) -> i32 {
    let mut n = 0;
    for it in c.items.iter() {
        match &it.kind {
            ItemKind::Impl(_g, _t, _ty, items) => { for m in items.iter() { match &m.kind { ItemKind::Fn(_gf, ps, _r, _b, _a) => { let mut hs = false; for pp in ps.iter() { if is_self_param(&pp.name) { hs = true; } } if hs { if streq(&m.ident, name) { n += 1; } } } _ => {} } } }
            _ => {}
        }
    }
    n
}
fn s9_method_dup(env: &S9Env, name: &str) -> bool { let key = format!("$mdup#{}", name); s9_reg_known(env, &key) }
// The receiver's struct type, when this tier can name it. Empty means "unknown", and an unknown receiver on a
// COLLIDING name emits the bare name, which will dangle -- visible to dangle.sh -- rather than mis-dispatch.
fn s9_method_target(recv: &ExprK, name: &str, env: &S9Env) -> String {
    if s9_method_dup(env, name) == false { return String::from(name); }
    let ty = s9_recv_struct(recv, env);
    if ty.len() == 0 { return String::from(name); }
    format!("{}::{}", ty, name)
}
fn s9_struct_count(sname: &str, env: &S9Env) -> i32 {
    let key = format!("$nflds#{}", sname);
    if s9_field_has(env, &key) { return s9_field_off(env, &key); }
    0
}
fn s9_offset_listed(sfields: &Vec<EField>, sname: &str, off: i32, env: &S9Env) -> bool {
    let mut hit = false;
    for f in sfields.iter() { let o = s9_field_off_q(env, sname, &f.name); if o == off { hit = true; } }
    hit
}
fn s9_struct_copy_rest(sname: &str, sfields: &Vec<EField>, dst: i32, src: i32, env: &S9Env) {
    let n = s9_struct_count(sname, env);
    let mut i = 0;
    while i < n {
        let off = i * 8;
        if s9_offset_listed(sfields, sname, off, env) == false {
            let v = s9_slot_load(src, off);
            s9_slot_store(dst, off, v);
        }
        i += 1;
    }
}
// C-S9-STRUCT-UPDATE-BASE. A separate path taken only when the literal carries `..base`; s9_struct is left
// byte-identical for every other literal.
// The blocker was NOT any of the five recorded landmines checked first -- it is ARITY. `s9_struct_copy_rest`
// with SIX parameters made L6 miscompile this function even when it was never called, and gen1 then
// segfaulted on every input including witnesses with no struct at all. At FIVE parameters it compiles.
// Bisected to that single call: identical body with a 5-arg call is 52/52, with the 6-arg call 0/52.
fn s9_struct_upd(sname: &str, sfields: &Vec<EField>, base_expr: &ExprK, env: &S9Env) -> i32 {
    let n = s9_struct_count(sname, env);
    let dst = s9_alloc(n * 8);
    for f in sfields.iter() { let off = s9_field_off_q(env, sname, &f.name); let v = s9expr(&f.expr, env); s9_slot_store(dst, off, v); }
    let src = s9expr(base_expr, env);
    s9_struct_copy_rest(sname, sfields, dst, src, env);
    dst
}
fn s9_is_static(env: &S9Env, name: &str) -> bool { let key = format!("$s#{}", name); s9_reg_known(env, &key) }
fn s9_global_load(name: &str) -> i32 { let g = s9fresh(); println!("  v{}: i32 = gref {}", g, name); let r = s9fresh(); println!("  v{}: i32 = load v{}", r, g); r }
fn s9_global_store(name: &str, rv: i32) { let g = s9fresh(); println!("  v{}: i32 = gref {}", g, name); println!("  store8 v{} v{}", g, rv); }
// enum variant construction (D3): tag@0 (a 4-byte header field in an 8-byte reserved slot), payload fields at
// 8, 16, 24, ... -- a UNIFORM 8-byte stride, matching s9_slot_store/s9_slot_load and the binder's 8/16/24/32/40/48.
// CORRECTED 2026-08-25: this said "payload(s) @4,+4.." which NEITHER this tier NOR L6 implements -- L6 uses
// disc@0 + payload@4 with a per-field-width stride (vpay_off), and this tier uses the @8/stride-8 layout below.
// The stale comment misled the whole C-S9-VARIANT-DISC1-ARITY5 investigation. See substrate-representation-oracle.md.
// A try-block's value is the block's value wrapped in `Ok`: the same [tag@0, payload@8] cell s9_enum_new
// builds, with the Ok tag 0 the registry records. A `?` INSIDE the block still propagates out of the enclosing
// FN rather than out of the block -- recorded, not built.
fn s9_ok_wrap(v: i32) -> i32 {
    let cell = s9_alloc(16);
    let z = s9const(0);
    s9_store_at(cell, 0, z);
    s9_slot_store(cell, 8, v);
    cell
}
fn s9_enum_new(env: &S9Env, vpath: &str, args: &Vec<ExprK>) -> i32 {
    let tag = s9_enum_tag(env, vpath);   // SR-2: enum-qualified variant tag (C-G2D-VARIANT-TAG-COLLISION)
    // reserve tag@0 + one word PER field BEFORE evaluating any arg (C-G2D-NESTED-OPTION-PAYLOAD). The old fixed s9_alloc(8)
    // held only tag + 1 field; a 2+-field variant's later fields stored PAST the block, and since each arg's own s9expr may
    // heap-alloc right after base, `K::F(9, Some(..))` stored field 1 on top of the nested Some block -> corruption. Sizing
    // the block to all fields keeps them disjoint from the arg allocations.
    let mut nf = 0; for _a in args.iter() { nf += 1; }
    let base = s9_alloc(8 + nf * 8);
    let tv = s9const(tag as i64); s9_store_at(base, 0, tv);   // tag stays a 4-byte header field at offset 0
    let mut off = 8;
    for a in args.iter() { let pv = s9expr(a, env); s9_slot_store(base, off, pv); off += 8; }
    base
}
// unit enum variant (E::B): tag@0 only.
fn s9_enum_unit(env: &S9Env, vpath: &str) -> i32 { let tag = s9_enum_tag(env, vpath); let base = s9_alloc(8); let tv = s9const(tag as i64); s9_store_at(base, 0, tv); base }   // SR-2: enum-qualified tag (C-G2D-VARIANT-TAG-COLLISION)
// Box::new(x) (SR-6): heap-alloc a 4-byte cell, store x@0, value = the pointer. In the 4-byte model x is a scalar value
// OR an aggregate ADDRESS (both 4 bytes), so no size_of is needed. `*b` (Deref) = load(b, 0).
fn s9_box_new(args: &Vec<ExprK>, env: &S9Env) -> i32 { let base = s9_alloc(8); for a in args.iter() { let v = s9expr(a, env); s9_slot_store(base, 0, v); } base }
// array index `a[idx]` (D-tier): RUNTIME offset = idx*4 (unlike the compile-time offsets of tuple/struct/enum). base and
// idx each lowered once (S9-F15). All-i32 elements (4-byte stride). Array LITERAL `[..]` reuses s9_tuple (same layout).
// indexed access a[i] / v[i] over the UNIFIED [len@0, data@4+] layout (arrays + Vec share it): addr = base + 4 + i*4.
// index v[i]/a[i] (E1-GROW/SR-5): Vec+array share the header [len@0, cap@4, dataptr@8]; the data is a SEPARATE buffer at
// dataptr. addr = load(base,8) + i*4. (String indexing is not this path -- it uses the inline byte accessors.)
fn s9_index_byte(base: i32, iv: i32) -> i32 {
    let dp = s9_load_at(base, 8); let addr = s9op("add", dp, iv);   // C-G2D-STR-GROW: bytes live at dataptr (base is the String/Vec<u8> header), not base+4
    let r = s9fresh(); println!("  v{}: i32 = loadb v{}", r, addr); r
}
fn s9_index_word(base: i32, iv: i32) -> i32 {
    let dp = s9_load_at(base, 8);
    let f1 = s9const(8); let ioff = s9op("mul", iv, f1);
    let addr = s9op("add", dp, ioff);
    let r = s9fresh(); println!("  v{}: i64 = load8 v{}", r, addr); r
}
// P-S8T-SLICE: `&s[a..b]` materialized as a fresh String = byte-copy s.bytes[a..b]. LEAF (no cycle call) so base/a/b/ns
// are held only in a non-cycle fn (S9-F7-safe). src = base+4+a, dst = ns+4, n = (incl?b+1:b) - a; then ns.len = n.
fn s9_slice_emit(base: i32, a: i32, hb: i32, incl: bool) -> i32 {
    let b0 = if hb < 0 { s9_vec_len(base) } else { hb };   // hb<0 sentinel = open upper bound -> s.len() (base is a PARAM, reuse safe)
    let hib = if incl { let one = s9const_fresh(1); s9op("add", b0, one) } else { b0 };
    let newlen = s9op("sub", hib, a);
    let ns = s9_str_new();
    let rg = s9fresh(); println!("  v{}: i32 = call __str_grow v{} v{}", rg, ns, newlen);   // C-G2D-STR-GROW: ensure ns cap before copy
    let sdp = s9_load_at(base, 8); let src = s9op("add", sdp, a);   // src = dataptr(base) + a
    let dst = s9_load_at(ns, 8);                                    // dst = dataptr(ns) (len 0)
    let r = s9fresh(); println!("  v{}: i32 = call __scat_at v{} v{} v{}", r, dst, src, newlen);
    s9_store_at(ns, 0, newlen);
    ns
}
fn s9_index(arr: &ExprK, idx: &ExprK, env: &S9Env) -> i32 {
    match idx {
        // slice `s[a..b]` -> materialize a String. lo/hi via Option.iter() (0-or-1, Ret precedent; avoids match-&*box).
        // open ranges default a=0 / b=s.len(); base held across the bound lowerings (<=2 values, within the lean budget).
        ExprK::Range(lo, hi, incl) => {
            let base = s9expr(arr, env);
            let a = match lo { Some(e) => s9expr(e, env), None => s9const(0) };
            let hb = match hi { Some(e) => s9expr(e, env), None => 0 - 1 };   // -1 = open upper bound; base used ONCE (the call) -> no S9-F13
            s9_slice_emit(base, a, hb, *incl)
        }
        _ => {
            let base = s9expr(arr, env);
            let iv = s9expr(idx, env);
            let aty = s9_lookup_type(env, &s9_bare_name(arr));
            if str_contains(&aty, "bytes") || str_contains(&aty, "u8") { s9_index_byte(base, iv) } else { s9_index_word(base, iv) }
        }
    }
}
// array literal [a,b,c] (E1-GROW/SR-5): the header [len@0, cap=len@4, dataptr@8] + a separate n-slot buffer. Arrays are
// fixed-size (never grown) but share the header so s9_index/s9_vec_len serve both arrays and Vecs uniformly.
// P-REPEAT (substrate-pipeline-oracle.md rank 4, 33 occurrences / 25 files). `[x; n]` had NO ARM in s9expr at
// all -- it fell to the silent-zero catch-all regardless of what the count was. Probed before costing: the
// classification holds, this is ROUTING. `[7,7,7]` already lowers through s9_array and `vec![7; 3]` already
// unrolls; only `ExprK::Repeat` was never wired to either.
// The count resolves as a literal, or as a registered top-level integer const (`$c#NAME`, added in s9_build_reg
// for the `[0; BYTE_LEN]` shape). A count that resolves to neither REFUSES, which is what the `vec!` path
// already does for the same situation (C-S9-VECMAC-REPEAT-DYN) -- consistent, and an honest failure rather than
// a silently-empty array.
// The guards live HERE, not in the caller, so the returned count is used exactly ONCE downstream. Written the
// other way -- `let n = s9_repeat_count(..)` followed by two range checks and a call -- L6 rejected the file with
// `use of moved value` and truncated its emission at 178,357 rzir lines, stopping inside `s9_repeat`. The
// boundary is narrower than "an i32 used twice": the `vec!` path does `let n = s9_litexpr_val(&ce); if n > 4096`
// and compiles, so TWO uses are fine and THREE are not. Keeping the producer responsible for validity is the
// shape that avoids having to know exactly where the line is.
// Returns the STATIC count, or -1 when there is none. It no longer refuses: a count that is not a literal or
// a registered const is now lowered at runtime instead (CT-11), so "unresolvable" is a routing decision here
// rather than a failure. The > 4096 cap still applies to the UNROLL path only -- the runtime path has no cap
// because it emits a loop rather than n stores.
fn s9_repeat_count(ce: &ExprK, env: &S9Env) -> i64 {
    let mut c: i64 = 0 - 1;
    if s9_litexpr_kind(ce) == 1 { c = s9_litexpr_val(ce) as i64; }
    else { let nm = s9_bare_name(ce); if nm.len() > 0 { let key = format!("$c#{}", nm); if s9_field_has(env, &key) { c = s9_field_off(env, &key) as i64; } } }
    if c > 4096 { return 0 - 1; }
    c
}
// The element is evaluated ONCE and its slot stored n times, which is Rust's own rule for `[x; n]` -- the value
// is copied, not the expression re-run. That also avoids cloning AST nodes, which this file has no helper for.
// The body deliberately mirrors s9_array's, which is known to compile on this tier.
// The static/runtime branch lives one frame down: with the count bound here, tested, and then passed on, it is
// a call result used twice ACROSS an early return, and L6 rejects the file with `use of moved value` and stops
// emitting inside this function (gen1 came out 1.70MB instead of 2.77MB). As a PARAMETER it is unrestricted.
// Fourth occurrence of C-L6-CALLRESULT-3USE; the recorded rule named the fix before it was tested each time.
fn s9_repeat(elem: &ExprK, count: &ExprK, env: &S9Env) -> i32 {
    let n = s9_repeat_count(count, env);
    s9_repeat_pick(elem, count, env, n)
}
fn s9_repeat_pick(elem: &ExprK, count: &ExprK, env: &S9Env, n: i64) -> i32 {
    if n < 0 { return s9_repeat_rt(elem, count, env); }
    let v = s9expr(elem, env);
    s9_repeat_fill(v, n)
}
fn s9_repeat_rt(elem: &ExprK, count: &ExprK, env: &S9Env) -> i32 {
    let cntv = s9expr(count, env);
    let v = s9expr(elem, env);
    s9_repeat_fill_dyn(v, cntv)
}
// `v` crosses a PARAMETER boundary here. It is a call result reused inside a loop, which is the shape the
// sharpened L6 landmine forbids (an i32 call result passed to more than one call); as a parameter it is
// unrestricted, exactly as s9_vec_iter_bounds' base is. Measured: written inline, L6 rejected the whole file
// with `use of moved value` and truncated its emission to 178,357 rzir lines.
// CT-11 / P-VECMAC-DYN: a repeat whose count is only known at RUNTIME. s9_repeat_fill below is a COMPILE-TIME
// unroll -- its `while i < cnt` runs inside the compiler and emits cnt stores -- so it cannot serve a count
// that is a local, a param or a field, which is 22 of the 24 sites measured in the refusing files.
// Allocation with a runtime size: identical to s9_alloc except the bump is a vreg rather than a const.
fn s9_alloc_dyn(szv: i32) -> i32 {
    let cur = s9_gcur();
    let off = s9fresh(); println!("  v{}: i32 = load v{}", off, cur);
    let hb = s9_gheap();
    let base = s9fresh(); println!("  v{}: i32 = add v{} v{}", base, hb, off);
    let nc = s9fresh(); println!("  v{}: i32 = add v{} v{}", nc, off, szv);
    println!("  store v{} v{}", cur, nc);
    base
}
// The emitted fill loop, in the `or`/`or8` slot-reuse shape the hand-written __scat_at/__print_str_at runtime
// helpers already witness: a cursor and a countdown, both rebound through `or` each iteration.
fn s9_fill_loop(buf: i32, cntv: i32, v: i32) {
    let p = s9fresh(); println!("  v{}: i32 = or v{} v{}", p, buf, buf);
    let i = s9fresh(); println!("  v{}: i32 = or v{} v{}", i, cntv, cntv);
    s9emit_loop_open();
    let z = s9const(0);
    let more = s9op("gt", i, z);
    s9emit_brz(more);
    println!("  store8 v{} v{}", p, v);
    let eight = s9const(8);
    let np = s9fresh(); println!("  v{}: i64 = add8 v{} v{}", np, p, eight);
    println!("  v{}: i32 = or v{} v{}", p, np, np);
    let one = s9const(1);
    let ni = s9fresh(); println!("  v{}: i64 = sub8 v{} v{}", ni, i, one);
    println!("  v{}: i32 = or v{} v{}", i, ni, ni);
    s9emit_loop_close();
}
fn s9_repeat_fill_dyn(v: i32, cntv: i32) -> i32 {
    let hdr = s9_alloc(3 * 4);
    let eight = s9const(8);
    let bytes = s9op("mul", cntv, eight);
    let buf = s9_alloc_dyn(bytes);
    s9_store_at(hdr, 0, cntv);
    s9_store_at(hdr, 4, cntv);
    s9_store_at(hdr, 8, buf);
    s9_fill_loop(buf, cntv, v);
    hdr
}
fn s9_repeat_fill(v: i32, n: i64) -> i32 {
    let cnt = n as i32;
    let hdr = s9_alloc(3 * 4);
    let buf = s9_alloc(cnt * 8);
    let lenc = s9const(n); s9_store_at(hdr, 0, lenc);
    let capc = s9const(n); s9_store_at(hdr, 4, capc);
    s9_store_at(hdr, 8, buf);
    let mut off = 0;
    let mut i = 0;
    while i < cnt { s9_slot_store(buf, off, v); off += 8; i += 1; }
    hdr
}
fn s9_array(elems: &Vec<ExprK>, env: &S9Env) -> i32 {
    let mut slots: Vec<i32> = Vec::new(); let mut n = 0;
    for e in elems.iter() { let s = s9expr(e, env); slots.push(s); n += 1; }
    let hdr = s9_alloc(3 * 4);
    let buf = s9_alloc(n * 8);
    let lenc = s9const(n as i64); s9_store_at(hdr, 0, lenc);
    let capc = s9const(n as i64); s9_store_at(hdr, 4, capc);
    s9_store_at(hdr, 8, buf);
    let mut off = 0;
    for s in slots.iter() { s9_slot_store(buf, off, *s); off += 8; }
    hdr
}
// Vec::new() (E1-GROW/SR-5): a growable Vec = a 3-word header [len=0@0, cap@4, dataptr@8]; value = the header address
// (STABLE across grow -- only dataptr moves, so callers holding the Vec value never rebind). Initial cap = 4 (small: most
// Vecs are small; push grows on demand via __vec_grow, which also shrinks per-Vec peak vs the old fixed-64 block).
fn s9_vec_new() -> i32 {
    let hdr = s9_alloc(3 * 4);
    let buf = s9_alloc(4 * 8);
    let z = s9const(0); s9_store_at(hdr, 0, z);
    let c = s9const(4); s9_store_at(hdr, 4, c);
    s9_store_at(hdr, 8, buf);
    hdr
}
// String::new() (E2, DECOUPLED from Vec at E1-GROW): String stays the inline [len@0, bytes@4+] block (its byte accessors
// s9_str_dataend/push_str read base+4 directly); Vec moved to the header model, so String::new gets its own constructor.
// C-G2D-STR-GROW: String is now the SAME growable header as Vec -- [len@0, cap@4, dataptr@8], bytes at dataptr (byte
// stride). Stable header address; only dataptr moves on grow. Was a fixed 256-byte inline block [len@0, bytes@4+] with no
// grow -> any built string >256 bytes overflowed adjacent heap (P25d). Initial cap 256 bytes.
// INITIAL CAP 64, not 256 (2026-08-26). The identical lever was measured on L6 and is recorded in the arena
// oracle: a fixed per-String block out of a no-free BUMP arena is paid in full by every short String, and this
// file is saturated with them. L6 went 4096 -> 64 and its self-emit fell from 1,902,336 to 897,280 kB with a
// byte-identical image. THE SIBLING TIER HAD THE SAME CLASS and it was never checked -- which is exactly what
// substrate-representation-oracle.md exists to prevent. Growth is already handled by __str_grow (cap*2).
fn s9_str_new() -> i32 {
    let hdr = s9_alloc(3 * 4);
    let buf = s9_alloc(64);
    let z = s9const(0); s9_store_at(hdr, 0, z);
    let c = s9const(64); s9_store_at(hdr, 4, c);
    s9_store_at(hdr, 8, buf);
    hdr
}
// v.push(x): grow-if-full then store x at dataptr+len*4, len++. base is a PARAM reused several times (sound -- the
// s9_read_scrut precedent). The grow is a brz/pool region: `if len==cap { __vec_grow(base) }`, then dataptr is RELOADED.
fn s9_vec_push1(base: i32, x: i32) {
    let len = s9_load_at(base, 0);
    let cap = s9_load_at(base, 4);
    let full = s9op("eq", len, cap);
    s9emit_brz(full);
    s9call1("__vec_grow", base);
    s9emit_pool();
    let dp = s9_load_at(base, 8);
    let f1 = s9const(8); let ioff = s9op("mul", len, f1);
    let addr = s9op("add", dp, ioff);
    println!("  store8 v{} v{}", addr, x);
    let one = s9const_fresh(1); let nlen = s9op("add", len, one);
    s9_store_at(base, 0, nlen);
}
fn s9_vec_push(base: i32, args: &Vec<ExprK>, env: &S9Env) -> i32 { for a in args.iter() { let x = s9expr(a, env); s9_vec_push1(base, x); } base }
// v.len(): load(base, 0). (Vec/array/String all keep len@0.)
fn s9_vec_len(base: i32) -> i32 { s9_load_at(base, 0) }
// String (E2, minimal): String::new reuses s9_vec_new (a [len=0@0, byte-cap] block; String is a Vec<u8>). The write
// position = base + 4 + len (the current byte length). base is a PARAM (reuse sound, s9_read_scrut precedent).
fn s9_str_dataend(base: i32) -> i32 { let curlen = s9_load_at(base, 0); let dp = s9_load_at(base, 8); s9op("add", dp, curlen) }
// s.push_str(LITERAL): copy the literal's bytes (storeb) at the write position, len += byte-count. Only STRING-LITERAL
// args this rung (the common case); &str-variable push_str is a follow-up. Bytes counted/iterated (S9-F8: no .len()+index).
// C-S9-STRING-PUSH-AS-VEC: `.push(..)` (method kind 1) routed UNCONDITIONALLY to s9_vec_push, so `String::push(c)`
// was lowered as a Vec ELEMENT push -- a 4-byte slot write at the wrong stride -- instead of a byte append. Combined
// with ExprK::Char having no s9expr arm, `s.push(' ')` under a SEM0-built SEM0 appended a NUL. Both were needed: the
// Char arm makes the value right, this makes the STORE right.
// Dispatch is on the ARGUMENT, not the receiver type: a char literal argument can only be a String push (nothing in
// this codebase holds a Vec<char>), and that needs no type environment, so it cannot mis-fire on an unknown type.
fn s9_str_push_char(base: i32, a: &ExprK, env: &S9Env) {
    let ub = s9const(1); let rg = s9fresh(); println!("  v{}: i32 = call __str_grow v{} v{}", rg, base, ub);
    let daddr = s9_str_dataend(base);
    let v = s9expr(a, env);
    println!("  storeb v{} v{}", daddr, v);
    let curlen = s9_load_at(base, 0); let one = s9const_fresh(1); let nlen = s9op("add", curlen, one); s9_store_at(base, 0, nlen);
}
// A char-VALUED argument, not only a char LITERAL: `out.push(by[j] as char)` is the byte-slicing idiom at a dozen
// sites in this file, and it is a Cast, not an ExprK::Char. Restricting the dispatch to literals left every such site
// pushing NUL -- which is what turned `i32` into `i\0\0` in the S5 typed-HIR dump.
fn s9_push_char_ty(t: &Ty) -> bool { let ts = dump_ty(t); str_contains(&ts, "char") }
fn s9_push_is_char(args: &Vec<ExprK>) -> bool {
    let mut r = false;
    for a in args.iter() {
        match a {
            ExprK::Char(_c) => { r = true; }
            ExprK::Cast(_e, ty) => { if s9_push_char_ty(ty) { r = true; } }
            _ => {}
        }
    }
    r
}
fn s9_push_dispatch(base: i32, args: &Vec<ExprK>, env: &S9Env) -> i32 {
    if s9_push_is_char(args) { for a in args.iter() { s9_str_push_char(base, a, env); } return base; }
    s9_vec_push(base, args, env)
}
fn s9_str_push1(base: i32, a: &ExprK, env: &S9Env) {
    match a {
        ExprK::Str(content) => {
            // decode escapes as we store (C-G2D-STR-ESCAPE): source index si walks content bytes, dest index di counts EMITTED
            // bytes (< si when escapes collapse two source bytes to one). len is bumped by di, not the source length.
            let cb = content.as_bytes();
            // C-G2D-STR-GROW: ensure capacity for up to cb.len() new bytes (escapes only shrink) BEFORE computing dataend
            // (grow may move dataptr). cb.len() is a compile-time upper bound.
            let ub = s9const(cb.len() as i64); let rg = s9fresh(); println!("  v{}: i32 = call __str_grow v{} v{}", rg, base, ub);
            let daddr = s9_str_dataend(base);
            let mut si = 0; let mut di = 0;
            while si < cb.len() {
                let mut byte = cb[si] as i64; let mut adv = 1;
                if cb[si] == 92 && si + 1 < cb.len() { let d = s9_esc_byte(cb[si + 1]); if d >= 0 { byte = d; adv = 2; } }
                let ic = s9const(di as i64); let addr = s9op("add", daddr, ic); let bc = s9const(byte); println!("  storeb v{} v{}", addr, bc);
                di += 1; si += adv;
            }
            let curlen2 = s9_load_at(base, 0); let nc = s9const_fresh(di as i64); let nlen = s9op("add", curlen2, nc); s9_store_at(base, 0, nlen);
        }
        // E2 variable push_str: `s.push_str(other)` where `other` is a String/&str VALUE (a var, &var, or expr, NOT a
        // literal) -> append its bytes via __scat_s(dst, src) (reads src.len@0 + src.bytes@4, byte-copies, bumps dst.len).
        // Reuses the landed runtime helper; src is lowered through s9expr (Ref is identity for the inline String block).
        _ => { let src = s9expr(a, env); let r = s9fresh(); println!("  v{}: i32 = call __scat_s v{} v{}", r, base, src); }
    }
}
fn s9_str_push(base: i32, args: &Vec<ExprK>, env: &S9Env) -> i32 { for a in args.iter() { s9_str_push1(base, a, env); } base }
fn s9lookup(env: &S9Env, name: &str) -> i32 {
    let mut i = 0; let mut hit = -1; for n in env.names.iter() { if streq(n, name) { hit = i; } i += 1; }
    if hit < 0 { return 0; }
    let mut j = 0; let mut out = 0; for s in env.slots.iter() { if j == hit { out = *s; } j += 1; } out
}
// C-S9-CMP-32: cf implements only `lt8` among the 64-bit relational forms, so every comparison stayed 32-bit and
// silently compared the low half of its operands. That is precisely what kept s9const's OWN `k > 2147483647` guard
// from firing inside gen2 -- the 64-bit constant path was never taken, so gen2 re-emitted wide constants truncated,
// and its self-emit diverged from gen1 exactly at that sequence. Build all six relations out of lt8; the boolean
// combinators then run on 0/1 results, where 32-bit and/or/eq are exact.
fn s9_not01(v: i32) -> i32 { let z = s9const(0); s9op("eq", v, z) }
fn s9_cmp(op: &str, va: i32, vb: i32) -> i32 {
    if streq(op, "Lt") { return s9op("lt8", va, vb); }
    if streq(op, "Gt") { return s9op("lt8", vb, va); }
    if streq(op, "Ge") { let t = s9op("lt8", va, vb); return s9_not01(t); }
    if streq(op, "Le") { let t = s9op("lt8", vb, va); return s9_not01(t); }
    let ab = s9op("lt8", va, vb); let ba = s9op("lt8", vb, va);
    if streq(op, "Ne") { return s9op("or", ab, ba); }
    let nab = s9_not01(ab); let nba = s9_not01(ba); s9op("and", nab, nba)
}
// Dispatch lives HERE, not at the s9expr call site: s9expr is the highest-pressure fn in the emitter and a
// value-selection inline there is the C-SCI-REFMATCH-L6-WALL shape.
// CL-7: is `e` an f64 in the softfloat (scaled-i64) representation? Same shape as s9_is_str_expr. Called ONLY
// after the operator has already been narrowed to Mul/Div (see the Bin arm), so it stays off the per-operand
// allocation path that the bool-Display note measured as doubling the arena.
fn s9_is_flt_expr(env: &S9Env, e: &ExprK) -> bool {
    match e { ExprK::LitFloat(_t) => true, _ => { let ty = s9_lookup_type(env, &s9_bare_name(e)); let fty = s9_field_type(env, &s9_field_name(e)); let ity = s9_init_ty(env, e); str_contains(&ty, "f64") || str_contains(&ty, "f32") || str_contains(&fty, "f64") || str_contains(&fty, "f32") || str_contains(&ity, "f64") || str_contains(&ity, "f32") } }
}
// CL-7: fixed point makes `+`/`-` free (both operands carry the same scale, and it survives the sum) and makes
// `*`/`/` NOT free. `a*b` carries scale SQUARED, so it needs one `/1e9`; `a/b` CANCELS the scale entirely, so
// the numerator needs one `*1e9` first. Everything is the 8-suffixed 64-bit form: `a*b` for two scaled operands
// is already ~1e18 before the division.
// BOTH ARE SPLIT to avoid a 1e18 intermediate. The naive forms (`a*b/1e9` and `a*1e9/b`) overflow i64 for
// ordinary values -- measured: `float_multiply_fixedpoint` gave -5.880384073 and `float_divide_no_overflow` gave
// -2.111686018 under them. The r2-corpus probe `float_divide_no_overflow` states the divide decomposition in its
// own header comment; the multiply takes the same shape on the other operand:
//     a*b/S  =  (a/S)*b + ((a%S)*b)/S      -- split the SCALE out of the left operand
//     a*S/b  =  (a/b)*S + ((a%b)*S)/b      -- split the DIVISOR out of the left operand
// Each product now has one factor already reduced, so the intermediate is bounded by a magnitude that fits.
// Truncation is toward zero on both `div8` and `mod8`, matching Rust's `/` and `%`, so the two terms of each sum
// carry the same sign and the identity holds for negatives.
fn s9_flt_mul(va: i32, vb: i32) -> i32 {
    let sc = s9const_fresh(1000000000);
    let q = s9op("div8", va, sc);
    let hi = s9op("mul8", q, vb);
    let r = s9op("mod8", va, sc);
    let rp = s9op("mul8", r, vb);
    let lo = s9op("div8", rp, sc);
    s9op("add8", hi, lo)
}
fn s9_flt_div(va: i32, vb: i32) -> i32 {
    let sc = s9const_fresh(1000000000);
    let q = s9op("div8", va, vb);
    let hi = s9op("mul8", q, sc);
    let r = s9op("mod8", va, vb);
    let rp = s9op("mul8", r, sc);
    let lo = s9op("div8", rp, vb);
    s9op("add8", hi, lo)
}
fn s9_binop_emit(op: &str, va: i32, vb: i32) -> i32 {
    if is_cmp_op(op) { return s9_cmp(op, va, vb); }
    let rop = s9_binop(op); s9op(&rop, va, vb)
}
fn s9_binop(op: &str) -> String {
    // 64-bit arithmetic: every emit_s9 slot is 8 bytes and cf stores values sign-extended, so the *8 forms are
    // correct for i32-range operands AND for the packed i64 literals that 32-bit ops silently truncated.
    if streq(op, "Add") { return String::from("add8"); } if streq(op, "Sub") { return String::from("sub8"); }
    if streq(op, "Mul") { return String::from("mul8"); } if streq(op, "Div") { return String::from("div8"); }
    if streq(op, "Rem") { return String::from("mod8"); }
    if streq(op, "Eq") { return String::from("eq"); } if streq(op, "Ne") { return String::from("ne"); }
    if streq(op, "Lt") { return String::from("lt"); } if streq(op, "Le") { return String::from("le"); }
    if streq(op, "Gt") { return String::from("gt"); } if streq(op, "Ge") { return String::from("ge"); }
    if streq(op, "BitAnd") { return String::from("and8"); } if streq(op, "BitOr") { return String::from("or8"); }
    if streq(op, "BitXor") { return String::from("xor8"); } if streq(op, "Shl") { return String::from("shl8"); }
    if streq(op, "Shr") { return String::from("shu8"); }
    // COMPARISONS stay 32-bit: cf implements only lt8 among the relational *8 forms.
    String::from("add")
}
// SHORT-CIRCUIT `&&` / `||` (C-G2D-SHORTCIRCUIT-COND). The eager `s9op(add, va, vb)` lowering evaluated BOTH operands and
// summed them -- wrong value AND, worse, it LOADS the RHS unconditionally, so the pervasive `i < len && b[i] ...` bounds-guard
// idiom read out of bounds -> SEGV (the gen2 lex crash). Model each as a value-if (mirrors s9_if's brz/then_tail/else_tail
// merge): `a && b` == `if a { b } else { 0 }`, `a || b` == `if a { 1 } else { b }` -- the RHS is emitted in exactly ONE arm,
// so it runs only when the LHS does not decide the result. rv is a fresh vreg; the merge label __Li{rv} is unique per node.
fn s9_and(a: &ExprK, b: &ExprK, env: &S9Env) -> i32 {
    let rv = s9fresh();
    let lbl = s9_lbl();
    let va = s9expr(a, env);
    s9emit_brz(va);
    let vb = s9expr(b, env);            // then-arm: only reached when va != 0 (short-circuit)
    s9emit_then_tail(rv, lbl, vb);
    let z = s9const(0);                 // else-arm: a false -> 0
    s9emit_else_tail(rv, lbl, z);
    rv
}
fn s9_or(a: &ExprK, b: &ExprK, env: &S9Env) -> i32 {
    let rv = s9fresh();
    let lbl = s9_lbl();
    let va = s9expr(a, env);
    s9emit_brz(va);
    let o = s9const(1);                 // then-arm: a true -> 1
    s9emit_then_tail(rv, lbl, o);
    let vb = s9expr(b, env);            // else-arm: only reached when va == 0 (short-circuit)
    s9emit_else_tail(rv, lbl, vb);
    rv
}
// lower a scalar expr to an rzir slot (A0 subset: int literal / paren / integer binop / bare var).
// [emit-s9-vecstring-elem locale R1] A String VALUE: build a fresh inline String [len@0, bytes@4..] from a literal's
// bytes (reuses s9_str_push1's Str path). Before this, ExprK::Str and format! fell to `_ => s9const(0)`, so a
// `Vec<String>` element pushed from a literal/format! was NULL (0) -> `w[i].len()` = load(0) -> segfault (the emit_s9
// side of P-CF-ACC's Vec<String> facet). s1.rz self-emitting s1_parser.rs (saturated with String literals) hit this.
fn s9_str_lit(content: &str, env: &S9Env) -> i32 {
    let base = s9_str_new();
    s9_str_push1(base, &ExprK::Str(format!("{}", content)), env);
    base
}
// decode a backslash-escape's following byte to its literal byte value, or -1 if `nx` is not a handled escape. The lexer keeps
// string content RAW (a backslash + its escape char are two source bytes in ExprK::Str content), so emit_s9 must decode at
// lowering time exactly as L6 does at compile time -- else every `\n`/`\t`/`\"`/`\\` in a string constant emits literally (gen2
// printed `Crate\n  Item` with a literal backslash-n). Set matches s1_parser's usage (\" \\ \n \t) plus \r \' \0. (C-G2D-STR-ESCAPE.)
fn s9_esc_byte(nx: u8) -> i64 { if nx == 110 { return 10; } if nx == 116 { return 9; } if nx == 114 { return 13; } if nx == 92 { return 92; } if nx == 34 { return 34; } if nx == 39 { return 39; } if nx == 48 { return 0; } -1 }
// [locale R2] append one literal byte to String `base` at its runtime dataend (base+4+len), then len++.
fn s9_str_append_byte(base: i32, bval: i64) {
    let one0 = s9const(1); let rg = s9fresh(); println!("  v{}: i32 = call __str_grow v{} v{}", rg, base, one0);   // C-G2D-STR-GROW
    let dlen = s9_load_at(base, 0);
    let dp = s9_load_at(base, 8); let daddr = s9op("add", dp, dlen);
    let bc = s9const(bval); println!("  storeb v{} v{}", daddr, bc);
    let one = s9const_fresh(1); let nlen = s9op("add", dlen, one); s9_store_at(base, 0, nlen);
}
// [locale R1+R2] format!: build a fresh inline String. Walk the format string; literal bytes are appended; each `{}`
// placeholder appends the k-th top-level-comma-separated arg -- a String/&str arg via __scat_s (byte-copy), else an int
// arg via __scat_i (decimal). Mirrors s9_println's format-walk + arg-type detection, but BUILDS a String instead of
// printing. N placeholders <-> N args.
fn s9_str_bool(base: i32, v: i32) {
    s9emit_brz(v);
    s9_str_append_byte(base, 116); s9_str_append_byte(base, 114); s9_str_append_byte(base, 117); s9_str_append_byte(base, 101);
    s9emit_pool();
    let z = s9const(0); let nv = s9op("eq", v, z);
    s9emit_brz(nv);
    s9_str_append_byte(base, 102); s9_str_append_byte(base, 97); s9_str_append_byte(base, 108); s9_str_append_byte(base, 115); s9_str_append_byte(base, 101);
    s9emit_pool();
}
fn s9_format(toks: &Vec<String>, env: &S9Env) -> i32 {
    let base = s9_str_new();
    if toks.len() == 0 { return base; }
    let fe = s9_reparse_expr(&toks[0]);
    let fmt = match fe { ExprK::Str(content) => content, _ => String::from("") };
    let fb = fmt.as_bytes();
    let mut j = 0;
    let mut k = 0;
    while j < fb.len() {
        if fb[j] == 123 && j + 1 < fb.len() && fb[j + 1] == 123 {
            s9_str_append_byte(base, 123); j += 2;   // `{{` -> literal `{`
        } else if fb[j] == 125 && j + 1 < fb.len() && fb[j + 1] == 125 {
            s9_str_append_byte(base, 125); j += 2;   // `}}` -> literal `}`
        } else if fb[j] == 123 && j + 1 < fb.len() && fb[j + 1] == 125 {
            let asrc = s9_nth_arg_src(toks, k);
            let e = s9_reparse_expr(&asrc);
            // C-S9-ITER-SCALAR-NO-AUTODEREF: a bare scalar `.iter()` binding printed its ADDRESS here too.
            // ty is computed FIRST and reused -- a second lookup inside a helper costs an arena String per
            // placeholder, which is what tripped the arena assertion.
            let ty = s9_lookup_type(env, &s9_bare_name(&e));
            let v = if streq(&ty, "iterscalar") { let t0 = s9expr(&e, env); s9_slot_load(t0, 0) } else { s9expr(&e, env) };
            let fty = s9_field_type(env, &s9_field_name(&e));
            let ity = s9_init_ty(env, &e);
            // C-S9-BOOL-DISPLAY, second site: format! has its own placeholder path, so fixing only println!
            // left `format!("{}", true)` yielding "1". Appends the literal bytes into the String being built.
            if s9_is_bool_val(&e, &ty, &fty) { s9_str_bool(base, v); } else if str_contains(&ty, "String") || str_contains(&ty, "str") || str_contains(&fty, "String") || str_contains(&fty, "str") || str_contains(&ity, "String") || str_contains(&ity, "str") || str_contains(&s9_elem_ty(env, &e), "String") || str_contains(&s9_elem_ty(env, &e), "str") {
                let r = s9fresh(); println!("  v{}: i32 = call __scat_s v{} v{}", r, base, v);
            } else {
                let r = s9fresh(); println!("  v{}: i32 = call __scat_i v{} v{}", r, base, v);
            }
            k += 1; j += 2;
        } else if fb[j] == 92 && j + 1 < fb.len() && s9_esc_byte(fb[j + 1]) >= 0 {
            s9_str_append_byte(base, s9_esc_byte(fb[j + 1])); j += 2;   // C-G2D-STR-ESCAPE: decode \n/\t/\"/\\ in a format literal
        } else {
            s9_str_append_byte(base, fb[j] as i64);
            j += 1;
        }
    }
    base
}
// The leading variant word of dump_expr's rendering ("Expr Field(..)" -> "Field"), for the unlowered-construct
// diagnostic. Byte-scan to the first '(', mirroring the lexer's proven idiom.
fn s9_kindhead(e: &ExprK) -> String {
    let d = dump_expr(e);
    let b = d.as_bytes();
    let mut i = 0;
    if b.len() >= 5 { if b[0] == 69 && b[1] == 120 && b[2] == 112 && b[3] == 114 && b[4] == 32 { i = 5; } }
    let st = i;
    let mut out = String::new();
    while i < b.len() { if b[i] == 40 { break; } i = i + 1; }
    let mut j = st;
    while j < i { out.push(b[j] as char); j = j + 1; }
    out
}
fn s9expr(e: &ExprK, env: &S9Env) -> i32 {
    match e {
        // C-S1-LITERAL-PACK-AMBIGUOUS, closed 2026-08-26. The lexer packs suffix+value as `value + code*lit_mul()`,
        // so lowering the raw carrier made every SUFFIXED literal wrong at runtime: `vec![1i64,2,3,4,5]` popped
        // 5,4,3,2 correctly and then 4000000000000001 for the one suffixed element. An earlier attempt to unpack
        // here was REFUTED by self-compilation (35 ok -> 11 ok) -- correctly, at the time: this file's own
        // suffix_offset_of table was twelve literals in the 1e15..12e15 band, so unpacking zeroed the packing
        // constants themselves. lit_mul() is now COMPUTED from two small factors and the table is `code * m`, so
        // NO literal >= 1e15 remains in this file and the unpack is safe. Residual: an unsuffixed literal >= 1e15
        // in a COMPILED PROGRAM still aliases into the suffix band. That is a representational limit of the
        // packing, and it is now the only one.
        ExprK::LitInt(n) => s9const_fresh(lit_value(*n)),
        // CL-6: was NO arm, so a float literal fell to `_ => s9const(0)` -- the file's own "a missing arm is a
        // silent 0" shape for the FOURTH time. Lowered as the fixed-point scaled i64 (see s9_flt_scaled).
        ExprK::LitFloat(t) => s9const_fresh(s9_flt_scaled(t)),
        ExprK::Bool(b) => { if *b { s9const_fresh(1) } else { s9const_fresh(0) } },
        // C-S9-CHAR-NO-ARM: `ExprK::Char` had NO arm here, so every char literal fell to `_ => s9const(0)` and
        // lowered to NUL. Exactly the history the Bool arm above records for itself, and C-G2D-UNARY-NOT records for
        // `!`: on this emitter a MISSING arm is not a compile error, it is a silent 0. Measured: `s.push(' ')` under a
        // SEM0-built SEM0 appended a NUL, so the macro engine's space-joined keys came out "a\0+\0b" -- which broke
        // stringify! directly and broke matcher matching indirectly, since metavar_of tests for byte 32 at mb[1].
        ExprK::Char(cv) => s9const_fresh(s9_char_val(cv)),   // A9 bool literal: true=1, false=0 (i32 bool model). Was `_ => s9const(0)` -> `true` mis-lowered to 0 (streq/str_contains always false).
        // unary `!` / `-`. Had NO arm -> fell to `_ => s9const(0)`, so `!drop`/`!emptysep`/`!(cond)` all lowered to 0 -> e.g.
        // parse_krate's `if !drop { items.push(it) }` never pushed -> the parsed crate had ZERO items (C-G2D-UNARY-NOT, the gen2
        // empty-dump). `!` is LOGICAL not on the i32 bool model (`eq x 0`: 0->1, nonzero->0; correct for the bool operands
        // s1_parser uses `!` on). `-` is `0 - x`. (Bitwise `!int` is not used in s1_parser; would need a distinct rzir op.)
        ExprK::Unary(op, a) => { let va = s9expr(a, env); if streq(op, "Not") { let z = s9const(0); s9op("eq", va, z) } else if streq(op, "Neg") { let z = s9const(0); s9op("sub8", z, va) } else { va } },
        ExprK::Str(content) => s9_str_lit(content, env),
        ExprK::MacCall(p, _delim, toks) => { let nm = dump_path(p); if streq(&nm, "panic") { s9_panic_macro() } else if streq(&nm, "assert") { s9_assert_macro(toks, env) } else if streq(&nm, "vec") { s9_vec_macro(toks, env) } else if streq(&nm, "format") { s9_format(toks, env) } else if streq(&nm, "println") || streq(&nm, "print") { s9_println(toks, env); s9const(0) } else { s9const(0) } },   // println!/print! in EXPRESSION position (a match arm / if-value / block tail) -- s9_stmt handles the statement form, but a match arm is lowered via s9expr; without this a `p => println!(..)` arm swallowed the print (C-G2D-PRINTLN-EXPR)
        ExprK::Repeat(elem, count) => s9_repeat(elem, count, env),
        ExprK::Range(lo, hi, incl) => s9_range_new(lo, hi, *incl, env),
        ExprK::Try(inner) => s9_try(inner, env),
        ExprK::Paren(a) => s9expr(a, env),
        ExprK::Bin(op, a, b) => { if streq(op, "And") { s9_and(a, b, env) } else if streq(op, "Or") { s9_or(a, b, env) } else if (streq(op, "Eq") || streq(op, "Ne")) && (s9_is_str_expr(env, a) || s9_is_str_expr(env, b)) {
            // string `==`/`!=` compares CONTENTS via __str_eq (a [len@0,bytes@4] compare), not the operand ADDRESSES that plain
            // `eq`/`ne` would compare (C-G2D-STR-EQ). Was a pointer compare -> `word == "struct"` always false -> lex keyword
            // recognition never fired -> everything lexed as an Ident, and every string `==` in the parser was wrong.
            let va = s9expr(a, env); let vb = s9expr(b, env); let r = s9call2("__str_eq", va, vb); if streq(op, "Ne") { let z = s9const(0); s9op("eq", r, z) } else { r }
        } else if (streq(op, "Mul") || streq(op, "Div")) && (s9_is_flt_expr(env, a) || s9_is_flt_expr(env, b)) {
            // CL-7: the operator is narrowed FIRST, so the float lookups run only for `*` and `/`.
            let va = s9_arith_val(a, env); let vb = s9_arith_val(b, env);
            if streq(op, "Mul") { s9_flt_mul(va, vb) } else { s9_flt_div(va, vb) }
        } else { let va = s9_arith_val(a, env); let vb = s9_arith_val(b, env); s9_binop_emit(op, va, vb) } }   // C-S9-ITER-SCALAR-NO-AUTODEREF: both operands, not just the assign-op RHS -- fixing one side only left `s = s + x` and `x + 1` still adding the ADDRESS
        ExprK::Loop(_lbl, lb) => s9_loop(lb, env),
        // S9-F12: `match &**wc` is an inline box double-deref, which this tier mis-compiles -- go through s9f12_deref,
        // exactly as the `if let` dispatch does. A `while let` arrives as ExprK::While whose condition is ExprK::Let.
        ExprK::While(_lbl, wc, wb) => { match s9f12_deref(wc) { ExprK::Let(wlp, wls) => s9_while_let(wlp, wls, wb, env), _ => { if s9_chain_has_let(wc) { s9_while_chain(wc, wb, env) } else { s9_while(wc, wb, env) } } } },
        ExprK::Assign(l, r) => { s9_assign(l, r, env); s9const(0) }   // C-G2D-ASSIGN-TAIL: a plain `x = v` as a block TAIL (no semi) is an ExprK::Assign; s9expr had only AssignOp, so it fell to `_ => 0` and the store was DROPPED (the parse_postfix arm `e = ExprK::Call(..)` was lost -> calls/fields/indexing dropped). s9_stmt already handled it.
        ExprK::Closure(_mv, _ps, _rt, _body) => { let h = s9_clos_hash(e); if s9clh_has(h) { s9_closure_value(h) } else { s9const(0) } },
        ExprK::AssignOp(op, l, r) => { s9_assignop(op, l, r, env); s9const(0) }
        ExprK::ForLoop(_lbl, fp, fi, fb) => s9_for(fp, fi, fb, env),
        ExprK::Match(msc, marms) => s9_match(msc, marms, env),
        ExprK::Ret(rv) => { let mut r = -1; for e0 in rv.iter() { r = s9expr(e0, env); } if r < 0 { r = s9const(0); } s9emit_ret(r); s9const(0) },
        // break/continue in EXPRESSION position (a match arm body, an if-value arm): handled in s9_stmt for statements, but a
        // match arm is lowered via s9expr, so without these `match x { p => break }` fell to the `_ => s9const(0)` default and
        // emitted NO `brk` -> `loop { match .. break }` never exits (C-G2D-BREAK-IN-MATCH, the gen2 parse_krate HANG).
        ExprK::Break(_lbl, bv) => { let mut r = -1; for e0 in bv.iter() { r = s9expr(e0, env); } if r >= 0 { let s = s9bv_get(); s9emit_break_val(s, r); } s9emit_brk(); s9const(0) },
        ExprK::Continue(_lbl) => { s9emit_cnt(); s9const(0) },
        ExprK::QPath(qty, _tr, proj) => { let qn = s9_qpath_name(qty, proj);
            if qn.len() == 0 { eprintln!("emit_s9: qself path with an uninferable head (`<_>::` / `<dyn ..>::`) is unlowered (CT-9)"); std::process::exit(1); }
            s9_call(&qn, &Vec::new(), env) }
        ExprK::Call(callee, args) => { let cn = s9_callee_name(callee); s9_qpath_guard(callee, &cn); let ls = s9_last_seg_str(&cn); if s9_is_closure_local(env, &cn) { s9_calli(s9lookup(env, &cn), args, env) } else if streq(&cn, "std::process::exit") || streq(&cn, "process::exit") { s9_exit(args, env) } else if streq(&cn, "Box::new") { s9_box_new(args, env) } else if streq(&cn, "Vec::new") { s9_vec_new() } else if streq(&cn, "String::new") { s9_str_new() } else if streq(&cn, "String::from") { s9_str_ident(args, env) } else if (s9_qualified(&cn) && s9_reg_known(env, &ls)) || streq(&ls, "Some") || streq(&ls, "Ok") || streq(&ls, "Err") { s9_enum_new(env, &cn, args) } else { s9_call(&cn, args, env) } },
        ExprK::Tuple(elems) => s9_tuple(elems, env),
        ExprK::Array(elems) => s9_array(elems, env),
        ExprK::Index(arr, idx) => { let an = s9_bare_name(arr); if s9_static_arr(env, &an) { let iv = s9expr(idx, env); let ad = s9_static_elem_addr(&an, iv); s9_slot_load(ad, 0) } else { s9_index(arr, idx, env) } },   // C-S9-STATIC-ARRAY-UNLOWERED: a static array is a FLAT block
        ExprK::Method(mrecv, mname, _mtys, margs) => {
            // C-S9-ITER-SUM: recognise `<v>.iter().sum()` before the generic method dispatch.
            if streq(mname, "sum") { if margs.len() == 0 { if s9_is_iter_call(mrecv) { return s9_iter_sum_of(mrecv, env); } } }
            s9_method(mrecv, mname, margs, env)
        },
        // C-S9-REF-LOCAL-DEREF: `*` was UNCONDITIONALLY load8(v,0), which is the correct lowering for exactly one
        // representation -- a Box cell. emit_s9 has no references (`&x` lowers to `x` via s9_ref_of's identity), so
        // deref of anything that came from a `&` must be the INVERSE of that identity, i.e. identity. Measured before
        // this change: Box deref MATCHED; `*r` on a &scalar/&struct, as local/param/condition, ALL SIGSEGV (8 probes,
        // one clean boundary, no conjunction).
        // The fallback stays `load`, NOT identity: s9_init_ty has no `Box::new` arm, so an unannotated `let b =
        // Box::new(41)` records type text "" and an identity-by-default would break the one case that worked.
        // Converting to identity therefore requires POSITIVE evidence of reference-ness -- s9_scrut_by_ref, which is
        // the predicate already used for match scrutinees (`ExprK::Ref` directly, or a Path whose recorded type is
        // `Ty Ref`). Unknown types keep the old behaviour, so this cannot regress a case that passes today.
        ExprK::Deref(dinner) => { let db = s9expr(dinner, env); if s9_deref_scalar(dinner, env) { s9_slot_load(db, 0) } else { if s9_scrut_by_ref(dinner, env) { db } else { s9_slot_load(db, 0) } } },
        ExprK::Ref(_m, rinner) => s9_ref_of(rinner, env),        // SR: aggregates are address-valued; &x = x's address; a Box<T> field derefs (C-G2D-BOX-FIELD-REF).
        ExprK::RawRef(_m, rinner) => s9expr(rinner, env),
        ExprK::Struct(path, sfields, rest) => { let sn = s9_last_seg_str(&dump_path(path)); match rest { Some(bx) => s9_struct_upd(&sn, sfields, bx, env), None => s9_struct(&sn, sfields, env) } },   // SR-2: pass the struct name so fields place at qualified offsets
        ExprK::Field(fe, fname) => { if is_all_digits(fname) { let b = s9expr(fe, env); s9_slot_load(b, (str_to_i64(fname) as i32) * 8) } else { let off = s9_field_off_r(env, fe, fname); let b = s9expr(fe, env); s9_slot_load(b, off) } },   // SR-2: offset by receiver struct (C-G2D-FIELD-COLLISION)
        ExprK::Path(p) => { let nm = dump_path(p); let ls = s9_last_seg_str(&nm); if (s9_qualified(&nm) && s9_reg_known(env, &ls)) || streq(&ls, "None") { s9_enum_unit(env, &nm) } else if s9_is_static(env, &nm) { s9_global_load(&nm) } else if s9_is_constitem(env, &nm) { s9_constitem_val(env, &nm) } else { s9lookup(env, &nm) } }   // C-G2D-STATIC-MUT: a bare Path naming a top-level `static [mut]` reads the GLOBAL (`gref NAME; load`), not a local slot -> gen2's own S9NC/S9LBL/S9BV counters now persist
        ExprK::Cast(a, _ty) => s9expr(a, env),   // A3/A4: int-to-int cast is identity at rzir i32 (width truncation is A7)
        ExprK::Unsafe(b) => s9_bv(b, env),       // C-G2D-STATIC-MUT: `unsafe { .. }` is a transparent block (tail = its value); was `_ => 0` so `unsafe { let r=N; N=N+1; r }` collapsed to 0
        ExprK::Block(b) => s9_bv(b, env),        // B7-lite: a block expr's value is its tail expr's slot
        // P-CONSTBLOCK / CT-10: rustc const-EVALUATES `const { .. }`; this tier has no const-eval. All three
        // sites in the rustc tree are compile-time assertion blocks whose value is `()`, so evaluating the
        // block at RUNTIME reaches the same observable for any program that compiles. Lowered as the block.
        // A const block that a program relies on being folded at compile time is NOT handled by this and would
        // simply run -- recorded, not built.
        ExprK::ConstBlock(b) => s9_bv(b, env),
        ExprK::TryBlock(b) => { let bv = s9_bv(b, env); s9_ok_wrap(bv) },
        ExprK::If(cond, then, els) => {
            // C-S9-IF-LET-UNLOWERED: an `if let` condition is a pattern test, not a boolean.
            // S9-F12: `match &**cond` is an inline box double-deref, which this tier mis-compiles -- it is
            // exactly what rung 6 removed from twelve sites. Go through the call boundary instead.
            match s9f12_deref(cond) {
                ExprK::Let(lp, lscrut) => { if els.len() == 0 { s9_if_let_stmt(lp, lscrut, then, env) } else { s9_if_let_val(lp, lscrut, then, &els[0], env) } }
                ExprK::Bin(bop, ba, bb) => { s9_if_bin(bop, ba, bb, cond, then, els, env) }
                _ => { if els.len() == 0 { let cv = s9expr(cond, env); s9emit_brz(cv); s9_walk(then, env); s9emit_pool(); s9const(0) } else { s9_if(cond, then, &els[0], env) } }
            }
        },
        // C-S9-UNLOWERED-SILENT-ZERO: this catch-all is why an unsupported construct yields 0 rather than
        // failing. The file records the SAME discovery four separate times, each found by accident and fixed one
        // arm at a time: Bool ("was `_ => s9const(0)` -> `true` mis-lowered to 0"), unary Not (C-G2D-UNARY-NOT),
        // Unsafe ("was `_ => 0` so `unsafe {..}` collapsed to 0"), and Char (rung 10).
        // Measured on the 904-probe corpus: 118 probes print exactly 0 where rustc prints something else, and they
        // are NOT one mechanism -- slice patterns, operator overloading, HashMap, iterator adapters, while-let,
        // map_err, supertrait defaults. `0` is the shared SYMPTOM of "unlowered", not a shared cause.
        // This diagnostic makes the class ENUMERABLE instead of rediscoverable. Inert to codegen, and inert in an
        // L6/SEM0-built s1_parser (eprintln! is dropped by that lowering -- measured); a rustc-built s1_parser
        // reports, which is enough to inventory what is missing.
        _ => { eprintln!("s9expr: UNLOWERED {}", s9_kindhead(e)); s9const(0) }
    }
}
fn s9_patname(p: &Pat) -> String {
    match p { Pat::Ident(s) => format!("{}", s), Pat::Bind(_mode, name) => format!("{}", name), _ => String::new() }
}
// `println!("{}", ARG)` -- A0: format is a single `{}`; ARG is a raw token string (MacCall args[1]) -- lower it (numeric
// -> const, else a bound var) then __pi + newline + flush. (Multi-placeholder / literal-text formats are later E1 work.)
// E3 slice-1: re-parse a raw-token source fragment into an ExprK (so println!/format! ARG tokens lower as a general
// expression, not a single token). Reuses the parser (lex + parse_expr) exactly as macro-expansion does (line ~5929).
// C-G2D-VECMAC-UNLOWERED (recorded in-source since before 2026-08-25; rediscovered as C-S9-VEC-LITERAL rung 2):
// emit_s9 knew only format!/print!/println!, so `vec![..]` fell through unlowered and its first USE segfaulted.
// The in-source note said lowering it "is macro EXPANSION, S2's job, not the lowerer's" -- that reasoning is
// OVERTAKEN: println! already reparses raw token strings via s9_reparse_expr, so the lowerer has had the
// machinery all along. Desugars to the proven Vec::new()+push path (which is exactly what s1_parser.rs was
// hand-written to use instead). Only the LIST form `vec![a, b, c]`; the REPEAT form `vec![x; n]` is still
// unlowered and is recorded as such.
// Segment extraction is O(N^2) by design: it rebuilds only the k-th segment each pass, so there is no
// String REASSIGNMENT and no Vec<String> -- both named L6 landmines in s9_nth_arg_src's own note.
fn s9_vec_seg_src(toks: &Vec<String>, k: i32, sep: &str) -> String {
    let mut src = String::new();
    let mut seg = 0;
    let mut depth = 0;
    for t in toks.iter() {
        if streq(t, "(") || streq(t, "[") || streq(t, "{") { depth += 1; if seg == k { src.push_str(t); src.push(' '); } }
        else if streq(t, ")") || streq(t, "]") || streq(t, "}") { depth -= 1; if seg == k { src.push_str(t); src.push(' '); } }
        else if streq(t, sep) && depth == 0 { seg += 1; }
        else { if seg == k { src.push_str(t); src.push(' '); } }
    }
    src
}
// C-SEM0-TERMINATION: `panic!` and `assert!` were silent no-ops, so a SEM0-compiled program CONTINUED past a failed
// assertion with wrong data and exited 0. Measured against rustc: panic! -> 101 vs 0; assert!(1==2) -> 101 vs 0 AND
// the following statement still ran. Same mechanism as s9_exit (flush, then the rzir `syscall` OP -- `call __syscall`
// is an rzrs intrinsic and is NOT callable from emitted rzir), with rustc's panic status 101.
// NOT included, deliberately: `.unwrap()`. All six unwrap sites in this file are on `std::fs::read_to_string`/stdin
// Results, where the emitter treats unwrap as IDENTITY on the string header by design (see s9_method_kind arm 7).
// A tag-zero abort there would fire on an EMPTY FILE. unwrap needs the Result representation settled first.
fn s9_abort_emit() {
    let fv = s9fresh(); println!("  v{}: i32 = call __flush", fv);
    let nv = s9fresh(); println!("  v{}: i32 = const 60", nv);
    let cv = s9fresh(); println!("  v{}: i32 = const 101", cv);
    let zv = s9fresh(); println!("  v{}: i32 = const 0", zv);
    let zw = s9fresh(); println!("  v{}: i32 = const 0", zw);
    let rv = s9fresh(); println!("  v{}: i32 = syscall v{} v{} v{} v{}", rv, nv, cv, zv, zw);
}
fn s9_panic_macro() -> i32 { s9_abort_emit(); s9const(0) }
// `assert!(COND, ..)`: abort when COND is FALSE. Reuses the top-level-comma splitter and the reparse path that
// lower vec!/println!, and the brz/pool block idiom the if-emitter uses -- brz runs the block when its operand is
// NONZERO, so the operand is `COND == 0`.
fn s9_assert_macro(toks: &Vec<String>, env: &S9Env) -> i32 {
    let src = s9_vec_seg_src(toks, 0, ",");
    if src.len() == 0 { return s9const(0); }
    let e = s9_reparse_expr(&src);
    let cv = s9expr(&e, env);
    let z = s9const(0);
    let nz = s9op("eq", cv, z);
    s9emit_brz(nz);
    s9_abort_emit();
    s9emit_pool();
    s9const(0)
}
fn s9_vec_nsegs(toks: &Vec<String>) -> i32 {
    let mut n = 0;
    let mut depth = 0;
    let mut any = 0;
    for t in toks.iter() {
        if streq(t, "(") || streq(t, "[") || streq(t, "{") { depth += 1; any = 1; }
        else if streq(t, ")") || streq(t, "]") || streq(t, "}") { depth -= 1; any = 1; }
        else if streq(t, ",") && depth == 0 { n += 1; }
        else { any = 1; }
    }
    if any == 0 { return 0; }
    n + 1
}
fn s9_vec_is_repeat(toks: &Vec<String>) -> bool {
    let mut depth = 0;
    for t in toks.iter() {
        if streq(t, "(") || streq(t, "[") || streq(t, "{") { depth += 1; }
        else if streq(t, ")") || streq(t, "]") || streq(t, "}") { depth -= 1; }
        else if streq(t, ";") && depth == 0 { return true; }
    }
    false
}
// `vec![x; n]` -- UNROLLED when n is an integer literal (the common `vec![0; 16]`). s9expr runs ONCE and the
// resulting temp is pushed n times, which is what vec![x; n] means for the Copy/Clone types this tier has.
// A NON-literal count cannot be unrolled without emitting a loop, which this desugar deliberately does not do;
// that case REFUSES (exit 1) rather than yielding a silently-empty Vec. Recorded as C-S9-VECMAC-REPEAT-DYN.
// The vec! mouth of the same runtime path. Both mouths reach one lowering, which is the point of CT-11: they
// were listed as two board entries (P-VECMAC-DYN and the P-REPEAT residue) and are one defect.
fn s9_vec_repeat_rt(esrc: &str, csrc: &str, env: &S9Env) -> i32 {
    let ce = s9_reparse_expr(csrc);
    let cntv = s9expr(&ce, env);
    let e = s9_reparse_expr(esrc);
    let v = s9expr(&e, env);
    s9_repeat_fill_dyn(v, cntv)
}
fn s9_vec_repeat(toks: &Vec<String>, env: &S9Env) -> i32 {
    let base = s9_vec_new();
    let esrc = s9_vec_seg_src(toks, 0, ";");
    let csrc = s9_vec_seg_src(toks, 1, ";");
    let ce = s9_reparse_expr(&csrc);
    let nstat = s9_repeat_count(&ce, env);
    return s9_vec_repeat_pick(&esrc, &csrc, env, nstat, base);
}
fn s9_vec_repeat_pick(esrc: &str, csrc: &str, env: &S9Env, nstat: i64, base: i32) -> i32 {
    if nstat < 0 { return s9_vec_repeat_rt(esrc, csrc, env); }
    let n = nstat;
    let e = s9_reparse_expr(esrc);
    let v = s9expr(&e, env);
    let mut k = 0;
    while k < n { s9_vec_push1(base, v); k += 1; }
    base
}
fn s9_vec_macro(toks: &Vec<String>, env: &S9Env) -> i32 {
    if s9_vec_is_repeat(toks) { return s9_vec_repeat(toks, env); }
    let base = s9_vec_new();
    let n = s9_vec_nsegs(toks);
    let mut k = 0;
    while k < n {
        let src = s9_vec_seg_src(toks, k, ",");
        if src.len() > 0 { let e = s9_reparse_expr(&src); let v = s9expr(&e, env); s9_vec_push1(base, v); }
        k += 1;
    }
    base
}
fn s9_reparse_expr(src: &str) -> ExprK {
    let (toks, offs, idn) = lex(src);
    let mut sub = Parser { idents: idn, toks: toks, offs: offs, pos: 0, no_struct: false, expr_path: false, pending_gt: 0, in_cond: false, expand: false, mac_names: Vec::new(), mac_matchers: Vec::new(), mac_bodies: Vec::new(), mac_metavars: Vec::new(), mac_reps: Vec::new(), mac_depth: 0, rule_toks: Vec::new(), rep_els: Vec::new(), drop_stmt: false, stmt_attrs: Vec::new(), resolve: false, moddir: String::from(""), pend_move: false, pend_async: false };
    sub.parse_expr(0)
}
fn s9_emit_lit_byte(bval: i64) { let c = s9const(bval); s9call1("__putc", c); }
// C-S9-BOOL-DISPLAY: a bool in Display position printed as 1/0, not true/false -- measured against rustc AND
// against L6, which gets it right. Every shape was wrong (literal, variable, comparison result, inside format!).
// Emitted as a runtime two-arm branch over the existing literal-byte path, the same mechanism println! already
// uses for the literal text around a placeholder. brz runs its block when the operand is NONZERO (the s9_if
// idiom), so the two arms test v and then v == 0.
fn s9_is_bool_op(op: &str) -> bool {
    if streq(op, "Eq") { return true; }
    if streq(op, "Ne") { return true; }
    if streq(op, "Lt") { return true; }
    if streq(op, "Gt") { return true; }
    if streq(op, "Le") { return true; }
    if streq(op, "Ge") { return true; }
    if streq(op, "And") { return true; }
    if streq(op, "Or") { return true; }
    false
}
// Takes the ALREADY-COMPUTED type texts. An earlier version did its own s9_lookup_type/s9_field_type lookups,
// which duplicated the ones every placeholder call site already performs -- measured: that DOUBLED gen2's arena
// requirement (floor ~40% -> ~70-80% of the declared arena) and tripped the arena gate. The lookups walk the
// whole env building Strings, and every String costs an arena allocation that is never freed.
fn s9_is_bool_val(e: &ExprK, ty: &str, fty: &str) -> bool {
    match e {
        ExprK::Bool(_b) => true,
        ExprK::Bin(op, _a, _b) => s9_is_bool_op(op),
        ExprK::Unary(op, _a) => streq(op, "Not"),
        ExprK::Paren(inner) => s9_is_bool_val(inner, ty, fty),
        _ => streq(ty, "Ty Path bool") || streq(fty, "Ty Path bool") || streq(ty, "bool") || streq(fty, "bool")
    }
}
fn s9_print_bool(v: i32) {
    s9emit_brz(v);
    s9_emit_lit_byte(116); s9_emit_lit_byte(114); s9_emit_lit_byte(117); s9_emit_lit_byte(101);   // "true"
    s9emit_pool();
    let z = s9const(0); let nv = s9op("eq", v, z);
    s9emit_brz(nv);
    s9_emit_lit_byte(102); s9_emit_lit_byte(97); s9_emit_lit_byte(108); s9_emit_lit_byte(115); s9_emit_lit_byte(101);   // "false"
    s9emit_pool();
}

fn s9_nth_arg_src(args: &Vec<String>, k: i32) -> String {
    // args[2..] are the RAW value tokens after the format string; they hold N args separated by TOP-LEVEL commas (commas at
    // paren/bracket/BRACE depth 0 -- a comma inside `f(a, b)`, `[a, b]` or a match arm does NOT separate). Return the k-th segment's source (only
    // the k-th segment's tokens are appended -- no Vec<String>, no String reassignment, both L6 landmines). O(N) per k.
    let mut src = String::new();
    let mut seg = 0;
    let mut depth = 0;
    let mut i = 0;
    for t in args.iter() {
        if i >= 2 {
            // CT-23: `{` and `}` were missing here, so a comma inside `match n { 0 => "a", _ => "b" }` split the
            // argument and `match n { 0 => "a"` was handed to s9_reparse_expr, which spun on the fragment until
            // the progress guard fired. s9_vec_seg_src twenty lines away already tracks all three delimiters.
            if streq(t, "(") || streq(t, "[") || streq(t, "{") { depth += 1; if seg == k { src.push_str(t); src.push(' '); } }
            else if streq(t, ")") || streq(t, "]") || streq(t, "}") { depth -= 1; if seg == k { src.push_str(t); src.push(' '); } }
            else if streq(t, ",") && depth == 0 { seg += 1; }
            else { if seg == k { src.push_str(t); src.push(' '); } }
        }
        i += 1;
    }
    src
}
// CL-6: an f64 literal's TEXT (`ExprK::LitFloat` keeps the source text) -> the fixed-point scaled i64,
// round(value * 1e9). Fraction digits past the 9th are TRUNCATED, which is the scale's own limit, not a bug here.
// A trailing `f64`/`f32` suffix is skipped. No exponent form (`1e3`) -- CL-7.
fn s9_flt_scaled(t: &str) -> i64 {
    let b = t.as_bytes();
    let mut i = 0;
    let mut ip: i64 = 0;
    while i < b.len() { if b[i] >= 48 && b[i] <= 57 { ip = ip * 10 + ((b[i] - 48) as i64); } else if b[i] == 95 { } else { break; } i = i + 1; }
    let mut fr: i64 = 0;
    let mut nd = 0;
    if i < b.len() { if b[i] == 46 {
        i = i + 1;
        while i < b.len() { if b[i] >= 48 && b[i] <= 57 { if nd < 9 { fr = fr * 10 + ((b[i] - 48) as i64); nd = nd + 1; } } else if b[i] == 95 { } else { break; } i = i + 1; }
    } }
    while nd < 9 { fr = fr * 10; nd = nd + 1; }
    let sc: i64 = 1000000000;
    ip * sc + fr
}
// is this placeholder argument an f64? a float LITERAL, or a binding/field/init whose declared type says so.
fn s9_is_float_val(e: &ExprK, ty: &str, fty: &str, ity: &str) -> bool {
    match e { ExprK::LitFloat(_t) => { return true; } _ => {} }
    if str_contains(ty, "f64") || str_contains(ty, "f32") { return true; }
    if str_contains(fty, "f64") || str_contains(fty, "f32") { return true; }
    str_contains(ity, "f64") || str_contains(ity, "f32")
}
// CL-6 print: integer part via __pi, then `.` + the stripped fraction when the fraction is non-zero (so `2.0`
// prints `2`, matching rustc's Display). Sign is carried by the integer part; the fraction is made positive so a
// negative value does not lose it (L6's own emission drops the fraction of a negative -- not adopted).
fn s9_print_flt(v: i32) {
    // EVERY op here is the 8-suffixed 64-bit form. The scaled value passes 2^31 at 2.15 -- measured: with the
    // 32-bit ops, 1.5 and 0.125 printed correctly and 3.14159 printed 4294967295.153377296. s9op selects the
    // width from the op NAME (`s9_op_is64` = last byte is '8'), so the width is chosen here, not inferred.
    let sc = s9const_fresh(1000000000);
    let ipart = s9op("div8", v, sc);
    s9call1("__pi", ipart);
    let raw = s9op("mod8", v, sc);
    let z = s9const_fresh(0);
    let neg = s9op("lt8", raw, z);
    let two = s9const_fresh(2);
    let m = s9op("mul8", two, neg);
    let one = s9const_fresh(1);
    let sgn = s9op("sub8", one, m);
    let fr = s9op("mul8", raw, sgn);
    let nz = s9op("gt8", fr, z);
    s9emit_brz(nz);
    let dot = s9const_fresh(46);
    s9call1("__putc", dot);
    let w = s9const_fresh(9);
    s9call2("__strip_frac", fr, w);
    s9emit_pool();
}
// CL-12 format specs. The placeholder scanner recognised ONLY the exact two bytes `{}`, so `{:.2}` matched
// nothing: its bytes were emitted LITERALLY and -- the worse half -- the argument was never consumed, so every
// later placeholder took the wrong argument. Measured: `println!("{:.1} {}", 1.25, 9)` printed `{:.1} 1.25`.
// The scanner below accepts ANY `{...}`, so the argument is always consumed and a spec this tier does not render
// degrades to plain Display instead of desynchronising the whole line.
// CL-16 spec prefix, per Rust's grammar `[[fill]align][sign][#][0][width][.precision][type]`. The three are
// parsed by ONE forward walk so they cannot disagree about where the width starts -- `{:04}` is the zero FLAG
// then width 4, while `{:10}` is width 10, and a `.` ends the width so `{:.2}` has none.
fn s9_spec_i0(sp: &str) -> usize {
    let b = sp.as_bytes();
    if b.len() >= 2 { if b[1] == 60 || b[1] == 62 || b[1] == 94 { return 2; } }
    if b.len() >= 1 { if b[0] == 60 || b[0] == 62 || b[0] == 94 { return 1; } }
    0
}
fn s9_spec_align(sp: &str) -> i64 {
    let b = sp.as_bytes();
    if b.len() >= 2 { if b[1] == 60 || b[1] == 62 || b[1] == 94 { return b[1] as i64; } }
    if b.len() >= 1 { if b[0] == 60 || b[0] == 62 || b[0] == 94 { return b[0] as i64; } }
    0
}
fn s9_spec_fill(sp: &str) -> i64 {
    let b = sp.as_bytes();
    if b.len() >= 2 { if b[1] == 60 || b[1] == 62 || b[1] == 94 { return b[0] as i64; } }
    let mut i = s9_spec_i0(sp);
    if i < b.len() { if b[i] == 43 { i = i + 1; } }
    if i < b.len() { if b[i] == 35 { i = i + 1; } }
    if i < b.len() { if b[i] == 48 { return 48; } }
    32
}
fn s9_spec_zero(sp: &str) -> bool {
    let b = sp.as_bytes();
    let mut i = s9_spec_i0(sp);
    if i < b.len() { if b[i] == 43 { i = i + 1; } }
    if i < b.len() { if b[i] == 35 { i = i + 1; } }
    if i < b.len() { if b[i] == 48 { return true; } }
    false
}
fn s9_spec_width(sp: &str) -> i64 {
    let b = sp.as_bytes();
    let mut i = s9_spec_i0(sp);
    if i < b.len() { if b[i] == 43 { i = i + 1; } }
    if i < b.len() { if b[i] == 35 { i = i + 1; } }
    if i < b.len() { if b[i] == 48 { i = i + 1; } }
    let mut n: i64 = 0;
    let mut any = false;
    while i < b.len() { if b[i] >= 48 && b[i] <= 57 { n = n * 10 + ((b[i] - 48) as i64); any = true; i = i + 1; } else { if b[i] == 36 { return 0; } i = b.len(); } }
    if any { return n; }
    0
}
fn s9_pow10(n: i64) -> i64 { let mut r: i64 = 1; let mut i: i64 = 0; while i < n { r = r * 10; i = i + 1; } r }
// the spec text of `{...}` -- everything after the `:`; "" for a bare `{}` or `{:}`.
// FINDING #8: takes the format STRING and re-derives its bytes here. An earlier revision took a `&Vec<u8>`
// param and the spec came back empty at every call site -- `{:x}` printed 255, `{:.2}` printed full precision --
// while the desync fix in the same change worked. A `&Vec` param is not a lowered subset access shape on this
// tier; the file already records the rule at Parser::rule_toks and under finding #8.
fn s9_fmt_spec(f: &str, st: usize, en: usize) -> String {
    let fb = f.as_bytes();
    let mut out = String::new();
    let mut i = st + 1;
    let mut seen = false;
    while i < en { if seen { out.push(fb[i] as char); } else { if fb[i] == 58 { seen = true; } } i = i + 1; }
    out
}
// the precision digits of a `.N` spec, or -1 when the spec carries none.
fn s9_spec_prec(sp: &str) -> i64 {
    let b = sp.as_bytes();
    let mut i = 0;
    while i < b.len() { if b[i] == 46 { let mut n: i64 = 0; let mut j = i + 1; let mut any = false; while j < b.len() { if b[j] >= 48 && b[j] <= 57 { n = n * 10 + ((b[j] - 48) as i64); any = true; } else { j = b.len(); } j = j + 1; } if any { return n; } return 0 - 1; } i = i + 1; }
    0 - 1
}
// the trailing type character of a spec (`e`/`x`/`X`/`b`/`o`/`?`), or 0.
fn s9_spec_kind(sp: &str) -> i64 { let b = sp.as_bytes(); if b.len() == 0 { return 0; } let c = b[b.len() - 1]; if c == 101 || c == 120 || c == 88 || c == 98 || c == 111 || c == 63 { return c as i64; } 0 }
fn s9_spec_alt(sp: &str) -> bool { let b = sp.as_bytes(); let mut i = 0; while i < b.len() { if b[i] == 35 { return true; } i = i + 1; } false }
// `{:.N}` on a softfloat. N is a COMPILE-TIME constant from the format string, so every power of ten here is an
// emit-time constant and the emitted code is a handful of arithmetic ops.
// Rust's Display rounds HALF TO EVEN, not half away from zero -- measured: `{:.0}` of 2.5 is `2`, not `3`. The
// tie is resolved branch-free: inc = (rem>half) + (rem==half)*(floor odd).
fn s9_print_flt_prec(v: i32, n: i64) {
    let z = s9const_fresh(0);
    let neg = s9op("lt8", v, z);
    s9emit_brz(neg);
    let mm = s9const_fresh(45);
    s9call1("__putc", mm);
    s9emit_pool();
    let two = s9const_fresh(2);
    let m = s9op("mul8", two, neg);
    let one = s9const_fresh(1);
    let sgn = s9op("sub8", one, m);
    let av = s9op("mul8", v, sgn);
    let p = s9_pow10(9 - n);
    let p10 = s9const_fresh(p);
    let fl = s9op("div8", av, p10);
    let rem = s9op("mod8", av, p10);
    let hf = s9const_fresh(p / 2);
    let gt = s9op("gt8", rem, hf);
    let eq = s9op("eq", rem, hf);
    let t2 = s9const_fresh(2);
    let odd = s9op("mod8", fl, t2);
    let tie = s9op("mul8", eq, odd);
    let inc = s9op("add8", gt, tie);
    let r = s9op("add8", fl, inc);
    let pn = s9const_fresh(s9_pow10(n));
    let ip = s9op("div8", r, pn);
    s9call1("__pi", ip);
    if n > 0 {
        let dot = s9const_fresh(46);
        s9call1("__putc", dot);
        let fr = s9op("mod8", r, pn);
        let w = s9const_fresh(n);
        s9call2("__print_padz", fr, w);
    }
}
fn s9_print_radix(v: i32, base: i64, up: bool, alt: bool, pfx: i64) {
    if alt { let z0 = s9const_fresh(48); s9call1("__putc", z0); let pc = s9const_fresh(pfx); s9call1("__putc", pc); }
    let b = s9const_fresh(base);
    if up { s9call2("__print_radixup", v, b); } else { s9call2("__print_radix", v, b); }
}
// CL-12 dispatch. The plain-Display path is EXACTLY the chain that used to sit inline in the scanner, moved out
// unchanged; the spec branches sit in front of it. Order matters: the recorded arm-drop hazard hits DEEP arms,
// and the plain path is the one that must never drop.
fn s9_fmt_plain(v: i32, isflt: bool, isbool: bool, isstr: bool) {
    if isflt { s9_print_flt(v); return; }
    if isbool { s9_print_bool(v); return; }
    if isstr { s9call1("__print_str", v); return; }
    s9call1("__pi", v);
}
// CL-16. Padding needs the RENDERED width at run time, which differs per kind, so each supported kind computes
// its own length and the pad is emitted around the render. Supported: plain integers, radix, and strings --
// exactly the kinds the corpus needs. A float with a width falls through unpadded (CL-18), as does a `N$`
// dynamic width (CL-17); both still render their value correctly, which is the whole point of the CL-12
// mouth-gate: an unsupported spec degrades, it does not desynchronise.
// Rust's DEFAULT alignment is right for numerics and LEFT for strings; `0` fill is numeric-only and goes
// between the sign and the digits, which is why `{:05}` of -42 is `-0042` and not `000-42`.
fn s9_is_str_val(e: &ExprK, ty: &str, fty: &str, ity: &str, env: &S9Env) -> bool {
    str_contains(ty, "String") || str_contains(ty, "str") || str_contains(fty, "String") || str_contains(fty, "str") || str_contains(ity, "String") || str_contains(ity, "str") || str_contains(&s9_elem_ty(env, e), "String") || str_contains(&s9_elem_ty(env, e), "str")
}
fn s9_fmt_pad(n: i32, fill: i64) { let f = s9const_fresh(fill); s9call2("__pfill", n, f); }
fn s9_fmt_width(sp: &str, v: i32, isstr: bool) {
    let w = s9_spec_width(sp);
    let kind = s9_spec_kind(sp);
    let al = s9_spec_align(sp);
    let fill = s9_spec_fill(sp);
    let zero = s9_spec_zero(sp);
    let wc = s9const_fresh(w);
    if isstr {
        let len = s9_load_at(v, 0);
        let pad = s9op("sub", wc, len);
        if al == 94 { let t2 = s9const_fresh(2); let lh = s9op("div", pad, t2); let rh = s9op("sub", pad, lh); s9_fmt_pad(lh, fill); s9call1("__print_str", v); s9_fmt_pad(rh, fill); return; }
        if al == 62 { s9_fmt_pad(pad, fill); s9call1("__print_str", v); return; }
        s9call1("__print_str", v); s9_fmt_pad(pad, fill);
        return;
    }
    let z = s9const_fresh(0);
    let neg = s9op("lt8", v, z);
    let two = s9const_fresh(2);
    let m = s9op("mul8", two, neg);
    let one = s9const_fresh(1);
    let sgn = s9op("sub8", one, m);
    let av = s9op("mul8", v, sgn);
    let base = s9_radix_of(kind);
    let d = if base > 0 { let bc = s9const_fresh(base); s9call2("__ndigb", av, bc) } else { s9call1("__ndig", av) };
    let len = s9op("add", d, neg);
    let pad = s9op("sub", wc, len);
    if zero {
        s9emit_brz(neg);
        let mm = s9const_fresh(45);
        s9call1("__putc", mm);
        s9emit_pool();
        s9_fmt_pad(pad, 48);
        s9_fmt_body(kind, sp, av);
        return;
    }
    if al == 60 { s9_fmt_body(kind, sp, v); s9_fmt_pad(pad, fill); return; }
    // `^` centre: the SMALLER half goes on the left, which is what Rust does for an odd remainder.
    if al == 94 { let t2 = s9const_fresh(2); let lh = s9op("div", pad, t2); let rh = s9op("sub", pad, lh); s9_fmt_pad(lh, fill); s9_fmt_body(kind, sp, v); s9_fmt_pad(rh, fill); return; }
    s9_fmt_pad(pad, fill);
    s9_fmt_body(kind, sp, v);
}
fn s9_radix_of(kind: i64) -> i64 { if kind == 120 { return 16; } if kind == 88 { return 16; } if kind == 98 { return 2; } if kind == 111 { return 8; } 0 }
fn s9_fmt_body(kind: i64, sp: &str, v: i32) {
    if kind == 120 { s9_print_radix(v, 16, false, s9_spec_alt(sp), 120); return; }
    if kind == 88 { s9_print_radix(v, 16, true, s9_spec_alt(sp), 120); return; }
    if kind == 98 { s9_print_radix(v, 2, false, s9_spec_alt(sp), 98); return; }
    if kind == 111 { s9_print_radix(v, 8, false, s9_spec_alt(sp), 111); return; }
    s9call1("__pi", v);
}
fn s9_fmt_emit(sp: &str, v: i32, isflt: bool, isbool: bool, isstr: bool) {
    let w = s9_spec_width(sp);
    if w > 0 { if isflt == false { s9_fmt_width(sp, v, isstr); return; } }
    let kind = s9_spec_kind(sp);
    if kind == 101 { s9call1("__print_exp", v); return; }
    if kind == 120 { s9_print_radix(v, 16, false, s9_spec_alt(sp), 120); return; }
    if kind == 88 { s9_print_radix(v, 16, true, s9_spec_alt(sp), 120); return; }
    if kind == 98 { s9_print_radix(v, 2, false, s9_spec_alt(sp), 98); return; }
    if kind == 111 { s9_print_radix(v, 8, false, s9_spec_alt(sp), 111); return; }
    let pr = s9_spec_prec(sp);
    if pr >= 0 { if isflt { s9_print_flt_prec(v, pr); return; } }
    s9_fmt_plain(v, isflt, isbool, isstr);
}
fn s9_println(args: &Vec<String>, env: &S9Env) {
    // args are RAW TOKENS: [format-string, ",", value-tokens..]. E3 slice-3 (multi-placeholder): re-parse args[0] to
    // ExprK::Str(content) (quotes stripped) then WALK the format string emitting each literal byte via __putc; at the k-th
    // `{}` placeholder emit the k-th top-level-comma-separated value arg (s9_nth_arg_src) re-parsed + lowered via __pi.
    // N placeholders <-> N args. STRING args still print as their address (str-vs-int dispatch is C5-1b); INT args are exact.
    let fe = s9_reparse_expr(&args[0]);
    let fmt = match fe { ExprK::Str(content) => content, _ => String::from("") };
    let have_arg = args.len() >= 3;
    let fb = fmt.as_bytes();
    let mut j = 0;
    let mut k = 0;
    while j < fb.len() {
        if fb[j] == 123 && j + 1 < fb.len() && fb[j + 1] == 123 {
            s9_emit_lit_byte(123); j += 2;   // `{{` -> literal `{`
        } else if fb[j] == 125 && j + 1 < fb.len() && fb[j + 1] == 125 {
            s9_emit_lit_byte(125); j += 2;   // `}}` -> literal `}`
        } else if fb[j] == 123 {
            // CL-12: ANY `{...}` is a placeholder. Scan to the closing brace; an unterminated `{` falls through
            // as a literal byte so a malformed format string cannot run off the end.
            let mut en = j + 1;
            while en < fb.len() { if fb[en] == 125 { en = fb.len() + 1; } else { en = en + 1; } }
            if en == fb.len() + 1 {
                let mut e2 = j + 1;
                while e2 < fb.len() { if fb[e2] == 125 { break; } e2 = e2 + 1; }
                let sp = s9_fmt_spec(&fmt, j, e2);
                if have_arg { let asrc = s9_nth_arg_src(args, k); let e = s9_reparse_expr(&asrc); let ty = s9_lookup_type(env, &s9_bare_name(&e)); let v = if streq(&ty, "iterscalar") { let t0 = s9expr(&e, env); s9_slot_load(t0, 0) } else { s9expr(&e, env) }; let fty = s9_field_type(env, &s9_field_name(&e)); let ity = s9_init_ty(env, &e); let isf = s9_is_float_val(&e, &ty, &fty, &ity); let isb = s9_is_bool_val(&e, &ty, &fty); let iss = s9_is_str_val(&e, &ty, &fty, &ity, env); s9_fmt_emit(&sp, v, isf, isb, iss); }
                k += 1;
                j = e2 + 1;
            } else { s9_emit_lit_byte(123); j += 1; }
        } else if fb[j] == 92 && j + 1 < fb.len() && s9_esc_byte(fb[j + 1]) >= 0 {
            s9_emit_lit_byte(s9_esc_byte(fb[j + 1])); j += 2;   // C-G2D-STR-ESCAPE: decode \n/\t/\"/\\ in a println! literal
        } else {
            s9_emit_lit_byte(fb[j] as i64);
            j += 1;
        }
    }
    let nl = s9const(10); s9call1("__putc", nl);
    let fv = s9fresh(); println!("  v{}: i32 = call __flush", fv);
}
fn is_all_digits(s: &str) -> bool { let b = s.as_bytes(); if b.len() == 0 { return false; } let mut i = 0; while i < b.len() { if b[i] < 48 || b[i] > 57 { return false; } i += 1; } true }
fn str_to_i64(s: &str) -> i64 { let b = s.as_bytes(); let mut n: i64 = 0; let mut i = 0; while i < b.len() { n = n * 10 + (b[i] - 48) as i64; i += 1; } n }
fn trim_ws(s: &str) -> String { let b = s.as_bytes(); let mut lo = 0; while lo < b.len() && b[lo] == 32 { lo += 1; } let mut hi = b.len(); while hi > lo && b[hi - 1] == 32 { hi -= 1; } format!("{}", &s[lo..hi]) }
fn s9_runtime() {
    // C-G2W-HEAP-EXHAUST: __heap was 1MB. It is a BUMP arena with no reclamation, so any O(n^2) allocation idiom
    // exhausts it -- and `s = format!("{}", s)` accumulation is exactly that: a fresh String per call, none freed.
    // Measured: format!-accumulation died past ~900 iterations where push_str reached 8000+. Exhaustion does not
    // fault, it hands back a corrupt string and lets the dump run to completion, so it presented as CONTENT DIFFs
    // (ast4/lexer4/rerrors) rather than as a crash -- "it did not crash" is not evidence of no capacity problem.
    // RAISED AGAIN 16MB -> 1GB (2026-08-24). 16MB was sized before the aggregate word doubled to 8 bytes; at that
    // width a 21-fixture input exhausted it, and exhaustion is SILENT: it produced a 641MB runaway at exit 0 in one
    // case and a 2-byte string corruption ("vis=priv" -> "vis=prK\xe3") in another. Both are worse than a fault.
    // 64MB clears the 35-fixture concatenation; 1GB clears the whole 6941-line parser DUMP; 1.75GB is what the
    // SELF-EMIT needs -- the emit workload allocates far more than the dump. At 1.75GB the SEM0 fixed point holds:
    // gen2 == gen3, byte-identical, 299,295 lines. This is exactly the value L6 independently arrived at for the
    // same workload. Costs address space only (demand-zero; cf sizes p_memsz to cover it, here 4048MB of the
    // 4080MB ceiling -- so this is close to the limit the 32-bit ebx frame pointer allows).
    println!("gdef __str_arena 16777216"); println!("gdef __str_cursor 8"); println!("gdef __heap 2013265920"); println!("gdef __heap_cursor 8"); println!("gdef __obuf 1048576"); println!("gdef __obuf_n 8");
    println!("fn __flush"); println!("  v0: i32 = gref __obuf_n"); println!("  v1: i32 = load v0"); println!("  v2: i32 = const 1"); println!("  v3: i32 = const 1"); println!("  v4: i32 = gref __obuf"); println!("  v5: i32 = syscall v2 v3 v4 v1"); println!("  v6: i32 = const 0"); println!("  store v0 v6"); println!("  ret v6");
    println!("fn __putc"); println!("  v0: i32 = param"); println!("  v1: i32 = gref __obuf_n"); println!("  v2: i32 = load v1"); println!("  v3: i32 = gref __obuf"); println!("  v4: i32 = add v3 v2"); println!("  storeb v4 v0"); println!("  v5: i32 = const 1"); println!("  v6: i32 = add v2 v5"); println!("  store v1 v6"); println!("  v7: i32 = const 1048576"); println!("  v8: i32 = ge v6 v7"); println!("  brz v8"); println!("  v9: i32 = call __flush"); println!("  pool"); println!("  v10: i32 = const 0"); println!("  ret v10");
    println!("fn __print_uint"); println!("  v0: i32 = param"); println!("  v1: i32 = const 0"); println!("  v2: i32 = const 10"); println!("  v3: i32 = lt8 v0 v2"); println!("  v4: i32 = eq v3 v1"); println!("  brz v4"); println!("  v5: i64 = div8 v0 v2"); println!("  v6: i32 = call __print_uint v5"); println!("  pool"); println!("  v7: i32 = const 48"); println!("  v9: i64 = mod8 v0 v2"); println!("  v10: i32 = add v7 v9"); println!("  v11: i32 = call __putc v10"); println!("  ret v1");
    println!("fn __pi"); println!("  v0: i32 = param"); println!("  v1: i32 = const 0"); println!("  v2: i32 = lt8 v0 v1"); println!("  brz v2"); println!("  v3: i32 = const 45"); println!("  v4: i32 = call __putc v3"); println!("  v5: i32 = const 0"); println!("  v6: i64 = sub8 v5 v0"); println!("  v7: i32 = call __print_uint v6"); println!("  v8: i32 = const 0"); println!("  ret v8"); println!("  pool"); println!("  v9: i32 = call __print_uint v0"); println!("  v10: i32 = const 0"); println!("  ret v10");
    // __print_str(base): a String/&str value is the [len@0, packed-bytes@4+] block. Read len, then recurse over the byte run
    // (data ptr = base+4). Recursion (not a raw loop) mirrors __print_uint -- no loop/cont ops needed in hand-written rzir.
    println!("fn __print_str"); println!("  v0: i32 = param"); println!("  v1: i32 = load v0"); println!("  v2: i32 = const 8"); println!("  v3: i32 = add v0 v2"); println!("  v4: i32 = load v3"); println!("  v5: i32 = call __print_str_at v4 v1"); println!("  v6: i32 = const 0"); println!("  ret v6");
    println!("fn __print_str_at"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  loop"); println!("  v2: i32 = const 0"); println!("  v3: i32 = lt v2 v1"); println!("  brz v3"); println!("  v4: i32 = loadb v0"); println!("  v5: i32 = call __putc v4"); println!("  v6: i32 = const 1"); println!("  v7: i64 = add8 v0 v6"); println!("  v0: i32 = or v7 v7"); println!("  v8: i64 = sub8 v1 v6"); println!("  v1: i32 = or v8 v8"); println!("  cont"); println!("  jmp"); println!("  endloop"); println!("  v9: i32 = const 0"); println!("  ret v9");
    // [emit-s9-vecstring-elem locale R2] format! placeholder substitution helpers, all recursive (no loop ops, mirroring
    // __print_str_at/__print_uint). __scat_at(ddst, src, n): copy n bytes src->ddst. __scat_s(dst, src): append String src's
    // bytes to String dst ([len@0, bytes@4]) + dst.len += src.len. __scat_i(dst, n): append n's decimal (n>=0) to dst.
    println!("fn __scat_at"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = param"); println!("  loop"); println!("  v3: i32 = const 0"); println!("  v4: i32 = lt v3 v2"); println!("  brz v4"); println!("  v5: i32 = loadb v1"); println!("  storeb v0 v5"); println!("  v6: i32 = const 1"); println!("  v7: i64 = add8 v0 v6"); println!("  v0: i32 = or v7 v7"); println!("  v8: i64 = add8 v1 v6"); println!("  v1: i32 = or v8 v8"); println!("  v9: i64 = sub8 v2 v6"); println!("  v2: i32 = or v9 v9"); println!("  cont"); println!("  jmp"); println!("  endloop"); println!("  v10: i32 = const 0"); println!("  ret v10");
    // __str_eq(a,b): 1 if the two String/&str blocks ([len@0, bytes@4+]) are byte-equal, else 0. lens differ -> 0; else compare
    // len bytes via __streq_at. (C-G2D-STR-EQ: string `==` was a pointer compare.) __streq_at(pa,pb,n): recurse over n bytes.
    println!("fn __str_eq"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = load v0"); println!("  v3: i32 = load v1"); println!("  v4: i32 = eq v2 v3"); println!("  brz v4"); println!("  v5: i32 = const 8"); println!("  v6: i32 = add v0 v5"); println!("  v7: i32 = load v6"); println!("  v8: i32 = add v1 v5"); println!("  v9: i32 = load v8"); println!("  v10: i32 = call __streq_at v7 v9 v2"); println!("  ret v10"); println!("  pool"); println!("  v11: i32 = const 0"); println!("  ret v11");
    println!("fn __streq_at"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = param"); println!("  loop"); println!("  v3: i32 = const 0"); println!("  v4: i32 = lt v3 v2"); println!("  brz v4"); println!("  v5: i32 = loadb v0"); println!("  v6: i32 = loadb v1"); println!("  v7: i32 = eq v5 v6"); println!("  brz v7"); println!("  v8: i32 = const 1"); println!("  v9: i64 = add8 v0 v8"); println!("  v0: i32 = or v9 v9"); println!("  v10: i64 = add8 v1 v8"); println!("  v1: i32 = or v10 v10"); println!("  v11: i64 = sub8 v2 v8"); println!("  v2: i32 = or v11 v11"); println!("  cont"); println!("  jmp"); println!("  endloop"); println!("  v12: i32 = const 0"); println!("  v13: i32 = eq v2 v12"); println!("  ret v13");
    println!("fn __scat_s"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = load v1"); println!("  v3: i32 = call __str_grow v0 v2"); println!("  v4: i32 = load v0"); println!("  v5: i32 = const 8"); println!("  v6: i32 = add v0 v5"); println!("  v7: i32 = load v6"); println!("  v8: i32 = add v7 v4"); println!("  v9: i32 = add v1 v5"); println!("  v10: i32 = load v9"); println!("  v11: i32 = call __scat_at v8 v10 v2"); println!("  v12: i32 = add v4 v2"); println!("  store v0 v12"); println!("  v13: i32 = const 0"); println!("  ret v13");
    println!("fn __scat_i"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = const 10"); println!("  v3: i32 = lt8 v1 v2"); println!("  v4: i32 = const 0"); println!("  v8: i32 = eq v3 v4"); println!("  brz v8"); println!("  v5: i64 = div8 v1 v2"); println!("  v6: i32 = call __scat_i v0 v5"); println!("  pool"); println!("  v7: i32 = const 48"); println!("  v9: i64 = mod8 v1 v2"); println!("  v10: i32 = add v7 v9"); println!("  v11: i32 = const 1"); println!("  v12: i32 = call __str_grow v0 v11"); println!("  v13: i32 = load v0"); println!("  v14: i32 = const 8"); println!("  v15: i32 = add v0 v14"); println!("  v16: i32 = load v15"); println!("  v17: i32 = add v16 v13"); println!("  storeb v17 v10"); println!("  v18: i32 = const 1"); println!("  v19: i32 = add v13 v18"); println!("  store v0 v19"); println!("  v20: i32 = const 0"); println!("  ret v20");
    // C-G2D-STR-GROW __str_grow(base, need): ensure a header-model String [len@0, cap@4, dataptr@8] has cap >= len+need
    // BYTES. If not, alloc newbuf(newcap = (len+need)*2) from the heap, byte-copy the live `len` bytes (reuse __scat_at),
    // write dataptr+cap back. Byte stride (String), vs __vec_grow's 4-byte element stride. Callers reload dataptr after.
    println!("fn __str_grow"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = load v0"); println!("  v3: i32 = const 4"); println!("  v4: i32 = add v0 v3"); println!("  v5: i32 = load v4"); println!("  v6: i32 = add v2 v1"); println!("  v7: i32 = lt v5 v6"); println!("  brz v7"); println!("  v8: i32 = add v6 v6"); println!("  v9: i32 = gref __heap_cursor"); println!("  v10: i32 = load v9"); println!("  v11: i32 = gref __heap"); println!("  v12: i32 = add v11 v10"); println!("  v13: i32 = add v10 v8"); println!("  v14: i32 = gref __heap_cursor"); println!("  store v14 v13"); println!("  v15: i32 = const 8"); println!("  v16: i32 = add v0 v15"); println!("  v17: i32 = load v16"); println!("  v18: i32 = call __scat_at v12 v17 v2"); println!("  store v16 v12"); println!("  v19: i32 = const 4"); println!("  v20: i32 = add v0 v19"); println!("  store v20 v8"); println!("  pool"); println!("  v21: i32 = const 0"); println!("  ret v21");
    // [E1-GROW / SR-5] __vec_grow(base): a growable Vec is a 3-word header [len@0, cap@4, dataptr@8]; value = the header
    // address (STABLE -- only dataptr moves). On a full push, double cap: alloc newbuf(newcap*4) via halloc, byte-copy the
    // old cap*4 bytes (reuse __scat_at), write dataptr+cap back. Returns 0; the caller reloads dataptr after the call.
    println!("fn __vec_grow"); println!("  v0: i32 = param"); println!("  v1: i32 = const 4"); println!("  v2: i32 = add v0 v1"); println!("  v3: i32 = load v2"); println!("  v4: i32 = const 8"); println!("  v5: i32 = add v0 v4"); println!("  v6: i32 = load v5"); println!("  v7: i32 = const 2"); println!("  v8: i32 = mul v3 v7"); println!("  v9: i32 = const 8"); println!("  v10: i32 = mul v8 v9"); println!("  v11: i32 = gref __heap_cursor"); println!("  v12: i32 = load v11"); println!("  v13: i32 = gref __heap"); println!("  v14: i32 = add v13 v12"); println!("  v15: i32 = add v12 v10"); println!("  v16: i32 = gref __heap_cursor"); println!("  store v16 v15"); println!("  v17: i32 = mul v3 v9"); println!("  v18: i32 = call __scat_at v14 v6 v17"); println!("  store v5 v14"); println!("  store v2 v8"); println!("  v19: i32 = const 0"); println!("  ret v19");
    // CL-6 SOFTFLOAT runtime. An f64 is a FIXED-POINT scaled i64 (round(value * 1e9)); the backend has no float
    // ops at any tier and never will at this rung, so this is the representation L6 already uses -- these four
    // helpers are its emission, adopted rather than re-authored (locales/s1-literal-representation CL-6).
    // __ndig(n) = decimal digit count; __pzeros(k) = k literal '0's; __print_padz(n,w) = n zero-padded to w;
    // __strip_frac(f,w) = drop trailing zeros from the fraction, then print it padded to the remaining width.
    println!("fn __ndig"); println!("  v0: i32 = param"); println!("  v1: i32 = const 10"); println!("  v2: i32 = ge v0 v1"); println!("  brz v2"); println!("  v3: i32 = const 10"); println!("  v4: i32 = div v0 v3"); println!("  v5: i32 = call __ndig v4"); println!("  v6: i32 = const 1"); println!("  v7: i32 = add v5 v6"); println!("  ret v7"); println!("  pool"); println!("  v8: i32 = const 1"); println!("  ret v8");
    println!("fn __pzeros"); println!("  v0: i32 = param"); println!("  v1: i32 = const 0"); println!("  v2: i32 = gt v0 v1"); println!("  brz v2"); println!("  v3: i32 = const 48"); println!("  v4: i32 = call __putc v3"); println!("  v5: i32 = const 1"); println!("  v6: i32 = sub v0 v5"); println!("  v7: i32 = call __pzeros v6"); println!("  pool"); println!("  v8: i32 = const 0"); println!("  ret v8");
    println!("fn __print_padz"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = call __ndig v0"); println!("  v3: i32 = sub v1 v2"); println!("  v4: i32 = call __pzeros v3"); println!("  v5: i32 = call __print_uint v0"); println!("  v6: i32 = const 0"); println!("  ret v6");
    println!("fn __pfill"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = const 0"); println!("  v3: i32 = gt v0 v2"); println!("  brz v3"); println!("  v4: i32 = call __putc v1"); println!("  v5: i32 = const 1"); println!("  v6: i32 = sub v0 v5"); println!("  v7: i32 = call __pfill v6 v1"); println!("  pool"); println!("  v8: i32 = const 0"); println!("  ret v8");
    println!("fn __ndigb"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = ge v0 v1"); println!("  brz v2"); println!("  v3: i32 = div v0 v1"); println!("  v4: i32 = call __ndigb v3 v1"); println!("  v5: i32 = const 1"); println!("  v6: i32 = add v4 v5"); println!("  ret v6"); println!("  pool"); println!("  v7: i32 = const 1"); println!("  ret v7");
    println!("fn __striptz"); println!("  v0: i32 = param"); println!("  v1: i32 = const 10"); println!("  v2: i32 = mod v0 v1"); println!("  v3: i32 = const 0"); println!("  v4: i32 = eq v2 v3"); println!("  brz v4"); println!("  v5: i32 = const 10"); println!("  v6: i32 = div v0 v5"); println!("  v7: i32 = call __striptz v6"); println!("  ret v7"); println!("  pool"); println!("  ret v0");
    println!("fn __ppow10"); println!("  v0: i32 = param"); println!("  v1: i32 = const 0"); println!("  v2: i32 = gt v0 v1"); println!("  brz v2"); println!("  v3: i32 = const 1"); println!("  v4: i32 = sub v0 v3"); println!("  v5: i32 = call __ppow10 v4"); println!("  v6: i32 = const 10"); println!("  v7: i32 = mul v5 v6"); println!("  ret v7"); println!("  pool"); println!("  v8: i32 = const 1"); println!("  ret v8");
    println!("fn __print_radix"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = ge v0 v1"); println!("  brz v2"); println!("  v3: i32 = div v0 v1"); println!("  v4: i32 = call __print_radix v3 v1"); println!("  pool"); println!("  v5: i32 = mod v0 v1"); println!("  v6: i32 = const 10"); println!("  v7: i32 = ge v5 v6"); println!("  v8: i32 = const 39"); println!("  v9: i32 = mul v7 v8"); println!("  v10: i32 = const 48"); println!("  v11: i32 = add v10 v5"); println!("  v12: i32 = add v11 v9"); println!("  v13: i32 = call __putc v12"); println!("  ret v13");
    println!("fn __print_radixup"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = ge v0 v1"); println!("  brz v2"); println!("  v3: i32 = div v0 v1"); println!("  v4: i32 = call __print_radixup v3 v1"); println!("  pool"); println!("  v5: i32 = mod v0 v1"); println!("  v6: i32 = const 10"); println!("  v7: i32 = ge v5 v6"); println!("  v8: i32 = const 7"); println!("  v9: i32 = mul v7 v8"); println!("  v10: i32 = const 48"); println!("  v11: i32 = add v10 v5"); println!("  v12: i32 = add v11 v9"); println!("  v13: i32 = call __putc v12"); println!("  ret v13");
    println!("fn __print_exp"); println!("  v50: i32 = param"); println!("  v51: i32 = const 0"); println!("  v52: i32 = lt v50 v51"); println!("  brz v52"); println!("  v53: i32 = const 45"); println!("  v54: i32 = call __putc v53"); println!("  pool"); println!("  v55: i32 = const 1"); println!("  v56: i32 = const 2"); println!("  v57: i32 = mul v56 v52"); println!("  v58: i32 = sub v55 v57"); println!("  v0: i32 = mul v50 v58"); println!("  v1: i32 = const 0"); println!("  v2: i32 = eq v0 v1"); println!("  brz v2"); println!("  v3: i32 = const 48"); println!("  v4: i32 = call __putc v3"); println!("  v5: i32 = const 101"); println!("  v6: i32 = call __putc v5"); println!("  v7: i32 = const 48"); println!("  v8: i32 = call __putc v7"); println!("  v9: i32 = const 0"); println!("  ret v9"); println!("  pool"); println!("  v10: i32 = call __ndig v0"); println!("  v11: i32 = call __striptz v0"); println!("  v12: i32 = call __ndig v11"); println!("  v13: i32 = const 1"); println!("  v14: i32 = sub v12 v13"); println!("  v15: i32 = call __ppow10 v14"); println!("  v16: i32 = div v11 v15"); println!("  v17: i32 = const 48"); println!("  v18: i32 = add v16 v17"); println!("  v19: i32 = call __putc v18"); println!("  v20: i32 = const 1"); println!("  v21: i32 = gt v12 v20"); println!("  brz v21"); println!("  v22: i32 = const 46"); println!("  v23: i32 = call __putc v22"); println!("  v24: i32 = mod v11 v15"); println!("  v25: i32 = call __print_padz v24 v14"); println!("  pool"); println!("  v26: i32 = const 101"); println!("  v27: i32 = call __putc v26"); println!("  v28: i32 = const 1"); println!("  v29: i32 = sub v10 v28"); println!("  v30: i32 = call __print_uint v29"); println!("  v31: i32 = const 0"); println!("  ret v31");
    println!("fn __strip_frac"); println!("  v0: i32 = param"); println!("  v1: i32 = param"); println!("  v2: i32 = const 10"); println!("  v3: i32 = mod v0 v2"); println!("  v4: i32 = const 0"); println!("  v5: i32 = eq v3 v4"); println!("  brz v5"); println!("  v6: i32 = const 10"); println!("  v7: i32 = div v0 v6"); println!("  v8: i32 = const 1"); println!("  v9: i32 = sub v1 v8"); println!("  v10: i32 = call __strip_frac v7 v9"); println!("  ret v10"); println!("  pool"); println!("  v11: i32 = call __print_padz v0 v1"); println!("  v12: i32 = const 0"); println!("  ret v12");

}
fn s9_main(b: &Block, base: &S9Env, c: &Krate) {
    unsafe { S9NC = 0; }
    s9ck_reset();
    println!("ir main -> i32");   // `ir` = the program ENTRY (not `fn`); the backend turns its ret into exit
    s9_heap_globals();
    s9_static_inits(c);
    let mut names: Vec<String> = Vec::new(); let mut slots: Vec<i32> = Vec::new(); let mut types: Vec<String> = Vec::new();
    for st in b.stmts.iter() {
        match st {
            Stmt::Let(pat, ty, init, _els) => { match init { Some(e) => { let env = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&base.fnames), foffs: veccopy_i32(&base.foffs), ftypes: veccopy(&base.ftypes), frets_name: veccopy(&base.frets_name), frets_ty: veccopy(&base.frets_ty), types: veccopy(&types) }; let v = s9expr(e, &env); let ar = s9_pat_tuple_arity(pat); if ar >= 0 { let tup = s9_tup4(v); let mut i = 0; while i < ar { let snm = s9_pat_nth_bind(pat, i); let fsi = s9_tup_nth(&tup, i); if snm.len() > 0 { names.push(snm); slots.push(fsi); types.push(String::from("")); } i += 1; } } else { let nm = s9_patname(pat); let tt = s9_pick_ty(&env, s9_let_ty(ty), e); if nm.len() > 0 { names.push(nm); slots.push(s9_let_copy(e, v)); types.push(tt); } } } None => { let nm = s9_patname(pat); if nm.len() > 0 { let e3 = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&base.fnames), foffs: veccopy_i32(&base.foffs), ftypes: veccopy(&base.ftypes), frets_name: veccopy(&base.frets_name), frets_ty: veccopy(&base.frets_ty), types: veccopy(&types) }; let tt = s9_defer_pick(s9_let_ty(ty), b, &nm, &e3); names.push(nm); slots.push(s9_defer_slot()); types.push(tt); } } } }
            Stmt::Semi(e) => { let env = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&base.fnames), foffs: veccopy_i32(&base.foffs), ftypes: veccopy(&base.ftypes), frets_name: veccopy(&base.frets_name), frets_ty: veccopy(&base.frets_ty), types: veccopy(&types) }; s9_stmt(e, &env); }
            Stmt::Expr(e) => { let env = S9Env { names: veccopy(&names), slots: veccopy_i32(&slots), fnames: veccopy(&base.fnames), foffs: veccopy_i32(&base.foffs), ftypes: veccopy(&base.ftypes), frets_name: veccopy(&base.frets_name), frets_ty: veccopy(&base.frets_ty), types: veccopy(&types) }; s9_stmt(e, &env); }
            _ => {}
        }
    }
    let z = s9const(0); println!("  ret v{}", z);
}
fn s9_stmt(e: &ExprK, env: &S9Env) {
    match e {
        ExprK::MacCall(p, _bang, args) => { let nm = dump_path(p); if streq(&nm, "panic") { s9_panic_macro(); } else if streq(&nm, "assert") { s9_assert_macro(args, env); } else if streq(&nm, "println") || streq(&nm, "print") { s9_println(args, env); } }   // C-SEM0-TERMINATION: the STATEMENT form. `panic!(..);`/`assert!(..);` are statements, so wiring only the expression-position MacCall arm left them no-ops -- measured.
        ExprK::Assign(l, r) => { s9_assign(l, r, env); }
        ExprK::AssignOp(op, l, r) => { s9_assignop(op, l, r, env); }
        ExprK::Break(_lbl, v) => { let mut rv = -1; for e0 in v.iter() { rv = s9expr(e0, env); } if rv >= 0 { let bv = s9bv_get(); s9emit_break_val(bv, rv); } s9emit_brk(); }
        ExprK::Continue(_lbl) => { s9emit_cnt(); }
        _ => { s9expr(e, env); }
    }
}
fn veccopy_i32(v: &Vec<i32>) -> Vec<i32> { let mut o: Vec<i32> = Vec::new(); for x in v.iter() { o.push(*x); } o }
// C1 P-LC-CALL: a NON-main fn -> `fn NAME` + params bound (param i -> `v{i} = param`, name -> slot in the body env) +
// body value-block -> tail slot + `ret v{tail}`. Own v-namespace (reset S9NC). main stays `ir main` (s9_main).
// ---- C-S9-CLOSURE-LIFT: non-capturing closures, following L6's verified ABI (L6.rzrs.rs:457 + its emission).
// A closure lifts to a top-level `fn __closure_<h>(env, params...)`; its VALUE is an 8-byte cell {code@0, env@4}
// with code = faddr; a call loads both halves and dispatches `calli code env args...`.
// Keyed by a HASH OF THE CLOSURE'S DUMP TEXT, never by traversal order: the lift pass and the expression
// lowering traverse differently and order-matching would bind the wrong body SILENTLY. Two textually identical
// NON-CAPTURING closures are genuinely the same function, so sharing one lifted body is correct.
// CAPTURES ARE OUT OF THIS SLICE, which is what makes text-hashing sound.
// The param list is a space-joined STRING, not a Vec<String>: a `&Vec<T>` param walked for a field and returned
// as Vec<String> makes L6 emit a binary that SIGSEGVs on large inputs -- localized by prefix bisection, and the
// shape this file already warns about at the Parser::rule_toks note and under finding #8.
static mut S9CLH: [i64; 512] = [0; 512];
static mut S9CLN: i32 = 0;
fn s9clh_has(h: i64) -> bool { unsafe { let mut i = 0; while i < S9CLN { if S9CLH[i as usize] == h { return true; } i = i + 1; } false } }
fn s9clh_add(h: i64) { unsafe { if S9CLN < 512 { S9CLH[S9CLN as usize] = h; S9CLN = S9CLN + 1; } } }
fn s9_hash_str(t: &str) -> i64 {
    let b = t.as_bytes();
    let mut h: i64 = 2166136261;
    let mut i = 0;
    while i < b.len() { h = (h ^ (b[i] as i64)) * 16777619; h = h % 1000000007; if h < 0 { h = 0 - h; } i = i + 1; }
    h
}
fn s9_clos_hash(e: &ExprK) -> i64 { s9_hash_str(&dump_expr(e)) }
fn s9_clos_params_s(ps: &Vec<CParam>) -> String { let mut out = String::new(); out.push_str(" "); for q in ps.iter() { out.push_str(&pat_name(&q.pat)); out.push_str(" "); } out }
fn s9_clos_names_ok(e: &ExprK, ps: &str) -> bool {
    match e {
        ExprK::Path(pp) => { let nm = dump_path(pp); if str_contains(&nm, "::") { return true; } s9_str_has_word(ps, &nm) }
        ExprK::Bin(_o, a, b) => { if !s9_clos_names_ok(a, ps) { return false; } s9_clos_names_ok(b, ps) }
        ExprK::Unary(_o, a) => s9_clos_names_ok(a, ps),
        ExprK::Paren(a) => s9_clos_names_ok(a, ps),
        ExprK::Cast(a, _t) => s9_clos_names_ok(a, ps),
        ExprK::LitInt(_n) => true,
        ExprK::Bool(_b) => true,
        ExprK::Str(_t) => true,
        ExprK::Char(_c) => true,
        // A BLOCK body. Without this arm `|x| { x + 1 }` fell to `_ => false`, was judged unliftable, was never
        // emitted, and its call dangled. The return type had nothing to do with it -- every witness that
        // carried one also had a block, which is what made the rung-2 label wrong.
        // Conservative: statement-only blocks. A `let` inside would bind a name that is not a closure param,
        // and the whole point of this walker is that every name resolves to one.
        ExprK::Block(bb) => s9_clos_block_ok(bb, ps),
        _ => false
    }
}
fn s9_clos_block_ok(b: &Block, ps: &str) -> bool {
    let mut ok = true;
    for st in b.stmts.iter() {
        match st {
            Stmt::Expr(e) => { if s9_clos_names_ok(e, ps) == false { ok = false; } }
            Stmt::Semi(e) => { if s9_clos_names_ok(e, ps) == false { ok = false; } }
            Stmt::Empty => {}
            _ => { ok = false; }
        }
    }
    ok
}
// allocation-free membership in a space-delimited list. The previous form built a `format!(" {} ", nm)` for
// EVERY path visited, and with ~227 closure forms in this file that is a large number of arena Strings, none of
// which are ever freed -- it tripped the arena gate. Same shape as the bool-Display predicate that doubled the
// arena earlier: a predicate allocating per call site.
fn s9_str_has_word(hay: &str, w: &str) -> bool {
    let hb = hay.as_bytes(); let wb = w.as_bytes();
    if wb.len() == 0 { return false; }
    let mut i = 0;
    while i + wb.len() + 1 < hb.len() + 1 {
        if hb[i] == 32 {
            let mut k = 0; let mut ok = 1;
            while k < wb.len() { if i + 1 + k >= hb.len() { ok = 0; k = wb.len(); } else if hb[i + 1 + k] != wb[k] { ok = 0; k = wb.len(); } else { k = k + 1; } }
            if ok == 1 { if i + 1 + wb.len() < hb.len() { if hb[i + 1 + wb.len()] == 32 { return true; } } }
        }
        i = i + 1;
    }
    false
}
fn s9_clos_liftable(ps: &Vec<CParam>, body: &ExprK) -> bool { let names = s9_clos_params_s(ps); s9_clos_names_ok(body, &names) }
fn s9_emit_closure_fn(h: i64, ps: &Vec<CParam>, body: &ExprK, base: &S9Env) {
    unsafe { S9NC = 0; }
    s9ck_reset();
    println!("fn __closure_{}", h);
    let ev = s9fresh(); println!("  v{}: i32 = param", ev);   // env slot -- unused while captures are out of scope
    let mut names: Vec<String> = Vec::new(); let mut slots: Vec<i32> = Vec::new(); let mut types: Vec<String> = Vec::new();
    for q in ps.iter() { let sl = s9fresh(); println!("  v{}: i32 = param", sl); names.push(pat_name(&q.pat)); slots.push(sl); types.push(String::new()); }
    s9_heap_globals();
    let env = S9Env { names: names, slots: slots, fnames: veccopy(&base.fnames), foffs: veccopy_i32(&base.foffs), ftypes: veccopy(&base.ftypes), frets_name: veccopy(&base.frets_name), frets_ty: veccopy(&base.frets_ty), types: types };
    let tail = s9expr(body, &env);
    s9emit_ret(tail);
}
fn s9_lift_expr(e: &ExprK, base: &S9Env) {
    match e {
        ExprK::Closure(_mv, ps, _rt, body) => { if s9_clos_liftable(ps, body) { let h = s9_clos_hash(e); if !s9clh_has(h) { s9clh_add(h); s9_emit_closure_fn(h, ps, body, base); } } }
        ExprK::Call(_c, args) => { for a in args.iter() { s9_lift_expr(a, base); } }
        ExprK::Method(recv, _n, _t, args) => { s9_lift_expr(recv, base); for a in args.iter() { s9_lift_expr(a, base); } }
        ExprK::Bin(_o, a, b) => { s9_lift_expr(a, base); s9_lift_expr(b, base); }
        ExprK::Unary(_o, a) => s9_lift_expr(a, base),
        ExprK::Paren(a) => s9_lift_expr(a, base),
        ExprK::Ref(_m, a) => s9_lift_expr(a, base),
        ExprK::Block(b) => s9_lift_block(b, base),
        ExprK::If(c, t, els) => { s9_lift_expr(c, base); s9_lift_block(t, base); for x in els.iter() { s9_lift_expr(x, base); } }
        ExprK::While(_l, c, b) => { s9_lift_expr(c, base); s9_lift_block(b, base); }
        ExprK::Loop(_l, b) => s9_lift_block(b, base),
        ExprK::ForLoop(_l, _p, it, b) => { s9_lift_expr(it, base); s9_lift_block(b, base); }
        ExprK::Match(sc, arms) => { s9_lift_expr(sc, base); for a in arms.iter() { s9_lift_expr(&a.body, base); } }
        ExprK::Assign(l, r) => { s9_lift_expr(l, base); s9_lift_expr(r, base); }
        ExprK::AssignOp(_o, l, r) => { s9_lift_expr(l, base); s9_lift_expr(r, base); }
        ExprK::Ret(vs) => { for x in vs.iter() { s9_lift_expr(x, base); } }
        _ => {}
    }
}
fn s9_lift_block(b: &Block, base: &S9Env) {
    for st in b.stmts.iter() {
        match st {
            Stmt::Let(_p, _t, init, _e) => { match init { Some(x) => s9_lift_expr(x, base), None => {} } }
            Stmt::Expr(x) => s9_lift_expr(x, base),
            Stmt::Semi(x) => s9_lift_expr(x, base),
            _ => {}
        }
    }
}
fn s9_closure_value(h: i64) -> i32 {
    let cell = s9_alloc(8);
    let fa = s9fresh(); println!("  v{}: i32 = faddr __closure_{}", fa, h);
    s9_store_at(cell, 0, fa);
    let z = s9const(0);
    s9_store_at(cell, 4, z);
    cell
}
fn s9_calli(slot: i32, args: &Vec<ExprK>, env: &S9Env) -> i32 {
    let code = s9_load_at(slot, 0);
    let ev = s9_load_at(slot, 4);
    let mut a0 = 0; let mut a1 = 0; let mut a2 = 0; let mut n = 0;
    for a in args.iter() { let sx = s9expr(a, env); if n == 0 { a0 = sx; } else if n == 1 { a1 = sx; } else if n == 2 { a2 = sx; } n = n + 1; }
    let r = s9fresh();
    let mut suf = format!(" v{} v{}", code, ev);
    if n >= 1 { suf.push_str(&format!(" v{}", a0)); }
    if n >= 2 { suf.push_str(&format!(" v{}", a1)); }
    if n >= 3 { suf.push_str(&format!(" v{}", a2)); }
    println!("  v{}: i32 = calli{}", r, suf);
    r
}
fn s9_is_local_name(env: &S9Env, nm: &str) -> bool { let mut r = false; for n in env.names.iter() { if streq(n, nm) { r = true; } } r }
// callable as a closure only if the slot actually HOLDS a fat pointer: a `let` from a liftable closure
// (recorded type "closure") or an Fn-bounded param. An UNLIFTED closure leaves 0 and `calli` on 0 segfaults.
fn s9_is_closure_local(env: &S9Env, nm: &str) -> bool {
    if !s9_is_local_name(env, nm) { return false; }
    let ty = s9_lookup_type(env, nm);
    if str_contains(&ty, "closure") { return true; }
    if str_contains(&ty, "Fn") { return true; }
    false
}
fn s9_userfn(name: &str, params: &Vec<Param>, b: &Block, base: &S9Env) {
    unsafe { S9NC = 0; }
    s9ck_reset();
    println!("fn {}", name);
    let mut names: Vec<String> = Vec::new(); let mut slots: Vec<i32> = Vec::new(); let mut types: Vec<String> = Vec::new();
    for p in params.iter() { let sl = s9fresh(); println!("  v{}: i32 = param", sl); names.push(format!("{}", p.name)); slots.push(sl); types.push(dump_ty(&p.ty)); }
    // The parser records a bare receiver under its RECEIVER FORM -- `&self`, `&mut self`, `mut self` -- with a
    // synthesized type of literally `Ty Path Self`. So a call site looking up `self` found nothing, and `Self`
    // was never the impl type anyway. Bind the name `self` to the same slot, typed with the impl type carried
    // in the env as `$Self`. An extra names/slots/types row is safe: lookup is last-writer-wins.
    let mut si = 0; let mut sslot = 0 - 1;
    for p in params.iter() { if is_self_param(&p.name) { sslot = slots[si]; } si += 1; }
    if sslot >= 0 { let st = s9_self_ty(base); if st.len() > 0 { names.push(String::from("self")); slots.push(sslot); types.push(format!("Ty Path {}", st)); } }
    s9_heap_globals();
    let env = S9Env { names: names, slots: slots, fnames: veccopy(&base.fnames), foffs: veccopy_i32(&base.foffs), ftypes: veccopy(&base.ftypes), frets_name: veccopy(&base.frets_name), frets_ty: veccopy(&base.frets_ty), types: types };
    let tail = s9_bv(b, &env);
    s9emit_ret(tail);
}
// CL-A6 / C-SEM0-DUP-FN-SILENT (locales/call-abi-arity). A duplicate item name was ACCEPTED and produced a
// crashing image; rustc raises E0428. It cost a rung: a text-slice patch left two `fn s9_fmt_plain` definitions
// in this very file, and the result impersonated a scale-dependent substrate defect -- deterministic SIGSEGV at
// a fixed self-emit offset -- because nothing checked.
//
// Rust has TWO namespaces and they do not collide: `fn f` and `struct f` are both legal together (measured).
// So values (fn / static / const) and types (struct / enum / type alias) are accumulated separately. Methods are
// checked PER IMPL BLOCK: two impls may each define `m` (measured legal); one impl may not define it twice.
//
// Membership uses the space-joined-String + s9_str_has_word idiom rather than a `&Vec<String>` helper, because
// finding #8 records that a `&Vec` param is not a lowered subset access shape on this tier.
fn dup_add(seen: &str, nm: &str) -> String { let mut o = String::new(); o.push_str(seen); o.push_str(nm); o.push_str(" "); o }
fn dup_impl_methods(items: &Vec<Item>) -> String {
    let mut seen = String::from(" ");
    let mut dup = String::new();
    for m in items.iter() {
        if dup_skip(m) { continue; }
        match &m.kind {
            ItemKind::Fn(_g, _p, _r, _b, _a) => { let nm = format!("{}", m.ident); if s9_str_has_word(&seen, &nm) { if dup.len() == 0 { dup = nm; } } else { seen = dup_add(&seen, &nm); } }
            _ => {}
        }
    }
    dup
}
// CL-A6 FALSE POSITIVES, found only by widening the probe to rustc's whole compiler tree (1,847 files) -- both
// were invisible in the 192-file sample AND in the 904-probe corpus:
//   * `#[cfg(..)]` ALTERNATIVES legitimately define one name twice and rustc accepts, because cfg eliminates one.
//     Measured: rustc_codegen_ssa/src/back/link.rs defines `escape_linker_output` at 1177 and 1184 under two cfg
//     attributes. Any item carrying an attribute is skipped -- conservative, since this check exists to catch a
//     duplicate nobody intended, not to adjudicate cfg.
//   * an item this parser cannot name renders its ident as `?`, and several such in one file read as duplicates
//     of each other. Ten files reported ``the name `?` is defined multiple times``.
// 28 of rustc's own files were being refused for these two reasons -- a regression my own CL-A6 check introduced.
fn dup_skip(it: &Item) -> bool {
    if it.attrs.len() > 0 { return true; }
    if it.ident.len() == 0 { return true; }
    streq(&it.ident, "?")
}
// SCOPE, narrowed 2026-08-26 after the full-tree probe. `Const`/`Static` are NO LONGER checked: this parser
// invents them. Measured on rustc_interface/src/passes.rs -- three `Item Const ident=path` at byte offsets that
// land INSIDE EXPRESSIONS (`(path, file_len, Some(checksum))`), where no item exists at all. rustc parses that
// file without a single syntax error. The phantom items are a parser-fidelity defect in their own right
// (C-S1-PHANTOM-CONST-ITEM, ledgered in locales/rustc-crate-targeting) and they made a duplicate-name check
// refuse valid source. The check keeps the scope it was BUILT for -- duplicate `fn` and duplicate type names,
// which is what a text-slice patch produces -- and gives up the scope where the input it reasons about is not
// trustworthy.
fn dup_item_name(c: &Krate) -> String {
    let mut vals = String::from(" ");
    let mut tys = String::from(" ");
    let mut dup = String::new();
    for it in c.items.iter() {
        if dup_skip(it) { continue; }
        let nm = format!("{}", it.ident);
        match &it.kind {
            ItemKind::Fn(_g, _p, _r, _b, _a) => { if s9_str_has_word(&vals, &nm) { if dup.len() == 0 { dup = format!("{}", nm); } } else { vals = dup_add(&vals, &nm); } }
            ItemKind::Struct(_g, _f) => { if s9_str_has_word(&tys, &nm) { if dup.len() == 0 { dup = format!("{}", nm); } } else { tys = dup_add(&tys, &nm); } }
            ItemKind::TupleStruct(_g, _f) => { if s9_str_has_word(&tys, &nm) { if dup.len() == 0 { dup = format!("{}", nm); } } else { tys = dup_add(&tys, &nm); } }
            ItemKind::UnitStruct(_g) => { if s9_str_has_word(&tys, &nm) { if dup.len() == 0 { dup = format!("{}", nm); } } else { tys = dup_add(&tys, &nm); } }
            ItemKind::Enum(_g, _v) => { if s9_str_has_word(&tys, &nm) { if dup.len() == 0 { dup = format!("{}", nm); } } else { tys = dup_add(&tys, &nm); } }
            ItemKind::Impl(_g, _tr, _ty, items) => { let d = dup_impl_methods(items); if d.len() > 0 { if dup.len() == 0 { dup = d; } } }
            _ => {}
        }
    }
    dup
}
fn emit_s9(c: &Krate) {
    let d = dup_item_name(c);
    if d.len() > 0 { eprintln!("error[E0428]: the name `{}` is defined multiple times", d); std::process::exit(1); }

    s9_runtime();
    // C-S9-STATIC-ARRAY-UNLOWERED: every `static` got a FIXED 4-byte slot regardless of its declared type, so a
    // `static mut A: [T; N]` had room for one element and `A[2] = v` wrote over the NEXT global -- measured: a
    // static array core-dumps for ANY element type, while static SCALARS work. Sized from the type now, at the
    // 8-byte stride the global accessors already use (s9_global_store emits store8).
    for it in c.items.iter() { match &it.kind { ItemKind::Static(ty, _init) => { println!("gdef {} {}", it.ident, s9_static_bytes(ty)); } _ => {} } }   // C-G2D-STATIC-MUT: a zeroed 4-byte global slot per top-level `static [mut]` (s1_parser's S9NC/S9LBL/S9BV are all i32 `= 0`)
    let base = s9_build_reg(c);
    // C-S9-ASSOC-FN-NAME-MISMATCH: impl fns were ALL emitted under their BARE name. A `&self` method is called
    // through s9_method_generic, which also uses the bare name, so those resolved. But an ASSOCIATED fn (no self)
    // is called through a qualified path -- the call site emits `call S::new` while the definition was `fn new`,
    // so the call dangled and the binary dumped core. Measured: `S::new()`/`S::make(v)` both crash; `s.get()` and
    // a free fn both work. Self-less impl fns are therefore emitted under the QUALIFIED name the call site uses.
    // Emitting them bare as well would collide: several corpus probes define more than one `X::new`.
    for it in c.items.iter() { match &it.kind { ItemKind::Impl(_g, _tr, ty, items) => { let tn = path_base(&hir_ty(ty)); let ienv = s9_env_with_self(&base, &tn); for m in items.iter() { match &m.kind { ItemKind::Fn(_gf, params, _rf, body, _af) => { let mut has_self = false; for pp in params.iter() { if is_self_param(&pp.name) { has_self = true; } } let dupm = s9_method_dup(&base, &m.ident); let nm = if has_self { if dupm { format!("{}::{}", tn, m.ident) } else { format!("{}", m.ident) } } else { format!("{}::{}", tn, m.ident) }; match body { Some(b) => s9_userfn(&nm, params, b, &ienv), None => {} } } _ => {} } } } _ => {} } }
    // C-S9-TRAIT-DEFAULT-BODY: a `trait T { fn m(&self) { .. } }` default body was never emitted at all, so a
    // call to it linked to nothing and returned garbage (measured: a Visitor whose `walk` default calls
    // `visit_ty` printed a heap pointer). Emitted BARE, like impl methods, and ONLY when no impl in the crate
    // supplies that name -- an impl's own body must win, and emitting both would collide under ld's
    // dedup-by-name. That collision is C-S9-BARE-METHOD-COLLISION, ledgered and NOT fixed here.
    for it in c.items.iter() { match &it.kind { ItemKind::Trait(_gt, titems) => { for m in titems.iter() { match &m.kind { ItemKind::Fn(_gf, params, _rf, body, _af) => { match body { Some(b) => { let mk = format!("$m#{}", m.ident); if s9_reg_known(&base, &mk) == false { s9_userfn(&m.ident, params, b, &base); } } None => {} } } _ => {} } } } _ => {} } }
    for it in c.items.iter() { match &it.kind { ItemKind::Fn(_g, _p, _r, body, _a) => { match body { Some(bb) => s9_lift_block(bb, &base), None => {} } } ItemKind::Impl(_g, _tr, _ty, items) => { for m in items.iter() { match &m.kind { ItemKind::Fn(_gf, _pf, _rf, mb, _af) => { match mb { Some(bb) => s9_lift_block(bb, &base), None => {} } } _ => {} } } } ItemKind::Trait(_gt2, titems2) => { for m in titems2.iter() { match &m.kind { ItemKind::Fn(_gf2, _pf2, _rf2, mb2, _af2) => { match mb2 { Some(bb) => s9_lift_block(bb, &base), None => {} } } _ => {} } } } _ => {} } }
    for it in c.items.iter() { match &it.kind { ItemKind::Fn(_g, params, _r, body, _a) => { if streq(&it.ident, "main") == false { match body { Some(b) => s9_userfn(&it.ident, params, b, &base), None => {} } } } _ => {} } }
    for it in c.items.iter() { match &it.kind { ItemKind::Fn(_g, _p, _r, body, _a) => { if streq(&it.ident, "main") { match body { Some(b) => s9_main(b, &base, c), None => {} } } } _ => {} } }
}
fn emit_typed(c: &Krate) {
    let genv = build_global_env(c);
    for it in c.items.iter() { let line = ty_item(it, &genv); if line.len() > 0 { println!("TYPED {}", line); } }
}

// ---- S6 monomorphization collector (`//@S6MONO` mode). Emits the collected mono-item SET == rustc -Zprint-mono-items.
// MONO-2 (TRANSITIVE): each non-generic top-level fn is a ROOT; processing an instance prints its `MONO key` and then
// walks its body WITH the instance's type-args substituted, recursing into each generic call (a worklist-to-fixpoint
// done via depth-capped RECURSION -- no &mut worklist; dupes are uniqed by the instrument, the cap breaks cycles).
// Reuses S5's ty_of/mono_type/base_of_sig/starts_generic/subst_var/nth_param/gparams_names + the &TyEnv idiom.
// The base name of an instance key `name::<args>` (before `::<`); the whole key if not generic.
// R51 refactor: one splitter for BOTH `name::<args>` instance keys AND `W<args>` generic-type strings -- the base is
// the text before the first `<` (minus a trailing `::` on a key); the args are gtype_args (identical for key + gtype).
fn angle_base(s: &str) -> String {
    let b = s.as_bytes(); let mut i = 0; let mut cut = b.len();
    while i < b.len() { if b[i] == 60 { cut = i; i = b.len(); } else { i += 1; } }
    if cut >= 2 { if b[cut-1] == 58 { if b[cut-2] == 58 { cut = cut - 2; } } }
    format!("{}", &s[0..cut])
}
fn strip_angle(s: &str) -> String { let b = s.as_bytes(); if b.len() >= 2 { format!("{}", &s[1..b.len()-1]) } else { String::new() } }
// A generic-type instance string `W<u8>`: has_angle detects it; the base is angle_base, the args are gtype_args (between
// the first `<` and the last `>`). Used to key a generic-type method instance `Base::<args>::method` (MONO-4).
fn has_angle(s: &str) -> bool { let b = s.as_bytes(); let mut i = 0; while i < b.len() { if b[i] == 60 { return true; } i += 1; } false }
fn gtype_args(s: &str) -> String { let b = s.as_bytes(); let mut start = 0; let mut found = false; let mut i = 0; while i < b.len() { if b[i] == 60 { start = i + 1; found = true; i = b.len(); } else { i += 1; } } if !found { return String::new(); } let mut end = start; let mut j = start; while j < b.len() { if b[j] == 62 { end = j; } j += 1; } format!("{}", &s[start..end]) }
// The generic-param NAMES (comma-joined, no brackets) of fn `base`, or "".
fn fn_gpnames(c: &Krate, base: &str) -> String {
    for it in c.items.iter() { match &it.kind { ItemKind::Fn(gp, _p, _r, _b, _a) => { if streq(&it.ident, base) { let gn = gparams_names(gp); return strip_angle(&gn); } }, _ => {} } }
    String::new()
}
// Apply the gparam->arg substitution (both `, `-lists) to a type string (`T` -> `u8`).
fn subst_all(t: &str, gpnames: &str, gpargs: &str) -> String {
    let mut r = format!("{}", t); let mut i = 0;
    while i < 16 { let g = nth_param(gpnames, i); if g.len() == 0 { i = 16; } else { let a = nth_param(gpargs, i); r = subst_var(&r, &g, &a); i += 1; } }
    r
}
// The block env for an instance of fn `base` with type-`args`: the global fn/type env + the fn's params with the
// gparam->arg substitution applied (so a call inside the body types under the instantiation).
fn build_instance_env(c: &Krate, base: &str, args: &str) -> TyEnv {
    let genv = build_global_env(c);
    let mut pn: Vec<String> = veccopy(&genv.names); let mut pt: Vec<String> = veccopy(&genv.types);
    let gpnames = fn_gpnames(c, base);
    for it in c.items.iter() { match &it.kind { ItemKind::Fn(_gp, params, _r, _b, _a) => { if streq(&it.ident, base) {
        for p in params.iter() { if is_self_param(&p.name) { pn.push(String::from("self")); pt.push(format!("{}", p.name)); } else { pn.push(format!("{}", p.name)); pt.push(subst_all(&hir_ty(&p.ty), &gpnames, args)); } }
    } }, _ => {} } }
    TyEnv { names: pn, types: pt }
}
// MONO-5: render one turbofish arg -- a const-generic arg `<3>` is its value; a type arg is hir_ty.
fn render_targ(t: &Ty) -> String {
    match t {
        Ty::ConstArg(_is_block, toks) => { let mut inner = String::new(); let mut first = true; for tk in toks.iter() { if first { inner = format!("{}", tk); first = false; } else { inner = format!("{} {}", inner, tk); } } inner }
        _ => hir_ty(t)
    }
}
// If a callee path carries an EXPLICIT turbofish (`arr::<3>` / `g::<i32, u8>`), return the instance key `name::<args>`;
// else "". The explicit turbofish drives the instance directly (no value-arg inference needed).
fn callee_turbofish(callee: &ExprK) -> String {
    match callee {
        ExprK::Path(p) => {
            let mut name = String::new(); let mut targs = String::new(); let mut has = false;
            for seg in p.segments.iter() { name = format!("{}", seg.ident); if seg.args.len() > 0 { has = true; let mut ts = String::new(); let mut first = true; for t in seg.args.iter() { let one = render_targ(t); if first { ts = one; first = false; } else { ts = format!("{}, {}", ts, one); } } targs = ts; } }
            if has { format!("{}::<{}>", name, targs) } else { String::new() }
        }
        _ => String::new()
    }
}
fn mono_walk_expr(c: &Krate, e: &ExprK, env: &TyEnv, depth: i32) {
    match e {
        ExprK::Call(callee, args) => {
            let tf = callee_turbofish(callee);
            if tf.len() > 0 { mono_instance(c, &tf, depth + 1); }
            else { let csig = ty_of(callee, env, ""); if starts_generic(&csig) { let mono = mono_type(&csig, args, env); let key = base_of_sig(&mono); mono_instance(c, &key, depth + 1); } }
            mono_walk_expr(c, callee, env, depth); for a in args.iter() { mono_walk_expr(c, a, env, depth); }
        }
        ExprK::Method(recv, name, _tys, args) => {
            // MONO-4: a method on a GENERIC-TYPE receiver `W<u8>` collects the instance `W::<u8>::method`.
            let rt = ty_of(recv, env, ""); let base_ty = strip_ref(&rt);
            if has_angle(&base_ty) { let key = format!("{}::<{}>::{}", angle_base(&base_ty), gtype_args(&base_ty), name); mono_instance(c, &key, depth + 1); }
            mono_walk_expr(c, recv, env, depth); for a in args.iter() { mono_walk_expr(c, a, env, depth); }
        }
        ExprK::Bin(_op, a, b) => { mono_walk_expr(c, a, env, depth); mono_walk_expr(c, b, env, depth); }
        ExprK::Unary(_op, a) => mono_walk_expr(c, a, env, depth),
        ExprK::Paren(a) => mono_walk_expr(c, a, env, depth),
        ExprK::Field(recv, _n) => mono_walk_expr(c, recv, env, depth),
        ExprK::Index(a, b) => { mono_walk_expr(c, a, env, depth); mono_walk_expr(c, b, env, depth); }
        ExprK::Ref(_m, a) => mono_walk_expr(c, a, env, depth),
        ExprK::Deref(a) => mono_walk_expr(c, a, env, depth),
        ExprK::Cast(a, _t) => mono_walk_expr(c, a, env, depth),
        ExprK::Assign(a, b) => { mono_walk_expr(c, a, env, depth); mono_walk_expr(c, b, env, depth); }
        ExprK::Ret(v) => { for x in v.iter() { mono_walk_expr(c, x, env, depth); } }
        ExprK::Tuple(items) => { for x in items.iter() { mono_walk_expr(c, x, env, depth); } }
        ExprK::Array(items) => { for x in items.iter() { mono_walk_expr(c, x, env, depth); } }
        ExprK::Block(b) => mono_walk_block(c, b, env, depth),
        ExprK::Unsafe(b) => mono_walk_block(c, b, env, depth),
        ExprK::If(cc, t, els) => { mono_walk_expr(c, cc, env, depth); mono_walk_block(c, t, env, depth); for x in els.iter() { mono_walk_expr(c, x, env, depth); } }
        ExprK::Match(scrut, arms) => { mono_walk_expr(c, scrut, env, depth); for a in arms.iter() { mono_walk_expr(c, &a.body, env, depth); } }
        ExprK::While(_l, cc, b) => { mono_walk_expr(c, cc, env, depth); mono_walk_block(c, b, env, depth); }
        ExprK::Loop(_l, b) => mono_walk_block(c, b, env, depth),
        ExprK::ForLoop(_l, _p, it, b) => { mono_walk_expr(c, it, env, depth); mono_walk_block(c, b, env, depth); }
        _ => {}
    }
}
fn mono_walk_block(c: &Krate, b: &Block, penv: &TyEnv, depth: i32) {
    let mut names: Vec<String> = veccopy(&penv.names); let mut types: Vec<String> = veccopy(&penv.types);
    for st in b.stmts.iter() {
        let env = TyEnv { names: veccopy(&names), types: veccopy(&types) };
        match st {
            Stmt::Let(pat, ty, init, _els) => { match init { Some(e) => mono_walk_expr(c, e, &env, depth), None => {} } let nm = pat_name(pat); if nm.len() > 0 { let exp = match ty { Some(t) => hir_ty(t), None => String::new() }; if exp.len() > 0 { names.push(nm); types.push(exp); } else { match init { Some(e) => { names.push(nm); types.push(ty_of(e, &env, "")); }, None => {} } } } }
            Stmt::Expr(e) => mono_walk_expr(c, e, &env, depth),
            Stmt::Semi(e) => mono_walk_expr(c, e, &env, depth),
            _ => {}
        }
    }
}
// Process one mono instance: print its key, then (depth-capped) walk its substituted body, recursing into its calls.
fn mono_instance(c: &Krate, key: &str, depth: i32) {
    println!("MONO {}", key);
    if depth > 32 { return; }
    let base = angle_base(key); let args = gtype_args(key);
    let env = build_instance_env(c, &base, &args);
    for it in c.items.iter() { match &it.kind { ItemKind::Fn(_gp, _p, _r, body, _a) => { if streq(&it.ident, &base) { match body { Some(b) => mono_walk_block(c, b, &env, depth), None => {} } } }, _ => {} } }
}
fn emit_mono(c: &Krate) {
    for it in c.items.iter() { match &it.kind { ItemKind::Fn(gp, _p, _r, _b, _a) => { if gp.len() == 0 { mono_instance(c, &it.ident, 0); } }, _ => {} } }
}

// C-S7-BOXDEREF-MATCH: an inline `match &**box` is mis-compiled to a runtime crash. The file has called this the
// S9-F12 class for a long time and says so at four other sites -- yet twelve inline instances survived here, and they
// are why s7-check SIGSEGVd under a SEM0-built SEM0. The sound idiom those notes name is the CALL BOUNDARY: passing a
// `&Box<T>` to a fn taking `&T` deref-coerces once, correctly. This helper IS that boundary and nothing else -- the
// identity on `&ExprK`, existing purely so the deref happens at a call rather than inline.
// Measured: 0 of 30 witnesses passed under gen2 before; 16 of 30 after, with 0 wrong verdicts. The remaining 14 crash
// on a DIFFERENT construct -- no box-deref form is left in this region -- and are tracked as C-S7-RESIDUAL-CRASH.
fn s9f12_deref(e: &ExprK) -> &ExprK { e }

// C-S7-PAYLOAD-SCALAR-DEREF: `*x` on a MATCH-ARM PAYLOAD BINDING of scalar type crashes. The binding is a
// reference to a scalar, and emit_s9 represents no references (s9_ref_of is the identity), so `*` falls back to
// load8(value,0) and dereferences an integer. The deref fix of the previous rung recognises `&`-expressions and
// Paths whose RECORDED type is `Ty Ref`; a match payload binding is neither, so it was not reached.
// Same remedy as the box case -- move the deref to a CALL BOUNDARY -- but NOT by passing the payload binding to a
// fn taking `&i64`/`&bool`. That formulation was tried and REVERTED: it fixes emit_s9 (30/30 witnesses) and
// BREAKS L6, which mis-compiles passing a match-payload binding across a call boundary. Measured on L6: a
// `&i32`/`&i64`/`&bool` param deref all MATCH, but `h(x)` for an `E::A(x)` binding SIGSEGVs. The two tiers fail on
// the SAME construct in complementary ways, which is the representation oracle's thesis, confirmed.
// The shape that satisfies BOTH is the one already proven in this file by s9_litexpr_val: deref inside a fn taking
// the ENUM, so no payload binding ever crosses a boundary and no `*` is ever written inline at a use site.
// These three sites are the whole residual -- they feed chk4_* (overflow), chk5_* (index) and chk3_* (borrow),
// exactly the observed crash clustering.
fn s9f12_lit_i64(e: &ExprK) -> i64 { match e { ExprK::LitInt(x) => lit_value(*x), _ => 0 } }
fn s9f12_ref_is_mut(e: &ExprK) -> bool { match e { ExprK::Ref(m, _i) => *m, _ => false } }
// ---- S7 checker tiers (`//@S7CHECK` mode). Emits the ACCEPT/REJECT verdict == rustc. CHK-1 = the E0308 type-check
// tier (conservative -- NEVER false-reject): a fn whose block TAIL is a bound VAR of a known concrete type != the
// declared return type is an E0308 reject (a bound var does not coerce; a literal tail WOULD coerce -> accept). Reuses
// S5's ty_of + the env-thread. Emits VERDICT E0308 if any fn mismatches, else VERDICT ACCEPT.
fn block_tail_var_type(b: &Block, penv: &TyEnv) -> String {
    let mut names: Vec<String> = veccopy(&penv.names); let mut types: Vec<String> = veccopy(&penv.types);
    let n = b.stmts.len(); let mut i = 0; let mut result = String::new();
    for st in b.stmts.iter() {
        let last = i + 1 == n;
        let env = TyEnv { names: veccopy(&names), types: veccopy(&types) };
        match st {
            Stmt::Let(pat, ty, init, _els) => { let nm = pat_name(pat); if nm.len() > 0 { let exp = match ty { Some(t) => hir_ty(t), None => String::new() }; if exp.len() > 0 { names.push(nm); types.push(exp); } else { match init { Some(e) => { names.push(nm); types.push(ty_of(e, &env, "")); }, None => {} } } } }
            Stmt::Expr(e) => { if last { match s9f12_deref(e) { ExprK::Path(_p) => { result = ty_of(e, &env, ""); }, _ => {} } } }
            _ => {}
        }
        i += 1;
    }
    result
}
fn check_fn_e0308(c: &Krate, it: &Item) -> bool {
    match &it.kind {
        ItemKind::Fn(_gp, params, ret, body, _a) => {
            let rt = match ret { Some(t) => hir_ty(t), None => String::new() };
            if rt.len() == 0 { return false; }
            match body {
                Some(b) => {
                    let genv = build_global_env(c);
                    let mut pn: Vec<String> = veccopy(&genv.names); let mut pt: Vec<String> = veccopy(&genv.types);
                    for p in params.iter() { if is_self_param(&p.name) { } else { pn.push(format!("{}", p.name)); pt.push(hir_ty(&p.ty)); } }
                    let penv = TyEnv { names: pn, types: pt };
                    let tt = block_tail_var_type(b, &penv);
                    if tt.len() > 0 { if is_known(&tt) { if !streq(&tt, &rt) { return true; } } }
                    false
                }
                None => false
            }
        }
        _ => false
    }
}
fn vec_contains(v: &Vec<String>, s: &str) -> bool { for x in v.iter() { if streq(x, s) { return true; } } false }
fn is_noncopy_struct(c: &Krate, tyname: &str) -> bool {
    for it in c.items.iter() { match &it.kind {
        ItemKind::UnitStruct(_g) => { if streq(&it.ident, tyname) { return it.attrs.len() == 0; } }
        ItemKind::Struct(_g, _f) => { if streq(&it.ident, tyname) { return it.attrs.len() == 0; } }
        ItemKind::TupleStruct(_g, _t) => { if streq(&it.ident, tyname) { return it.attrs.len() == 0; } }
        _ => {}
    } }
    false
}
fn block_has_moved_reuse(b: &Block, c: &Krate) -> bool {
    let mut moved: Vec<String> = Vec::new();
    let mut lnoncopy: Vec<String> = Vec::new();
    for st in b.stmts.iter() { match st {
        Stmt::Let(pat, _ty, init, _els) => { match init { Some(e) => { match s9f12_deref(e) {
            ExprK::Path(pp) => {
                let rhs = hir_path(pp);
                if vec_contains(&moved, &rhs) { return true; }
                let np = pat_name(pat);
                if vec_contains(&lnoncopy, &rhs) { moved.push(rhs); if np.len() > 0 { lnoncopy.push(np); } }
                else { if is_noncopy_struct(c, &rhs) { if np.len() > 0 { lnoncopy.push(np); } } }
            },
            _ => {} } }, None => {} } },
        _ => {} } }
    false
}
fn check_fn_e0382(c: &Krate, it: &Item) -> bool {
    match &it.kind {
        ItemKind::Fn(_gp, _params, _ret, body, _a) => { match body { Some(b) => block_has_moved_reuse(b, c), None => false } }
        _ => false
    }
}
// CHK-4 (E0080 overflow const-fold): a const/static initialized with a compile-time integer overflow
// (a Bin of two int literals whose result leaves the declared bounded-int type's range) is rejected by rustc
// (error[E0080], "which would overflow"). CONSERVATIVE (Rule-14): only fold a TOP-LEVEL Bin of two LitInt over a
// KNOWN bounded-int type; anything else (unknown type, non-literal operand, wider type) -> ACCEPT (never false-reject).
fn is_bounded_int(t: &str) -> bool {
    if streq(t, "u8") { return true; } if streq(t, "i8") { return true; }
    if streq(t, "u16") { return true; } if streq(t, "i16") { return true; }
    if streq(t, "u32") { return true; } if streq(t, "i32") { return true; }
    false
}
fn int_max(t: &str) -> i64 {
    if streq(t, "u8") { return 255; } if streq(t, "i8") { return 127; }
    if streq(t, "u16") { return 65535; } if streq(t, "i16") { return 32767; }
    if streq(t, "u32") { return 4294967295; } if streq(t, "i32") { return 2147483647; }
    0
}
fn int_min(t: &str) -> i64 {
    if streq(t, "i8") { return -128; } if streq(t, "i16") { return -32768; }
    if streq(t, "i32") { return -2147483648; }
    0
}
fn bin_apply(op: &str, x: i64, y: i64) -> i64 {
    if streq(op, "Add") { return x + y; }
    if streq(op, "Sub") { return x - y; }
    if streq(op, "Mul") { return x * y; }
    x
}
fn const_expr_overflows(e: &ExprK, tname: &str) -> bool {
    if !is_bounded_int(tname) { return false; }
    match e {
        ExprK::Bin(op, a, b) => { match s9f12_deref(a) {
            ExprK::LitInt(x) => { match s9f12_deref(b) {
                ExprK::LitInt(y) => {
                    let v = bin_apply(op, s9f12_lit_i64(s9f12_deref(a)), s9f12_lit_i64(s9f12_deref(b)));
                    if v > int_max(tname) { return true; }
                    if v < int_min(tname) { return true; }
                    false
                },
                _ => false } },
            _ => false } },
        _ => false
    }
}
fn check_const_overflow(it: &Item) -> bool {
    match &it.kind {
        ItemKind::Const(ty, rhs) => { match rhs { Some(e) => { let tn = hir_ty(ty); const_expr_overflows(e, &tn) }, None => false } }
        ItemKind::Static(ty, rhs) => { match rhs { Some(e) => { let tn = hir_ty(ty); const_expr_overflows(e, &tn) }, None => false } }
        _ => false
    }
}
// CHK-5 (E0080 const-index out-of-bounds): a const/static initialized with an ARRAY-LITERAL index whose LitInt index
// is >= the literal's length -> rustc error[E0080] ("index out of bounds"). CONSERVATIVE (Rule-14): only the direct
// `[a, b, ..][k]` literal-base + literal-index shape; a named/other base or non-literal index -> ACCEPT. The length
// count is EXTRACTED into a free fn taking &Vec<ExprK> (finding #8 -- never inline a loop through a destructured field).
fn arr_len_i64(items: &Vec<ExprK>) -> i64 { let mut n: i64 = 0; for _it in items.iter() { n = n + 1; } n }
fn const_index_oob(e: &ExprK) -> bool {
    match e {
        ExprK::Index(recv, idx) => { match s9f12_deref(recv) {
            ExprK::Array(items) => { match s9f12_deref(idx) {
                ExprK::LitInt(_k) => { if s9f12_lit_i64(s9f12_deref(idx)) >= arr_len_i64(items) { return true; } false },
                _ => false } },
            _ => false } },
        _ => false
    }
}
fn check_const_index(it: &Item) -> bool {
    match &it.kind {
        ItemKind::Const(_ty, rhs) => { match rhs { Some(e) => const_index_oob(e), None => false } }
        ItemKind::Static(_ty, rhs) => { match rhs { Some(e) => const_index_oob(e), None => false } }
        _ => false
    }
}
// CHK-3 (E0499/E0502 alias/mut-borrow): a conservative liveness approximation. rustc rejects two borrows of the same
// place when the FIRST is still live at the SECOND (NLL). Provably-safe reject (Rule-14): within one fn block, a borrow
// `A = &mut/& X` at stmt i and a later `B = &mut/& X` (same X) at stmt j>i, NOT both-shared, where A's binding is USED
// at some stmt AFTER j AND A's binding is bound exactly once in the block (no shadowing) -> A is live across B -> real
// E0499 (both &mut) / E0502 (mixed). If A is not used after j, or the name is re-bound, or both are shared -> ACCEPT.
// All walks are extracted free fns (finding #8); the string codec ("bind|var|M/S|idx") + nth_field byte-scan follow the
// proven sig_ret idiom.
fn digit_val(c: u8) -> i64 {
    if c == 48 { return 0; } if c == 49 { return 1; } if c == 50 { return 2; } if c == 51 { return 3; }
    if c == 52 { return 4; } if c == 53 { return 5; } if c == 54 { return 6; } if c == 55 { return 7; }
    if c == 56 { return 8; } if c == 57 { return 9; } 0
}
fn parse_i64(s: &str) -> i64 {
    let by = s.as_bytes(); let mut v: i64 = 0; let mut i: usize = 0;
    while i < by.len() { let c = by[i]; if c >= 48 { if c <= 57 { v = v * 10 + digit_val(c); } } i = i + 1; }
    v
}
fn nth_field(s: &str, k: i64) -> String {
    let by = s.as_bytes(); let mut seg: i64 = 0; let mut start: usize = 0; let mut i: usize = 0;
    while i < by.len() { if by[i] == 124 { if seg == k { return format!("{}", &s[start..i]); } seg = seg + 1; start = i + 1; } i = i + 1; }
    if seg == k { return format!("{}", &s[start..by.len()]); }
    String::new()
}
fn borrow_record(st: &Stmt, idx: i64) -> String {
    match st {
        Stmt::Let(pat, _t, init, _el) => { match init { Some(e) => { match s9f12_deref(e) {
            ExprK::Ref(_m, inner) => { match s9f12_deref(inner) {
                ExprK::Path(pp) => { let bind = pat_name(pat); let var = hir_path(pp); let mk = if s9f12_ref_is_mut(s9f12_deref(e)) { String::from("M") } else { String::from("S") }; format!("{}|{}|{}|{}", bind, var, mk, idx) },
                _ => String::new() } },
            _ => String::new() } }, None => String::new() } },
        _ => String::new()
    }
}
fn collect_borrows(b: &Block) -> Vec<String> {
    let mut out: Vec<String> = Vec::new(); let mut i: i64 = 0;
    for st in b.stmts.iter() { let r = borrow_record(st, i); if r.len() > 0 { out.push(r); } i = i + 1; }
    out
}
fn both_mut(m1: &str, m2: &str) -> bool { if streq(m1, "M") { if streq(m2, "M") { return true; } } false }
fn both_shared(m1: &str, m2: &str) -> bool { if streq(m1, "S") { if streq(m2, "S") { return true; } } false }
// FLATTENED to guard-clauses (finding S7-F2): deep nested if-with-returns mis-lowers under newstage0; single-level
// early-return guards do not.
fn pair_verdict(b: &Block, r1: &str, r2: &str, idx2: i64) -> String {
    let bind1 = nth_field(r1, 0);
    let m1 = nth_field(r1, 2);
    let m2 = nth_field(r2, 2);
    if both_shared(&m1, &m2) { return String::new(); }
    // SINGLE-PASS over b.stmts computing BOTH the shadow count (`cnt` = lets binding bind1) AND the used-after flag
    // (`used` = bind1 used at a stmt after idx2). finding S7-F2: a SECOND walk of the same &Block field from a separate
    // helper (count_lets_named + a bool-returning block_uses_after) mis-lowers under newstage0; folding both into one
    // inline walk with accumulators is correct. CONSERVATIVE (Rule-14): detects only a bare-Path use; a use inside a
    // compound expr is under-detected -> under-reject, never a false-reject.
    let mut used = false; let mut cnt: i64 = 0; let mut i: i64 = 0;
    for st in b.stmts.iter() {
        match st {
            Stmt::Let(pat, _t, init, _el) => {
                if streq(&pat_name(pat), &bind1) { cnt = cnt + 1; }
                if i > idx2 { match init { Some(e) => { match s9f12_deref(e) { ExprK::Path(pp) => { if streq(&hir_path(pp), &bind1) { used = true; } }, _ => {} } }, None => {} } }
            },
            Stmt::Semi(e) => { if i > idx2 { match s9f12_deref(e) { ExprK::Path(pp) => { if streq(&hir_path(pp), &bind1) { used = true; } }, _ => {} } } },
            Stmt::Expr(e) => { if i > idx2 { match s9f12_deref(e) { ExprK::Path(pp) => { if streq(&hir_path(pp), &bind1) { used = true; } }, _ => {} } } },
            _ => {}
        }
        i = i + 1;
    }
    if cnt != 1 { return String::new(); }
    if !used { return String::new(); }
    if both_mut(&m1, &m2) { return String::from("E0499"); }
    String::from("E0502")
}
fn borrow_conflict(b: &Block) -> String {
    let recs = collect_borrows(b);
    for r2 in recs.iter() {
        for r1 in recs.iter() {
            let i1 = parse_i64(&nth_field(r1, 3));
            let i2 = parse_i64(&nth_field(r2, 3));
            if i1 < i2 { if streq(&nth_field(r1, 1), &nth_field(r2, 1)) {
                let v = pair_verdict(b, r1, r2, i2);
                if v.len() > 0 { return v; }
            } }
        }
    }
    String::new()
}
fn check_fn_borrow(it: &Item) -> String {
    match &it.kind {
        ItemKind::Fn(_g, _p, _r, body, _a) => { match body { Some(b) => borrow_conflict(b), None => String::new() } }
        _ => String::new()
    }
}
// CHK-6 (E0425 undefined call -- a name-resolution reject; group-A recurse_undefined_call): a bare single-segment
// LOWERCASE call `x()` whose callee names no crate fn/const/static, no enclosing param/local, and is not the prelude
// `drop`, is undefined -> rustc error[E0425]. CONSERVATIVE (Rule-14): only LOWERCASE callees (CamelCase constructors
// Some/Ok/S(..) are never flagged); SKIP the whole tier if the crate has ANY use/extern-crate/mod (an import could
// define the name -- avoids a false-reject); only STATEMENT-LEVEL calls (a nested call is under-detected -> under-reject).
// All stmt/param walks are EXTRACTED &Block/&Vec-param free fns (finding #8).
fn is_lower_ident(s: &str) -> bool { let by = s.as_bytes(); if by.len() == 0 { return false; } let c = by[0]; if c >= 97 { if c <= 122 { return true; } } false }
fn crate_has_imports(c: &Krate) -> bool {
    for it in c.items.iter() { match &it.kind { ItemKind::Use(_u) => { return true; } ItemKind::ExternCrate => { return true; } ItemKind::Mod(_m) => { return true; } _ => {} } }
    false
}
fn collect_fn_names(c: &Krate) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for it in c.items.iter() { match &it.kind {
        ItemKind::Fn(_g, _p, _r, _b, _a) => { out.push(format!("{}", it.ident)); },
        ItemKind::Const(_t, _e) => { out.push(format!("{}", it.ident)); },
        ItemKind::Static(_t, _e) => { out.push(format!("{}", it.ident)); },
        _ => {}
    } }
    out
}
fn params_names(params: &Vec<Param>) -> Vec<String> { let mut out: Vec<String> = Vec::new(); for p in params.iter() { out.push(format!("{}", p.name)); } out }
fn block_let_names(b: &Block) -> Vec<String> { let mut out: Vec<String> = Vec::new(); for st in b.stmts.iter() { match st { Stmt::Let(pat, _t, _in, _el) => { out.push(pat_name(pat)); }, _ => {} } } out }
fn call_callee(e: &ExprK) -> String {
    match e { ExprK::Call(f, _args) => { match s9f12_deref(f) { ExprK::Path(pp) => hir_path(pp), _ => String::new() } }, _ => String::new() }
}
fn stmt_call_callee(st: &Stmt) -> String {
    match st {
        Stmt::Semi(e) => call_callee(e),
        Stmt::Expr(e) => call_callee(e),
        Stmt::Let(_p, _t, init, _el) => { match init { Some(e) => call_callee(e), None => String::new() } },
        _ => String::new()
    }
}
fn block_has_undefined_call(b: &Block, fnames: &Vec<String>, pnames: &Vec<String>, lnames: &Vec<String>) -> bool {
    for st in b.stmts.iter() {
        let callee = stmt_call_callee(st);
        if callee.len() > 0 { if is_lower_ident(&callee) { if !streq(&callee, "drop") {
            if !vec_contains(fnames, &callee) { if !vec_contains(pnames, &callee) { if !vec_contains(lnames, &callee) { return true; } } }
        } } }
    }
    false
}
fn check_fn_undefined_call(it: &Item, fnames: &Vec<String>) -> bool {
    match &it.kind {
        ItemKind::Fn(_g, params, _r, body, _a) => { match body { Some(b) => { let pnames = params_names(params); let lnames = block_let_names(b); block_has_undefined_call(b, fnames, &pnames, &lnames) }, None => false } }
        _ => false
    }
}
// TYCK-9 coherence (rustc E0119 conflicting implementations): count the trait-impls in the crate whose rendered
// (trait-path, self-ty) header equals (t, s). Inherent impls (no trait) are skipped -- coherence is a per-trait-per-type
// property. Free fn over &Krate (lowers); hir_ty on the &P<Ty> members via Box auto-deref (the proven render idiom).
fn impl_header_count(c: &Krate, t: &str, s: &str) -> i32 {
    let mut n = 0;
    for it in c.items.iter() {
        match &it.kind {
            ItemKind::Impl(_g, tr, sty, _items) => {
                let ts = match tr { Some(x) => hir_ty(x), None => String::new() };
                if ts.len() > 0 { let ss = hir_ty(sty); if streq(&ts, t) && streq(&ss, s) { n = n + 1; } }
            }
            _ => {}
        }
    }
    n
}
// True iff any trait-impl header (trait, self-ty) appears >=2 times -> rustc E0119. Two single-level &Krate loops
// (outer here + inner in impl_header_count) keep each fn small (finding 8: a 2-level loop inlined in one fn segvs rz).
// CONSERVATIVE (Rule-14): fires only on a SYNTACTICALLY identical duplicate header, never on a merely-overlapping one,
// so it can never false-reject a valid crate; deeper overlap (blanket/generic-arg unification) is a later rung.
fn check_coherence_overlap(c: &Krate) -> bool {
    for it in c.items.iter() {
        match &it.kind {
            ItemKind::Impl(_g, tr, sty, _items) => {
                match tr { Some(x) => { let ts = hir_ty(x); let ss = hir_ty(sty); if impl_header_count(c, &ts, &ss) >= 2 { return true; } } None => {} }
            }
            _ => {}
        }
    }
    false
}
// The base identifier of a rendered type/trait path: drop the generic args (before the first `<`) and any path
// qualifier (after the last `::`). "core::clone::Clone" -> "Clone"; "Vec<i32>" -> "Vec"; "i32" -> "i32". &str byte-scan.
fn path_base(s: &str) -> String {
    let b = s.as_bytes(); let n = b.len();
    let mut end = n; let mut i = 0; while i < n { if b[i] == 60 { end = i; i = n; } i = i + 1; }
    let mut start = 0; let mut j = 0; while j + 1 < end { if b[j] == 58 && b[j + 1] == 58 { start = j + 2; } j = j + 1; }
    format!("{}", &s[start..end])
}
// A built-in primitive scalar type name -- ALWAYS foreign to a user crate and never #[fundamental], so an impl of a
// foreign trait for it is unconditionally orphan-violating (the safe conservative firing set for E0117).
fn is_primitive_ty(name: &str) -> bool {
    if streq(name, "i8") || streq(name, "i16") || streq(name, "i32") || streq(name, "i64") || streq(name, "i128") || streq(name, "isize") { return true; }
    if streq(name, "u8") || streq(name, "u16") || streq(name, "u32") || streq(name, "u64") || streq(name, "u128") || streq(name, "usize") { return true; }
    if streq(name, "f32") || streq(name, "f64") || streq(name, "bool") || streq(name, "char") || streq(name, "str") { return true; }
    false
}
// True iff `name` names a TRAIT declared in this crate (a local trait). Free fn over &Krate.
fn is_local_trait(c: &Krate, name: &str) -> bool {
    for it in c.items.iter() {
        if streq(&it.ident, name) { match &it.kind { ItemKind::Trait(_g, _m) => return true, ItemKind::TraitAlias(_g, _b) => return true, ItemKind::AutoTrait(_g, _m) => return true, _ => {} } }
    }
    false
}
// Orphan rule (rustc E0117): a trait-impl `impl ForeignTrait for <primitive>` -- the trait is NOT locally declared AND
// the self-ty base is a primitive. CONSERVATIVE (Rule-14): a LOCAL trait, or any non-primitive self-ty (which may be a
// local or #[fundamental]-wrapped-local type), does NOT fire -> never a false-reject; broader foreign self-types are a
// later rung. Single-level &Krate loop (helpers do their own single loops -- finding 8).
fn check_orphan(c: &Krate) -> bool {
    for it in c.items.iter() {
        match &it.kind {
            ItemKind::Impl(_g, tr, sty, _items) => {
                match tr { Some(x) => { let tb = path_base(&hir_ty(x)); let sb = path_base(&hir_ty(sty)); if !is_local_trait(c, &tb) { if is_primitive_ty(&sb) { return true; } } } None => {} }
            }
            _ => {}
        }
    }
    false
}
fn emit_check(c: &Krate) {
    for it in c.items.iter() { if check_const_overflow(it) { println!("VERDICT E0080"); return; } }
    for it in c.items.iter() { if check_const_index(it) { println!("VERDICT E0080"); return; } }
    if !crate_has_imports(c) { let fnames = collect_fn_names(c); for it in c.items.iter() { if check_fn_undefined_call(it, &fnames) { println!("VERDICT E0425"); return; } } }
    // Coherence (E0119 overlap / E0117 orphan) is a pre-typeck phase in rustc -> report it before the body verdicts.
    if check_coherence_overlap(c) { println!("VERDICT E0119"); return; }
    if check_orphan(c) { println!("VERDICT E0117"); return; }
    for it in c.items.iter() { if check_fn_e0308(c, it) { println!("VERDICT E0308"); return; } }
    for it in c.items.iter() { if check_fn_e0382(c, it) { println!("VERDICT E0382"); return; } }
    for it in c.items.iter() { let bv = check_fn_borrow(it); if streq(&bv, "E0499") { println!("VERDICT E0499"); return; } if streq(&bv, "E0502") { println!("VERDICT E0502"); return; } }
    println!("VERDICT ACCEPT");
}

// S3 RES-1 (visibility): a `use MOD::NAME` where MOD is an inline module and NAME is a NON-`pub` item of MOD is a
// privacy violation (rustc E0603 "`NAME` is private"). Emit `PRIVUSE NAME` for each such use. Free fn taking &Krate
// (the proven dump idiom). Simple 2-segment `mod::name` uses only (nested/glob use trees are later rungs).
// If the joined use path `pathstr` is `MOD::NAME` (single `::`) and NAME is a NON-`pub` item of the inline module MOD
// (a privacy violation, rustc E0603), return NAME; else "". Splits the &str PARAM (proven), NOT a Vec-indexed clone
// (`segments[i].ident.clone()` through the Box mis-lowered to garbage under rzrs).
// Descend `rest` (a `::`-joined remainder path) into module `mi`, returning the FIRST segment that crosses a private
// boundary (rustc E0603), or "". Each step looks up the head segment among mi's inline items: a private item (vis
// "priv") is the violation (pub/pub(crate)/pub(super) resolve); a pub intermediate module is descended into. Recurses
// over &Item + the &str PARAM `rest` (both lower); splits `rest` on the leading `::` via &str-param byte ops.
fn check_from(mi: &Item, rest: &str) {
    let rb = rest.as_bytes();
    let n = rb.len();
    let mut ci = 0;
    let mut has = false;
    let mut i = 0;
    while i + 1 < n { if rb[i] == 58 && rb[i + 1] == 58 { ci = i; has = true; i = n; } i = i + 1; }
    let head = if has { &rest[0..ci] } else { rest };
    match &mi.kind {
        ItemKind::Mod(inner) => {
            for inr in inner.iter() {
                if streq(&inr.ident, head) {
                    if streq(&inr.vis, "priv") { println!("PRIVUSE {}", head); return; }   // E0603
                    if has { let tail = &rest[ci + 2..n]; check_from(inr, tail); return; }
                    return;   // found + visible -> resolves ok
                }
            }
            println!("NOUSE {}", head);   // no `head` in this (resolved) module -> rustc E0432 unresolved import
        }
        _ => {}
    }
}
// N-segment private-use resolution for `pathstr` = a::b::...::name from a crate-root use site. The FIRST segment names
// a crate-root item (nameable from the crate root even if private); every deeper segment must be pub/pub(crate)/
// pub(super) to be named from outside its module. Returns the first private segment (rustc E0603), or "".
fn resolve_use_path(k: &Krate, pathstr: &str) {
    let ab = pathstr.as_bytes();
    let an = ab.len();
    // Strip a leading `crate::` / `self::` -- both denote the crate root in this single-file context, so the walk
    // proceeds from the crate-root items. Byte-compare the prefix (raw bytes; the &str-param slice is proven).
    let mut start = 0;
    if an >= 7 && ab[0] == 99 && ab[1] == 114 && ab[2] == 97 && ab[3] == 116 && ab[4] == 101 && ab[5] == 58 && ab[6] == 58 { start = 7; }
    else if an >= 6 && ab[0] == 115 && ab[1] == 101 && ab[2] == 108 && ab[3] == 102 && ab[4] == 58 && ab[5] == 58 { start = 6; }
    let p = &pathstr[start..an];
    let pb = p.as_bytes();
    let n = pb.len();
    let mut ci = 0;
    let mut found = false;
    let mut i = 0;
    while i + 1 < n { if pb[i] == 58 && pb[i + 1] == 58 { ci = i; found = true; i = n; } i = i + 1; }
    if !found { return; }
    let head = &p[0..ci];
    let rest = &p[ci + 2..n];
    // The first segment names a crate-root item; if it is not an inline item (e.g. an external crate like `std`),
    // do NOT flag it -- only descend existence/privacy checks into modules we can actually see.
    for m in k.items.iter() {
        if streq(&m.ident, head) { check_from(m, rest); return; }
    }
}
// Walk a use tree (recursing into `Nested`), accumulating the module prefix, and emit `PRIVUSE NAME` for each leaf
// that names a private item. `base` is the accumulated prefix ("" at the top); a Simple leaf's full path is
// base::prefix. &UseTree + &str params (the dump idiom); recursion mirrors dump_item.
fn emit_privuse_tree(k: &Krate, base: &str, ut: &UseTree) {
    let pp = dump_path(&ut.prefix);
    let full = if base.len() == 0 { pp } else { format!("{}::{}", base, pp) };
    match &ut.kind {
        UseKind::Simple(_r) => { resolve_use_path(k, &full); }
        UseKind::Nested(kids) => { for c in kids.iter() { emit_privuse_tree(k, &full, c); } }
        _ => {}
    }
}
// Print a `DUPUSE` line. The name arrives as a &str PARAM (proven to Display); formatting a Vec-indexed String value
// -- even `&names[i]` -- mis-lowered to a raw pointer under rzrs.
fn print_dup(nm: &str) { println!("DUPUSE {}", nm); }
// The last `::`-segment of a joined path (`a::b::f` -> `f`). Slices the &str PARAM (proven).
fn last_seg(p: &str) -> String {
    let pb = p.as_bytes();
    let n = pb.len();
    let mut li = 0;
    let mut i = 0;
    while i + 1 < n { if pb[i] == 58 && pb[i + 1] == 58 { li = i + 2; } i = i + 1; }
    let s = &p[li..n];
    s.to_string()
}
// The imported names of a use tree, space-joined: a Simple leaf imports its rename (if any) else the last path
// segment; a Nested node concatenates its children; a Glob imports no single name. Recurses over &UseTree.
fn use_names(ut: &UseTree) -> String {
    match &ut.kind {
        UseKind::Simple(rename) => {
            let r = format!("{}", rename);
            if r.len() > 0 { r } else { let pp = dump_path(&ut.prefix); last_seg(&pp) }
        }
        UseKind::Nested(kids) => {
            let mut s = String::new();
            for c in kids.iter() { let cn = use_names(c); if cn.len() > 0 { if s.len() > 0 { s.push(' '); } s.push_str(&cn); } }
            s
        }
        _ => String::from(""),
    }
}
// S3 RES-7: emit `DUPUSE name` for each name imported by two-or-more use leaves (rustc E0252). Collects all imported
// names into one space-joined string (no &mut Vec param), then flags each name whose first occurrence sees count>=2.
fn emit_dupuse(k: &Krate) {
    let mut all = String::new();
    for it in k.items.iter() {
        match &it.kind {
            ItemKind::Use(ut) => { let ns = use_names(ut); if ns.len() > 0 { if all.len() > 0 { all.push(' '); } all.push_str(&ns); } }
            _ => {}
        }
    }
    let names = split_ws(&all);
    let m = names.len();
    let mut i = 0;
    while i < m {
        let mut cnt = 0;
        let mut firstidx = 0;
        let mut ff = true;
        let mut j = 0;
        while j < m { if streq(&names[j], &names[i]) { cnt = cnt + 1; if ff { firstidx = j; ff = false; } } j = j + 1; }
        if cnt >= 2 && firstidx == i { print_dup(&names[i]); }
        i = i + 1;
    }
}
fn emit_privuse(k: &Krate) {
    for it in k.items.iter() {
        match &it.kind {
            ItemKind::Use(ut) => { emit_privuse_tree(k, "", ut); }
            _ => {}
        }
    }
}

// S3 RES-0: emit one `RESPATH <path>` line per crate-owned def (name-resolution differential vs rustc hir-tree
// def-paths). A named top-level item -> `RESPATH NAME`; an inline `mod M { .. }` -> `RESPATH M` + `RESPATH M::INNER`
// per inner named item; a `use` -> `RESPATH {use#K}` (K counts SOURCE uses 1-based; rustc reserves {use#0} for the
// injected prelude). Takes &Krate (the proven dump idiom); iterates via .iter().
// ---- S3 CRATE DRIVER (`//@CRATE` mode). Ingest a DELIMITED STDIN IMAGE (//@CRATE + //@UNIT crate=/path=/kind=dep/root=
// boundaries) and emit the cross-crate resolution map (`CRATERES <crate>|<defpath>`). CD-1: the module/unit inventory --
// parse each //@UNIT body (with resolve:false, so `mod a;` is an empty decl + `extern crate` is plain; the module tree
// comes from the //@UNIT structure, NOT a disk file-read) and emit each item's def-path prefixed by its crate + module
// base (root unit -> base ""; a `path=a.rs` mod unit -> base "a"). All string walks are &str-param byte-scans (finding
// #6/7); the def-path recursion mirrors the proven emit_item_defpath (finding-#8-safe).
fn str_match_at(hay: &str, i: usize, needle: &str) -> bool {
    let hb = hay.as_bytes(); let nb = needle.as_bytes();
    let mut j: usize = 0;
    while j < nb.len() { if i + j >= hb.len() { return false; } if hb[i+j] != nb[j] { return false; } j = j + 1; }
    true
}
fn split_lines(s: &str) -> Vec<String> {
    let by = s.as_bytes(); let mut out: Vec<String> = Vec::new(); let mut start: usize = 0; let mut i: usize = 0;
    while i < by.len() { if by[i] == 10 { out.push(format!("{}", &s[start..i])); start = i + 1; } i = i + 1; }
    if start < by.len() { out.push(format!("{}", &s[start..by.len()])); }
    out
}
fn is_unit_header(ln: &str) -> bool { str_match_at(ln, 0, "//@UNIT") }
// extract `key<value>` (key includes the trailing `=`), value read up to the next space or end. "" if absent.
fn unit_attr(ln: &str, key: &str) -> String {
    let hb = ln.as_bytes(); let kl = key.len();
    let mut i: usize = 0;
    while i + kl <= hb.len() {
        if str_match_at(ln, i, key) {
            let mut e = i + kl;
            while e < hb.len() { if hb[e] == 32 { break; } e = e + 1; }
            return format!("{}", &ln[i+kl..e]);
        }
        i = i + 1;
    }
    String::new()
}
fn mod_base_from_path(path: &str, is_root: bool) -> String {
    if is_root { return String::new(); }
    let n = path.len();
    if n > 3 { return format!("{}", &path[0..n-3]); }
    format!("{}", path)
}
// CD-10: conservative cfg-eval. In a plain rustc build (no --cfg features, host target, not test), an item gated by
// `#[cfg(feature = "..")]` or `#[cfg(test)]` is DROPPED. CONSERVATIVE (Rule-14): only the clearly-off cases -- a feature
// cfg (NO features are enabled by default) or test; `not(..)`, target cfgs, all()/any() are KEPT (unsure -> keep).
fn attr_is_cfg_off(a: &Attr) -> bool {
    if !streq(&dump_path(&a.path), "cfg") { return false; }
    if str_match_at(&a.args, 0, "feature") { return true; }
    if streq(&a.args, "test") { return true; }
    if streq(&a.args, "any()") { return true; }   // `#[cfg(any())]` is VACUOUSLY FALSE -> rustc always drops the item (host-independent, like MAC-8). WITHOUT this the driver emits the cfg'd-off item -> a spurious owner def-path AND (for an impl) a spurious {impl#N} that drifts subsequent impl numbering.
    if streq(&a.args, "windows") { return true; }   // CD-21: the default rustc target is non-windows (linux oracle), so `#[cfg(windows)]` is OFF -- a universal fact for the target (like the atomic-128 skip). `not(windows)`/`unix`/`any(unix,..)` are NOT matched (kept). Drops the windows arm of target-cfg duplicate fns (rustc_fs_util fix_windows_verbatim_for_gcc / path_to_c_string) + their body uses.
    if str_match_at(&a.args, 0, "all") { if str_contains(&a.args, "wasi") { return true; } }   // CD-21: a TOP-LEVEL `#[cfg(all(target_os="wasi",..))]` is OFF (wasi is never the linux target; rustc_fs_util path_to_c_string's wasi-gated body use). Gated on a leading `all` so an `any(unix, all(..wasi..))` (unix-true, KEPT) is NOT matched.
    false
}
fn attrs_cfg_off(attrs: &Vec<Attr>) -> bool {
    for a in attrs.iter() { if attr_is_cfg_off(a) { return true; } }
    false
}
// Any `#[cfg(..)]` present (regardless of true/false). The fn-scope disambiguation pass EXCLUDES cfg-attributed items:
// host-dependent cfg (target_os/unix/..) the driver cannot evaluate may gate same-named items into mutually-exclusive
// branches (rustc keeps one -> no `#k`; the driver would count both -> a false `NAME#1`). Conservative: skip cfg items.
fn attrs_have_cfg(attrs: &Vec<Attr>) -> bool {
    for a in attrs.iter() { if streq(&dump_path(&a.path), "cfg") { return true; } }
    false
}
// CD-11: a `macro_rules! NAME { .. }` DEFINITION is ItemKind::MacCall wrapping ExprK::MacCall(path=NAME, "MacroDefBrace",
// toks) with an empty item ident. rustc emits it as a crate def-path (NAME). A macro CALL in item position has a different
// delim sentinel -> emits nothing. Returns NAME iff the MacroDefBrace sentinel is present.
// S2-MACRO first slice: does a registered macro's arm BODY have the `$( impl PATH for $T { members } )+` shape (the
// ast_traits impl_has_*!/impl_not_ast_deref! family)? Strict: body's first tokens are `$ ( impl` -> the repetition
// generates ONE impl per matched $T. Everything else (language_item_table!, expect_*, structs/enums) is NOT matched, so
// no false positive -> no numbering regression. body is the space-joined arm-body tokens.
fn mac_body_is_impl_rep(body: &str) -> bool {
    // A `$( .. )` repetition body whose generated item is an `impl` -- optionally preceded by attribute groups
    // (`#[stable(..)]` etc; ubiquitous in std, e.g. core hash `impl_write!`). Require leading `$ (`, then skip
    // whitespace + `# [ .. ]` attr groups, then expect `impl `. Byte-scan a &str param.
    if !str_match_at(body, 0, "$ (") { return false; }
    let bb = body.as_bytes(); let n = bb.len();
    let mut i: usize = 3;   // past "$ ("
    loop {
        while i < n { if bb[i] != 32 { break; } i = i + 1; }
        if i < n { if bb[i] == 35 || bb[i] == 63 {   // '#' (rendered '?' via the token-render fallback) -> an attribute; skip it then the `[ .. ]` group
            i = i + 1;
            while i < n { if bb[i] == 91 { break; } i = i + 1; }   // to '['
            let mut depth: i64 = 0;
            while i < n { if bb[i] == 91 { depth = depth + 1; } if bb[i] == 93 { depth = depth - 1; if depth == 0 { i = i + 1; break; } } i = i + 1; }
        } else { break; } } else { break; }
    }
    str_match_at(body, i, "impl ")
}
// EMIT `{ip}::member` for each fn/const associated item of the impl in an impl-rep macro body, by BYTE-SCANNING body and
// printing INLINE (rzrs #6/7: a Vec<String> of names printed via .iter() mis-lowers each String to a raw pointer; a LOCAL
// String printed immediately is fine). Track `{`/`}` depth; at impl-body depth == 1 a `fn`/`const` word names the member
// (the following word, up to a space or `(`). `body` must be a LOCAL &str (Vec-index-derived body byte-scans mis-lower).
fn mac_body_gen_struct(body: &str) -> bool {
    // word-iteration; single-var indexing ONLY (bb[i]/bb[j], never bb[i+k] offset -- offset indexing mis-lowers under
    // newstage0, the B1 root cause). A generating macro has a `struct` word followed by a `$`-metavar word.
    let bb = body.as_bytes(); let n = bb.len(); let mut i: usize = 0; let mut prev_struct = false;
    while i < n {
        let c = bb[i];
        if c == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if prev_struct { let d = bb[i]; if d == 36 { return true; } }
            let w = &body[i..j]; prev_struct = streq(w, "struct");
            i = j;
        }
    }
    false
}
fn mac_typealias_meta(body: &str) -> String {
    // a `pub type $ META = ..` -- the macro generates a type alias named by the metavar META. Byte-scan (large body).
    let bb = body.as_bytes(); let n = bb.len(); let mut i: usize = 0; let mut want_dollar = false; let mut want_meta = false;
    while i < n {
        let c = bb[i];
        if c == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if want_meta { let mut nm = String::new(); let mut q = i; while q < j { nm.push(bb[q] as char); q = q + 1; } return nm; }
            let w = &body[i..j];
            if want_dollar && streq(w, "$") { want_meta = true; want_dollar = false; }
            else { want_dollar = streq(w, "type"); want_meta = false; }
            i = j;
        }
    }
    String::from("")
}
fn anchor_stop(w: &str) -> bool {
    streq(w, "[") || streq(w, "]") || streq(w, "{") || streq(w, "}") || streq(w, "(") || streq(w, ")") || streq(w, "$") || streq(w, ":") || streq(w, ",")
}
fn metavar_prefix(matcher: &str, meta: &str) -> String {
    // find the matcher comma-part binding `$ META` and return the LITERAL anchor immediately before it (scan back,
    // collecting simple tokens, stopping at ]/[/{/}/(/)/$/:/,). E.g. `#[$stability:meta] Self = $ Ty : ident` -> "Self =".
    let parts = split_commas(matcher);
    let mut pi: usize = 0;
    while pi < parts.len() {
        let tk = split_ws(&parts[pi]);
        let mut j: usize = 0;
        while j + 1 < tk.len() {
            if streq(&tk[j], "$") { if streq(&tk[j+1], meta) {
                // collect literal tokens immediately before j (backwards), stop at a boundary
                let mut lo = j;
                while lo > 0 { if anchor_stop(&tk[lo-1]) { break; } lo = lo - 1; }
                let mut pre = String::new(); let mut q: usize = lo;
                while q < j { if pre.len() > 0 { pre.push(' '); } pre.push_str(&tk[q]); q = q + 1; }
                return pre;
            } }
            j = j + 1;
        }
        pi = pi + 1;
    }
    String::from("")
}
fn emit_typealias_names(cratename: &str, base: &str, inv: &str, prefix: &str) {
    // for each invocation comma-part starting with PREFIX (e.g. `Self = NonZeroU8`), emit the token AFTER the prefix.
    if prefix.len() == 0 { return; }
    let pretk = split_ws(prefix); let np = pretk.len(); if np == 0 { return; }
    let parts = split_commas(inv);
    let mut pi: usize = 0;
    while pi < parts.len() {
        let tk = split_ws(&parts[pi]);
        if tk.len() > np {
            let mut ok = true; let mut q: usize = 0;
            while q < np { if !streq(&tk[q], &pretk[q]) { ok = false; } q = q + 1; }
            if ok { let mut val = String::new(); val.push_str(&tk[np]);
                let full = if base.len() == 0 { format!("{}", val) } else { format!("{}::{}", base, val) };
                println!("CRATERES {}|{}", cratename, full); }
        }
        pi = pi + 1;
    }
}
fn mac_body_gen_enum(body: &str) -> bool {
    // a `enum` word followed by a literal (non-$) ident -> the macro generates a named enum (language_item_table! -> LangItem).
    let bb = body.as_bytes(); let n = bb.len(); let mut i: usize = 0; let mut prev_enum = false;
    while i < n {
        let c = bb[i];
        if c == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if prev_enum { let d = bb[i]; if d != 36 { return true; } }
            let w = &body[i..j]; prev_enum = streq(w, "enum");
            i = j;
        }
    }
    false
}
fn emit_mac_enum_names(cratename: &str, base: &str, body: &str) {
    let bb = body.as_bytes(); let n = bb.len(); let mut i: usize = 0; let mut prev_enum = false;
    while i < n {
        let c = bb[i];
        if c == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if prev_enum { let d = bb[i]; if d != 36 { let mut nm = String::new(); let mut q = i; while q < j { nm.push(bb[q] as char); q = q + 1; }
                if !struct_name_reserved(&nm) { let full = if base.len() == 0 { format!("{}", nm) } else { format!("{}::{}", base, nm) }; println!("CRATERES {}|{}", cratename, full); } } }
            let w = &body[i..j]; prev_enum = streq(w, "enum");
            i = j;
        }
    }
}
fn struct_name_reserved(nm: &str) -> bool {
    // words that can follow `struct` in a macro invocation but are NOT a struct name (keywords / non-def positions).
    streq(nm, "self") || streq(nm, "Self") || streq(nm, "super") || streq(nm, "crate") || streq(nm, "mut") || streq(nm, "dyn") || streq(nm, "ref") || streq(nm, "move") || streq(nm, "impl") || streq(nm, "where") || streq(nm, "as")
}
fn emit_mac_struct_names2(cratename: &str, base: &str, inv: &str) {
    // scan the space-joined INVOCATION for the token AFTER `struct` (that is the struct name; tokens are already
    // space-separated so `P1 ( u32 )` yields `P1` cleanly). Split on SPACE ONLY (mirrors emit_impl_members) -- breaking
    // the word loop on `(`/`;`/etc caused a no-advance infinite loop (segfault) when i pointed AT such a char.
    let bb = inv.as_bytes(); let n = bb.len(); let mut i: usize = 0; let mut prev_struct = false;
    while i < n {
        let c = bb[i];
        if c == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if prev_struct { let mut nm = String::new(); let mut q = i; while q < j { nm.push(bb[q] as char); q = q + 1; }
                if !struct_name_reserved(&nm) {
                    let full = if base.len() == 0 { format!("{}", nm) } else { format!("{}::{}", base, nm) };
                    println!("CRATERES {}|{}", cratename, full); } }
            let w = &inv[i..j]; prev_struct = streq(w, "struct");
            i = j;
        }
    }
}
// The metavars naming structs GENERATED by a macro whose body has `pub struct $META(..)` (the struct name is a metavar,
// bound POSITIONALLY from the invocation -- e.g. `typed_def_id! { ModDefId, LocalModDefId }` body `pub struct $Name(DefId);
// pub struct $LocalName(LocalDefId);`). Distinct from emit_mac_struct_names2 (the struct keyword is in the INVOCATION).
// Body tokens render `$Name` as `$` `Name`; collect the word after each `struct $`. Free fn -> Vec<String> (lowers).
fn body_struct_metavars(body: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let bb = body.as_bytes(); let n = bb.len(); let mut i: usize = 0; let mut state: i64 = 0;
    while i < n {
        if bb[i] == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if state == 2 { let mut nm = String::new(); let mut q = i; while q < j { nm.push(bb[q] as char); q = q + 1; } out.push(nm); state = 0; }
            else { if state == 1 { if str_match_at(body, i, "$") { if j == i + 1 { state = 2; } else { state = 0; } } else { state = 0; } }
            else { if str_match_at(body, i, "struct") { if j == i + 6 { state = 1; } else { state = 0; } } } }
            i = j;
        }
    }
    out
}
// hay[k..e] equals needle (a whole-token compare on a &str param).
fn range_eq(hay: &str, k: usize, e: usize, needle: &str) -> bool {
    let nb = needle.as_bytes(); if e < k { return false; } if e - k != nb.len() { return false; }
    str_match_at(hay, k, needle)
}
// The comma-position of metavar `meta` in a NON-repetition matcher (`$ A : ident , $ B : ident` -> A=0, B=1); -1 if absent.
// Direct &str-param byte-scan (no split_commas / Vec-iter byte-scan -- those mis-lower).
fn matcher_metavar_index(matcher: &str, meta: &str) -> i64 {
    let bb = matcher.as_bytes(); let n = bb.len(); let mut i: usize = 0; let mut group: i64 = 0; let mut got: bool = false;
    while i < n {
        let c = bb[i];
        if c == 44 { group = group + 1; got = false; i = i + 1; }
        else { if c == 36 { if !got { got = true;
            let mut k = i + 1; while k < n { if bb[k] == 32 { k = k + 1; } else { break; } }
            let mut e = k; while e < n { if bb[e] == 32 { break; } if bb[e] == 58 { break; } e = e + 1; }
            if range_eq(matcher, k, e, meta) { return group; }
            i = e;
        } else { i = i + 1; } } else { i = i + 1; } }
    }
    -1
}
// The first ident of the idx-th comma-group of an invocation. Direct &str-param byte-scan.
fn nth_group_first_ident(inv: &str, idx: i64) -> String {
    let bb = inv.as_bytes(); let n = bb.len(); let mut i: usize = 0; let mut group: i64 = 0;
    while i < n {
        let c = bb[i];
        if c == 44 { group = group + 1; i = i + 1; }
        else { if c == 32 { i = i + 1; }
        else { if group == idx {
            let mut e = i; while e < n { if bb[e] == 32 { break; } if bb[e] == 44 { break; } e = e + 1; }
            let mut nm = String::new(); let mut q = i; while q < e { nm.push(bb[q] as char); q = q + 1; } return nm;
        } else { i = i + 1; } } }
    }
    String::new()
}
// Emit the positionally-bound generated-struct NAMES of a NON-repetition struct-gen macro (body `struct $META`, matcher
// `$A:ident, $B:ident`, invocation bare idents). Gated on a non-rep matcher (no `$ (`) so it never fires on the
// repetition struct-gen macros emit_mac_struct_names2 handles (rung-10 atomic fragility was repetition + host-cfg).
fn emit_positional_struct_names(cratename: &str, base: &str, inv: &str, matcher: &str, body: &str) {
    if str_contains(matcher, "$ (") { return; }
    let metas = body_struct_metavars(body);
    for meta in metas.iter() {
        let idx = matcher_metavar_index(matcher, meta);
        if idx >= 0 {
            let nm = nth_group_first_ident(inv, idx);
            if nm.len() > 0 { if !struct_name_reserved(&nm) {
                let full = if base.len() == 0 { format!("{}", nm) } else { format!("{}::{}", base, nm) };
                println!("CRATERES {}|{}", cratename, full);
            } }
        }
    }
}
// CD-16: a module-level fn-generating macro -- body has a `fn` word followed by a `$`-metavar word (`pub fn $fn_name(..)`,
// rustc_serialize leb128 impl_*_leb128!). Mirror of mac_body_gen_struct (single-var byte-scan, B1-safe).
fn mac_body_gen_fn(body: &str) -> bool {
    let bb = body.as_bytes(); let n = bb.len(); let mut i: usize = 0; let mut prev_fn = false;
    while i < n {
        let c = bb[i];
        if c == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if prev_fn { let d = bb[i]; if d == 36 { return true; } }
            let w = &body[i..j]; prev_fn = streq(w, "fn");
            i = j;
        }
    }
    false
}
// CD-16: the metavar NAMES bound as a generated fn name (`fn $ META` -> META). Mirror of body_struct_metavars, keyed on `fn`.
fn body_fn_metavars(body: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let bb = body.as_bytes(); let n = bb.len(); let mut i: usize = 0; let mut state: i64 = 0;
    while i < n {
        if bb[i] == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if state == 2 { let mut nm = String::new(); let mut q = i; while q < j { nm.push(bb[q] as char); q = q + 1; } out.push(nm); state = 0; }
            else { if state == 1 { if str_match_at(body, i, "$") { if j == i + 1 { state = 2; } else { state = 0; } } else { state = 0; } }
            else { if str_match_at(body, i, "fn") { if j == i + 2 { state = 1; } else { state = 0; } } } }
            i = j;
        }
    }
    out
}
// CD-16: emit the positionally-bound generated-FN names of a non-repetition fn-gen macro (body `fn $META`, matcher
// `$fn:ident, $ty:ty`, invocation bare idents -> `<base>::<inv ident at meta's comma-position>`). Mirror of
// emit_positional_struct_names; reuses matcher_metavar_index + nth_group_first_ident + struct_name_reserved.
fn emit_positional_fn_names(cratename: &str, base: &str, inv: &str, matcher: &str, body: &str) {
    if str_contains(matcher, "$ (") { return; }
    let metas = body_fn_metavars(body);
    for meta in metas.iter() {
        let idx = matcher_metavar_index(matcher, meta);
        if idx >= 0 {
            let nm = nth_group_first_ident(inv, idx);
            if nm.len() > 0 { if !struct_name_reserved(&nm) {
                let full = if base.len() == 0 { format!("{}", nm) } else { format!("{}::{}", base, nm) };
                println!("CRATERES {}|{}", cratename, full);
            } }
        }
    }
}
// The name in `mod NAME {` inside a macro body (the macro generates a nested module). "" if none. Direct &str-param scan.
fn body_mod_name(body: &str) -> String {
    let bb = body.as_bytes(); let n = bb.len(); let mut i: usize = 0; let mut want: bool = false;
    while i < n {
        if bb[i] == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if want { let mut nm = String::new(); let mut q = i; while q < j { nm.push(bb[q] as char); q = q + 1; } return nm; }
            if str_match_at(body, i, "mod") { if j == i + 3 { want = true; } }
            i = j;
        }
    }
    String::new()
}
// FIXED (non-metavar) type/value owner names declared in a macro body (each item keyword's following word, when it is not a
// `$`-metavar). Used to emit a macro-generated mod's fixed interior items (nonzero `mod private { pub trait Sealed {} .. }`
// -> private::Sealed). Excludes `mod` (the mod name is emitted as the base). Direct &str-param byte-scan.
// EMIT `{modbase}::<name>` for each FIXED (non-metavar) type owner inside the macro body's `mod .. { }` scope, printing the
// local name String IMMEDIATELY (a Vec<String> returned + printed via .iter() mis-lowers each String to a raw pointer).
fn emit_mod_fixed_items(cratename: &str, modbase: &str, body: &str) {
    let bb = body.as_bytes(); let n = bb.len(); let mut i: usize = 0;
    let mut in_mod: bool = false; let mut depth: i64 = 0; let mut sawmod: bool = false; let mut want: bool = false;
    while i < n {
        if bb[i] == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            let is_open = j == i + 1 && bb[i] == 123;
            let is_close = j == i + 1 && bb[i] == 125;
            if is_open { if sawmod { in_mod = true; depth = 1; sawmod = false; } else { if in_mod { depth = depth + 1; } } }
            else { if is_close { if in_mod { depth = depth - 1; if depth == 0 { in_mod = false; } } }
            else { if in_mod && depth == 1 {
                if want { if bb[i] != 36 { let mut nm = String::new(); let mut q = i; while q < j { nm.push(bb[q] as char); q = q + 1; } println!("CRATERES {}|{}::{}", cratename, modbase, nm); } want = false; }
                else {
                    if str_match_at(body, i, "trait") { if j == i + 5 { want = true; } }
                    else { if str_match_at(body, i, "struct") { if j == i + 6 { want = true; } }
                    else { if str_match_at(body, i, "enum") { if j == i + 4 { want = true; } }
                    else { if str_match_at(body, i, "union") { if j == i + 5 { want = true; } }
                    else { if str_match_at(body, i, "type") { if j == i + 4 { want = true; } } } } } }
                }
            }
            else { if str_match_at(body, i, "mod") { if j == i + 3 { sawmod = true; } } } } }
            i = j;
        }
    }
}
// The LAST `$`-metavar name in a matcher. atomic_int!'s matcher ends `.. $int_type:ident $atomic_type:ident` (two
// space-separated metavars in the final comma-group); the generated `pub struct $atomic_type` is named by the LAST one,
// bound to the invocation's LAST ident (`.. i8 AtomicI8` -> AtomicI8). Direct &str-param byte-scan (find last `$` then read).
fn matcher_last_metavar(matcher: &str) -> String {
    let bb = matcher.as_bytes(); let n = bb.len(); let mut last_dollar: i64 = -1; let mut i: usize = 0;
    while i < n { if bb[i] == 36 { last_dollar = i as i64; } i = i + 1; }
    if last_dollar < 0 { return String::new(); }
    let mut k = (last_dollar as usize) + 1; while k < n { if bb[k] == 32 { k = k + 1; } else { break; } }
    let mut e = k; while e < n { if bb[e] == 32 { break; } if bb[e] == 58 { break; } e = e + 1; }
    let mut nm = String::new(); let mut q = k; while q < e { nm.push(bb[q] as char); q = q + 1; } nm
}
// The LAST whitespace-delimited token of an invocation (`.. i8 AtomicI8` -> AtomicI8). Direct &str-param byte-scan.
fn inv_last_ident(inv: &str) -> String {
    let bb = inv.as_bytes(); let mut e = bb.len();
    while e > 0 { if bb[e-1] == 32 { e = e - 1; } else { break; } }
    if e == 0 { return String::new(); }
    let mut s = e; while s > 0 { if bb[s-1] == 32 { break; } s = s - 1; }
    let mut nm = String::new(); let mut q = s; while q < e { nm.push(bb[q] as char); q = q + 1; } nm
}
// A non-repetition struct-gen macro whose struct is named by the matcher's LAST metavar (atomic_int!): emit the invocation's
// LAST ident as the struct name. SKIP invocations mentioning "128" -- `target_has_atomic*="128"` is UNIVERSALLY false (no
// stable Rust target has 128-bit atomics; rustc drops AtomicI128/U128), so binding them would be a false ext.
fn emit_last_metavar_struct(cratename: &str, base: &str, inv: &str, matcher: &str, body: &str) {
    if str_contains(matcher, "$ (") { return; }
    if str_contains(inv, "128") { return; }
    let lastmv = matcher_last_metavar(matcher);
    if lastmv.len() == 0 { return; }
    let metas = body_struct_metavars(body);
    let mut has: bool = false; for m in metas.iter() { if streq(m, &lastmv) { has = true; } }
    if !has { return; }
    let nm = inv_last_ident(inv);
    if nm.len() > 0 { if !struct_name_reserved(&nm) {
        let full = if base.len() == 0 { format!("{}", nm) } else { format!("{}::{}", base, nm) };
        println!("CRATERES {}|{}", cratename, full);
    } }
}
// A macro whose body generates a nested `mod NAME { <fixed items> $( struct $META .. )+ }` (nonzero impl_zeroable_primitive):
// emit `{base}::NAME`, each fixed interior item `{base}::NAME::<name>`, and (if the mod has a repetition struct) the
// invocation groups' first idents `{base}::NAME::<group-ident>`. Free fns + reuses count_type_groups/nth_group_first_ident.
fn emit_mac_mod(cratename: &str, base: &str, inv: &str, body: &str) {
    let modname = body_mod_name(body);
    if modname.len() == 0 { return; }
    let modbase = if base.len() == 0 { let mut s = String::new(); s.push_str(&modname); s } else { format!("{}::{}", base, modname) };
    println!("CRATERES {}|{}", cratename, modbase);
    emit_mod_fixed_items(cratename, &modbase, body);
    if str_contains(body, "struct $") {
        let cnt = count_type_groups(inv);
        let mut g: i64 = 0;
        while g < cnt { let nm = nth_group_first_ident(inv, g); if nm.len() > 0 { if !struct_name_reserved(&nm) { println!("CRATERES {}|{}::{}", cratename, modbase, nm); } } g = g + 1; }
    }
}
// (P5t isolation) byte-scan a macro body for `const|static [mut] NAME :` item names -> `{base}::NAME`.
fn scan_item_consts(cratename: &str, base: &str, body: &str) {
    let bb = body.as_bytes(); let n = bb.len(); let mut i: usize = 0;
    while i < n {
        if bb[i] == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if str_match_at(body, i, "const ") || str_match_at(body, i, "static ") {
                let mut ks = j; while ks < n { if bb[ks] == 32 { ks = ks + 1; } else { break; } }
                let mut ke = ks; while ke < n { if bb[ke] == 32 { break; } ke = ke + 1; }
                if str_match_at(body, ks, "mut ") {
                    ks = ke; while ks < n { if bb[ks] == 32 { ks = ks + 1; } else { break; } }
                    ke = ks; while ke < n { if bb[ke] == 32 { break; } ke = ke + 1; }
                }
                let mut p = ke; while p < n { if bb[p] == 32 { p = p + 1; } else { break; } }
                if p < n { if bb[p] == 58 { if ke > ks {
                    let mut nm = String::new(); let mut q = ks; while q < ke { nm.push(bb[q] as char); q = q + 1; }
                    println!("CRATERES {}|{}::{}", cratename, base, nm);
                } } }
                i = ke;
            } else { i = j; }
        }
    }
}
// A macro CALL in a method body -> if its registered body generates const/static ITEMS, emit them under `base`. Free fn +
// &Vec<String> registry (B1); body copied to a LOCAL then byte-scanned (the emit_one_mac_impl_rep idiom, P5l).
fn emit_maccall_consts(cratename: &str, base: &str, e: &ExprK, macn: &Vec<String>, macb: &Vec<String>) {
    match e {
        ExprK::MacCall(pp, delim, _toks) => {
            if streq(delim, "MacroDefBrace") { return; }
            let name = dump_path(pp);
            let mut bi: i64 = -1; let mut i: usize = 0; let nn = macn.len();
            while i < nn { if streq(&macn[i], &name) { bi = i as i64; } i = i + 1; }
            if bi < 0 { return; }
            let mut bl = String::new(); bl.push_str(&macb[bi as usize]);
            scan_item_consts(cratename, base, &bl);
        }
        _ => {}
    }
}
// For one impl METHOD (Fn), emit consts its body-tail/stmt macro calls generate, under `{ipath}::method`. A &Item free fn
// (finding-#8: the m.kind->Fn->body walk lives inside a &Item-param fn). core time try_from_secs! -> {impl#N}::method::CONST.
// The stmt loop lives in a &Block-param fn (finding-#8/S7-F1: a loop over b.stmts reached through m.kind->Fn->body inline
// mis-lowers newstage0; emit_fn_body_items lowers only because it DELEGATES to a &Block fn -- same discipline here).
fn emit_block_mac_consts(cratename: &str, base: &str, b: &Block, macn: &Vec<String>, macb: &Vec<String>) {
    for st in b.stmts.iter() {
        match st {
            Stmt::Expr(e) => { emit_maccall_consts(cratename, base, e, macn, macb); }
            Stmt::Semi(e) => { emit_maccall_consts(cratename, base, e, macn, macb); }
            _ => {}
        }
    }
}
fn emit_method_mac_consts(cratename: &str, ipath: &str, m: &Item, macn: &Vec<String>, macb: &Vec<String>) {
    match &m.kind {
        ItemKind::Fn(_g, _p, _r, body, _a) => {
            if m.ident.len() > 0 {
                let base = format!("{}::{}", ipath, m.ident);
                match body { Some(b) => { emit_block_mac_consts(cratename, &base, b, macn, macb); }, None => {} }
            }
        }
        _ => {}
    }
}
fn emit_impl_members(cratename: &str, ip: &str, body: &str) {
    let bb = body.as_bytes(); let n = bb.len();
    let mut depth: i64 = 0; let mut i: usize = 0;
    while i < n {
        let c = bb[i];
        if c == 123 { depth = depth + 1; i = i + 1; }
        else { if c == 125 { depth = depth - 1; i = i + 1; }
        else { if c == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if depth == 1 { let w = &body[i..j]; if streq(w, "fn") || streq(w, "const") {
                let mut k = j; while k < n { if bb[k] != 32 { break; } k = k + 1; }
                let mut m = k; while m < n { if bb[m] == 32 { break; } if bb[m] == 40 { break; } m = m + 1; }
                if m > k { let mut nm = String::new(); let mut p = k; while p < m { nm.push(bb[p] as char); p = p + 1; } println!("CRATERES {}|{}::{}", cratename, ip, nm); }
            } }
            i = j;
        } } }
    }
}
// Number of top-level (bracket-depth-0) NON-EMPTY comma-separated groups in a macro invocation's joined tokens -- i.e. how
// many `$T` the `$($T:ty),+ $(,)?` matched (trailing comma yields an empty final segment, not counted).
// S2-MACRO slice 2: a method-generating macro (`$( ... fn $name(..) {..} )*` matched by `$( $name:ident, .. ; )*`, e.g. hir
// expect_methods_self_kind!/expect_methods_self!) generates one method per `;`-separated invocation group, NAMED by the
// group's FIRST ident. EMIT `{ipath}::name` per group (methods in the enclosing impl -> no impl_k). Byte-scan the LOCAL
// joined invocation `inv`; the name is the first ident of each top-level `;`-group (rzrs S2-MAC-F1: local + inline).
fn emit_semi_group_first_idents(cratename: &str, ipath: &str, inv: &str) {
    let bb = inv.as_bytes(); let n = bb.len();
    let mut depth: i64 = 0; let mut i: usize = 0; let mut at_group_start: bool = true;
    while i < n {
        let c = bb[i];
        if c == 60 || c == 40 || c == 91 || c == 123 { depth = depth + 1; i = i + 1; }
        else { if c == 62 || c == 41 || c == 93 || c == 125 { depth = depth - 1; i = i + 1; }
        else { if c == 59 && depth == 0 { at_group_start = true; i = i + 1; }
        else { if c == 32 { i = i + 1; }
        else {
            if at_group_start && depth == 0 {
                let mut m = i; while m < n { if bb[m] == 32 { break; } if bb[m] == 44 { break; } m = m + 1; }
                if m > i { let mut nm = String::new(); let mut p = i; while p < m { nm.push(bb[p] as char); p = p + 1; } println!("CRATERES {}|{}::{}", cratename, ipath, nm); }
                at_group_start = false;
                i = m;
            } else { i = i + 1; }
        } } } }
    }
}
// COMMA-separated variant of the method-gen emitter: one method per top-level `,`-group, named by the group's FIRST ident
// (proc_macro `suffixed_int_literals!`/`unsuffixed_int_literals!`: `$name => $kind,` groups). Depth tracks ONLY `([{`/`)]}`
// -- NOT `<`/`>` -- so the `=>` arrow's `>` does not corrupt the depth (the semi variant tracks angles for `;`-shaped
// method macros whose bodies carry generics). Methods -> no impl_k effect (pure additions).
fn emit_comma_group_first_idents(cratename: &str, ipath: &str, inv: &str) {
    let bb = inv.as_bytes(); let n = bb.len();
    let mut depth: i64 = 0; let mut i: usize = 0; let mut at_group_start: bool = true;
    while i < n {
        let c = bb[i];
        if c == 40 || c == 91 || c == 123 { depth = depth + 1; i = i + 1; }
        else { if c == 41 || c == 93 || c == 125 { depth = depth - 1; i = i + 1; }
        else { if c == 44 && depth == 0 { at_group_start = true; i = i + 1; }
        else { if c == 32 { i = i + 1; }
        else {
            if at_group_start && depth == 0 {
                let mut m = i; while m < n { if bb[m] == 32 { break; } if bb[m] == 44 { break; } m = m + 1; }
                if m > i { let mut nm = String::new(); let mut p = i; while p < m { nm.push(bb[p] as char); p = p + 1; } println!("CRATERES {}|{}::{}", cratename, ipath, nm); }
                at_group_start = false;
                i = m;
            } else { i = i + 1; }
        } } } }
    }
}
fn count_type_groups(inv: &str) -> i64 {
    let bb = inv.as_bytes(); let n = bb.len();
    let mut groups: i64 = 0; let mut depth: i64 = 0; let mut seg_has: bool = false; let mut i: usize = 0;
    let mut had_comma: bool = false; let mut had_bracket: bool = false;   // to detect the separator-less `$($x:ty)*` space list
    let mut words: i64 = 0; let mut in_word: bool = false;                 // depth-0 whitespace-separated word count (bracket-free)
    while i < n {
        let c = bb[i];
        if c == 60 || c == 40 || c == 91 || c == 123 { depth = depth + 1; had_bracket = true; }
        if c == 62 || c == 41 || c == 93 || c == 125 { depth = depth - 1; }
        if c == 44 && depth == 0 { had_comma = true; if seg_has { groups = groups + 1; seg_has = false; } }
        else { if c != 32 { seg_has = true; } }
        if c == 32 { in_word = false; } else { if !in_word { words = words + 1; in_word = true; } }
        i = i + 1;
    }
    if seg_has { groups = groups + 1; }
    // A separator-less repetition (`$($ty:ty)*` / `$($x:ident)*`, e.g. ascii `into_int_impl!(u8 u16 u32 char)`) has NO
    // top-level commas; when the invocation is also bracket-free (all simple idents) each space-separated word is one
    // matched group. Complex space lists (with `<`/`(`/`[`) stay at the comma-group count (conservative: no over-count).
    if !had_comma { if !had_bracket { return words; } }
    groups
}
fn macro_def_name(e: &ExprK) -> String {
    match e {
        ExprK::MacCall(pp, delim, _toks) => { if streq(delim, "MacroDefBrace") { dump_path(pp) } else { String::new() } }
        _ => String::new()
    }
}
// A `mod`'s items get the SAME two-pass emission as the crate root (emit_unit): PASS A = uses + hand-written impls
// ({base}::{impl#K}, mk resets to 0 -- a mod is a fresh def scope) + item names/recursion; PASS B = derives. The bare
// emit_crate_defpath recursion dropped mod-nested IMPLS (empty ident) and their DERIVES. A &Item param (S3-CD-F1-safe).
fn emit_mod_items(cratename: &str, base: &str, it: &Item) -> i64 {
    match &it.kind {
        ItemKind::Mod(inner) => {
            let mut mk: i64 = 0; let mut uk: i64 = 0;
            for inr in inner.iter() {
                if item_is_use(inr) { if !attrs_cfg_off(&inr.attrs) { let cnt = use_item_count(inr); let mut j: i64 = 0; while j < cnt { println!("CRATERES {}|{}::{{use#{}}}", cratename, base, uk); uk = uk + 1; j = j + 1; } } }
                else { match &inr.kind {
                    ItemKind::Impl(_g, _tr, _ty, _im) => { if !attrs_cfg_off(&inr.attrs) { let ip = format!("{}::{{impl#{}}}", base, mk); println!("CRATERES {}|{}", cratename, ip); emit_impl_methods(cratename, &ip, inr); mk = mk + 1; } }
                    _ => { emit_crate_defpath(cratename, base, inr); }
                } }
            }
            for inr in inner.iter() { mk = emit_derives(cratename, base, inr, mk); }
            return mk;   // the mod's impl counter after PASS A (hand-written) + PASS B (derives) -- the phase-2 start for mod-nested impl-rep macro CALLS (PASS C, run by the caller with the registry)
        }
        _ => {}
    }
    0
}
// PASS C-macro for a MOD: expand mod-nested impl-rep macro CALLS (`$(impl Tr for $t {..})+`) to {base}::{impl#N} in the
// phase-2 tier (impl_k starts at `mk`, the mod's post-A/B counter). A &Item free fn (finding-#8-safe walk) taking the
// macro registry as &Vec<String> (finding-#3 is Vec<T>-with-.kind; &Vec<String> lowers -- vec_contains/block_has_undefined_call
// precedent). No new Parser method (finding-B1). Mirrors emit_unit's top-level PASS C (emit_mac_impls' impl-rep branch).
fn emit_mod_mac_impls(cratename: &str, base: &str, it: &Item, mk: i64, macn: &Vec<String>, macb: &Vec<String>) {
    match &it.kind {
        ItemKind::Mod(inner) => {
            let mut k = mk;
            for inr in inner.iter() {
                match &inr.kind {
                    ItemKind::MacCall(e) => { k = emit_one_mac_impl_rep(cratename, base, e, k, macn, macb); }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
// The impl-rep branch of emit_mac_impls as a FREE fn (registry via &Vec<String> params instead of &self). For a macro CALL
// whose registered body is `$( impl .. )+`, emit `{base}::{impl#k}` + its members per invocation type-group, continuing k.
fn emit_one_mac_impl_rep(cratename: &str, base: &str, e: &ExprK, impl_k: i64, macn: &Vec<String>, macb: &Vec<String>) -> i64 {
    match e {
        ExprK::MacCall(pp, delim, toks) => {
            if streq(delim, "MacroDefBrace") { return impl_k; }
            let name = dump_path(pp);
            let mut bi: i64 = -1; let mut i: usize = 0; let nn = macn.len();
            while i < nn { if streq(&macn[i], &name) { let mut bl = String::new(); bl.push_str(&macb[i]); if mac_body_is_impl_rep(&bl) { bi = i as i64; } } i = i + 1; }
            if bi < 0 { return impl_k; }
            let mut bodyloc = String::new(); bodyloc.push_str(&macb[bi as usize]);
            let mut inv = String::new(); for t in toks.iter() { if inv.len() > 0 { inv.push(' '); } inv.push_str(t); }
            let cnt = count_type_groups(&inv);
            let mut k = impl_k; let mut g: i64 = 0;
            while g < cnt {
                let ip = if base.len() == 0 { format!("{{impl#{}}}", k) } else { format!("{}::{{impl#{}}}", base, k) };
                println!("CRATERES {}|{}", cratename, ip);
                emit_impl_members(cratename, &ip, &bodyloc);
                k = k + 1; g = g + 1;
            }
            k
        }
        _ => impl_k
    }
}
fn emit_crate_defpath(cratename: &str, base: &str, it: &Item) {
    if attrs_cfg_off(&it.attrs) { return; }
    match &it.kind {
        ItemKind::Use(_ut) => {}
        ItemKind::MacCall(e) => {
            let nm = macro_def_name(e);
            if nm.len() > 0 {
                let full = if base.len() == 0 { format!("{}", nm) } else { format!("{}::{}", base, nm) };
                println!("CRATERES {}|{}", cratename, full);
            }
        }
        _ => {
            if it.ident.len() > 0 {
                let full = if base.len() == 0 { format!("{}", it.ident) } else { format!("{}::{}", base, it.ident) };
                println!("CRATERES {}|{}", cratename, full);
                match &it.kind {
                    ItemKind::Mod(_inner) => { let _ = emit_mod_items(cratename, &full, it); }
                    ItemKind::Fn(_g, _p, _r, _b, _a) => { emit_fn_body_items(cratename, &full, it); }
                    ItemKind::Trait(_gt, _inner) => { emit_impl_methods(cratename, &full, it); }
                    _ => {}
                }
            }
        }
    }
}
// S1-scale-up: items NESTED IN A FN BODY (`struct`/`impl`/`fn` inside `fn make_display(){..}`) get rustc def-paths
// `<fn>::<item>` / `<fn>::{impl#K}::<method>`. The stmt LOOP lives in a &Block-PARAM fn (emit_block_items) -- NOT reached
// through it.kind->Fn->body->Some(b) in a loop (S7-F1 truncates that); emit_fn_body_items just extracts b + delegates.
// The per-item .kind dispatch is a &Item-PARAM fn (emit_nested_item, S3-CD-F1).
fn emit_nested_item(cratename: &str, fnbase: &str, inner: &Item, impl_k: i64) -> i64 {
    match &inner.kind {
        ItemKind::Impl(_g2, _t2, _y2, _i2) => {
            if !attrs_cfg_off(&inner.attrs) {
                let ipath = format!("{}::{{impl#{}}}", fnbase, impl_k);
                println!("CRATERES {}|{}", cratename, ipath);
                emit_impl_methods(cratename, &ipath, inner);
                return impl_k + 1;
            }
            impl_k
        }
        _ => { emit_crate_defpath(cratename, fnbase, inner); impl_k }
    }
}
// Nested-block ITEM NAMES: an item in a non-closure nested block (match arm / if-else / loop / bare block) shares the
// enclosing fn scope -- def-path `{fnbase}::name` (no block segment). NAMES only (via emit_crate_defpath) -- impls (empty
// ident) + uses are skipped here, so this can NOT perturb the fn's impl_k/use_k (pure additions; sort -u dedups). &Block/
// &ExprK/&Arm params (the proven dump idiom). Mutually recursive with emit_expr_nested_items.
fn emit_block_nested_items(cratename: &str, base: &str, b: &Block) {
    for st in b.stmts.iter() {
        match st {
            Stmt::Item(inner) => { emit_crate_defpath(cratename, base, inner); }
            Stmt::Expr(e) => { emit_expr_nested_items(cratename, base, e); }
            Stmt::Semi(e) => { emit_expr_nested_items(cratename, base, e); }
            Stmt::Let(_p, _t, init, els) => { match init { Some(e) => { emit_expr_nested_items(cratename, base, e); }, _ => {} } match els { Some(bl) => { emit_block_nested_items(cratename, base, bl); }, _ => {} } }
            _ => {}
        }
    }
}
fn emit_expr_nested_items(cratename: &str, base: &str, e: &ExprK) {
    match e {
        ExprK::If(_c, b, els) => { emit_block_nested_items(cratename, base, b); for x in els.iter() { emit_expr_nested_items(cratename, base, x); } }
        ExprK::Match(_s, arms) => { for a in arms.iter() { emit_expr_nested_items(cratename, base, &a.body); } }
        ExprK::Block(b) => { emit_block_nested_items(cratename, base, b); }
        ExprK::Unsafe(b) => { emit_block_nested_items(cratename, base, b); }
        ExprK::Loop(_l, b) => { emit_block_nested_items(cratename, base, b); }
        ExprK::While(_l, _c, b) => { emit_block_nested_items(cratename, base, b); }
        ExprK::ForLoop(_l, _p, _i, b) => { emit_block_nested_items(cratename, base, b); }
        ExprK::Async(_m, b) => { emit_block_nested_items(cratename, base, b); }
        ExprK::ConstBlock(b) => { emit_block_nested_items(cratename, base, b); }
        ExprK::TryBlock(b) => { emit_block_nested_items(cratename, base, b); }
        _ => {}
    }
}
// CLOSURE bodies are a FRESH def scope: rustc gives every closure in a body a `{closure#N}` segment (numbered over the
// WHOLE enclosing body, source order, INCLUDING closures in nested blocks), and items/uses inside get
// `{base}::{closure#N}::..`. A closure has no standalone owner_id (only its inner defs surface), so we emit nothing for
// the closure itself -- we number it (ck) and, for a `{ .. }` body, run emit_block_items on it with the {closure#N} base
// (fresh impl_k/use_k/ck inside). ck THREADS by return value through a complete &ExprK walk mirroring dump_expr (proven to
// lower). Nested non-closure blocks share the enclosing ck (a closure in a match-arm still counts in the fn's sequence).
fn emit_closures_block(cratename: &str, base: &str, b: &Block, ck: i64) -> i64 {
    let mut k = ck;
    for st in b.stmts.iter() {
        match st {
            Stmt::Expr(e) => { k = emit_closures_expr(cratename, base, e, k); }
            Stmt::Semi(e) => { k = emit_closures_expr(cratename, base, e, k); }
            Stmt::Let(_p, _t, init, els) => { match init { Some(e) => { k = emit_closures_expr(cratename, base, e, k); }, _ => {} } match els { Some(bl) => { k = emit_closures_block(cratename, base, bl, k); }, _ => {} } }
            _ => {}
        }
    }
    k
}
fn emit_closures_expr(cratename: &str, base: &str, e: &ExprK, ck: i64) -> i64 {
    match e {
        ExprK::Closure(_mv, _ps, _rt, body) => {
            let cb = format!("{}::{{closure#{}}}", base, ck);
            match &**body { ExprK::Block(bl) => { emit_block_items(cratename, &cb, bl); }, _ => { let _ = emit_closures_expr(cratename, &cb, body, 0); } }
            ck + 1
        }
        ExprK::Bin(_o, a, b) => { let k = emit_closures_expr(cratename, base, a, ck); emit_closures_expr(cratename, base, b, k) }
        ExprK::Unary(_o, a) => { emit_closures_expr(cratename, base, a, ck) }
        ExprK::Paren(a) => { emit_closures_expr(cratename, base, a, ck) }
        ExprK::Field(r, _n) => { emit_closures_expr(cratename, base, r, ck) }
        ExprK::Method(r, _n, _t, args) => { let mut k = emit_closures_expr(cratename, base, r, ck); for a in args.iter() { k = emit_closures_expr(cratename, base, a, k); } k }
        ExprK::Call(c, args) => { let mut k = emit_closures_expr(cratename, base, c, ck); for a in args.iter() { k = emit_closures_expr(cratename, base, a, k); } k }
        ExprK::Index(r, i) => { let k = emit_closures_expr(cratename, base, r, ck); emit_closures_expr(cratename, base, i, k) }
        ExprK::If(c, t, els) => { let mut k = emit_closures_expr(cratename, base, c, ck); k = emit_closures_block(cratename, base, t, k); for x in els.iter() { k = emit_closures_expr(cratename, base, x, k); } k }
        ExprK::Block(b) => { emit_closures_block(cratename, base, b, ck) }
        ExprK::Unsafe(b) => { emit_closures_block(cratename, base, b, ck) }
        ExprK::Match(s, arms) => { let mut k = emit_closures_expr(cratename, base, s, ck); for a in arms.iter() { match &a.guard { Some(g) => { k = emit_closures_expr(cratename, base, g, k); }, _ => {} } k = emit_closures_expr(cratename, base, &a.body, k); } k }
        ExprK::Ref(_m, a) => { emit_closures_expr(cratename, base, a, ck) }
        ExprK::RawRef(_m, a) => { emit_closures_expr(cratename, base, a, ck) }
        ExprK::Deref(a) => { emit_closures_expr(cratename, base, a, ck) }
        ExprK::Cast(a, _ty) => { emit_closures_expr(cratename, base, a, ck) }
        ExprK::Repeat(v, n) => { let k = emit_closures_expr(cratename, base, v, ck); emit_closures_expr(cratename, base, n, k) }
        ExprK::Async(_m, b) => { emit_closures_block(cratename, base, b, ck) }
        ExprK::ConstBlock(b) => { emit_closures_block(cratename, base, b, ck) }
        ExprK::Array(items) => { let mut k = ck; for a in items.iter() { k = emit_closures_expr(cratename, base, a, k); } k }
        ExprK::Tuple(items) => { let mut k = ck; for a in items.iter() { k = emit_closures_expr(cratename, base, a, k); } k }
        ExprK::Range(a, b, _inc) => { let mut k = ck; match a { Some(x) => { k = emit_closures_expr(cratename, base, x, k); }, _ => {} } match b { Some(x) => { k = emit_closures_expr(cratename, base, x, k); }, _ => {} } k }
        ExprK::While(_l, c, b) => { let k = emit_closures_expr(cratename, base, c, ck); emit_closures_block(cratename, base, b, k) }
        ExprK::Loop(_l, b) => { emit_closures_block(cratename, base, b, ck) }
        ExprK::ForLoop(_l, _p, it, b) => { let k = emit_closures_expr(cratename, base, it, ck); emit_closures_block(cratename, base, b, k) }
        ExprK::Assign(l, r) => { let k = emit_closures_expr(cratename, base, l, ck); emit_closures_expr(cratename, base, r, k) }
        ExprK::AssignOp(_o, l, r) => { let k = emit_closures_expr(cratename, base, l, ck); emit_closures_expr(cratename, base, r, k) }
        ExprK::Try(a) => { emit_closures_expr(cratename, base, a, ck) }
        ExprK::Await(a) => { emit_closures_expr(cratename, base, a, ck) }
        ExprK::Let(_p, s) => { emit_closures_expr(cratename, base, s, ck) }
        ExprK::Ret(v) => { let mut k = ck; for x in v.iter() { k = emit_closures_expr(cratename, base, x, k); } k }
        ExprK::Break(_l, v) => { let mut k = ck; for x in v.iter() { k = emit_closures_expr(cratename, base, x, k); } k }
        ExprK::Struct(_p, fields, bs) => { let mut k = ck; for f in fields.iter() { k = emit_closures_expr(cratename, base, &f.expr, k); } match bs { Some(x) => { k = emit_closures_expr(cratename, base, x, k); }, _ => {} } k }
        _ => ck
    }
}
// PASS A depth-first: hand-written impls ({base}::{impl#K}) + item NAMES, in SOURCE order over this block AND its NON-closure
// nested blocks (match arm/if-else/loop) -- rustc numbers a scope's hand-written impls source-order-depth-first, a nested
// impl interleaved by position (verified: `if c{impl A} impl B` -> A=impl#0, B=impl#1). Uses/derives/closures excluded here.
fn emit_scope_a(cratename: &str, base: &str, b: &Block, ik: i64) -> i64 {
    let mut k = ik;
    for st in b.stmts.iter() {
        match st {
            Stmt::Item(inner) => {
                if item_is_use(inner) {}
                else { match &inner.kind {
                    ItemKind::Impl(_g, _tr, _ty, _im) => { if !attrs_cfg_off(&inner.attrs) { let ip = format!("{}::{{impl#{}}}", base, k); println!("CRATERES {}|{}", cratename, ip); emit_impl_methods(cratename, &ip, inner); k = k + 1; } }
                    _ => { emit_crate_defpath(cratename, base, inner); }
                } }
            }
            Stmt::Expr(e) => { k = emit_scope_a_expr(cratename, base, e, k); }
            Stmt::Semi(e) => { k = emit_scope_a_expr(cratename, base, e, k); }
            Stmt::Let(_p, _t, init, els) => { match init { Some(e) => { k = emit_scope_a_expr(cratename, base, e, k); }, _ => {} } match els { Some(bl) => { k = emit_scope_a(cratename, base, bl, k); }, _ => {} } }
            _ => {}
        }
    }
    k
}
fn emit_scope_a_expr(cratename: &str, base: &str, e: &ExprK, ik: i64) -> i64 {
    match e {
        ExprK::If(_c, b, els) => { let mut k = emit_scope_a(cratename, base, b, ik); for x in els.iter() { k = emit_scope_a_expr(cratename, base, x, k); } k }
        ExprK::Match(_s, arms) => { let mut k = ik; for a in arms.iter() { k = emit_scope_a_expr(cratename, base, &a.body, k); } k }
        ExprK::Block(b) => { emit_scope_a(cratename, base, b, ik) }
        ExprK::Unsafe(b) => { emit_scope_a(cratename, base, b, ik) }
        ExprK::Loop(_l, b) => { emit_scope_a(cratename, base, b, ik) }
        ExprK::While(_l, _c, b) => { emit_scope_a(cratename, base, b, ik) }
        ExprK::ForLoop(_l, _p, _i, b) => { emit_scope_a(cratename, base, b, ik) }
        ExprK::Async(_m, b) => { emit_scope_a(cratename, base, b, ik) }
        ExprK::ConstBlock(b) => { emit_scope_a(cratename, base, b, ik) }
        _ => ik
    }
}
// PASS B depth-first: DERIVED impls (continuing impl_k after PASS A), source order over the block + non-closure nested blocks.
fn emit_scope_b(cratename: &str, base: &str, b: &Block, ik: i64) -> i64 {
    let mut k = ik;
    for st in b.stmts.iter() {
        match st {
            Stmt::Item(inner) => { k = emit_derives(cratename, base, inner, k); }
            Stmt::Expr(e) => { k = emit_scope_b_expr(cratename, base, e, k); }
            Stmt::Semi(e) => { k = emit_scope_b_expr(cratename, base, e, k); }
            Stmt::Let(_p, _t, init, els) => { match init { Some(e) => { k = emit_scope_b_expr(cratename, base, e, k); }, _ => {} } match els { Some(bl) => { k = emit_scope_b(cratename, base, bl, k); }, _ => {} } }
            _ => {}
        }
    }
    k
}
fn emit_scope_b_expr(cratename: &str, base: &str, e: &ExprK, ik: i64) -> i64 {
    match e {
        ExprK::If(_c, b, els) => { let mut k = emit_scope_b(cratename, base, b, ik); for x in els.iter() { k = emit_scope_b_expr(cratename, base, x, k); } k }
        ExprK::Match(_s, arms) => { let mut k = ik; for a in arms.iter() { k = emit_scope_b_expr(cratename, base, &a.body, k); } k }
        ExprK::Block(b) => { emit_scope_b(cratename, base, b, ik) }
        ExprK::Unsafe(b) => { emit_scope_b(cratename, base, b, ik) }
        ExprK::Loop(_l, b) => { emit_scope_b(cratename, base, b, ik) }
        ExprK::While(_l, _c, b) => { emit_scope_b(cratename, base, b, ik) }
        ExprK::ForLoop(_l, _p, _i, b) => { emit_scope_b(cratename, base, b, ik) }
        ExprK::Async(_m, b) => { emit_scope_b(cratename, base, b, ik) }
        ExprK::ConstBlock(b) => { emit_scope_b(cratename, base, b, ik) }
        _ => ik
    }
}
// PASS-uses depth-first: `{base}::{use#K}` over the block + non-closure nested blocks (a match-arm use continues the fn use#).
fn vec_count(v: &Vec<String>, s: &str) -> i64 { let mut c: i64 = 0; for x in v.iter() { if streq(x, s) { c = c + 1; } } c }
// A fn-body nested item's namespace tag for disambiguation: value-ns (const/static/fn) vs type-ns (struct/enum/union/type/
// trait/mod). Same-named same-ns items in one fn scope get rustc DefPath disambiguators 0,1,2.. (mbe is_in_follow's 5
// `const TOKENS` in match arms -> TOKENS, TOKENS#1..4). Cross-ns same name does NOT collide -> tag keeps them separate.
fn item_ns_tag(it: &Item) -> String {
    match &it.kind {
        ItemKind::Const(_a, _b) => String::from("V"), ItemKind::Static(_a, _b) => String::from("V"), ItemKind::Fn(_a, _b, _c, _d, _e) => String::from("V"),
        ItemKind::Struct(_a, _b) => String::from("T"), ItemKind::TupleStruct(_a, _b) => String::from("T"), ItemKind::UnitStruct(_a) => String::from("T"),
        ItemKind::Enum(_a, _b) => String::from("T"), ItemKind::Union(_a, _b) => String::from("T"), ItemKind::TypeAlias(_a, _b) => String::from("T"),
        ItemKind::Trait(_a, _b) => String::from("T"), ItemKind::Mod(_a) => String::from("T"),
        _ => String::from("")
    }
}
// ADDITIVE disambiguation pass: emit ONLY the `NAME#k` (k>=1) repeats of a same-named same-ns nested item within the fn
// scope (the base NAME is already emitted by emit_scope_a; sort -u yields the disambiguator-0). `seen` is a Vec of
// `<nstag>:<name>` threaded in source order (Vec<String> return lowers, P5u). A nested fn's OWN body is a different scope
// -> not descended (like emit_scope_uses).
fn emit_scope_disambig(cratename: &str, base: &str, b: &Block, seen: Vec<String>) -> Vec<String> {
    let mut sv = seen;
    for st in b.stmts.iter() {
        match st {
            Stmt::Item(inner) => {
                if item_is_use(inner) {}
                else { if inner.ident.len() > 0 { if !attrs_cfg_off(&inner.attrs) { if !attrs_have_cfg(&inner.attrs) {
                    let tag = item_ns_tag(inner);
                    if tag.len() > 0 {
                        let mut key = String::new(); key.push_str(&tag); key.push_str(":"); key.push_str(&inner.ident);
                        let c = vec_count(&sv, &key);
                        if c >= 1 { println!("CRATERES {}|{}::{}#{}", cratename, base, inner.ident, c); }
                        sv.push(key);
                    }
                } } } }
            }
            Stmt::Expr(e) => { sv = emit_scope_disambig_expr(cratename, base, e, sv); }
            Stmt::Semi(e) => { sv = emit_scope_disambig_expr(cratename, base, e, sv); }
            Stmt::Let(_p, _t, init, els) => { match init { Some(e) => { sv = emit_scope_disambig_expr(cratename, base, e, sv); }, _ => {} } match els { Some(bl) => { sv = emit_scope_disambig(cratename, base, bl, sv); }, _ => {} } }
            _ => {}
        }
    }
    sv
}
fn emit_scope_disambig_expr(cratename: &str, base: &str, e: &ExprK, seen: Vec<String>) -> Vec<String> {
    match e {
        ExprK::If(_c, b, els) => { let mut sv = emit_scope_disambig(cratename, base, b, seen); for x in els.iter() { sv = emit_scope_disambig_expr(cratename, base, x, sv); } sv }
        ExprK::Match(_s, arms) => { let mut sv = seen; for a in arms.iter() { sv = emit_scope_disambig_expr(cratename, base, &a.body, sv); } sv }
        ExprK::Block(b) => { emit_scope_disambig(cratename, base, b, seen) }
        ExprK::Unsafe(b) => { emit_scope_disambig(cratename, base, b, seen) }
        ExprK::Loop(_l, b) => { emit_scope_disambig(cratename, base, b, seen) }
        ExprK::While(_l, _c, b) => { emit_scope_disambig(cratename, base, b, seen) }
        ExprK::ForLoop(_l, _p, _i, b) => { emit_scope_disambig(cratename, base, b, seen) }
        ExprK::Async(_m, b) => { emit_scope_disambig(cratename, base, b, seen) }
        ExprK::ConstBlock(b) => { emit_scope_disambig(cratename, base, b, seen) }
        ExprK::Assign(l, r) => { let sv = emit_scope_disambig_expr(cratename, base, l, seen); emit_scope_disambig_expr(cratename, base, r, sv) }
        ExprK::AssignOp(_op, l, r) => { let sv = emit_scope_disambig_expr(cratename, base, l, seen); emit_scope_disambig_expr(cratename, base, r, sv) }
        _ => seen
    }
}
fn emit_scope_uses(cratename: &str, base: &str, b: &Block, uk: i64) -> i64 {
    let mut k = uk;
    for st in b.stmts.iter() {
        match st {
            Stmt::Item(inner) => { if item_is_use(inner) { if !attrs_cfg_off(&inner.attrs) { let cnt = use_item_count(inner); let mut j: i64 = 0; while j < cnt { println!("CRATERES {}|{}::{{use#{}}}", cratename, base, k); k = k + 1; j = j + 1; } } } }
            Stmt::Expr(e) => { k = emit_scope_uses_expr(cratename, base, e, k); }
            Stmt::Semi(e) => { k = emit_scope_uses_expr(cratename, base, e, k); }
            Stmt::Let(_p, _t, init, els) => { match init { Some(e) => { k = emit_scope_uses_expr(cratename, base, e, k); }, _ => {} } match els { Some(bl) => { k = emit_scope_uses(cratename, base, bl, k); }, _ => {} } }
            _ => {}
        }
    }
    k
}
fn emit_scope_uses_expr(cratename: &str, base: &str, e: &ExprK, uk: i64) -> i64 {
    match e {
        ExprK::If(_c, b, els) => { let mut k = emit_scope_uses(cratename, base, b, uk); for x in els.iter() { k = emit_scope_uses_expr(cratename, base, x, k); } k }
        ExprK::Match(_s, arms) => { let mut k = uk; for a in arms.iter() { k = emit_scope_uses_expr(cratename, base, &a.body, k); } k }
        ExprK::Block(b) => { emit_scope_uses(cratename, base, b, uk) }
        ExprK::Unsafe(b) => { emit_scope_uses(cratename, base, b, uk) }
        ExprK::Loop(_l, b) => { emit_scope_uses(cratename, base, b, uk) }
        ExprK::While(_l, _c, b) => { emit_scope_uses(cratename, base, b, uk) }
        ExprK::ForLoop(_l, _p, _i, b) => { emit_scope_uses(cratename, base, b, uk) }
        ExprK::Async(_m, b) => { emit_scope_uses(cratename, base, b, uk) }
        ExprK::ConstBlock(b) => { emit_scope_uses(cratename, base, b, uk) }
        ExprK::Assign(l, r) => { let k = emit_scope_uses_expr(cratename, base, l, uk); emit_scope_uses_expr(cratename, base, r, k) }   // `x = match {.. use ..}` -- a use in an assignment RHS (rustc pattern_analysis lower_pat: `ctor = match value {..}`)
        ExprK::AssignOp(_op, l, r) => { let k = emit_scope_uses_expr(cratename, base, l, uk); emit_scope_uses_expr(cratename, base, r, k) }
        _ => uk
    }
}
// PASS-externs depth-first: a fn-body `extern "ABI" { .. }` block -> `{base}::{extern#K}` + `{base}::{extern#K}::name`
// (a per-fn-scope extern-only counter, SEPARATE from impl#/use#, fresh at 0 -- rustc numbers in-fn extern blocks under the
// fn). Mirrors emit_scope_uses (own counter, descends non-closure nested blocks). Free fns (B1); uses extern_path +
// emit_foreign_members (the top-level P5e helpers). Needs `extern` in parse_stmt's starts_item (else the block is not a
// nested item and its foreign fn leaks as `{base}::name`).
fn emit_scope_externs(cratename: &str, base: &str, b: &Block, xk: i64) -> i64 {
    let mut k = xk;
    for st in b.stmts.iter() {
        match st {
            Stmt::Item(inner) => { match &inner.kind { ItemKind::ForeignMod(_fm) => { if !attrs_cfg_off(&inner.attrs) { let ep = extern_path(base, k); println!("CRATERES {}|{}", cratename, ep); emit_foreign_members(cratename, &ep, inner); k = k + 1; } }, _ => {} } }
            Stmt::Expr(e) => { k = emit_scope_externs_expr(cratename, base, e, k); }
            Stmt::Semi(e) => { k = emit_scope_externs_expr(cratename, base, e, k); }
            Stmt::Let(_p, _t, init, els) => { match init { Some(e) => { k = emit_scope_externs_expr(cratename, base, e, k); }, _ => {} } match els { Some(bl) => { k = emit_scope_externs(cratename, base, bl, k); }, _ => {} } }
            _ => {}
        }
    }
    k
}
fn emit_scope_externs_expr(cratename: &str, base: &str, e: &ExprK, xk: i64) -> i64 {
    match e {
        ExprK::If(_c, b, els) => { let mut k = emit_scope_externs(cratename, base, b, xk); for x in els.iter() { k = emit_scope_externs_expr(cratename, base, x, k); } k }
        ExprK::Match(_s, arms) => { let mut k = xk; for a in arms.iter() { k = emit_scope_externs_expr(cratename, base, &a.body, k); } k }
        ExprK::Block(b) => { emit_scope_externs(cratename, base, b, xk) }
        ExprK::Unsafe(b) => { emit_scope_externs(cratename, base, b, xk) }
        ExprK::Loop(_l, b) => { emit_scope_externs(cratename, base, b, xk) }
        ExprK::While(_l, _c, b) => { emit_scope_externs(cratename, base, b, xk) }
        ExprK::ForLoop(_l, _p, _i, b) => { emit_scope_externs(cratename, base, b, xk) }
        ExprK::Async(_m, b) => { emit_scope_externs(cratename, base, b, xk) }
        ExprK::ConstBlock(b) => { emit_scope_externs(cratename, base, b, xk) }
        _ => xk
    }
}
fn emit_block_items(cratename: &str, fnbase: &str, b: &Block) {
    let _ = emit_scope_uses(cratename, fnbase, b, 0);        // uses (own counter), depth-first
    let _ = emit_scope_externs(cratename, fnbase, b, 0);     // in-fn extern blocks ({extern#K}, own counter)
    let _ = emit_scope_disambig(cratename, fnbase, b, Vec::new());   // NAME#k repeats of same-named same-ns nested items (mbe TOKENS#1..4)
    let ik = emit_scope_a(cratename, fnbase, b, 0);          // PASS A: hand-written impls + names, depth-first source order
    let _ = emit_scope_b(cratename, fnbase, b, ik);          // PASS B: derives, continuing impl_k
    let _ = emit_closures_block(cratename, fnbase, b, 0);    // PASS D: closure bodies ({closure#N}, fresh scopes)
}
fn emit_fn_body_items(cratename: &str, fnbase: &str, it: &Item) {
    match &it.kind {
        ItemKind::Fn(_g, _p, _r, body, _a) => { match body { Some(b) => { emit_block_items(cratename, fnbase, b); }, None => {} } }
        _ => {}
    }
}
// CD-2: an impl block is a rustc pseudo-module `<base>::{impl#K}` (K = 0-based impl index in the module, source order)
// whose Fn items are `<base>::{impl#K}::<method>`. The method walk is EXTRACTED into a &Item-param fn (S7-F1: the walk
// through the destructured `inner` lives INSIDE a &Item-param fn, NOT inlined in the outer loop and NOT a &Vec<Item> param
// -- both of those truncate newstage0). The caller passes the impl item + its precomputed ipath.
// One associated item of an impl OR trait -> its def-path. rustc gives every NAMED assoc item a def-path: Fn (::m, +
// recurse its body for nested items), Const (::C), associated Type / AssocTyDecl (::T). Use/nested-impl are not named
// assoc items. A &Item param (S3-CD-F1-safe); shared by impls and trait definitions (identical assoc-item shapes).
// The TYPE generic-param names of an impl/trait (skip lifetimes + const params). Returns a Vec<String> (the CD-3b
// returned-record idiom). Used to disambiguate an assoc TYPE that shadows a same-named type param: rustc counts the type
// param (TypeNs disambiguator 0) before the assoc type, so `impl<T> Tr for X<T> { type T = ..; }` -> `{impl#N}::T#1`.
fn impl_gparam_type_names(it: &Item) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    match &it.kind {
        ItemKind::Impl(g, _tr, _ty, _inner) => { for gp in g.iter() { if !gp.is_lt { if !gp.is_const { let mut nm = String::new(); nm.push_str(&gp.name); out.push(nm); } } } }
        ItemKind::Trait(g, _inner) => { for gp in g.iter() { if !gp.is_lt { if !gp.is_const { let mut nm = String::new(); nm.push_str(&gp.name); out.push(nm); } } } }
        _ => {}
    }
    out
}
// An assoc TYPE (TypeAlias/AssocTyDecl) whose name shadows a type generic param of the impl gets rustc disambiguator `#1`
// (`NAME#1`); a same-named CONST/FN does NOT (different namespace). Const/Fn assoc items never shadow a type param.
fn assoc_type_defpath(cratename: &str, ipath: &str, nm: &str, gpn: &Vec<String>) {
    if vec_contains(gpn, nm) { println!("CRATERES {}|{}::{}#1", cratename, ipath, nm); }
    else { println!("CRATERES {}|{}::{}", cratename, ipath, nm); }
}
fn emit_assoc_item(cratename: &str, ipath: &str, m: &Item, gpn: &Vec<String>) {
    match &m.kind {
        ItemKind::Fn(_g2, _p2, _r2, _b2, _a2) => { if m.ident.len() > 0 { println!("CRATERES {}|{}::{}", cratename, ipath, m.ident); let mb = format!("{}::{}", ipath, m.ident); emit_fn_body_items(cratename, &mb, m); } }
        ItemKind::Const(_t2, _e2) => { if m.ident.len() > 0 { println!("CRATERES {}|{}::{}", cratename, ipath, m.ident); } }
        ItemKind::TypeAlias(_g3, _t3) => { if m.ident.len() > 0 { assoc_type_defpath(cratename, ipath, &m.ident, gpn); } }
        ItemKind::AssocTyDecl(_g4) => { if m.ident.len() > 0 { assoc_type_defpath(cratename, ipath, &m.ident, gpn); } }
        _ => {}
    }
}
// The named associated items of an impl OR a trait DEFINITION (`trait T { fn foo(); const C; type Out; }` -> T::foo/T::C/
// T::Out, exactly like impl assoc items). rustc def-paths a trait's items under the trait name (no {impl#N}).
// Scan a MACRO BODY for each `impl .. { .. }` block; emit {base}::{impl#k} + that block's members (fn/const/type at the
// block's top level), threading k. (impl_pos! body = `$( struct ..; impl Pos for $id{..} impl Add for $id{..} impl Sub ..)*`;
// self_ty is the metavar $ident, irrelevant to the {impl#k}::member owner path.) Returns the next k. Called once per
// invocation struct -> struct-grouped numbering.
// CD-17: count the `impl ` blocks in a macro repetition body. A body with 2+ impl blocks per repetition (rustc_serialize
// direct_serialize_impls! -> Encodable + Decodable per $ty) must emit that many {impl#N} per invocation group, not one.
fn count_impl_blocks(body: &str) -> i64 {
    let bb = body.as_bytes(); let n = bb.len(); let mut c: i64 = 0; let mut i: usize = 0;
    while i < n {
        if i + 4 <= n { if streq(&body[i..i+4], "impl") {
            let prevok = if i == 0 { true } else { bb[i-1] == 32 };
            let nextok = if i + 4 >= n { false } else { bb[i+4] == 32 };
            if prevok && nextok { c = c + 1; }
        } }
        i = i + 1;
    }
    c
}
// CD-18/19: a peano-recursive tuple-impl macro (rustc_serialize tuple!/peel!; core/std tuple impls). Signature: an
// ident-repetition matcher, a body with impl block(s), and a trailing NESTED macro call (`peel! {..}` = the recursion).
// It generates count_impl_blocks x recursion_depth impls, depth = the invocation ident count. Def-paths carry no
// tuple-specific names (only {impl#N} + members), so emitting count_impl_blocks impls per level == rustc.
fn mac_is_recursive_impl(matcher: &str, body: &str) -> bool {
    if !str_contains(matcher, "$ (") { return false; }
    if !str_contains(matcher, ": ident") { return false; }
    if count_impl_blocks(body) < 1 { return false; }
    str_contains(body, "! {") || str_contains(body, "! (")
}
fn emit_body_impl_blocks(cratename: &str, base: &str, body: &str, impl_k: i64) -> i64 {
    let bb = body.as_bytes(); let n = bb.len();
    let mut k = impl_k; let mut i: usize = 0;
    while i < n {
        let mut hit = false;
        if i + 4 <= n { if streq(&body[i..i+4], "impl") {
            let prevok = if i == 0 { true } else { bb[i-1] == 32 };
            let nextok = if i + 4 >= n { false } else { bb[i+4] == 32 };
            if prevok && nextok { hit = true; }
        } }
        if hit {
            let mut j = i + 4; while j < n { if bb[j] == 123 { break; } j = j + 1; }   // block open brace
            if j < n {
                let mut d: i64 = 0; let mut e = j;
                while e < n { let cc = bb[e]; if cc == 123 { d = d + 1; } else { if cc == 125 { d = d - 1; if d == 0 { e = e + 1; break; } } } e = e + 1; }
                let ip = if base.len() == 0 { format!("{{impl#{}}}", k) } else { format!("{}::{{impl#{}}}", base, k) };
                println!("CRATERES {}|{}", cratename, ip);
                if e > j + 2 { emit_fixed_impl_methods(cratename, &ip, &body[j+1..e-1]); }   // members between the block braces (depth 0 of the inner)
                k = k + 1; i = e;
            } else { i = i + 1; }
        } else { i = i + 1; }
    }
    k
}
// For an impl_pos!-shape invocation: per `# [ derive ( LIST ) ] .. struct NAME` (source order), emit that struct's derived
// impls (emit_derive_list), threading impl_k. The impl_pos! structs are non-generic -> sup_trivial=false (TrivialClone kept).
// Count `struct` keyword occurrences (space-bounded) in an impl_pos!-shape invocation -> the number of generated structs
// (each gets the macro body's impl blocks). count_type_groups mis-counts the `;`-separated struct list, so use this.
fn count_inv_structs(inv: &str) -> i64 {
    let bb = inv.as_bytes(); let n = bb.len();
    let mut c: i64 = 0; let mut i: usize = 0;
    while i < n {
        if i + 6 <= n { if streq(&inv[i..i+6], "struct") {
            let prevok = if i == 0 { true } else { bb[i-1] == 32 };
            let nextok = if i + 6 >= n { true } else { bb[i+6] == 32 };
            if prevok && nextok { c = c + 1; i = i + 6; continue; }
        } }
        i = i + 1;
    }
    c
}
fn emit_invocation_struct_derives(cratename: &str, base: &str, inv: &str, impl_k: i64) -> i64 {
    let bb = inv.as_bytes(); let n = bb.len();
    let mut k = impl_k; let mut i: usize = 0;
    while i < n {
        let mut hit = false;
        if i + 6 <= n { if streq(&inv[i..i+6], "derive") { let prevok = if i == 0 { true } else { bb[i-1] == 32 }; if prevok { hit = true; } } }
        if hit {
            let mut a = i + 6; while a < n { if bb[a] == 40 { break; } a = a + 1; }   // (
            if a < n {
                let mut d: i64 = 1; let mut b = a + 1;
                while b < n { let cc = bb[b]; if cc == 40 { d = d + 1; } else { if cc == 41 { d = d - 1; if d == 0 { break; } } } b = b + 1; }
                if b > a + 1 { let dl = &inv[a+1..b]; k = emit_derive_list(cratename, base, dl, false, k); }
                i = b + 1;
            } else { i = i + 1; }
        } else { i = i + 1; }
    }
    k
}
// Emit the FIXED-name fn/const/type members of a macro expansion body (depth 0), name terminating at space/(/</;/=. For a
// method-gen macro with an EMPTY matcher and concrete method names (e.g. bit_relations_inherent_impls! -> union/subtract/
// intersect added to the enclosing inherent impl). No new impls / no numbering change -- just the member owners.
fn emit_fixed_impl_methods(cratename: &str, ip: &str, body: &str) {
    let bb = body.as_bytes(); let n = bb.len();
    let mut depth: i64 = 0; let mut i: usize = 0;
    while i < n {
        let c = bb[i];
        if c == 123 { depth = depth + 1; i = i + 1; }
        else { if c == 125 { depth = depth - 1; i = i + 1; }
        else { if c == 32 { i = i + 1; }
        else {
            let mut j = i; while j < n { if bb[j] == 32 { break; } j = j + 1; }
            if depth == 0 { let w = &body[i..j]; if streq(w, "fn") || streq(w, "const") || streq(w, "type") {   // macro body has NO outer braces -> members at depth 0 (type = assoc type, e.g. impl_pos! Add/Sub `type Output`)
                let mut k = j; while k < n { if bb[k] != 32 { break; } k = k + 1; }
                let mut m = k; while m < n { let d = bb[m]; if d == 32 || d == 40 || d == 60 || d == 59 || d == 61 { break; } m = m + 1; }
                if m > k { let mut nm = String::new(); let mut p = k; while p < m { nm.push(bb[p] as char); p = p + 1; } println!("CRATERES {}|{}::{}", cratename, ip, nm); }
            } }
            i = j;
        } } }
    }
}
fn emit_impl_methods(cratename: &str, ipath: &str, it: &Item) {
    match &it.kind {
        ItemKind::Impl(_g, _tr, _ty, inner) => { let gpn = impl_gparam_type_names(it); for m in inner.iter() { emit_assoc_item(cratename, ipath, m, &gpn); } }
        ItemKind::Trait(_gt, inner) => { let gpn = impl_gparam_type_names(it); for m in inner.iter() { emit_assoc_item(cratename, ipath, m, &gpn); } }
        _ => {}
    }
}
// A `extern "ABI" { .. }` foreign module -> rustc gives the block a `{extern#N}` def-path (a per-scope extern-only
// counter, SEPARATE from {impl#N} -- verified: two extern blocks + two impls yield extern#0/extern#1 with impl#0/impl#1
// untouched), and each foreign item (fn/static/type) a `{extern#N}::name`. Same &Item-param + destructured-Vec .iter()
// shape as emit_impl_methods (the proven lowering pattern); emit_assoc_item skips Static, so this emits every named member
// directly (foreign items are all named, bodyless).
fn emit_foreign_members(cratename: &str, epath: &str, it: &Item) {
    match &it.kind {
        ItemKind::ForeignMod(inner) => { for m in inner.iter() { if m.ident.len() > 0 { if !attrs_cfg_off(&m.attrs) { println!("CRATERES {}|{}::{}", cratename, epath, m.ident); } } } }
        _ => {}
    }
}
fn extern_path(base: &str, k: i64) -> String {
    if base.len() == 0 { format!("{{extern#{}}}", k) } else { format!("{}::{{extern#{}}}", base, k) }
}
// DE-1: built-in DERIVE expansion at the def-path observable. rustc generates an impl per #[derive(Trait)] on a
// struct/enum/union: Clone -> {impl#K} + {impl#K}::clone. The derived impls follow the hand-written ones (PHASE 2), in
// item source order + derive-list order, continuing the crate {impl#K} counter. CONSERVATIVE (Rule-14): only KNOWN
// built-in derives emit; unknown derive paths emit nothing. Byte-scan the derive-list &str (findings #6/7).
fn item_has_custom_derive(it: &Item) -> bool {
    // OPTION B (keeper-authorized 2026-08-11): rustc DEFERS an item carrying an UNRESOLVABLE custom derive
    // (Encodable/Decodable/HashStable_Generic) to AFTER the builtin-only-derive items in phase-2 DefId order. Detect any
    // such derive on the item so PASS-B can emit builtin-only items first, then custom-derive-bearing items.
    for a in it.attrs.iter() {
        if streq(&dump_path(&a.path), "derive") {
            if str_contains(&a.args, "Encodable") { return true; }
            if str_contains(&a.args, "Decodable") { return true; }
            if str_contains(&a.args, "HashStable") { return true; }
        }
    }
    false
}
fn item_has_pure_builtin_attr(it: &Item) -> bool {
    // S8 PORT (2026-08-13): among custom-derive-bearing items, rustc splits phase-2 into B2a BEFORE B2b. B2a = items with
    // a SEPARATE pure-builtin `#[derive(..)]` ATTRIBUTE (no Encodable/Decodable/HashStable in THAT attr); its builtins
    // resolve+expand one phase earlier. B2b = items whose builtins are MIXED into the custom attr (deferred wholesale).
    // Returns true iff the item has at least one derive attribute containing no unresolvable custom derive.
    for a in it.attrs.iter() {
        if streq(&dump_path(&a.path), "derive") {
            if !str_contains(&a.args, "Encodable") && !str_contains(&a.args, "Decodable") && !str_contains(&a.args, "HashStable") { return true; }
        }
    }
    false
}
fn item_derivable(it: &Item) -> bool {
    match &it.kind {
        ItemKind::Struct(_g, _f) => true,
        ItemKind::TupleStruct(_g, _t) => true,
        ItemKind::UnitStruct(_g) => true,
        ItemKind::Enum(_g, _v) => true,
        ItemKind::Union(_g, _f) => true,
        _ => false
    }
}
fn trim_sp(s: &str) -> String {
    let by = s.as_bytes(); let mut a: usize = 0; let mut b: usize = by.len();
    while a < b { if by[a] == 32 { a = a + 1; } else { b = b; break; } }
    while b > a { if by[b-1] == 32 { b = b - 1; } else { a = a; break; } }
    format!("{}", &s[a..b])
}
fn impl_path(base: &str, k: i64) -> String {
    if base.len() == 0 { format!("{{impl#{}}}", k) } else { format!("{}::{{impl#{}}}", base, k) }
}
fn emit_derive_1(cratename: &str, ip: &str, method: &str) {
    println!("CRATERES {}|{}", cratename, ip);
    println!("CRATERES {}|{}::{}", cratename, ip, method);
}
fn str_contains(hay: &str, needle: &str) -> bool {
    let hb = hay.as_bytes(); let nl = needle.len(); let mut i: usize = 0;
    while i + nl <= hb.len() { if str_match_at(hay, i, needle) { return true; } i = i + 1; }
    false
}
// per-derive -> impl(s)+method(s), verified on rustc 1.83 witnesses. Derive-list order; continues the crate {impl#K}.
// has_copy: the derive list ALSO contains Copy -- then Clone emits an EXTRA TrivialClone MARKER impl before Clone::clone.
// rustc emits the Copy+Clone `TrivialClone` marker impl ONLY when the type has NO TYPE parameter (lifetime-only and
// const-only generics keep it, like a non-generic type; a TYPE param suppresses it -- trivial-copy specialization is
// unavailable when a param may not be Copy). Verified on rustc: struct T<A> -> 2 impls (clone + Copy), struct L<'a> /
// C<const N> / N -> 3 impls (TrivialClone + clone + Copy). Same &Item-param + destructured-Vec .iter() shape as
// item_derivable (the proven lowering pattern; a type param is one that is neither a lifetime nor a const).
// A `use` item -> rustc gives it a `{use#N}` def-path, numbered by a per-SCOPE use-only counter (other item kinds do not
// consume a use#). Top-level (crate-root) uses are filtered by the measure oracle (^{use#); nested uses -- in a fn/method
// body or a `mod` body -- are real def-paths (`{base}::{use#N}`) the driver must emit. Simple/glob uses are 1 use each
// (nested-group `use a::{b,c}` expands to 1+members, but none appear in ast.rs/hir.rs -> deferred).
fn item_is_use(it: &Item) -> bool {
    match &it.kind { ItemKind::Use(_u) => true, _ => false }
}
// How many `{use#N}` def-paths one `use` item yields: rustc numbers a nested-group use `use a::{b, c}` as 1 (the group
// node) + one per member (recursively) -- e.g. `use a::{b, c}` -> 3, `use a::{b::{c,d}, e}` -> 5. Simple/glob -> 1. Mirrors
// emit_privuse_tree's &UseTree recursion (the proven lowering shape).
fn use_defpath_count(ut: &UseTree) -> i64 {
    match &ut.kind {
        UseKind::Nested(kids) => { let mut c: i64 = 1; for k in kids.iter() { c = c + use_defpath_count(k); } c }
        _ => 1
    }
}
fn use_item_count(it: &Item) -> i64 {
    match &it.kind { ItemKind::Use(ut) => use_defpath_count(ut), _ => 0 }
}
fn item_has_typaram(it: &Item) -> bool {
    match &it.kind {
        ItemKind::Struct(g, _f) => { for gp in g.iter() { if !gp.is_lt && !gp.is_const { return true; } } false }
        ItemKind::TupleStruct(g, _t) => { for gp in g.iter() { if !gp.is_lt && !gp.is_const { return true; } } false }
        ItemKind::UnitStruct(g) => { for gp in g.iter() { if !gp.is_lt && !gp.is_const { return true; } } false }
        ItemKind::Enum(g, _v) => { for gp in g.iter() { if !gp.is_lt && !gp.is_const { return true; } } false }
        ItemKind::Union(g, _f) => { for gp in g.iter() { if !gp.is_lt && !gp.is_const { return true; } } false }
        _ => false
    }
}
fn emit_one_derive(cratename: &str, base: &str, dname: &str, has_copy: bool, sup_trivial: bool, impl_k: i64) -> i64 {
    let ip = impl_path(base, impl_k);
    if streq(dname, "Clone") {
        if has_copy && !sup_trivial { println!("CRATERES {}|{}", cratename, ip); let ip1 = impl_path(base, impl_k + 1); emit_derive_1(cratename, &ip1, "clone"); return impl_k + 2; }
        emit_derive_1(cratename, &ip, "clone"); return impl_k + 1;
    }
    if streq(dname, "Debug") { emit_derive_1(cratename, &ip, "fmt"); return impl_k + 1; }
    if streq(dname, "Hash") { emit_derive_1(cratename, &ip, "hash"); return impl_k + 1; }
    if streq(dname, "Default") { emit_derive_1(cratename, &ip, "default"); return impl_k + 1; }
    if streq(dname, "Copy") { println!("CRATERES {}|{}", cratename, ip); return impl_k + 1; }
    if streq(dname, "Eq") { emit_derive_1(cratename, &ip, "assert_fields_are_eq"); return impl_k + 1; }
    if streq(dname, "PartialOrd") { emit_derive_1(cratename, &ip, "partial_cmp"); return impl_k + 1; }
    if streq(dname, "Ord") { emit_derive_1(cratename, &ip, "cmp"); return impl_k + 1; }
    // PartialEq: a StructuralPartialEq MARKER impl (no method) THEN the PartialEq impl (::eq) -- 2 impls.
    if streq(dname, "PartialEq") { println!("CRATERES {}|{}", cratename, ip); let ip1 = impl_path(base, impl_k + 1); emit_derive_1(cratename, &ip1, "eq"); return impl_k + 2; }
    impl_k
}
fn emit_derive_list(cratename: &str, base: &str, dl: &str, sup_trivial: bool, impl_k: i64) -> i64 {
    let has_copy = str_contains(dl, "Copy");
    let by = dl.as_bytes(); let n = by.len(); let mut k = impl_k; let mut start: usize = 0; let mut i: usize = 0;
    while i < n {
        if by[i] == 44 { let nm = trim_sp(&dl[start..i]); if nm.len() > 0 { k = emit_one_derive(cratename, base, &nm, has_copy, sup_trivial, k); } start = i + 1; }
        i = i + 1;
    }
    if start < n { let nm = trim_sp(&dl[start..n]); if nm.len() > 0 { k = emit_one_derive(cratename, base, &nm, has_copy, sup_trivial, k); } }
    k
}
fn emit_derives(cratename: &str, base: &str, it: &Item, impl_k: i64) -> i64 {
    if !item_derivable(it) { return impl_k; }
    if attrs_cfg_off(&it.attrs) { return impl_k; }
    let mut k = impl_k;
    let sup_trivial = item_has_typaram(it);
    for a in it.attrs.iter() {
        if streq(&dump_path(&a.path), "derive") { k = emit_derive_list(cratename, base, &a.args, sup_trivial, k); }
    }
    k
}
fn tok_is_hash(t: &Tok) -> bool { match t { Tok::Hash => true, _ => false } }
fn tok_is_lbrack(t: &Tok) -> bool { match t { Tok::LBrack => true, _ => false } }
fn tok_is_rbrack(t: &Tok) -> bool { match t { Tok::RBrack => true, _ => false } }
fn tok_is_lparen(t: &Tok) -> bool { match t { Tok::LParen => true, _ => false } }
fn tok_is_rparen(t: &Tok) -> bool { match t { Tok::RParen => true, _ => false } }
fn tok_is_lbrace(t: &Tok) -> bool { match t { Tok::LBrace => true, _ => false } }
fn tok_is_rbrace(t: &Tok) -> bool { match t { Tok::RBrace => true, _ => false } }
fn tok_ident_is(t: &Tok, idn: &Vec<String>, s: &str) -> bool { match t { Tok::Ident(ix) => id_eq_raw(idn, *ix, s), _ => false } }
// CD-22 (rz-LIFT resolution): the cfg-eval-on-block-statement fix WITHOUT growing the Parser struct (a 23rd field
// mis-lowers rz's extern-crate emission -- the codegen-stability LIFT, pin-art CD-14/15/20). Runs on the TOKEN stream
// before the Parser: strips a cfg-OFF block statement `# [ cfg ( PRED ) ] { .. }` (feature/test/windows PRED) so its
// body uses never reach the AST. Item-level cfg (fn/struct/mod) is left to attrs_cfg_off; only a block (LBrace after `]`)
// is stripped here. A free fn on the token vecs (element-clone idiom; EMIT-path, no struct growth -> rz-safe, unlike any
// parse_stmt/field addition).
fn strip_cfg_off_block_toks(toks: &Vec<Tok>, offs: &Vec<i64>, idn: &Vec<String>) -> (Vec<Tok>, Vec<i64>) {
    let mut t: Vec<Tok> = Vec::new(); let mut o: Vec<i64> = Vec::new();
    let n = toks.len(); let mut i: usize = 0;
    while i < n {
        let mut stripped = false;
        // match `# [ cfg ( ...`
        if i + 4 < n { if tok_is_hash(&toks[i]) { if tok_is_lbrack(&toks[i+1]) { if tok_ident_is(&toks[i+2], idn, "cfg") { if tok_is_lparen(&toks[i+3]) {
            // the predicate's FIRST token (i+4): feature/test/windows -> OFF (not/any/unix/debug_assertions kept)
            let off = tok_ident_is(&toks[i+4], idn, "feature") || tok_ident_is(&toks[i+4], idn, "test") || tok_ident_is(&toks[i+4], idn, "windows");
            if off {
                // find the matching RParen for the LParen at i+3
                let mut d: i64 = 0; let mut j = i + 3; let mut pe: usize = 0; let mut pfound = false;
                while j < n { if tok_is_lparen(&toks[j]) { d = d + 1; } if tok_is_rparen(&toks[j]) { d = d - 1; if d == 0 { pe = j; pfound = true; j = n; } } if j < n { j = j + 1; } }
                if pfound { if pe + 2 < n { if tok_is_rbrack(&toks[pe+1]) { if tok_is_lbrace(&toks[pe+2]) {
                    // brace-match the block from pe+2 to its RBrace, then skip [i .. be]
                    let mut bd: i64 = 0; let mut k = pe + 2; let mut be: usize = 0; let mut bfound = false;
                    while k < n { if tok_is_lbrace(&toks[k]) { bd = bd + 1; } if tok_is_rbrace(&toks[k]) { bd = bd - 1; if bd == 0 { be = k; bfound = true; k = n; } } if k < n { k = k + 1; } }
                    if bfound { i = be + 1; stripped = true; }
                } } } }
            }
        } } } } }
        if !stripped { t.push(toks[i].clone()); o.push(offs[i]); i = i + 1; }
    }
    (t, o)
}
fn emit_unit(cratename: &str, base: &str, body: &str) -> Vec<String> {
    let (bt0, bo0, idn) = lex(body);
    let (bt, bo) = strip_cfg_off_block_toks(&bt0, &bo0, &idn);
    let mut sub = Parser { idents: idn, toks: bt, offs: bo, pos: 0, no_struct: false, expr_path: false, pending_gt: 0, in_cond: false, expand: false, mac_names: Vec::new(), mac_matchers: Vec::new(), mac_bodies: Vec::new(), mac_metavars: Vec::new(), mac_reps: Vec::new(), mac_depth: 0, rule_toks: Vec::new(), rep_els: Vec::new(), drop_stmt: false, stmt_attrs: Vec::new(), resolve: false, moddir: String::from(""), pend_move: false, pend_async: false };
    let k = sub.parse_krate();
    let mut impl_k: i64 = 0;
    let mut extern_k: i64 = 0;   // {extern#N}: a per-scope counter SEPARATE from impl_k (no numbering interaction)
    let mut use_k: i64 = 0;      // {use#N}: a per-unit use-only counter. Root-unit uses (base="") are filtered by the
    // measure oracle (^{use#); a NON-root unit IS a submodule (`mod cursor;` <- cursor.rs), so its top-level `use` items
    // are real `{base}::{use#N}` def-paths rustc emits and the driver must match (mirrors emit_mod_items 3957 for inline mods).
    for it in k.items.iter() {
        match &it.kind {
            ItemKind::Use(_ut) => {
                if base.len() > 0 { if !attrs_cfg_off(&it.attrs) { let cnt = use_item_count(it); let mut j: i64 = 0; while j < cnt { println!("CRATERES {}|{}::{{use#{}}}", cratename, base, use_k); use_k = use_k + 1; j = j + 1; } } }
            }
            ItemKind::Impl(_g, _tr, _ty, _inner) => {
                if !attrs_cfg_off(&it.attrs) {
                    let ipath = if base.len() == 0 { format!("{{impl#{}}}", impl_k) } else { format!("{}::{{impl#{}}}", base, impl_k) };
                    println!("CRATERES {}|{}", cratename, ipath);
                    emit_impl_methods(cratename, &ipath, it);
                    sub.emit_impl_macro_methods(cratename, &ipath, it);   // S2 slice 2: method-gen macros inside this impl
                    impl_k = impl_k + 1;
                }
            }
            ItemKind::ForeignMod(_inner) => {
                if !attrs_cfg_off(&it.attrs) {
                    let epath = extern_path(base, extern_k);
                    println!("CRATERES {}|{}", cratename, epath);
                    emit_foreign_members(cratename, &epath, it);
                    extern_k = extern_k + 1;
                }
            }
            ItemKind::Mod(_m) => {
                if !attrs_cfg_off(&it.attrs) {
                    let full = if base.len() == 0 { format!("{}", it.ident) } else { format!("{}::{}", base, it.ident) };
                    println!("CRATERES {}|{}", cratename, full);   // the mod NAME owner (replicates emit_crate_defpath's Mod name emission)
                    let mk = emit_mod_items(cratename, &full, it);   // PASS A (hand-written impls/uses/names) + PASS B (derives); returns the phase-2 start
                    emit_mod_mac_impls(cratename, &full, it, mk, &sub.mac_names, &sub.mac_bodies);   // PASS C-macro (mod-nested impl-rep macro calls, registry-driven)
                }
            }
            ItemKind::MacCall(_e) => { emit_crate_defpath(cratename, base, it); }
            _ => { emit_crate_defpath(cratename, base, it); }
        }
    }
    // S8 PORT (2026-08-13): rustc numbers phase-2 derives in THREE source-ordered tiers (converged sim, ast 562/562 +
    // hir 663/663 exact): B1 items with NO custom derive; B2a custom items WITH a separate pure-builtin #[derive] attr;
    // B2b custom items with builtins MIXED into the custom attr. Order B1 < B2a < B2b.
    // S8 (2026-08-14): GROUP 1 = items with NO custom derive OR a separate pure-builtin attr, in SOURCE order (the
    // merge of the old B1 + B2a tiers -- equivalent for ast/hir, source-correct for span where b1/B2a interleave).
    // GROUP 2 = items whose builtins are mixed into the custom attr (B2b), source order.
    // GROUP 1 also carries item-position macro-generated impls (impl_pos! Pos/Add/Sub + macro-struct derives) at the
    // MACRO CALL's SOURCE POSITION (verified span #93-130 between hand and the later group-1 derives). So the macro call is
    // handled INLINE here in source order, not in a trailing PASS-C.
    for it in k.items.iter() {
        match &it.kind {
            ItemKind::MacCall(e) => { impl_k = sub.emit_mac_impls(cratename, base, e, impl_k); }
            _ => { if !item_has_custom_derive(it) || item_has_pure_builtin_attr(it) { impl_k = emit_derives(cratename, base, it, impl_k); } }
        }
    }
    for it in k.items.iter() { if item_has_custom_derive(it) && !item_has_pure_builtin_attr(it) { impl_k = emit_derives(cratename, base, it, impl_k); } }
    // CD-3b: RETURN the DEFS records -- `crate|modbase|name|vis` per named non-Use/non-Impl item (the symbol+visibility
    // inventory the cross-unit privacy check consumes). Reuses the SAME parsed k (no reparse).
    let mut defs: Vec<String> = Vec::new();
    for it in k.items.iter() {
        match &it.kind {
            ItemKind::Use(_ut) => {}
            ItemKind::Impl(_g, _tr, _ty, _inner) => {}
            _ => { if it.ident.len() > 0 { if !attrs_cfg_off(&it.attrs) { defs.push(format!("{}|{}|{}|{}", cratename, base, it.ident, it.vis)); } } }
        }
    }
    defs
}
// CD-3b: everything before the LAST `::` of a path (the module path); "" if single-segment. Byte-scan a &str param.
fn path_but_last(p: &str) -> String {
    let by = p.as_bytes(); let mut last: usize = 0; let mut found = false; let mut i: usize = 0;
    while i + 1 < by.len() { if by[i] == 58 { if by[i+1] == 58 { last = i; found = true; } } i = i + 1; }
    if !found { return String::new(); }
    format!("{}", &p[0..last])
}
// CD-3b: a brought-in use path `<modpath>::<name>` in crate `cratename`: if the DEFS table has `cratename|modpath|name`
// with vis "priv" -> it is a private cross-unit item -> E0603 (`CRATEPRIV name`). CONSERVATIVE (Rule-14): only when the
// exact (crate, modpath, name) is a KNOWN private def; a not-found path (a dep-crate use, or genuinely absent -> CD-4)
// emits nothing here. nth_field on defs.iter() elements (a Vec-iter element, safe -- the borrow_conflict idiom).
// CD-3b + CD-4: resolve a brought-in use path `<modpath>::<name>` in `cratename` against the DEFS table:
//   found + vis priv -> CRATEPRIV name (E0603); NOT found but `modpath` IS a KNOWN module of the crate (some
//   crate|modpath|* def exists) -> CRATENO name (E0432 unresolved import). A not-found path into an UNKNOWN module
//   (a dep-crate use, or a nested/unmodeled path) emits nothing (Rule-14 conservative -- never false-flag).
fn use_check(cratename: &str, fullpath: &str, defs: &Vec<String>) {
    let name = last_seg(fullpath);
    let modp = path_but_last(fullpath);
    let mut found = false;
    let mut knownmod = false;
    for d in defs.iter() {
        if streq(&nth_field(d, 0), cratename) { if streq(&nth_field(d, 1), &modp) {
            knownmod = true;
            if streq(&nth_field(d, 2), &name) {
                found = true;
                if streq(&nth_field(d, 3), "priv") { println!("CRATEPRIV {}", name); }
            }
        } }
    }
    if !found { if knownmod { println!("CRATENO {}", name); } }
}
// Mirrors the frozen emit_privuse_tree: build the full path per Simple leaf, recurse into Nested, run priv_check.
fn check_use_tree(cratename: &str, base: &str, ut: &UseTree, defs: &Vec<String>) {
    let pp = dump_path(&ut.prefix);
    let full = if base.len() == 0 { pp } else { format!("{}::{}", base, pp) };
    match &ut.kind {
        UseKind::Simple(_r) => { use_check(cratename, &full, defs); }
        UseKind::Nested(kids) => { for c in kids.iter() { check_use_tree(cratename, &full, c, defs); } }
        _ => {}
    }
}
fn unit_privacy(cratename: &str, base: &str, body: &str, defs: &Vec<String>) {
    let (bt, bo, idn) = lex(body);
    let mut sub = Parser { idents: idn, toks: bt, offs: bo, pos: 0, no_struct: false, expr_path: false, pending_gt: 0, in_cond: false, expand: false, mac_names: Vec::new(), mac_matchers: Vec::new(), mac_bodies: Vec::new(), mac_metavars: Vec::new(), mac_reps: Vec::new(), mac_depth: 0, rule_toks: Vec::new(), rep_els: Vec::new(), drop_stmt: false, stmt_attrs: Vec::new(), resolve: false, moddir: String::from(""), pend_move: false, pend_async: false };
    let k = sub.parse_krate();
    for it in k.items.iter() {
        match &it.kind {
            ItemKind::Use(ut) => { check_use_tree(cratename, base, ut, defs); }
            _ => {}
        }
    }
}
// PASS 2: re-split the image + run the cross-unit privacy check per unit against the full DEFS table.
fn emit_crate_privacy(whole: &str, defs: &Vec<String>) {
    let lines = split_lines(whole);
    let mut cur_crate = String::new(); let mut cur_base = String::new(); let mut body = String::new(); let mut have = false;
    let mut from_file = false;
    for ln in lines.iter() {
        if is_unit_header(ln) {
            if have { unit_privacy(&cur_crate, &cur_base, &body, defs); }
            cur_crate = unit_attr(ln, "crate=");
            let path = unit_attr(ln, "path=");
            let mut is_root = streq(&unit_attr(ln, "root="), "1");
            if streq(&unit_attr(ln, "kind="), "dep") { is_root = true; }
            cur_base = mod_base_from_path(&path, is_root);
            let filep = unit_attr(ln, "file=");
            if filep.len() > 0 { body = std::fs::read_to_string(&filep).unwrap(); from_file = true; } else { body = String::new(); from_file = false; }
            have = true;
        } else {
            if have { if !from_file { body.push_str(ln); body.push_str("\n"); } }
        }
    }
    if have { unit_privacy(&cur_crate, &cur_base, &body, defs); }
}
fn emit_crate(whole: &str) {
    let lines = split_lines(whole);
    // PASS 1: def-paths (emit_unit prints CRATERES) + accumulate the DEFS table across units.
    let mut defs: Vec<String> = Vec::new();
    let mut cur_crate = String::new();
    let mut cur_base = String::new();
    let mut body = String::new();
    let mut have = false;
    let mut from_file = false;
    for ln in lines.iter() {
        if is_unit_header(ln) {
            if have { let ud = emit_unit(&cur_crate, &cur_base, &body); for d in ud { defs.push(d); } }
            cur_crate = unit_attr(ln, "crate=");
            let path = unit_attr(ln, "path=");
            // CD-5: a kind=dep unit IS its dependency crate's ROOT (base ""). root=1 OR kind=dep -> root base.
            let mut is_root = streq(&unit_attr(ln, "root="), "1");
            if streq(&unit_attr(ln, "kind="), "dep") { is_root = true; }
            cur_base = mod_base_from_path(&path, is_root);
            // CD-7: a `file=<path>` unit reads its body from a REAL .rs file on disk (fs::read_to_string, the proven
            // resolve-mode idiom) instead of inline content -- the crate driver over real separate files, no argv needed.
            let filep = unit_attr(ln, "file=");
            if filep.len() > 0 { body = std::fs::read_to_string(&filep).unwrap(); from_file = true; } else { body = String::new(); from_file = false; }
            have = true;
        } else {
            if have { if !from_file { body.push_str(ln); body.push_str("\n"); } }
        }
    }
    if have { let ud = emit_unit(&cur_crate, &cur_base, &body); for d in ud { defs.push(d); } }
    emit_crate_privacy(whole, &defs);
}
fn emit_resolve(k: &Krate) {
    // Real item def-paths only. `use` imports are NOT emitted: rustc allocates internal `{use#K}` DefIds (a group +
    // one per nested member -- e.g. `use a::{f,g}` -> {use#1,2,3}), a bookkeeping counter that is not durable
    // name-resolution content (the derive-brittleness lesson: do not couple the gate to rustc-internal allocation).
    // The harness filters `{use#` from the rustc oracle to match. Use RESOLUTION is validated by the PRIVUSE set.
    for it in k.items.iter() { emit_item_defpath("", it); }
}
// Emit `RESPATH base::ident` for a named item and recurse into an inline module's items to any depth (a::b::f).
// `use` items emit nothing. &Item + &str PARAM base (the dump idiom); recursion mirrors dump_item.
fn emit_item_defpath(base: &str, it: &Item) {
    if streq(&it.vis, "extern") { return; }   // a synthetic extern-crate mod: its defs live in the dep crate's DefId space, NOT this crate's def-paths (rustc's hir-tree of the local crate excludes them)
    match &it.kind {
        ItemKind::Use(_ut) => {}
        _ => {
            if it.ident.len() > 0 {
                let full = if base.len() == 0 { format!("{}", it.ident) } else { format!("{}::{}", base, it.ident) };
                println!("RESPATH {}", full);
                match &it.kind {
                    ItemKind::Mod(inner) => { for inr in inner.iter() { emit_item_defpath(&full, inr); } }
                    _ => {}
                }
            }
        }
    }
}

// REFUSE-CLEANLY (locales/sem0-refuse-cleanly, C-SEM0-UNTERMINATED-GROUP). Returns 0 balanced, 1 unmatched
// closer, 2 unclosed at EOF. ONE check at the entry rather than an EOF guard in each of the 31 delimiter-scanning
// loops: those loops make no progress on Tok::End, so the rustc build HANGS and the rz build SIGSEGVs on input as
// small as `fn f() {`. Adding `Tok::End => break` to parse_block + capture_group DID fix the hang under rustc, but
// L6 could then no longer compile this file at all (it reported "no method named parse_attrs" 400 lines away) --
// another L6 parse desync, family C-L6-ELSEIF-CHAIN-ARM-DROP. So the loops are left untouched and malformed input
// is refused BEFORE parsing, which is smaller and is what the telos asks for.
fn delim_balance(toks: &Vec<Tok>) -> i32 {
    let mut dep = 0;
    let mut ti = 0;
    while ti < toks.len() {
        let t = toks[ti].clone();
        match t {
            Tok::LParen => { dep = dep + 1; }
            Tok::LBrack => { dep = dep + 1; }
            Tok::LBrace => { dep = dep + 1; }
            Tok::RParen => { dep = dep - 1; }
            Tok::RBrack => { dep = dep - 1; }
            Tok::RBrace => { dep = dep - 1; }
            _ => {}
        }
        if dep < 0 { return 1; }
        ti = ti + 1;
    }
    if dep > 0 { return 2; }
    0
}
fn main() {
    // AST-10: whole-file mode. Read a real .rs source from stdin; if non-empty, parse the WHOLE crate and dump its AST
    // (the crate-level differential vs rustc -Zunpretty=ast-tree). Empty stdin (/dev/null) -> the embedded-witness
    // differential below. Portable std Rust: rustc reads stdin natively; rzrs lowers read_to_string to syscall(0)
    // (L6 STDIN-READ), so the SAME source dual-compiles and BOOTSTRAP holds.
    let mut whole = String::new();
    std::io::stdin().read_to_string(&mut whole).unwrap();
    // S2 mode marker: a leading `//@S2EXPAND` line enables macro expansion (the S1 gate omits it, keeping the
    // unexpanded goldens; the S2 harness prepends it). It is a `//` line comment, so the lexer skips it -- we only
    // sniff its bytes to set the flag, leaving the source unchanged (spans shift equally on both builds, so the
    // rz==rc bootstrap holds; the EXPANSION compare strips spans). Byte-compare loop to stay in the rzrs subset.
    let mut xp = false;
    let mut rp = false;
    if whole.len() >= 11 {
        let wb = whole.as_bytes();
        // `//@S2EXPAND` = 47 47 64 83 50 69 88 80 65 78 68 (compare raw bytes; matches the lexer's known-good idiom).
        if wb[0] == 47 && wb[1] == 47 && wb[2] == 64 && wb[3] == 83 && wb[4] == 50 && wb[5] == 69 && wb[6] == 88 && wb[7] == 80 && wb[8] == 65 && wb[9] == 78 && wb[10] == 68 { xp = true; }
    }
    if whole.len() >= 12 {
        let wb = whole.as_bytes();
        // S3 mode marker `//@S3RESOLVE` = 47 47 64 83 51 82 69 83 79 76 86 69 -> emit the def-path map instead of the AST dump.
        if wb[0] == 47 && wb[1] == 47 && wb[2] == 64 && wb[3] == 83 && wb[4] == 51 && wb[5] == 82 && wb[6] == 69 && wb[7] == 83 && wb[8] == 79 && wb[9] == 76 && wb[10] == 86 && wb[11] == 69 { rp = true; }
    }
    let mut hrp = false;
    if whole.len() >= 11 {
        let wb = whole.as_bytes();
        // S4 HIR-6 mode marker `//@S4HIRRES` = 47 47 64 83 52 72 73 82 82 69 83 -> emit the Res/DefId bridge map.
        if wb[0] == 47 && wb[1] == 47 && wb[2] == 64 && wb[3] == 83 && wb[4] == 52 && wb[5] == 72 && wb[6] == 73 && wb[7] == 82 && wb[8] == 82 && wb[9] == 69 && wb[10] == 83 { hrp = true; }
    }
    let mut hp = false;
    if whole.len() >= 8 {
        let wb = whole.as_bytes();
        // S4 mode marker `//@S4HIR` = 47 47 64 83 52 72 73 82 -> emit the HIR dump (`-Zunpretty=hir` source form).
        // NOTE: `//@S4HIRRES` shares this 8-byte prefix, so hrp is checked FIRST in the dispatch below.
        if wb[0] == 47 && wb[1] == 47 && wb[2] == 64 && wb[3] == 83 && wb[4] == 52 && wb[5] == 72 && wb[6] == 73 && wb[7] == 82 { hp = true; }
    }
    let mut tp = false;
    if whole.len() >= 10 {
        let wb = whole.as_bytes();
        // S5 mode marker `//@S5TYPED` = 47 47 64 83 53 84 89 80 69 68 -> emit the typed-HIR dump (`-Zunpretty=hir,typed`).
        if wb[0] == 47 && wb[1] == 47 && wb[2] == 64 && wb[3] == 83 && wb[4] == 53 && wb[5] == 84 && wb[6] == 89 && wb[7] == 80 && wb[8] == 69 && wb[9] == 68 { tp = true; }
    }
    let mut mp = false;
    if whole.len() >= 9 {
        let wb = whole.as_bytes();
        // S6 mode marker `//@S6MONO` = 47 47 64 83 54 77 79 78 79 -> emit the collected mono-item set (`-Zprint-mono-items`).
        if wb[0] == 47 && wb[1] == 47 && wb[2] == 64 && wb[3] == 83 && wb[4] == 54 && wb[5] == 77 && wb[6] == 79 && wb[7] == 78 && wb[8] == 79 { mp = true; }
    }
    let mut kp = false;
    if whole.len() >= 10 {
        let wb = whole.as_bytes();
        // S7 mode marker `//@S7CHECK` = 47 47 64 83 55 67 72 69 67 75 -> emit the ACCEPT/REJECT verdict.
        if wb[0] == 47 && wb[1] == 47 && wb[2] == 64 && wb[3] == 83 && wb[4] == 55 && wb[5] == 67 && wb[6] == 72 && wb[7] == 69 && wb[8] == 67 && wb[9] == 75 { kp = true; }
    }
    let mut cp = false;
    if whole.len() >= 8 {
        let wb = whole.as_bytes();
        // S3 crate-driver marker `//@CRATE` = 47 47 64 67 82 65 84 69 -> ingest the delimited multi-unit image.
        if wb[0] == 47 && wb[1] == 47 && wb[2] == 64 && wb[3] == 67 && wb[4] == 82 && wb[5] == 65 && wb[6] == 84 && wb[7] == 69 { cp = true; }
    }
    let mut s9p = false;
    if whole.len() >= 5 {
        let wb = whole.as_bytes();
        // S9 codegen marker `//@S9` = 47 47 64 83 57 -> lower the AST to executable rzir (emit_s9).
        if wb[0] == 47 && wb[1] == 47 && wb[2] == 64 && wb[3] == 83 && wb[4] == 57 { s9p = true; }
    }
    if whole.len() > 0 {
        if cp { emit_crate(&whole); return; }   // S3 crate driver: multi-unit delimited image -> cross-crate resolution map
        let (toks, offs, idn) = lex(&whole);
        let bal = delim_balance(&toks);
        if bal == 1 { eprintln!("error: unmatched closing delimiter"); std::process::exit(1); }
        if bal == 2 { eprintln!("error: unclosed delimiter at end of file"); std::process::exit(1); }
        let mut p = Parser { idents: idn, toks: toks, offs: offs, pos: 0, no_struct: false, expr_path: false, pending_gt: 0, in_cond: false, expand: xp, mac_names: Vec::new(), mac_matchers: Vec::new(), mac_bodies: Vec::new(), mac_metavars: Vec::new(), mac_reps: Vec::new(), mac_depth: 0, rule_toks: Vec::new(), rep_els: Vec::new(), drop_stmt: false, stmt_attrs: Vec::new(), resolve: rp, moddir: String::from(""), pend_move: false, pend_async: false };
        let ast = p.parse_krate();
        if s9p { emit_s9(&ast); return; }   // S9: lower to executable rzir (run == rustc)
        if kp { emit_check(&ast); return; }   // S7: checker-tier ACCEPT/REJECT verdict
        if mp { emit_mono(&ast); return; }   // S6: monomorphization collection (`-Zprint-mono-items`)
        if tp { emit_typed(&ast); return; }   // S5: AST -> typed HIR (`-Zunpretty=hir,typed`)
        if hrp { emit_hirres(&ast); return; }   // S4 HIR-6: the S3->S4 Res/DefId bridge (fn-call path -> Def(Fn, name))
        if hp { emit_hir(&ast); return; }   // S4: AST -> HIR lowering (`-Zunpretty=hir` source form)
        if rp { emit_resolve(&ast); emit_privuse(&ast); emit_dupuse(&ast); return; }   // S3: def-paths + privacy + unresolved + duplicate-import
        println!("{}", dump(&ast));
        return;
    }
    // Embedded witnesses (the per-construct differential). Each maps in order to a stanza in goldens/s1-ast.txt.
    let witnesses = [
        "pub const N: u32 = 5;",
        "pub const X: a::b::C = 5;",
        "pub const A: &u32 = 0;",
        "pub const B: &mut u32 = 0;",
        "pub const C: *const u32 = 0;",
        "pub const D: *mut u32 = 0;",
        "pub const E: [u32; 4] = 0;",
        "pub const F: [u32] = 0;",
        "pub const G: (u32, u8) = 0;",
        "pub const H: Vec<u32> = 0;",
        "pub const I: Option<a::B> = 0;",
        "pub const AA: i32 = 1 + 2 * 3;",
        "pub const BB: i32 = -5;",
        "pub const CC: i32 = (1 + 2) * 3;",
        "pub const PP: i32 = a::B;",
        "pub const QQ: bool = 1 == 2;",
        "pub struct S { pub a: u32, b: bool }",
        "pub enum E { A, B(u32), C { x: u8 } }",
        "pub fn add(a: u32, b: u32) -> u32 { a + b }",
        "pub fn f(x: u32) -> u32 { let y: u32 = x + 1; y }",
        "pub fn nt(x: u32) -> u32 { let z = x + 2; z }",
        "pub fn g(c: u32) -> u32 { let _: u32 = 1; let (a, b): (u32, u32) = c; a }",
        "pub fn h(v: Vec<u32>, s: Foo) -> u32 { let n: u32 = v.len(); let z: u32 = g(n); let m: u32 = s.x; let k: u32 = v[0]; k }",
        "pub fn i(a: u32) -> u32 { if a == 1 { 2 } else { 3 } }",
        "pub fn j(o: Foo) -> u32 { match o { Bar(x) => x, Baz => 0, _ => 9 } }",
        "pub fn n(x: u32) -> u32 { match x { 1 => 10, 2 | 3 => 20, _ => 0 } }",
        "pub fn gp<T: Clone>(x: T) -> T { x }",
        "pub struct W<T> { v: T }",
        "pub type A = u32;",
        "use a::b::C;",
        "pub mod m { pub const K: u32 = 1; }",
        "impl S { pub fn m(&self) -> u32 { 0 } }",
        "impl<T> W<T> { pub fn get(&self) -> u32 { 1 } }",
        "#[derive(Clone)] pub struct D { x: u32 }",
        "#[inline] pub fn si() -> u32 { 0 }",
        "#[stable(feature = \"x\", since = \"1.0\")] pub fn st() -> u32 { 0 }",
        "pub const MSG: &str = \"hi\";",
        "pub const C: char = 'a';",
        "pub const R: u32 = &x;",
        "pub const Dr: u32 = *p;",
        "pub const K: u64 = x as u64;",
        "pub const F: u32 = |x| x;",
        "pub const A: u32 = [1, 2, 3];",
        "pub const T: u32 = (1, 2);",
        "pub const RG: u32 = 0..10;",
        "pub fn nr(x: u32) { let y: u32 = x; }",
        "pub fn wl(a: u32) -> u32 { while a { let x: u32 = 1; } a }",
        "pub fn lp(a: u32) -> u32 { loop { let y: u32 = 2; } }",
        "pub fn fl(v: Vec<u32>) -> u32 { for x in v { let z: u32 = x; } 0 }",
        "pub fn asg(x: u32) -> u32 { let y: u32 = 0; y = x; y += 1; y }",
        "pub fn mk(a: u32) -> S { S { x: a, y: 1 } }",
        "pub fn cnd(a: u32) -> u32 { if a == 1 { let s: S = S { x: a }; a } else { 0 } }",
        "pub trait Tr { fn m(&self) -> u32; }",
        "impl Tr for S { fn m(&self) -> u32 { 0 } }",
        "#[derive(Clone)] pub enum E2 { A, B }",
        "#[repr(C)] pub type Al = u32;",
        "pub struct Ref<'a> { r: &'a str }",
        "pub(crate) const VC: u32 = 7;",
        "pub fn tq(o: Foo) -> u32 { o.next()? }",
        "pub fn pp(k: E) -> u32 { match k { E::A => 1, E::B(x) => x, _ => 0 } }",
        "pub const BT: bool = true;",
        "pub fn sp(p: P) -> u32 { match p { P { x, y } => x, Q { a: n, .. } => n, _ => 0 } }",
        "pub fn il(o: Foo) -> u32 { if let Some(c) = o.next() { c } else { 0 } }",
        "pub fn wc<T>(x: T) -> T where T: Clone { x }",
        "pub fn tf() -> u32 { Vec::<u32>::new() }",
        "pub fn mt(v: V) -> u32 { v.parse::<u32>() }",
        "pub fn lo(a: bool, b: bool) -> bool { a || b && a }",
        "pub fn cp(c: char) -> bool { match c { 'a' | 'b' => true, _ => false } }",
        "pub fn rp(c: char) -> bool { match c { '0'..='9' => true, _ => false } }",
        "pub fn rb(o: Foo) -> u32 { match o { Some(ref x) => 1, _ => 0 } }",
        "pub fn cmp(x: u32, y: u32) -> bool { x < y }",
        "pub fn rt(x: u32) -> u32 { if x > 0 { return x; } return 0; }",
        "pub fn op(a: u32, b: u32) -> u32 { a | b ^ a & b % a }",
        "pub fn tc() -> u32 { let g = |x: u32| x; 0 }",
        "pub fn ab(n: u32) -> u32 { match n { x @ 1..=5 => x, _ => 0 } }",
        "pub fn it(x: u32) -> impl Iterator { x }",
        "pub fn dt(x: Box<dyn Trait>) -> u32 { 0 }",
        "pub const CM: u32 = 5; /* c */",
        "pub fn ca(x: u32) -> u32 { let mut y = 0; y += x; y -= x; y *= 2; y /= 2; y %= 3; y &= x; y |= x; y ^= x; y <<= 1; y >>= 1; y }",
        "use a::b::c;",
        "use a::b as f;",
        "use a::*;",
        "use a::b::{c, d as e};",
        "pub const fn cf(x: u32) -> u32 { x + 1 }",
        "pub fn ub(p: u32) -> u32 { let y = unsafe { p }; y }",
        "pub fn cl() { let f = || { 1 }; let g = || 2; }",
        "pub fn ti(t: T) -> u32 { t.0 + t.1 }",
        "pub fn br(a: bool) -> u32 { loop { if a { break 3; } continue; } }",
        "pub fn le(o: Option<u32>) -> u32 { let Some(x) = o else { return 0; }; x }",
        "pub fn mc(b: bool) { let v = vec![1, 2]; assert!(b); foo!(a, b); }",
        "pub fn ll(x: bool) { 'a: loop { if x { break 'a; } continue 'a; } }",
        "pub struct Fp { f: fn(u32, bool) -> u8, g: PhantomData<fn(I)> }",
        "pub trait Pos { const ZERO: usize; fn to_usize(&self) -> usize; }",
        "pub fn sp(s: &[u32]) -> u32 { match s { [a, ..] => *a, [x, y] => x, [] => 0 } }",
        "pub fn mg(n: u32) -> u32 { match n { x if x > 10 => 2, _ => 3 } }",
        "pub fn su(base: C, k: u32) -> C { C { level: k, ..base } }",
        "pub fn fl(r: f64) -> f64 { r * 1.5 + 0.0 }",
        "pub fn ft(g: impl Fn(u8) -> u8, h: Box<dyn FnMut()>) -> u8 { g(1) }",
        "pub fn mv(base: u32) -> u32 { let g = move |x| x + base; g(2) }",
        "pub fn rw() -> u32 { let a = r\"C:\\p\"; let b = r#\"x \"q\" y\"#; 0 }",
        "pub fn by() -> u8 { let c = b'A'; let s = b\"xy\"; c }",
        "pub fn nl() -> u32 { let h = 0xff; let bn = 0b1010; let u = 1_000u32; let f = 1.5f64; h + bn }",
        "pub fn ld(c: bool) -> u32 { let x: u32; let y; if c { 1 } else { 2 } }",
        "pub fn ue() -> Result<(), E> { let u = (); g(()); Ok(()) }",
        "pub fn rg(a: A) -> u32 { let p = a[1..]; let q = a[..2]; let r = a[..]; let s = a[1..3]; 0 }",
        "unsafe impl Tr for S {}",
        "unsafe trait Ut { fn m(&self); }",
        "pub fn nv() -> ! { loop {} }",
        "pub struct Ar { a: [u8; N], b: [u8; 4], c: [u8; M * 2] }",
        "pub async fn af() {}",
        "pub fn ab() { let x = async { 1 }; let f = async move |y| y; }",
        "pub fn dr(x: T) -> u32 { let p = &&x; let d = x.0.1; if x.0 { 1 } else { 0 } }",
        "pub fn lp(s: S) -> u32 { match s { \"hi\" => 1, true => 2, false => 3, 7 => 4, _ => 5 } }",
        "pub fn mm(x: u32) -> u32 { match x { | 1 | 2 => 1, _ => 2 } }",
        "pub fn mk(y: u32) -> bool { matches!(y, A | B if y > 0) }",
        "pub enum Ed { A = 1, B = 1 + 2, C, D(u32) = 4 }",
        "pub const QT: <X as Trait>::Assoc = 0;",
        "pub fn qe() -> u32 { <T as Trait>::method() }",
        "pub fn cmp(a: u32, b: u32) -> bool { a < b }",
        "pub fn ib() -> u32 { fn inner() -> u32 { 1 } struct Local; let y = 2;; inner() }",
        "#[repr(align(8))] pub struct Al { a: u8 }",
        "#[doc = \"d\"] pub struct Dv { w: u32 }",
        "#[cfg_attr(test, derive(Debug))] pub struct Ca { a: u8 }",
        "pub fn pa(#[allow(unused)] x: u8) -> u8 { x }",
        "pub trait It { type Item; }",
        "pub trait Bd { type Out: Clone + Send; }",
        "pub trait A: B + C {}",
        "pub type Alias<T> = Vec<T>;",
        "impl Tr for S { type Out = u8; }",
        "pub const G: bool = |&n| n == 2;",
        "pub const H: u8 = |(a, b)| a;",
        "pub const I: u8 = |_| 0;",
        "fn mm(v: u8) { match v { ref mut y @ Some(_) => 1, mut z @ 1..=5 => 2, ref w @ None => 3, x @ Other(_) => 4, _ => 0 }; }",
        "fn nn(v: i8) { match v { -1 => 1, -2.5 => 2, -5..=-1 => 3, 0..=10 => 4, _ => 0 }; }",
        "fn aw() { let a = x.await; let b = foo().await?; let c = a.b.await.c(); }",
        "pub trait Tg { type Item<'a>; type Iter<'a>: Iterator; }",
        "thread_local! { static X: u8 = 0; }",
        "my_mac!(a, b);",
        "fn hb<F: for<'a> Fn(&'a i32)>(x: Box<dyn for<'a> Fn(&'a str)>) { for y in x {} }",
        "struct Cg<const N: usize = 3, T: Clone>(T);",
        "struct Ob<'a, 'b: 'a, T: 'a = u8> { x: T }",
        "fn ow<'a, 'b>() where 'a: 'b, T: 'static {}",
        "macro_rules! foo { () => {} }",
        "macro_rules! bar { (a) => { 1 }; (x: expr) => { x }; }",
        "fn ri() { let x = r#type; let y = r#match + 1; }",
        "fn tf() { let v = x.iter().collect::<Vec<u8>>(); let w = a.m::<A<B>>(); }",
        "fn cs() { let a = c\"abc\"; let b = cr\"x\"; }",
        "fn lp() { let x = ::std::u8::MAX; let y = ::core::mem::size_of::<T>(); }",
        "impl S { fn pm(mut p: u8) {} fn sr<'a>(&'a self, k: &K, m: u8) -> u8 { 0 } }",
        "fn st() { #[cfg(test)] let x = 1; #[allow(x)] g(); }",
        "fn lc() { if let Some(x) = a && let Some(y) = b {} if let Some(x) = a && x > 0 {} let z = a && b; }",
        "fn ex() { let a = [0; 4]; let b = async move { 1 }; let c = const { 2 }; let d = &raw const x; let e = &raw mut y; }",
        "impl !Send for A {}",
        "auto trait Foo {}",
        "unsafe extern \"C\" { fn f(); }",
        "extern crate foo;",
        "pub const Ve: extern \"C\" fn(u8, ...) = x;",
        "type Tg2 = Foo<3>; type Tg3 = Bar<{ N + 1 }>;",
        "fn dl(x: &dyn A + B + 'a) {}",
        "struct Fa { #[cfg(a)] b: u8, c: u16 }",
        "enum Ea { #[cfg(x)] V, W(u8) }",
        "struct Ta(#[allow(x)] u8, u16);",
        "fn cb() { let g = |x| -> u8 { x + 1 }; let k = || -> i32 { 5 }; let h = |y| y; }",
        "extern \"C\" fn ef() {}",
        "pub unsafe extern \"C\" fn eg(x: u8) -> u8 { x }",
        "fn ge() { let x: Vec<u8>=b; let n: Vec<Vec<u8>>=m; let z = a >= c; }",
        "static Sa: u8 = 1; static mut Sb: u32 = 2; pub static Sc: bool = true;",
        "union Un { a: u8, b: u16 } pub union Vn<T> { x: T, y: u32 }",
        "fn mm(c: char) -> bool { matches!(c, '0'..='9') }",
        "fn uc() { let a = '\\u{1F}'; let b = '\\u{7FF}'; let c = 'x'; let d = '\\n'; }",
        "fn rp(x: i64) { match x { i64::MIN..=1 => {}, 5..=A::B => {}, i64::MIN..=-1 => {}, _ => {} } }",
        "fn sr() { let a = S { a, .. }; let b = S { x: 1, .. }; let c = S { }; let d = S { m, ..base }; }",
        "fn bp(b: B) { let box y = b; }",
        "fn bd<T: ?Sized>(x: T) where T: Copy + ?Sized {} pub trait Tb: ?Sized {} struct Sd<T: ?Sized>(T);",
        "impl S { fn c(mut self) {} fn d(mut self) -> u8 { 1 } fn e(self) {} }",
        "fn hx() { let a = '\\x41'; let b = '\\x7F'; let c = 'y'; let d = '\\n'; }",
        "struct Sc<const N: usize = 8>(u8); fn fc<const M: u32 = 4, T = u16>() {}",
        "type Aw<T> = u8 where T: Copy; type Bw = i32;",
        "fn tp((a, b): (u8, u8)) {} fn tq(x: u8, [p, q]: [u8; 2], Q(z): Q) {}",
        "fn ab(x: Box<dyn Iterator<Item: Clone>>, y: Box<dyn Iterator<Item = u8>>) {}",
        "#![no_std] #![feature(x)] fn ia() {}",
        "#!/usr/bin/env rust\nfn sh(x: u8) -> u8 { x }",
        "fn hr(f: for<'a> fn(&'a u8) -> u8) {} fn hb<T: for<'a> Trait<'a>>() {}",
        "impl S { fn a(self: Box<Self>) {} fn b(self: Rc<Self>) -> u8 { 1 } fn c(&self) {} fn d(self) {} }",
        "fn cb() { const { 1 } } fn cc() { let a = const || 5; let b = const move || 6; const { 2 }; }",
        "fn rg(x: i64) { match x { 1.. => 9, ..5 => 8, ..=5 => 7, X.. => 6, 1..=5 => 5, _ => 0 } }",
        "fn ga<#[may_dangle] T>() {} struct Gs<#[a] 'a, #[b] const N: usize, #[c] T>(T);",
        "fn rr(x: &&i32, y: &&mut u8, z: &&&u8) {} fn rp() { let &&a = x; }",
        "fn uf() { let a = 1.; let b = 2. + 3.; let c = x.0; let d = 1..2; } fn café() { let naïve = 1; }",
        "default impl Foo for Bar {} extern \"C\" { fn v(x: u8, ...); } trait Ta = Bar + Baz; trait Tb<T> = C<T> where T: Clone;",
        "pub const unsafe extern \"C\" fn cm(x: u8) -> u8 { x } const extern \"C\" fn cn() {}",
        "struct Pv { pub a: u8, pub(crate) b: u16, pub(in c::d) e: u8, f: bool }",
        "fn mp() { match x { foo!(a, b) => {}, _ => {} } let m![1, 2] = y; }",
        "fn ma() { match x { #[cfg(a)] Foo => 1, #[allow(x)] Bar => 2, _ => 0 }; }",
        "fn nb<T: !Send>() where T: Clone + !Sync {}",
        "fn wa<T, U>() where #[cfg(a)] T: Clone, U: Send {}",
        "fn mt() { x.method::<'a, T, 3>(); y.n::<{ N }>(); }",
        "fn ac(x: T<Item = 3, Other = u8>) {}",
    ];
    for src in witnesses.iter() {
        let (toks, offs, idn) = lex(src);
        let mut p = Parser { idents: idn, toks: toks, offs: offs, pos: 0, no_struct: false, expr_path: false, pending_gt: 0, in_cond: false, expand: false, mac_names: Vec::new(), mac_matchers: Vec::new(), mac_bodies: Vec::new(), mac_metavars: Vec::new(), mac_reps: Vec::new(), mac_depth: 0, rule_toks: Vec::new(), rep_els: Vec::new(), drop_stmt: false, stmt_attrs: Vec::new(), resolve: false, moddir: String::from(""), pend_move: false, pend_async: false };
        let ast = p.parse_krate();
        println!("{}", dump(&ast));
    }
}
