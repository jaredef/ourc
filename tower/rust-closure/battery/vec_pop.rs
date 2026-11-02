fn main() {
    let mut v = vec![1, 2, 3];
    while v.len() > 0 {
        match v.pop() { Some(x) => println!("popped {}", x), None => println!("empty") }
    }
    match v.pop() { Some(x) => println!("got {}", x), None => println!("done") }
}
