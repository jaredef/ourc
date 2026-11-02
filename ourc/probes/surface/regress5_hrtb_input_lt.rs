fn apply<F>(f: F) -> u8
where
    F: for<'a> Fn(&'a u8) -> &'a u8,
{
    let x = 7u8;
    *f(&x)
}
fn main() {
    println!("{}", apply(|v| v));
}
