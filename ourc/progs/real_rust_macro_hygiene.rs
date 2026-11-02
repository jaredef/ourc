macro_rules! bad_macro {
    ($ex:expr) => ({let _x = 9; $ex})
}
pub fn main() {
    let _x = 8;
    println!("{}", bad_macro!(_x));
}
