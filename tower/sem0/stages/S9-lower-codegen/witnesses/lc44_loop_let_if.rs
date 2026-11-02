fn f() -> i32 {
    let mut hit = 0;
    let mut i = 0;
    while i < 4 {
        let r = if i < 2 { 10 } else { 1 };
        hit = hit + r;
        i = i + 1;
    }
    hit
}
fn main() { println!("{}", f()); }
