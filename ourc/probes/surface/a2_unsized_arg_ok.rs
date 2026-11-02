fn main() {
    let a: Option<&[u8]> = None;
    let c: Vec<String> = Vec::new();
    let d: Option<Box<[i32]>> = None;
    println!("{} {} {}", a.is_none(), c.len(), d.is_none());
}
