fn main(){ let s="hello"; let mut sum=0i64; for b in s.bytes() { sum+=b as i64; } println!("{}", sum); }
