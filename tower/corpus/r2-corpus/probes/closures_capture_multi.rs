fn main() {
    let a = 3; let b = 4; let c = 5;
    let f = || a + b * c;
    println!("{}", f());
}
