// should-reject (E0609): access to a non-existent struct field.
// rustc rejects this too; the derived frontend now REJECTS (was a silent wrong-offset no-op).
struct P { x: i32, y: i32 }
fn main() {
    let p = P { x: 1, y: 2 };
    println!("{}", p.z);
}
