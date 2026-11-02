struct Expr { v: i64 }
fn tot(args: &Vec<Box<Expr>>) -> i64 { let mut s = 0; for a in args.iter() { s = s + a.v; } s }
fn main() { let mut v: Vec<Box<Expr>> = Vec::new(); v.push(Box::new(Expr{v:1})); v.push(Box::new(Expr{v:2})); println!("{}", tot(&v)); }
