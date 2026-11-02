const A: usize = 1;
mod m {
    pub const X: usize = 3;
}
mod n {
    pub const X: usize = 4;
}
fn kind(v: usize) -> &'static str {
    match v {
        A => "a",
        m::X => "m",
        n::X => "n",
        _ => "other",
    }
}
fn main() {
    println!("{} {} {} {}", kind(1), kind(3), kind(4), kind(9));
}
