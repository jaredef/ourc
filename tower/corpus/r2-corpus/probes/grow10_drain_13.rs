fn main() {
    let mut v = vec![1,2,3,4,5];
    let d: Vec<i32> = v.drain(1..3).collect();
    let sd: i32 = d.iter().sum();
    let sv: i32 = v.iter().sum();
    println!("{} {}", sd, sv);
}
