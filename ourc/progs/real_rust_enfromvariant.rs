// A match's scrutinee may carry no recorded type -- here it is itself a payload binding, reached through
// a Box -- and the matched ENUM was then unknown, so every arm binding got no payload type and every
// field read on one failed. The variant NAMES identify the enum, and oast::enum_of_variant had existed all
// along without either match site calling it.
struct A { k:i32, v:Vec<i32> }
enum E { Leaf(A), Node(Box<E>, A) }
fn sum(e:&E)->i32{
    match e { E::Leaf(a)=>a.k + a.v.len() as i32,
              E::Node(inner, a)=>a.k + sum(inner) } }
fn main(){
    let mut v:Vec<i32>=Vec::new(); v.push(1); v.push(2);
    let leaf=E::Leaf(A{k:10, v:v});
    let node=E::Node(Box::new(leaf), A{k:100, v:Vec::new()});
    println!("{}", sum(&node)); }
