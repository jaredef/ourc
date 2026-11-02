fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 { f(x) }
fn main() {
    let double = |x| x * 2;
    println!("double 5 = {}", double(5));
    let n = 10;
    let add_n = |x| x + n;
    println!("add_n 7 = {}", add_n(7));
    println!("apply double 8 = {}", apply(double, 8));
    println!("apply inline = {}", apply(|x| x * x, 6));
    let base = 100;
    let total = apply(|x| x + base, 23);
    println!("total = {}", total);
    let add = |a, b| a + b;
    println!("add 3 4 = {}", add(3, 4));
}
