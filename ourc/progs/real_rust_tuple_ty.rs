fn main() {
    let a = (1, 2);                 println!("{:?}", a);
    let b = (7, "x", true);         println!("{:?}", b);
    let c = (1.5, 'z');             println!("{:?}", c);
    let d = (5,);                   println!("{:?}", d);
    let e = ();                     println!("{:?}", e);
    let f: (i64, i64) = (3, 4);     println!("{:?}", f);
    println!("{:?}", (5,));
    println!("{:?}", ((1, 2), 3));
    println!("{:?}", (3, (1, 2)));
    println!("{:?}", (vec![1, 2], 9));
    println!("{:?}", (-3, "a\"b"));
    let h = (1, 2);                 println!("{} {} {:?}", h.0, h.1, h.0 + h.1);
    let i = (true, 'q');            println!("{:?} {:?}", i.0, i.1);
    let p = (1, 2);                 println!("{:?}", (p, 3));
}
