// Vec<String> built from String::from, iterated by ref, summing element lengths.
// Regression guard for TI3e: the Vec<String> binding must stay Vec-kind (not be
// contaminated to String-kind by EISSTRING bubbling from the last element), so
// `&v` threads the element Ty and `s.len()` reads the heap-String len at offset 4.
fn main() {
    let v = vec![String::from("ab"), String::from("cde"), String::from("f")];
    let mut t = 0;
    for s in &v { t = t + s.len(); }
    println!("{}", t);
    let mut joined = String::new();
    for s in &v { joined.push_str(s); }
    println!("{}", joined);
    println!("{}", v.len());
}
