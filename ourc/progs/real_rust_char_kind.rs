// P-NOM rung 2: a char literal keeps its KIND, so `{}` prints the character and not its code point
// (we used to print 97 for 'a'), `char` is a known type name, and a char still works everywhere an
// integer literal does -- a match literal, a range endpoint, an array repeat count, an `as` cast.
fn cls(c: char) -> isize { match c { 'a'..='z' => 1, 'A'..='Z' => 2, '0'..='9' => 3, _ => 0 } }
fn main() {
    let c = 'a';
    println!("{}", c);
    let d: char = 'Z';
    println!("{}", d);
    print!("{}{}", 'h', 'i');
    println!();
    println!("{} {} {} {}", cls('q'), cls('Q'), cls('7'), cls('!'));
    match c { 'a' => println!("exactly a"), _ => println!("other") }
    println!("{}", 'a' as isize);
    let n = ['x'; 3];
    println!("{} {} {}", n[0] as isize, n[1] as isize, n[2] as isize);
}
