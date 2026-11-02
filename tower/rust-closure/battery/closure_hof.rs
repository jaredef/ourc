fn map_sum(v: &Vec<i32>, f: impl Fn(i32) -> i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < v.len() { s = s + f(v[i]); i = i + 1; }
    s
}
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("sum of squares = {}", map_sum(&v, |x| x * x));
    println!("sum doubled = {}", map_sum(&v, |x| x * 2));
    let bias = 10;
    println!("sum + bias each = {}", map_sum(&v, |x| x + bias));
}
