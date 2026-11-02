enum E { Foo{f : isize}, Bar }
fn take(s: String) -> usize { s.len() }
fn main() { let e = E::Foo{f: 1}; let a = match e { E::Foo{..} => 1, _ => 2 }; let b = match e { E::Foo{f} => f, _ => 2 };
  let s = String::from("ab"); let n = match s { ref t => t.len() }; let m = take(s); println!("{} {} {} {}", a, b, n, m); }
