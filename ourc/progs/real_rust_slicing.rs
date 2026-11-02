fn main() {
    let a = [1, 2, 3, 4, 5];
    let s = &a[1..3];
    println!("{} {}", s.len(), s[0] + s[1]);
    let v = vec![10, 20, 30, 40];
    let t = &v[2..];
    println!("{} {}", t.len(), t[0]);
    let u = &v[..2];
    println!("{} {}", u.len(), u[1]);
    let w = &a[..];
    println!("{}", w.len());
}
