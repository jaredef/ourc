// LAYER2: Option::filter(|&v| pred) -- Some(v) if pred(v) else None. The existing handler built the cell
// but did not flag it as an Option, so {:?}/bind saw the raw disc; now it restores EISOPT + the payload kind.
fn main() {
    let a: Option<i32> = Some(8);
    let b: Option<i32> = Some(3);
    let c: Option<i32> = None;
    println!("{:?}", a.filter(|&v| v > 5));
    println!("{:?}", b.filter(|&v| v > 5));
    println!("{:?}", c.filter(|&v| v > 5));
    let kept = a.filter(|&v| v % 2 == 0).unwrap_or(-1);
    println!("{}", kept);
}
