// C-CELL: a `&Cell<T>` PARAMETER -- the type name says one-slot vector of T, so the accessor table, the
// expression types and the tag scheme all need to learn nothing new
use std::cell::Cell;
fn bump(c: &Cell<i64>){ c.set(c.get() + 1); }
fn main(){ let c = Cell::new(5); bump(&c); bump(&c); println!("{}", c.get()); }
