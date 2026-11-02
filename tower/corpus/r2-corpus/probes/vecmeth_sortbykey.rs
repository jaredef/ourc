fn main() {
    let mut v = vec![3, 1, 2];
    v.sort_by_key(|&x| x);
    println!("{}{}{}", v[0], v[1], v[2]);
}
