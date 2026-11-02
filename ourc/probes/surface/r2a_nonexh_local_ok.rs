// R2a P6 NONEXH guard: within ONE crate a `#[non_exhaustive]` enum is matched exhaustively without a wildcard (only
// ANOTHER crate needs one: E0004 -- p5-oracle 22-nonexhaustive)
#[non_exhaustive] enum E{A,B} fn main(){ match E::A { E::A => println!("a"), E::B => println!("b") } }
