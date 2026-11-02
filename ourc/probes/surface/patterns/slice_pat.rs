fn main() { let v = [1, 2, 3, 4]; match v { [first, .., last] => println!("{} {}", first, last) } }
