// R2a P5 XCRATE-MAC guard (reject side): in ONE crate a #[macro_export] macro lives at the crate root, so reaching it as
// `l::twice!` through its defining module is E0433. Across crates (`--extern l`) the same path is valid (p5-oracle 12-macro).
mod l {
#[macro_export] macro_rules! twice{($e:expr)=>{$e*2}}
}
fn main(){println!("{}",l::twice!(21));}
