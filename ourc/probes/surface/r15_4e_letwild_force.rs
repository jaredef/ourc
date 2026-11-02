struct Item {
    v: Vec<i32>,
}
fn main() {
    let it = Item { v: vec![1, 2, 3] };
    let _ = it;
    let _ = it;
    println!("{}", it.v.len());
}
