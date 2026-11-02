fn main() {
    let v = vec![1,2,3,4,1,2];
    let a: Vec<i32> = v.iter().cloned().take_while(|&x| x < 4).collect();
    let b: Vec<i32> = v.iter().cloned().skip_while(|&x| x < 4).collect();
    let sa: i32 = a.iter().sum();
    let sb: i32 = b.iter().sum();
    println!("{} {}", sa, sb);
}
