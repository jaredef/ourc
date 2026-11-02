fn main() { let mut s = vec!["pear", "b", "apple", "ab", "a", ""]; s.sort(); let mut t: Vec<String> = vec![String::from("zz"), String::from("Za"), String::from("é"), String::from("e")]; t.sort();
  println!("{:?} {:?} {} {} {}", s, t, 'a'.min('c'), 'x'.max('b'), (-2i8).min(5)); }
