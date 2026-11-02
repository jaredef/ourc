fn main() {
    let c = 'k';
    let r = match c { 'a'..='j' | 'A'..='J' => 1, 'k'..='z' => 2, _ => 3 };
    println!("{}", r);
}
