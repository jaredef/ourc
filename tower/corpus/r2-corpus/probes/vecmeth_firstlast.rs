fn main() {
    let v = vec![7, 8, 9];
    println!("{}", v.first().unwrap() + v.last().unwrap());
}
