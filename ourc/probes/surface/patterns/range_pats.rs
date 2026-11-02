fn main() { let c = 'q'; let r = match c { 'a'..='m' => 1, 'n'..='z' => 2, _ => 3 }; println!("{}", r); }
