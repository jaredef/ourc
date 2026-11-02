struct S<'a> { n: i32, s: &'a str }
const C: &str = "c";
fn f(s: &str, t: &str) -> usize { s.len() + t.len() }
fn main() {
    let x: &str = "ab";
    let v = S { n: 1, s: "xyz" };
    println!("{} {} {}", f(x, "q"), v.n + v.s.len() as i32, C);
}
