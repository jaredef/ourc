fn main() {
    let line = "10,20,30,40";
    let mut sum = 0i64;
    for f in line.split(',') {
        let n: i64 = f.parse().unwrap();
        sum += n;
    }
    println!("{}", sum);
}
