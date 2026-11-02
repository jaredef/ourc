fn main() {
    let s = String::from("hi");
    let f = move || s.len();
    println!("{}", s.len());
    println!("{}", f());
}
