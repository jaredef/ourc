struct Node { tag: i64, val: i64 }
fn parse(n: i64) -> Result<Node, i64> { if n < 0 { Err(0 - n) } else { Ok(Node { tag: 1, val: n * 10 }) } }
fn main() {
    match parse(4) { Ok(nd) => println!("ok {} {}", nd.tag, nd.val), Err(e) => println!("err {}", e) }
    match parse(-3) { Ok(nd) => println!("ok {}", nd.val), Err(e) => println!("err {}", e) }
}
