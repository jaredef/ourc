// BLOCK-LOCAL-TAIL-TY witness: a let whose block initializer returns the block's OWN locals through an if/else was
// untyped, so format!("{}") printed the String pointer. SEM0's own s9_result_eq had this shape, which made the
// tower-built SEM0 drop the Err arm of Result<_, Agg> == (R4: lib_std / rustc_hir_typeck diverged from native).
fn f(a: &str) -> String { let e = { let x = String::from(a); let y = String::new(); if x.len() > 0 { x } else { y } }; format!("k#{}", e) }
fn g(a: &str) -> String { let e: String = { let x = String::from(a); let y = String::new(); if x.len() > 0 { x } else { y } }; format!("k#{}", e) }
fn h(a: &str) -> String { let x = String::from(a); let e = if x.len() > 0 { x } else { String::new() }; format!("k#{}", e) }
fn main() { println!("{} {} {}", f("Abc"), g("Abc"), h("Abc")); }
