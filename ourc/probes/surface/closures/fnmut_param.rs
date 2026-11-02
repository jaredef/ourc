fn twice<F: FnMut()>(mut f: F) { f(); f(); } fn main() { let mut n = 40; twice(|| n += 1); println!("{}", n); }
