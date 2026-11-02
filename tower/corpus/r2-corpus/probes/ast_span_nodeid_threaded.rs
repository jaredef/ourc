type P<T> = Box<T>;
#[derive(Clone, Copy)]
struct Span { lo: u32, hi: u32 }
enum ExprKind { Lit(i64), Bin(u8, P<Expr>, P<Expr>) }
struct Expr { kind: ExprKind, span: Span, id: u32 }
struct Parser { src: Vec<u8>, pos: usize, next_id: u32 }
impl Parser {
    fn mk_id(&mut self) -> u32 { let i = self.next_id; self.next_id += 1; i }
    fn skip_ws(&mut self) { while self.pos < self.src.len() && self.src[self.pos] == 32 { self.pos += 1; } }
    fn parse_atom(&mut self) -> P<Expr> {
        self.skip_ws();
        let lo = self.pos as u32;
        let mut n: i64 = 0;
        while self.pos < self.src.len() && self.src[self.pos] >= 48 && self.src[self.pos] <= 57 {
            n = n * 10 + (self.src[self.pos] - 48) as i64;
            self.pos += 1;
        }
        let hi = self.pos as u32;
        let id = self.mk_id();
        Box::new(Expr { kind: ExprKind::Lit(n), span: Span { lo: lo, hi: hi }, id: id })
    }
    fn parse_expr(&mut self, min_prec: u8) -> P<Expr> {
        let mut lhs = self.parse_atom();
        loop {
            self.skip_ws();
            if self.pos >= self.src.len() { break; }
            let c = self.src[self.pos];
            let prec: u8 = if c == 43 { 1 } else if c == 42 { 2 } else { 0 };
            if prec == 0 || prec < min_prec { break; }
            self.pos += 1;
            let rhs = self.parse_expr(prec + 1);
            let lo = lhs.span.lo;
            let hi = rhs.span.hi;
            let id = self.mk_id();
            lhs = Box::new(Expr { kind: ExprKind::Bin(c, lhs, rhs), span: Span { lo: lo, hi: hi }, id: id });
        }
        lhs
    }
}
fn dump(e: &Expr) -> String {
    match &e.kind {
        ExprKind::Lit(n) => format!("{}@{}..{}#{}", n, e.span.lo, e.span.hi, e.id),
        ExprKind::Bin(op, a, b) => format!("({} {} {})@{}..{}#{}", *op as char, dump(a), dump(b), e.span.lo, e.span.hi, e.id),
    }
}
fn main() {
    let inputs = ["1 + 2 * 3", "10 + 20"];
    for src in inputs.iter() {
        let mut p = Parser { src: src.as_bytes().to_vec(), pos: 0, next_id: 0 };
        let ast = p.parse_expr(0);
        println!("{}", dump(&ast));
    }
}
