fn main() {
    let o: Option<i32> = Some(7);
    let n: Option<i32> = None;
    let a: i32 = match o { Some(v) => v + 1, None => 0 };
    let b: i32 = match n { Some(v) => v, None => 99 };
    let s: i32 = a + b;
    println!("{}", s);
}
