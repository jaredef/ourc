macro_rules! r { (0) => { 0 }; (1) => { r!(0) + 1 }; (2) => { r!(1) + 1 }; }
fn main() { let n = r!(2); }
