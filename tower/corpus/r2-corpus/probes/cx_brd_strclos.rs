// P-BRD-CLOSURE-STR-PARAM: a closure's &str param, typed by its annotation or by the callee's `F: Fn(&str) -> R` bound.
fn ap<F: Fn(&str) -> String>(s: &str, f: F) -> String { f(s) }
fn count<F: Fn(&str) -> usize>(s: &str, f: F) -> usize { f(s) }
fn main() {
    let twice = |n: &str| n.repeat(2);
    println!("{}", twice("ab"));
    println!("{} {}", ap("dog", |n| format!("{}!", n)), ap("x", |n| n.to_uppercase()));
    println!("{}", count("hello", |n| n.len()) + ap("abc", |n| n.to_string()).len());
}
