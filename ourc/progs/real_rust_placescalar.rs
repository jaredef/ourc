// a place param handed STRAIGHT ON to another place function: the call-site rewrite takes a bare name as
// well as a `&mut` expression, so `inner(x)` inside `outer` becomes `x = inner(x)`
fn inner(x:&mut i64){ *x += 1; }
fn outer(x:&mut i64){ inner(x); *x += 10; }
fn main(){ let mut n=0; outer(&mut n); println!("{}",n); }
