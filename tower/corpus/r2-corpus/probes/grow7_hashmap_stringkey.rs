use std::collections::HashMap;
fn main() {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert(String::from("x"), 100);
    m.insert(String::from("y"), 200);
    let k = String::from("x");
    println!("{}", m[&k] + m["y"]);
}
