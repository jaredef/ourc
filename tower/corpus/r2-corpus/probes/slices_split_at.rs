fn main() {
    let a = [1, 2, 3, 4, 5];
    let (l, r) = a.split_at(2);
    let ls: i32 = l.iter().sum();
    let rs: i32 = r.iter().sum();
    println!("{}", ls * 100 + rs);
}
