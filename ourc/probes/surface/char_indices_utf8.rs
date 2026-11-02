fn main() { for (i, c) in "a€😀b".char_indices() { print!("{}:{} ", i, c); }
  let v: Vec<usize> = "héy".char_indices().map(|(i, _)| i).collect();
  println!("{:?} {} {:?}", v, "h€y😀z".char_indices().count(), "héllo".char_indices().map(|(i, _)| i).collect::<Vec<_>>()); }
