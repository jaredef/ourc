// Box is transparent in this value model, so as_ref and as_mut are the identity on any receiver -- which
// is how a Box<Enum> is matched on and how a Box<Struct>'s fields are read.
enum E { V(i32), W }
struct S { a:i32 }
fn f(b:&Box<E>)->i32{ match b.as_ref() { E::V(n)=>*n, E::W=>0 } }
fn main(){
    let b=Box::new(E::V(7)); let w=Box::new(E::W);
    let s=Box::new(S{a:5}); let r=s.as_ref();
    let mut m=Box::new(S{a:1}); { let q=m.as_mut(); q.a=9; }
    println!("{} {} {} {}", f(&b), f(&w), r.a, m.a); }
