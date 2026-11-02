struct T { x: String }
fn main() {
    let s = String::from("a");
    let t = T { x: s };
    println!("{} {}", s, t.x);
}
