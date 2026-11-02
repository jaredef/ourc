type P<T> = Box<T>;
type ThinVec<T> = Vec<T>;
#[derive(Clone)]
enum Tok { Num(i64), Plus, Star, LParen, RParen, LBrack, RBrack, End }
enum TokenTree { Leaf(u8), Group(u8, ThinVec<P<TokenTree>>) }
fn lex(src: &str) -> Vec<Tok> {
    let b = src.as_bytes(); let mut i = 0; let mut out: Vec<Tok> = Vec::new();
    while i < b.len() { let c = b[i];
        if c == 32 { i += 1; continue; }
        if c >= 48 && c <= 57 { let mut n: i64 = 0; while i < b.len() && b[i] >= 48 && b[i] <= 57 { n = n*10 + (b[i]-48) as i64; i += 1; } out.push(Tok::Num(n)); continue; }
        if c == 43 { out.push(Tok::Plus); i += 1; continue; }
        if c == 42 { out.push(Tok::Star); i += 1; continue; }
        if c == 40 { out.push(Tok::LParen); i += 1; continue; }
        if c == 41 { out.push(Tok::RParen); i += 1; continue; }
        if c == 91 { out.push(Tok::LBrack); i += 1; continue; }
        if c == 93 { out.push(Tok::RBrack); i += 1; continue; }
        i += 1;
    }
    out.push(Tok::End); out
}
struct Cursor { toks: Vec<Tok>, pos: usize }
impl Cursor {
    fn peek(&self) -> Tok { self.toks[self.pos].clone() }
    fn bump(&mut self) -> Tok { let t = self.toks[self.pos].clone(); self.pos += 1; t }
    fn parse_stream(&mut self, close: u8) -> ThinVec<P<TokenTree>> {
        let mut out: ThinVec<P<TokenTree>> = Vec::new();
        loop {
            let t = self.peek();
            match t {
                Tok::End => break,
                Tok::RParen => { if close == 41 { self.bump(); } break; }
                Tok::RBrack => { if close == 93 { self.bump(); } break; }
                Tok::LParen => { self.bump(); let inner = self.parse_stream(41); out.push(Box::new(TokenTree::Group(40, inner))); }
                Tok::LBrack => { self.bump(); let inner = self.parse_stream(93); out.push(Box::new(TokenTree::Group(91, inner))); }
                Tok::Num(_n) => { self.bump(); out.push(Box::new(TokenTree::Leaf(110))); }
                Tok::Plus => { self.bump(); out.push(Box::new(TokenTree::Leaf(43))); }
                Tok::Star => { self.bump(); out.push(Box::new(TokenTree::Leaf(42))); }
                _ => { self.bump(); }
            }
        }
        out
    }
}
fn dump_tt(tt: &TokenTree) -> String {
    match tt {
        TokenTree::Leaf(c) => format!("{}", *c as char),
        TokenTree::Group(d, kids) => {
            let open = *d as char;
            let close = if *d == 40 { ')' } else { ']' };
            let mut s = String::new();
            s.push(open);
            let mut first = true;
            for k in kids.iter() { if !first { s.push(' '); } first = false; s.push_str(&dump_tt(k)); }
            s.push(close);
            s
        }
    }
}
fn dump_stream(ts: &ThinVec<P<TokenTree>>) -> String {
    let mut s = String::new(); let mut first = true;
    for k in ts.iter() { if !first { s.push(' '); } first = false; s.push_str(&dump_tt(k)); }
    s
}
fn main() {
    let inputs = ["1 + 2", "( 1 + 2 ) * 3", "[ 1 + ( 2 * 3 ) ]", "( ( 1 ) )"];
    for src in inputs.iter() {
        let toks = lex(src);
        let mut c = Cursor { toks: toks, pos: 0 };
        let ts = c.parse_stream(0);
        println!("{}", dump_stream(&ts));
    }
}
