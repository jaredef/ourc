fn main() {
    let v = vec![10, 20, 30];
    let pairs: Vec<(usize, i32)> = v.iter().enumerate().map(|(i, &x)| (i, x * 2)).collect();
    let mut s = 0;
    for (i, x) in &pairs { s += (*i as i32) + x; }
    println!("{}", s);
}
