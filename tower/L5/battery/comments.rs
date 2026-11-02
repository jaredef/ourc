/* a block comment
   spanning multiple lines */
fn sq(x: i32) -> i32 { x * x /* inline */ }
fn main() {
    let a = 5; // line comment
    let b = sq(a); /* another */ let c = b + 1;
    println!("{} {} {}", a, b, c);   /* trailing */
}
