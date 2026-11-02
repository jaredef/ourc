fn main() { let a = ['a', 'b']; let v: Vec<char> = "héy".chars().collect(); let e = vec!['\'', '\n', 'z', '"', '\\', '\t'];
  println!("{:?} {:?} {:?} {:?}", a, v, e, "ab".char_indices().map(|(_i, c)| c).collect::<Vec<char>>()); }
