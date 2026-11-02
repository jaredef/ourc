fn main() {
    let mut n = 5;
    match n { ref mut x => *x += 10 }
    println!("{}", n);
}
