fn main() {
    let v = vec![1,2,3,4];
    let r: Vec<i32> = v.iter().scan(0, |acc, &x| { *acc += x; Some(*acc) }).collect();
    println!("{:?}", r);
}
