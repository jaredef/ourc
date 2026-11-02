// cx10 tail witness (ourc construct-generation, 2026-10-25): max_by_key/min_by_key typed like max/min; an INLINE
// `.map(|c| c.to_vec()).collect::<Vec<_>>()` over chunks/windows and `.map(|t| t.parse::<i32>()).collect::<Vec<_>>()`
// Debug as nested Vec<Vec<_>> / Vec<Result<_, ParseIntError>> (element kind, not the outer "Result" substring).
fn main() {
    let xs = [1, 2, 3, 4, 5, 6, 7];
    println!("{:?} {:?}", xs.iter().max_by_key(|&&x| x % 4), xs.iter().min_by_key(|&&x| x % 3));
    println!("{:?}", xs.chunks(3).map(|c| c.to_vec()).collect::<Vec<_>>());
    println!("{:?}", xs.windows(5).map(|w| w.to_vec()).collect::<Vec<_>>());
    println!("{:?}", "10,x,30".split(',').map(|t| t.parse::<i32>()).collect::<Vec<_>>());
}
