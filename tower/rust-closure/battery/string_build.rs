fn shout(s: &String) -> String {
    let mut out = String::new();
    out.push_str(s);
    out.push('!');
    out
}
fn main() {
    let mut greeting = String::from("Hello");
    greeting.push_str(", world");
    greeting.push('!');
    println!("{}", greeting);
    println!("len {}", greeting.len());
    let mut acc = String::new();
    let mut i = 0;
    while i < 5 { acc.push_str("ab"); i = i + 1; }
    println!("{}", acc);
    println!("acc len {}", acc.len());
    let base = String::from("loud");
    println!("{}", shout(&base));
}
