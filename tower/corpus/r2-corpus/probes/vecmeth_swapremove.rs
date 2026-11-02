fn main() {
    let mut v = vec![1, 2, 3, 4];
    let x = v.swap_remove(1);
    println!("{}{}", x, v[1]);
}
