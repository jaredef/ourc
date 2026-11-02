fn main() {
    let v: Vec<&str> = "a,b,c".split(',').collect();
    println!("{}", v[1]);
}
