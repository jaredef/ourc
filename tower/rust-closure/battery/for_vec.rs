fn main() {
    let mut v = Vec::new();
    for i in 0..8 { v.push(i * i); }
    let mut sum = 0;
    for x in &v { sum = sum + x; }
    println!("sum of squares = {}", sum);
    let mut maxv = 0;
    let mut j = 0;
    while j < v.len() { if v[j] > maxv { maxv = v[j]; } j = j + 1; }
    println!("max = {}", maxv);
    for (i, x) in v.iter().enumerate() { if i % 2 == 0 { println!("even idx {} -> {}", i, x); } }
}
