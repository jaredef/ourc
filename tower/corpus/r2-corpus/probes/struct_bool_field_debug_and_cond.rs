struct S { flag: bool, n: i64 }
fn main() {
    let s = S { flag: false, n: 5 };
    println!("{:?} {}", s.flag, if s.flag { 1 } else { 0 });
    let s2 = S { flag: true, n: 9 };
    if s2.flag { println!("yes {}", s2.n); }
}
