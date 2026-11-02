// P-ITER-ADAPTER: `for (i, &x) in v.iter().enumerate()` -- the `&x` ref sub-pattern in
// the enumerate tuple pattern (element binds by value). Was: rd_ident on `&` read
// nothing -> x unbound -> empty output.
fn main() {
    let v = vec![10, 20, 30];
    let mut s = 0;
    for (i, &x) in v.iter().enumerate() { s += i as i32 + x; }
    println!("{}", s);                       // 0+10 +1+20 +2+30 = 63
    for (i, &x) in v.iter().enumerate() { println!("{} {}", i, x * 2); }
}
