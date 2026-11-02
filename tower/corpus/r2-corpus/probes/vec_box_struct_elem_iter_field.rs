struct Expr { v: i64 }
fn main() {
    let mut vv: Vec<Box<Expr>> = Vec::new();
    vv.push(Box::new(Expr{v:10})); vv.push(Box::new(Expr{v:20}));
    let mut s = 0;
    for a in vv.iter() { s = s + a.v; }
    println!("{}", s);
}
