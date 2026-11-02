fn sum<const N: usize>(a: [i64; N]) -> i64 { let mut s = 0; for i in 0..N { s += a[i]; } s }
fn first_last<const N: usize>(a: [i64; N]) -> (i64, i64) { (a[0], a[N - 1]) }
fn size<const N: usize>(_a: [u8; N]) -> usize { N * 2 }
fn main() {
    let x = [3, 1, 4, 1, 5];
    println!("{} {:?} {} {}", sum(x), first_last(x), size([0u8; 7]), sum::<2>([10, 20]));
}
