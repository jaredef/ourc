use std::cell::Cell; fn main(){ let c = Cell::new(5); c.set(c.get() + 1); println!("{}", c.get()); }
