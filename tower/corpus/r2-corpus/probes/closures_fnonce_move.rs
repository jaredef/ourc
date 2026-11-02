fn main() {
    let s = String::from("hello");
    let f = move || s.len();
    println!("{}", f());
}
