// P-PAT stage 4: `while let` loops, range patterns with CONST and char endpoints, and or-patterns
// over literals.
const S: isize = 1;
const E: isize = 42;
enum Q { Item(isize), Done }
fn step(n: isize) -> Q { if n < 4 { Q::Item(n) } else { Q::Done } }
fn cls(n: isize) -> isize { match n { S..=E => 1, 100 | 200 | 300 => 2, _ => 3 } }
fn ch(c: char) -> isize { match c { 'a'..='z' => 1, '0'..='9' => 2, _ => 0 } }
fn main() {
    let mut i = 0;
    let mut sum = 0;
    while let Q::Item(v) = step(i) { sum = sum + v; i = i + 1; }
    println!("{} {}", i, sum);
    println!("{} {} {} {}", cls(7), cls(200), cls(50), cls(1));
    println!("{} {} {}", ch('q'), ch('5'), ch('!'));
}
