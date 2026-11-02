// a mini recursive-descent parser: tokenize -> Parser{toks,pos} -> parse_expr -> Expr AST -> eval
enum Expr { Num(i32), Add(Box<Expr>, Box<Expr>), Mul(Box<Expr>, Box<Expr>) }
impl Expr { fn eval(&self) -> i32 { match self { Expr::Num(n) => *n, Expr::Add(a,b) => a.eval()+b.eval(), Expr::Mul(a,b) => a.eval()*b.eval() } } }
struct Parser { toks: Vec<i32>, pos: usize }   // toks: >=0 is a number; -1 '+', -2 '*', -3 '(', -4 ')'
impl Parser {
    fn new(toks: Vec<i32>) -> Parser { Parser { toks: toks, pos: 0 } }
    fn peek(&self) -> i32 { if self.pos < self.toks.len() { self.toks[self.pos] } else { -99 } }
    fn bump(&mut self) -> i32 { let t = self.peek(); self.pos = self.pos + 1; t }
    fn parse_atom(&mut self) -> Box<Expr> {
        let t = self.bump();
        if t == -3 { let e = self.parse_expr(); self.bump(); e } else { Box::new(Expr::Num(t)) }
    }
    fn parse_term(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_atom();
        while self.peek() == -2 { self.bump(); let rhs = self.parse_atom(); lhs = Box::new(Expr::Mul(lhs, rhs)); }
        lhs
    }
    fn parse_expr(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_term();
        while self.peek() == -1 { self.bump(); let rhs = self.parse_term(); lhs = Box::new(Expr::Add(lhs, rhs)); }
        lhs
    }
}
fn main() {
    // "2 + 3 * 4" = 14 ; "( 2 + 3 ) * 4" = 20
    let mut p = Parser::new(vec![2, -1, 3, -2, 4]);
    println!("{}", p.parse_expr().eval());
    let mut q = Parser::new(vec![-3, 2, -1, 3, -4, -2, 4]);
    println!("{}", q.parse_expr().eval());
}
