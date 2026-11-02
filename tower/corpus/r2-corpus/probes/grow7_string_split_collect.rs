fn main() {
    let s = "a,b,c,d,e";
    let parts: Vec<&str> = s.split(',').collect();
    println!("{} {}", parts.len(), parts[2]);
}
