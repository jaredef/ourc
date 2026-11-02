// STATEFUL-ITER (str-cursor): s.splitn(n, sep) -> a cursor over sep-delimited pieces;
// it.next() yields the next piece as &str, the nth piece keeps the whole rest.
fn main() {
    let mut it = "k=v=w".splitn(2, '=');
    let a = it.next().unwrap();
    let b = it.next().unwrap();
    println!("{} {}", a, b);                 // k v=w
    let mut it2 = "1,2,3,4".splitn(3, ',');
    println!("{}", it2.next().unwrap());     // 1
    println!("{}", it2.next().unwrap());     // 2
    println!("{}", it2.next().unwrap());     // 3,4
}
