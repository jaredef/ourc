enum List{ Cons(i32, Box<List>), Nil }
fn sum(l:&List)->i32{ match l { List::Cons(x,n)=>x+sum(n), List::Nil=>0 } }
fn main(){ let l=List::Cons(1,Box::new(List::Cons(2,Box::new(List::Nil)))); println!("{}", sum(&l)); }
