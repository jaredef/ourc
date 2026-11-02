enum Op{Add,Sub}
impl Op{ fn apply(&self,a:i64,b:i64)->i64{ match self{ Op::Add=>a+b, Op::Sub=>a-b } } }
fn main(){ let o=Op::Sub; println!("{}",o.apply(10,3)); }
