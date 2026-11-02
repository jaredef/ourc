// LAYER2: i32/i64 signum() (-1/0/1) and clamp(lo,hi) (max(v,lo) then min(_,hi)).
fn main() {
    println!("{} {} {}", (-5i32).signum(), 0i32.signum(), 7i32.signum());
    let vals = [7i32, -3, 3];
    for &v in vals.iter() {
        print!("{} ", v.clamp(0, 5));
    }
    println!();
    let big: i64 = 9000000000;
    println!("{}", big.signum());
}
