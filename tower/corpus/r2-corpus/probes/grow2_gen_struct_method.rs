struct Wrap<T>{ v:T }
impl<T:Copy> Wrap<T>{ fn get(&self)->T { self.v } }
fn main(){ let w=Wrap{v:42i64}; let u=Wrap{v:8i64}; println!("{}", w.get()+u.get()); }
