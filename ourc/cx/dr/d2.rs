fn main() { let args = ["-n", "x"]; let mut it = args.iter().peekable(); while let Some(&a) = it.next() { println!("{}", a); } }
