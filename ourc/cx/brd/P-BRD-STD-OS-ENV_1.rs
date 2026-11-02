use std::env;
fn main() { env::set_var("WITNESS_VAR", "val42"); println!("{:?} {}", env::var("WITNESS_VAR"), env::var("WITNESS_NOPE_XYZ").is_err()); }
