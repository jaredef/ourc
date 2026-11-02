// C-PUSHCOERCE: the element coercion lived on the CALL form of a vector push and not on the METHOD form,
// so pushing a trait object stored the bare value instead of its {tag, payload} record -- and every
// element then dispatched to the FIRST impl of the trait. Accepted, and wrong.
trait T { fn n(&self)->i64; }
struct A; struct B;
impl T for A { fn n(&self)->i64{ 1 } }
impl T for B { fn n(&self)->i64{ 2 } }
fn main(){ let mut v: Vec<Box<dyn T>> = Vec::new(); v.push(Box::new(A)); v.push(Box::new(B));
    let mut s=0; for x in v.iter() { s=s+x.n(); }
    println!("{} {} {}", v[0].n(), v[1].n(), s); }
