// RUSTC-TARGET (P-ACCUMULATED-STATE-PARSE-DRIFT root #2): `impl <::-qualified-trait> for T { .. }` with
// MULTIPLE methods -- e.g. either`s `impl fmt::Write for Either<L,R> { fn write_str.. fn write_char.. }`.
// parse_impl read only the FIRST trait-name segment (`fmt`), then the `for` check ran rd_ident on the `:`
// of `::Write` and failed, so hastrait stayed 0 and the whole impl header desynced. Recoverable with ONE
// method; with TWO the first method`s body over-consumed the next (localized by the parse-drift guard to
// either`s fmt::Write impl). Now parse_impl advances a `::`-qualified trait name (and the `for`-type) to
// its LAST segment before the `<..>`/`for` checks.
mod m { pub trait T { fn f(&self) -> i32; fn g(&self) -> i32; } }
struct P;
impl m::T for P {
    fn f(&self) -> i32 { 1 }
    fn g(&self) -> i32 { 2 }
}
fn main() { use m::T; let p = P; println!("{} {}", p.f(), p.g()); }
