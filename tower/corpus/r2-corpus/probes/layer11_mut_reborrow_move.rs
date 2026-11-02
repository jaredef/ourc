struct Body { v: Vec<i64> }
fn cp(b: &mut Body) -> bool { b.v[0] = b.v[0] + 1; b.v[0] < 3 }
fn dce(b: &mut Body) -> bool { b.v[0] = b.v[0] + 10; false }
fn optimize(b: &mut Body) { let mut any = true; while any { let c1 = cp(b); let c2 = dce(b); any = c1 || c2; } }
fn main() { let mut b = Body { v: Vec::new() }; b.v.push(0); optimize(&mut b); println!("{}", b.v[0]); }
