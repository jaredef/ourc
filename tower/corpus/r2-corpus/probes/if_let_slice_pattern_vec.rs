// LAYER2 P3: refutable slice pattern `if let [a, b, c] = v[..]` on a Vec -- length guard (eq arity) routes
// to the else branch on mismatch, binds each element by offset on match. Scoped to full-range `v[..]`
// (no leading &, no rest/trailing -- those are separate gaps).
fn classify(v: &Vec<i32>) -> i32 {
    if let [a, b, c] = v[..] {
        a * 100 + b * 10 + c
    } else if let [a, b] = v[..] {
        a * 10 + b
    } else {
        -1
    }
}
fn main() {
    let three = vec![1, 2, 3];
    let two = vec![7, 8];
    let four = vec![1, 2, 3, 4];
    println!("{}", classify(&three));
    println!("{}", classify(&two));
    println!("{}", classify(&four));
    let v = vec![5, 6, 7];
    if let [_, mid, _] = v[..] {
        println!("mid {}", mid);
    }
}
