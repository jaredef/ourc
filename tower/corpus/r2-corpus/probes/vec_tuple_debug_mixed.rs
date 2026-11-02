// DBG: {:?} on a Vec<(..)> with str tuple fields must quote them. The R8 Vec<tuple> printer used the int-only
// __dbg_els per element; now it unrolls per-field via tupty_fk (like R10), so Vec<(&str,i32)> prints correctly.
fn main() {
    let a: Vec<(&str, i32)> = vec![("a", 1), ("b", 2)];
    let b: Vec<(i32, &str)> = vec![(1, "x"), (2, "y")];
    let c = vec![(1, 2), (3, 4)];
    println!("{:?} {:?} {:?}", a, b, c);
}
