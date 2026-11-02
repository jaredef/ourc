fn main(){ let mut s=String::from("ab"); s.push_str("cd"); s.push('e'); println!("{} {}", s, s.len()); }
