fn main() {
    let v = [10i64, 20, 30, 40, 50];
    let i: i64 = 3;
    let s = v[i as usize] + v[(i - 1) as usize];
    println!("{}", s);
}
