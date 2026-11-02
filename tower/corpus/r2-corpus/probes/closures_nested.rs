fn main() {
    let a = 2;
    let outer = |b: i64| {
        let inner = |c: i64| a + b + c;
        inner(10)
    };
    println!("{}", outer(20));
}
