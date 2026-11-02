fn f<const N: usize>() -> usize { N }
fn g<const B: bool>() -> i32 { if B { 1 } else { 2 } }
fn h<T: Copy, const K: i64>(x: T) -> T { if K < 0 { x } else { x } }
fn sum<const N: usize>(a: [i32; N]) -> usize { N + a.len() }
fn main() {
    println!("{} {} {}", f::<3>(), f::<{ 7 }>(), g::<true>() + g::<false>());
    println!("{} {}", h::<i32, -2>(5), sum([1, 2, 3]));
}
