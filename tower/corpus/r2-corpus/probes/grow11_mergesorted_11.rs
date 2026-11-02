fn merge(a: &[i64], b: &[i64]) -> Vec<i64> {
    let mut out = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] { out.push(a[i]); i += 1; }
        else { out.push(b[j]); j += 1; }
    }
    while i < a.len() { out.push(a[i]); i += 1; }
    while j < b.len() { out.push(b[j]); j += 1; }
    out
}
fn main() {
    let a = [1, 4, 6, 9];
    let b = [2, 3, 5, 10];
    let m = merge(&a, &b);
    let mut sum = 0;
    for (idx, v) in m.iter().enumerate() {
        sum += (idx as i64) * v;
    }
    println!("{}", sum);
}
