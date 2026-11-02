enum List { Cons(i64, Box<List>), Nil }
fn sum(l: &List) -> i64 { match l { List::Cons(v, n) => v + sum(n), List::Nil => 0 } }
fn main() {
    let l = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{}", sum(&l));
}
