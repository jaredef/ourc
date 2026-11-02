// The second half of C-TYRESOLVE. A PARAMETER's recorded type is the bare name its own module wrote
// (local_sty keeps it verbatim), so the field lookup needs the same bare-to-qualified resolution the
// expression-type chain got in rung 71. Rung 70 put this in the field helpers first and it looked inert,
// because back then the type was EMPTY and no name reached that far.
mod a { pub struct T { pub k:i32, pub lo:i32, pub hi:i32 }
        pub struct V { pub k:i32, pub v:Vec<i32> } }
mod b { use crate::a::*;
        pub fn rd(t:&T)->i32{ t.k*100 + t.lo*10 + t.hi }
        pub fn rv(x:&V)->i32{ x.k + x.v.len() as i32 } }
use a::*;
fn main(){
    let t=T{k:1, lo:2, hi:3};
    let mut x=V{k:5, v:Vec::new()}; x.v.push(9);
    println!("{} {}", b::rd(&t), b::rv(&x)); }
