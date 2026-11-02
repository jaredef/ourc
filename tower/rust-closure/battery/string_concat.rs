fn greet(name: &str) -> String {
    let mut s = String::from("Hello, ");
    s.push_str(name);
    s.push('!');
    s
}
fn main() {
    let a = greet("Ada");
    let b = greet("Linus");
    println!("{}", a);
    println!("{}", b);
    let mut all = String::new();
    all.push_str(&a);
    all.push(' ');
    all.push_str(&b);
    println!("{}", all);
    println!("total {}", all.len());
}
