struct S(i64, i64, i64, i64);
fn main(){ let s = S(1,2,3,4);
match s { S(.., c, d) => println!("A {} {}", c, d) }
match s { S(a, .., d) => println!("B {} {}", a, d) }
match s { S(.., d) => println!("C {}", d) }
match s { S(a, b, ..) => println!("D {} {}", a, b) }
match s { S(1, .., x) => println!("E {}", x), _ => println!("no") }
}
