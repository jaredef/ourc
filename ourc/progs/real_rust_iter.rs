// P-ITER: an iterator IS a vector here (eager), so `.iter()` is the identity and each adaptor is a
// prelude function written in our own language. A bounded range materialises; `*x` is `x` (C2).
fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{}", v.iter().sum::<i32>());
    println!("{}", v.iter().map(|x| x * 2).sum::<i32>());
    println!("{}", v.iter().filter(|x| **x % 2 == 1).count());
    println!("{}", v.iter().fold(10, |a, b| a + b));
    println!("{}", (1..5).sum::<i32>());
    let w: Vec<i32> = (1..4).map(|x| x * x).collect();
    println!("{} {} {}", w[0], w[1], w[2]);
    println!("{} {}", v.iter().any(|x| *x > 3), v.iter().all(|x| *x > 0));
    let r: Vec<i32> = v.iter().rev().map(|x| *x).collect();
    println!("{} {}", r[0], r[3]);
}
