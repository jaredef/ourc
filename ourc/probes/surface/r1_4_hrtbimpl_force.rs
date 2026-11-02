fn apply(f: impl for<'a> Fn(&'a u32) -> &'a u32) -> u32 {
    *f(&22) + *f(&44)
}
fn main() {
    println!("{}", apply(|x| x));
}
