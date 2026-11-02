use std::collections::HashMap;
#[derive(Clone)]
enum Tok { Num(i64), Ident(String), Plus, Star, Assign, End }
enum Expr { Num(i64), Var(String), Bin(Box<Expr>, u8, Box<Expr>), Assign(String, Box<Expr>) }
struct P { toks: Vec<Tok>, pos: usize }
impl P {
    fn peek(&self) -> Tok { self.toks[self.pos].clone() }
    fn bump(&mut self) -> Tok { let t = self.toks[self.pos].clone(); self.pos += 1; t }
    fn parse_expr(&mut self) -> Box<Expr> {
        if let Tok::Ident(name) = self.peek() {
            let nxt = self.toks[self.pos + 1].clone();
            if let Tok::Assign = nxt { self.bump(); self.bump(); let rhs = self.parse_expr(); return Box::new(Expr::Assign(name, rhs)); }
        }
        self.parse_add()
    }
    fn parse_add(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_mul();
        loop { match self.peek() { Tok::Plus => { self.bump(); let r = self.parse_mul(); lhs = Box::new(Expr::Bin(lhs, 43, r)); } _ => break, } }
        lhs
    }
    fn parse_mul(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_atom();
        loop { match self.peek() { Tok::Star => { self.bump(); let r = self.parse_atom(); lhs = Box::new(Expr::Bin(lhs, 42, r)); } _ => break, } }
        lhs
    }
    fn parse_atom(&mut self) -> Box<Expr> { match self.bump() { Tok::Num(n) => Box::new(Expr::Num(n)), Tok::Ident(s) => Box::new(Expr::Var(s)), _ => Box::new(Expr::Num(0)) } }
}
fn eval(e: &Expr, env: &mut HashMap<String, i64>) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Var(name) => { if env.contains_key(name) { env[name] } else { 0 } }
        Expr::Bin(a, op, b) => { let x = eval(a, env); let y = eval(b, env); if *op == 43 { x + y } else { x * y } }
        Expr::Assign(name, rhs) => { let v = eval(rhs, env); env.insert(name.clone(), v); v }
    }
}
fn main() {
    // x = 3 + 4 * 2  -> tokens
    let mut p = P { toks: vec![Tok::Ident(String::from("x")), Tok::Assign, Tok::Num(3), Tok::Plus, Tok::Num(4), Tok::Star, Tok::Num(2), Tok::End], pos: 0 };
    let ast = p.parse_expr();
    let mut env: HashMap<String, i64> = HashMap::new();
    println!("{}", eval(&ast, &mut env));
}
