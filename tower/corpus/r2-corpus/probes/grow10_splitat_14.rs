fn main() {
    let v = [1,2,3,4,5,6];
    let (a, b) = v.split_at(2);
    let sa: i32 = a.iter().sum();
    let sb: i32 = b.iter().sum();
    println!("{} {}", sa, sb);
}
