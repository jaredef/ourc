fn main() {
    let mut it = "hi".chars();
    let a = it.next().unwrap();
    let b = it.next().unwrap_or('?');
    let c = it.next().unwrap_or('!');
    println!("{}{}{}", a, b, c);
}
