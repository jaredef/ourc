macro_rules! rec { (1) => { 1 }; (2) => { rec!(1) + 2 }; }
fn main() { let n = rec!(2); }
