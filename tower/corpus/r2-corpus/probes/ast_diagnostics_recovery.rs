type P<T> = Box<T>;
struct ParseError { msg: String, pos: u32 }
enum ExprKind { Lit(i64), Bin(u8, P<Expr>, P<Expr>), Error }
struct Expr { kind: ExprKind }
struct Parser { src: Vec<u8>, pos: usize, errors: Vec<ParseError> }
impl Parser {
    fn peek(&self) -> u8 { if self.pos < self.src.len() { self.src[self.pos] } else { 0 } }
    fn skip_ws(&mut self) { while self.pos < self.src.len() && self.src[self.pos] == 32 { self.pos += 1; } }
    fn err(&mut self, m: &str) { self.errors.push(ParseError { msg: String::from(m), pos: self.pos as u32 }); }
    fn parse_atom(&mut self) -> P<Expr> {
        self.skip_ws();
        let c = self.peek();
        if c >= 48 && c <= 57 {
            let mut n: i64 = 0;
            while self.pos < self.src.len() && self.src[self.pos] >= 48 && self.src[self.pos] <= 57 { n = n * 10 + (self.src[self.pos] - 48) as i64; self.pos += 1; }
            return Box::new(Expr { kind: ExprKind::Lit(n) });
        }
        self.err("expected number");
        self.pos += 1;
        Box::new(Expr { kind: ExprKind::Error })
    }
    fn parse_expr(&mut self) -> P<Expr> {
        let mut lhs = self.parse_atom();
        loop {
            self.skip_ws();
            let c = self.peek();
            if c != 43 && c != 42 { break; }
            self.pos += 1;
            let rhs = self.parse_atom();
            lhs = Box::new(Expr { kind: ExprKind::Bin(c, lhs, rhs) });
        }
        lhs
    }
}
fn dump(e: &Expr) -> String {
    match &e.kind {
        ExprKind::Lit(n) => format!("{}", n),
        ExprKind::Error => String::from("<err>"),
        ExprKind::Bin(op, a, b) => format!("({} {} {})", *op as char, dump(a), dump(b)),
    }
}
fn main() {
    let src = "1 + @ * 3";
    let mut p = Parser { src: src.as_bytes().to_vec(), pos: 0, errors: Vec::new() };
    let ast = p.parse_expr();
    println!("{}", dump(&ast));
    for e in p.errors.iter() { println!("error at {}: {}", e.pos, e.msg); }
}
