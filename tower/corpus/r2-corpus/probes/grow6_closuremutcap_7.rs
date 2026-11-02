fn main() {
    let mut total = 0;
    let mut add = |x: i64| { total += x; };
    add(3);
    add(4);
    add(10);
    println!("{}", total);
}
