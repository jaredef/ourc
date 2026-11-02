fn main() {
    let v = vec![10, 20, 30, 40, 50];
    let p = v.iter().position(|&x| x == 30);
    match p { Some(i) => println!("{}", i), None => println!("none") }
}
