mod ospan {
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
#[derive(Copy,Clone)] pub struct Span { pub lo: u32, pub hi: u32 }
pub fn sp(lo:u32, hi:u32)->Span{ Span{lo:lo, hi:hi} }
pub fn sp_to(a:&Span, b:&Span)->Span{ Span{lo:a.lo, hi:b.hi} }
pub fn sp_len(s:&Span)->u32{ s.hi - s.lo }
pub fn u32s(n:u32)->String{ if n==0 {return String::from("0");} let mut v=n; let mut s=String::from(""); while v>0 {let d=(v%10) as u32;let c=(48+d) as u8 as char;let mut t=String::from("");t.push(c);t.push_str(&s);s=t;v=v/10;} s }
pub fn sp_str(s:&Span)->String{ let mut r=u32s(s.lo); r.push_str(".."); r.push_str(&u32s(s.hi)); r }
}
mod oerrors {
    use crate::ospan::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
#[derive(Copy,Clone)] pub enum Level { Error, Warning, Note }
pub struct Diag { pub level: Level, pub msg: String, pub lo: u32, pub hi: u32 }
pub fn err(msg:String, lo:u32, hi:u32)->Diag{ Diag{level:Level::Error, msg:msg, lo:lo, hi:hi} }
pub fn lvl_name(l:&Level)->String{ match l { Level::Error=>String::from("error"), Level::Warning=>String::from("warning"), Level::Note=>String::from("note") } }
pub fn is_error(d:&Diag)->bool{ match d.level { Level::Error=>true, _=>false } }
pub fn render(d:&Diag)->String{ let mut s=lvl_name(&d.level); s.push_str("["); s.push_str(&crate::ospan::sp_str(&crate::ospan::sp(d.lo, d.hi))); s.push_str("]: "); s.push_str(&d.msg); s }
}
mod oindex {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
#[derive(Copy,Clone)] pub struct HirId { pub v: u32 }
#[derive(Copy,Clone)] pub struct LocalId { pub v: u32 }
#[derive(Copy,Clone)] pub struct Temp { pub v: u32 }
pub fn hid(v:u32)->HirId{ HirId{v:v} }
pub fn lid(v:u32)->LocalId{ LocalId{v:v} }
pub fn tid(v:u32)->Temp{ Temp{v:v} }
pub fn hidx(h:&HirId)->u32{ h.v }
pub fn lidx(l:&LocalId)->u32{ l.v }
pub fn tidx(t:&Temp)->u32{ t.v }
// IndexVec: a Vec addressed by a typed index (the crate's defining shape)
pub struct TempVec { pub raw: Vec<i64> }
pub fn tv_new()->TempVec{ TempVec{raw:Vec::new()} }
pub fn tv_push(v:&mut TempVec, x:i64)->Temp{ let i=v.raw.len() as u32; v.raw.push(x); tid(i) }
pub fn tv_get(v:&TempVec, t:&Temp)->i64{ v.raw[t.v as usize] }
pub fn tv_set(v:&mut TempVec, t:&Temp, x:i64){ let i=t.v as usize; v.raw[i]=x; }
pub fn tv_len(v:&TempVec)->u32{ v.raw.len() as u32 }
}
mod oarena {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
pub struct IdArena { pub ids: Vec<u32>, pub kinds: Vec<u32> }
pub fn ar_new()->IdArena{ IdArena{ids:Vec::new(), kinds:Vec::new()} }
pub fn ar_alloc(a:&mut IdArena, kind:u32)->u32{ let i=a.ids.len() as u32; a.ids.push(i); a.kinds.push(kind); i }
pub fn ar_kind(a:&IdArena, i:u32)->u32{ a.kinds[i as usize] }
pub fn ar_len(a:&IdArena)->u32{ a.ids.len() as u32 }
}
mod oabi {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
#[derive(Copy,Clone)] pub struct Layout { pub size: u32, pub align: u32 }
pub fn layout_int()->Layout{ Layout{size:8, align:8} }
pub fn layout_bool()->Layout{ Layout{size:1, align:1} }
pub fn align_to(x:u32, a:u32)->u32{ let m=a-1; (x+m) & (!m) }
pub fn ll_ty(l:&Layout)->String{ if l.size==8 { String::from("i64") } else { String::from("i1") } }
pub fn frame_size(nslots:u32)->u32{ align_to(nslots*8, 16) }
// struct layout: n i64 fields, each 8-byte aligned -> field k at offset k*8, size = n*8
pub fn struct_size(nfields:u32)->u32{ align_to(nfields*8, 8) }
pub fn field_offset(k:u32)->u32{ k*8 }
}
mod olex {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
#[derive(Clone)] pub enum Tk { LoopT, Dollar, DeriveCopy, WhereT, TypeT, UnsafeT, Bang2, UseT, ExternT, CrateT, Caret, Percent, Shl, Shr, AsT, Amp, ConstT, StaticT, BreakT, ContT, PlusEq, MinusEq, StarEq, SlashEq, ModT, Quest, DotDot, InT, Bar, DynT, Ret, Trait, ForT, StrL(String), Num(i64), ChL(i64), FloatL(String), Ident(String), Fn, While, Mut, Struct, Enum, Impl, Match, FatArrow, PathSep, LBrack, RBrack, Dot, Colon, Gt, Le, Ge, Ne, AndAnd, OrOr, Bang, Slash, Comma, Plus, Minus, Star, Lt, EqEq, LParen, RParen, LBrace, RBrace, Semi, Eq, Let, If, Else, TrueT, FalseT, Eof }
pub struct Tok { pub k: Tk, pub lo: u32, pub hi: u32 }
// does this attribute's text derive Copy?
fn oast_is_num_suffix(t:&str)->bool{ t=="u8"||t=="u16"||t=="u32"||t=="u64"||t=="usize"||t=="i8"||t=="i16"||t=="i32"||t=="i64"||t=="isize"||t=="f32"||t=="f64" }
fn has_copy(t:&str)->bool{ let b=t.as_bytes(); let mut i=0; while i+3<b.len(){ if b[i]==67 && b[i+1]==111 && b[i+2]==112 && b[i+3]==121 { return true; } i=i+1; } false }
fn is_a(c:char)->bool{ let x=c as u32; (x>=97&&x<=122)||(x>=65&&x<=90)||x==95 }
fn is_d(c:char)->bool{ let x=c as u32; x>=48&&x<=57 }
pub fn lex(src:&str)->Vec<Tok>{
    let cs:Vec<char>={let mut v:Vec<char>=Vec::new(); for c in src.chars(){v.push(c);} v};
    let mut out:Vec<Tok>=Vec::new(); let mut i=0;
    while i<cs.len(){
        let st=i as u32; let c=cs[i];
        if c==' '||c=='\n'||c=='\t'||c=='\r' { i=i+1; }
        else if c=='#' { // an ATTRIBUTE: `#[..]` or `#![..]`, skipped to the matching `]`
            i=i+1; if i<cs.len() && cs[i]=='!' { i=i+1; }
            if i<cs.len() && cs[i]=='[' { let mut d=0; let abeg=i;
                while i<cs.len(){ if cs[i]=='[' { d=d+1; } if cs[i]==']' { d=d-1; if d==0 { i=i+1; break; } } i=i+1; }
                { let mut txt=String::from(""); let mut q=abeg; while q<i { txt.push(cs[q]); q=q+1; }
                  if has_copy(&txt) { out.push(Tok{k:Tk::DeriveCopy, lo:st, hi:i as u32}); } } } }
        else if c=='/' && i+1<cs.len() && cs[i+1]=='/' { while i<cs.len() && cs[i]!='\n' { i=i+1; } }
        else if c=='/' && i+1<cs.len() && cs[i+1]=='*' { i=i+2; while i+1<cs.len() && !(cs[i]=='*' && cs[i+1]=='/') { i=i+1; } if i+1<cs.len() { i=i+2; } else { i=cs.len(); } }
        // a BYTE literal `b'x'` is an INTEGER (`println!("{}", b'A')` prints 65, not A), so it is a Num
        else if c=='b' && i+1<cs.len() && cs[i+1]=='\'' {
            if i+3<cs.len() && cs[i+3]=='\'' { let v=cs[i+2] as u32; i=i+4; out.push(Tok{k:Tk::Num(v as i64), lo:st, hi:i as u32}); continue; }
            if i+4<cs.len() && cs[i+2]=='\\' && cs[i+4]=='\'' { let e=cs[i+3]; let v=if e=='n' { 10 } else if e=='t' { 9 } else if e=='r' { 13 } else if e=='0' { 0 } else { e as u32 }; i=i+5; out.push(Tok{k:Tk::Num(v as i64), lo:st, hi:i as u32}); continue; }
            // `b'\xNN'` -- a hex byte escape
            if i+6<cs.len() && cs[i+2]=='\\' && cs[i+3]=='x' && cs[i+6]=='\'' {
                let hx=|ch:char| -> i64 { let x=ch as u32; if x>=48 && x<=57 { (x-48) as i64 } else if x>=97 && x<=102 { (x-97+10) as i64 } else if x>=65 && x<=70 { (x-65+10) as i64 } else { 0 } };
                let v=hx(cs[i+4])*16+hx(cs[i+5]); i=i+7; out.push(Tok{k:Tk::Num(v), lo:st, hi:i as u32}); continue; } }
        else if c=='\'' {
            // the one-char case must not claim `'\''`: cs[i+2] is the closing quote there too, and taking
            // cs[i+1] read the BACKSLASH as the character
            if i+2<cs.len() && cs[i+2]=='\'' && cs[i+1]!='\\' { let v=cs[i+1] as u32; i=i+3; out.push(Tok{k:Tk::ChL(v as i64), lo:st, hi:i as u32}); }
            else if i+3<cs.len() && cs[i+1]=='\\' && cs[i+3]=='\'' { let e=cs[i+2]; let v=if e=='n' { 10 } else if e=='t' { 9 } else if e=='r' { 13 } else if e=='0' { 0 } else { e as u32 }; i=i+4; out.push(Tok{k:Tk::ChL(v as i64), lo:st, hi:i as u32}); }
            else { i=i+1; while i<cs.len() && (is_a(cs[i])||is_d(cs[i])) { i=i+1; } }
        }
        else if c=='"' { i=i+1; let mut s=String::from("");
            while i<cs.len() && cs[i]!='"' {
                if cs[i]=='\\' && i+1<cs.len() {
                    let e=cs[i+1];
                    if e=='n' { s.push('\n'); i=i+2; continue; }
                    if e=='t' { s.push('\t'); i=i+2; continue; }
                    if e=='r' { s.push('\r'); i=i+2; continue; }
                    if e=='0' { s.push('\0'); i=i+2; continue; }
                    if e=='\\' { s.push('\\'); i=i+2; continue; }
                    if e=='"' { s.push('"'); i=i+2; continue; }
                    if e=='\'' { s.push('\''); i=i+2; continue; }
                    if e=='x' && i+3<cs.len() {
                        let hx=|ch:char| -> u32 { let x=ch as u32; if x>=48 && x<=57 { x-48 } else if x>=97 && x<=102 { x-97+10 } else if x>=65 && x<=70 { x-65+10 } else { 0 } };
                        let v=hx(cs[i+2])*16+hx(cs[i+3]); s.push(v as u8 as char); i=i+4; continue; } }
                s.push(cs[i]); i=i+1; }
            if i<cs.len() { i=i+1; } out.push(Tok{k:Tk::StrL(s), lo:st, hi:i as u32}); }
        else if is_d(c) { let mut n:i64=0;
            // a FLOAT literal: digits `.` digits, or an exponent. `1..2` is a range, not a float, so a
            // second `.` immediately after the first stops the read. A radix literal can match neither
            // shape, so this read is radix-independent and comes first.
            {
                let mut j=i; let mut isf=false;
                while j<cs.len() && (is_d(cs[j]) || cs[j]=='_') { j=j+1; }
                if j<cs.len() && cs[j]=='.' && !(j+1<cs.len() && cs[j+1]=='.') {
                    if j+1>=cs.len() || !is_a(cs[j+1]) || cs[j+1]=='e' || cs[j+1]=='E' { isf=true; } }
                if j<cs.len() && (cs[j]=='e' || cs[j]=='E') && j+1<cs.len() && (is_d(cs[j+1]) || cs[j+1]=='-' || cs[j+1]=='+') { isf=true; }
                if isf {
                    let mut txt=String::from(""); let mut k=i;
                    while k<cs.len() && (is_d(cs[k]) || cs[k]=='_') { if cs[k]!='_' { txt.push(cs[k]); } k=k+1; }
                    if k<cs.len() && cs[k]=='.' { txt.push('.'); k=k+1;
                        while k<cs.len() && (is_d(cs[k]) || cs[k]=='_') { if cs[k]!='_' { txt.push(cs[k]); } k=k+1; } }
                    if k<cs.len() && (cs[k]=='e' || cs[k]=='E') { txt.push('e'); k=k+1;
                        if k<cs.len() && (cs[k]=='-' || cs[k]=='+') { txt.push(cs[k]); k=k+1; }
                        while k<cs.len() && is_d(cs[k]) { txt.push(cs[k]); k=k+1; } }
                    // a width suffix is consumed with the literal; f32 is a separate rung
                    if k+2<cs.len() && cs[k]=='f' && cs[k+1]=='6' && cs[k+2]=='4' { k=k+3; }
                    // an f32 suffix is deliberately NOT consumed: f32 is its own rung (F-3), and a
                    // silently widened f32 would print 16 digits where rustc prints 8

                    i=k; out.push(Tok{k:Tk::FloatL(txt), lo:st, hi:i as u32}); continue; } }
            // a RADIX literal `0x..` / `0o..` / `0b..` (digit separators allowed), before the decimal read
            let mut radix:i64=10;
            if c=='0' && i+1<cs.len() { let r=cs[i+1];
                if r=='x' || r=='X' { radix=16; } else if r=='o' || r=='O' { radix=8; } else if r=='b' || r=='B' { radix=2; }
                if radix!=10 { i=i+2; } }
            if radix!=10 {
                loop { if i>=cs.len() { break; }
                    let d=cs[i];
                    if d=='_' { i=i+1; continue; }
                    let v:i64 = if is_d(d) { ((d as u32)-48) as i64 }
                                else if d>='a' && d<='f' { (((d as u32)-97)+10) as i64 }
                                else if d>='A' && d<='F' { (((d as u32)-65)+10) as i64 }
                                else { 0-1 };
                    if v<0 || v>=radix { break; }
                    n=n*radix+v; i=i+1; } }
            else {
            while i<cs.len() && (is_d(cs[i]) || cs[i]=='_') { if cs[i]!='_' { n=n*10+((cs[i] as u32)-48) as i64; } i=i+1; } }
            // a TYPE SUFFIX on the literal (`1u32`, `42i64`, `0usize`) is consumed with it
            if i<cs.len() && (cs[i]=='u' || cs[i]=='i' || cs[i]=='f') {
                let sv=i; let mut j=i+1; while j<cs.len() && (is_a(cs[j])||is_d(cs[j])) { j=j+1; }
                let mut suf=String::from(""); let mut q=sv; while q<j { suf.push(cs[q]); q=q+1; }
                if oast_is_num_suffix(&suf) { i=j; } }
            out.push(Tok{k:Tk::Num(n), lo:st, hi:i as u32}); }
        else if is_a(c) { let mut s=String::from(""); while i<cs.len() && (is_a(cs[i])||is_d(cs[i])) { s.push(cs[i]); i=i+1; }
            if s.as_str()=="pub" { continue; }
            let k = if s.as_str()=="fn" { Tk::Fn } else if s.as_str()=="struct" { Tk::Struct } else if s.as_str()=="enum" { Tk::Enum } else if s.as_str()=="impl" { Tk::Impl } else if s.as_str()=="trait" { Tk::Trait } else if s.as_str()=="return" { Tk::Ret } else if s.as_str()=="dyn" { Tk::DynT } else if s.as_str()=="in" { Tk::InT } else if s.as_str()=="mod" { Tk::ModT } else if s.as_str()=="break" { Tk::BreakT } else if s.as_str()=="continue" { Tk::ContT } else if s.as_str()=="const" { Tk::ConstT } else if s.as_str()=="static" { Tk::StaticT } else if s.as_str()=="as" { Tk::AsT } else if s.as_str()=="use" { Tk::UseT } else if s.as_str()=="extern" { Tk::ExternT } else if s.as_str()=="crate" { Tk::CrateT } else if s.as_str()=="unsafe" { Tk::UnsafeT } else if s.as_str()=="where" { Tk::WhereT } else if s.as_str()=="type" { Tk::TypeT } else if s.as_str()=="loop" { Tk::LoopT } else if s.as_str()=="for" { Tk::ForT } else if s.as_str()=="match" { Tk::Match } else if s.as_str()=="while" { Tk::While } else if s.as_str()=="mut" { Tk::Mut } else if s.as_str()=="let" { Tk::Let } else if s.as_str()=="if" { Tk::If } else if s.as_str()=="else" { Tk::Else } else if s.as_str()=="true" { Tk::TrueT } else if s.as_str()=="false" { Tk::FalseT } else { Tk::Ident(s) };
            out.push(Tok{k:k, lo:st, hi:i as u32}); }
        else {
            let k2 = if c=='+' { Tk::Plus } else if c=='-' { Tk::Minus } else if c=='*' { Tk::Star } else if c=='/' { Tk::Slash } else if c=='%' { Tk::Percent } else if c=='<' { Tk::Lt } else if c=='>' { Tk::Gt }
                else if c=='(' { Tk::LParen } else if c==')' { Tk::RParen } else if c=='{' { Tk::LBrace } else if c=='}' { Tk::RBrace } else if c=='[' { Tk::LBrack } else if c==']' { Tk::RBrack }
                else if c==';' { Tk::Semi } else if c==',' { Tk::Comma } else if c=='.' { Tk::Dot } else if c==':' { Tk::Colon } else if c=='=' { Tk::Eq } else { Tk::Eof };
            if c=='=' && i+1<cs.len() && cs[i+1]=='>' { i=i+2; out.push(Tok{k:Tk::FatArrow, lo:st, hi:i as u32}); }
            else if c==':' && i+1<cs.len() && cs[i+1]==':' { i=i+2; out.push(Tok{k:Tk::PathSep, lo:st, hi:i as u32}); }
            else if c=='=' && i+1<cs.len() && cs[i+1]=='=' { i=i+2; out.push(Tok{k:Tk::EqEq, lo:st, hi:i as u32}); }
            else if c=='<' && i+1<cs.len() && cs[i+1]=='=' { i=i+2; out.push(Tok{k:Tk::Le, lo:st, hi:i as u32}); }
            else if c=='>' && i+1<cs.len() && cs[i+1]=='=' { i=i+2; out.push(Tok{k:Tk::Ge, lo:st, hi:i as u32}); }
            else if c=='!' && i+1<cs.len() && cs[i+1]=='=' { i=i+2; out.push(Tok{k:Tk::Ne, lo:st, hi:i as u32}); }
            else if c=='&' && i+1<cs.len() && cs[i+1]=='&' { i=i+2; out.push(Tok{k:Tk::AndAnd, lo:st, hi:i as u32}); }
            else if c=='$' { i=i+1; out.push(Tok{k:Tk::Dollar, lo:st, hi:i as u32}); }
        else if c=='^' { i=i+1; out.push(Tok{k:Tk::Caret, lo:st, hi:i as u32}); }
            else if c=='<' && i+1<cs.len() && cs[i+1]=='<' { i=i+2; out.push(Tok{k:Tk::Shl, lo:st, hi:i as u32}); }
            else if c=='>' && i+1<cs.len() && cs[i+1]=='>' { i=i+2; out.push(Tok{k:Tk::Shr, lo:st, hi:i as u32}); }
            else if c=='&' && (i+1>=cs.len() || cs[i+1]!='&') { i=i+1; out.push(Tok{k:Tk::Amp, lo:st, hi:i as u32}); }
            else if c=='+' && i+1<cs.len() && cs[i+1]=='=' { i=i+2; out.push(Tok{k:Tk::PlusEq, lo:st, hi:i as u32}); }
            else if c=='-' && i+1<cs.len() && cs[i+1]=='=' { i=i+2; out.push(Tok{k:Tk::MinusEq, lo:st, hi:i as u32}); }
            else if c=='*' && i+1<cs.len() && cs[i+1]=='=' { i=i+2; out.push(Tok{k:Tk::StarEq, lo:st, hi:i as u32}); }
            else if c=='/' && i+1<cs.len() && cs[i+1]=='=' { i=i+2; out.push(Tok{k:Tk::SlashEq, lo:st, hi:i as u32}); }
            else if c=='?' { i=i+1; out.push(Tok{k:Tk::Quest, lo:st, hi:i as u32}); }
            else if c=='.' && i+1<cs.len() && cs[i+1]=='.' { i=i+2; out.push(Tok{k:Tk::DotDot, lo:st, hi:i as u32}); }
            else if c=='|' && i+1<cs.len() && cs[i+1]=='|' { i=i+2; out.push(Tok{k:Tk::OrOr, lo:st, hi:i as u32}); }
            else if c=='|' { i=i+1; out.push(Tok{k:Tk::Bar, lo:st, hi:i as u32}); }
            else if c=='!' { i=i+1; out.push(Tok{k:Tk::Bang, lo:st, hi:i as u32}); }
            else { i=i+1; out.push(Tok{k:k2, lo:st, hi:i as u32}); }
        }
    }
    let e=cs.len() as u32; out.push(Tok{k:Tk::Eof, lo:e, hi:e}); out
}
}
mod oast {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
pub enum E { Cast(Box<E>, String, u32, u32), Ref(Box<E>, bool, u32, u32), BreakE(u32, u32), ContE(u32, u32), Try(Box<E>, u32, u32), ClosRef(String, Vec<String>, u32, u32), TupLit(Vec<E>, u32, u32), TupIdx(Box<E>, u32, u32, u32), RetE(Box<E>, u32, u32), StrLit(String, u32, u32), MethodCall(Box<E>, String, Vec<E>, u32, u32), AssignTo(Box<E>, Box<E>, u32, u32), ArrLit(Vec<E>, u32, u32), Index(Box<E>, Box<E>, u32, u32), EnumLit(String, String, Vec<E>, u32, u32), Match(Box<E>, Vec<Arm>, u32, u32), Not(Box<E>, u32, u32), StructLit(String, Vec<E>, u32, u32), Field(Box<E>, String, u32, u32), While(Box<E>, Box<E>, u32, u32), Seq(Box<E>, Box<E>, u32, u32), Assign(String, Box<E>, u32, u32), Call(String, Vec<E>, u32, u32), Int(i64,u32,u32), Bool(bool,u32,u32), Var(String,u32,u32), Bin(String, Box<E>, Box<E>, u32, u32), If(Box<E>, Box<E>, Box<E>, u32, u32), Let(String, Box<E>, Box<E>, u32, u32), LetT(String, String, Box<E>, Box<E>, u32, u32) }
pub struct Sdef { pub copy: bool, pub sgp: Vec<String>, pub name: String, pub fields: Vec<String>, pub ftys: Vec<String> }
pub struct Vdef { pub name: String, pub npayload: u32, pub ptys: Vec<String>, pub fnames: Vec<String> }
pub struct Edef { pub name: String, pub variants: Vec<Vdef> }
pub struct Arm { pub slit: String, pub has_slit: bool, pub lit: i64, pub has_lit: bool, pub has_guard: bool, pub guard: E, pub variant: String, pub binds: Vec<String>, pub body: E, pub wild: bool }
pub struct Fun { pub pref: Vec<bool>, pub ncap: i32, pub gbounds: Vec<String>, pub dflt: i32, pub name: String, pub params: Vec<String>, pub ptys: Vec<String>, pub ret: String, pub gparams: Vec<String>, pub owner: String, pub body: E }
pub struct Tdef { pub name: String, pub mnames: Vec<String> }
pub struct ImplRec { pub tr: String, pub ty: String, pub lo: u32, pub hi: u32 }
pub struct Prog { pub perrexp: String, pub perrgot: String, pub perr: u32, pub badfmt: u32, pub trailing: u32, pub sdefs: Vec<Sdef>, pub edefs: Vec<Edef>, pub funs: Vec<Fun>, pub tdefs: Vec<Tdef>, pub impls: Vec<ImplRec>, pub main: E }
pub fn tridx(ts:&Vec<Tdef>, n:&str)->i32{ let mut i=0; while i<ts.len(){ if ts[i].name.as_str()==n { return i as i32; } i=i+1; } 0-1 }
// A default trait method is MATERIALIZED per implementing type as its own Fun, but its BODY is not copied:
// `dflt` indexes the Fun whose body to use (-1 = own body). Keeps the AST copy-free while still giving each
// implementing type its own typed entry, so `self` inside a default body is the CONCRETE type and any
// self-method call in it resolves per-type (the shared-body alternative reintroduces dup dispatch).
// A generic parameter carrying a trait bound is MONOMORPHIZED: `fn f<T: Area>(x: T)` called with a `Sq`
// materializes `f$Sq` with T substituted, body shared via `dflt` (same device as a trait default). Unbounded
// generics keep the single untyped body, so nothing that already worked is re-routed.
pub fn has_bound(f:&Fun)->bool{ let mut i=0; while i<f.gbounds.len(){ if f.gbounds[i].len()>0 { return true; } i=i+1; } false }
pub fn ends_with_seg(t:&str, g:&str)->bool{ let a=t.as_bytes(); let b=g.as_bytes(); if b.len()+1>a.len() { return false; } let base=a.len()-b.len(); if a[base-1]!=36 { return false; } let mut i=0; while i<b.len(){ if a[base+i]!=b[i] { return false; } i=i+1; } true }
// `Vec$T` + `Vec$int` -> `int`: the concrete type the generic parameter stands for
pub fn unify_seg(pty:&str, aty:&str, g:&str)->String{
    if pty==g { return String::from(aty); }
    if !ends_with_seg(pty,g) { return String::from(""); }
    let keep=pty.len()-g.len();
    if aty.len()<=keep { return String::from(""); }
    let pb=pty.as_bytes(); let ab=aty.as_bytes(); let mut i=0;
    while i<keep { if pb[i]!=ab[i] { return String::from(""); } i=i+1; }
    String::from(&aty[keep..]) }
pub fn subst_seg(t:&str, g:&str, conc:&str)->String{
    if t==g { return String::from(conc); }
    if ends_with_seg(t,g) { let keep=t.len()-g.len(); let mut r=String::from(&t[..keep]); r.push_str(conc); return r; }
    String::from(t) }
pub fn has_composite_gp(f:&Fun)->bool{ let mut i=0; while i<f.gparams.len(){ let mut k=0; while k<f.ptys.len(){ if ends_with_seg(f.ptys[k].as_str(), f.gparams[i].as_str()) { return true; } k=k+1; } i=i+1; } false }
pub fn subst_ty(t:&str, gps:&Vec<String>, gbs:&Vec<String>, conc:&str)->String{ let mut i=0; while i<gps.len(){ if gps[i].as_str()==t && gbs[i].len()>0 { return String::from(conc); } i=i+1; } String::from(t) }
pub fn is_int_width(t:&str)->bool{ t=="u8" || t=="i8" || t=="u16" || t=="i16" || t=="u32" || t=="i32" || t=="u64" || t=="i64" || t=="usize" || t=="isize" }
pub fn width_bits(t:&str)->u32{ if t=="u8" || t=="i8" { 8 } else if t=="u16" || t=="i16" { 16 } else if t=="u32" || t=="i32" { 32 } else { 64 } }
pub fn width_signed(t:&str)->bool{ let b=t.as_bytes(); if b.len()==0 { return true; } b[0]==105 }
pub fn is_copy_struct(ss:&Vec<Sdef>, n:&str)->bool{ let mut i=0; while i<ss.len(){ if ss[i].name.as_str()==n { return ss[i].copy; } i=i+1; } false }
pub fn in_list(v:&Vec<String>, n:&str)->bool{ let mut i=0; while i<v.len(){ if v[i].as_str()==n { return true; } i=i+1; } false }
pub fn collect_free(e:&E, bound:&Vec<String>, out:&mut Vec<String>){
    match e {
        E::Var(n,_l,_h)=>{ if !in_list(bound,n) && !in_list(out,n) { out.push(String::from(n.as_str())); } },
        E::Let(n,v,b,_l,_h)=>{ collect_free(v,bound,out); let mut b2:Vec<String>=Vec::new(); let mut i=0; while i<bound.len(){ b2.push(String::from(bound[i].as_str())); i=i+1; } b2.push(String::from(n.as_str())); collect_free(b,&b2,out); },
        E::LetT(n,_t,v,b,_l,_h)=>{ collect_free(v,bound,out); let mut b2:Vec<String>=Vec::new(); let mut i=0; while i<bound.len(){ b2.push(String::from(bound[i].as_str())); i=i+1; } b2.push(String::from(n.as_str())); collect_free(b,&b2,out); },
        E::Match(sc,arms,_l,_h)=>{ collect_free(sc,bound,out); { let mut gi=0; while gi<arms.len(){ if arms[gi].has_guard { collect_free(&arms[gi].guard,bound,out); } gi=gi+1; } } let mut a=0; while a<arms.len(){ let mut b2:Vec<String>=Vec::new(); let mut i=0; while i<bound.len(){ b2.push(String::from(bound[i].as_str())); i=i+1; } let mut k=0; while k<arms[a].binds.len(){ b2.push(String::from(arms[a].binds[k].as_str())); k=k+1; } collect_free(&arms[a].body,&b2,out); a=a+1; } },
        E::ClosRef(_n,caps,_l,_h)=>{ let mut i=0; while i<caps.len(){ if !in_list(bound,&caps[i]) && !in_list(out,&caps[i]) { out.push(String::from(caps[i].as_str())); } i=i+1; } },
        E::Seq(a,b,_l,_h)=>{ collect_free(a,bound,out); collect_free(b,bound,out); },
        E::Bin(_o,a,b,_l,_h)=>{ collect_free(a,bound,out); collect_free(b,bound,out); },
        E::If(c,t,f,_l,_h)=>{ collect_free(c,bound,out); collect_free(t,bound,out); collect_free(f,bound,out); },
        E::While(c,b,_l,_h)=>{ collect_free(c,bound,out); collect_free(b,bound,out); },
        E::Call(_n,args,_l,_h)=>{ let mut k=0; while k<args.len(){ collect_free(&args[k],bound,out); k=k+1; } },
        E::MethodCall(r,_m,args,_l,_h)=>{ collect_free(r,bound,out); let mut k=0; while k<args.len(){ collect_free(&args[k],bound,out); k=k+1; } },
        E::Assign(n,v,_l,_h)=>{ if !in_list(bound,n) && !in_list(out,n) { out.push(String::from(n.as_str())); } collect_free(v,bound,out); },
        E::AssignTo(p,v,_l,_h)=>{ collect_free(p,bound,out); collect_free(v,bound,out); },
        E::Field(o,_f,_l,_h)=>{ collect_free(o,bound,out); },
        E::TupIdx(o,_i,_l,_h)=>{ collect_free(o,bound,out); },
        E::Index(b,i,_l,_h)=>{ collect_free(b,bound,out); collect_free(i,bound,out); },
        E::Not(x,_l,_h)=>{ collect_free(x,bound,out); },
        E::RetE(x,_l,_h)=>{ collect_free(x,bound,out); },
        E::Try(x,_l,_h)=>{ collect_free(x,bound,out); },
        E::Ref(x,_m,_l,_h)=>{ collect_free(x,bound,out); },
        E::Cast(x,_t,_l,_h)=>{ collect_free(x,bound,out); },
        E::TupLit(ps,_l,_h)=>{ let mut k=0; while k<ps.len(){ collect_free(&ps[k],bound,out); k=k+1; } },
        E::ArrLit(ps,_l,_h)=>{ let mut k=0; while k<ps.len(){ collect_free(&ps[k],bound,out); k=k+1; } },
        E::StructLit(_n,ps,_l,_h)=>{ let mut k=0; while k<ps.len(){ collect_free(&ps[k],bound,out); k=k+1; } },
        E::EnumLit(_n,_v,ps,_l,_h)=>{ let mut k=0; while k<ps.len(){ collect_free(&ps[k],bound,out); k=k+1; } },
        _=>{} } }
pub fn fbody_ix(fs:&Vec<Fun>, i:usize)->usize{ if fs[i].dflt>=0 { fs[i].dflt as usize } else { i } }
pub fn sfields(ss:&Vec<Sdef>, n:&str)->i32{ let mut i=0; while i<ss.len(){ if ss[i].name.as_str()==n { return ss[i].fields.len() as i32; } i=i+1; } -1 }
pub fn find_method(fs:&Vec<Fun>, owner:&str, m:&str)->i32{ let mut i=0; while i<fs.len(){ if fs[i].owner.as_str()==owner && fs[i].name.as_str()==m { return i as i32; } i=i+1; } -1 }
pub fn fgparams(fs:&Vec<Fun>, n:&str)->i32{ let mut i=0; while i<fs.len(){ if fs[i].name.as_str()==n { return fs[i].gparams.len() as i32; } i=i+1; } -1 }
// a bare type name matches a module-qualified registration when exactly one such registration exists
pub fn sfx_unique(names:&Vec<String>, n:&str)->i32{
    if n.len()==0 { return 0-1; }
    { let b=n.as_bytes(); let mut i=0; while i<b.len(){ if b[i]==36 { return 0-1; } i=i+1; } }   // already qualified
    let nb=n.as_bytes();
    let mut hit=0-1; let mut seen=0; let mut i=0;
    while i<names.len(){
        let mb=names[i].as_bytes();
        if mb.len()>nb.len()+1 {
            let base=mb.len()-nb.len();
            if mb[base-1]==36 {
                let mut same=true; let mut j=0;
                while j<nb.len(){ if mb[base+j]!=nb[j] { same=false; } j=j+1; }
                if same { hit=i as i32; seen=seen+1; } } }
        i=i+1; }
    if seen==1 { hit } else { 0-1 } }
pub fn snames_of(ss:&Vec<Sdef>)->Vec<String>{ let mut o:Vec<String>=Vec::new(); let mut i=0; while i<ss.len(){ o.push(String::from(ss[i].name.as_str())); i=i+1; } o }
pub fn enames_of(es:&Vec<Edef>)->Vec<String>{ let mut o:Vec<String>=Vec::new(); let mut i=0; while i<es.len(){ o.push(String::from(es[i].name.as_str())); i=i+1; } o }
pub fn eidx(es:&Vec<Edef>, n:&str)->i32{ let mut i=0; while i<es.len(){ if es[i].name.as_str()==n { return i as i32; } i=i+1; }
    { let nm=enames_of(es); let k=sfx_unique(&nm,n); if k>=0 { return k; } }
    -1 }
// the SUCCESS variants of the Try desugar: `Ok`/`Some` yield their payload, anything else is returned as-is
pub fn is_ok_variant(n:&str)->bool{ n=="Ok" || n=="Some" }
pub fn variant_name(es:&Vec<Edef>, en:&str, k:usize)->String{ let mut i=0; while i<es.len(){ if es[i].name.as_str()==en { if k<es[i].variants.len() { return String::from(es[i].variants[k].name.as_str()); } return String::from(""); } i=i+1; } String::from("") }
pub fn variant_payload_n(es:&Vec<Edef>, en:&str, k:usize)->u32{ let mut i=0; while i<es.len(){ if es[i].name.as_str()==en { if k<es[i].variants.len() { return es[i].variants[k].npayload; } return 0; } i=i+1; } 0 }
pub fn vidx(es:&Vec<Edef>, en:&str, v:&str)->i32{ let en=&{ let nm=enames_of(es); let k=sfx_unique(&nm,en); if k>=0 { String::from(es[k as usize].name.as_str()) } else { String::from(en) } }; let mut i=0; while i<es.len(){ if es[i].name.as_str()==en { let mut k=0; while k<es[i].variants.len(){ if es[i].variants[k].name.as_str()==v { return k as i32; } k=k+1; } return -1; } i=i+1; } -1 }
// the declared TYPE of a variant's payload slot (empty = untyped), which is what makes a recursive ADT
// usable: a match binding gets the declared type, so `Add(a, b) => eval(a) + eval(b)` type-checks
pub fn vpayload_ty(es:&Vec<Edef>, en:&str, v:&str, k:usize)->String{ let en=&{ let nm=enames_of(es); let k=sfx_unique(&nm,en); if k>=0 { String::from(es[k as usize].name.as_str()) } else { String::from(en) } }; let mut i=0; while i<es.len(){ if es[i].name.as_str()==en { let mut q=0; while q<es[i].variants.len(){ if es[i].variants[q].name.as_str()==v { if k<es[i].variants[q].ptys.len() { return String::from(es[i].variants[q].ptys[k].as_str()); } return String::from(""); } q=q+1; } return String::from(""); } i=i+1; } String::from("") }
pub fn max_payload(es:&Vec<Edef>, en:&str)->u32{ let mut i=0; while i<es.len(){ if es[i].name.as_str()==en { let mut m=0; let mut k=0; while k<es[i].variants.len(){ if es[i].variants[k].npayload>m { m=es[i].variants[k].npayload; } k=k+1; } return m; } i=i+1; } 0 }
pub fn vpayload(es:&Vec<Edef>, en:&str, v:&str)->i32{ let mut i=0; while i<es.len(){ if es[i].name.as_str()==en { let mut k=0; while k<es[i].variants.len(){ if es[i].variants[k].name.as_str()==v { return es[i].variants[k].npayload as i32; } k=k+1; } return -1; } i=i+1; } -1 }
pub fn nvariants(es:&Vec<Edef>, en:&str)->i32{ let mut i=0; while i<es.len(){ if es[i].name.as_str()==en { return es[i].variants.len() as i32; } i=i+1; } -1 }
pub fn enum_of_variant(es:&Vec<Edef>, v:&str)->String{ let mut i=0; while i<es.len(){ let mut k=0; while k<es[i].variants.len(){ if es[i].variants[k].name.as_str()==v { return String::from(es[i].name.as_str()); } k=k+1; } i=i+1; } String::from("") }
pub fn sfield_ty(ss:&Vec<Sdef>, n:&str, f:&str)->String{ { let nm=snames_of(ss); let k=sfx_unique(&nm,n); if k>=0 { return sfield_ty_at(ss,k as usize,f); } } let mut i=0; while i<ss.len(){ if ss[i].name.as_str()==n { let mut k=0; while k<ss[i].fields.len(){ if ss[i].fields[k].as_str()==f { return String::from(ss[i].ftys[k].as_str()); } k=k+1; } return String::from(""); } i=i+1; } String::from("") }
pub fn sfield_ty_at(ss:&Vec<Sdef>, i:usize, f:&str)->String{ let mut k=0; while k<ss[i].fields.len(){ if ss[i].fields[k].as_str()==f { if k<ss[i].ftys.len() { return String::from(ss[i].ftys[k].as_str()); } return String::from(""); } k=k+1; } String::from("") }
pub fn sfield_idx(ss:&Vec<Sdef>, n:&str, f:&str)->i32{ let mut i=0; while i<ss.len(){ if ss[i].name.as_str()==n { let mut k=0; while k<ss[i].fields.len(){ if ss[i].fields[k].as_str()==f { return k as i32; } k=k+1; } return -1; } i=i+1; } -1 }
pub fn e_lo(e:&E)->u32{ match e { E::Cast(_x,_t,l,_h)=>*l, E::Ref(_x,_m,l,_h)=>*l, E::BreakE(l,_h)=>*l, E::ContE(l,_h)=>*l, E::Try(_x,l,_h)=>*l, E::ClosRef(_n,_c,l,_h)=>*l, E::LetT(_n,_t,_v,_b,l,_h)=>*l, E::TupLit(_x,l,_h)=>*l, E::TupIdx(_b,_i,l,_h)=>*l, E::RetE(_x,l,_h)=>*l, E::StrLit(_s,l,_h)=>*l, E::MethodCall(_r,_n,_a,l,_h)=>*l, E::AssignTo(_p,_v,l,_h)=>*l, E::ArrLit(_x,l,_h)=>*l, E::Index(_b,_i,l,_h)=>*l, E::EnumLit(_e2,_v,_p,l,_h)=>*l, E::Match(_s,_a,l,_h)=>*l, E::Not(_x,l,_h)=>*l, E::StructLit(_n,_f,l,_h)=>*l, E::Field(_o,_f,l,_h)=>*l, E::While(_c,_b,l,_h)=>*l, E::Seq(_a,_b,l,_h)=>*l, E::Assign(_n,_v,l,_h)=>*l, E::Call(_n,_a,l,_h)=>*l, E::Int(_v,l,_h)=>*l, E::Bool(_v,l,_h)=>*l, E::Var(_n,l,_h)=>*l, E::Bin(_o,_a,_b,l,_h)=>*l, E::If(_c,_t,_f,l,_h)=>*l, E::Let(_n,_v,_b,l,_h)=>*l } }
pub fn e_hi(e:&E)->u32{ match e { E::Cast(_x,_t,_l,h)=>*h, E::Ref(_x,_m,_l,h)=>*h, E::BreakE(_l,h)=>*h, E::ContE(_l,h)=>*h, E::Try(_x,_l,h)=>*h, E::ClosRef(_n,_c,_l,h)=>*h, E::LetT(_n,_t,_v,_b,_l,h)=>*h, E::TupLit(_x,_l,h)=>*h, E::TupIdx(_b,_i,_l,h)=>*h, E::RetE(_x,_l,h)=>*h, E::StrLit(_s,_l,h)=>*h, E::MethodCall(_r,_n,_a,_l,h)=>*h, E::AssignTo(_p,_v,_l,h)=>*h, E::ArrLit(_x,_l,h)=>*h, E::Index(_b,_i,_l,h)=>*h, E::EnumLit(_e2,_v,_p,_l,h)=>*h, E::Match(_s,_a,_l,h)=>*h, E::Not(_x,_l,h)=>*h, E::StructLit(_n,_f,_l,h)=>*h, E::Field(_o,_f,_l,h)=>*h, E::While(_c,_b,_l,h)=>*h, E::Seq(_a,_b,_l,h)=>*h, E::Assign(_n,_v,_l,h)=>*h, E::Call(_n,_a,_l,h)=>*h, E::Int(_v,_l,h)=>*h, E::Bool(_v,_l,h)=>*h, E::Var(_n,_l,h)=>*h, E::Bin(_o,_a,_b,_l,h)=>*h, E::If(_c,_t,_f,_l,h)=>*h, E::Let(_n,_v,_b,_l,h)=>*h } }

// shared with the typechecker and the driver: the name/type environment and two small string helpers
pub struct Env { pub n:Vec<String>, pub t:Vec<i32> }
pub fn vc(v:&Vec<String>)->Vec<String>{ let mut o:Vec<String>=Vec::new(); let mut i=0; while i<v.len(){o.push(String::from(v[i].as_str()));i=i+1;} o }
pub fn vci(v:&Vec<i32>)->Vec<i32>{ let mut o:Vec<i32>=Vec::new(); let mut i=0; while i<v.len(){o.push(v[i]);i=i+1;} o }
pub fn u32s2(n:u32)->String{ if n==0 {return String::from("0");} let mut v=n; let mut s=String::from(""); while v>0 {let d=(v%10) as u32;let c=(48+d) as u8 as char;let mut t=String::from("");t.push(c);t.push_str(&s);s=t;v=v/10;} s }
// i64::MIN has no positive counterpart, so negating it to build digits overflows and produced a bare
// "-". The digits are built from the NEGATIVE side instead, which is total.
pub fn i64s(n:i64)->String{ if n==0 {return String::from("0");} let neg=n<0;
    if neg { let mut v=n; let mut s=String::from("");
        while v<0 { let d=(0-(v%10)) as u32; let c=(48+d) as u8 as char; let mut t=String::from(""); t.push(c); t.push_str(&s); s=t; v=v/10; }
        let mut t=String::from("-"); t.push_str(&s); return t; }
    let mut u=if neg {0-n} else {n}; let mut s=String::from(""); while u>0 {let d=(u%10) as u32;let c=(48+d) as u8 as char;let mut t=String::from("");t.push(c);t.push_str(&s);s=t;u=u/10;} if neg {let mut t=String::from("-");t.push_str(&s);s=t;} s }

// deep copies of the program tables, and name lookups over them (shared by every later phase)
pub fn copy_edefs(es:&Vec<Edef>)->Vec<Edef>{ let mut o:Vec<Edef>=Vec::new(); let mut i=0; while i<es.len(){ let mut vs:Vec<Vdef>=Vec::new(); let mut k=0; while k<es[i].variants.len(){ vs.push(Vdef{name:String::from(es[i].variants[k].name.as_str()), npayload:es[i].variants[k].npayload, ptys:copy_sgp(&es[i].variants[k].ptys), fnames:copy_sgp(&es[i].variants[k].fnames)}); k=k+1; } o.push(Edef{name:String::from(es[i].name.as_str()), variants:vs}); i=i+1; } o }
pub fn is_enum_name(es:&Vec<Edef>, n:&str)->bool{ let mut i=0; while i<es.len(){ if es[i].name.as_str()==n { return true; } i=i+1; } false }
pub fn is_struct_name(ss:&Vec<Sdef>, n:&str)->bool{ let mut i=0; while i<ss.len(){ if ss[i].name.as_str()==n { return true; } i=i+1; } { let nm=snames_of(ss); if sfx_unique(&nm,n)>=0 { return true; } } false }
pub fn copy_sgp(v:&Vec<String>)->Vec<String>{ let mut o:Vec<String>=Vec::new(); let mut i=0; while i<v.len(){ o.push(String::from(v[i].as_str())); i=i+1; } o }
pub fn copy_sdefs(ss:&Vec<Sdef>)->Vec<Sdef>{ let mut o:Vec<Sdef>=Vec::new(); let mut i=0; while i<ss.len(){ let mut fl:Vec<String>=Vec::new(); let mut k=0; while k<ss[i].fields.len(){ fl.push(String::from(ss[i].fields[k].as_str())); k=k+1; } let mut fy:Vec<String>=Vec::new(); let mut k2=0; while k2<ss[i].ftys.len(){ fy.push(String::from(ss[i].ftys[k2].as_str())); k2=k2+1; } o.push(Sdef{copy:ss[i].copy, sgp:copy_sgp(&ss[i].sgp), name:String::from(ss[i].name.as_str()), fields:fl, ftys:fy}); i=i+1; } o }
}
mod oresolve {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
pub struct Rib { pub names: Vec<String>, pub marks: Vec<usize> }
pub fn rib_new()->Rib{ Rib{names:Vec::new(), marks:Vec::new()} }
pub fn push_scope(r:&mut Rib){ r.marks.push(r.names.len()); }
pub fn pop_scope(r:&mut Rib){ if r.marks.len()>0 { let m=r.marks[r.marks.len()-1]; r.marks.pop(); while r.names.len()>m { r.names.pop(); } } }
pub fn nnames(r:&Rib)->usize{ r.names.len() }
pub fn define(r:&mut Rib, n:&str){ r.names.push(String::from(n)); }
pub fn resolves(r:&Rib, n:&str)->bool{ let mut i=r.names.len(); while i>0 { i=i-1; if r.names[i].as_str()==n { return true; } } false }
// the resolution pass: walk the AST, report every unresolved Var as a spanned diagnostic
pub fn resolve_expr(e:&E, r:&mut Rib, ds:&mut Vec<Diag>){
    match e {
        E::Not(x,_l,_h)=>{ resolve_expr(x,r,ds); },
        E::StrLit(_v,_l,_h)=>{},
        E::RetE(x,_l,_h)=>{ resolve_expr(x,r,ds); },
        E::ClosRef(_n,_c,_l,_h)=>{},
        E::Try(x,_l,_h)=>{ resolve_expr(x,r,ds); },
        E::BreakE(_l,_h)=>{},
        E::Ref(x,_m,_l,_h)=>{ resolve_expr(x,r,ds); },
        E::Cast(x,_t,_l,_h)=>{ resolve_expr(x,r,ds); },
        E::ContE(_l,_h)=>{},
        E::LetT(n,_t,v,b,_l,_h)=>{ resolve_expr(v,r,ds); push_scope(r); define(r,n); resolve_expr(b,r,ds); pop_scope(r); },
        E::TupLit(parts,_l,_h)=>{ let mut k=0; while k<parts.len(){ resolve_expr(&parts[k],r,ds); k=k+1; } },
        E::TupIdx(b,_i,_l,_h)=>{ resolve_expr(b,r,ds); },
        E::MethodCall(rv,_m,args,_l,_h)=>{ resolve_expr(rv,r,ds); let mut k=0; while k<args.len(){ resolve_expr(&args[k],r,ds); k=k+1; } },
        E::AssignTo(pl,v,_l,_h)=>{ resolve_expr(pl,r,ds); resolve_expr(v,r,ds); },
        E::ArrLit(xs,_l,_h)=>{ let mut k=0; while k<xs.len(){ resolve_expr(&xs[k],r,ds); k=k+1; } },
        E::Index(b,ix,_l,_h)=>{ resolve_expr(b,r,ds); resolve_expr(ix,r,ds); },
        E::EnumLit(_e2,_v,ps,_l,_h)=>{ let mut k=0; while k<ps.len(){ resolve_expr(&ps[k],r,ds); k=k+1; } },
        E::Match(sc,arms,_l,_h)=>{ resolve_expr(sc,r,ds); let mut k=0; while k<arms.len(){ push_scope(r); let mut bi=0; while bi<arms[k].binds.len(){ define(r, &arms[k].binds[bi]); bi=bi+1; } resolve_expr(&arms[k].body,r,ds); pop_scope(r); k=k+1; } },
        E::StructLit(_n,fs,_l,_h)=>{ let mut k=0; while k<fs.len(){ resolve_expr(&fs[k],r,ds); k=k+1; } },
        E::Field(o,_f,_l,_h)=>{ resolve_expr(o,r,ds); },
        E::While(c,b,_l,_h)=>{ resolve_expr(c,r,ds); push_scope(r); resolve_expr(b,r,ds); pop_scope(r); },
        E::Seq(a,b,_l,_h)=>{ resolve_expr(a,r,ds); resolve_expr(b,r,ds); },
        E::Assign(n,v,l,h)=>{ resolve_expr(v,r,ds); if !resolves(r,n) { let mut m=String::from("cannot find value `"); m.push_str(n); m.push_str("` in this scope"); ds.push(crate::oerrors::err(m,*l,*h)); } },
        E::Call(_n,args,_l,_h)=>{ let mut k=0; while k<args.len(){ resolve_expr(&args[k],r,ds); k=k+1; } },
        E::Int(_v,_l,_h)=>{},
        E::Bool(_v,_l,_h)=>{},
        E::Var(n,l,h)=>{ if n.as_str()!="_" && !resolves(r,n) { let mut m=String::from("cannot find value `"); m.push_str(n); m.push_str("` in this scope"); ds.push(crate::oerrors::err(m,*l,*h)); } },
        E::Bin(_o,a,b,_l,_h)=>{ resolve_expr(a,r,ds); resolve_expr(b,r,ds); },
        E::If(c,t,f,_l,_h)=>{ resolve_expr(c,r,ds); push_scope(r); resolve_expr(t,r,ds); pop_scope(r); push_scope(r); resolve_expr(f,r,ds); pop_scope(r); },
        E::Let(n,v,b,_l,_h)=>{ resolve_expr(v,r,ds); push_scope(r); define(r,n); resolve_expr(b,r,ds); pop_scope(r); },
    }
}
}
mod ohir {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
pub enum H { CastH(Box<H>, String, u32), BreakH(u32), ContH(u32), RetH(Box<H>, u32), StrH(String, u32), StoreIdxH(Box<H>, Box<H>, Box<H>, u32), StoreFieldH(Box<H>, u32, Box<H>, u32), ArrLitH(Vec<H>, u32), IndexH(Box<H>, Box<H>, u32), EnumLitH(u32, Vec<H>, u32), MatchH(Box<H>, Vec<HArm>, u32), StructLitH(u32, Vec<H>, u32), FieldH(Box<H>, u32, u32), WhileH(Box<H>, Box<H>, u32), SeqH(Box<H>, Box<H>, u32), AssignH(u32, Box<H>, u32), CallH(String, Vec<H>, u32), Lit(i64,u32), LitB(bool,u32), Local(u32,u32), BinH(String, Box<H>, Box<H>, u32), IfH(Box<H>, Box<H>, Box<H>, u32), LetH(u32, Box<H>, Box<H>, u32) }
pub struct HArm { pub tag: i32, pub nbinds: u32, pub slots: Vec<u32>, pub body: H }
pub fn h_id(h:&H)->u32{ match h { H::CastH(_x,_t,i)=>*i, H::BreakH(i)=>*i, H::ContH(i)=>*i, H::RetH(_x,i)=>*i, H::StrH(_v,i)=>*i, H::StoreIdxH(_b,_i,_v,i2)=>*i2, H::StoreFieldH(_b,_f,_v,i2)=>*i2, H::ArrLitH(_x,i)=>*i, H::IndexH(_b,_x,i)=>*i, H::EnumLitH(_t,_p,i)=>*i, H::MatchH(_s,_a,i)=>*i, H::StructLitH(_n,_f,i)=>*i, H::FieldH(_b,_x,i)=>*i, H::WhileH(_c,_b,i)=>*i, H::SeqH(_a,_b,i)=>*i, H::AssignH(_s,_v,i)=>*i, H::CallH(_n,_a,i)=>*i, H::Lit(_v,i)=>*i, H::LitB(_v,i)=>*i, H::Local(_s,i)=>*i, H::BinH(_o,_a,_b,i)=>*i, H::IfH(_c,_t,_f,i)=>*i, H::LetH(_s,_v,_b,i)=>*i } }
}
mod olower {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
pub struct LCx { pub selfty: String, pub names: Vec<String>, pub next_id: u32, pub ar: crate::oarena::IdArena, pub sdefs: Vec<Sdef>, pub stypes: Vec<String>, pub fnames: Vec<String>, pub frets: Vec<String>, pub fptys: Vec<String>, pub edefs: Vec<Edef>, pub fgps: Vec<String>, pub fgbs: Vec<String>, pub demands: Vec<String>, pub dimpls: Vec<String>, pub fncaps: Vec<i32> }
pub fn lcx_new()->LCx{ LCx{selfty:String::from(""), names:Vec::new(), next_id:0, ar:crate::oarena::ar_new(), sdefs:Vec::new(), stypes:Vec::new(), fnames:Vec::new(), frets:Vec::new(), fptys:Vec::new(), edefs:Vec::new(), fgps:Vec::new(), fgbs:Vec::new(), demands:Vec::new(), dimpls:Vec::new(), fncaps:Vec::new()} }
// C-LCXREBUILD: the nine table fields (sdefs, fnames, frets, fptys, edefs, fgps, fgbs, dimpls, fncaps) are
// WHOLE-PROGRAM and identical for every function lowered against the same Prog; only selfty/names/stypes/
// next_id/ar/demands are per-function. Rebuilding the tables per function made the signature table the
// arena dominator (measured: ~84% of it), so a caller that lowers many functions builds ONE context and
// resets just the per-function half between them. This is exactly lcx_new() minus the tables.
// C-PSFILTER: the recorded type of a PARAMETER (what lowering later reads back as its "static type"), in
// ONE place. It was written out four times -- odrv's emit loop and three sites in omono -- which is why
// nothing could widen it: a change had to be made in four copies to take effect. Scalars are recorded as
// the empty string DELIBERATELY: for a struct, enum, str, Fn, dyn object or Vec the NAME is the dispatch
// key, and a scalar needed none. That blindness is what blocks an integer `!` (see C-BITNOT, rung 44).
pub fn local_sty(ss:&Vec<Sdef>, ez:&Vec<Edef>, pt:&str)->String{
    if crate::oast::is_struct_name(ss,pt) || crate::oast::is_enum_name(ez,pt) || pt=="str" || pt=="Fn"
       || dyn_trait_of(pt).len()>0 || vec_elem_of(pt).len()>0 { return String::from(pt); }
    // widened: a SCALAR's name is recorded too, normalised to the three this tier actually distinguishes.
    // Nothing dispatches on it, but every rule that must tell an int local from a bool local was blind
    // without it (C-BITNOT, and the width-dependent arithmetic in the C1 ledger).
    if pt=="bool" { return String::from("bool"); }
    if pt=="f64" || pt=="f32" { return String::from("f64"); }
    // C-INTWIDTH: the DECLARED width is kept, not normalised away. Every rule that only wants
    // "is this an integer" already pairs its == "int" test with is_int_width, and the rules that need the
    // width -- wrapping arithmetic, and the C1 ledger's narrow-width arithmetic -- have no other source
    // for it. A char is an integer here with no width of its own, so it keeps answering "int".
    if pt=="char" || pt=="int" { return String::from("int"); }
    if crate::oast::is_int_width(pt) { return String::from(pt); }
    String::from("") }
pub fn lcx_reset(lc:&mut LCx){
    lc.selfty=String::from("");
    while lc.names.len()>0 { lc.names.pop(); }
    while lc.stypes.len()>0 { lc.stypes.pop(); }
    while lc.demands.len()>0 { lc.demands.pop(); }
    lc.next_id=0;
    lc.ar=crate::oarena::ar_new(); }
fn fret_of(cx:&LCx, n:&str)->String{ let mut i=0; while i<cx.fnames.len(){ if cx.fnames[i].as_str()==n { return String::from(cx.frets[i].as_str()); } i=i+1; } String::from("") }
fn fidx_of(cx:&LCx, n:&str)->i32{ let mut i=0; while i<cx.fnames.len(){ if cx.fnames[i].as_str()==n { return i as i32; } i=i+1; } -1 }
// the index of the parameter whose DECLARED type name equals `ty` (-1 if none) -- how a generic return `T` is
// tied back to the argument that supplies T.
// A `dyn Trait` value is a FAT PAIR {type_tag, data}: the tag indexes the trait's implementing types in
// declaration order (cx.dimpls holds "Trait|Type" in that order), and `data` is the ordinary struct handle.
// A method call on it emits a tag-dispatch CHAIN of direct calls -- a vtable whose entries are resolved
// statically, which is what this tier can express without function pointers.
// `Vec$dyn$Trait` is the type NAME of a heterogeneous vector; its ELEMENT type is the `dyn$Trait` suffix.
// the method name each overloadable operator dispatches to (Rust's std::ops names)
// the builtin a method call on a BUILT-IN type routes to (so programs can read like Rust)
// `$gload$NAME` / `$gstore$NAME`
pub fn is_gname(n:&str)->bool{ let b=n.as_bytes(); b.len()>7 && b[0]==36 && b[1]==103 && (b[2]==108 || b[3]==116) }
pub fn gname_of(n:&str)->String{ let b=n.as_bytes(); let mut i=1; let mut seen=0;
    while i<b.len(){ if b[i]==36 { seen=seen+1; if seen==1 { return String::from(&n[i+1..]); } } i=i+1; }
    String::from("") }
pub fn self_fix(cx:&LCx, n:&str)->String{
    let b=n.as_bytes();
    if b.len()>5 && b[0]==83 && b[1]==101 && b[2]==108 && b[3]==102 && b[4]==36 && cx.selfty.len()>0 {
        let mut r=String::from(cx.selfty.as_str()); r.push('$'); r.push_str(&n[5..]); return r; }
    String::from(n) }
// can this type's Debug be emitted from its static shape? (and which shape)
pub fn dbg_emit_ty(ty:&str)->String{
    if ty=="arr" { return String::from("ai"); }
    { let el=vec_elem_of(ty);
      if el.len()>0 {
          if el.as_str()=="str" { return String::from("vs"); }
          if vec_elem_of(&el).len()>0 { if vec_elem_of(&el).as_str()=="int" || crate::oast::is_int_width(&vec_elem_of(&el)) { return String::from("vvi"); } return String::from(""); }
          if el.as_str()=="int" || crate::oast::is_int_width(&el) { return String::from("vi"); }
          return String::from(""); } }
    String::from("") }
pub fn dbg_emit(cx:&LCx, e:&E, ty:&str)->String{
    if ty=="unit" { return String::from("unit"); }
    // a tuple LITERAL carries its elements, so its Debug is emittable even though a tuple VARIABLE's is
    // not (our tuple type name records only the arity)
    match e { E::TupLit(p,_l,_h)=>{ if p.len()>0 { return String::from("tuplit"); } }, _=>{} }
    // ... and a tuple VARIABLE is printable too once its name carries them (crate::oparse::tup_name)
    if crate::oparse::tup_elems(ty).len()>0 { return String::from("tupty"); }
    { let k=dbg_emit_ty(ty); if k.len()>0 { return k; } }
    if ty=="arr" { return String::from("ai"); }
    { let el=vec_elem_of(ty);
      if el.len()>0 {
          if el.as_str()=="str" { return String::from("vs"); }
          if vec_elem_of(&el).len()>0 { if vec_elem_of(&el).as_str()=="int" || crate::oast::is_int_width(&vec_elem_of(&el)) { return String::from("vvi"); } return String::from(""); }
          if el.as_str()=="int" || crate::oast::is_int_width(&el) { return String::from("vi"); }
          return String::from(""); } }
    // a bare `Vec` carries no element type: refuse rather than assume (a str printed as an int is a
    // pointer value, which is the worst kind of answer)
    if ty=="Vec" { return String::from(""); }
    if crate::oast::eidx(&cx.edefs,ty)>=0 { return String::from("enum"); }
    { let mut i=0; while i<cx.sdefs.len(){ if cx.sdefs[i].name.as_str()==ty { return String::from("struct"); } i=i+1; } }
    String::from("") }
fn dbg_str(cx:&mut LCx, t:&str)->H{ let i=fresh(cx); H::StrH(String::from(t), i) }
fn dbg_print(cx:&mut LCx, t:&str)->H{ let sh=dbg_str(cx,t); let mut a:Vec<H>=Vec::new(); a.push(sh); let i=fresh(cx); H::CallH(String::from("print_raw"), a, i) }
fn dbg_seq(cx:&mut LCx, a:H, b:H)->H{ let i=fresh(cx); H::SeqH(Box::new(a), Box::new(b), i) }
// the Debug printer for a value of a known type, as a sequence of prints
pub fn dbg_build(cx:&mut LCx, e:&E, ty:&str)->H{
    let kind=dbg_emit(cx,e,ty);
    if kind.as_str()=="vi" || kind.as_str()=="vs" || kind.as_str()=="vvi" || kind.as_str()=="ai" {
        let mut nm=String::from("dbg_"); nm.push_str(kind.as_str());
        let mut a:Vec<H>=Vec::new(); a.push(lower(e,cx)); let i=fresh(cx); return H::CallH(nm, a, i); }
    if kind.as_str()=="unit" { return dbg_print(cx,"()"); }
    if kind.as_str()=="tuplit" {
        match e { E::TupLit(parts,_l,_h)=>{
            let mut out=dbg_print(cx,"(");
            let mut k=0;
            while k<parts.len(){
                if k>0 { let c=dbg_print(cx,", "); out=dbg_seq(cx,out,c); }
                let pt=ety_of(cx,&parts[k]);
                // a NESTED tuple element is a tuple, not a scalar: print it with the same printer
                let pv=lower(&parts[k],cx);
                let pp=dbg_scalar(cx, pv, &pt);
                out=dbg_seq(cx,out,pp); k=k+1; }
            // rustc prints a 1-tuple as `(5,)` -- the comma is how it stays a tuple
            if parts.len()==1 { let cm=dbg_print(cx,","); out=dbg_seq(cx,out,cm); }
            let c2=dbg_print(cx,")"); return dbg_seq(cx,out,c2); }, _=>{} } }
    if kind.as_str()=="tupty" { let lv=lower(e,cx); return dbg_scalar(cx, lv, ty); }
    if kind.as_str()=="struct" {
        // `Point { x: 1, y: 2 }`, or `P(1, 2)` for a positional one, or `U` for a unit struct
        let mut fl:Vec<String>=Vec::new(); let mut ft:Vec<String>=Vec::new();
        { let mut i=0; while i<cx.sdefs.len(){ if cx.sdefs[i].name.as_str()==ty {
              let mut k=0; while k<cx.sdefs[i].fields.len(){ fl.push(String::from(cx.sdefs[i].fields[k].as_str()));
                  let t2=if k<cx.sdefs[i].ftys.len() { String::from(cx.sdefs[i].ftys[k].as_str()) } else { String::from("") };
                  ft.push(t2); k=k+1; } } i=i+1; } }
        if fl.len()==0 { return dbg_print(cx,ty); }
        let positional = fl[0].as_str()=="0";
        // bind the value once; every field read goes through the binding
        let lv=lower(e,cx);
        cx.names.push(String::from("$dbgv")); cx.stypes.push(String::from(ty));
        let sl=(cx.names.len()-1) as u32;
        let mut out=dbg_print(cx,ty);
        { let open=if positional { String::from("(") } else { String::from(" { ") };
          let o=dbg_print(cx,&open); out=dbg_seq(cx,out,o); }
        let mut k=0;
        while k<fl.len(){
            if k>0 { let c=dbg_print(cx,", "); out=dbg_seq(cx,out,c); }
            if !positional { let mut lab=String::from(fl[k].as_str()); lab.push_str(": "); let l2=dbg_print(cx,&lab); out=dbg_seq(cx,out,l2); }
            let ri=fresh(cx); let rl=H::Local(sl, ri); let fi2=fresh(cx);
            let fh=H::FieldH(Box::new(rl), k as u32, fi2);
            let fp=dbg_scalar(cx, fh, &ft[k]);
            out=dbg_seq(cx,out,fp); k=k+1; }
        { let close=if positional { String::from(")") } else { String::from(" }") };
          let c2=dbg_print(cx,&close); out=dbg_seq(cx,out,c2); }
        let fi3=fresh(cx); return H::LetH(sl, Box::new(lv), Box::new(out), fi3); }
    if kind.as_str()=="enum" {
        let ei=crate::oast::eidx(&cx.edefs,ty); let ed=&cx.edefs[ei as usize];
        let mut vn:Vec<String>=Vec::new(); let mut vnp:Vec<u32>=Vec::new(); let mut vpt:Vec<String>=Vec::new(); let mut vfn:Vec<String>=Vec::new();
        { let mut i=0; while i<ed.variants.len(){ vn.push(String::from(ed.variants[i].name.as_str()));
              vnp.push(ed.variants[i].npayload);
              let mut j=String::from(""); let mut k=0;
              while k<ed.variants[i].ptys.len(){ if k>0 { j.push(','); } j.push_str(ed.variants[i].ptys[k].as_str()); k=k+1; }
              vpt.push(j);
              { let mut j2=String::from(""); let mut k2=0;
                while k2<ed.variants[i].fnames.len(){ if k2>0 { j2.push(','); } j2.push_str(ed.variants[i].fnames[k2].as_str()); k2=k2+1; }
                vfn.push(j2); }
              i=i+1; } }
        let lv=lower(e,cx);
        cx.names.push(String::from("$dbge")); cx.stypes.push(String::from(ty));
        let sl=(cx.names.len()-1) as u32;
        let i0=fresh(cx); let mut chain=H::Lit(0, i0);
        let mut q=vn.len();
        while q>0 { q=q-1;
            let mut arm=dbg_print(cx,&vn[q]);
            if vnp[q]>0 {
                let fns=split_c(vfn[q].as_str());
                let named = fns.len()>0 && fns[0].len()>0;
                let o=dbg_print(cx, if named { " { " } else { "(" }); arm=dbg_seq(cx,arm,o);
                let pts=split_c(vpt[q].as_str());
                let mut k=0; while k<(vnp[q] as usize){
                    if k>0 { let c=dbg_print(cx,", "); arm=dbg_seq(cx,arm,c); }
                    if named && k<fns.len() { let mut lab=String::from(fns[k].as_str()); lab.push_str(": "); let l3=dbg_print(cx,&lab); arm=dbg_seq(cx,arm,l3); }
                    let ri=fresh(cx); let rl=H::Local(sl, ri); let fi2=fresh(cx);
                    let fh=H::FieldH(Box::new(rl), (k+1) as u32, fi2);
                    let pt=if k<pts.len() { String::from(pts[k].as_str()) } else { String::from("int") };
                    let pp=dbg_scalar(cx, fh, &pt); arm=dbg_seq(cx,arm,pp); k=k+1; }
                let c2=dbg_print(cx, if named { " }" } else { ")" }); arm=dbg_seq(cx,arm,c2); }
            let ri2=fresh(cx); let rl2=H::Local(sl, ri2); let ti=fresh(cx);
            let tagh=H::FieldH(Box::new(rl2), 0, ti);
            let li=fresh(cx); let lith=H::Lit(q as i64, li);
            let bi=fresh(cx); let test=H::BinH(String::from("=="), Box::new(tagh), Box::new(lith), bi);
            let ii=fresh(cx); chain=H::IfH(Box::new(test), Box::new(arm), Box::new(chain), ii); }
        let fi3=fresh(cx); return H::LetH(sl, Box::new(lv), Box::new(chain), fi3); }
    lower(e,cx) }
// a already-lowered SCALAR field, printed the Debug way
fn dbg_scalar(cx:&mut LCx, v:H, ty:&str)->H{
    // `{:?}` on a str/char ESCAPES (rustc: `"a\"b"`, `'\''`); the quotes are the printer's, not the value's
    if ty=="str" { let mut a:Vec<H>=Vec::new(); a.push(v); let i=fresh(cx); return H::CallH(String::from("dbgq"), a, i); }
    if ty=="char" { let mut a:Vec<H>=Vec::new(); a.push(v); let i=fresh(cx); return H::CallH(String::from("dbgc"), a, i); }
    if ty=="f64" { let mut a:Vec<H>=Vec::new(); a.push(v); let z=fresh(cx); a.push(H::Lit(1,z)); let i=fresh(cx);
        let st=H::CallH(String::from("f64_str"), a, i);
        let mut p:Vec<H>=Vec::new(); p.push(st); let i2=fresh(cx); return H::CallH(String::from("print_raw"), p, i2); }
    if ty=="bool" { let i1=fresh(cx); let t1=H::StrH(String::from("true"), i1);
        let i2=fresh(cx); let f1=H::StrH(String::from("false"), i2);
        let i3=fresh(cx); let pick=H::IfH(Box::new(v), Box::new(t1), Box::new(f1), i3);
        let mut p:Vec<H>=Vec::new(); p.push(pick); let i4=fresh(cx); return H::CallH(String::from("print_raw"), p, i4); }
    { let k=dbg_emit_ty(ty);
      if k.as_str()=="vi" || k.as_str()=="vs" || k.as_str()=="vvi" || k.as_str()=="ai" {
          let mut nm=String::from("dbg_"); nm.push_str(k.as_str());
          let mut a:Vec<H>=Vec::new(); a.push(v); let i=fresh(cx); return H::CallH(nm, a, i); } }
    { let es=crate::oparse::tup_elems(ty);
      if es.len()>0 {
          cx.names.push(String::from("$dbgt")); cx.stypes.push(String::from(ty));
          let sl=(cx.names.len()-1) as u32;
          let mut out=dbg_print(cx,"(");
          let mut k=0;
          while k<es.len(){
              if k>0 { let c=dbg_print(cx,", "); out=dbg_seq(cx,out,c); }
              let ri=fresh(cx); let rl=H::Local(sl, ri); let fi=fresh(cx);
              let fh=H::FieldH(Box::new(rl), k as u32, fi);
              let el=String::from(es[k].as_str());
              let pp=dbg_scalar(cx, fh, el.as_str());
              out=dbg_seq(cx,out,pp); k=k+1; }
          // rustc prints a 1-tuple as `(5,)` -- the comma is how it stays a tuple
          if es.len()==1 { let cm=dbg_print(cx,","); out=dbg_seq(cx,out,cm); }
          let c2=dbg_print(cx,")"); out=dbg_seq(cx,out,c2);
          let fi3=fresh(cx); return H::LetH(sl, Box::new(v), Box::new(out), fi3); } }
    if ty.len()>0 && ty!="int" && !crate::oast::is_int_width(ty) {
        // a field we cannot print must not be shown as its handle
        let mut a:Vec<H>=Vec::new(); let z=fresh(cx); a.push(H::Lit(0,z)); let i=fresh(cx);
        return H::CallH(String::from("$dbg_unsupported"), a, i); }
    let mut a:Vec<H>=Vec::new(); a.push(v); let i=fresh(cx);
    H::CallH(String::from("print_int_raw"), a, i) }
pub fn unique_method(cx:&LCx, m:&str)->String{
    let mut hit=String::from(""); let mut n=0;
    let mut i=0;
    while i<cx.fnames.len(){
        let fname=&cx.fnames[i];
        let b=fname.as_bytes(); let mb=m.as_bytes();
        if b.len()>mb.len()+1 {
            let base=b.len()-mb.len();
            if b[base-1]==36 { let mut j=0; let mut same=true;
                while j<mb.len(){ if b[base+j]!=mb[j] { same=false; } j=j+1; }
                if same { n=n+1; hit=String::from(fname.as_str()); } } }
        i=i+1; }
    if n==1 { return hit; }
    // a `Trait$m` entry is a DECLARATION or a default, not an implementation target. When exactly one
    // candidate is owned by a non-trait type, that one is the method -- which is what `impl Trait` in an
    // argument position needs, since the parameter carries no concrete type name.
    { let mut hit2=String::from(""); let mut n2=0; let mut k=0;
      while k<cx.fnames.len(){
        let fname=&cx.fnames[k];
        let b=fname.as_bytes(); let mb=m.as_bytes();
        if b.len()>mb.len()+1 {
            let base=b.len()-mb.len();
            if b[base-1]==36 { let mut j=0; let mut same=true;
                while j<mb.len(){ if b[base+j]!=mb[j] { same=false; } j=j+1; }
                if same { let owner=String::from(&fname[0..base-1]);
                    if !is_trait_name(cx,&owner) { n2=n2+1; hit2=String::from(fname.as_str()); } } } }
        k=k+1; }
      if n2==1 { return hit2; } }
    String::from("") }
pub fn is_trait_name(cx:&LCx, n:&str)->bool{
    let mut i=0; while i<cx.dimpls.len(){ let p=split_bar(cx.dimpls[i].as_str());
        if p.len()==2 && p[0].as_str()==n { return true; } i=i+1; }
    false }
// C-IDENTMETHOD: the methods that are the IDENTITY in a handle-based value model, stated once. A str and
// its bytes are the same handle (C8), a Box and its contents are the same handle (C-BOX), and a clone of
// an immutable value is the value -- so every one of these is a no-op whatever the receiver is. The str
// branch of builtin_method listed most of them and the Box branch listed two more; an UNNAMED receiver
// had no list at all, which is how `$as_str`, `$as_bytes` and friends reached the emitter undefined.
pub fn ident_method(m:&str)->bool{
    m=="as_str" || m=="as_ref" || m=="as_mut" || m=="as_bytes" || m=="chars"
    || m=="to_string" || m=="to_owned" || m=="clone" || m=="iter" || m=="into_iter"
    || m=="trim_matches" || m=="borrow" || m=="borrow_mut" || m=="into_inner" }
pub fn builtin_method(rt:&str, m:&str, na:usize)->String{
    // C-ASREF: `Box<T>` is TRANSPARENT in this value model (C-BOX) -- a boxed value and the value itself
    // are the same handle -- so `b.as_ref()`, `b.as_mut()` and `b.borrow()` are the identity on ANY
    // receiver, exactly as `as_str` already is on a str. Without this, matching on `pl.as_ref()` (the
    // ordinary way to look inside a Box<E>) was refused.
    if (m=="as_ref" || m=="as_mut") && na==0 { return String::from("$id"); }
    // C-WRAPPING: at 64 bits the wrapping methods ARE the plain operators, exactly -- probed against
    // rustc -O, a u64 and an i64 running the same wrapping chain give identical bits. BELOW 64 bits they
    // are not: 0u8.wrapping_sub(1) is 255 and we would answer -1 (rung 51 reverted the unguarded form
    // after the rc-topology gate produced three such witnesses). So they are admitted only at a declared
    // 64-bit width, which C-INTWIDTH keeps, and stay REFUSED at every narrower one.
    if (m=="wrapping_mul" || m=="wrapping_add" || m=="wrapping_sub") && na==1
       && (rt=="i64" || rt=="u64" || rt=="isize" || rt=="usize") { return String::from("$wrap"); }

    // see C-ENUMCLONE: an enum value is immutable here, so cloning it is the identity. The caller passes
    // the receiver's TYPE NAME, and an enum name is anything the enum table knows -- which the caller
    // checks before asking, so the name test here is the enum-shaped prefix the tagger uses.

    if rt=="arr" || arr_elem_of(rt).len()>0 { if m=="len" && na==0 { return String::from("arr_len"); } return String::from(""); }
    // O1-O9: the Option/Result surface, as prelude functions over the prelude's own enums
    if rt=="Option" {
        if (m=="unwrap" || m=="expect") && (na==0 || na==1) { return String::from("ounwrap"); }
        if m=="unwrap_or" && na==1 { return String::from("ounwrap_or"); }
        if m=="is_some" && na==0 { return String::from("ois_some"); }
        if m=="is_none" && na==0 { return String::from("ois_none"); }
        if m=="map" && na==1 { return String::from("omap"); }
        return String::from(""); }
    if rt=="Result" {
        if (m=="unwrap" || m=="expect") && (na==0 || na==1) { return String::from("runwrap"); }
        if m=="is_ok" && na==0 { return String::from("rok"); }
        if m=="is_err" && na==0 { return String::from("rerr"); }
        return String::from(""); }
    if rt=="bstr" { if (m=="len" || m=="count") && na==0 { return String::from("len"); }
                     if m=="next" && na==0 { return String::from("bnext"); }
                     return String::from(""); }
    if rt=="str" { if m=="len" && na==0 { return String::from("len"); }
                   if m=="at" && na==1 { return String::from("str_at"); }
                   // S1-S12: the str surface our C-string model expresses EXACTLY. `chars()` is absent on
                   // purpose: a byte walk diverges on non-ASCII (the s13 probe), and mutation needs a place.
                   // push/push_str ARE expressible now: the growth is a re-assignment of the receiver's
                   // slot (see the MethodCall arm in `lower`), so the "mutation needs a place" reason the
                   // chars() note gives no longer holds for them
                   // `push(char)` joins push_str now that `__chr1` exists: the refusal named a missing
                   // one-byte-string runtime, and the runtime is ours to add
                   if (m=="push_str" || m=="push") && na==1 { return String::from("$strgrow"); }
                   // a str IS its bytes here, so as_bytes is the identity plus a name (`bstr`) that makes
                   // `b[i]` read with str_at and `b.len()` read with len
                   if (m=="as_bytes" || m=="chars") && na==0 { return String::from("$id"); }
                   if m=="is_empty" && na==0 { return String::from("sempty"); }
                   if (m=="to_string" || m=="to_owned" || m=="as_str" || m=="trim_matches" || m=="clone") && na==0 { return String::from("$id"); }
                   if m=="contains" && na==1 { return String::from("shas"); }
                   if m=="find" && na==1 { return String::from("sfind"); }
                   if m=="starts_with" && na==1 { return String::from("sstarts"); }
                   if m=="ends_with" && na==1 { return String::from("sends"); }
                   if m=="trim" && na==0 { return String::from("strim"); }
                   if m=="trim_start" && na==0 { return String::from("strims"); }
                   if m=="trim_end" && na==0 { return String::from("strime"); }
                   if m=="to_uppercase" && na==0 { return String::from("supper"); }
                   if m=="to_lowercase" && na==0 { return String::from("slower"); }
                   if m=="repeat" && na==1 { return String::from("srepeat"); }
                   if m=="replace" && na==2 { return String::from("sreplace"); }
                   if m=="parse" && na==0 { return String::from("sparse"); }
                   return String::from(""); }
    if vec_elem_of(rt).len()>0 || rt=="Vec" {
        if m=="len" && na==0 { return String::from("vec_len"); }
        if m=="push" && na==1 { return String::from("vec_push"); }
        if m=="get" && na==1 { return String::from("vec_get"); }
        if m=="set" && na==2 { return String::from("vec_set"); }
        if m=="remove" && na==1 { return String::from("vremove"); }   // C-VECREMOVE: a prelude function, like max and pop
        // P-ITER: an iterator IS a vector here (eager), so `.iter()` is the identity and each adaptor is
        // a prelude function written in our own language
        if (m=="iter" || m=="into_iter" || m=="iter_mut" || m=="collect" || m=="cloned" || m=="copied" || m=="to_vec" || m=="clone") && na==0 { return String::from("$id"); }
        if m=="sum" && na==0 { return String::from("sum"); }
        if m=="count" && na==0 { return String::from("vec_len"); }
        if m=="is_empty" && na==0 { return String::from("vempty"); }
        if m=="max" && na==0 { return String::from("omax"); }
        if m=="min" && na==0 { return String::from("omin"); }
        if m=="rev" && na==0 { return String::from("vrev"); }
        if m=="last" && na==0 { return String::from("olast"); }
        if (m=="first" || m=="next") && na==0 { return String::from("ofirst"); }
        if m=="map" && na==1 { return String::from("map"); }
        if m=="filter" && na==1 { return String::from("filter"); }
        if m=="fold" && na==2 { return String::from("fold"); }
        if m=="any" && na==1 { return String::from("vany"); }
        if m=="all" && na==1 { return String::from("vall"); }
        if m=="position" && na==1 { return String::from("opos"); }
        if m=="take" && na==1 { return String::from("vtake"); }
        if m=="skip" && na==1 { return String::from("vskip"); }
        if m=="contains" && na==1 { return String::from("vhas"); }
        if m=="enumerate" && na==0 { return String::from("venum"); }
        if m=="pop" && na==0 { return String::from("vpop"); } }
    String::from("") }
pub fn arms_have_lit_or_guard(arms:&Vec<Arm>)->bool{ let mut i=0; while i<arms.len(){ if arms[i].has_lit || arms[i].has_slit || arms[i].has_guard { return true; } i=i+1; } false }
pub fn op_method(o:&str)->String{ if o=="+" { String::from("add") } else if o=="-" { String::from("sub") } else if o=="*" { String::from("mul") } else if o=="/" { String::from("div") } else if o=="==" { String::from("eq") } else { String::from("") } }
pub fn clos_name_ix(n:&str)->i32{ let b=n.as_bytes(); if b.len()<6 { return 0-1; } if b[0]!=36 || b[1]!=99 || b[2]!=108 || b[3]!=111 || b[4]!=115 { return 0-1; } let mut v=0; let mut i=5; while i<b.len(){ if b[i]<48 || b[i]>57 { return 0-1; } v=v*10+((b[i]-48) as i32); i=i+1; } v }
pub fn itoa_u(x:u32)->String{ if x==0 { return String::from("0"); } let mut v=x; let mut t:Vec<char>=Vec::new(); while v>0 { t.push((48+(v%10)) as u8 as char); v=v/10; } let mut r=String::from(""); let mut i=t.len(); while i>0 { r.push(t[i-1]); i=i-1; } r }
pub fn clos_ix_of(n:&str)->i32{ let b=n.as_bytes(); if b.len()<6 { return 0-1; } let mut v=0; let mut i=5; while i<b.len(){ if b[i]<48 || b[i]>57 { return 0-1; } v=v*10+((b[i]-48) as i32); i=i+1; } v }
pub fn clos_of(t:&str)->String{ let b=t.as_bytes(); if b.len()<6 { return String::from(""); } if b[0]==99 && b[1]==108 && b[2]==111 && b[3]==115 && b[4]==36 { return String::from(&t[5..]); } String::from("") }
pub fn is_vec_ty(t:&str)->bool{ t=="Vec" || vec_elem_of(t).len()>0 }
// `arr$T` is the typed-array name, the array twin of `Vec$T`
pub fn arr_elem_of(t:&str)->String{ let b=t.as_bytes(); if b.len()<5 { return String::from(""); } if b[0]==97 && b[1]==114 && b[2]==114 && b[3]==36 { return String::from(&t[4..]); } String::from("") }
pub fn elem_of(t:&str)->String{ let v=vec_elem_of(t); if v.len()>0 { return v; } arr_elem_of(t) }
pub fn vec_elem_of(t:&str)->String{ let b=t.as_bytes(); if b.len()<5 { return String::from(""); } if b[0]==86 && b[1]==101 && b[2]==99 && b[3]==36 { return String::from(&t[4..]); } String::from("") }
pub fn dyn_trait_of(t:&str)->String{ let b=t.as_bytes(); if b.len()<5 { return String::from(""); } if b[0]==100 && b[1]==121 && b[2]==110 && b[3]==36 { return String::from(&t[4..]); } String::from("") }
pub fn dyn_types(cx:&LCx, tr:&str)->Vec<String>{ let mut o:Vec<String>=Vec::new(); let mut i=0; while i<cx.dimpls.len(){ let p=split_bar(cx.dimpls[i].as_str()); if p.len()==2 && p[0].as_str()==tr { o.push(String::from(p[1].as_str())); } i=i+1; } o }
pub fn dyn_tag(cx:&LCx, tr:&str, ty:&str)->i32{ let ts=dyn_types(cx,tr); let mut i=0; while i<ts.len(){ if ts[i].as_str()==ty { return i as i32; } i=i+1; } 0-1 }
pub fn split_bar(s:&str)->Vec<String>{ let b=s.as_bytes(); let mut out:Vec<String>=Vec::new(); let mut start=0; let mut i=0;
    while i<=b.len(){ if i==b.len() || b[i]==124 { out.push(String::from(&s[start..i])); start=i+1; } i=i+1; } out }
pub fn split_c(s:&str)->Vec<String>{ let b=s.as_bytes(); let mut out:Vec<String>=Vec::new(); let mut start=0; let mut i=0;
    while i<=b.len(){ if i==b.len() || b[i]==44 { out.push(String::from(&s[start..i])); start=i+1; } i=i+1; } out }
fn pty_pos(cx:&LCx, fi:i32, ty:&str)->i32{
    if fi<0 { return -1; }
    let joined=String::from(cx.fptys[fi as usize].as_str());
    let b=joined.as_bytes(); let mut start=0; let mut idx=0; let mut i=0;
    while i<=b.len(){
        if i==b.len() || b[i]==44 { let seg=String::from(&joined[start..i]); if seg.as_str()==ty { return idx; } idx=idx+1; start=i+1; }
        i=i+1;
    }
    -1
}
fn is_known_ty(cx:&LCx, n:&str)->bool{
    // COMPOSITE type names count as known: without this a function returning `Vec<int>` propagated no type,
    // so a generic call could not unify its element and fell back to the un-monomorphized bare name
    if n=="str" || n=="Fn" || n=="bool" || n=="unit" { return true; }
    // K2 (probed from RC): a WIDTH is a property of the value and must survive a return, a parameter and
    // a let -- `let x = id(250u8); x + 10` is 4, and the suffix is three steps from the arithmetic. A
    // width name that is not "known" evaporates the moment it leaves the annotation that introduced it.
    if crate::oast::is_int_width(n) || n=="char" || n=="f64" { return true; }
    if vec_elem_of(n).len()>0 { return true; }
    if arr_elem_of(n).len()>0 { return true; }
    if crate::oparse::tup_elems(n).len()>0 { return true; }
    if dyn_trait_of(n).len()>0 { return true; }
    if crate::oast::eidx(&cx.edefs,n)>=0 { return true; } let mut i=0; while i<cx.sdefs.len(){ if cx.sdefs[i].name.as_str()==n { return true; } i=i+1; } false }
// the STRUCT TYPE NAME of an arbitrary expression ("" if not a struct) -- the lowering's own small inference,
// needed so `p.field` resolves to the right INDEX for params, call results, nested fields and literals.
pub fn ety_of(cx:&LCx, e:&E)->String{
    match e {
        E::Var(n,_a,_b)=>sty_of(cx,n),
        E::StructLit(sn,_f,_a,_b)=>String::from(sn.as_str()),
        E::EnumLit(en,_v,_p,_a,_b)=>String::from(en.as_str()),
        E::StrLit(_v,_l,_h)=>String::from("str"),
        // an integer literal's type was left UNKNOWN here (the engine reads "" as int anyway), which was
        // invisible until a tuple's name had to be minted from its elements: one unknown element made the
        // whole name fall back to the bare arity
        E::Int(_v,_l,_h)=>String::from("int"),
        E::Bool(_v,_l,_h)=>String::from("bool"),
        E::Not(x,_l,_h)=>{ let tx=ety_of(cx,x); if tx.as_str()=="int" || crate::oast::is_int_width(&tx) { String::from("int") } else { String::from("bool") } },
        E::RetE(x,_l,_h)=>ety_of(cx,x),
        // an overloaded operator's result type is its method's return type, so `(p + q).x` resolves
        E::Bin(o,a,b2,_l,_h) if (ety_of(cx,a).as_str()=="f64" || ety_of(cx,b2).as_str()=="f64") && arith_op(o) =>String::from("f64"),
        E::Bin(o,_a,b2,_l,_h) if o.as_str()=="^" && ety_of(cx,b2).as_str()=="f64" =>String::from("f64"),
        E::Bin(o,_a,_b,_l,_h) if cmp_op(o) =>String::from("bool"),
        // C-ETYARITH: arithmetic over integers ANSWERS an integer. ety_of used to fall through to "" for
        // it, so any rule that needed the kind of `a-1` was blind -- which is why `!(a-1)` reached the
        // bool lowering and computed a comparison with zero (C-BITNOT, rungs 44-45).
        E::Bin(o,a,b2,_l,_h) if arith_op(o) && { let ta=ety_of(cx,a); let tb=ety_of(cx,b2);
              ta.as_str()=="int" || crate::oast::is_int_width(&ta) || tb.as_str()=="int" || crate::oast::is_int_width(&tb) } =>String::from("int"),
        E::Bin(o,a,_b,_l,_h)=>{ let at=ety_of(cx,a); let m=op_method(o);
            if at.len()>0 && m.len()>0 { let mut mn=String::from(at.as_str()); mn.push('$'); mn.push_str(&m);
                if fidx_of(cx,&mn)>=0 { let r=fret_of(cx,&mn); if r.len()>0 && is_known_ty(cx,&r) { return r; } } }
            String::from("") },
        E::Try(_x,_l,_h)=>String::from(""),
        E::Ref(x,_m,_l,_h)=>ety_of(cx,x),
        E::Cast(_x,t,_l,_h)=>String::from(t.as_str()),
        E::ClosRef(n,_c,_l,_h)=>{ let mut r=String::from("clos$"); r.push_str(n.as_str()); r },
        E::TupLit(p,_l,_h)=>{ if p.len()==0 { String::from("unit") } else {
            let mut es:Vec<String>=Vec::new(); let mut k=0; while k<p.len(){ es.push(ety_of(cx,&p[k])); k=k+1; }
            crate::oparse::tup_name(p.len() as u32, &es) } },
        E::TupIdx(b,ix,_l,_h)=>{ let bt=ety_of(cx,b);
            // a tuple STRUCT field has a declared type; a plain tuple does not
            if bt.len()>0 { let fname=crate::oast::u32s2(*ix); let t=crate::oast::sfield_ty(&cx.sdefs,&bt,&fname); if t.len()>0 { return t; } }
            // a plain tuple's element type now comes from its NAME
            { let es=crate::oparse::tup_elems(&bt); if (*ix as usize)<es.len() { return String::from(es[*ix as usize].as_str()); } }
            String::from("") },
        E::MethodCall(r,m,a2,_l,_h) if m.as_str()=="clone" && a2.len()==0 && crate::oast::eidx(&cx.edefs,&ety_of(cx,r))>=0 =>ety_of(cx,r),
        // C-STRASRESULT: as_str, to_string and to_owned are the IDENTITY in value but their TYPE is
        // always str, whatever the receiver. ety_of was answering the receiver's type, so a String built
        // from an unnameable receiver came out unnameable itself -- and String::from is erased at parse
        // time (oparse maps it to the identity), so nothing downstream restored the name. That chain is
        // how six push_str calls in oparse reached the emitter as `$push_str`: trn unnamed, then
        // trn.as_str() unnamed, then dn unnamed.
        E::MethodCall(_r,m,a2,_l,_h) if (m.as_str()=="as_str" || m.as_str()=="to_string" || m.as_str()=="to_owned") && a2.len()==0 =>String::from("str"),
        E::MethodCall(r,m,a2,_l,_h) if (m.as_str()=="as_ref" || m.as_str()=="as_mut") && a2.len()==0 =>ety_of(cx,r),
        E::MethodCall(r,m,a2,_l,_h) if (m.as_str()=="get" || m.as_str()=="remove") && a2.len()==1 && vec_elem_of(&ety_of(cx,r)).len()>0 =>vec_elem_of(&ety_of(cx,r)),
        E::MethodCall(r,m,a2,_l,_h) if (m.as_str()=="wrapping_mul" || m.as_str()=="wrapping_add" || m.as_str()=="wrapping_sub") && a2.len()==1 =>ety_of(cx,r),
        E::MethodCall(r,m,_a2,_l,_h) if (m.as_str()=="as_bytes" || m.as_str()=="chars") && ety_of(cx,r).as_str()=="str" =>String::from("bstr"),
        E::MethodCall(r,m,_a2,_l,_h) if m.as_str()=="count" && ety_of(cx,r).as_str()=="bstr" =>String::from("int"),
        E::MethodCall(_r,m,_a2,_l,_h) if m.as_str()=="to_string" || m.as_str()=="to_owned" =>String::from("str"),
        E::MethodCall(r,m,_a2,_l,_h) if m.as_str()=="get" =>{ let vt=ety_of(cx,r); vec_elem_of(&vt) },
        // a BUILT-IN method's result type, by the same rule the typechecker uses: an adaptor answers the
        // same sequence, a predicate a bool, the rest an int
        E::MethodCall(r,m,a2,_ab,_bb) if builtin_method(&ety_of(cx,r), m, a2.len()).len()>0 =>{
            let rt=ety_of(cx,r);
            if rt.as_str()=="Option" {
                if m.as_str()=="map" { return String::from("Option"); }
                if m.as_str()=="is_some" || m.as_str()=="is_none" { return String::from("bool"); }
                return String::from("int"); }
            if rt.as_str()=="Result" {
                if m.as_str()=="is_ok" || m.as_str()=="is_err" { return String::from("bool"); }
                return String::from("int"); }
            if rt.as_str()=="str" {
                if m.as_str()=="push" || m.as_str()=="push_str" { return String::from("int"); }   // C-PUSHUNIT: () here
                if m.as_str()=="parse" { return String::from("Result"); }
                if m.as_str()=="find" { return String::from("Option"); }
                if m.as_str()=="is_empty" || m.as_str()=="contains" || m.as_str()=="starts_with" || m.as_str()=="ends_with" { return String::from("bool"); }
                if m.as_str()=="len" { return String::from("int"); }
                return String::from("str"); }
            if m.as_str()=="iter" || m.as_str()=="into_iter" || m.as_str()=="iter_mut" || m.as_str()=="collect"
               || m.as_str()=="cloned" || m.as_str()=="copied" || m.as_str()=="to_vec" || m.as_str()=="clone"
               || m.as_str()=="map" || m.as_str()=="filter" || m.as_str()=="rev" || m.as_str()=="take" || m.as_str()=="skip" { return rt; }
            if m.as_str()=="enumerate" { return String::from("Vec$tup2"); }
            if m.as_str()=="max" || m.as_str()=="min" || m.as_str()=="last" || m.as_str()=="first"
               || m.as_str()=="next" || m.as_str()=="position" || m.as_str()=="pop" { return String::from("Option"); }
            if m.as_str()=="get" { return vec_elem_of(&rt); }
            // a PREDICATE answers a bool, and `{}` must spell it `true`/`false`
            if m.as_str()=="any" || m.as_str()=="all" || m.as_str()=="is_empty" || m.as_str()=="contains" { return String::from("bool"); }
            String::from("int") },
        E::MethodCall(r,m,_a2,_a,_b)=>{ let rt=ety_of(cx,r); let mut nm=String::from(rt.as_str()); nm.push_str("$"); nm.push_str(m); if fidx_of(cx,&nm)<0 { let u=unique_method(cx,m); if u.len()>0 { nm=u; } } let r2=fret_of(cx,&nm); if r2.len()>0 && is_known_ty(cx,&r2) { r2 } else { String::from("") } },
        E::Call(fnm,_c2,_a,_b) if fnm.as_str()=="readfile" || fnm.as_str()=="argv_at" =>String::from("str"),
        E::Call(fnm,_c2,_a,_b) if fnm.as_str()=="str_cat" || fnm.as_str()=="str_sub" || fnm.as_str()=="str_of" || fnm.as_str()=="str_from_int" =>String::from("str"),
        E::Call(fnm,_cg,_a,_b) if fnm.as_str()=="vec_new" =>String::from("Vec"),
        E::Call(fnm,_cg,_a,_b) if fnm.as_str()=="f64_parse" || fnm.as_str()=="f64_of_int" =>String::from("f64"),
        E::Call(fnm,_cg,_a,_b) if fnm.as_str()=="f64_snprintf" || fnm.as_str()=="f64_snprintf_e" =>String::from("str"),
        E::Call(fnm,_cg,_a,_b) if fnm.as_str()=="f64_isnan" =>String::from("int"),
        E::Call(fnm,_cg,_a,_b) if fnm.as_str()=="f64_bits" =>String::from("int"),
        E::Call(fnm,_cg,_a,_b) if is_gname(fnm) =>String::from("int"),
        E::Call(fnm,cg,_a,_b) if fnm.as_str()=="seq_slice" && cg.len()==3 && ety_of(cx,&cg[0]).as_str()=="str" =>String::from("str"),
        E::Call(fnm,cg,_a,_b) if fnm.as_str()=="seq_slice" && cg.len()==3 =>{ let st=ety_of(cx,&cg[0]); let el=vec_elem_of(&st); if el.len()>0 { let mut r=String::from("Vec$"); r.push_str(el.as_str()); r } else { String::from("Vec") } },
        E::Let(n,v,b,_l,_h)=>{ match b.as_ref() { E::Var(bn,_l2,_h2)=>{ if bn.as_str()==n.as_str() { return ety_of(cx,v); } }, _=>{} } ety_of(cx,b) },
        E::ArrLit(x,_l,_h)=>{ let mut allstr=x.len()>0; let mut k=0;
            while k<x.len(){ if ety_of(cx,&x[k]).as_str()!="str" { allstr=false; } k=k+1; }
            if allstr { String::from("arr$str") } else { String::from("arr") } },
        E::Index(b,_ix,_l,_h)=>{ let bt=ety_of(cx,b); if bt.as_str()=="bstr" { return String::from("int"); }
            let el=elem_of(&bt); if el.len()>0 { el } else { String::from("") } },
        E::Match(_s,arms,_l,_h)=>{ if arms.len()>0 { ety_of(cx,&arms[0].body) } else { String::from("") } },
        E::If(_c,t,_f,_l,_h)=>ety_of(cx,t),
        E::LetT(_n,t,_v,_b,_l,_h)=>String::from(t.as_str()),
        E::Seq(_a,b,_l,_h)=>ety_of(cx,b),
        E::Call(fnm,cg,_a,_b) if (fnm.as_str()=="vec_get" || fnm.as_str()=="seq_get") && cg.len()==2 =>{ let vt=ety_of(cx,&cg[0]); let el=vec_elem_of(&vt); el },
        E::Call(fnm0,cargs,_a,_b)=>{ let fnm=&self_fix(cx,fnm0); let rt=fret_of(cx,fnm); if rt.len()>0 && is_known_ty(cx,&rt) { rt } else { let fi=fidx_of(cx,fnm); let pp=pty_pos(cx,fi,&rt); if pp>=0 && (pp as usize)<cargs.len() { ety_of(cx,&cargs[pp as usize]) } else { String::from("") } } },
        E::Field(o,f,_a,_b)=>{ let os=ety_of(cx,o); if os.len()==0 { String::from("") } else { crate::oast::sfield_ty(&cx.sdefs,&os,f) } },
        _=>String::from(""),
    }
}
fn sty_of(cx:&LCx, n:&str)->String{ let mut i=cx.names.len(); while i>0 { i=i-1; if cx.names[i].as_str()==n { return String::from(cx.stypes[i].as_str()); } } String::from("") }
fn fresh(cx:&mut LCx)->u32{ let i=crate::oarena::ar_alloc(&mut cx.ar, 1); cx.next_id=i+1; crate::oindex::hidx(&crate::oindex::hid(i)) }   // ids come from the ARENA and are typed through crate::oindex::HirId
fn slot_of(cx:&LCx, n:&str)->u32{ let mut i=cx.names.len(); while i>0 { i=i-1; if cx.names[i].as_str()==n { return i as u32; } } 0 }
// route a call to a BOUND-generic fn to its monomorphization: the concrete type comes from the argument whose
// declared type IS the bounded parameter. No concrete type recoverable -> the original name (no silent mis-route).
pub fn mono_name(cx:&mut LCx, n:&str, args:&Vec<E>)->String{
    let mut atys:Vec<String>=Vec::new(); let mut i=0; while i<args.len(){ atys.push(ety_of(cx,&args[i])); i=i+1; }
    mono_name_tys(cx, n, &atys) }
// the same rule over argument TYPES rather than expressions -- a builtin method already knows its
// receiver's type and cannot hand over the expression twice (`E` has no Clone), and the DEMAND must be
// recorded identically or the instance is never materialised
pub fn mono_name_tys(cx:&mut LCx, n:&str, atys:&Vec<String>)->String{
    let fi=fidx_of(cx,n); if fi<0 { return String::from(n); }
    let gi=fi as usize; if gi>=cx.fgbs.len() { return String::from(n); }
    let gps=split_c(&cx.fgps[gi]); let ptys=split_c(&cx.fptys[gi]);
    if gps.len()==0 { return String::from(n); }
    // resolve EVERY type parameter, in declaration order: for each one find a parameter whose declared type
    // mentions it (as the whole type, or inside a composite like `Vec<T>`) and unify against that argument's
    // type. All of them must resolve, which is what lets `fn pair<A, B>(a: A, b: B)` monomorphize -- the
    // single-parameter version returned at the first match and could not name a two-parameter instance.
    let mut concs:Vec<String>=Vec::new();
    let mut g=0;
    while g<gps.len(){
        if gps[g].len()==0 { return String::from(n); }
        let mut found=String::from("");
        let mut k=0;
        while k<ptys.len() && k<atys.len(){
            if ptys[k].as_str()==gps[g].as_str() || crate::oast::ends_with_seg(ptys[k].as_str(), gps[g].as_str()) {
                let at=String::from(atys[k].as_str());
                let c=crate::oast::unify_seg(ptys[k].as_str(), at.as_str(), gps[g].as_str());
                if c.len()>0 { found=c; k=ptys.len(); } }
            k=k+1; }
        if found.len()==0 {
            let mut comp2=false; let mut k3=0;
            while k3<ptys.len() && k3<atys.len(){
                if crate::oast::ends_with_seg(ptys[k3].as_str(), gps[g].as_str()) {
                    let base=&ptys[k3][0..ptys[k3].len()-gps[g].len()];
                    let mut ab=String::from(atys[k3].as_str()); ab.push('$');
                    if ab.as_str()==base { comp2=true; } }
                k3=k3+1; }
            if comp2 { found=String::from("int"); } }
        if found.len()==0 { return String::from(n); }
        concs.push(found); g=g+1; }
    // an UNBOUNDED single parameter used as a whole parameter type keeps the old untyped path, so nothing
    // that already worked is re-routed
    if cx.fgbs[gi].len()==0 {
        let mut comp=false; let mut q=0;
        while q<gps.len(){ let mut k2=0; while k2<ptys.len(){ if crate::oast::ends_with_seg(ptys[k2].as_str(), gps[q].as_str()) { comp=true; } k2=k2+1; } q=q+1; }
        if !comp && gps.len()<2 { return String::from(n); } }
    let mut r=String::from(n); let mut d=String::from(n); d.push('|');
    let mut c2=0;
    while c2<concs.len(){ r.push('$'); r.push_str(concs[c2].as_str());
        if c2>0 { d.push(','); } d.push_str(concs[c2].as_str()); c2=c2+1; }
    cx.demands.push(d);
    r
}
// lower `v`, wrapping it into a dyn fat pair when the TARGET type is `dyn Trait` and v's static type is an
// implementing struct. One place for the coercion, used by a dyn-annotated `let` and by a Vec<dyn T> push.
// the operators whose RESULT has the operands' type (a comparison answers a bool, so it never wraps)
pub fn cmp_op(o:&str)->bool{ o=="==" || o=="!=" || o=="<" || o=="<=" || o==">" || o==">=" || o=="&&" || o=="||" }
pub fn arith_op(o:&str)->bool{ o=="+" || o=="-" || o=="*" || o=="/" || o=="%" || o=="<<" || o==">>" || o=="&" || o=="|" || o=="^" }
pub fn lower_coerce(v:&E, cx:&mut LCx, target:&str)->H{
    let tr=dyn_trait_of(target);
    if tr.len()>0 { let at=ety_of(cx,v); let tg=dyn_tag(cx,&tr,&at);
        if tg>=0 { let z=fresh(cx); let tagh=H::Lit(tg as i64, z); let dat=lower(v,cx);
            let mut pr:Vec<H>=Vec::new(); pr.push(tagh); pr.push(dat); let i=fresh(cx); return H::StructLitH(2, pr, i); }
        // no impl of this trait names this concrete type: the value cannot carry a tag, so dispatching
        // through it would read a raw word as a {tag, payload} record. Emit a call the driver's own IR
        // verification cannot resolve, which turns a segfault into a diagnostic.
        { let dat=lower(v,cx); let mut pr:Vec<H>=Vec::new(); pr.push(dat); let i=fresh(cx);
          return H::CallH(String::from("$dyn_no_impl"), pr, i); } }
    lower(v,cx)
}
pub fn lower(e:&E, cx:&mut LCx)->H{
    match e {
        // the closure VALUE is a heap record of its captured values; its identity is in its TYPE
        E::ClosRef(n,caps,_l,_h)=>{ let mut lp:Vec<H>=Vec::new();
            let ci=clos_ix_of(n); let z=fresh(cx); lp.push(H::Lit(ci as i64, z));
            let mut k=0; while k<caps.len(){ let sl=slot_of(cx,&caps[k]); let i2=fresh(cx); lp.push(H::Local(sl, i2)); k=k+1; }
            let n2=lp.len() as u32; let i=fresh(cx); H::StructLitH(n2, lp, i) },
        E::TupLit(parts,_l,_h) if parts.len()==0 =>{ let i=fresh(cx); H::Lit(0, i) },
        E::TupLit(parts,_l,_h)=>{ let mut lp:Vec<H>=Vec::new(); let mut k=0; while k<parts.len(){ lp.push(lower(&parts[k],cx)); k=k+1; } let i=fresh(cx); H::StructLitH(parts.len() as u32, lp, i) },
        E::TupIdx(b,ix,_l,_h)=>{ let lb=lower(b,cx); let i=fresh(cx); H::FieldH(Box::new(lb), *ix, i) },
        E::Ref(x,_m,_l,_h)=>lower(x,cx),
        E::Cast(x,t,_l,_h)=>{ let lx=lower(x,cx); let i=fresh(cx); H::CastH(Box::new(lx), String::from(t.as_str()), i) },
        E::BreakE(_l,_h)=>{ let i=fresh(cx); H::BreakH(i) },
        E::ContE(_l,_h)=>{ let i=fresh(cx); H::ContH(i) },
        E::RetE(x,_l,_h)=>{ let lx=lower(x,cx); let i=fresh(cx); H::RetH(Box::new(lx), i) },
        E::StrLit(v,_l,_h)=>{ let i=fresh(cx); H::StrH(String::from(v.as_str()), i) },
        E::MethodCall(r,m,args,_l,_h)=>{
            let rt=ety_of(cx,r);
            if (m.as_str()=="to_string" || m.as_str()=="to_owned") && args.len()==0 {
                if rt.as_str()=="str" { return lower(r,cx); }
                if rt.as_str()=="f64" { let mut fa:Vec<H>=Vec::new(); fa.push(lower(r,cx)); let z=fresh(cx); fa.push(H::Lit(0,z)); let i=fresh(cx); return H::CallH(String::from("f64_str"), fa, i); }
                let mut la:Vec<H>=Vec::new(); la.push(lower(r,cx)); let i=fresh(cx); return H::CallH(String::from("str_from_int"), la, i); }
            { let bi=builtin_method(&rt,m,args.len());
              if bi.as_str()=="$id" { return lower(r,cx); }
              // C-ENUMCLONE: cloning an enum is the identity (an enum value is immutable here)
              if m.as_str()=="clone" && args.len()==0 && crate::oast::eidx(&cx.edefs,&rt)>=0 { return lower(r,cx); }
              // `s.push_str(x)` / `s.push(c)`: rustc's String is a growable owned buffer; ours is an
              // immutable value (ledger C8), so the growth is a re-assignment of the receiver's own slot --
              // which is what rustc's semantics come to for a local. Dispatched on the receiver TYPE,
              // because `v.push(x)` on a Vec is a different method entirely.
              // C-WRAPPING: the wrapping methods ARE the plain operators at a 64-bit width
              if bi.as_str()=="$wrap" {
                  let rv=lower(r,cx); let av=lower(&args[0],cx);
                  let o=if m.as_str()=="wrapping_mul" { String::from("*") } else if m.as_str()=="wrapping_sub" { String::from("-") } else { String::from("+") };
                  let i=fresh(cx); return H::BinH(o, Box::new(rv), Box::new(av), i); }
              if bi.as_str()=="$strgrow" {
                  let rv=lower(r,cx);
                  let av=lower(&args[0],cx);
                  let arg=if m.as_str()=="push" { let mut p1:Vec<H>=Vec::new(); p1.push(av); let i0=fresh(cx); H::CallH(String::from("chr1"), p1, i0) } else { av };
                  let mut ca:Vec<H>=Vec::new(); ca.push(rv); ca.push(arg); let i1=fresh(cx);
                  let cat=H::CallH(String::from("str_cat"), ca, i1);
                  match r.as_ref() { E::Var(n2,_a2,_b2)=>{ let sl=slot_of(cx,n2); let i2=fresh(cx); return H::AssignH(sl, Box::new(cat), i2); },
                                     _=>{ return cat; } } }
              if bi.len()>0 {
                // LAZINESS IS OBSERVABLE: rustc runs an adaptor's closure during the consuming call, so a
                // closure that PRINTS makes the output order part of the answer. Our adaptors are eager,
                // so such a chain is refused rather than silently reordered.
                { let mut q=0; while q<args.len(){
                    match &args[q] { E::ClosRef(cn,_c,_l2,_h2)=>{ let b=cn.as_bytes();
                        if b.len()>3 && b[b.len()-3]==36 && b[b.len()-2]==105 && b[b.len()-1]==111 {
                            let mut pr:Vec<H>=Vec::new(); let z=fresh(cx); pr.push(H::Lit(0,z));
                            let i=fresh(cx); return H::CallH(String::from("$lazy_io"), pr, i); } }, _=>{} }
                    q=q+1; } }
                // `sum`/`map`/`filter`/`fold` are generic PRELUDE functions, so the call needs the same
                // monomorphized name any other call to them would get
                let nm2={ let mut atys:Vec<String>=Vec::new(); atys.push(String::from(rt.as_str()));
                          let mut q=0; while q<args.len(){ atys.push(ety_of(cx,&args[q])); q=q+1; }
                          mono_name_tys(cx,&bi,&atys) };
                let mut la:Vec<H>=Vec::new(); la.push(lower(r,cx));
                let mut k=0; while k<args.len(){ la.push(lower(&args[k],cx)); k=k+1; }
                let i=fresh(cx); return H::CallH(nm2, la, i); } }
            { let tr=dyn_trait_of(&rt);
              if tr.len()>0 {
                // bind the receiver (and each argument) to a synthetic local so the dispatch chain can read
                // them once per branch without re-evaluating the expressions
                let rv=lower(r,cx);
                cx.names.push(String::from("$dynrecv")); cx.stypes.push(String::from(rt.as_str()));
                let rs=(cx.names.len()-1) as u32;
                let mut aslots:Vec<u32>=Vec::new(); let mut avals:Vec<H>=Vec::new();
                let mut k=0; while k<args.len(){ let av=lower(&args[k],cx); avals.push(av);
                    cx.names.push(String::from("$dynarg")); cx.stypes.push(String::from(""));
                    aslots.push((cx.names.len()-1) as u32); k=k+1; }
                let tys=dyn_types(cx,&tr);
                let i0=fresh(cx); let mut chain=H::Lit(0, i0);
                let mut q=tys.len();
                while q>0 { q=q-1;
                    let mut cn=String::from(tys[q].as_str()); cn.push_str("$"); cn.push_str(m);
                    let ri=fresh(cx); let rl=H::Local(rs, ri); let di=fresh(cx);
                    let mut cargs:Vec<H>=Vec::new(); cargs.push(H::FieldH(Box::new(rl), 1, di));
                    let mut j=0; while j<aslots.len(){ let aj=fresh(cx); cargs.push(H::Local(aslots[j], aj)); j=j+1; }
                    let ci=fresh(cx); let callh=H::CallH(cn, cargs, ci);
                    let ri2=fresh(cx); let rl2=H::Local(rs, ri2); let ti=fresh(cx); let tagh=H::FieldH(Box::new(rl2), 0, ti);
                    let li=fresh(cx); let lith=H::Lit(q as i64, li);
                    let bi=fresh(cx); let test=H::BinH(String::from("=="), Box::new(tagh), Box::new(lith), bi);
                    let ii=fresh(cx); chain=H::IfH(Box::new(test), Box::new(callh), Box::new(chain), ii); }
                // consume the lowered argument values from the END (H has no Clone), which is exactly the
                // order that nests the first argument outermost
                let mut body=chain; let mut w=aslots.len();
                while w>0 { w=w-1; match avals.pop() { Some(vv)=>{ let wi=fresh(cx); body=H::LetH(aslots[w], Box::new(vv), Box::new(body), wi); }, None=>{} } }
                let fi2=fresh(cx); return H::LetH(rs, Box::new(rv), Box::new(body), fi2); } }
            // C-UNNAMEDRECV: a receiver whose type lowering cannot NAME still answers the methods that
            // are representation-independent. Without this the mangled name came out as the empty type
            // name plus the method -- `$len`, then `$as_str` -- and nothing ever defined it, so the
            // driver's own IR check refused the program. A str receiver always names itself "str" and an
            // array "arr", so the only receivers that reach here are vector-shaped or genuinely unknown.
            // ORIGINAL NOTE: a receiver whose type lowering cannot NAME still has a length if it is a
            // sequence. Without this the mangled name came out as `$len` -- the empty type name, a `$`
            // and the method -- and nothing ever defined it, so the driver's own IR check refused the
            // program. A str receiver always names itself "str" and an array "arr", so the only receivers
            // that reach here are vector-shaped or genuinely unknown, which is exactly what seq_len
            // dispatches on.
            if rt.len()==0 && args.len()==0 {
                // the PASS-THROUGH methods are the identity on any receiver, named or not
                if ident_method(m) { return lower(r,cx); }
                // the MUTATORS on an unnamed receiver. A str receiver always names itself "str", so an
                // unnamed `push`/`pop` is vector-shaped; `push_str` is the one that can only be a str, and
                // it takes the same re-assignment path the named case uses (C8: our String is a value).
                if m.as_str()=="push" && args.len()==1 {
                    let lr=lower(r,cx); let la=lower(&args[0],cx);
                    let mut a2:Vec<H>=Vec::new(); a2.push(lr); a2.push(la); let iq=fresh(cx);
                    return H::CallH(String::from("vec_push"), a2, iq); }
                if m.as_str()=="pop" && args.len()==0 {
                    let lr=lower(r,cx); let mut a2:Vec<H>=Vec::new(); a2.push(lr); let iq=fresh(cx);
                    return H::CallH(String::from("vec_popraw"), a2, iq); }
                if m.as_str()=="len" {
                    let lr=lower(r,cx); let mut a1:Vec<H>=Vec::new(); a1.push(lr); let iq=fresh(cx);
                    return H::CallH(String::from("vec_len"), a1, iq); } }
            let mut nm=String::from(rt.as_str()); nm.push_str("$"); nm.push_str(m);
            // the same fallback `ety_of` already applies when deriving the RETURN type: a receiver whose
            // concrete type is not known (an `impl Trait` parameter, a lifted closure's parameter) still
            // dispatches when the method name is unambiguous. Without it the CALL named `$m` while the
            // return type resolved fine -- one rule, and only one of its two sites had it.
            if fidx_of(cx,&nm)<0 { let u=unique_method(cx,m); if u.len()>0 { nm=u; } }
            let mut la:Vec<H>=Vec::new(); la.push(lower(r,cx));
            let mut k=0; while k<args.len(){ la.push(lower(&args[k],cx)); k=k+1; }
            let i=fresh(cx); H::CallH(nm, la, i) },
        E::AssignTo(pl,v,_l,_h)=>{
            match &**pl {
                E::Call(n,a2,_a,_b2) if is_gname(n) && a2.len()==0 =>{
                    let lv=lower(v,cx); let mut sa:Vec<H>=Vec::new(); sa.push(lv);
                    let mut sn=String::from("$gstore$"); sn.push_str(gname_of(n).as_str());
                    let i=fresh(cx); H::CallH(sn, sa, i) },
                E::Index(b,ix,_a,_b2) if is_vec_ty(&ety_of(cx,b)) =>{ let lb=lower(b,cx); let li=lower(ix,cx); let lv=lower(v,cx); let mut ag:Vec<H>=Vec::new(); ag.push(lb); ag.push(li); ag.push(lv); let i=fresh(cx); H::CallH(String::from("vec_set"), ag, i) },
                E::Index(b,ix,_a,_b2)=>{ let lb=lower(b,cx); let li=lower(ix,cx); let lv=lower(v,cx); let i=fresh(cx); H::StoreIdxH(Box::new(lb), Box::new(li), Box::new(lv), i) },
                E::Field(o,f,_a,_b2)=>{ let sn=ety_of(cx,o); let idx=crate::oast::sfield_idx(&cx.sdefs,&sn,f); let ix=if idx<0 {0} else {idx as u32}; let lo2=lower(o,cx); let lv=lower(v,cx); let i=fresh(cx); H::StoreFieldH(Box::new(lo2), ix, Box::new(lv), i) },
                E::Var(n,_a,_b2)=>{ let lv=lower(v,cx); let s=slot_of(cx,n); let i=fresh(cx); H::AssignH(s, Box::new(lv), i) },
                _=>{ let lv=lower(v,cx); lv },
            } },
        E::ArrLit(xs,_l,_h)=>{ let mut le:Vec<H>=Vec::new(); let mut k=0; while k<xs.len(){ le.push(lower(&xs[k],cx)); k=k+1; } let i=fresh(cx); H::ArrLitH(le, i) },
        E::Index(b,ix,_l,_h)=>{
            // a BYTE VIEW reads the string itself
            if ety_of(cx,b).as_str()=="bstr" { let lb=lower(b,cx); let li=lower(ix,cx);
                let mut ab:Vec<H>=Vec::new(); ab.push(lb); ab.push(li); let i=fresh(cx);
                return H::CallH(String::from("str_at"), ab, i); }
            // a VECTOR index is vec_get; a fixed array keeps the direct slot read
            if is_vec_ty(&ety_of(cx,b)) { let lb=lower(b,cx); let li=lower(ix,cx); let mut ag:Vec<H>=Vec::new(); ag.push(lb); ag.push(li); let i=fresh(cx); return H::CallH(String::from("vec_get"), ag, i); }
            let lb=lower(b,cx); let li=lower(ix,cx); let i=fresh(cx); H::IndexH(Box::new(lb), Box::new(li), i) },
        E::EnumLit(en,v,ps,_l,_h)=>{ let tag=crate::oast::vidx(&cx.edefs,en,v); let t2=if tag<0 {0} else {tag as u32}; let mut lp:Vec<H>=Vec::new(); let mut k=0; while k<ps.len(){ lp.push(lower(&ps[k],cx)); k=k+1; }
            { let mx=crate::oast::max_payload(&cx.edefs,en); while (lp.len() as u32)<mx { let z=fresh(cx); lp.push(H::Lit(0, z)); } }
            let i=fresh(cx); H::EnumLitH(t2, lp, i) },
        // `x?` desugars HERE, where the value's enum is known: bind it once, then match -- the success
        // variant yields its payload, every other variant RETURNS the value unchanged.
        E::Try(x,_l,_h)=>{
            let en=ety_of(cx,x); let lv=lower(x,cx);
            cx.names.push(String::from("$try")); cx.stypes.push(String::from(en.as_str()));
            let ts=(cx.names.len()-1) as u32;
            let nv=crate::oast::nvariants(&cx.edefs,&en);
            let mut ha:Vec<HArm>=Vec::new(); let mut k=0;
            while k<nv { let vn=crate::oast::variant_name(&cx.edefs,&en,k as usize);
                let np=crate::oast::variant_payload_n(&cx.edefs,&en,k as usize);
                let base=cx.names.len() as u32; let mut sl:Vec<u32>=Vec::new();
                let mut bi=0; while bi<np { cx.names.push(String::from("$tp")); cx.stypes.push(String::from("")); sl.push(base+bi); bi=bi+1; }
                let body=if crate::oast::is_ok_variant(&vn) && np>0 { let i2=fresh(cx); H::Local(base, i2) }
                         else { let i3=fresh(cx); let lv2=H::Local(ts, i3); let i4=fresh(cx); H::RetH(Box::new(lv2), i4) };
                let mut bp=0; while bp<np { let _=cx.names.pop(); let _=cx.stypes.pop(); bp=bp+1; }
                ha.push(HArm{tag:k, nbinds:np, slots:sl, body:body}); k=k+1; }
            let mi=fresh(cx); let mh=H::MatchH(Box::new(H::Local(ts, fresh(cx))), ha, mi);
            let li=fresh(cx); let r=H::LetH(ts, Box::new(lv), Box::new(mh), li);
            let _=cx.names.pop(); let _=cx.stypes.pop(); r },
        E::Match(sc,arms,_l,_h) if { let en0=ety_of(cx,sc); crate::oast::nvariants(&cx.edefs,&en0)<0 || arms_have_lit_or_guard(arms) } =>{
            // IF-CHAIN match: integer literals, `_`, and GUARDS (MatchOp dispatches on a tag and has no
            // fallthrough, so a guarded arm cannot be expressed there). An ENUM scrutinee is still handled
            // exactly: the test is `tag == variant` and the payload bindings are read out of the value's
            // fields (layout [tag, payload..]), so `Some(v) if v > 3` both tests and binds.
            let en=ety_of(cx,sc);
            let is_en=crate::oast::nvariants(&cx.edefs,&en)>=0;
            let lv=lower(sc,cx);
            cx.names.push(String::from("$mscrut")); cx.stypes.push(String::from(en.as_str()));
            let ss2=(cx.names.len()-1) as u32;
            let z0=fresh(cx); let mut chain=H::Lit(0, z0);
            let mut q=arms.len();
            while q>0 { q=q-1;
                let nb=arms[q].binds.len();
                // bind the payloads first so the guard and the body can both see them
                let base=cx.names.len() as u32; let mut bi=0;
                while bi<nb { cx.names.push(String::from(arms[q].binds[bi].as_str())); cx.stypes.push(crate::oast::vpayload_ty(&cx.edefs,&en,&arms[q].variant,bi)); bi=bi+1; }
                let body0=lower(&arms[q].body,cx);
                let mut test=H::Lit(1, fresh(cx));
                if arms[q].has_slit { let si=fresh(cx); let sv=H::Local(ss2, si); let li=fresh(cx); let lit=H::StrH(String::from(arms[q].slit.as_str()), li);
                    let mut ag:Vec<H>=Vec::new(); ag.push(sv); ag.push(lit); let ci=fresh(cx); test=H::CallH(String::from("streq"), ag, ci); }
                else if arms[q].has_lit { let li=fresh(cx); let lit=H::Lit(arms[q].lit, li); let si=fresh(cx); let sv=H::Local(ss2, si); let b2=fresh(cx); test=H::BinH(String::from("=="), Box::new(sv), Box::new(lit), b2); }
                else if is_en && !arms[q].wild { let tg=crate::oast::vidx(&cx.edefs,&en,&arms[q].variant);
                    let si=fresh(cx); let sv=H::Local(ss2, si); let f0=fresh(cx); let tagh=H::FieldH(Box::new(sv), 0, f0);
                    let li=fresh(cx); let lit=H::Lit(tg as i64, li); let b2=fresh(cx);
                    test=H::BinH(String::from("=="), Box::new(tagh), Box::new(lit), b2); }
                if arms[q].has_guard { let g=lower(&arms[q].guard,cx);
                    let hasprev=arms[q].has_lit || arms[q].has_slit || (is_en && !arms[q].wild);
                    if hasprev { let ii=fresh(cx); let zz=H::Lit(0, fresh(cx)); test=H::IfH(Box::new(test), Box::new(g), Box::new(zz), ii); } else { test=g; } }
                let mut bp=0; while bp<nb { let _=cx.names.pop(); let _=cx.stypes.pop(); bp=bp+1; }
                // the payload reads wrap TEST AND BODY: a guard like `S(v) if v > 100` must see `v`
                let ii2=fresh(cx); let mut arm=H::IfH(Box::new(test), Box::new(body0), Box::new(chain), ii2);
                let mut w=nb;
                while w>0 { w=w-1; let si=fresh(cx); let sv=H::Local(ss2, si); let f1=fresh(cx);
                    let fld=H::FieldH(Box::new(sv), (w+1) as u32, f1); let wi=fresh(cx);
                    arm=H::LetH(base+(w as u32), Box::new(fld), Box::new(arm), wi); }
                chain=arm; }
            let li2=fresh(cx); let r=H::LetH(ss2, Box::new(lv), Box::new(chain), li2);
            let _=cx.names.pop(); let _=cx.stypes.pop(); r },
        E::Match(sc,arms,_l,_h)=>{
            let ls=lower(sc,cx);
            let en=ety_of(cx,sc);
            let mut ha:Vec<HArm>=Vec::new();
            let mut k=0;
            while k<arms.len(){
                let a=&arms[k];
                let tg = if a.wild { -1 } else { crate::oast::vidx(&cx.edefs,&en,&a.variant) };
                // bind payload names as fresh LOCAL slots, visible in the arm body
                let base=cx.names.len() as u32;
                let mut sl:Vec<u32>=Vec::new(); let mut bi=0;
                while bi<a.binds.len(){ cx.names.push(String::from(a.binds[bi].as_str())); cx.stypes.push(crate::oast::vpayload_ty(&cx.edefs,&en,&a.variant,bi)); sl.push(base + (bi as u32)); bi=bi+1; }
                let lb=lower(&a.body,cx);
                let mut bp=0; while bp<a.binds.len(){ let _=cx.names.pop(); let _=cx.stypes.pop(); bp=bp+1; }
                ha.push(HArm{tag:tg, nbinds:a.binds.len() as u32, slots:sl, body:lb});
                k=k+1;
            }
            let i=fresh(cx); H::MatchH(Box::new(ls), ha, i) },
        E::Not(x,_l,_h)=>{
            // C-BITNOT: on an INTEGER operand `!` is the bitwise not, which two's complement gives as
            // 0-x-1; on a bool it stays the logical not, x==0. Telling them apart needs the operand's
            // kind, which C-PSFILTER and C-ETYARITH made available.
            let tx=ety_of(cx,x);
            if tx.as_str()=="int" || crate::oast::is_int_width(&tx) {
                let lx=lower(x,cx); let z=fresh(cx); let zero=H::Lit(0,z); let i1=fresh(cx);
                let neg=H::BinH(String::from("-"), Box::new(zero), Box::new(lx), i1);
                let o=fresh(cx); let one=H::Lit(1,o); let i2=fresh(cx);
                return H::BinH(String::from("-"), Box::new(neg), Box::new(one), i2); }
            let lx=lower(x,cx); let z=fresh(cx); let zero=H::Lit(0,z); let i=fresh(cx); H::BinH(String::from("=="), Box::new(lx), Box::new(zero), i) },
        E::StructLit(n,fs,_l,_h)=>{ let mut lf:Vec<H>=Vec::new(); let mut k=0; while k<fs.len(){ lf.push(lower(&fs[k],cx)); k=k+1; } let i=fresh(cx); H::StructLitH(fs.len() as u32, lf, i) },
        E::Field(o,f,_l,_h)=>{ let sn=ety_of(cx,o); let idx=crate::oast::sfield_idx(&cx.sdefs, &sn, f); let ix=if idx<0 {0} else {idx as u32}; let lo2=lower(o,cx); let i=fresh(cx); H::FieldH(Box::new(lo2), ix, i) },
        E::While(c,b,_l,_h)=>{ let lc=lower(c,cx); let lb=lower(b,cx); let i=fresh(cx); H::WhileH(Box::new(lc), Box::new(lb), i) },
        E::Seq(a,b,_l,_h)=>{ let la=lower(a,cx); let lb=lower(b,cx); let i=fresh(cx); H::SeqH(Box::new(la), Box::new(lb), i) },
        E::Assign(n,v,_l,_h)=>{ let lv=lower(v,cx); let s=slot_of(cx,n); let i=fresh(cx); H::AssignH(s, Box::new(lv), i) },
        // println!'s desugar emits print_any(x): the primitive is chosen HERE, where the type is known
        E::Call(n,args,_l,_h) if n.as_str()=="seq_set" && args.len()==3 =>{
            let st=ety_of(cx,&args[0]);
            if st.as_str()=="arr" { let lb=lower(&args[0],cx); let li=lower(&args[1],cx); let lv=lower(&args[2],cx); let i=fresh(cx); return H::StoreIdxH(Box::new(lb), Box::new(li), Box::new(lv), i); }
            let mut la:Vec<H>=Vec::new(); let mut k=0; while k<args.len(){ la.push(lower(&args[k],cx)); k=k+1; }
            let i=fresh(cx); H::CallH(String::from("vec_set"), la, i) },
        // an f64 IS our 64-bit word, so reading its bits is a no-op -- which makes a bit-exact
        // round-trip comparison an ordinary integer `==`
        E::Call(n,args,_l,_h) if n.as_str()=="f64_bits" && args.len()==1 =>lower(&args[0],cx),
        // P-PLACE: a global load/store rides as a NAMED call, the way every other primitive here does
        E::Call(n,args,_l,_h) if n.as_str()=="argc" && args.len()==0 =>{ let a:Vec<H>=Vec::new(); let i=fresh(cx); H::CallH(String::from("argc"), a, i) },
        E::Call(n,args,_l,_h) if n.as_str()=="argv_at" && args.len()==1 =>{
            let mut a:Vec<H>=Vec::new(); a.push(lower(&args[0],cx)); let i=fresh(cx); H::CallH(String::from("argv_at"), a, i) },
        E::Call(n,args,_l,_h) if n.as_str()=="fileok" && args.len()==1 =>{
            let mut a:Vec<H>=Vec::new(); a.push(lower(&args[0],cx)); let i=fresh(cx); H::CallH(String::from("fileok"), a, i) },
        E::Call(n,args,_l,_h) if n.as_str()=="readfile" && args.len()==1 =>{
            let mut a:Vec<H>=Vec::new(); a.push(lower(&args[0],cx)); let i=fresh(cx); H::CallH(String::from("readfile"), a, i) },
        E::Call(n,args,_l,_h) if n.as_str()=="proc_exit" && args.len()==1 =>{
            let mut a:Vec<H>=Vec::new(); a.push(lower(&args[0],cx)); let i=fresh(cx); H::CallH(String::from("proc_exit"), a, i) },
        E::Call(n,args,_l,_h) if is_gname(n) =>{
            let mut la:Vec<H>=Vec::new(); let mut k=0; while k<args.len(){ la.push(lower(&args[k],cx)); k=k+1; }
            let i=fresh(cx); H::CallH(String::from(n.as_str()), la, i) },
        // S11 (probed from RC): `&s[a..b]` on a str is a SUBSTRING. It used to reach the vector slicer
        // with a string pointer, which segfaulted -- a divergence, not a gap.
        // `seq_len` on a STR is `len`: seq_len reads the length slot of a sequence handle, which for a
        // string is the first bytes of its text (the open-end slice `&s[a..]` emitted exactly this)
        E::Call(n,args,_l,_h) if n.as_str()=="seq_len" && args.len()==1 && ety_of(cx,&args[0]).as_str()=="str" =>{
            let lx=lower(&args[0],cx); let mut a:Vec<H>=Vec::new(); a.push(lx); let i=fresh(cx);
            H::CallH(String::from("len"), a, i) },
        E::Call(n,args,_l,_h) if n.as_str()=="seq_slice" && args.len()==3 && ety_of(cx,&args[0]).as_str()=="str" =>{
            let ls=lower(&args[0],cx); let lo2=lower(&args[1],cx); let hi2=lower(&args[2],cx);
            let i1=fresh(cx); let cnt=H::BinH(String::from("-"), Box::new(hi2), Box::new(lo2), i1);
            // the offset is needed twice, so it is bound once
            let lo3=lower(&args[1],cx);
            let mut a:Vec<H>=Vec::new(); a.push(ls); a.push(lo3); a.push(cnt);
            let i2=fresh(cx); H::CallH(String::from("str_sub"), a, i2) },
        E::Call(n,args,_l,_h) if n.as_str()=="seq_slice" && args.len()==3 =>{
            // a slice of a vector or of an array; both answer with a fresh vector
            let st=ety_of(cx,&args[0]);
            let prim=if st.as_str()=="arr" { String::from("arr_slice") } else { String::from("vec_slice") };
            let mut la:Vec<H>=Vec::new(); let mut k=0; while k<args.len(){ la.push(lower(&args[k],cx)); k=k+1; }
            let i=fresh(cx); H::CallH(prim, la, i) },
        E::Call(n,args,_l,_h) if (n.as_str()=="seq_len" && args.len()==1) || (n.as_str()=="seq_get" && args.len()==2) =>{
            // a `for` loop works over a vector, a fixed array, OR a byte view of a str; the primitive is
            // chosen here. The byte view was missing, so `for b in s.as_bytes()` fell through to the
            // VECTOR primitives and read a C string as a vector handle -- accepted, then a crash at run
            // time, which breaks the accepted==ran invariant. (C-SEQBSTR)
            let st=ety_of(cx,&args[0]);
            if st.as_str()=="bstr" {
                let lb=lower(&args[0],cx);
                if n.as_str()=="seq_len" { let mut a:Vec<H>=Vec::new(); a.push(lb); let i=fresh(cx); return H::CallH(String::from("len"), a, i); }
                let li=lower(&args[1],cx); let mut a:Vec<H>=Vec::new(); a.push(lb); a.push(li); let i=fresh(cx);
                return H::CallH(String::from("str_at"), a, i); }
            let isarr=st.as_str()=="arr";
            let prim=if n.as_str()=="seq_len" { if isarr { String::from("arr_len") } else { String::from("vec_len") } }
                     else { if isarr { String::from("$arrget") } else { String::from("vec_get") } };
            if prim.as_str()=="$arrget" { let lb=lower(&args[0],cx); let li=lower(&args[1],cx); let i=fresh(cx); return H::IndexH(Box::new(lb), Box::new(li), i); }
            let mut la:Vec<H>=Vec::new(); let mut k=0; while k<args.len(){ la.push(lower(&args[k],cx)); k=k+1; }
            let i=fresh(cx); H::CallH(prim, la, i) },
        E::Call(n,args,_l,_h) if n.as_str()=="str_of" && args.len()==1 =>{
            let at=ety_of(cx,&args[0]);
            if at.as_str()=="str" { return lower(&args[0],cx); }
            let mut la:Vec<H>=Vec::new(); la.push(lower(&args[0],cx)); let i=fresh(cx); H::CallH(String::from("str_from_int"), la, i) },
        E::Call(n,args,_l,_h) if (n.as_str()=="print_any" || n.as_str()=="print_dbg") && args.len()==1 =>{
            let at=ety_of(cx,&args[0]);
            let prim=if at.as_str()=="str" { String::from("print_raw") }
                     else if at.as_str()=="char" { String::from("print_char") }
                     else if at.as_str()=="f64" { if n.as_str()=="print_dbg" { String::from("$f64dbg") } else { String::from("$f64disp") } }
                     else { String::from("print_int_raw") };
            // the formatter is written in OUR language, in the prelude: it needs a round-trip search and
            // string trimming, neither of which belongs in hand-written IR
            // D1 (probed from RC): `{:?}` QUOTES -- a char as `'a'`, a str as `"hi"`. `{}` does not.
            // D1 (probed from RC): the quotes AND the escaping are the printer's job, and dbg_scalar is where
            // that rule lives -- it was kept here a second time, so a str's `{:?}` did not escape while the
            // same str inside a tuple did
            if n.as_str()=="print_dbg" && (at.as_str()=="char" || at.as_str()=="str") {
                let v0=lower(&args[0],cx); return dbg_scalar(cx, v0, at.as_str()); }
            if at.as_str()=="bool" {
                let c=lower(&args[0],cx); let i1=fresh(cx); let t1=H::StrH(String::from("true"), i1);
                let i2=fresh(cx); let f1=H::StrH(String::from("false"), i2);
                let i3=fresh(cx); let pick=H::IfH(Box::new(c), Box::new(t1), Box::new(f1), i3);
                let mut pa:Vec<H>=Vec::new(); pa.push(pick); let i4=fresh(cx);
                return H::CallH(String::from("print_raw"), pa, i4); }
            if n.as_str()=="print_dbg" {
                let d=dbg_emit(cx, &args[0], &at);
                if d.len()>0 { return dbg_build(cx, &args[0], &at); }
                // a non-scalar whose Debug shape we cannot emit must be REFUSED, not printed as a word
                if at.len()>0 && at.as_str()!="int" && at.as_str()!="bool" && at.as_str()!="str" && at.as_str()!="char" && at.as_str()!="f64" && !crate::oast::is_int_width(&at) {
                    let mut pr:Vec<H>=Vec::new(); let z=fresh(cx); pr.push(H::Lit(0,z));
                    let i=fresh(cx); return H::CallH(String::from("$dbg_unsupported"), pr, i); } }
            if prim.as_str()=="$f64disp" || prim.as_str()=="$f64dbg" {
                let dbg=if prim.as_str()=="$f64dbg" { 1 } else { 0 };
                let mut fa:Vec<H>=Vec::new(); fa.push(lower(&args[0],cx));
                let z=fresh(cx); fa.push(H::Lit(dbg, z));
                let i2=fresh(cx); let st=H::CallH(String::from("f64_str"), fa, i2);
                let mut pa:Vec<H>=Vec::new(); pa.push(st); let i3=fresh(cx);
                return H::CallH(String::from("print_raw"), pa, i3); }
            let mut la:Vec<H>=Vec::new(); la.push(lower(&args[0],cx)); let i=fresh(cx); H::CallH(prim, la, i) },
        E::Call(n,args,_l,_h) if n.as_str()=="vec_push" && args.len()==2 =>{
            let vt=ety_of(cx,&args[0]); let el=vec_elem_of(&vt);
            let a0=lower(&args[0],cx); let a1=if el.len()>0 { lower_coerce(&args[1],cx,&el) } else { lower(&args[1],cx) };
            let mut la:Vec<H>=Vec::new(); la.push(a0); la.push(a1); let i=fresh(cx); H::CallH(String::from("vec_push"), la, i) },
        E::Call(n,args,_l,_h) if sty_of(cx,n).as_str()=="Fn" =>{
            // the closure's identity is not known statically here, so dispatch on the tag the value carries
            let sl=slot_of(cx,n);
            let mut aslots:Vec<u32>=Vec::new(); let mut avals:Vec<H>=Vec::new();
            let mut k=0; while k<args.len(){ let av=lower(&args[k],cx); avals.push(av);
                cx.names.push(String::from("$fnarg")); cx.stypes.push(String::from("")); aslots.push((cx.names.len()-1) as u32); k=k+1; }
            let mut ids:Vec<i32>=Vec::new(); let mut q=0;
            while q<cx.fnames.len(){ let ci=clos_name_ix(cx.fnames[q].as_str()); if ci>=0 { ids.push(ci); } q=q+1; }
            let z0=fresh(cx); let mut chain=H::Lit(0, z0);
            let mut w=ids.len();
            while w>0 { w=w-1; let ci=ids[w];
                let mut cn=String::from("$clos"); cn.push_str(&itoa_u(ci as u32));
                let fi=fidx_of(cx,&cn); let nc=if fi>=0 { cx.fncaps[fi as usize] } else { 0 };
                let mut ca:Vec<H>=Vec::new(); let mut j=0;
                while j<nc { let li=fresh(cx); let lv=H::Local(sl, li); let f2=fresh(cx); ca.push(H::FieldH(Box::new(lv), (j+1) as u32, f2)); j=j+1; }
                let mut a2=0; while a2<aslots.len(){ let ai=fresh(cx); ca.push(H::Local(aslots[a2], ai)); a2=a2+1; }
                let callid=fresh(cx); let callh=H::CallH(cn, ca, callid);
                let li2=fresh(cx); let lv2=H::Local(sl, li2); let t0=fresh(cx); let tagh=H::FieldH(Box::new(lv2), 0, t0);
                let l0=fresh(cx); let lith=H::Lit(ci as i64, l0);
                let b0=fresh(cx); let test=H::BinH(String::from("=="), Box::new(tagh), Box::new(lith), b0);
                let i0=fresh(cx); chain=H::IfH(Box::new(test), Box::new(callh), Box::new(chain), i0); }
            let mut body=chain; let mut v2=aslots.len();
            while v2>0 { v2=v2-1; match avals.pop() { Some(vv)=>{ let wi=fresh(cx); body=H::LetH(aslots[v2], Box::new(vv), Box::new(body), wi); }, None=>{} } }
            body },
        E::Call(n,args,_l,_h) if clos_of(&sty_of(cx,n)).len()>0 =>{
            let cn=clos_of(&sty_of(cx,n)); let fi=fidx_of(cx,&cn);
            let nc=if fi>=0 { cx.fncaps[fi as usize] } else { 0 };
            let sl=slot_of(cx,n);
            let mut la:Vec<H>=Vec::new(); let mut k=0;
            while k<nc { let li=fresh(cx); let lv=H::Local(sl, li); let fi2=fresh(cx); la.push(H::FieldH(Box::new(lv), (k+1) as u32, fi2)); k=k+1; }
            let mut j=0; while j<args.len(){ la.push(lower(&args[j],cx)); j=j+1; }
            let i=fresh(cx); H::CallH(cn, la, i) },
        E::Call(n0,args,_l,_h)=>{ let n=&self_fix(cx,n0); let nm=mono_name(cx,n,args);
            let fi=fidx_of(cx,&nm); let ptys=if fi>=0 { split_c(cx.fptys[fi as usize].as_str()) } else { Vec::new() };
            let mut la:Vec<H>=Vec::new(); let mut k=0;
            while k<args.len(){
                let want=if k<ptys.len() { String::from(ptys[k].as_str()) } else { String::from("") };
                let tr=dyn_trait_of(&want);
                if tr.len()>0 { let at=ety_of(cx,&args[k]); let tg=dyn_tag(cx,&tr,&at);
                    if tg>=0 { let z=fresh(cx); let tagh=H::Lit(tg as i64, z); let dat=lower(&args[k],cx);
                        let mut pr:Vec<H>=Vec::new(); pr.push(tagh); pr.push(dat); let i2=fresh(cx);
                        la.push(H::StructLitH(2, pr, i2)); k=k+1; continue; } }
                la.push(lower(&args[k],cx)); k=k+1; }
            let i=fresh(cx); H::CallH(nm, la, i) },
        E::Int(v,_l,_h)=>{ let i=fresh(cx); H::Lit(*v, i) },
        E::Bool(v,_l,_h)=>{ let i=fresh(cx); H::LitB(*v, i) },
        E::Var(n,_l,_h)=>{ let s=slot_of(cx,n); let i=fresh(cx); H::Local(s, i) },
        E::Bin(o,a,b,_l,_h)=>{
            // OPERATOR OVERLOADING: a struct operand whose type has the operator's method (Add::add etc.)
            // routes to `Type$method`. Checked before the integer path, and only when that method exists, so
            // every primitive operation is untouched.
            { let at=ety_of(cx,a); let m=op_method(o);
              if at.len()>0 && m.len()>0 { let mut mn=String::from(at.as_str()); mn.push('$'); mn.push_str(&m);
                if fidx_of(cx,&mn)>=0 { let la=lower(a,cx); let lb=lower(b,cx); let mut ag:Vec<H>=Vec::new(); ag.push(la); ag.push(lb); let i=fresh(cx); return H::CallH(mn, ag, i); } } }
            // `==`/`!=` on str compares CONTENT, so it lowers to a streq call (strcmp in codegen), not an
            // integer compare of two addresses. Keyed on ety_of, which now reports "str" for a literal, a
            // str-typed param, and a let bound to either.
            // O12: an ENUM compares structurally. Every payload must be word-comparable; a payload we
            // cannot compare that way (a str, whose equality is by CONTENT) is refused, not guessed.
            { let ea=ety_of(cx,a); let eb=ety_of(cx,b);
              let en=if crate::oast::eidx(&cx.edefs,&ea)>=0 { ea } else if crate::oast::eidx(&cx.edefs,&eb)>=0 { eb } else { String::from("") };
              if (o.as_str()=="==" || o.as_str()=="!=") && en.len()>0 {
                let ei=crate::oast::eidx(&cx.edefs,&en) as usize;
                let mut wordy=true;
                { let ed=&cx.edefs[ei]; let mut vi=0;
                  while vi<ed.variants.len(){ let mut pi=0;
                      while pi<ed.variants[vi].ptys.len(){ let t=&ed.variants[vi].ptys[pi];
                          if !(t.as_str()=="int" || t.as_str()=="bool" || t.as_str()=="char" || crate::oast::is_int_width(t) || t.len()==0) { wordy=false; }
                          pi=pi+1; }
                      vi=vi+1; } }
                if wordy {
                    let n=1+crate::oast::max_payload(&cx.edefs,&en);
                    let la=lower(a,cx); let lb=lower(b,cx);
                    let z=fresh(cx); let nn=H::Lit(n as i64, z);
                    let mut ag:Vec<H>=Vec::new(); ag.push(la); ag.push(lb); ag.push(nn);
                    let i=fresh(cx); let c=H::CallH(String::from("enumeq"), ag, i);
                    if o.as_str()=="==" { return c; }
                    let z2=fresh(cx); let zero=H::Lit(0,z2); let i2=fresh(cx);
                    return H::BinH(String::from("=="), Box::new(c), Box::new(zero), i2); }
                else { let mut pr:Vec<H>=Vec::new(); let z3=fresh(cx); pr.push(H::Lit(0,z3));
                       let i3=fresh(cx); return H::CallH(String::from("$enumeq_unsupported"), pr, i3); } } }
            if (o.as_str()=="==" || o.as_str()=="!=") && (ety_of(cx,a).as_str()=="arr" || ety_of(cx,b).as_str()=="arr") {
                // two ARRAYS compare by CONTENTS (they carry their length at slot 0)
                let la=lower(a,cx); let lb=lower(b,cx); let mut ag:Vec<H>=Vec::new(); ag.push(la); ag.push(lb);
                let i=fresh(cx); let c=H::CallH(String::from("arreq"), ag, i);
                if o.as_str()=="==" { return c; }
                let z=fresh(cx); let zero=H::Lit(0,z); let i2=fresh(cx); return H::BinH(String::from("=="), Box::new(c), Box::new(zero), i2); }
            if (o.as_str()=="==" || o.as_str()=="!=") && (is_vec_ty(&ety_of(cx,a)) || is_vec_ty(&ety_of(cx,b))) {
                // two VECTORS compare by CONTENTS, not by handle
                let la=lower(a,cx); let lb=lower(b,cx); let mut ag:Vec<H>=Vec::new(); ag.push(la); ag.push(lb);
                let i=fresh(cx); let c=H::CallH(String::from("veceq"), ag, i);
                if o.as_str()=="==" { return c; }
                let z=fresh(cx); let zero=H::Lit(0,z); let i2=fresh(cx); return H::BinH(String::from("=="), Box::new(c), Box::new(zero), i2); }
            // S14: strs ORDER lexicographically; we used to refuse the comparison outright
            if (o.as_str()=="<" || o.as_str()=="<=" || o.as_str()==">" || o.as_str()==">=") && (ety_of(cx,a).as_str()=="str" || ety_of(cx,b).as_str()=="str") {
                let la=lower(a,cx); let lb=lower(b,cx); let mut ag:Vec<H>=Vec::new(); ag.push(la); ag.push(lb);
                let i=fresh(cx); let c=H::CallH(String::from("scmp"), ag, i);
                let z=fresh(cx); let zero=H::Lit(0,z); let i2=fresh(cx);
                return H::BinH(String::from(o.as_str()), Box::new(c), Box::new(zero), i2); }
            if (o.as_str()=="==" || o.as_str()=="!=") && (ety_of(cx,a).as_str()=="str" || ety_of(cx,b).as_str()=="str") {
                let la=lower(a,cx); let lb=lower(b,cx); let mut ag:Vec<H>=Vec::new(); ag.push(la); ag.push(lb);
                let i=fresh(cx); let c=H::CallH(String::from("streq"), ag, i);
                if o.as_str()=="==" { return c; }
                let z=fresh(cx); let zero=H::Lit(0,z); let i2=fresh(cx); return H::BinH(String::from("=="), Box::new(c), Box::new(zero), i2); }
            // F-1: either operand being an f64 selects the FLOAT operation. The word is unchanged --
            // an f64 is exactly 64 bits -- so only the opcode differs, and the name that selects it is
            // the one P-NOM already carries.
            { let fa=ety_of(cx,a).as_str()=="f64"; let fb=ety_of(cx,b).as_str()=="f64";
              // `-x` desugars to `0 - x`; on a float that must be a SIGN FLIP, because `0.0 - 0.0` is
              // +0.0 while Rust's `-0.0` is negative zero (and Display spells it `-0`)
              if fb && o.as_str()=="-" { let isz=match a.as_ref() { E::Int(0,_l2,_h2)=>true, _=>false };
                  if isz { let lb=lower(b,cx); let z=fresh(cx); let sgn=H::Lit(0-9223372036854775807-1, z);
                      let i=fresh(cx); return H::BinH(String::from("^"), Box::new(lb), Box::new(sgn), i); } }
              if fa || fb {
                let fo=if o.as_str()=="+" { String::from("f+") } else if o.as_str()=="-" { String::from("f-") }
                       else if o.as_str()=="*" { String::from("f*") } else if o.as_str()=="/" { String::from("f/") }
                       else if o.as_str()=="%" { String::from("f%") } else if o.as_str()=="==" { String::from("f==") }
                       else if o.as_str()=="!=" { String::from("f!=") } else if o.as_str()=="<" { String::from("f<") }
                       else if o.as_str()=="<=" { String::from("f<=") } else if o.as_str()==">" { String::from("f>") }
                       else if o.as_str()==">=" { String::from("f>=") } else { String::from("") };
                if fo.len()>0 { let la=lower(a,cx); let lb=lower(b,cx); let i=fresh(cx);
                    return H::BinH(fo, Box::new(la), Box::new(lb), i); } } }
            if o.as_str()=="&&" { let la=lower(a,cx); let lb=lower(b,cx); let z=fresh(cx); let f=H::Lit(0,z); let i=fresh(cx); return H::IfH(Box::new(la), Box::new(lb), Box::new(f), i); }
            if o.as_str()=="||" { let la=lower(a,cx); let lb=lower(b,cx); let o1=fresh(cx); let tt=H::Lit(1,o1); let i=fresh(cx); return H::IfH(Box::new(la), Box::new(tt), Box::new(lb), i); }
            let la=lower(a,cx); let lb=lower(b,cx); let i=fresh(cx);
            let bh=H::BinH(String::from(o.as_str()), Box::new(la), Box::new(lb), i);
            // P-WIDTH: `250u8 + 10` is 4, not 260. An `as` cast already truncates in the folder and in
            // the emitted IR; arithmetic truncated NOWHERE, so a width-typed operation wrapped only by
            // accident of the 64-bit word. The same trunc/ext pair is applied here, and only when a
            // width is actually KNOWN and narrower than the word -- an untyped or isize value is left
            // alone.
            { let wa=ety_of(cx,a); let wb=ety_of(cx,b);
              // a SHIFT's width is the LEFT operand's alone -- the amount is a count, not a value of the
              // same type -- and K4 (probed from RC) says the amount is taken modulo that width, so
              // `1u8 << 9` is 2 and not 0 and not 512
              let shift = o.as_str()=="<<" || o.as_str()==">>";
              let w=if crate::oast::is_int_width(&wa) && crate::oast::width_bits(&wa)<64 { wa }
                    else if !shift && crate::oast::is_int_width(&wb) && crate::oast::width_bits(&wb)<64 { wb }
                    else { String::from("") };
              if w.len()>0 && shift {
                  let bits=crate::oast::width_bits(&w);
                  let la2=lower(a,cx); let lb2=lower(b,cx);
                  let z=fresh(cx); let m=H::Lit((bits-1) as i64, z);
                  let i3=fresh(cx); let amt=H::BinH(String::from("&"), Box::new(lb2), Box::new(m), i3);
                  let i4=fresh(cx); let sh=H::BinH(String::from(o.as_str()), Box::new(la2), Box::new(amt), i4);
                  let i5=fresh(cx); return H::CastH(Box::new(sh), w, i5); }
              if w.len()>0 && arith_op(o) { let i2=fresh(cx); return H::CastH(Box::new(bh), w, i2); } }
            bh },
        E::If(cd,t,f,_l,_h)=>{ let lc=lower(cd,cx); let lt=lower(t,cx); let lf=lower(f,cx); let i=fresh(cx); H::IfH(Box::new(lc), Box::new(lt), Box::new(lf), i) },
        E::LetT(n,t,v,b,_l,_h)=>{ let lv=lower_coerce(v,cx,t); cx.stypes.push(String::from(t.as_str())); cx.names.push(String::from(n.as_str())); let s=(cx.names.len()-1) as u32; let lb=lower(b,cx); let _=cx.names.pop(); let _=cx.stypes.pop(); let i=fresh(cx); H::LetH(s, Box::new(lv), Box::new(lb), i) },
        // POP the binding when its body ends. Lowering used to let scopes grow monotonically, so a shadowed
        // name stayed visible AFTER its block: `let x = 1; if c { let x = 7; 0 }; x` read 7. The slot index
        // baked into LetH stays valid (it is a frame slot) and a later sibling `let` reuses it -- correct,
        // because the two scopes are disjoint.
        E::Let(n,v,b,_l,_h)=>{ let lv=lower(v,cx); let vt=ety_of(cx,v); cx.stypes.push(vt); cx.names.push(String::from(n.as_str())); let s=(cx.names.len()-1) as u32; let lb=lower(b,cx); let _=cx.names.pop(); let _=cx.stypes.pop(); let i=fresh(cx); H::LetH(s, Box::new(lv), Box::new(lb), i) },
    }
}

// fill an LCx from the program's function table / trait impls (the lowering context's own setup)
pub fn fill_dyn(lc:&mut LCx, impls:&Vec<ImplRec>){ let mut i=0; while i<impls.len(){ let mut e=String::from(impls[i].tr.as_str()); e.push('|'); e.push_str(impls[i].ty.as_str()); lc.dimpls.push(e); i=i+1; } }
pub fn fill_fsigs(lc:&mut LCx, funs:&Vec<Fun>){ let mut i=0; while i<funs.len(){ lc.fnames.push(String::from(funs[i].name.as_str())); lc.frets.push(String::from(funs[i].ret.as_str()));
    let mut j=String::from(""); let mut k=0; while k<funs[i].ptys.len(){ if k>0 { j.push(','); } j.push_str(&funs[i].ptys[k]); k=k+1; } lc.fptys.push(j);
    lc.fncaps.push(funs[i].ncap);
    { let mut g=String::from(""); let mut k=0; while k<funs[i].gparams.len(){ if k>0 { g.push(','); } g.push_str(&funs[i].gparams[k]); k=k+1; } lc.fgps.push(g); }
    { let mut b=String::from(""); let mut k=0; while k<funs[i].gbounds.len(){ if k>0 { b.push(','); } b.push_str(&funs[i].gbounds[k]); k=k+1; } lc.fgbs.push(b); }
    i=i+1; } }
}
mod omir {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
pub enum Op { CastOp(u32,u32,String), Brk(u32), Cont(u32), RetEarly(u32), StrC(u32,String), ParamOp(u32,u32), CallOp(u32,String,Vec<u32>), Const(u32,i64), Bin(u32,String,u32,u32),
              IfOp(u32,u32,Vec<Op>,u32,Vec<Op>,u32), LoopOp(Vec<Op>,u32,Vec<Op>),
              StoreLoc(u32,u32), LoadLoc(u32,u32), MakeStruct(u32,u32,Vec<u32>), LoadField(u32,u32,u32),
              MakeEnum(u32,u32,Vec<u32>), MatchOp(u32,u32,Vec<MArm>), MakeArr(u32,u32,Vec<u32>), LoadIdx(u32,u32,u32), StoreIdx(u32,u32,u32), StoreField(u32,u32,u32), Ret(u32) }
pub struct MArm { pub tag: i32, pub slots: Vec<u32>, pub ops: Vec<Op>, pub res: u32 }
pub struct Body { pub ops: Vec<Op>, pub ntmp: u32, pub nloc: u32 }
pub fn veccopy_u32(v:&Vec<u32>)->Vec<u32>{ let mut o:Vec<u32>=Vec::new(); let mut i=0; while i<v.len(){ o.push(v[i]); i=i+1; } o }
fn t_new(b:&mut Body)->u32{ let t=b.ntmp; b.ntmp=t+1; t }
fn note_loc(b:&mut Body, s:u32){ if s+1 > b.nloc { b.nloc=s+1; } }
pub fn build(h:&H, b:&mut Body)->u32{
    match h {
        H::StoreIdxH(bs,ix,v,_i)=>{ let bt=build(bs,b); let it=build(ix,b); let vt=build(v,b); b.ops.push(Op::StoreIdx(bt,it,vt)); vt },
        H::StoreFieldH(bs,fx,v,_i)=>{ let bt=build(bs,b); let vt=build(v,b); b.ops.push(Op::StoreField(bt,*fx,vt)); vt },
        H::ArrLitH(xs,_i)=>{ let mut av:Vec<u32>=Vec::new(); let mut k=0; while k<xs.len(){ av.push(build(&xs[k],b)); k=k+1; } let t=t_new(b); b.ops.push(Op::MakeArr(t, xs.len() as u32, av)); t },
        H::IndexH(bs,ix,_i)=>{ let bt=build(bs,b); let it=build(ix,b); let t=t_new(b); b.ops.push(Op::LoadIdx(t,bt,it)); t },
        H::EnumLitH(tag,ps,_i)=>{ let mut av:Vec<u32>=Vec::new(); let mut k=0; while k<ps.len(){ av.push(build(&ps[k],b)); k=k+1; } let t=t_new(b); b.ops.push(Op::MakeEnum(t,*tag,av)); t },
        H::MatchH(sc,arms,_i)=>{
            let st=build(sc,b);
            let dest=t_new(b);
            let mut ma:Vec<MArm>=Vec::new();
            let mut k=0;
            while k<arms.len(){
                let a=&arms[k];
                let mut ab=Body{ops:Vec::new(), ntmp:b.ntmp, nloc:b.nloc};
                let r=build(&a.body,&mut ab); b.ntmp=ab.ntmp; if ab.nloc>b.nloc { b.nloc=ab.nloc; }
                let mut sl:Vec<u32>=Vec::new(); let mut q=0; while q<a.slots.len(){ sl.push(a.slots[q]); if a.slots[q]+1 > b.nloc { b.nloc=a.slots[q]+1; } q=q+1; }
                ma.push(MArm{tag:a.tag, slots:sl, ops:ab.ops, res:r});
                k=k+1;
            }
            b.ops.push(Op::MatchOp(dest, st, ma)); dest },
        H::StructLitH(nf,fs,_i)=>{ let mut av:Vec<u32>=Vec::new(); let mut k=0; while k<fs.len(){ av.push(build(&fs[k],b)); k=k+1; } let t=t_new(b); b.ops.push(Op::MakeStruct(t,*nf,av)); t },
        H::FieldH(base,ix,_i)=>{ let bt=build(base,b); let t=t_new(b); b.ops.push(Op::LoadField(t,bt,*ix)); t },
        H::Lit(v,_i)=>{ let t=t_new(b); b.ops.push(Op::Const(t,*v)); t },
        H::CastH(x,t,_i)=>{ let sx=build(x,b); let r=t_new(b); b.ops.push(Op::CastOp(r,sx,String::from(t.as_str()))); r },
        H::BreakH(_i)=>{ let bid=t_new(b); b.ops.push(Op::Brk(bid)); let t=t_new(b); b.ops.push(Op::Const(t,0)); t },
        H::ContH(_i)=>{ let cid=t_new(b); b.ops.push(Op::Cont(cid)); let t=t_new(b); b.ops.push(Op::Const(t,0)); t },
        H::RetH(x,_i)=>{ let t=build(x,b); b.ops.push(Op::RetEarly(t)); t },
        H::StrH(v,_i)=>{ let t=t_new(b); b.ops.push(Op::StrC(t,String::from(v.as_str()))); t },
        H::LitB(v,_i)=>{ let t=t_new(b); b.ops.push(Op::Const(t, if *v {1} else {0})); t },
        H::Local(s,_i)=>{ note_loc(b,*s); let t=t_new(b); b.ops.push(Op::LoadLoc(t,*s)); t },
        H::AssignH(s,v,_i)=>{ let vt=build(v,b); note_loc(b,*s); b.ops.push(Op::StoreLoc(*s,vt)); vt },
        H::SeqH(a,c,_i)=>{ let _=build(a,b); build(c,b) },
        H::BinH(o,x,y,_i)=>{ let a=build(x,b); let c=build(y,b); let t=t_new(b); b.ops.push(Op::Bin(t,String::from(o.as_str()),a,c)); t },
        H::CallH(n,args,_i)=>{ let mut av:Vec<u32>=Vec::new(); let mut k=0; while k<args.len(){ av.push(build(&args[k],b)); k=k+1; } let t=t_new(b); b.ops.push(Op::CallOp(t, String::from(n.as_str()), av)); t },
        H::IfH(cd,x,y,_i)=>{
            let c0=build(cd,b);
            let mut tb=Body{ops:Vec::new(), ntmp:b.ntmp, nloc:b.nloc}; let tr=build(x,&mut tb); b.ntmp=tb.ntmp; if tb.nloc>b.nloc { b.nloc=tb.nloc; }
            let mut eb=Body{ops:Vec::new(), ntmp:b.ntmp, nloc:b.nloc}; let er=build(y,&mut eb); b.ntmp=eb.ntmp; if eb.nloc>b.nloc { b.nloc=eb.nloc; }
            let t=t_new(b); b.ops.push(Op::IfOp(t, c0, tb.ops, tr, eb.ops, er)); t },
        H::WhileH(c,bd,_i)=>{
            let mut cb=Body{ops:Vec::new(), ntmp:b.ntmp, nloc:b.nloc}; let cr=build(c,&mut cb); b.ntmp=cb.ntmp; if cb.nloc>b.nloc { b.nloc=cb.nloc; }
            let mut bb=Body{ops:Vec::new(), ntmp:b.ntmp, nloc:b.nloc}; let _=build(bd,&mut bb); b.ntmp=bb.ntmp; if bb.nloc>b.nloc { b.nloc=bb.nloc; }
            b.ops.push(Op::LoopOp(cb.ops, cr, bb.ops));
            let t=t_new(b); b.ops.push(Op::Const(t,0)); t },
        H::LetH(s,v,bd,_i)=>{ let vt=build(v,b); note_loc(b,*s); b.ops.push(Op::StoreLoc(*s,vt)); build(bd,b) },
    }
}
}
mod omirtrans {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
pub fn fold(b:&Body)->i64{
    let mut tv=crate::oindex::tv_new(); let mut i=0; while i<b.ntmp { let _=crate::oindex::tv_push(&mut tv, 0); i=i+1; }
    let mut lv:Vec<i64>=Vec::new(); let mut j=0; while j<b.nloc { lv.push(0); j=j+1; }
    let mut heap:Vec<i64>=Vec::new();
    run_ops(&b.ops, &mut tv, &mut lv, &mut heap)
}
fn tget(tv:&crate::oindex::TempVec, t:u32)->i64{ crate::oindex::tv_get(tv, &crate::oindex::tid(t)) }
// two's-complement truncation, matching what codegen emits (trunc + zext/sext). Without this the folder
// reported a cast as identity and the driver printed a value the compiled program would not produce.
pub fn trunc_val(v:i64, bits:u32, signed:bool)->i64{
    if bits>=64 { return v; }
    let mut p:i64=1; let mut i=0; while i<bits { p=p*2; i=i+1; }
    let mut m=v - (v/p)*p;
    if m<0 { m=m+p; }
    if signed { let h=p/2; if m>=h { m=m-p; } }
    m }
fn tset(tv:&mut crate::oindex::TempVec, t:u32, v:i64){ crate::oindex::tv_set(tv, &crate::oindex::tid(t), v); }
fn run_ops(ops:&Vec<Op>, tv:&mut crate::oindex::TempVec, lv:&mut Vec<i64>, heap:&mut Vec<i64>)->i64{
    let mut k=0; let mut last:i64=0;
    while k<ops.len(){
        match &ops[k] {
            Op::StrC(t,_v)=>{ tset(tv,*t,0); }
            Op::CastOp(t,sx,ty)=>{ let v=tget(tv,*sx); tset(tv,*t,trunc_val(v, crate::oast::width_bits(ty), crate::oast::width_signed(ty))); }
            Op::Brk(_b)=>{}
            Op::Cont(_c)=>{}
            Op::RetEarly(t)=>{ return tget(tv,*t); }
            Op::ParamOp(t,_p)=>{ tset(tv,*t,0); last=0; },
            Op::CallOp(t,_n,_a)=>{ tset(tv,*t,0); last=0; },
            Op::Const(t,v)=>{ tset(tv,*t,*v); last=*v; },
            Op::Bin(t,o,x,y)=>{ let a=tget(tv,*x); let c=tget(tv,*y); let v= if o.as_str()=="+" {a+c} else if o.as_str()=="-" {a-c} else if o.as_str()=="*" {a*c} else if o.as_str()=="/" { if c!=0 {a/c} else {0} } else if o.as_str()=="%" { if c!=0 {a-(a/c)*c} else {0} } else if o.as_str()=="&" {a&c} else if o.as_str()=="|" {a|c} else if o.as_str()=="^" {a^c} else if o.as_str()=="<<" { if c>=0 && c<64 {a<<c} else {0} } else if o.as_str()==">>" { if c>=0 && c<64 {a>>c} else {0} } else if o.as_str()=="<" { if a<c {1} else {0} } else if o.as_str()=="<=" { if a<=c {1} else {0} } else if o.as_str()==">" { if a>c {1} else {0} } else if o.as_str()==">=" { if a>=c {1} else {0} } else if o.as_str()=="!=" { if a!=c {1} else {0} } else { if a==c {1} else {0} }; tset(tv,*t,v); last=v; },
            Op::LoadLoc(t,s)=>{ let v=lv[*s as usize]; tset(tv,*t,v); last=v; },
            Op::StoreLoc(s,t)=>{ let v=tget(tv,*t); lv[*s as usize]=v; last=v; },
            Op::IfOp(t,c0,tops,tr,eops,er)=>{ let cv=tget(tv,*c0); let v= if cv!=0 { let _=run_ops(tops,tv,lv,heap); tget(tv,*tr) } else { let _=run_ops(eops,tv,lv,heap); tget(tv,*er) }; tset(tv,*t,v); last=v; },
            Op::LoopOp(cops,cr,bops)=>{ let mut guard=0; loop { let _=run_ops(cops,tv,lv,heap); if tget(tv,*cr)==0 { break; } let _=run_ops(bops,tv,lv,heap); guard=guard+1; if guard>1000000 { break; } } last=0; },
            Op::MakeStruct(t,_nf,av)=>{ let base=heap.len() as i64; let mut k=0; while k<av.len(){ let v=tget(tv,av[k]); heap.push(v); k=k+1; } tset(tv,*t,base); last=base; },
            Op::LoadField(t,bt,ix)=>{ let base=tget(tv,*bt); let a=base + (*ix as i64); let v= if a>=0 && (a as usize)<heap.len() { heap[a as usize] } else { 0 }; tset(tv,*t,v); last=v; },
            Op::StoreIdx(bt,it,vt)=>{ let base=tget(tv,*bt); let ix=tget(tv,*it)+1; let v=tget(tv,*vt); let a=base+ix; if a>=0 && (a as usize)<heap.len() { heap[a as usize]=v; } last=v; },
            Op::StoreField(bt,fx,vt)=>{ let base=tget(tv,*bt); let v=tget(tv,*vt); let a=base+(*fx as i64); if a>=0 && (a as usize)<heap.len() { heap[a as usize]=v; } last=v; },
            Op::MakeArr(t,n,av)=>{ let base=heap.len() as i64; heap.push(*n as i64); let mut k=0; while k<av.len(){ let v=tget(tv,av[k]); heap.push(v); k=k+1; } tset(tv,*t,base); last=base; },
            Op::LoadIdx(t,bt,it)=>{ let base=tget(tv,*bt); let ix=tget(tv,*it)+1; let a=base+ix; let v= if a>=0 && (a as usize)<heap.len() { heap[a as usize] } else { 0 }; tset(tv,*t,v); last=v; },
            Op::MakeEnum(t,tag,av)=>{ let base=heap.len() as i64; heap.push(*tag as i64); let mut k=0; while k<av.len(){ let v=tget(tv,av[k]); heap.push(v); k=k+1; } tset(tv,*t,base); last=base; },
            Op::MatchOp(dest,st,arms)=>{
                let base=tget(tv,*st);
                let tg= if base>=0 && (base as usize)<heap.len() { heap[base as usize] } else { -1 };
                let mut k=0; let mut done=false;
                while k<arms.len(){
                    let a=&arms[k];
                    if !done && (a.tag<0 || (a.tag as i64)==tg) {
                        let mut q=0; while q<a.slots.len(){ let hp=base + 1 + (q as i64); let pv= if hp>=0 && (hp as usize)<heap.len() { heap[hp as usize] } else { 0 }; let sl=a.slots[q] as usize; while lv.len()<=sl { lv.push(0); } lv[sl]=pv; q=q+1; }
                        let _=run_ops(&a.ops, tv, lv, heap);
                        let v=tget(tv,a.res); tset(tv,*dest,v); last=v; done=true;
                    }
                    k=k+1;
                }
                if !done { tset(tv,*dest,0); last=0; } },
            Op::Ret(s)=>{ last=tget(tv,*s); },
        }
        k=k+1;
    }
    last
}
pub fn nops(b:&Body)->u32{ b.ops.len() as u32 }
// does this body contain a CALL? then the const-folder cannot produce the program's value (it is produced by
// RUNNING the emitted code), so the driver reports the TYPE only -- never a misleading "= 0".
pub fn fop_in(o:&str)->bool{ let b=o.as_bytes(); b.len()>1 && b[0]==102 }
pub fn has_call(b:&Body)->bool{ ops_have_call(&b.ops) }
fn ops_have_call(ops:&Vec<Op>)->bool{
    let mut k=0;
    while k<ops.len(){
        match &ops[k] {
            Op::RetEarly(_t)=>{},
            // a FLOAT op is not foldable by an i64 interpreter: treat it as a call so the value is
            // computed by the emitted code, never by the folder
            Op::Bin(_t,o,_x,_y) if fop_in(o) =>{ return true; },
            Op::Brk(_b)=>{ return true; }
            Op::Cont(_c)=>{ return true; }
            Op::CallOp(_t,_n,_a)=>{ return true; },
            Op::StrC(_t,_v)=>{ return true; },
            Op::IfOp(_t,_c,tops,_tr,eops,_er)=>{ if ops_have_call(tops) { return true; } if ops_have_call(eops) { return true; } },
            Op::LoopOp(cops,_cr,bops)=>{ if ops_have_call(cops) { return true; } if ops_have_call(bops) { return true; } },
            Op::MatchOp(_d,_s,arms)=>{ let mut q=0; while q<arms.len(){ if ops_have_call(&arms[q].ops) { return true; } q=q+1; } },
            _=>{},
        }
        k=k+1;
    }
    false
}
}
mod ocodegen {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
// the same i64::MIN hazard as crate::oast::i64s: negating MIN to build digits overflows and emitted a bare "-",
// which is invalid IR. Build from the negative side.
fn n64(n:i64)->String{ if n==0 {return String::from("0");} let neg=n<0;
    if neg { let mut v=n; let mut s=String::from("");
        while v<0 { let d=(0-(v%10)) as u32; let c=(48+d) as u8 as char; let mut t=String::from(""); t.push(c); t.push_str(&s); s=t; v=v/10; }
        let mut t=String::from("-"); t.push_str(&s); return t; }
    let mut u=if neg {0-n} else {n}; let mut s=String::from(""); while u>0 {let d=(u%10) as u32;let c=(48+d) as u8 as char;let mut t=String::from("");t.push(c);t.push_str(&s);s=t;u=u/10;} if neg {let mut t=String::from("-");t.push_str(&s);s=t;} s }
fn n32(n:u32)->String{ if n==0 {return String::from("0");} let mut v=n; let mut s=String::from(""); while v>0 {let d=(v%10) as u32;let c=(48+d) as u8 as char;let mut t=String::from("");t.push(c);t.push_str(&s);s=t;v=v/10;} s }
fn tmp(t:u32)->String{ let mut s=String::from("%t"); s.push_str(&n32(t)); s }
fn loc(s:u32)->String{ let mut r=String::from("%L"); r.push_str(&n32(s)); r }
fn is_fop(o:&str)->bool{ let b=o.as_bytes(); b.len()>1 && b[0]==102 }
fn fllop(o:&str)->String{
    if o=="f+" { String::from("fadd") } else if o=="f-" { String::from("fsub") }
    else if o=="f*" { String::from("fmul") } else if o=="f/" { String::from("fdiv") }
    else if o=="f%" { String::from("frem") }
    else if o=="f==" { String::from("oeq") } else if o=="f!=" { String::from("une") }
    else if o=="f<" { String::from("olt") } else if o=="f<=" { String::from("ole") }
    else if o=="f>" { String::from("ogt") } else { String::from("oge") } }
fn fop_is_cmp(o:&str)->bool{ o=="f==" || o=="f!=" || o=="f<" || o=="f<=" || o=="f>" || o=="f>=" }
fn llop(o:&str)->String{ if o=="&" {String::from("and")} else if o=="|" {String::from("or")} else if o=="^" {String::from("xor")} else if o=="<<" {String::from("shl")} else if o==">>" {String::from("ashr")} else if o=="+" {String::from("add")} else if o=="-" {String::from("sub")} else if o=="*" {String::from("mul")} else if o=="/" {String::from("sdiv")} else if o=="%" {String::from("srem")} else {String::from("icmp")} }
fn match_slots(ops:&Vec<Op>)->String{ let mut s=String::from(""); let mut k=0; while k<ops.len(){ match &ops[k] { Op::MatchOp(d,_st,arms)=>{ s.push_str("  "); s.push_str(&tmp(*d)); s.push_str(".slot = alloca i64\n"); let mut q=0; while q<arms.len(){ s.push_str(&match_slots(&arms[q].ops)); q=q+1; } }, Op::IfOp(_t,_c,t2,_r,e2,_r2)=>{ s.push_str(&match_slots(t2)); s.push_str(&match_slots(e2)); }, Op::LoopOp(c2,_cr,b2)=>{ s.push_str(&match_slots(c2)); s.push_str(&match_slots(b2)); }, _=>{} } k=k+1; } s }
fn prologue(b:&Body)->String{ let lt=crate::oabi::ll_ty(&crate::oabi::layout_int()); let mut s=String::from(""); s.push_str(&match_slots(&b.ops)); let mut i=0; while i<b.nloc { s.push_str("  "); s.push_str(&loc(i)); s.push_str(" = alloca "); s.push_str(&lt); s.push_str("\n"); s.push_str("  store "); s.push_str(&lt); s.push_str(" 0, "); s.push_str(&lt); s.push_str("* "); s.push_str(&loc(i)); s.push_str("\n"); i=i+1; } s }
pub fn emit(b:&Body)->String{
    let lt=crate::oabi::ll_ty(&crate::oabi::layout_int());
    // C-ARGV: main takes argc/argv and stashes them in globals, so the argv primitives can read them.
    // Our own driver reads its options from std::env::args, so a compiler meant to compile that driver
    // needs them, and a signature main() cannot supply them. Every emitted program gets the wider
    // signature; a program that never touches argv is unaffected (cc links either form).
    let mut s=String::from("define "); s.push_str(&lt); s.push_str(" @main(i32 %argc, i8** %argv) {\nentry:\n");
    s.push_str("  %acw = sext i32 %argc to i64\n  store i64 %acw, i64* @__argc\n");
    s.push_str("  %avw = ptrtoint i8** %argv to i64\n  store i64 %avw, i64* @__argv\n");
    s.push_str(&prologue(b)); s.push_str(&body_ops_l(&b.ops)); s.push_str("}\n"); s
}
pub fn emit_named(b:&Body, name:&str, nparams:u32)->String{
    let lt=crate::oabi::ll_ty(&crate::oabi::layout_int());
    let mut s=String::from("define "); s.push_str(&lt); s.push_str(" @"); s.push_str(name); s.push_str("(");
    let mut p=0; while p<nparams { if p>0 { s.push_str(", "); } s.push_str("i64 %p"); s.push_str(&n32(p)); p=p+1; }
    s.push_str(") {\nentry:\n"); s.push_str(&prologue(b)); s.push_str(&body_ops_l(&b.ops)); s.push_str("}\n"); s
}
// Each distinct string literal becomes a private constant array; StrC yields its ADDRESS as an i64, so a
// string is just a pointer-sized value and needs no new MIR type. The globals and the puts/strlen declarations
// are emitted once in the module preamble (collect_strs walks the whole op tree, nested blocks included).
// C-STRGID: a string constant's global NAME is derived from its content, so the derivation must be
// injective enough to never name two literals the same. It was not, twice over. The step was *33, which
// for a two-character literal reduces to c1*33+c2 and collides whenever c1 differs by one and c2 by 33 --
// ".." and "-O" are exactly that pair, and they are what the composed unit tripped over. Worse, the
// name was the DECIMAL of the hash printed by n64u, whose loop runs while v>0: with one scalar width a
// hash with the high bit set is a negative i64, so n64u answered the empty string and 134 of the unit's
// 761 literals were all named `.s`. Now the step is *257 (injective over byte pairs) and the name is all
// 64 bits in fixed-width hex, which cannot be empty. Simulated over the composed unit's 761 literals
// before building: *33 with hex names still collides twice, *131 and *257 collide zero times.
pub fn str_gid(lit:&str)->String{ let mut h:u64=5381; for c in lit.chars(){ h=h.wrapping_mul(257).wrapping_add(c as u64); }
    let d="0123456789abcdef"; let db=d.as_bytes();
    let mut r=String::from(".s"); let mut i=60;
    while i>=0 { r.push(db[((h>>i)&15) as usize] as char); if i==0 { i=-1; } else { i=i-4; } }
    r }
fn esc_ll(lit:&str)->String{ let mut r=String::from(""); for c in lit.chars(){ let x=c as u32; if x>=32 && x<127 && c!='"' && c!='\\' { r.push(c); } else { r.push_str("\\"); r.push_str(&hex2(x)); } } r }
fn hex2(x:u32)->String{ let d="0123456789ABCDEF"; let db:Vec<char>={let mut v:Vec<char>=Vec::new(); for c in d.chars(){v.push(c);} v}; let mut r=String::from(""); r.push(db[((x>>4)&15) as usize]); r.push(db[(x&15) as usize]); r }
fn n64u(x:u64)->String{ if x==0 { return String::from("0"); } let mut v=x; let mut t:Vec<char>=Vec::new(); while v>0 { t.push((48+(v%10) as u32) as u8 as char); v=v/10; } let mut r=String::from(""); let mut i=t.len(); while i>0 { r.push(t[i-1]); i=i-1; } r }
pub fn collect_globals(ops:&Vec<Op>, out:&mut Vec<String>){
    let mut k=0; while k<ops.len(){
        match &ops[k] {
            Op::CallOp(_t,n,_av) if crate::olower::is_gname(n) =>{ let g=crate::olower::gname_of(n);
                let mut seen=false; let mut j=0; while j<out.len(){ if out[j].as_str()==g.as_str() { seen=true; } j=j+1; }
                if !seen { out.push(g); } },
            Op::IfOp(_t,_c,t2,_r,e2,_r2)=>{ collect_globals(t2,out); collect_globals(e2,out); },
            Op::LoopOp(c2,_cr,b2)=>{ collect_globals(c2,out); collect_globals(b2,out); },
            Op::MatchOp(_d,_st,arms)=>{ let mut q=0; while q<arms.len(){ collect_globals(&arms[q].ops,out); q=q+1; } },
            _=>{} }
        k=k+1; } }
pub fn global_defs(gs:&Vec<String>)->String{
    let mut s=String::from(""); let mut i=0;
    while i<gs.len(){ s.push_str("@g."); s.push_str(gs[i].as_str()); s.push_str(" = global i64 0\n"); i=i+1; }
    s }
pub fn collect_strs(ops:&Vec<Op>, out:&mut Vec<String>){
    let mut k=0; while k<ops.len(){
        match &ops[k] {
            Op::StrC(_t,v)=>{ let mut seen=false; let mut j=0; while j<out.len(){ if out[j].as_str()==v.as_str() { seen=true; } j=j+1; } if !seen { out.push(String::from(v.as_str())); } },
            Op::IfOp(_r,_c,th,_tv,el,_ev)=>{ collect_strs(th,out); collect_strs(el,out); },
            Op::LoopOp(cc,_c,bd)=>{ collect_strs(cc,out); collect_strs(bd,out); },
            Op::MatchOp(_r,_s,arms)=>{ let mut a=0; while a<arms.len(){ collect_strs(&arms[a].ops,out); a=a+1; } },
            _=>{} }
        k=k+1; } }
pub fn str_globals(lits:&Vec<String>)->String{
    let mut s=String::from("");
    let mut i=0; while i<lits.len(){
        let n=lits[i].chars().count()+1;
        s.push_str("@"); s.push_str(&str_gid(lits[i].as_str())); s.push_str(" = private unnamed_addr constant [");
        s.push_str(&n32(n as u32)); s.push_str(" x i8] c\""); s.push_str(&esc_ll(lits[i].as_str())); s.push_str("\\00\"\n");
        i=i+1; }
    // the libc surface the string primitives need, plus printf's integer format; declared unconditionally so
    // print_int works in a program with no string literal of its own
    s.push_str("@.fmtd = private unnamed_addr constant [5 x i8] c\"%ld\\0A\\00\"\n");
    s.push_str("@.fmts = private unnamed_addr constant [3 x i8] c\"%s\\00\"\n");
    s.push_str("@.fmtdr = private unnamed_addr constant [4 x i8] c\"%ld\\00\"\n");
    s.push_str("@.fmtnl = private unnamed_addr constant [2 x i8] c\"\\0A\\00\"\n");
    s.push_str("@.fmtc = private unnamed_addr constant [3 x i8] c\"%c\\00\"\n");
    s.push_str("@.fmtfp = private unnamed_addr constant [5 x i8] c\"%.*f\\00\"\n@.fmtfe = private unnamed_addr constant [5 x i8] c\"%.*e\\00\"\n");
    s.push_str("declare void @exit(i32)\n");
    s.push_str("declare i32 @puts(i8*)\ndeclare i64 @strlen(i8*)\ndeclare i32 @strcmp(i8*, i8*)\ndeclare i32 @printf(i8*, ...)\n");
    s.push_str(&vec_runtime());
    s.push_str(&str_runtime());
    s
}
// A GROWABLE VECTOR as real heap memory: header [len, cap, data] from malloc, data doubled with realloc.
// Emitted as LLVM functions in the preamble rather than inlined at each call site, so the branchy grow path
// lives in its own function and `vec_push` stays a plain call in the caller's straight-line block.
// String primitives a tokenizer needs. A str is an i64 address of NUL-terminated bytes (rung 8o), so a
// SUBSTRING or a CONCATENATION must allocate: both malloc a fresh buffer, memcpy into it, and NUL-terminate,
// which makes a computed string indistinguishable from a literal to every consumer (len/print/==).
pub fn str_runtime()->String{
    let mut s=String::from("");
    s.push_str("declare i8* @memcpy(i8*, i8*, i64)\n");
    s.push_str("define i64 @__str_at(i64 %s, i64 %i) {\nentry:\n  %p = inttoptr i64 %s to i8*\n  %q = getelementptr i8, i8* %p, i64 %i\n  %c = load i8, i8* %q\n  %r = zext i8 %c to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__str_cat(i64 %a, i64 %b) {\nentry:\n  %ap = inttoptr i64 %a to i8*\n  %bp = inttoptr i64 %b to i8*\n  %la = call i64 @strlen(i8* %ap)\n  %lb = call i64 @strlen(i8* %bp)\n  %t1 = add i64 %la, %lb\n  %t2 = add i64 %t1, 1\n  %d = call i8* @malloc(i64 %t2)\n  %c1 = call i8* @memcpy(i8* %d, i8* %ap, i64 %la)\n  %dh = getelementptr i8, i8* %d, i64 %la\n  %lb1 = add i64 %lb, 1\n  %c2 = call i8* @memcpy(i8* %dh, i8* %bp, i64 %lb1)\n  %r = ptrtoint i8* %d to i64\n  ret i64 %r\n}\n");
    // C-READFILE: our own driver reads its input with fs::read_to_string, so a compiler that is to
    // compile that driver needs the primitive. It is a runtime helper over libc (fopen, fseek, ftell,
    // fread, fclose) answering a NUL-terminated buffer, i.e. our str, and the empty string when the file
    // cannot be opened -- which is exactly what the driver's Err arm already does.
    s.push_str("declare i8* @fopen(i8*, i8*)\ndeclare i64 @fseek(i8*, i64, i32)\ndeclare i64 @ftell(i8*)\ndeclare i64 @fread(i8*, i64, i64, i8*)\ndeclare i32 @fclose(i8*)\n");
    s.push_str("@.rdmode = private unnamed_addr constant [2 x i8] c\"r\\00\"\n");
    s.push_str("@__argc = global i64 0\n@__argv = global i64 0\n");
    s.push_str("define i64 @__argc_get() {\nentry:\n  %v = load i64, i64* @__argc\n  ret i64 %v\n}\n");
    s.push_str("define i64 @__argv_at(i64 %i) {\nentry:\n  %b = load i64, i64* @__argv\n  %p = inttoptr i64 %b to i8**\n  %q = getelementptr i8*, i8** %p, i64 %i\n  %c = load i8*, i8** %q\n  %r = ptrtoint i8* %c to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__fileok(i64 %pth) {\nentry:\n  %p = inttoptr i64 %pth to i8*\n  %m = getelementptr [2 x i8], [2 x i8]* @.rdmode, i64 0, i64 0\n  %f = call i8* @fopen(i8* %p, i8* %m)\n  %isn = icmp eq i8* %f, null\n  br i1 %isn, label %bad, label %ok\nbad:\n  ret i64 0\nok:\n  %cl = call i32 @fclose(i8* %f)\n  ret i64 1\n}\n");
    s.push_str("define i64 @__readfile(i64 %pth) {\nentry:\n  %p = inttoptr i64 %pth to i8*\n  %m = getelementptr [2 x i8], [2 x i8]* @.rdmode, i64 0, i64 0\n  %f = call i8* @fopen(i8* %p, i8* %m)\n  %isn = icmp eq i8* %f, null\n  br i1 %isn, label %bad, label %ok\nbad:\n  %z = call i8* @malloc(i64 1)\n  store i8 0, i8* %z\n  %zr = ptrtoint i8* %z to i64\n  ret i64 %zr\nok:\n  %s0 = call i64 @fseek(i8* %f, i64 0, i32 2)\n  %n = call i64 @ftell(i8* %f)\n  %s1 = call i64 @fseek(i8* %f, i64 0, i32 0)\n  %n1 = add i64 %n, 1\n  %d = call i8* @malloc(i64 %n1)\n  %got = call i64 @fread(i8* %d, i64 1, i64 %n, i8* %f)\n  %e = getelementptr i8, i8* %d, i64 %got\n  store i8 0, i8* %e\n  %cl = call i32 @fclose(i8* %f)\n  %r = ptrtoint i8* %d to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__chr1(i64 %c) {\nentry:\n  %d = call i8* @malloc(i64 2)\n  %b = trunc i64 %c to i8\n  store i8 %b, i8* %d\n  %e = getelementptr i8, i8* %d, i64 1\n  store i8 0, i8* %e\n  %r = ptrtoint i8* %d to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__str_sub(i64 %s, i64 %o, i64 %n) {\nentry:\n  %p = inttoptr i64 %s to i8*\n  %q = getelementptr i8, i8* %p, i64 %o\n  %n1 = add i64 %n, 1\n  %d = call i8* @malloc(i64 %n1)\n  %c1 = call i8* @memcpy(i8* %d, i8* %q, i64 %n)\n  %e = getelementptr i8, i8* %d, i64 %n\n  store i8 0, i8* %e\n  %r = ptrtoint i8* %d to i64\n  ret i64 %r\n}\n");
    s
}
pub fn vec_runtime()->String{
    let mut s=String::from("");
    s.push_str("declare i8* @malloc(i64)\ndeclare i8* @realloc(i8*, i64)\n");
    s.push_str("declare double @strtod(i8*, i8**)\ndeclare i32 @snprintf(i8*, i64, i8*, ...)\n");
    // a zero divisor exits 101, the way rustc's panic does
    s.push_str("define i64 @__divchk(i64 %d) {\nentry:\n  %z = icmp eq i64 %d, 0\n  br i1 %z, label %bad, label %ok\nbad:\n  call void @exit(i32 101)\n  unreachable\nok:\n  ret i64 %d\n}\n");
    // N2: an out-of-range index exits 101 too -- our reads used to run off the end and print garbage
    s.push_str("define i64 @__ixchk(i64 %i, i64 %n) {\nentry:\n  %lo = icmp slt i64 %i, 0\n  %hi = icmp sge i64 %i, %n\n  %bad = or i1 %lo, %hi\n  br i1 %bad, label %oob, label %ok\noob:\n  call void @exit(i32 101)\n  unreachable\nok:\n  ret i64 %i\n}\n");
    s.push_str("define i64 @__vec_new() {\nentry:\n  %p = call i8* @malloc(i64 24)\n  %h = bitcast i8* %p to i64*\n  store i64 0, i64* %h\n  %cp = getelementptr i64, i64* %h, i64 1\n  store i64 4, i64* %cp\n  %d = call i8* @malloc(i64 32)\n  %di = ptrtoint i8* %d to i64\n  %dp = getelementptr i64, i64* %h, i64 2\n  store i64 %di, i64* %dp\n  %r = ptrtoint i64* %h to i64\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__vec_push(i64 %v, i64 %x) {\nentry:\n  %h = inttoptr i64 %v to i64*\n  %len = load i64, i64* %h\n  %cp = getelementptr i64, i64* %h, i64 1\n  %cap = load i64, i64* %cp\n  %full = icmp sge i64 %len, %cap\n  br i1 %full, label %grow, label %put\ngrow:\n  %nc = mul i64 %cap, 2\n  %dpg = getelementptr i64, i64* %h, i64 2\n  %old = load i64, i64* %dpg\n  %oldp = inttoptr i64 %old to i8*\n  %bytes = mul i64 %nc, 8\n  %new = call i8* @realloc(i8* %oldp, i64 %bytes)\n  %newi = ptrtoint i8* %new to i64\n  store i64 %newi, i64* %dpg\n  store i64 %nc, i64* %cp\n  br label %put\nput:\n  %dp2 = getelementptr i64, i64* %h, i64 2\n  %d = load i64, i64* %dp2\n  %dpp = inttoptr i64 %d to i64*\n  %slot = getelementptr i64, i64* %dpp, i64 %len\n  store i64 %x, i64* %slot\n  %nl = add i64 %len, 1\n  store i64 %nl, i64* %h\n  ret i64 %nl\n}\n");
    s.push_str("define i64 @__vec_get(i64 %v, i64 %i) {\nentry:\n  %h = inttoptr i64 %v to i64*\n  %ln = load i64, i64* %h\n  %ck = call i64 @__ixchk(i64 %i, i64 %ln)\n  %dp = getelementptr i64, i64* %h, i64 2\n  %d = load i64, i64* %dp\n  %dpp = inttoptr i64 %d to i64*\n  %slot = getelementptr i64, i64* %dpp, i64 %ck\n  %r = load i64, i64* %slot\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__vec_set(i64 %v, i64 %i, i64 %x) {\nentry:\n  %h = inttoptr i64 %v to i64*\n  %dp = getelementptr i64, i64* %h, i64 2\n  %d = load i64, i64* %dp\n  %dpp = inttoptr i64 %d to i64*\n  %slot = getelementptr i64, i64* %dpp, i64 %i\n  store i64 %x, i64* %slot\n  ret i64 %x\n}\n");
    s.push_str("define i64 @__vec_popraw(i64 %v) {\nentry:\n  %h = inttoptr i64 %v to i64*\n  %n = load i64, i64* %h\n  %n1 = sub i64 %n, 1\n  store i64 %n1, i64* %h\n  %dp = getelementptr i64, i64* %h, i64 2\n  %d = load i64, i64* %dp\n  %dpp = inttoptr i64 %d to i64*\n  %slot = getelementptr i64, i64* %dpp, i64 %n1\n  %r = load i64, i64* %slot\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__arr_get(i64 %a, i64 %i) {\nentry:\n  %p = inttoptr i64 %a to i64*\n  %n = load i64, i64* %p\n  %ck = call i64 @__ixchk(i64 %i, i64 %n)\n  %ix = add i64 1, %ck\n  %ep = getelementptr i64, i64* %p, i64 %ix\n  %r = load i64, i64* %ep\n  ret i64 %r\n}\n");
    s.push_str("define i64 @__arr_set(i64 %a, i64 %i, i64 %v) {\nentry:\n  %p = inttoptr i64 %a to i64*\n  %n = load i64, i64* %p\n  %ck = call i64 @__ixchk(i64 %i, i64 %n)\n  %ix = add i64 1, %ck\n  %ep = getelementptr i64, i64* %p, i64 %ix\n  store i64 %v, i64* %ep\n  ret i64 %v\n}\n");
    s.push_str("define i64 @__arr_len(i64 %a) {\nentry:\n  %p = inttoptr i64 %a to i64*\n  %n = load i64, i64* %p\n  ret i64 %n\n}\n");
    // two enum values are equal when their tag and every payload WORD agree (the static side refuses a
    // payload whose type is not word-comparable, so this never compares a pointer as a value)
    s.push_str("define i64 @__enumeq(i64 %a, i64 %b, i64 %n) {\nentry:\n  %ap = inttoptr i64 %a to i64*\n  %bp = inttoptr i64 %b to i64*\n  br label %loop\nloop:\n  %i = phi i64 [ 0, %entry ], [ %i2, %body ]\n  %c = icmp slt i64 %i, %n\n  br i1 %c, label %body, label %same\nbody:\n  %aep = getelementptr i64, i64* %ap, i64 %i\n  %bep = getelementptr i64, i64* %bp, i64 %i\n  %av = load i64, i64* %aep\n  %bv = load i64, i64* %bep\n  %eq = icmp eq i64 %av, %bv\n  %i2 = add i64 %i, 1\n  br i1 %eq, label %loop, label %diff\ndiff:\n  ret i64 0\nsame:\n  ret i64 1\n}\n");
    // SLICING copies: `v[a..b]` / `arr[a..b]` produce a fresh VECTOR, so one representation answers
    // every later read (len, index, equality) regardless of what was sliced
    s.push_str("define i64 @__vec_slice(i64 %v, i64 %a, i64 %b) {\nentry:\n  %n = call i64 @__vec_new()\n  br label %loop\nloop:\n  %i = phi i64 [ %a, %entry ], [ %i2, %body ]\n  %c = icmp slt i64 %i, %b\n  br i1 %c, label %body, label %done\nbody:\n  %x = call i64 @__vec_get(i64 %v, i64 %i)\n  %p = call i64 @__vec_push(i64 %n, i64 %x)\n  %i2 = add i64 %i, 1\n  br label %loop\ndone:\n  ret i64 %n\n}\n");
    s.push_str("define i64 @__arr_slice(i64 %v, i64 %a, i64 %b) {\nentry:\n  %n = call i64 @__vec_new()\n  %bp = inttoptr i64 %v to i64*\n  br label %loop\nloop:\n  %i = phi i64 [ %a, %entry ], [ %i2, %body ]\n  %c = icmp slt i64 %i, %b\n  br i1 %c, label %body, label %done\nbody:\n  %ix = add i64 1, %i\n  %ep = getelementptr i64, i64* %bp, i64 %ix\n  %x = load i64, i64* %ep\n  %p = call i64 @__vec_push(i64 %n, i64 %x)\n  %i2 = add i64 %i, 1\n  br label %loop\ndone:\n  ret i64 %n\n}\n");
    s.push_str("define i64 @__arreq(i64 %a, i64 %b) {\nentry:\n  %pa = inttoptr i64 %a to i64*\n  %pb = inttoptr i64 %b to i64*\n  %na = load i64, i64* %pa\n  %nb = load i64, i64* %pb\n  %sl = icmp eq i64 %na, %nb\n  br i1 %sl, label %go, label %ne\nne:\n  ret i64 0\ngo:\n  br label %lc\nlc:\n  %i = phi i64 [ 1, %go ], [ %i2, %lb2 ]\n  %more = icmp sle i64 %i, %na\n  br i1 %more, label %lb2, label %eq\nlb2:\n  %ga = getelementptr i64, i64* %pa, i64 %i\n  %gb = getelementptr i64, i64* %pb, i64 %i\n  %va = load i64, i64* %ga\n  %vb = load i64, i64* %gb\n  %same = icmp eq i64 %va, %vb\n  %i2 = add i64 %i, 1\n  br i1 %same, label %lc, label %ne\neq:\n  ret i64 1\n}\n");
    s.push_str("define i64 @__veceq(i64 %a, i64 %b) {\nentry:\n  %ha = inttoptr i64 %a to i64*\n  %hb = inttoptr i64 %b to i64*\n  %la = load i64, i64* %ha\n  %lb = load i64, i64* %hb\n  %sl = icmp eq i64 %la, %lb\n  br i1 %sl, label %go, label %ne\nne:\n  ret i64 0\ngo:\n  %dpa = getelementptr i64, i64* %ha, i64 2\n  %dpb = getelementptr i64, i64* %hb, i64 2\n  %da = load i64, i64* %dpa\n  %db = load i64, i64* %dpb\n  %pa = inttoptr i64 %da to i64*\n  %pb = inttoptr i64 %db to i64*\n  br label %lc\nlc:\n  %i = phi i64 [ 0, %go ], [ %i2, %lb2 ]\n  %more = icmp slt i64 %i, %la\n  br i1 %more, label %lb2, label %eq\nlb2:\n  %ga = getelementptr i64, i64* %pa, i64 %i\n  %gb = getelementptr i64, i64* %pb, i64 %i\n  %va = load i64, i64* %ga\n  %vb = load i64, i64* %gb\n  %same = icmp eq i64 %va, %vb\n  %i2 = add i64 %i, 1\n  br i1 %same, label %lc, label %ne\neq:\n  ret i64 1\n}\n");
    s.push_str("define i64 @__vec_len(i64 %v) {\nentry:\n  %h = inttoptr i64 %v to i64*\n  %r = load i64, i64* %h\n  ret i64 %r\n}\n");
    s
}
pub fn ops_return(ops:&Vec<Op>)->bool{ let mut k=0; while k<ops.len(){ match &ops[k] { Op::RetEarly(_t)=>{ return true; }, _=>{} } k=k+1; } false }
fn body_ops_l(ops:&Vec<Op>)->String{ body_ops_in(ops, 0-1) }
// `lid` is the ENCLOSING loop's label id (-1 = none): what `break`/`continue` need to branch to
fn body_ops_in(ops:&Vec<Op>, lid0:i64)->String{ let lid=lid0;
    let lt=crate::oabi::ll_ty(&crate::oabi::layout_int());
    let mut s=String::from("");
    let mut k=0;
    while k<ops.len(){
        match &ops[k] {
            Op::ParamOp(t,p)=>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = add i64 0, %p"); s.push_str(&n32(*p)); s.push_str("\n"); },
            Op::CastOp(t,sx,ty)=>{
                // a width cast really truncates: trunc to the width, then zext (unsigned) or sext (signed).
                // 64-bit targets are identity.
                let bits=crate::oast::width_bits(ty); let sg=crate::oast::width_signed(ty);
                if bits>=64 { s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = add i64 0, "); s.push_str(&tmp(*sx)); s.push_str("\n"); }
                else { s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("w = trunc i64 "); s.push_str(&tmp(*sx)); s.push_str(" to i"); s.push_str(&n32(bits)); s.push_str("\n  "); s.push_str(&tmp(*t)); s.push_str(" = "); if sg { s.push_str("sext i"); } else { s.push_str("zext i"); } s.push_str(&n32(bits)); s.push_str(" "); s.push_str(&tmp(*t)); s.push_str("w to i64\n"); } },
            Op::Brk(bid)=>{ if lid>=0 { s.push_str("  br label %lend"); s.push_str(&n32(lid as u32)); s.push_str("\nbk"); s.push_str(&n32(*bid)); s.push_str("_"); s.push_str(&n32(k as u32)); s.push_str(":\n"); } },
            Op::Cont(cid)=>{ if lid>=0 { s.push_str("  br label %lcond"); s.push_str(&n32(lid as u32)); s.push_str("\nct"); s.push_str(&n32(*cid)); s.push_str("_"); s.push_str(&n32(k as u32)); s.push_str(":\n"); } },
            Op::RetEarly(t)=>{ s.push_str("  ret i64 "); s.push_str(&tmp(*t)); s.push_str("\n"); s.push_str("ar"); s.push_str(&n32(*t)); s.push_str(":\n"); },
            Op::StrC(t,v)=>{ let n=v.chars().count()+1; s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = ptrtoint ["); s.push_str(&n32(n as u32)); s.push_str(" x i8]* @"); s.push_str(&str_gid(v.as_str())); s.push_str(" to i64\n"); },
            // print/len are the two string PRIMITIVES, lowered to libc on the i8* recovered from the i64 value
            Op::CallOp(t,n,av) if n.as_str()=="print" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("p = inttoptr i64 "); s.push_str(&tmp(av[0])); s.push_str(" to i8*\n  "); s.push_str(&tmp(*t)); s.push_str("c = call i32 @puts(i8* "); s.push_str(&tmp(*t)); s.push_str("p)\n  "); s.push_str(&tmp(*t)); s.push_str(" = sext i32 "); s.push_str(&tmp(*t)); s.push_str("c to i64\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="str_at" || n.as_str()=="str_cat" || n.as_str()=="str_sub" || n.as_str()=="chr1" =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = call i64 @__"); s.push_str(n); s.push_str("("); let mut j=0; while j<av.len(){ if j>0 {s.push_str(", ");} s.push_str("i64 "); s.push_str(&tmp(av[j])); j=j+1; } s.push_str(")\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="vec_new" || n.as_str()=="vec_push" || n.as_str()=="vec_get" || n.as_str()=="vec_len" || n.as_str()=="vec_set" =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = call i64 @__"); s.push_str(n); s.push_str("("); let mut j=0; while j<av.len(){ if j>0 {s.push_str(", ");} s.push_str("i64 "); s.push_str(&tmp(av[j])); j=j+1; } s.push_str(")\n"); },
            Op::CallOp(t,n,av) if (n.as_str()=="arreq" && av.len()==2) || (n.as_str()=="arr_len" && av.len()==1) || ((n.as_str()=="vec_slice" || n.as_str()=="arr_slice") && av.len()==3) || (n.as_str()=="enumeq" && av.len()==3) || (n.as_str()=="vec_popraw" && av.len()==1) =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = call i64 @__"); s.push_str(n); s.push_str("("); let mut j=0; while j<av.len(){ if j>0 {s.push_str(", ");} s.push_str("i64 "); s.push_str(&tmp(av[j])); j=j+1; } s.push_str(")\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="veceq" && av.len()==2 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = call i64 @__veceq(i64 "); s.push_str(&tmp(av[0])); s.push_str(", i64 "); s.push_str(&tmp(av[1])); s.push_str(")\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="streq" && av.len()==2 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("a = inttoptr i64 "); s.push_str(&tmp(av[0])); s.push_str(" to i8*\n  "); s.push_str(&tmp(*t)); s.push_str("b = inttoptr i64 "); s.push_str(&tmp(av[1])); s.push_str(" to i8*\n  "); s.push_str(&tmp(*t)); s.push_str("c = call i32 @strcmp(i8* "); s.push_str(&tmp(*t)); s.push_str("a, i8* "); s.push_str(&tmp(*t)); s.push_str("b)\n  "); s.push_str(&tmp(*t)); s.push_str("z = icmp eq i32 "); s.push_str(&tmp(*t)); s.push_str("c, 0\n  "); s.push_str(&tmp(*t)); s.push_str(" = zext i1 "); s.push_str(&tmp(*t)); s.push_str("z to i64\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="print_raw" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("p = inttoptr i64 "); s.push_str(&tmp(av[0])); s.push_str(" to i8*\n  "); s.push_str(&tmp(*t)); s.push_str("c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmts, i64 0, i64 0), i8* "); s.push_str(&tmp(*t)); s.push_str("p)\n  "); s.push_str(&tmp(*t)); s.push_str(" = sext i32 "); s.push_str(&tmp(*t)); s.push_str("c to i64\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="print_int_raw" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.fmtdr, i64 0, i64 0), i64 "); s.push_str(&tmp(av[0])); s.push_str(")\n  "); s.push_str(&tmp(*t)); s.push_str(" = sext i32 "); s.push_str(&tmp(*t)); s.push_str("c to i64\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="proc_exit" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("w = trunc i64 "); s.push_str(&tmp(av[0])); s.push_str(" to i32\n  call void @exit(i32 "); s.push_str(&tmp(*t)); s.push_str("w)\n  "); s.push_str(&tmp(*t)); s.push_str(" = add i64 0, 0\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="panic_exit" && av.len()==0 =>{ s.push_str("  call void @exit(i32 101)\n  "); s.push_str(&tmp(*t)); s.push_str(" = add i64 0, 0\n"); },
            Op::CallOp(t,n,av) if crate::olower::is_gname(n) =>{
                let g=crate::olower::gname_of(n);
                if av.len()==0 { s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = load i64, i64* @g."); s.push_str(&g); s.push_str("\n"); }
                else { s.push_str("  store i64 "); s.push_str(&tmp(av[0])); s.push_str(", i64* @g."); s.push_str(&g); s.push_str("\n  "); s.push_str(&tmp(*t)); s.push_str(" = add i64 0, "); s.push_str(&tmp(av[0])); s.push_str("\n"); } },
            Op::CallOp(t,n,av) if n.as_str()=="f64_parse" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("p = inttoptr i64 "); s.push_str(&tmp(av[0])); s.push_str(" to i8*\n  "); s.push_str(&tmp(*t)); s.push_str("d = call double @strtod(i8* "); s.push_str(&tmp(*t)); s.push_str("p, i8** null)\n  "); s.push_str(&tmp(*t)); s.push_str(" = bitcast double "); s.push_str(&tmp(*t)); s.push_str("d to i64\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="f64_snprintf" && av.len()==2 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("b = call i8* @malloc(i64 1400)\n  "); s.push_str(&tmp(*t)); s.push_str("d = bitcast i64 "); s.push_str(&tmp(av[1])); s.push_str(" to double\n  "); s.push_str(&tmp(*t)); s.push_str("w = trunc i64 "); s.push_str(&tmp(av[0])); s.push_str(" to i32\n  "); s.push_str(&tmp(*t)); s.push_str("c = call i32 (i8*, i64, i8*, ...) @snprintf(i8* "); s.push_str(&tmp(*t)); s.push_str("b, i64 1400, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmtfp, i64 0, i64 0), i32 "); s.push_str(&tmp(*t)); s.push_str("w, double "); s.push_str(&tmp(*t)); s.push_str("d)\n  "); s.push_str(&tmp(*t)); s.push_str(" = ptrtoint i8* "); s.push_str(&tmp(*t)); s.push_str("b to i64\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="f64_snprintf_e" && av.len()==2 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("b = call i8* @malloc(i64 1400)\n  "); s.push_str(&tmp(*t)); s.push_str("d = bitcast i64 "); s.push_str(&tmp(av[1])); s.push_str(" to double\n  "); s.push_str(&tmp(*t)); s.push_str("w = trunc i64 "); s.push_str(&tmp(av[0])); s.push_str(" to i32\n  "); s.push_str(&tmp(*t)); s.push_str("c = call i32 (i8*, i64, i8*, ...) @snprintf(i8* "); s.push_str(&tmp(*t)); s.push_str("b, i64 1400, i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmtfe, i64 0, i64 0), i32 "); s.push_str(&tmp(*t)); s.push_str("w, double "); s.push_str(&tmp(*t)); s.push_str("d)\n  "); s.push_str(&tmp(*t)); s.push_str(" = ptrtoint i8* "); s.push_str(&tmp(*t)); s.push_str("b to i64\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="f64_isnan" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("d = bitcast i64 "); s.push_str(&tmp(av[0])); s.push_str(" to double\n  "); s.push_str(&tmp(*t)); s.push_str("c = fcmp uno double "); s.push_str(&tmp(*t)); s.push_str("d, "); s.push_str(&tmp(*t)); s.push_str("d\n  "); s.push_str(&tmp(*t)); s.push_str(" = zext i1 "); s.push_str(&tmp(*t)); s.push_str("c to i64\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="print_char" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("w = trunc i64 "); s.push_str(&tmp(av[0])); s.push_str(" to i32\n  "); s.push_str(&tmp(*t)); s.push_str("c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @.fmtc, i64 0, i64 0), i32 "); s.push_str(&tmp(*t)); s.push_str("w)\n  "); s.push_str(&tmp(*t)); s.push_str(" = sext i32 "); s.push_str(&tmp(*t)); s.push_str("c to i64\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="print_nl" && av.len()==0 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @.fmtnl, i64 0, i64 0))\n  "); s.push_str(&tmp(*t)); s.push_str(" = sext i32 "); s.push_str(&tmp(*t)); s.push_str("c to i64\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="print_int" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("c = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @.fmtd, i64 0, i64 0), i64 "); s.push_str(&tmp(av[0])); s.push_str(")\n  "); s.push_str(&tmp(*t)); s.push_str(" = sext i32 "); s.push_str(&tmp(*t)); s.push_str("c to i64\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="argc" && av.len()==0 =>{ s.push_str("  "); s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__argc_get()\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="argv_at" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__argv_at(i64 "); s.push_str(&tmp(av[0])); s.push_str(")\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="fileok" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__fileok(i64 "); s.push_str(&tmp(av[0])); s.push_str(")\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="readfile" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t));
                s.push_str(" = call i64 @__readfile(i64 "); s.push_str(&tmp(av[0])); s.push_str(")\n"); },
            Op::CallOp(t,n,av) if n.as_str()=="len" && av.len()==1 =>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("p = inttoptr i64 "); s.push_str(&tmp(av[0])); s.push_str(" to i8*\n  "); s.push_str(&tmp(*t)); s.push_str(" = call i64 @strlen(i8* "); s.push_str(&tmp(*t)); s.push_str("p)\n"); },
            Op::CallOp(t,n,av)=>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = call i64 @"); s.push_str(n); s.push_str("("); let mut j=0; while j<av.len(){ if j>0 {s.push_str(", ");} s.push_str("i64 "); s.push_str(&tmp(av[j])); j=j+1; } s.push_str(")\n"); },
            Op::Const(t,v)=>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = add i64 0, "); s.push_str(&n64(*v)); s.push_str("\n"); },
            Op::Bin(t,o,x,y) if is_fop(o) =>{
                // an f64 rides our 64-bit word, so a float op is a bitcast pair around the real opcode
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("a = bitcast i64 "); s.push_str(&tmp(*x)); s.push_str(" to double\n");
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("b = bitcast i64 "); s.push_str(&tmp(*y)); s.push_str(" to double\n");
                if fop_is_cmp(o) {
                    s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("c = fcmp "); s.push_str(&fllop(o)); s.push_str(" double "); s.push_str(&tmp(*t)); s.push_str("a, "); s.push_str(&tmp(*t)); s.push_str("b\n");
                    s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = zext i1 "); s.push_str(&tmp(*t)); s.push_str("c to i64\n");
                } else {
                    s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("r = "); s.push_str(&fllop(o)); s.push_str(" double "); s.push_str(&tmp(*t)); s.push_str("a, "); s.push_str(&tmp(*t)); s.push_str("b\n");
                    s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = bitcast double "); s.push_str(&tmp(*t)); s.push_str("r to i64\n"); }
            },
            Op::Bin(t,o,x,y)=>{
                if o.as_str()=="<" || o.as_str()=="==" || o.as_str()=="<=" || o.as_str()==">" || o.as_str()==">=" || o.as_str()=="!=" {
                    let pred = if o.as_str()=="<" { "slt" } else if o.as_str()=="<=" { "sle" } else if o.as_str()==">" { "sgt" } else if o.as_str()==">=" { "sge" } else if o.as_str()=="!=" { "ne" } else { "eq" };
                    s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".c = icmp "); s.push_str(pred); s.push_str(" i64 "); s.push_str(&tmp(*x)); s.push_str(", "); s.push_str(&tmp(*y)); s.push_str("\n");
                    s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = zext i1 "); s.push_str(&tmp(*t)); s.push_str(".c to i64\n");
                } else if o.as_str()=="/" || o.as_str()=="%" {
                    s.push_str("  "); s.push_str(&tmp(*t)); s.push_str("d = call i64 @__divchk(i64 "); s.push_str(&tmp(*y)); s.push_str(")\n");
                    s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = "); s.push_str(&llop(o)); s.push_str(" i64 "); s.push_str(&tmp(*x)); s.push_str(", "); s.push_str(&tmp(*t)); s.push_str("d\n");
                } else { s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = "); s.push_str(&llop(o)); s.push_str(" i64 "); s.push_str(&tmp(*x)); s.push_str(", "); s.push_str(&tmp(*y)); s.push_str("\n"); }
            },
            Op::LoadLoc(t,sl)=>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = load "); s.push_str(&lt); s.push_str(", "); s.push_str(&lt); s.push_str("* "); s.push_str(&loc(*sl)); s.push_str("\n"); },
            Op::StoreLoc(sl,t)=>{ s.push_str("  store "); s.push_str(&lt); s.push_str(" "); s.push_str(&tmp(*t)); s.push_str(", "); s.push_str(&lt); s.push_str("* "); s.push_str(&loc(*sl)); s.push_str("\n"); },
            Op::IfOp(t,c0,tops,tr,eops,er)=>{
                let lid=*t; let lid2=lid0;
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".b = icmp ne i64 "); s.push_str(&tmp(*c0)); s.push_str(", 0\n");
                s.push_str("  br i1 "); s.push_str(&tmp(*t)); s.push_str(".b, label %then"); s.push_str(&n32(lid)); s.push_str(", label %else"); s.push_str(&n32(lid)); s.push_str("\n");
                let tret=ops_return(tops); let eret=ops_return(eops);
                s.push_str("then"); s.push_str(&n32(lid)); s.push_str(":\n"); s.push_str(&body_ops_in(tops, lid2));
                s.push_str("  br label %thx"); s.push_str(&n32(lid)); s.push_str("\n");
                s.push_str("thx"); s.push_str(&n32(lid)); s.push_str(":\n");
                s.push_str("  br label %join"); s.push_str(&n32(lid)); s.push_str("\n");
                s.push_str("else"); s.push_str(&n32(lid)); s.push_str(":\n"); s.push_str(&body_ops_in(eops, lid2));
                s.push_str("  br label %elx"); s.push_str(&n32(lid)); s.push_str("\n");
                s.push_str("elx"); s.push_str(&n32(lid)); s.push_str(":\n");
                s.push_str("  br label %join"); s.push_str(&n32(lid)); s.push_str("\n");
                s.push_str("join"); s.push_str(&n32(lid)); s.push_str(":\n");
                // a branch whose body RETURNS reaches `thx`/`elx` only through the unreachable label the
                // early return left behind, and LLVM would still demand a value for it; with one live arm the
                // join takes that arm's value directly instead of a two-incoming phi.
                if tret && !eret { s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = add i64 0, "); s.push_str(&tmp(*er)); s.push_str("\n"); }
                else if eret && !tret { s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = add i64 0, "); s.push_str(&tmp(*tr)); s.push_str("\n"); }
                else { s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = phi i64 [ "); s.push_str(&tmp(*tr)); s.push_str(", %thx"); s.push_str(&n32(lid)); s.push_str(" ], [ "); s.push_str(&tmp(*er)); s.push_str(", %elx"); s.push_str(&n32(lid)); s.push_str(" ]\n"); }   // C11: explicit join-PREDECESSOR blocks (thx/elx) -- with a NESTED if the control arriving at join comes from an inner join, so naming then/else in the phi is invalid IR
            },
            Op::LoopOp(cops,cr,bops)=>{
                let lid=*cr;
                s.push_str("  br label %lcond"); s.push_str(&n32(lid)); s.push_str("\n");
                s.push_str("lcond"); s.push_str(&n32(lid)); s.push_str(":\n"); s.push_str(&body_ops_in(cops, 0-1));
                s.push_str("  %lc"); s.push_str(&n32(lid)); s.push_str(" = icmp ne i64 "); s.push_str(&tmp(*cr)); s.push_str(", 0\n");
                s.push_str("  br i1 %lc"); s.push_str(&n32(lid)); s.push_str(", label %lbody"); s.push_str(&n32(lid)); s.push_str(", label %lend"); s.push_str(&n32(lid)); s.push_str("\n");
                s.push_str("lbody"); s.push_str(&n32(lid)); s.push_str(":\n"); s.push_str(&body_ops_in(bops, lid as i64));
                s.push_str("  br label %lcond"); s.push_str(&n32(lid)); s.push_str("\n");
                s.push_str("lend"); s.push_str(&n32(lid)); s.push_str(":\n");
            },
            Op::MakeStruct(t,nf,av)=>{
                let sz=crate::oabi::struct_size(*nf);
                // HEAP, not alloca: an aggregate built in a callee's frame is RETURNED as a handle, and a
                // stack slot dies with the frame (`Pt::origin()` then read a clobbered 2 instead of 0). Our
                // value model is "aggregates are heap handles", so the allocation has to outlive the frame.
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".mb = call i8* @malloc(i64 "); s.push_str(&n32(sz)); s.push_str(")\n");
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".m = bitcast i8* "); s.push_str(&tmp(*t)); s.push_str(".mb to ["); s.push_str(&n32(*nf)); s.push_str(" x i64]*\n");
                let mut k=0; while k<av.len(){
                    s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".f"); s.push_str(&n32(k as u32)); s.push_str(" = getelementptr [" ); s.push_str(&n32(*nf)); s.push_str(" x i64], [" ); s.push_str(&n32(*nf)); s.push_str(" x i64]* "); s.push_str(&tmp(*t)); s.push_str(".m, i64 0, i64 "); s.push_str(&n32(k as u32)); s.push_str("\n");
                    s.push_str("  store i64 "); s.push_str(&tmp(av[k])); s.push_str(", i64* "); s.push_str(&tmp(*t)); s.push_str(".f"); s.push_str(&n32(k as u32)); s.push_str("\n");
                    k=k+1; }
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".p = getelementptr [" ); s.push_str(&n32(*nf)); s.push_str(" x i64], [" ); s.push_str(&n32(*nf)); s.push_str(" x i64]* "); s.push_str(&tmp(*t)); s.push_str(".m, i64 0, i64 0\n");
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = ptrtoint i64* "); s.push_str(&tmp(*t)); s.push_str(".p to i64   ; struct size="); s.push_str(&n32(sz)); s.push_str("\n");
            },
            Op::LoadField(t,bt,ix)=>{
                let off=crate::oabi::field_offset(*ix);
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".bp = inttoptr i64 "); s.push_str(&tmp(*bt)); s.push_str(" to i8*\n");
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".fp8 = getelementptr i8, i8* "); s.push_str(&tmp(*t)); s.push_str(".bp, i64 "); s.push_str(&n32(off)); s.push_str("\n");
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".fp = bitcast i8* "); s.push_str(&tmp(*t)); s.push_str(".fp8 to i64*\n");
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = load i64, i64* "); s.push_str(&tmp(*t)); s.push_str(".fp\n");
            },
            Op::StoreIdx(bt,it,vt)=>{ s.push_str("  %si"); s.push_str(&n32(*vt)); s.push_str(" = call i64 @__arr_set(i64 "); s.push_str(&tmp(*bt)); s.push_str(", i64 "); s.push_str(&tmp(*it)); s.push_str(", i64 "); s.push_str(&tmp(*vt)); s.push_str(")\n"); },
            Op::StoreField(bt,fx,vt)=>{
                s.push_str("  %sf"); s.push_str(&n32(*vt)); s.push_str(".bp = inttoptr i64 "); s.push_str(&tmp(*bt)); s.push_str(" to i64*\n");
                s.push_str("  %sf"); s.push_str(&n32(*vt)); s.push_str(".ep = getelementptr i64, i64* %sf"); s.push_str(&n32(*vt)); s.push_str(".bp, i64 "); s.push_str(&n32(*fx)); s.push_str("\n");
                s.push_str("  store i64 "); s.push_str(&tmp(*vt)); s.push_str(", i64* %sf"); s.push_str(&n32(*vt)); s.push_str(".ep\n");
            },
            Op::MakeArr(t,n,av)=>{
                let np=(*n)+1;
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".ab = call i8* @malloc(i64 "); s.push_str(&n32(crate::oabi::struct_size(np))); s.push_str(")\n");
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".a = bitcast i8* "); s.push_str(&tmp(*t)); s.push_str(".ab to ["); s.push_str(&n32(np)); s.push_str(" x i64]*\n");
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".lp = getelementptr ["); s.push_str(&n32(np)); s.push_str(" x i64], ["); s.push_str(&n32(np)); s.push_str(" x i64]* "); s.push_str(&tmp(*t)); s.push_str(".a, i64 0, i64 0\n");
                s.push_str("  store i64 "); s.push_str(&n32(*n)); s.push_str(", i64* "); s.push_str(&tmp(*t)); s.push_str(".lp\n");
                let mut k=0; while k<av.len(){
                    s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".e"); s.push_str(&n32(k as u32)); s.push_str(" = getelementptr ["); s.push_str(&n32(np)); s.push_str(" x i64], ["); s.push_str(&n32(np)); s.push_str(" x i64]* "); s.push_str(&tmp(*t)); s.push_str(".a, i64 0, i64 "); s.push_str(&n32((k as u32)+1)); s.push_str("\n");
                    s.push_str("  store i64 "); s.push_str(&tmp(av[k])); s.push_str(", i64* "); s.push_str(&tmp(*t)); s.push_str(".e"); s.push_str(&n32(k as u32)); s.push_str("\n"); k=k+1; }
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = ptrtoint ["); s.push_str(&n32(np)); s.push_str(" x i64]* "); s.push_str(&tmp(*t)); s.push_str(".a to i64\n");
            },
            Op::LoadIdx(t,bt,it)=>{ s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = call i64 @__arr_get(i64 "); s.push_str(&tmp(*bt)); s.push_str(", i64 "); s.push_str(&tmp(*it)); s.push_str(")\n"); },
            Op::MakeEnum(t,tag,av)=>{
                let n=(av.len() as u32)+1;
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".mb = call i8* @malloc(i64 "); s.push_str(&n32(crate::oabi::struct_size(n))); s.push_str(")\n");
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".m = bitcast i8* "); s.push_str(&tmp(*t)); s.push_str(".mb to ["); s.push_str(&n32(n)); s.push_str(" x i64]*\n");
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".t = getelementptr ["); s.push_str(&n32(n)); s.push_str(" x i64], ["); s.push_str(&n32(n)); s.push_str(" x i64]* "); s.push_str(&tmp(*t)); s.push_str(".m, i64 0, i64 0\n");
                s.push_str("  store i64 "); s.push_str(&n32(*tag)); s.push_str(", i64* "); s.push_str(&tmp(*t)); s.push_str(".t\n");
                let mut k=0; while k<av.len(){
                    s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(".p"); s.push_str(&n32(k as u32)); s.push_str(" = getelementptr ["); s.push_str(&n32(n)); s.push_str(" x i64], ["); s.push_str(&n32(n)); s.push_str(" x i64]* "); s.push_str(&tmp(*t)); s.push_str(".m, i64 0, i64 "); s.push_str(&n32((k as u32)+1)); s.push_str("\n");
                    s.push_str("  store i64 "); s.push_str(&tmp(av[k])); s.push_str(", i64* "); s.push_str(&tmp(*t)); s.push_str(".p"); s.push_str(&n32(k as u32)); s.push_str("\n"); k=k+1; }
                s.push_str("  "); s.push_str(&tmp(*t)); s.push_str(" = ptrtoint ["); s.push_str(&n32(n)); s.push_str(" x i64]* "); s.push_str(&tmp(*t)); s.push_str(".m to i64\n");
            },
            Op::MatchOp(dest,st,arms)=>{
                let mid=*dest;
                s.push_str("  "); s.push_str(&tmp(mid)); s.push_str(".bp = inttoptr i64 "); s.push_str(&tmp(*st)); s.push_str(" to i64*\n");
                s.push_str("  "); s.push_str(&tmp(mid)); s.push_str(".tg = load i64, i64* "); s.push_str(&tmp(mid)); s.push_str(".bp\n");
                s.push_str("  br label %m"); s.push_str(&n32(mid)); s.push_str("a0\n");   // enter the first arm TEST block
                let mut k=0;
                while k<arms.len(){
                    let a=&arms[k];
                    let armlbl=format!("m{}a{}", n32(mid), n32(k as u32));
                    let nextlbl= if k+1<arms.len() { format!("m{}a{}", n32(mid), n32((k as u32)+1)) } else { format!("m{}dflt", n32(mid)) };
                    s.push_str(&armlbl); s.push_str(":\n");   // C11: every arm TEST needs its own basic block -- emitting the next test after a terminator with no label produced an undefined branch target
                    if a.tag>=0 {
                        s.push_str("  "); s.push_str(&tmp(mid)); s.push_str(".c"); s.push_str(&n32(k as u32)); s.push_str(" = icmp eq i64 "); s.push_str(&tmp(mid)); s.push_str(".tg, "); s.push_str(&n32(a.tag as u32)); s.push_str("\n");
                        s.push_str("  br i1 "); s.push_str(&tmp(mid)); s.push_str(".c"); s.push_str(&n32(k as u32)); s.push_str(", label %"); s.push_str(&armlbl); s.push_str("b, label %"); s.push_str(&nextlbl); s.push_str("\n");
                    } else { s.push_str("  br label %"); s.push_str(&armlbl); s.push_str("b\n"); }
                    s.push_str(&armlbl); s.push_str("b:\n");
                    // bind payload slots from the cell
                    let mut q=0; while q<a.slots.len(){
                        s.push_str("  "); s.push_str(&tmp(mid)); s.push_str(".g"); s.push_str(&n32(k as u32)); s.push_str("_"); s.push_str(&n32(q as u32)); s.push_str(" = getelementptr i64, i64* "); s.push_str(&tmp(mid)); s.push_str(".bp, i64 "); s.push_str(&n32((q as u32)+1)); s.push_str("\n");
                        s.push_str("  "); s.push_str(&tmp(mid)); s.push_str(".v"); s.push_str(&n32(k as u32)); s.push_str("_"); s.push_str(&n32(q as u32)); s.push_str(" = load i64, i64* "); s.push_str(&tmp(mid)); s.push_str(".g"); s.push_str(&n32(k as u32)); s.push_str("_"); s.push_str(&n32(q as u32)); s.push_str("\n");
                        s.push_str("  store i64 "); s.push_str(&tmp(mid)); s.push_str(".v"); s.push_str(&n32(k as u32)); s.push_str("_"); s.push_str(&n32(q as u32)); s.push_str(", i64* "); s.push_str(&loc(a.slots[q])); s.push_str("\n");
                        q=q+1; }
                    s.push_str(&body_ops_in(&a.ops, lid0));
                    s.push_str("  store i64 "); s.push_str(&tmp(a.res)); s.push_str(", i64* "); s.push_str(&tmp(mid)); s.push_str(".slot\n");
                    s.push_str("  br label %m"); s.push_str(&n32(mid)); s.push_str("end\n");
                    k=k+1;
                }
                s.push_str("m"); s.push_str(&n32(mid)); s.push_str("dflt:\n");
                s.push_str("  store i64 0, i64* "); s.push_str(&tmp(mid)); s.push_str(".slot\n");
                s.push_str("  br label %m"); s.push_str(&n32(mid)); s.push_str("end\n");
                s.push_str("m"); s.push_str(&n32(mid)); s.push_str("end:\n");
                s.push_str("  "); s.push_str(&tmp(mid)); s.push_str(" = load i64, i64* "); s.push_str(&tmp(mid)); s.push_str(".slot\n");
            },
            Op::Ret(src)=>{ s.push_str("  ret i64 "); s.push_str(&tmp(*src)); s.push_str("\n"); },
        }
        k=k+1;
    }
    s
}
}
mod osession {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
pub struct Opts { pub out: String, pub emit_ir: bool, pub opt: u32, pub input: String }
pub fn opts_new()->Opts{ Opts{out:String::from("a.out"), emit_ir:false, opt:0, input:String::from("")} }
pub fn parse_args(args:&Vec<String>)->Opts{
    let mut o=opts_new(); let mut i=0;
    while i<args.len(){
        let a=&args[i];
        if a.as_str()=="-o" { i=i+1; if i<args.len() { o.out=String::from(args[i].as_str()); } }
        else if a.as_str()=="--emit-ir" { o.emit_ir=true; }
        else if a.as_str()=="-O" { o.opt=2; }
        else { o.input=String::from(a.as_str()); }
        i=i+1;
    }
    o
}
pub fn opt_name(o:&Opts)->String{ if o.opt==2 {String::from("O2")} else {String::from("O0")} }
}
mod oparse {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::otypeck::*;
    use crate::omono::*;
    use crate::oborrowck::*;
// oparse -- the PARSER: tokens -> Prog (items, expressions, desugars)
pub struct MacRule { pub pat: Vec<Tok>, pub body: Vec<Tok> }
pub struct Mac { pub name: String, pub rules: Vec<MacRule> }
pub fn tok_clone(t:&Tok)->Tok{ Tok{k:t.k.clone(), lo:t.lo, hi:t.hi} }
pub fn toks_clone(v:&Vec<Tok>)->Vec<Tok>{ let mut o:Vec<Tok>=Vec::new(); let mut i=0; while i<v.len(){ o.push(tok_clone(&v[i])); i=i+1; } o }
pub fn tok_word(t:&Tok)->String{ match t.k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("") } }
// do two tokens denote the same thing? (kind and, for an identifier, its text)
pub fn tok_same(a:&Tok, b:&Tok)->bool{
    let wa=tok_word(a); let wb=tok_word(b);
    if wa.len()>0 || wb.len()>0 { return wa.as_str()==wb.as_str(); }
    let ks=["(",")","{","}","[","]",",",";",":","=","=>","::","$","+","-","*","/","%","<",">","==","!=","<=",">=","&","|","^","!","?","..","<<",">>","."];
    let mut i=0;
    while i<ks.len(){ let ia=tk_is(&a.k, ks[i]); let ib=tk_is(&b.k, ks[i]); if ia || ib { return ia && ib; } i=i+1; }
    match a.k.clone() { Tk::Num(x)=>{ match b.k.clone() { Tk::Num(y)=>x==y, _=>false } },
                        Tk::ChL(x)=>{ match b.k.clone() { Tk::ChL(y)=>x==y, _=>false } },
                        Tk::StrL(x)=>{ match b.k.clone() { Tk::StrL(y)=>x.as_str()==y.as_str(), _=>false } },
                        _=>false } }
pub fn tok_opens(t:&Tok)->bool{ tk_is(&t.k,"(") || tk_is(&t.k,"{") || tk_is(&t.k,"[") }
pub fn tok_closes(t:&Tok)->bool{ tk_is(&t.k,")") || tk_is(&t.k,"}") || tk_is(&t.k,"]") }
pub struct P { globs: Vec<String>, fnames: Vec<String>, lvhit: Vec<String>, loopv: Vec<String>, mscal: Vec<String>, smut: Vec<String>, vfn: Vec<String>, atn: Vec<String>, att: Vec<String>, selfty: String, usen: Vec<String>, uset: Vec<String>, namesonly: bool, perrexp: String, perrgot: String, nhyg: u32, macs: Vec<Mac>, depth: u32, vnames: Vec<String>, vowner: Vec<String>, sfnames: Vec<String>, sawcopy: bool, aliasn: Vec<String>, aliast: Vec<String>, perr: u32, badfmt: u32, consts: Vec<String>, t: Vec<Tok>, i: usize, snames: Vec<String>, enames: Vec<String>, clos: Vec<Fun>, nfor: u32, mpfx: String, mods: Vec<String> }
// an integer literal, whichever KIND it was written as: 1 = a number, 2 = a char literal, 0 = neither.
// Every position that only needs the VALUE (an array repeat count, a range endpoint, a match literal)
// asks this, so a char literal works everywhere a number does without 15 copies of the test.
pub fn ilit(t:&Tok)->i32{ match &t.k { Tk::Num(_n)=>1, Tk::ChL(_n)=>2, _=>0 } }
pub fn is_float_tok(t:&Tok)->bool{ match &t.k { Tk::FloatL(_x)=>true, _=>false } }
pub fn ilit_val(t:&Tok)->i64{ match &t.k { Tk::Num(n)=>*n, Tk::ChL(n)=>*n, _=>0 } }
// the owner prefix of a qualified item name: `S$n` -> `S`, `main` -> ""
// is this expression a PLACE? (read at format time, the way rustc reads a reference)
pub fn is_place_expr(e:&E)->bool{
    match e { E::Var(_n,_l,_h)=>true,
              E::Field(_o,_f,_l,_h)=>true,
              E::TupIdx(_o,_i,_l,_h)=>true,
              E::Index(_b,_i,_l,_h)=>true,
              E::Call(n,a,_l,_h)=>{ a.len()==0 && crate::olower::is_gname(n) },
              _=>false } }
// a `&mut` to a SCALAR needs a place our value model does not have; a `&mut` to an AGGREGATE needs
// nothing, because our aggregates are heap handles and a handle already behaves as a reference (the
// probe's p5/p6 agree today). So the refusal is exactly scalar-shaped.
pub fn find_sname(ss:&Vec<Sdef>, n:&str)->bool{ let mut i=0; while i<ss.len(){ if ss[i].name.as_str()==n { return true; } i=i+1; } false }
pub fn scalar_ty(n:&str)->bool{ n=="int" || n=="bool" || n=="char" || n=="f64" || n=="str" || crate::oast::is_int_width(n) }
pub fn fn_bounded(gps:&Vec<String>, gbs:&Vec<String>, ty:&str)->bool{
    let mut i=0; while i<gps.len(){
        if gps[i].as_str()==ty && i<gbs.len() {
            let b=gbs[i].as_bytes();
            if b.len()>=2 && b[0]==70 && b[1]==110 { return true; } }
        i=i+1; }
    false }
pub fn lit_ty(e:&E)->String{
    match e {
        E::StrLit(_v,_l,_h)=>String::from("str"),
        E::Int(_v,_l,_h)=>String::from("int"),
        E::Bool(_v,_l,_h)=>String::from("bool"),
        E::Cast(_x,t,_l,_h)=>String::from(t.as_str()),
        E::StructLit(n,_f,_l,_h)=>String::from(n.as_str()),
        E::EnumLit(n,_v,_p,_l,_h)=>String::from(n.as_str()),
        E::LetT(_n,t,_v,_b,_l,_h)=>String::from(t.as_str()),
        E::TupLit(parts,_l,_h)=>{ let mut es:Vec<String>=Vec::new(); let mut k=0;
            while k<parts.len(){ es.push(lit_ty(&parts[k])); k=k+1; }
            tup_name(parts.len() as u32, &es) },
        E::Call(n,_a,_l,_h)=>{ if n.as_str()=="f64_parse" { String::from("f64") } else { String::from("") } },
        _=>String::from("") } }
// A tuple is a NAME like every other type here, and until now that name carried only the ARITY -- so a
// tuple VARIABLE could not be Debug-printed (its elements were unknown) while a tuple LITERAL could (its
// elements were right there). The name now carries the element types: `tup2$int$str`. Only LEAF element
// names go in -- an element that is itself composite (a Vec, an array, another tuple, a closure, a dyn)
// would make the name ambiguous to read back, so such a tuple keeps the bare arity name and keeps the
// refusal it has today. Integer widths normalize to `int` so an annotation `(i64, i64)` and the literal
// `(1, 2)` mint the SAME name.
pub fn tup_leaf(t:&str)->String{
    if t.len()==0 { return String::from(""); }
    { let b=t.as_bytes(); let mut i=0; while i<b.len(){ if b[i]==36 { return String::from(""); } i=i+1; } }
    if t=="Vec" || t=="arr" || t=="Fn" || t=="unit" { return String::from(""); }
    if tup_arity_name(t)>=0 { return String::from(""); }
    if t=="char" || t=="bool" || t=="str" || t=="f64" { return String::from(t); }
    if t=="int" || is_int_width(t) { return String::from("int"); }
    String::from(t) }
// C-SLICEREAD: the methods that WRITE through their receiver. A slice in a value position is a copy in
// this value model, so these would write to the copy and the caller would never see it -- they stay
// refused. Everything else only reads, and rustc accepts those on a slice, so they are admitted.
pub fn slice_mutator(m:&str)->bool{
    m=="push" || m=="pop" || m=="push_str" || m=="insert" || m=="remove" || m=="clear"
    || m=="sort" || m=="sort_by" || m=="sort_unstable" || m=="reverse" || m=="truncate"
    || m=="set" || m=="swap" || m=="retain" || m=="dedup" || m=="fill" || m=="extend"
    || m=="append" || m=="drain" || m=="resize" }
pub fn tup_arity_name(tn:&str)->i32{ let b=tn.as_bytes(); if b.len()<4 { return -1; } if b[0]!=116 || b[1]!=117 || b[2]!=112 { return -1; } let mut v=0; let mut i=3; let mut any=false; while i<b.len(){ if b[i]==36 { break; } if b[i]<48 || b[i]>57 { return -1; } v=v*10+((b[i]-48) as i32); any=true; i=i+1; } if !any { return -1; } v }
pub fn tup_name(n:u32, es:&Vec<String>)->String{
    let mut r=String::from("tup"); r.push_str(&u32s2(n));
    if (es.len() as u32)!=n { return r; }
    let mut parts=String::from(""); let mut k=0;
    while k<es.len(){ let l=tup_leaf(es[k].as_str()); if l.len()==0 { return r; } parts.push('$'); parts.push_str(l.as_str()); k=k+1; }
    r.push_str(parts.as_str()); r }
// the element types of a tuple name, or empty when the name carries only the arity
pub fn tup_elems(tn:&str)->Vec<String>{ let mut o:Vec<String>=Vec::new();
    let b=tn.as_bytes(); let mut i=3; if tup_arity_name(tn)<0 { return o; }
    while i<b.len(){ if b[i]==36 { break; } i=i+1; }
    if i>=b.len() { return o; }
    let mut start=i+1; let mut j=start;
    while j<=b.len(){ if j==b.len() || b[j]==36 { o.push(String::from(&tn[start..j])); start=j+1; } j=j+1; }
    o }
pub fn name_owner(n:&str)->String{ let b=n.as_bytes(); let mut i=0;
    while i<b.len(){ if b[i]==36 { return String::from(&n[0..i]); } i=i+1; }
    String::from("") }
// does this expression perform OUTPUT? (the only side effect our value model has)
// does this body WRITE through the named binding? (`*x = v` erases to `x = v` here)
pub fn assigns_to(e:&E, n:&str)->bool{
    match e {
        E::Assign(m,v,_l,_h)=>{ if m.as_str()==n { return true; } assigns_to(v,n) },
        E::AssignTo(pl,v,_l,_h)=>{ match pl.as_ref() { E::Var(m,_l2,_h2)=>{ if m.as_str()==n { return true; } }, _=>{} } assigns_to(pl,n) || assigns_to(v,n) },
        E::Seq(a,b,_l,_h)=>assigns_to(a,n) || assigns_to(b,n),
        E::Let(_x,v,b,_l,_h)=>assigns_to(v,n) || assigns_to(b,n),
        E::LetT(_x,_t,v,b,_l,_h)=>assigns_to(v,n) || assigns_to(b,n),
        E::If(c,t,f,_l,_h)=>assigns_to(c,n) || assigns_to(t,n) || assigns_to(f,n),
        E::While(c,b,_l,_h)=>assigns_to(c,n) || assigns_to(b,n),
        E::Bin(_o,a,b,_l,_h)=>assigns_to(a,n) || assigns_to(b,n),
        E::Match(s2,arms,_l,_h)=>{ if assigns_to(s2,n) { return true; } let mut i=0; while i<arms.len(){ if assigns_to(&arms[i].body,n) { return true; } i=i+1; } false },
        E::Call(_f,a,_l,_h)=>{ let mut i=0; while i<a.len(){ if assigns_to(&a[i],n) { return true; } i=i+1; } false },
        E::MethodCall(r,_m,a,_l,_h)=>{ if assigns_to(r,n) { return true; } let mut i=0; while i<a.len(){ if assigns_to(&a[i],n) { return true; } i=i+1; } false },
        E::RetE(x,_l,_h)=>assigns_to(x,n),
        _=>false } }
pub fn body_has_io(e:&E)->bool{
    match e {
        E::Call(n,a,_l,_h)=>{ let io = n.as_str()=="print_any" || n.as_str()=="print_dbg" || n.as_str()=="print_raw"
                                 || n.as_str()=="print_int_raw" || n.as_str()=="print_nl" || n.as_str()=="print"
                                 || n.as_str()=="print_int" || n.as_str()=="panic_exit" || n.as_str()=="say";
                              if io { return true; }
                              let mut i=0; while i<a.len(){ if body_has_io(&a[i]) { return true; } i=i+1; } false },
        E::Seq(a,b,_l,_h)=>body_has_io(a) || body_has_io(b),
        E::Let(_n,v,b,_l,_h)=>body_has_io(v) || body_has_io(b),
        E::LetT(_n,_t,v,b,_l,_h)=>body_has_io(v) || body_has_io(b),
        E::If(c,t,f,_l,_h)=>body_has_io(c) || body_has_io(t) || body_has_io(f),
        E::While(c,b,_l,_h)=>body_has_io(c) || body_has_io(b),
        E::Bin(_o,a,b,_l,_h)=>body_has_io(a) || body_has_io(b),
        E::MethodCall(r,_m,a,_l,_h)=>{ if body_has_io(r) { return true; } let mut i=0; while i<a.len(){ if body_has_io(&a[i]) { return true; } i=i+1; } false },
        E::Match(s2,arms,_l,_h)=>{ if body_has_io(s2) { return true; } let mut i=0; while i<arms.len(){ if body_has_io(&arms[i].body) { return true; } i=i+1; } false },
        E::RetE(x,_l,_h)=>body_has_io(x),
        E::Assign(_n,v,_l,_h)=>body_has_io(v),
        _=>false } }
pub fn split_dollar(s:&str)->Vec<String>{ let b=s.as_bytes(); let mut o:Vec<String>=Vec::new(); let mut st=0; let mut i=0;
    while i<=b.len(){ if i==b.len() || b[i]==36 { o.push(String::from(&s[st..i])); st=i+1; } i=i+1; } o }
pub fn tk_is(t:&Tk, k:&str)->bool{ match t { Tk::Fn=>k=="fn", Tk::While=>k=="while", Tk::Mut=>k=="mut", Tk::Struct=>k=="struct", Tk::Dot=>k==".", Tk::Colon=>k==":", Tk::Gt=>k==">", Tk::Le=>k=="<=", Tk::Ge=>k==">=", Tk::Ne=>k=="!=", Tk::AndAnd=>k=="&&", Tk::OrOr=>k=="||", Tk::Bang=>k=="!", Tk::Enum=>k=="enum", Tk::Impl=>k=="impl", Tk::Trait=>k=="trait", Tk::Ret=>k=="return", Tk::DynT=>k=="dyn", Tk::Bar=>k=="|", Tk::DotDot=>k=="..", Tk::Quest=>k=="?", Tk::ModT=>k=="mod", Tk::AsT=>k=="as", Tk::UnsafeT=>k=="unsafe", Tk::LoopT=>k=="loop", Tk::Dollar=>k=="$", Tk::DeriveCopy=>k=="#copy", Tk::WhereT=>k=="where", Tk::TypeT=>k=="type", Tk::UseT=>k=="use", Tk::ExternT=>k=="extern", Tk::CrateT=>k=="crate", Tk::Caret=>k=="^", Tk::Shl=>k=="<<", Tk::Shr=>k==">>", Tk::Amp=>k=="&", Tk::ConstT=>k=="const", Tk::StaticT=>k=="static", Tk::BreakT=>k=="break", Tk::ContT=>k=="continue", Tk::PlusEq=>k=="+=", Tk::MinusEq=>k=="-=", Tk::StarEq=>k=="*=", Tk::SlashEq=>k=="/=", Tk::InT=>k=="in", Tk::StrL(_x)=>k=="str", Tk::ForT=>k=="for", Tk::Match=>k=="match", Tk::FatArrow=>k=="=>", Tk::PathSep=>k=="::", Tk::LBrack=>k=="[", Tk::RBrack=>k=="]", Tk::Comma=>k==",", Tk::Plus=>k=="+", Tk::Minus=>k=="-", Tk::Star=>k=="*", Tk::Slash=>k=="/", Tk::Percent=>k=="%", Tk::Lt=>k=="<", Tk::EqEq=>k=="==", Tk::LParen=>k=="(", Tk::RParen=>k==")", Tk::LBrace=>k=="{", Tk::RBrace=>k=="}", Tk::Let=>k=="let", Tk::If=>k=="if", Tk::Else=>k=="else", Tk::Eq=>k=="=", Tk::Semi=>k==";", _=>false } }
// `Vec::new()` / `String::new()` / `i32::MAX` -- the std names whose meaning our value model already
// has. A name the program itself declares always wins (a user `struct Vec` keeps its own constructor).
pub fn std_path(segs:&Vec<String>, user:bool)->String{
    if user || segs.len()!=2 { return String::from(""); }
    let a=segs[0].as_str(); let b=segs[1].as_str();
    if a=="Vec" && (b=="new" || b=="with_capacity") { return String::from("vec_new"); }
    if a=="String" && (b=="new" || b=="with_capacity") { return String::from("$emptystr"); }
    if a=="String" && (b=="from" || b=="from_str") { return String::from("$id1"); }
    if a=="process" && b=="exit" { return String::from("proc_exit"); }
    if a=="fs" && (b=="read_to_string" || b=="read") { return String::from("freadok"); }
    if a=="env" && b=="args" { return String::from("eargs"); }   // C-ARGV   // C-READFILE: a Result, like rustc, via a prelude wrapper over the raw primitive
    if crate::oast::is_int_width(a) && (b=="MAX" || b=="MIN") { return String::from("$int"); }
    String::from("") }
pub fn std_int_bound(segs:&Vec<String>)->i64{
    let a=segs[0].as_str(); let mx=segs[1].as_str()=="MAX";
    let bits:i64 = if a=="i8" || a=="u8" { 8 } else if a=="i16" || a=="u16" { 16 }
                   else if a=="i32" || a=="u32" { 32 } else { 64 };
    let uns = a.len()>0 && &a[0..1]=="u";
    if uns { if mx { if bits>=64 { return 0-1; } let mut v:i64=1; let mut i=0; while i<bits { v=v*2; i=i+1; } return v-1; } return 0; }
    if bits>=64 { if mx { return 9223372036854775807; } return 0-9223372036854775807-1; }
    let mut v:i64=1; let mut i=0; while i<bits-1 { v=v*2; i=i+1; }
    if mx { v-1 } else { 0-v } }
// A SLICE type `[T]` names no representation of its own: an array literal and a vector both reach it.
// In a PARAMETER or return position the value is a vector (`&v`), so the slice reads as `Vec$T`; in a
// `let`/`const` annotation the INITIALIZER's own shape is the truth, so the annotation is dropped.
pub fn sl_param(t:&str)->String{
    if t.len()>=5 && &t[0..5]=="slice" { let mut r=String::from("Vec"); if t.len()>5 { r.push_str(&t[5..]); } return r; }
    String::from(t) }
pub fn sl_const(t:&str)->String{ if t.len()>=5 && &t[0..5]=="slice" { return String::from("arr"); } String::from(t) }
pub fn sl_val(t:&str)->String{ if t.len()>=5 && &t[0..5]=="slice" { return String::from(""); } String::from(t) }
impl P {
    fn variant_owner(&self, n:&str)->String{ let mut i=0; while i<self.vnames.len(){ if self.vnames[i].as_str()==n { return String::from(self.vowner[i].as_str()); } i=i+1; } String::from("") }
    fn enames_has(&self, n:&str)->bool{ let mut i=0; while i<self.enames.len(){ if self.enames[i].as_str()==n { return true; } i=i+1; } false }
    fn mknown(&self, n:&str)->bool{ let mut i=0; while i<self.mods.len(){ if self.mods[i].as_str()==n { return true; } i=i+1; } false }
    // an item inside `mod M` is named `M$item`; references use the qualified path `M::item`
    fn qual(&self, n:&str)->String{ if self.mpfx.len()==0 { return String::from(n); } let mut r=String::from(self.mpfx.as_str()); r.push_str(n); r }
    fn sfields_of(&self, n:&str)->String{ let mut i=0; while i<self.snames.len(){ if self.snames[i].as_str()==n { if i<self.sfnames.len() { return String::from(self.sfnames[i].as_str()); } return String::from(""); } i=i+1; } String::from("") }
    // a bare type name inside a module means THIS module's item first: structs are registered under both
    // the bare and the qualified name, so asking bare first picks the unqualified one and the emitted name
    // stops matching the definition
    fn sres(&self, n:&str)->String{ let r=self.tyres(n); if self.sknown(&r) { return r; } String::from(n) }
    fn sknown_here(&self, n:&str)->bool{ let r=self.tyres(n); self.sknown(&r) }
    fn sknown(&self, n:&str)->bool{ let mut i=0; while i<self.snames.len(){ if self.snames[i].as_str()==n { return true; } i=i+1; } false }
    fn at_end(&self)->bool{ self.i+1 >= self.t.len() }
    fn cknown(&self, n:&str)->bool{ let mut i=0; while i<self.consts.len(){ if self.consts[i].as_str()==n { return true; } i=i+1; } false }
    fn expr_no_seq(&mut self)->E{ self.cmp_e() }
    // A type-level `>` may arrive glued to the next glyph, because the lexer munches maximally and is right to:
    // `Vec<i64>=` lexes as `Vec` `<` `i64` `>=`. rustc splits these in the PARSER, not the lexer (the same
    // glyphs stay operators in expression position -- probed), so the split belongs here too. It is local: take
    // one `>` and leave the remainder in place, which needs no depth counter and so handles any nesting.
    fn at_gt(&self)->bool{ if self.i >= self.t.len() { return false; } match &self.t[self.i].k { Tk::Gt=>true, Tk::Ge=>true, Tk::Shr=>true, _=>false } }
    fn eat_gt(&mut self)->bool{ if self.i >= self.t.len() { return false; }
        let k=self.t[self.i].k.clone();
        match k { Tk::Gt=>{ let _=self.adv(); true },
                  Tk::Ge=>{ self.t[self.i].k=Tk::Eq; self.t[self.i].lo=self.t[self.i].lo+1; true },
                  Tk::Shr=>{ self.t[self.i].k=Tk::Gt; self.t[self.i].lo=self.t[self.i].lo+1; true },
                  _=>false } }
    // NOTE for the next rung: the bare-name branch below still has its OWN copy of this reader. Pointing
    // it here was attempted TWICE -- once by extracting its body into a method, once by replacing its span
    // with a delegating call -- and both times the file broke (a 3MB binary, every gate program failing),
    // because that branch's text runs into the structure of the enclosing match. It needs a careful manual
    // edit, not a span replacement. One rule in two places is the defect class this very rung fixed, so it
    // is worth doing properly.
    // read a struct literal's body: named fields are collected and emitted in DECLARATION order,
    // positional ones are taken as written. `{` is the current token.
    fn slit_named(&mut self, s:&str, lo:u32, hi:u32)->E{
        let _=self.adv();
        let mut fv:Vec<E>=Vec::new();
        let named={ let mut r=false; if self.i+1 < self.t.len() { match self.t[self.i].k.clone() { Tk::Ident(_x)=>{ r=tk_is(&self.t[self.i+1].k, ":"); }, _=>{} } } r };
        if named {
            let mut ns:Vec<String>=Vec::new(); let mut vs:Vec<E>=Vec::new();
            loop { if self.is_tok("}") { let _=self.adv(); break; } if self.at_end() { break; }
                let fi=self.adv(); let fnm=match self.t[fi].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
                if self.is_tok(":") { let _=self.adv(); }
                let v=self.cmp_e(); ns.push(fnm); vs.push(v);
                if self.is_tok(",") { let _=self.adv(); } }
            let order=split_commas(&self.sfields_of(s));
            let mut oi=0;
            while oi<order.len(){
                let mut pick=0-1; let mut q=0;
                while q<ns.len(){ if ns[q].as_str()==order[oi].as_str() { pick=q as i32; q=ns.len(); } else { q=q+1; } }
                if pick>=0 { let pu=pick as usize; let mut taken=E::Int(0,lo,hi);
                    { let mut tmp:Vec<E>=Vec::new(); let mut r=vs.len();
                      while r>0 { r=r-1; match vs.pop() { Some(x)=>{ if r==pu { taken=x; } else { tmp.push(x); } }, None=>{} } }
                      loop { match tmp.pop() { Some(x)=>{ vs.push(x); }, None=>{ break; } } } }
                    fv.push(taken); ns.remove(pu); }
                oi=oi+1; } }
        else { loop { if self.is_tok("}") { let _=self.adv(); break; } if self.at_end() { break; }
                   let aprog=self.i; let v=self.cmp_e(); fv.push(v); if self.is_tok(",") { let _=self.adv(); } if self.noprog(aprog) { } } }
        let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
        E::StructLit(String::from(s), fv, lo, h2) }
    fn is_tok(&self, k:&str)->bool{ if self.i >= self.t.len() { return false; } tk_is(&self.t[self.i].k, k) }
    fn lo(&self)->u32{ self.t[self.i].lo }
    fn adv(&mut self)->usize{ let x=self.i; if self.i+1 < self.t.len() { self.i=self.i+1; } x }
    fn eat(&mut self, k:&str){ if self.is_tok(k) { let _=self.adv(); }
        else if self.perr==0 { self.perr=self.lo()+1; self.perrexp=String::from(k); self.perrgot=self.tok_text(); } }
    // a printable form of the current token, for diagnostics
    fn tok_text(&self)->String{
        if self.i>=self.t.len() { return String::from("<end of input>"); }
        let w=tok_word(&self.t[self.i]); if w.len()>0 { return w; }
        match self.t[self.i].k.clone() {
            Tk::Num(n)=>i64s(n), Tk::ChL(n)=>i64s(n), Tk::FloatL(x)=>String::from(x.as_str()), Tk::StrL(_x)=>String::from("<string>"),
            _=>{ let ks=["(",")","{","}","[","]",",",";",":","=","=>","::","$","+","-","*","/","<",">","==","!=","<=",">=","&","|","^","!","?","..","<<",">>",".","fn","let","if","else","while","for","in","match","impl","trait","struct","enum","mod","use","return","break","continue","loop","as","where","type","const","static","unsafe","dyn","mut","true","false"];
                 let mut i2=0; while i2<ks.len(){ if self.is_tok(ks[i2]) { return String::from(ks[i2]); } i2=i2+1; } String::from("<token>") } } }
    fn atom(&mut self)->E{
        // P-MC-PATH: a leading `crate::` names the crate root; one compilation unit means one root, so at
        // every path position it is a no-op prefix.
        if self.is_tok("crate") && self.peek_is(1,"::") { let _=self.adv(); let _=self.adv(); return self.atom(); }
        // `move |..| ..` is the same closure as `|..| ..` in our value model (captures are lifted into
        // leading parameters either way), so the keyword is skipped
        if self.i<self.t.len() && self.i+1<self.t.len() {
            let w=tok_word(&self.t[self.i]);
            if w.as_str()=="move" && (tk_is(&self.t[self.i+1].k,"|") || tk_is(&self.t[self.i+1].k,"||")) { let _=self.adv(); } }
        // a path may be written from the crate ROOT: `::std::vec::Vec::new()`. The leading `::` adds no
        // segment, so it is skipped.
        if self.is_tok("::") { let _=self.adv(); }
        let ix=self.adv(); let lo=self.t[ix].lo; let hi=self.t[ix].hi;
        match self.t[ix].k.clone() {
            Tk::StrL(v)=>E::StrLit(v, lo, hi),
            Tk::ForT=>{
                // the loop's synthetic names must be UNIQUE: nested `for`s sharing `$fi` made the outer
                // increment hit the inner binding's slot, so the outer loop never advanced (lowering keeps a
                // monotonically growing scope and resolves a name to its INNERMOST slot)
                let fsn=self.nfor; self.nfor=self.nfor+1;
                let mut nfi=String::from("$fi"); nfi.push_str(&crate::oast::u32s2(fsn));
                let mut nfe=String::from("$fe"); nfe.push_str(&crate::oast::u32s2(fsn));
                let mut nfc=String::from("$fc"); nfc.push_str(&crate::oast::u32s2(fsn));
                let mut tnames:Vec<String>=Vec::new();
                if self.is_tok("(") { let _=self.adv();
                    loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; }
                        if self.is_tok(",") || self.is_tok("mut") || self.is_word("ref") || self.is_tok("&") { let _=self.adv(); continue; }
                        let w=tok_word(&self.t[self.i]); if w.len()==0 { let _=self.adv(); continue; }
                        let _=self.adv(); tnames.push(w); } }
                let vn=if tnames.len()>0 { let mut t=String::from("$ftup"); t.push_str(&crate::oast::u32s2(fsn)); t }
                       else { let ni=self.adv(); match self.t[ni].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") } };
                if self.is_tok("in") { let _=self.adv(); }
                let mut mutref=self.is_tok("&") && self.peek_is(1,"mut");
                let src=self.or_e();
                // `v.iter_mut()` is `&mut v`: the loop writes the binding back after the body. The call
                // lowers to the IDENTITY, so the source does not need unwrapping -- which also keeps us
                // off a `*boxed` move, a shape semc miscompiles (it produced a compiler that crashed).
                { match &src { E::MethodCall(_r,m,a2,_l,_h)=>{ if m.as_str()=="iter_mut" && a2.len()==0 { mutref=true; } }, _=>{} } }
                let mut hi2=self.t[self.i].hi;
                if self.is_tok("..") { let _=self.adv(); let end=self.or_e(); self.eat("{"); let body=self.expr(); self.eat("}");
                    hi2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                    // for i in a..b  ==>  let $e = b; let $i = a; while $i < $e { let i = $i; body; $i = $i + 1 }
                    let iv=E::Var(String::from(nfi.as_str()), lo, hi2);
                    let inner=E::Let(String::from(vn.as_str()), Box::new(iv), Box::new(body), lo, hi2);
                    // the step lives in the CONDITION, so `continue` cannot skip it
                    let step=E::Assign(String::from(nfi.as_str()), Box::new(E::Bin(String::from("+"), Box::new(E::Var(String::from(nfi.as_str()), lo, hi2)), Box::new(E::Int(1, lo, hi2)), lo, hi2)), lo, hi2);
                    let test=E::Bin(String::from("<"), Box::new(E::Var(String::from(nfi.as_str()), lo, hi2)), Box::new(E::Var(String::from(nfe.as_str()), lo, hi2)), lo, hi2);
                    let cond=E::Seq(Box::new(step), Box::new(test), lo, hi2);
                    let wh=E::While(Box::new(cond), Box::new(inner), lo, hi2);
                    let start1=E::Bin(String::from("-"), Box::new(src), Box::new(E::Int(1, lo, hi2)), lo, hi2);
                    let l1=E::Let(String::from(nfi.as_str()), Box::new(start1), Box::new(wh), lo, hi2);
                    return E::Let(String::from(nfe.as_str()), Box::new(end), Box::new(l1), lo, hi2); }
                self.eat("{"); let body=self.expr(); self.eat("}");
                hi2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                let body=if tnames.len()>0 {
                        let mut acc=body; let mut q=tnames.len();
                        while q>0 { q=q-1;
                            let sv=E::Var(String::from(vn.as_str()), lo, hi2);
                            let get2=E::TupIdx(Box::new(sv), q as u32, lo, hi2);
                            if tnames[q].as_str()=="_" { acc=E::Seq(Box::new(get2), Box::new(acc), lo, hi2); }
                            else { acc=E::Let(String::from(tnames[q].as_str()), Box::new(get2), Box::new(acc), lo, hi2); } }
                        acc } else { body };
                // for x in v  ==>  let $fc = v; let $fi = 0; while $fi < vec_len($fc) { let x = vec_get($fc,$fi); body; $fi = $fi + 1 }
                let mut ga:Vec<E>=Vec::new(); ga.push(E::Var(String::from(nfc.as_str()), lo, hi2)); ga.push(E::Var(String::from(nfi.as_str()), lo, hi2));
                let get=E::Call(String::from("seq_get"), ga, lo, hi2);
                let step=E::Assign(String::from(nfi.as_str()), Box::new(E::Bin(String::from("+"), Box::new(E::Var(String::from(nfi.as_str()), lo, hi2)), Box::new(E::Int(1, lo, hi2)), lo, hi2)), lo, hi2);
                // by-MUTABLE-REFERENCE iteration writes the binding back after the body
                // the step lives in the CONDITION (below), so `continue` cannot skip it
                let inner=if mutref {
                        let mut sa:Vec<E>=Vec::new();
                        sa.push(E::Var(String::from(nfc.as_str()), lo, hi2));
                        sa.push(E::Var(String::from(nfi.as_str()), lo, hi2));
                        sa.push(E::Var(String::from(vn.as_str()), lo, hi2));
                        let back=E::Call(String::from("seq_set"), sa, lo, hi2);
                        E::Let(String::from(vn.as_str()), Box::new(get), Box::new(E::Seq(Box::new(body), Box::new(back), lo, hi2)), lo, hi2) }
                    else { E::Let(String::from(vn.as_str()), Box::new(get), Box::new(body), lo, hi2) };
                let mut la:Vec<E>=Vec::new(); la.push(E::Var(String::from(nfc.as_str()), lo, hi2));
                let test=E::Bin(String::from("<"), Box::new(E::Var(String::from(nfi.as_str()), lo, hi2)), Box::new(E::Call(String::from("seq_len"), la, lo, hi2)), lo, hi2);
                let cond=E::Seq(Box::new(step), Box::new(test), lo, hi2);
                let wh=E::While(Box::new(cond), Box::new(inner), lo, hi2);
                let l1=E::Let(String::from(nfi.as_str()), Box::new(E::Int(0-1, lo, hi2)), Box::new(wh), lo, hi2);
                E::Let(String::from(nfc.as_str()), Box::new(src), Box::new(l1), lo, hi2) },
            Tk::Bar=>{ let mut ps:Vec<String>=Vec::new(); let mut pts:Vec<String>=Vec::new();
                let mut destr:Vec<String>=Vec::new(); let mut destr_of=String::from("");
                loop { if self.is_tok("|") { let _=self.adv(); break; } if self.at_end() { break; }
                    if self.is_tok("(") { // a tuple PATTERN parameter: one value, destructured in the body
                        let _=self.adv();
                        let synth=String::from("$cp"); 
                        loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; }
                            let bi=self.adv(); match self.t[bi].k.clone() { Tk::Ident(x)=>destr.push(String::from(x.as_str())), _=>{} }
                            if self.is_tok(",") { let _=self.adv(); } }
                        destr_of=String::from(synth.as_str()); ps.push(synth);
                        if self.is_tok(":") { let _=self.adv(); let _=self.parse_tyname(); }
                        pts.push(String::from(""));
                        if self.is_tok(",") { let _=self.adv(); } continue; }
                    let pi=self.adv(); match self.t[pi].k.clone() { Tk::Ident(x)=>ps.push(String::from(x.as_str())), _=>{} }
                    if self.is_tok(":") { let _=self.adv(); let pt=sl_param(&self.parse_tyname()); pts.push(pt); } else { pts.push(String::from("")); }
                    if self.is_tok(",") { let _=self.adv(); } }
                // C-CLOSRET: a closure may declare its return type between the params and the body
                // (`|ch:char| -> i64 { .. }`). Probed from rustc: every arity accepts it, and the body is
                // then REQUIRED to be a block. Our closures are lifted with inferred returns, so the
                // declared type is read and discarded -- what matters is not derailing the parse.
                let mut cret=String::from("");
                if self.is_tok("-") && self.peek_is(1,">") { let _=self.adv(); let _=self.adv(); cret=self.parse_tyname(); }
                let _=&cret;
                let body0=if self.is_tok("{") { let _=self.adv(); let b=self.expr(); self.eat("}"); b } else { self.cmp_e() };
                // wrap the destructured names around the body: `|(a, b)| ..` binds a = $cp.0, b = $cp.1
                let body=if destr.len()>0 { let mut bb=body0; let mut q=destr.len();
                        while q>0 { q=q-1; let src=E::TupIdx(Box::new(E::Var(String::from(destr_of.as_str()), lo, hi)), q as u32, lo, hi);
                            bb=E::Let(String::from(destr[q].as_str()), Box::new(src), Box::new(bb), lo, hi); }
                        bb } else { body0 };
                let mut caps:Vec<String>=Vec::new(); crate::oast::collect_free(&body, &ps, &mut caps);
                let mut nm=String::from("$clos"); nm.push_str(&crate::oast::u32s2(self.clos.len() as u32));
                let mut allp:Vec<String>=Vec::new(); let mut allt:Vec<String>=Vec::new();
                let mut k=0; while k<caps.len(){ allp.push(String::from(caps[k].as_str())); allt.push(String::from("")); k=k+1; }
                let mut j=0; while j<ps.len(){ allp.push(String::from(ps[j].as_str()));
                    let dt=if j<pts.len() { String::from(pts[j].as_str()) } else { String::from("") };
                    allt.push(dt); j=j+1; }
                let io=body_has_io(&body);
                let nm=if io { let mut t=String::from(nm.as_str()); t.push_str("$io"); t } else { nm };
                self.clos.push(Fun{pref:Vec::new(), ncap:caps.len() as i32, gbounds:Vec::new(), dflt:0-1, name:String::from(nm.as_str()), params:allp, ptys:allt, ret:String::from(""), gparams:Vec::new(), owner:String::from(""), body:body});
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                E::ClosRef(nm, caps, lo, h2) },
            Tk::Amp=>{ let mut mt=false; if self.is_tok("mut") { let _=self.adv(); mt=true; }
                let x=self.unary(); let h=crate::oast::e_hi(&x); E::Ref(Box::new(x), mt, lo, h) },
            Tk::Star=>self.unary(),
            Tk::BreakT=>{
                // `break v` inside a `loop` assigns the loop's result before leaving
                let hasv = !self.is_tok(";") && !self.is_tok("}") && self.loopv.len()>0;
                if hasv { let lv=String::from(self.loopv[self.loopv.len()-1].as_str());
                    self.lvhit.push(String::from(lv.as_str()));
                    let v=self.cmp_e(); let h2=crate::oast::e_hi(&v);
                    let asg=E::Assign(lv, Box::new(v), lo, h2);
                    E::Seq(Box::new(asg), Box::new(E::BreakE(lo, h2)), lo, h2) }
                else { E::BreakE(lo, hi) } },
            Tk::ContT=>E::ContE(lo, hi),
            // `<Type as Trait>::ITEM` / `<Type>::ITEM` -- a qualified path. A prefix `<` is unambiguous
            // here because `<` is otherwise only an infix comparison.
            Tk::Lt=>{
                let ty=self.parse_tyname();
                if self.is_tok("as") { let _=self.adv(); let _tr=self.parse_tyname(); }
                if self.at_gt() { let _=self.eat_gt(); }
                if self.is_tok("::") { let _=self.adv(); }
                let ii=self.adv(); let item=match self.t[ii].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
                let mut nm=String::from(ty.as_str()); nm.push('$'); nm.push_str(&item);
                let mut ps:Vec<E>=Vec::new();
                if self.is_tok("(") { let _=self.adv(); loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; } let aprog=self.i; ps.push(self.cmp_e()); if self.is_tok(",") { let _=self.adv(); } if self.noprog(aprog) { }  } }
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                E::Call(nm, ps, lo, h2) },
            Tk::LoopT=>{
                // a `loop` is a VALUE: a result slot is bound around it, `break v` assigns the slot, and
                // the loop's own value is the slot afterwards
                let lsn=self.nfor; self.nfor=self.nfor+1;
                let mut lv=String::from("$lv"); lv.push_str(&crate::oast::u32s2(lsn));
                self.loopv.push(String::from(lv.as_str()));
                self.eat("{"); let bd=self.expr(); self.eat("}");
                let _=self.loopv.pop();
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                let wh=E::While(Box::new(E::Bool(true, lo, h2)), Box::new(bd), lo, h2);
                let used={ let mut r=false; let mut i2=0; while i2<self.lvhit.len(){ if self.lvhit[i2].as_str()==lv.as_str() { r=true; } i2=i2+1; } r };
                if !used { return wh; }
                let tail=E::Var(String::from(lv.as_str()), lo, h2);
                E::Let(lv, Box::new(E::Int(0, lo, h2)), Box::new(E::Seq(Box::new(wh), Box::new(tail), lo, h2)), lo, h2) },
            Tk::UnsafeT=>{ if self.is_tok("{") { let _=self.adv(); let b=self.expr(); self.eat("}"); b } else { self.atom() } },
            Tk::LBrace=>{ let b=self.expr(); self.eat("}"); b },
            Tk::Num(n)=>E::Int(n, lo, hi),
            // F-1 (probed from RC): the literal's DECIMAL TEXT is the durable form; the exact
            // decimal->binary64 conversion is strtod's, in the emitted program
            Tk::FloatL(t)=>{ let mut a:Vec<E>=Vec::new(); a.push(E::StrLit(String::from(t.as_str()), lo, hi));
                             E::Call(String::from("f64_parse"), a, lo, hi) },
            // P-NOM: a char literal is an integer that KNOWS it is a char, so `{}` can print the
            // character rather than its code point. The cast to a 64-bit name is identity in codegen.
            Tk::ChL(n)=>E::Cast(Box::new(E::Int(n, lo, hi)), String::from("char"), lo, hi),
            Tk::TrueT=>E::Bool(true, lo, hi),
            Tk::FalseT=>E::Bool(false, lo, hi),
            Tk::LBrack=>{ let mut xs:Vec<E>=Vec::new();
                loop { if self.is_tok("]") { let _=self.adv(); break; } if self.at_end() { break; }
                    let x=self.cmp_e();
                    if self.is_tok(";") { // the REPEAT form `[e; n]`
                        let _=self.adv();
                        let mut cnt:i64=0;
                        if self.i<self.t.len() { if ilit(&self.t[self.i])>0 { cnt=ilit_val(&self.t[self.i]); let _=self.adv(); } else { let _=self.cmp_e(); } }
                        loop { if self.is_tok("]") { let _=self.adv(); break; } if self.at_end() { break; } let _=self.adv(); }
                        let mut q=0; while q<cnt { xs.push(clone_simple(&x, lo, hi)); q=q+1; }
                        if cnt>0 { let _=xs.pop(); xs.push(x); }
                        break; }
                    xs.push(x); if self.is_tok(",") { let _=self.adv(); } } let h2=if self.i>0 { self.t[self.i-1].hi } else { hi }; E::ArrLit(xs, lo, h2) },
            Tk::Match=>{ let sc=self.cmp_e(); self.eat("{"); let mut arms:Vec<Arm>=Vec::new();
                // P-PAT: the scrutinee is bound ONCE to a synthetic name, so a structural arm pattern can
                // desugar into reads of it (a tuple/struct/slice pattern becomes bindings around the body,
                // a range pattern becomes a guard) without the pattern grammar reaching the lowering
                let msn=self.nfor; self.nfor=self.nfor+1;
                let mut msv=String::from("$ms"); msv.push_str(&crate::oast::u32s2(msn));
                loop { if self.is_tok("}") { let _=self.adv(); break; } if self.at_end() { break; }
                    // a pattern may be prefixed by `&` / `&mut` (matching through a reference); our value
                    // model has no distinct reference, so the prefix is skipped
                    loop { if self.is_tok("&") { let _=self.adv(); if self.is_tok("mut") { let _=self.adv(); } } else { break; } }
                    // a structural pattern: `(a, b)`, `[a, b]`, `P { x, y }`. Each position is a NAME
                    // (bound from a component read), a LITERAL (an equality test folded into the guard),
                    // or `..` (a rest, which relaxes the length test). A literal AFTER a rest cannot be
                    // addressed positionally, so such a pattern is refused rather than silently matched.
                    { let dk = if self.is_tok("(") { 1 } else if self.is_tok("[") { 2 }
                               else { let mut r=0;
                                      if self.i+1<self.t.len() {
                                          let w=tok_word(&self.t[self.i]);
                                          if w.len()>0 && self.sknown(&w) {
                                              if tk_is(&self.t[self.i+1].k,"{") { r=3; }
                                              else if tk_is(&self.t[self.i+1].k,"(") { r=4; } } }
                                      r };
                      if dk>0 {
                        if dk==3 || dk==4 { let _=self.adv(); }
                        let _=self.adv();
                        let mut ns:Vec<String>=Vec::new(); let mut kinds:Vec<i32>=Vec::new(); let mut lits:Vec<i64>=Vec::new();
                        let mut rest=false; let mut bad=false; let mut restat:i32=0-1;
                        loop { if self.is_tok(")") || self.is_tok("]") || self.is_tok("}") { let _=self.adv(); break; }
                               if self.at_end() { break; }
                               // a NESTED group -- `(Some(f), Some(b))`, `[Foo { .. }, _]` -- needs a tag
                               // test per position, which the flat position reader cannot express. Refuse
                               // it: mis-binding it silently answered the wrong arm.
                               if self.is_tok("(") || self.is_tok("[") || self.is_tok("{") {
                                   if self.perr==0 { self.perr=lo+1; self.perrexp=String::from("a pattern without a nested group"); self.perrgot=self.tok_text(); }
                                   let mut d3=0;
                                   loop { if self.at_end() { break; }
                                       if self.is_tok("(") || self.is_tok("[") || self.is_tok("{") { d3=d3+1; }
                                       else if self.is_tok(")") || self.is_tok("]") || self.is_tok("}") { d3=d3-1; if d3<=0 { let _=self.adv(); break; } }
                                       let _=self.adv(); }
                                   continue; }
                               if self.is_tok(",") || self.is_tok("mut") || self.is_word("ref") || self.is_tok("&") { let _=self.adv(); continue; }
                               if self.is_tok("..") { let _=self.adv(); rest=true; restat=kinds.len() as i32; continue; }
                               { let isn = ilit(&self.t[self.i])>0;
                                 if isn { let nv=ilit_val(&self.t[self.i]); let _=self.adv();
                                     ns.push(String::from("_")); kinds.push(1); lits.push(nv); continue; } }
                               let w=tok_word(&self.t[self.i]);
                               if w.len()==0 { let _=self.adv(); continue; }
                               let _=self.adv();
                               if self.is_tok(":") { let _=self.adv();
                                   { let isn2 = ilit(&self.t[self.i])>0;
                                     if isn2 { let nv=ilit_val(&self.t[self.i]); let _=self.adv();
                                         ns.push(w); kinds.push(1); lits.push(nv); continue; } }
                                   let w2=tok_word(&self.t[self.i]);
                                   if w2.len()>0 { let _=self.adv(); ns.push(w2); } else { ns.push(w); }
                                   kinds.push(0); lits.push(0); continue; }
                               ns.push(w); kinds.push(0); lits.push(0); }
                        let _=&bad;
                        let mut hg2=false; let mut gd2=E::Bool(true, lo, lo);
                        // positional literal tests, plus a LENGTH test for a slice pattern
                        { let mut q=0;
                          while q<kinds.len(){
                              if kinds[q]==1 {
                                  let sv=E::Var(String::from(msv.as_str()), lo, hi);
                                  let ixe = if restat>=0 && (q as i32)>=restat {
                                      let mut la2:Vec<E>=Vec::new(); la2.push(E::Var(String::from(msv.as_str()), lo, hi));
                                      let ln2=E::Call(String::from("seq_len"), la2, lo, hi);
                                      E::Bin(String::from("-"), Box::new(ln2), Box::new(E::Int((kinds.len()-q) as i64, lo, hi)), lo, hi) }
                                      else { E::Int(q as i64, lo, hi) };
                                  let get = if dk==1 { E::TupIdx(Box::new(sv), q as u32, lo, hi) }
                                            else if dk==2 { E::Index(Box::new(sv), Box::new(ixe), lo, hi) }
                                            else if dk==4 { E::Field(Box::new(sv), crate::oast::u32s2(q as u32), lo, hi) }
                                            else { E::Field(Box::new(sv), String::from(ns[q].as_str()), lo, hi) };
                                  let eqt=E::Bin(String::from("=="), Box::new(get), Box::new(E::Int(lits[q], lo, hi)), lo, hi);
                                  gd2=E::Bin(String::from("&&"), Box::new(gd2), Box::new(eqt), lo, hi); hg2=true; }
                              q=q+1; }
                          if dk==2 {
                              let mut la:Vec<E>=Vec::new(); la.push(E::Var(String::from(msv.as_str()), lo, hi));
                              let ln=E::Call(String::from("seq_len"), la, lo, hi);
                              let opl=if rest { String::from(">=") } else { String::from("==") };
                              let lt=E::Bin(opl, Box::new(ln), Box::new(E::Int(kinds.len() as i64, lo, hi)), lo, hi);
                              gd2=E::Bin(String::from("&&"), Box::new(gd2), Box::new(lt), lo, hi); hg2=true; } }
                        if self.is_tok("if") { let _=self.adv(); let g2=self.or_e();
                            gd2=E::Bin(String::from("&&"), Box::new(gd2), Box::new(g2), lo, hi); hg2=true; }
                        self.eat("=>"); let b0=self.cmp_e(); if self.is_tok(",") { let _=self.adv(); }
                        let hb=crate::oast::e_hi(&b0);
                        let mut acc=b0; let mut q=ns.len();
                        while q>0 { q=q-1;
                            if kinds[q]!=0 { continue; }
                            let nm2=String::from(ns[q].as_str());
                            let sv=E::Var(String::from(msv.as_str()), lo, hb);
                            let ixe2 = if restat>=0 && (q as i32)>=restat {
                                let mut la3:Vec<E>=Vec::new(); la3.push(E::Var(String::from(msv.as_str()), lo, hb));
                                let ln3=E::Call(String::from("seq_len"), la3, lo, hb);
                                E::Bin(String::from("-"), Box::new(ln3), Box::new(E::Int((kinds.len()-q) as i64, lo, hb)), lo, hb) }
                                else { E::Int(q as i64, lo, hb) };
                            let get = if dk==1 { E::TupIdx(Box::new(sv), q as u32, lo, hb) }
                                      else if dk==2 { E::Index(Box::new(sv), Box::new(ixe2), lo, hb) }
                                      else if dk==4 { E::Field(Box::new(sv), crate::oast::u32s2(q as u32), lo, hb) }
                                      else { E::Field(Box::new(sv), String::from(ns[q].as_str()), lo, hb) };
                            if nm2.as_str()=="_" { acc=E::Seq(Box::new(get), Box::new(acc), lo, hb); }
                            else { acc=E::Let(nm2, Box::new(get), Box::new(acc), lo, hb); } }
                        arms.push(Arm{slit:String::from(""), has_slit:false, lit:0, has_lit:false, has_guard:hg2, guard:gd2, variant:String::from("_"), binds:Vec::new(), body:acc, wild:true});
                        continue; } }
                    // a RANGE pattern `lo..=hi` / `lo..hi` becomes a guarded wildcard
                    { let isnum = if self.i<self.t.len() { if ilit(&self.t[self.i])>0 { true } else { match self.t[self.i].k.clone() { Tk::Ident(w)=>self.cknown(&w), _=>false } } } else { false };
                      let rng = isnum && self.i+1<self.t.len() && tk_is(&self.t[self.i+1].k,"..");
                      if rng {
                        // an endpoint is a literal OR a const name (`s..=e`), so both ends are EXPRESSIONS
                        let e0=self.pat_endpoint(lo, hi); let _=self.adv();  // endpoint, then `..`
                        let incl=if self.is_tok("=") { let _=self.adv(); true } else { false };
                        let e1=self.pat_endpoint(lo, hi);
                        let opc=if incl { String::from("<=") } else { String::from("<") };
                        let gl=E::Bin(String::from(">="), Box::new(E::Var(String::from(msv.as_str()), lo, hi)), Box::new(e0), lo, hi);
                        let gh=E::Bin(opc, Box::new(E::Var(String::from(msv.as_str()), lo, hi)), Box::new(e1), lo, hi);
                        let mut gd3=E::Bin(String::from("&&"), Box::new(gl), Box::new(gh), lo, hi);
                        if self.is_tok("if") { let _=self.adv(); let g2=self.or_e();
                            gd3=E::Bin(String::from("&&"), Box::new(gd3), Box::new(g2), lo, hi); }
                        self.eat("=>"); let b2=self.cmp_e(); if self.is_tok(",") { let _=self.adv(); }
                        arms.push(Arm{slit:String::from(""), has_slit:false, lit:0, has_lit:false, has_guard:true, guard:gd3, variant:String::from("_"), binds:Vec::new(), body:b2, wild:true});
                        continue; } }
                    // an OR pattern over literals (`1 | 2 => ..`) becomes a guarded wildcard: duplicating
                    // the body is impossible (our `E` has no Clone), and a disjunction of tests is exact
                    { let isn3 = if self.i<self.t.len() { ilit(&self.t[self.i])>0 } else { false };
                      if isn3 && self.i+1<self.t.len() && tk_is(&self.t[self.i+1].k,"|") {
                        let mut gd4=E::Bool(false, lo, lo);
                        loop { let isn4 = if self.i<self.t.len() { ilit(&self.t[self.i])>0 } else { false };
                               if !isn4 { break; }
                               let nv=ilit_val(&self.t[self.i]); let _=self.adv();
                               let eqt=E::Bin(String::from("=="), Box::new(E::Var(String::from(msv.as_str()), lo, hi)), Box::new(E::Int(nv, lo, hi)), lo, hi);
                               gd4=E::Bin(String::from("||"), Box::new(gd4), Box::new(eqt), lo, hi);
                               if self.is_tok("|") { let _=self.adv(); } else { break; } }
                        if self.is_tok("if") { let _=self.adv(); let g5=self.or_e();
                            gd4=E::Bin(String::from("&&"), Box::new(gd4), Box::new(g5), lo, hi); }
                        self.eat("=>"); let b5=self.cmp_e(); if self.is_tok(",") { let _=self.adv(); }
                        arms.push(Arm{slit:String::from(""), has_slit:false, lit:0, has_lit:false, has_guard:true, guard:gd4, variant:String::from("_"), binds:Vec::new(), body:b5, wild:true});
                        continue; } }
                    // `E::C { x, y } =>` -- a struct-variant PATTERN: the names bind in the variant's
                    // declaration order, so the payload slots line up with the construction above
                    { let w0=tok_word(&self.t[self.i]);
                      if w0.len()>0 {
                        let mut vnm=String::from(w0.as_str()); let mut look=self.i+1;
                        if look<self.t.len() && tk_is(&self.t[look].k,"::") { if look+1<self.t.len() { vnm=tok_word(&self.t[look+1]); look=look+2; } }
                        let vf=self.variant_fields(&vnm);
                        if vf.len()>0 && look<self.t.len() && tk_is(&self.t[look].k,"{") {
                            while self.i<look { let _=self.adv(); }
                            let _=self.adv();
                            let mut given:Vec<String>=Vec::new();
                            loop { if self.is_tok("}") { let _=self.adv(); break; } if self.at_end() { break; }
                                if self.is_tok(",") || self.is_tok("..") || self.is_tok("mut") || self.is_word("ref") { let _=self.adv(); continue; }
                                let w=tok_word(&self.t[self.i]); if w.len()==0 { let _=self.adv(); continue; }
                                let _=self.adv(); given.push(w); }
                            let order=split_commas(&vf);
                            let mut binds:Vec<String>=Vec::new();
                            { let mut oi=0; while oi<order.len(){
                                let mut hit=String::from("$ignore"); let mut q=0;
                                while q<given.len(){ if given[q].as_str()==order[oi].as_str() { hit=String::from(given[q].as_str()); q=given.len(); } else { q=q+1; } }
                                binds.push(hit); oi=oi+1; } }
                            let mut hg5=false; let mut gd5=E::Int(0, lo, lo);
                            if self.is_tok("if") { let _=self.adv(); gd5=self.or_e(); hg5=true; }
                            self.eat("=>"); let b5=self.cmp_e(); if self.is_tok(",") { let _=self.adv(); }
                            arms.push(Arm{slit:String::from(""), has_slit:false, lit:0, has_lit:false, has_guard:hg5, guard:gd5, variant:vnm, binds:binds, body:b5, wild:false});
                            continue; } } }
                    if self.is_word("ref") { let _=self.adv(); }
                    let pi=self.adv();
                    let mut plit:i64=0; let mut phas=false; let mut pslit=String::from(""); let mut pshas=false;
                    let (vn, wild) = match self.t[pi].k.clone() { Tk::Ident(x)=>{ if x.as_str()=="_" { (String::from("_"), true) } else { (String::from(x.as_str()), false) } },
                        Tk::Num(nv)=>{ plit=nv; phas=true; (String::from("_"), true) },
                        Tk::ChL(nv)=>{ plit=nv; phas=true; (String::from("_"), true) },
                        Tk::StrL(x)=>{ pslit=String::from(x.as_str()); pshas=true; (String::from("_"), true) },
                        Tk::TrueT=>{ plit=1; phas=true; (String::from("_"), true) },
                        Tk::FalseT=>{ plit=0; phas=true; (String::from("_"), true) },
                        _=>(String::from("_"), true) };
                    // optional `Enum::Variant`
                    let mut vname=vn; if self.is_tok("::") { let _=self.adv(); let v2=self.adv(); vname=match self.t[v2].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>vname }; }
                    let mut binds:Vec<String>=Vec::new();
                    if self.is_tok("(") { let _=self.adv();
                        loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; }
                            if self.is_tok("..") || self.is_word("ref") || self.is_tok("mut") || self.is_tok("&") { let _=self.adv(); continue; }
                            let bi=self.adv();
                            match self.t[bi].k.clone() { Tk::Ident(x)=>{ let nmp=String::from(x.as_str()); if nmp.as_str()=="_" { binds.push(String::from("$ignore")); } else { binds.push(nmp); } }, _=>{} }
                            if self.is_tok(",") { let _=self.adv(); } } }
                    let mut hg=false; let mut gd=E::Int(0, lo, lo);
                    if self.is_tok("if") { let _=self.adv(); gd=self.or_e(); hg=true; }
                    self.eat("=>"); let body=self.cmp_e(); if self.is_tok(",") { let _=self.adv(); }
                    let bindall = !wild && binds.len()==0 && !phas && !pshas
                                  && self.variant_owner(&vname).len()==0 && !self.cknown(&vname)
                                  && !self.sknown(&vname) && !self.enames_has(&vname);
                    if bindall {
                        let bodyb=E::Let(String::from(vname.as_str()), Box::new(E::Var(String::from(msv.as_str()), lo, lo)), Box::new(body), lo, lo);
                        arms.push(Arm{slit:String::from(""), has_slit:false, lit:0, has_lit:false, has_guard:hg, guard:gd, variant:String::from("_"), binds:Vec::new(), body:bodyb, wild:true}); }
                    else {
                    arms.push(Arm{slit:pslit, has_slit:pshas, lit:plit, has_lit:phas, has_guard:hg, guard:gd, variant:vname, binds:binds, body:body, wild:wild}); } }
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                let m=E::Match(Box::new(E::Var(String::from(msv.as_str()), lo, h2)), arms, lo, h2);
                E::Let(msv, Box::new(sc), Box::new(m), lo, h2) },
            // `assert!(c)` / `assert_eq!(a, b)` -> a real check with a PANIC (exit 101, as rustc does), so an
            // asserting run-pass test genuinely verifies behavior instead of being ignored
            Tk::Ident(s) if self.is_tok("!") && self.mac_index(&s)>=0 =>{
                // a USER macro: expand it in place and parse the expansion
                let mi=self.mac_index(&s) as usize;
                self.i=self.i-1;                       // back up onto the macro name
                if self.mac_expand(mi) { return self.cmp_e(); }
                let _=self.adv(); let _=self.adv();
                if self.perr==0 { self.perr=self.lo()+1; }
                E::Int(0, lo, hi) },
            Tk::Ident(s) if self.is_tok("!") && (s.as_str()=="format" || s.as_str()=="write" || s.as_str()=="writeln") =>{
                let _=self.adv(); self.eat("(");
                // `write!`/`writeln!` take a sink first; we ignore the sink and build the string
                if s.as_str()!="format" { let _=self.cmp_e(); if self.is_tok(",") { let _=self.adv(); } }
                let mut fmt=String::from(""); let mut args:Vec<E>=Vec::new();
                if !self.is_tok(")") {
                    let fi=self.adv(); match self.t[fi].k.clone() { Tk::StrL(x)=>{ fmt=String::from(x.as_str()); }, _=>{} }
                    loop { if self.is_tok(",") { let _=self.adv(); if self.is_tok(")") { break; } args.push(self.cmp_e()); } else { break; } } }
                if self.is_tok(")") { let _=self.adv(); }
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                let fb=fmt.as_bytes(); let mut parts:Vec<String>=Vec::new(); let mut cur=String::from(""); let mut k=0;
                while k<fb.len(){
                    if fb[k]==123 && k+1<fb.len() && fb[k+1]==125 { parts.push(String::from(cur.as_str())); cur=String::from(""); k=k+2; }
                    else if fb[k]==123 && k+3<fb.len() && fb[k+1]==58 && fb[k+2]==63 && fb[k+3]==125 { parts.push(String::from(cur.as_str())); cur=String::from(""); k=k+4; }
                    else { if fb[k]==123 && self.badfmt==0 { self.badfmt=lo+1; } cur.push_str(&fmt[k..(k+1)]); k=k+1; } }
                parts.push(cur);
                let mut argsr:Vec<E>=Vec::new();
                loop { match args.pop() { Some(a)=>{ argsr.push(a); }, None=>{ break; } } }
                let mut acc=E::StrLit(String::from(""), lo, h2);
                let mut pi2=0;
                while pi2<parts.len(){
                    if parts[pi2].len()>0 { let mut c1:Vec<E>=Vec::new(); c1.push(acc); c1.push(E::StrLit(String::from(parts[pi2].as_str()), lo, h2));
                        acc=E::Call(String::from("str_cat"), c1, lo, h2); }
                    match argsr.pop() { Some(a)=>{ let mut sa:Vec<E>=Vec::new(); sa.push(a);
                            let conv=E::Call(String::from("str_of"), sa, lo, h2);
                            let mut c2:Vec<E>=Vec::new(); c2.push(acc); c2.push(conv);
                            acc=E::Call(String::from("str_cat"), c2, lo, h2); }, None=>{} }
                    pi2=pi2+1; }
                return acc; },
            Tk::Ident(s) if self.is_tok("!") && s.as_str()=="vec" =>{
                // `vec![a, b, c]` == `{ let $v = vec_new(); $v.push(a); ..; $v }`
                let _=self.adv();
                let close=if self.is_tok("[") { String::from("]") } else { String::from(")") };
                let _=self.adv();
                let mut els:Vec<E>=Vec::new(); let mut rep=E::Int(0, lo, hi); let mut isrep=false;
                loop { if self.is_tok(close.as_str()) { let _=self.adv(); break; } if self.at_end() { break; }
                    let aprog=self.i; let e1=self.cmp_e();
                    if self.is_tok(";") { // `vec![x; n]` -- the REPEAT form
                        let _=self.adv(); rep=self.cmp_e(); isrep=true; els.push(e1);
                        loop { if self.is_tok(close.as_str()) { let _=self.adv(); break; } if self.at_end() { break; } let _=self.adv(); }
                        break; }
                    els.push(e1); if self.is_tok(",") { let _=self.adv(); } if self.noprog(aprog) { } }
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                let vty={ let mut t=String::from("Vec");
                          if els.len()>0 { let e0=lit_ty(&els[0]); if e0.len()>0 { t.push('$'); t.push_str(e0.as_str()); } }
                          t };
                let vn=String::from("$vecl");
                let mut tail=E::Var(String::from(vn.as_str()), lo, h2);
                if isrep {
                    // `let $v = vec_new(); for $i in 0..n { $v.push(x); } $v`
                    let mut ar:Vec<E>=Vec::new(); ar.push(E::Var(String::from(vn.as_str()), lo, h2));
                    match els.pop() { Some(x)=>{ ar.push(x); }, None=>{ ar.push(E::Int(0, lo, h2)); } }
                    let push=E::Call(String::from("vec_push"), ar, lo, h2);
                    let iv=String::from("$vri");
                    let step=E::Assign(String::from(iv.as_str()), Box::new(E::Bin(String::from("+"), Box::new(E::Var(String::from(iv.as_str()), lo, h2)), Box::new(E::Int(1, lo, h2)), lo, h2)), lo, h2);
                    let cond=E::Bin(String::from("<"), Box::new(E::Var(String::from(iv.as_str()), lo, h2)), Box::new(rep), lo, h2);
                    let wh=E::While(Box::new(cond), Box::new(E::Seq(Box::new(push), Box::new(step), lo, h2)), lo, h2);
                    let body=E::Let(iv, Box::new(E::Int(0, lo, h2)), Box::new(E::Seq(Box::new(wh), Box::new(tail), lo, h2)), lo, h2);
                    let mk2:Vec<E>=Vec::new();
                    return E::LetT(vn, vty, Box::new(E::Call(String::from("vec_new"), mk2, lo, h2)), Box::new(body), lo, h2); }
                let mut q=els.len();
                while q>0 { q=q-1; match els.pop() { Some(x)=>{ let mut ar:Vec<E>=Vec::new();
                            ar.push(E::Var(String::from(vn.as_str()), lo, h2)); ar.push(x);
                            let push=E::Call(String::from("vec_push"), ar, lo, h2);
                            tail=E::Seq(Box::new(push), Box::new(tail), lo, h2); }, None=>{} } }
                let mk:Vec<E>=Vec::new();
                return E::LetT(vn, vty, Box::new(E::Call(String::from("vec_new"), mk, lo, h2)), Box::new(tail), lo, h2); },
            Tk::Ident(s) if self.is_tok("!") && (s.as_str()=="cfg" || s.as_str()=="stringify" || s.as_str()=="line" || s.as_str()=="file" || s.as_str()=="column") =>{
                // `cfg!(..)` with no configuration flags is false; the others are placeholders
                let _=self.adv();
                if self.is_tok("(") { let mut d=0; loop { if self.at_end() { break; }
                        if self.is_tok("(") { d=d+1; } if self.is_tok(")") { d=d-1; let _=self.adv(); if d==0 { break; } continue; }
                        let _=self.adv(); } }
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                if s.as_str()=="cfg" { E::Bool(false, lo, h2) } else { E::Int(0, lo, h2) } },
            Tk::Ident(s) if self.is_tok("!") && (s.as_str()=="panic" || s.as_str()=="unreachable" || s.as_str()=="todo" || s.as_str()=="unimplemented") =>{
                let _=self.adv();
                if self.is_tok("(") { let mut d=0; loop { if self.at_end() { break; }
                        if self.is_tok("(") { d=d+1; } if self.is_tok(")") { d=d-1; let _=self.adv(); if d==0 { break; } continue; }
                        let _=self.adv(); } }
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                let pa:Vec<E>=Vec::new(); E::Call(String::from("panic_exit"), pa, lo, h2) },
            Tk::Ident(s) if self.is_tok("!") && (s.as_str()=="assert" || s.as_str()=="assert_eq" || s.as_str()=="assert_ne") =>{
                let kind=String::from(s.as_str());
                let _=self.adv(); self.eat("(");
                let a=self.cmp_e();
                let mut b=E::Int(0, lo, lo); let mut two=false;
                if self.is_tok(",") { let _=self.adv(); if !self.is_tok(")") { b=self.cmp_e(); two=true; } }
                loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; } let _=self.adv(); }
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                let cond = if two { let op=if kind.as_str()=="assert_ne" { String::from("==") } else { String::from("!=") };
                                    E::Bin(op, Box::new(a), Box::new(b), lo, h2) }
                           else { E::Not(Box::new(a), lo, h2) };
                let pa:Vec<E>=Vec::new();
                let thn=E::Call(String::from("panic_exit"), pa, lo, h2);
                return E::If(Box::new(cond), Box::new(thn), Box::new(E::Int(0, lo, h2)), lo, h2); },
            Tk::Ident(s) if self.is_tok("!") && (s.as_str()=="println" || s.as_str()=="print") =>{
                let nl = s.as_str()=="println";
                let _=self.adv(); self.eat("(");
                let mut fmt=String::from(""); let mut args:Vec<E>=Vec::new();
                if !self.is_tok(")") {
                    let fi=self.adv(); match self.t[fi].k.clone() { Tk::StrL(x)=>{ fmt=String::from(x.as_str()); }, _=>{} }
                    loop { if self.is_tok(",") { let _=self.adv(); if self.is_tok(")") { break; } args.push(self.cmp_e()); } else { break; } } }
                if self.is_tok(")") { let _=self.adv(); }
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                // split the format string on `{}`
                let fb=fmt.as_bytes(); let mut parts:Vec<String>=Vec::new(); let mut cur=String::from(""); let mut k=0; let mut dbg_at:Vec<usize>=Vec::new();
                while k<fb.len(){
                    if fb[k]==123 && k+1<fb.len() && fb[k+1]==125 { parts.push(String::from(cur.as_str())); cur=String::from(""); k=k+2; }
                    else if fb[k]==123 && k+2<fb.len() && fb[k+1]==58 && fb[k+2]==63 && k+3<fb.len() && fb[k+3]==125 {
                        // `{:?}` -- for an int or a str this is the same output as `{}`; a user type would need
                        // a derived Debug, which print_any reports as a plain value
                        parts.push(String::from(cur.as_str())); cur=String::from(""); k=k+4; dbg_at.push(parts.len()-1); }
                    else { if fb[k]==123 && self.badfmt==0 { self.badfmt=lo+1; } cur.push_str(&fmt[k..(k+1)]); k=k+1; } }
                parts.push(cur);
                // the arguments are CONSUMED in order (E has no Clone): reverse once, then pop
                let mut argsr:Vec<E>=Vec::new();
                loop { match args.pop() { Some(a)=>{ argsr.push(a); }, None=>{ break; } } }
                // F-ARGS: evaluate the non-place arguments FIRST, into temporaries, and leave the places
                // to be read when they are printed -- which is where rustc reads them
                let mut pre:Vec<E>=Vec::new(); let mut prenames:Vec<String>=Vec::new();
                { let mut argsr2:Vec<E>=Vec::new();
                  let mut idx=0;
                  loop { match argsr.pop() { Some(a)=>{
                        if is_place_expr(&a) { argsr2.push(a); }
                        else {
                          // a TUPLE LITERAL keeps its shape -- the Debug printer reads its element types
                          // from it -- so its ELEMENTS are what get bound, not the tuple
                          let istup=match &a { E::TupLit(p2,_l2,_h2)=>p2.len()>0, _=>false };
                          if istup {
                            match a { E::TupLit(mut p2,l2,h3)=>{
                                let mut np2:Vec<E>=Vec::new(); let mut rev:Vec<E>=Vec::new();
                                loop { match p2.pop() { Some(x)=>{ rev.push(x); }, None=>{ break; } } }
                                loop { match rev.pop() { Some(x)=>{
                                      if is_place_expr(&x) { np2.push(x); }
                                      else { let mut nm=String::from("$pa"); nm.push_str(&crate::oast::u32s2(self.nfor)); nm.push('_'); nm.push_str(&crate::oast::u32s2(idx));
                                             np2.push(E::Var(String::from(nm.as_str()), l2, h3));
                                             prenames.push(String::from(nm.as_str())); pre.push(x); idx=idx+1; } }, None=>{ break; } } }
                                argsr2.push(E::TupLit(np2, l2, h3)); }, _=>{} } }
                          else {
                            let mut nm=String::from("$pa"); nm.push_str(&crate::oast::u32s2(self.nfor)); nm.push('_'); nm.push_str(&crate::oast::u32s2(idx));
                            argsr2.push(E::Var(String::from(nm.as_str()), lo, h2));
                            prenames.push(String::from(nm.as_str())); pre.push(a); } }
                        idx=idx+1; }, None=>{ break; } } }
                  self.nfor=self.nfor+1;
                  // argsr is popped from the END below, so restore that order
                  loop { match argsr2.pop() { Some(a)=>{ argsr.push(a); }, None=>{ break; } } } }
                let mut seq:Vec<E>=Vec::new();
                let mut pi2=0;
                while pi2<parts.len(){
                    if parts[pi2].len()>0 { let mut a1:Vec<E>=Vec::new(); a1.push(E::StrLit(String::from(parts[pi2].as_str()), lo, h2)); seq.push(E::Call(String::from("print_raw"), a1, lo, h2)); }
                    match argsr.pop() { Some(a)=>{ let mut a2:Vec<E>=Vec::new(); a2.push(a);
                            let isdbg={ let mut r=false; let mut z=0; while z<dbg_at.len(){ if dbg_at[z]==pi2 { r=true; } z=z+1; } r };
                            let prim=if isdbg { String::from("print_dbg") } else { String::from("print_any") };
                            seq.push(E::Call(prim, a2, lo, h2)); }, None=>{} }
                    pi2=pi2+1; }
                if nl { let a3:Vec<E>=Vec::new(); seq.push(E::Call(String::from("print_nl"), a3, lo, h2)); }
                let mut out=E::Int(0, lo, h2); let mut q=seq.len();
                while q>0 { q=q-1; match seq.pop() { Some(x)=>{ out=E::Seq(Box::new(x), Box::new(out), lo, h2); }, None=>{} } }
                // wrap the temporaries around the prints, innermost LAST so they evaluate left to right
                { let mut w=pre.len();
                  while w>0 { w=w-1;
                      match pre.pop() { Some(v)=>{ let nm=String::from(prenames[w].as_str());
                            out=E::Let(nm, Box::new(v), Box::new(out), lo, h2); }, None=>{} } } }
                return out; },
            Tk::Ident(s) if self.sknown_here(&s) && self.sfields_of(&self.sres(&s)).len()==0 && !self.is_tok("::") && !self.is_tok("(") && !self.is_tok("{") =>{
                // a unit struct used as a value
                let fv:Vec<E>=Vec::new(); E::StructLit(self.sres(&s), fv, lo, hi) },
            // P-MC-VARIANT (REFUTED attempt, kept as a comment so the next rung does not retry it): adding
            // `&& !self.sknown_here(&s)` here took the composed unit from 3 errors to 94. sknown_here
            // consults GLOBS, and in the composed unit every module globs seventeen others, so the guard
            // fires for any variant whose name matches any struct anywhere. The real fix is to scope the
            // VARIANT table by module (register variants under the declaring module prefix) rather than to
            // add a preference here.
            Tk::Ident(s) if self.variant_owner(&s).len()>0 && !self.is_tok("::") =>{
                // a bare VARIANT constructor: `Some(x)` / `None`, the way Rust's prelude exposes them
                let own=self.variant_owner(&s);
                let mut ps:Vec<E>=Vec::new();
                if self.is_tok("(") { let _=self.adv(); loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; } let aprog=self.i; ps.push(self.cmp_e()); if self.is_tok(",") { let _=self.adv(); } if self.noprog(aprog) { }  } }
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                E::EnumLit(own, String::from(s.as_str()), ps, lo, h2) },
            // a read of a MUTABLE STATIC loads the global
            Tk::Ident(s) if self.is_smut(&s) && !self.is_tok("::") && !self.is_tok("(") =>{
                let mut gn=String::from("$gload$"); gn.push_str(s.as_str());
                let a:Vec<E>=Vec::new(); E::Call(gn, a, lo, hi) },
            Tk::Ident(s) if self.use_alias(&s).len()>0 && !self.is_tok("::") =>{
                // a name a `use` imported: `use m::fifteen;` then `fifteen()` is `m$fifteen()`
                let full=self.use_alias(&s);
                let mut ps:Vec<E>=Vec::new();
                if self.is_tok("(") { let _=self.adv();
                    loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; }
                        let aprog=self.i; ps.push(self.cmp_e()); if self.is_tok(",") { let _=self.adv(); } if self.noprog(aprog) { } } }
                let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                if self.enames_has(&full) { E::Call(full, ps, lo, h2) } else { E::Call(full, ps, lo, h2) } },
            Tk::Ident(s)=>{
                // a const name used as a VALUE. No `{` guard: `for i in 0..L {` legitimately has a brace after the
                // name, and a struct literal is recognised by its own name table anyway
                if self.cknown(&s) && !self.is_tok("(") && !self.is_tok("::") { let ps:Vec<E>=Vec::new(); return E::Call(String::from(s.as_str()), ps, lo, hi); }
                if self.is_tok("::") {
                    // P-PATH: collect the WHOLE path, then resolve it once (the old code matched
                    // `Ident::Ident` as a shape, so `a::b::c` and `Type::assoc::<T>` derailed)
                    let mut segs:Vec<String>=Vec::new(); segs.push(String::from(s.as_str()));
                    self.read_path_tail(&mut segs);
                    let mut ps:Vec<E>=Vec::new();
                    if self.is_tok("(") { let _=self.adv();
                        loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; }
                            let aprog=self.i; ps.push(self.cmp_e()); if self.is_tok(",") { let _=self.adv(); } if self.noprog(aprog) { } } }
                    let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                    if segs.len()==1 {
                        // a pure TURBOFISH (`None::<String>`): the path is one segment, so this is the
                        // bare-variant/const/ident case the arms above would have taken without the `::`
                        if self.variant_owner(&s).len()>0 { return E::EnumLit(self.variant_owner(&s), String::from(s.as_str()), ps, lo, h2); }
                        return E::Call(String::from(segs[0].as_str()), ps, lo, h2); }
                    let last=String::from(segs[segs.len()-1].as_str());
                    // a smart-pointer constructor is identity in our value model
                    if segs.len()==2 && (segs[0].as_str()=="Box" || segs[0].as_str()=="Rc" || segs[0].as_str()=="Arc")
                       && last.as_str()=="new" && ps.len()==1 && !self.sknown(&segs[0]) {
                        match ps.pop() { Some(x)=>{ return x; }, None=>{} } }
                    // `E::C { x: 2 }` -- a STRUCT VARIANT construction: order the named values by the
                    // variant's own declaration, the way a struct literal already does
                    { let last2=String::from(segs[segs.len()-1].as_str());
                      let vf=self.variant_fields(&last2);
                      if vf.len()>0 && ps.len()==0 && self.is_tok("{") && self.variant_owner(&last2).len()>0 {
                          let _=self.adv();
                          let mut ns:Vec<String>=Vec::new(); let mut vs2:Vec<E>=Vec::new();
                          loop { if self.is_tok("}") { let _=self.adv(); break; } if self.at_end() { break; }
                              if self.is_tok(",") { let _=self.adv(); continue; }
                              let fi3=self.adv(); let fn3=match self.t[fi3].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
                              if self.is_tok(":") { let _=self.adv(); }
                              let v2=self.cmp_e(); ns.push(fn3); vs2.push(v2); }
                          let order=split_commas(&vf);
                          let mut pay:Vec<E>=Vec::new();
                          let mut oi=0;
                          while oi<order.len(){
                              let mut pick=0-1; let mut q=0;
                              while q<ns.len(){ if ns[q].as_str()==order[oi].as_str() { pick=q as i32; q=ns.len(); } else { q=q+1; } }
                              if pick>=0 { let pu=pick as usize; let mut taken=E::Int(0,lo,h2);
                                  { let mut tmp:Vec<E>=Vec::new(); let mut r=vs2.len();
                                    while r>0 { r=r-1; match vs2.pop() { Some(x)=>{ if r==pu { taken=x; } else { tmp.push(x); } }, None=>{} } }
                                    loop { match tmp.pop() { Some(x)=>{ vs2.push(x); }, None=>{ break; } } } }
                                  pay.push(taken); ns.remove(pu); }
                              else { pay.push(E::Int(0, lo, h2)); }
                              oi=oi+1; }
                          let h4=if self.i>0 { self.t[self.i-1].hi } else { h2 };
                          return E::EnumLit(self.variant_owner(&last2), last2, pay, lo, h4); } }
                    // an ENUM variant, possibly reached through a module path
                    { let mut en=String::from(segs[0].as_str());
                      let mut q=1; while q+1<segs.len(){ en.push('$'); en.push_str(segs[q].as_str()); q=q+1; }
                      { let er=self.tyres(&en); if self.enames_has(&er) { return E::EnumLit(er, last, ps, lo, h2); } } }
                    // a path rooted in the standard library is TRANSPARENT down to its last two
                    // segments: `::std::vec::Vec::new` is `Vec::new`, which P-STD then resolves. A user
                    // module of the same name wins, exactly as with the Box/Rc transparency.
                    { let r=String::from(segs[0].as_str());
                      if (r.as_str()=="std" || r.as_str()=="core" || r.as_str()=="alloc") && !self.mod_known(&r) {
                          while segs.len()>2 { segs.remove(0); } } }
                    // P-STD: a small std surface that is ALREADY expressible in our value model -- a
                    // vector constructor, an empty string, and the integer bounds. Anything whose
                    // meaning we cannot produce stays unresolved and is reported, not guessed.
                    { let sp=std_path(&segs, self.sknown(&segs[0]) || self.enames_has(&segs[0]));
                      if sp.len()>0 {
                          if sp.as_str()=="$int" { return E::Int(std_int_bound(&segs), lo, h2); }
                          if sp.as_str()=="$emptystr" { return E::StrLit(String::from(""), lo, h2); }
                          if sp.as_str()=="$id1" { if ps.len()==1 { match ps.pop() { Some(x)=>{ return x; }, None=>{} } } }
                          return E::Call(sp, ps, lo, h2); } }
                    let joined=self.resolve_path(&segs);
                    // `M::S { .. }` -- a struct literal through a path
                    if ps.len()==0 && self.is_tok("{") && self.sknown(&joined) { return self.slit_named(&joined, lo, hi); }
                    return E::Call(joined, ps, lo, h2); }
                if self.is_tok("{") && self.sknown_here(&s) {
                    let s=self.sres(&s);
 let _=self.adv(); let mut fv:Vec<E>=Vec::new();
                    // NAMED fields (`Pt { x: 1, y: 2 }`) are collected by name, then emitted in DECLARATION
                    // order; positional literals (our own dialect) are taken as written
                    let named = { let mut r=false; if self.i+1 < self.t.len() { match self.t[self.i].k.clone() { Tk::Ident(_x)=>{ r=tk_is(&self.t[self.i+1].k, ":"); }, _=>{} } } r };
                    if named { let mut ns:Vec<String>=Vec::new(); let mut vs:Vec<E>=Vec::new();
                        loop { if self.is_tok("}") { let _=self.adv(); break; } if self.at_end() { break; }
                            let fi3=self.adv(); let fn3=match self.t[fi3].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
                            if self.is_tok(":") { let _=self.adv(); }
                            let v=self.cmp_e(); ns.push(fn3); vs.push(v);
                            if self.is_tok(",") { let _=self.adv(); } }
                        let order=split_commas(&self.sfields_of(&s));
                        let mut oi=0;
                        while oi<order.len(){
                            let mut pick=0-1; let mut q=0;
                            while q<ns.len(){ if ns[q].as_str()==order[oi].as_str() { pick=q as i32; q=ns.len(); } else { q=q+1; } }
                            if pick>=0 { let pu=pick as usize; let mut taken=E::Int(0,lo,hi);
                                { let mut tmp:Vec<E>=Vec::new(); let mut r=vs.len();
                                  while r>0 { r=r-1; match vs.pop() { Some(x)=>{ if r==pu { taken=x; } else { tmp.push(x); } }, None=>{} } }
                                  loop { match tmp.pop() { Some(x)=>{ vs.push(x); }, None=>{ break; } } } }
                                fv.push(taken); ns.remove(pu); }
                            oi=oi+1; } }
                    else { loop { if self.is_tok("}") { let _=self.adv(); break; } if self.at_end() { break; } let aprog=self.i; let v=self.cmp_e(); fv.push(v); if self.is_tok(",") { let _=self.adv(); } if self.noprog(aprog) { }  } } let h2=if self.i>0 { self.t[self.i-1].hi } else { hi }; E::StructLit(String::from(s.as_str()), fv, lo, h2) } else if self.is_tok("=") { let _=self.adv(); let v=self.cmp_e(); let h2=crate::oast::e_hi(&v); E::Assign(String::from(s.as_str()), Box::new(v), lo, h2) } else if self.is_tok("(") { let _=self.adv(); let mut args:Vec<E>=Vec::new(); loop { if self.is_tok(")") { let _=self.adv(); break; } let a=self.cmp_e(); args.push(a); if self.is_tok(",") { let _=self.adv(); } } let h2=if self.i>0 { self.t[self.i-1].hi } else { hi }; E::Call(String::from(s.as_str()), args, lo, h2) } else { E::Var(String::from(s.as_str()), lo, hi) } },
            Tk::LParen=>{
                if self.is_tok(")") { let _=self.adv(); let h2=if self.i>0 { self.t[self.i-1].hi } else { hi };
                    let z:Vec<E>=Vec::new(); return E::TupLit(z, lo, h2); }
                let e=self.cmp_e();
                if self.is_tok(",") { let mut ps:Vec<E>=Vec::new(); ps.push(e);
                    loop { if self.is_tok(",") { let _=self.adv(); } if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; } ps.push(self.cmp_e()); }
                    let b=if self.i>0 { self.t[self.i-1].hi } else { lo }; E::TupLit(ps, lo, b) }
                else if self.is_tok("..") { let _=self.adv();
                    if self.is_tok("=") { let _=self.adv(); }
                    if self.is_tok(")") { let _=self.adv();
                        if self.perr==0 { self.perr=lo+1; self.perrexp=String::from("a bounded range (an endless one cannot be materialised)"); self.perrgot=self.tok_text(); }
                        return e; }
                    let hi2=self.cmp_e(); self.eat(")");
                    let h3=if self.i>0 { self.t[self.i-1].hi } else { hi };
                    let mut ra:Vec<E>=Vec::new(); ra.push(e); ra.push(hi2);
                    E::Call(String::from("range2"), ra, lo, h3) }
                else { self.eat(")"); e } },
            Tk::If=>{
                // `if let V(binds) = e { body } else { alt }` desugars to a two-arm match -- the variant arm
                // and a wildcard -- which the if-chain lowering already handles (test + payload binds)
                if self.is_tok("let") { let _=self.adv();
                    let pi2=self.adv(); let mut pv=match self.t[pi2].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
                    if self.is_tok("::") { let _=self.adv(); let p2=self.adv(); pv=match self.t[p2].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>pv }; }
                    let mut pb:Vec<String>=Vec::new();
                    if self.is_tok("(") { let _=self.adv(); loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; } let bi=self.adv(); match self.t[bi].k.clone() { Tk::Ident(x)=>pb.push(String::from(x.as_str())), _=>{} } if self.is_tok(",") { let _=self.adv(); } } }
                    self.eat("="); let sc=self.or_e();
                    self.eat("{"); let th=self.expr(); self.eat("}");
                    let el = if self.is_tok("else") { let _=self.adv(); self.eat("{"); let e2=self.expr(); self.eat("}"); e2 } else { let zh=if self.i>0 { self.t[self.i-1].hi } else { hi }; E::Int(0, zh, zh) };
                    let h3=if self.i>0 { self.t[self.i-1].hi } else { hi };
                    let mut arms:Vec<Arm>=Vec::new();
                    arms.push(Arm{slit:String::from(""), has_slit:false, lit:0, has_lit:false, has_guard:false, guard:E::Int(0,lo,lo), variant:pv, binds:pb, body:th, wild:false});
                    arms.push(Arm{slit:String::from(""), has_slit:false, lit:0, has_lit:false, has_guard:false, guard:E::Int(0,lo,lo), variant:String::from("_"), binds:Vec::new(), body:el, wild:true});
                    return E::Match(Box::new(sc), arms, lo, h3); }
                let c=self.cmp_e(); self.eat("{"); let th=self.expr(); self.eat("}"); let el = if self.is_tok("else") { let _=self.adv(); if self.is_tok("if") { self.atom() } else { self.eat("{"); let e2=self.expr(); self.eat("}"); e2 } } else { let zh=if self.i>0 { self.t[self.i-1].hi } else { hi }; E::Int(0, zh, zh) }; let h=crate::oast::e_hi(&el); E::If(Box::new(c),Box::new(th),Box::new(el), lo, h) },
            Tk::While=>{
                // `while let V(b) = e { body }` is `loop { match e { V(b) => body, _ => break } }` -- the
                // scrutinee is re-evaluated each iteration, which is what an iterator loop needs
                if self.is_tok("let") { let _=self.adv();
                    let pi2=self.adv(); let mut pv=match self.t[pi2].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
                    if self.is_tok("::") { let _=self.adv(); let p2=self.adv(); pv=match self.t[p2].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>pv }; }
                    let mut pb:Vec<String>=Vec::new();
                    if self.is_tok("(") { let _=self.adv();
                        loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; }
                            if self.is_tok("..") || self.is_word("ref") || self.is_tok("mut") || self.is_tok("&") { let _=self.adv(); continue; }
                            let bi=self.adv(); match self.t[bi].k.clone() { Tk::Ident(x)=>pb.push(String::from(x.as_str())), _=>{} }
                            if self.is_tok(",") { let _=self.adv(); } } }
                    self.eat("="); let sc=self.or_e();
                    self.eat("{"); let bd=self.expr(); self.eat("}");
                    let h4=if self.i>0 { self.t[self.i-1].hi } else { hi };
                    let mut arms:Vec<Arm>=Vec::new();
                    arms.push(Arm{slit:String::from(""), has_slit:false, lit:0, has_lit:false, has_guard:false, guard:E::Int(0,lo,lo), variant:pv, binds:pb, body:bd, wild:false});
                    arms.push(Arm{slit:String::from(""), has_slit:false, lit:0, has_lit:false, has_guard:false, guard:E::Int(0,lo,lo), variant:String::from("_"), binds:Vec::new(), body:E::BreakE(lo,h4), wild:true});
                    let m=E::Match(Box::new(sc), arms, lo, h4);
                    return E::While(Box::new(E::Bool(true, lo, lo)), Box::new(m), lo, h4); }
                let c=self.cmp_e(); self.eat("{"); let bd=self.expr(); self.eat("}"); let h2=if self.i>0 { self.t[self.i-1].hi } else { hi }; E::While(Box::new(c), Box::new(bd), lo, h2) },
            Tk::Let=>{ if self.is_tok("mut") { let _=self.adv(); }
                // P-PAT: a DESTRUCTURING `let` -- `let (a, b) = e`, `let [a, b] = e`, `let P { x, y } = e`
                // -- desugars to one binding for the scrutinee plus one binding per name, each reading its
                // own component. The pattern is a shape the `let` reader owns; nothing below it changes.
                { let dk = if self.is_tok("(") { 1 } else if self.is_tok("[") { 2 }
                           else { let mut r=0;
                                  if self.i+1<self.t.len() {
                                      let w=tok_word(&self.t[self.i]);
                                      if w.len()>0 && self.sknown(&w) {
                                          if tk_is(&self.t[self.i+1].k,"{") { r=3; }
                                          else if tk_is(&self.t[self.i+1].k,"(") { r=4; } } }
                                  r };
                  if dk>0 {
                    let mut sn=String::from("");
                    if dk==3 || dk==4 { sn=tok_word(&self.t[self.i]); let _=self.adv(); }
                    let _=self.adv();                                     // `(` / `[` / `{`
                    let mut ns:Vec<String>=Vec::new();
                    loop { if self.is_tok(")") || self.is_tok("]") || self.is_tok("}") { let _=self.adv(); break; }
                           if self.at_end() { break; }
                           if self.is_tok(",") || self.is_tok("mut") || self.is_word("ref") || self.is_tok("..") { let _=self.adv(); continue; }
                           let w=tok_word(&self.t[self.i]);
                           if w.len()==0 { let _=self.adv(); continue; }
                           let _=self.adv();
                           // `P { x: a }` renames the field; the BINDING is the name after the colon
                           if self.is_tok(":") { let _=self.adv(); let w2=tok_word(&self.t[self.i]);
                               if w2.len()>0 { let _=self.adv(); ns.push(w2); } else { ns.push(w); }
                               continue; }
                           ns.push(w); }
                    if self.is_tok(":") { let _=self.adv(); let _=self.parse_tyname(); }
                    self.eat("="); let v=self.cmp_e(); self.eat(";");
                    let body=self.expr(); let h=crate::oast::e_hi(&body);
                    let dsn=self.nfor; self.nfor=self.nfor+1;
                    let mut dn=String::from("$dp"); dn.push_str(&crate::oast::u32s2(dsn));
                    // innermost-out, so each component reads the scrutinee binding
                    let mut acc=body; let mut q=ns.len();
                    while q>0 { q=q-1;
                        let nm2=String::from(ns[q].as_str());
                        let sv=E::Var(String::from(dn.as_str()), lo, h);
                        let get = if dk==1 { E::TupIdx(Box::new(sv), q as u32, lo, h) }
                                  else if dk==2 { E::Index(Box::new(sv), Box::new(E::Int(q as i64, lo, h)), lo, h) }
                                  else if dk==4 { E::Field(Box::new(sv), crate::oast::u32s2(q as u32), lo, h) }
                                  else { E::Field(Box::new(sv), String::from(ns[q].as_str()), lo, h) };
                        if nm2.as_str()=="_" { acc=E::Seq(Box::new(get), Box::new(acc), lo, h); }
                        else { acc=E::Let(nm2, Box::new(get), Box::new(acc), lo, h); } }
                    let _=&sn;
                    return E::Let(dn, Box::new(v), Box::new(acc), lo, h); } }
                let ni=self.adv(); let nm=match self.t[ni].k.clone() { Tk::Ident(s)=>String::from(s.as_str()), _=>String::from("_") };
                let mut ann=String::from("");
                if self.is_tok(":") { let _=self.adv(); ann=sl_val(&self.parse_tyname()); }
                // `let x: T;` -- a declaration with no initializer (Rust allows it before first assignment)
                if self.is_tok(";") { let _=self.adv(); let b=self.expr(); let h=crate::oast::e_hi(&b);
                    return E::Let(nm, Box::new(E::Int(0, lo, lo)), Box::new(b), lo, h); }
                self.eat("="); let v=self.cmp_e();
                if ann.len()>0 { self.eat(";"); let b=self.expr(); let h=crate::oast::e_hi(&b); return E::LetT(nm, ann, Box::new(v), Box::new(b), lo, h); } self.eat(";"); let b=self.expr(); let h=crate::oast::e_hi(&b); E::Let(nm, Box::new(v), Box::new(b), lo, h) },
            _=>E::Int(0, lo, hi) } }
    fn postfix(&mut self)->E{ let mut e=self.atom();
        loop {
            if self.is_tok("as") { let _=self.adv(); let t=self.parse_tyname(); let a=crate::oast::e_lo(&e); let b=if self.i>0 { self.t[self.i-1].hi } else { a }; e=E::Cast(Box::new(e), t, a, b); }
            else if self.is_tok("?") { let _=self.adv(); let a=crate::oast::e_lo(&e); let b=if self.i>0 { self.t[self.i-1].hi } else { a }; e=E::Try(Box::new(e), a, b); }
            else if self.is_tok("[") { let _=self.adv();
                let a=crate::oast::e_lo(&e);
                // a RANGE index `v[a..b]` / `v[a..]` / `v[..b]` / `v[..]` is a slice: it COPIES into a
                // fresh vector. A copy is only honest in a value position, so a slice used as a method
                // RECEIVER (`self[..n].clone_from_slice(..)`, which mutates through it) is refused.
                let open=self.is_tok("..");
                let ix=if open { E::Int(0, a, a) } else { self.cmp_e() };
                if self.is_tok("..") { let _=self.adv();
                    if self.is_tok("=") { let _=self.adv(); }
                    let endx = if self.is_tok("]") { let mut la:Vec<E>=Vec::new(); la.push(clone_simple(&e, a, a)); E::Call(String::from("seq_len"), la, a, a) }
                               else { self.cmp_e() };
                    self.eat("]");
                    let b=if self.i>0 { self.t[self.i-1].hi } else { a };
                    // C-SLICEREAD: the refusal below is about MUTATION -- our seq_slice answers a COPY, so a
                    // method that writes through the slice would silently do nothing. A method that READS is
                    // a different matter and rustc accepts it everywhere: probed, s[1..4].len(),
                    // s[1..4].to_string(), src[i..i+1].chars() and v[1..3].to_vec() all compile and run. So
                    // the refusal is narrowed to the mutators, and a read is admitted.
                    if self.is_tok(".") && self.i+1<self.t.len() && slice_mutator(&tok_word(&self.t[self.i+1]))
                       { if self.perr==0 { self.perr=a+1; self.perrexp=String::from("a mutating method on a slice in a value position (our slice is a COPY, so the write would be lost)"); self.perrgot=self.tok_text(); } }
                    let mut sa:Vec<E>=Vec::new(); sa.push(e); sa.push(ix); sa.push(endx);
                    e=E::Call(String::from("seq_slice"), sa, a, b); }
                else { self.eat("]"); let b=if self.i>0 { self.t[self.i-1].hi } else { a }; e=E::Index(Box::new(e), Box::new(ix), a, b); } }
            else if self.is_tok(".") { let _=self.adv(); let fi=self.adv();
                match self.t[fi].k.clone() { Tk::Num(n)=>{ let a0=crate::oast::e_lo(&e); let b0=self.t[fi].hi; e=E::TupIdx(Box::new(e), n as u32, a0, b0); continue; }, _=>{} }
                let fname=match self.t[fi].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") }; let a=crate::oast::e_lo(&e); let b=self.t[fi].hi;
                // a METHOD may carry a turbofish: `.sum::<i32>()`, `.collect::<Vec<_>>()`
                if self.is_tok("::") && self.peek_is(1,"<") { let _=self.adv(); self.skip_generics(); }
                if self.is_tok("(") { let _=self.adv(); let mut args:Vec<E>=Vec::new(); loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; } let aprog=self.i; let ag=self.cmp_e(); args.push(ag); if self.is_tok(",") { let _=self.adv(); } if self.noprog(aprog) { }  } let b2=if self.i>0 { self.t[self.i-1].hi } else { b }; e=E::MethodCall(Box::new(e), fname, args, a, b2); }
                else { e=E::Field(Box::new(e), fname, a, b); } }
            else { break; }
        }
        e }
    fn postfix_old(&mut self)->E{ let mut e=self.atom(); while self.is_tok(".") { let _=self.adv(); let fi=self.adv(); let fname=match self.t[fi].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") }; let a=crate::oast::e_lo(&e); let b=self.t[fi].hi; e=E::Field(Box::new(e), fname, a, b); } e }
    fn unary(&mut self)->E{
        // prefix DEREFERENCE: `*x` / `**x` is `x` here -- C2 says a reference is the value. Without this
        // a closure body like `**x % 2 == 1` read as a multiplication.
        if self.is_tok("*") { let _=self.adv(); return self.unary(); }
        // unary MINUS: `-5`, `-x`, `-0xBEEF` -- read as `0 - operand`, which is what our int model is
        if self.is_tok("-") { let lo=self.lo(); let _=self.adv(); let x=self.unary(); let h=crate::oast::e_hi(&x);
            return E::Bin(String::from("-"), Box::new(E::Int(0, lo, lo)), Box::new(x), lo, h); }
        if self.is_tok("!") { let lo=self.lo(); let _=self.adv(); let x=self.unary(); let h=crate::oast::e_hi(&x); return E::Not(Box::new(x), lo, h); } self.postfix() }
    fn mul(&mut self)->E{ let mut l=self.unary(); loop {
            if self.is_tok("*") && block_form(&l) { break; }
            if self.is_tok("*") { let _=self.adv(); let r=self.unary(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from("*"),Box::new(l),Box::new(r),a,b); } else if self.is_tok("/") { let _=self.adv(); let r=self.unary(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from("/"),Box::new(l),Box::new(r),a,b); }
            // K3 (probed from RC): `%` is multiplicative, and on negatives it truncates toward zero --
            // the remainder takes the DIVIDEND's sign (`-7 % 2 == -1`). Our own prelude had been writing
            // `v - v / 10 * 10` for want of it, which is a constraint we had accumulated without stating.
            else if self.is_tok("%") { let _=self.adv(); let r=self.unary(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from("%"),Box::new(l),Box::new(r),a,b); }
            else { break; } } l }
    // the BITWISE layers, in Rust's order: `|` binds loosest, then `^`, then `&`, then the shifts
    fn bor_e(&mut self)->E{ let mut l=self.bxor_e(); while self.is_tok("|") { let _=self.adv(); let r=self.bxor_e(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from("|"),Box::new(l),Box::new(r),a,b); } l }
    fn bxor_e(&mut self)->E{ let mut l=self.band_e(); while self.is_tok("^") { let _=self.adv(); let r=self.band_e(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from("^"),Box::new(l),Box::new(r),a,b); } l }
    fn band_e(&mut self)->E{ let mut l=self.shift_e(); while self.is_tok("&") { let _=self.adv(); let r=self.shift_e(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from("&"),Box::new(l),Box::new(r),a,b); } l }
    fn shift_e(&mut self)->E{ let mut l=self.add();
        loop { if self.is_tok("<<") { let _=self.adv(); let r=self.add(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from("<<"),Box::new(l),Box::new(r),a,b); }
               else if self.is_tok(">>") { let _=self.adv(); let r=self.add(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from(">>"),Box::new(l),Box::new(r),a,b); }
               else { break; } } l }
    fn add(&mut self)->E{ let mut l=self.mul(); loop { if self.is_tok("+") { let _=self.adv(); let r=self.mul(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from("+"),Box::new(l),Box::new(r),a,b); } else if self.is_tok("-") { let _=self.adv(); let r=self.mul(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from("-"),Box::new(l),Box::new(r),a,b); } else { break; } } l }
    // Sequencing. Rust lets a BLOCK-FORM statement (`while .. {}`, `if .. {}`) stand without a `;`, so after
    // parsing one element continue the sequence when the element ended in a block and something other than `}`
    // follows. Without this the remainder of the block was silently DROPPED (a tokenizer written in this
    // language lost every statement after an inner `while`, and the compiler reported ok).
    // A type NAME as a string: `int`, `Sq`, `dyn Area`, `Vec<dyn Area>`, `Vec<int>`. The layers below speak
    // type names, so parsing a nested type to a name (`Vec$dyn$Area`) keeps one representation everywhere.
    // step over a generic parameter list we do not need to bind (`impl<T, 'a>`, `trait Foo<T>`)
    // P-GEN: the one generic-signature reader. A parameter may carry type ARGUMENTS, associated-type
    // constraints, `+`-composed bounds and `?Sized`; the FIRST bound's name is what monomorphization keys
    // on. Every item kind calls this, so a bound form learned once is learned everywhere.
    // P-PATH: read a path's remaining `::seg` segments (and any turbofish) after the first identifier.
    // The old shape matched `Ident::Ident` as a two-segment special case, so `a::b::c`,
    // `Type::assoc::<T>` and `m::T::item` all derailed.
    // a range-pattern endpoint: an integer literal, a char literal (already a Num in our lexer), or a
    // CONST name used as a value
    fn pat_endpoint(&mut self, lo:u32, hi:u32)->E{
        if self.i>=self.t.len() { return E::Int(0, lo, hi); }
        match self.t[self.i].k.clone() {
            Tk::Num(n)=>{ let _=self.adv(); E::Int(n, lo, hi) },
            Tk::ChL(n)=>{ let _=self.adv(); E::Int(n, lo, hi) },
            Tk::Ident(w)=>{ let _=self.adv(); let ps:Vec<E>=Vec::new(); E::Call(String::from(w.as_str()), ps, lo, hi) },
            _=>E::Int(0, lo, hi) } }
    fn read_path_tail(&mut self, segs:&mut Vec<String>){
        loop {
            if !self.is_tok("::") { break; }
            if self.peek_is(1,"<") { let _=self.adv(); self.skip_generics(); continue; }   // turbofish
            let _=self.adv();
            if self.i>=self.t.len() { break; }
            let w=tok_word(&self.t[self.i]);
            if w.len()==0 { break; }
            segs.push(w); let _=self.adv(); } }
    // resolve a collected path to a callable/constructible name
    fn mod_known(&self, n:&str)->bool{ let mut i=0; while i<self.mods.len(){ if self.mods[i].as_str()==n { return true; } i=i+1; } false }
    // the field names of a struct VARIANT, in declaration order ("" when it is positional)
    fn is_smut(&self, n:&str)->bool{ let mut i=0; while i<self.smut.len(){ if self.smut[i].as_str()==n { return true; } i=i+1; } false }
    fn variant_fields(&self, v:&str)->String{
        let mut i=0; while i<self.vnames.len(){ if self.vnames[i].as_str()==v && i<self.vfn.len() { return String::from(self.vfn[i].as_str()); } i=i+1; }
        String::from("") }
    fn assoc_ty(&self, ty:&str, name:&str)->String{
        let mut key=String::from(ty); key.push('$'); key.push_str(name);
        let mut i=0; while i<self.atn.len(){ if self.atn[i].as_str()==key.as_str() { return String::from(self.att[i].as_str()); } i=i+1; }
        String::from("") }
    fn fknown(&self, n:&str)->bool{ let mut i=0; while i<self.fnames.len(){ if self.fnames[i].as_str()==n { return true; } i=i+1; } false }
    // a glob is in effect only inside the module that declared it; entries are `<declaring mpfx>|<prefix>`
    fn gres(&self, n:&str)->String{
        let mut i=0;
        while i<self.globs.len(){
            let b=self.globs[i].as_bytes(); let mut cut=0; let mut k=0;
            while k<b.len(){ if b[k]==124 { cut=k; k=b.len(); } else { k=k+1; } }
            let own=String::from(&self.globs[i][..cut]);
            if own.as_str()==self.mpfx.as_str() {
                let mut q=String::from(&self.globs[i][(cut+1)..]); q.push('$'); q.push_str(n);
                if self.sknown(&q) || self.enames_has(&q) || self.fknown(&q) { return q; } }
            i=i+1; }
        String::from("") }
    // the TYPES-only glob resolver: a struct or enum name a glob brings into this module
    // the one resolver for a bare type name: exact, then this module's own, then an explicit `use` alias,
    // then a glob import. Asked by the struct-literal test, the enum-literal path and parse_tyname.
    fn tyres(&self, n:&str)->String{
        if self.mpfx.len()>0 { let q=self.qual(n); if self.sknown(&q) || self.enames_has(&q) { return q; } }
        { let a=self.use_alias_ty(n); if a.len()>0 { return a; } }
        { let g=self.gres_ty(n); if g.len()>0 { return g; } }
        if self.sknown(n) || self.enames_has(n) { return String::from(n); }
        String::from(n) }
    // the explicit-`use` half, types only (use_alias itself is the call-position resolver)
    fn use_alias_ty(&self, n:&str)->String{
        let mut i=0;
        while i<self.usen.len(){ if self.usen[i].as_str()==n {
            let t=String::from(self.uset[i].as_str());
            if self.sknown(&t) || self.enames_has(&t) { return t; } } i=i+1; }
        String::from("") }
    fn gres_ty(&self, n:&str)->String{
        let mut i=0;
        while i<self.globs.len(){
            let b=self.globs[i].as_bytes(); let mut cut=0; let mut k=0;
            while k<b.len(){ if b[k]==124 { cut=k; k=b.len(); } else { k=k+1; } }
            if String::from(&self.globs[i][..cut]).as_str()==self.mpfx.as_str() {
                let mut q=String::from(&self.globs[i][(cut+1)..]); q.push('$'); q.push_str(n);
                if self.sknown(&q) || self.enames_has(&q) { return q; } }
            i=i+1; }
        String::from("") }
    fn use_alias(&self, n:&str)->String{
        let mut i=0; while i<self.usen.len(){ if self.usen[i].as_str()==n { return String::from(self.uset[i].as_str()); } i=i+1; }
        // P-MC-MODFN: a bare call to a SIBLING function inside the same module. Functions in a module are
        // registered under the module prefix (`h$a`), so a bare `a()` in that module's own code did not
        // resolve -- the same rule P-MC-MODTYPE fixed for TYPES, missing for functions. Call position only,
        // so a local binding that shares a sibling's name is untouched.
        if self.mpfx.len()>0 && !self.fknown(n) && self.peek_is(0,"(") { let q=self.qual(n); if self.fknown(&q) { return q; } }
        // then a glob import -- but a LOCAL item wins, as in rustc, and this runs only when a glob is in scope
        if self.globs.len()>0 && !self.fknown(n) && self.peek_is(0,"(") {
            let mut local=false;
            if self.mpfx.len()>0 { let q=self.qual(n); if self.fknown(&q) { local=true; } }
            if !local { let g=self.gres(n); if g.len()>0 && self.fknown(&g) { return g; } } }
        String::from("") }
    fn resolve_path(&self, segs:&Vec<String>)->String{
        if segs.len()==0 { return String::from(""); }
        if segs.len()>1 && segs[0].as_str()=="crate" { let mut rest:Vec<String>=Vec::new(); let mut k=1; while k<segs.len(){ rest.push(String::from(segs[k].as_str())); k=k+1; } return self.resolve_path(&rest); }
        if segs.len()==1 { return String::from(segs[0].as_str()); }
        // `a::b::c..` -- join with `$`, which is how modules and associated items are named
        let mut joined=String::from(segs[0].as_str());
        let mut i=1; while i<segs.len(){ joined.push('$'); joined.push_str(segs[i].as_str()); i=i+1; }
        // a MODULE-qualified item may itself be a two-segment name inside the module
        joined }
    fn read_generics(&mut self, gps:&mut Vec<String>, gbs:&mut Vec<String>){
        if !self.is_tok("<") { return; }
        let _=self.adv();
        loop { if self.at_gt() { let _=self.eat_gt(); break; }
            
            if self.at_end() { break; }
            if self.is_tok(",") { let _=self.adv(); continue; }
            let gi=self.adv();
            match self.t[gi].k.clone() { Tk::Ident(x)=>{ gps.push(String::from(x.as_str()));
                  if self.is_tok(":") { let _=self.adv();
                      if self.is_tok("?") { let _=self.adv(); }
                      if self.is_tok("for") { let _=self.adv(); self.skip_generics(); }
                      let mut bn=String::from("");
                      if !self.at_gt() && !self.is_tok(",") { let bi=self.adv(); match self.t[bi].k.clone() { Tk::Ident(y)=>{ bn=String::from(y.as_str()); }, _=>{} } }
                      self.skip_bound_tail();
                      loop { if self.is_tok("+") { let _=self.adv(); if self.is_tok("?") { let _=self.adv(); } if !self.at_gt() && !self.is_tok(",") { let _=self.adv(); } self.skip_bound_tail(); } else { break; } }
                      gbs.push(bn); }
                  else { gbs.push(String::from("")); } }, _=>{} } } }
    // after a bound NAME: `Fn(A, B) -> R` carries a parenthesized list and a return type
    fn skip_bound_tail(&mut self){
        self.skip_generics();
        if self.is_tok("(") { let mut d=0;
            loop { if self.at_end() { break; }
                if self.is_tok("(") { d=d+1; } else if self.is_tok(")") { d=d-1; let _=self.adv(); if d==0 { break; } continue; }
                let _=self.adv(); } }
        if self.is_tok("-") && self.peek_is(1,">") { let _=self.adv(); let _=self.adv(); let _=self.parse_tyname(); } }
    fn skip_generics(&mut self){ if !self.is_tok("<") { return; } let mut d=0;
        loop { if self.at_end() { break; }
            if self.is_tok("<") { d=d+1; } else if self.at_gt() { d=d-1; let _=self.eat_gt(); if d==0 { break; } continue; }
            else if self.is_tok(">>") { d=d-2; let _=self.adv(); if d<=0 { break; } continue; }
            let _=self.adv(); } }
    // a `where` clause carries no meaning here: skip to the body or the `;`
    // consume a balanced token run up to (not including) the matching `}` -- the opening brace has
    // already been eaten by the caller
    fn skip_body(&mut self)->E{
        let lo=self.lo(); let mut d=0;
        loop { if self.at_end() { break; }
            if self.is_tok("}") { if d==0 { break; } d=d-1; }
            else if self.is_tok("{") { d=d+1; }
            let _=self.adv(); }
        E::Int(0, lo, lo) }
    fn skip_to_semi(&mut self)->E{
        let lo=self.lo(); let mut d=0;
        loop { if self.at_end() { break; }
            if self.is_tok(";") && d==0 { break; }
            if self.is_tok("{") || self.is_tok("(") || self.is_tok("[") { d=d+1; }
            else if self.is_tok("}") || self.is_tok(")") || self.is_tok("]") { d=d-1; }
            let _=self.adv(); }
        E::Int(0, lo, lo) }
    fn skip_where(&mut self){ let g:Vec<String>=Vec::new(); let g2:Vec<String>=Vec::new(); let mut b:Vec<String>=Vec::new(); self.read_where(&g, &mut b, &g2); }
    // `where T: Mark` carries the SAME bound a `<T: Mark>` list does -- discarding it made a bounded
    // generic look unbounded, so no monomorphization ran and the method call emitted an undefined name
    fn read_where(&mut self, gps:&Vec<String>, gbs:&mut Vec<String>, _unused:&Vec<String>){
        if !self.is_tok("where") { return; }
        let _=self.adv();
        loop { if self.at_end() { break; } if self.is_tok("{") || self.is_tok(";") { break; }
            if self.is_tok(",") { let _=self.adv(); continue; }
            let w=tok_word(&self.t[self.i]);
            if w.len()==0 { let _=self.adv(); continue; }
            let _=self.adv();
            if !self.is_tok(":") { continue; }
            let _=self.adv();
            if self.is_tok("?") { let _=self.adv(); }
            // a HIGHER-RANKED bound: `for<'a> Fn(&'a u8) -> u8`. The quantifier binds lifetimes, which we
            // do not have, so it is skipped -- and the bound NAME after it is what must be recorded.
            if self.is_tok("for") { let _=self.adv(); self.skip_generics(); }
            let mut bn=String::from("");
            if !self.is_tok("{") && !self.is_tok(",") && !self.is_tok(";") {
                let bw=tok_word(&self.t[self.i]);
                if bw.len()>0 { bn=bw; let _=self.adv(); } else { let _=self.adv(); } }
            self.skip_bound_tail();
            loop { if self.is_tok("+") { let _=self.adv(); if self.is_tok("?") { let _=self.adv(); }
                       if !self.is_tok("{") && !self.is_tok(",") && !self.is_tok(";") { let _=self.adv(); }
                       self.skip_bound_tail(); } else { break; } }
            // record it against the generic parameter of that name, if the caller gave us the list
            if bn.len()>0 { let mut q=0; while q<gps.len(){
                if gps[q].as_str()==w.as_str() && q<gbs.len() && gbs[q].len()==0 { gbs[q]=String::from(bn.as_str()); }
                q=q+1; } } } }
    // a loop iteration that consumed nothing: report it and step over one token. Used by every inner loop
    // that can meet a token it does not expect -- without it the parser spins forever.
    // is the current token the identifier `w`? (keywords go through tk_is; this is for words like
    // `macro_rules` that the lexer has no token for)
    // ---- macro_rules EXPANSION ----
    // Match an invocation's tokens against a rule's pattern, binding each `$name:frag` to a token run.
    // A metavariable consumes tokens until the pattern's NEXT literal token appears at depth 0 (or the
    // arguments run out), which covers the shapes real macros use without a full fragment parser.
    fn mac_match(&self, pat:&Vec<Tok>, args:&Vec<Tok>, ns:&mut Vec<String>, vs:&mut Vec<Vec<Tok>>)->bool{
        let mut pi=0; let mut ai=0;
        while pi<pat.len(){
            if tk_is(&pat[pi].k,"$") && pi+1<pat.len() {
                let nm=tok_word(&pat[pi+1]);
                let mut pj=pi+2;
                if pj<pat.len() && tk_is(&pat[pj].k,":") { pj=pj+2; }      // skip `:frag`
                // the delimiter that ends this capture
                let mut cap:Vec<Tok>=Vec::new(); let mut d=0;
                while ai<args.len(){
                    if d==0 && pj<pat.len() && tok_same(&args[ai], &pat[pj]) { break; }
                    if tok_opens(&args[ai]) { d=d+1; }
                    if tok_closes(&args[ai]) { if d==0 { break; } d=d-1; }
                    cap.push(tok_clone(&args[ai])); ai=ai+1; }
                if cap.len()==0 { return false; }
                ns.push(nm); vs.push(cap); pi=pj; continue; }
            if ai>=args.len() { return false; }
            if !tok_same(&pat[pi], &args[ai]) { return false; }
            pi=pi+1; ai=ai+1; }
        ai>=args.len() }
    // Substitute the bound captures into a rule body.
    // the identifiers a macro body BINDS with `let`
    fn mac_body_lets(&self, body:&Vec<Tok>)->Vec<String>{
        let mut out:Vec<String>=Vec::new(); let mut i=0;
        while i<body.len(){
            if tk_is(&body[i].k,"let") {
                let mut j=i+1;
                if j<body.len() && tk_is(&body[j].k,"mut") { j=j+1; }
                if j<body.len() { let w=tok_word(&body[j]); if w.len()>0 && !crate::oast::in_list(&out,&w) { out.push(w); } } }
            i=i+1; }
        out }
    fn mac_subst(&self, body:&Vec<Tok>, ns:&Vec<String>, vs:&Vec<Vec<Tok>>)->Vec<Tok>{ self.mac_subst_h(body, ns, vs, 0) }
    fn mac_subst_h(&self, body:&Vec<Tok>, ns:&Vec<String>, vs:&Vec<Vec<Tok>>, hyg:u32)->Vec<Tok>{
        let lets=self.mac_body_lets(body);
        let mut out:Vec<Tok>=Vec::new(); let mut i=0;
        while i<body.len(){
            if tk_is(&body[i].k,"$") && i+1<body.len() {
                let nm=tok_word(&body[i+1]);
                let mut hit=0-1; let mut q=0;
                while q<ns.len(){ if ns[q].as_str()==nm.as_str() { hit=q as i32; } q=q+1; }
                if hit>=0 { let hv=&vs[hit as usize]; let mut k=0; while k<hv.len(){ out.push(tok_clone(&hv[k])); k=k+1; } i=i+2; continue; } }
            // rename a body-introduced binding (and its uses) so it cannot capture the caller's name
            { let w=tok_word(&body[i]);
              if w.len()>0 && crate::oast::in_list(&lets,&w) {
                let mut nw=String::from(w.as_str()); nw.push_str("$h"); nw.push_str(&u32s2(hyg));
                out.push(Tok{k:Tk::Ident(nw), lo:body[i].lo, hi:body[i].hi}); i=i+1; continue; } }
            out.push(tok_clone(&body[i])); i=i+1; }
        out }
    // is this pattern a single repetition group? returns (inner, separator-is-comma) via out-params
    fn mac_rep_inner(&self, pat:&Vec<Tok>, inner:&mut Vec<Tok>)->bool{
        if pat.len()<4 { return false; }
        if !tk_is(&pat[0].k,"$") { return false; }
        if !tok_opens(&pat[1]) { return false; }
        // find the matching close
        let mut d=0; let mut i=1; let mut close=0;
        while i<pat.len(){ if tok_opens(&pat[i]) { d=d+1; } else if tok_closes(&pat[i]) { d=d-1; if d==0 { close=i; i=pat.len(); continue; } } i=i+1; }
        if close==0 { return false; }
        // what follows must be `,*` / `;*` / `*` / `+` (optionally with a trailing `,`)
        let mut j=close+1;
        if j<pat.len() && (tk_is(&pat[j].k,",") || tk_is(&pat[j].k,";")) { j=j+1; }
        if j>=pat.len() { return false; }
        if !(tk_is(&pat[j].k,"*") || tk_is(&pat[j].k,"+")) { return false; }
        let mut k=2; while k<close { inner.push(tok_clone(&pat[k])); k=k+1; }
        true }
    // split a token run on top-level commas
    fn mac_split(&self, args:&Vec<Tok>)->Vec<Vec<Tok>>{
        let mut out:Vec<Vec<Tok>>=Vec::new(); let mut cur:Vec<Tok>=Vec::new(); let mut d=0; let mut i=0;
        while i<args.len(){
            if tok_opens(&args[i]) { d=d+1; } else if tok_closes(&args[i]) { d=d-1; }
            if d==0 && tk_is(&args[i].k,",") { if cur.len()>0 { out.push(toks_clone(&cur)); cur=Vec::new(); } }
            else { cur.push(tok_clone(&args[i])); }
            i=i+1; }
        if cur.len()>0 { out.push(cur); }
        out }
    // expand a body containing `$( inner ),*` once per captured group
    fn mac_subst_rep(&self, body:&Vec<Tok>, groups:&Vec<Vec<Tok>>, inner:&Vec<Tok>)->Vec<Tok>{
        let mut out:Vec<Tok>=Vec::new(); let mut i=0;
        while i<body.len(){
            if tk_is(&body[i].k,"$") && i+1<body.len() && tok_opens(&body[i+1]) {
                // the repeated body, its close, and the separator
                let mut d=0; let mut j=i+1; let mut close=0;
                while j<body.len(){ if tok_opens(&body[j]) { d=d+1; } else if tok_closes(&body[j]) { d=d-1; if d==0 { close=j; j=body.len(); continue; } } j=j+1; }
                if close>0 {
                    let mut rb:Vec<Tok>=Vec::new(); let mut k=i+2; while k<close { rb.push(tok_clone(&body[k])); k=k+1; }
                    let mut sep:Vec<Tok>=Vec::new(); let mut n2=close+1;
                    if n2<body.len() && (tk_is(&body[n2].k,",") || tk_is(&body[n2].k,";")) { sep.push(tok_clone(&body[n2])); n2=n2+1; }
                    if n2<body.len() && (tk_is(&body[n2].k,"*") || tk_is(&body[n2].k,"+")) { n2=n2+1; }
                    let mut g=0;
                    while g<groups.len(){
                        if g>0 { let mut q=0; while q<sep.len(){ out.push(tok_clone(&sep[q])); q=q+1; } }
                        let mut ns:Vec<String>=Vec::new(); let mut vs:Vec<Vec<Tok>>=Vec::new();
                        if self.mac_match(inner, &groups[g], &mut ns, &mut vs) {
                            let piece=self.mac_subst(&rb, &ns, &vs);
                            let mut q2=0; while q2<piece.len(){ out.push(tok_clone(&piece[q2])); q2=q2+1; } }
                        g=g+1; }
                    i=n2; continue; } }
            out.push(tok_clone(&body[i])); i=i+1; }
        out }
    fn mac_index(&self, n:&str)->i32{ let mut i=0; while i<self.macs.len(){ if self.macs[i].name.as_str()==n { return i as i32; } i=i+1; } 0-1 }
    // Expand `name!(args)` at the cursor: collect the argument tokens, match a rule, splice the expansion
    // into the token stream in place of the invocation, and keep parsing.
    fn mac_expand(&mut self, mi:usize)->bool{
        let start=self.i;
        let _=self.adv();                 // the macro name
        if self.is_tok("!") { let _=self.adv(); } else { self.i=start; return false; }
        if !(self.is_tok("(") || self.is_tok("[") || self.is_tok("{")) { self.i=start; return false; }
        let _=self.adv();
        let mut args:Vec<Tok>=Vec::new(); let mut d=1;
        while self.i<self.t.len(){
            if tok_opens(&self.t[self.i]) { d=d+1; }
            else if tok_closes(&self.t[self.i]) { d=d-1; if d==0 { let _=self.adv(); break; } }
            args.push(tok_clone(&self.t[self.i])); let _=self.adv(); }
        let after=self.i;
        // try each rule in order
        let mut r=0;
        while r<self.macs[mi].rules.len(){
            let mut ns:Vec<String>=Vec::new(); let mut vs:Vec<Vec<Tok>>=Vec::new();
            // a REPETITION rule first: `$( .. ),*` binds a list of groups
            { let mut inner:Vec<Tok>=Vec::new();
              let isrep={ let pat=&self.macs[mi].rules[r].pat; self.mac_rep_inner(pat, &mut inner) };
              if isrep {
                let groups=self.mac_split(&args);
                self.nhyg=self.nhyg+1;
                let exp={ let body=&self.macs[mi].rules[r].body; self.mac_subst_rep(body, &groups, &inner) };
                let mut nt:Vec<Tok>=Vec::new();
                let mut k=0; while k<start { nt.push(tok_clone(&self.t[k])); k=k+1; }
                let mut e=0; while e<exp.len(){ nt.push(tok_clone(&exp[e])); e=e+1; }
                let mut z=after; while z<self.t.len(){ nt.push(tok_clone(&self.t[z])); z=z+1; }
                self.t=nt; self.i=start;
                return true; } }
            let okm={ let pat=&self.macs[mi].rules[r].pat; self.mac_match(pat, &args, &mut ns, &mut vs) };
            if okm {
                self.nhyg=self.nhyg+1; let hyg=self.nhyg;
                let exp={ let body=&self.macs[mi].rules[r].body; self.mac_subst_h(body, &ns, &vs, hyg) };
                let mut nt:Vec<Tok>=Vec::new();
                let mut k=0; while k<start { nt.push(tok_clone(&self.t[k])); k=k+1; }
                let mut e=0; while e<exp.len(){ nt.push(tok_clone(&exp[e])); e=e+1; }
                let mut z=after; while z<self.t.len(){ nt.push(tok_clone(&self.t[z])); z=z+1; }
                self.t=nt; self.i=start;
                return true; }
            r=r+1; }
        self.i=start; false }
    // `macro_rules! name { (pat) => (body) ; .. }` -- each rule's pattern and body are kept as TOKEN RUNS
    fn read_macro(&mut self){
        let _=self.adv(); if self.is_tok("!") { let _=self.adv(); }
        let nm=if self.i<self.t.len() { tok_word(&self.t[self.i]) } else { String::from("") };
        let _=self.adv();
        if !(self.is_tok("{") || self.is_tok("(") || self.is_tok("[")) { return; }
        let _=self.adv();
        let mut rules:Vec<MacRule>=Vec::new();
        loop {
            if self.at_end() { break; }
            if self.is_tok("}") || self.is_tok(")") || self.is_tok("]") { let _=self.adv(); break; }
            if self.is_tok(";") || self.is_tok(",") { let _=self.adv(); continue; }
            // the PATTERN group
            if !(self.is_tok("(") || self.is_tok("[") || self.is_tok("{")) { let _=self.adv(); continue; }
            let _=self.adv();
            let mut pat:Vec<Tok>=Vec::new(); let mut d=1;
            while self.i<self.t.len(){
                if tok_opens(&self.t[self.i]) { d=d+1; }
                else if tok_closes(&self.t[self.i]) { d=d-1; if d==0 { let _=self.adv(); break; } }
                pat.push(tok_clone(&self.t[self.i])); let _=self.adv(); }
            if self.is_tok("=>") { let _=self.adv(); }
            if !(self.is_tok("(") || self.is_tok("[") || self.is_tok("{")) { continue; }
            let _=self.adv();
            let mut body:Vec<Tok>=Vec::new(); let mut d2=1;
            while self.i<self.t.len(){
                if tok_opens(&self.t[self.i]) { d2=d2+1; }
                else if tok_closes(&self.t[self.i]) { d2=d2-1; if d2==0 { let _=self.adv(); break; } }
                body.push(tok_clone(&self.t[self.i])); let _=self.adv(); }
            rules.push(MacRule{pat:pat, body:body});
        }
        if self.is_tok(";") { let _=self.adv(); }
        if nm.len()>0 { self.macs.push(Mac{name:nm, rules:rules}); }
    }
    fn is_word(&self, w:&str)->bool{ if self.i>=self.t.len() { return false; } match self.t[self.i].k.clone() { Tk::Ident(x)=>x.as_str()==w, _=>false } }
    fn noprog(&mut self, at:usize)->bool{ if self.i==at { if self.perr==0 { self.perr=self.lo()+1; self.perrexp=String::from("an expression"); self.perrgot=self.tok_text(); } let _=self.adv(); true } else { false } }
    fn peek_is(&self, d:usize, k:&str)->bool{ if self.i+d >= self.t.len() { return false; } tk_is(&self.t[self.i+d].k, k) }
    // the assignment TARGET for a compound assignment: rebuild the place from the parsed left side
    fn reparse_place(&mut self, l:&E, a:u32, b:u32)->E{
        match l { E::Var(n,_l,_h)=>E::Var(String::from(n.as_str()), a, b),
                  // a global LOAD is a place: `COUNT += 1` rebuilds the same load and lowering turns the
                  // assignment into a store
                  E::Call(n,ar,_l,_h) if ar.len()==0 && crate::olower::is_gname(n) =>{ let z:Vec<E>=Vec::new(); E::Call(String::from(n.as_str()), z, a, b) },
                  E::Field(o,f,_l,_h)=>E::Field(Box::new(self.reparse_place(o,a,b)), String::from(f.as_str()), a, b),
                  E::TupIdx(o,ix,_l,_h)=>E::TupIdx(Box::new(self.reparse_place(o,a,b)), *ix, a, b),
                  // an INDEX place is only rebuilt when the index is side-effect-free (a local or a literal);
                  // anything else would evaluate it twice, so it is refused rather than mis-compiled
                  E::Index(bb,ix,_l,_h)=>{ match ix.as_ref() {
                        E::Var(n2,_l2,_h2)=>E::Index(Box::new(self.reparse_place(bb,a,b)), Box::new(E::Var(String::from(n2.as_str()), a, b)), a, b),
                        E::Int(v2,_l2,_h2)=>E::Index(Box::new(self.reparse_place(bb,a,b)), Box::new(E::Int(*v2, a, b)), a, b),
                        _=>E::Var(String::from("$noplace"), a, b) } },
                  _=>E::Var(String::from("$noplace"), a, b) } }
    fn parse_tyname(&mut self)->String{
        if self.is_tok("crate") && self.peek_is(1,"::") { let _=self.adv(); let _=self.adv(); return self.parse_tyname(); }
        if self.depth>96 { if self.perr==0 { self.perr=self.lo()+1; self.perrexp=String::from("a type (nesting too deep)"); self.perrgot=self.tok_text(); } let _=self.adv(); return String::from(""); }
        self.depth=self.depth+1;
        let r=self.parse_tyname_inner();
        self.depth=self.depth-1;
        r }
    fn parse_tyname_inner(&mut self)->String{
        if self.is_tok("::") { let _=self.adv(); }
        if self.is_tok("&") { let _=self.adv(); if self.is_tok("mut") { let _=self.adv(); } return self.parse_tyname(); }
        // `()` is the unit type; `(A, B)` a tuple; `[T]` / `[T; N]` a slice or array; `<T as Tr>::It` a
        // qualified path. Each renders to a NAME, the representation every layer below speaks.
        if self.is_tok("(") { let _=self.adv(); let mut nc=0; let mut es:Vec<String>=Vec::new();
            loop { if self.is_tok(")") { let _=self.adv(); break; } if self.at_end() { break; }
                   if self.is_tok(",") { let _=self.adv(); } else { es.push(self.parse_tyname()); nc=nc+1; } }
            if nc==0 { return String::from("unit"); }
            return tup_name(nc as u32, &es); }
        if self.is_tok("[") { let _=self.adv(); let el=self.parse_tyname();
            // `[T; N]` is an ARRAY (length at slot 0, data from 1); `[T]` is a SLICE, and the only values
            // that reach a slice parameter are vectors -- typing it `arr` read the vec header as data
            let mut semi=false;
            loop { if self.is_tok("]") { let _=self.adv(); break; } if self.at_end() { break; }
                   if self.is_tok(";") { semi=true; } let _=self.adv(); }
            if semi { return String::from("arr"); }
            let mut r=String::from("slice"); if el.len()>0 { r.push('$'); r.push_str(el.as_str()); } return r; }
        if self.is_tok("<") { let _=self.adv(); let base=self.parse_tyname();
            if self.is_tok("as") { let _=self.adv(); let _t=self.parse_tyname(); }
            if self.at_gt() { let _=self.eat_gt(); }
            if self.is_tok("::") { let _=self.adv();
                let ii=self.adv(); let an=match self.t[ii].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("") };
                // `<S as T1>::Out` is the impl's own `type Out = ..`
                let r=self.assoc_ty(&base,&an); if r.len()>0 { return r; } }
            return base; }
        if self.is_tok("impl") { let _=self.adv();
            if self.i<self.t.len() { let _=self.adv(); }          // the trait name
            self.skip_bound_tail();
            loop { if self.is_tok("+") { let _=self.adv(); if self.i<self.t.len() { let _=self.adv(); } self.skip_bound_tail(); } else { break; } }
            return String::from(""); }
        if self.is_tok("dyn") { let _=self.adv(); let i=self.adv(); let n=match self.t[i].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("") };
            // `dyn Trait<T>` / `dyn Trait<Assoc = T>` -- the arguments do not change the object's name
            self.skip_generics(); self.skip_bound_tail();
            let mut r=String::from("dyn$"); r.push_str(&n); return r; }
        // a raw pointer type `*const T` / `*mut T` is the referent's type, like a reference
        if self.is_tok("*") { let _=self.adv(); if self.is_tok("mut") { let _=self.adv(); } let i2=self.i; if self.is_tok("const") { let _=self.adv(); } let _=i2; return self.parse_tyname(); }
        let i=self.adv(); let mut n=match self.t[i].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("") };
        if n.as_str()=="_" { return String::from(""); }
        if n.as_str()=="Self" && self.is_tok("::") && self.selfty.len()>0 { let _=self.adv();
            let ii=self.adv(); let an=match self.t[ii].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("") };
            let sty=String::from(self.selfty.as_str());
            let r=self.assoc_ty(&sty,&an); if r.len()>0 { return r; } return sty; }
        if n.as_str()=="Self" && self.selfty.len()>0 { return String::from(self.selfty.as_str()); }
        // `M::T` inside a path, and a SHORT name inside `mod M` resolving to `M$T` -- without the latter a
        // parameter typed `Pt` inside `mod geom` missed the `geom$Pt` declaration and every field read
        // collapsed to offset 0
        if self.is_tok("::") {
            // every segment, not just one: `std::vec::Vec` is three
            loop { if !self.is_tok("::") { break; }
                if self.peek_is(1,"<") { break; }
                let _=self.adv();
                if self.i>=self.t.len() { break; }
                let w=tok_word(&self.t[self.i]); if w.len()==0 { break; }
                let _=self.adv(); n.push('$'); n.push_str(&w); }
            // the LAST segment is the type; a std path is transparent (`std$vec$Vec` is `Vec`)
            { let segs=split_dollar(&n); if segs.len()>1 { let last=String::from(segs[segs.len()-1].as_str());
                if !self.sknown(&n) && !self.enames_has(&n) { n=last; } } } }
        // P-MC-GLOBTYPE (REFUTED here, kept so the next rung does not retry it): rewriting a glob-imported
        // type name to the declaring module's name (`Arm` -> `oast$Arm`) fixes the field/element lookups
        // that key on the name, and the composed unit advanced 9.2% -> 11.4% -- but it REGRESSED a
        // module's own enum literal (`E::M(v)` inside the declaring module became "cannot find enum E")
        // and took the error count 26 -> 57. The lookups are what should become glob-aware, not the
        // recorded name. gres_ty below is kept: it is correct and unused until that rung.
        else if self.mpfx.len()>0 { let q=self.qual(&n); if self.sknown(&q) || self.enames_has(&q) { n=q; } }
        // a std smart pointer is transparent -- but ONLY when the program has not declared a type of that
        // name (a user `struct Rc { w, h }` must win, or its fields vanish)
        if (n.as_str()=="Box" || n.as_str()=="Rc" || n.as_str()=="Arc") && !self.sknown(&n) && !self.enames_has(&n) {
            if self.is_tok("<") { let _=self.adv(); let inner=self.parse_tyname();
                
                
                if self.at_gt() { let _=self.eat_gt(); }
                return inner; }
            return String::from("int"); }
        if n.as_str()=="String" && !self.sknown(&n) && !self.enames_has(&n) { return String::from("str"); }
        { let mut hit=false; let mut ai=0;
          while ai<self.aliasn.len(){ if self.aliasn[ai].as_str()==n.as_str() { n=String::from(self.aliast[ai].as_str()); hit=true; ai=self.aliasn.len(); } else { ai=ai+1; } }
          if hit && self.is_tok("<") { self.skip_generics(); return n; } }
        if self.is_tok("<") {
            if self.enames_has(&n) { self.skip_generics(); return n; }
            // more than one argument (or an empty/lifetime-only list) -> keep the base name and skip the group
            { let mut d=0; let mut j=self.i; let mut commas=0;
              while j<self.t.len(){
                  if tk_is(&self.t[j].k,"<") { d=d+1; }
                  else if tk_is(&self.t[j].k,">") { d=d-1; if d==0 { j=self.t.len(); continue; } }
                  else if tk_is(&self.t[j].k,">=") { d=d-1; if d==0 { j=self.t.len(); continue; } }
                  else if tk_is(&self.t[j].k,">>") { d=d-2; if d<=0 { j=self.t.len(); continue; } }
                  else if d==1 && tk_is(&self.t[j].k,",") { commas=commas+1; }
                  j=j+1; }
              if commas>0 { self.skip_generics(); return n; } }
            let _=self.adv();
            if self.at_gt() { let _=self.eat_gt(); return n; }
            let inner=self.parse_tyname();
            
            
            if self.at_gt() { let _=self.eat_gt(); }
            n.push('$'); n.push_str(&inner); }
        n }
    fn expr(&mut self)->E{
        if self.is_word("macro_rules") { self.read_macro(); return self.expr(); }
        if self.is_tok("}") || self.at_end() { let l=if self.i<self.t.len() { self.t[self.i].lo } else { 0 }; return E::Int(0, l, l); }
        if self.is_tok("return") { let lo=self.lo(); let _=self.adv();
            let v=if self.is_tok(";") || self.is_tok("}") { E::Int(0,lo,lo) } else { self.cmp_e() };
            let hi=crate::oast::e_hi(&v); let r=E::RetE(Box::new(v), lo, hi);
            if self.is_tok(";") { let _=self.adv(); if self.is_tok("}") || self.at_end() { return r; } let rest=self.expr(); let b=crate::oast::e_hi(&rest); return E::Seq(Box::new(r), Box::new(rest), lo, b); }
            return r; }
        // C-BLOCKSTMT: a bare `{ .. }` in STATEMENT position is a statement, and the statements after it
        // continue the enclosing block (rustc). Our block-form test knew `if`/`while`/`match` and not a
        // plain block, so everything after one was read as if the enclosing block had ended -- which is how
        // a comment-free `{ .. }` in olex's lexer derailed the whole composed unit. A statement that BEGAN
        // with `{` is the precise test; an expression that merely CONTAINS a block is untouched.
        let stmt0=self.i;
        let e=self.cmp_e();
        let was_block=stmt0<self.t.len() && tk_is(&self.t[stmt0].k, "{");
        if self.is_tok(";") { let _=self.adv(); if self.is_tok("}") || self.at_end() { return e; } let r=self.expr(); let a=crate::oast::e_lo(&e); let b=crate::oast::e_hi(&r); return E::Seq(Box::new(e), Box::new(r), a, b); }
        if (block_form(&e) || was_block) && !self.is_tok("}") && !self.at_end() { let r=self.expr(); let a=crate::oast::e_lo(&e); let b=crate::oast::e_hi(&r); return E::Seq(Box::new(e), Box::new(r), a, b); }
        e }
    fn cmp_e(&mut self)->E{ let l=self.or_e();
        { let mut op=String::from("");
          // `+=` is ONE token: otherwise `add()` consumes the `+` before this layer ever sees it
          if self.is_tok("+=") { op=String::from("+"); }
          else if self.is_tok("-=") { op=String::from("-"); }
          else if self.is_tok("*=") { op=String::from("*"); }
          else if self.is_tok("/=") { op=String::from("/"); }
          if op.len()>0 { let _=self.adv(); let v=self.or_e();
            let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&v);
            // `l op= v` == `l = l op v`; the place is re-parsed from the same tokens, which is sound because
            // our places (a local, a field, an index) have no side effects
            let lhs2=self.reparse_place(&l, a, b);
            let rhs=E::Bin(op, Box::new(l), Box::new(v), a, b);
            return E::AssignTo(Box::new(lhs2), Box::new(rhs), a, b); } }
        if self.is_tok("=") { let _=self.adv(); let v=self.or_e(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&v); return E::AssignTo(Box::new(l), Box::new(v), a, b); } l }
    fn or_e(&mut self)->E{ let mut l=self.and_e(); while self.is_tok("||") { let _=self.adv(); let r=self.and_e(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from("||"),Box::new(l),Box::new(r),a,b); } l }
    fn and_e(&mut self)->E{ let mut l=self.rel_e(); while self.is_tok("&&") { let _=self.adv(); let r=self.rel_e(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); l=E::Bin(String::from("&&"),Box::new(l),Box::new(r),a,b); } l }
    fn rel_e(&mut self)->E{ let l=self.bor_e(); let ops=["<","<=",">",">=","==","!="]; let mut oi=0; while oi<ops.len(){ if self.is_tok(ops[oi]) { let _=self.adv(); let r=self.bor_e(); let a=crate::oast::e_lo(&l); let b=crate::oast::e_hi(&r); return E::Bin(String::from(ops[oi]),Box::new(l),Box::new(r),a,b); } oi=oi+1; } l }
}
pub fn split_commas(s:&str)->Vec<String>{ let b=s.as_bytes(); let mut o:Vec<String>=Vec::new(); let mut st=0; let mut i=0;
    while i<=b.len(){ if i==b.len() || b[i]==44 { if i>st { o.push(String::from(&s[st..i])); } st=i+1; } i=i+1; } o }
// a shallow clone for the elements of a repeat literal (`[0; 32]`): literals and variables are enough, and
// anything else is reproduced as 0 rather than silently duplicating side effects
pub fn clone_simple(e:&E, lo:u32, hi:u32)->E{
    match e { E::Int(v,_l,_h)=>E::Int(*v, lo, hi),
              E::Bool(b,_l,_h)=>E::Bool(*b, lo, hi),
              E::StrLit(sx,_l,_h)=>E::StrLit(String::from(sx.as_str()), lo, hi),
              E::Var(n,_l,_h)=>E::Var(String::from(n.as_str()), lo, hi),
              // a CAST of something simple is simple -- without this, `['x'; 3]` (a char literal is a
              // cast to `char`) silently became zeros, which is the worst kind of answer
              E::Cast(x,t,_l,_h)=>E::Cast(Box::new(clone_simple(x, lo, hi)), String::from(t.as_str()), lo, hi),
              _=>E::Int(0, lo, hi) } }
fn block_form(e:&E)->bool{ match e { E::Let(_n,_v,b,_l,_h)=>block_form(b), E::LetT(_n,_t,_v,b,_l,_h)=>block_form(b), E::Match(_s,_a,_l,_h)=>true, E::While(_c,_b,_l,_h)=>true, E::If(_c,_t,_f,_l,_h)=>true, _=>false } }
pub fn parse(t:Vec<Tok>)->E{ let mut p=P{globs:Vec::new(),fnames:Vec::new(),lvhit:Vec::new(),loopv:Vec::new(),mscal:Vec::new(),smut:Vec::new(),vfn:Vec::new(),atn:Vec::new(),att:Vec::new(),selfty:String::from(""),usen:Vec::new(),uset:Vec::new(),namesonly:false,perrexp:String::from(""),perrgot:String::from(""),nhyg:0,macs:Vec::new(),depth:0,vnames:Vec::new(),vowner:Vec::new(),sfnames:Vec::new(),sawcopy:false,aliasn:Vec::new(),aliast:Vec::new(),perr:0,badfmt:0,consts:Vec::new(),t:t,i:0,snames:Vec::new(),enames:Vec::new(),clos:Vec::new(),nfor:0,mpfx:String::from(""),mods:Vec::new()}; p.expr() }
// MONOMORPHIZE bound generics. Lowering is where concrete types are known (locals carry recorded struct
// types), so run it once as a DISCOVERY pass -- `crate::olower::mono_name` records every `f|Conc` it routes -- then
// materialize `f$Conc` (ptys/ret substituted, body shared via `dflt`) and let the real lowering find them.
// Lowering is side-effect-free apart from id allocation, so running it twice is sound.
// An unannotated return type is INFERRED from the body: `fn mk(a) { P { a, 2 } }` returns P. Runs before
// monomorphization and typeck, so callers see the real type (without it, a struct-returning fn had to be
// annotated or its callers saw `int` and every field read failed).
// Generic STRUCTS are monomorphized from the type names that actually appear: for a demanded `Wrap$int`
// where `Wrap` is generic, materialize an Sdef with the field types substituted. Iterated to a fixpoint so a
// nested instance (`Wrap<Vec<int>>`) also lands.
pub fn find_fun_ix(fs:&Vec<Fun>, n:&str)->i32{ let mut i=0; while i<fs.len(){ if fs[i].name.as_str()==n { return i as i32; } i=i+1; } 0-1 }
// P-ITEM: a nested item (`fn` / `struct` / `enum` / `impl` / `trait` / `const` / `static` / `type`
// declared INSIDE a function body) is position-independent in Rust -- it captures nothing from the
// enclosing scope -- so it is HOISTED out of the body to the top level before parsing, where the one
// item reader already handles it. The alternative is a second item reader reachable from statement
// position, which would be two readers for one grammar.
fn item_kw(t:&Tok)->i32{
    if tk_is(&t.k,"fn") { return 1; }
    if tk_is(&t.k,"impl") || tk_is(&t.k,"trait") || tk_is(&t.k,"mod") { return 1; }
    if tk_is(&t.k,"struct") || tk_is(&t.k,"enum") { return 2; }
    if tk_is(&t.k,"const") || tk_is(&t.k,"static") || tk_is(&t.k,"type") || tk_is(&t.k,"use") { return 3; }
    0 }
pub fn hoist_items(t:Vec<Tok>)->Vec<Tok>{
    let mut keep:Vec<Tok>=Vec::new(); let mut out:Vec<Tok>=Vec::new();
    // the brace stack records what each open brace OPENS: 1 = a function body (items inside it are
    // hoistable), anything else = an item container (an `impl`/`trait`/`mod`/`struct`/`enum` body, whose
    // items already belong to the one item reader) -- a plain block inherits its enclosing kind
    let mut st:Vec<i32>=Vec::new(); let mut pend:i32=0; let mut i=0; let mut prev_ok=true;
    while i<t.len(){
        if tk_is(&t[i].k,"{") {
            let top=if st.len()>0 { st[st.len()-1] } else { 0 };
            if pend>0 { st.push(pend); pend=0; } else { st.push(top); } }
        else if tk_is(&t[i].k,"}") { if st.len()>0 { let _=st.pop(); } }
        let depth:i32=if st.len()>0 { st[st.len()-1] } else { 0 };
        let kw=item_kw(&t[i]);
        // `const fn` / `unsafe fn` keep their modifier with them; a `const` BLOCK expression does not
        // start a statement with a following `fn`
        let mut k2=kw;
        if kw==3 && i+1<t.len() && tk_is(&t[i+1].k,"fn") { k2=1; }
        if k2>0 { pend=if k2==1 && tk_is(&t[i].k,"fn") { 1 } else if k2==1 { 2 } else { 3 }; }
        if depth==1 && k2>0 && prev_ok {
            // find the item's end
            let mut j=i; let mut end=i;
            if k2==1 {
                // scan to the body brace, then to its match; a `fn` DECLARATION in a trait ends at `;`
                let mut d2:i32=0; let mut started=false;
                while j<t.len(){
                    if tk_is(&t[j].k,"{") { d2=d2+1; started=true; }
                    else if tk_is(&t[j].k,"}") { d2=d2-1; if started && d2==0 { end=j; j=t.len(); continue; } }
                    else if tk_is(&t[j].k,";") && !started && d2==0 { end=j; j=t.len(); continue; }
                    j=j+1; }
                if end==i { end=t.len()-1; } }
            else if k2==2 {
                let mut d2:i32=0; let mut started=false;
                while j<t.len(){
                    if tk_is(&t[j].k,"{") || tk_is(&t[j].k,"(") { d2=d2+1; started=true; }
                    else if tk_is(&t[j].k,"}") || tk_is(&t[j].k,")") { d2=d2-1;
                        if started && d2==0 {
                            // a tuple struct is `struct S(..);` -- take the `;` too
                            if j+1<t.len() && tk_is(&t[j+1].k,";") { end=j+1; } else { end=j; }
                            j=t.len(); continue; } }
                    else if tk_is(&t[j].k,";") && !started && d2==0 { end=j; j=t.len(); continue; }
                    j=j+1; }
                if end==i { end=t.len()-1; } }
            else {
                let mut d2:i32=0;
                while j<t.len(){
                    if tk_is(&t[j].k,"{") || tk_is(&t[j].k,"(") || tk_is(&t[j].k,"[") { d2=d2+1; }
                    else if tk_is(&t[j].k,"}") || tk_is(&t[j].k,")") || tk_is(&t[j].k,"]") { d2=d2-1; }
                    else if tk_is(&t[j].k,";") && d2==0 { end=j; j=t.len(); continue; }
                    j=j+1; }
                if end==i { end=t.len()-1; } }
            let mut q=i;
            while q<=end { out.push(Tok{k:t[q].k.clone(), lo:t[q].lo, hi:t[q].hi}); q=q+1; }
            i=end+1; prev_ok=true; continue; }
        prev_ok = tk_is(&t[i].k,"{") || tk_is(&t[i].k,"}") || tk_is(&t[i].k,";");
        keep.push(Tok{k:t[i].k.clone(), lo:t[i].lo, hi:t[i].hi});
        i=i+1; }
    // the hoisted items go in FIRST: the reader registers const and enum names as it reads them, and a
    // body that uses one must be read after its declaration
    if out.len()==0 { return keep; }
    { let mut q=0; while q<keep.len(){ out.push(Tok{k:keep[q].k.clone(), lo:keep[q].lo, hi:keep[q].hi}); q=q+1; } }
    out }
pub fn parse_prog(t0:Vec<Tok>)->Prog{
    let t=hoist_items(t0);
    let mut p=P{globs:Vec::new(),fnames:Vec::new(),lvhit:Vec::new(),loopv:Vec::new(),mscal:Vec::new(),smut:Vec::new(),vfn:Vec::new(),atn:Vec::new(),att:Vec::new(),selfty:String::from(""),usen:Vec::new(),uset:Vec::new(),namesonly:false,perrexp:String::from(""),perrgot:String::from(""),nhyg:0,macs:Vec::new(),depth:0,vnames:Vec::new(),vowner:Vec::new(),sfnames:Vec::new(),sawcopy:false,aliasn:Vec::new(),aliast:Vec::new(),perr:0,badfmt:0,consts:Vec::new(),t:t,i:0,snames:Vec::new(),enames:Vec::new(),clos:Vec::new(),nfor:0,mpfx:String::from(""),mods:Vec::new()}; let mut funs:Vec<Fun>=Vec::new(); let mut ginit:Vec<E>=Vec::new(); let mut sdefs:Vec<Sdef>=Vec::new(); let mut edefs:Vec<Edef>=Vec::new(); let mut tdefs:Vec<Tdef>=Vec::new(); let mut impls:Vec<ImplRec>=Vec::new();
    // P-ORDER: Rust item names are order-independent -- a function may use a struct declared
    // below it. Our reader builds its name tables as it reads, so the FIRST pass exists only to
    // populate them (its outputs are thrown away) and the second does the real parse. One reader,
    // two passes: the alternative is a second name-only scanner, i.e. two readers for one grammar.
    let mut pass=0;
    let tsave:Vec<Tok>={ let mut v:Vec<Tok>=Vec::new(); let mut q=0; while q<p.t.len(){ v.push(tok_clone(&p.t[q])); q=q+1; } v };
    while pass<2 {
      p.i=0; p.namesonly = pass==0;
      { let mut rt2:Vec<Tok>=Vec::new(); let mut q=0; while q<tsave.len(){ rt2.push(tok_clone(&tsave[q])); q=q+1; } p.t=rt2; }
    loop {
        let progress_at=p.i;
        if p.is_tok("struct") {
          let _=p.adv();
          let ni=p.adv(); let nm=match p.t[ni].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
          let mut sgp:Vec<String>=Vec::new();
          if p.is_tok("<") { let _=p.adv(); loop { if p.at_gt() { let _=p.eat_gt(); break; } if p.at_end() { break; } let gi=p.adv(); match p.t[gi].k.clone() { Tk::Ident(x)=>sgp.push(String::from(x.as_str())), _=>{} } if p.is_tok(",") { let _=p.adv(); } } }
          p.skip_where();                 // `struct G<T> where T: Trait { .. }`
          if p.is_tok("(") {
              let _=p.adv();
              let mut fl:Vec<String>=Vec::new(); let mut ft:Vec<String>=Vec::new();
              let mut ps:Vec<String>=Vec::new(); let mut cargs:Vec<E>=Vec::new();
              let mut fi2=0;
              loop { if p.is_tok(")") { let _=p.adv(); break; } if p.at_end() { break; }
                  let t=p.parse_tyname();
                  fl.push(u32s2(fi2)); ft.push(t);
                  let mut pn=String::from("f"); pn.push_str(&u32s2(fi2));
                  cargs.push(E::Var(String::from(pn.as_str()), 0, 0)); ps.push(pn);
                  fi2=fi2+1; if p.is_tok(",") { let _=p.adv(); } }
              if p.is_tok(";") { let _=p.adv(); }
              let qn=p.qual(&nm);
              { let mut j=String::from(""); let mut k=0; while k<fl.len(){ if k>0 { j.push(','); } j.push_str(fl[k].as_str()); k=k+1; }
                p.snames.push(String::from(nm.as_str())); p.sfnames.push(String::from(j.as_str()));
                if qn.len()!=nm.len() { p.snames.push(String::from(qn.as_str())); p.sfnames.push(j); } }
              let cp=p.sawcopy; p.sawcopy=false;
              let mut pt2:Vec<String>=Vec::new(); { let mut k=0; while k<ft.len(){ pt2.push(String::from(ft[k].as_str())); k=k+1; } }
              let ctor=E::StructLit(String::from(qn.as_str()), cargs, 0, 0);
              funs.push(Fun{pref:Vec::new(), ncap:0, gbounds:Vec::new(), dflt:0-1, name:String::from(qn.as_str()), params:ps, ptys:pt2, ret:String::from(qn.as_str()), gparams:Vec::new(), owner:String::from(""), body:ctor});
              sdefs.push(Sdef{copy:cp, sgp:sgp, name:qn, fields:fl, ftys:ft}); continue; }
          if p.is_tok(";") { let _=p.adv(); let fl:Vec<String>=Vec::new(); let ft:Vec<String>=Vec::new();
              p.snames.push(String::from(nm.as_str())); p.sfnames.push(String::from(""));
              { let q=p.qual(&nm); if q.len()!=nm.len() { p.snames.push(q); p.sfnames.push(String::from("")); } }
              sdefs.push(Sdef{copy:{let c=p.sawcopy; p.sawcopy=false; c}, sgp:sgp, name:p.qual(&nm), fields:fl, ftys:ft}); continue; }
          p.eat("{"); let mut fl:Vec<String>=Vec::new(); let mut ft:Vec<String>=Vec::new();
          loop { if p.is_tok("}") { let _=p.adv(); break; } if p.at_end() { break; }
              let fprog=p.i; let _=fprog;
              let fi=p.adv(); match p.t[fi].k.clone() { Tk::Ident(x)=>fl.push(String::from(x.as_str())), _=>{} }
              let mut tn=String::from("int");
              if p.is_tok(":") { let _=p.adv();
                  let mut wasref=false; if p.is_tok("&") { wasref=true; } let _=&wasref; let wasmut=p.is_tok("&") && p.peek_is(1,"mut");
                  tn=sl_param(&p.parse_tyname());

                  }
              ft.push(tn);
              if p.is_tok(",") { let _=p.adv(); } }
          { let mut j=String::from(""); let mut k=0; while k<fl.len(){ if k>0 { j.push(','); } j.push_str(fl[k].as_str()); k=k+1; }
            p.snames.push(String::from(nm.as_str())); p.sfnames.push(String::from(j.as_str()));
            { let q=p.qual(&nm); if q.len()!=nm.len() { p.snames.push(q); p.sfnames.push(j); } } }
          sdefs.push(Sdef{copy:{let c=p.sawcopy; p.sawcopy=false; c}, sgp:sgp, name:p.qual(&nm), fields:fl, ftys:ft});
        } else
        if p.is_tok("enum") {
          let _=p.adv();
          let ni=p.adv(); let en=match p.t[ni].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
          p.skip_generics(); p.skip_where();
          p.eat("{"); let mut vs:Vec<Vdef>=Vec::new();
          loop { if p.is_tok("}") { let _=p.adv(); break; } if p.at_end() { break; } let vprog=p.i; let _=vprog;
              let vi=p.adv(); let vn=match p.t[vi].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
              let mut np=0; let mut pts:Vec<String>=Vec::new(); let mut fns:Vec<String>=Vec::new();
              // `C { x: i32, y: u8 }` -- a STRUCT variant: a positional payload whose components are named
              if p.is_tok("{") { let _=p.adv();
                  loop { if p.is_tok("}") { let _=p.adv(); break; } if p.at_end() { break; }
                      if p.is_tok(",") { let _=p.adv(); continue; }
                      let fi2=p.adv(); let fn2=match p.t[fi2].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
                      let mut ft2=String::from("");
                      if p.is_tok(":") { let _=p.adv(); ft2=p.parse_tyname(); }
                      fns.push(fn2); pts.push(ft2); np=np+1; } }
              if p.is_tok("(") { let _=p.adv();
                  loop { if p.is_tok(")") { let _=p.adv(); break; } if p.at_end() { break; }
                      let t=p.parse_tyname();
                      // `Some(v)` names a binding, `Add(E, E)` names a TYPE: a known type counts as a type
                      if p.sknown(&t) || p.enames_has(&t) || t.as_str()=="int" || t.as_str()=="str" || t.as_str()=="bool" || crate::olower::vec_elem_of(&t).len()>0 { pts.push(t); } else { pts.push(String::from("")); }
                      np=np+1; if p.is_tok(",") { let _=p.adv(); } } }
              p.vnames.push(String::from(vn.as_str())); p.vowner.push(String::from(en.as_str()));
              { let mut j=String::from(""); let mut k=0; while k<fns.len(){ if k>0 { j.push(','); } j.push_str(fns[k].as_str()); k=k+1; } p.vfn.push(j); }
              vs.push(Vdef{name:vn, npayload:np, ptys:pts, fnames:fns});
              if p.is_tok(",") { let _=p.adv(); } }
          p.enames.push(String::from(en.as_str())); { let q=p.qual(&en); if q.len()!=en.len() { p.enames.push(q); } } edefs.push(Edef{name:p.qual(&en), variants:vs});
        } else
        if p.is_word("macro_rules") { p.read_macro(); } else
        if { let mi={ let w=if p.i<p.t.len() { tok_word(&p.t[p.i]) } else { String::from("") }; p.mac_index(&w) };
             mi>=0 && p.peek_is(1,"!") } {
          // a macro invoked at ITEM level: expand it in place and parse the items it produced
          let w=tok_word(&p.t[p.i]); let mi=p.mac_index(&w) as usize;
          if !p.mac_expand(mi) { let _=p.adv(); let _=p.adv(); if p.perr==0 { p.perr=p.lo()+1; } }
        } else
        if p.is_tok("type") {
          let _=p.adv();
          let ni=p.adv(); let an=match p.t[ni].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
          p.skip_generics();
          p.eat("="); let at=p.parse_tyname();
          loop { if p.at_end() { break; } if p.is_tok(";") { let _=p.adv(); break; } let _=p.adv(); }
          p.aliasn.push(an); p.aliast.push(at);
        } else
        if p.is_tok("use") || p.is_tok("extern") {
          // `use` / `extern crate` carry no meaning here; an `extern "C" { .. }` BLOCK is skipped whole.
          // A `use` path may itself contain braces (`use std::task::{Context, Poll};`), so only `extern` can
          // open a block.
          let was_extern=p.is_tok("extern");
          let _=p.adv();
          let mut braced=false;
          let mut glob=false;
          let mut segs:Vec<String>=Vec::new();
          loop { if p.at_end() { break; }
              if p.is_tok("{") && was_extern { braced=true; break; }
              if p.is_tok(";") { let _=p.adv(); break; }
              if !was_extern { if p.is_tok("*") { glob=true; } else { let w=tok_word(&p.t[p.i]); if w.len()>0 { segs.push(w); } } }
              let _=p.adv(); }
          // `use m::fifteen;` makes `fifteen` name `m$fifteen` -- the alias the module prefixing produced
          if !was_extern && segs.len()>=2 {
              let last=String::from(segs[segs.len()-1].as_str());
              let mut full=String::from(segs[0].as_str());
              let mut q=1; while q<segs.len(){ full.push('$'); full.push_str(segs[q].as_str()); q=q+1; }
              if last.as_str()!="self" && last.as_str()!="*" { p.usen.push(last); p.uset.push(full); } }
          if !was_extern && glob && segs.len()>=1 {
              let mut pfx=String::from(segs[0].as_str()); let mut q3=1;
              while q3<segs.len(){ pfx.push('$'); pfx.push_str(segs[q3].as_str()); q3=q3+1; }
              let mut key=String::from(p.mpfx.as_str()); key.push('|'); key.push_str(pfx.as_str());
              p.globs.push(key); }
          if braced { let mut d=0; loop { if p.at_end() { break; }
                  if p.is_tok("{") { d=d+1; } if p.is_tok("}") { d=d-1; let _=p.adv(); if d==0 { break; } continue; }
                  let _=p.adv(); } }
        } else
        if p.is_tok("const") || p.is_tok("static") {
          let _=p.adv();
          let mut ismut=false;
          if p.is_tok("mut") { let _=p.adv(); ismut=true; }
          let ni=p.adv(); let cn=match p.t[ni].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
          let mut ct=String::from("");
          if p.is_tok(":") { let _=p.adv(); ct=sl_const(&p.parse_tyname()); }
          p.eat("="); let cv=if p.namesonly { p.skip_to_semi() } else { p.expr_no_seq() }; if p.is_tok(";") { let _=p.adv(); }
          if ismut {
              // a MUTABLE static is a global, not a value: the name is recorded so reads become a load
              // and writes a store, and the initializer is kept to run first in main
              p.smut.push(String::from(cn.as_str()));
              let mut gn=String::from("$gstore$"); gn.push_str(&cn);
              let mut ia:Vec<E>=Vec::new(); ia.push(cv);
              ginit.push(E::Call(gn, ia, 0, 1));
          } else {
          p.consts.push(String::from(cn.as_str()));
          funs.push(Fun{pref:Vec::new(), ncap:0, gbounds:Vec::new(), dflt:0-1, name:p.qual(&cn), params:Vec::new(), ptys:Vec::new(), ret:ct, gparams:Vec::new(), owner:String::from(""), body:cv}); }
        } else
        if p.is_tok("#copy") { let _=p.adv(); p.sawcopy=true; } else
        if p.is_tok("unsafe") { let _=p.adv(); } else
        if p.is_tok("mod") {
          // modules are flat PREFIX scopes in the one item loop: `mod M {` pushes `M$`, the matching `}` pops it
          let _=p.adv();
          let ni=p.adv(); let mn=match p.t[ni].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
          p.mods.push(String::from(mn.as_str()));
          let mut np=String::from(p.mpfx.as_str()); np.push_str(&mn); np.push('$'); p.mpfx=np;
          p.eat("{");
        } else
        if p.is_tok("}") && p.mpfx.len()>0 {
          let _=p.adv();
          // drop the innermost `Name$` segment
          let b=p.mpfx.as_bytes(); let mut cut=0; let mut k=0;
          while k+1<b.len() { if b[k]==36 { cut=k+1; } k=k+1; }
          p.mpfx=String::from(&p.mpfx[..cut]);
        } else
        if p.is_tok("trait") {
          let _=p.adv();
          let ni=p.adv(); let trn=match p.t[ni].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
          p.skip_generics();
          if p.is_tok(":") { loop { if p.at_end() { break; } if p.is_tok("{") || p.is_tok("where") { break; } let _=p.adv(); } }
          p.skip_where();
          let mut mns:Vec<String>=Vec::new();
          p.eat("{");
          loop {
            if p.is_tok("}") { let _=p.adv(); break; }
            if p.at_end() { break; }
            if p.is_tok("type") { loop { if p.at_end() { break; } if p.is_tok(";") { let _=p.adv(); break; } let _=p.adv(); } continue; }
            if p.is_tok("const") {
              // an associated CONST: a zero-argument function, like a free `const`
              let _=p.adv();
              let ci=p.adv(); let cn=match p.t[ci].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
              let mut ct=String::from(""); if p.is_tok(":") { let _=p.adv(); ct=sl_const(&p.parse_tyname()); }
              mns.push(String::from(cn.as_str()));
              if p.is_tok("=") { let _=p.adv(); let cv=if p.namesonly { p.skip_to_semi() } else { p.cmp_e() };
                  let mut dn=String::from(trn.as_str()); dn.push_str("$"); dn.push_str(&cn);
                  funs.push(Fun{pref:Vec::new(), ncap:0, gbounds:Vec::new(), dflt:0-1, name:dn, params:Vec::new(), ptys:Vec::new(), ret:ct, gparams:Vec::new(), owner:String::from(trn.as_str()), body:cv}); }
              if p.is_tok(";") { let _=p.adv(); }
              continue; }
            if !p.is_tok("fn") { let _=p.adv(); continue; }
            let _=p.adv();
            let mi=p.adv(); let mn=match p.t[mi].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
            { let mut tgp:Vec<String>=Vec::new(); let mut tgb:Vec<String>=Vec::new(); p.read_generics(&mut tgp, &mut tgb); }
            p.eat("("); let mut ps:Vec<String>=Vec::new(); let mut pt:Vec<String>=Vec::new(); let mut pr:Vec<bool>=Vec::new();
            loop { if p.is_tok(")") { let _=p.adv(); break; } if p.at_end() { break; }
              if p.is_tok("&") { let _=p.adv(); if p.is_tok("mut") { let _=p.adv(); } }
              if p.is_tok("mut") { let _=p.adv(); }
              let pi=p.adv(); let pname=match p.t[pi].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
              let mut tnm=if pname.as_str()=="self" { String::from("Self") } else { String::from("") };
              let mut wasref=false;
              if p.is_tok(":") { let _=p.adv();
                  if p.is_tok("&") { wasref=true; } let wasmut=p.is_tok("&") && p.peek_is(1,"mut");
                  tnm=sl_param(&p.parse_tyname());
                  if wasmut && scalar_ty(&tnm) { let nm0=if ps.len()>0 { String::from(ps[ps.len()-1].as_str()) } else { String::from("") };
                      if nm0.len()>0 { p.mscal.push(nm0); } } }
              ps.push(pname); pt.push(tnm); pr.push(wasref);
              if p.is_tok(",") { let _=p.adv(); } }
            let mut rt=String::from("int");
            if p.is_tok("-") { let _=p.adv(); if p.is_tok(">") { let _=p.adv(); }
              if p.is_tok("(") { let _=p.adv(); let mut nc=0; let mut es:Vec<String>=Vec::new();
                  loop { if p.is_tok(")") { let _=p.adv(); break; } if p.at_end() { break; }
                      if p.is_tok(",") { let _=p.adv(); } else { es.push(p.parse_tyname()); nc=nc+1; } }
                  rt=tup_name(nc as u32, &es); }
              else { let wasmut2=p.is_tok("&") && p.peek_is(1,"mut");
                     rt=sl_param(&p.parse_tyname());
                     if wasmut2 && p.perr==0 { p.perr=p.lo()+1; p.perrexp=String::from("a return type that is not a `&mut` (returning a PLACE is not implemented)"); p.perrgot=p.tok_text(); } } }
            p.skip_where();
            mns.push(String::from(mn.as_str()));
            if p.is_tok("{") {
              // a DEFAULT body: kept as `Trait$m`, the template every implementing type's entry points at
              let _=p.adv(); let body=if p.namesonly { p.skip_body() } else { p.expr() }; p.eat("}");
              { loop { match p.mscal.pop() { Some(nm0)=>{
                    if assigns_to(&body,&nm0) && p.perr==0 {
                        p.perr=p.lo()+1; p.perrexp=String::from("a `&mut` to an aggregate (writing through a `&mut` to a SCALAR needs a place our value model does not have)"); p.perrgot=p.tok_text(); } }, None=>{ break; } } } }
              let mut dn=String::from(trn.as_str()); dn.push_str("$"); dn.push_str(&mn);
              funs.push(Fun{pref:pr, ncap:0, gbounds:Vec::new(), dflt:0-1, name:dn, params:ps, ptys:pt, ret:rt, gparams:Vec::new(), owner:String::from(trn.as_str()), body:body});
            } else { if p.is_tok(";") { let _=p.adv(); } }
          }
          tdefs.push(Tdef{name:trn, mnames:mns});
        } else
        if p.is_tok("impl") {
          let _=p.adv();
          p.skip_generics();
          let ti=p.i;
          let mut tn=sl_param(&p.parse_tyname());
          let mut trn=String::from("");
          let _=ti;
          if p.is_tok("for") { let _=p.adv();
            trn=String::from(tn.as_str());
            let tpos=if p.i<p.t.len() { p.t[p.i].lo } else { 0 };
            tn=sl_param(&p.parse_tyname());
            impls.push(ImplRec{tr:String::from(trn.as_str()), ty:String::from(tn.as_str()), lo:tpos, hi:tpos+1}); }
          p.skip_where();
          p.eat("{");
          p.selfty=String::from(tn.as_str());
          loop {
            if p.is_tok("}") { let _=p.adv(); break; }
            if p.at_end() { break; }
            // P-SELF: an associated TYPE is a per-impl entry -- `type Out = u8;` inside `impl T1 for S`
            // makes `<S as T1>::Out` and `Self::Out` name `u8`
            if p.is_tok("type") { let _=p.adv();
                let ai=p.adv(); let an=match p.t[ai].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
                p.skip_generics();
                if p.is_tok("=") { let _=p.adv(); let av=p.parse_tyname();
                    let mut key=String::from(tn.as_str()); key.push('$'); key.push_str(&an);
                    p.atn.push(key); p.att.push(av); }
                loop { if p.at_end() { break; } if p.is_tok(";") { let _=p.adv(); break; } let _=p.adv(); }
                continue; }
            if p.is_tok("const") {
              let _=p.adv();
              let ci=p.adv(); let cn=match p.t[ci].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
              let mut ct=String::from(""); if p.is_tok(":") { let _=p.adv(); ct=sl_const(&p.parse_tyname()); }
              if ct.as_str()=="Self" { ct=String::from(tn.as_str()); }
              if p.is_tok("=") { let _=p.adv(); let cv=if p.namesonly { p.skip_to_semi() } else { p.cmp_e() };
                  let mut dn=String::from(tn.as_str()); dn.push_str("$"); dn.push_str(&cn);
                  funs.push(Fun{pref:Vec::new(), ncap:0, gbounds:Vec::new(), dflt:0-1, name:dn, params:Vec::new(), ptys:Vec::new(), ret:ct, gparams:Vec::new(), owner:String::from(tn.as_str()), body:cv}); }
              if p.is_tok(";") { let _=p.adv(); }
              continue; }
            if !p.is_tok("fn") { let _=p.adv(); continue; }
            let _=p.adv();
            let mi=p.adv(); let mn=match p.t[mi].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
            let mut gps:Vec<String>=Vec::new();
          { let mut mgbs:Vec<String>=Vec::new(); p.read_generics(&mut gps, &mut mgbs); }
            p.eat("("); let mut ps:Vec<String>=Vec::new(); let mut pt:Vec<String>=Vec::new(); let mut pr:Vec<bool>=Vec::new();
            loop { if p.is_tok(")") { let _=p.adv(); break; } if p.at_end() { break; }
              if p.is_tok("&") { let _=p.adv(); if p.is_tok("mut") { let _=p.adv(); } }
              if p.is_tok("mut") { let _=p.adv(); }
              let pi=p.adv(); let pname=match p.t[pi].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
              let mut tnm=if pname.as_str()=="self" { String::from(tn.as_str()) } else { String::from("") };
              let mut wasref=false;
              if p.is_tok(":") { let _=p.adv();
                  if p.is_tok("&") { wasref=true; } let wasmut=p.is_tok("&") && p.peek_is(1,"mut");
                  tnm=sl_param(&p.parse_tyname());
                  if wasmut && scalar_ty(&tnm) { let nm0=if ps.len()>0 { String::from(ps[ps.len()-1].as_str()) } else { String::from("") };
                      if nm0.len()>0 { p.mscal.push(nm0); } } }
              ps.push(pname); pt.push(tnm); pr.push(wasref);
              if p.is_tok(",") { let _=p.adv(); } }
            let mut rt=String::from("int");
            if p.is_tok("-") { let _=p.adv(); if p.is_tok(">") { let _=p.adv(); }
              if p.is_tok("(") { let _=p.adv(); let mut nc=0; let mut es:Vec<String>=Vec::new();
                  loop { if p.is_tok(")") { let _=p.adv(); break; } if p.at_end() { break; }
                      if p.is_tok(",") { let _=p.adv(); } else { es.push(p.parse_tyname()); nc=nc+1; } }
                  rt=tup_name(nc as u32, &es); }
              else { let wasmut2=p.is_tok("&") && p.peek_is(1,"mut");
                     rt=sl_param(&p.parse_tyname());
                     if wasmut2 && p.perr==0 { p.perr=p.lo()+1; p.perrexp=String::from("a return type that is not a `&mut` (returning a PLACE is not implemented)"); p.perrgot=p.tok_text(); } } }
            p.skip_where();
            p.skip_where();
          p.eat("{"); let body=if p.namesonly { p.skip_body() } else { p.expr() }; p.eat("}");
          // a `&mut` SCALAR parameter that the body WRITES needs a place; one it only reads behaves
          // identically by value, so only the write is refused
          { loop { match p.mscal.pop() { Some(nm0)=>{
                if assigns_to(&body,&nm0) && p.perr==0 {
                    p.perr=p.lo()+1; p.perrexp=String::from("a `&mut` to an aggregate (writing through a `&mut` to a SCALAR needs a place our value model does not have)"); p.perrgot=p.tok_text(); } }, None=>{ break; } } } }
            if rt.as_str()=="Self" { rt=String::from(tn.as_str()); }
            let mut mangled=String::from(tn.as_str()); mangled.push_str("$"); mangled.push_str(&mn);
            funs.push(Fun{pref:pr, ncap:0, gbounds:Vec::new(), dflt:0-1, name:mangled, params:ps, ptys:pt, ret:rt, gparams:gps, owner:String::from(tn.as_str()), body:body});
          }
        } else
        if p.is_tok("fn") {
          let _=p.adv();
          let ni=p.adv(); let nm=match p.t[ni].k.clone() { Tk::Ident(x)=>String::from(x.as_str()), _=>String::from("_") };
          let mut gps:Vec<String>=Vec::new();
          let mut gbs:Vec<String>=Vec::new();
          p.read_generics(&mut gps, &mut gbs);
          p.eat("("); let mut ps:Vec<String>=Vec::new(); let mut pt:Vec<String>=Vec::new(); let mut pr:Vec<bool>=Vec::new();
          loop { if p.is_tok(")") { let _=p.adv(); break; } if p.at_end() { break; }
              let pi=p.adv(); match p.t[pi].k.clone() { Tk::Ident(x)=>ps.push(String::from(x.as_str())), _=>{} }
              let mut tn=String::from("");
              let mut wasref=false;
              if p.is_tok(":") { let _=p.adv();
                  if p.is_tok("&") { wasref=true; } let wasmut=p.is_tok("&") && p.peek_is(1,"mut");
                  tn=sl_param(&p.parse_tyname());
                  if wasmut && scalar_ty(&tn) { let nm0=if ps.len()>0 { String::from(ps[ps.len()-1].as_str()) } else { String::from("") };
                      if nm0.len()>0 { p.mscal.push(nm0); } } }
              pt.push(tn); pr.push(wasref);
              if p.is_tok(",") { let _=p.adv(); } }
          let mut rt=String::from("");   // unannotated -> INFERRED from the body (see infer_rets)
          let _=&rt;
          if p.is_tok("-") { let _=p.adv(); if p.is_tok(">") { let _=p.adv(); }
              if p.is_tok("(") { let _=p.adv(); let mut nc=0; let mut es:Vec<String>=Vec::new();
                  loop { if p.is_tok(")") { let _=p.adv(); break; } if p.at_end() { break; }
                      if p.is_tok(",") { let _=p.adv(); } else { es.push(p.parse_tyname()); nc=nc+1; } }
                  rt=tup_name(nc as u32, &es); }
              else { let wasmut2=p.is_tok("&") && p.peek_is(1,"mut");
                     rt=sl_param(&p.parse_tyname());
                     if wasmut2 && p.perr==0 { p.perr=p.lo()+1; p.perrexp=String::from("a return type that is not a `&mut` (returning a PLACE is not implemented)"); p.perrgot=p.tok_text(); } } }
          { let gpc=crate::oast::vc(&gps); p.read_where(&gpc, &mut gbs, &gpc); }
          p.eat("{"); let body=if p.namesonly { p.skip_body() } else { p.expr() }; p.eat("}");
          // a `&mut` SCALAR parameter that the body WRITES needs a place; one it only reads behaves
          // identically by value, so only the write is refused
          { loop { match p.mscal.pop() { Some(nm0)=>{
                if assigns_to(&body,&nm0) && p.perr==0 {
                    p.perr=p.lo()+1; p.perrexp=String::from("a `&mut` to an aggregate (writing through a `&mut` to a SCALAR needs a place our value model does not have)"); p.perrgot=p.tok_text(); } }, None=>{ break; } } } }
          { let nm2=if p.mpfx.len()==0 && nm.as_str()=="main" { String::from("user_main") } else { p.qual(&nm) };
            p.fnames.push(String::from(nm2.as_str()));
          { let mut used:Vec<String>=Vec::new();
            let mut k=0; while k<pt.len(){ if fn_bounded(&gps,&gbs,pt[k].as_str()) { used.push(String::from(pt[k].as_str())); pt[k]=String::from("Fn"); } k=k+1; }
            if used.len()>0 {
                let mut ngps:Vec<String>=Vec::new(); let mut ngbs:Vec<String>=Vec::new();
                let mut g=0; while g<gps.len(){
                    let mut drop_it=false; let mut u=0;
                    while u<used.len(){ if used[u].as_str()==gps[g].as_str() { drop_it=true; u=used.len(); } else { u=u+1; } }
                    if !drop_it { ngps.push(String::from(gps[g].as_str()));
                                  ngbs.push(if g<gbs.len() { String::from(gbs[g].as_str()) } else { String::from("") }); }
                    g=g+1; }
                gps=ngps; gbs=ngbs; } }
          funs.push(Fun{pref:pr, ncap:0, gbounds:gbs, dflt:0-1, name:nm2, params:ps, ptys:pt, ret:rt, gparams:gps, owner:String::from(""), body:body}); }
        } else { break; }
        // NO-PROGRESS GUARD: any iteration that consumed nothing would loop forever (`None::<String>` hung the
        // compiler). Report it and step over one token instead.
        if p.i==progress_at { if p.perr==0 { p.perr=p.lo()+1; p.perrexp=String::from("an item"); p.perrgot=p.tok_text(); } let _=p.adv(); if p.at_end() { break; } }
      }
      // a file that is only ITEMS has no trailing expression: its program is a call to `main`, the way Rust
      // works. A trailing expression still wins, so every program written in our own dialect is unaffected.
  
      if pass==0 { funs=Vec::new(); ginit=Vec::new(); sdefs=Vec::new(); edefs=Vec::new(); tdefs=Vec::new(); impls=Vec::new();
                   p.perr=0; p.perrexp=String::from(""); p.perrgot=String::from(""); p.badfmt=0;
                   loop { match p.clos.pop() { Some(_f)=>{}, None=>{ break; } } } }
      pass=pass+1; }
    let m=if p.at_end() {
        let ar:Vec<E>=Vec::new();
        let ux=find_fun_ix(&funs,"user_main");
        if ux>=0 {
            let call=E::Call(String::from("user_main"), ar, 0, 0);
            let rt=String::from(funs[ux as usize].ret.as_str());
            // a `()`-returning main DISCARDS its value: the process exits 0. A `Result`-returning main
            // exits with the VARIANT TAG -- our prelude's Result is Ok=0, Err=1, which is exactly rustc's
            // 0-on-Ok / 1-on-Err. Any other aggregate return is not an exit code at all, so it exits 0.
            if rt.len()==0 || rt.as_str()=="()" || rt.as_str()=="unit" { E::Seq(Box::new(call), Box::new(E::Int(0,0,0)), 0, 0) }
            else if rt.as_str()=="Result" { let mut ra:Vec<E>=Vec::new(); ra.push(call); E::Call(String::from("rexit"), ra, 0, 0) }
            else if find_sname(&sdefs,&rt) || rt.as_str()=="Option" { E::Seq(Box::new(call), Box::new(E::Int(0,0,0)), 0, 0) }
            else { call }
        } else { E::Int(0,0,0) } } else { p.expr() };
    // the global initializers run before anything else in the program
    let m=if ginit.len()>0 { let mut acc=m; loop { match ginit.pop() { Some(g)=>{ acc=E::Seq(Box::new(g), Box::new(acc), 0, 1); }, None=>{ break; } } } acc } else { m };
    // tokens left over after the program means the input was not understood
    let tr_pos=if p.i+1 < p.t.len() { p.t[p.i].lo } else { 0 };
    // MATERIALIZE trait defaults: for every `impl Trait for Type` and every trait method the impl did not
    // provide, give Type its own entry pointing at the trait's default body (dflt), with `self` typed as Type.
    { let mut ii=0; while ii<impls.len(){
        let ti=tridx(&tdefs, impls[ii].tr.as_str());
        if ti>=0 { let td=&tdefs[ti as usize]; let mut mi=0; while mi<td.mnames.len(){
            let mut own=String::from(impls[ii].ty.as_str()); own.push_str("$"); own.push_str(td.mnames[mi].as_str());
            if find_fun_ix(&funs,&own)<0 {
              let mut dn=String::from(impls[ii].tr.as_str()); dn.push_str("$"); dn.push_str(td.mnames[mi].as_str());
              let dx=find_fun_ix(&funs,&dn);
              if dx>=0 { let d=&funs[dx as usize];
                let mut ps:Vec<String>=Vec::new(); let mut pt:Vec<String>=Vec::new(); let mut pr:Vec<bool>=Vec::new(); let mut k=0;
                while k<d.params.len(){ ps.push(String::from(d.params[k].as_str()));
                  if k==0 && d.params[k].as_str()=="self" { pt.push(String::from(impls[ii].ty.as_str())); }
                  else { pt.push(String::from(d.ptys[k].as_str())); } k=k+1; }
                let rt0=String::from(d.ret.as_str());
                let rt=if rt0.as_str()=="Self" { String::from(impls[ii].ty.as_str()) } else { rt0 };
                funs.push(Fun{pref:pr, ncap:0, gbounds:Vec::new(), dflt:dx, name:own, params:ps, ptys:pt, ret:rt, gparams:Vec::new(), owner:String::from(impls[ii].ty.as_str()), body:E::Int(0,0,0)});
              }
            }
            mi=mi+1; } }
        ii=ii+1; } }
    // MOVE the lifted closures into the program (popping, since `E` has no Clone -- order is irrelevant,
    // a closure's identity rides in its NAME `$closN`)
    loop { match p.clos.pop() { Some(f)=>{ funs.push(f); }, None=>{ break; } } }
    Prog{perrexp:String::from(p.perrexp.as_str()), perrgot:String::from(p.perrgot.as_str()), perr:p.perr, badfmt:p.badfmt, trailing:tr_pos, sdefs:sdefs, edefs:edefs, funs:funs, tdefs:tdefs, impls:impls, main:m}
}
}
mod otypeck {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::omono::*;
    use crate::oborrowck::*;
// otypeck -- the TYPE SYSTEM: the tag lattice, type-name resolution, expression checking, call checks
pub fn env_get(e:&crate::oast::Env, nm:&str)->i32{ let mut i=e.n.len(); while i>0 { i=i-1; if e.n[i].as_str()==nm { return e.t[i]; } } -1 }
// the type NAME a tag stands for, for the one builtin-method table
// C-VECCOMPAT: `Vec::new()` carries no element type, so it answers the untyped Vec tag 201 and is
// compatible with ANY typed Vec (1000+i). The builtin-argument check already knew this and skipped a 201
// argument, but every other place that compares two tags for compatibility used a plain !=, so assigning
// Vec::new() into a Vec<T> place was rejected. The rule is stated once here and used at every comparison.
// C-BITNOT: rustc's `!` is the logical not on a bool and the BITWISE not on an integer. This tier has
// one scalar width (C1), so an integer `!` is faithful only where the high bits cannot escape -- the mask
// idiom `x & !m`. It is admitted under a bitwise operator and stays REFUSED elsewhere, so a directly
// printed `!7u32` keeps its clean refusal instead of answering -8 where rustc says 4294967288.
pub fn is_bitop(op:&str)->bool{ op=="&" || op=="|" || op=="^" }
pub fn tyeq(a:i32, b:i32)->bool{
    if a==b { return true; }
    if a==201 && b>=1000 { return true; }
    if b==201 && a>=1000 { return true; }
    false }
pub fn tagty(t:i32)->String{ if t==4 { String::from("bstr") } else if t>=2000 { String::from("arr") } else if t==2 { String::from("str") } else if t==200 { String::from("arr") } else if t==201 || t>=1000 { String::from("Vec") } else { String::from("") } }
pub fn tyname2(t:i32, ss:&Vec<Sdef>)->String{ if t==4 { return String::from("&[u8]"); } if t>=2000 { return String::from("[str]"); } if t==0-3 {String::from("<generic>")} else if t==0 {String::from("int")} else if t==1 {String::from("bool")} else if t==2 {String::from("str")} else if t==3 {String::from("f64")} else if t==200 { String::from("[int]") } else if t==201 { String::from("Vec") } else if t>=1000 { String::from("Vec<..>") } else if t==900 { String::from("Fn") } else if t>=800 { String::from("closure") } else if t>=400 { String::from("dyn") } else if t>=300 { String::from("tuple") } else if t>=100 { String::from("enum") } else if t>=10 { let i=(t-10) as usize; if i<ss.len() { String::from(ss[i].name.as_str()) } else { String::from("struct") } } else {String::from("{unknown}")} }
pub fn tyname(t:i32)->String{ if t==0 {String::from("int")} else if t==1 {String::from("bool")} else if t>=10 {String::from("struct")} else {String::from("{unknown}")} }
pub fn etag(ei:i32)->i32{ 100+ei }
pub fn is_gp(gps:&Vec<String>, n:&str)->bool{ let mut i=0; while i<gps.len(){ if gps[i].as_str()==n { return true; } i=i+1; } false }
pub fn tup_arity(tn:&str)->i32{ let b=tn.as_bytes(); if b.len()<4 { return -1; } if b[0]!=116 || b[1]!=117 || b[2]!=112 { return -1; } let mut v=0; let mut i=3; let mut any=false; while i<b.len(){ if b[i]==36 { break; } if b[i]<48 || b[i]>57 { return -1; } v=v*10+((b[i]-48) as i32); any=true; i=i+1; } if !any { return -1; } v }
// `dyn Trait` is tag 400+<trait index>: a value whose concrete type is not known statically, only its trait.
pub fn dyn_name_trait(tn:&str)->String{ let b=tn.as_bytes(); if b.len()<5 { return String::from(""); } if b[0]==100 && b[1]==121 && b[2]==110 && b[3]==36 { return String::from(&tn[4..]); } String::from("") }
pub fn dyn_tag_of(tz:&Vec<Tdef>, tn:&str)->i32{ let tr=dyn_name_trait(tn); if tr.len()==0 { return 0-1; } let i=crate::oast::tridx(tz,&tr); if i<0 { return 0-1; } 400+i }
// A typed vector is ONE rule: tag = 1000 + its ELEMENT's tag, so Vec<int>=1000, Vec<Sq>=1010+i,
// Vec<dyn Area>=1400+i, and the element is recovered by subtraction. (The first cut special-cased
// Vec<dyn T> at 600+i, which did not generalize to a vector of structs.)
pub fn elem_tag_of(ss:&Vec<Sdef>, ez:&Vec<Edef>, tz:&Vec<Tdef>, tn:&str)->i32{
    // RECURSIVE: a vector's element may itself be a vector (`Vec<Vec<int>>` = 1000 + (1000 + int))
    let v=vec_tag_of(ss,ez,tz,tn); if v>=0 { return v; }
    let d=dyn_tag_of(tz,tn); if d>=0 { return d; }
    let b=tag_of_tyname(ss,tn); if b>=0 { return b; }
    let ei=crate::oast::eidx(ez,tn); if ei>=0 { return 100+ei; }
    0-1 }
pub fn clos_idx_of_name(n:&str)->i32{ let b=n.as_bytes(); if b.len()<6 { return 0-1; } if b[0]!=36 || b[1]!=99 || b[2]!=108 || b[3]!=111 || b[4]!=115 { return 0-1; } let mut v=0; let mut i=5; while i<b.len(){ if b[i]<48 || b[i]>57 { return 0-1; } v=v*10+((b[i]-48) as i32); i=i+1; } v }
pub fn clos_name_of_tag(t:i32)->String{ let mut r=String::from("$clos"); r.push_str(&crate::oast::u32s2((t-800) as u32)); r }
pub fn type_tag(ss:&Vec<Sdef>, ez:&Vec<Edef>, tz:&Vec<Tdef>, tn:&str)->i32{
    if tn.len()==0 { return 0-1; }
    let v=vec_tag_of(ss,ez,tz,tn); if v>=0 { return v; }
    let d=dyn_tag_of(tz,tn); if d>=0 { return d; }
    let b=tag_of_tyname(ss,tn); if b>=0 { return b; }
    let e=crate::oast::eidx(ez,tn); if e>=0 { return 100+e; }
    0-1 }
pub fn vec_tag_of(ss:&Vec<Sdef>, ez:&Vec<Edef>, tz:&Vec<Tdef>, tn:&str)->i32{ let el=crate::olower::vec_elem_of(tn); if el.len()==0 { return 0-1; } let e=elem_tag_of(ss,ez,tz,&el); if e<0 { return 0-1; }
    if e>=1000 { return 201; }
    1000+e }
pub fn tag_of_tyname(ss:&Vec<Sdef>, tn:&str)->i32{ if tn=="bstr" { return 4; } let ta=tup_arity(tn); if ta>=0 { return 300+ta; } if tn=="Fn" { return 900; } if crate::oast::is_int_width(tn) || tn=="char" { return 0; } if tn=="f64" { return 3; } if tn=="int" { 0 } else if tn=="str" { 2 } else if tn=="Vec" { 201 } else if tn=="bool" { 1 } else if tn=="arr" { 200 } else { let i=sidx_of(ss,tn); if i<0 { -1 } else { 10+i } } }
pub fn sidx_of(ss:&Vec<Sdef>, n:&str)->i32{ let mut i=0; while i<ss.len(){ if ss[i].name.as_str()==n { return i as i32; } i=i+1; }
    { let nm=crate::oast::snames_of(ss); let k=crate::oast::sfx_unique(&nm,n); if k>=0 { return k; } }
    -1 }
// typeck: returns the type tag (0=int,1=bool,-1=err) and PUSHES spanned diagnostics
pub fn check(e:&E, env:&mut crate::oast::Env, ds:&mut Vec<Diag>)->i32{ check_s(e, env, ds, &empty_sdefs()) }
pub fn empty_sdefs()->Vec<Sdef>{ let o:Vec<Sdef>=Vec::new(); o }
pub fn check_s(e:&E, env:&mut crate::oast::Env, ds:&mut Vec<Diag>, ss:&Vec<Sdef>)->i32{ check_f(e, env, ds, ss, &empty_funs()) }
pub fn empty_funs()->Vec<Fun>{ let o:Vec<Fun>=Vec::new(); o }
pub fn check_f(e:&E, env:&mut crate::oast::Env, ds:&mut Vec<Diag>, ss:&Vec<Sdef>, fz:&Vec<Fun>)->i32{ let ez:Vec<Edef>=Vec::new(); let tz:Vec<Tdef>=Vec::new(); check_e(e,env,ds,ss,fz,&ez,&tz) }
pub fn check_e(e:&E, env:&mut crate::oast::Env, ds:&mut Vec<Diag>, ss:&Vec<Sdef>, fz:&Vec<Fun>, ez:&Vec<Edef>, tz:&Vec<Tdef>)->i32{
    match e {
        E::StrLit(_v,_l,_h)=>2,
        E::BreakE(_l,_h)=>0-1,
        E::Ref(x,_m,_l,_h)=>check_e(x,env,ds,ss,fz,ez,tz),
        E::Cast(x,t,l,h)=>{ let tx=check_e(x,env,ds,ss,fz,ez,tz);
            if crate::oast::is_int_width(t) || t.as_str()=="int" || t.as_str()=="char" { if tx!=0 && tx!=1 && tx>=0 && tx!=0-3 { let mut m=String::from("cannot cast "); m.push_str(&tyname2(tx,ss)); m.push_str(" to "); m.push_str(t); ds.push(crate::oerrors::err(m,*l,*h)); } return 0; }
            // only NUMERIC casts are implemented: an `as` to a trait object or struct would need a real
            // coercion, and pretending otherwise produced a garbage value and a segfault at runtime
            { let mut m=String::from("`as` to `"); m.push_str(t); m.push_str("` is not implemented (only numeric casts)"); ds.push(crate::oerrors::err(m,*l,*h)); }
            0 },
        E::ContE(_l,_h)=>0-1,
        E::Try(x,l,h)=>{ let tx=check_e(x,env,ds,ss,fz,ez,tz);
            if tx<100 || tx>=300 { if tx>=0 { let mut m=String::from("`?` needs an enum, found "); m.push_str(&tyname2(tx,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } return 0; }
            0 },
        E::ClosRef(n,_c,_l,_h)=>{ let i=clos_idx_of_name(n); if i<0 { 0 } else { 800+i } },
        E::LetT(n,t,v,b,l,h)=>{ let tv=check_e(v,env,ds,ss,fz,ez,tz);
            let mut td=type_tag(ss,ez,tz,t);
            if td<0 { let mut m=String::from("unknown type `"); m.push_str(t); m.push_str("`"); ds.push(crate::oerrors::err(m,*l,*h)); td=tv; }
            // a TRAIT OBJECT carries a tag plus a payload, so it can only be made from a value whose
            // concrete type we tag: a struct or an enum. `impl Trait for i32` then `&x as &dyn Trait`
            // handed the raw int to a tag dispatch and segfaulted -- refuse it instead.
            // P-BOX: a trait object is a {tag, payload} record, and with P-NOM the payload's concrete
            // type has a NAME, so a primitive can carry the tag too. What must still hold is that an
            // impl EXISTS for that type -- lowering refuses the untagged case, where no name matched.
            if td>=400 && td<800 && tv>=0 && !(tv>=10 && tv<200) && !(tv>=0 && tv<=2) {
                let mut m=String::from("a trait object from "); m.push_str(&tyname2(tv,ss));
                m.push_str(" is not implemented (that type carries no tag)");
                ds.push(crate::oerrors::err(m,*l,*h)); }
            env.n.push(String::from(n.as_str())); env.t.push(td);
            let rb=check_e(b,env,ds,ss,fz,ez,tz); env.n.pop(); env.t.pop(); rb },
        E::TupLit(parts,_l,_h)=>{ let mut k=0; while k<parts.len(){ let _=check_e(&parts[k],env,ds,ss,fz,ez,tz); k=k+1; } 300+(parts.len() as i32) },
        E::TupIdx(b,ix,l,h)=>{ let tb=check_e(b,env,ds,ss,fz,ez,tz);
            // an UNKNOWN receiver (a lifted closure's parameter, say) cannot be checked statically; the index
            // is used directly by lowering, so permit it rather than reject valid code
            if tb==0-3 { return 0; }
            // a TUPLE STRUCT's `.0` is a field read: its fields are named "0", "1", ...
            if tb>=10 && tb<100 { let si=(tb-10) as usize;
                if si<ss.len() { let sn=String::from(ss[si].name.as_str()); let fname=u32s2(*ix);
                    let fi=crate::oast::sfield_idx(ss,&sn,&fname);
                    if fi<0 { let mut m=String::from("no field `"); m.push_str(&fname); m.push_str("` on `"); m.push_str(&sn); m.push_str("`"); ds.push(crate::oerrors::err(m,*l,*h)); return 0; }
                    let fty=crate::oast::sfield_ty(ss,&sn,&fname); let ft=type_tag(ss,ez,tz,&fty); if ft>=0 { return ft; } return 0; } }
            if tb<300 { let mut m=String::from("`."); m.push_str(&crate::oast::u32s2(*ix)); m.push_str("` needs a tuple, found "); m.push_str(&tyname2(tb,ss)); ds.push(crate::oerrors::err(m,*l,*h)); return 0; }
            let n=tb-300; if (*ix as i32)>=n { let mut m=String::from("tuple has "); m.push_str(&crate::oast::u32s2(n as u32)); m.push_str(" element(s), index "); m.push_str(&crate::oast::u32s2(*ix)); m.push_str(" is out of range"); ds.push(crate::oerrors::err(m,*l,*h)); }
            0-3 },
        E::RetE(x,_l,_h)=>{ let _=check_e(x,env,ds,ss,fz,ez,tz); 0-1 },
        E::MethodCall(r,m,args,_lt,_ht) if (m.as_str()=="to_string" || m.as_str()=="to_owned") && args.len()==0 =>{ let _=check_e(r,env,ds,ss,fz,ez,tz); 2 },
        // the BUILT-IN method set lives in ONE place (`crate::olower::builtin_method`); this arm asks it rather
        // than keeping a second copy of the names, and decides only the RESULT type
        E::MethodCall(r,m,args,l,h)=>{
            let tr=check_e(r,env,ds,ss,fz,ez,tz);
            let mut k=0; while k<args.len(){ let _=check_e(&args[k],env,ds,ss,fz,ez,tz); k=k+1; }
            // C-WRAPPING: an INT receiver has no name in the tag scheme (tagty answers ""), so the
            // wrapping methods are admitted here and the WIDTH gate lives in olower, which is the only
            // place that still knows the declared width. A narrower receiver is refused there.
            if (m.as_str()=="wrapping_mul" || m.as_str()=="wrapping_add" || m.as_str()=="wrapping_sub") && args.len()==1 && tr==0 { return 0; }
            let nm2=if tr>=100 && tr<200 { let i=(tr-100) as usize; if i<ez.len() { String::from(ez[i].name.as_str()) } else { tagty(tr) } } else { tagty(tr) };
            let eclone = m.as_str()=="clone" && args.len()==0 && tr>=100 && tr<200;      // C-ENUMCLONE
            if eclone || crate::olower::builtin_method(&nm2, m, args.len()).len()>0 {
                if m.as_str()=="get" && tr>=1000 { return tr-1000; }
                if m.as_str()=="clone" && args.len()==0 && tr>=100 && tr<200 { return tr; }   // C-ENUMCLONE
                if (m.as_str()=="as_ref" || m.as_str()=="as_mut") && args.len()==0 { return tr; }   // C-ASREF: Box is transparent
                if (m.as_str()=="as_bytes" || m.as_str()=="chars") && tr==2 { return 4; }   // a str read element-wise (C-CHARS)
                if m.as_str()=="len" && tr==4 { return 0; }
    
                // an adaptor answers the same sequence; a predicate answers a bool; everything else an int
                if m.as_str()=="iter" || m.as_str()=="into_iter" || m.as_str()=="iter_mut" || m.as_str()=="collect"
                   || m.as_str()=="cloned" || m.as_str()=="copied" || m.as_str()=="to_vec" || m.as_str()=="clone"
                   || m.as_str()=="map" || m.as_str()=="filter" || m.as_str()=="rev" || m.as_str()=="take" || m.as_str()=="skip" { return tr; }
                { let rn2=if tr>=100 && tr<200 { let i=(tr-100) as usize; if i<ez.len() { String::from(ez[i].name.as_str()) } else { String::from("") } } else { String::from("") };
                  if rn2.as_str()=="Option" || rn2.as_str()=="Result" {
                      if m.as_str()=="map" { return tr; }
                      if m.as_str()=="is_some" || m.as_str()=="is_none" || m.as_str()=="is_ok" || m.as_str()=="is_err" { return 1; }
                      return 0; } }
                if tr==2 {
                    if m.as_str()=="push" || m.as_str()=="push_str" { return 0; }   // C-PUSHUNIT: these answer ()
                    if m.as_str()=="parse" { let ri=crate::oast::eidx(ez,"Result"); if ri>=0 { return 100+ri; } return 0; }
                    if m.as_str()=="find" { let oi=crate::oast::eidx(ez,"Option"); if oi>=0 { return 100+oi; } return 0; }
                    if m.as_str()=="is_empty" || m.as_str()=="contains" || m.as_str()=="starts_with" || m.as_str()=="ends_with" { return 1; }
                    if m.as_str()=="len" { return 0; }
                    return 2; }
                if m.as_str()=="enumerate" { return 1000+302; }
                // the Option-returning consumers answer the prelude's `Option`, which `{:?}` can print
                if m.as_str()=="max" || m.as_str()=="min" || m.as_str()=="last" || m.as_str()=="first"
                   || m.as_str()=="next" || m.as_str()=="position" || m.as_str()=="pop" { let oi=crate::oast::eidx(ez,"Option"); if oi>=0 { return 100+oi; } return 0; }
                if m.as_str()=="any" || m.as_str()=="all" || m.as_str()=="is_empty" || m.as_str()=="contains" { return 1; }
                0
            } else {
                // the receiver's TYPE NAME, then look up Type$m
                if tr>=400 { let ti=(tr-400) as usize;
                    if ti<tz.len() { let td=&tz[ti]; let mut found=false; let mut mi=0;
                        while mi<td.mnames.len(){ if td.mnames[mi].as_str()==m.as_str() { found=true; } mi=mi+1; }
                        if !found { let mut msg=String::from("trait `"); msg.push_str(td.name.as_str()); msg.push_str("` has no method `"); msg.push_str(m); msg.push_str("`"); ds.push(crate::oerrors::err(msg,*l,*h)); return 0; }
                        // the return type is the trait's own declaration, shared by every implementor
                        let mut tq=String::from(td.name.as_str()); tq.push_str("$"); tq.push_str(m);
                        let mut rr=String::from(""); let mut q=0; while q<fz.len(){ if fz[q].name.as_str()==tq.as_str() { rr=String::from(fz[q].ret.as_str()); } q=q+1; }
                        if rr.len()==0 { let mut q2=0; while q2<fz.len(){ if fz[q2].owner.len()>0 && fz[q2].name.len()>m.len() { let mut cand=String::from(fz[q2].owner.as_str()); cand.push_str("$"); cand.push_str(m); if fz[q2].name.as_str()==cand.as_str() { rr=String::from(fz[q2].ret.as_str()); } } q2=q2+1; } }
                        let t3=tag_of_tyname(ss,&rr); if t3>=0 { return t3; } let ei2=crate::oast::eidx(ez,&rr); if ei2>=0 { return 100+ei2; } return 0; }
                    return 0; }
                let rn = if tr>=10 && tr<100 { let i=(tr-10) as usize; if i<ss.len() { String::from(ss[i].name.as_str()) } else { String::from("") } }
                         else if tr>=100 { let i=(tr-100) as usize; if i<ez.len() { String::from(ez[i].name.as_str()) } else { String::from("") } }
                         else { String::from("") };
                if rn.len()==0 { if tr>=0 { let mut msg=String::from("no method `"); msg.push_str(m); msg.push_str("` on "); msg.push_str(&tyname2(tr,ss)); ds.push(crate::oerrors::err(msg,*l,*h)); } return -1; }
                let mut mangled=String::from(rn.as_str()); mangled.push_str("$"); mangled.push_str(m);
                let fi=crate::oast::find_method(fz,&rn,&mangled);
                if fi<0 { let mut msg=String::from("no method `"); msg.push_str(m); msg.push_str("` on `"); msg.push_str(&rn); msg.push_str("`"); ds.push(crate::oerrors::err(msg,*l,*h)); return -1; }
                let f=&fz[fi as usize];
                let want=(f.params.len() as i32)-1;
                if want!=(args.len() as i32) { let mut msg=String::from("method `"); msg.push_str(m); msg.push_str("` takes "); msg.push_str(&crate::oast::u32s2(want as u32)); msg.push_str(" argument(s) but "); msg.push_str(&crate::oast::u32s2(args.len() as u32)); msg.push_str(" supplied"); ds.push(crate::oerrors::err(msg,*l,*h)); }
                let t2=type_tag(ss,ez,tz,&f.ret); if t2>=0 { t2 } else { 0 }
            } },
        E::AssignTo(pl,v,l,h)=>{ let tp=check_e(pl,env,ds,ss,fz,ez,tz); let tv2=check_e(v,env,ds,ss,fz,ez,tz); if tp>=0 && tv2>=0 && !tyeq(tp,tv2) { let mut m=String::from("cannot assign "); m.push_str(&tyname2(tv2,ss)); m.push_str(" to "); m.push_str(&tyname2(tp,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } 0 },
        E::ArrLit(xs,l,h)=>{
            // every element must agree; a str array is as valid as an int array (both are 8-byte slots)
            let mut et=0-2; let mut k=0;
            while k<xs.len(){ let tx=check_e(&xs[k],env,ds,ss,fz,ez,tz);
                if tx>=0 { if et==0-2 { et=tx; }
                           else if et!=tx { let mut m=String::from("array elements must agree, found "); m.push_str(&tyname2(tx,ss)); m.push_str(" after "); m.push_str(&tyname2(et,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } }
                k=k+1; }
            if et==2 { return 2000+2; }
            if et==0-2 || et==0 { return 200; }
            if et>0 { return 2000+et; }
            200 },
        E::Index(b,ix,l,h)=>{ let tb=check_e(b,env,ds,ss,fz,ez,tz); let ti=check_e(ix,env,ds,ss,fz,ez,tz);
            if ti!=0 && ti>=0 { let mut m=String::from("index must be int, found "); m.push_str(&tyname2(ti,ss)); ds.push(crate::oerrors::err(m,*l,*h)); }
            if tb<0 { return tb; }                     // UNKNOWN stays unknown -- never invent `int`
            if tb==4 { return 0; }                     // a BYTE VIEW yields a byte
            if tb>=2000 { return tb-2000; }            // a typed ARRAY yields its element type
            if tb==201 { return 0-3; }
            if tb>=1000 { return tb-1000; }            // a typed vector yields its ELEMENT type
            if tb!=200 && tb>=0 { let mut m=String::from("cannot index "); m.push_str(&tyname2(tb,ss)); ds.push(crate::oerrors::err(m,*l,*h)); return -1; }
            0 },
        E::EnumLit(en,v,ps,l,h)=>{
            let ei=crate::oast::eidx(ez,en);
            if ei<0 { let mut m=String::from("cannot find enum `"); m.push_str(en); m.push_str("` in this scope"); ds.push(crate::oerrors::err(m,*l,*h)); return -1; }
            let want=crate::oast::vpayload(ez,en,v);
            if want<0 { let mut m=String::from("no variant `"); m.push_str(v); m.push_str("` on enum `"); m.push_str(en); m.push_str("`"); ds.push(crate::oerrors::err(m,*l,*h)); return -1; }
            if want!=(ps.len() as i32) { let mut m=String::from("variant `"); m.push_str(v); m.push_str("` takes "); m.push_str(&crate::oast::u32s2(want as u32)); m.push_str(" payload(s) but "); m.push_str(&crate::oast::u32s2(ps.len() as u32)); m.push_str(" given"); ds.push(crate::oerrors::err(m,*l,*h)); }
            let mut k=0; while k<ps.len(){ let _=check_e(&ps[k],env,ds,ss,fz,ez,tz); k=k+1; }
            etag(ei) },
        E::Match(sc,arms,l,h)=>{
            let ts=check_e(sc,env,ds,ss,fz,ez,tz);
            let structural={ let mut f=false; let mut k=0; while k<arms.len(){ if arms[k].has_lit || arms[k].has_slit || arms[k].has_guard { f=true; } k=k+1; } f };
            if ts<100 || ts>=300 || structural {
                let mut r=0; let mut k=0;
                while k<arms.len(){ if arms[k].has_guard { let tg=check_e(&arms[k].guard,env,ds,ss,fz,ez,tz);
                        if tg!=1 && tg>=0 { let mut m=String::from("match guard must be bool, found "); m.push_str(&tyname2(tg,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } }
                    r=check_e(&arms[k].body,env,ds,ss,fz,ez,tz); k=k+1; }
                r
            } else {
                if ts<100 { if ts>=0 { let mut m=String::from("`match` needs an enum, found "); m.push_str(&tyname2(ts,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } return -1; }
                let ei=ts-100; let en=if (ei as usize)<ez.len() { String::from(ez[ei as usize].name.as_str()) } else { String::from("") };
                let nv=crate::oast::nvariants(ez,&en); let mut covered=0; let mut haswild=false; let mut rty=-2;
                let mut k=0;
                while k<arms.len(){
                    let a=&arms[k];
                    if a.wild { haswild=true; } else { let vi=crate::oast::vidx(ez,&en,&a.variant); if vi<0 { let mut m=String::from("no variant `"); m.push_str(&a.variant); m.push_str("` on enum `"); m.push_str(&en); m.push_str("`"); ds.push(crate::oerrors::err(m,*l,*h)); } else { covered=covered+1; let np=crate::oast::vpayload(ez,&en,&a.variant); if np!=(a.binds.len() as i32) { let mut m=String::from("variant `"); m.push_str(&a.variant); m.push_str("` binds "); m.push_str(&crate::oast::u32s2(np as u32)); m.push_str(" payload(s) but "); m.push_str(&crate::oast::u32s2(a.binds.len() as u32)); m.push_str(" named"); ds.push(crate::oerrors::err(m,*l,*h)); } } }
                    let mut nb=0; let mut bi=0;
                    while bi<a.binds.len(){ env.n.push(String::from(a.binds[bi].as_str()));
                        let pt=crate::oast::vpayload_ty(ez,&en,&a.variant,bi);
                        let tg=type_tag(ss,ez,tz,&pt); env.t.push(if tg>=0 { tg } else { 0-3 }); nb=nb+1; bi=bi+1; }
                    let at=check_e(&a.body,env,ds,ss,fz,ez,tz);
                    { let mut q=0; while q<nb { env.n.pop(); env.t.pop(); q=q+1; } }
                    if rty==-2 { rty=at; } else if at>=0 && rty>=0 && !tyeq(at,rty) { let mut m=String::from("match arms have incompatible types: "); m.push_str(&tyname2(rty,ss)); m.push_str(" vs "); m.push_str(&tyname2(at,ss)); ds.push(crate::oerrors::err(m,*l,*h)); }
                    k=k+1;
                }
                if !haswild && nv>=0 && covered<nv { let mut m=String::from("non-exhaustive `match` on `"); m.push_str(&en); m.push_str("`: "); m.push_str(&crate::oast::u32s2((nv-covered) as u32)); m.push_str(" variant(s) not covered"); ds.push(crate::oerrors::err(m,*l,*h)); }
                if rty==-2 { 0 } else { rty }
            } },
        E::Not(x,l,h)=>{ let tx=check_e(x,env,ds,ss,fz,ez,tz); if tx!=1 && tx>=0 { let mut m=String::from("`!` needs a bool, found "); m.push_str(&tyname2(tx,ss)); ds.push(crate::oerrors::err(m,*l,*h)); return -1; } 1 },
        E::StructLit(n,fs,l,h)=>{
            let si=sidx_of(ss,n);
            if si<0 { let mut m=String::from("cannot find struct `"); m.push_str(n); m.push_str("` in this scope"); ds.push(crate::oerrors::err(m,*l,*h)); let mut k=0; while k<fs.len(){ let _=check_e(&fs[k],env,ds,ss,fz,ez,tz); k=k+1; } return -1; }
            let want=crate::oast::sfields(ss,n);
            if want>=0 && want!=(fs.len() as i32) { let mut m=String::from("struct `"); m.push_str(n); m.push_str("` has "); m.push_str(&crate::oast::u32s2(want as u32)); m.push_str(" field(s) but "); m.push_str(&crate::oast::u32s2(fs.len() as u32)); m.push_str(" given"); ds.push(crate::oerrors::err(m,*l,*h)); }
            let mut k=0; while k<fs.len(){ let tf=check_e(&fs[k],env,ds,ss,fz,ez,tz);
                if k < ss[si as usize].ftys.len() { let want=tag_of_tyname(ss, &ss[si as usize].ftys[k]); if want>=0 && tf>=0 && !tyeq(tf,want) { let mut m=String::from("field `"); m.push_str(&ss[si as usize].fields[k]); m.push_str("` expects "); m.push_str(&tyname2(want,ss)); m.push_str(", found "); m.push_str(&tyname2(tf,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } }
                k=k+1; }
            10+si },
        E::Field(o,f,l,h)=>{
            let to=check_e(o,env,ds,ss,fz,ez,tz);
            if to<10 { if to>=0 { let mut m=String::from("no field `"); m.push_str(f); m.push_str("` on "); m.push_str(&tyname2(to,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } return -1; }
            let si=(to-10) as usize;
            if si<ss.len() { let sn=String::from(ss[si].name.as_str()); let fi=crate::oast::sfield_idx(ss,&sn,f); if fi<0 { let mut m=String::from("no field `"); m.push_str(f); m.push_str("` on struct `"); m.push_str(&sn); m.push_str("`"); ds.push(crate::oerrors::err(m,*l,*h)); return -1; } let fty=crate::oast::sfield_ty(ss,&sn,f); let ft=type_tag(ss,ez,tz,&fty); if ft>=0 { return ft; } }
            0 },
        E::While(c,b,_l,_h)=>{ let tc=check_e(c,env,ds,ss,fz,ez,tz); if tc!=1 && tc>=0 { let mut m=String::from("`while` condition must be bool, found "); m.push_str(&tyname(tc)); ds.push(crate::oerrors::err(m, crate::oast::e_lo(c), crate::oast::e_hi(c))); } let _=check_e(b,env,ds,ss,fz,ez,tz); 0 },
        E::Seq(a,b,_l,_h)=>{ let _=check_e(a,env,ds,ss,fz,ez,tz); check_e(b,env,ds,ss,fz,ez,tz) },
        E::Assign(n,v,l,h)=>{ let tv=check_e(v,env,ds,ss,fz,ez,tz); let tn=env_get(env,n); if tn>=0 && tv>=0 && !tyeq(tn,tv) { let mut m=String::from("cannot assign "); m.push_str(&tyname2(tv,ss)); m.push_str(" to "); m.push_str(&tyname2(tn,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } 0 },
        E::Call(n2,args,l,h) if n2.as_str()=="str_at" || n2.as_str()=="str_cat" || n2.as_str()=="str_sub" || n2.as_str()=="chr1" =>{
            let want=if n2.as_str()=="str_sub" { 3 } else { 2 };
            if (args.len() as i32)!=want { let mut m=String::from("`"); m.push_str(n2); m.push_str("` takes "); m.push_str(&crate::oast::u32s2(want as u32)); m.push_str(" argument(s)"); ds.push(crate::oerrors::err(m,*l,*h)); }
            let mut k=0; while k<args.len(){ let ta=check_e(&args[k],env,ds,ss,fz,ez,tz);
              let need=if k==0 || (n2.as_str()=="str_cat" && k==1) { 2 } else { 0 };
              if ta!=need && ta!=0-3 { let mut m=String::from("`"); m.push_str(n2); m.push_str("` argument "); m.push_str(&crate::oast::u32s2(k as u32)); m.push_str(" must be "); m.push_str(if need==2 { "a str" } else { "an int" }); m.push_str(", found "); m.push_str(&tyname2(ta,ss)); ds.push(crate::oerrors::err(m,*l,*h)); }
              k=k+1; }
            if n2.as_str()=="str_at" { 0 } else { 2 } },
        E::Call(n2,args,l,h) if n2.as_str()=="vec_new" || n2.as_str()=="vec_push" || n2.as_str()=="vec_get" || n2.as_str()=="vec_set" || n2.as_str()=="vec_len" =>{
            let want=if n2.as_str()=="vec_new" { 0 } else if n2.as_str()=="vec_set" { 3 } else if n2.as_str()=="vec_push" || n2.as_str()=="vec_get" { 2 } else { 1 };
            if (args.len() as i32)!=want { let mut m=String::from("`"); m.push_str(n2); m.push_str("` takes "); m.push_str(&crate::oast::u32s2(want as u32)); m.push_str(" argument(s)"); ds.push(crate::oerrors::err(m,*l,*h)); let mut k=0; while k<args.len(){ let _=check_e(&args[k],env,ds,ss,fz,ez,tz); k=k+1; } return if n2.as_str()=="vec_new" { 201 } else { 0 }; }
            let mut k=0; while k<args.len(){ let ta=check_e(&args[k],env,ds,ss,fz,ez,tz);
              let need=if k==0 && n2.as_str()!="vec_new" { 201 } else { 0 };
              if k==0 && ta>=1000 { k=k+1; continue; }
              if k==1 && n2.as_str()=="vec_push" { let t0=check_e(&args[0],env,ds,ss,fz,ez,tz);
                  if t0>=1000 { let el=t0-1000;
                    if el>=400 { if ta>=10 && ta<100 { k=k+1; continue; } }
                    if ta==el { k=k+1; continue; }
                    let mut m=String::from("`vec_push` value must be "); m.push_str(&tyname2(el,ss)); m.push_str(", found "); m.push_str(&tyname2(ta,ss)); ds.push(crate::oerrors::err(m,*l,*h)); k=k+1; continue; }
                  if t0==201 { k=k+1; continue; } }
              if ta!=need && ta!=0-3 { let mut m=String::from("`"); m.push_str(n2); m.push_str("` argument "); m.push_str(&crate::oast::u32s2(k as u32)); m.push_str(" must be "); m.push_str(if need==201 { "a Vec" } else { "an int" }); m.push_str(", found "); m.push_str(&tyname2(ta,ss)); ds.push(crate::oerrors::err(m,*l,*h)); }
              k=k+1; }
            if n2.as_str()=="vec_new" { return 201; }
            if n2.as_str()=="vec_get" && args.len()==2 { let t0=check_e(&args[0],env,ds,ss,fz,ez,tz); if t0>=1000 { return t0-1000; } }
            0 },
        E::Call(n2,args,_lf1,_hf1) if n2.as_str()=="f64_parse" && args.len()==1 =>{ let _=check_e(&args[0],env,ds,ss,fz,ez,tz); 3 },
        E::Call(n2,args,_lf2,_hf2) if (n2.as_str()=="f64_snprintf" || n2.as_str()=="f64_snprintf_e") && args.len()==2 =>{ let _=check_e(&args[0],env,ds,ss,fz,ez,tz); let _=check_e(&args[1],env,ds,ss,fz,ez,tz); 2 },
        // a global load/store: an int in, an int out
        E::Call(n2,args,_lg,_hg) if crate::olower::is_gname(n2) =>{ let mut k=0; while k<args.len(){ let _=check_e(&args[k],env,ds,ss,fz,ez,tz); k=k+1; } 0 },
        E::Call(n2,args,_lf3,_hf3) if (n2.as_str()=="f64_bits" || n2.as_str()=="f64_isnan") && args.len()==1 =>{ let _=check_e(&args[0],env,ds,ss,fz,ez,tz); 0 },
        E::Call(n2,args,_lq,_hq) if n2.as_str()=="seq_slice" && args.len()==3 =>{ let t0=check_e(&args[0],env,ds,ss,fz,ez,tz); let mut k=1; while k<args.len(){ let _=check_e(&args[k],env,ds,ss,fz,ez,tz); k=k+1; } if t0==0-3 { 0-3 } else if t0==2 { 2 } else { 201 } },
        E::Call(n2,args,_ls,_hs) if n2.as_str()=="seq_len" || n2.as_str()=="seq_get" || n2.as_str()=="seq_set" =>{
            let t0=if args.len()>0 { check_e(&args[0],env,ds,ss,fz,ez,tz) } else { 0 };
            let mut k=1; while k<args.len(){ let _=check_e(&args[k],env,ds,ss,fz,ez,tz); k=k+1; }
            if n2.as_str()=="seq_len" { return 0; }
            if t0>=1000 { return t0-1000; }
            0 },
        E::Call(n2,args,_lp,_hp) if n2.as_str()=="panic_exit" && args.len()==0 =>0-1,
        E::Call(n2,args,_lp,_hp) if n2.as_str()=="print_nl" && args.len()==0 =>0,
        E::Call(n2,args,l,h) if n2.as_str()=="argc" =>{
            // C-ARGV: argc is the one builtin with NO argument, so it cannot ride the one-argument family
            if args.len()!=0 { let mut m=String::from("`argc` takes no argument"); ds.push(crate::oerrors::err(m,*l,*h)); }
            0 },
        E::Call(n2,args,l,h) if n2.as_str()=="print" || n2.as_str()=="len" || n2.as_str()=="print_int" || n2.as_str()=="print_raw" || n2.as_str()=="print_int_raw" || n2.as_str()=="print_char" || n2.as_str()=="print_any" || n2.as_str()=="print_dbg" || n2.as_str()=="print_nl" || n2.as_str()=="panic_exit" || n2.as_str()=="arreq" || n2.as_str()=="arr_len" || n2.as_str()=="veceq" || n2.as_str()=="enumeq" || n2.as_str()=="proc_exit" || n2.as_str()=="vec_popraw" || n2.as_str()=="str_of"  || n2.as_str()=="readfile" || n2.as_str()=="fileok" || n2.as_str()=="argv_at"=>{
            if args.len()!=1 { let mut m=String::from("`"); m.push_str(n2); m.push_str("` takes 1 argument"); ds.push(crate::oerrors::err(m,*l,*h)); return 0; }
            let ta=check_e(&args[0],env,ds,ss,fz,ez,tz);
            if n2.as_str()=="print_any" || n2.as_str()=="print_int_raw" || n2.as_str()=="print_char" || n2.as_str()=="arreq" || n2.as_str()=="arr_len" || n2.as_str()=="veceq" || n2.as_str()=="enumeq" || n2.as_str()=="proc_exit" || n2.as_str()=="vec_popraw" { return 0; }
            if n2.as_str()=="str_of" || n2.as_str()=="argv_at" || n2.as_str()=="readfile" { return 2; }
            if n2.as_str()=="fileok" { return 0; }
            if n2.as_str()=="print_dbg" {
                // `{:?}` has no derived Debug here: a scalar or str prints like `{}`, anything else is refused
                // D2-D6: a SEQUENCE, a STRUCT and an ENUM now have a Debug printer emitted from their
                // static shape; what is still unsupported is caught in lowering, where the shape is known
                // a TUPLE is admitted here and decided in lowering: a tuple LITERAL carries its element
                // types and is emittable, a tuple VARIABLE does not and is refused there
                let dbgable = ta==200 || ta==201 || ta>=1000 || (ta>=10 && ta<200) || (ta>=300 && ta<400);
                if !dbgable && ta!=0 && ta!=1 && ta!=2 && ta!=3 && ta!=0-3 && ta>=0 { let m=String::from("`{:?}` on a non-scalar needs a derived Debug (not implemented)"); ds.push(crate::oerrors::err(m,*l,*h)); }
                return 0; }
            if n2.as_str()=="print_raw" { if ta!=2 && ta!=0-3 { let mut m=String::from("`print_raw` needs a str"); ds.push(crate::oerrors::err(m,*l,*h)); } return 0; }
            if n2.as_str()=="print_int" { if ta!=0 && ta!=0-3 { let mut m=String::from("`print_int` needs an int, found "); m.push_str(&tyname2(ta,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } return 0; }
            if ta!=2 && ta!=0-3 { let mut m=String::from("`"); m.push_str(n2); m.push_str("` needs a str, found "); m.push_str(&tyname2(ta,ss)); ds.push(crate::oerrors::err(m,*l,*h)); }
            0 },
        E::Call(n2,args,_lf,_hf) if { let mut tt=0-1; let mut q=env.n.len(); while q>0 { q=q-1; if env.n[q].as_str()==n2.as_str() { tt=env.t[q]; q=0; } } tt==900 } =>{ let mut k=0; while k<args.len(){ let _=check_e(&args[k],env,ds,ss,fz,ez,tz); k=k+1; }
            // a call through an `Fn`-typed parameter has an UNKNOWN result type: our closures carry no
            // signature. Answering `int` let our own dialect's `f(x) == 1` type-check and rejected a
            // bool-returning Rust closure in `if f(x)`; answering unknown admits both, which is what we know.
            0-3 },
        E::Call(n2,args,l,h) if { let mut tt=0-1; let mut q=env.n.len(); while q>0 { q=q-1; if env.n[q].as_str()==n2.as_str() { tt=env.t[q]; q=0; } } tt>=800 && tt<1000 } =>{
            let mut tv=0; { let mut q=env.n.len(); while q>0 { q=q-1; if env.n[q].as_str()==n2.as_str() { tv=env.t[q]; q=0; } } }
            let cn=clos_name_of_tag(tv);
            let mut k=0; while k<args.len(){ let _=check_e(&args[k],env,ds,ss,fz,ez,tz); k=k+1; }
            let mut want=0-1; let mut rr=String::from(""); let mut q2=0;
            while q2<fz.len(){ if fz[q2].name.as_str()==cn.as_str() { want=(fz[q2].params.len() as i32)-fz[q2].ncap; rr=String::from(fz[q2].ret.as_str()); } q2=q2+1; }
            if want>=0 && want!=(args.len() as i32) { let mut m=String::from("closure takes "); m.push_str(&crate::oast::u32s2(want as u32)); m.push_str(" argument(s) but "); m.push_str(&crate::oast::u32s2(args.len() as u32)); m.push_str(" supplied"); ds.push(crate::oerrors::err(m,*l,*h)); }
            let t2=tag_of_tyname(ss,&rr); if t2>=0 { t2 } else { let ei=crate::oast::eidx(ez,&rr); if ei>=0 { 100+ei } else { 0 } } },
        E::Call(n2,args,_l,_h)=>{
            // C-ONEWALK: the per-argument TYPE check lives here, where every argument is already typed
            // exactly once, instead of in check_calls_t -- which typed each argument with check_e and then
            // recursed into the same subtree, making the pass quadratic in call-nesting depth. The NAME and
            // ARITY diagnostics stay in check_calls_t: they need no types, and check_e reaches Call nodes
            // that walk deliberately never reports (a trait associated const read through Self).
            let mut fx=0-1; { let mut fi=0; while fi<fz.len(){ if fz[fi].name.as_str()==n2 { fx=fi as i32; fi=fz.len(); } else { fi=fi+1; } } }
            let mut k=0; while k<args.len(){ let ta=check_e(&args[k],env,ds,ss,fz,ez,tz);
              if fx>=0 { let fq=fx as usize;
                  if k<fz[fq].ptys.len() && !is_gp(&fz[fq].gparams, &fz[fq].ptys[k]) {
                      let want=tag_of_tyname(ss, &fz[fq].ptys[k]);
                      let ok_fn=want==900 && ta>=800 && ta<1000;
                      if want>=0 && ta>=0 && !tyeq(ta,want) && !ok_fn {
                          let mut m=String::from("argument "); m.push_str(&crate::oast::u32s2((k+1) as u32)); m.push_str(" of `"); m.push_str(n2); m.push_str("` expects "); m.push_str(&tyname2(want,ss)); m.push_str(", found "); m.push_str(&tyname2(ta,ss)); ds.push(crate::oerrors::err(m,*_l,*_h)); } } }
              // a TRAIT OBJECT parameter can only take a struct or an enum (those carry the tag)
              { let mut fj=0; while fj<fz.len(){ if fz[fj].name.as_str()==n2 && k<fz[fj].ptys.len() {
                    if dyn_name_trait(fz[fj].ptys[k].as_str()).len()>0 && ta>=0 && !(ta>=10 && ta<200) && !(ta>=0 && ta<=2) {
                        let mut m2=String::from("a trait object from "); m2.push_str(&tyname2(ta,ss));
                        m2.push_str(" is not implemented (only a struct or enum carries the tag)");
                        ds.push(crate::oerrors::err(m2,*_l,*_h)); }
                    fj=fz.len(); } else { fj=fj+1; } } }
              k=k+1; } let rt=fn_ret(fz,n2);
            { let mut fi=0; while fi<fz.len(){ if fz[fi].name.as_str()==n2 { if is_gp(&fz[fi].gparams,&rt) { let mut q=0; while q<fz[fi].ptys.len(){ if fz[fi].ptys[q].as_str()==rt.as_str() && q<args.len() { return check_e(&args[q],env,ds,ss,fz,ez,tz); } q=q+1; } return 0-3; } } fi=fi+1; } }
            let t2=type_tag(ss,ez,tz,&rt); if t2>=0 { t2 } else { 0 } },
        E::Int(_v,_l,_h)=>0,
        E::Bool(_v,_l,_h)=>1,
        E::Var(n,_l,_h)=>env_get(env,n),
        E::Bin(op,a,b,l,h)=>{
            let ta=if is_bitop(op) { match a.as_ref() { E::Not(ax,_la,_ha)=>{ let ti=check_e(ax,env,ds,ss,fz,ez,tz); if ti==0 { 0 } else { check_e(a,env,ds,ss,fz,ez,tz) } }, _=>check_e(a,env,ds,ss,fz,ez,tz) } } else { check_e(a,env,ds,ss,fz,ez,tz) };
            let tb=if is_bitop(op) { match b.as_ref() { E::Not(bx,_lb,_hb)=>{ let ti=check_e(bx,env,ds,ss,fz,ez,tz); if ti==0 { 0 } else { check_e(b,env,ds,ss,fz,ez,tz) } }, _=>check_e(b,env,ds,ss,fz,ez,tz) } } else { check_e(b,env,ds,ss,fz,ez,tz) };
            let ovl={ let mm=crate::olower::op_method(op);
                ta>=10 && ta<100 && mm.len()>0 && { let si=(ta-10) as usize;
                  if si<ss.len() { let mut q=String::from(ss[si].name.as_str()); q.push('$'); q.push_str(&mm); crate::oast::find_method(fz,ss[si].name.as_str(),&q)>=0 } else { false } } };
            let _=tb;
            if ovl {
                let mm=crate::olower::op_method(op); let si=(ta-10) as usize;
                let mut q=String::from(ss[si].name.as_str()); q.push('$'); q.push_str(&mm);
                let fi=crate::oast::find_method(fz,ss[si].name.as_str(),&q);
                if fi<0 { return 0; }
                let rr=String::from(fz[fi as usize].ret.as_str());
                let t2=type_tag(ss,ez,tz,&rr); if t2>=0 { t2 } else { 0 }
            } else {
    
                // F-1: an f64 operand (tag 3) makes the operation a FLOAT operation -- arithmetic answers an
                // f64, a comparison a bool. Mixing a float with an int is a type error in Rust and here.
                if ta==3 || tb==3 {
                    // the `0 - x` shape that a unary minus desugars to is a float negation when x is a float
                    { let isz=match a.as_ref() { E::Int(0,_lz,_hz)=>true, _=>false };
                      if isz && tb==3 && op.as_str()=="-" { return 3; } }
                    let both = (ta==3 || ta==0-3) && (tb==3 || tb==0-3);
                    if !both { if ta>=0 && tb>=0 { let mut m=String::from("cannot apply `"); m.push_str(op); m.push_str("` to "); m.push_str(&tyname2(ta,ss)); m.push_str(" and "); m.push_str(&tyname2(tb,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } return 3; }
                    if op.as_str()=="=="||op.as_str()=="!="||op.as_str()=="<"||op.as_str()=="<="||op.as_str()==">"||op.as_str()==">=" { return 1; }
                    return 3; }
                if op.as_str()=="+"||op.as_str()=="-"||op.as_str()=="*"||op.as_str()=="/"||op.as_str()=="%"||op.as_str()=="&"||op.as_str()=="|"||op.as_str()=="^"||op.as_str()=="<<"||op.as_str()==">>" {
                    if (ta==0||ta==0-3) && (tb==0||tb==0-3) { 0 } else { if ta>=0 && tb>=0 { let mut m=String::from("cannot apply `"); m.push_str(op); m.push_str("` to "); m.push_str(&tyname(ta)); m.push_str(" and "); m.push_str(&tyname(tb)); ds.push(crate::oerrors::err(m,*l,*h)); } -1 } }
                else if op.as_str()=="&&" || op.as_str()=="||" { if ta==1 && tb==1 { 1 } else { if ta>=0 && tb>=0 { let mut m=String::from("`"); m.push_str(op); m.push_str("` needs bool operands, found "); m.push_str(&tyname2(ta,ss)); m.push_str(" and "); m.push_str(&tyname2(tb,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } -1 } } else if op.as_str()=="==" || op.as_str()=="!=" { if (ta==tb && ta>=0) || ta==0-3 || tb==0-3 { 1 } else { if ta>=0 && tb>=0 { let mut m=String::from("cannot compare "); m.push_str(&tyname2(ta,ss)); m.push_str(" with "); m.push_str(&tyname2(tb,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } -1 } } else { if (ta==0||ta==0-3) && (tb==0||tb==0-3) { 1 } else { if ta==2 && tb==2 { 1 } else { if ta>=0 && tb>=0 { let mut m=String::from("cannot order "); m.push_str(&tyname2(ta,ss)); m.push_str(" with "); m.push_str(&tyname2(tb,ss)); ds.push(crate::oerrors::err(m,*l,*h)); } -1 } } }
            } },
        E::If(c,th,el,_l,_h)=>{ let tc=check_e(c,env,ds,ss,fz,ez,tz); let tt=check_e(th,env,ds,ss,fz,ez,tz); let te=check_e(el,env,ds,ss,fz,ez,tz);
            if tc!=1 && tc>=0 { let mut m=String::from("`if` condition must be bool, found "); m.push_str(&tyname(tc)); ds.push(crate::oerrors::err(m, crate::oast::e_lo(c), crate::oast::e_hi(c))); }
            if !tyeq(tt,te) && tt>=0 && te>=0 { let mut m=String::from("`if` and `else` have incompatible types: "); m.push_str(&tyname(tt)); m.push_str(" vs "); m.push_str(&tyname(te)); ds.push(crate::oerrors::err(m, crate::oast::e_lo(el), crate::oast::e_hi(el))); return -1; }
            if tc!=1 { -1 } else { tt } },
        E::Let(n,v,b,_l,_h)=>{ let tv=check_e(v,env,ds,ss,fz,ez,tz); env.n.push(String::from(n.as_str())); env.t.push(tv); let rb=check_e(b,env,ds,ss,fz,ez,tz); env.n.pop(); env.t.pop(); rb },
    }
}
pub fn fenv_of(f:&Fun, ss:&Vec<Sdef>)->crate::oast::Env{ let mut nn:Vec<String>=Vec::new(); let mut tt:Vec<i32>=Vec::new(); let mut i=0; while i<f.params.len(){ nn.push(String::from(f.params[i].as_str())); let t0=if i<f.ptys.len() { tag_of_tyname(ss, &f.ptys[i]) } else { 0 }; let t=if t0<0 { 0-3 } else { t0 }; tt.push(t); i=i+1; } crate::oast::Env{n:nn,t:tt} }
pub fn fenv_of_t(f:&Fun, ss:&Vec<Sdef>, ez:&Vec<Edef>, tz:&Vec<Tdef>)->crate::oast::Env{ let mut nn:Vec<String>=Vec::new(); let mut tt:Vec<i32>=Vec::new(); let mut i=0; while i<f.params.len(){ nn.push(String::from(f.params[i].as_str())); let t0=if i<f.ptys.len() { type_tag(ss, ez, tz, &f.ptys[i]) } else { 0 }; let t=if t0<0 { 0-3 } else { t0 }; tt.push(t); i=i+1; } crate::oast::Env{n:nn,t:tt} }
pub fn fn_arity(funs:&Vec<Fun>, n:&str)->i32{ let mut i=0; while i<funs.len(){ if funs[i].name.as_str()==n { return funs[i].params.len() as i32; } i=i+1; } -1 }
pub fn fn_ret(funs:&Vec<Fun>, n:&str)->String{ let mut i=0; while i<funs.len(){ if funs[i].name.as_str()==n { return String::from(funs[i].ret.as_str()); } i=i+1; } String::from("int") }
pub fn check_calls(e:&E, funs:&Vec<Fun>, ds:&mut Vec<Diag>){ let ez:Vec<Edef>=Vec::new(); let tz:Vec<Tdef>=Vec::new(); let mut ev=crate::oast::Env{n:Vec::new(),t:Vec::new()}; check_calls_t(e, funs, ds, &empty_sdefs(), &mut ev, &ez, &tz) }
pub fn check_calls_t(e:&E, funs:&Vec<Fun>, ds:&mut Vec<Diag>, ss:&Vec<Sdef>, env:&mut crate::oast::Env, ez:&Vec<Edef>, tz:&Vec<Tdef>){
    match e {
        E::Call(n,args,_lg2,_hg2) if crate::olower::is_gname(n) =>{ let mut k=0; while k<args.len(){ check_calls_t(&args[k],funs,ds,ss,env,ez,tz); k=k+1; } },
        E::Call(n,args,_l2,_h2) if n.as_str()=="print" || n.as_str()=="len" || n.as_str()=="print_int" || n.as_str()=="streq" || n.as_str()=="vec_new" || n.as_str()=="vec_push" || n.as_str()=="vec_get" || n.as_str()=="vec_set" || n.as_str()=="vec_len" || n.as_str()=="str_at" || n.as_str()=="str_cat" || n.as_str()=="str_sub" || n.as_str()=="print_any" || n.as_str()=="print_dbg" || n.as_str()=="print_raw" || n.as_str()=="print_int_raw" || n.as_str()=="print_char" || n.as_str()=="print_nl" || n.as_str()=="panic_exit" || n.as_str()=="arreq" || n.as_str()=="arr_len" || n.as_str()=="veceq" || n.as_str()=="enumeq" || n.as_str()=="proc_exit" || n.as_str()=="vec_popraw" || n.as_str()=="str_of" || n.as_str()=="seq_len" || n.as_str()=="seq_get" || n.as_str()=="seq_set" || n.as_str()=="seq_slice" || n.as_str()=="f64_parse" || n.as_str()=="f64_snprintf" || n.as_str()=="f64_snprintf_e" || n.as_str()=="f64_bits" || n.as_str()=="f64_isnan" || n.as_str()=="readfile" || n.as_str()=="fileok" || n.as_str()=="argc" || n.as_str()=="argv_at" =>{ let mut k=0; while k<args.len(){ check_calls_t(&args[k],funs,ds,ss,env,ez,tz); k=k+1; } },
        E::Call(n,args,_l9,_h9) if { let mut t=0-1; let mut q=env.n.len(); while q>0 { q=q-1; if env.n[q].as_str()==n.as_str() { t=env.t[q]; q=0; } } t==900 } =>{ let mut k=0; while k<args.len(){ check_calls_t(&args[k],funs,ds,ss,env,ez,tz); k=k+1; } },
        E::Call(n,args,l,h) if { let mut t=0-1; let mut q=env.n.len(); while q>0 { q=q-1; if env.n[q].as_str()==n.as_str() { t=env.t[q]; q=0; } } t>=800 && t<1000 } =>{ let mut k=0; while k<args.len(){ check_calls_t(&args[k],funs,ds,ss,env,ez,tz); k=k+1; } },
        E::Call(n,args,l,h)=>{ let a=fn_arity(funs,n); if a<0 { let mut m=String::from("cannot find function `"); m.push_str(n); m.push_str("` in this scope"); ds.push(crate::oerrors::err(m,*l,*h)); } else if a!=(args.len() as i32) { let mut m=String::from("function `"); m.push_str(n); m.push_str("` takes "); m.push_str(&crate::oast::u32s2(a as u32)); m.push_str(" argument(s) but "); m.push_str(&crate::oast::u32s2(args.len() as u32)); m.push_str(" supplied"); ds.push(crate::oerrors::err(m,*l,*h)); }
                      let mut k=0; while k<args.len(){ check_calls_t(&args[k],funs,ds,ss,env,ez,tz); k=k+1; } },
        E::Bin(_o,a,b,_l,_h)=>{ check_calls_t(a,funs,ds,ss,env,ez,tz); check_calls_t(b,funs,ds,ss,env,ez,tz); },
        E::While(c,b,_l,_h)=>{ check_calls_t(c,funs,ds,ss,env,ez,tz); check_calls_t(b,funs,ds,ss,env,ez,tz); },
        E::Not(x,_l,_h)=>{ check_calls_t(x,funs,ds,ss,env,ez,tz); },
        E::StrLit(_v,_l,_h)=>{},
        E::ClosRef(_n,_c,_l,_h)=>{},
        E::Try(x,_l,_h)=>{ check_calls_t(x,funs,ds,ss,env,ez,tz); },
        E::BreakE(_l,_h)=>{},
        E::Ref(x,_m,_l,_h)=>{ check_calls_t(x,funs,ds,ss,env,ez,tz); },
        E::Cast(x,_t,_l,_h)=>{ check_calls_t(x,funs,ds,ss,env,ez,tz); },
        E::ContE(_l,_h)=>{},
        E::LetT(n,_t,v,b,_l,_h)=>{ check_calls_t(v,funs,ds,ss,env,ez,tz);
            match v.as_ref() { E::ClosRef(cn,_c,_l2,_h2)=>{ let ci=clos_idx_of_name(cn);
                    env.n.push(String::from(n.as_str())); env.t.push(800+ci);
                    check_calls_t(b,funs,ds,ss,env,ez,tz); env.n.pop(); env.t.pop(); },
                _=>{ let td=type_tag(ss,ez,tz,_t);
                     env.n.push(String::from(n.as_str())); env.t.push(td);
                     check_calls_t(b,funs,ds,ss,env,ez,tz); env.n.pop(); env.t.pop(); } } },
        E::TupLit(parts,_l,_h)=>{ let mut k=0; while k<parts.len(){ check_calls_t(&parts[k],funs,ds,ss,env,ez,tz); k=k+1; } },
        E::TupIdx(b,_i,_l,_h)=>{ check_calls_t(b,funs,ds,ss,env,ez,tz); },
        E::RetE(x,_l,_h)=>{ check_calls_t(x,funs,ds,ss,env,ez,tz); },
        E::MethodCall(rv,_m,args,_l,_h)=>{ check_calls_t(rv,funs,ds,ss,env,ez,tz); let mut k=0; while k<args.len(){ check_calls_t(&args[k],funs,ds,ss,env,ez,tz); k=k+1; } },
        E::AssignTo(pl,v,_l,_h)=>{ check_calls_t(pl,funs,ds,ss,env,ez,tz); check_calls_t(v,funs,ds,ss,env,ez,tz); },
        E::ArrLit(xs,_l,_h)=>{ let mut k=0; while k<xs.len(){ check_calls_t(&xs[k],funs,ds,ss,env,ez,tz); k=k+1; } },
        E::Index(b,ix,_l,_h)=>{ check_calls_t(b,funs,ds,ss,env,ez,tz); check_calls_t(ix,funs,ds,ss,env,ez,tz); },
        E::EnumLit(_e2,_v,ps,_l,_h)=>{ let mut k=0; while k<ps.len(){ check_calls_t(&ps[k],funs,ds,ss,env,ez,tz); k=k+1; } },
        // C-ARGENUM: an arm's payload BINDINGS must be in scope while its body is checked -- the main type
        // rule binds them, this walk did not, so any argument expression over a binding fell to the
        // untyped fallback and reported a type error at the call instead.
        E::Match(sc,arms,_l,_h)=>{ check_calls_t(sc,funs,ds,ss,env,ez,tz);
            let en={ let mut dsx:Vec<Diag>=Vec::new(); let ts=check_e(sc,env,&mut dsx,ss,funs,ez,tz);
                     if ts>=100 && ts<200 && ((ts-100) as usize)<ez.len() { String::from(ez[(ts-100) as usize].name.as_str()) } else { String::from("") } };
            let mut k=0;
            while k<arms.len(){
                if en.len()>0 && arms[k].binds.len()>0 {
                    let mut nb=0;
                    let mut bi=0;
                    while bi<arms[k].binds.len(){
                        env.n.push(String::from(arms[k].binds[bi].as_str()));
                        let pt=crate::oast::vpayload_ty(ez,&en,&arms[k].variant,bi);
                        let tg=type_tag(ss,ez,tz,&pt); env.t.push(if tg>=0 { tg } else { 0-3 }); nb=nb+1; bi=bi+1; }
                    check_calls_t(&arms[k].body,funs,ds,ss,env,ez,tz);
                    { let mut q=0; while q<nb { env.n.pop(); env.t.pop(); q=q+1; } } }
                else { check_calls_t(&arms[k].body,funs,ds,ss,env,ez,tz); }
                k=k+1; } },
        E::StructLit(_n,fs,_l,_h)=>{ let mut k=0; while k<fs.len(){ check_calls_t(&fs[k],funs,ds,ss,env,ez,tz); k=k+1; } },
        E::Field(o,_f,_l,_h)=>{ check_calls_t(o,funs,ds,ss,env,ez,tz); },
        E::Seq(a,b,_l,_h)=>{ check_calls_t(a,funs,ds,ss,env,ez,tz); check_calls_t(b,funs,ds,ss,env,ez,tz); },
        E::Assign(_n,v,_l,_h)=>{ check_calls_t(v,funs,ds,ss,env,ez,tz); },
        E::If(c,t,f,_l,_h)=>{ check_calls_t(c,funs,ds,ss,env,ez,tz); let mut ev=crate::oast::Env{n:Vec::new(),t:Vec::new()}; check_calls_t(t,funs,ds,ss,&mut ev,ez,tz); check_calls_t(f,funs,ds,ss,&mut ev,ez,tz); },
        E::Let(n,v,b,_l,_h)=>{ check_calls_t(v,funs,ds,ss,env,ez,tz);
            match v.as_ref() { E::ClosRef(cn,_c,_l2,_h2)=>{ let ci=clos_idx_of_name(cn);
                    env.n.push(String::from(n.as_str())); env.t.push(800+ci);
                    check_calls_t(b,funs,ds,ss,env,ez,tz); env.n.pop(); env.t.pop(); },
                _=>{ let mut dsx:Vec<Diag>=Vec::new();
                     let tv=check_e(v,env,&mut dsx,ss,funs,ez,tz);
                     env.n.push(String::from(n.as_str())); env.t.push(tv);
                     check_calls_t(b,funs,ds,ss,env,ez,tz); env.n.pop(); env.t.pop(); } } },
        _=>{},
    }
}
}
mod omono {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::oborrowck::*;
// omono -- MONOMORPHIZATION and signature inference: generic struct instances, unannotated return
// types, and the demanded (fn, concrete-type) instantiation set discovered by a throwaway lowering pass
pub fn sdef_ix(ss:&Vec<Sdef>, n:&str)->i32{ let mut i=0; while i<ss.len(){ if ss[i].name.as_str()==n { return i as i32; } i=i+1; } 0-1 }
pub fn want_sinst(prog:&Prog, out:&mut Vec<String>){
    let mut add=|t:&str, o:&mut Vec<String>| { if t.len()>0 && sdef_ix(&prog.sdefs,t)<0 && !crate::oast::in_list(o,t) { o.push(String::from(t)); } };
    let mut i=0; while i<prog.funs.len(){
        let mut k=0; while k<prog.funs[i].ptys.len(){ add(prog.funs[i].ptys[k].as_str(), out); k=k+1; }
        add(prog.funs[i].ret.as_str(), out); i=i+1; }
    let mut j=0; while j<prog.sdefs.len(){ let mut k=0; while k<prog.sdefs[j].ftys.len(){ add(prog.sdefs[j].ftys[k].as_str(), out); k=k+1; } j=j+1; }
}
pub fn mono_structs(prog:&mut Prog){
    let mut rounds=0;
    while rounds<4 {
        let mut want:Vec<String>=Vec::new(); want_sinst(prog, &mut want);
        let mut made=0; let mut q=0;
        while q<want.len(){
            let t=String::from(want[q].as_str());
            // split at the LAST `$`: `Wrap$int` -> base `Wrap`, arg `int`
            let b=t.as_bytes(); let mut cut=0-1; let mut k=0; while k<b.len(){ if b[k]==36 { cut=k as i32; } k=k+1; }
            if cut>0 { let base=String::from(&t[..(cut as usize)]); let arg=String::from(&t[((cut as usize)+1)..]);
                let bi=sdef_ix(&prog.sdefs,&base);
                if bi>=0 && prog.sdefs[bi as usize].sgp.len()>0 {
                    let mut fl:Vec<String>=Vec::new(); let mut fy:Vec<String>=Vec::new();
                    { let d=&prog.sdefs[bi as usize]; let mut f=0;
                      while f<d.fields.len(){ fl.push(String::from(d.fields[f].as_str()));
                        let mut ty=String::from(d.ftys[f].as_str()); let mut g=0;
                        while g<d.sgp.len(){ ty=crate::oast::subst_seg(ty.as_str(), d.sgp[g].as_str(), arg.as_str()); g=g+1; }
                        fy.push(ty); f=f+1; } }
                    { let bc=prog.sdefs[bi as usize].copy; prog.sdefs.push(Sdef{copy:bc, sgp:Vec::new(), name:t, fields:fl, ftys:fy}); } made=made+1; } }
            q=q+1; }
        if made==0 { rounds=4; } else { rounds=rounds+1; } }
}
pub fn infer_rets(prog:&mut Prog){
    let mut got:Vec<String>=Vec::new();
    { let mut lc=crate::olower::lcx_new(); lc.sdefs=crate::oast::copy_sdefs(&prog.sdefs); crate::olower::fill_fsigs(&mut lc, &prog.funs); crate::olower::fill_dyn(&mut lc, &prog.impls); lc.edefs=crate::oast::copy_edefs(&prog.edefs);
      let mut i=0; while i<prog.funs.len(){
        let mut r=String::from("");
        if prog.funs[i].ret.len()==0 && !crate::oast::has_bound(&prog.funs[i]) && !crate::oast::has_composite_gp(&prog.funs[i]) {
            let bx=crate::oast::fbody_ix(&prog.funs, i);
            crate::olower::lcx_reset(&mut lc);
            let mut pi=0; while pi<prog.funs[i].params.len(){ lc.names.push(String::from(prog.funs[i].params[pi].as_str()));
                let pt=if pi<prog.funs[i].ptys.len() { String::from(prog.funs[i].ptys[pi].as_str()) } else { String::from("") };
                let ps=crate::olower::local_sty(&prog.sdefs, &prog.edefs, &pt);
                lc.stypes.push(ps); pi=pi+1; }
            let et=crate::olower::ety_of(&lc, &prog.funs[bx].body);
            if et.len()>0 && (crate::oast::is_struct_name(&prog.sdefs,&et) || crate::oast::is_enum_name(&prog.edefs,&et)) { r=et; } }
        got.push(r); i=i+1; } }
    let mut j=0; while j<got.len(){ if got[j].len()>0 { prog.funs[j].ret=String::from(got[j].as_str()); } j=j+1; }
}
pub fn monomorphize(prog:&mut Prog){
    let mut dem:Vec<String>=Vec::new();
    { let mut lc=crate::olower::lcx_new(); lc.sdefs=crate::oast::copy_sdefs(&prog.sdefs); crate::olower::fill_fsigs(&mut lc, &prog.funs); crate::olower::fill_dyn(&mut lc, &prog.impls); lc.edefs=crate::oast::copy_edefs(&prog.edefs);
      let _=crate::olower::lower(&prog.main, &mut lc);
      let mut l2=crate::olower::lcx_new(); l2.sdefs=crate::oast::copy_sdefs(&prog.sdefs); crate::olower::fill_fsigs(&mut l2, &prog.funs); crate::olower::fill_dyn(&mut l2, &prog.impls); l2.edefs=crate::oast::copy_edefs(&prog.edefs);
      let mut i=0; while i<prog.funs.len(){
        if !crate::oast::has_bound(&prog.funs[i]) && !crate::oast::has_composite_gp(&prog.funs[i]) {
          let bx=crate::oast::fbody_ix(&prog.funs, i);
          crate::olower::lcx_reset(&mut l2);
          let mut pi=0; while pi<prog.funs[i].params.len(){ l2.names.push(String::from(prog.funs[i].params[pi].as_str()));
            let pt=if pi<prog.funs[i].ptys.len() { String::from(prog.funs[i].ptys[pi].as_str()) } else { String::from("") };
            let ps=crate::olower::local_sty(&prog.sdefs, &prog.edefs, &pt);
            l2.stypes.push(ps); pi=pi+1; }
          let _=crate::olower::lower(&prog.funs[bx].body, &mut l2);
          let mut k=0; while k<l2.demands.len(){ dem.push(String::from(l2.demands[k].as_str())); k=k+1; } }
        i=i+1; }
      let mut k=0; while k<lc.demands.len(){ dem.push(String::from(lc.demands[k].as_str())); k=k+1; } }
    let mut q=0; while q<dem.len(){
        let parts=crate::olower::split_bar(dem[q].as_str());
        if parts.len()==2 { let fx=crate::oparse::find_fun_ix(&prog.funs, parts[0].as_str());
          if fx>=0 {
            let cs=crate::olower::split_c(parts[1].as_str());
            let mut mn=String::from(parts[0].as_str());
            { let mut ci=0; while ci<cs.len(){ mn.push_str("$"); mn.push_str(cs[ci].as_str()); ci=ci+1; } }
            if crate::oparse::find_fun_ix(&prog.funs,&mn)<0 {
              let mut ps:Vec<String>=Vec::new(); let mut pt:Vec<String>=Vec::new();
              let mut rt=String::from(""); let mut gp:Vec<String>=Vec::new(); let mut gb:Vec<String>=Vec::new();
              { let f=&prog.funs[fx as usize]; let mut k2=0;
                while k2<f.params.len(){ ps.push(String::from(f.params[k2].as_str())); k2=k2+1; }
                let mut k3=0; while k3<f.gparams.len(){ gp.push(String::from(f.gparams[k3].as_str())); k3=k3+1; }
                let mut k4=0; while k4<f.gbounds.len(){ gb.push(String::from(f.gbounds[k4].as_str())); k4=k4+1; }
                let mut k5=0; while k5<f.ptys.len(){ let mut t=String::from(f.ptys[k5].as_str()); let mut gi2=0; while gi2<gp.len(){ if gi2<cs.len() { t=crate::oast::subst_seg(t.as_str(), gp[gi2].as_str(), cs[gi2].as_str()); } gi2=gi2+1; } pt.push(t); k5=k5+1; }
                { let mut t=String::from(f.ret.as_str()); let mut gi3=0; while gi3<gp.len(){ if gi3<cs.len() { t=crate::oast::subst_seg(t.as_str(), gp[gi3].as_str(), cs[gi3].as_str()); } gi3=gi3+1; } rt=t; } }
              prog.funs.push(Fun{pref:Vec::new(), ncap:0, gbounds:Vec::new(), dflt:fx, name:mn, params:ps, ptys:pt, ret:rt, gparams:Vec::new(), owner:String::from(""), body:E::Int(0,0,0)});
              // TRANSITIVE demands: a bounded generic's body is skipped by the pre-pass (its parameter
              // types are not concrete yet), so the instances IT calls were never demanded and the call
              // emitted an undefined name. Lower the instance we just made, with its concrete parameter
              // types, and feed its demands back into this same worklist.
              { let ni=prog.funs.len()-1;
                let mut l3=crate::olower::lcx_new(); l3.sdefs=crate::oast::copy_sdefs(&prog.sdefs);
                crate::olower::fill_fsigs(&mut l3, &prog.funs); crate::olower::fill_dyn(&mut l3, &prog.impls);
                l3.edefs=crate::oast::copy_edefs(&prog.edefs);
                let mut pi=0; while pi<prog.funs[ni].params.len(){
                    l3.names.push(String::from(prog.funs[ni].params[pi].as_str()));
                    let pt2=if pi<prog.funs[ni].ptys.len() { String::from(prog.funs[ni].ptys[pi].as_str()) } else { String::from("") };
                    let ps2=crate::olower::local_sty(&prog.sdefs, &prog.edefs, &pt2);
                    l3.stypes.push(ps2); pi=pi+1; }
                let bx2=crate::oast::fbody_ix(&prog.funs, ni);
                let _=crate::olower::lower(&prog.funs[bx2].body, &mut l3);
                let mut k6=0; while k6<l3.demands.len(){ dem.push(String::from(l3.demands[k6].as_str())); k6=k6+1; } }
            } } }
        q=q+1; }
}
}
mod oborrowck {
    use crate::ospan::*;
    use crate::oerrors::*;
    use crate::oindex::*;
    use crate::oarena::*;
    use crate::oabi::*;
    use crate::olex::*;
    use crate::oast::*;
    use crate::oresolve::*;
    use crate::ohir::*;
    use crate::olower::*;
    use crate::omir::*;
    use crate::omirtrans::*;
    use crate::ocodegen::*;
    use crate::osession::*;
    use crate::oparse::*;
    use crate::otypeck::*;
    use crate::omono::*;
// oborrowck -- MOVE CHECKING: Rust's rule that passing a non-Copy value BY VALUE moves it, and using it
// afterwards is an error. This is the phase ourc had no equivalent of at all.
//
// Scope of the rule implemented here: a local whose type is a STRUCT or an ENUM is move-tracked (ints and
// bools are Copy; `str`/`Vec` are deliberately left out of the first cut). Passing such a local by value to a
// function moves it; passing `&x` / `&mut x` does NOT (which is why references are explicit in the AST).
// A move inside a LOOP body is also a second use of the same value, so it is reported.
pub struct Mv { pub names: Vec<String>, pub moved: Vec<bool>, pub aggr: Vec<bool>, pub ldep: Vec<i32> }
pub fn mv_new()->Mv{ Mv{names:Vec::new(), moved:Vec::new(), aggr:Vec::new(), ldep:Vec::new()} }
pub fn mv_ix(m:&Mv, n:&str)->i32{ let mut i=m.names.len(); while i>0 { i=i-1; if m.names[i].as_str()==n { return i as i32; } } 0-1 }
pub fn mv_bind_d(m:&mut Mv, n:&str, aggregate:bool, depth:i32){ m.names.push(String::from(n)); m.moved.push(false); m.aggr.push(aggregate); m.ldep.push(depth); }
pub fn mv_bind(m:&mut Mv, n:&str, aggregate:bool){ mv_bind_d(m,n,aggregate,0) }
pub fn mv_pop(m:&mut Mv){ let _=m.names.pop(); let _=m.moved.pop(); let _=m.aggr.pop(); let _=m.ldep.pop(); }

// is this expression's type a struct or enum (hence move-tracked)?
pub fn aggr_ty(ss:&Vec<Sdef>, ez:&Vec<Edef>, t:&str)->bool{
    if t.len()==0 { return false; }
    // `#[derive(Copy)]` types are NOT moved by a by-value pass -- without this the checker reports
    // use-after-move on perfectly valid Rust
    if crate::oast::is_copy_struct(ss,t) { return false; }
    crate::oast::is_struct_name(ss,t) || crate::oast::is_enum_name(ez,t) }
pub fn init_is_aggr(prog:&Prog, e:&E)->bool{
    match e {
        // a struct LITERAL is move-tracked unless its type is Copy
        E::StructLit(n,_f,_l,_h)=>!crate::oast::is_copy_struct(&prog.sdefs,n),
        E::EnumLit(_n,_v,_p,_l,_h)=>true,
        E::Call(n,_a,_l,_h)=>{ let mut i=0; while i<prog.funs.len(){ if prog.funs[i].name.as_str()==n.as_str() { return aggr_ty(&prog.sdefs,&prog.edefs,prog.funs[i].ret.as_str()); } i=i+1; } false },
        _=>false } }

pub fn check_prog(prog:&Prog, ds:&mut Vec<Diag>){
    { let mut m=mv_new(); walk(&prog.main, prog, &mut m, ds, 0); }
    let mut i=0;
    while i<prog.funs.len(){
        if prog.funs[i].dflt<0 {
            let mut m=mv_new();
            let mut p=0;
            while p<prog.funs[i].params.len(){
                let pt=if p<prog.funs[i].ptys.len() { String::from(prog.funs[i].ptys[p].as_str()) } else { String::from("") };
                // a param declared `&T`/`&mut T` is a REFERENCE binding: passing it onward is a REBORROW,
                // never a move, however many times (rustc's table, C2 axis (c) -- unbounded). The parser is the
                // only tier that still knows, because parse_tyname strips the `&`: `&T` IS `T` in the value
                // model and that stays true; this is the one bit the move checker needs.
                let isref=p<prog.funs[i].pref.len() && prog.funs[i].pref[p];
                mv_bind(&mut m, prog.funs[i].params[p].as_str(), aggr_ty(&prog.sdefs,&prog.edefs,&pt) && !isref);
                p=p+1; }
            walk(&prog.funs[i].body, prog, &mut m, ds, 0); }
        i=i+1; }
}

fn use_of(m:&mut Mv, n:&str, lo:u32, hi:u32, ds:&mut Vec<Diag>){
    let ix=mv_ix(m,n);
    if ix>=0 { let k=ix as usize; if m.aggr[k] && m.moved[k] {
        let mut msg=String::from("use of moved value `"); msg.push_str(n); msg.push_str("`");
        ds.push(crate::oerrors::err(msg, lo, hi)); } }
}
fn do_move(m:&mut Mv, n:&str, lo:u32, hi:u32, ds:&mut Vec<Diag>, depth:i32){
    let ix=mv_ix(m,n);
    if ix>=0 { let k=ix as usize;
        if m.aggr[k] {
            if m.moved[k] { let mut msg=String::from("value `"); msg.push_str(n); msg.push_str("` moved twice");
                ds.push(crate::oerrors::err(msg, lo, hi)); }
            else if depth>0 && k<m.ldep.len() && m.ldep[k]<depth { let mut msg=String::from("value `"); msg.push_str(n); msg.push_str("` moved in a loop body (moved on every iteration)");
                ds.push(crate::oerrors::err(msg, lo, hi)); }
            m.moved[k]=true; } }
}

fn walk(e:&E, prog:&Prog, m:&mut Mv, ds:&mut Vec<Diag>, depth:i32){
    match e {
        E::Var(n,l,h)=>{ use_of(m,n,*l,*h,ds); },
        E::Ref(x,_mt,_l,_h)=>{ // a BORROW is not a move: only the inner place is "used"
            match x.as_ref() { E::Var(n,l2,h2)=>{ use_of(m,n,*l2,*h2,ds); }, _=>{ walk(x,prog,m,ds,depth); } } },
        E::Call(_n,args,_l,_h)=>{
            let mut k=0;
            while k<args.len(){
                match &args[k] { E::Var(n2,l2,h2)=>{ use_of(m,n2,*l2,*h2,ds); do_move(m,n2,*l2,*h2,ds,depth); },
                                 _=>{ walk(&args[k],prog,m,ds,depth); } }
                k=k+1; } },
        E::MethodCall(r,_mm,args,_l,_h)=>{ walk(r,prog,m,ds,depth); let mut k=0; while k<args.len(){ walk(&args[k],prog,m,ds,depth); k=k+1; } },
        E::Let(n,v,b,_l,_h)=>{ walk(v,prog,m,ds,depth);
            let ag=init_is_aggr(prog,v);
            mv_bind_d(m,n,ag,depth); walk(b,prog,m,ds,depth); mv_pop(m); },
        E::LetT(n,t,v,b,_l,_h)=>{ walk(v,prog,m,ds,depth);
            mv_bind_d(m,n,aggr_ty(&prog.sdefs,&prog.edefs,t),depth); walk(b,prog,m,ds,depth); mv_pop(m); },
        E::Seq(a,b,_l,_h)=>{ walk(a,prog,m,ds,depth); walk(b,prog,m,ds,depth); },
        E::Bin(_o,a,b,_l,_h)=>{ walk(a,prog,m,ds,depth); walk(b,prog,m,ds,depth); },
        E::If(c,t,f,_l,_h)=>{ walk(c,prog,m,ds,depth); walk(t,prog,m,ds,depth); walk(f,prog,m,ds,depth); },
        E::While(c,b,_l,_h)=>{ walk(c,prog,m,ds,depth); walk(b,prog,m,ds,depth+1); },
        E::Match(sc,arms,_l,_h)=>{ walk(sc,prog,m,ds,depth); let mut a=0;
            while a<arms.len(){ let mut bi=0; while bi<arms[a].binds.len(){ mv_bind_d(m,arms[a].binds[bi].as_str(),false,depth); bi=bi+1; }
                walk(&arms[a].body,prog,m,ds,depth);
                let mut bp=0; while bp<arms[a].binds.len(){ mv_pop(m); bp=bp+1; } a=a+1; } },
        E::Field(o,_f,_l,_h)=>{ walk(o,prog,m,ds,depth); },
        E::TupIdx(o,_i,_l,_h)=>{ walk(o,prog,m,ds,depth); },
        E::Index(b,i,_l,_h)=>{ walk(b,prog,m,ds,depth); walk(i,prog,m,ds,depth); },
        E::Assign(_n,v,_l,_h)=>{ walk(v,prog,m,ds,depth); },
        E::AssignTo(p,v,_l,_h)=>{ walk(p,prog,m,ds,depth); walk(v,prog,m,ds,depth); },
        E::Not(x,_l,_h)=>{ walk(x,prog,m,ds,depth); },
        E::RetE(x,_l,_h)=>{ walk(x,prog,m,ds,depth); },
        E::Try(x,_l,_h)=>{ walk(x,prog,m,ds,depth); },
        E::TupLit(ps,_l,_h)=>{ let mut k=0; while k<ps.len(){ walk(&ps[k],prog,m,ds,depth); k=k+1; } },
        E::ArrLit(ps,_l,_h)=>{ let mut k=0; while k<ps.len(){ walk(&ps[k],prog,m,ds,depth); k=k+1; } },
        E::StructLit(_n,ps,_l,_h)=>{ let mut k=0; while k<ps.len(){ walk(&ps[k],prog,m,ds,depth); k=k+1; } },
        E::EnumLit(_n,_v,ps,_l,_h)=>{ let mut k=0; while k<ps.len(){ walk(&ps[k],prog,m,ds,depth); k=k+1; } },
        _=>{} }
}
}
use ospan::*;
use oerrors::*;
use oindex::*;
use oarena::*;
use oabi::*;
use olex::*;
use oast::*;
use oresolve::*;
use ohir::*;
use olower::*;
use omir::*;
use omirtrans::*;
use ocodegen::*;
use osession::*;
use oparse::*;
use otypeck::*;
use omono::*;
use oborrowck::*;
fn main() {
    let mut want:Vec<String>=Vec::new();
    let prog=oparse::parse_prog(olex::lex("fn main(){ 1 }"));
    omono::want_sinst(&prog, &mut want);
    println!("{}", want.len()); }
