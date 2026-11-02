fn main() {
    let v = vec![1, 2, 2, 2, 2, 3, 5, 5, 8, 13, 13, 13];
    for x in [0, 1, 2, 3, 4, 5, 6, 8, 12, 13, 14] { println!("{:?}", v.binary_search(&x)); }
    let e: Vec<i64> = Vec::new(); println!("{:?}", e.binary_search(&3));
    let w = vec![7]; println!("{:?} {:?} {:?}", w.binary_search(&7), w.binary_search(&1), w.binary_search(&9));
    let d = vec![4, 4, 4, 4, 4, 4, 4]; println!("{:?}", d.binary_search(&4));
    match v.binary_search(&5) { Ok(i) => println!("found {}", i), Err(i) => println!("insert {}", i) }
    let r = v.binary_search(&6); println!("{:?} {}", r, r.is_err());
}
