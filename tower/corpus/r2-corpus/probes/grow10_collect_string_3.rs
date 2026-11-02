fn main() {
    let s: String = ('a'..='e').collect();
    let t: String = vec!['x','y','z'].into_iter().collect();
    println!("{} {}", s, t);
}
