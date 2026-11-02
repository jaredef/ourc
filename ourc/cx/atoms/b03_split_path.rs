fn main() { let p = "crate::c::MAX"; let segs: Vec<&str> = p.split("::").collect(); let mut i = 0; let mut m = 5usize;
  while i + 1 < segs.len() { m = match segs[i] { "crate" => 0, "self" => m, s => s.len() }; i += 1; } println!("{:?} {} {}", segs, m, segs[i]); }
