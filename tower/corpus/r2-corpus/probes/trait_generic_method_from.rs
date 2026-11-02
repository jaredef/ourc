// P-TRAIT R4: <prim/generic-T>::from(x) -- numeric From is identity/widen; a trait method
// with a generic return T inferred from the let annotation returns T::from(literal).
trait Conv { fn conv<T: From<i64>>(&self) -> T; }
struct S;
impl Conv for S { fn conv<T: From<i64>>(&self) -> T { T::from(8) } }
fn main() {
    let x: i64 = S.conv();
    println!("{}", x);            // 8
    println!("{}", i64::from(5)); // 5
    let s = String::from("hi");   // unregressed
    println!("{}", s);            // hi
}
