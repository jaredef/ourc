type PE = Box<E>;
struct E{v:i64}
fn atom()->PE{ Box::new(E{v:9}) }
fn main(){ let a=atom(); println!("{}",a.v); }
