fn main(){ let x = (1, 2, 3, 4, 5);
match x { (.., d, e) => println!("A {} {}", d, e) }
match x { (a, .., e) => println!("B {} {}", a, e) }
match x { (a, b, .., e) => println!("C {} {} {}", a, b, e) }
match x { (1, .., 5) => println!("D lit"), _ => println!("D no") }
match x { (_, .., 9) => println!("E 9"), (.., z) => println!("E {}", z) }
match (1, "s", 2.5) { (.., f) => println!("F {}", f) }
}
