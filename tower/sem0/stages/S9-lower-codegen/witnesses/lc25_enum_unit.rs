enum Dir { N, S, E }
fn main() {
    let d: Dir = Dir::S;
    let x: i32 = match d { Dir::N => 10, Dir::S => 20, Dir::E => 30 };
    println!("{}", x);
}
