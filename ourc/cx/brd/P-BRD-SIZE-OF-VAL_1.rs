fn sv<T: ?Sized>(x: &T) -> usize { std::mem::size_of_val(x) }
fn main() { println!("{} {} {}", sv("hello"), sv(&[1u16, 2, 3][..]), sv(&7u64)); }
