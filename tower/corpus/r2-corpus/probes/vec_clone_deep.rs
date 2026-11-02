// R4/std: Vec::clone() must produce an independent (deep) copy tagged as a Vec. `.clone()` (5 chars) was
// unmatched in dispatch_vec (only `cloned` (6) matched), so let s=v.clone() bound a non-Vec (garbage on
// len/index/{:?}/sort). Now a deep copy: mutating the clone does not alias the original.
fn main() {
    let v = vec![3, 1, 2];
    let mut s = v.clone();
    s.sort();
    s.push(9);
    println!("{:?} {:?} {} {}", v, s, v.len(), s.iter().sum::<i32>());
}
