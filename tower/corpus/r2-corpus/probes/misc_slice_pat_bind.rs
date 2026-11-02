fn main() { let a = [1, 2, 3, 4]; if let [first, .., last] = a { println!("{}", first * 10 + last); } }
