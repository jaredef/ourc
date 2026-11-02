fn main() {
    let k = 3;
    let f: fn(i32) -> i32 = |x| x + 1;
    let g = |x: i32| x + k;
    println!("{} {}", f(1), g(1));
}
