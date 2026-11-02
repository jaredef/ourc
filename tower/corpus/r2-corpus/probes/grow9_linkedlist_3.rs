enum List { Cons(i64, Box<List>), Nil }
fn push(l: List, v: i64) -> List { List::Cons(v, Box::new(l)) }
fn len(l: &List) -> i64 { match l { List::Cons(_, n) => 1 + len(n), List::Nil => 0 } }
fn sum(l: &List) -> i64 { match l { List::Cons(v, n) => v + sum(n), List::Nil => 0 } }
fn main() {
    let mut l = List::Nil;
    l = push(l, 10); l = push(l, 20); l = push(l, 30);
    println!("{} {}", len(&l), sum(&l));
}
