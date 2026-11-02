// RUSTC-TARGET (P-ACCUMULATED-STATE-PARSE-DRIFT root #1): a `::`-qualified type ENDING in generics in
// RETURN and PARAM position -- `-> std::result::Result<usize, i32>`, `buf: &mut std::vec::Vec<u8>`. rzrs
// erased the `::`-path but the `<..>`-skip ran BEFORE the path-erase, so the trailing `<..>` was left
// unparsed (return), and the param parser read only the FIRST segment (`std`) leaving `::vec::Vec<u8>` --
// both drifted POS past the method`s `}` and over-consumed the NEXT method (surfaced hundreds of lines
// later; localized by the parse-drift guard to either`s Read impl `for_both!` body). Now the return-type
// erase skips a trailing `<..>` (->-aware) and the param parser advances a `::`-path to its LAST segment.
// The following method `c` must survive (it was being swallowed by the over-consume).
struct S;
impl S {
    fn a(&mut self, buf: &mut std::vec::Vec<u8>) -> std::result::Result<usize, i32> { std::result::Result::Ok(0) }
    fn b(&self, s: &str) -> std::fmt::Result { std::fmt::Result::Ok(()) }
    fn c(&self) -> i32 { 42 }
}
fn main() { let s = S; println!("{}", s.c()); }
