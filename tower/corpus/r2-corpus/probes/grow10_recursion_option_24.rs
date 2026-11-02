fn find(v: &[i32], target: i32, i: usize) -> Option<usize> {
    if i >= v.len() { None }
    else if v[i] == target { Some(i) }
    else { find(v, target, i+1) }
}
fn main() {
    let v = [10,20,30,40];
    println!("{:?} {:?}", find(&v, 30, 0), find(&v, 99, 0));
}
