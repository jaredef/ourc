fn main() {
    let a = [1, 2, 3];
    let s: &[i32] = &a;
    let mut v = vec![4, 5];
    let m: &mut Vec<i32> = &mut v;
    m.push(6);
    let t: String = String::from("x");
    let b: Box<String> = Box::new(String::new());
    assert_eq!(10u64, 10u64);
    println!("{} {} {} {}", s.len(), v.len(), t, b.len());
}
