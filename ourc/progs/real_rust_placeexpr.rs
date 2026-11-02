// C-PLACEEXPR: a place is a NAME, a FIELD of a place, or an INDEX of one. The mem desugars knew only
// names, so swapping two fields -- ordinary Rust -- was refused.
struct P { a:i64, b:i64 }
struct I { n:i64 }
struct O { i:I, j:I }
fn main(){ let mut p=P{a:1,b:2}; std::mem::swap(&mut p.a, &mut p.b);
    let mut o=O{ i:I{n:3}, j:I{n:4} }; std::mem::swap(&mut o.i.n, &mut o.j.n);
    println!("{} {} {} {}", p.a, p.b, o.i.n, o.j.n); }
