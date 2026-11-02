fn main() {
    let v = vec![(1,"a"),(3,"b"),(2,"c")];
    let mn = v.iter().min_by_key(|&&(k,_)| k).unwrap();
    let mx = v.iter().max_by_key(|&&(k,_)| k).unwrap();
    println!("{} {}", mn.1, mx.1);
}
