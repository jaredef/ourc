macro_rules! loop_x { ($e: expr) => { 'x: loop { $e; println!("inner"); break; } }; }
fn main() { 'x: for i in 0..2 { loop_x!(break 'x); println!("after inner {}", i); } println!("done"); }
