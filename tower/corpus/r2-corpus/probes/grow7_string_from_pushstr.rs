fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");
    let t = s + "!";
    println!("{} {}", t, t.len());
}
