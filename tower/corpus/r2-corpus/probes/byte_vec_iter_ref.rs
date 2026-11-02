fn main() {
    let v: Vec<u8> = "abc".as_bytes().to_vec();
    let mut s = 0i64;
    for x in &v { s += *x as i64; }
    let b = "de".as_bytes();
    for y in b { s += *y as i64; }
    println!("{}|{}", s, v[1]);
}
