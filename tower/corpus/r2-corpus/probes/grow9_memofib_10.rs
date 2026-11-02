use std::collections::HashMap;
fn fib(n: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if n < 2 { return n; }
    if let Some(v) = memo.get(&n) { return *v; }
    let r = fib(n-1, memo) + fib(n-2, memo);
    memo.insert(n, r);
    r
}
fn main() {
    let mut m: HashMap<i64, i64> = HashMap::new();
    println!("{}", fib(30, &mut m));
}
