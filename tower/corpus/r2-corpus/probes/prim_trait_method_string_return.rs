// R16b (extends R1 keystone): a trait method implemented on a primitive returning String/&str must tag the
// result's string kind. The prim-receiver dispatch (opt_postfix `call i32_g`) didn't consult the method's
// return type, so `42.g()` returning a String printed a pointer. Now BIRET records the sig's return kind.
trait G { fn g(&self) -> String; }
impl G for i32 { fn g(&self) -> String { format!("n{}", self) } }
fn wrap<T: G>(x: T) -> String { x.g() }
fn main() {
    println!("{} {}", 42.g(), wrap(7));
}
