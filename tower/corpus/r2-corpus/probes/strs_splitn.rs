fn main() {
    let mut it = "k=v=w".splitn(2, '=');
    let a = it.next().unwrap();
    let b = it.next().unwrap();
    println!("{} {}", a, b);
}
