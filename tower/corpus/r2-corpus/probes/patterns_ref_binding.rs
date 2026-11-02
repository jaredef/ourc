fn main() {
    let v = vec![10, 20, 30];
    let mut sum = 0;
    for x in &v { sum += *x; }
    match v.first() { Some(&n) => println!("{}", n + sum), None => println!("0") }
}
