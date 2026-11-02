// L11 P-ASBYTES-ARG-LEAK-STRUCT-RET regression: a `.as_bytes()` arg (a &[u8] byte slice) must NOT
// leak its byte-slice tag (EISBYTES) onto a struct-returning free fn's call result. Before the fix
// emit_call omitted EISBYTES from its post-arg flag reset, so `let t = mk(src.as_bytes())` bound t
// byte-typed not struct-typed -> `t.kind.len()` emitted garbage + a stray `call len`. Also covers the
// Vec<enum-payload> facet (a lex returning Vec<Tok>, match+clone-iterated), which was the same leak.
#[derive(Clone)]
enum Tok { Ident(u32), Int(i64), Semi, Eof }
struct Out { kind: Vec<u32>, sum: i64 }
fn lex(src: &[u8]) -> Vec<Tok> {
    let mut t: Vec<Tok> = Vec::new(); let n = src.len(); let mut i = 0;
    while i < n { let c = src[i];
        if c == 32 { i = i + 1; }
        else if c >= 48 && c <= 57 { t.push(Tok::Int(c as i64 - 48)); i = i + 1; }
        else if c >= 97 && c <= 122 { t.push(Tok::Ident(c as u32)); i = i + 1; }
        else { if c == 59 { t.push(Tok::Semi); } i = i + 1; } }
    t.push(Tok::Eof); t
}
fn build(toks: &Vec<Tok>) -> Out {
    let mut o = Out { kind: Vec::new(), sum: 0 };
    let mut p = 0;
    while p < toks.len() { let tk = toks[p].clone();
        match tk { Tok::Int(v) => { o.sum = o.sum + v; o.kind.push(1); } Tok::Ident(_) => { o.kind.push(2); } _ => {} }
        p = p + 1; }
    o
}
fn main() {
    let text = "a 1 2 ; b 5 ;";
    let src = text.as_bytes();
    let toks = lex(src);
    let o = build(&toks);
    println!("{} {}", o.kind.len(), o.sum);   // struct field .len() + scalar field, both after a .as_bytes()-fed call chain
}
