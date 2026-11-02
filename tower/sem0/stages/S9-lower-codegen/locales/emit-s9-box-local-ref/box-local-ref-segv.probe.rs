struct P { n: i32, kid: Vec<P> }
fn depth(p: &P) -> i32 { let mut d = 0; for k in p.kid.iter() { let kd = depth(k); if kd > d { d = kd; } } d + 1 }
fn main() {
    let leaf: Box<P> = Box::new(P { n: 1, kid: Vec::new() });
    println!("{}", depth(&leaf));
}
