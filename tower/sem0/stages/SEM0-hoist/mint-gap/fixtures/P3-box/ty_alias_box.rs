type P<T> = Box<T>;
struct E{v:i64}
fn atom()->P<E>{ Box::new(E{v:9}) }
fn main(){ let a=atom(); println!("{}",a.v); }
