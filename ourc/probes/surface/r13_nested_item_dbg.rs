fn main() {
    #[derive(Default, Debug)]
    struct Q { x: i32 }
    #[derive(Debug)]
    struct R(i32, bool);
    #[derive(Debug)]
    enum E { A, B(i32) }
    let q = Q::default();
    println!("{:?} {:?} {:?} {:?}", q, R(1, true), E::A, E::B(2));
}
