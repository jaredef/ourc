fn main() { let s = "héllo"; let v: Vec<&str> = "aébéc".split('é').collect();
  println!("{:?} {} {:?} {} {}", s.find('l'), s.replace('l', "L"), v, s.contains('é'), "a-b".replace('-', "+")); }
