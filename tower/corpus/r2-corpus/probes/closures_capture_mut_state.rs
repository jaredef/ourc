fn main() {
    let mut sum = 0;
    let mut add = |n: i64| { sum += n; };
    for i in 1..=5 { add(i); }
    println!("{}", sum);
}
