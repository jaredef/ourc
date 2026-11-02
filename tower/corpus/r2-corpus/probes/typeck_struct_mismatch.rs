/* R2bn should-reject probe: E0308 mismatched types -- comparing two DIFFERENT struct types.
   rustc rejects (A and B are distinct types even with identical fields); the derived rzrs.rs
   must also reject with `error: mismatched types` (frontend rc 1, no ELF). */
#[derive(PartialEq)] struct A { x: i32 }
#[derive(PartialEq)] struct B { x: i32 }
fn main() {
    let a = A { x: 1 };
    let b = B { x: 1 };
    println!("{}", a == b);
}
