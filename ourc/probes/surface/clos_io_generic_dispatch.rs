fn ap<F: Fn(i64) -> i64>(f: F, x: i64) -> i64 { f(x) }
fn main(){ let k = 3; let p = |x: i64| { println!("p{}", x); x + k }; let q = |x: i64| x * k; println!("{} {}", ap(p, 1), ap(q, 2)); }
