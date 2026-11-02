fn main() {
    let mut x: Option<i32> = None;
    let v = x.get_or_insert(8);
    println!("{}", *v);
}
