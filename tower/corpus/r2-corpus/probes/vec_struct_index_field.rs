struct Expr { a: i32, b: i32 }
fn main() { let mut vv: Vec<Expr> = Vec::new(); vv.push(Expr{a:1,b:10}); vv.push(Expr{a:2,b:20}); println!("{}", vv[0].b + vv[1].b); }
