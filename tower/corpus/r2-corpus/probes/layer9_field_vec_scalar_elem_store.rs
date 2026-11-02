struct C { table: Vec<i32> }
impl C { fn get(&self, i: usize) -> i32 { self.table[i] } }
fn main() { let mut c = C { table: vec![0, 0] }; c.table[0] = 5; println!("{}", c.get(0)); }
