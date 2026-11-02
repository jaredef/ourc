#[derive(PartialEq, PartialOrd, Clone, Copy)] enum D { N, E, S, W }
fn main() { let v = vec![D::S, D::N, D::W]; let mut c = 0; for d in &v { if *d > D::E { c += 1; } }
  println!("{} {} {} {} {} {}", D::N < D::W, D::S > D::W, D::E <= D::E, D::W >= D::N, D::N > D::N, c); }
