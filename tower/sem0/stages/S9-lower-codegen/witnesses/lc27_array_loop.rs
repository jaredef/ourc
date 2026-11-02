fn main() {
    let a: [i32; 4] = [5, 10, 15, 20];
    let mut sum: i32 = 0;
    let mut i: usize = 0;
    while i < 4 { sum = sum + a[i]; i = i + 1; }
    println!("{}", sum);
}
