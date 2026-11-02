fn main() {
    let v: Vec<i64> = vec![7, 5, 3];
    let mut s = 0;
    s += v.iter().position(|&x| x == 3).unwrap();
    s += v.iter().position(|&x| x == 7).unwrap() * 10;
    if v.iter().position(|&x| x == 9).is_none() { s += 100; }
    println!("{}", s);
}
