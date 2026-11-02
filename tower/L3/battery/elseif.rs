fn grade(x: i32) -> i32 { if x >= 90 { return 4; } else if x >= 80 { return 3; } else if x >= 70 { return 2; } else { return 1; } }
fn main() { std::process::exit(grade(85) * 10 + grade(60)); }   // 3*10 + 1 = 31
