// WITNESS for rung 57's refutation: `push` on a receiver lowering cannot NAME. Routing it to vec_push
// (which rung 57 tried) ACCEPTS this program and prints "3 885703360 49" where rustc prints "3 9 7" -- an
// unnamed receiver is not guaranteed to be a Vec handle, so the push corrupts. Left refused instead.
fn pick()->(Vec<i32>, i32){ let mut v:Vec<i32>=Vec::new(); v.push(1); (v, 9) }
fn main(){ let (mut a, n)=pick(); a.push(n); a.push(7); println!("{} {} {}", a.len(), a[1], a[2]); }
