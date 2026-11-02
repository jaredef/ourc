fn first_ten<F>(mut it: F) where F: FnMut(isize) {
    let mut i: isize = 0;
    while i < 10 { it(i); i = i + 1; }
}
fn twice(mut it: impl FnMut(i32)) { it(1); it(2); }
fn main() {
    let mut sum: isize = 0;
    first_ten(|i| { println!("at {}", i); sum = sum + i; });
    let mut s = 0;
    let mut c = |i: i32| { s = s + i; println!("in {}", s); };
    c(1);
    c(2);
    let mut v: Vec<i32> = Vec::new();
    twice(|x| { println!("push {}", x); v.push(x * 10); });
    println!("{} {} {:?}", sum, s, v);
}
