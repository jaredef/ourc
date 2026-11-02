fn main() {
    let a: [i32; 3] = [10, 20, 30];
    let i: usize = 1;
    let s: i32 = a[0] + a[i] + a[2];
    println!("{}", s);
}
