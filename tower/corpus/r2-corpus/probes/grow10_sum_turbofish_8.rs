fn main() {
    let v = [10i32, 20, 30];
    let s = v.iter().sum::<i32>();
    let p = v.iter().product::<i32>();
    println!("{} {}", s, p);
}
