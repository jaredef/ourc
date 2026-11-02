fn main() {
    let mut v = vec![5, 2, 8, 1, 9, 3];
    v.sort();
    match v.binary_search(&8) {
        Ok(i) => println!("found {}", i),
        Err(i) => println!("miss {}", i),
    }
}
