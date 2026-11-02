// LAYER2 P3+: `if let [a, b, c] = &v[..]` -- the leading & on a local Vec full-range slice is transparent
// (same [ptr,len] descriptor), reusing the R11 refutable-slice path. Length guard routes to else on mismatch.
// Scoped to a LOCAL Vec with i32 elements (the &Vec-param indirection + String elements are separate rungs).
fn main() {
    let three = vec![1, 2, 3];
    if let [a, b, c] = &three[..] {
        println!("{}", a * 100 + b * 10 + c);
    }
    let one = vec![9];
    if let [x] = &one[..] {
        println!("one {}", x);
    } else {
        println!("no");
    }
    let two = vec![4, 5];
    if let [a, b, c] = &two[..] {
        println!("{} {} {}", a, b, c);
    } else {
        println!("not three");
    }
}
