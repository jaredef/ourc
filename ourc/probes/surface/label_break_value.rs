fn main() { let mut i = 0; let v = 'o: loop { loop { i += 1; if i == 3 { break 'o i * 10; } } };
  let w = 'a: loop { let u = 'b: loop { break 'b 4; }; break 'a u + 1; };
  let z = 'a: loop { let mut k = 0; 'b: loop { k += 1; if k > 2 { break 'a k; } continue 'b; } }; println!("{} {} {}", v, w, z); }
