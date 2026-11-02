struct S { id: i32 }
fn main() {
    let mut v = vec![S { id: 1 }, S { id: 2 }, S { id: 3 }];
    let mut s = 0;
    while let Some(x) = v.pop() { s = s * 10 + x.id; }
    println!("{}", s);
}
