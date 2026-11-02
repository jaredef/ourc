use std::cell::RefCell; fn main(){ let c = RefCell::new(5); println!("{}", c.borrow()); let v = RefCell::new(vec![1, 2]); v.borrow_mut().push(3); println!("{} {:?}", v.borrow().len(), v.borrow()); }
