fn main() {
    assert_eq!(vec![1], vec![1,]);
    assert_eq!(vec![1, 2, 3], vec![1, 2, 3,]);
    let a = vec![1, 2];
    let b = vec![1, 3];
    println!("{}", if a == b { 0 } else { 42 });
}
