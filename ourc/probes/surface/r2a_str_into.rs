// R2a P5 guard: `"m".into()` to a String (`From<&str> for String`) -- in a return tail, a let, and a trait default
// body's callee; it was "no method `into` on str"
trait T {
    fn name(&self) -> String;
    fn hi(&self) -> String {
        format!("hi {}", self.name())
    }
}
struct M;
impl T for M {
    fn name(&self) -> String {
        "m".into()
    }
}
fn main() {
    let s: String = "ab".into();
    println!("{} {} {}", M.hi(), s, s.len());
}
