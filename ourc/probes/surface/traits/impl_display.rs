use std::fmt; struct P(i64); impl fmt::Display for P { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "P{}", self.0) } } fn main() { println!("{}", P(42)); }
