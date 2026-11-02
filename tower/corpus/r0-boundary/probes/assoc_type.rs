trait Container{ type Item; fn first(&self)->Self::Item; }
struct IntBox(i32);
impl Container for IntBox{ type Item=i32; fn first(&self)->i32{ self.0 } }
fn main(){ let b=IntBox(7); println!("{}", b.first()); }
