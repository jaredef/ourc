fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let n = v.len();
    let mut i = 0;
    while i < n / 2 {
        let t = v[i];
        v[i] = v[n - 1 - i];
        v[n - 1 - i] = t;
        i = i + 1;
    }
    let mut k = 0;
    while k < n { print!("{} ", v[k]); k = k + 1; }
    println!("");
}
