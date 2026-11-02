fn twice(f: impl Fn(i32) -> i32, x: i32) -> i32 { f(f(x)) }
fn square(n: i32) -> i32 { n * n }
fn main() {
    let inc = |x| x + 1;
    println!("{}", twice(inc, 10));
    let compose = |x| square(x) + 1;
    println!("{}", compose(4));
    let v = vec![1, 2, 3, 4];
    let sum_plus = |base| {
        let mut s = base;
        let mut i = 0;
        while i < v.len() { s = s + v[i]; i = i + 1; }
        s
    };
    println!("{}", sum_plus(100));
    let mut acc = 0;
    for k in 0..5 {
        let f = |x| x * k;
        acc = acc + f(2);
    }
    println!("{}", acc);
}
