// gen1 and gen2 silently DISAGREED ON A VALUE here -- the quietest and worst shape a defect can take, since
// neither tier fails and a hoist installs the disagreeing one.
//
// The lexer accumulates `n = n*radix + d` in i64 and packs the type suffix into the same word. A literal too
// large for that packing used to WRAP, and the two builds of the same parser wrapped DIFFERENTLY:
//     9223372036854775808  ->  L6-built: -123372036854775808   SEM0-built: <empty dump>
//     73236519889708027473620326106273939584_i128 (real, from rustc's own test suite)
//                          ->  L6-built: -396250801992368000   SEM0-built: 0
// The boundary was exact: i64::MAX agreed, i64::MAX+1 diverged.
//
// Now refused, at `lit_mul()` -- the point where a value aliases into the suffix band, which is the same
// representational limit locales/s1-literal-representation CL-1 records. That residual was "a silent wrong
// answer"; it is now a clean refusal, and both tiers agree because both refuse. rustc rejects the unsuffixed
// form outright ("literal out of range"), so refusing is also the closer behaviour.
fn main() { let v = 73236519889708027473620326106273939584_i128; }
