// Vec::new() carries no element type, so it is compatible with any typed Vec: as a reassignment to a
// typed local, to a struct field, and as one arm of an if whose other arm is a typed Vec.
struct S { v:Vec<i32> }
fn main(){
    let mut v:Vec<String>=Vec::new();
    v.push(String::from("a")); v=Vec::new(); v.push(String::from("b"));
    let mut s=S{v:Vec::new()}; s.v.push(1); s.v=Vec::new(); s.v.push(7);
    let c=true; let w:Vec<i32>=if c { vec![1,2,3] } else { Vec::new() };
    println!("{} {} {} {}", v.len(), v[0], s.v[0], w.len()); }
