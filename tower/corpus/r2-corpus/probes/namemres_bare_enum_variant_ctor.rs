// RUSTC-TARGET NAMERES (P-NAMERES-BARE-VARIANT-CTOR): a bare enum-variant constructor
// `A(x)` / `Left(x)` brought into scope by `use E::*` (not the qualified `E::A(x)` form).
// Pervasive in real crates (the `either` crate's `Left`/`Right`, iterator/error enums that
// glob-import their own variants). The variant name reaches parse_atom directly; without this,
// it falls through to the E0425 free-call gate and rejects as "cannot find fn A".
enum Either<L, R> { Left(L), Right(R) }
use Either::*;
fn mk(b: i32) -> Either<i32, i32> { if b > 0 { Left(b) } else { Right(0 - b) } }
fn val(e: Either<i32, i32>) -> i32 { match e { Left(n) => n, Right(n) => 100 + n } }
fn main() {
    println!("{} {} {}", val(mk(5)), val(mk(0 - 3)), val(Left(9)));
}
