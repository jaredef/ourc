mod foo { pub mod bar { pub fn baz() {} } }
use foo::{bar}::baz
fn main() {}
