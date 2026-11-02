macro_rules! pick { () => { 1 }; (a) => { 2 }; }
fn main() { let x = pick!(); let y = pick!(a); }
