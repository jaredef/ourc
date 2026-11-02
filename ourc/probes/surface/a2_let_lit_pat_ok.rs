fn main() {
    let o: Option<i32> = Some(3);
    let Some(v) = o else { return; };
    let none_seen = true;
    println!("{} {}", v, none_seen);
}
