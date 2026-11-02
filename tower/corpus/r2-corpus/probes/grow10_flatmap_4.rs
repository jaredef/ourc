fn main() {
    let v = vec![1,2,3];
    let r: Vec<i32> = v.iter().flat_map(|&x| vec![x, x*10]).collect();
    let sum: i32 = r.iter().sum();
    println!("{} {}", r.len(), sum);
}
