struct P { a: i32, b: i32 }
enum Ex { Num(i32), Pair(i32, i32) }
fn sum(e: &Ex) -> i32 { match e { Ex::Num(v) => *v, Ex::Pair(a, _b) => *a } }
fn main() {
    let mut p = P { a: 1, b: 2 };
    let mut i = 0;
    while i < 3 {
        p = P { a: p.b, b: p.a + p.b };
        i += 1;
    }
    let mut s = String::from("a");
    let mut v = vec![1];
    for _ in 0..3 {
        s.push_str("b");
    }
    for k in 0..2 {
        v = vec![v[0] + k, 9];
    }
    let mut e = Ex::Num(1);
    for k in 0..2 {
        e = Ex::Pair(sum(&e), k);
    }
    println!("{} {} {} {} {:?}", p.a, p.b, s, sum(&e), v);
}
