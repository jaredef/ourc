fn main() { let s = "--e=a,b"; let mut out: Vec<String> = Vec::new(); out.extend(s[4..].split(',').map(String::from)); println!("{:?}", out);
  let mut o2: Vec<String> = Vec::new(); o2.extend(s.split(',').map(|x| x.to_string())); println!("{:?}", o2);
  let v: Vec<String> = s[4..].split(',').map(String::from).collect(); println!("{:?}", v); }
