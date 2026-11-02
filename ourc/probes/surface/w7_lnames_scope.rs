mod bar {
    pub fn baz() -> i32 { 7 }
}
fn dbl(x: i32) -> i32 { x * 2 }
fn main() {
    use bar::baz;
    let a = baz();
    {
        let baz = |x: i32| x + 1;
        println!("{}", baz(a));
    }
    println!("{}", baz());
    if a > 0 {
        let dbl = |x: i32| x + 100;
        println!("{}", dbl(1));
    }
    println!("{}", dbl(5));
}
