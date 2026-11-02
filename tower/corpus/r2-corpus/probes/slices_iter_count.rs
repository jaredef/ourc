fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let c = a.iter().filter(|&&x| x % 2 == 0).count();
    println!("{}", c);
}
