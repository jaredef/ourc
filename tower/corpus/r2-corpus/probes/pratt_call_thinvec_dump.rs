type P<T> = Box<T>;
type ThinVec<T> = Vec<T>;
#[derive(Clone)]
enum Tok { Num(i64), Plus, Star, LParen, RParen, Comma, Ident, End }
enum ExprKind { Lit(i64), Bin(u8, P<Expr>, P<Expr>), Call(ThinVec<P<Expr>>) }
struct Expr { kind: ExprKind, lo: u32, hi: u32 }
fn lex(src: &str) -> Vec<Tok> {
    let b = src.as_bytes(); let mut i = 0; let mut out: Vec<Tok> = Vec::new();
    while i < b.len() { let c = b[i];
        if c == 32 { i += 1; continue; }
        if c >= 48 && c <= 57 { let mut n: i64 = 0; while i < b.len() && b[i] >= 48 && b[i] <= 57 { n = n*10 + (b[i]-48) as i64; i += 1; } out.push(Tok::Num(n)); continue; }
        if c == 43 { out.push(Tok::Plus); i += 1; continue; }
        if c == 42 { out.push(Tok::Star); i += 1; continue; }
        if c == 40 { out.push(Tok::LParen); i += 1; continue; }
        if c == 41 { out.push(Tok::RParen); i += 1; continue; }
        if c == 44 { out.push(Tok::Comma); i += 1; continue; }
        if c == 102 { out.push(Tok::Ident); i += 1; continue; }
        i += 1;
    }
    out.push(Tok::End); out
}
struct Parser { toks: Vec<Tok>, pos: usize }
impl Parser {
    fn peek(&self) -> Tok { self.toks[self.pos].clone() }
    fn bump(&mut self) -> Tok { let t = self.toks[self.pos].clone(); self.pos += 1; t }
    fn prec(&self, t: &Tok) -> u8 { match t { Tok::Plus => 1, Tok::Star => 2, _ => 0 } }
    fn op_byte(&self, t: &Tok) -> u8 { match t { Tok::Plus => 43, Tok::Star => 42, _ => 0 } }
    fn parse_expr(&mut self, min_prec: u8) -> P<Expr> {
        let mut lhs = self.parse_atom();
        loop { let t = self.peek(); let p = self.prec(&t); if p == 0 || p < min_prec { break; }
            let op = self.op_byte(&t); self.bump(); let rhs = self.parse_expr(p + 1);
            lhs = Box::new(Expr { kind: ExprKind::Bin(op, lhs, rhs), lo: 0, hi: 0 }); }
        lhs
    }
    fn parse_atom(&mut self) -> P<Expr> {
        match self.bump() {
            Tok::Num(n) => Box::new(Expr { kind: ExprKind::Lit(n), lo: 0, hi: 0 }),
            Tok::Ident => {
                let mut args: ThinVec<P<Expr>> = Vec::new();
                if let Tok::LParen = self.peek() { self.bump();
                    loop { if let Tok::RParen = self.peek() { self.bump(); break; }
                        args.push(self.parse_expr(0));
                        if let Tok::Comma = self.peek() { self.bump(); } else { self.bump(); break; } } }
                Box::new(Expr { kind: ExprKind::Call(args), lo: 0, hi: 0 })
            }
            _ => Box::new(Expr { kind: ExprKind::Lit(0), lo: 0, hi: 0 })
        }
    }
}
fn dump(e: &Expr) -> String {
    match &e.kind {
        ExprKind::Lit(n) => format!("{}", n),
        ExprKind::Bin(op, a, b) => format!("({} {} {})", *op as char, dump(a), dump(b)),
        ExprKind::Call(args) => { let mut s = String::new(); s.push_str("(call"); for a in args.iter() { s.push_str(" "); s.push_str(&dump(a)); } s.push_str(")"); s }
    }
}
fn main() {
    let inputs = ["1 + 2 * 3", "f(1, 2)", "f(1 + 2, 3 * 4)", "f()"];
    for src in inputs.iter() {
        let toks = lex(src);
        let mut p = Parser { toks: toks, pos: 0 };
        let ast = p.parse_expr(0);
        println!("{}", dump(&ast));
    }
}
