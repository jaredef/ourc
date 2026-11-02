// RUSTC-TARGET S8 (P-RANGE-VALUE-IN-VARIANT-ARG, open-ended case): an OPEN range `lo..` (RangeFrom) used
// as a variant-constructor arg -- `Right(17..)` (either`s `fn iter()` test). emit_range_vec parsed a
// closed `lo..hi` but on `17..` ran parse_expr on the `)` delimiter, drifting POS past it and corrupting
// the enclosing parse ~100 lines downstream (a spurious `cannot find fn FnOnce`). An infinite range can`t
// be materialized; emit an empty Vec so the parse does not desync (iterating it is out of scope). This
// probe checks the control-flow (both arms) compiles + picks correctly.
enum W { A(std::ops::Range<i32>), B(std::ops::RangeFrom<i32>) }
fn choose(x: i32) -> i32 {
    match x {
        3 => { let _ = W::A(0..10); 1 }
        _ => { let _ = W::B(17..); 2 }
    }
}
fn main() { println!("{} {}", choose(3), choose(9)); }
