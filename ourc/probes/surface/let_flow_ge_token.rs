enum W { A, B }
fn main() {
    let v:Vec<i32>=vec![1, 2];
    let o:Option<W>=Some(W::A);
    let k = match o { Some(W::A) => 1, Some(W::B) => 2, None => 0 };
    println!("{} {}", v.len(), k);
}
