fn main() {
    let v = vec![7, 8, 9];
    let f = *v.first().unwrap();
    let l = *v.last().unwrap();
    println!("{}", f * 10 + l);
}
