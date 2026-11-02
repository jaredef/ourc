fn main() {
    let start = 7;
    let step = 3;
    let at = |i| start + i * step;
    for i in 0..5 { print!("{} ", at(i)); }
    println!("");
    let mut total = 0;
    let contribute = |x| x * x + 1;
    for k in 1..=4 { total = total + contribute(k); }
    println!("total = {}", total);
}
