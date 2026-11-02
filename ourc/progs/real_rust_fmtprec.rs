// C-FMTPREC: two different precisions and a plain `{}` in one format string, so the precision is
// per-ARGUMENT and not per-call
fn id(v: f64) -> f64 { v }
fn main(){ println!("{:.2} and {:.4} and {}", id(1.5), id(1.5), id(1.5)); }
