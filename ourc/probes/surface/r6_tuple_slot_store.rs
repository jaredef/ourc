struct S { t: (i64, i64) }
fn main() {
    let mut a = (1, 2, 3);
    let b = a;
    a.1 = 20;
    a.0 += 5;
    let mut s = S { t: (7, 8) };
    s.t.1 = 80;
    let mut c = (1, "x");
    c.1 = "y";
    println!("{:?} {:?} {:?} {:?}", a, b, s.t, c);
}
