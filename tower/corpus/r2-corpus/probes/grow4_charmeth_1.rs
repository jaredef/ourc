fn main(){ let mut a=0; let mut u=String::new(); for c in "aB3z!".chars(){ if c.is_alphabetic(){a+=1;} u.push(c.to_ascii_uppercase()); } println!("{} {}", a, u); }
