fn main() { let args = ["-n", "x"]; let mut it = args.iter(); while let Some(&a) = it.next() { println!("{}", a); } }
