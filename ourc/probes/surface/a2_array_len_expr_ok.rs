const N: usize = 3;
fn main() {
    let a = [7u8; 4];
    let b: [i32; N] = [1; N];
    let c = [0; 2 * N];
    let v: Vec<u8> = vec![1; 3];
    println!("{} {} {} {}", a[3], b[2], c.len(), v.len());
}
