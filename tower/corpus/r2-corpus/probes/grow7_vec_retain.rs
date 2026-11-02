fn main() {
    let mut v: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    v.retain(|x| x % 3 == 0);
    let mut s = 0;
    for x in &v { s += x; }
    println!("{} {}", v.len(), s);
}
