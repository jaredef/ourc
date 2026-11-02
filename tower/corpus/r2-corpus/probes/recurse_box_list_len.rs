enum List { Cons(i64, Box<List>), Nil }
fn len(l: &List) -> i64 { match l { List::Cons(_, n) => 1 + len(n), List::Nil => 0 } }
fn main() {
    let l = List::Cons(9, Box::new(List::Cons(8, Box::new(List::Nil))));
    println!("{}", len(&l));
}
