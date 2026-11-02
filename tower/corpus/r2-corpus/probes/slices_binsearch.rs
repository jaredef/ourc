fn main() {
    let a = [1, 3, 5, 7, 9];
    match a.binary_search(&7) {
        Ok(i) => println!("{}", i),
        Err(i) => println!("-{}", i),
    }
}
