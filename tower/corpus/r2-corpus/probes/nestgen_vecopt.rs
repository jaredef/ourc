fn main() {
    let v: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
    let s: i32 = v.iter().filter_map(|x| *x).sum();
    println!("{}", s);
}
