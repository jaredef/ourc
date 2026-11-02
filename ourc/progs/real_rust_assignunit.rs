// An assignment expression is () in rustc, so a field or index assignment used as a statement inside an
// if-without-else must agree with the implicit else rather than carry the assigned value's type.
struct P { a:i32, s:String }
fn main(){
    let mut p=P{a:1, s:String::from("")};
    let mut v=vec![String::from("x"), String::from("y")];
    if p.a>0 { p.s=String::from("set"); }
    if v.len()>1 { v[1]=String::from("z"); }
    let mut k=0;
    while k<v.len(){ if v[k].len()>0 { p.a=p.a+1; } k=k+1; }
    println!("{} {} {}", p.a, p.s, v[1]); }
