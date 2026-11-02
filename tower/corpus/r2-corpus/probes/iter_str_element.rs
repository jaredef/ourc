// for w in <&str array>.iter() -- regression guard for TI3e-m (N-iter-str-element):
// .iter() over a bound &str array yields &&str; the loop var was left kind-0, so
// *w did a real eload and w=="lit"/entry(w) used a pointer compare. Now the array
// element interned Ty threads via SARRVTY -> loop var tagged &str, *w a no-op.
use std::collections::HashMap;
fn main() {
    let words = ["a", "b", "a", "c", "b", "a"];
    let mut count = 0;
    for w in words.iter() { if *w == "a" { count += 1; } }
    println!("{}", count);
    let mut m: HashMap<&str, i32> = HashMap::new();
    for w in words.iter() { *m.entry(w).or_insert(0) += 1; }
    println!("{} {} {}", m["a"], m["b"], m["c"]);
    // int .iter() and by-value still work (no regression)
    let nums = [10, 20, 30];
    let mut s = 0;
    for n in nums.iter() { s += *n; }
    println!("{}", s);
}
