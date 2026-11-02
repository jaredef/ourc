#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct In { a: i32 }
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Out { i: In, b: u8 }
fn main() {
    let x = Out { i: In { a: 1 }, b: 2 };
    let y = Out { i: In { a: 1 }, b: 3 };
    println!("{}", x == y);
    println!("{:?}", x);
}
