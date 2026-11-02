// W-C (authoring-unlock proof): std::cmp::max / std::cmp::min free functions, authored by DISPATCHING ON
// the interned Ty (reading TYTAG[EITY] for the operand width) rather than threading a new flag. The i64
// case proves the Ty-dispatch: the >i32 value is compared/copied 64-bit because the arg's Ty is tag 2.
fn main() {
    println!("{}", std::cmp::max(3, 7));
    println!("{}", std::cmp::min(3, 7));
    let a: i64 = 9000000000;
    let b: i64 = 42;
    println!("{}", std::cmp::max(a, b));
    println!("{}", std::cmp::min(a, b));
    let m = std::cmp::max(10, std::cmp::min(20, 5));
    println!("{}", m);
}
