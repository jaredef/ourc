// R2a P4 guard: str::rfind answers the byte index of the LAST match (char and &str patterns), None when absent.
fn main() {
    let p = "a|b|12";
    let cut = p.rfind('|').unwrap_or(0);
    let ar: usize = p[cut + 1..].parse().unwrap_or(0);
    println!("{} {} {}", cut, &p[..cut], ar);
    println!("{:?} {:?} {:?}", "abcabc".rfind("bc"), "abc".rfind('z'), "x".rfind("x"));
    match "no-bar".rfind('|') { Some(i) => println!("got {}", i), None => println!("none") }
}
