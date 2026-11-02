enum Dir { North, East, South, West }
fn turn(d: Dir) -> i32 { match d { Dir::North => 1, Dir::East => 2, Dir::South => 3, _ => 4 } }
fn main() { std::process::exit(turn(Dir::South) * 10 + turn(Dir::West)); }   // 3*10 + 4 = 34
