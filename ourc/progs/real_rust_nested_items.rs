fn main() {
    struct S(isize, isize);
    const K: isize = 7;
    fn twice(x: isize) -> isize { x * 2 }
    let s = S(3, 4);
    println!("{}", twice(s.0 + s.1) + K);
    enum C { A, B }
    let c = C::B;
    println!("{}", match c { C::A => 1, C::B => 2 });
}
