const A: i32 = [1, 2, 3][2];
static B: [i32; 2] = [4, 5];
const C: i32 = [7; 4][3];
fn main() { println!("{} {} {}", A, B[1], C); }
