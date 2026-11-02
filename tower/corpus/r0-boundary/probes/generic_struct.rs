struct W<T>{ v:T }
impl<T:Copy> W<T>{ fn get(&self)->T{ self.v } }
fn main(){ let w=W{v:42}; println!("{}", w.get()); }
