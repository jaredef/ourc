// RUSTC-FE sub-arc 00 (fail-closed) / FC: a macro_rules! whose matcher/body contain KEYWORD tokens
// (struct/enum/fn) inside repetitions must not hang the item-prepasses (they scanned into the macro
// body and parsed the inner `struct` as a real item). skip_tok now skips a whole macro_rules as a
// balanced token tree. Real rustc uses these pervasively (rustc_span's newtype-index macro).
macro_rules! newtype { ( $( $v:vis struct $n:ident($t:ty); )* ) => { $( $v struct $n($t); )* }; }
fn main() { let x = 3 + 4; println!("{}", x); }
