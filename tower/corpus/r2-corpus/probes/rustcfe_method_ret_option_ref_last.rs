struct B { s: Vec<i32> }
impl B { fn last(&self) -> Option<&i32> { self.s.last() } }
fn main() { let b = B { s: vec![1,2,3] }; match b.last() { Some(x)=>println!("{}",*x), None=>println!("none") } }
