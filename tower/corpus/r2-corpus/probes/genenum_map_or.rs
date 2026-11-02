fn main() {
    let x: Option<i32> = Some(3);
    println!("{}", x.map_or(0, |v| v + 1));
}
