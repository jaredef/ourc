fn main() {
    let mut sum = 0;
    for i in 0..10 { sum = sum + i; }
    println!("sum 0..10 = {}", sum);
    for i in 1..=5 { print!("{} ", i); }
    println!("");
    let v = vec![10, 20, 30, 40];
    let mut total = 0;
    for x in &v { total = total + x; }
    println!("total {}", total);
    for (i, x) in v.iter().enumerate() { println!("v[{}] = {}", i, x); }
    let mut c = 0;
    for k in 0..20 { if k % 2 == 1 { continue; } c = c + 1; }
    println!("evens in 0..20 = {}", c);
}
