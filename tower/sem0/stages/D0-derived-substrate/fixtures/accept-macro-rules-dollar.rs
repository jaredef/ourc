macro_rules! twice { ($x:expr) => { $x + $x }; }
fn main(){ println!("{}", twice!(3)); }
