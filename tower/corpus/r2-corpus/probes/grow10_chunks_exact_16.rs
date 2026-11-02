fn main() {
    let v = [1,2,3,4,5,6,7];
    let mut sums = Vec::new();
    for c in v.chunks_exact(2) {
        sums.push(c[0] + c[1]);
    }
    println!("{:?}", sums);
}
