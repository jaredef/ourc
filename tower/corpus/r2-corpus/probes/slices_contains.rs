fn main() {
    let a = [2, 4, 6, 8];
    println!("{}", a.contains(&6) as i32 + a.contains(&5) as i32);
}
