// R2a P4 guard: a block whose last statement ends in `;` has type `()` -- a tail `if` whose branches are such blocks
// (one a String-returning call, one an assignment) agrees; a diverging tail (`return x;`) keeps its value.
fn dedup(mut v: Vec<String>, nm: &str) -> usize {
    let mut k = 0;
    while k < v.len() {
        if v[k].as_str() == nm {
            v.remove(k);
        } else {
            k = k + 1;
        }
    }
    v.len()
}
fn pick(s: String, c: bool) -> String {
    if c {
        return s;
    }
    String::from("other")
}
fn main() {
    let v = vec![String::from("a"), String::from("b"), String::from("a")];
    let mut n = 0;
    for x in [1, 2, 3] { if x > 1 { n = n + x; } else { String::from("skip"); } }
    println!("{} {} {} {}", dedup(v, "a"), pick(String::from("s"), true), pick(String::from("s"), false), n);
}
