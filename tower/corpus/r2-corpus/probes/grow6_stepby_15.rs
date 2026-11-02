fn main() {
    let mut sum = 0;
    for i in (0..20).step_by(3) {
        sum += i;
    }
    println!("{}", sum);
}
