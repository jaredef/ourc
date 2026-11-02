use std::iter;
fn main() {
    let e: Vec<i32> = iter::empty().collect();
    let o: Vec<i32> = iter::once(4).chain(iter::repeat(9).take(2)).collect();
    let v = vec![1, 2, 3];
    let s: i32 = IntoIterator::into_iter(v).sum();
    let r: Vec<u8> = (250u8..).take(3).collect();
    let t: i64 = (1..).take(4).sum();
    let n = iter::once(7).count();
    let q: i32 = iter::repeat(3).take(4).sum();
    println!("{:?} {:?} {} {:?} {} {} {}", e, o, s, r, t, n, q);
    for i in (5..).take(2) { println!("{}", i); }
}
