trait G { fn v(&self) -> i64 { 1 } } struct A; struct B; impl G for A {} impl G for B { fn v(&self) -> i64 { 41 } } fn main() { println!("{}", A.v() + B.v()); }
