// N1-N5: the PANIC topology. A zero divisor and an out-of-range index exit 101 (they used to raise
// SIGFPE and read off the end); `std::process::exit(n)` exits n; a `Result`-returning main exits with
// its variant, which our prelude numbers Ok=0 / Err=1 -- exactly rustc's answer.
fn id(v: i32) -> i32 { v }
fn idx(v: usize) -> usize { v }
fn main() -> Result<(), i32> {
    let a = id(10);
    let b = id(3);
    println!("{} {}", a / b, a % b);
    let v = vec![1, 2, 3];
    println!("{}", v[idx(2)]);
    let arr = [4, 5, 6];
    println!("{}", arr[idx(1)]);
    assert_eq!(id(1), 1);
    Ok(())
}
