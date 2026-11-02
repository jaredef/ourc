enum Tok { Num(i64), Ident(String), Plus, Minus, Star, Slash, LParen, RParen, Assign, Semi, End }
enum Expr { Num(i64), Var(String), Bin(Box<Expr>, u8, Box<Expr>), Assign(String, Box<Expr>) }
fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Var(_s) => 0,
        Expr::Bin(a, _op, b) => eval(a) + eval(b),
        Expr::Assign(_n, rhs) => eval(rhs),
    }
}
fn main() { let _t = Tok::End; let e = Expr::Num(7); println!("{}", eval(&e)); }
