fn main() {
    let a = [10, 20, 30, 40, 50];
    let s = &a[1..4];
    println!("{}", s.iter().sum::<i32>());
}
