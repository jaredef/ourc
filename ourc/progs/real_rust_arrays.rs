fn main() {
    let a = [1, 2, 3];
    let b = [1, 2, 3];
    let c = [1, 2, 4];
    println!("{}", a.len());
    println!("{}", if a == b { 42 } else { 0 });
    println!("{}", if a == c { 0 } else { 42 });
    println!("{}", a[0] + a[2]);
}
