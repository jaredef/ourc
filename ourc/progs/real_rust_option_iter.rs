fn main() {
    let v = vec![3, 9, 2];
    println!("{:?} {:?}", v.iter().max(), v.iter().min());
    println!("{:?} {:?}", v.first(), v.last());
    println!("{:?}", v.iter().position(|x| *x == 9));
    println!("{:?}", v.iter().rev().next());
    let e: Vec<i32> = Vec::new();
    println!("{:?} {:?}", e.iter().max(), e.first());
}
